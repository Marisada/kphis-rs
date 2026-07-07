use futures_util::stream::StreamExt;
use sqlx::{
    AssertSqlSafe, Executor, FromRow, MySql, Pool, Row,
    mysql::{MySqlQueryResult, MySqlRow},
};
use std::collections::HashMap;

use kphis_model::{
    fetch::ExecuteResponse,
    order::OrderItemSave,
    pre_order::order::{PreOrder, PreOrderItem, PreOrderParams, PreOrderSave},
};
use kphis_sql::{ipd::order, opd_er, pre_order};
use kphis_util::error::{AppError, Source};

use crate::query_all;

// ipd-dr-pre-order-one-day-data.php
// ipd-dr-pre-order-continuous-data.php
pub async fn get_order(params: &PreOrderParams, intern_roles: &[String], pool: &Pool<MySql>, hosxp: &str, kphis: &str) -> Result<Vec<PreOrder>, AppError> {
    if params.pre_order_master_id.is_none() && params.order_id.is_none() {
        return Ok(Vec::new());
    }

    let sql = pre_order::order::select_order(params, intern_roles, hosxp, kphis);
    let mut query = sqlx::query(AssertSqlSafe(sql));
    if let Some(order_id) = params.order_id.as_ref() {
        query = query.bind(order_id);
    }
    if let Some(order_confirm) = params.order_confirm.as_ref() {
        query = query.bind(order_confirm);
    }
    query
        .fetch_all(pool)
        .await
        .map(|rows| {
            rows.iter()
                .map(PreOrder::from_row)
                .collect::<sqlx::Result<Vec<PreOrder>>>()
                .map_err(|e| Source::SQLx.to_error(500, e, "Select PreOrder"))
        })
        .map_err(|e| Source::SQLx.to_error(500, e, "Select PreOrder"))?
}

pub async fn get_order_types(ids: &[u32], order_type: &str, pool: &Pool<MySql>, kphis: &str) -> Result<Vec<(u32, String)>, AppError> {
    if ids.is_empty() {
        Ok(Vec::new())
    } else {
        let sql = pre_order::order::select_order_types(ids, order_type, kphis);
        query_all(&sql, pool, "Select OrderItemType").await.map(|rows| {
            rows.iter()
                .filter_map(|row| {
                    let item_type: Option<String> = row.try_get("order_item_type").ok();
                    let order_id: Option<u32> = row.try_get("order_id").ok();
                    order_id.zip(item_type)
                })
                .collect()
        })
    }
}

pub async fn get_order_item(order_id: u32, order_item_type: &str, order_type: &str, pool: &Pool<MySql>, hosxp: &str, kphis: &str) -> Result<Vec<PreOrderItem>, AppError> {
    let sql = pre_order::order::select_order_item(order_id, order_item_type, order_type, hosxp, kphis);
    query_all(&sql, pool, "Select PreOrderItem")
        .await?
        .iter()
        .map(PreOrderItem::from_row)
        .collect::<sqlx::Result<Vec<PreOrderItem>>>()
        .map_err(|e| Source::SQLx.to_error(500, e, "Select PreOrderItem"))
}

pub async fn select_pre_order_to(pre_order_master_id: u32, pool: &Pool<MySql>, kphis: &str) -> Result<Vec<PreOrderSave>, AppError> {
    let select_sql = pre_order::order::select_pre_order_to(kphis);
    sqlx::query(AssertSqlSafe(select_sql))
        .bind(pre_order_master_id)
        .fetch_all(pool)
        .await
        .map_err(|e| Source::SQLx.to_error(500, e, "Select PreOrder"))?
        .iter()
        .map(PreOrderSave::from_row)
        .collect::<sqlx::Result<Vec<PreOrderSave>>>()
        .map_err(|e| Source::SQLx.to_error(500, e, "Select PreOrder"))
}

