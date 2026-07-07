use axum::{
    Json,
    extract::{Path, Query},
    http::Method,
};

use kphis_api_core::{
    open_api::{DocVec, DocVecU32},
    state::RequestState,
};
use kphis_api_query::{ipd::focus_list, transform::query::check_an_can_execute};
use kphis_model::{
    fetch::ExecuteResponse,
    focus_list::{FocusList, FocusListParams, FocusListSave, FocusListSaveParams},
};
use kphis_util::error::AppError;

/// /api/ipd/focus-list-an/{an}
///
/// Get list of IPD Focus List by AN and PARAMS, return list of IPD Focus List
#[utoipa::path(
    get,
    path = "/ipd/focus-list-an/{an}",
    responses(DocVec<FocusList>),
    params(
        ("an" = String, Path, description = "Admission Number: AN", example = "660001234"),
        FocusListParams,
    ),
)]
pub async fn get_ipd_focus_list(Path(an): Path<String>, Query(params): Query<FocusListParams>, ctx: RequestState) -> Result<Json<Vec<FocusList>>, AppError> {
    ctx.user_state.trace_req_by();
    let is_pre_admit = ctx.api_state.is_pre_admit(&an);
    ctx.authorize_and_access_log(&Method::GET, is_pre_admit).await?;

    let response = focus_list::get_focus_list(&an, &params, &ctx.api_state.db_pool, &ctx.api_state.hosxp(), &ctx.api_state.kphis()).await?;

    Ok(Json(response))
}

// ipd-nurse-focus-list-save.php OR ipd-nurse-focus-list-update.php
/// /api/ipd/focus-list-an/{an}
///
/// Tries to create/edit IPD Focus List
/// - Query parameter `hn` must not null or empty
/// - Checking `an` is `pre-admit` by `an` length
#[utoipa::path(
    post,
    path = "/ipd/focus-list-an/{an}",
    request_body = FocusListSave,
    responses(DocVecU32<ExecuteResponse>),
    params(
        ("an" = String, Path, description = "Admission Number: AN", example = "660001234"),
        FocusListSaveParams,
    ),
)]
pub async fn post_ipd_focus_list(
    Path(an): Path<String>,
    Query(params): Query<FocusListSaveParams>,
    ctx: RequestState,
    Json(payload): Json<FocusListSave>,
) -> Result<Json<(u32, Vec<ExecuteResponse>)>, AppError> {
    ctx.user_state.trace_req_by();
    let is_pre_admit = ctx.api_state.is_pre_admit(&an);
    ctx.authorize_and_access_log(&Method::POST, is_pre_admit).await?;

    // check AN is valid (pre-admit was admited or admit was revoked)
    check_an_can_execute(&an, ctx.api_state.hosxp_an_len(), &ctx.api_state.db_pool, &ctx.api_state.hosxp(), &ctx.api_state.kphis()).await?;

    if let Some(hn) = &params.hn
        && !hn.is_empty()
    {
        let result = focus_list::post_focus_list(
            &an,
            hn,
            &payload,
            &ctx.user_state.user.loginname,
            &ctx.api_state.db_pool,
            &ctx.api_state.hosxp(),
            &ctx.api_state.kphis(),
            &ctx.api_state.kphis_log(),
        )
        .await?;

        Ok(Json(result))
    } else {
        Err(AppError::app_400("Post FocusList"))
    }
}

// ipd-nurse-focus-list-delete.php
/// /api/ipd/focus-list-an/{an}
///
/// Tries to delete IPD Focus List by PARAMS
/// - Query parameter `fclist_id` and `version` must not null
#[utoipa::path(
    delete,
    path = "/ipd/focus-list-an/{an}",
    responses(DocVec<ExecuteResponse>),
    params(
        ("an" = String, Path, description = "Admission Number: AN (Optional)", example = "660001234"),
        FocusListParams,
    ),
)]
pub async fn delete_ipd_focus_list(Path(_an): Path<String>, Query(params): Query<FocusListParams>, ctx: RequestState) -> Result<Json<Vec<ExecuteResponse>>, AppError> {
    ctx.user_state.trace_req_by();
    ctx.authorize_and_access_log(&Method::DELETE, false).await?;

    if let (Some(fclist_id), Some(version)) = (params.fclist_id, params.version) {
        let result = focus_list::delete_focus_list(
            fclist_id,
            version,
            &ctx.user_state.user.loginname,
            &ctx.api_state.db_pool,
            &ctx.api_state.hosxp(),
            &ctx.api_state.kphis(),
            &ctx.api_state.kphis_log(),
        )
        .await?;

        Ok(Json(result))
    } else {
        Err(AppError::app_400("Delete FocusList"))
    }
}
