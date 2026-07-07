use axum::{
    Json,
    extract::{Path, Query},
    http::Method,
};

use kphis_api_core::{
    open_api::{DocOne, DocVec},
    state::RequestState,
};
use kphis_api_query::lab;
use kphis_model::{
    fetch::ExecuteResponse,
    lab::{LabHead, LabHeadParams, LabItem, LabItemParams, LabWbcBand},
};
use kphis_util::error::AppError;

/// /api/lab/wbc-key-value/{key}/{value}
///
/// Get CBC's wbc and band by Key and Value, return list of CBC's wbc and band
#[utoipa::path(
    get,
    path = "/lab/wbc-key-value/{key}/{value}",
    responses(DocVec<LabWbcBand>),
    params(
        ("key" = String, Path, description = "Key is 'vn' or 'hn'", example = "hn"),
        ("value" = String, Path, description = "Value of Key to be search", example = "0001234"),
    ),
)]
pub async fn get_wbc_band(Path((key, value)): Path<(String, String)>, ctx: RequestState) -> Result<Json<Vec<LabWbcBand>>, AppError> {
    ctx.user_state.trace_req_by();
    ctx.authorize_and_access_log(&Method::GET, false).await?;

    let response = lab::get_wbc_band(
        &key,
        &value,
        ctx.api_state.app_config.hosxp_lab_wbc_code,
        ctx.api_state.app_config.hosxp_lab_band_code,
        &ctx.api_state.db_pool,
        &ctx.api_state.hosxp(),
    )
    .await?;

    Ok(Json(response))
}

/// /api/lab/head
///
/// Get Lab Head by PARAMS, return list of Lab Head
#[utoipa::path(
    get,
    path = "/lab/head",
    responses(DocVec<LabHead>),
    params(LabHeadParams),
)]
pub async fn get_lab_head(Query(params): Query<LabHeadParams>, ctx: RequestState) -> Result<Json<Vec<LabHead>>, AppError> {
    ctx.user_state.trace_req_by();
    ctx.authorize_and_access_log(&Method::GET, false).await?;

    let response = lab::get_lab_head(
        &params,
        &ctx.user_state.user.doctorcode,
        &ctx.user_state.user.groupname,
        &ctx.api_state.db_pool,
        &ctx.api_state.hosxp(),
        &ctx.api_state.kphis(),
    )
    .await?;

    Ok(Json(response))
}

/// /api/lab/item
///
/// Get Lab Item by PARAMS, return list of Lab Item
#[utoipa::path(
    get,
    path = "/lab/item",
    responses(DocVec<LabItem>),
    params(LabItemParams),
)]
pub async fn get_lab_item(Query(params): Query<LabItemParams>, ctx: RequestState) -> Result<Json<Vec<LabItem>>, AppError> {
    ctx.user_state.trace_req_by();
    ctx.authorize_and_access_log(&Method::GET, false).await?;

    let response = lab::get_lab_item(&params, &ctx.user_state.user.doctorcode, &ctx.user_state.user.groupname, &ctx.api_state.db_pool, &ctx.api_state.hosxp()).await?;

    Ok(Json(response))
}

/// /api/lab/read-id/{lab_order_number}
///
/// Tries to mark Lab Head as readed
#[utoipa::path(
    post,
    path = "/lab/read-id/{lab_order_number}",
    responses(DocOne<ExecuteResponse>),
    params(
        ("lab_order_number" = i32, Path, description = "Lab Order Number", example = "1")
    ),
)]
pub async fn post_lab_read(Path(lab_order_number): Path<i32>, ctx: RequestState) -> Result<Json<ExecuteResponse>, AppError> {
    ctx.user_state.trace_req_by();
    ctx.authorize_and_access_log(&Method::POST, false).await?;

    let result = lab::post_lab_read(lab_order_number, &ctx.user_state.user.loginname, &ctx.api_state.db_pool, &ctx.api_state.kphis()).await?;

    Ok(Json(result))
}

/// /api/lab/read-id/{lab_order_number}
///
/// Tries to remove mark of Lab Head as readed
#[utoipa::path(
    delete,
    path = "/lab/read-id/{lab_order_number}",
    responses(DocOne<ExecuteResponse>),
    params(
        ("lab_order_number" = i32, Path, description = "Lab Order Number", example = "1")
    ),
)]
pub async fn delete_lab_read(Path(lab_order_number): Path<i32>, ctx: RequestState) -> Result<Json<ExecuteResponse>, AppError> {
    ctx.user_state.trace_req_by();
    ctx.authorize_and_access_log(&Method::DELETE, false).await?;

    let result = lab::delete_lab_read(lab_order_number, &ctx.api_state.db_pool, &ctx.api_state.kphis()).await?;

    Ok(Json(result))
}
