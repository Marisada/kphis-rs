use axum::{
    Json,
    extract::{Path, Query},
    http::Method,
};

use kphis_api_core::{
    open_api::{DocVec, DocVecU32},
    state::RequestState,
};
use kphis_api_query::{ipd::io, transform::query::check_an_opt_can_execute};
use kphis_model::{
    fetch::ExecuteResponse,
    ipd::io::{IoDate, IoParams, IoShift},
};
use kphis_util::error::AppError;

// ipd-vital-sign-io-select-date.php
/// /api/ipd/io-date-an/{an}
///
/// Get list of IPD IO Date by AN, return list of IPD IO Date
#[utoipa::path(
    get,
    path = "/ipd/io-date-an/{an}",
    responses(DocVec<IoDate>),
    params(
        ("an" = String, Path, description = "Admission Number: AN", example = "660001234"),
    ),
)]
pub async fn get_ipd_io_date(Path(an): Path<String>, ctx: RequestState) -> Result<Json<Vec<IoDate>>, AppError> {
    ctx.user_state.trace_req_by();
    let is_pre_admit = ctx.api_state.is_pre_admit(&an);
    ctx.authorize_and_access_log(&Method::GET, is_pre_admit).await?;

    let dates = io::get_io_date(&an, &ctx.api_state.db_pool, &ctx.api_state.kphis()).await?;

    Ok(Json(dates))
}

// ipd-vital-sign-io-table.php
/// /api/ipd/io
///
/// Get list of IPD IO by PARAMS, return list of IPD IO
///
/// Require AN in PARAMS
#[utoipa::path(
    get,
    path = "/ipd/io",
    responses(DocVec<IoShift>),
    params(IoParams),
)]
pub async fn get_ipd_io_shift(Query(params): Query<IoParams>, ctx: RequestState) -> Result<Json<Vec<IoShift>>, AppError> {
    ctx.user_state.trace_req_by();
    let is_pre_admit = ctx.api_state.is_pre_admit_opt(&params.an);
    ctx.authorize_and_access_log(&Method::GET, is_pre_admit).await?;

    // if let (Some(an), Some(date)) = (params.an, params.date) {
    if params.an.as_ref().map(|s| !s.is_empty()).unwrap_or_default() {
        let ios = io::get_io_shift(
            &params,
            ctx.api_state.app_config.shift_day_start,
            ctx.api_state.app_config.shift_evening_start,
            ctx.api_state.app_config.shift_night_start,
            &ctx.api_state.db_pool,
            &ctx.api_state.hosxp(),
            &ctx.api_state.kphis(),
        )
        .await?;

        Ok(Json(ios))
    } else {
        Ok(Json(Vec::new()))
    }
}

// ipd-vital-sign-io-save.php
// ipd-vital-sign-io-update.php
/// /api/ipd/io
///
/// Tries to create/edit IPD IO
/// - Payload's `an` must not null or empty
/// - Checking `an` is `pre-admit` by `an` length
#[utoipa::path(
    post,
    path = "/ipd/io",
    request_body = IoShift,
    responses(DocVecU32<ExecuteResponse>),
)]
pub async fn post_ipd_io_shift(ctx: RequestState, Json(payload): Json<IoShift>) -> Result<Json<(u32, Vec<ExecuteResponse>)>, AppError> {
    ctx.user_state.trace_req_by();
    let is_pre_admit = ctx.api_state.is_pre_admit_opt(&payload.an);
    ctx.authorize_and_access_log(&Method::POST, is_pre_admit).await?;

    // check AN is valid (pre-admit was admited or admit was revoked)
    check_an_opt_can_execute(&payload.an, ctx.api_state.hosxp_an_len(), &ctx.api_state.db_pool, &ctx.api_state.hosxp(), &ctx.api_state.kphis()).await?;

    if payload.an.is_some() {
        let response = io::post_io_shift(&payload, &ctx.user_state.user.loginname, &ctx.api_state.db_pool, &ctx.api_state.kphis(), &ctx.api_state.kphis_log()).await?;

        Ok(Json(response))
    } else {
        Err(AppError::app_400("Post IoShift"))
    }
}

// ipd-vital-sign-io-delete.php
/// /api/ipd/io
///
/// Tries to delete IPD IO by PARAMS
/// - Query parameter `io_id` and `version` must not null
#[utoipa::path(
    delete,
    path = "/ipd/io",
    responses(DocVec<ExecuteResponse>),
    params(IoParams),
)]
pub async fn delete_ipd_io_shift(Query(params): Query<IoParams>, ctx: RequestState) -> Result<Json<Vec<ExecuteResponse>>, AppError> {
    ctx.user_state.trace_req_by();
    let is_pre_admit = ctx.api_state.is_pre_admit_opt(&params.an);
    ctx.authorize_and_access_log(&Method::DELETE, is_pre_admit).await?;

    if let (Some(io_id), Some(version)) = (params.io_id, params.version) {
        let response = io::delete_io_shift(
            io_id,
            version,
            &ctx.user_state.user.loginname,
            &ctx.api_state.db_pool,
            &ctx.api_state.kphis(),
            &ctx.api_state.kphis_log(),
        )
        .await?;

        Ok(Json(response))
    } else {
        Err(AppError::app_400("Delete IoShift"))
    }
}
