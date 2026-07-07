use axum::{
    Json,
    extract::{Path, Query},
    http::Method,
};
use sqlx::{MySql, Pool};

use kphis_api_core::{
    open_api::{DocOne, DocVec, DocVecU32},
    state::RequestState,
};
use kphis_api_query::{
    pre_order::{into, master, order, progress_note},
    transform::query::check_an_opt_can_execute,
};
use kphis_model::{
    fetch::ExecuteResponse,
    order::OrderTypeName,
    pre_order::{
        master::{PreOrderMaster, PreOrderMasterParams, PreOrderMasterSave},
        order::{PreOrder, PreOrderIntoCommand, PreOrderItemType, PreOrderParams, PreOrderSave},
        progress_note::{PreProgressNote, PreProgressNoteItemType, PreProgressNoteParams, PreProgressNoteSave},
    },
    progress_note::ProgressNoteTypeName,
};
use kphis_util::error::AppError;

// ipd-dr-pre-order-list-data.php
/// /api/ipd/pre-order/master
///
/// Get list of IPD Pre-Order Master by PARAMS, return list of IPD Pre-Order Master
#[utoipa::path(
    get,
    path = "/ipd/pre-order/master",
    responses(DocVec<PreOrderMaster>),
    params(PreOrderMasterParams),
)]
pub async fn get_ipd_pre_order_list(Query(params): Query<PreOrderMasterParams>, ctx: RequestState) -> Result<Json<Vec<PreOrderMaster>>, AppError> {
    ctx.user_state.trace_req_by();
    ctx.authorize_and_access_log(&Method::GET, false).await?;

    let response = master::get_pre_order_list(
        &params,
        &ctx.api_state.app_config.doctor_intern_roles,
        &ctx.api_state.db_pool,
        &ctx.api_state.hosxp(),
        &ctx.api_state.kphis(),
    )
    .await?;

    Ok(Json(response))
}

// ipd-dr-pre-order-master-save.php
/// /api/ipd/pre-order/master
///
/// Tries to create/edit IPD Pre-Order Master
#[utoipa::path(
    post,
    path = "/ipd/pre-order/master",
    request_body = PreOrderMasterSave,
    responses(DocVecU32<ExecuteResponse>),
)]
pub async fn post_ipd_pre_order_master(ctx: RequestState, Json(payload): Json<PreOrderMasterSave>) -> Result<Json<(u32, ExecuteResponse)>, AppError> {
    ctx.user_state.trace_req_by();
    ctx.authorize_and_access_log(&Method::POST, false).await?;

    let response = master::post_pre_order_master(&payload, &ctx.user_state.user.loginname, &ctx.api_state.db_pool, &ctx.api_state.kphis()).await?;

    Ok(Json(response))
}

// ipd-dr-pre-order-one-day-data.php
// ipd-dr-pre-order-continuous-data.php
/// /api/ipd/pre-order/order
///
/// Get list of IPD Pre-Order by PARAMS, return list of IPD Pre-Order
///
/// Require order_type in PARAMS
#[utoipa::path(
    get,
    path = "/ipd/pre-order/order",
    responses(DocVec<PreOrder>),
    params(PreOrderParams),
)]
pub async fn get_ipd_pre_order(Query(params): Query<PreOrderParams>, ctx: RequestState) -> Result<Json<Vec<PreOrder>>, AppError> {
    ctx.user_state.trace_req_by();
    ctx.authorize_and_access_log(&Method::GET, false).await?;

    let orders = get_ipd_pre_order_bundle(
        &params,
        &ctx.api_state.app_config.doctor_intern_roles,
        &ctx.api_state.db_pool,
        &ctx.api_state.hosxp(),
        &ctx.api_state.kphis(),
    )
    .await?;

    Ok(Json(orders))
}

pub async fn get_ipd_pre_order_bundle(params: &PreOrderParams, intern_roles: &[String], pool: &Pool<MySql>, hosxp: &str, kphis: &str) -> Result<Vec<PreOrder>, AppError> {
    if let Some(order_type) = params.order_type.as_ref() {
        let mut orders = order::get_order(params, intern_roles, pool, hosxp, kphis).await?;
        let ids = orders.iter().map(|op| op.order_id).collect::<Vec<u32>>();
        let item_types = order::get_order_types(&ids, order_type, pool, kphis).await?;
        for (id, order_item_type) in item_types {
            if let Some(pos) = orders.iter().position(|or| or.order_id == id) {
                let order_items = order::get_order_item(id, &order_item_type, order_type, pool, hosxp, kphis).await?;
                orders[pos].order_item_types.push(PreOrderItemType {
                    order_item_type: OrderTypeName::from_string(&order_item_type),
                    order_items,
                });
            }
        }

        Ok(orders)
    } else {
        Ok(Vec::new())
    }
}

