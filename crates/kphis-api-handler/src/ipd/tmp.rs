use axum::{Json, extract::Query, http::Method};

use kphis_api_core::{
    open_api::{DocOne, DocVec},
    state::RequestState,
};
use kphis_api_query::ipd::tmp;
use kphis_model::{
    fetch::ExecuteResponse,
    ipd::tmp::{TmpDlc, TmpFocus, TmpGoal, TmpGroup, TmpIntvt, TmpParams, TmpSubGroup},
};
use kphis_util::error::AppError;

// setting-template-nurse-note-smp-data.php
/// /api/ipd/tmp/group
///
/// Get IPD Nursing Template Group by PARAMS, return list of Nursing Template Group
#[utoipa::path(
    get,
    path = "/ipd/tmp/group",
    responses(DocVec<TmpGroup>),
    params(TmpParams),
)]
pub async fn get_ipd_tmp_group(Query(params): Query<TmpParams>, ctx: RequestState) -> Result<Json<Vec<TmpGroup>>, AppError> {
    ctx.user_state.trace_req_by();
    ctx.authorize_and_access_log(&Method::GET, false).await?;

    let response = tmp::get_group(&params, &ctx.api_state.db_pool, &ctx.api_state.kphis()).await?;

    Ok(Json(response))
}

// setting-template-nurse-note-smp-save.php
/// /api/ipd/tmp/group
///
/// Tries to create/edit Nursing Template Group
/// - Payload's `smp_name` must not null or empty
#[utoipa::path(
    post,
    path = "/ipd/tmp/group",
    request_body = TmpGroup,
    responses(DocOne<ExecuteResponse>),
)]
pub async fn post_ipd_tmp_group(ctx: RequestState, Json(payload): Json<TmpGroup>) -> Result<Json<ExecuteResponse>, AppError> {
    ctx.user_state.trace_req_by();
    ctx.authorize_and_access_log(&Method::POST, false).await?;

    if payload.smp_name.as_ref().map(|s| s.is_empty()).unwrap_or(true) {
        Err(AppError::app_400("Post IpdTmpGroup"))
    } else {
        let response = tmp::post_group(&payload, &ctx.user_state.user.loginname, &ctx.api_state.db_pool, &ctx.api_state.kphis()).await?;

        Ok(Json(response))
    }
}

// setting-template-nurse-note-smp-delete.php
/// /api/ipd/tmp/group
///
/// Tries to delete Nursing Template Group by PARAMS
/// - Query parameter `smp_id` must not null
#[utoipa::path(
    delete,
    path = "/ipd/tmp/group",
    responses(DocOne<ExecuteResponse>),
    params(TmpParams),
)]
pub async fn delete_ipd_tmp_group(Query(params): Query<TmpParams>, ctx: RequestState) -> Result<Json<ExecuteResponse>, AppError> {
    ctx.user_state.trace_req_by();
    ctx.authorize_and_access_log(&Method::DELETE, false).await?;

    if params.smp_id.is_some() {
        let response = tmp::delete_group(&params, &ctx.user_state.user.loginname, &ctx.api_state.db_pool, &ctx.api_state.kphis()).await?;

        Ok(Json(response))
    } else {
        Err(AppError::app_400("Delete IpdTmpGroup"))
    }
}

// setting-template-nurse-note-subgroup-data.php
/// /api/ipd/tmp/subgroup
///
/// Get IPD Nursing Template Sub-Group by PARAMS, return list of Nursing Template Sub-Group
#[utoipa::path(
    get,
    path = "/ipd/tmp/subgroup",
    responses(DocVec<TmpSubGroup>),
    params(TmpParams),
)]
pub async fn get_ipd_subgroup(Query(params): Query<TmpParams>, ctx: RequestState) -> Result<Json<Vec<TmpSubGroup>>, AppError> {
    ctx.user_state.trace_req_by();
    ctx.authorize_and_access_log(&Method::GET, false).await?;

    let response = tmp::get_subgroup(&params, &ctx.api_state.db_pool, &ctx.api_state.kphis()).await?;

    Ok(Json(response))
}

