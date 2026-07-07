use sqlx::{
    AssertSqlSafe, FromRow, MySql, Pool, Row,
    mysql::{MySqlQueryResult, MySqlRow},
};

use kphis_model::{
    app::VisitTypeId,
    fetch::ExecuteResponse,
    index_action::{IndexAction, IndexActionOnly},
};
use kphis_sql::ipd::index_action;
use kphis_util::error::{AppError, Source};

pub async fn get_index_action(plan_id: u32, pool: &Pool<MySql>, hosxp: &str, kphis: &str, kphis_extra: &str) -> Result<Vec<IndexAction>, AppError> {
    let sql = index_action::select_index_action(hosxp, kphis, kphis_extra);
    sqlx::query(AssertSqlSafe(sql))
        .bind(plan_id)
        .fetch_all(pool)
        .await
        .map_err(|e| Source::SQLx.to_error(500, e, "Select IndexAction"))?
        .iter()
        .map(index_action_from_row)
        .collect::<sqlx::Result<Vec<IndexAction>>>()
        .map_err(|e| Source::SQLx.to_error(500, e, "Select IndexAction"))
}
fn index_action_from_row(row: &MySqlRow) -> sqlx::Result<IndexAction> {
    let an: String = row.try_get("an")?;
    Ok(IndexAction {
        visit_type: VisitTypeId::Ipd(an.clone()),
        plan_id: row.try_get("plan_id")?,
        action_id: row.try_get("action_id")?,
        vs_id: row.try_get("vs_id")?,
        check_datetime: row.try_get("check_datetime")?,
        check_person: row.try_get("check_person")?,
        check_person_name: row.try_get("check_person_name")?,
        check_person_entryposition: row.try_get("check_person_entryposition")?,
        action_result: row.try_get("action_result")?,
        action_remark: row.try_get("action_remark")?,
        action_date: row.try_get("action_date")?,
        action_time: row.try_get("action_time")?,
        action_report_back: row.try_get("action_report_back")?,
        action_blood_had: row.try_get("action_blood_had")?,
        action_person_1: row.try_get("action_person_1")?,
        action_person_2: row.try_get("action_person_2")?,
        action_person_1_name: row.try_get("action_person_1_name")?,
        action_person_2_name: row.try_get("action_person_2_name")?,
        action_person_1_entryposition: row.try_get("action_person_1_entryposition")?,
        action_person_2_entryposition: row.try_get("action_person_2_entryposition")?,
        has_monitor: row.try_get("has_monitor")?,
        monitors: Vec::new(),
    })
}

pub async fn get_index_action_only(plan_id: u32, pool: &Pool<MySql>, kphis: &str, kphis_extra: &str) -> Result<Vec<IndexActionOnly>, AppError> {
    let sql = index_action::select_index_action_only(kphis, kphis_extra);
    sqlx::query(AssertSqlSafe(sql))
        .bind(plan_id)
        .fetch_all(pool)
        .await
        .map_err(|e| Source::SQLx.to_error(500, e, "Select IndexActionOnly"))?
        .iter()
        .map(IndexActionOnly::from_row)
        .collect::<sqlx::Result<Vec<IndexActionOnly>>>()
        .map_err(|e| Source::SQLx.to_error(500, e, "Select IndexActionOnly"))
}

// ipd-nurse-index-plan-action-save.php
pub async fn post_index_action(save: &IndexAction, user: &str, pool: &Pool<MySql>, kphis: &str) -> Result<(u32, Vec<ExecuteResponse>), AppError> {
    let is_update = match save.action_id {
        Some(id) => id > 0,
        None => false,
    };
    let action_id;
    let mut results = Vec::with_capacity(1);
    if is_update {
        action_id = save.action_id;
        let update_action_result = update_index_action(save, user, pool, kphis).await?;
        results.push(ExecuteResponse::from_query_result(update_action_result, "Update IndexPlanAction"));
    } else {
        let insert_action_result = insert_index_action(save, user, pool, kphis).await?;
        action_id = Some(insert_action_result.last_insert_id() as u32);
        results.push(ExecuteResponse::from_query_result(insert_action_result, "Insert IndexPlanAction"));
    }

    Ok((action_id.unwrap_or_default(), results))
}

