use sqlx::{
    AssertSqlSafe, FromRow, MySql, Pool, Row,
    mysql::{MySqlQueryResult, MySqlRow},
};

use kphis_model::{
    app::VisitTypeId,
    fetch::ExecuteResponse,
    index_monitor::{IndexMonitor, IndexMonitorOnly},
};
use kphis_sql::opd_er::index_monitor;
use kphis_util::error::{AppError, Source};

pub async fn get_index_monitor(action_id: u32, pool: &Pool<MySql>, hosxp: &str, kphis_extra: &str) -> Result<Vec<IndexMonitor>, AppError> {
    let sql = index_monitor::select_index_monitor(hosxp, kphis_extra);
    sqlx::query(AssertSqlSafe(sql))
        .bind(action_id)
        .fetch_all(pool)
        .await
        .map_err(|e| Source::SQLx.to_error(500, e, "Select IndexMonitor"))?
        .iter()
        .map(index_monitor_from_row)
        .collect::<sqlx::Result<Vec<IndexMonitor>>>()
        .map_err(|e| Source::SQLx.to_error(500, e, "Select IndexMonitor"))
}
fn index_monitor_from_row(row: &MySqlRow) -> sqlx::Result<IndexMonitor> {
    let opd_er_order_master_id: u32 = row.try_get("opd_er_order_master_id")?;
    Ok(IndexMonitor {
        visit_type: VisitTypeId::OpdEr(String::new(), opd_er_order_master_id),
        monitor_id: row.try_get("monitor_id")?,
        action_id: row.try_get("action_id")?,
        monitor_datetime: row.try_get("monitor_datetime")?,
        monitor_doctor: row.try_get("monitor_doctor")?,
        monitor_abnormal: row.try_get("monitor_abnormal")?,
        monitor_result: row.try_get("monitor_result")?,
        monitor_remark: row.try_get("monitor_remark")?,
        monitor_doctor_name: row.try_get("monitor_doctor_name")?,
    })
}

pub async fn get_index_monitor_only(action_id: u32, pool: &Pool<MySql>, kphis_extra: &str) -> Result<Vec<IndexMonitorOnly>, AppError> {
    let sql = index_monitor::select_index_monitor_only(kphis_extra);
    sqlx::query(AssertSqlSafe(sql))
        .bind(action_id)
        .fetch_all(pool)
        .await
        .map_err(|e| Source::SQLx.to_error(500, e, "Select IndexMonitorOnly"))?
        .iter()
        .map(IndexMonitorOnly::from_row)
        .collect::<sqlx::Result<Vec<IndexMonitorOnly>>>()
        .map_err(|e| Source::SQLx.to_error(500, e, "Select IndexMonitorOnly"))
}

pub async fn post_index_monitor(save: &IndexMonitor, doctorcode: &str, user: &str, pool: &Pool<MySql>, kphis_extra: &str) -> Result<(u32, Vec<ExecuteResponse>), AppError> {
    let is_update = match save.monitor_id {
        Some(id) => id > 0,
        None => false,
    };
    let monitor_id;
    let mut results = Vec::with_capacity(1);
    if is_update {
        monitor_id = save.monitor_id;
        let update_monitor_result = update_index_monitor(save, doctorcode, user, pool, kphis_extra).await?;
        results.push(ExecuteResponse::from_query_result(update_monitor_result, "Update IndexMonitor"));
    } else {
        let insert_monitor_result = insert_index_monitor(save, doctorcode, user, pool, kphis_extra).await?;
        monitor_id = Some(insert_monitor_result.last_insert_id() as u32);
        results.push(ExecuteResponse::from_query_result(insert_monitor_result, "Insert IndexMonitor"));
    }

    Ok((monitor_id.unwrap_or_default(), results))
}

async fn update_index_monitor(save: &IndexMonitor, doctorcode: &str, user: &str, pool: &Pool<MySql>, kphis_extra: &str) -> Result<MySqlQueryResult, AppError> {
    let update_monitor_sql = index_monitor::update_index_monitor(kphis_extra);
    sqlx::query(AssertSqlSafe(update_monitor_sql))
        .bind(save.monitor_datetime)
        .bind(doctorcode)
        .bind(&save.monitor_abnormal)
        .bind(&save.monitor_result)
        .bind(&save.monitor_remark)
        .bind(user)
        .bind(save.monitor_id.unwrap_or_default())
        .execute(pool)
        .await
        .map_err(|e| Source::SQLx.to_error(500, e, "Update IndexMonitor"))
}

async fn insert_index_monitor(save: &IndexMonitor, doctorcode: &str, user: &str, pool: &Pool<MySql>, kphis_extra: &str) -> Result<MySqlQueryResult, AppError> {
    if let VisitTypeId::OpdEr(_, opd_er_order_master_id) = &save.visit_type {
        let insert_monitor_sql = index_monitor::insert_index_monitor(kphis_extra);
        sqlx::query(AssertSqlSafe(insert_monitor_sql))
            .bind(save.action_id)
            .bind(opd_er_order_master_id)
            .bind(save.monitor_datetime)
            .bind(doctorcode)
            .bind(&save.monitor_abnormal)
            .bind(&save.monitor_result)
            .bind(&save.monitor_remark)
            .bind(user)
            .bind(user)
            .execute(pool)
            .await
            .map_err(|e| Source::SQLx.to_error(500, e, "Insert IndexMonitor"))
    } else {
        Err(AppError::app_400("Insert IndexAction"))
    }
}

