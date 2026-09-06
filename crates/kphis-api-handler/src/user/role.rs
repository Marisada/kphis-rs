use axum::{Json, extract::Query};

use kphis_api_core::{
    open_api::{DocOne, DocVec},
    state::RequestState,
};
use kphis_api_query::user::role;
use kphis_model::{
    fetch::ExecuteResponse,
    user::role::{RolePermissionList, RolePermissionSave, UserRoleList, UserRoleOptions, UserRoleParams, UserRoleSave},
};
use kphis_util::error::AppError;

// system-ac-role-user-list-data.php
/// /api/user-role/user
///
/// Get User Role for List by PARAMS, return list of User Role for List
#[utoipa::path(
    get,
    path = "/user-role/user",
    responses(DocOne<UserRoleList>),
    params(UserRoleParams),
)]
pub async fn get_user_role_list(Query(params): Query<UserRoleParams>, ctx: RequestState) -> Result<Json<UserRoleList>, AppError> {
    ctx.authorize(false).await?;

    let response = role::get_user_role_list(params, &ctx.api_state.db_pool, &ctx.api_state.hosxp(), &ctx.api_state.kphis(), &ctx.api_state.kphis_extra()).await?;

    Ok(Json(response))
}

/// /api/user-role/prelude
///
/// Get User Role start assets, return list of User Role start assets
#[utoipa::path(
    get,
    path = "/user-role/prelude",
    responses(DocOne<UserRoleOptions>),
)]
pub async fn get_user_role_prelude(ctx: RequestState) -> Result<Json<UserRoleOptions>, AppError> {
    ctx.authorize(false).await?;

    let response = role::get_user_role_prelude(&ctx.api_state.db_pool, &ctx.api_state.hosxp(), &ctx.api_state.kphis()).await?;

    Ok(Json(response))
}

/// /api/user-role/user
///
/// Tries to create/edit User Role
#[utoipa::path(
    post,
    path = "/user-role/user",
    request_body = UserRoleSave,
    responses(DocVec<ExecuteResponse>),
)]
pub async fn post_user_role(ctx: RequestState, Json(payload): Json<UserRoleSave>) -> Result<Json<Vec<ExecuteResponse>>, AppError> {
    ctx.authorize(false).await?;

    let loginname = payload.loginname.clone();
    let mut responses = role::post_user_role(payload, &ctx.user_state.user.loginname, &ctx.api_state.db_pool, &ctx.api_state.kphis()).await?;

    let rows_affected = ctx.api_state.online_remove_by_loginname(&loginname, "มีการเปลี่ยนแปลงสิทธิ์ กรุณาเข้าสู่ระบบใหม่").await;
    responses.push(ExecuteResponse {
        last_insert_id: 0,
        rows_affected,
        error: None,
        action: Some(String::from("LogOut")),
    });

    Ok(Json(responses))
}

/// /api/user-role/role
///
/// Get User Role-Permission, return list of User Role-Permission
#[utoipa::path(
    get,
    path = "/user-role/role",
    responses(DocVec<RolePermissionList>),
    params(UserRoleParams),
)]
pub async fn get_role_permission_list(Query(params): Query<UserRoleParams>, ctx: RequestState) -> Result<Json<Vec<RolePermissionList>>, AppError> {
    ctx.authorize(false).await?;

    let response = role::get_role_permission_list(params, &ctx.api_state.db_pool, &ctx.api_state.kphis()).await?;

    Ok(Json(response))
}

/// /api/user-role/role
///
/// Tries to create/edit User Role-Permission
#[utoipa::path(
    post,
    path = "/user-role/role",
    request_body = RolePermissionSave,
    responses(DocVec<ExecuteResponse>),
)]
pub async fn post_role_permission(ctx: RequestState, Json(payload): Json<RolePermissionSave>) -> Result<Json<Vec<ExecuteResponse>>, AppError> {
    ctx.authorize(false).await?;

    let role_prev_opt = payload.role_prev.as_ref().map(|r| r.to_owned());
    let mut responses = role::post_role_permission(payload, &ctx.user_state.user.loginname, &ctx.api_state.db_pool, &ctx.api_state.kphis()).await?;

    ctx.api_state.update_role_permission().await?;

    if let Some(role) = role_prev_opt {
        let rows_affected = ctx.api_state.online_remove_by_role(&role, "มีการเปลี่ยนแปลงสิทธิ์ กรุณาเข้าสู่ระบบใหม่").await;
        responses.push(ExecuteResponse {
            last_insert_id: 0,
            rows_affected,
            error: None,
            action: Some(String::from("LogOut")),
        });
    }

    Ok(Json(responses))
}

/// /api/user-role/role
///
/// Tries to delete User Role-Permission by PARAMS
#[utoipa::path(
    delete,
    path = "/user-role/role",
    responses(DocVec<ExecuteResponse>),
    params(UserRoleParams),
)]
pub async fn delete_role_permission(Query(params): Query<UserRoleParams>, ctx: RequestState) -> Result<Json<Vec<ExecuteResponse>>, AppError> {
    ctx.authorize(false).await?;

    let role_opt = params.role.as_ref().map(|r| r.to_owned());
    let mut responses = role::delete_role_permission(params, &ctx.user_state.user.loginname, &ctx.api_state.db_pool, &ctx.api_state.kphis()).await?;

    if let Some(role) = role_opt {
        ctx.api_state.update_role_permission().await?;
        let rows_affected = ctx.api_state.online_remove_by_role(&role, "มีการเปลี่ยนแปลงสิทธิ์ กรุณาเข้าสู่ระบบใหม่").await;
        responses.push(ExecuteResponse {
            last_insert_id: 0,
            rows_affected,
            error: None,
            action: Some(String::from("LogOut")),
        });
    }

    Ok(Json(responses))
}
