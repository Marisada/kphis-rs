use axum::{Json, extract::Query, http::Method};

use kphis_api_core::{open_api::DocVec, state::RequestState};
use kphis_api_query::search::ipd_search_patient_pharmacist;
use kphis_model::search::ipd_search_patient_pharmacist::{IpdSearchPatientPharmacistRequest, IpdSearchPatientPharmacistResponse};
use kphis_util::error::AppError;

// ipd-pharmacy-search-patient-table.php
/// /api/search/pharmacist
///
/// Get IPD Search Patient result for Pharmacist by PARAMS, return a list of IPD Search Patient result for Pharmacist
#[utoipa::path(
    get,
    path = "/search/pharmacist",
    responses(DocVec<IpdSearchPatientPharmacistResponse>),
    params(IpdSearchPatientPharmacistRequest),
)]
pub async fn get_ipd_pharmacist_search_patient(Query(params): Query<IpdSearchPatientPharmacistRequest>, ctx: RequestState) -> Result<Json<Vec<IpdSearchPatientPharmacistResponse>>, AppError> {
    ctx.user_state.trace_req_by();
    ctx.authorize_and_access_log(&Method::GET, false).await?;

    let response = ipd_search_patient_pharmacist::get_ipd_pharmacist_search_patient(
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