pub async fn select_pre_order_item_to(order_ids: &[u32], pool: &Pool<MySql>, kphis: &str) -> Result<Vec<OrderItemSave>, AppError> {
    let select_item_sql = pre_order::order::select_pre_order_item_to(order_ids, kphis);
    sqlx::query(AssertSqlSafe(select_item_sql))
        .fetch_all(pool)
        .await
        .map_err(|e| Source::SQLx.to_error(500, e, "Select PreOrderItem"))?
        .iter()
        .map(order_item_save_from_row)
        .collect::<sqlx::Result<Vec<OrderItemSave>>>()
        .map_err(|e| Source::SQLx.to_error(500, e, "Select PreOrderItem"))
}
fn order_item_save_from_row(row: &MySqlRow) -> sqlx::Result<OrderItemSave> {
    Ok(OrderItemSave {
        order_id: row.try_get("order_id")?,
        order_item_type: row.try_get("order_item_type")?,
        order_item_detail: row.try_get("order_item_detail")?,
        stat: row.try_get("stat")?,
        off_order_item_id: row.try_get("off_order_item_id")?,
        icode: row.try_get("icode")?,
        med_reconciliation_item_id: None,
        first_qty: None,
        qty: None,
    })
}

// ipd-dr-pre-order-one-day-save.php
// ipd-dr-pre-order-continuous-save.php
pub async fn post_order(save: &PreOrderSave, user: &str, pool: &Pool<MySql>, kphis: &str) -> Result<(u32, Vec<ExecuteResponse>), AppError> {
    if save.pre_order_master_id > 0 {
        let order_id;
        let mut results = Vec::with_capacity(2 + save.order_items.len());
        if save.order_id > 0 {
            order_id = save.order_id;
            let update_result = update_pre_order(order_id, &save.order_doctor, user, pool, kphis).await?;
            results.push(ExecuteResponse::from_query_result(update_result, "Update PreOrder"));
            let delete_result = delete_pre_order_item(save.order_id, pool, kphis).await?;
            results.push(ExecuteResponse::from_query_result(delete_result, "Delete PreOrder"));
        } else {
            let insert_result = insert_pre_order(save.pre_order_master_id, save, user, pool, kphis).await?;
            order_id = insert_result.last_insert_id() as u32;
            results.push(ExecuteResponse::from_query_result(insert_result, "Insert PreOrder"));
        }
        if order_id > 0 {
            for order_item in save.order_items.iter() {
                let insert_order_item_result = insert_pre_order_item(order_id, save.pre_order_master_id, order_item, user, pool, kphis).await?;
                results.push(ExecuteResponse::from_query_result(insert_order_item_result, "Insert PreOrderItem"));
            }
        }

        Ok((order_id, results))
    } else {
        Err(AppError::app_400("Post PreOrder"))
    }
}

async fn insert_pre_order(pre_order_master_id: u32, save: &PreOrderSave, user: &str, pool: &Pool<MySql>, kphis: &str) -> Result<MySqlQueryResult, AppError> {
    let insert_sql = pre_order::order::insert_pre_order(kphis);
    sqlx::query(AssertSqlSafe(insert_sql))
        .bind(pre_order_master_id)
        .bind(&save.order_doctor)
        .bind(&save.order_type)
        .bind(&save.order_owner_type)
        .bind(user)
        .bind(user)
        .execute(pool)
        .await
        .map_err(|e| Source::SQLx.to_error(500, e, "Insert PreOrder"))
}

pub async fn insert_many_pre_orders(pre_orders: &[PreOrderSave], pre_order_master_id: u32, loginname: &str, doctorcode: &str, pool: &Pool<MySql>, kphis: &str) -> Result<Vec<u64>, AppError> {
    let insert_order_sql = pre_order::order::insert_many_pre_orders(pre_orders, loginname, pre_order_master_id, doctorcode, kphis);
    let res = pool
        .execute_many(AssertSqlSafe(insert_order_sql))
        .collect::<Vec<_>>()
        .await
        .into_iter()
        .collect::<sqlx::Result<Vec<MySqlQueryResult>>>()
        .map_err(|e| Source::SQLx.to_error(500, e, "Insert Template to PreOrder"))?
        .into_iter()
        .map(|r| r.last_insert_id())
        .collect::<Vec<u64>>();
    Ok(res)
}

