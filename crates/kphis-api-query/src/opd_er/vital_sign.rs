use sqlx::{AssertSqlSafe, FromRow, MySql, Pool, mysql::MySqlQueryResult};

use kphis_model::{
    fetch::ExecuteResponse,
    vital_sign::{VitalSign, VitalSignOnly, VitalSignParams, VitalSignSave},
};
use kphis_sql::{TABLE_CREATE_COLUMNS, TABLE_CREATE_PREPARED, TABLE_UPDATE_SET, opd_er::vital_sign};
use kphis_util::error::{AppError, Source};

// opd_er-vital-sign-show-chart-table-data.php
pub async fn get_vital_sign(params: &VitalSignParams, pool: &Pool<MySql>, hosxp: &str, kphis: &str) -> Result<Vec<VitalSign>, AppError> {
    let sql = vital_sign::select_chart_data(params, hosxp, kphis);

    let mut query = sqlx::query(AssertSqlSafe(sql));

    if let Some(vs_id) = params.vs_id.as_ref() {
        query = query.bind(vs_id);
    }
    if let Some(opd_er_order_master_id) = params.opd_er_order_master_id {
        query = query.bind(opd_er_order_master_id);
    }
    if let Some(start_date) = params.start_date.as_ref() {
        query = query.bind(start_date);
    }
    if let Some(end_date) = params.end_date.as_ref() {
        query = query.bind(end_date);
    }

    query
        .fetch_all(pool)
        .await
        .map_err(|e| Source::SQLx.to_error(500, e, "Select VitalSign"))?
        .iter()
        .map(VitalSign::from_row)
        .collect::<sqlx::Result<Vec<VitalSign>>>()
        .map_err(|e| Source::SQLx.to_error(500, e, "Select VitalSign"))
}

pub async fn get_vital_sign_only(opd_er_order_master_id: u32, pool: &Pool<MySql>, kphis: &str) -> Result<Vec<VitalSignOnly>, AppError> {
    let sql = vital_sign::select_vs_only(kphis);

    sqlx::query(AssertSqlSafe(sql))
        .bind(opd_er_order_master_id)
        .fetch_all(pool)
        .await
        .map_err(|e| Source::SQLx.to_error(500, e, "Select VitalSignOnly"))?
        .iter()
        .map(VitalSignOnly::from_row)
        .collect::<sqlx::Result<Vec<VitalSignOnly>>>()
        .map_err(|e| Source::SQLx.to_error(500, e, "Select VitalSignOnly"))
}

// opd-er-vital-sign-save.php
pub async fn post_vital_sign(opd_er_order_master_id: u32, form: &VitalSignSave, user: &str, pool: &Pool<MySql>, kphis: &str) -> Result<MySqlQueryResult, AppError> {
    form.insert(
        Some("vs_id"),
        Some("opd_er_vs_vital_sign"),
        &[",opd_er_order_master_id", TABLE_CREATE_COLUMNS].concat(),
        &[",?", TABLE_CREATE_PREPARED].concat(),
        &[&opd_er_order_master_id.to_string(), user, user],
        pool,
        kphis,
    )
    .await
    .map_err(|e| Source::SQLx.to_error(500, e, "Insert VitalSign"))
}

pub async fn insert_vital_sign_only(opd_er_order_master_id: u32, only: &VitalSignOnly, pool: &Pool<MySql>, kphis: &str) -> Result<MySqlQueryResult, AppError> {
    only.insert(
        Some("vs_id"),
        Some("opd_er_vs_vital_sign"),
        ",opd_er_order_master_id",
        ",?",
        &[&opd_er_order_master_id.to_string()],
        pool,
        kphis,
    )
    .await
    .map_err(|e| Source::SQLx.to_error(500, e, "Insert VitalSignOnly"))
}

