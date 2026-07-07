use std::{
    net::SocketAddr,
    path::{Path, PathBuf},
};
use strum::IntoEnumIterator;
use tokio::{fs::File, io::AsyncWriteExt, sync::broadcast};
use typst_layout::PagedDocument;
use typst_library::diag::{EcoString, FileError};

use kphis_api_core::{
    pdf::{loader::strip_slash, runtime::SystemWorld, signer::write_sig_content},
    state::{ApiState, UserState},
    utils::join_api_handle,
};
use kphis_api_pdf::{bundle_data::pdf_data, handler, test_state::new_test_state};
use kphis_api_query::transform::trigger::{add_update_all_an_procedure, call_update_all_an_procedure};
use kphis_model::{
    image::ImageBase64,
    report::{SystemReport, TypstReport},
    user::his::{CurrentUserRole, UserDb},
};
use kphis_sqlx_tester::MySqlMocker;
use kphis_util::error::{AppError, Source};

#[tokio::test(flavor = "multi_thread", worker_threads = 1)]
#[ignore]
async fn pdf_create_by_api() {
    let mock = MySqlMocker::new_all().await;
    let (shutdown_sender, _shutdown_recv) = broadcast::channel(5);
    let state = new_test_state(mock.db_pool.clone(), shutdown_sender).await;
    let mut world = SystemWorld::new();

    println!("");
    println!("1. Test bundle_data::pdf_data");
    pdf_creator(&state, &mut world, false, true).await;
    pdf_all(&state, &mut world, false, true).await;
    // test api fetch from `JsonActor` and `get_json_data::json_data`
    println!("2. Test get_json_data::json_data");
    pdf_creator(&state, &mut world, false, false).await;
    pdf_all(&state, &mut world, false, false).await;

    println!("Clean up");
    join_api_handle(state.json_handle);
}

#[tokio::test(flavor = "multi_thread", worker_threads = 1)]
#[ignore]
async fn pdf_create_by_api_pre_admit() {
    let mock = MySqlMocker::new_all().await;
    let (shutdown_sender, _shutdown_recv) = broadcast::channel(5);
    let state = new_test_state(mock.db_pool.clone(), shutdown_sender).await;
    let mut world = SystemWorld::new();

    println!("");
    println!("Change AN with VN (for testing Pre-Admit)");
    add_update_all_an_procedure(&state.db_pool, &mock.kphis, &mock.kphis_extra).await.unwrap();
    call_update_all_an_procedure("660001234", "661231235959", &state.db_pool, &mock.kphis).await.unwrap();

    // Pre-Admit reports
    println!("1. Test bundle_data::pdf_data");
    pdf_creator(&state, &mut world, true, true).await;
    pdf_all(&state, &mut world, true, true).await;
    // test api fetch from `JsonActor` and `get_json_data::json_data`
    println!("2. Test get_json_data::json_data");
    pdf_creator(&state, &mut world, true, false).await;
    pdf_all(&state, &mut world, true, false).await;

    println!("Clean up");
    join_api_handle(state.json_handle);
}

