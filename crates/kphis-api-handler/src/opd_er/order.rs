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
use kphis_api_query::opd_er::{index_action, index_monitor, index_plan, order};
use kphis_model::{
    fetch::ExecuteResponse,
    opd_er::pharmacy_monitor::{OpdErOrderPharmacyMonitor, OpdErOrderPharmacyParams},
    order::{Order, OrderItem, OrderItemPatch, OrderItemPatchAction, OrderItemType, OrderParams, OrderPatch, OrderSave, OrderTypeName},
};
use kphis_util::error::AppError;

// opd-er-order-one-day-data.php
// opd-er-order-continuous-data.php
/// /api/opd-er/order/order
///
/// Get OPD-ER Order by PARAMS, return a list of OPD-ER Order
#[utoipa::path(
    get,
    path = "/opd-er/order/order",
    responses(DocVec<Order>),
    params(OrderParams),
)]
pub async fn get_opd_er_order(Query(params): Query<OrderParams>, ctx: RequestState) -> Result<Json<Vec<Order>>, AppError> {
    ctx.user_state.trace_req_by();
    ctx.authorize_and_access_log(&Method::GET, false).await?;

    let orders = get_opd_er_order_bundle(
        &params,
        &ctx.user_state.user.doctorcode,
        &ctx.api_state.app_config.doctor_intern_roles,
        &ctx.api_state.db_pool,
        &ctx.api_state.hosxp(),
        &ctx.api_state.kphis(),
        &ctx.api_state.kphis_extra(),
    )
    .await?;

    Ok(Json(orders))
}

pub async fn get_opd_er_order_bundle(
    params: &OrderParams,
    doctorcode: &Option<String>,
    intern_roles: &[String],
    pool: &Pool<MySql>,
    hosxp: &str,
    kphis: &str,
    kphis_extra: &str,
) -> Result<Vec<Order>, AppError> {
    let view_by = params.view_by.clone().unwrap_or_default();
    let mut orders = order::get_order(params, doctorcode, intern_roles, pool, hosxp, kphis).await?;
    let ids = orders.iter().map(|op| op.order_id).collect::<Vec<u32>>();
    let item_types = order::get_order_types(&ids, &params.order_type, pool, kphis).await?;
    for (id, order_item_type) in item_types {
        if let Some(pos) = orders.iter().position(|or| or.order_id == id) {
            let mut order_items = order::get_order_item(Some(id), Some(order_item_type.clone()), params, pool, hosxp, kphis).await?;
            if !params.without_plan.as_ref().map(|s| s.as_str() == "Y").unwrap_or_default() {
                get_order_items_plans(&mut order_items, &view_by, pool, hosxp, kphis, kphis_extra).await?;
            }
            orders[pos].order_item_types.push(OrderItemType {
                order_item_type: OrderTypeName::from_string(&order_item_type),
                order_items,
            });
        }
    }

    Ok(orders)
}

async fn get_order_items_plans(order_items: &mut [OrderItem], view_by: &str, pool: &Pool<MySql>, hosxp: &str, kphis: &str, kphis_extra: &str) -> Result<(), AppError> {
    if ["doctor", "nurse", "pharmacist"].contains(&view_by) {
        for order_item in order_items.iter_mut() {
            let mut plans = index_plan::get_index_plan_by_order_item_id(order_item.order_item_id, pool, kphis).await?;
            for plan in plans.iter_mut() {
                plan.order_type = order_item.order_type.clone();
                plan.order_date = order_item.order_date;
                plan.order_time = order_item.order_time;
                plan.off_by_datetime = order_item.off_by_datetime;
                let mut actions = index_action::get_index_action(plan.plan_id, pool, hosxp, kphis, kphis_extra).await?;
                for action in actions.iter_mut() {
                    if action.has_monitor
                        && let Some(action_id) = action.action_id
                    {
                        action.monitors = index_monitor::get_index_monitor(action_id, pool, hosxp, kphis_extra).await?;
                    }
                }
                plan.actions = actions;
            }
            order_item.index_plans = plans;
        }
    }

    Ok(())
}

