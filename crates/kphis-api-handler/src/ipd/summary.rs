use axum::{
    Json,
    extract::{Path, Query},
    http::Method,
};

use kphis_api_core::{
    open_api::{DocOne, DocVec, DocVecU32},
    state::RequestState,
};
use kphis_api_query::{ipd::summary, transform::query::check_an_can_execute};
use kphis_model::{
    fetch::ExecuteResponse,
    ipd::summary::{Summary, SummaryCodeSave, SummaryNote, SummaryNoteSave, SummaryParams, SummarySave, SummaryStatus},
    user::permission::Permission,
};
use kphis_util::error::AppError;

// ipd-summary-2-data.php
// ipd-summary-2-hosxp-ct-mri-data.php
// ipd-summary-2-hosxp-ipt-data.php
// ipd-summary-2-hosxp-or-data.php
// ipd-summary-2-lab-data.php
// ipd-summary-2-problem-list-data.php
/// /api/ipd/summary
///
/// Get IPD Summary by PARAMS, return single IPD Summary
/// - Query parameter `an` or `summary_id` must not null
#[utoipa::path(
    get,
    path = "/ipd/summary",
    responses(DocOne<Summary>),
    params(SummaryParams),
)]
pub async fn get_ipd_summary(Query(params): Query<SummaryParams>, ctx: RequestState) -> Result<Json<Summary>, AppError> {
    ctx.user_state.trace_req_by();
    let is_pre_admit = ctx.api_state.is_pre_admit_opt(&params.an);
    ctx.authorize_and_access_log(&Method::GET, is_pre_admit).await?;

    if params.an.is_some() || params.summary_id.is_some() {
        let summary = summary::get_ipd_summary(
            &params,
            &ctx.user_state.user.doctorcode,
            &ctx.user_state.user.groupname,
            &ctx.api_state.app_config.lab_alerts,
            &ctx.api_state.operation_success(),
            &ctx.api_state.db_pool,
            &ctx.api_state.hosxp(),
            &ctx.api_state.kphis(),
        )
        .await?;

        Ok(Json(summary))
    } else {
        Err(AppError::app_400("Select IpdSummary"))
    }
}

// // ipd-summary-2-save.php
/// /api/ipd/summary
///
/// Tries to create/edit IPD Summary
/// - Payload's `status` must not `claim` or `done`
/// - Payload's `an` must not empty
/// - Checking `an` is `pre-admit` by `an` length
#[utoipa::path(
    post,
    path = "/ipd/summary",
    request_body = SummarySave,
    responses(DocVecU32<ExecuteResponse>),
)]
pub async fn post_ipd_summary(ctx: RequestState, Json(payload): Json<SummarySave>) -> Result<Json<(u32, Vec<ExecuteResponse>)>, AppError> {
    ctx.user_state.trace_req_by();
    let is_pre_admit = ctx.api_state.is_pre_admit(&payload.summary.an);
    ctx.authorize_and_access_log(&Method::POST, is_pre_admit).await?;

    // AuditStatus::Claim | AuditStatus::Done
    if payload.is_summary_locked() {
        Err(AppError::app_400("Post IpdSummary"))
    } else {
        // check AN is valid (pre-admit was admited or admit was revoked)
        check_an_can_execute(
            &payload.summary.an,
            ctx.api_state.hosxp_an_len(),
            &ctx.api_state.db_pool,
            &ctx.api_state.hosxp(),
            &ctx.api_state.kphis(),
        )
        .await?;

        let is_doctor = if ctx.api_state.production() {
            ctx.user_state.permissions.contains(&Permission::DataTypeDoctorUse)
        } else {
            true
        };

        // only doctor can edit diagnosis 1-4
        let result = summary::post_ipd_summary(
            &payload,
            &ctx.user_state.user.doctorcode,
            &ctx.user_state.user.loginname,
            is_doctor,
            &ctx.api_state.db_pool,
            &ctx.api_state.kphis(),
        )
        .await?;

        Ok(Json(result))
    }
}

