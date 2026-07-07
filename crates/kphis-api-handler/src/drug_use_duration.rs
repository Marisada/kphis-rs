use axum::{Json, extract::Query, http::Method};

use kphis_api_core::{
    open_api::{DocOne, DocVec},
    state::RequestState,
};
use kphis_api_query::drug_use_duration;
use kphis_model::{
    drug_use_duration::{DrugUseDuration, DrugUseDurationParams},
    fetch::ExecuteResponse,
};
use kphis_util::error::AppError;

/// /api/drug-use-duration
///
/// Get list of drug dosage and duration alerts
#[utoipa::path(
    get,
    path = "/drug-use-duration",
    responses(DocVec<DrugUseDuration>),
    params(DrugUseDurationParams),
)]
pub async fn get_drug_use_duration(Query(params): Query<DrugUseDurationParams>, ctx: RequestState) -> Result<Json<Vec<DrugUseDuration>>, AppError> {
    ctx.user_state.trace_req_by();
    ctx.authorize_and_access_log(&Method::GET, false).await?;

    let response = drug_use_duration::get_drug_use_duration(&params, &ctx.api_state.db_pool, &ctx.api_state.hosxp(), &ctx.api_state.kphis()).await?;

    Ok(Json(response))
}

/// /api/drug-use-duration
///
/// Tries to add/update drug dosage and duration alerts
#[utoipa::path(
    post,
    path = "/drug-use-duration",
    request_body = DrugUseDuration,
    responses(DocOne<ExecuteResponse>),
)]
pub async fn post_drug_use_duration(ctx: RequestState, Json(payload): Json<DrugUseDuration>) -> Result<Json<ExecuteResponse>, AppError> {
    ctx.user_state.trace_req_by();
    ctx.authorize_and_access_log(&Method::POST, false).await?;

    let result = drug_use_duration::post_drug_use_duration(&payload, &ctx.user_state.user.loginname, &ctx.api_state.db_pool, &ctx.api_state.kphis()).await?;

    Ok(Json(result))
}
