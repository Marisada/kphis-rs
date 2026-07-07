use axum::{
    Json,
    extract::{Path, Query},
    http::Method,
};

use kphis_api_core::{
    open_api::{DocVec, DocVecU32},
    state::RequestState,
};
use kphis_api_query::opd_er::focus_note;
use kphis_model::{
    fetch::ExecuteResponse,
    focus_note::{FocusNote, FocusNoteParams, FocusNoteSave},
};
use kphis_util::error::AppError;

/// /api/opd-er/focus-note-id/{opd_er_order_master_id}
///
/// Get list of OPD-ER Focus Note by ID and PARAMS, return list of OPD-ER Focus Note
#[utoipa::path(
    get,
    path = "/opd-er/focus-note-id/{opd_er_order_master_id}",
    responses(DocVec<FocusNote>),
    params(
        ("opd_er_order_master_id" = u32, Path, description = "OPD-ER Order Master ID", example = "1"),
        FocusNoteParams,
    ),
)]
pub async fn get_opd_er_focus_note(Path(opd_er_order_master_id): Path<u32>, Query(params): Query<FocusNoteParams>, ctx: RequestState) -> Result<Json<Vec<FocusNote>>, AppError> {
    ctx.user_state.trace_req_by();
    ctx.authorize_and_access_log(&Method::GET, false).await?;

    let response = focus_note::get_focus_note(
        opd_er_order_master_id,
        &params,
        &ctx.api_state.db_pool,
        &ctx.api_state.hosxp(),
        &ctx.api_state.kphis(),
        &ctx.api_state.kphis_extra(),
    )
    .await?;

    Ok(Json(response))
}

// opd-er-nurse-focus-note-save.php OR opd-er-nurse-focus-note-update.php
/// /api/opd-er/focus-note-id/{opd_er_order_master_id}
///
/// Tries to create/edit OPD-ER Focus Note
#[utoipa::path(
    post,
    path = "/opd-er/focus-note-id/{opd_er_order_master_id}",
    request_body = FocusNoteSave,
    responses(DocVecU32<ExecuteResponse>),
    params(
        ("opd_er_order_master_id" = u32, Path, description = "OPD-ER Order Master ID", example = "1"),
    ),
)]
pub async fn post_opd_er_focus_note(Path(opd_er_order_master_id): Path<u32>, ctx: RequestState, Json(payload): Json<FocusNoteSave>) -> Result<Json<(u32, Vec<ExecuteResponse>)>, AppError> {
    ctx.user_state.trace_req_by();
    ctx.authorize_and_access_log(&Method::POST, false).await?;

    let result = focus_note::post_focus_note(
        opd_er_order_master_id,
        &payload,
        &ctx.user_state.user.loginname,
        &ctx.api_state.db_pool,
        &ctx.api_state.kphis(),
        &ctx.api_state.kphis_log(),
    )
    .await?;

    Ok(Json(result))
}

// opd-er-nurse-focus-note-delete.php
/// /api/opd-er/focus-note-id/{opd_er_order_master_id}
/// - Query parameter `fcnote_id` and `version` must not null
#[utoipa::path(
    delete,
    path = "/opd-er/focus-note-id/{opd_er_order_master_id}",
    responses(DocVec<ExecuteResponse>),
    params(
        ("opd_er_order_master_id" = u32, Path, description = "OPD-ER Order Master ID (Optional)", example = "1"),
        FocusNoteParams,
    ),
)]
pub async fn delete_opd_er_focus_note(Path(_opd_er_order_master_id): Path<u32>, Query(params): Query<FocusNoteParams>, ctx: RequestState) -> Result<Json<Vec<ExecuteResponse>>, AppError> {
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
