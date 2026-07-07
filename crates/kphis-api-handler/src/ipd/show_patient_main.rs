use axum::{Json, extract::Path, http::Method};

use kphis_api_core::{open_api::DocOpt, state::RequestState};
use kphis_api_query::ipd::show_patient_main;
use kphis_model::patient_info::PatientInfo;
use kphis_util::error::AppError;

// ipd-show-patient-main.php
/// /api/ipd/show-patient-main-an/{an}
///
/// Get IPD Patient Header by AN, return single IPD Patient Header
#[utoipa::path(
    get,
    path = "/ipd/show-patient-main-an/{an}",
    responses(DocOpt<PatientInfo>),
    params(
        ("an" = String, Path, description = "Admission Number: AN", example = "660001234"),
    ),
)]
pub async fn get_ipd_show_patient_main(Path(an): Path<String>, ctx: RequestState) -> Result<Json<Option<PatientInfo>>, AppError> {
    ctx.user_state.trace_req_by();
    let is_pre_admit = ctx.api_state.is_pre_admit(&an);
    ctx.authorize_and_access_log(&Method::GET, is_pre_admit).await?;

    let response = show_patient_main::get_show_patient_main(&an, &ctx.api_state.db_pool, &ctx.api_state.hosxp(), &ctx.api_state.kphis()).await?;

    Ok(Json(response))
}
