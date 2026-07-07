use axum::{Json, extract::Query, http::Method};

use kphis_api_core::{open_api::DocVec, state::RequestState};
use kphis_api_query::image::scan_his;
use kphis_model::image::scan_his::{ScanImage, ScanImageParams};
use kphis_util::error::AppError;

/// /api/scan/his/image
///
/// Get Scan Image from HIS by Key (opd, pe, er, lab) and VN, return a list of Scan Image
/// - Query parameter `vn` must not null, `key` must be `opd` or `er` or `pe`, `lab`
#[utoipa::path(
    get,
    path = "/scan/his/image",
    responses(DocVec<ScanImage>),
    params(ScanImageParams),
)]
pub async fn get_scan_his_image(Query(params): Query<ScanImageParams>, ctx: RequestState) -> Result<Json<Vec<ScanImage>>, AppError> {
    ctx.user_state.trace_req_by();
    ctx.authorize_and_access_log(&Method::GET, false).await?;

    if let Some(key) = params.key.clone() {
        if params.vn.is_some() && ["opd", "er", "pe", "lab"].contains(&key.as_str()) {
            let response = scan_his::get_scan_his_image(&params, &ctx.api_state.db_pool, &ctx.api_state.hosxp()).await?;

            Ok(Json(response))
        } else {
            Err(AppError::app_400("Select ScanHisImage"))
        }
    } else {
        Err(AppError::app_400("Select ScanHisImage"))
    }
}
