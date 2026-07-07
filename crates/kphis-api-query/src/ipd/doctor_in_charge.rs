use sqlx::{AssertSqlSafe, FromRow, MySql, Pool, mysql::MySqlQueryResult};

use kphis_model::{fetch::ExecuteResponse, ipd::doctor_in_charge::IpdDoctorInCharge};
use kphis_sql::{
    data_history_utils::{KeyValue, SourceTable},
    ipd::doctor_in_charge,
};
use kphis_util::{
    error::{AppError, Source},
    util::zero_none,
};

use crate::{log::insert_history_log, query1_all};

// ipd-nurse-doctor-in-charge-table.php
pub async fn get_doctor_in_charge(an: &str, pool: &Pool<MySql>, hosxp: &str, kphis: &str) -> Result<Vec<IpdDoctorInCharge>, AppError> {
    let sql = doctor_in_charge::select_doctor_in_charges(hosxp, kphis);
    let result = query1_all(an, &sql, pool, "Select IpdDoctorInCharge")
        .await?
        .iter()
        .map(IpdDoctorInCharge::from_row)
        .collect::<sqlx::Result<Vec<IpdDoctorInCharge>>>()
        .map_err(|e| Source::SQLx.to_error(500, e, "Select IpdDoctorInCharge"))?;

    Ok(result)
}

// ipd-nurse-doctor-in-charge-save.php
// ipd-nurse-doctor-in-charge-update.php
// POST /ipd/doctor-in-charge
pub async fn post_doctor_in_charge(form: &IpdDoctorInCharge, user: &str, pool: &Pool<MySql>, kphis: &str, kphis_log: &str) -> Result<(u32, Vec<ExecuteResponse>), AppError> {
    let id;
    let mut results = Vec::with_capacity(4);
    if let Some(doctor_in_charge_id) = zero_none(form.doctor_in_charge_id) {
        id = doctor_in_charge_id;
        // UPDATE
        // 1. Update ipd_doctor_in_charge
        let update_result = update_doctor_in_charge(form, user, pool, kphis).await?;
        let is_update = update_result.rows_affected() > 0;
        results.push(ExecuteResponse::from_query_result(update_result, "Update IpdDoctorInCharge"));
        // 2. Insert to history
        if is_update {
            let insert_history_result = insert_history_log(SourceTable::IpdDoctorInCharge, "U", user, &[KeyValue("doctor_in_charge_id", id.to_string())], kphis, kphis_log, pool).await?;
            results.push(ExecuteResponse::from_query_result(insert_history_result, "Insert IpdDoctorInCharge History"));
        }
    } else {
        // SAVE
        // 1. Insert ipd_doctor_in_charge
        let insert_result = insert_doctor_in_charge(form, user, pool, kphis).await?;
        let is_insert = insert_result.rows_affected() > 0;
        id = insert_result.last_insert_id() as u32;
        results.push(ExecuteResponse::from_query_result(insert_result, "Insert IpdDoctorInCharge"));
        // 2. Insert to history
        if is_insert {
            let insert_history_result = insert_history_log(SourceTable::IpdDoctorInCharge, "I", user, &[KeyValue("doctor_in_charge_id", id.to_string())], kphis, kphis_log, pool).await?;
            results.push(ExecuteResponse::from_query_result(insert_history_result, "Insert IpdDoctorInCharge History"));
        }
    }

    Ok((id, results))
}

async fn update_doctor_in_charge(form: &IpdDoctorInCharge, user: &str, pool: &Pool<MySql>, kphis: &str) -> Result<MySqlQueryResult, AppError> {
    let update_sql = doctor_in_charge::update_doctor_in_charge(kphis);
    sqlx::query(AssertSqlSafe(update_sql))
        .bind(&form.an)
        .bind(&form.hn)
        .bind(&form.doctor)
        .bind(&form.spclty)
        .bind(&form.status)
        .bind(&form.activated)
        .bind(user)
        .bind(form.doctor_in_charge_id)
        .bind(form.version)
        .execute(pool)
        .await
        .map_err(|e| Source::SQLx.to_error(500, e, "Update IpdDoctorInCharge"))
}

async fn insert_doctor_in_charge(form: &IpdDoctorInCharge, user: &str, pool: &Pool<MySql>, kphis: &str) -> Result<MySqlQueryResult, AppError> {
    let insert_sql = doctor_in_charge::insert_doctor_in_charge(kphis);
    sqlx::query(AssertSqlSafe(insert_sql))
        .bind(&form.an)
        .bind(&form.hn)
        .bind(&form.doctor)
        .bind(&form.spclty)
        .bind(&form.status)
        .bind(&form.activated)
        .bind(user)
        .bind(user)
        .execute(pool)
        .await
        .map_err(|e| Source::SQLx.to_error(500, e, "Insert IpdDoctorInCharge"))
}

