use axum::{Json, extract::Path, http::Method};

use kphis_api_core::{
    open_api::{DocOne, DocVec},
    state::RequestState,
};
use kphis_api_query::opd_er::document;
use kphis_model::{
    fetch::ExecuteResponse,
    {image::file_path::DocumentType, ipd::document::DocumentScan, opd_er::document::OpdErDocumentExists},
};
use kphis_util::error::AppError;

/// /api/opd-er/document/list-vn-id/{vn}/{opd_er_order_master_id}
///
/// Get amount of all OPD-ER Documents by VN and ID, return single OPD-ER Documents amount
#[utoipa::path(
    get,
    path = "/opd_er/document/list-vn-id/{vn}/{opd_er_order_master_id}",
    responses(DocOne<OpdErDocumentExists>),
    params(
        ("vn" = String, Path, description = "Visit Number: VN", example = "661231235959"),
        ("opd_er_order_master_id" = u32, Path, description = "OPD-ER Order Master ID", example = "1"),
    ),
)]
pub async fn get_opd_er_document_list(Path((vn, opd_er_order_master_id)): Path<(String, u32)>, ctx: RequestState) -> Result<Json<OpdErDocumentExists>, AppError> {
    ctx.user_state.trace_req_by();
    ctx.authorize_and_access_log(&Method::GET, false).await?;

    let response = document::get_opd_er_document_list(
        &vn,
        opd_er_order_master_id,
        &ctx.api_state.db_pool,
        &ctx.api_state.hosxp(),
        &ctx.api_state.kphis(),
        &ctx.api_state.kphis_extra(),
    )
    .await?;

    Ok(Json(response))
}

/// /api/opd_er/document/scan-id/{opd_er_order_master_id}
///
/// Get Document Scan by opd_er_order_master_id, return list of DocumentScan
#[utoipa::path(
    get,
    path = "/opd-er/document/scan-id/{opd_er_order_master_id}",
    responses(DocVec<DocumentScan>),
    params(
        ("opd_er_order_master_id" = u32, Path, description = "OPD-ER Order Master ID", example = "1"),
    ),
)]
pub async fn get_opd_er_document_types(Path(opd_er_order_master_id): Path<u32>, ctx: RequestState) -> Result<Json<Vec<DocumentScan>>, AppError> {
    ctx.user_state.trace_req_by();
    ctx.authorize_and_access_log(&Method::GET, false).await?;

    let response = document::get_opd_er_document_types(opd_er_order_master_id, &ctx.api_state.db_pool, &ctx.api_state.kphis_extra()).await?;

    Ok(Json(response))
}

/// /api/opd-er/document/scan-id/{opd_er_order_master_id}
///
/// Tries to create OPD-ER Document Scan Type
#[utoipa::path(
    post,
    path = "/opd-er/document/scan-id/{opd_er_order_master_id}",
    request_body = u8,
    responses(DocOne<ExecuteResponse>),
    params(
        ("opd_er_order_master_id" = u32, Path, description = "OPD-ER Order Master ID", example = "1"),
    ),
)]
pub async fn post_opd_er_document_type(Path(opd_er_order_master_id): Path<u32>, ctx: RequestState, Json(payload): Json<DocumentType>) -> Result<Json<ExecuteResponse>, AppError> {
    ctx.user_state.trace_req_by();
    ctx.authorize_and_access_log(&Method::POST, false).await?;

    let result = document::post_opd_er_document_type(
        opd_er_order_master_id,
        payload as u8,
        &ctx.user_state.user.loginname,
        &ctx.api_state.db_pool,
        &ctx.api_state.kphis_extra(),
    )
    .await?;

    Ok(Json(result))
}

/// /api/opd-er/document/scan-id/{opd_er_order_master_id}
///
/// Tries to delete OPD-ER Document Scan Type
#[utoipa::path(
    delete,
    path = "/opd-er/document/scan-id/{opd_er_order_master_id}",
    request_body = u8,
    responses(DocOne<ExecuteResponse>),
    params(
        ("opd_er_order_master_id" = u32, Path, description = "OPD-ER Order Master ID", example = "1"),
    ),
)]
pub async fn delete_opd_er_document_type(Path(opd_er_order_master_id): Path<u32>, ctx: RequestState, Json(payload): Json<DocumentType>) -> Result<Json<ExecuteResponse>, AppError> {
    ctx.user_state.trace_req_by();
    ctx.authorize_and_access_log(&Method::DELETE, false).await?;

    let result = document::delete_opd_er_document_type(opd_er_order_master_id, payload as u8, &ctx.api_state.db_pool, &ctx.api_state.kphis_extra()).await?;

    Ok(Json(result))
}