async fn update_index_action(save: &IndexAction, user: &str, pool: &Pool<MySql>, kphis: &str) -> Result<MySqlQueryResult, AppError> {
    let update_action_sql = index_action::update_index_action(kphis);
    sqlx::query(AssertSqlSafe(update_action_sql))
        .bind(save.check_datetime)
        .bind(&save.check_person)
        .bind(&save.action_result)
        .bind(&save.action_remark)
        .bind(save.action_date)
        .bind(save.action_time)
        .bind(&save.action_report_back)
        .bind(&save.action_blood_had)
        .bind(&save.action_person_1)
        .bind(&save.action_person_2)
        .bind(user)
        .bind(save.action_id.unwrap_or_default())
        .execute(pool)
        .await
        .map_err(|e| Source::SQLx.to_error(500, e, "Update IndexPlanAction"))
}

async fn insert_index_action(save: &IndexAction, user: &str, pool: &Pool<MySql>, kphis: &str) -> Result<MySqlQueryResult, AppError> {
    match &save.visit_type {
        VisitTypeId::Ipd(an) | VisitTypeId::PreAdmit(an) => {
            let insert_action_sql = index_action::insert_index_action(kphis);
            sqlx::query(AssertSqlSafe(insert_action_sql))
                .bind(save.plan_id)
                .bind(an)
                .bind(save.check_datetime)
                .bind(&save.check_person)
                .bind(&save.action_result)
                .bind(&save.action_remark)
                .bind(save.action_date)
                .bind(save.action_time)
                .bind(&save.action_report_back)
                .bind(&save.action_blood_had)
                .bind(&save.action_person_1)
                .bind(&save.action_person_2)
                .bind(user)
                .bind(user)
                .execute(pool)
                .await
                .map_err(|e| Source::SQLx.to_error(500, e, "Insert IndexPlanAction"))
        }
        VisitTypeId::OpdEr(_, _) | VisitTypeId::Visit(_) => Err(AppError::app_400("Insert IndexPlanAction")),
    }
}

pub async fn insert_index_action_only(plan_id: u32, an: &str, only: &mut IndexActionOnly, pool: &Pool<MySql>, kphis: &str) -> Result<MySqlQueryResult, AppError> {
    only.plan_id = Some(plan_id);
    only.insert(Some("action_id"), Some("ipd_nurse_index_action"), ",an", ",?", &[an], pool, kphis)
        .await
        .map_err(|e| Source::SQLx.to_error(500, e, "Insert IndexActionOnly"))
}

// pub async fn insert_index_actions_only(plan_id: u32, an: &str, index_actions_only: &[IndexActionOnly], pool: &Pool<MySql>, kphis: &str) -> Result<MySqlQueryResult, AppError> {
//     let insert_action_sql = index_action::insert_index_actions_only(plan_id, an, index_actions_only, kphis);
//     sqlx::query(AssertSqlSafe(insert_action_sql)).execute(pool).await.map_err(|e| Source::SQLx.to_error(500, e, "Insert IndexActionOnly"))
// }

// ipd-nurse-index-plan-action-delete.php
pub async fn delete_index_action(action_id: u32, pool: &Pool<MySql>, kphis: &str) -> Result<ExecuteResponse, AppError> {
    let sql = index_action::delete_index_action(kphis);
    let delete_result = sqlx::query(AssertSqlSafe(sql))
        .bind(action_id)
        .execute(pool)
        .await
        .map_err(|e| Source::SQLx.to_error(500, e, "Delete IndexAction"))?;

    Ok(ExecuteResponse::from_query_result(delete_result, "Delete IndexAction"))
}

#[cfg(test)]
#[rustfmt::skip]
mod tests {

    use super::*;
    use kphis_sqlx_tester::MySqlTester;

    #[tokio::test]
    #[ignore]
    async fn sqlx_get_index_action() {
        let tester = MySqlTester::new_hosxp_and_kphis_and_kphis_extra().await;
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis/ipd_nurse_index_action.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis/ipd_vs_vital_sign.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis_extra/ipd_nurse_index_monitor.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/hosxp/doctor.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/hosxp/opduser.sql")).execute(&tester.db_pool).await.unwrap();

        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/kphis/ipd_nurse_index_action.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/kphis/ipd_vs_vital_sign.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/kphis_extra/ipd_nurse_index_monitor.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/hosxp/doctor.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/hosxp/opduser.sql")).execute(&tester.db_pool).await.unwrap();

        let found = get_index_action(1, &tester.db_pool, &tester.hosxp, &tester.kphis, &tester.kphis_extra).await.unwrap();
        assert_eq!(found.len(), 1);
        let not_found = get_index_action(999, &tester.db_pool, &tester.hosxp, &tester.kphis, &tester.kphis_extra).await.unwrap();
        assert!(not_found.is_empty());
    }

