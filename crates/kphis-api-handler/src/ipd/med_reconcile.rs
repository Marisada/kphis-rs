use axum::{
    Json,
    extract::{Path, Query},
    http::Method,
};

use kphis_api_core::{
    open_api::{DocOne, DocOpt, DocVec, DocVecU32},
    state::RequestState,
};
use kphis_api_query::{ipd::med_reconcile, transform::query::check_an_opt_can_execute};
use kphis_model::{
    fetch::ExecuteResponse,
    med_reconcile::{
        AdmissionNoteLastDose, MedReconciliation, MedReconciliationDetail, MedReconciliationItemPatch, MedReconciliationItemSave, MedReconciliationNote, MedReconciliationParams, ReMedMedication,
        ReMedVisit,
    },
};
use kphis_util::{
    error::{AppError, Source},
    util::zero_none,
};

// ipd-dr-med-reconcile-data.php
/// /api/ipd/med-reconcile
///
/// Get list of IPD Medical Reconciliation by PARAMS, return list of IPD Medical Reconciliation
///
/// Require HN and (AN or med_reconciliation_id) in PARAMS
#[utoipa::path(
    get,
    path = "/ipd/med-reconcile",
    responses(DocVec<MedReconciliation>),
    params(MedReconciliationParams),
)]
pub async fn get_ipd_med_reconcile(Query(params): Query<MedReconciliationParams>, ctx: RequestState) -> Result<Json<Vec<MedReconciliation>>, AppError> {
    ctx.user_state.trace_req_by();
    let is_pre_admit = ctx.api_state.is_pre_admit_opt(&params.an);
    ctx.authorize_and_access_log(&Method::GET, is_pre_admit).await?;

    if (params.an.is_some() || params.med_reconciliation_id.is_some()) && params.hn.is_some() {
        let recons = med_reconcile::get_ipd_med_reconcile(&params, &ctx.user_state.user.doctorcode, &ctx.api_state.db_pool, &ctx.api_state.hosxp(), &ctx.api_state.kphis()).await?;

        Ok(Json(recons))
    } else {
        Ok(Json(Vec::new()))
    }
}

// // ipd-dr-med-reconcile-save.php
/// /api/ipd/med-reconcile
///
/// Tries to create a new IPD Medical Reconciliation
/// - Query parameters's `an` must not null or empty
/// - Checking `an` is `pre-admit` by `an` length
#[utoipa::path(
    post,
    path = "/ipd/med-reconcile",
    request_body = Vec<MedReconciliationItemSave>,
    responses(DocVecU32<ExecuteResponse>),
    params(MedReconciliationParams),
)]
pub async fn post_ipd_med_reconcile(
    Query(params): Query<MedReconciliationParams>,
    ctx: RequestState,
    Json(payload): Json<Vec<MedReconciliationItemSave>>,
) -> Result<Json<(u32, Vec<ExecuteResponse>)>, AppError> {
    ctx.user_state.trace_req_by();
    let is_pre_admit = ctx.api_state.is_pre_admit_opt(&params.an);
    ctx.authorize_and_access_log(&Method::POST, is_pre_admit).await?;

    if let Some(an) = &params.an {
        // check AN is valid (pre-admit was admited or admit was revoked)
        check_an_opt_can_execute(&params.an, ctx.api_state.hosxp_an_len(), &ctx.api_state.db_pool, &ctx.api_state.hosxp(), &ctx.api_state.kphis()).await?;

        let result = med_reconcile::post_ipd_med_reconcile(
            an,
            &payload,
            &ctx.user_state.user.doctorcode,
            &ctx.user_state.user.loginname,
            &ctx.api_state.db_pool,
            &ctx.api_state.kphis(),
        )
        .await?;

        Ok(Json(result))
    } else {
        Err(AppError::app_400("Post IpdMedReconcile"))
    }
}