/// /api/opd-er/order/item
///
/// Get list of IPD Order Item by PARAMS, return list of IPD Order Item
///
/// opd_er_order_master_id and view_by are required
///
/// Require opd_er_order_master_id and view_by in PARAMS
#[utoipa::path(
    get,
    path = "/opd-er/order/item",
    responses(DocVec<OrderItem>),
    params(OrderParams),
)]
pub async fn get_opd_er_order_item(Query(params): Query<OrderParams>, ctx: RequestState) -> Result<Json<Vec<OrderItem>>, AppError> {
    ctx.user_state.trace_req_by();
    ctx.authorize_and_access_log(&Method::GET, false).await?;

    let items = get_opd_er_order_item_bundle(&params, &ctx.api_state.db_pool, &ctx.api_state.hosxp(), &ctx.api_state.kphis(), &ctx.api_state.kphis_extra()).await?;

    Ok(Json(items))
}

pub async fn get_opd_er_order_item_bundle(params: &OrderParams, pool: &Pool<MySql>, hosxp: &str, kphis: &str, kphis_extra: &str) -> Result<Vec<OrderItem>, AppError> {
    if params.opd_er_order_master_id.is_none() || params.view_by.is_none() {
        return Ok(Vec::new());
    }

    let opd_er_order_master_id = params.opd_er_order_master_id.unwrap_or_default();
    if params.without_order.is_some() {
        // default order + plans
        let mut plans = index_plan::get_index_plan_without_order_item_id(opd_er_order_master_id, pool, kphis).await?;
        for plan in plans.iter_mut() {
            plan.actions = index_action::get_index_action(plan.plan_id, pool, hosxp, kphis, kphis_extra).await?;
        }
        let order_item = OrderItem {
            index_plans: plans,
            ..Default::default()
        };

        Ok(vec![order_item])
    } else {
        // with order
        let view_by = params.view_by.clone().unwrap_or_default();
        let mut order_items = order::get_order_item(params.order_id, None, params, pool, hosxp, kphis).await?;
        if !params.without_plan.as_ref().map(|s| s.as_str() == "Y").unwrap_or_default() {
            get_order_items_plans(&mut order_items, &view_by, pool, hosxp, kphis, kphis_extra).await?;
        }

        Ok(order_items)
    }
}

// opd-er-order-one-day-save.php
// opd-er-order-continuous-save.php
/// /api/opd-er/order/order
///
/// Tries to create/edit OPD-ER Order
#[utoipa::path(
    post,
    path = "/opd-er/order/order",
    request_body = OrderSave,
    responses(DocVecU32<ExecuteResponse>),
)]
pub async fn post_opd_er_order(ctx: RequestState, Json(payload): Json<OrderSave>) -> Result<Json<(u32, Vec<ExecuteResponse>)>, AppError> {
    ctx.user_state.trace_req_by();
    ctx.authorize_and_access_log(&Method::POST, false).await?;

    let response = order::post_order(&payload, &ctx.user_state.user.loginname, &ctx.api_state.db_pool, &ctx.api_state.kphis()).await?;

    Ok(Json(response))
}

// opd-er-order-one-day-confirm.php
// opd-er-order-one-day-nurse_accept.php
// opd-er-order-one-day-pharmacist_accept.php
// opd-er-order-one-day-pharmacist_done.php
/// /api/opd-er/order/order
///
/// Tries to edit OPD-ER Order
///
/// ConfirmAs, EditAs : `order_id` > 0, has `nurse_order_as`<br>
/// DoctorConfirm : has `nurse_order_as`<br>
/// others : `order_id` > 0
#[utoipa::path(
    patch,
    path = "/opd-er/order/order",
    request_body = OrderPatch,
    responses(DocVec<ExecuteResponse>),
)]
pub async fn patch_opd_er_order(ctx: RequestState, Json(payload): Json<OrderPatch>) -> Result<Json<Vec<ExecuteResponse>>, AppError> {
    ctx.user_state.trace_req_by();
    ctx.authorize_and_access_log(&Method::PATCH, false).await?;

    let responses = order::patch_order(
        &payload,
        &ctx.user_state.user.doctorcode,
        &ctx.user_state.user.loginname,
        &ctx.api_state.db_pool,
        &ctx.api_state.kphis(),
    )
    .await?;

    Ok(Json(responses))
}

