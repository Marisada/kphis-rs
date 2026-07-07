use axum::{Json, extract::Query, http::Method};

use kphis_api_core::{open_api::DocOne, state::RequestState};
use kphis_api_query::prescription;
use kphis_model::{
    fetch::ExecuteResponse,
    prescription::{PrescriptionScreen, PrescriptionScreenParams, PrescriptionScreenPatch},
};
use kphis_util::error::AppError;

// pharmacy-prescription-screen-data-select.php
/// /api/prescription/screen
///
/// Get Prescription Screening data by PARAMS, return single Prescription Screening data
#[utoipa::path(
    get,
    path = "/prescription/screen",
    responses(DocOne<PrescriptionScreen>),
    params(PrescriptionScreenParams),
)]
pub async fn get_prescription_screen(Query(params): Query<PrescriptionScreenParams>, ctx: RequestState) -> Result<Json<PrescriptionScreen>, AppError> {
    ctx.user_state.trace_req_by();
    ctx.authorize_and_access_log(&Method::GET, false).await?;

    let response = prescription::get_prescription_screen(
        params,
        ctx.api_state.hosxp_hn_len(),
        ctx.api_state.hosxp_vn_len(),
        &ctx.api_state.egfr_codes(),
        &ctx.api_state.scr_codes(),
        &ctx.api_state.lab_codes(),
        &ctx.api_state.message_icodes(),
        &ctx.api_state.message_egfr_icodes(),
        &ctx.api_state.message_crcl_icodes(),
        &ctx.api_state.db_pool,
        &ctx.api_state.hosxp(),
        &ctx.api_state.kphis_extra(),
    )
    .await?;

    Ok(Json(response))
}

/// /api/prescription/screen
///
/// Tries to create Prescription Screen with Accept action
/// - Query parameter `vn` must not null
#[utoipa::path(
    post,
    path = "/prescription/screen",
    responses(DocOne<ExecuteResponse>),
    params(PrescriptionScreenParams),
)]
pub async fn post_prescription_screen(Query(params): Query<PrescriptionScreenParams>, ctx: RequestState) -> Result<Json<ExecuteResponse>, AppError> {
    ctx.user_state.trace_req_by();
    ctx.authorize_and_access_log(&Method::POST, false).await?;

    if let Some(vn) = params.vn {
        if let Some(doctorcode) = ctx.user_state.user.doctorcode {
            let response = prescription::post_prescription_screen(&vn, &doctorcode, &ctx.user_state.user.loginname, &ctx.api_state.db_pool, &ctx.api_state.kphis_extra()).await?;

            Ok(Json(response))
        } else {
            Err(AppError::app_401("Post PrescriptionScreen"))
        }
    } else {
        Err(AppError::app_400("Post PrescriptionScreen"))
    }
}

/// /api/prescription/screen
///
/// Tries to update Prescription Screen with 'accept', 'check' or 'done'
/// - Query parameter `vn` must not null, `action` must be `check` or `done`
#[utoipa::path(
    patch,
    path = "/prescription/screen",
    responses(DocOne<ExecuteResponse>),
    params(PrescriptionScreenParams),
    request_body = PrescriptionScreenPatch,
)]
pub async fn patch_prescription_screen(Query(params): Query<PrescriptionScreenParams>, ctx: RequestState, Json(payload): Json<PrescriptionScreenPatch>) -> Result<Json<ExecuteResponse>, AppError> {
    ctx.user_state.trace_req_by();
    ctx.authorize_and_access_log(&Method::PATCH, false).await?;

    if let (Some(vn), Some(action)) = (params.vn, params.action) {
        if let Some(doctorcode) = ctx.user_state.user.doctorcode {
            let response = prescription::patch_prescription_screen(
                &vn,
                &action,
                &payload,
                &doctorcode,
                &ctx.user_state.user.loginname,
                &ctx.api_state.db_pool,
                &ctx.api_state.kphis_extra(),
            )
            .await?;

            Ok(Json(response))
        } else {
            Err(AppError::app_401("Patch PrescriptionScreen"))
        }
    } else {
        Err(AppError::app_400("Patch PrescriptionScreen"))
    }
}
