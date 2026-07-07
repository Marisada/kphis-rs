use axum::{Json, http::Method};

use kphis_api_core::{open_api::DocOne, state::RequestState};
use kphis_api_query::user::config;
use kphis_model::{
    fetch::ExecuteResponse,
    user::config::{UserConfig, UserConfigCommand, UserConfigResponse},
};
use kphis_util::error::AppError;

/// /api/user-config
///
/// Tries to create/edit User Config
#[utoipa::path(
    post,
    path = "/user-config",
    request_body = UserConfig,
    responses(DocOne<UserConfigResponse>),
)]
pub async fn post_user_config(ctx: RequestState, Json(payload): Json<UserConfig>) -> Result<Json<UserConfigResponse>, AppError> {
    ctx.user_state.trace_req_by();
    ctx.authorize_and_access_log(&Method::POST, false).await?;

    let (response, pk) = config::insert_dup_user_config(&payload, &ctx.user_state.user.loginname, &ctx.api_state.db_pool, &ctx.api_state.kphis_extra()).await?;
    // we update online_user here because `GET /api/user` getting user from state's online_user, not from database
    // read more at kphis-api-handler::user::token::refresh_token
    ctx.api_state.online_update_user_config(ctx.user_state.state_id, &payload.theme, &payload.wide_screen, &pk).await;

    Ok(Json(response))
}

/// /api/user-config
///
/// Tries to modify user config with command
#[utoipa::path(
    patch,
    path = "/user-config",
    request_body = UserConfigCommand,
    responses(DocOne<ExecuteResponse>),
)]
pub async fn patch_user_config(ctx: RequestState, Json(payload): Json<UserConfigCommand>) -> Result<Json<ExecuteResponse>, AppError> {
    ctx.user_state.trace_req_by();
    ctx.authorize_and_access_log(&Method::PATCH, false).await?;

    let response = match payload {
        UserConfigCommand::Clear2fa(target_loginname) => config::remove_totp(&target_loginname, &ctx.user_state.user.loginname, &ctx.api_state.db_pool, &ctx.api_state.kphis_extra()).await?,
    };

    Ok(Json(response))
}
