use sqlx::{AssertSqlSafe, FromRow, MySql, Pool, Row, mysql::MySqlQueryResult};

use kphis_model::{
    fetch::ExecuteResponse,
    pre_order::master::{PreOrderMaster, PreOrderMasterParams, PreOrderMasterSave},
};
use kphis_sql::pre_order;
use kphis_util::error::{AppError, Source};

use super::{order, progress_note};

// ipd-dr-pre-order-list-data.php
pub async fn get_pre_order_list(params: &PreOrderMasterParams, intern_roles: &[String], pool: &Pool<MySql>, hosxp: &str, kphis: &str) -> Result<Vec<PreOrderMaster>, AppError> {
    let sql = pre_order::master::select_pre_order_list(params, intern_roles, hosxp, kphis);

    let mut query = sqlx::query(AssertSqlSafe(sql));

    if let Some(pre_order_master_id) = params.pre_order_master_id.as_ref() {
        query = query.bind(pre_order_master_id);
    }
    if let Some(hn) = params.hn.as_ref() {
        query = query.bind(hn);
    }
    if let Some(start_order_date) = params.start_order_date.as_ref() {
        query = query.bind(start_order_date);
    }
    if let Some(end_order_date) = params.end_order_date.as_ref() {
        query = query.bind(end_order_date);
    }
    if let Some(order_doctor) = params.order_doctor.as_ref() {
        query = query.bind(order_doctor);
    }
    if let Some(pre_order_type) = params.pre_order_type.as_ref() {
        if pre_order_type != "pre_order" {
            query = query.bind(pre_order_type);
        }
    };
    if let Some(template_name) = params.template_name.as_ref() {
        query = query.bind(["%", template_name.trim(), "%"].concat());
    }
    if let Some(used) = params.used.as_ref() {
        query = query.bind(used);
    }

    query
        .fetch_all(pool)
        .await
        .map(|rows| {
            rows.iter()
                .map(PreOrderMaster::from_row)
                .collect::<sqlx::Result<Vec<PreOrderMaster>>>()
                .map_err(|e| Source::SQLx.to_error(500, e, "Select PreOrderList"))
        })
        .map_err(|e| Source::SQLx.to_error(500, e, "Select PreOrderList"))?
}

pub async fn select_pre_order_used_by_master_id(pre_order_master_id: u32, pool: &Pool<MySql>, kphis: &str) -> Result<Option<bool>, AppError> {
    let select_used = pre_order::master::select_pre_order_used_by_master_id(kphis);
    let res = sqlx::query(AssertSqlSafe(select_used))
        .bind(pre_order_master_id)
        .fetch_optional(pool)
        .await
        .map_err(|e| Source::SQLx.to_error(500, e, "Select PreOrder Used"))?
        .map(|row| {
            let used_text: String = row.try_get("used").unwrap_or_default();
            used_text.as_str() == "Y"
        });
    Ok(res)
}

// ipd-dr-pre-order-master-save.php
pub async fn post_pre_order_master(save: &PreOrderMasterSave, user: &str, pool: &Pool<MySql>, kphis: &str) -> Result<(u32, ExecuteResponse), AppError> {
    // $IPD_ORDER_TEMPLATE_SHARE = SessionManager::checkPermission('IPD_ORDER_TEMPLATE', 'SHARE');
    // TODO if not $IPD_ORDER_TEMPLATE_SHARE -> save.shared_template = None;
    if let Some(pre_order_master_id) = save.pre_order_master_id {
        if pre_order_master_id > 0 {
            update_pre_order_master(save, pre_order_master_id, user, pool, kphis).await
        } else {
            insert_pre_order_master(save, user, pool, kphis).await
        }
    } else {
        insert_pre_order_master(save, user, pool, kphis).await
    }
}