pub async fn insert_index_monitors_only(
    action_id: u32,
    opd_er_order_master_id: u32,
    index_monitors_only: &[IndexMonitorOnly],
    pool: &Pool<MySql>,
    kphis_extra: &str,
) -> Result<MySqlQueryResult, AppError> {
    let insert_monitors_sql = index_monitor::insert_index_monitors_only(action_id, opd_er_order_master_id, index_monitors_only, kphis_extra);
    sqlx::query(AssertSqlSafe(insert_monitors_sql))
        .execute(pool)
        .await
        .map_err(|e| Source::SQLx.to_error(500, e, "Insert IndexMonitorOnly"))
}

pub async fn delete_index_monitor(monitor_id: u32, pool: &Pool<MySql>, kphis_extra: &str) -> Result<ExecuteResponse, AppError> {
    let sql = index_monitor::delete_index_monitor(kphis_extra);
    let delete_result = sqlx::query(AssertSqlSafe(sql))
        .bind(monitor_id)
        .execute(pool)
        .await
        .map_err(|e| Source::SQLx.to_error(500, e, "Delete IndexMonitor"))?;

    Ok(ExecuteResponse::from_query_result(delete_result, "Delete IndexMonitor"))
}

#[cfg(test)]
#[rustfmt::skip]
mod tests {

    use super::*;
    use kphis_sqlx_tester::MySqlTester;

    #[tokio::test]
    #[ignore]
    async fn sqlx_get_index_monitor() {
        let tester = MySqlTester::new_hosxp_and_kphis_extra().await;
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis_extra/opd_er_nurse_index_monitor.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/hosxp/doctor.sql")).execute(&tester.db_pool).await.unwrap();

        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/kphis_extra/opd_er_nurse_index_monitor.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/hosxp/doctor.sql")).execute(&tester.db_pool).await.unwrap();

        let found = get_index_monitor(1, &tester.db_pool, &tester.hosxp, &tester.kphis_extra).await.unwrap();
        assert_eq!(found.len(), 2);
        let not_found = get_index_monitor(999, &tester.db_pool, &tester.hosxp, &tester.kphis_extra).await.unwrap();
        assert!(not_found.is_empty());
    }

    #[tokio::test]
    #[ignore]
    async fn sqlx_get_index_monitor_only() {
        let tester = MySqlTester::new_kphis_extra().await;
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis_extra/opd_er_nurse_index_monitor.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/kphis_extra/opd_er_nurse_index_monitor.sql")).execute(&tester.db_pool).await.unwrap();

        let found = get_index_monitor_only(1, &tester.db_pool, &tester.kphis_extra).await.unwrap();
        assert_eq!(found.len(), 2);
        let not_found = get_index_monitor_only(999, &tester.db_pool, &tester.kphis_extra).await.unwrap();
        assert!(not_found.is_empty());
    }

    #[tokio::test]
    #[ignore]
    async fn sqlx_insert_index_monitor() {
        let tester = MySqlTester::new_kphis_extra().await;
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis_extra/opd_er_nurse_index_monitor.sql")).execute(&tester.db_pool).await.unwrap();

        let mut index_monitor = IndexMonitor::demo();
        index_monitor.visit_type = VisitTypeId::OpdEr(String::new(), 1);

        let success = insert_index_monitor(&index_monitor, "008", "user",&tester.db_pool,&tester.kphis_extra).await.unwrap();
        assert_eq!(success.rows_affected(), 1);
        let again_success = insert_index_monitor(&index_monitor,"008", "user",&tester.db_pool,&tester.kphis_extra).await.unwrap();
        assert_eq!(again_success.rows_affected(), 1);
    }

    #[tokio::test]
    #[ignore]
    async fn sqlx_insert_index_monitors_only() {
        let tester = MySqlTester::new_kphis_extra().await;
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis_extra/opd_er_nurse_index_monitor.sql")).execute(&tester.db_pool).await.unwrap();

        let success = insert_index_monitors_only(1,1,&[IndexMonitorOnly::demo()],&tester.db_pool,&tester.kphis_extra).await.unwrap();
        assert_eq!(success.rows_affected(), 1);
        let again_success = insert_index_monitors_only(1,1,&[IndexMonitorOnly::demo()],&tester.db_pool,&tester.kphis_extra).await.unwrap();
        assert_eq!(again_success.rows_affected(), 1);
    }

    #[tokio::test]
    #[ignore]
    async fn sqlx_update_index_monitor() {
        let tester = MySqlTester::new_kphis_extra().await;
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis_extra/opd_er_nurse_index_monitor.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/kphis_extra/opd_er_nurse_index_monitor.sql")).execute(&tester.db_pool).await.unwrap();

        let success = update_index_monitor(&IndexMonitor::demo(),"008", "user",&tester.db_pool,&tester.kphis_extra).await.unwrap();
        assert_eq!(success.rows_affected(), 1);
        let again_success = update_index_monitor(&IndexMonitor::demo(),"008", "user",&tester.db_pool,&tester.kphis_extra).await.unwrap();
        assert_eq!(again_success.rows_affected(), 1);
    }

    #[tokio::test]
    #[ignore]
    async fn sqlx_delete_index_monitor() {
        let tester = MySqlTester::new_kphis_extra().await;
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis_extra/opd_er_nurse_index_monitor.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/kphis_extra/opd_er_nurse_index_monitor.sql")).execute(&tester.db_pool).await.unwrap();

        let success = delete_index_monitor(1, &tester.db_pool, &tester.kphis_extra).await.unwrap();
        assert_eq!(success.rows_affected, 1);
        let again_not_found = delete_index_monitor(1, &tester.db_pool, &tester.kphis_extra).await.unwrap();
        assert_eq!(again_not_found.rows_affected, 0);
    }
}
