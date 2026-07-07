use axum::{
    Json,
    extract::{Path, Query},
    http::Method,
};

use kphis_api_core::{
    open_api::{DocVec, DocVecU32},
    state::RequestState,
};
use kphis_api_query::opd_er::dc_plan;
use kphis_model::{
    dc_plan::{DischargePlan, DischargePlanParams, DischargePlanSave},
    fetch::ExecuteResponse,
};
use kphis_util::error::AppError;

/// /api/opd-er/dc-plan-id/{opd_er_order_master_id}
///
/// Get list of OPD-ER Discharge Plan by opd_er_order_master_id, return list of OPD-ER Discharge Plan
#[utoipa::path(
    get,
    path = "/opd-er/dc-plan-id/{opd_er_order_master_id}",
    responses(DocVec<DischargePlan>),
    params(
        ("opd_er_order_master_id" = u32, Path, description = "OPD-ER Order Master ID", example = "1"),
    ),
)]
pub async fn get_opd_er_dc_plan(Path(opd_er_order_master_id): Path<u32>, ctx: RequestState) -> Result<Json<Vec<DischargePlan>>, AppError> {
    ctx.user_state.trace_req_by();
    ctx.authorize_and_access_log(&Method::GET, false).await?;

    let response = dc_plan::get_dc_plan(opd_er_order_master_id, &ctx.api_state.db_pool, &ctx.api_state.hosxp(), &ctx.api_state.kphis_extra()).await?;

    Ok(Json(response))
}

/// /api/opd-er/dc-plan-id/{opd_er_order_master_id}
///
/// Tries to create/edit OPD-ER Discharge Plan
#[utoipa::path(
    post,
    path = "/opd-er/dc-plan-id/{opd_er_order_master_id}",
    request_body = DischargePlanSave,
    responses(DocVecU32<ExecuteResponse>),
    params(
        ("opd_er_order_master_id" = u32, Path, description = "OPD-ER Order Master ID", example = "1"),
    ),
)]
pub async fn post_opd_er_dc_plan(Path(opd_er_order_master_id): Path<u32>, ctx: RequestState, Json(payload): Json<DischargePlanSave>) -> Result<Json<(u32, Vec<ExecuteResponse>)>, AppError> {
    ctx.user_state.trace_req_by();
    ctx.authorize_and_access_log(&Method::POST, false).await?;

    let result = dc_plan::post_dc_plan(opd_er_order_master_id, &payload, &ctx.user_state.user.loginname, &ctx.api_state.db_pool, &ctx.api_state.kphis_extra()).await?;

    Ok(Json(result))
}

/// /api/opd-er/dc-plan-id/{opd_er_order_master_id}
///
/// Tries to delete OPD-ER Discharge Plan by PARAMS
/// - Query parameter `dc_plan_id` and `version` must not null
#[utoipa::path(
    delete,
    path = "/opd-er/dc-plan-id/{opd_er_order_master_id}",
    responses(DocVec<ExecuteResponse>),
    params(
        ("opd_er_order_master_id" = u32, Path, description = "OPD-ER Order Master ID (Optional)", example = "1"),
        DischargePlanParams,
    ),
)]
pub async fn delete_opd_er_dc_plan(Path(_opd_er_order_master_id): Path<u32>, Query(params): Query<DischargePlanParams>, ctx: RequestState) -> Result<Json<Vec<ExecuteResponse>>, AppError> {
    ctx.user_state.trace_req_by();
    ctx.authorize_and_access_log(&Method::DELETE, false).await?;

    if let (Some(dc_plan_id), Some(version)) = (params.dc_plan_id, params.version) {
        let result = dc_plan::delete_dc_plan(dc_plan_id, version, &ctx.api_state.db_pool, &ctx.api_state.kphis_extra()).await?;

        Ok(Json(result))
    } else {
        Err(AppError::app_400("Delete DcPlan"))
    }
}