// setting-template-nurse-note-subgroup-save.php
/// /api/ipd/tmp/subgroup
///
/// Tries to create/edit Nursing Template Sub-Group
/// - Payload's `smp_id` must not 0, `subgroup_name` must not null or empty
#[utoipa::path(
    post,
    path = "/ipd/tmp/subgroup",
    request_body = TmpSubGroup,
    responses(DocOne<ExecuteResponse>),
)]
pub async fn post_ipd_subgroup(ctx: RequestState, Json(payload): Json<TmpSubGroup>) -> Result<Json<ExecuteResponse>, AppError> {
    ctx.user_state.trace_req_by();
    ctx.authorize_and_access_log(&Method::POST, false).await?;

    if payload.smp_id == 0 || payload.subgroup_name.as_ref().map(|s| s.is_empty()).unwrap_or(true) {
        Err(AppError::app_400("Post IpdTmpSubGroup"))
    } else {
        let response = tmp::post_subgroup(&payload, &ctx.user_state.user.loginname, &ctx.api_state.db_pool, &ctx.api_state.kphis()).await?;

        Ok(Json(response))
    }
}

// setting-template-nurse-note-subgroup-delete.php
/// /api/ipd/tmp/subgroup
///
/// Tries to delete Nursing Template Sub-Group by PARAMS
/// - Query parameter `smp_id` and `subgroup` must not null
#[utoipa::path(
    delete,
    path = "/ipd/tmp/subgroup",
    responses(DocOne<ExecuteResponse>),
    params(TmpParams),
)]
pub async fn delete_ipd_subgroup(Query(params): Query<TmpParams>, ctx: RequestState) -> Result<Json<ExecuteResponse>, AppError> {
    ctx.user_state.trace_req_by();
    ctx.authorize_and_access_log(&Method::DELETE, false).await?;

    if params.smp_id.is_some() && params.subgroup.is_some() {
        let response = tmp::delete_subgroup(&params, &ctx.user_state.user.loginname, &ctx.api_state.db_pool, &ctx.api_state.kphis()).await?;

        Ok(Json(response))
    } else {
        Err(AppError::app_400("Delete IpdTmpSubGroup"))
    }
}

// setting-template-nurse-note-focus-data.php
/// /api/ipd/tmp/focus
///
/// Get IPD Nursing Template Focus by PARAMS, return list of Nursing Template Focus
#[utoipa::path(
    get,
    path = "/ipd/tmp/focus",
    responses(DocVec<TmpFocus>),
    params(TmpParams),
)]
pub async fn get_ipd_focus(Query(params): Query<TmpParams>, ctx: RequestState) -> Result<Json<Vec<TmpFocus>>, AppError> {
    ctx.user_state.trace_req_by();
    ctx.authorize_and_access_log(&Method::GET, false).await?;

    let response = tmp::get_focus(&params, &ctx.api_state.db_pool, &ctx.api_state.kphis()).await?;

    Ok(Json(response))
}

// setting-template-nurse-note-focus-save.php
/// /api/ipd/tmp/focus
///
/// Tries to create/edit Nursing Template Focus
/// - Payload's `smp_id` must not 0, `focus_name` must not null or empty
#[utoipa::path(
    post,
    path = "/ipd/tmp/focus",
    request_body = TmpFocus,
    responses(DocOne<ExecuteResponse>),
)]
pub async fn post_ipd_focus(ctx: RequestState, Json(payload): Json<TmpFocus>) -> Result<Json<ExecuteResponse>, AppError> {
    ctx.user_state.trace_req_by();
    ctx.authorize_and_access_log(&Method::POST, false).await?;

    if payload.smp_id == 0 || payload.focus_name.as_ref().map(|s| s.is_empty()).unwrap_or(true) {
        Err(AppError::app_400("Post IpdTmpFocus"))
    } else {
        let response = tmp::post_focus(&payload, &ctx.user_state.user.loginname, &ctx.api_state.db_pool, &ctx.api_state.kphis()).await?;

        Ok(Json(response))
    }
}

// setting-template-nurse-note-focus-delete.php
/// /api/ipd/tmp/focus
///
/// Tries to delete Nursing Template Focus by PARAMS
/// - Query parameter `id` must not null
#[utoipa::path(
    delete,
    path = "/ipd/tmp/focus",
    responses(DocOne<ExecuteResponse>),
    params(TmpParams),
)]
pub async fn delete_ipd_focus(Query(params): Query<TmpParams>, ctx: RequestState) -> Result<Json<ExecuteResponse>, AppError> {
    ctx.user_state.trace_req_by();
    ctx.authorize_and_access_log(&Method::DELETE, false).await?;

    if params.id.is_some() {
        let response = tmp::delete_focus(&params, &ctx.user_state.user.loginname, &ctx.api_state.db_pool, &ctx.api_state.kphis()).await?;

        Ok(Json(response))
    } else {
        Err(AppError::app_400("Delete IpdTmpFocus"))
    }
}

