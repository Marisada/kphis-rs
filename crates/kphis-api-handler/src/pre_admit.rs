use axum::{Json, extract::Query, http::Method};
use kphis_api_core::{
    open_api::{DocOne, DocVec},
    state::RequestState,
};
use kphis_api_query::pre_admit;
use kphis_model::{
    fetch::ExecuteResponse,
    pre_admit::{PreAdmitList, PreAdmitParams, PreAdmitPatch, PreAdmitSave},
};
use kphis_util::error::AppError;

/// /api/ipd/pre-admit
///
/// Get list of Pre-Admit by PARAMS, return a list of Pre-Admit
#[utoipa::path(
    get,
    path = "/ipd/pre-admit",
    responses(DocVec<PreAdmitList>),
    params(PreAdmitParams),
)]
pub async fn get_ipd_pre_admit_list(Query(params): Query<PreAdmitParams>, ctx: RequestState) -> Result<Json<Vec<PreAdmitList>>, AppError> {
    ctx.user_state.trace_req_by();
    ctx.authorize_and_access_log(&Method::GET, false).await?;

    let response = pre_admit::get_pre_admit_list(
        &params,
        ctx.api_state.hosxp_hn_len(),
        ctx.api_state.hosxp_an_len(),
        &ctx.api_state.db_pool,
        &ctx.api_state.hosxp(),
        &ctx.api_state.kphis(),
    )
    .await?;

    Ok(Json(response))
}

/// /api/ipd/pre-admit
///
/// Tries to create Pre-Admit
#[utoipa::path(
    post,
    path = "/ipd/pre-admit",
    request_body = PreAdmitSave,
    responses(DocOne<ExecuteResponse>),
)]
pub async fn post_ipd_pre_admit(ctx: RequestState, Json(payload): Json<PreAdmitSave>) -> Result<Json<ExecuteResponse>, AppError> {
    ctx.user_state.trace_req_by();
    ctx.authorize_and_access_log(&Method::POST, false).await?;

    let response = pre_admit::insert_pre_admit(&payload, &ctx.user_state.user.loginname, &ctx.api_state.db_pool, &ctx.api_state.kphis()).await?;

    Ok(Json(response))
}

/// /api/ipd/pre-admit
///
/// Tries to change Pre-Admit with command
#[utoipa::path(
    patch,
    path = "/ipd/pre-admit",
    request_body = PreAdmitPatch,
    responses(DocVec<ExecuteResponse>),
)]
pub async fn patch_ipd_pre_admit(ctx: RequestState, Json(payload): Json<PreAdmitPatch>) -> Result<Json<Vec<ExecuteResponse>>, AppError> {
    ctx.user_state.trace_req_by();
    ctx.authorize_and_access_log(&Method::PATCH, false).await?;

    let response = pre_admit::patch_pre_admit(
        &payload,
        &ctx.user_state.user.loginname,
        &ctx.api_state.db_pool,
        &ctx.api_state.hosxp(),
        &ctx.api_state.kphis(),
        &ctx.api_state.kphis_extra(),
    )
    .await?;

    Ok(Json(response))
}
