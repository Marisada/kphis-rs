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
    ipd::{index_action, index_monitor, index_plan, order},
    transform::query::check_an_can_execute,
};
use kphis_model::{
    fetch::ExecuteResponse,
    ipd::pharmacy_monitor::{IpdOrderPharmacyMonitor, IpdOrderPharmacyParams},
    order::{MedOrderItem, Order, OrderDate, OrderItem, OrderItemPatch, OrderItemPatchAction, OrderItemType, OrderParams, OrderPatch, OrderSave, OrderTypeName},
};
use kphis_util::error::AppError;

// ipd-dr-order.php
/// /api/ipd/order/order-date-an/{an}
///
/// Get list of IPD Order Date by AN, return list of IPD Order Date
#[utoipa::path(
    get,
    path = "/ipd/order/order-date-an/{an}",
    responses(DocVec<OrderDate>),
    params(
        ("an" = String, Path, description = "Admission Number: AN", example = "660001234"),
    ),
)]
pub async fn get_ipd_order_date(Path(an): Path<String>, ctx: RequestState) -> Result<Json<Vec<OrderDate>>, AppError> {
    ctx.user_state.trace_req_by();
    let is_pre_admit = ctx.api_state.is_pre_admit(&an);
    ctx.authorize_and_access_log(&Method::GET, is_pre_admit).await?;

    let response = order::get_order_date(&an, &ctx.api_state.db_pool, &ctx.api_state.kphis()).await?;

    Ok(Json(response))
}

// ipd-dr-order-one-day-data.php
// ipd-dr-order-continuous-data.php
/// /api/ipd/order/order
///
/// Get list of IPD Order by PARAMS, return list of IPD Order
#[utoipa::path(
    get,
    path = "/ipd/order/order",
    responses(DocVec<Order>),
    params(OrderParams),
)]
pub async fn get_ipd_order(Query(params): Query<OrderParams>, ctx: RequestState) -> Result<Json<Vec<Order>>, AppError> {
    ctx.user_state.trace_req_by();
    let is_pre_admit = ctx.api_state.is_pre_admit_opt(&params.an);
    ctx.authorize_and_access_log(&Method::GET, is_pre_admit).await?;

    let orders = get_ipd_order_bundle(
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

pub async fn get_ipd_order_bundle(
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
            let mut order_items = order::get_order_item(Some(id), Some(order_item_type.clone()), params, &[], pool, hosxp, kphis).await?;
            // let mut order_items = order::get_order_item(orders[pos].order_date, Some(id), Some(order_item_type.clone()), params, pool, hosxp, kphis).await?;
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

/// /api/ipd/order/item
///
/// Get list of IPD Order Item by PARAMS, return list of IPD Order Item
/// (order_id or order_item_id or an) and current_date and view_by is required
///
/// Require AN and view_by in PARAMS
#[utoipa::path(
    get,
    path = "/ipd/order/item",
    responses(DocVec<OrderItem>),
    params(OrderParams),
)]
pub async fn get_ipd_order_item(Query(params): Query<OrderParams>, ctx: RequestState) -> Result<Json<Vec<OrderItem>>, AppError> {
    ctx.user_state.trace_req_by();
    let is_pre_admit = ctx.api_state.is_pre_admit_opt(&params.an);
    ctx.authorize_and_access_log(&Method::GET, is_pre_admit).await?;

    let items = get_ipd_order_item_bundle(&params, &ctx.api_state.db_pool, &ctx.api_state.hosxp(), &ctx.api_state.kphis(), &ctx.api_state.kphis_extra()).await?;

    Ok(Json(items))
}

pub async fn get_ipd_order_item_bundle(params: &OrderParams, pool: &Pool<MySql>, hosxp: &str, kphis: &str, kphis_extra: &str) -> Result<Vec<OrderItem>, AppError> {
    if params.an.as_ref().map(|s| s.is_empty()).unwrap_or(true) || params.view_by.is_none() {
        return Ok(Vec::new());
    }

    let an = params.an.clone().unwrap_or_default();
    if params.without_order.is_some() {
        // default order + plans
        let mut plans = index_plan::get_index_plan_without_order_item_id(&an, params.plan_date, pool, kphis).await?;
        for plan in plans.iter_mut() {
            plan.actions = index_action::get_index_action(plan.plan_id, pool, hosxp, kphis, kphis_extra).await?;
        }
        let order_item = OrderItem {
            index_plans: plans,
            ..Default::default()
        };

        Ok(vec![order_item])
    } else {
        // with order, from plan_date
        let mut order_items = if let Some(plan_date) = params.plan_date {
            let order_item_ids = order::select_order_item_ids_by_an_and_plan_date(&an, plan_date, pool, kphis).await?;
            if !order_item_ids.is_empty() {
                order::get_order_item(params.order_id, None, params, &order_item_ids, pool, hosxp, kphis).await?
            } else {
                Vec::new()
            }
        // with order, from order_id
        } else {
            order::get_order_item(params.order_id, None, params, &[], pool, hosxp, kphis).await?
        };
        // let mut order_items = order::get_order_item(params.current_date.unwrap_or(now().date()), params.order_id, None, params, pool, hosxp, kphis).await?;
        if !params.without_plan.as_ref().map(|s| s.as_str() == "Y").unwrap_or_default() {
            let view_by = params.view_by.clone().unwrap_or_default();
            get_order_items_plans(&mut order_items, &view_by, pool, hosxp, kphis, kphis_extra).await?;
        }
        Ok(order_items)
    }
}

// ipd-dr-order-continuous-previous-data.php
/// /api/ipd/order/previous
///
/// Get list of Previous IPD Order Item by PARAMS, return list of Previous IPD Order Item
#[utoipa::path(
    get,
    path = "/ipd/order/previous",
    responses(DocVec<OrderItem>),
    params(OrderParams),
)]
pub async fn get_ipd_order_previous(Query(params): Query<OrderParams>, ctx: RequestState) -> Result<Json<Vec<OrderItem>>, AppError> {
    ctx.user_state.trace_req_by();
    let is_pre_admit = ctx.api_state.is_pre_admit_opt(&params.an);
    ctx.authorize_and_access_log(&Method::GET, is_pre_admit).await?;

    let order_items = get_ipd_order_previous_bundle(&params, &ctx.api_state.db_pool, &ctx.api_state.hosxp(), &ctx.api_state.kphis(), &ctx.api_state.kphis_extra()).await?;
    Ok(Json(order_items))
}

