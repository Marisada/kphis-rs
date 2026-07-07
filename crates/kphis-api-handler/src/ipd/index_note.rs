use axum::{
    Json,
    extract::{Path, Query},
    http::Method,
};

use kphis_api_core::{
    open_api::{DocOne, DocVec, DocVecU32},
    state::RequestState,
};
use kphis_api_query::{ipd::index_note, transform::query::check_an_opt_can_execute};
use kphis_model::{
    fetch::ExecuteResponse,
    ipd::index_note::{IndexNote, IndexNoteParams},
};
use kphis_util::error::AppError;

/// /api/ipd/index-note
///
/// Get list of IPD Index Note by PARAMS, return list of IPD Index Note
#[utoipa::path(
    get,
    path = "/ipd/index-note",
    responses(DocVec<IndexNote>),
    params(IndexNoteParams),
)]
pub async fn get_ipd_index_note(Query(params): Query<IndexNoteParams>, ctx: RequestState) -> Result<Json<Vec<IndexNote>>, AppError> {
    ctx.user_state.trace_req_by();
    let is_pre_admit = ctx.api_state.is_pre_admit_opt(&params.an);
    ctx.authorize_and_access_log(&Method::GET, is_pre_admit).await?;

    let response = index_note::get_index_note(&params, &ctx.api_state.db_pool, &ctx.api_state.kphis()).await?;

    Ok(Json(response))
}

// ipd-nurse-index-note-save.php
/// /api/ipd/index-note
///
/// Tries to create/edit IPD Index Note
/// - Payload's `an` must not null or empty
/// - Checking `an` is `pre-admit` by `an` length
#[utoipa::path(
    post,
    path = "/ipd/index-note",
    request_body = IndexNote,
    responses(DocVecU32<ExecuteResponse>),
)]
pub async fn post_ipd_index_note(ctx: RequestState, Json(payload): Json<IndexNote>) -> Result<Json<(u32, ExecuteResponse)>, AppError> {
    ctx.user_state.trace_req_by();
    let is_pre_admit = ctx.api_state.is_pre_admit_opt(&payload.an);
    ctx.authorize_and_access_log(&Method::POST, is_pre_admit).await?;

    // check AN is valid (pre-admit was admited or admit was revoked)
    check_an_opt_can_execute(&payload.an, ctx.api_state.hosxp_an_len(), &ctx.api_state.db_pool, &ctx.api_state.hosxp(), &ctx.api_state.kphis()).await?;

    let response = index_note::post_index_note(&payload, &ctx.user_state.user.loginname, &ctx.api_state.db_pool, &ctx.api_state.kphis()).await?;

    Ok(Json(response))
}

// ipd-nurse-index-note-delete.php
/// /api/ipd/index-note-id/{nurse_index_note_id}
///
/// Tries to delete IPD Index Note by ID
#[utoipa::path(
    delete,
    path = "/ipd/index-note-id/{nurse_index_note_id}",
    responses(DocOne<ExecuteResponse>),
    params(
        ("nurse_index_note_id" = u32, Path, description = "Nursing Index Note ID", example = "1"),
    ),
)]
pub async fn delete_ipd_index_note(Path(nurse_index_note_id): Path<u32>, ctx: RequestState) -> Result<Json<ExecuteResponse>, AppError> {
    ctx.user_state.trace_req_by();
    ctx.authorize_and_access_log(&Method::DELETE, false).await?;

    let response = index_note::delete_index_note(nurse_index_note_id, &ctx.api_state.db_pool, &ctx.api_state.kphis()).await?;

    Ok(Json(response))
}