async fn insert_pre_order_item(order_id: u32, pre_order_master_id: u32, order_item: &OrderItemSave, user: &str, pool: &Pool<MySql>, kphis: &str) -> Result<MySqlQueryResult, AppError> {
    let insert_order_item_sql = pre_order::order::insert_pre_order_item(kphis);
    sqlx::query(AssertSqlSafe(insert_order_item_sql))
        .bind(order_id)
        .bind(pre_order_master_id)
        .bind(&order_item.order_item_type)
        .bind(&order_item.order_item_detail)
        .bind(&order_item.stat)
        .bind(order_item.off_order_item_id)
        .bind(&order_item.icode)
        .bind(user)
        .bind(user)
        .execute(pool)
        .await
        .map_err(|e| Source::SQLx.to_error(500, e, "Insert PreOrderItem"))
}

pub async fn insert_pre_order_items(
    pre_order_items: &[OrderItemSave],
    order_id_map: &HashMap<u32, u64>,
    pre_order_master_id: u32,
    loginname: &str,
    pool: &Pool<MySql>,
    kphis: &str,
) -> Result<MySqlQueryResult, AppError> {
    let insert_order_item_sql = pre_order::order::insert_pre_order_items(pre_order_items, order_id_map, pre_order_master_id, loginname, kphis);
    sqlx::query(AssertSqlSafe(insert_order_item_sql))
        .execute(pool)
        .await
        .map_err(|e| Source::SQLx.to_error(500, e, "Insert TemplateItem to PreOrderItem"))
}

pub async fn insert_many_ipd_orders(pre_orders: &[PreOrderSave], an: &str, loginname: &str, doctorcode: &str, pool: &Pool<MySql>, kphis: &str) -> Result<Vec<u64>, AppError> {
    let insert_order_sql = order::insert_many_orders(pre_orders, an, loginname, doctorcode, kphis);
    let res = pool
        .execute_many(AssertSqlSafe(insert_order_sql))
        .collect::<Vec<_>>()
        .await
        .into_iter()
        .collect::<sqlx::Result<Vec<MySqlQueryResult>>>()
        .map_err(|e| Source::SQLx.to_error(500, e, "Insert IPD Orders"))?
        .into_iter()
        .map(|r| r.last_insert_id())
        .collect::<Vec<u64>>();
    Ok(res)
}

pub async fn insert_many_ipd_orders_with_pre(pre_orders: &[PreOrderSave], an: &str, loginname: &str, pool: &Pool<MySql>, kphis: &str) -> Result<Vec<u64>, AppError> {
    let insert_order_sql = order::insert_many_orders_with_pre(pre_orders, an, loginname, kphis);
    let res = pool
        .execute_many(AssertSqlSafe(insert_order_sql))
        .collect::<Vec<_>>()
        .await
        .into_iter()
        .collect::<sqlx::Result<Vec<MySqlQueryResult>>>()
        .map_err(|e| Source::SQLx.to_error(500, e, "Insert PreOrder To Order"))?
        .into_iter()
        .map(|r| r.last_insert_id())
        .collect::<Vec<u64>>();
    Ok(res)
}

pub async fn insert_ipd_order_items(
    pre_order_items: &[OrderItemSave],
    order_id_map: &HashMap<u32, u64>,
    an: &str,
    loginname: &str,
    pool: &Pool<MySql>,
    kphis: &str,
) -> Result<MySqlQueryResult, AppError> {
    let insert_order_item_sql = order::insert_pre_to_order_items(pre_order_items, order_id_map, an, loginname, kphis);
    sqlx::query(AssertSqlSafe(insert_order_item_sql))
        .execute(pool)
        .await
        .map_err(|e| Source::SQLx.to_error(500, e, "Insert PreOrderItem to OrderItem"))
}

pub async fn insert_many_opd_er_orders_with_pre(
    pre_orders: &[PreOrderSave],
    opd_er_order_master_id: u32,
    loginname: &str,
    doctorcode: &str,
    pool: &Pool<MySql>,
    kphis: &str,
) -> Result<Vec<u64>, AppError> {
    let insert_order_sql = opd_er::order::insert_many_orders_with_pre(pre_orders, opd_er_order_master_id, loginname, doctorcode, kphis);
    let res = pool
        .execute_many(AssertSqlSafe(insert_order_sql))
        .collect::<Vec<_>>()
        .await
        .into_iter()
        .collect::<sqlx::Result<Vec<MySqlQueryResult>>>()
        .map_err(|e| Source::SQLx.to_error(500, e, "Insert IPD Orders"))?
        .into_iter()
        .map(|r| r.last_insert_id())
        .collect::<Vec<u64>>();
    Ok(res)
}