/// /api/ipd/summary
///
/// Tries to add codes to IPD Summary
/// - Payload's `an` must not empty
/// - Checking `an` is `pre-admit` by `an` length
#[utoipa::path(
    patch,
    path = "/ipd/summary",
    request_body = SummaryCodeSave,
    responses(DocOne<ExecuteResponse>),
)]
pub async fn patch_ipd_summary(ctx: RequestState, Json(payload): Json<SummaryCodeSave>) -> Result<Json<ExecuteResponse>, AppError> {
    ctx.user_state.trace_req_by();
    let is_pre_admit = ctx.api_state.is_pre_admit(&payload.an);
    ctx.authorize_and_access_log(&Method::PATCH, is_pre_admit).await?;

    // check AN is valid (pre-admit was admited or admit was revoked)
    check_an_can_execute(&payload.an, ctx.api_state.hosxp_an_len(), &ctx.api_state.db_pool, &ctx.api_state.hosxp(), &ctx.api_state.kphis()).await?;

    let result = summary::update_summary2_code(&payload, &ctx.user_state.user.loginname, &ctx.api_state.db_pool, &ctx.api_state.kphis())
        .await
        .map(|res| ExecuteResponse::from_query_result(res, "Patch Summary"))?;

    Ok(Json(result))
}

/// /api/ipd/summary-note-id/{summary_id}
///
/// Get IPD Summary Note by summary_id, return a list IPD Summary Note
#[utoipa::path(
    get,
    path = "/ipd/summary-note-id/{summary_id}",
    responses(DocVec<SummaryNote>),
    params(
        ("summary_id" = u32, Path, description = "Summary ID", example = "1"),
    ),
)]
pub async fn get_ipd_summary_note(Path(summary_id): Path<u32>, ctx: RequestState) -> Result<Json<Vec<SummaryNote>>, AppError> {
    ctx.user_state.trace_req_by();
    ctx.authorize_and_access_log(&Method::GET, false).await?;

    let notes = summary::select_summary_note(summary_id, &ctx.api_state.db_pool, &ctx.api_state.hosxp(), &ctx.api_state.kphis()).await?;

    Ok(Json(notes))
}

/// /api/ipd/summary-note-id/{summary_id}
///
/// Tries to create IPD Summary Note
#[utoipa::path(
    post,
    path = "/ipd/summary-note-id/{summary_id}",
    request_body = SummaryNoteSave,
    responses(DocOne<ExecuteResponse>),
    params(
        ("summary_id" = u32, Path, description = "Summary ID", example = "1"),
    ),
)]
pub async fn post_ipd_summary_note(Path(summary_id): Path<u32>, ctx: RequestState, Json(payload): Json<SummaryNoteSave>) -> Result<Json<ExecuteResponse>, AppError> {
    ctx.user_state.trace_req_by();
    ctx.authorize_and_access_log(&Method::POST, false).await?;

    let result = summary::insert_summary_note(
        summary_id,
        &payload,
        &ctx.user_state.user.doctorcode,
        &ctx.user_state.user.loginname,
        &ctx.api_state.db_pool,
        &ctx.api_state.kphis(),
    )
    .await
    .map(|res| ExecuteResponse::from_query_result(res, "Insert SummaryNoteSave"))?;

    Ok(Json(result))
}

