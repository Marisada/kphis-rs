use axum::{
    Json,
    extract::{Path, Query},
    http::Method,
};

use kphis_api_core::{
    open_api::{DocOne, DocVec},
    state::RequestState,
};
use kphis_api_query::{ipd::vital_sign, transform::query::check_an_can_execute};
use kphis_model::{
    fetch::ExecuteResponse,
    vital_sign::{VitalSign, VitalSignParams, VitalSignSave},
};
use kphis_util::error::AppError;

// ipd-vital-sign-show-chart-table-data.php
/// /api/ipd/vital-sign
///
/// Get IPD Vital Sign by PARAMS, return list of IPD Vital Sign
#[utoipa::path(
    get,
    path = "/ipd/vital-sign",
    responses(DocVec<VitalSign>),
    params(VitalSignParams),
)]
pub async fn get_ipd_vital_sign(Query(params): Query<VitalSignParams>, ctx: RequestState) -> Result<Json<Vec<VitalSign>>, AppError> {
    ctx.user_state.trace_req_by();
    let is_pre_admit = ctx.api_state.is_pre_admit_opt(&params.an);
    ctx.authorize_and_access_log(&Method::GET, is_pre_admit).await?;

    let response = vital_sign::get_vital_sign(&params, &ctx.api_state.db_pool, &ctx.api_state.hosxp(), &ctx.api_state.kphis()).await?;

    Ok(Json(response))
}

// ipd-vital-sign-save.php
/// /api/ipd/vital-sign
///
/// Tries to create a new IPD Vital Sign
/// - Query parameters's `an` and `hn` must not null or empty
#[utoipa::path(
    post,
    path = "/ipd/vital-sign",
    params(VitalSignParams),
    request_body = VitalSignSave,
    responses(DocOne<ExecuteResponse>),
)]
pub async fn post_ipd_vital_sign(Query(params): Query<VitalSignParams>, ctx: RequestState, Json(payload): Json<VitalSignSave>) -> Result<Json<ExecuteResponse>, AppError> {
    ctx.user_state.trace_req_by();
    let is_pre_admit = ctx.api_state.is_pre_admit_opt(&params.an);
    ctx.authorize_and_access_log(&Method::POST, is_pre_admit).await?;

    if let (Some(an), Some(hn)) = (&params.an, &params.hn) {
        // check AN is valid (pre-admit was admited or admit was revoked)
        check_an_can_execute(an, ctx.api_state.hosxp_an_len(), &ctx.api_state.db_pool, &ctx.api_state.hosxp(), &ctx.api_state.kphis()).await?;

        let note_result = vital_sign::post_vital_sign(an, hn, &payload, &ctx.user_state.user.loginname, &ctx.api_state.db_pool, &ctx.api_state.kphis()).await?;

        Ok(Json(ExecuteResponse::from_query_result(note_result, "Post VitalSign")))
    } else {
        Err(AppError::app_400("Post VitalSign"))
    }
}

// ipd-vital-sign-save.php
/// /api/ipd/vital-sign
///
/// Tries to edit IPD Vital Sign
/// - Query parameters's `an` and `hn` must not null or empty
#[utoipa::path(
    put,
    path = "/ipd/vital-sign",
    params(VitalSignParams),
    request_body = VitalSignSave,
    responses(DocOne<ExecuteResponse>),
)]
pub async fn put_ipd_vital_sign(Query(params): Query<VitalSignParams>, ctx: RequestState, Json(payload): Json<VitalSignSave>) -> Result<Json<ExecuteResponse>, AppError> {
    ctx.user_state.trace_req_by();
    let is_pre_admit = ctx.api_state.is_pre_admit_opt(&params.an);
    ctx.authorize_and_access_log(&Method::PUT, is_pre_admit).await?;

    if let (Some(an), Some(hn)) = (&params.an, &params.hn) {
        // check AN is valid (pre-admit was admited or admit was revoked)
        check_an_can_execute(an, ctx.api_state.hosxp_an_len(), &ctx.api_state.db_pool, &ctx.api_state.hosxp(), &ctx.api_state.kphis()).await?;

        let note_result = vital_sign::put_vital_sign(an, hn, &payload, &ctx.user_state.user.loginname, &ctx.api_state.db_pool, &ctx.api_state.kphis()).await?;

        Ok(Json(ExecuteResponse::from_query_result(note_result, "Put VitalSign")))
    } else {
        Err(AppError::app_400("Put VitalSign"))
    }
}

// ipd-vital-sign-save.php
/// /api/ipd/vital-sign-id/{vs_id}
///
/// Tries to delete IPD Vital Sign by ID
#[utoipa::path(
    delete,
    path = "/ipd/vital-sign-id/{vs_id}",
    responses(DocOne<ExecuteResponse>),
    params(
        ("vs_id" = u32, Path, description = "Vital Sign ID", example = "1"),
    ),
)]
pub async fn delete_ipd_vital_sign(Path(vs_id): Path<u32>, ctx: RequestState) -> Result<Json<ExecuteResponse>, AppError> {
    ctx.user_state.trace_req_by();
    ctx.authorize_and_access_log(&Method::DELETE, false).await?;

    let response = vital_sign::delete_vital_sign(vs_id, &ctx.api_state.db_pool, &ctx.api_state.kphis()).await?;

    Ok(Json(response))
}