pub async fn insert_opd_er_order_items(
    pre_order_items: &[OrderItemSave],
    order_id_map: &HashMap<u32, u64>,
    opd_er_order_master_id: u32,
    loginname: &str,
    pool: &Pool<MySql>,
    kphis: &str,
) -> Result<MySqlQueryResult, AppError> {
    let insert_order_item_sql = opd_er::order::insert_pre_to_order_items(pre_order_items, order_id_map, opd_er_order_master_id, loginname, kphis);
    sqlx::query(AssertSqlSafe(insert_order_item_sql))
        .execute(pool)
        .await
        .map_err(|e| Source::SQLx.to_error(500, e, "Insert OpdEr OrderItems"))
}

async fn update_pre_order(order_id: u32, order_doctor: &str, user: &str, pool: &Pool<MySql>, kphis: &str) -> Result<MySqlQueryResult, AppError> {
    let update_sql = pre_order::order::update_pre_order(kphis);
    sqlx::query(AssertSqlSafe(update_sql))
        .bind(order_doctor)
        .bind(user)
        .bind(order_id)
        .execute(pool)
        .await
        .map_err(|e| Source::SQLx.to_error(500, e, "Update PreOrder"))
}

// ipd-dr-pre-order-one-day-delete.php, ipd-dr-pre-order-continuous-delete.php
/// delete ipd_pre_order and ipd_pre_order_item
pub async fn delete_pre_order(order_id: u32, pool: &Pool<MySql>, kphis: &str) -> Result<ExecuteResponse, AppError> {
    let sql = pre_order::order::delete_pre_order(kphis);
    let delete_result = sqlx::query(AssertSqlSafe(sql))
        .bind(order_id)
        .execute(pool)
        .await
        .map_err(|e| Source::SQLx.to_error(500, e, "Delete PreOrder"))?;

    Ok(ExecuteResponse::from_query_result(delete_result, "Delete PreOrder"))
}

pub async fn delete_pre_order_by_master_id_inner(pre_order_master_id: u32, pool: &Pool<MySql>, kphis: &str) -> Result<MySqlQueryResult, AppError> {
    let delete_pre_order_sql = pre_order::order::delete_pre_order_by_master_id(kphis);
    sqlx::query(AssertSqlSafe(delete_pre_order_sql))
        .bind(pre_order_master_id)
        .execute(pool)
        .await
        .map_err(|e| Source::SQLx.to_error(500, e, "Delete PreOrder"))
}

pub async fn delete_pre_order_item_by_master_id(pre_order_master_id: u32, pool: &Pool<MySql>, kphis: &str) -> Result<MySqlQueryResult, AppError> {
    let delete_pre_order_item_sql = pre_order::order::delete_pre_order_item_by_master_id(kphis);
    sqlx::query(AssertSqlSafe(delete_pre_order_item_sql))
        .bind(pre_order_master_id)
        .execute(pool)
        .await
        .map_err(|e| Source::SQLx.to_error(500, e, "Delete PreOrderItem"))
}

async fn delete_pre_order_item(order_id: u32, pool: &Pool<MySql>, kphis: &str) -> Result<MySqlQueryResult, AppError> {
    let delete_sql = pre_order::order::delete_pre_order_item(kphis);
    sqlx::query(AssertSqlSafe(delete_sql))
        .bind(order_id)
        .execute(pool)
        .await
        .map_err(|e| Source::SQLx.to_error(500, e, "Delete PreOrder"))
}

#[cfg(test)]
#[rustfmt::skip]
mod tests {

    use super::*;
    use kphis_sqlx_tester::MySqlTester;