/// /api/ipd/summary-note-id/{summary_id}
///
/// Tries to update IPD Summary Note's text
/// - Payload's `note_id` must not null
#[utoipa::path(
    patch,
    path = "/ipd/summary-note-id/{summary_id}",
    request_body = SummaryNoteSave,
    responses(DocOne<ExecuteResponse>),
    params(
        ("summary_id" = u32, Path, description = "Summary ID", example = "1"),
    ),
)]
pub async fn patch_ipd_summary_note(Path(summary_id): Path<u32>, ctx: RequestState, Json(payload): Json<SummaryNoteSave>) -> Result<Json<ExecuteResponse>, AppError> {
    ctx.user_state.trace_req_by();
    ctx.authorize_and_access_log(&Method::POST, false).await?;

    if payload.note_id.is_some() {
        let result = summary::update_summary_note(
            summary_id,
            &payload,
            &ctx.user_state.user.doctorcode,
            &ctx.user_state.user.loginname,
            &ctx.api_state.db_pool,
            &ctx.api_state.kphis(),
        )
        .await
        .map(|res| ExecuteResponse::from_query_result(res, "Update SummaryNoteSave"))?;

        Ok(Json(result))
    } else {
        Err(AppError::app_400("Update SummaryNoteSave"))
    }
}

/// /api/ipd/summary-note-id/{summary_id}
///
/// Tries to delete IPD Summary Note
/// - Payload's `note_id` must not null
#[utoipa::path(
    post,
    path = "/ipd/summary-note-id/{summary_id}",
    request_body = SummaryNoteSave,
    responses(DocOne<ExecuteResponse>),
    params(
        ("summary_id" = u32, Path, description = "Summary ID", example = "1"),
    ),
)]
pub async fn delete_ipd_summary_note(Path(summary_id): Path<u32>, ctx: RequestState, Json(payload): Json<SummaryNoteSave>) -> Result<Json<ExecuteResponse>, AppError> {
    ctx.user_state.trace_req_by();
    ctx.authorize_and_access_log(&Method::POST, false).await?;

    if payload.note_id.is_some() {
        let result = summary::delete_summary_note(summary_id, &payload, &ctx.user_state.user.doctorcode, &ctx.api_state.db_pool, &ctx.api_state.kphis())
            .await
            .map(|res| ExecuteResponse::from_query_result(res, "Delete SummaryNoteSave"))?;

        Ok(Json(result))
    } else {
        Err(AppError::app_400("Delete SummaryNoteSave"))
    }
}

/// /api/ipd/summary-status-id
///
/// Get IPD Summary Status by summary_id, return Status
#[utoipa::path(
    get,
    path = "/ipd/summary-status-id/{summary_id}",
    responses(DocOne<Option<SummaryStatus>>),
    params(
        ("summary_id" = u32, Path, description = "Summary ID", example = "1"),
    ),
)]
pub async fn get_ipd_summary_status(Path(summary_id): Path<u32>, ctx: RequestState) -> Result<Json<Option<SummaryStatus>>, AppError> {
    ctx.user_state.trace_req_by();
    ctx.authorize_and_access_log(&Method::GET, false).await?;

    let status = summary::get_summary_status(summary_id, &ctx.api_state.db_pool, &ctx.api_state.kphis()).await?;

    Ok(Json(status))
}

/// /api/ipd/summary-status-id/{summary_id}
///
/// Tries to update IPD Summary Status
#[utoipa::path(
    put,
    path = "/ipd/summary-status-id/{summary_id}",
    request_body = SummaryStatus,
    responses(DocOne<ExecuteResponse>),
    params(
        ("summary_id" = u32, Path, description = "Summary ID", example = "1"),
    ),
)]
pub async fn put_ipd_summary_status(Path(summary_id): Path<u32>, ctx: RequestState, Json(payload): Json<SummaryStatus>) -> Result<Json<ExecuteResponse>, AppError> {
    ctx.user_state.trace_req_by();
    ctx.authorize_and_access_log(&Method::PUT, false).await?;

    let result = summary::update_summary2_status(&payload.status, summary_id, &ctx.user_state.user.loginname, &ctx.api_state.db_pool, &ctx.api_state.kphis())
        .await
        .map(|res| ExecuteResponse::from_query_result(res, "Update SummaryStatus"))?;

    Ok(Json(result))
}