// opd-er-vital-sign-save.php
pub async fn put_vital_sign(opd_er_order_master_id: u32, form: &VitalSignSave, user: &str, pool: &Pool<MySql>, kphis: &str) -> Result<MySqlQueryResult, AppError> {
    form.update(
        "vs_id",
        Some("opd_er_vs_vital_sign"),
        &[",opd_er_order_master_id=?", TABLE_UPDATE_SET].concat(),
        &[&opd_er_order_master_id.to_string(), user],
        pool,
        kphis,
    )
    .await
    .map_err(|e| Source::SQLx.to_error(500, e, "Update VitalSign"))
}

// opd-er-vital-sign-save.php
pub async fn delete_vital_sign(vs_id: u32, pool: &Pool<MySql>, kphis: &str) -> Result<ExecuteResponse, AppError> {
    let sql = vital_sign::delete_vital_sign(kphis);
    let delete_result = sqlx::query(AssertSqlSafe(sql))
        .bind(vs_id)
        .execute(pool)
        .await
        .map_err(|e| Source::SQLx.to_error(500, e, "Delete VitalSign"))?;

    Ok(ExecuteResponse::from_query_result(delete_result, "Delete VitalSign"))
}

#[cfg(test)]
#[rustfmt::skip]
mod tests {

    use super::*;
    use kphis_sqlx_tester::MySqlTester;
    use kphis_util::datetime::date_8601;

    #[tokio::test]
    #[ignore]
    async fn sqlx_get_vital_sign() {
        let tester = MySqlTester::new_hosxp_and_kphis().await;
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis/opd_er_vs_vital_sign.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis/ipd_vs_conscious.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis/ipd_vs_line.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis/ipd_vs_cha.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis/ipd_vs_va.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis/ipd_vs_mass.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis/ipd_vs_o2.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis/ipd_vs_tube.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis/ipd_vs_intake.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis/ipd_vs_output.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis/ipd_vs_lr_sta.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis/ipd_vs_lr_mem.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis/ipd_vs_lr_moulding.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis/ipd_vs_dipstick.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis/ipd_vs_lt_arm.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis/ipd_vs_lt_leg.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis/ipd_vs_rt_arm.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis/ipd_vs_rt_leg.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis/ipd_vs_stage_of_change.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/hosxp/opduser.sql")).execute(&tester.db_pool).await.unwrap();

        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/kphis/opd_er_vs_vital_sign.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/kphis/ipd_vs_conscious.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/kphis/ipd_vs_line.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/kphis/ipd_vs_cha.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/kphis/ipd_vs_va.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/kphis/ipd_vs_mass.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/kphis/ipd_vs_o2.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/kphis/ipd_vs_tube.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/kphis/ipd_vs_intake.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/kphis/ipd_vs_output.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/kphis/ipd_vs_lr_sta.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/kphis/ipd_vs_lr_mem.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/kphis/ipd_vs_lr_moulding.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/kphis/ipd_vs_dipstick.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/kphis/ipd_vs_lt_arm.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/kphis/ipd_vs_lt_leg.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/kphis/ipd_vs_rt_arm.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/kphis/ipd_vs_rt_leg.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/kphis/ipd_vs_stage_of_change.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/hosxp/opduser.sql")).execute(&tester.db_pool).await.unwrap();

        let default = get_vital_sign(&VitalSignParams::default(),&tester.db_pool,&tester.hosxp,&tester.kphis).await.unwrap();
        assert_eq!(default.len(), 3);
        let found_vs_id = get_vital_sign(&VitalSignParams {vs_id: Some(1),..Default::default()},&tester.db_pool,&tester.hosxp,&tester.kphis).await.unwrap();
        assert_eq!(found_vs_id.len(), 1);
        let found_opd_er_order_master_id = get_vital_sign(&VitalSignParams {opd_er_order_master_id: Some(1),..Default::default()},&tester.db_pool,&tester.hosxp,&tester.kphis).await.unwrap();
        assert_eq!(found_opd_er_order_master_id.len(), 3);
        let found_start_date = get_vital_sign(&VitalSignParams {start_date: date_8601("2024-01-05"),..Default::default()},&tester.db_pool,&tester.hosxp,&tester.kphis).await.unwrap();
        assert_eq!(found_start_date.len(), 2);
        let found_end_date = get_vital_sign(&VitalSignParams {end_date: date_8601("2024-01-05"),..Default::default()},&tester.db_pool,&tester.hosxp,&tester.kphis).await.unwrap();
        assert_eq!(found_end_date.len(), 2);
        let found_between_date = get_vital_sign(&VitalSignParams {start_date: date_8601("2024-01-05"),end_date: date_8601("2024-01-05"),..Default::default()},&tester.db_pool,&tester.hosxp,&tester.kphis).await.unwrap();
        assert_eq!(found_between_date.len(), 1);
        let not_found = get_vital_sign(&VitalSignParams {vs_id: Some(999),..Default::default()},&tester.db_pool,&tester.hosxp,&tester.kphis).await.unwrap();
        assert!(not_found.is_empty());
    }

