use axum::{
    Json,
    extract::{Path, Query},
    http::Method,
};

use kphis_api_core::{
    open_api::{DocVec, DocVecU32},
    state::RequestState,
};
use kphis_api_query::{ipd::focus_note, transform::query::check_an_can_execute};
use kphis_model::{
    fetch::ExecuteResponse,
    focus_note::{FocusNote, FocusNoteParams, FocusNoteSave, FocusNoteSaveParams},
};
use kphis_util::{error::AppError, util::str_some};

/// /api/ipd/focus-note-an/{an}
///
/// Get list of IPD Focus Note by AN and PARAMS, return list of IPD Focus Note
#[utoipa::path(
    get,
    path = "/ipd/focus-note-an/{an}",
    responses(DocVec<FocusNote>),
    params(
        ("an" = String, Path, description = "Admission Number: AN", example = "660001234"),
        FocusNoteParams,
    ),
)]
pub async fn get_ipd_focus_note(Path(an): Path<String>, Query(params): Query<FocusNoteParams>, ctx: RequestState) -> Result<Json<Vec<FocusNote>>, AppError> {
    ctx.user_state.trace_req_by();
    let is_pre_admit = ctx.api_state.is_pre_admit(&an);
    ctx.authorize_and_access_log(&Method::GET, is_pre_admit).await?;

    let response = focus_note::get_focus_note(&an, &params, &ctx.api_state.db_pool, &ctx.api_state.hosxp(), &ctx.api_state.kphis(), &ctx.api_state.kphis_extra()).await?;

    Ok(Json(response))
}

// ipd-nurse-focus-note-save.php OR ipd-nurse-focus-note-update.php
/// /api/ipd/focus-note-an/{an}
///
/// Tries to create/edit IPD Focus Note
/// - Query parameter `hn` and `ward` must not null or empty
/// - Checking `an` is `pre-admit` by `an` length
#[utoipa::path(
    post,
    path = "/ipd/focus-note-an/{an}",
    request_body = FocusNoteSave,
    responses(DocVecU32<ExecuteResponse>),
    params(
        ("an" = String, Path, description = "Admission Number: AN", example = "660001234"),
        FocusNoteSaveParams,
    ),
)]
pub async fn post_ipd_focus_note(
    Path(an): Path<String>,
    Query(params): Query<FocusNoteSaveParams>,
    ctx: RequestState,
    Json(payload): Json<FocusNoteSave>,
) -> Result<Json<(u32, Vec<ExecuteResponse>)>, AppError> {
    ctx.user_state.trace_req_by();
    let is_pre_admit = ctx.api_state.is_pre_admit(&an);
    ctx.authorize_and_access_log(&Method::POST, is_pre_admit).await?;

    // check AN is valid (pre-admit was admited or admit was revoked)
    check_an_can_execute(&an, ctx.api_state.hosxp_an_len(), &ctx.api_state.db_pool, &ctx.api_state.hosxp(), &ctx.api_state.kphis()).await?;

    if let (Some(hn), Some(ward)) = (params.hn.and_then(str_some), params.ward.and_then(str_some)) {
        let result = focus_note::post_focus_note(
            &an,
            &hn,
            &ward,
            &payload,
            &ctx.user_state.user.loginname,
            &ctx.api_state.db_pool,
            &ctx.api_state.kphis(),
            &ctx.api_state.kphis_log(),
        )
        .await?;

        Ok(Json(result))
    } else {
        Err(AppError::app_400("Post FocusNote"))
    }
}

// ipd-nurse-focus-note-delete.php
/// /api/ipd/focus-note-an/{an}
///
/// Tries to delete IPD Focus Note by PARAMS
/// - Query parameter `fcnote_id` and `version` must not null
#[utoipa::path(
    delete,
    path = "/ipd/focus-note-an/{an}",
    responses(DocVec<ExecuteResponse>),
    params(
        ("an" = String, Path, description = "Admission Number: AN (Optional)", example = "660001234"),
        FocusNoteParams,
    ),
)]
pub async fn delete_ipd_focus_note(Path(_an): Path<String>, Query(params): Query<FocusNoteParams>, ctx: RequestState) -> Result<Json<Vec<ExecuteResponse>>, AppError> {
    ctx.user_state.trace_req_by();
    ctx.authorize_and_access_log(&Method::DELETE, false).await?;

    if let (Some(fcnote_id), Some(version)) = (params.fcnote_id, params.version) {
        let result = focus_note::delete_focus_note(
            fcnote_id,
            version,
            &ctx.user_state.user.loginname,
            &ctx.api_state.db_pool,
            &ctx.api_state.kphis(),
            &ctx.api_state.kphis_log(),
        )
        .await?;

        Ok(Json(result))
    } else {
        Err(AppError::app_400("Delete FocusNote"))
    }
}
