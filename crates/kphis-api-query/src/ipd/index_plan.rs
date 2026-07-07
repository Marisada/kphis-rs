use sqlx::{
    AssertSqlSafe, FromRow, MySql, Pool, Row,
    mysql::{MySqlQueryResult, MySqlRow},
};
use time::Date;

use kphis_model::{
    app::VisitTypeId,
    fetch::ExecuteResponse,
    index_plan::{IndexPlan, IndexPlanDate, IndexPlanOnly, IndexPlanSave, IpdIndexMedPay},
};
use kphis_sql::ipd::index_plan;
use kphis_util::error::{AppError, Source};

use crate::query1_all;

pub async fn get_index_plan_date(an: &str, pool: &Pool<MySql>, kphis: &str) -> Result<Vec<IndexPlanDate>, AppError> {
    let sql = index_plan::select_index_plan_date(false, kphis);
    query1_all(an, &sql, pool, "Select IndexPlanDate")
        .await?
        .iter()
        .map(IndexPlanDate::from_row)
        .collect::<sqlx::Result<Vec<IndexPlanDate>>>()
        .map_err(|e| Source::SQLx.to_error(500, e, "Select IndexPlanDate"))
}

pub async fn get_index_plan_by_order_item_id(order_item_id: u32, pool: &Pool<MySql>, kphis: &str) -> Result<Vec<IndexPlan>, AppError> {
    let sql = index_plan::select_index_plan_by_order_item_id(kphis);
    sqlx::query(AssertSqlSafe(sql))
        .bind(order_item_id)
        .fetch_all(pool)
        .await
        .map_err(|e| Source::SQLx.to_error(500, e, "Select IpdIndexPlan"))?
        .iter()
        .map(index_plan_from_row)
        .collect::<sqlx::Result<Vec<IndexPlan>>>()
        .map_err(|e| Source::SQLx.to_error(500, e, "Select IpdIndexPlan"))
}
fn index_plan_from_row(row: &MySqlRow) -> sqlx::Result<IndexPlan> {
    let an: String = row.try_get("an")?;
    Ok(IndexPlan {
        visit_type: VisitTypeId::Ipd(an.clone()),
        plan_id: row.try_get("plan_id")?,
        plan_detail: row.try_get("plan_detail")?,
        plan_date: row.try_get("plan_date")?,
        plan_time: row.try_get("plan_time")?,
        plan_sch_type: row.try_get("plan_sch_type")?,
        order_item_id: row.try_get("order_item_id")?,
        order_type: None,
        order_date: None,
        order_time: None,
        off_by_datetime: None,
        actions: Vec::new(),
    })
}

pub async fn get_index_plan_only(order_item_id: u32, pool: &Pool<MySql>, kphis: &str) -> Result<Vec<IndexPlanOnly>, AppError> {
    let sql = index_plan::select_index_plan_only(kphis);
    sqlx::query(AssertSqlSafe(sql))
        .bind(order_item_id)
        .fetch_all(pool)
        .await
        .map_err(|e| Source::SQLx.to_error(500, e, "Select IndexPlanOnly"))?
        .iter()
        .map(IndexPlanOnly::from_row)
        .collect::<sqlx::Result<Vec<IndexPlanOnly>>>()
        .map_err(|e| Source::SQLx.to_error(500, e, "Select IndexPlanOnly"))
}

pub async fn get_index_plan_without_order_item_id(an: &str, plan_date_opt: Option<Date>, pool: &Pool<MySql>, kphis: &str) -> Result<Vec<IndexPlan>, AppError> {
    let sql = index_plan::select_index_plan_without_order_item_id(plan_date_opt.is_some(), kphis);
    let mut query = sqlx::query(AssertSqlSafe(sql)).bind(an);
    if let Some(plan_date) = plan_date_opt {
        query = query.bind(plan_date);
    }
    query
        .fetch_all(pool)
        .await
        .map_err(|e| Source::SQLx.to_error(500, e, "Select IpdIndexPlan"))?
        .iter()
        .map(index_plan_from_row)
        .collect::<sqlx::Result<Vec<IndexPlan>>>()
        .map_err(|e| Source::SQLx.to_error(500, e, "Select IpdIndexPlan"))
}