async fn insert_pre_order_master(save: &PreOrderMasterSave, user: &str, pool: &Pool<MySql>, kphis: &str) -> Result<(u32, ExecuteResponse), AppError> {
    let sql = pre_order::master::insert_pre_order_master(kphis);
    let result = sqlx::query(AssertSqlSafe(sql))
        .bind(&save.hn)
        .bind(save.order_for_date)
        .bind(save.order_for_time)
        .bind(&save.order_doctor)
        .bind(&save.pre_order_type)
        .bind(&save.template_name)
        .bind(&save.shared_template)
        .bind(user)
        .bind(user)
        .execute(pool)
        .await
        .map_err(|e| Source::SQLx.to_error(500, e, "Insert PerOrderMaster"))?;

    Ok((result.last_insert_id() as u32, ExecuteResponse::from_query_result(result, " Insert PerOrderMaster")))
}

async fn update_pre_order_master(save: &PreOrderMasterSave, pre_order_master_id: u32, user: &str, pool: &Pool<MySql>, kphis: &str) -> Result<(u32, ExecuteResponse), AppError> {
    let sql = pre_order::master::update_pre_order_master(kphis);
    let result = sqlx::query(AssertSqlSafe(sql))
        .bind(&save.hn)
        .bind(save.order_for_date)
        .bind(save.order_for_time)
        .bind(&save.template_name)
        .bind(&save.shared_template)
        .bind(user)
        .bind(pre_order_master_id)
        .execute(pool)
        .await
        .map_err(|e| Source::SQLx.to_error(500, e, "Update PreOrderMaster"))?;

    Ok((pre_order_master_id, ExecuteResponse::from_query_result(result, "Update PreOrderMaster")))
}

pub async fn update_pre_order_master_used(pre_order_master_id: u32, loginname: &str, pool: &Pool<MySql>, kphis: &str) -> Result<MySqlQueryResult, AppError> {
    let update_used = pre_order::master::update_pre_order_master_used(kphis);
    sqlx::query(AssertSqlSafe(update_used))
        .bind(loginname)
        .bind(pre_order_master_id)
        .execute(pool)
        .await
        .map_err(|e| Source::SQLx.to_error(500, e, "Update PreOrderMaster used"))
}

// ipd-dr-pre-order-master-delete.php
pub async fn delete_pre_order_by_master_id(pre_order_master_id: u32, pool: &Pool<MySql>, kphis: &str) -> Result<Vec<ExecuteResponse>, AppError> {
    let used_opt = select_pre_order_used_by_master_id(pre_order_master_id, pool, kphis).await?;
    if used_opt.unwrap_or_default() {
        Ok(vec![ExecuteResponse {
            last_insert_id: 0,
            rows_affected: 0,
            error: Some(String::from("Used")),
            action: Some(String::from("Delete PreOrder")),
        }])
    } else {
        let mut results = Vec::with_capacity(5);

        let delete_pre_order_item_result = order::delete_pre_order_item_by_master_id(pre_order_master_id, pool, kphis).await?;
        results.push(ExecuteResponse::from_query_result(delete_pre_order_item_result, "Delete PreOrderItem"));

        let delete_pre_order_result = order::delete_pre_order_by_master_id_inner(pre_order_master_id, pool, kphis).await?;
        results.push(ExecuteResponse::from_query_result(delete_pre_order_result, "Delete PreOrder"));

        let delete_progress_note_item_result = progress_note::delete_progress_note_item_by_master_id(pre_order_master_id, pool, kphis).await?;
        results.push(ExecuteResponse::from_query_result(delete_progress_note_item_result, "Delete ProgressNoteItem"));

        let delete_progress_note_result = progress_note::delete_progress_note_by_master_id(pre_order_master_id, pool, kphis).await?;
        results.push(ExecuteResponse::from_query_result(delete_progress_note_result, "Delete ProgressNoteItem"));

        let delete_pre_order_master_result = delete_pre_order_master_by_master_id(pre_order_master_id, pool, kphis).await?;
        results.push(ExecuteResponse::from_query_result(delete_pre_order_master_result, "Delete PreOrderMaster"));

        Ok(results)
    }
}

async fn delete_pre_order_master_by_master_id(pre_order_master_id: u32, pool: &Pool<MySql>, kphis: &str) -> Result<MySqlQueryResult, AppError> {
    let delete_pre_order_master_sql = pre_order::master::delete_pre_order_master_by_master_id(kphis);
    sqlx::query(AssertSqlSafe(delete_pre_order_master_sql))
        .bind(pre_order_master_id)
        .execute(pool)
        .await
        .map_err(|e| Source::SQLx.to_error(500, e, "Delete PreOrderMaster"))
}

