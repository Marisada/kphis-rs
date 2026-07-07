use axum::{Json, extract::Path, http::Method};

use kphis_api_core::{
    open_api::{DocOne, DocVecU32},
    state::RequestState,
};
use kphis_api_query::opd_er::index_action;
use kphis_model::{fetch::ExecuteResponse, index_action::IndexAction};
use kphis_util::error::AppError;

// opd-er-nurse-index-plan-action-save.php
/// /api/opd-er/index-action
///
/// Tries to create/edit OPD-ER Index Action
#[utoipa::path(
    post,
    path = "/opd-er/index-action",
    request_body = IndexAction,
    responses(DocVecU32<ExecuteResponse>),
)]
pub async fn post_opd_er_index_action(ctx: RequestState, Json(payload): Json<IndexAction>) -> Result<Json<(u32, Vec<ExecuteResponse>)>, AppError> {
    ctx.user_state.trace_req_by();
    ctx.authorize_and_access_log(&Method::POST, false).await?;

    let response = index_action::post_index_action(&payload, &ctx.user_state.user.loginname, &ctx.api_state.db_pool, &ctx.api_state.kphis()).await?;

    Ok(Json(response))
}

// opd-er-nurse-index-plan-action-delete.php
/// /api/opd-er/index-action-id/{action_id}
///
/// Tries to delete OPD-ER Index Action by ID
#[utoipa::path(
    delete,
    path = "/opd-er/index-action-id/{action_id}",
    responses(DocOne<ExecuteResponse>),
    params(
        ("action_id" = u32, Path, description = "Action ID", example = "1"),
    ),
)]
pub async fn delete_opd_er_index_action(Path(action_id): Path<u32>, ctx: RequestState) -> Result<Json<ExecuteResponse>, AppError> {
    ctx.user_state.trace_req_by();
    ctx.authorize_and_access_log(&Method::DELETE, false).await?;

    let response = index_action::delete_index_action(action_id, &ctx.api_state.db_pool, &ctx.api_state.kphis()).await?;

    Ok(Json(response))
}
