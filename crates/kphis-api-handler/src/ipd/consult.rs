use axum::{
    Json,
    extract::{Path, Query},
    http::Method,
};

use kphis_api_core::{
    open_api::{DocOpt, DocVec, DocVecU32},
    state::RequestState,
};
use kphis_api_query::{ipd::consult, transform::query::check_an_can_execute};
use kphis_model::{
    fetch::ExecuteResponse,
    ipd::consult::{Consult, ConsultParams, ConsultSave, ConsultWithName, IpdConsultList, IpdConsultListParams},
};
use kphis_util::error::AppError;

// ipd-consult-list-table.php
/// /api/ipd/consult
///
/// Get IPD Consult by PARAMS, return list of IPD Consult
#[utoipa::path(
    get,
    path = "/ipd/consult",
    responses(DocVec<IpdConsultList>),
    params(IpdConsultListParams),
)]
pub async fn get_ipd_consult_list(Query(params): Query<IpdConsultListParams>, ctx: RequestState) -> Result<Json<Vec<IpdConsultList>>, AppError> {
    ctx.user_state.trace_req_by();
    ctx.authorize_and_access_log(&Method::GET, false).await?;

    let response = consult::get_ipd_consult_list(
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

// ipd-dr-consult-data.php
/// /api/ipd/consult-an/{an}
///
/// Get IPD Consult by AN, return list of IPD Consult
#[utoipa::path(
    get,
    path = "/ipd/consult-an/{an}",
    responses(DocVec<ConsultWithName>),
    params(
        ("an" = String, Path, description = "Admission Number: AN", example = "660001234")
    ),
)]
pub async fn get_ipd_consult_by_an(Path(an): Path<String>, ctx: RequestState) -> Result<Json<Vec<ConsultWithName>>, AppError> {
    ctx.user_state.trace_req_by();
    let is_pre_admit = ctx.api_state.is_pre_admit(&an);
    ctx.authorize_and_access_log(&Method::GET, is_pre_admit).await?;

    let response = consult::get_ipd_consult_by_an(&an, &ctx.api_state.db_pool, &ctx.api_state.hosxp(), &ctx.api_state.kphis(), &ctx.api_state.kphis_extra()).await?;

    Ok(Json(response))
}

// ipd-dr-consult-edit.php
/// /api/ipd/consult-id/{consult_id}
///
/// Get IPD Consult by ID, return single of IPD Consult or none
#[utoipa::path(
    get,
    path = "/ipd/consult-id/{consult_id}",
    responses(DocOpt<Consult>),
    params(
        ("consult_id" = u32, Path, description = "Consult ID", example = 1)
    ),
)]
pub async fn get_ipd_consult_by_id(Path(consult_id): Path<u32>, ctx: RequestState) -> Result<Json<Option<Consult>>, AppError> {
    ctx.user_state.trace_req_by();
    ctx.authorize_and_access_log(&Method::GET, false).await?;

    let response = consult::get_ipd_consult_by_id(consult_id, &ctx.api_state.db_pool, &ctx.api_state.hosxp(), &ctx.api_state.kphis()).await?;

    Ok(Json(response))
}

// ipd-dr-consult-save.php
// ipd-dr-consult-update.php
/// /api/ipd/consult
///
/// Tries to create/edit IPD Consult
/// - Payload's `an` must not empty
/// - Checking `an` is `pre-admit` by `an` length
#[utoipa::path(
    post,
    path = "/ipd/consult",
    request_body = ConsultSave,
    responses(DocVecU32<ExecuteResponse>),
)]
pub async fn post_ipd_consult(ctx: RequestState, Json(payload): Json<ConsultSave>) -> Result<Json<(u32, Vec<ExecuteResponse>)>, AppError> {
    ctx.user_state.trace_req_by();
    let is_pre_admit = ctx.api_state.is_pre_admit(&payload.an);
    ctx.authorize_and_access_log(&Method::POST, is_pre_admit).await?;

    // check AN is valid (pre-admit was admited or admit was revoked)
    check_an_can_execute(&payload.an, ctx.api_state.hosxp_an_len(), &ctx.api_state.db_pool, &ctx.api_state.hosxp(), &ctx.api_state.kphis()).await?;

    let response = consult::post_ipd_consult(&payload, &ctx.user_state.user.loginname, &ctx.api_state.db_pool, &ctx.api_state.kphis(), &ctx.api_state.kphis_log()).await?;

    Ok(Json(response))
}

// ipd-dr-consult-delete.php
/// /api/ipd/consult
///
/// Tries to delete IPD Consult by PARAMS
/// - Query parameter `consult_id` and `version` must not null
#[utoipa::path(
    delete,
    path = "/ipd/consult",
    responses(DocVec<ExecuteResponse>),
    params(ConsultParams),
)]
pub async fn delete_ipd_consult_by_id(Query(params): Query<ConsultParams>, ctx: RequestState) -> Result<Json<Vec<ExecuteResponse>>, AppError> {
    ctx.user_state.trace_req_by();
    ctx.authorize_and_access_log(&Method::DELETE, false).await?;

    if let (Some(consult_id), Some(version)) = (params.consult_id, params.version) {
        let response = consult::delete_ipd_consult_by_id(
            consult_id,
            version,
            &ctx.user_state.user.loginname,
            &ctx.api_state.db_pool,
            &ctx.api_state.kphis(),
            &ctx.api_state.kphis_log(),
        )
        .await?;

        Ok(Json(response))
    } else {
        Err(AppError::app_400("Delete IpdConsult"))
    }
}
