use axum::{Json, extract::Query, http::Method};

use kphis_api_core::{
    open_api::{DocOne, DocOpt, DocVec, DocVecU32},
    state::RequestState,
};
use kphis_api_query::opd_er::medical_history;
use kphis_model::{
    fetch::ExecuteResponse,
    opd_er::medical_history::{AllergyHistory, ConsultHistory, NurseScreeningHistory, OpdErMedicalHistory, OpdErMedicalHistoryParams, ScanHistory, SetFtHistory, TraumaHistory},
};
use kphis_util::error::{AppError, Source};

// opd-er-medical-history-data.php
/// /api/opd-er/medical-history
///
/// Get OPD-ER Medical History by PARAMS, return single OPD-ER Medical History
/// - Query parameter `only_opdscreen` is true: `vn` must not null or empty
/// - Query parameter `only_opdscreen` is null or false: `opd_er_order_master_id` and `hn` and `vn` and `visit_datetime` must not null
#[utoipa::path(
    get,
    path = "/opd-er/medical-history",
    responses(DocOne<OpdErMedicalHistory>),
    params(OpdErMedicalHistoryParams),
)]
pub async fn get_opd_er_medical_history(Query(params): Query<OpdErMedicalHistoryParams>, ctx: RequestState) -> Result<Json<OpdErMedicalHistory>, AppError> {
    ctx.user_state.trace_req_by();
    ctx.authorize_and_access_log(&Method::GET, false).await?;

    let hospital_name = &ctx.api_state.app_config.hospital_name;
    let params = params.clean();
    if params.valid() {
        let response = medical_history::get_medical_history(&params, hospital_name, &ctx.api_state.db_pool, &ctx.api_state.hosxp(), &ctx.api_state.kphis()).await?;

        Ok(Json(response))
    } else {
        Err(AppError::app_400("Get OpdErMedicalHistory"))
    }
}

// opd-er-medical-history-dr-data.php
/// /api/opd-er/medical-history-trauma
///
/// Get OPD-ER Trauma History by PARAMS, return single OPD-ER Trauma History or none
///
/// Require opd_er_order_master_id in PARAMS
#[utoipa::path(
    get,
    path = "/opd-er/medical-history-trauma",
    responses(DocOpt<TraumaHistory>),
    params(OpdErMedicalHistoryParams),
)]
pub async fn get_opd_er_trauma_history(Query(params): Query<OpdErMedicalHistoryParams>, ctx: RequestState) -> Result<Json<Option<TraumaHistory>>, AppError> {
    ctx.user_state.trace_req_by();
    ctx.authorize_and_access_log(&Method::GET, false).await?;

    if let Some(opd_er_order_master_id) = params.opd_er_order_master_id {
        let response = medical_history::get_trauma_history(
            opd_er_order_master_id,
            &ctx.api_state.db_pool,
            &ctx.api_state.hosxp(),
            &ctx.api_state.kphis(),
            &ctx.api_state.kphis_extra(),
        )
        .await?;

        Ok(Json(response))
    } else {
        Ok(Json(None))
    }
}

// opd-er-medical-history-dr-save.php
// opd-er-medical-history-dr-update.php
/// /api/opd-er/medical-history-trauma
///
/// Tries to create/edit OPD-ER Trauma History
#[utoipa::path(
    post,
    path = "/opd-er/medical-history-trauma",
    request_body = TraumaHistory,
    responses(DocVecU32<ExecuteResponse>),
)]
pub async fn post_opd_er_trauma_history(ctx: RequestState, Json(payload): Json<TraumaHistory>) -> Result<Json<(u32, Vec<ExecuteResponse>)>, AppError> {
    ctx.user_state.trace_req_by();
    ctx.authorize_and_access_log(&Method::POST, false).await?;

    let response = medical_history::post_trauma_history(
        &payload,
        &ctx.user_state.user.doctorcode,
        &ctx.user_state.user.loginname,
        &ctx.api_state.db_pool,
        &ctx.api_state.kphis(),
        &ctx.api_state.kphis_log(),
    )
    .await?;

    Ok(Json(response))
}

