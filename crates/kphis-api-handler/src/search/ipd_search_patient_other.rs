use axum::{Json, extract::Query, http::Method};

use kphis_api_core::{open_api::DocVec, state::RequestState};
use kphis_api_query::search::ipd_search_patient_other;
use kphis_model::search::ipd_search_patient_other::{IpdSearchPatientOtherRequest, IpdSearchPatientOtherResponse};
use kphis_util::error::AppError;

// ipd-other-search-patient-table.php
/// /api/search/other
///
/// Get IPD Search Patient result for Other user by PARAMS, return a list of IPD Search Patient result for Other user
#[utoipa::path(
    get,
    path = "/search/other",
    responses(DocVec<IpdSearchPatientOtherResponse>),
    params(IpdSearchPatientOtherRequest),
)]
pub async fn get_ipd_other_search_patient(Query(params): Query<IpdSearchPatientOtherRequest>, ctx: RequestState) -> Result<Json<Vec<IpdSearchPatientOtherResponse>>, AppError> {
    ctx.user_state.trace_req_by();
    ctx.authorize_and_access_log(&Method::GET, false).await?;

    let response = ipd_search_patient_other::get_ipd_other_search_patient(
        params,
        ctx.api_state.hosxp_hn_len(),
        ctx.api_state.hosxp_an_len(),
        &ctx.api_state.db_pool,
        &ctx.api_state.hosxp(),
        &ctx.api_state.kphis(),
    )
    .await?;

    Ok(Json(response))
}
