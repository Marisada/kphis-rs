use axum::{
    BoxError,
    handler::HandlerWithoutStateExt,
    http::{StatusCode, Uri, uri::Authority},
    response::Redirect,
};
use axum_server::{Address, Handle};
use config::Config;
use sqlx::{
    AssertSqlSafe, Executor, Pool, Row,
    mysql::{MySql, MySqlConnectOptions, MySqlPoolOptions, MySqlRow},
};
use std::{
    net::SocketAddr,
    ops::DerefMut,
    sync::{Arc, RwLock},
    time::Duration,
};
use tokio::signal;
use tokio::sync::broadcast;
use tracing::{debug, info};

use kphis_api_query::log;
use kphis_model::user::permission::Permission;
use kphis_sql::user::role::select_all_permissions;
use kphis_util::error::{AppError, Source};

use crate::{pdf::core::JsonActorHandle, state::ApiState};

/// create logs folder and parse config
pub fn get_config(mode: &str) -> Config {
    let config_path = ["./volume/config/", mode, ".toml"].concat();
    let cfg = config::Config::builder()
        .add_source(config::File::with_name(&config_path))
        .build()
        .expect("Error create config from file");
    println!("Loading config from {}", &config_path);

    cfg
}

pub async fn get_db(config: &Config) -> Pool<MySql> {
    let url = config.get_string("db-url").expect("Not found 'db-url' in config file");
    let max_connections = config.get_int("db-max-connections").ok().and_then(|max| u32::try_from(max).ok()).unwrap_or(5);
    let acquire_timeout_sec = config.get_int("db-acquire-timeout-sec").ok().and_then(|max| u64::try_from(max).ok()).unwrap_or(10);
    let after_connect_sqls = config
        .get_array("db-after-connect-sqls")
        .ok()
        .and_then(|vs| vs.into_iter().map(|v| v.into_string()).collect::<Result<Vec<String>, config::ConfigError>>().ok())
        .unwrap_or_default();
    let params = MysqlParams {
        url: url.clone(),
        max_connections,
        acquire_timeout_sec,
        after_connect_sqls,
    };

    params.get_pool().await
}

struct MysqlParams {
    url: String,
    max_connections: u32,
    acquire_timeout_sec: u64,
    after_connect_sqls: Vec<String>,
}

impl MysqlParams {
    async fn get_pool(self) -> Pool<MySql> {
        let conn: MySqlConnectOptions = self.url.parse().expect("Fail to parse Mysql url");
        // let conn = conn.collation("tis620_thai_ci");
        // let conn = conn.charset("tis620");
        let pool = MySqlPoolOptions::new()
            .max_connections(self.max_connections)
            .acquire_timeout(Duration::from_secs(self.acquire_timeout_sec))
            .after_connect(move |conn, _meta| {
                let sqls = self.after_connect_sqls.clone();
                Box::pin(async move {
                    for sql in sqls {
                        conn.execute(AssertSqlSafe(sql)).await?;
                    }

                    Ok(())
                })
            })
            .connect_with(conn)
            .await
            .expect("Fail create MySQL pool");
        let url_split = self.url.split('@').collect::<Vec<&str>>();

        info!("Initializing MySql connection pool to {}", url_split[url_split.len() - 1]);
        pool
    }
}

/// return error when system_ac_permission and system_ac_role_permission <br>
/// has un-registered value
pub async fn check_permissions(pool: &Pool<MySql>, kphis: &str) -> Result<(), AppError> {
    let sql = select_all_permissions(kphis);
    let has_unknown = sqlx::query(AssertSqlSafe(sql))
        .fetch_all(pool)
        .await
        .map_err(|e| Source::SQLx.to_error(500, e, "Check Permission"))?
        .iter()
        .map(permission_from_row)
        .collect::<sqlx::Result<Vec<Permission>>>()
        .map_err(|e| Source::SQLx.to_error(500, e, "Check Permission"))?
        .contains(&Permission::Unknown);
    if has_unknown {
        Err(Source::App.to_error(500, "Has unknown permission in database", "Check Permission"))
    } else {
        Ok(())
    }
}

pub fn permission_from_row(row: &MySqlRow) -> sqlx::Result<Permission> {
    let name: String = row.try_get("permission")?;
    Ok(Permission::new(&name))
}

pub async fn delete_expired_log_and_message(days: i64, pool: &Pool<MySql>, kphis_log: &str) -> Result<(), AppError> {
    let _ = log::delete_expired_access_log(days, pool, kphis_log).await?;
    let _ = log::delete_expired_history_log(days, pool, kphis_log).await?;
    let _ = log::delete_expired_message(days, pool, kphis_log).await?;
    Ok(())
}

