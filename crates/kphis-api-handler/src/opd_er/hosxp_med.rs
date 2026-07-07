use axum::{Json, extract::Path, http::Method};

use kphis_api_core::{open_api::DocVec, state::RequestState};
use kphis_api_query::opd_er::hosxp_med;
use kphis_model::opd_er::hosxp_med::OpdMed;
use kphis_util::error::AppError;

// opd-er-hosxp-med-data.php
/// /api/opd-er/his-med-vn/{vn}
///
/// Get list of OPD-ER Focus Note by ID and PARAMS, return list of OPD-ER Focus Note
#[utoipa::path(
    get,
    path = "/opd-er/his-med-vn/{vn}",
    responses(DocVec<OpdMed>),
    params(
        ("vn" = String, Path, description = "Visit Number: VN", example = "661231235959"),
    ),
)]
pub async fn get_opd_med(Path(vn): Path<String>, ctx: RequestState) -> Result<Json<Vec<OpdMed>>, AppError> {
    ctx.user_state.trace_req_by();
    ctx.authorize_and_access_log(&Method::GET, false).await?;

    let response = hosxp_med::get_opd_med(&vn, &ctx.api_state.db_pool, &ctx.api_state.hosxp()).await?;

    Ok(Json(response))
}