/// Test all reports excepts `pre_admit` will test only IPD reports with id: `661231235959`
async fn pdf_creator(state: &ApiState, world: &mut SystemWorld, is_pre_admit: bool, using_bundle: bool) {
    let user_db = UserDb {
        loginname: String::from("tester"),
        passweb: String::from("abcdef"),
        name: String::from("TesterMan"),
        doctorcode: Some(String::from("007")),
        groupname: Some(String::from("ER")),
        accessright: String::from("[]"),
        entryposition: Some(String::from("admin")),
        licenseno: Some(String::from("11111")),
        image: Some(ImageBase64::new_user()),
        ..Default::default()
    };
    let user = UserState {
        state_id: 1,
        user: user_db,
        roles: vec![CurrentUserRole::demo()],
        permissions: Vec::new(),
        addr: "127.0.0.1:12345".parse::<SocketAddr>().unwrap(),
    };

    let output_folder = create_pdf_output_folder(using_bundle, is_pre_admit).unwrap();
    let mut volume_path = PathBuf::new();
    volume_path.push("../../");

    let reports = if is_pre_admit {
        SystemReport::iter().filter(|rp| rp.key_names() == "AN").collect::<Vec<SystemReport>>()
    } else {
        SystemReport::iter().collect::<Vec<SystemReport>>()
    };
    for report in reports {
        // for testing `all` report
        let id = report_test_id(&report, is_pre_admit);
        let typ_file = tokio::fs::read_to_string(volume_path.join(report.typ_path_server())).await.unwrap();
        let mut output_path = output_folder.join(report.template_name());
        output_path.set_extension("pdf");
        // create pdf bytes
        let bytes = create_pdf(using_bundle, report, id, &typ_file, state, world, &user).await.unwrap();
        // save pdf file to `test_output` folder
        save_file(&bytes, &output_path).await.unwrap();
        println!("{} created", &output_path.to_str().unwrap_or_default());
    } // for testing `all` report

    if !using_bundle && !is_pre_admit {
        // check `kphis-sqlx-tester/src/test_sqls/insert/report_template.sql` for `demo` template
        let custom_bytes = create_pdf_custom(String::from("demo"), "99999|007", state, world, &user).await.unwrap();
        let mut custom_path = output_folder.join("demo");
        custom_path.set_extension("pdf");
        save_file(&custom_bytes, &custom_path).await.unwrap();
        println!("{} created", &custom_path.to_str().unwrap_or_default());
    }
}

/// Test all reports to single pdf file (excepts `pre_admit` will test only IPD reports with id: `661231235959`)
async fn pdf_all(state: &ApiState, world: &mut SystemWorld, is_pre_admit: bool, using_bundle: bool) {
    let user_db = UserDb {
        loginname: String::from("tester"),
        passweb: String::from("abcdef"),
        name: String::from("TesterMan"),
        doctorcode: Some(String::from("007")),
        groupname: Some(String::from("ER")),
        accessright: String::from("[]"),
        entryposition: Some(String::from("admin")),
        licenseno: Some(String::from("11111")),
        image: Some(ImageBase64::new_user()),
        ..Default::default()
    };
    let user = UserState {
        state_id: 1,
        user: user_db,
        roles: vec![CurrentUserRole::demo()],
        permissions: Vec::new(),
        addr: "127.0.0.1:12345".parse::<SocketAddr>().unwrap(),
    };

    let output_folder = create_pdf_output_folder(using_bundle, is_pre_admit).unwrap();
    let mut volume_path = PathBuf::new();
    volume_path.push("../../");

    let reports = if is_pre_admit {
        SystemReport::iter().filter(|rp| rp.key_names() == "AN").collect::<Vec<SystemReport>>()
    } else {
        SystemReport::iter().collect::<Vec<SystemReport>>()
    };

    let mut documents = Vec::new();
    for report in reports {
        // for testing `all` report
        let id = report_test_id(&report, is_pre_admit);
        let typ_file = tokio::fs::read_to_string(volume_path.join(report.typ_path_server())).await.unwrap();
        // create pdf bytes
        let document = create_paged_document(using_bundle, report, id, &typ_file, state, world, &user).await.unwrap();
        documents.push(document);
    }
    let (pdf_option, need_signing) = handler::generate_pdf_option(state);
    let pdf_unsign_buf = typst_pdf::merge(&documents, &pdf_option).unwrap();
    let pdf_buf = if need_signing { write_sig_content(pdf_unsign_buf, state).unwrap() } else { pdf_unsign_buf };

    let mut output_path = output_folder.join("merged");
    output_path.set_extension("pdf");
    // save pdf file to `test_output` folder
    save_file(&pdf_buf, &output_path).await.unwrap();
    println!("{} created", &output_path.to_str().unwrap_or_default());
}

async fn create_pdf(using_bundle: bool, report: SystemReport, id: &str, typ_file: &str, state: &ApiState, world: &mut SystemWorld, user: &UserState) -> Result<Vec<u8>, AppError> {
    let title = [report.title(), " (", report.key_names(), ": ", id, ")"].concat();
    let mut document = create_paged_document(using_bundle, report, id, typ_file, state, world, user).await?;

    handler::generate_pdf_file(&mut document, &title, state, user)
}

