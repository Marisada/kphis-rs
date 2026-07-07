use axum::{
    Json,
    extract::{Path, Query},
    http::Method,
};

use kphis_api_core::{
    open_api::{DocOpt, DocVec, DocVecU32},
    state::RequestState,
};
use kphis_api_query::opd_er::order_master;
use kphis_model::{
    fetch::ExecuteResponse,
    opd_er::order_master::{OpdErOrderMaster, OpdErOrderMasterCheck, OpdErOrderMasterList, OpdErOrderMasterParams, OpdErOrderMasterSave},
};
use kphis_util::error::AppError;

// opd-er-order-master-check.php
/// /api/opd-er/order/master/check-vn/{vn}
///
/// Get list of OPD-ER Order Master Checker by VN, return list of OPD-ER Order Master Checker
#[utoipa::path(
    get,
    path = "/opd-er/order/master/check-vn/{vn}",
    responses(DocVec<OpdErOrderMasterCheck>),
    params(
        ("vn" = String, Path, description = "Visit Number: VN", example = "661231235959"),
    ),
)]
pub async fn get_opd_er_order_master_check(Path(vn): Path<String>, ctx: RequestState) -> Result<Json<Vec<OpdErOrderMasterCheck>>, AppError> {
    ctx.user_state.trace_req_by();
    ctx.authorize_and_access_log(&Method::GET, false).await?;

    let response = order_master::get_order_master_check(&vn, &ctx.api_state.db_pool, &ctx.api_state.kphis()).await?;

    Ok(Json(response))
}

// opd-er-order-list-data.php
/// /api/opd-er/order/master
///
/// Get list of OPD-ER Order Master for List by PARAMS, return a list of OPD-ER Order Master for List
#[utoipa::path(
    get,
    path = "/opd-er/order/master",
    responses(DocVec<OpdErOrderMasterList>),
    params(OpdErOrderMasterParams),
)]
pub async fn get_opd_er_order_master_list(Query(params): Query<OpdErOrderMasterParams>, ctx: RequestState) -> Result<Json<Vec<OpdErOrderMasterList>>, AppError> {
    ctx.user_state.trace_req_by();
    ctx.authorize_and_access_log(&Method::GET, false).await?;

    let response = order_master::get_order_master_list(
        &params,
        &ctx.api_state.app_config.doctor_intern_roles,
        &ctx.api_state.db_pool,
        &ctx.api_state.hosxp(),
        &ctx.api_state.kphis(),
    )
    .await?;

    Ok(Json(response))
}

// opd-er-order-master-data.php
/// /api/opd-er/order/master-id/{opd_er_order_master_id}
///
/// Get OPD-ER Order Master by ID, return single OPD-ER Order Master or none
#[utoipa::path(
    get,
    path = "/opd-er/order/master-id/{opd_er_order_master_id}",
    responses(DocOpt<OpdErOrderMaster>),
    params(
        ("opd_er_order_master_id" = u32, Path, description = "OPD-ER Order Master ID", example = "1"),
    ),
)]
pub async fn get_opd_er_order_master(Path(opd_er_order_master_id): Path<u32>, ctx: RequestState) -> Result<Json<Option<OpdErOrderMaster>>, AppError> {
    ctx.user_state.trace_req_by();
    ctx.authorize_and_access_log(&Method::GET, false).await?;

    let response = order_master::get_order_master(
        opd_er_order_master_id,
        &ctx.api_state.app_config.doctor_intern_roles,
        &ctx.api_state.db_pool,
        &ctx.api_state.hosxp(),
        &ctx.api_state.kphis(),
    )
    .await?;

    Ok(Json(response))
}

// opd-er-order-master-save.php
/// /api/opd-er/order/master
///
/// Tries to create/edit OPD-ER Order Master
#[utoipa::path(
    post,
    path = "/opd-er/order/master",
    request_body = OpdErOrderMasterSave,
    responses(DocVecU32<ExecuteResponse>),
)]
pub async fn post_opd_er_order_master(ctx: RequestState, Json(payload): Json<OpdErOrderMasterSave>) -> Result<Json<(u32, ExecuteResponse)>, AppError> {
    ctx.user_state.trace_req_by();
    ctx.authorize_and_access_log(&Method::POST, false).await?;

    let response = order_master::post_order_master(
        &payload,
        &ctx.user_state.user.loginname,
        &ctx.user_state.user.doctorcode,
        &ctx.api_state.db_pool,
        &ctx.api_state.kphis(),
    )
    .await?;

    Ok(Json(response))
}
