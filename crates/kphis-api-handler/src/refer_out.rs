use axum::{Json, extract::Path, http::Method};

use kphis_api_core::{open_api::DocVec, state::RequestState};
use kphis_api_query::refer_out;
use kphis_model::{
    fetch::ExecuteResponse,
    refer_out::{HisReferOutData, HisReferOutSave},
};
use kphis_util::error::AppError;

/// /api/his/refer-out-vnan/{vnan}
///
/// Get Refer-Out data from HIS by VN or AN, return list of Refer-Out data
#[utoipa::path(
    get,
    path = "/his/refer-out-vnan/{vnan}",
    responses(DocVec<HisReferOutData>),
    params(
        ("vnan" = String, Path, description = "Visit Number or Admit Number", example = "660001234"),
    ),
)]
pub async fn get_his_referout_data(Path(vnan): Path<String>, ctx: RequestState) -> Result<Json<Vec<HisReferOutData>>, AppError> {
    ctx.user_state.trace_req_by();
    ctx.authorize_and_access_log(&Method::GET, false).await?;

    let response = refer_out::select_his_referout_data(&vnan, &ctx.api_state.db_pool, &ctx.api_state.hosxp()).await?;

    Ok(Json(response))
}

/// /api/his/refer-out-vnan/{vnan}
///
/// Tries to insert or update Refer-Out data in HIS
#[utoipa::path(
    post,
    path = "/his/refer-out-vnan/{vnan}",
    request_body = HisReferOutSave,
    responses(DocVec<ExecuteResponse>),
    params(
        ("vnan" = String, Path, description = "Visit Number or Admit Number", example = "660001234"),
    ),
)]
pub async fn post_his_referout(Path(vnan): Path<String>, ctx: RequestState, Json(payload): Json<HisReferOutSave>) -> Result<Json<Vec<ExecuteResponse>>, AppError> {
    ctx.user_state.trace_req_by();
    ctx.authorize_and_access_log(&Method::POST, false).await?;

    if payload.vn == vnan {
        let results = refer_out::post_his_referout(&payload, &ctx.user_state.user.doctorcode, &ctx.api_state.db_pool, &ctx.api_state.hosxp()).await?;

        Ok(Json(results))
    } else {
        Err(AppError::app_400("Post HisReferOut"))
    }
}