// opd-er-allergy-history-edit.php
/// /api/opd-er/medical-history-allergy
///
/// Get OPD-ER Allergy History by PARAMS, return a list of OPD-ER Allergy History
///
/// Require opd_er_order_master_id in PARAMS
#[utoipa::path(
    get,
    path = "/opd-er/medical-history-allergy",
    responses(DocVec<AllergyHistory>),
    params(OpdErMedicalHistoryParams),
)]
pub async fn get_opd_er_allergy_history(Query(params): Query<OpdErMedicalHistoryParams>, ctx: RequestState) -> Result<Json<Vec<AllergyHistory>>, AppError> {
    ctx.user_state.trace_req_by();
    ctx.authorize_and_access_log(&Method::GET, false).await?;

    if let Some(opd_er_order_master_id) = params.opd_er_order_master_id {
        let responses = medical_history::get_allergy_history(opd_er_order_master_id, &ctx.api_state.db_pool, &ctx.api_state.hosxp(), &ctx.api_state.kphis()).await?;

        Ok(Json(responses))
    } else {
        Ok(Json(Vec::new()))
    }
}

// opd-er-allergy-history-save.php
/// /api/opd-er/medical-history-allergy
///
/// Tries to create/edit OPD-ER Allergy History
/// - Payload's `opd_er_order_master_id` and `version` must has the same value
#[utoipa::path(
    post,
    path = "/opd-er/medical-history-allergy",
    request_body = Vec<AllergyHistory>,
    responses(DocVec<ExecuteResponse>),
)]
pub async fn post_opd_er_allergy_history(ctx: RequestState, Json(payload): Json<Vec<AllergyHistory>>) -> Result<Json<Vec<ExecuteResponse>>, AppError> {
    ctx.user_state.trace_req_by();
    ctx.authorize_and_access_log(&Method::POST, false).await?;

    if payload.is_empty() {
        Ok(Json(Vec::new()))
    } else {
        // check opd_er_order_master_id and version MUST BE the same value in each item
        let mut ids = payload.iter().map(|s| s.opd_er_order_master_id.unwrap_or_default()).collect::<Vec<u32>>();
        ids.dedup();
        let mut versions = payload.iter().map(|s| s.version).collect::<Vec<i32>>();
        versions.dedup();

        if ids.len() == 1 && versions.len() == 1 {
            let response = medical_history::post_allergy_history(
                &payload,
                ids[0],
                versions[0],
                &ctx.user_state.user.doctorcode,
                &ctx.user_state.user.loginname,
                &ctx.api_state.db_pool,
                &ctx.api_state.kphis(),
                &ctx.api_state.kphis_log(),
            )
            .await?;

            Ok(Json(response))
        } else {
            Err(AppError::app_400("Post AllergyHistory"))
        }
    }
}

// opd-er-medical-history-nurse-data.php
/// /api/opd-er/medical-history-screen
///
/// Get OPD-ER Screening History by PARAMS, return single OPD-ER Screening History or none
///
/// Require opd_er_order_master_id in PARAMS
#[utoipa::path(
    get,
    path = "/opd-er/medical-history-screen",
    responses(DocOpt<NurseScreeningHistory>),
    params(OpdErMedicalHistoryParams),
)]
pub async fn get_opd_er_screen_history(Query(params): Query<OpdErMedicalHistoryParams>, ctx: RequestState) -> Result<Json<Option<NurseScreeningHistory>>, AppError> {
    ctx.user_state.trace_req_by();
    ctx.authorize_and_access_log(&Method::GET, false).await?;

    if let Some(opd_er_order_master_id) = params.opd_er_order_master_id {
        let response = medical_history::get_screen_history(opd_er_order_master_id, &ctx.api_state.db_pool, &ctx.api_state.hosxp(), &ctx.api_state.kphis()).await?;

        Ok(Json(response))
    } else {
        Ok(Json(None))
    }
}