    #[tokio::test]
    #[ignore]
    async fn sqlx_get_order() {
        let tester = MySqlTester::new_hosxp_and_kphis().await;
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis/ipd_pre_order.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis/system_ac_role.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis/system_acc_role_user.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/hosxp/doctor.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/hosxp/opduser.sql")).execute(&tester.db_pool).await.unwrap();

        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/kphis/ipd_pre_order.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/kphis/system_ac_role.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/kphis/system_acc_role_user.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/hosxp/doctor.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/hosxp/opduser.sql")).execute(&tester.db_pool).await.unwrap();

        // MUST have 'pre_order_master_id' or 'order_id' in params
        let default = get_order(&PreOrderParams::default(),&[String::from("DOCTOR_INTERN")],&tester.db_pool,&tester.hosxp,&tester.kphis).await.unwrap();
        assert!(default.is_empty());
        let found_order_id = get_order(&PreOrderParams {order_id: Some(1),..Default::default()},&[String::from("DOCTOR_INTERN")],&tester.db_pool,&tester.hosxp,&tester.kphis).await.unwrap();
        assert_eq!(found_order_id.len(), 1);
        let found_pre_order_master_id = get_order(&PreOrderParams {pre_order_master_id: Some(1),..Default::default()},&[String::from("DOCTOR_INTERN")],&tester.db_pool,&tester.hosxp,&tester.kphis).await.unwrap();
        assert_eq!(found_pre_order_master_id.len(), 6);
        // test by subset with 'pre_order_master_id' == 1
        let found_order_type = get_order(&PreOrderParams {order_type: Some(String::from("oneday")),pre_order_master_id: Some(1),..Default::default()},&[String::from("DOCTOR_INTERN")],&tester.db_pool,&tester.hosxp,&tester.kphis).await.unwrap();
        assert_eq!(found_order_type.len(), 4);
        let found_order_confirm = get_order(&PreOrderParams {order_confirm: Some(String::from("Y")),pre_order_master_id: Some(1),..Default::default()},&[String::from("DOCTOR_INTERN")],&tester.db_pool,&tester.hosxp,&tester.kphis).await.unwrap();
        assert_eq!(found_order_confirm.len(), 3);
        let found_order_owner_type = get_order(&PreOrderParams {order_owner_type: Some(String::from("nurse")),pre_order_master_id: Some(1),..Default::default()},&[String::from("DOCTOR_INTERN")],&tester.db_pool,&tester.hosxp,&tester.kphis).await.unwrap();
        assert_eq!(found_order_owner_type.len(), 2);
        let found_view_by_doctor = get_order(&PreOrderParams {view_by: Some(String::from("doctor")),pre_order_master_id: Some(1),..Default::default()},&[String::from("DOCTOR_INTERN")],&tester.db_pool,&tester.hosxp,&tester.kphis).await.unwrap();
        assert_eq!(found_view_by_doctor.len(), 4);
        let found_view_by_nurse = get_order(&PreOrderParams {view_by: Some(String::from("nurse")),pre_order_master_id: Some(1),..Default::default()},&[String::from("DOCTOR_INTERN")],&tester.db_pool,&tester.hosxp,&tester.kphis).await.unwrap();
        assert_eq!(found_view_by_nurse.len(), 4);
        let found_view_by_pharm = get_order(&PreOrderParams {view_by: Some(String::from("pharmacist")),pre_order_master_id: Some(1),..Default::default()},&[String::from("DOCTOR_INTERN")],&tester.db_pool,&tester.hosxp,&tester.kphis).await.unwrap();
        assert_eq!(found_view_by_pharm.len(), 3);
        let found_view_by_other = get_order(&PreOrderParams {view_by: Some(String::from("other")),pre_order_master_id: Some(1),..Default::default()},&[String::from("DOCTOR_INTERN")],&tester.db_pool,&tester.hosxp,&tester.kphis).await.unwrap();
        assert_eq!(found_view_by_other.len(), 3);
        let view_by_unknown = get_order(&PreOrderParams {view_by: Some(String::from("xxxx")),pre_order_master_id: Some(1),..Default::default()},&[String::from("DOCTOR_INTERN")],&tester.db_pool,&tester.hosxp,&tester.kphis).await.unwrap();
        assert!(view_by_unknown.is_empty());
    }

