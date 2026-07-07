use axum::{Json, extract::Path, http::Method};

use kphis_api_core::{
    open_api::{DocOne, DocVecU32},
    state::RequestState,
};
use kphis_api_query::{ipd::index_action, transform::query::check_an_can_execute};
use kphis_model::{fetch::ExecuteResponse, index_action::IndexAction};
use kphis_util::error::AppError;

// ipd-nurse-index-plan-action-save.php
/// /api/ipd/index-action
///
/// Tries to create/edit IPD Index Action
/// - Payload's `visit_type` must has not-empty `an`
/// - Checking `an` is `pre-admit` by `an` length
#[utoipa::path(
    post,
    path = "/ipd/index-action",
    request_body = IndexAction,
    responses(DocVecU32<ExecuteResponse>),
)]
pub async fn post_ipd_index_action(ctx: RequestState, Json(payload): Json<IndexAction>) -> Result<Json<(u32, Vec<ExecuteResponse>)>, AppError> {
    ctx.user_state.trace_req_by();

    if let Some((an, is_pre_admit)) = payload.visit_type.an_and_is_pre_admit() {
        if an.is_empty() {
            return Err(AppError::app_400("PostIpdIndexAction"));
        } else {
            ctx.authorize_and_access_log(&Method::POST, is_pre_admit).await?;
            // check AN is valid (pre-admit was admited or admit was revoked)
            check_an_can_execute(an, ctx.api_state.hosxp_an_len(), &ctx.api_state.db_pool, &ctx.api_state.hosxp(), &ctx.api_state.kphis()).await?;
        }
    } else {
        return Err(AppError::app_400("PostIpdIndexAction"));
    }

    let response = index_action::post_index_action(&payload, &ctx.user_state.user.loginname, &ctx.api_state.db_pool, &ctx.api_state.kphis()).await?;

    Ok(Json(response))
}

// ipd-nurse-index-plan-action-delete.php
/// /api/ipd/index-action-id/{action_id}
///
/// Tries to delete IPD Index Action by ID
#[utoipa::path(
    delete,
    path = "/ipd/index-action-id/{action_id}",
    responses(DocOne<ExecuteResponse>),
    params(
        ("action_id" = u32, Path, description = "Action ID", example = "1"),
    ),
)]
pub async fn delete_ipd_index_action(Path(action_id): Path<u32>, ctx: RequestState) -> Result<Json<ExecuteResponse>, AppError> {
    ctx.user_state.trace_req_by();
    ctx.authorize_and_access_log(&Method::DELETE, false).await?;

    let response = index_action::delete_index_action(action_id, &ctx.api_state.db_pool, &ctx.api_state.kphis()).await?;

    Ok(Json(response))
}