pub async fn get_ipd_order_previous_bundle(params: &OrderParams, pool: &Pool<MySql>, hosxp: &str, kphis: &str, kphis_extra: &str) -> Result<Vec<OrderItem>, AppError> {
    let view_by = params.view_by.clone().unwrap_or_default();
    let mut order_items = order::get_previous_order(params, pool, hosxp, kphis).await?;
    get_order_items_plans(&mut order_items, &view_by, pool, hosxp, kphis, kphis_extra).await?;

    Ok(order_items)
}

// ipd-dr-order-previous-one-day-order-data.php
/// /api/ipd/order/one-day-previous-an/{an}
///
/// Get list of Previous IPD One-Day Order Item by AN, return list of Previous IPD One-Day Order Item
#[utoipa::path(
    get,
    path = "/ipd/order/one-day-previous-an/{an}",
    responses(DocVec<MedOrderItem>),
    params(
        ("an" = String, Path, description = "Admission Number: AN", example = "660001234"),
    ),
)]
pub async fn get_ipd_order_one_day_previous(Path(an): Path<String>, ctx: RequestState) -> Result<Json<Vec<MedOrderItem>>, AppError> {
    ctx.user_state.trace_req_by();
    let is_pre_admit = ctx.api_state.is_pre_admit(&an);
    ctx.authorize_and_access_log(&Method::GET, is_pre_admit).await?;

    let order_items = order::get_previous_one_day_order(&an, &ctx.api_state.db_pool, &ctx.api_state.hosxp(), &ctx.api_state.kphis()).await?;

    Ok(Json(order_items))
}

// ipd-dr-order-one-day-save.php
// ipd-dr-order-continuous-save.php
/// /api/ipd/order/order
///
/// Tries to create/edit IPD Order
/// - Payload's `visit_type` must has not-empty `an`
/// - Checking `an` is `pre-admit` by `an` length
#[utoipa::path(
    post,
    path = "/ipd/order/order",
    request_body = OrderSave,
    responses(DocVecU32<ExecuteResponse>),
)]
pub async fn post_ipd_order(ctx: RequestState, Json(payload): Json<OrderSave>) -> Result<Json<(u32, Vec<ExecuteResponse>)>, AppError> {
    ctx.user_state.trace_req_by();

    if let Some((an, is_pre_admit)) = payload.visit_type.an_and_is_pre_admit() {
        ctx.authorize_and_access_log(&Method::POST, is_pre_admit).await?;
        // check AN is valid (pre-admit was admited or admit was revoked)
        check_an_can_execute(an, ctx.api_state.hosxp_an_len(), &ctx.api_state.db_pool, &ctx.api_state.hosxp(), &ctx.api_state.kphis()).await?;
    } else {
        return Err(AppError::app_400("PostIpdOrder"));
    }

    let response = order::post_order(&payload, &ctx.user_state.user.loginname, &ctx.api_state.db_pool, &ctx.api_state.kphis()).await?;

    Ok(Json(response))
}

