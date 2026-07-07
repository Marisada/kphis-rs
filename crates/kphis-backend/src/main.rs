// Using Jemalloc in MUSL has a 10x boost than MUSL default allocator
// and in some cases performs even better than the libc alloator.
// MiMalloc also has a same boost but MiMalloc eats a bit more memory than Jemalloc.
// https://github.com/clux/muslrust/issues/142#issuecomment-2184935013
#[cfg(all(target_arch = "x86_64", target_env = "musl"))]
#[global_allocator]
static GLOBAL: tikv_jemallocator::Jemalloc = tikv_jemallocator::Jemalloc;

#[cfg(all(target_arch = "aarch64", target_env = "musl"))]
#[global_allocator]
static GLOBAL: mimalloc::MiMalloc = mimalloc::MiMalloc;

use axum::Router;
use axum_server::{Handle, tls_rustls::RustlsConfig};
use clap::Parser;
use std::{
    net::SocketAddr,
    sync::{Arc, RwLock},
};
use time::{UtcOffset, format_description::well_known::Rfc3339};
use tokio::sync::broadcast;
use tokio_cron_scheduler::{Job, JobScheduler};
use tracing::{debug, info, warn};
use tracing_appender::rolling::{RollingFileAppender, Rotation};
use tracing_subscriber::{EnvFilter, Layer, fmt::time::OffsetTime, layer::SubscriberExt};
// use utoipa_swagger_ui::SwaggerUi;
// use utoipa_rapidoc::RapiDoc;
// use utoipa_redoc::{Redoc, Servable};
// use utoipa_scalar::{Scalar, Servable as ScalarServable};

use kphis_api_core::{
    pdf::core::JsonActorHandle,
    state::ApiState,
    utils::{Ports, create_log_folder, delete_expired_log_and_message, get_config, get_db, join_api_handle, redirect_http_to_https, shutdown_signal},
};
use kphis_api_pdf::actor::run_api_actor;
use kphis_api_router::new_router;

// Command line management
#[derive(Clone, Debug, Parser)]
#[clap(author, version, about, long_about = None,
    rename_all = "kebab-case",
    rename_all_env = "screaming-snake")]
struct Args {
    /// Set config environment by config file name Ex. /volume/config/debug.toml -> debug
    #[arg(value_enum, value_parser, default_value = "debug")]
    mode: String,
}

fn main() {
    // parse Clap Arguments
    let args = Args::parse();
    let Args { mode } = args;

    // get command line argument, load config
    let config = get_config(&mode);

    // create log folder
    create_log_folder("rolling").expect("Cannot create ./volume/logs/rolling");

    // set Tracing Subscriber
    let log_file = config.get_string("log-file").expect("'log-file' not found in config file");
    let log_console = config.get_string("log-console").expect("'log-console' not found in config file");
    let keep_log_day = config.get_int("app-keep-log-day").expect("Not found 'app-keep-log-day' in config file") as usize;

    let file_appender = RollingFileAppender::builder()
        .rotation(Rotation::HOURLY)
        .max_log_files(keep_log_day * 24)
        .filename_prefix("log")
        .build("./volume/logs/rolling")
        .expect("Error create LogAppender");
    let (file_non_blocking, _guard) = tracing_appender::non_blocking(file_appender);
    let timer = OffsetTime::new(UtcOffset::from_hms(7, 0, 0).unwrap_or(UtcOffset::UTC), Rfc3339);
    let subscriber = tracing_subscriber::registry()
        // log to file
        .with(
            tracing_subscriber::fmt::Layer::new()
                .with_writer(file_non_blocking)
                .with_timer(timer.clone())
                .with_ansi(false)
                .with_target(false)
                .with_filter(EnvFilter::new(log_file)),
        )
        // log to console
        .with(
            tracing_subscriber::fmt::Layer::new()
                .with_writer(std::io::stdout)
                .with_timer(timer)
                .with_ansi(true)
                .with_target(true)
                .with_filter(EnvFilter::new(log_console)),
        );
    tracing::subscriber::set_global_default(subscriber).expect("Unable to set a global subscriber");
    info!("Start logging");

    // handle for loading Typst's json data by calling GET query fn internally
    let json_handle = Arc::new(RwLock::new(JsonActorHandle::new(run_api_actor)));

    let rt = tokio::runtime::Builder::new_multi_thread().enable_all().build().expect("Failed to build Multi-Thread Runtime");
    rt.block_on(async move { run(config, json_handle).await })
}