// ipd-dr-med-reconcile-doctor-confirm.php
// ipd-dr-med-reconcile-pharmacist-confirm.php
// ipd-dr-med-reconcile-pharmacist-unconfirm.php
// ipd-dr-med-reconcile-last-dose-save.php
/// /api/ipd/med-reconcile
///
/// Tries to edit IPD Medical Reconciliation
/// - Query parameter `med_reconciliation_id` must not null or 0, `patch` must be `doctor`, `pharm`, `unconfirm`, `receive` or `last`
#[utoipa::path(
    patch,
    path = "/ipd/med-reconcile",
    request_body = Vec<MedReconciliationItemPatch>,
    responses(DocVec<ExecuteResponse>),
    params(MedReconciliationParams),
)]
pub async fn patch_ipd_med_reconcile(
    Query(params): Query<MedReconciliationParams>,
    ctx: RequestState,
    Json(payload): Json<Vec<MedReconciliationItemPatch>>,
) -> Result<Json<Vec<ExecuteResponse>>, AppError> {
    ctx.user_state.trace_req_by();
    ctx.authorize_and_access_log(&Method::PATCH, false).await?;

    if let (Some(med_reconciliation_id), Some(patch)) = (params.med_reconciliation_id.and_then(zero_none), params.patch) {
        if ["doctor", "pharm", "unconfirm", "receive", "last"].contains(&patch.as_str()) {
            let result = med_reconcile::patch_ipd_med_reconcile(
                med_reconciliation_id,
                &patch,
                &payload,
                &ctx.user_state.user.doctorcode,
                &ctx.user_state.user.loginname,
                &ctx.api_state.db_pool,
                &ctx.api_state.kphis(),
            )
            .await?;

            Ok(Json(result))
        } else {
            Err(Source::App.to_error(400, "Invalid Patch", "Patch IpdMedReconcile"))
        }
    } else {
        Err(AppError::app_400("Patch IpdMedReconcile"))
    }
}

// ipd-dr-med-reconcile-delete.php
/// /api/ipd/med-reconcile
///
/// Tries to delete IPD Medical Reconciliation by PARAMS
/// - Query parameter `med_reconciliation_id` or `med_reconciliation_item_id` must not null
#[utoipa::path(
    delete,
    path = "/ipd/med-reconcile",
    responses(DocOne<ExecuteResponse>),
    params(MedReconciliationParams),
)]
pub async fn delete_ipd_med_reconcile(Query(params): Query<MedReconciliationParams>, ctx: RequestState) -> Result<Json<ExecuteResponse>, AppError> {
    ctx.user_state.trace_req_by();
    let is_pre_admit = ctx.api_state.is_pre_admit_opt(&params.an);
    ctx.authorize_and_access_log(&Method::DELETE, is_pre_admit).await?;

    if let Some(med_reconciliation_id) = params.med_reconciliation_id {
        let result = med_reconcile::delete_ipd_med_reconcile(med_reconciliation_id, &ctx.api_state.db_pool, &ctx.api_state.kphis()).await?;

        Ok(Json(result))
    } else if let Some(med_reconciliation_item_id) = params.med_reconciliation_item_id {
        let result = med_reconcile::delete_ipd_med_reconcile_item(med_reconciliation_item_id, &ctx.api_state.db_pool, &ctx.api_state.kphis()).await?;

        Ok(Json(result))
    } else {
        Err(AppError::app_400("Delete IpdMedReconcile"))
    }
}

// ipd-dr-med-reconcile-from-hosxp.php
/// /api/ipd/med-reconcile-hosxp-an/{an}
///
/// Get IPD Medical Reconciliation Detail by AN, return list of IPD Medical Reconciliation Detail
#[utoipa::path(
    get,
    path = "/ipd/med-reconcile-hosxp-an/{an}",
    responses(DocVec<MedReconciliationDetail>),
    params(
        ("an" = String, Path, description = "Admission Number: AN", example = "660001234"),
    ),
)]
pub async fn get_ipd_med_reconcile_hosxp(Path(an): Path<String>, ctx: RequestState) -> Result<Json<Vec<MedReconciliationDetail>>, AppError> {
    ctx.user_state.trace_req_by();
    ctx.authorize_and_access_log(&Method::GET, false).await?;

    let recons = med_reconcile::get_ipd_med_reconcile_hosxp(&an, &ctx.api_state.db_pool, &ctx.api_state.hosxp()).await?;

    Ok(Json(recons))
}

// ipd-dr-med-reconcile-dr-admission-note-last-dose.php
/// /api/ipd/med-reconcile-last-dose-an/{an}
///
/// Get IPD Medical Reconciliation Last-Dose by AN, return single IPD Medical Reconciliation Last-Dose or none
#[utoipa::path(
    get,
    path = "/ipd/med-reconcile-last-dose-an/{an}",
    responses(DocOpt<AdmissionNoteLastDose>),
    params(
        ("an" = String, Path, description = "Admission Number: AN", example = "660001234"),
    ),
)]
pub async fn get_ipd_med_reconcile_last_dose(Path(an): Path<String>, ctx: RequestState) -> Result<Json<Option<AdmissionNoteLastDose>>, AppError> {
    ctx.user_state.trace_req_by();
    let is_pre_admit = ctx.api_state.is_pre_admit(&an);
    ctx.authorize_and_access_log(&Method::GET, is_pre_admit).await?;

    let last = med_reconcile::get_ipd_med_reconcile_last_dose(&an, &ctx.api_state.db_pool, &ctx.api_state.kphis()).await?;

    Ok(Json(last))
}

