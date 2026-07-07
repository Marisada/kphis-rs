use axum::{Json, extract::Path, http::Method};

use kphis_api_core::{
    open_api::{DocOne, DocVec},
    state::RequestState,
};
use kphis_api_query::{ipd::document, transform::query::check_an_can_execute};
use kphis_model::{
    fetch::ExecuteResponse,
    image::file_path::DocumentType,
    ipd::document::{DocumentScan, IpdDocumentDatetime, IpdDocumentExists},
};
use kphis_util::error::AppError;

/// /api/ipd/document/list-vn-an/{vn}/{an}
///
/// Get amount of all IPD Documents by VN and AN, return single IPD Documents amount
#[utoipa::path(
    get,
    path = "/ipd/document/list-vn-an/{vn}/{an}",
    responses(DocOne<IpdDocumentExists>),
    params(
        ("vn" = String, Path, description = "Visit Number: VN", example = "661231235959"),
        ("an" = String, Path, description = "Admission Number: AN", example = "660001234"),
    ),
)]
pub async fn get_ipd_document_list(Path((vn, an)): Path<(String, String)>, ctx: RequestState) -> Result<Json<IpdDocumentExists>, AppError> {
    ctx.user_state.trace_req_by();
    ctx.authorize_and_access_log(&Method::GET, false).await?;

    let response = document::get_ipd_document_list(&vn, &an, &ctx.api_state.db_pool, &ctx.api_state.hosxp(), &ctx.api_state.kphis(), &ctx.api_state.kphis_extra()).await?;

    Ok(Json(response))
}

/// /api/ipd/document/datetime-an/{an}
///
/// Get DateTime of specific IPD Documents by AN, return single IPD Documents DateTime
#[utoipa::path(
    get,
    path = "/ipd/document/datetime-an/{an}",
    responses(DocOne<IpdDocumentDatetime>),
    params(
        ("an" = String, Path, description = "Admission Number: AN", example = "660001234"),
    ),
)]
pub async fn get_ipd_document_datetime(Path(an): Path<String>, ctx: RequestState) -> Result<Json<IpdDocumentDatetime>, AppError> {
    ctx.user_state.trace_req_by();
    ctx.authorize_and_access_log(&Method::GET, false).await?;

    let response = document::get_ipd_document_datetime(&an, &ctx.api_state.db_pool, &ctx.api_state.kphis()).await?;

    Ok(Json(response))
}

/// /api/ipd/document/scan-an/{an}
///
/// Get Document Scan by AN, return list of DocumentScan
#[utoipa::path(
    get,
    path = "/ipd/document/scan-an/{an}",
    responses(DocVec<DocumentScan>),
    params(
        ("an" = String, Path, description = "Admission Number: AN", example = "660001234"),
    ),
)]
pub async fn get_ipd_document_types(Path(an): Path<String>, ctx: RequestState) -> Result<Json<Vec<DocumentScan>>, AppError> {
    ctx.user_state.trace_req_by();
    ctx.authorize_and_access_log(&Method::GET, false).await?;

    let response = document::get_ipd_document_types(&an, &ctx.api_state.db_pool, &ctx.api_state.kphis_extra()).await?;

    Ok(Json(response))
}

/// /api/ipd/document/scan-an/{an}
///
/// Tries to create IPD Document Scan Type
/// - Checking `an` is `pre-admit` by `an` length
#[utoipa::path(
    post,
    path = "/ipd/document/scan-an/{an}",
    request_body = u8,
    responses(DocOne<ExecuteResponse>),
    params(
        ("an" = String, Path, description = "Admission Number: AN", example = "660001234"),
    ),
)]
pub async fn post_ipd_document_type(Path(an): Path<String>, ctx: RequestState, Json(payload): Json<DocumentType>) -> Result<Json<ExecuteResponse>, AppError> {
    ctx.user_state.trace_req_by();
    ctx.authorize_and_access_log(&Method::POST, false).await?;

    // check AN is valid (pre-admit was admited or admit was revoked)
    check_an_can_execute(&an, ctx.api_state.hosxp_an_len(), &ctx.api_state.db_pool, &ctx.api_state.hosxp(), &ctx.api_state.kphis()).await?;

    let result = document::post_ipd_document_type(&an, payload as u8, &ctx.user_state.user.loginname, &ctx.api_state.db_pool, &ctx.api_state.kphis_extra()).await?;

    Ok(Json(result))
}

/// /api/ipd/document/scan-an/{an}
///
/// Tries to delete IPD Document Scan Type
/// - Checking `an` is `pre-admit` by `an` length
#[utoipa::path(
    delete,
    path = "/ipd/document/scan-an/{an}",
    request_body = u8,
    responses(DocOne<ExecuteResponse>),
    params(
        ("an" = String, Path, description = "Admission Number: AN", example = "660001234"),
    ),
)]
pub async fn delete_ipd_document_type(Path(an): Path<String>, ctx: RequestState, Json(payload): Json<DocumentType>) -> Result<Json<ExecuteResponse>, AppError> {
    ctx.user_state.trace_req_by();
    ctx.authorize_and_access_log(&Method::DELETE, false).await?;

    // check AN is valid (pre-admit was admited or admit was revoked)
    check_an_can_execute(&an, ctx.api_state.hosxp_an_len(), &ctx.api_state.db_pool, &ctx.api_state.hosxp(), &ctx.api_state.kphis()).await?;

    let result = document::delete_ipd_document_type(&an, payload as u8, &ctx.api_state.db_pool, &ctx.api_state.kphis_extra()).await?;

    Ok(Json(result))
}
