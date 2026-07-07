use axum::{Json, extract::Query, http::Method};

use kphis_api_core::{open_api::DocVec, state::RequestState};
use kphis_api_query::avatar;
use kphis_model::avatar::{AvatarOpdEr, AvatarParams, AvatarWard};
use kphis_util::error::AppError;

/// /api/avatar/opd-er
///
/// Get All OPD-ER Patient Avatar data, return list of OPD-ER Patient Avatar data
#[utoipa::path(
    get,
    path = "/avatar/opd-er",
    responses(DocVec<AvatarOpdEr>),
)]
pub async fn get_avatar_opd_er(ctx: RequestState) -> Result<Json<Vec<AvatarOpdEr>>, AppError> {
    ctx.user_state.trace_req_by();
    ctx.authorize_and_access_log(&Method::GET, false).await?;

    let response = avatar::get_avatar_opd_er(&ctx.api_state.db_pool, &ctx.api_state.hosxp(), &ctx.api_state.kphis()).await?;

    Ok(Json(response))
}

/// /api/avatar/ipd
///
/// Get IPD Patient Avatar data by PARAMS, return list of Patient Avatar data
#[utoipa::path(
    get,
    path = "/avatar/ipd",
    responses(DocVec<AvatarWard>),
    params(AvatarParams),
)]
pub async fn get_avatar_ipd(Query(params): Query<AvatarParams>, ctx: RequestState) -> Result<Json<Vec<AvatarWard>>, AppError> {
    ctx.user_state.trace_req_by();
    ctx.authorize_and_access_log(&Method::GET, false).await?;

    let response = avatar::get_avatar_ipd(
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