    #[tokio::test]
    #[ignore]
    async fn sqlx_get_vital_sign_only() {
        let tester = MySqlTester::new_hosxp_and_kphis().await;
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis/opd_er_vs_vital_sign.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/kphis/opd_er_vs_vital_sign.sql")).execute(&tester.db_pool).await.unwrap();

        let found = get_vital_sign_only(1,&tester.db_pool,&tester.kphis).await.unwrap();
        assert_eq!(found.len(), 3);
        let not_found = get_vital_sign_only(999,&tester.db_pool,&tester.kphis).await.unwrap();
        assert!(not_found.is_empty());
    }

    #[tokio::test]
    #[ignore]
    async fn sqlx_post_vital_sign() {
        let tester = MySqlTester::new_kphis().await;
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis/opd_er_vs_vital_sign.sql")).execute(&tester.db_pool).await.unwrap();

        let success = post_vital_sign(1,&VitalSignSave::demo(),"user",&tester.db_pool,&tester.kphis).await.unwrap();
        assert_eq!(success.rows_affected(), 1);
        let again_success = post_vital_sign(1,&VitalSignSave::demo(),"user",&tester.db_pool,&tester.kphis).await.unwrap();
        assert_eq!(again_success.rows_affected(), 1);
    }

    #[tokio::test]
    #[ignore]
    async fn sqlx_insert_vital_sign_only() {
        let tester = MySqlTester::new_kphis().await;
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis/opd_er_vs_vital_sign.sql")).execute(&tester.db_pool).await.unwrap();

        let success = insert_vital_sign_only(1, &VitalSignOnly::demo(),&tester.db_pool,&tester.kphis).await.unwrap();
        assert_eq!(success.rows_affected(), 1);
        let again_success = insert_vital_sign_only(1, &VitalSignOnly::demo(),&tester.db_pool,&tester.kphis).await.unwrap();
        assert_eq!(again_success.rows_affected(), 1);
    }

    #[tokio::test]
    #[ignore]
    async fn sqlx_put_vital_sign() {
        let tester = MySqlTester::new_kphis().await;
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis/opd_er_vs_vital_sign.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/kphis/opd_er_vs_vital_sign.sql")).execute(&tester.db_pool).await.unwrap();

        let success = put_vital_sign(1,&VitalSignSave::demo(),"user",&tester.db_pool,&tester.kphis).await.unwrap();
        assert_eq!(success.rows_affected(), 1);
        let again_success = put_vital_sign(1,&VitalSignSave::demo(),"user",&tester.db_pool,&tester.kphis).await.unwrap();
        assert_eq!(again_success.rows_affected(), 1);
    }

    #[tokio::test]
    #[ignore]
    async fn sqlx_delete_vital_sign() {
        let tester = MySqlTester::new_kphis().await;
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis/opd_er_vs_vital_sign.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/kphis/opd_er_vs_vital_sign.sql")).execute(&tester.db_pool).await.unwrap();

        let success = delete_vital_sign(1, &tester.db_pool, &tester.kphis).await.unwrap();
        assert_eq!(success.rows_affected, 1);
        let again_not_found = delete_vital_sign(1, &tester.db_pool, &tester.kphis).await.unwrap();
        assert_eq!(again_not_found.rows_affected, 0);
    }
}