/// create ./volume/logs/{name} folder
pub fn create_log_folder(name: &str) -> Result<(), std::io::Error> {
    let dir = std::env::current_dir()?;
    let log_dir = dir.join("volume").join("logs").join(name);
    std::fs::create_dir_all(log_dir)
}

#[derive(Clone, Copy)]
pub struct Ports {
    http_port: u16,
    https_port: Option<u16>,
}

impl Ports {
    pub fn new(http_port: u16) -> Self {
        Self { http_port, https_port: None }
    }
    pub fn set_https(self, https_port: u16) -> Self {
        Self {
            http_port: self.http_port,
            https_port: Some(https_port),
        }
    }
    pub fn https(&self) -> u16 {
        self.https_port.unwrap_or(443)
    }
    pub fn http(&self) -> u16 {
        self.http_port
    }
}

/// generate http server to redirect to https server
/// with axum-server's handle to pass to shutdown-handle(handle)
pub async fn redirect_http_to_https<A>(ports: Ports, handle: Handle<A>, shutdown_recv: broadcast::Receiver<()>, json_handle: Arc<RwLock<JsonActorHandle>>, state: ApiState)
where
    A: Address + std::marker::Send + 'static,
{
    fn make_https(uri: Uri, ports: Ports) -> Result<Uri, BoxError> {
        let mut parts = uri.into_parts();

        parts.scheme = Some(axum::http::uri::Scheme::HTTPS);

        if parts.path_and_query.is_none() {
            parts.path_and_query = "/".parse().ok();
        }

        parts.authority = parts
            .authority
            .map(|auth| auth.as_str().replace(&ports.http_port.to_string(), &ports.https().to_string()).parse::<Authority>())
            .transpose()?;

        Ok(Uri::from_parts(parts)?)
    }

    let redirect = move |uri: Uri| async move {
        match make_https(uri, ports) {
            Ok(uri) => Ok(Redirect::permanent(&uri.to_string())),
            Err(error) => {
                tracing::warn!(%error, "failed to convert URI to HTTPS");
                Err(StatusCode::BAD_REQUEST)
            }
        }
    };

    let addr = SocketAddr::from(([0, 0, 0, 0], ports.http_port));
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    tracing::info!("HTTP service started at 0.0.0.0:{}, please Ctrl-c to terminate server.", ports.http_port);
    axum::serve(listener, redirect.into_make_service())
        .with_graceful_shutdown(shutdown_signal(handle, json_handle, shutdown_recv, state))
        .await
        .unwrap();
}

/// async function used for axum's graceful shutdown
/// with axum-server's handle for calling handle.graceful_shutdown
pub async fn shutdown_signal<A: Address>(handle: Handle<A>, json_handle: Arc<RwLock<JsonActorHandle>>, mut shutdown_recv: broadcast::Receiver<()>, state: ApiState) {
    let ctrl_c = async {
        signal::ctrl_c().await.expect("failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate()).expect("failed to install signal handler").recv().await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => info!("Ctrl-c signal received, starting graceful shutdown"),
        _ = terminate => info!("Terminate signal received, starting graceful shutdown"),
        _ = shutdown_recv.recv() => info!("Terminate signal received, starting graceful shutdown"),
    }

    // do cleanup stuff
    state.sse_clear_all().await;
    state.db_pool.close().await;
    join_api_handle(json_handle);
    handle.graceful_shutdown(Some(Duration::from_secs(5)));
}

pub fn join_api_handle(json_handle: Arc<RwLock<JsonActorHandle>>) {
    match json_handle.write() {
        Ok(mut lock) => {
            let h = lock.deref_mut();
            if let Some(join) = h.take_join_handle() {
                join.join().expect("error joining json actor thread");
                debug!("json actor thread joined");
            }
        }
        Err(e) => {
            println!("{e}")
        }
    }
}

#[cfg(test)]
#[rustfmt::skip]
mod tests {

    use super::*;
    use kphis_sqlx_tester::MySqlTester;

    #[tokio::test]
    #[ignore]
    async fn sqlx_check_permissions() {
        let tester = MySqlTester::new_kphis().await;
        sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/create/kphis/system_ac_role.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/create/kphis/system_ac_operation.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/create/kphis/system_ac_resource.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/create/kphis/system_acc_permission.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/create/kphis/system_acc_role_permission.sql")).execute(&tester.db_pool).await.unwrap();

        sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/insert/kphis/system_ac_role.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/insert/kphis/system_ac_operation.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/insert/kphis/system_ac_resource.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/insert/kphis/system_acc_permission.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/insert/kphis/system_acc_role_permission.sql")).execute(&tester.db_pool).await.unwrap();

        assert!(check_permissions(&tester.db_pool, &tester.kphis).await.is_ok());
    }
}