// opd-er-medical-history-nurse-save.php
// opd-er-medical-history-nurse-update.php
/// /api/opd-er/medical-history-screen
///
/// Tries to create/edit OPD-ER Screeing History
/// - Query parameter `view_by` must be `doctor` or `nurse`
#[utoipa::path(
    post,
    path = "/opd-er/medical-history-screen",
    request_body = NurseScreeningHistory,
    responses(DocVecU32<ExecuteResponse>),
    params(OpdErMedicalHistoryParams),
)]
pub async fn post_opd_er_screen_history(
    Query(params): Query<OpdErMedicalHistoryParams>,
    ctx: RequestState,
    Json(payload): Json<NurseScreeningHistory>,
) -> Result<Json<(u32, Vec<ExecuteResponse>)>, AppError> {
    ctx.user_state.trace_req_by();
    ctx.authorize_and_access_log(&Method::POST, false).await?;

    if let Some(view_by) = params.view_by.as_ref() {
        if ["doctor", "nurse"].contains(&view_by.as_str()) {
            let response = medical_history::post_screen_history(
                &payload,
                view_by,
                &ctx.user_state.user.doctorcode,
                &ctx.user_state.user.loginname,
                &ctx.api_state.db_pool,
                &ctx.api_state.kphis(),
                &ctx.api_state.kphis_log(),
            )
            .await?;

            Ok(Json(response))
        } else {
            Err(Source::App.to_error(400, "Invalid view_by", "Post ScreenHistory"))
        }
    } else {
        Err(Source::App.to_error(400, "view_by not found", "Post ScreenHistory"))
    }
}

// opd-er-consult-dr-edit.php
/// /api/opd-er/medical-history-consult
///
/// Get OPD-ER Consult History by PARAMS, return a list of OPD-ER Consult History
///
/// Require opd_er_order_master_id in PARAMS
#[utoipa::path(
    get,
    path = "/opd-er/medical-history-consult",
    responses(DocVec<ConsultHistory>),
    params(OpdErMedicalHistoryParams),
)]
pub async fn get_opd_er_consult_history(Query(params): Query<OpdErMedicalHistoryParams>, ctx: RequestState) -> Result<Json<Vec<ConsultHistory>>, AppError> {
    ctx.user_state.trace_req_by();
    ctx.authorize_and_access_log(&Method::GET, false).await?;

    if let Some(opd_er_order_master_id) = params.opd_er_order_master_id {
        let responses = medical_history::get_consult_history(opd_er_order_master_id, &ctx.api_state.db_pool, &ctx.api_state.hosxp(), &ctx.api_state.kphis()).await?;

        Ok(Json(responses))
    } else {
        Ok(Json(Vec::new()))
    }
}

// opd-er-consult-dr-save.php
/// /api/opd-er/medical-history-consult
///
/// Tries to create/edit OPD-ER Consult History
/// - Payload's `opd_er_order_master_id` and `version` must has the same value
#[utoipa::path(
    post,
    path = "/opd-er/medical-history-consult",
    request_body = Vec<ConsultHistory>,
    responses(DocVec<ExecuteResponse>),
)]
pub async fn post_opd_er_consult_history(ctx: RequestState, Json(payload): Json<Vec<ConsultHistory>>) -> Result<Json<Vec<ExecuteResponse>>, AppError> {
    ctx.user_state.trace_req_by();
    ctx.authorize_and_access_log(&Method::POST, false).await?;

    if payload.is_empty() {
        Ok(Json(Vec::new()))
    } else {
        // check opd_er_order_master_id and version MUST BE the same value in each item
        let mut ids = payload.iter().map(|s| s.opd_er_order_master_id.unwrap_or_default()).collect::<Vec<u32>>();
        ids.dedup();
        let mut versions = payload.iter().map(|s| s.version).collect::<Vec<i32>>();
        versions.dedup();

        if ids.len() == 1 && versions.len() == 1 {
            let response = medical_history::post_consult_history(
                &payload,
                ids[0],
                versions[0],
                &ctx.user_state.user.doctorcode,
                &ctx.user_state.user.loginname,
                &ctx.api_state.db_pool,
                &ctx.api_state.kphis(),
                &ctx.api_state.kphis_log(),
            )
            .await?;

            Ok(Json(response))
        } else {
            Err(AppError::app_400("Post ConsultHistory"))
        }
    }
}