// ipd-dr-pre-order-one-day-save.php
// ipd-dr-pre-order-continuous-save.php
/// /api/ipd/pre-order/order
///
/// Tries to create/edit IPD Pre-Order
#[utoipa::path(
    post,
    path = "/ipd/pre-order/order",
    request_body = PreOrderSave,
    responses(DocVecU32<ExecuteResponse>),
)]
pub async fn post_ipd_pre_order(ctx: RequestState, Json(payload): Json<PreOrderSave>) -> Result<Json<(u32, Vec<ExecuteResponse>)>, AppError> {
    ctx.user_state.trace_req_by();
    ctx.authorize_and_access_log(&Method::POST, false).await?;

    let response = order::post_order(&payload, &ctx.user_state.user.loginname, &ctx.api_state.db_pool, &ctx.api_state.kphis()).await?;

    Ok(Json(response))
}

// ipd-dr-order-progress-note-data.php
/// /api/ipd/pre-order/progress-note
///
/// Get list of IPD Pre-Progress Note by PARAMS, return list of IPD Pre-Progress Note
#[utoipa::path(
    get,
    path = "/ipd/pre-order/progress-note",
    responses(DocVec<PreProgressNote>),
    params(PreProgressNoteParams),
)]
pub async fn get_ipd_pre_progress_note(Query(params): Query<PreProgressNoteParams>, ctx: RequestState) -> Result<Json<Vec<PreProgressNote>>, AppError> {
    ctx.user_state.trace_req_by();
    ctx.authorize_and_access_log(&Method::GET, false).await?;

    let progress_notes = get_ipd_pre_progress_note_bundle(
        &params,
        &ctx.api_state.app_config.doctor_intern_roles,
        &ctx.api_state.db_pool,
        &ctx.api_state.hosxp(),
        &ctx.api_state.kphis(),
    )
    .await?;

    Ok(Json(progress_notes))
}

pub async fn get_ipd_pre_progress_note_bundle(params: &PreProgressNoteParams, intern_roles: &[String], pool: &Pool<MySql>, hosxp: &str, kphis: &str) -> Result<Vec<PreProgressNote>, AppError> {
    let mut progress_notes = progress_note::get_progress_note(params, intern_roles, pool, hosxp, kphis).await?;
    let ids = progress_notes.iter().map(|op| op.progress_note_id).collect::<Vec<u32>>();
    let item_types = progress_note::get_progress_note_types(&ids, pool, kphis).await?;
    for (id, progress_note_item_type) in item_types {
        if let Some(pos) = progress_notes.iter().position(|or| or.progress_note_id == id) {
            let progress_note_items = progress_note::get_progress_note_item(id, &progress_note_item_type, pool, kphis).await?;
            progress_notes[pos].progress_note_item_types.push(PreProgressNoteItemType {
                progress_note_item_type: ProgressNoteTypeName::from_string(&progress_note_item_type),
                progress_note_items,
            });
        }
    }

    Ok(progress_notes)
}

// ipd-dr-pre_order-progress-note-save.php
/// /api/ipd/pre-order/progress-note
///
/// Tries to create/edit IPD Pre-Progress Note
#[utoipa::path(
    post,
    path = "/ipd/pre-order/progress-note",
    request_body = PreProgressNoteSave,
    responses(DocVecU32<ExecuteResponse>),
)]
pub async fn post_ipd_pre_progress_note(ctx: RequestState, Json(payload): Json<PreProgressNoteSave>) -> Result<Json<(u32, Vec<ExecuteResponse>)>, AppError> {
    ctx.user_state.trace_req_by();
    ctx.authorize_and_access_log(&Method::POST, false).await?;

    let response = progress_note::post_progress_note(&payload, &ctx.user_state.user.loginname, &ctx.api_state.db_pool, &ctx.api_state.kphis()).await?;

    Ok(Json(response))
}

