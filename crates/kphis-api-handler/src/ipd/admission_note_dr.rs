use axum::{Json, extract::Path, http::Method};

use kphis_api_core::{
    open_api::{DocOne, DocVec},
    state::RequestState,
};
use kphis_api_query::{ipd::admission_note_dr, transform::query::check_an_can_execute};
use kphis_model::{
    fetch::ExecuteResponse,
    ipd::admission_note_dr::{IpdAdmissionNoteDrRaw, IpdAdmissionNoteDrSave},
};
use kphis_util::error::AppError;

// ipd-dr-admission-note-form.php
/// /api/ipd/admission-note-dr-an/{an}
///
/// Get Doctor's IPD Admission Note by AN, return single Doctor's IPD Admission Note and assiciated data
#[utoipa::path(
    get,
    path = "/ipd/admission-note-dr-an/{an}",
    responses(DocOne<IpdAdmissionNoteDrRaw>),
    params(
        ("an" = String, Path, description = "Admission Number: AN", example = "660001234")
    ),
)]
pub async fn get_ipd_admission_note_dr(Path(an): Path<String>, ctx: RequestState) -> Result<Json<IpdAdmissionNoteDrRaw>, AppError> {
    ctx.user_state.trace_req_by();
    let is_pre_admit = ctx.api_state.is_pre_admit(&an);
    ctx.authorize_and_access_log(&Method::GET, is_pre_admit).await?;

    // We use VN as AN in case of `before admission` input
    // so `an` argument will be `VN` or `AN`
    let response = if is_pre_admit {
        admission_note_dr::get_ipd_admission_note_dr_from_vn(&an, &ctx.api_state.db_pool, &ctx.api_state.hosxp(), &ctx.api_state.kphis(), &ctx.api_state.kphis_extra()).await?
    } else {
        admission_note_dr::get_ipd_admission_note_dr_from_an(&an, &ctx.api_state.db_pool, &ctx.api_state.hosxp(), &ctx.api_state.kphis(), &ctx.api_state.kphis_extra()).await?
    };

    Ok(Json(response))
}

// ipd-dr-admission-note-save.php
/// /api/ipd/admission-note-dr
///
/// Tries to create new Doctor's IPD Admission Note
/// - Payload's `an` must not empty
/// - Checking `an` is `pre-admit` by `an` length
#[utoipa::path(
    post,
    path = "/ipd/admission-note-dr",
    request_body = IpdAdmissionNoteDrSave,
    responses(DocVec<ExecuteResponse>),
)]
pub async fn post_ipd_admission_note_dr(ctx: RequestState, Json(payload): Json<IpdAdmissionNoteDrSave>) -> Result<Json<Vec<ExecuteResponse>>, AppError> {
    ctx.user_state.trace_req_by();
    let is_pre_admit = ctx.api_state.is_pre_admit(&payload.admission_note.an);
    ctx.authorize_and_access_log(&Method::POST, is_pre_admit).await?;

    // check AN is valid (pre-admit was admited or admit was revoked)
    check_an_can_execute(
        &payload.admission_note.an,
        ctx.api_state.hosxp_an_len(),
        &ctx.api_state.db_pool,
        &ctx.api_state.hosxp(),
        &ctx.api_state.kphis(),
    )
    .await?;

    let mut result = Vec::with_capacity(2);
    let note_result = admission_note_dr::post_ipd_admission_note_dr(&payload.admission_note, &ctx.user_state.user.loginname, &ctx.api_state.db_pool, &ctx.api_state.kphis()).await?;
    let admission_note_id = note_result.last_insert_id() as u32;
    result.push(ExecuteResponse::from_query_result(note_result, "Insert IpdDrAdmissionNote"));

    if !payload.admission_note_doctors.is_empty() {
        let item_result = admission_note_dr::insert_ipd_admission_note_dr_items(
            &payload.admission_note_doctors,
            admission_note_id,
            &payload.admission_note.an,
            &ctx.user_state.user.loginname,
            &ctx.api_state.db_pool,
            &ctx.api_state.kphis(),
        )
        .await?;
        result.push(ExecuteResponse::from_query_result(item_result, "Insert IpdDrAdmissionNoteItem"));
    }

    Ok(Json(result))
}