// opd-er-document-scan-data.php
/// /api/opd-er/medical-history-scan
///
/// Get OPD-ER Scan History by PARAMS, return single OPD-ER Scan History or none
///
/// Require opd_er_order_master_id in PARAMS
#[utoipa::path(
    get,
    path = "/opd-er/medical-history-scan",
    responses(DocOpt<ScanHistory>),
    params(OpdErMedicalHistoryParams),
)]
pub async fn get_opd_er_scan_history(Query(params): Query<OpdErMedicalHistoryParams>, ctx: RequestState) -> Result<Json<Option<ScanHistory>>, AppError> {
    ctx.user_state.trace_req_by();
    ctx.authorize_and_access_log(&Method::GET, false).await?;

    if let Some(opd_er_order_master_id) = params.opd_er_order_master_id {
        let response = medical_history::get_scan_history(opd_er_order_master_id, &ctx.api_state.db_pool, &ctx.api_state.kphis()).await?;

        Ok(Json(response))
    } else {
        Ok(Json(None))
    }
}

// opd-er-document-scan-save.php
// opd-er-document-scan-update.php
/// /api/opd-er/medical-history-scan
///
/// Tries to create/edit OPD-ER Scan History
#[utoipa::path(
    post,
    path = "/opd-er/medical-history-scan",
    request_body = ScanHistory,
    responses(DocVecU32<ExecuteResponse>),
)]
pub async fn post_opd_er_scan_history(ctx: RequestState, Json(payload): Json<ScanHistory>) -> Result<Json<(u32, Vec<ExecuteResponse>)>, AppError> {
    ctx.user_state.trace_req_by();
    ctx.authorize_and_access_log(&Method::POST, false).await?;

    let response = medical_history::post_scan_history(
        &payload,
        &ctx.user_state.user.doctorcode,
        &ctx.user_state.user.loginname,
        &ctx.api_state.db_pool,
        &ctx.api_state.kphis(),
        &ctx.api_state.kphis_log(),
    )
    .await?;

    Ok(Json(response))
}

// opd-er-set-ft-data.php
/// /api/opd-er/medical-history-ft
///
/// Get OPD-ER Set-FT History by PARAMS, return single OPD-ER Set-FT History or none
///
/// Require opd_er_order_master_id in PARAMS
#[utoipa::path(
    get,
    path = "/opd-er/medical-history-ft",
    responses(DocOpt<SetFtHistory>),
    params(OpdErMedicalHistoryParams),
)]
pub async fn get_opd_er_ft_history(Query(params): Query<OpdErMedicalHistoryParams>, ctx: RequestState) -> Result<Json<Option<SetFtHistory>>, AppError> {
    ctx.user_state.trace_req_by();
    ctx.authorize_and_access_log(&Method::GET, false).await?;

    if let Some(opd_er_order_master_id) = params.opd_er_order_master_id {
        let response = medical_history::get_ft_history(opd_er_order_master_id, &ctx.api_state.db_pool, &ctx.api_state.hosxp(), &ctx.api_state.kphis()).await?;

        Ok(Json(response))
    } else {
        Ok(Json(None))
    }
}

// opd-er-set-ft-save.php
// opd-er-set-ft-update.php
/// /api/opd-er/medical-history-ft
///
/// Tries to create/edit OPD-ER Set-FT History
#[utoipa::path(
    post,
    path = "/opd-er/medical-history-ft",
    request_body = SetFtHistory,
    responses(DocVecU32<ExecuteResponse>),
)]
pub async fn post_opd_er_ft_history(ctx: RequestState, Json(payload): Json<SetFtHistory>) -> Result<Json<(u32, Vec<ExecuteResponse>)>, AppError> {
    ctx.user_state.trace_req_by();
    ctx.authorize_and_access_log(&Method::POST, false).await?;

    let response = medical_history::post_ft_history(
        &payload,
        &ctx.user_state.user.doctorcode,
        &ctx.user_state.user.loginname,
        &ctx.api_state.db_pool,
        &ctx.api_state.kphis(),
        &ctx.api_state.kphis_log(),
    )
    .await?;

    Ok(Json(response))
}