// ipd-dr-pre-order-to-order.php
// ipd-dr-template-to-opd-er-order.php
// ipd-dr-template-to-order.php
// ipd-dr-template-to-pre-order.php
/// /api/ipd/pre-order/into
///
/// Tries to copy/edit IPD Pre-Order into another Type
/// - Query parameter `from`, `into`, `from_id` and `into_id` must not null
/// - Payload's `into_id` must not null or empty when `into` is Some(order)
/// - Checking `an` is `pre-admit` by `an` length (`into_id`)
#[utoipa::path(
    post,
    path = "/ipd/pre-order/into",
    request_body = PreOrderIntoCommand,
    responses(DocVec<ExecuteResponse>),
)]
pub async fn post_ipd_pre_order_into(ctx: RequestState, Json(payload): Json<PreOrderIntoCommand>) -> Result<Json<Vec<ExecuteResponse>>, AppError> {
    ctx.user_state.trace_req_by();
    ctx.authorize_and_access_log(&Method::POST, false).await?;

    let response = if payload.is_valid() {
        // check AN is valid (pre-admit was admited or admit was revoked)
        if payload.into == Some(String::from("order")) {
            check_an_opt_can_execute(&payload.into_id, ctx.api_state.hosxp_an_len(), &ctx.api_state.db_pool, &ctx.api_state.hosxp(), &ctx.api_state.kphis()).await?;
        }
        into::post_pre_order_into(payload, &ctx.user_state.user.loginname, &ctx.user_state.user.doctorcode, &ctx.api_state.db_pool, &ctx.api_state.kphis()).await?
    } else {
        return Err(AppError::app_400("PreOrderInto"));
    };

    Ok(Json(response))
}

// ipd-dr-pre-order-master-delete.php
/// /api/ipd/pre-order/master-id/{pre_order_master_id}
///
/// Tries to delete IPD Pre-Order Master by ID
#[utoipa::path(
    delete,
    path = "/ipd/pre-order/master-id/{pre_order_master_id}",
    responses(DocVec<ExecuteResponse>),
    params(
        ("pre_order_master_id" = u32, Path, description = "Pre-Order Master ID", example = "1"),
    ),
)]
pub async fn delete_ipd_pre_order_master(Path(pre_order_master_id): Path<u32>, ctx: RequestState) -> Result<Json<Vec<ExecuteResponse>>, AppError> {
    ctx.user_state.trace_req_by();
    ctx.authorize_and_access_log(&Method::DELETE, false).await?;

    let response = master::delete_pre_order_by_master_id(pre_order_master_id, &ctx.api_state.db_pool, &ctx.api_state.kphis()).await?;

    Ok(Json(response))
}

// ipd-dr-pre-order-one-day-delete.php
// ipd-dr-pre-order-continuous-delete.php
/// /api/ipd/pre-order/order-id/{order_id}
///
/// Tries to delete IPD Pre-Order by ID
#[utoipa::path(
    delete,
    path = "/ipd/pre-order/order-id/{order_id}",
    responses(DocOne<ExecuteResponse>),
    params(
        ("order_id" = u32, Path, description = "Order ID", example = "1"),
    ),
)]
pub async fn delete_ipd_pre_order(Path(order_id): Path<u32>, ctx: RequestState) -> Result<Json<ExecuteResponse>, AppError> {
    ctx.user_state.trace_req_by();
    ctx.authorize_and_access_log(&Method::DELETE, false).await?;

    let response = order::delete_pre_order(order_id, &ctx.api_state.db_pool, &ctx.api_state.kphis()).await?;

    Ok(Json(response))
}

// ipd-dr-pre-order-progress-note-delete.php
/// /api/ipd/pre-order/progress-note-id/{progress_note_id}
///
/// Tries to delete IPD Pre-Progress Note by ID
#[utoipa::path(
    delete,
    path = "/ipd/pre-order/progress-note-id/{progress_note_id}",
    responses(DocOne<ExecuteResponse>),
    params(
        ("progress_note_id" = u32, Path, description = "Progress Note ID", example = "1"),
    ),
)]
pub async fn delete_ipd_pre_progress_note(Path(progress_note_id): Path<u32>, ctx: RequestState) -> Result<Json<ExecuteResponse>, AppError> {
    ctx.user_state.trace_req_by();
    ctx.authorize_and_access_log(&Method::DELETE, false).await?;

    let response = progress_note::delete_progress_note(progress_note_id, &ctx.api_state.db_pool, &ctx.api_state.kphis()).await?;

    Ok(Json(response))
}
