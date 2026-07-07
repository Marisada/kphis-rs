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
use kphis_api_query::{ipd::progress_note, transform::query::check_an_can_execute};
use kphis_model::{
    fetch::ExecuteResponse,
    progress_note::{ProgressNote, ProgressNoteItem, ProgressNoteItemType, ProgressNoteParams, ProgressNoteSave, ProgressNoteTypeName},
};
use kphis_util::error::AppError;

// ipd-dr-order-progress-note-previous-problem-list-data.php
/// /api/ipd/order/progress-previous
///
/// Get list of Previous IPD Progress Note Item by PARAMS, return list of IPD Progress Note Item
///
/// Require AN and progress_note_owner_type in PARAMS
#[utoipa::path(
    get,
    path = "/ipd/order/progress-previous",
    responses(DocVec<ProgressNoteItem>),
    params(ProgressNoteParams),
)]
pub async fn get_ipd_progress_previous(Query(params): Query<ProgressNoteParams>, ctx: RequestState) -> Result<Json<Vec<ProgressNoteItem>>, AppError> {
    ctx.user_state.trace_req_by();
    let is_pre_admit = ctx.api_state.is_pre_admit_opt(&params.an);
    ctx.authorize_and_access_log(&Method::GET, is_pre_admit).await?;

    if let (Some(an), Some(progress_note_owner_type)) = (params.an, params.progress_note_owner_type) {
        let progress_items = progress_note::get_previous_progress(&an, &progress_note_owner_type, &ctx.api_state.db_pool, &ctx.api_state.kphis()).await?;

        Ok(Json(progress_items))
    } else {
        Ok(Json(Vec::new()))
    }
}

// ipd-dr-order-progress-note-data.php
/// /api/ipd/order/progress-note
///
/// Get list of IPD Progress Note by PARAMS, return list of IPD Progress Note
#[utoipa::path(
    get,
    path = "/ipd/order/progress-note",
    responses(DocVec<ProgressNote>),
    params(ProgressNoteParams),
)]
pub async fn get_ipd_progress_note(Query(params): Query<ProgressNoteParams>, ctx: RequestState) -> Result<Json<Vec<ProgressNote>>, AppError> {
    ctx.user_state.trace_req_by();
    let is_pre_admit = ctx.api_state.is_pre_admit_opt(&params.an);
    ctx.authorize_and_access_log(&Method::GET, is_pre_admit).await?;

    let progress_notes = get_ipd_progress_note_bundle(
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

pub async fn get_ipd_progress_note_bundle(
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

// ipd-dr-order-progress-note-save.php
/// /api/ipd/order/progress-note
///
/// Tries to create/edit IPD Progress Note
/// - Payload's `visit_type` must has not-empty `an`
/// - Checking `an` is `pre-admit` by `an` length
#[utoipa::path(
    post,
    path = "/ipd/order/progress-note",
    request_body = ProgressNoteSave,
    responses(DocVecU32<ExecuteResponse>),
)]
pub async fn post_ipd_progress_note(ctx: RequestState, Json(payload): Json<ProgressNoteSave>) -> Result<Json<(u32, Vec<ExecuteResponse>)>, AppError> {
    ctx.user_state.trace_req_by();

    if let Some((an, is_pre_admit)) = payload.visit_type.an_and_is_pre_admit() {
        ctx.authorize_and_access_log(&Method::POST, is_pre_admit).await?;
        // check AN is valid (pre-admit was admited or admit was revoked)
        check_an_can_execute(an, ctx.api_state.hosxp_an_len(), &ctx.api_state.db_pool, &ctx.api_state.hosxp(), &ctx.api_state.kphis()).await?;
    } else {
        return Err(AppError::app_400("PostIpdProgressNote"));
    }

    let response = progress_note::post_progress_note(&payload, &ctx.user_state.user.loginname, &ctx.api_state.db_pool, &ctx.api_state.kphis()).await?;

    Ok(Json(response))
}

// ipd-dr-order-progress-note-delete.php
/// /api/ipd/order/progress-note-id/{progress_note_id}
///
/// Tries to delete IPD Progress Note by ID
#[utoipa::path(
    delete,
    path = "/ipd/order/progress-note-id/{progress_note_id}",
    responses(DocOne<ExecuteResponse>),
    params(
        ("progress_note_id" = u32, Path, description = "Progress Note ID", example = "1"),
    ),
)]
pub async fn delete_ipd_progress_note(Path(progress_note_id): Path<u32>, ctx: RequestState) -> Result<Json<ExecuteResponse>, AppError> {
    ctx.user_state.trace_req_by();
    ctx.authorize_and_access_log(&Method::DELETE, false).await?;

    let response = progress_note::delete_progress_note(progress_note_id, &ctx.api_state.db_pool, &ctx.api_state.kphis()).await?;

    Ok(Json(response))
}
