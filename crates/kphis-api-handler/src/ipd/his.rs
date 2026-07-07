use axum::{Json, extract::Path, http::Method};

use kphis_api_core::{open_api::DocVec, state::RequestState};
use kphis_api_query::ipd::his;
use kphis_model::ipd::his::{HisIptDiag, HisIptOprt, HisMedPlanIpd, HisOperationAdmit};
use kphis_util::error::AppError;

/// /api/his/operation-admit-an/{an}
///
/// Get list of HIS's operation by AN, return list of Operation
#[utoipa::path(
    get,
    path = "/his/operation-admit-an/{an}",
    responses(DocVec<HisOperationAdmit>),
    params(
        ("an" = String, Path, description = "Admission Number: AN", example = "660001234")
    ),
)]
pub async fn get_ipd_his_opertion_admit(Path(an): Path<String>, ctx: RequestState) -> Result<Json<Vec<HisOperationAdmit>>, AppError> {
    ctx.user_state.trace_req_by();
    ctx.authorize_and_access_log(&Method::GET, false).await?;

    let ops = his::get_operation_admit(&an, &ctx.api_state.operation_success(), &ctx.api_state.db_pool, &ctx.api_state.hosxp()).await?;

    Ok(Json(ops))
}

/// /api/his/med-plan-ipd-an/{an}
///
/// Get list of active HIS's MedPlanIpd by AN, return list of MedPlanIpd
#[utoipa::path(
    get,
    path = "/his/med-plan-ipd-an/{an}",
    responses(DocVec<HisMedPlanIpd>),
    params(
        ("an" = String, Path, description = "Admission Number: AN", example = "660001234")
    ),
)]
pub async fn get_med_plan_ipd_remains(Path(an): Path<String>, ctx: RequestState) -> Result<Json<Vec<HisMedPlanIpd>>, AppError> {
    ctx.user_state.trace_req_by();
    ctx.authorize_and_access_log(&Method::GET, false).await?;

    let ops = his::get_medplan_ipd_remains(&an, &ctx.api_state.db_pool, &ctx.api_state.hosxp()).await?;

    Ok(Json(ops))
}

/// /api/his/ipt-diag-an/{an}
///
/// Get list of HIS's Summary's Diagnoses by AN, return list of Summary Diagnosis
#[utoipa::path(
    get,
    path = "/his/ipt-diag-an/{an}",
    responses(DocVec<HisIptDiag>),
    params(
        ("an" = String, Path, description = "Admission Number: AN", example = "660001234")
    ),
)]
pub async fn get_his_ipt_diag(Path(an): Path<String>, ctx: RequestState) -> Result<Json<Vec<HisIptDiag>>, AppError> {
    ctx.user_state.trace_req_by();
    ctx.authorize_and_access_log(&Method::GET, false).await?;

    let results = his::get_ipt_diag(&an, &ctx.api_state.db_pool, &ctx.api_state.hosxp()).await?;

    Ok(Json(results))
}

/// /api/his/ipt-oprt-an/{an}
///
/// Get list of HIS's Summary's Procedures by AN, return list of Summary Procedures
#[utoipa::path(
    get,
    path = "/his/ipt-oprt-an/{an}",
    responses(DocVec<HisIptOprt>),
    params(
        ("an" = String, Path, description = "Admission Number: AN", example = "660001234")
    ),
)]
pub async fn get_his_ipt_oprt(Path(an): Path<String>, ctx: RequestState) -> Result<Json<Vec<HisIptOprt>>, AppError> {
    ctx.user_state.trace_req_by();
    ctx.authorize_and_access_log(&Method::GET, false).await?;

    let results = his::get_ipt_oprt(&an, &ctx.api_state.db_pool, &ctx.api_state.hosxp()).await?;

    Ok(Json(results))
}
