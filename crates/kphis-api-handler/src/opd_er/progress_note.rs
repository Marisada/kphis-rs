use axum::{
    Json,
    extract::{Path, Query},
    http::Method,
};
use sqlx::{MySql, Pool};

use kphis_api_core::{
    open_api::{DocOne, DocVec, DocVecU32},
    state::RequestState,
};
use kphis_api_query::opd_er::progress_note;
use kphis_model::{
    fetch::ExecuteResponse,
    progress_note::{ProgressNote, ProgressNoteItemType, ProgressNoteParams, ProgressNoteSave, ProgressNoteTypeName},
};
use kphis_util::error::AppError;

// opd-er-order-progress-note-data.php
/// /api/opd-er/order/progress-note
///
/// Get list of OPD-ER Progress Note by PARAMS, return list of OPD-ER Progress Note
#[utoipa::path(
    get,
    path = "/opd-er/order/progress-note",
    responses(DocVec<ProgressNote>),
    params(ProgressNoteParams),
)]
pub async fn get_opd_er_progress_note(Query(params): Query<ProgressNoteParams>, ctx: RequestState) -> Result<Json<Vec<ProgressNote>>, AppError> {
    ctx.user_state.trace_req_by();
    ctx.authorize_and_access_log(&Method::GET, false).await?;

    let progress_notes = get_opd_er_progress_note_bundle(
        &params,
        &ctx.api_state.app_config.doctor_intern_roles,
        &ctx.api_state.db_pool,
        &ctx.api_state.hosxp(),
        &ctx.api_state.kphis(),
        &ctx.api_state.kphis_extra(),
    )
    .await?;

    Ok(Json(progress_notes))
}

pub async fn get_opd_er_progress_note_bundle(
    params: &ProgressNoteParams,
    intern_roles: &[String],
    pool: &Pool<MySql>,
    hosxp: &str,
    kphis: &str,
    kphis_extra: &str,
) -> Result<Vec<ProgressNote>, AppError> {
    let mut progress_notes = progress_note::get_progress_note(params, intern_roles, pool, hosxp, kphis, kphis_extra).await?;
    let ids = progress_notes.iter().map(|op| op.progress_note_id).collect::<Vec<u32>>();
    let item_types = progress_note::get_progress_note_types(&ids, pool, kphis).await?;
    for (id, progress_note_item_type) in item_types {
        if let Some(pos) = progress_notes.iter().position(|or| or.progress_note_id == id) {
            let progress_note_items = progress_note::get_progress_note_item(id, &progress_note_item_type, pool, kphis).await?;
            progress_notes[pos].progress_note_item_types.push(ProgressNoteItemType {
                progress_note_item_type: ProgressNoteTypeName::from_string(&progress_note_item_type),
                progress_note_items,
            });
        }
    }

    Ok(progress_notes)
}

// opd-er-order-progress-note-save.php
/// /api/opd-er/order/progress-note
///
/// Tries to create/edit OPD-ER Progress Note
#[utoipa::path(
    post,
    path = "/opd-er/order/progress-note",
    request_body = ProgressNoteSave,
    responses(DocVecU32<ExecuteResponse>),
)]
pub async fn post_opd_er_progress_note(ctx: RequestState, Json(payload): Json<ProgressNoteSave>) -> Result<Json<(u32, Vec<ExecuteResponse>)>, AppError> {
    ctx.user_state.trace_req_by();
    ctx.authorize_and_access_log(&Method::POST, false).await?;

    let response = progress_note::post_progress_note(&payload, &ctx.user_state.user.loginname, &ctx.api_state.db_pool, &ctx.api_state.kphis()).await?;

    Ok(Json(response))
}

// opd-er-order-progress-note-delete.php
/// /api/opd-er/order/progress-note-id/{progress_note_id}
///
/// Tries to delete OPD-ER Progress Note by ID
#[utoipa::path(
    delete,
    path = "/opd-er/order/progress-note-id/{progress_note_id}",
    responses(DocOne<ExecuteResponse>),
    params(
        ("progress_note_id" = u32, Path, description = "Progress Note ID", example = "1"),
    ),
)]
pub async fn delete_opd_er_progress_note(Path(progress_note_id): Path<u32>, ctx: RequestState) -> Result<Json<ExecuteResponse>, AppError> {
    ctx.user_state.trace_req_by();
    ctx.authorize_and_access_log(&Method::DELETE, false).await?;

    let response = progress_note::delete_progress_note(progress_note_id, &ctx.api_state.db_pool, &ctx.api_state.kphis()).await?;

    Ok(Json(response))
}