    #[tokio::test]
    #[ignore]
    async fn sqlx_get_index_action_only() {
        let tester = MySqlTester::new_kphis_and_kphis_extra().await;
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis/ipd_nurse_index_action.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis_extra/ipd_nurse_index_monitor.sql")).execute(&tester.db_pool).await.unwrap();

        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/kphis/ipd_nurse_index_action.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/kphis_extra/ipd_nurse_index_monitor.sql")).execute(&tester.db_pool).await.unwrap();

        let found = get_index_action_only(1, &tester.db_pool, &tester.kphis, &tester.kphis_extra).await.unwrap();
        assert_eq!(found.len(), 1);
        let not_found = get_index_action_only(999, &tester.db_pool, &tester.kphis, &tester.kphis_extra).await.unwrap();
        assert!(not_found.is_empty());
    }

    #[tokio::test]
    #[ignore]
    async fn sqlx_insert_index_action() {
        let tester = MySqlTester::new_kphis().await;
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis/ipd_nurse_index_action.sql")).execute(&tester.db_pool).await.unwrap();

        let success = insert_index_action(&IndexAction::demo(),"user",&tester.db_pool,&tester.kphis).await.unwrap();
        assert_eq!(success.rows_affected(), 1);
        let again_success = insert_index_action(&IndexAction::demo(),"user",&tester.db_pool,&tester.kphis).await.unwrap();
        assert_eq!(again_success.rows_affected(), 1);
    }

    #[tokio::test]
    #[ignore]
    async fn sqlx_insert_index_action_only() {
        let tester = MySqlTester::new_kphis().await;
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis/ipd_nurse_index_action.sql")).execute(&tester.db_pool).await.unwrap();

        let success = insert_index_action_only(1,"660001234",&mut IndexActionOnly::demo(),&tester.db_pool,&tester.kphis).await.unwrap();
        assert_eq!(success.rows_affected(), 1);
        let again_success = insert_index_action_only(1,"",&mut IndexActionOnly::demo(),&tester.db_pool,&tester.kphis).await.unwrap();
        assert_eq!(again_success.rows_affected(), 1);
    }

    // #[tokio::test]
    // #[ignore]
    // async fn sqlx_insert_index_actions_only() {
    //     let tester = MySqlTester::new_kphis().await;
    //     sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis/ipd_nurse_index_action.sql")).execute(&tester.db_pool).await.unwrap();

    //     let success = insert_index_actions_only(1,"660001234",&[IndexActionOnly::demo()],&tester.db_pool,&tester.kphis).await.unwrap();
    //     assert_eq!(success.rows_affected(), 1);
    //     let again_success = insert_index_actions_only(1,"",&[IndexActionOnly::demo()],&tester.db_pool,&tester.kphis).await.unwrap();
    //     assert_eq!(again_success.rows_affected(), 1);
    // }

    #[tokio::test]
    #[ignore]
    async fn sqlx_update_index_action() {
        let tester = MySqlTester::new_kphis().await;
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis/ipd_nurse_index_action.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/kphis/ipd_nurse_index_action.sql")).execute(&tester.db_pool).await.unwrap();

        let success = update_index_action(&IndexAction::demo(),"user",&tester.db_pool,&tester.kphis).await.unwrap();
        assert_eq!(success.rows_affected(), 1);
        let again_success = update_index_action(&IndexAction::demo(),"user",&tester.db_pool,&tester.kphis).await.unwrap();
        assert_eq!(again_success.rows_affected(), 1);
    }

    #[tokio::test]
    #[ignore]
    async fn sqlx_delete_index_action() {
        let tester = MySqlTester::new_kphis().await;
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis/ipd_nurse_index_action.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/kphis/ipd_nurse_index_action.sql")).execute(&tester.db_pool).await.unwrap();

        let success = delete_index_action(1, &tester.db_pool, &tester.kphis).await.unwrap();
        assert_eq!(success.rows_affected, 1);
        let again_not_found = delete_index_action(1, &tester.db_pool, &tester.kphis).await.unwrap();
        assert_eq!(again_not_found.rows_affected, 0);
    }
}