    #[tokio::test]
    #[ignore]
    async fn sqlx_get_order_types() {
        let tester = MySqlTester::new_kphis().await;
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis/ipd_pre_order.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis/ipd_pre_order_item.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis/ipd_order_item_type.sql")).execute(&tester.db_pool).await.unwrap();

        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/kphis/ipd_pre_order.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/kphis/ipd_pre_order_item.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/kphis/ipd_order_item_type.sql")).execute(&tester.db_pool).await.unwrap();

        let found = get_order_types(&[1], "oneday", &tester.db_pool, &tester.kphis).await.unwrap();
        assert_eq!(found.len(), 1);
        let not_found = get_order_types(&[], "continuous", &tester.db_pool, &tester.kphis).await.unwrap();
        assert!(not_found.is_empty());
    }

    #[tokio::test]
    #[ignore]
    async fn sqlx_get_order_item() {
        let tester = MySqlTester::new_hosxp_and_kphis().await;
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis/ipd_pre_order_master.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis/ipd_pre_order.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis/ipd_pre_order_item.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/hosxp/drugitems.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/hosxp/opd_allergy.sql")).execute(&tester.db_pool).await.unwrap();

        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/kphis/ipd_pre_order_master.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/kphis/ipd_pre_order.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/kphis/ipd_pre_order_item.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/hosxp/drugitems.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/hosxp/opd_allergy.sql")).execute(&tester.db_pool).await.unwrap();

        let found = get_order_item(1,"med","oneday",&tester.db_pool,&tester.hosxp,&tester.kphis).await.unwrap();
        assert_eq!(found.len(), 2);
        let not_found = get_order_item(1,"med","continuous",&tester.db_pool,&tester.hosxp,&tester.kphis).await.unwrap();
        assert!(not_found.is_empty());
    }

    #[tokio::test]
    #[ignore]
    async fn sqlx_select_pre_order_to() {
        let tester = MySqlTester::new_kphis().await;
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis/ipd_pre_order.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/kphis/ipd_pre_order.sql")).execute(&tester.db_pool).await.unwrap();

        let found = select_pre_order_to(1, &tester.db_pool, &tester.kphis).await.unwrap();
        assert_eq!(found.len(), 6);
        let not_found = select_pre_order_to(999, &tester.db_pool, &tester.kphis).await.unwrap();
        assert!(not_found.is_empty());
    }

    #[tokio::test]
    #[ignore]
    async fn sqlx_select_pre_order_item_to() {
        let tester = MySqlTester::new_kphis().await;
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis/ipd_pre_order_item.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/kphis/ipd_pre_order_item.sql")).execute(&tester.db_pool).await.unwrap();

        let found = select_pre_order_item_to(&[1], &tester.db_pool, &tester.kphis).await.unwrap();
        assert_eq!(found.len(), 2);
        let not_found = select_pre_order_item_to(&[], &tester.db_pool, &tester.kphis).await.unwrap();
        assert!(not_found.is_empty());
    }

    #[tokio::test]
    #[ignore]
    async fn sqlx_insert_pre_order() {
        let tester = MySqlTester::new_kphis().await;
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis/ipd_pre_order.sql")).execute(&tester.db_pool).await.unwrap();

        let success = insert_pre_order(1,&PreOrderSave::demo(),"user",&tester.db_pool,&tester.kphis).await.unwrap();
        assert_eq!(success.rows_affected(), 1);
        let again_success = insert_pre_order(1,&PreOrderSave::demo(),"user",&tester.db_pool,&tester.kphis).await.unwrap();
        assert_eq!(again_success.rows_affected(), 1);
    }

    #[tokio::test]
    #[ignore]
    async fn sqlx_insert_many_pre_orders() {
        let tester = MySqlTester::new_kphis().await;
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis/ipd_pre_order.sql")).execute(&tester.db_pool).await.unwrap();

        let success = insert_many_pre_orders(&[PreOrderSave::demo(),PreOrderSave::demo()],1,"user","007",&tester.db_pool,&tester.kphis).await.unwrap();
        assert_eq!(success.len(), 2);
        let again_success = insert_many_pre_orders(&[PreOrderSave::demo(),PreOrderSave::demo()],1,"user","007",&tester.db_pool,&tester.kphis).await.unwrap();
        assert_eq!(again_success.len(), 2);
    }