// setting-template-nurse-note-goal-data.php
/// /api/ipd/tmp/goal
///
/// Get IPD Nursing Template Goal by PARAMS, return list of Nursing Template Goal
#[utoipa::path(
    get,
    path = "/ipd/tmp/goal",
    responses(DocVec<TmpGoal>),
    params(TmpParams),
)]
pub async fn get_ipd_goal(Query(params): Query<TmpParams>, ctx: RequestState) -> Result<Json<Vec<TmpGoal>>, AppError> {
    ctx.user_state.trace_req_by();
    ctx.authorize_and_access_log(&Method::GET, false).await?;

    let response = tmp::get_goal(&params, &ctx.api_state.db_pool, &ctx.api_state.kphis()).await?;

    Ok(Json(response))
}

// setting-template-nurse-note-goal-save.php
/// /api/ipd/tmp/goal
///
/// Tries to create/edit Nursing Template Goal
/// - Payload's `smp_id` must not 0, `goal_name` must not null or empty
#[utoipa::path(
    post,
    path = "/ipd/tmp/goal",
    request_body = TmpGoal,
    responses(DocOne<ExecuteResponse>),
)]
pub async fn post_ipd_goal(ctx: RequestState, Json(payload): Json<TmpGoal>) -> Result<Json<ExecuteResponse>, AppError> {
    ctx.user_state.trace_req_by();
    ctx.authorize_and_access_log(&Method::POST, false).await?;

    if payload.smp_id == 0 || payload.goal_name.as_ref().map(|s| s.is_empty()).unwrap_or(true) {
        Err(AppError::app_400("Post IpdTmpGoal"))
    } else {
        let response = tmp::post_goal(&payload, &ctx.user_state.user.loginname, &ctx.api_state.db_pool, &ctx.api_state.kphis()).await?;

        Ok(Json(response))
    }
}

// setting-template-nurse-note-goal-delete.php
/// /api/ipd/tmp/goal
///
/// Tries to delete Nursing Template Goal by PARAMS
/// - Query parameter `id` must not null
#[utoipa::path(
    delete,
    path = "/ipd/tmp/goal",
    responses(DocOne<ExecuteResponse>),
    params(TmpParams),
)]
pub async fn delete_ipd_goal(Query(params): Query<TmpParams>, ctx: RequestState) -> Result<Json<ExecuteResponse>, AppError> {
    ctx.user_state.trace_req_by();
    ctx.authorize_and_access_log(&Method::DELETE, false).await?;

    if params.id.is_some() {
        let response = tmp::delete_goal(&params, &ctx.user_state.user.loginname, &ctx.api_state.db_pool, &ctx.api_state.kphis()).await?;

        Ok(Json(response))
    } else {
        Err(AppError::app_400("Delete IpdTmpGoal"))
    }
}

// setting-template-nurse-note-intvt-data.php
/// /api/ipd/tmp/intvt
///
/// Get IPD Nursing Template Intervention by PARAMS, return list of Nursing Template Intervention
#[utoipa::path(
    get,
    path = "/ipd/tmp/intvt",
    responses(DocVec<TmpIntvt>),
    params(TmpParams),
)]
pub async fn get_ipd_intvt(Query(params): Query<TmpParams>, ctx: RequestState) -> Result<Json<Vec<TmpIntvt>>, AppError> {
    ctx.user_state.trace_req_by();
    ctx.authorize_and_access_log(&Method::GET, false).await?;

    let response = tmp::get_intvt(&params, &ctx.api_state.db_pool, &ctx.api_state.kphis()).await?;

    Ok(Json(response))
}

