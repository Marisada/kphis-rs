use axum::{Json, extract::Query, http::Method};

use kphis_api_core::{
    open_api::{DocOne, DocVec},
    state::RequestState,
};
use kphis_api_query::ipd::dc_plan_tmp;
use kphis_model::{
    fetch::ExecuteResponse,
    ipd::dc_plan_tmp::{DcPlanTmpDiet, DcPlanTmpDx, DcPlanTmpEnv, DcPlanTmpMed, DcPlanTmpParams, DcPlanTmpTx},
};
use kphis_util::error::AppError;

/// /api/ipd/dc-plan-tmp/dx
///
/// Get IPD Nursing Discharge Plan Diagnosis Template by PARAMS, return list of Nursing Discharge Plan Diagnosis
#[utoipa::path(
    get,
    path = "/ipd/dc-plan-tmp/dx",
    responses(DocVec<DcPlanTmpDx>),
    params(DcPlanTmpParams),
)]
pub async fn get_ipd_dc_plan_tmp_dx(Query(params): Query<DcPlanTmpParams>, ctx: RequestState) -> Result<Json<Vec<DcPlanTmpDx>>, AppError> {
    ctx.user_state.trace_req_by();
    ctx.authorize_and_access_log(&Method::GET, false).await?;

    let response = dc_plan_tmp::get_dx(&params, &ctx.api_state.db_pool, &ctx.api_state.kphis_extra()).await?;

    Ok(Json(response))
}

/// /api/ipd/dc-plan-tmp/dx
///
/// Tries to create/edit Nursing Discharge Plan Diagnosis
/// - Payload's `dx_name` must not null or empty
#[utoipa::path(
    post,
    path = "/ipd/dc-plan-tmp/dx",
    request_body = DcPlanTmpDx,
    responses(DocOne<ExecuteResponse>),
)]
pub async fn post_ipd_dc_plan_tmp_dx(ctx: RequestState, Json(payload): Json<DcPlanTmpDx>) -> Result<Json<ExecuteResponse>, AppError> {
    ctx.user_state.trace_req_by();
    ctx.authorize_and_access_log(&Method::POST, false).await?;

    if payload.dx_name.as_ref().map(|s| s.is_empty()).unwrap_or(true) {
        Err(AppError::app_400("Post DcPlanTmpDx"))
    } else {
        let response = dc_plan_tmp::post_dx(&payload, &ctx.user_state.user.loginname, &ctx.api_state.db_pool, &ctx.api_state.kphis_extra()).await?;

        Ok(Json(response))
    }
}

/// /api/ipd/dc-plan-tmp/dx
///
/// Tries to delete Nursing Discharge Plan Diagnosis by PARAMS
/// - Query parameter `id` must not null
#[utoipa::path(
    delete,
    path = "/ipd/dc-plan-tmp/dx",
    responses(DocOne<ExecuteResponse>),
    params(DcPlanTmpParams),
)]
pub async fn delete_ipd_dc_plan_tmp_dx(Query(params): Query<DcPlanTmpParams>, ctx: RequestState) -> Result<Json<ExecuteResponse>, AppError> {
    ctx.user_state.trace_req_by();
    ctx.authorize_and_access_log(&Method::DELETE, false).await?;

    if params.id.is_some() {
        let response = dc_plan_tmp::delete_dx(&params, &ctx.user_state.user.loginname, &ctx.api_state.db_pool, &ctx.api_state.kphis_extra()).await?;

        Ok(Json(response))
    } else {
        Err(AppError::app_400("Delete DcPlanTmpDx"))
    }
}

/// /api/ipd/dc-plan-tmp/med
///
/// Get IPD Nursing Discharge Plan Medication Template by PARAMS, return list of Nursing Discharge Plan Medication
#[utoipa::path(
    get,
    path = "/ipd/dc-plan-tmp/med",
    responses(DocVec<DcPlanTmpMed>),
    params(DcPlanTmpParams),
)]
pub async fn get_ipd_dc_plan_tmp_med(Query(params): Query<DcPlanTmpParams>, ctx: RequestState) -> Result<Json<Vec<DcPlanTmpMed>>, AppError> {
    ctx.user_state.trace_req_by();
    ctx.authorize_and_access_log(&Method::GET, false).await?;

    let response = dc_plan_tmp::get_med(&params, &ctx.api_state.db_pool, &ctx.api_state.kphis_extra()).await?;

    Ok(Json(response))
}

/// /api/ipd/dc-plan-tmp/med
///
/// Tries to create/edit Nursing Discharge Plan Medication
/// - Payload's `med_text` must not null or empty
#[utoipa::path(
    post,
    path = "/ipd/dc-plan-tmp/med",
    request_body = DcPlanTmpMed,
    responses(DocOne<ExecuteResponse>),
)]
pub async fn post_ipd_dc_plan_tmp_med(ctx: RequestState, Json(payload): Json<DcPlanTmpMed>) -> Result<Json<ExecuteResponse>, AppError> {
    ctx.user_state.trace_req_by();
    ctx.authorize_and_access_log(&Method::POST, false).await?;

    if payload.med_text.as_ref().map(|s| s.is_empty()).unwrap_or(true) {
        Err(AppError::app_400("Post DcPlanTmpMed"))
    } else {
        let response = dc_plan_tmp::post_med(&payload, &ctx.user_state.user.loginname, &ctx.api_state.db_pool, &ctx.api_state.kphis_extra()).await?;

        Ok(Json(response))
    }
}

