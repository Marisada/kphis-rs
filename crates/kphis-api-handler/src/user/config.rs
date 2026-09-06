use axum::Json;

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
    ctx.authorize(false).await?;

    let response = config::insert_dup_user_config(&payload, &ctx.user_state.user.loginname, &ctx.api_state.db_pool, &ctx.api_state.kphis_extra()).await?;

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
    ctx.authorize(false).await?;

    let response = match payload {
        UserConfigCommand::Clear2fa(target_loginname) => config::remove_totp(&target_loginname, &ctx.user_state.user.loginname, &ctx.api_state.db_pool, &ctx.api_state.kphis_extra()).await?,
        UserConfigCommand::ClearFailed(target_loginname) => config::clear_failed(&target_loginname, &ctx.user_state.user.loginname, &ctx.api_state.db_pool, &ctx.api_state.kphis_extra()).await?,
    };

    Ok(Json(response))
}
