use axum::{Json, extract::Query, http::Method};

use kphis_api_core::{open_api::DocVec, state::RequestState};
use kphis_api_query::search::ipd_search_patient_nurse;
use kphis_model::search::ipd_search_patient_nurse::{IpdSearchPatientNurseRequest, IpdSearchPatientNurseResponse};
use kphis_util::error::AppError;

// ipd-nurse-search-patient-table.php
/// /api/search/nurse
///
/// Get IPD Search Patient result for Nurse by PARAMS, return a list of IPD Search Patient result for Nurse
#[utoipa::path(
    get,
    path = "/search/nurse",
    responses(DocVec<IpdSearchPatientNurseResponse>),
    params(IpdSearchPatientNurseRequest),
)]
pub async fn get_ipd_nurse_search_patient(Query(params): Query<IpdSearchPatientNurseRequest>, ctx: RequestState) -> Result<Json<Vec<IpdSearchPatientNurseResponse>>, AppError> {
    ctx.user_state.trace_req_by();
    ctx.authorize_and_access_log(&Method::GET, false).await?;

    let response = ipd_search_patient_nurse::get_ipd_nurse_search_patient(
        params,
        ctx.api_state.hosxp_hn_len(),
        ctx.api_state.hosxp_an_len(),
        &ctx.api_state.db_pool,
        &ctx.api_state.hosxp(),
        &ctx.api_state.kphis(),
        &ctx.api_state.kphis_extra(),
    )
    .await?;

    Ok(Json(response))
}