/// /api/ipd/dc-plan-tmp/med
///
/// Tries to delete Nursing Discharge Plan Medication by PARAMS
/// - Query parameter `id` must not null
#[utoipa::path(
    delete,
    path = "/ipd/dc-plan-tmp/med",
    responses(DocOne<ExecuteResponse>),
    params(DcPlanTmpParams),
)]
pub async fn delete_ipd_dc_plan_tmp_med(Query(params): Query<DcPlanTmpParams>, ctx: RequestState) -> Result<Json<ExecuteResponse>, AppError> {
    ctx.user_state.trace_req_by();
    ctx.authorize_and_access_log(&Method::DELETE, false).await?;

    if params.id.is_some() {
        let response = dc_plan_tmp::delete_med(&params, &ctx.user_state.user.loginname, &ctx.api_state.db_pool, &ctx.api_state.kphis_extra()).await?;

        Ok(Json(response))
    } else {
        Err(AppError::app_400("Delete DcPlanTmpMed"))
    }
}

/// /api/ipd/dc-plan-tmp/env
///
/// Get IPD Nursing Discharge Plan Environment Template by PARAMS, return list of Nursing Discharge Plan Environment
#[utoipa::path(
    get,
    path = "/ipd/dc-plan-tmp/env",
    responses(DocVec<DcPlanTmpEnv>),
    params(DcPlanTmpParams),
)]
pub async fn get_ipd_dc_plan_tmp_env(Query(params): Query<DcPlanTmpParams>, ctx: RequestState) -> Result<Json<Vec<DcPlanTmpEnv>>, AppError> {
    ctx.user_state.trace_req_by();
    ctx.authorize_and_access_log(&Method::GET, false).await?;

    let response = dc_plan_tmp::get_env(&params, &ctx.api_state.db_pool, &ctx.api_state.kphis_extra()).await?;

    Ok(Json(response))
}

/// /api/ipd/dc-plan-tmp/env
///
/// Tries to create/edit Nursing Discharge Plan Environment
/// - Payload's `env_text` must not null or empty
#[utoipa::path(
    post,
    path = "/ipd/dc-plan-tmp/env",
    request_body = DcPlanTmpEnv,
    responses(DocOne<ExecuteResponse>),
)]
pub async fn post_ipd_dc_plan_tmp_env(ctx: RequestState, Json(payload): Json<DcPlanTmpEnv>) -> Result<Json<ExecuteResponse>, AppError> {
    ctx.user_state.trace_req_by();
    ctx.authorize_and_access_log(&Method::POST, false).await?;

    if payload.env_text.as_ref().map(|s| s.is_empty()).unwrap_or(true) {
        Err(AppError::app_400("Post DcPlanTmpEnv"))
    } else {
        let response = dc_plan_tmp::post_env(&payload, &ctx.user_state.user.loginname, &ctx.api_state.db_pool, &ctx.api_state.kphis_extra()).await?;

        Ok(Json(response))
    }
}

/// /api/ipd/dc-plan-tmp/env
///
/// Tries to delete Nursing Discharge Plan Environment by PARAMS
/// - Query parameter `id` must not null
#[utoipa::path(
    delete,
    path = "/ipd/dc-plan-tmp/env",
    responses(DocOne<ExecuteResponse>),
    params(DcPlanTmpParams),
)]
pub async fn delete_ipd_dc_plan_tmp_env(Query(params): Query<DcPlanTmpParams>, ctx: RequestState) -> Result<Json<ExecuteResponse>, AppError> {
    ctx.user_state.trace_req_by();
    ctx.authorize_and_access_log(&Method::DELETE, false).await?;

    if params.id.is_some() {
        let response = dc_plan_tmp::delete_env(&params, &ctx.user_state.user.loginname, &ctx.api_state.db_pool, &ctx.api_state.kphis_extra()).await?;

        Ok(Json(response))
    } else {
        Err(AppError::app_400("Delete DcPlanTmpEnv"))
    }
}

/// /api/ipd/dc-plan-tmp/tx
///
/// Get IPD Nursing Discharge Plan Treatment Template by PARAMS, return list of Nursing Discharge Plan Treatment
#[utoipa::path(
    get,
    path = "/ipd/dc-plan-tmp/tx",
    responses(DocVec<DcPlanTmpTx>),
    params(DcPlanTmpParams),
)]
pub async fn get_ipd_dc_plan_tmp_tx(Query(params): Query<DcPlanTmpParams>, ctx: RequestState) -> Result<Json<Vec<DcPlanTmpTx>>, AppError> {
    ctx.user_state.trace_req_by();
    ctx.authorize_and_access_log(&Method::GET, false).await?;

    let response = dc_plan_tmp::get_tx(&params, &ctx.api_state.db_pool, &ctx.api_state.kphis_extra()).await?;

    Ok(Json(response))
}