#[cfg(test)]
#[rustfmt::skip]
mod tests {

    use super::*;
    use kphis_sqlx_tester::MySqlTester;
    use kphis_util::datetime::date_8601;

    #[tokio::test]
    #[ignore]
    async fn sqlx_get_pre_order_list() {
        let tester = MySqlTester::new_hosxp_and_kphis().await;
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis/ipd_pre_order_master.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis/system_ac_role.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis/system_acc_role_user.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/hosxp/patient.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/hosxp/doctor.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/hosxp/opduser.sql")).execute(&tester.db_pool).await.unwrap();

        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/kphis/ipd_pre_order_master.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/kphis/system_ac_role.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/kphis/system_acc_role_user.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/hosxp/patient.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/hosxp/doctor.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/hosxp/opduser.sql")).execute(&tester.db_pool).await.unwrap();

        let default = get_pre_order_list(&PreOrderMasterParams::default(),&[String::from("DOCTOR_INTERN")],&tester.db_pool,&tester.hosxp,&tester.kphis).await.unwrap();
        assert_eq!(default.len(), 5);
        let found_pre_order_master_id = get_pre_order_list(&PreOrderMasterParams {pre_order_master_id: Some(1),..Default::default()},&[String::from("DOCTOR_INTERN")],&tester.db_pool,&tester.hosxp,&tester.kphis).await.unwrap();
        assert_eq!(found_pre_order_master_id.len(), 1);
        let found_start_order_date = get_pre_order_list(&PreOrderMasterParams {start_order_date: date_8601("2024-11-11"),..Default::default()},&[String::from("DOCTOR_INTERN")],&tester.db_pool,&tester.hosxp,&tester.kphis).await.unwrap();
        assert_eq!(found_start_order_date.len(), 1);
        let found_end_order_date = get_pre_order_list(&PreOrderMasterParams {end_order_date: date_8601("2024-01-11"),..Default::default()},&[String::from("DOCTOR_INTERN")],&tester.db_pool,&tester.hosxp,&tester.kphis).await.unwrap();
        assert_eq!(found_end_order_date.len(), 1);
        let found_between_order_date = get_pre_order_list(&PreOrderMasterParams {start_order_date: date_8601("2024-01-11"),end_order_date: date_8601("2024-01-11"),..Default::default()},&[String::from("DOCTOR_INTERN")],&tester.db_pool,&tester.hosxp,&tester.kphis).await.unwrap();
        assert_eq!(found_between_order_date.len(), 1);
        let found_order_doctor = get_pre_order_list(&PreOrderMasterParams {order_doctor: Some(String::from("007")),..Default::default()},&[String::from("DOCTOR_INTERN")],&tester.db_pool,&tester.hosxp,&tester.kphis).await.unwrap();
        assert_eq!(found_order_doctor.len(), 4);
        // MUST use with order_doctor, to inclueds shared items of another order_doctor
        let found_include_shared_template = get_pre_order_list(&PreOrderMasterParams {include_shared_template: Some(String::from("Y")),order_doctor: Some(String::from("007")),..Default::default()},&[String::from("DOCTOR_INTERN")],&tester.db_pool,&tester.hosxp,&tester.kphis).await.unwrap();
        assert_eq!(found_include_shared_template.len(), 5);
        let found_pre_order_type = get_pre_order_list(&PreOrderMasterParams {pre_order_type: Some(String::from("opd")),..Default::default()},&[String::from("DOCTOR_INTERN")],&tester.db_pool,&tester.hosxp,&tester.kphis).await.unwrap();
        assert_eq!(found_pre_order_type.len(), 1);
        let found_template_name = get_pre_order_list(&PreOrderMasterParams {template_name: Some(String::from("plate")),..Default::default()},&[String::from("DOCTOR_INTERN")],&tester.db_pool,&tester.hosxp,&tester.kphis).await.unwrap();
        assert_eq!(found_template_name.len(), 3);
        let found_used = get_pre_order_list(&PreOrderMasterParams {used: Some(String::from("Y")),..Default::default()},&[String::from("DOCTOR_INTERN")],&tester.db_pool,&tester.hosxp,&tester.kphis).await.unwrap();
        assert_eq!(found_used.len(), 2);

        let not_found = get_pre_order_list(&PreOrderMasterParams {pre_order_master_id: Some(999),..Default::default()},&[String::from("DOCTOR_INTERN")],&tester.db_pool,&tester.hosxp,&tester.kphis).await.unwrap();
        assert!(not_found.is_empty());
    }

