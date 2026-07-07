use axum::{Json, extract::Query, http::Method};

use kphis_api_core::{
    open_api::{DocOne, DocVec},
    state::RequestState,
};
use kphis_api_query::{ipd::mra, transform::query::check_an_can_execute};
use kphis_model::{
    fetch::ExecuteResponse,
    ipd::mra::{IpdMra, MraParams},
};
use kphis_util::error::AppError;

/// /api/ipd/mra
///
/// Get IPD Medical Record Audit(MRA) by PARAMS, return list of IPD MRA
#[utoipa::path(
    get,
    path = "/ipd/mra",
    responses(DocVec<IpdMra>),
    params(MraParams),
)]
pub async fn get_ipd_mra(Query(params): Query<MraParams>, ctx: RequestState) -> Result<Json<Vec<IpdMra>>, AppError> {
    ctx.user_state.trace_req_by();
    let is_pre_admit = ctx.api_state.is_pre_admit_opt(&params.an);
    ctx.authorize_and_access_log(&Method::GET, is_pre_admit).await?;

    let response = mra::get_ipd_mra(&params, &ctx.api_state.db_pool, &ctx.api_state.kphis_extra()).await?;

    Ok(Json(response))
}

/// /api/ipd/mra
///
/// Tries to create a new IPD Medical Record Audit(MRA)
/// - Payload's `an` must not empty
/// - Checking `an` is `pre-admit` by `an` length
#[utoipa::path(
    post,
    path = "/ipd/mra",
    request_body = IpdMra,
    responses(DocOne<ExecuteResponse>),
)]
pub async fn post_ipd_mra(ctx: RequestState, Json(payload): Json<IpdMra>) -> Result<Json<ExecuteResponse>, AppError> {
    ctx.user_state.trace_req_by();
    let is_pre_admit = ctx.api_state.is_pre_admit(&payload.an);
    ctx.authorize_and_access_log(&Method::POST, is_pre_admit).await?;

    // check AN is valid (pre-admit was admited or admit was revoked)
    check_an_can_execute(&payload.an, ctx.api_state.hosxp_an_len(), &ctx.api_state.db_pool, &ctx.api_state.hosxp(), &ctx.api_state.kphis()).await?;

    let result = mra::post_ipd_mra(&payload, &ctx.user_state.user.loginname, &ctx.api_state.db_pool, &ctx.api_state.kphis_extra()).await?;

    Ok(Json(ExecuteResponse::from_query_result(result, "Post IpdMra")))
}

/// /api/ipd/mra
///
/// Tries to edit IPD Medical Record Audit(MRA)
/// - Payload's `an` must not empty
/// - Checking `an` is `pre-admit` by `an` length
#[utoipa::path(
    put,
    path = "/ipd/mra",
    request_body = IpdMra,
    responses(DocOne<ExecuteResponse>),
)]
pub async fn put_ipd_mra(ctx: RequestState, Json(payload): Json<IpdMra>) -> Result<Json<ExecuteResponse>, AppError> {
    ctx.user_state.trace_req_by();
    let is_pre_admit = ctx.api_state.is_pre_admit(&payload.an);
    ctx.authorize_and_access_log(&Method::PUT, is_pre_admit).await?;

    // check AN is valid (pre-admit was admited or admit was revoked)
    check_an_can_execute(&payload.an, ctx.api_state.hosxp_an_len(), &ctx.api_state.db_pool, &ctx.api_state.hosxp(), &ctx.api_state.kphis()).await?;

    let result = mra::put_ipd_mra(&payload, &ctx.user_state.user.loginname, &ctx.api_state.db_pool, &ctx.api_state.kphis_extra()).await?;

    Ok(Json(ExecuteResponse::from_query_result(result, "Put IpdMra")))
}

/// /api/ipd/mra
///
/// Tries to delete IPD Medical Record Audit(MRA)
/// - Query parameter `mra_id` must not null
#[utoipa::path(
    delete,
    path = "/ipd/mra",
    responses(DocOne<ExecuteResponse>),
    params(MraParams),
)]
pub async fn delete_ipd_mra(Query(params): Query<MraParams>, ctx: RequestState) -> Result<Json<ExecuteResponse>, AppError> {
    ctx.user_state.trace_req_by();
    let is_pre_admit = ctx.api_state.is_pre_admit_opt(&params.an);
    ctx.authorize_and_access_log(&Method::DELETE, is_pre_admit).await?;

    if let Some(mra_id) = params.mra_id {
        let response = mra::delete_ipd_mra(mra_id, &ctx.api_state.db_pool, &ctx.api_state.kphis_extra())
            .await
            .map(|result| ExecuteResponse::from_query_result(result, "Delete IpdMra"))?;

        Ok(Json(response))
    } else {
        Err(AppError::app_400("Delete IpdMra"))
    }
}