    #[tokio::test]
    #[ignore]
    async fn sqlx_insert_pre_order_item() {
        let tester = MySqlTester::new_kphis().await;
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis/ipd_pre_order_item.sql")).execute(&tester.db_pool).await.unwrap();

        let success = insert_pre_order_item(1,1,&OrderItemSave::demo(),"user",&tester.db_pool,&tester.kphis).await.unwrap();
        assert_eq!(success.rows_affected(), 1);
        let again_success = insert_pre_order_item(1,1,&OrderItemSave::demo(),"user",&tester.db_pool,&tester.kphis).await.unwrap();
        assert_eq!(again_success.rows_affected(), 1);
    }

    #[tokio::test]
    #[ignore]
    async fn sqlx_insert_pre_order_items() {
        let tester = MySqlTester::new_kphis().await;
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis/ipd_pre_order_item.sql")).execute(&tester.db_pool).await.unwrap();

        let mut order_id_map = HashMap::new();
        order_id_map.insert(1, 1);
        let success = insert_pre_order_items(&[OrderItemSave::demo()],&order_id_map,1,"user",&tester.db_pool,&tester.kphis).await.unwrap();
        assert_eq!(success.rows_affected(), 1);
        let again_success = insert_pre_order_items(&[OrderItemSave::demo()],&order_id_map,1,"user",&tester.db_pool,&tester.kphis).await.unwrap();
        assert_eq!(again_success.rows_affected(), 1);
    }

    #[tokio::test]
    #[ignore]
    async fn sqlx_insert_many_ipd_orders() {
        let tester = MySqlTester::new_kphis().await;
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis/ipd_order.sql")).execute(&tester.db_pool).await.unwrap();

        let success = insert_many_ipd_orders(&[PreOrderSave::demo(),PreOrderSave::demo()],"660001234","user","007",&tester.db_pool,&tester.kphis).await.unwrap();
        assert_eq!(success.len(), 2);
        let again_success = insert_many_ipd_orders(&[PreOrderSave::demo(),PreOrderSave::demo()],"660001234","user","007",&tester.db_pool,&tester.kphis).await.unwrap();
        assert_eq!(again_success.len(), 2);
    }

    #[tokio::test]
    #[ignore]
    async fn sqlx_insert_many_ipd_orders_with_pre() {
        let tester = MySqlTester::new_kphis().await;
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis/ipd_order.sql")).execute(&tester.db_pool).await.unwrap();

        let success = insert_many_ipd_orders_with_pre(&[PreOrderSave::demo(),PreOrderSave::demo()],"660001234","user",&tester.db_pool,&tester.kphis).await.unwrap();
        assert_eq!(success.len(), 2);
        let again_success = insert_many_ipd_orders_with_pre(&[PreOrderSave::demo(),PreOrderSave::demo()],"660001234","user",&tester.db_pool,&tester.kphis).await.unwrap();
        assert_eq!(again_success.len(), 2);
    }

    #[tokio::test]
    #[ignore]
    async fn sqlx_insert_ipd_order_items() {
        let tester = MySqlTester::new_kphis().await;
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis/ipd_order_item.sql")).execute(&tester.db_pool).await.unwrap();

        let mut order_id_map = HashMap::new();
        order_id_map.insert(1, 1);
        let success = insert_ipd_order_items(&[OrderItemSave::demo()],&order_id_map,"660001234","user",&tester.db_pool,&tester.kphis).await.unwrap();
        assert_eq!(success.rows_affected(), 1);
        let again_success = insert_ipd_order_items(&[OrderItemSave::demo()],&order_id_map,"660001234","user",&tester.db_pool,&tester.kphis).await.unwrap();
        assert_eq!(again_success.rows_affected(), 1);
    }

    #[tokio::test]
    #[ignore]
    async fn sqlx_insert_many_opd_er_orders_with_pre() {
        let tester = MySqlTester::new_kphis().await;
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis/opd_er_order.sql")).execute(&tester.db_pool).await.unwrap();

        let success = insert_many_opd_er_orders_with_pre(&[PreOrderSave::demo(),PreOrderSave::demo()],1,"user","007",&tester.db_pool,&tester.kphis).await.unwrap();
        assert_eq!(success.len(), 2);
        let again_success = insert_many_opd_er_orders_with_pre(&[PreOrderSave::demo(),PreOrderSave::demo()],1,"user","007",&tester.db_pool,&tester.kphis).await.unwrap();
        assert_eq!(again_success.len(), 2);
    }

