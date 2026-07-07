use axum::{
    Json,
    extract::{Path, Query},
    http::Method,
};

use kphis_api_core::{
    open_api::{DocVec, DocVecU32},
    state::RequestState,
};
use kphis_api_query::{ipd::dc_plan, transform::query::check_an_can_execute};
use kphis_model::{
    dc_plan::{DischargePlan, DischargePlanParams, DischargePlanSave},
    fetch::ExecuteResponse,
};
use kphis_util::error::AppError;

/// /api/ipd/dc-plan-an/{an}
///
/// Get list of IPD Discharge Plan by AN, return list of IPD Discharge Plan
#[utoipa::path(
    get,
    path = "/ipd/dc-plan-an/{an}",
    responses(DocVec<DischargePlan>),
    params(
        ("an" = String, Path, description = "Admission Number: AN", example = "660001234"),
    ),
)]
pub async fn get_ipd_dc_plan(Path(an): Path<String>, ctx: RequestState) -> Result<Json<Vec<DischargePlan>>, AppError> {
    ctx.user_state.trace_req_by();
    let is_pre_admit = ctx.api_state.is_pre_admit(&an);
    ctx.authorize_and_access_log(&Method::GET, is_pre_admit).await?;

    let response = dc_plan::get_dc_plan(&an, &ctx.api_state.db_pool, &ctx.api_state.hosxp(), &ctx.api_state.kphis_extra()).await?;

    Ok(Json(response))
}

/// /api/ipd/dc-plan-an/{an}
///
/// Tries to create/edit IPD Discharge Plan
/// - Checking `an` is `pre-admit` by `an` length
#[utoipa::path(
    post,
    path = "/ipd/dc-plan-an/{an}",
    request_body = DischargePlanSave,
    responses(DocVecU32<ExecuteResponse>),
    params(
        ("an" = String, Path, description = "Admission Number: AN", example = "660001234"),
    ),
)]
pub async fn post_ipd_dc_plan(Path(an): Path<String>, ctx: RequestState, Json(payload): Json<DischargePlanSave>) -> Result<Json<(u32, Vec<ExecuteResponse>)>, AppError> {
    ctx.user_state.trace_req_by();
    let is_pre_admit = ctx.api_state.is_pre_admit(&an);
    ctx.authorize_and_access_log(&Method::POST, is_pre_admit).await?;

    // check AN is valid (pre-admit was admited or admit was revoked)
    check_an_can_execute(&an, ctx.api_state.hosxp_an_len(), &ctx.api_state.db_pool, &ctx.api_state.hosxp(), &ctx.api_state.kphis()).await?;

    let result = dc_plan::post_dc_plan(&an, &payload, &ctx.user_state.user.loginname, &ctx.api_state.db_pool, &ctx.api_state.kphis_extra()).await?;

    Ok(Json(result))
}

/// /api/ipd/dc-plan-an/{an}
///
/// Tries to delete IPD Discharge Plan by PARAMS
/// - Query parameter `dc_plan_id` and `version` must not null
#[utoipa::path(
    delete,
    path = "/ipd/dc-plan-an/{an}",
    responses(DocVec<ExecuteResponse>),
    params(
        ("an" = String, Path, description = "Admission Number: AN (Optional)", example = "660001234"),
        DischargePlanParams,
    ),
)]
pub async fn delete_ipd_dc_plan(Path(_an): Path<String>, Query(params): Query<DischargePlanParams>, ctx: RequestState) -> Result<Json<Vec<ExecuteResponse>>, AppError> {
    ctx.user_state.trace_req_by();
    ctx.authorize_and_access_log(&Method::DELETE, false).await?;

    if let (Some(dc_plan_id), Some(version)) = (params.dc_plan_id, params.version) {
        let result = dc_plan::delete_dc_plan(dc_plan_id, version, &ctx.api_state.db_pool, &ctx.api_state.kphis_extra()).await?;

        Ok(Json(result))
    } else {
        Err(AppError::app_400("Delete DcPlan"))
    }
}
