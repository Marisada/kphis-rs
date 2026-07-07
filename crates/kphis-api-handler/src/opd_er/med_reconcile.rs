use axum::{
    Json,
    extract::{Path, Query},
    http::Method,
};

use kphis_api_core::{
    open_api::{DocOne, DocOpt, DocVec, DocVecU32},
    state::RequestState,
};
use kphis_api_query::opd_er::med_reconcile;
use kphis_model::{
    fetch::ExecuteResponse,
    med_reconcile::{MedReconciliation, MedReconciliationItemPatch, MedReconciliationItemSave, MedReconciliationNote, MedReconciliationParams},
};
use kphis_util::{
    error::{AppError, Source},
    util::zero_none,
};

/// /api/opd-er/med-reconcile
///
/// Get list of OPD-ER Medical Reconciliation by PARAMS, return list of OPD-ER Medical Reconciliation
///
/// Require HN and (opd_er_order_master_id or med_reconciliation_id) in PARAMS
#[utoipa::path(
    get,
    path = "/opd-er/med-reconcile",
    responses(DocVec<MedReconciliation>),
    params(MedReconciliationParams),
)]
pub async fn get_opd_er_med_reconcile(Query(params): Query<MedReconciliationParams>, ctx: RequestState) -> Result<Json<Vec<MedReconciliation>>, AppError> {
    ctx.user_state.trace_req_by();
    ctx.authorize_and_access_log(&Method::GET, false).await?;

    if (params.opd_er_order_master_id.is_some() || params.med_reconciliation_id.is_some()) && params.hn.is_some() {
        let recons = med_reconcile::get_opd_er_med_reconcile(&params, &ctx.user_state.user.doctorcode, &ctx.api_state.db_pool, &ctx.api_state.hosxp(), &ctx.api_state.kphis()).await?;

        Ok(Json(recons))
    } else {
        Ok(Json(Vec::new()))
    }
}

/// /api/opd-er/med-reconcile
///
/// Tries to create a new OPD-ER Medical Reconciliation
/// - Query parameter `opd_er_order_master_id` must not null or 0
#[utoipa::path(
    post,
    path = "/opd-er/med-reconcile",
    request_body = Vec<MedReconciliationItemSave>,
    responses(DocVecU32<ExecuteResponse>),
    params(MedReconciliationParams),
)]
pub async fn post_opd_er_med_reconcile(
    Query(params): Query<MedReconciliationParams>,
    ctx: RequestState,
    Json(payload): Json<Vec<MedReconciliationItemSave>>,
) -> Result<Json<(u32, Vec<ExecuteResponse>)>, AppError> {
    ctx.user_state.trace_req_by();
    ctx.authorize_and_access_log(&Method::POST, false).await?;

    if let Some(opd_er_order_master_id) = params.opd_er_order_master_id.and_then(zero_none) {
        let result = med_reconcile::post_opd_er_med_reconcile(
            opd_er_order_master_id,
            &payload,
            &ctx.user_state.user.doctorcode,
            &ctx.user_state.user.loginname,
            &ctx.api_state.db_pool,
            &ctx.api_state.kphis(),
        )
        .await?;

        Ok(Json(result))
    } else {
        Err(AppError::app_400("Post OpdErMedReconcile"))
    }
}

/// /api/opd-er/med-reconcile
///
/// Tries to edit OPD-ER Medical Reconciliation
/// - Query parameter `med_reconciliation_id` must not null or 0, `patch` must be `doctor`, `pharm`, `unconfirm`, `receive` or `last`
#[utoipa::path(
    patch,
    path = "/opd-er/med-reconcile",
    request_body = Vec<MedReconciliationItemPatch>,
    responses(DocVec<ExecuteResponse>),
    params(MedReconciliationParams),
)]
pub async fn patch_opd_er_med_reconcile(
    Query(params): Query<MedReconciliationParams>,
    ctx: RequestState,
    Json(payload): Json<Vec<MedReconciliationItemPatch>>,
) -> Result<Json<Vec<ExecuteResponse>>, AppError> {
    ctx.user_state.trace_req_by();
    ctx.authorize_and_access_log(&Method::PATCH, false).await?;

    if let (Some(med_reconciliation_id), Some(patch)) = (params.med_reconciliation_id.and_then(zero_none), params.patch) {
        if ["doctor", "pharm", "unconfirm", "receive", "last"].contains(&patch.as_str()) {
            let result = med_reconcile::patch_opd_er_med_reconcile(
                med_reconciliation_id,
                &patch,
                &payload,
                &ctx.user_state.user.doctorcode,
                &ctx.user_state.user.loginname,
                &ctx.api_state.db_pool,
                &ctx.api_state.kphis(),
            )
            .await?;

            Ok(Json(result))
        } else {
            Err(Source::App.to_error(400, "Invalid Patch", "Patch OpdErMedReconcile"))
        }
    } else {
        Err(AppError::app_400("Patch OpdErMedReconcile"))
    }
}