    #[tokio::test]
    #[ignore]
    async fn sqlx_insert_opd_er_order_items() {
        let tester = MySqlTester::new_kphis().await;
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis/opd_er_order_item.sql")).execute(&tester.db_pool).await.unwrap();

        let mut order_id_map = HashMap::new();
        order_id_map.insert(1, 1);
        let success = insert_opd_er_order_items(&[OrderItemSave::demo()],&order_id_map,1,"user",&tester.db_pool,&tester.kphis).await.unwrap();
        assert_eq!(success.rows_affected(), 1);
        let again_success = insert_opd_er_order_items(&[OrderItemSave::demo()],&order_id_map,1,"user",&tester.db_pool,&tester.kphis).await.unwrap();
        assert_eq!(again_success.rows_affected(), 1);
    }

    #[tokio::test]
    #[ignore]
    async fn sqlx_update_pre_order() {
        let tester = MySqlTester::new_kphis().await;
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis/ipd_pre_order.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/kphis/ipd_pre_order.sql")).execute(&tester.db_pool).await.unwrap();

        let success = update_pre_order(1, "007", "user", &tester.db_pool, &tester.kphis).await.unwrap();
        assert_eq!(success.rows_affected(), 1);
        let again_success = update_pre_order(1, "007", "user", &tester.db_pool, &tester.kphis).await.unwrap();
        assert_eq!(again_success.rows_affected(), 1);
    }

    #[tokio::test]
    #[ignore]
    async fn sqlx_delete_pre_order() {
        let tester = MySqlTester::new_kphis().await;
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis/ipd_pre_order.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis/ipd_pre_order_item.sql")).execute(&tester.db_pool).await.unwrap();

        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/kphis/ipd_pre_order.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/kphis/ipd_pre_order_item.sql")).execute(&tester.db_pool).await.unwrap();

        let success = delete_pre_order(1, &tester.db_pool, &tester.kphis).await.unwrap();
        assert_eq!(success.rows_affected, 3);
        let again_not_found = delete_pre_order(1, &tester.db_pool, &tester.kphis).await.unwrap();
        assert_eq!(again_not_found.rows_affected, 0);
    }

    #[tokio::test]
    #[ignore]
    async fn sqlx_delete_pre_order_by_master_id_inner() {
        let tester = MySqlTester::new_kphis().await;
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis/ipd_pre_order.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/kphis/ipd_pre_order.sql")).execute(&tester.db_pool).await.unwrap();

        let success = delete_pre_order_by_master_id_inner(1, &tester.db_pool, &tester.kphis).await.unwrap();
        assert_eq!(success.rows_affected(), 6);
        let again_not_found = delete_pre_order_by_master_id_inner(1, &tester.db_pool, &tester.kphis).await.unwrap();
        assert_eq!(again_not_found.rows_affected(), 0);
    }

    #[tokio::test]
    #[ignore]
    async fn sqlx_delete_pre_order_item_by_master_id() {
        let tester = MySqlTester::new_kphis().await;
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis/ipd_pre_order_item.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/kphis/ipd_pre_order_item.sql")).execute(&tester.db_pool).await.unwrap();

        let success = delete_pre_order_item_by_master_id(1, &tester.db_pool, &tester.kphis).await.unwrap();
        assert_eq!(success.rows_affected(), 2);
        let again_not_found = delete_pre_order_item_by_master_id(1, &tester.db_pool, &tester.kphis).await.unwrap();
        assert_eq!(again_not_found.rows_affected(), 0);
    }

    #[tokio::test]
    #[ignore]
    async fn sqlx_delete_pre_order_item() {
        let tester = MySqlTester::new_kphis().await;
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis/ipd_pre_order_item.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/kphis/ipd_pre_order_item.sql")).execute(&tester.db_pool).await.unwrap();

        let success = delete_pre_order_item(1, &tester.db_pool, &tester.kphis).await.unwrap();
        assert_eq!(success.rows_affected(), 2);
        let again_not_found = delete_pre_order_item(1, &tester.db_pool, &tester.kphis).await.unwrap();
        assert_eq!(again_not_found.rows_affected(), 0);
    }
}