async fn create_paged_document(using_bundle: bool, report: SystemReport, id: &str, typ_file: &str, state: &ApiState, world: &mut SystemWorld, user: &UserState) -> Result<PagedDocument, AppError> {
    let data_json = if using_bundle {
        pdf_data(&TypstReport::System(report), id, state, user).await?
    } else if let Some(v) = id.split('|').next() {
        serde_json::json!({"id": v}).to_string()
    } else {
        serde_json::json!({"id": id}).to_string()
    };

    // tokio::task::spawn_blocking(move || handler::create_pdf(&typ_file, &data_json, &title, state, user, world)).await.unwrap()
    handler::create_paged_document(typ_file, &data_json, state, user, world, load_file)
}

async fn create_pdf_custom(template: String, ids: &str, state: &ApiState, world: &mut SystemWorld, user: &UserState) -> Result<Vec<u8>, AppError> {
    if let Some(typst_report) = handler::new_typst_report(template, String::from("custom"), state).await? {
        let (typ_file, data_json) = handler::prepare_template_data(&typst_report, &ids, state, user).await?;

        handler::create_pdf(&typ_file, &data_json, &typst_report.title_with_ids(&ids), state, user, world, load_file)
    } else {
        Err(Source::App.to_error(404, "Template Not Found", "Get Template"))
    }
}

/// - Lab, OPD-ER and `pre-admit` IPD reports will test with `661231235959`
/// - `admited` IPD reports will test with `660001234`
fn report_test_id(report: &SystemReport, is_pre_admit: bool) -> &'static str {
    match (report.key_names(), is_pre_admit) {
        ("AN", false) => "660001234",
        ("VN/AN|DOC-TYPE-ID|PER-PAGE", false) => "660001234|1|1",
        ("VN/AN|DOC-TYPE-ID|PER-PAGE", true) => "661231235959|1|1",
        ("VN/AN|KEY|PER-PAGE", false) => "660001234|lab|1",
        ("VN/AN|KEY|PER-PAGE", true) => "661231235959|lab|1",
        _ => "661231235959",
    }
}

fn create_pdf_output_folder(using_bundle: bool, is_pre_admit: bool) -> Result<PathBuf, std::io::Error> {
    let main = if using_bundle { "bundle" } else { "api" };
    let sub = if is_pre_admit { "pre-admit" } else { "default" };
    let dir = std::env::current_dir()?;
    let pdf_out_dir = dir.join("test_output").join(main).join(sub);
    std::fs::create_dir_all(&pdf_out_dir)?;

    Ok(pdf_out_dir)
}

async fn save_file(bytes: &[u8], path: &Path) -> Result<(), std::io::Error> {
    let mut file = File::create(path).await?;
    file.write_all(bytes).await?;

    Ok(())
}

// copy from `kphis_api_core::pdf::loader::load_file` + add `../../` to path
fn load_file(path: PathBuf, state: &Option<ApiState>, user: &Option<UserState>) -> Result<Vec<u8>, FileError> {
    let path = strip_slash(&path);
    if ["jsons", "statics", "templates", "typsts"].iter().any(|key| path.starts_with(key)) {
        let mut p = PathBuf::new();
        // modify this line from `kphis_api_core::pdf::loader::load_file`
        p.push("../../volume/pwa");
        p.push(path);
        std::fs::read(&p).map_err(|e| FileError::from_io(e, &p))
    } else if path.starts_with("thumbs") || path.starts_with("images") {
        let mut p = PathBuf::new();
        p.push("../../volume");
        p.push(path);
        std::fs::read(&p).map_err(|e| FileError::from_io(e, &p))
    } else if let (Some(app_state), Some(user_state)) = (state, user) {
        app_state.get_api(&path, user_state)
    } else {
        Err(FileError::Other(Some(EcoString::from("State Not Found"))))
    }
}
