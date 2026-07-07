use axum::{Json, extract::Path, http::Method};

use kphis_api_core::{
    open_api::{DocOne, DocOpt},
    state::RequestState,
};
use kphis_api_query::{ipd::admission_note_nurse, transform::query::check_an_can_execute};
use kphis_model::{fetch::ExecuteResponse, ipd::admission_note_nurse::IpdNurseAdmissionNote};
use kphis_util::error::AppError;

// ipd-nurse-admission-note-edit.php
/// /api/ipd/admission-note-nurse-an/{an}
///
/// Get Nurse's IPD Admission Note by AN, return single Nurse's IPD Admission Note or none
#[utoipa::path(
    get,
    path = "/ipd/admission-note-nurse-an/{an}",
    responses(DocOpt<IpdNurseAdmissionNote>),
    params(
        ("an" = String, Path, description = "Admission Number: AN", example = "660001234")
    ),
)]
pub async fn get_ipd_admission_note_nurse(Path(an): Path<String>, ctx: RequestState) -> Result<Json<Option<IpdNurseAdmissionNote>>, AppError> {
    ctx.user_state.trace_req_by();
    let is_pre_admit = ctx.api_state.is_pre_admit(&an);
    ctx.authorize_and_access_log(&Method::GET, is_pre_admit).await?;

    let response = admission_note_nurse::get_ipd_admission_note_nurse(&an, &ctx.api_state.db_pool, &ctx.api_state.hosxp(), &ctx.api_state.kphis()).await?;

    Ok(Json(response))
}

// ipd-nurse-admission-note-save.php
/// /api/ipd/admission-note-nurse
///
/// Tries to create new Nurse's IPD Admission Note
/// - Payload's `an` must not empty
/// - Checking `an` is `pre-admit` by `an` length
#[utoipa::path(
    post,
    path = "/ipd/admission-note-nurse",
    request_body = IpdNurseAdmissionNote,
    responses(DocOne<ExecuteResponse>),
)]
pub async fn post_ipd_admission_note_nurse(ctx: RequestState, Json(payload): Json<IpdNurseAdmissionNote>) -> Result<Json<ExecuteResponse>, AppError> {
    ctx.user_state.trace_req_by();
    let is_pre_admit = ctx.api_state.is_pre_admit(&payload.an);
    ctx.authorize_and_access_log(&Method::POST, is_pre_admit).await?;

    // check AN is valid (pre-admit was admited or admit was revoked)
    check_an_can_execute(&payload.an, ctx.api_state.hosxp_an_len(), &ctx.api_state.db_pool, &ctx.api_state.hosxp(), &ctx.api_state.kphis()).await?;

    let note_result = admission_note_nurse::post_ipd_admission_note_nurse(&payload, &ctx.user_state.user.loginname, &ctx.api_state.db_pool, &ctx.api_state.kphis()).await?;

    Ok(Json(ExecuteResponse::from_query_result(note_result, "Post IpdAdmissionNoteNurse")))
}

// ipd-nurse-admission-note-update.php
/// /api/ipd/admission-note-nurse
///
/// Tries to edit Nurse's IPD Admission Note
/// - Payload's `an` must not empty
/// - Checking `an` is `pre-admit` by `an` length
#[utoipa::path(
    put,
    path = "/ipd/admission-note-nurse",
    request_body = IpdNurseAdmissionNote,
    responses(DocOne<ExecuteResponse>),
)]
pub async fn put_ipd_admission_note_nurse(ctx: RequestState, Json(payload): Json<IpdNurseAdmissionNote>) -> Result<Json<ExecuteResponse>, AppError> {
    ctx.user_state.trace_req_by();
    let is_pre_admit = ctx.api_state.is_pre_admit(&payload.an);
    ctx.authorize_and_access_log(&Method::PUT, is_pre_admit).await?;

    // check AN is valid (pre-admit was admited or admit was revoked)
    check_an_can_execute(&payload.an, ctx.api_state.hosxp_an_len(), &ctx.api_state.db_pool, &ctx.api_state.hosxp(), &ctx.api_state.kphis()).await?;

    let note_result = admission_note_nurse::put_ipd_admission_note_nurse(&payload, &ctx.user_state.user.loginname, &ctx.api_state.db_pool, &ctx.api_state.kphis()).await?;

    Ok(Json(ExecuteResponse::from_query_result(note_result, "Post IpdAdmissionNoteNurse")))
}
