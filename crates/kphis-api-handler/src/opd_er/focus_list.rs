use axum::{
    Json,
    extract::{Path, Query},
    http::Method,
};

use kphis_api_core::{
    open_api::{DocVec, DocVecU32},
    state::RequestState,
};
use kphis_api_query::opd_er::focus_list;
use kphis_model::{
    fetch::ExecuteResponse,
    focus_list::{FocusList, FocusListParams, FocusListSave},
};
use kphis_util::error::AppError;

/// /api/opd-er/focus-list-id/{opd_er_order_master_id}
///
/// Get list of OPD-ER Focus List by ID and PARAMS, return list of OPD-ER Focus List
#[utoipa::path(
    get,
    path = "/opd-er/focus-list-id/{opd_er_order_master_id}",
    responses(DocVec<FocusList>),
    params(
        ("opd_er_order_master_id" = u32, Path, description = "OPD-ER Order Master ID", example = "1"),
        FocusListParams,
    ),
)]
pub async fn get_opd_er_focus_list(Path(opd_er_order_master_id): Path<u32>, Query(params): Query<FocusListParams>, ctx: RequestState) -> Result<Json<Vec<FocusList>>, AppError> {
    ctx.user_state.trace_req_by();
    ctx.authorize_and_access_log(&Method::GET, false).await?;

    let response = focus_list::get_focus_list(opd_er_order_master_id, &params, &ctx.api_state.db_pool, &ctx.api_state.hosxp(), &ctx.api_state.kphis()).await?;

    Ok(Json(response))
}

// iod-er-nurse-focus-list-save.php OR opd-er-nurse-focus-list-update.php
/// /api/opd-er/focus-list-id/{opd_er_order_master_id}
///
/// Tries to create/edit OPD-ER Focus List
#[utoipa::path(
    post,
    path = "/opd-er/focus-list-id/{opd_er_order_master_id}",
    request_body = FocusListSave,
    responses(DocVecU32<ExecuteResponse>),
    params(
        ("opd_er_order_master_id" = u32, Path, description = "OPD-ER Order Master ID", example = "1"),
    ),
)]
pub async fn post_opd_er_focus_list(Path(opd_er_order_master_id): Path<u32>, ctx: RequestState, Json(payload): Json<FocusListSave>) -> Result<Json<(u32, Vec<ExecuteResponse>)>, AppError> {
    ctx.user_state.trace_req_by();
    ctx.authorize_and_access_log(&Method::POST, false).await?;

    let result = focus_list::post_focus_list(
        opd_er_order_master_id,
        &payload,
        &ctx.user_state.user.loginname,
        &ctx.api_state.db_pool,
        &ctx.api_state.hosxp(),
        &ctx.api_state.kphis(),
        &ctx.api_state.kphis_log(),
    )
    .await?;

    Ok(Json(result))
}

// opd_er-nurse-focus-list-delete.php
/// /api/opd-er/focus-list-id/{opd_er_order_master_id}
/// - Query parameter `fclist_id` and `version` must not null
#[utoipa::path(
    delete,
    path = "/opd-er/focus-list-id/{opd_er_order_master_id}",
    responses(DocVec<ExecuteResponse>),
    params(
        ("opd_er_order_master_id" = u32, Path, description = "OPD-ER Order Master ID (Optional)", example = "1"),
        FocusListParams,
    ),
)]
pub async fn delete_opd_er_focus_list(Path(_opd_er_order_master_id): Path<u32>, Query(params): Query<FocusListParams>, ctx: RequestState) -> Result<Json<Vec<ExecuteResponse>>, AppError> {
    ctx.user_state.trace_req_by();
    ctx.authorize_and_access_log(&Method::DELETE, false).await?;

    if let (Some(fclist_id), Some(version)) = (params.fclist_id, params.version) {
        let result = focus_list::delete_focus_list(
            fclist_id,
            version,
            &ctx.user_state.user.loginname,
            &ctx.api_state.db_pool,
            &ctx.api_state.hosxp(),
            &ctx.api_state.kphis(),
            &ctx.api_state.kphis_log(),
        )
        .await?;

        Ok(Json(result))
    } else {
        Err(AppError::app_400("Delete FocusList"))
    }
}