/// /api/opd-er/order/item
///
/// Tries to edit OPD-ER OrderItem nurse_assign, order_item_type, due_doctor, due_pharm
#[utoipa::path(
    patch,
    path = "/opd-er/order/item",
    request_body = OrderItemPatch,
    responses(DocOne<ExecuteResponse>),
)]
pub async fn patch_opd_er_order_item(ctx: RequestState, Json(payload): Json<OrderItemPatch>) -> Result<Json<ExecuteResponse>, AppError> {
    ctx.user_state.trace_req_by();
    ctx.authorize_and_access_log(&Method::PATCH, false).await?;

    let response = match payload.action {
        OrderItemPatchAction::NurseAssign => order::update_order_item_nurse_assign(&payload, &ctx.user_state.user.loginname, &ctx.api_state.db_pool, &ctx.api_state.kphis()).await?,
        OrderItemPatchAction::OrderItemType => order::update_order_item_type(&payload, &ctx.user_state.user.loginname, &ctx.api_state.db_pool, &ctx.api_state.kphis()).await?,
        OrderItemPatchAction::DueDoctor => order::update_order_item_due_doctor(&payload, &ctx.user_state.user.loginname, &ctx.api_state.db_pool, &ctx.api_state.kphis()).await?,
        OrderItemPatchAction::DuePharm => order::update_order_item_due_pharm(&payload, &ctx.user_state.user.loginname, &ctx.api_state.db_pool, &ctx.api_state.kphis()).await?,
    };

    Ok(Json(response))
}

// opd-er-order-one-day-delete.php, opd-er-order-continuous-delete.php
/// /api/opd-er/order/order-id/{order_id}
///
/// Tries to delete OPD-ER Order by ID
#[utoipa::path(
    delete,
    path = "/opd-er/order/order-id/{order_id}",
    responses(DocOne<ExecuteResponse>),
    params(
        ("order_id" = u32, Path, description = "Order ID", example = "1"),
    ),
)]
pub async fn delete_opd_er_order(Path(order_id): Path<u32>, ctx: RequestState) -> Result<Json<ExecuteResponse>, AppError> {
    ctx.user_state.trace_req_by();
    ctx.authorize_and_access_log(&Method::DELETE, false).await?;

    let response = order::delete_order(order_id, &ctx.api_state.db_pool, &ctx.api_state.kphis()).await?;

    Ok(Json(response))
}

// opd-er-pharmacy-order-monitor-table.php
/// /api/opd-er/order/pharmacy
///
/// Get OPD-ER Order for Pharmacy Monitoring by PARAMS, return single OPD-ER Order for Pharmacy Monitoring
#[utoipa::path(
    get,
    path = "/opd-er/order/pharmacy",
    responses(DocOne<OpdErOrderPharmacyMonitor>),
    params(OpdErOrderPharmacyParams),
)]
pub async fn get_opd_er_order_pharmacy(Query(params): Query<OpdErOrderPharmacyParams>, ctx: RequestState) -> Result<Json<OpdErOrderPharmacyMonitor>, AppError> {
    ctx.user_state.trace_req_by();
    ctx.authorize_and_access_log(&Method::GET, false).await?;

    let order_monitor = order::get_pharmacy_order(
        &params,
        ctx.api_state.hosxp_hn_len(),
        ctx.api_state.hosxp_vn_len(),
        &ctx.api_state.db_pool,
        &ctx.api_state.hosxp(),
        &ctx.api_state.kphis(),
    )
    .await?;

    Ok(Json(order_monitor))
}