async fn run(config: config::Config, json_handle: Arc<RwLock<JsonActorHandle>>) {
    // init pool
    let db_pool = get_db(&config).await;

    let (shutdown_sender, shutdown_recv) = broadcast::channel(5);
    let shutdown_recv2 = shutdown_sender.subscribe();

    // create ApiState
    let state = ApiState::new(&config, db_pool, json_handle.clone(), shutdown_sender).await;

    // set https certificate
    let https_port_result = config.get_int("https-port").map(|port| u16::try_from(port).expect("Fail parsing 'https-port'"));
    let https_config = match https_port_result {
        Ok(https_port) => {
            // Fix error : no process-level CryptoProvider available -- call CryptoProvider::install_default() before this point
            // MUST run this line before RustlsConfig creation
            // read more at https://docs.rs/rustls/latest/rustls/crypto/struct.CryptoProvider.html
            rustls::crypto::aws_lc_rs::default_provider().install_default().expect("Fail to install crypto provider");

            let https_cert_path = config.get_string("https-cert-path").expect("'https-cert-path' not found in config file");
            let https_key_path = config.get_string("https-key-path").expect("'https-key-path' not found in config file");
            let tls_config = RustlsConfig::from_pem_file(&https_cert_path, &https_key_path).await.expect("certificate not a valid pem file");
            Some((https_port, tls_config, https_cert_path, https_key_path))
        }
        Err(_) => None,
    };

    // Cron job scheduler
    let sched = JobScheduler::new().await.expect("Fail create Schedule");
    let keep_log_day = config.get_int("app-keep-log-day").expect("Not found 'app-keep-log-day' in config file");

    // #1 cron job: Cleaning logs and messages
    let state_c1 = state.clone();
    let cron_cleaner = config.get_string("app-cron-cleaner").expect("Not found 'app-cron-cleaner' in config file");
    let clean_job = Job::new_async(cron_cleaner.as_str(), move |_uuid, _l| {
        let state_cc = state_c1.clone();
        Box::pin(async move {
            info!("Cleaning logs and messages schedule start..");
            if let Err(e) = delete_expired_log_and_message(keep_log_day, &state_cc.db_pool, &state_cc.kphis_log()).await {
                warn!("Cannot {}: {}", &e.action, &e.message);
            }
            debug!("Cleaning logs and messages finished");
        })
    })
    .expect("Fail to create cleaning job");
    let _ = sched.add(clean_job).await.expect("Fail adding clean job to Schedule");

    // #2 cron job : Check triggers
    let state_c2 = state.clone();
    let cron_trigger = config.get_string("app-cron-trigger").expect("Not found 'app-cron-trigger' in config file");
    let trigger_job = Job::new_async(cron_trigger.as_str(), move |_uuid, _l| {
        let state_cc = state_c2.clone();
        Box::pin(async move {
            info!("Check triggers schedule start..");
            state_cc.check_and_apply_triggers().await;
            debug!("Checking triggers finished");
        })
    })
    .expect("Fail to create trigger job");
    let _ = sched.add(trigger_job).await.expect("Fail adding trigger job to Schedule");

    // #3 cron job
    if let Some((_, tls_config, cert_path, key_path)) = https_config.clone() {
        let cron_reload_cert = config.get_string("app-cron-reload-cert").expect("Not found 'app-cron-reload-cert' in config file");
        let cert_job = Job::new_async(cron_reload_cert.as_str(), move |_uuid, _l| {
            let tls_config_c = tls_config.clone();
            let cert_path_c = cert_path.clone();
            let key_path_c = key_path.clone();
            Box::pin(async move {
                info!("Reload Certificate schedule start..");
                match tls_config_c.reload_from_pem_file(cert_path_c, key_path_c).await {
                    Err(e) => {
                        warn!("Cannot reload certificate due to: {}", e);
                    }
                    Ok(()) => {
                        debug!("reload certificate finished");
                    }
                }
            })
        })
        .expect("Fail to create cleaning job");
        let _ = sched.add(cert_job).await.expect("Fail adding cert job to Schedule");
    }
    // start cron job
    sched.start().await.expect("Fail start Scheduler");

    // create router
    let router = new_router(&state);

    // prepare server variables
    let http_port = u16::try_from(config.get_int("http-port").expect("'http-port' not found  in config file")).expect("Fail parsing 'http-port'");

    // graceful shutdown issue:
    // 1. axum use 'future finished' to trigger graceful shutdown
    // 2. axum-server use 'handle' passed to server, call handler.graceful_shutdown(duration) to start shutdown process
    // so we call 'handle' in http server's .with_graceful_shutdown process
    // that mean ctrl-c -> shutdown http -> call handle -> shutdown https
    let shutdown_handle = Handle::new();

    let port = Ports::new(http_port);

    match https_config {
        Some((https_port, tls_config, ..)) => {
            let http_to_https = config.get_bool("http-to-https").expect("'http-to-https' not found in config file");

            let ports = port.set_https(https_port);
            // run http server
            // we needd to pass handle to both redirect/no-redirect path
            if http_to_https {
                // redirect : spawn a second server to redirect http requests to this server
                tokio::spawn(redirect_http_to_https(ports, shutdown_handle.clone(), shutdown_recv, json_handle.clone(), state.clone()));
            } else {
                // no-redirect : spawn a second http server to serve same router as https
                spawn_http("HTTP", ports.http(), router.clone(), shutdown_handle.clone(), shutdown_recv, json_handle.clone(), state.clone());
            }
            // run https server
            serve_https(ports.https(), tls_config, router, shutdown_handle, shutdown_recv2, json_handle.clone(), state.clone()).await;
        }
        None => {
            // run http server
            serve_http(port.http(), router, shutdown_handle, shutdown_recv, json_handle.clone(), state.clone()).await;
        }
    }
}