// ipd-dr-med-reconcile-note-data.php
/// /api/ipd/med-reconcile-note-id/{med_reconciliation_id}
///
/// Get IPD Medical Reconciliation Note by ID, return single IPD Medical Reconciliation Note or none
#[utoipa::path(
    get,
    path = "/ipd/med-reconcile-note-id/{med_reconciliation_id}",
    responses(DocOpt<MedReconciliationNote>),
    params(
        ("med_reconciliation_id" = u32, Path, description = "Medical Reconciliation ID", example = "1"),
    ),
)]
pub async fn get_ipd_med_reconcile_note(Path(med_reconciliation_id): Path<u32>, ctx: RequestState) -> Result<Json<Option<MedReconciliationNote>>, AppError> {
    ctx.user_state.trace_req_by();
    ctx.authorize_and_access_log(&Method::GET, false).await?;

    let note = med_reconcile::get_ipd_med_reconcile_note(med_reconciliation_id, &ctx.api_state.db_pool, &ctx.api_state.kphis()).await?;

    Ok(Json(note))
}

// ipd-dr-med-reconcile-note-save.php
/// /api/ipd/med-reconcile-note-id/{med_reconciliation_id}
///
/// Tries to create/edit IPD Medical Reconciliation Note
#[utoipa::path(
    post,
    path = "/ipd/med-reconcile-note-id/{med_reconciliation_id}",
    request_body = String,
    responses(DocOne<ExecuteResponse>),
    params(
        ("med_reconciliation_id" = u32, Path, description = "Medical Reconciliation ID", example = "1"),
    ),
)]
pub async fn post_ipd_med_reconcile_note(Path(med_reconciliation_id): Path<u32>, ctx: RequestState, Json(payload): Json<String>) -> Result<Json<ExecuteResponse>, AppError> {
    ctx.user_state.trace_req_by();
    ctx.authorize_and_access_log(&Method::POST, false).await?;

    let response = med_reconcile::post_ipd_med_reconcile_note(med_reconciliation_id, &payload, &ctx.user_state.user.loginname, &ctx.api_state.db_pool, &ctx.api_state.kphis()).await?;

    Ok(Json(response))
}

// ipd-dr-med-reconcile-remed-visit-data.php
/// /api/ipd/med-reconcile-remed-visit-hn/{hn}
///
/// Get IPD Medical Reconciliation Visit for Remed by HN, return list of IPD Medical Reconciliation Visit for Remed
#[utoipa::path(
    get,
    path = "/ipd/med-reconcile-remed-visit-hn/{hn}",
    responses(DocVec<ReMedVisit>),
    params(
        ("hn" = String, Path, description = "Hospital Number: HN", example = "0001234"),
    ),
)]
pub async fn get_ipd_med_reconcile_remed_visit(Path(hn): Path<String>, ctx: RequestState) -> Result<Json<Vec<ReMedVisit>>, AppError> {
    ctx.user_state.trace_req_by();
    ctx.authorize_and_access_log(&Method::GET, false).await?;

    let note = med_reconcile::get_ipd_med_reconcile_remed_visit(&hn, &ctx.api_state.db_pool, &ctx.api_state.hosxp()).await?;

    Ok(Json(note))
}

// ipd-dr-med-reconcile-remed-med-data.php
/// /api/ipd/med-reconcile-remed-med
///
/// Get IPD Medical Reconciliation Remed by PARAMS, return list of IPD Medical Reconciliation Remed
///
/// Require AN or VN in PARAMS
#[utoipa::path(
    get,
    path = "/ipd/med-reconcile-remed-med",
    responses(DocVec<ReMedMedication>),
    params(MedReconciliationParams),
)]
pub async fn get_ipd_med_reconcile_remed_med(Query(params): Query<MedReconciliationParams>, ctx: RequestState) -> Result<Json<Vec<ReMedMedication>>, AppError> {
    ctx.user_state.trace_req_by();
    let is_pre_admit = ctx.api_state.is_pre_admit_opt(&params.an);
    ctx.authorize_and_access_log(&Method::GET, is_pre_admit).await?;

    if params.vn.is_some() || params.an.is_some() {
        let meds = med_reconcile::get_ipd_med_reconcile_remed_med(&params, &ctx.api_state.db_pool, &ctx.api_state.hosxp()).await?;

        Ok(Json(meds))
    } else {
        Ok(Json(Vec::new()))
    }
}
