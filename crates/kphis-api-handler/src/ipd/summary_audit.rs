use axum::{Json, extract::Query, http::Method};

use kphis_api_core::{
    open_api::{DocOne, DocVec},
    state::RequestState,
};
use kphis_api_query::ipd::summary_audit;
use kphis_model::{
    fetch::ExecuteResponse,
    ipd::summary_audit::{SummaryAudit, SummaryAuditParams},
};
use kphis_util::error::AppError;

/// /api/ipd/summary-audit
///
/// Get IPD Summary Audit by PARAMS, return list of IPD Summary Audit
#[utoipa::path(
    get,
    path = "/ipd/summary-audit",
    responses(DocVec<SummaryAudit>),
    params(SummaryAuditParams),
)]
pub async fn get_ipd_summary_audit(Query(params): Query<SummaryAuditParams>, ctx: RequestState) -> Result<Json<Vec<SummaryAudit>>, AppError> {
    ctx.user_state.trace_req_by();
    ctx.authorize_and_access_log(&Method::GET, false).await?;

    let response = summary_audit::get_ipd_summary_audit(&params, &ctx.api_state.db_pool, &ctx.api_state.hosxp(), &ctx.api_state.kphis(), &ctx.api_state.kphis_extra()).await?;

    Ok(Json(response))
}

/// /api/ipd/summary-audit
///
/// Tries to create/update a IPD Summary Audit
/// - Payload's `summary_id`, `sa` and `ca` must not empty
#[utoipa::path(
    post,
    path = "/ipd/summary-audit",
    request_body = SummaryAudit,
    responses(DocOne<ExecuteResponse>),
)]
pub async fn post_ipd_summary_audit(ctx: RequestState, Json(payload): Json<SummaryAudit>) -> Result<Json<(u32, Vec<ExecuteResponse>)>, AppError> {
    ctx.user_state.trace_req_by();
    ctx.authorize_and_access_log(&Method::POST, false).await?;

    let result = summary_audit::post_ipd_summary_audit(&payload, &ctx.user_state.user.loginname, &ctx.api_state.db_pool, &ctx.api_state.kphis_extra()).await?;

    Ok(Json(result))
}

/// /api/ipd/summary-audit
///
/// Tries to delete IPD Summary Audit
/// - Query parameter `summary_audit_id` must not null
#[utoipa::path(
    delete,
    path = "/ipd/summary-audit",
    responses(DocOne<ExecuteResponse>),
    params(SummaryAuditParams),
)]
pub async fn delete_ipd_summary_audit(Query(params): Query<SummaryAuditParams>, ctx: RequestState) -> Result<Json<ExecuteResponse>, AppError> {
    ctx.user_state.trace_req_by();
    ctx.authorize_and_access_log(&Method::DELETE, false).await?;

    if let Some(summary_audit_id) = params.summary_audit_id {
        let response = summary_audit::delete_summary_audit(summary_audit_id, &ctx.user_state.user.loginname, &ctx.api_state.db_pool, &ctx.api_state.kphis_extra())
            .await
            .map(|result| ExecuteResponse::from_query_result(result, "Delete IpdSummaryAudit"))?;

        Ok(Json(response))
    } else {
        Err(AppError::app_400("Delete IpdSummaryAudit"))
    }
}