fn spawn_http(service_name: &'static str, port: u16, router: Router, handle: Handle<SocketAddr>, shutdown_recv: broadcast::Receiver<()>, json_handle: Arc<RwLock<JsonActorHandle>>, state: ApiState) {
    tokio::spawn(async move {
        let addr = SocketAddr::from(([0, 0, 0, 0], port));
        let listener = tokio::net::TcpListener::bind(addr).await.expect("Failed to bind Tcp Socket Address");
        info!("{} service started at 0.0.0.0:{}, please Ctrl-c to terminate server.", service_name, port);
        axum::serve(listener, router.into_make_service_with_connect_info::<SocketAddr>())
            .with_graceful_shutdown(shutdown_signal(handle, json_handle, shutdown_recv, state))
            .await
            .expect("Failed Serve")
    });
}

async fn serve_http(port: u16, router: Router, handle: Handle<SocketAddr>, shutdown_recv: broadcast::Receiver<()>, json_handle: Arc<RwLock<JsonActorHandle>>, state: ApiState) {
    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    let listener = tokio::net::TcpListener::bind(addr).await.expect("Failed to bind Tcp Socket Address");
    info!("HTTP service started at 0.0.0.0:{}, please Ctrl-c to terminate server.", port);
    axum::serve(listener, router.into_make_service_with_connect_info::<SocketAddr>())
        .with_graceful_shutdown(shutdown_signal(handle, json_handle, shutdown_recv, state))
        .await
        .expect("Failed Serve HTTP")
}

async fn serve_https(port: u16, tls: RustlsConfig, router: Router, handle: Handle<SocketAddr>, shutdown_recv: broadcast::Receiver<()>, json_handle: Arc<RwLock<JsonActorHandle>>, state: ApiState) {
    let https_addr = SocketAddr::from(([0, 0, 0, 0], port));

    info!("HTTPs service started at 0.0.0.0:{}, please Ctrl-c to terminate server.", port);

    tokio::spawn(shutdown_signal(handle.clone(), json_handle.clone(), shutdown_recv, state));

    axum_server::bind_rustls(https_addr, tls)
        .handle(handle)
        .serve(router.into_make_service_with_connect_info::<SocketAddr>())
        .await
        .expect("Failed Serve HTTPs");

    join_api_handle(json_handle);
}
