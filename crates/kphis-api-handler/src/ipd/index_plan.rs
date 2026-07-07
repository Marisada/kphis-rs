use axum::{Json, extract::Path, http::Method};

use kphis_api_core::{
    open_api::{DocOne, DocVec, DocVecU32},
    state::RequestState,
};
use kphis_api_query::{ipd::index_plan, transform::query::check_an_can_execute};
use kphis_model::{
    fetch::ExecuteResponse,
    index_plan::{IndexPlanDate, IndexPlanSave, IpdIndexMedPay},
};
use kphis_util::error::AppError;

/// /api/ipd/index-plan-date-an/{an}
///
/// Get list of IPD Index Plan Date by AN, return list of IPD Index Plan Date
#[utoipa::path(
    get,
    path = "/ipd/index-plan-date-an/{an}",
    responses(DocVec<IndexPlanDate>),
    params(
        ("an" = String, Path, description = "Admission Number: AN", example = "660001234"),
    ),
)]
pub async fn get_index_plan_date(Path(an): Path<String>, ctx: RequestState) -> Result<Json<Vec<IndexPlanDate>>, AppError> {
    ctx.user_state.trace_req_by();
    let is_pre_admit = ctx.api_state.is_pre_admit(&an);
    ctx.authorize_and_access_log(&Method::GET, is_pre_admit).await?;

    let response = index_plan::get_index_plan_date(&an, &ctx.api_state.db_pool, &ctx.api_state.kphis()).await?;

    Ok(Json(response))
}

// ipd-nurse-index-plan-action-save.php
/// /api/ipd/index-plan
///
/// Tries to create/edit IPD Index Plan
/// - Payload's `visit_type` must has not-empty `an`
/// - Checking `an` is `pre-admit` by `an` length
#[utoipa::path(
    post,
    path = "/ipd/index-plan",
    request_body = IndexPlanSave,
    responses(DocVecU32<ExecuteResponse>),
)]
pub async fn post_ipd_index_plan(ctx: RequestState, Json(payload): Json<IndexPlanSave>) -> Result<Json<(u32, Vec<ExecuteResponse>)>, AppError> {
    ctx.user_state.trace_req_by();

    if let Some((an, is_pre_admit)) = payload.visit_type.an_and_is_pre_admit() {
        if an.is_empty() {
            return Err(AppError::app_400("PostIpdIndexPlan"));
        } else {
            ctx.authorize_and_access_log(&Method::POST, is_pre_admit).await?;
            // check AN is valid (pre-admit was admited or admit was revoked)
            check_an_can_execute(an, ctx.api_state.hosxp_an_len(), &ctx.api_state.db_pool, &ctx.api_state.hosxp(), &ctx.api_state.kphis()).await?;
        }
    } else {
        return Err(AppError::app_400("PostIpdIndexPlan"));
    }

    let response = index_plan::post_index_plan(&payload, &ctx.user_state.user.loginname, &ctx.api_state.db_pool, &ctx.api_state.kphis()).await?;

    Ok(Json(response))
}

// ipd-nurse-index-plan-action-delete.php
/// /api/ipd/index-plan-id/{plan_id}
///
/// Tries to delete IPD Index Plan by ID
#[utoipa::path(
    delete,
    path = "/ipd/index-plan-id/{plan_id}",
    responses(DocOne<ExecuteResponse>),
    params(
        ("plan_id" = u32, Path, description = "Plan ID", example = "1"),
    ),
)]
pub async fn delete_ipd_index_plan(Path(plan_id): Path<u32>, ctx: RequestState) -> Result<Json<ExecuteResponse>, AppError> {
    ctx.user_state.trace_req_by();
    ctx.authorize_and_access_log(&Method::DELETE, false).await?;

    let response = index_plan::delete_index_plan(plan_id, &ctx.api_state.db_pool, &ctx.api_state.kphis()).await?;

    Ok(Json(response))
}

/// /api/ipd/index-med-pay-an/{an}
///
/// Get IPD Medical Order and Pay by AN, return a list of IPD Medical Order and Pay
#[utoipa::path(
    get,
    path = "/ipd/index-med-pay-an/{an}",
    responses(DocVec<IpdIndexMedPay>),
    params(
        ("an" = String, Path, description = "Admission Number: AN", example = "660001234"),
    ),
)]
pub async fn get_ipd_index_med_pay(Path(an): Path<String>, ctx: RequestState) -> Result<Json<Vec<IpdIndexMedPay>>, AppError> {
    ctx.user_state.trace_req_by();
    let is_pre_admit = ctx.api_state.is_pre_admit(&an);
    ctx.authorize_and_access_log(&Method::GET, is_pre_admit).await?;

    let response = index_plan::get_index_med_pay(&an, &ctx.api_state.db_pool, &ctx.api_state.hosxp()).await?;

    Ok(Json(response))
}