// ipd-nurse-doctor-in-charge-delete.php
// DELETE /ipd/doctor-in-charge
pub async fn delete_doctor_in_charge(doctor_in_charge_id: u32, version: i32, user: &str, pool: &Pool<MySql>, kphis: &str, kphis_log: &str) -> Result<Vec<ExecuteResponse>, AppError> {
    let mut results = Vec::with_capacity(2);
    // 1. Insert to history
    let insert_history_result = insert_history_log(
        SourceTable::IpdDoctorInCharge,
        "D",
        user,
        &[KeyValue("doctor_in_charge_id", doctor_in_charge_id.to_string()), KeyValue("version", version.to_string())],
        kphis,
        kphis_log,
        pool,
    )
    .await?;
    let will_delete = insert_history_result.rows_affected() > 0;
    results.push(ExecuteResponse::from_query_result(insert_history_result, "Insert IpdDoctorInCharge History"));

    // 2. Delete focus list
    if will_delete {
        let delete_result = delete_doctor_in_charge_inner(doctor_in_charge_id, version, pool, kphis).await?;
        results.push(ExecuteResponse::from_query_result(delete_result, "Delete IpdDoctorInCharge"));
    }

    Ok(results)
}

async fn delete_doctor_in_charge_inner(doctor_in_charge_id: u32, version: i32, pool: &Pool<MySql>, kphis: &str) -> Result<MySqlQueryResult, AppError> {
    let delete_sql = doctor_in_charge::delete_doctor_in_charge(kphis);
    sqlx::query(AssertSqlSafe(delete_sql))
        .bind(doctor_in_charge_id)
        .bind(version)
        .execute(pool)
        .await
        .map_err(|e| Source::SQLx.to_error(500, e, "Delete IpdDoctorInCharge"))
}

#[cfg(test)]
#[rustfmt::skip]
mod tests {

    use super::*;
    use kphis_sqlx_tester::MySqlTester;

    #[tokio::test]
    #[ignore]
    async fn sqlx_get_doctor_in_charge() {
        let tester = MySqlTester::new_hosxp_and_kphis().await;
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis/ipd_doctor_in_charge.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/hosxp/doctor.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/hosxp/spclty.sql")).execute(&tester.db_pool).await.unwrap();

        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/kphis/ipd_doctor_in_charge.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/hosxp/doctor.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/hosxp/spclty.sql")).execute(&tester.db_pool).await.unwrap();

        let found = get_doctor_in_charge("660001234", &tester.db_pool, &tester.hosxp, &tester.kphis).await.unwrap();
        assert_eq!(found.len(), 1);
        let not_found = get_doctor_in_charge("660006666", &tester.db_pool, &tester.hosxp, &tester.kphis).await.unwrap();
        assert!(not_found.is_empty());
    }

    #[tokio::test]
    #[ignore]
    async fn sqlx_update_doctor_in_charge() {
        let tester = MySqlTester::new_kphis().await;
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis/ipd_doctor_in_charge.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/kphis/ipd_doctor_in_charge.sql")).execute(&tester.db_pool).await.unwrap();

        let success = update_doctor_in_charge(&IpdDoctorInCharge::demo(),"user",&tester.db_pool,&tester.kphis).await.unwrap();
        assert_eq!(success.rows_affected(), 1);
        let again_fail_version = update_doctor_in_charge(&IpdDoctorInCharge::demo(),"user",&tester.db_pool,&tester.kphis).await.unwrap();
        assert_eq!(again_fail_version.rows_affected(), 0);
    }

    #[tokio::test]
    #[ignore]
    async fn sqlx_insert_doctor_in_charge() {
        let tester = MySqlTester::new_kphis().await;
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis/ipd_doctor_in_charge.sql")).execute(&tester.db_pool).await.unwrap();

        let success = insert_doctor_in_charge(&IpdDoctorInCharge::demo(),"user",&tester.db_pool,&tester.kphis).await.unwrap();
        assert_eq!(success.rows_affected(), 1);
        let again_success = insert_doctor_in_charge(&IpdDoctorInCharge::demo(),"user",&tester.db_pool,&tester.kphis).await.unwrap();
        assert_eq!(again_success.rows_affected(), 1);
    }

    #[tokio::test]
    #[ignore]
    async fn sqlx_delete_doctor_in_charge_inner() {
        let tester = MySqlTester::new_kphis().await;
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis/ipd_doctor_in_charge.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/kphis/ipd_doctor_in_charge.sql")).execute(&tester.db_pool).await.unwrap();

        let success = delete_doctor_in_charge_inner(1, 1, &tester.db_pool, &tester.kphis).await.unwrap();
        assert_eq!(success.rows_affected(), 1);
        let again_not_found = delete_doctor_in_charge_inner(1, 1, &tester.db_pool, &tester.kphis).await.unwrap();
        assert_eq!(again_not_found.rows_affected(), 0);
    }
}
