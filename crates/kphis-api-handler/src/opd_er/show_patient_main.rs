use axum::{Json, extract::Path, http::Method};

use kphis_api_core::{open_api::DocOpt, state::RequestState};
use kphis_api_query::opd_er::show_patient_main;
use kphis_model::patient_info::PatientInfo;
use kphis_util::error::AppError;

// ope-er-show-patient-main.php
/// /api/opd-er/show-patient-main-id/{opd_er_order_master_id}
///
/// Get OPD-ER Patient Header by ID, return single OPD-ER Patient Information
#[utoipa::path(
    get,
    path = "/opd-er/show-patient-main-id/{opd_er_order_master_id}",
    responses(DocOpt<PatientInfo>),
    params(
        ("opd_er_order_master_id" = u32, Path, description = "OPD-ER Order Master ID", example = "1"),
    ),
)]
pub async fn get_opd_er_show_patient_main_id(Path(opd_er_order_master_id): Path<u32>, ctx: RequestState) -> Result<Json<Option<PatientInfo>>, AppError> {
    ctx.user_state.trace_req_by();
    ctx.authorize_and_access_log(&Method::GET, false).await?;

    let response = show_patient_main::get_show_patient_main_id(opd_er_order_master_id, &ctx.api_state.db_pool, &ctx.api_state.hosxp(), &ctx.api_state.kphis()).await?;

    Ok(Json(response))
}

/// /api/opd-er/show-patient-main-vn/{vn}
///
/// Get IPD Patient Header by AN, return single IPD Patient Information
#[utoipa::path(
    get,
    path = "/opd-er/show-patient-main-vn/{vn}",
    responses(DocOpt<PatientInfo>),
    params(
        ("vn" = String, Path, description = "Visit Number: VN", example = "661231235959"),
    ),
)]
pub async fn get_opd_er_show_patient_main_vn(Path(vn): Path<String>, ctx: RequestState) -> Result<Json<Option<PatientInfo>>, AppError> {
    ctx.user_state.trace_req_by();
    ctx.authorize_and_access_log(&Method::GET, false).await?;

    let response = show_patient_main::get_show_patient_main_vn(&vn, &ctx.api_state.db_pool, &ctx.api_state.hosxp(), &ctx.api_state.kphis()).await?;

    Ok(Json(response))
}