// ipd-nurse-index-plan-action-save.php
pub async fn post_index_plan(save: &IndexPlanSave, user: &str, pool: &Pool<MySql>, kphis: &str) -> Result<(u32, Vec<ExecuteResponse>), AppError> {
    let is_update = match save.plan_id {
        Some(id) => id > 0,
        None => false,
    };
    let plan_id;
    let mut results = Vec::with_capacity(1);
    if is_update {
        plan_id = save.plan_id;
        results.push(update_index_plan(save, user, pool, kphis).await?);
    } else {
        let insert_plan_result = insert_index_plan(save, user, pool, kphis).await?;
        plan_id = Some(insert_plan_result.last_insert_id as u32);
        results.push(insert_plan_result);
    }

    Ok((plan_id.unwrap_or_default(), results))
}

async fn update_index_plan(save: &IndexPlanSave, user: &str, pool: &Pool<MySql>, kphis: &str) -> Result<ExecuteResponse, AppError> {
    let update_plan_sql = index_plan::update_index_plan(kphis);
    sqlx::query(AssertSqlSafe(update_plan_sql))
        .bind(&save.plan_detail)
        .bind(save.plan_date)
        .bind(save.plan_time)
        .bind(&save.plan_sch_type)
        .bind(user)
        .bind(save.plan_id.unwrap_or_default())
        .execute(pool)
        .await
        .map(|result| ExecuteResponse::from_query_result(result, "Update IndexPlan"))
        .map_err(|e| Source::SQLx.to_error(500, e, "Update IndexPlan"))
}

async fn insert_index_plan(save: &IndexPlanSave, user: &str, pool: &Pool<MySql>, kphis: &str) -> Result<ExecuteResponse, AppError> {
    match &save.visit_type {
        VisitTypeId::Ipd(an) | VisitTypeId::PreAdmit(an) => {
            let insert_plan_sql = index_plan::insert_index_plan(kphis);
            sqlx::query(AssertSqlSafe(insert_plan_sql))
                .bind(save.order_item_id)
                .bind(an)
                .bind(&save.plan_detail)
                .bind(save.plan_date)
                .bind(save.plan_time)
                .bind(&save.plan_sch_type)
                .bind(user)
                .bind(user)
                .execute(pool)
                .await
                .map(|result| ExecuteResponse::from_query_result(result, "Insert IndexPlan"))
                .map_err(|e| Source::SQLx.to_error(500, e, "Insert IndexPlan"))
        }
        _ => Err(AppError::app_400("Insert IndexPlan")),
    }
}

pub async fn insert_index_plan_only(order_item_id: u32, an: &str, only: &mut IndexPlanOnly, pool: &Pool<MySql>, kphis: &str) -> Result<MySqlQueryResult, AppError> {
    only.order_item_id = Some(order_item_id);
    only.insert(Some("plan_id"), Some("ipd_nurse_index_plan"), ",an", ",?", &[an], pool, kphis)
        .await
        .map_err(|e| Source::SQLx.to_error(500, e, "Insert IndexPlansOnly"))
}

// ipd-nurse-index-plan-action-delete.php
/// delete plan and action
pub async fn delete_index_plan(plan_id: u32, pool: &Pool<MySql>, kphis: &str) -> Result<ExecuteResponse, AppError> {
    let sql = index_plan::delete_index_plan(kphis);
    let delete_result = sqlx::query(AssertSqlSafe(sql))
        .bind(plan_id)
        .execute(pool)
        .await
        .map_err(|e| Source::SQLx.to_error(500, e, "Delete IndexPlan"))?;

    Ok(ExecuteResponse::from_query_result(delete_result, "Delete IndexPlan"))
}