// ipd-dr-order-continuous-order-to-home-med-data.php
/// /api/ipd/order/to-home-med-an/{an}
///
/// Get list of IPD Medical Order Item (from Previous Continuous Order) by AN, return list of IPD Medical Order Item
#[utoipa::path(
    get,
    path = "/ipd/order/to-home-med-an/{an}",
    responses(DocVec<MedOrderItem>),
    params(
        ("an" = String, Path, description = "Admission Number: AN", example = "660001234"),
    ),
)]
pub async fn get_ipd_home_med_from_cont(Path(an): Path<String>, ctx: RequestState) -> Result<Json<Vec<MedOrderItem>>, AppError> {
    ctx.user_state.trace_req_by();
    let is_pre_admit = ctx.api_state.is_pre_admit(&an);
    ctx.authorize_and_access_log(&Method::GET, is_pre_admit).await?;

    let response = order::get_home_med_from_cont(&an, &ctx.api_state.db_pool, &ctx.api_state.hosxp(), &ctx.api_state.kphis()).await?;

    Ok(Json(response))
}

// ipd-dr-order-one-day-confirm.php, ipd-dr-order-continuous-confirm.php
// ipd-dr-order-one-day-nurse_accept.php, ipd-dr-order-continuous-nurse_accept.php
// ipd-dr-order-one-day-pharmacist_accept.php, ipd-dr-order-continuous-pharmacist_accept.php
// ipd-dr-order-one-day-pharmacist_done.php, ipd-dr-order-continuous-pharmacist_done.php
/// /api/ipd/order/order
///
/// Tries to edit IPD Order
///
/// ConfirmAs, EditAs : `order_id` > 0, has `nurse_order_as`<br>
/// DoctorConfirm : has `nurse_order_as`<br>
/// others : `order_id` > 0
#[utoipa::path(
    patch,
    path = "/ipd/order/order",
    request_body = OrderPatch,
    responses(DocVec<ExecuteResponse>),
)]
pub async fn patch_ipd_order(ctx: RequestState, Json(payload): Json<OrderPatch>) -> Result<Json<Vec<ExecuteResponse>>, AppError> {
    ctx.user_state.trace_req_by();
    ctx.authorize_and_access_log(&Method::PATCH, false).await?;

    let response = order::patch_order(
        &payload,
        &ctx.user_state.user.doctorcode,
        &ctx.api_state.app_config.hosxp_med_reconcilation_icode,
        &ctx.user_state.user.loginname,
        &ctx.api_state.db_pool,
        &ctx.api_state.hosxp(),
        &ctx.api_state.kphis(),
    )
    .await?;

    Ok(Json(response))
}

/// /api/ipd/order/item
///
/// Tries to edit IPD OrderItem's nurse_assign, order_item_type, due_doctor, due_pharm
#[utoipa::path(
    patch,
    path = "/ipd/order/item",
    request_body = OrderItemPatch,
    responses(DocOne<ExecuteResponse>),
)]
pub async fn patch_ipd_order_item(ctx: RequestState, Json(payload): Json<OrderItemPatch>) -> Result<Json<ExecuteResponse>, AppError> {
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

// ipd-dr-order-one-day-delete.php, ipd-dr-order-continuous-delete.php
/// /api/ipd/order/order-id/{order_id}
///
/// Tries to delete IPD Order by ID
#[utoipa::path(
    delete,
    path = "/ipd/order/order-id/{order_id}",
    responses(DocOne<ExecuteResponse>),
    params(
        ("order_id" = u32, Path, description = "Order ID", example = "1"),
    ),
)]
pub async fn delete_ipd_order(Path(order_id): Path<u32>, ctx: RequestState) -> Result<Json<ExecuteResponse>, AppError> {
    ctx.user_state.trace_req_by();
    ctx.authorize_and_access_log(&Method::DELETE, false).await?;

    let response = order::delete_order(order_id, &ctx.api_state.db_pool, &ctx.api_state.kphis()).await?;

    Ok(Json(response))
}

// ipd-pharmacy-order-monitor-table.php
/// /api/ipd/order/pharmacy
///
/// Get IPD Order for Pharmacy Monitoring by PARAMS, return single IPD Order for Pharmacy Monitoring
#[utoipa::path(
    get,
    path = "/ipd/order/pharmacy",
    responses(DocOne<IpdOrderPharmacyMonitor>),
    params(IpdOrderPharmacyParams),
)]
pub async fn get_ipd_order_pharmacy(Query(params): Query<IpdOrderPharmacyParams>, ctx: RequestState) -> Result<Json<IpdOrderPharmacyMonitor>, AppError> {
    ctx.user_state.trace_req_by();
    ctx.authorize_and_access_log(&Method::GET, false).await?;

    let order_monitor = order::get_pharmacy_order(
        &params,
        ctx.api_state.hosxp_hn_len(),
        ctx.api_state.hosxp_an_len(),
        &ctx.api_state.db_pool,
        &ctx.api_state.hosxp(),
        &ctx.api_state.kphis(),
    )
    .await?;

    Ok(Json(order_monitor))
}
