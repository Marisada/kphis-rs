use axum::{Json, extract::Path, http::Method};

use kphis_api_core::{
    open_api::{DocOpt, DocVec},
    state::RequestState,
};
use kphis_api_query::emr;
use kphis_model::emr::{EmrDate, EmrVisit};
use kphis_util::error::AppError;

/// /api/emr/date-hn/{hn}
///
/// Get dates of EMR by HN, return list of date
#[utoipa::path(
    get,
    path = "/emr/date-hn/{hn}",
    responses(DocVec<EmrDate>),
    params(
        ("hn" = String, Path, description = "Hospital Number: HN", example = "0001234")
    ),
)]
pub async fn get_emr_date(Path(hn): Path<String>, ctx: RequestState) -> Result<Json<Vec<EmrDate>>, AppError> {
    ctx.user_state.trace_req_by();
    ctx.authorize_and_access_log(&Method::GET, false).await?;

    let response = emr::get_emr_date(&hn, &ctx.api_state.db_pool, &ctx.api_state.hosxp(), &ctx.api_state.kphis()).await?;

    Ok(Json(response))
}

/// /api/emr/visit-vn/{vn}
///
/// Get EMR visit by VN, return single EMR visit or none
#[utoipa::path(
    get,
    path = "/emr/visit-vn/{vn}",
    responses(DocOpt<EmrVisit>),
    params(
        ("vn" = String, Path, description = "Visit Number: VN", example = "661231235959")
    ),
)]
pub async fn get_emr_visit(Path(vn): Path<String>, ctx: RequestState) -> Result<Json<Option<EmrVisit>>, AppError> {
    ctx.user_state.trace_req_by();
    ctx.authorize_and_access_log(&Method::GET, false).await?;

    let response = emr::get_emr_visit(&vn, &ctx.api_state.db_pool, &ctx.api_state.hosxp()).await?;

    Ok(Json(response))
}