pub async fn get_index_med_pay(an: &str, pool: &Pool<MySql>, hosxp: &str) -> Result<Vec<IpdIndexMedPay>, AppError> {
    let sql = index_plan::select_index_med_pay_hosxp(hosxp);

    sqlx::query(AssertSqlSafe(sql))
        .bind(an)
        .fetch_all(pool)
        .await
        .map_err(|e| Source::SQLx.to_error(500, e, "Select IndexMedPay"))?
        .iter()
        .map(IpdIndexMedPay::from_row)
        .collect::<sqlx::Result<Vec<IpdIndexMedPay>>>()
        .map_err(|e| Source::SQLx.to_error(500, e, "Select IndexMedPay"))
}

#[cfg(test)]
#[rustfmt::skip]
mod tests {

    use super::*;
    use time::macros::date;
    use kphis_sqlx_tester::MySqlTester;

    #[tokio::test]
    #[ignore]
    async fn sqlx_get_index_plan_date() {
        let tester = MySqlTester::new_kphis().await;
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis/ipd_nurse_index_plan.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/kphis/ipd_nurse_index_plan.sql")).execute(&tester.db_pool).await.unwrap();

        let found = get_index_plan_date("670001234", &tester.db_pool, &tester.kphis).await.unwrap();
        assert_eq!(found.len(), 4);
        let not_found = get_index_plan_date("660006666", &tester.db_pool, &tester.kphis).await.unwrap();
        assert!(not_found.is_empty());
    }

    #[tokio::test]
    #[ignore]
    async fn sqlx_get_index_plan_by_order_item_id() {
        let tester = MySqlTester::new_kphis().await;
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis/ipd_nurse_index_plan.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/kphis/ipd_nurse_index_plan.sql")).execute(&tester.db_pool).await.unwrap();

        let found = get_index_plan_by_order_item_id(11, &tester.db_pool, &tester.kphis).await.unwrap();
        assert_eq!(found.len(), 3);
        let not_found = get_index_plan_by_order_item_id(999, &tester.db_pool, &tester.kphis).await.unwrap();
        assert!(not_found.is_empty());
    }

    #[tokio::test]
    #[ignore]
    async fn sqlx_get_index_plan_only() {
        let tester = MySqlTester::new_kphis().await;
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis/ipd_nurse_index_plan.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/kphis/ipd_nurse_index_plan.sql")).execute(&tester.db_pool).await.unwrap();

        let found = get_index_plan_only(11, &tester.db_pool, &tester.kphis).await.unwrap();
        assert_eq!(found.len(), 3);
        let not_found = get_index_plan_only(999, &tester.db_pool, &tester.kphis).await.unwrap();
        assert!(not_found.is_empty());
    }

    #[tokio::test]
    #[ignore]
    async fn sqlx_get_index_plan_without_order_item_id() {
        let tester = MySqlTester::new_kphis().await;
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis/ipd_nurse_index_plan.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/kphis/ipd_nurse_index_plan.sql")).execute(&tester.db_pool).await.unwrap();

        let found = get_index_plan_without_order_item_id("660001234", Some(date!(2024-01-01)), &tester.db_pool, &tester.kphis).await.unwrap();
        assert_eq!(found.len(), 2);
        let not_found = get_index_plan_without_order_item_id("660006666", None, &tester.db_pool, &tester.kphis).await.unwrap();
        assert!(not_found.is_empty());
    }

    #[tokio::test]
    #[ignore]
    async fn sqlx_get_index_med_pay() {
        let tester = MySqlTester::new_hosxp().await;
        // sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/hosxp/medplan_ipd.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/hosxp/medpay_ipd.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/hosxp/ipt_order_no.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/hosxp/drugitems.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/hosxp/opduser.sql")).execute(&tester.db_pool).await.unwrap();

        // sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/hosxp/medplan_ipd.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/hosxp/medpay_ipd.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/hosxp/ipt_order_no.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/hosxp/drugitems.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/hosxp/opduser.sql")).execute(&tester.db_pool).await.unwrap();

        let found = get_index_med_pay("660001234", &tester.db_pool, &tester.hosxp).await.unwrap();
        assert_eq!(found.len(), 1);
        let not_found = get_index_med_pay("660006666", &tester.db_pool, &tester.hosxp).await.unwrap();
        assert!(not_found.is_empty());
    }