    #[tokio::test]
    #[ignore]
    async fn sqlx_select_pre_order_used_by_master_id() {
        let tester = MySqlTester::new_kphis().await;
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis/ipd_pre_order_master.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/kphis/ipd_pre_order_master.sql")).execute(&tester.db_pool).await.unwrap();

        let found = select_pre_order_used_by_master_id(1, &tester.db_pool, &tester.kphis).await.unwrap();
        assert!(found.is_some());
        let not_found = select_pre_order_used_by_master_id(999, &tester.db_pool, &tester.kphis).await.unwrap();
        assert!(not_found.is_none());
    }

    #[tokio::test]
    #[ignore]
    async fn sqlx_insert_pre_order_master() {
        let tester = MySqlTester::new_kphis().await;
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis/ipd_pre_order_master.sql")).execute(&tester.db_pool).await.unwrap();

        let success = insert_pre_order_master(&PreOrderMasterSave::demo(),"user",&tester.db_pool,&tester.kphis).await.unwrap();
        assert_eq!(success.1.rows_affected, 1);
        let again_success = insert_pre_order_master(&PreOrderMasterSave::demo(),"user",&tester.db_pool,&tester.kphis).await.unwrap();
        assert_eq!(again_success.1.rows_affected, 1);
    }

    #[tokio::test]
    #[ignore]
    async fn sqlx_update_pre_order_master() {
        let tester = MySqlTester::new_kphis().await;
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis/ipd_pre_order_master.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/kphis/ipd_pre_order_master.sql")).execute(&tester.db_pool).await.unwrap();

        let used_failure = update_pre_order_master(&PreOrderMasterSave::demo(),1,"user",&tester.db_pool,&tester.kphis).await.unwrap();
        assert_eq!(used_failure.1.rows_affected, 0);
        let success = update_pre_order_master(&PreOrderMasterSave::demo(),2,"user",&tester.db_pool,&tester.kphis).await.unwrap();
        assert_eq!(success.1.rows_affected, 1);
        let again_success = update_pre_order_master(&PreOrderMasterSave::demo(),2,"user",&tester.db_pool,&tester.kphis).await.unwrap();
        assert_eq!(again_success.1.rows_affected, 1);
    }

    #[tokio::test]
    #[ignore]
    async fn sqlx_update_pre_order_master_used() {
        let tester = MySqlTester::new_kphis().await;
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis/ipd_pre_order_master.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/kphis/ipd_pre_order_master.sql")).execute(&tester.db_pool).await.unwrap();

        let success = update_pre_order_master_used(1, "user", &tester.db_pool, &tester.kphis).await.unwrap();
        assert_eq!(success.rows_affected(), 1);
        let again_success = update_pre_order_master_used(1, "user", &tester.db_pool, &tester.kphis).await.unwrap();
        assert_eq!(again_success.rows_affected(), 1);
    }

    #[tokio::test]
    #[ignore]
    async fn sqlx_delete_pre_order_master_by_master_id() {
        let tester = MySqlTester::new_kphis().await;
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis/ipd_pre_order_master.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/kphis/ipd_pre_order_master.sql")).execute(&tester.db_pool).await.unwrap();

        let success = delete_pre_order_master_by_master_id(1, &tester.db_pool, &tester.kphis).await.unwrap();
        assert_eq!(success.rows_affected(), 1);
        let again_not_found = delete_pre_order_master_by_master_id(1, &tester.db_pool, &tester.kphis).await.unwrap();
        assert_eq!(again_not_found.rows_affected(), 0);
    }
}
