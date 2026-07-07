use axum::{Json, extract::Path, http::Method};

use kphis_api_core::{
    open_api::{DocOne, DocVecU32},
    state::RequestState,
};
use kphis_api_query::opd_er::index_plan;
use kphis_model::{fetch::ExecuteResponse, index_plan::IndexPlanSave};
use kphis_util::error::AppError;

// // opd-er-nurse-index-plan-data.php
// // opd-er-nurse-index-plan-list-data.php
// // opd-er-nurse-index-plan-monitor-data.php
// /// /api/opd-er/index-plan
// ///
// /// Get list of OPD-ER Index Plan by PARAMS, return list of OPD-ER Index Plan
// #[utoipa::path(
//     get,
//     path = "/opd-er/index-plan",
//     responses(DocVec<IndexPlanPlus>),
//     params(IndexPlanParams),
// )]
// pub async fn get_opd_er_index_plan(Query(params): Query<IndexPlanParams>, ctx: RequestState) -> Result<Json<Vec<IndexPlanPlus>>, AppError> {
//     ctx.user_state.trace_req_by();
//     ctx.authorize_and_access_log(&Method::GET, false).await?;

//     let plans = get_opd_er_index_plan_plus_bundle(
//         &params,
//         ctx.api_state.hosxp_hn_len(),
//         ctx.api_state.hosxp_an_len(),
//         &ctx.api_state.db_pool,
//         &ctx.api_state.hosxp(),
//         &ctx.api_state.kphis(),
//     )
//     .await?;

//     Ok(Json(plans))
// }

// pub async fn get_opd_er_index_plan_plus_bundle(params: &IndexPlanParams, hn_len: usize, an_len: usize, pool: &Pool<MySql>, hosxp: &str, kphis: &str) -> Result<Vec<IndexPlanPlus>, AppError> {
//     let mut plans = index_plan::get_index_plan_plus(params, hn_len, an_len, pool, hosxp, kphis).await?;

//     for plan in plans.iter_mut() {
//         plan.actions = index_action::get_index_action(plan.plan_id, pool, hosxp, kphis).await?;
//     }

//     Ok(plans)
// }

// opd-er-nurse-index-plan-action-save.php
/// /api/opd-er/index-plan
///
/// Tries to create/edit OPD-ER Index Plan
#[utoipa::path(
    post,
    path = "/opd-er/index-plan",
    request_body = IndexPlanSave,
    responses(DocVecU32<ExecuteResponse>),
)]
pub async fn post_opd_er_index_plan(ctx: RequestState, Json(payload): Json<IndexPlanSave>) -> Result<Json<(u32, Vec<ExecuteResponse>)>, AppError> {
    ctx.user_state.trace_req_by();
    ctx.authorize_and_access_log(&Method::POST, false).await?;

    let response = index_plan::post_index_plan(&payload, &ctx.user_state.user.loginname, &ctx.api_state.db_pool, &ctx.api_state.kphis()).await?;

    Ok(Json(response))
}

// opd-er-nurse-index-plan-action-delete.php
/// /api/opd-er/index-plan-id/{plan_id}
///
/// Tries to delete OPD-ER Index Plan by ID
#[utoipa::path(
    delete,
    path = "/opd-er/index-plan-id/{plan_id}",
    responses(DocOne<ExecuteResponse>),
    params(
        ("plan_id" = u32, Path, description = "Plan ID", example = "1"),
    ),
)]
pub async fn delete_opd_er_index_plan(Path(plan_id): Path<u32>, ctx: RequestState) -> Result<Json<ExecuteResponse>, AppError> {
    ctx.user_state.trace_req_by();
    ctx.authorize_and_access_log(&Method::DELETE, false).await?;

    let response = index_plan::delete_index_plan(plan_id, &ctx.api_state.db_pool, &ctx.api_state.kphis()).await?;

    Ok(Json(response))
}
