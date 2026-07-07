use axum::{
    Json,
    extract::{Path, Query},
    http::Method,
};

use kphis_api_core::{
    open_api::{DocOne, DocVec},
    state::RequestState,
};
use kphis_api_query::opd_er::vital_sign;
use kphis_model::{
    fetch::ExecuteResponse,
    vital_sign::{VitalSign, VitalSignParams, VitalSignSave},
};
use kphis_util::error::AppError;

// opd_er-vital-sign-show-chart-table-data.php
/// /api/opd-er/vital-sign
///
/// Get OPD-ER Vital Sign by PARAMS, return list of OPD-ER Vital Sign
#[utoipa::path(
    get,
    path = "/opd-er/vital-sign",
    responses(DocVec<VitalSign>),
    params(VitalSignParams),
)]
pub async fn get_opd_er_vital_sign(Query(params): Query<VitalSignParams>, ctx: RequestState) -> Result<Json<Vec<VitalSign>>, AppError> {
    ctx.user_state.trace_req_by();
    ctx.authorize_and_access_log(&Method::GET, false).await?;

    let response = vital_sign::get_vital_sign(&params, &ctx.api_state.db_pool, &ctx.api_state.hosxp(), &ctx.api_state.kphis()).await?;

    Ok(Json(response))
}

// opd-er-vital-sign-save.php
/// /api/opd-er/vital-sign
///
/// Tries to create a new OPD-ER Vital Sign
/// - Query parameter `med_reconciliation_id` must not null
#[utoipa::path(
    post,
    path = "/opd-er/vital-sign",
    params(VitalSignParams),
    request_body = VitalSignSave,
    responses(DocOne<ExecuteResponse>),
)]
pub async fn post_opd_er_vital_sign(Query(params): Query<VitalSignParams>, ctx: RequestState, Json(payload): Json<VitalSignSave>) -> Result<Json<ExecuteResponse>, AppError> {
    ctx.user_state.trace_req_by();
    ctx.authorize_and_access_log(&Method::POST, false).await?;

    if let Some(opd_er_order_master_id) = params.opd_er_order_master_id {
        let note_result = vital_sign::post_vital_sign(opd_er_order_master_id, &payload, &ctx.user_state.user.loginname, &ctx.api_state.db_pool, &ctx.api_state.kphis()).await?;

        Ok(Json(ExecuteResponse::from_query_result(note_result, "Post VitalSign")))
    } else {
        Err(AppError::app_400("Post VitalSign"))
    }
}

// opd-er-vital-sign-save.php
/// /api/opd-er/vital-sign
///
/// Tries to edit OPD-ER Vital Sign
/// - Query parameter `med_reconciliation_id` must not null
#[utoipa::path(
    put,
    path = "/opd-er/vital-sign",
    params(VitalSignParams),
    request_body = VitalSignSave,
    responses(DocOne<ExecuteResponse>),
)]
pub async fn put_opd_er_vital_sign(Query(params): Query<VitalSignParams>, ctx: RequestState, Json(payload): Json<VitalSignSave>) -> Result<Json<ExecuteResponse>, AppError> {
    ctx.user_state.trace_req_by();
    ctx.authorize_and_access_log(&Method::PUT, false).await?;

    if let Some(opd_er_order_master_id) = params.opd_er_order_master_id {
        let note_result = vital_sign::put_vital_sign(opd_er_order_master_id, &payload, &ctx.user_state.user.loginname, &ctx.api_state.db_pool, &ctx.api_state.kphis()).await?;

        Ok(Json(ExecuteResponse::from_query_result(note_result, "Put VitalSign")))
    } else {
        Err(AppError::app_400("Put VitalSign"))
    }
}

// opd-er-vital-sign-save.php
/// /api/opd-er/vital-sign-id/{vs_id}
///
/// Tries to delete OPD-ER Vital Sign by ID
#[utoipa::path(
    delete,
    path = "/opd-er/vital-sign-id/{vs_id}",
    responses(DocOne<ExecuteResponse>),
    params(
        ("vs_id" = u32, Path, description = "Vital Sign ID", example = "1"),
    ),
)]
pub async fn delete_opd_er_vital_sign(Path(vs_id): Path<u32>, ctx: RequestState) -> Result<Json<ExecuteResponse>, AppError> {
    ctx.user_state.trace_req_by();
    ctx.authorize_and_access_log(&Method::DELETE, false).await?;

    let response = vital_sign::delete_vital_sign(vs_id, &ctx.api_state.db_pool, &ctx.api_state.kphis()).await?;

    Ok(Json(response))
}
