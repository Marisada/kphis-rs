use axum::{Json, extract::Query, http::Method};

use kphis_api_core::{
    open_api::{DocVec, DocVecU32},
    state::RequestState,
};
use kphis_api_query::{ipd::doctor_in_charge, transform::query::check_an_opt_can_execute};
use kphis_model::{
    fetch::ExecuteResponse,
    ipd::doctor_in_charge::{DoctorInChargeParams, IpdDoctorInCharge},
};
use kphis_util::error::AppError;

// ipd-nurse-doctor-in-charge-table.php
/// /api/ipd/doctor-in-charge
///
/// Get IPD Doctor who care this patient by PARAMS, return list of Doctor data
///
/// Require AN in PARAMS
#[utoipa::path(
    get,
    path = "/ipd/doctor-in-charge",
    responses(DocVec<IpdDoctorInCharge>),
    params(DoctorInChargeParams),
)]
pub async fn get_ipd_doctor_in_charge(Query(params): Query<DoctorInChargeParams>, ctx: RequestState) -> Result<Json<Vec<IpdDoctorInCharge>>, AppError> {
    ctx.user_state.trace_req_by();
    let is_pre_admit = ctx.api_state.is_pre_admit_opt(&params.an);
    ctx.authorize_and_access_log(&Method::GET, is_pre_admit).await?;

    if let Some(an) = params.an {
        let responses = doctor_in_charge::get_doctor_in_charge(&an, &ctx.api_state.db_pool, &ctx.api_state.hosxp(), &ctx.api_state.kphis()).await?;

        Ok(Json(responses))
    } else {
        Ok(Json(Vec::new()))
    }
}

// ipd-nurse-doctor-in-charge-save.php
// ipd-nurse-doctor-in-charge-update.php
/// /api/ipd/doctor-in-charge
///
/// Tries to create/edit IPD Doctor in-charge
/// - Payload's `an` must not null or empty
/// - Checking `an` is `pre-admit` by `an` length
#[utoipa::path(
    post,
    path = "/ipd/doctor-in-charge",
    request_body = IpdDoctorInCharge,
    responses(DocVecU32<ExecuteResponse>),
)]
pub async fn post_ipd_doctor_in_charge(ctx: RequestState, Json(payload): Json<IpdDoctorInCharge>) -> Result<Json<(u32, Vec<ExecuteResponse>)>, AppError> {
    ctx.user_state.trace_req_by();
    let is_pre_admit = ctx.api_state.is_pre_admit_opt(&payload.an);
    ctx.authorize_and_access_log(&Method::POST, is_pre_admit).await?;

    // check AN is valid (pre-admit was admited or admit was revoked)
    check_an_opt_can_execute(&payload.an, ctx.api_state.hosxp_an_len(), &ctx.api_state.db_pool, &ctx.api_state.hosxp(), &ctx.api_state.kphis()).await?;

    let result = doctor_in_charge::post_doctor_in_charge(&payload, &ctx.user_state.user.loginname, &ctx.api_state.db_pool, &ctx.api_state.kphis(), &ctx.api_state.kphis_log()).await?;

    Ok(Json(result))
}

// ipd-nurse-doctor-in-charge-delete.php
/// /api/ipd/doctor-in-charge
///
/// Tries to delete IPD Doctor in-charge by PARAMS
/// - Query parameter `doctor_in_charge_id` and `version` must not null
#[utoipa::path(
    delete,
    path = "/ipd/doctor-in-charge",
    responses(DocVec<ExecuteResponse>),
    params(DoctorInChargeParams),
)]
pub async fn delete_ipd_doctor_in_charge(Query(params): Query<DoctorInChargeParams>, ctx: RequestState) -> Result<Json<Vec<ExecuteResponse>>, AppError> {
    ctx.user_state.trace_req_by();
    let is_pre_admit = ctx.api_state.is_pre_admit_opt(&params.an);
    ctx.authorize_and_access_log(&Method::DELETE, is_pre_admit).await?;

    if let (Some(doctor_in_charge_id), Some(version)) = (params.doctor_in_charge_id, params.version) {
        let result = doctor_in_charge::delete_doctor_in_charge(
            doctor_in_charge_id,
            version,
            &ctx.user_state.user.loginname,
            &ctx.api_state.db_pool,
            &ctx.api_state.kphis(),
            &ctx.api_state.kphis_log(),
        )
        .await?;

        Ok(Json(result))
    } else {
        Err(AppError::app_400("Delete IpdDoctorInCharge"))
    }
}