// ipd-dr-admission-note-update.php
/// /api/ipd/admission-note-dr
///
/// Tries to edit Doctor's IPD Admission Note
/// - Payload's `an` must not empty
/// - Checking `an` is `pre-admit` by `an` length
#[utoipa::path(
    put,
    path = "/ipd/admission-note-dr",
    request_body = IpdAdmissionNoteDrSave,
    responses(DocVec<ExecuteResponse>),
)]
pub async fn put_ipd_admission_note_dr(ctx: RequestState, Json(payload): Json<IpdAdmissionNoteDrSave>) -> Result<Json<Vec<ExecuteResponse>>, AppError> {
    ctx.user_state.trace_req_by();
    let is_pre_admit = ctx.api_state.is_pre_admit(&payload.admission_note.an);
    ctx.authorize_and_access_log(&Method::PUT, is_pre_admit).await?;

    // check AN is valid (pre-admit was admited or admit was revoked)
    check_an_can_execute(
        &payload.admission_note.an,
        ctx.api_state.hosxp_an_len(),
        &ctx.api_state.db_pool,
        &ctx.api_state.hosxp(),
        &ctx.api_state.kphis(),
    )
    .await?;

    let mut result = Vec::with_capacity(3);
    let note_result = admission_note_dr::put_ipd_admission_note_dr(&payload.admission_note, &ctx.user_state.user.loginname, &ctx.api_state.db_pool, &ctx.api_state.kphis()).await?;
    result.push(ExecuteResponse::from_query_result(note_result, "Update IpdDrAdmissionNote"));

    if !payload.admission_note_doctors.is_empty() {
        let delete_item_result = admission_note_dr::delete_ipd_admission_note_dr_items(&payload.admission_note.an, &ctx.api_state.db_pool, &ctx.api_state.kphis()).await?;
        result.push(ExecuteResponse::from_query_result(delete_item_result, "Delete IpdDrAdmissionNoteItem"));

        let insert_item_result = admission_note_dr::insert_ipd_admission_note_dr_items(
            &payload.admission_note_doctors,
            payload.admission_note.admission_note_id,
            &payload.admission_note.an,
            &ctx.user_state.user.loginname,
            &ctx.api_state.db_pool,
            &ctx.api_state.kphis(),
        )
        .await?;
        result.push(ExecuteResponse::from_query_result(insert_item_result, "Insert IpdDrAdmissionNoteItem"));
    }

    Ok(Json(result))
}

// ipd-dr-admission-note-pharmacy-check-save.php
/// /api/ipd/admission-note-dr/pharmacy-check-an/{an}
///
/// Tries to mark Doctor's IPD Admission Note's drug allergy of selected AN as checked
/// - Checking `an` is `pre-admit` by `an` length
#[utoipa::path(
    patch,
    path = "/ipd/admission-note-dr/pharmacy-check-an/{an}",
    params(
        ("an" = String, Path, description = "Admission Number: AN", example = "660001234")
    ),
    responses(DocOne<ExecuteResponse>),
)]
pub async fn patch_ipd_pharmacy_check(Path(an): Path<String>, ctx: RequestState) -> Result<Json<ExecuteResponse>, AppError> {
    ctx.user_state.trace_req_by();
    let is_pre_admit = ctx.api_state.is_pre_admit(&an);
    ctx.authorize_and_access_log(&Method::PATCH, is_pre_admit).await?;

    // check AN is valid (pre-admit was admited or admit was revoked)
    check_an_can_execute(&an, ctx.api_state.hosxp_an_len(), &ctx.api_state.db_pool, &ctx.api_state.hosxp(), &ctx.api_state.kphis()).await?;

    let response = admission_note_dr::patch_pharamacy_check(&an, &ctx.user_state.user.doctorcode, &ctx.user_state.user.loginname, &ctx.api_state.db_pool, &ctx.api_state.kphis()).await?;

    Ok(Json(ExecuteResponse::from_query_result(response, "Patch PharmacyCheck")))
}
