use axum::{Json, extract::Path, http::Method};

use kphis_api_core::{
    open_api::{DocOne, DocVec},
    state::RequestState,
};
use kphis_api_query::xray;
use kphis_model::{fetch::ExecuteResponse, xray::XrayReport};
use kphis_util::error::AppError;

/// /api/xray/report-hn/{hn}
///
/// Get Xray Reports by HN, return list of Xray Report
#[utoipa::path(
    get,
    path = "/xray/report-hn/{hn}",
    responses(DocVec<XrayReport>),
    params(
        ("hn" = String, Path, description = "Hospital Number", example = "0001234"),
    ),
)]
pub async fn get_xray_report(Path(hn): Path<String>, ctx: RequestState) -> Result<Json<Vec<XrayReport>>, AppError> {
    ctx.user_state.trace_req_by();
    ctx.authorize_and_access_log(&Method::GET, false).await?;

    let response = xray::get_xray_report(&hn, &ctx.api_state.db_pool, &ctx.api_state.hosxp(), &ctx.api_state.kphis()).await?;

    Ok(Json(response))
}

/// /api/xray/read-id/{xn}
///
/// Tries to mark Xray Report as readed
#[utoipa::path(
    post,
    path = "/xray/read-id/{xn}",
    responses(DocOne<ExecuteResponse>),
    params(
        ("xn" = i32, Path, description = "Xray Number", example = "1")
    ),
)]
pub async fn post_xray_read(Path(xn): Path<i32>, ctx: RequestState) -> Result<Json<ExecuteResponse>, AppError> {
    ctx.user_state.trace_req_by();
    ctx.authorize_and_access_log(&Method::POST, false).await?;

    let result = xray::post_xray_read(xn, &ctx.user_state.user.loginname, &ctx.api_state.db_pool, &ctx.api_state.kphis()).await?;

    Ok(Json(result))
}

/// /api/xray/read-id/{xn}
///
/// Tries to remove mark of Xray Report as readed
#[utoipa::path(
    delete,
    path = "/xray/read-id/{xn}",
    responses(DocOne<ExecuteResponse>),
    params(
        ("xn" = i32, Path, description = "Xray Number", example = "1")
    ),
)]
pub async fn delete_xray_read(Path(xn): Path<i32>, ctx: RequestState) -> Result<Json<ExecuteResponse>, AppError> {
    ctx.user_state.trace_req_by();
    ctx.authorize_and_access_log(&Method::DELETE, false).await?;

    let result = xray::delete_xray_read(xn, &ctx.api_state.db_pool, &ctx.api_state.kphis()).await?;

    Ok(Json(result))
}
