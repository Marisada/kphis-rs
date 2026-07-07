use axum::{
    Json,
    extract::{Path, Query},
    http::Method,
};

use kphis_api_core::{
    open_api::{DocVec, DocVecU32},
    state::RequestState,
};
use kphis_api_query::opd_er::io;
use kphis_model::{
    fetch::ExecuteResponse,
    ipd::io::{IoDate, IoParams, IoShift},
};
use kphis_util::{error::AppError, util::zero_none};

/// /api/opd-er/io-date-id/{opd_er_order_master_id}
///
/// Get list of OPD-ER IO Date by ID, return list of OPD-ER IO Date
#[utoipa::path(
    get,
    path = "/opd-er/io-date-id/{opd_er_order_master_id}",
    responses(DocVec<IoDate>),
    params(
        ("opd_er_order_master_id" = u32, Path, description = "OPD-ER Order Master ID", example = "1"),
    ),
)]
pub async fn get_opd_er_io_date(Path(opd_er_order_master_id): Path<u32>, ctx: RequestState) -> Result<Json<Vec<IoDate>>, AppError> {
    ctx.user_state.trace_req_by();
    ctx.authorize_and_access_log(&Method::GET, false).await?;

    let dates = io::get_io_date(opd_er_order_master_id, &ctx.api_state.db_pool, &ctx.api_state.kphis()).await?;

    Ok(Json(dates))
}

// opd-er-vital-sign-io-table.php
/// /api/opd-er/io
///
/// Get list of OPD-ER IO by PARAMS, return list of OPD-ER IO
///
/// Require opd_er_order_master_id in PARAMS
#[utoipa::path(
    get,
    path = "/opd-er/io",
    responses(DocVec<IoShift>),
    params(IoParams),
)]
pub async fn get_opd_er_io_shift(Query(params): Query<IoParams>, ctx: RequestState) -> Result<Json<Vec<IoShift>>, AppError> {
    ctx.user_state.trace_req_by();
    ctx.authorize_and_access_log(&Method::GET, false).await?;

    // if let (Some(opd_er_order_master_id), Some(date)) = (params.opd_er_order_master_id, params.date) {
    if params.opd_er_order_master_id.is_some() {
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

// iod-er-vital-sign-io-save.php
// opd-er-vital-sign-io-update.php
/// /api/opd-er/io
///
/// Tries to create/edit OPD-ER IO
/// - Payload's `opd_er_order_master_id` must not null or 0
#[utoipa::path(
    post,
    path = "/opd-er/io",
    request_body = IoShift,
    responses(DocVecU32<ExecuteResponse>),
)]
pub async fn post_opd_er_io_shift(ctx: RequestState, Json(payload): Json<IoShift>) -> Result<Json<(u32, Vec<ExecuteResponse>)>, AppError> {
    ctx.user_state.trace_req_by();
    ctx.authorize_and_access_log(&Method::POST, false).await?;

    if payload.opd_er_order_master_id.and_then(zero_none).is_some() {
        let response = io::post_io_shift(&payload, &ctx.user_state.user.loginname, &ctx.api_state.db_pool, &ctx.api_state.kphis(), &ctx.api_state.kphis_log()).await?;

        Ok(Json(response))
    } else {
        Err(AppError::app_400("Post IoShift"))
    }
}

// opd-er-vital-sign-io-delete.php
/// /api/opd-er/io
///
/// Tries to delete OPD-ER IO by PARAMS
/// - Query parameter `io_id` and `version` must not null
#[utoipa::path(
    delete,
    path = "/opd-er/io",
    responses(DocVec<ExecuteResponse>),
    params(IoParams),
)]
pub async fn delete_opd_er_io_shift(Query(params): Query<IoParams>, ctx: RequestState) -> Result<Json<Vec<ExecuteResponse>>, AppError> {
    ctx.user_state.trace_req_by();
    ctx.authorize_and_access_log(&Method::DELETE, false).await?;

    if let (Some(opd_er_io_id), Some(version)) = (params.io_id, params.version) {
        let response = io::delete_io_shift(
            opd_er_io_id,
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
