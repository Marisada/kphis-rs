use axum::{Json, extract::Query, http::Method};

use kphis_api_core::{open_api::DocVec, state::RequestState};
use kphis_api_query::search::ipd_search_patient_dr;
use kphis_model::search::ipd_search_patient_dr::{IpdSearchPatientDrRequest, IpdSearchPatientDrResponse};
use kphis_util::error::AppError;

// ipd-dr-search-patient-table.php
/// /api/search/dr
///
/// Get IPD Search Patient result for Doctor by PARAMS, return a list of IPD Search Patient result for Doctor
#[utoipa::path(
    get,
    path = "/search/dr",
    responses(DocVec<IpdSearchPatientDrResponse>),
    params(IpdSearchPatientDrRequest),
)]
pub async fn get_ipd_dr_search_patient(Query(params): Query<IpdSearchPatientDrRequest>, ctx: RequestState) -> Result<Json<Vec<IpdSearchPatientDrResponse>>, AppError> {
    ctx.user_state.trace_req_by();
    ctx.authorize_and_access_log(&Method::GET, false).await?;

    let response = ipd_search_patient_dr::get_ipd_dr_search_patient(
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
