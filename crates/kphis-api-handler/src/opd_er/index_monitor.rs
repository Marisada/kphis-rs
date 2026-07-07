use axum::{Json, extract::Path, http::Method};

use kphis_api_core::{
    open_api::{DocOne, DocVecU32},
    state::RequestState,
};
use kphis_api_query::opd_er::index_monitor;
use kphis_model::{fetch::ExecuteResponse, index_monitor::IndexMonitor};
use kphis_util::error::AppError;

/// /api/opd-er/index-monitor
///
/// Tries to create/edit OPD-ER Index Monitor
/// - user MUST has doctorcode (or return 403)
#[utoipa::path(
    post,
    path = "/opd-er/index-monitor",
    request_body = IndexMonitor,
    responses(DocVecU32<ExecuteResponse>),
)]
pub async fn post_opd_er_index_monitor(ctx: RequestState, Json(payload): Json<IndexMonitor>) -> Result<Json<(u32, Vec<ExecuteResponse>)>, AppError> {
    ctx.user_state.trace_req_by();
    ctx.authorize_and_access_log(&Method::POST, false).await?;

    if let Some(doctorcode) = &ctx.user_state.user.doctorcode {
        let response = index_monitor::post_index_monitor(&payload, doctorcode, &ctx.user_state.user.loginname, &ctx.api_state.db_pool, &ctx.api_state.kphis_extra()).await?;

        Ok(Json(response))
    } else {
        Err(AppError::app_403("PostIpdIndexMonitor"))
    }
}

/// /api/opd-er/index-monitor-id/{monitor_id}
///
/// Tries to delete OPD-ER Index Monitor by ID
#[utoipa::path(
    delete,
    path = "/opd-er/index-monitor-id/{monitor_id}",
    responses(DocOne<ExecuteResponse>),
    params(
        ("monitor_id" = u32, Path, description = "Monitor ID", example = "1"),
    ),
)]
pub async fn delete_opd_er_index_monitor(Path(monitor_id): Path<u32>, ctx: RequestState) -> Result<Json<ExecuteResponse>, AppError> {
    ctx.user_state.trace_req_by();
    ctx.authorize_and_access_log(&Method::DELETE, false).await?;

    let response = index_monitor::delete_index_monitor(monitor_id, &ctx.api_state.db_pool, &ctx.api_state.kphis_extra()).await?;

    Ok(Json(response))
}