// setting-template-nurse-note-intvt-save.php
/// /api/ipd/tmp/intvt
///
/// Tries to create/edit Nursing Template Intervention
/// - Payload's `smp_id` must not 0, `intvt_name` must not null or empty
#[utoipa::path(
    post,
    path = "/ipd/tmp/intvt",
    request_body = TmpIntvt,
    responses(DocOne<ExecuteResponse>),
)]
pub async fn post_ipd_intvt(ctx: RequestState, Json(payload): Json<TmpIntvt>) -> Result<Json<ExecuteResponse>, AppError> {
    ctx.user_state.trace_req_by();
    ctx.authorize_and_access_log(&Method::POST, false).await?;

    if payload.smp_id == 0 || payload.intvt_name.as_ref().map(|s| s.is_empty()).unwrap_or(true) {
        Err(AppError::app_400("Post IpdTmpIntvt"))
    } else {
        let response = tmp::post_intvt(&payload, &ctx.user_state.user.loginname, &ctx.api_state.db_pool, &ctx.api_state.kphis()).await?;

        Ok(Json(response))
    }
}

// setting-template-nurse-note-intvt-delete.php
/// /api/ipd/tmp/intvt
///
/// Tries to delete Nursing Template Intervention by PARAMS
/// - Query parameter `id` must not null
#[utoipa::path(
    delete,
    path = "/ipd/tmp/intvt",
    responses(DocOne<ExecuteResponse>),
    params(TmpParams),
)]
pub async fn delete_ipd_intvt(Query(params): Query<TmpParams>, ctx: RequestState) -> Result<Json<ExecuteResponse>, AppError> {
    ctx.user_state.trace_req_by();
    ctx.authorize_and_access_log(&Method::DELETE, false).await?;

    if params.id.is_some() {
        let response = tmp::delete_intvt(&params, &ctx.user_state.user.loginname, &ctx.api_state.db_pool, &ctx.api_state.kphis()).await?;

        Ok(Json(response))
    } else {
        Err(AppError::app_400("Delete IpdTmpIntvt"))
    }
}

/// /api/ipd/tmp/dlc
///
/// Get IPD Nursing Template Daily-Care by PARAMS, return list of Nursing Template Daily-Care
#[utoipa::path(
    get,
    path = "/ipd/tmp/dlc",
    responses(DocVec<TmpDlc>),
    params(TmpParams),
)]
pub async fn get_ipd_dlc(Query(params): Query<TmpParams>, ctx: RequestState) -> Result<Json<Vec<TmpDlc>>, AppError> {
    ctx.user_state.trace_req_by();
    ctx.authorize_and_access_log(&Method::GET, false).await?;

    let response = tmp::get_dlc(&params, &ctx.api_state.db_pool, &ctx.api_state.kphis()).await?;

    Ok(Json(response))
}

/// /api/ipd/tmp/dlc
///
/// Tries to create/edit Nursing Template Daily-Care
/// - Payload's `dlc_name` must not empty
#[utoipa::path(
    post,
    path = "/ipd/tmp/dlc",
    request_body = TmpDlc,
    responses(DocOne<ExecuteResponse>),
)]
pub async fn post_ipd_dlc(ctx: RequestState, Json(payload): Json<TmpDlc>) -> Result<Json<ExecuteResponse>, AppError> {
    ctx.user_state.trace_req_by();
    ctx.authorize_and_access_log(&Method::POST, false).await?;

    if payload.dlc_name.is_empty() {
        Err(AppError::app_400("Post IpdTmpDlc"))
    } else {
        let response = tmp::post_dlc(&payload, &ctx.user_state.user.loginname, &ctx.api_state.db_pool, &ctx.api_state.kphis()).await?;

        Ok(Json(response))
    }
}

/// /api/ipd/tmp/dlc
///
/// Tries to delete Nursing Template Daily-Care by PARAMS
/// - Query parameter `id` must not null
#[utoipa::path(
    delete,
    path = "/ipd/tmp/dlc",
    responses(DocOne<ExecuteResponse>),
    params(TmpParams),
)]
pub async fn delete_ipd_dlc(Query(params): Query<TmpParams>, ctx: RequestState) -> Result<Json<ExecuteResponse>, AppError> {
    ctx.user_state.trace_req_by();
    ctx.authorize_and_access_log(&Method::DELETE, false).await?;

    if params.id.is_some() {
        let response = tmp::delete_dlc(&params, &ctx.user_state.user.loginname, &ctx.api_state.db_pool, &ctx.api_state.kphis()).await?;

        Ok(Json(response))
    } else {
        Err(AppError::app_400("Delete IpdTmpDlc"))
    }
}
