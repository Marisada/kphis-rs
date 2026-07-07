use axum::{Json, extract::Path, http::Method};

use kphis_api_core::{
    open_api::{DocOne, DocVecU32},
    state::RequestState,
};
use kphis_api_query::{ipd::index_monitor, transform::query::check_an_can_execute};
use kphis_model::{fetch::ExecuteResponse, index_monitor::IndexMonitor};
use kphis_util::error::AppError;

/// /api/ipd/index-monitor
///
/// Tries to create/edit IPD Index Monitor
/// - Payload's `visit_type` must has not-empty `an`
/// - Checking `an` is `pre-admit` by `an` length
/// - user MUST has doctorcode (or return 403)
#[utoipa::path(
    post,
    path = "/ipd/index-monitor",
    request_body = IndexMonitor,
    responses(DocVecU32<ExecuteResponse>),
)]
pub async fn post_ipd_index_monitor(ctx: RequestState, Json(payload): Json<IndexMonitor>) -> Result<Json<(u32, Vec<ExecuteResponse>)>, AppError> {
    ctx.user_state.trace_req_by();

    if let Some((an, is_pre_admit)) = payload.visit_type.an_and_is_pre_admit() {
        if an.is_empty() {
            return Err(AppError::app_400("PostIpdIndexMonitor"));
        } else {
            ctx.authorize_and_access_log(&Method::POST, is_pre_admit).await?;
            // check AN is valid (pre-admit was admited or admit was revoked)
            check_an_can_execute(an, ctx.api_state.hosxp_an_len(), &ctx.api_state.db_pool, &ctx.api_state.hosxp(), &ctx.api_state.kphis()).await?;
        }
    } else {
        return Err(AppError::app_400("PostIpdIndexMonitor"));
    }

    if let Some(doctorcode) = &ctx.user_state.user.doctorcode {
        let response = index_monitor::post_index_monitor(&payload, doctorcode, &ctx.user_state.user.loginname, &ctx.api_state.db_pool, &ctx.api_state.kphis_extra()).await?;

        Ok(Json(response))
    } else {
        Err(AppError::app_403("PostIpdIndexMonitor"))
    }
}

/// /api/ipd/index-monitor-id/{monitor_id}
///
/// Tries to delete IPD Index Monitor by ID
#[utoipa::path(
    delete,
    path = "/ipd/index-monitor-id/{monitor_id}",
    responses(DocOne<ExecuteResponse>),
    params(
        ("monitor_id" = u32, Path, description = "Monitor ID", example = "1"),
    ),
)]
pub async fn delete_ipd_index_monitor(Path(monitor_id): Path<u32>, ctx: RequestState) -> Result<Json<ExecuteResponse>, AppError> {
    ctx.user_state.trace_req_by();
    ctx.authorize_and_access_log(&Method::DELETE, false).await?;

    let response = index_monitor::delete_index_monitor(monitor_id, &ctx.api_state.db_pool, &ctx.api_state.kphis_extra()).await?;

    Ok(Json(response))
}
