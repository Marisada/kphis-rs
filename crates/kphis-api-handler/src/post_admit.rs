use axum::{Json, extract::Query, http::Method};
use kphis_api_core::{
    open_api::{DocOne, DocVec},
    state::RequestState,
};
use kphis_api_query::post_admit;
use kphis_model::post_admit::{PostAdmitList, PostAdmitParams};
use kphis_util::error::AppError;

/// /api/ipd/post-admit/list
///
/// Get list of Post-Admit by PARAMS, return a list of Post-Admit
#[utoipa::path(
    get,
    path = "/ipd/post-admit/list",
    responses(DocVec<PostAdmitList>),
    params(PostAdmitParams),
)]
pub async fn get_ipd_post_admit_list(Query(params): Query<PostAdmitParams>, ctx: RequestState) -> Result<Json<Vec<PostAdmitList>>, AppError> {
    ctx.user_state.trace_req_by();
    ctx.authorize_and_access_log(&Method::GET, false).await?;

    let response = post_admit::get_post_admit_list(
        &params,
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

/// /api/ipd/post-admit/count
///
/// Get Post-Admit count of request doctor which discharge by request doctor and having status `Null` or `Review`
#[utoipa::path(
    get,
    path = "/ipd/post-admit/count",
    responses(DocOne<i64>),
)]
pub async fn get_ipd_post_admit_count(ctx: RequestState) -> Result<Json<i64>, AppError> {
    ctx.user_state.trace_req_by();
    ctx.authorize_and_access_log(&Method::GET, false).await?;

    let response = if let Some(doctorcode) = &ctx.user_state.user.doctorcode {
        post_admit::get_post_admit_count(doctorcode, &ctx.api_state.db_pool, &ctx.api_state.hosxp(), &ctx.api_state.kphis()).await?
    } else {
        0
    };

    Ok(Json(response))
}