/// /api/ipd/dc-plan-tmp/tx
///
/// Tries to create/edit Nursing Discharge Plan Treatment
/// - Payload's `tx_text` must not null or empty
#[utoipa::path(
    post,
    path = "/ipd/dc-plan-tmp/tx",
    request_body = DcPlanTmpTx,
    responses(DocOne<ExecuteResponse>),
)]
pub async fn post_ipd_dc_plan_tmp_tx(ctx: RequestState, Json(payload): Json<DcPlanTmpTx>) -> Result<Json<ExecuteResponse>, AppError> {
    ctx.user_state.trace_req_by();
    ctx.authorize_and_access_log(&Method::POST, false).await?;

    if payload.tx_text.as_ref().map(|s| s.is_empty()).unwrap_or(true) {
        Err(AppError::app_400("Post DcPlanTmpTx"))
    } else {
        let response = dc_plan_tmp::post_tx(&payload, &ctx.user_state.user.loginname, &ctx.api_state.db_pool, &ctx.api_state.kphis_extra()).await?;

        Ok(Json(response))
    }
}

/// /api/ipd/dc-plan-tmp/tx
///
/// Tries to delete Nursing Discharge Plan Treatment by PARAMS
/// - Query parameter `id` must not null
#[utoipa::path(
    delete,
    path = "/ipd/dc-plan-tmp/tx",
    responses(DocOne<ExecuteResponse>),
    params(DcPlanTmpParams),
)]
pub async fn delete_ipd_dc_plan_tmp_tx(Query(params): Query<DcPlanTmpParams>, ctx: RequestState) -> Result<Json<ExecuteResponse>, AppError> {
    ctx.user_state.trace_req_by();
    ctx.authorize_and_access_log(&Method::DELETE, false).await?;

    if params.id.is_some() {
        let response = dc_plan_tmp::delete_tx(&params, &ctx.user_state.user.loginname, &ctx.api_state.db_pool, &ctx.api_state.kphis_extra()).await?;

        Ok(Json(response))
    } else {
        Err(AppError::app_400("Delete DcPlanTmpTx"))
    }
}

/// /api/ipd/dc-plan-tmp/diet
///
/// Get IPD Nursing Discharge Plan Diet Template by PARAMS, return list of Nursing Discharge Plan Diet
#[utoipa::path(
    get,
    path = "/ipd/dc-plan-tmp/diet",
    responses(DocVec<DcPlanTmpDiet>),
    params(DcPlanTmpParams),
)]
pub async fn get_ipd_dc_plan_tmp_diet(Query(params): Query<DcPlanTmpParams>, ctx: RequestState) -> Result<Json<Vec<DcPlanTmpDiet>>, AppError> {
    ctx.user_state.trace_req_by();
    ctx.authorize_and_access_log(&Method::GET, false).await?;

    let response = dc_plan_tmp::get_diet(&params, &ctx.api_state.db_pool, &ctx.api_state.kphis_extra()).await?;

    Ok(Json(response))
}

/// /api/ipd/dc-plan-tmp/diet
///
/// Tries to create/edit Nursing Discharge Plan Diet
/// - Payload's `diet_text` must not null or empty
#[utoipa::path(
    post,
    path = "/ipd/dc-plan-tmp/diet",
    request_body = DcPlanTmpDiet,
    responses(DocOne<ExecuteResponse>),
)]
pub async fn post_ipd_dc_plan_tmp_diet(ctx: RequestState, Json(payload): Json<DcPlanTmpDiet>) -> Result<Json<ExecuteResponse>, AppError> {
    ctx.user_state.trace_req_by();
    ctx.authorize_and_access_log(&Method::POST, false).await?;

    if payload.diet_text.as_ref().map(|s| s.is_empty()).unwrap_or(true) {
        Err(AppError::app_400("Post DcPlanTmpDiet"))
    } else {
        let response = dc_plan_tmp::post_diet(&payload, &ctx.user_state.user.loginname, &ctx.api_state.db_pool, &ctx.api_state.kphis_extra()).await?;

        Ok(Json(response))
    }
}

/// /api/ipd/dc-plan-tmp/diet
///
/// Tries to delete Nursing Discharge Plan Diet by PARAMS
/// - Query parameter `id` must not null
#[utoipa::path(
    delete,
    path = "/ipd/dc-plan-tmp/diet",
    responses(DocOne<ExecuteResponse>),
    params(DcPlanTmpParams),
)]
pub async fn delete_ipd_dc_plan_tmp_diet(Query(params): Query<DcPlanTmpParams>, ctx: RequestState) -> Result<Json<ExecuteResponse>, AppError> {
    ctx.user_state.trace_req_by();
    ctx.authorize_and_access_log(&Method::DELETE, false).await?;

    if params.id.is_some() {
        let response = dc_plan_tmp::delete_diet(&params, &ctx.user_state.user.loginname, &ctx.api_state.db_pool, &ctx.api_state.kphis_extra()).await?;

        Ok(Json(response))
    } else {
        Err(AppError::app_400("Delete DcPlanTmpDiet"))
    }
}