    #[tokio::test]
    #[ignore]
    async fn sqlx_insert_index_plan() {
        let tester = MySqlTester::new_kphis().await;
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis/ipd_nurse_index_plan.sql")).execute(&tester.db_pool).await.unwrap();

        let success = insert_index_plan(&IndexPlanSave::demo(),"user",&tester.db_pool,&tester.kphis).await.unwrap();
        assert_eq!(success.rows_affected, 1);
        let again_success = insert_index_plan(&IndexPlanSave::demo(),"user",&tester.db_pool,&tester.kphis).await.unwrap();
        assert_eq!(again_success.rows_affected, 1);
    }

    #[tokio::test]
    #[ignore]
    async fn sqlx_insert_index_plan_only() {
        let tester = MySqlTester::new_kphis().await;
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis/ipd_nurse_index_plan.sql")).execute(&tester.db_pool).await.unwrap();

        let success = insert_index_plan_only(1,"660001234",&mut IndexPlanOnly::demo(),&tester.db_pool,&tester.kphis).await.unwrap();
        assert_eq!(success.rows_affected(), 1);
        let again_success = insert_index_plan_only(1,"",&mut IndexPlanOnly::demo(),&tester.db_pool,&tester.kphis).await.unwrap();
        assert_eq!(again_success.rows_affected(), 1);
    }

    // #[tokio::test]
    // #[ignore]
    // async fn sqlx_insert_index_plans_only() {
    //     let tester = MySqlTester::new_kphis().await;
    //     sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis/ipd_nurse_index_plan.sql")).execute(&tester.db_pool).await.unwrap();

    //     let success = insert_index_plans_only(1,"660001234",&[IndexPlanOnly::demo()],&tester.db_pool,&tester.kphis).await.unwrap();
    //     assert_eq!(success.rows_affected(), 1);
    //     let again_success = insert_index_plans_only(1,"",&[IndexPlanOnly::demo()],&tester.db_pool,&tester.kphis).await.unwrap();
    //     assert_eq!(again_success.rows_affected(), 1);
    // }

    #[tokio::test]
    #[ignore]
    async fn sqlx_update_index_plan() {
        let tester = MySqlTester::new_kphis().await;
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis/ipd_nurse_index_plan.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/kphis/ipd_nurse_index_plan.sql")).execute(&tester.db_pool).await.unwrap();

        let success = update_index_plan(&IndexPlanSave::demo(),"user",&tester.db_pool,&tester.kphis).await.unwrap();
        assert_eq!(success.rows_affected, 1);
        let again_success = update_index_plan(&IndexPlanSave::demo(),"user",&tester.db_pool,&tester.kphis).await.unwrap();
        assert_eq!(again_success.rows_affected, 1);
    }

    #[tokio::test]
    #[ignore]
    async fn sqlx_delete_index_plan() {
        let tester = MySqlTester::new_kphis().await;
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis/ipd_nurse_index_plan.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis/ipd_nurse_index_action.sql")).execute(&tester.db_pool).await.unwrap();

        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/kphis/ipd_nurse_index_plan.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/kphis/ipd_nurse_index_action.sql")).execute(&tester.db_pool).await.unwrap();

        let success = delete_index_plan(1, &tester.db_pool, &tester.kphis).await.unwrap();
        assert_eq!(success.rows_affected, 2);
        let again_not_found = delete_index_plan(1, &tester.db_pool, &tester.kphis).await.unwrap();
        assert_eq!(again_not_found.rows_affected, 0);
    }
}