/// /api/opd-er/med-reconcile
///
/// Tries to delete OPD-ER Medical Reconciliation by PARAMS
/// - Query parameter `med_reconciliation_id` or `med_reconciliation_item_id` must not null
#[utoipa::path(
    delete,
    path = "/opd-er/med-reconcile",
    responses(DocOne<ExecuteResponse>),
    params(MedReconciliationParams),
)]
pub async fn delete_opd_er_med_reconcile(Query(params): Query<MedReconciliationParams>, ctx: RequestState) -> Result<Json<ExecuteResponse>, AppError> {
    ctx.user_state.trace_req_by();
    ctx.authorize_and_access_log(&Method::DELETE, false).await?;

    if let Some(med_reconciliation_id) = params.med_reconciliation_id {
        let result = med_reconcile::delete_opd_er_med_reconcile(med_reconciliation_id, &ctx.api_state.db_pool, &ctx.api_state.kphis()).await?;

        Ok(Json(result))
    } else if let Some(med_reconciliation_item_id) = params.med_reconciliation_item_id {
        let result = med_reconcile::delete_opd_er_med_reconcile_item(med_reconciliation_item_id, &ctx.api_state.db_pool, &ctx.api_state.kphis()).await?;

        Ok(Json(result))
    } else {
        Err(AppError::app_400("Delete OpdErMedReconcile"))
    }
}

/// /api/opd-er/med-reconcile-note-id/{med_reconciliation_id}
///
/// Get OPD-ER Medical Reconciliation Note by ID, return single OPD-ER Medical Reconciliation Note or none
#[utoipa::path(
    get,
    path = "/opd-er/med-reconcile-note-id/{med_reconciliation_id}",
    responses(DocOpt<MedReconciliationNote>),
    params(
        ("med_reconciliation_id" = u32, Path, description = "Medical Reconciliation ID", example = "1"),
    ),
)]
pub async fn get_opd_er_med_reconcile_note(Path(med_reconciliation_id): Path<u32>, ctx: RequestState) -> Result<Json<Option<MedReconciliationNote>>, AppError> {
    ctx.user_state.trace_req_by();
    ctx.authorize_and_access_log(&Method::GET, false).await?;

    let note = med_reconcile::get_opd_er_med_reconcile_note(med_reconciliation_id, &ctx.api_state.db_pool, &ctx.api_state.kphis()).await?;

    Ok(Json(note))
}

/// /api/opd-er/med-reconcile-note-id/{med_reconciliation_id}
///
/// Tries to create/edit OPD-ER Medical Reconciliation Note
#[utoipa::path(
    post,
    path = "/opd-er/med-reconcile-note-id/{med_reconciliation_id}",
    request_body = String,
    responses(DocOne<ExecuteResponse>),
    params(
        ("med_reconciliation_id" = u32, Path, description = "Medical Reconciliation ID", example = "1"),
    ),
)]
pub async fn post_opd_er_med_reconcile_note(Path(med_reconciliation_id): Path<u32>, ctx: RequestState, Json(payload): Json<String>) -> Result<Json<ExecuteResponse>, AppError> {
    ctx.user_state.trace_req_by();
    ctx.authorize_and_access_log(&Method::POST, false).await?;

    let response = med_reconcile::post_opd_er_med_reconcile_note(med_reconciliation_id, &payload, &ctx.user_state.user.loginname, &ctx.api_state.db_pool, &ctx.api_state.kphis()).await?;

    Ok(Json(response))
}
