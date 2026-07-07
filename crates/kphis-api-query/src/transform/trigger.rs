use sqlx::{AssertSqlSafe, MySql, Pool, Row};

use kphis_model::fetch::ExecuteResponse;
use kphis_sql::transform;
use kphis_util::error::{AppError, Source};

pub async fn select_exists_trg_kphis_ipt_log_insert(pool: &Pool<MySql>, hosxp: &str) -> Result<bool, AppError> {
    let sql = transform::select_exists_trg_kphis_ipt_log_insert(hosxp);
    sqlx::query(AssertSqlSafe(sql))
        .fetch_one(pool)
        .await
        .map(|row| row.try_get(0).unwrap_or_default())
        .map_err(|e| Source::SQLx.to_error(500, e, "ExistIptInsertTrigger"))
}

pub async fn select_exists_trg_kphis_ipt_log_delete(pool: &Pool<MySql>, hosxp: &str) -> Result<bool, AppError> {
    let sql = transform::select_exists_trg_kphis_ipt_log_delete(hosxp);
    sqlx::query(AssertSqlSafe(sql))
        .fetch_one(pool)
        .await
        .map(|row| row.try_get(0).unwrap_or_default())
        .map_err(|e| Source::SQLx.to_error(500, e, "ExistIptDeleteTrigger"))
}

#[rustfmt::skip]
pub async fn add_ipt_insert_trigger(pool: &Pool<MySql>, hosxp: &str, kphis_log: &str) -> Result<Vec<ExecuteResponse>, AppError> {
    let mut results = Vec::with_capacity(2);

    let drop_sql = transform::drop_trg_kphis_ipt_log_insert(hosxp);
    let drop_result = sqlx::query(AssertSqlSafe(drop_sql))
        .execute(pool)
        .await
        .map(|result| ExecuteResponse::from_query_result(result, "DropIptInsertTrigger"))
        .map_err(|e| Source::SQLx.to_error(500, e, "DropIptInsertTrigger"))?;
    results.push(drop_result);

    let create_sql = transform::create_trg_kphis_ipt_log_insert(hosxp, kphis_log);
    let create_result = sqlx::query(AssertSqlSafe(create_sql))
        .execute(pool)
        .await
        .map(|result| ExecuteResponse::from_query_result(result, "CreateIptInsertTrigger"))
        .map_err(|e| Source::SQLx.to_error(500, e, "CreateIptInsertTrigger"))?;
    results.push(create_result);

    Ok(results)
}

#[rustfmt::skip]
pub async fn add_ipt_delete_trigger(pool: &Pool<MySql>, hosxp: &str, kphis_log: &str) -> Result<Vec<ExecuteResponse>, AppError> {
    let mut results = Vec::with_capacity(2);

    let drop_sql = transform::drop_trg_kphis_ipt_log_delete(hosxp);
    let drop_result = sqlx::query(AssertSqlSafe(drop_sql))
        .execute(pool)
        .await
        .map(|result| ExecuteResponse::from_query_result(result, "DropIptDeleteTrigger"))
        .map_err(|e| Source::SQLx.to_error(500, e, "DropIptDeleteTrigger"))?;
    results.push(drop_result);

    let create_sql = transform::create_trg_kphis_ipt_log_delete(hosxp, kphis_log);
    let create_result = sqlx::query(AssertSqlSafe(create_sql))
        .execute(pool)
        .await
        .map(|result| ExecuteResponse::from_query_result(result, "CreateIptDeleteTrigger"))
        .map_err(|e| Source::SQLx.to_error(500, e, "CreateIptDeleteTrigger"))?;
    results.push(create_result);

    Ok(results)
}

#[rustfmt::skip]
pub async fn add_ipt_log_insert_trigger(pool: &Pool<MySql>, kphis: &str, kphis_log: &str) -> Result<Vec<ExecuteResponse>, AppError> {
    let mut results = Vec::with_capacity(2);

    let drop_sql = transform::drop_trg_ipt_log_insert(kphis_log);
    let drop_result = sqlx::query(AssertSqlSafe(drop_sql))
        .execute(pool)
        .await
        .map(|result| ExecuteResponse::from_query_result(result, "DropIptLogInsertTrigger"))
        .map_err(|e| Source::SQLx.to_error(500, e, "DropIptLogInsertTrigger"))?;
    results.push(drop_result);

    let create_sql = transform::create_trg_ipt_log_insert(kphis, kphis_log);
    let create_result = sqlx::query(AssertSqlSafe(create_sql))
        .execute(pool)
        .await
        .map(|result| ExecuteResponse::from_query_result(result, "CreateIptLogInsertTrigger"))
        .map_err(|e| Source::SQLx.to_error(500, e, "CreateIptLogInsertTrigger"))?;
    results.push(create_result);

    Ok(results)
}

#[rustfmt::skip]
pub async fn add_update_all_an_procedure(pool: &Pool<MySql>, kphis: &str, kphis_extra: &str) -> Result<Vec<ExecuteResponse>, AppError> {
    let mut results = Vec::with_capacity(2);

    let drop_sql = transform::drop_proc_update_all_an(kphis);
    let drop_result = sqlx::query(AssertSqlSafe(drop_sql))
        .execute(pool)
        .await
        .map(|result| ExecuteResponse::from_query_result(result, "DropUpdateAllAnProcedure"))
        .map_err(|e| Source::SQLx.to_error(500, e, "DropUpdateAllAnProcedure"))?;
    results.push(drop_result);

    let create_sql = transform::create_proc_update_all_an(kphis, kphis_extra);
    let create_result = sqlx::query(AssertSqlSafe(create_sql))
        .execute(pool)
        .await
        .map(|result| ExecuteResponse::from_query_result(result, "CreateUpdateAllAnProcedure"))
        .map_err(|e| Source::SQLx.to_error(500, e, "CreateUpdateAllAnProcedure"))?;
    results.push(create_result);

    Ok(results)
}

#[rustfmt::skip]
pub async fn add_any_an_exists_procedure(pool: &Pool<MySql>, kphis: &str, kphis_extra: &str) -> Result<Vec<ExecuteResponse>, AppError> {
    let mut results = Vec::with_capacity(2);

    let drop_sql = transform::drop_proc_any_an_exists(kphis);
    let drop_result = sqlx::query(AssertSqlSafe(drop_sql))
        .execute(pool)
        .await
        .map(|result| ExecuteResponse::from_query_result(result, "DropAllAnExistsProcedure"))
        .map_err(|e| Source::SQLx.to_error(500, e, "DropAllAnExistsProcedure"))?;
    results.push(drop_result);

    let create_sql = transform::create_proc_any_an_exists(kphis, kphis_extra);
    let create_result = sqlx::query(AssertSqlSafe(create_sql))
        .execute(pool)
        .await
        .map(|result| ExecuteResponse::from_query_result(result, "CreateAllAnExistsProcedure"))
        .map_err(|e| Source::SQLx.to_error(500, e, "CreateAllAnExistsProcedure"))?;
    results.push(create_result);

    Ok(results)
}

pub async fn call_update_all_an_procedure(old_an: &str, new_an: &str, pool: &Pool<MySql>, kphis: &str) -> Result<ExecuteResponse, AppError> {
    let sql = transform::call_proc_update_all_an(kphis);
    sqlx::query(AssertSqlSafe(sql))
        .bind(old_an)
        .bind(new_an)
        .execute(pool)
        .await
        .map(|result| ExecuteResponse::from_query_result(result, "CallUpdateProcedure"))
        .map_err(|e| Source::SQLx.to_error(500, e, "CallUpdateProcedure"))
}

#[cfg(test)]
#[rustfmt::skip]
mod tests {

    use sqlx::Row;
    use kphis_model::transform::IptLog;
    use kphis_sqlx_tester::MySqlTester;

    use crate::{
        pre_admit::{AnTester, create_all_an, insert_all_an, select_pre_admit_by_vn},
        transform::query::select_ipt_log,
    };
    use super::*;

    async fn test_any_an_exists_procedure(old_an: &str, pool: &Pool<MySql>, kphis: &str) -> Result<bool, AppError> {
        let create_sql = [
            "CREATE PROCEDURE `", kphis, "`.`test_any_an_exists`(IN old_an VARCHAR(13)) \
            BEGIN \
                CALL `", kphis, "`.`proc_any_an_exists`(old_an, @an_exists);\
                SELECT @an_exists;\
            END;"].concat();
        let _create_result = sqlx::query(AssertSqlSafe(create_sql))
            .execute(pool)
            .await
            .map(|result| ExecuteResponse::from_query_result(result, "CreateTestAnyAnExistsProcedure"))
            .map_err(|e| Source::SQLx.to_error(500, e, "CreateTestAnyAnExistsProcedure"))?;
        let sql = ["CALL ",kphis,".test_any_an_exists('",old_an,"');"].concat();
        sqlx::query(AssertSqlSafe(sql)).fetch_one(pool).await
            .map_err(|e| Source::SQLx.to_error(500, e, "CallTestAnyAnExistsProcedure"))?
            .try_get(0)
            .map_err(|e| Source::SQLx.to_error(500, e, "CallTestAnyAnExistsProcedure"))
    }

    async fn insert_ipt_log(ipt_log: &IptLog, pool: &Pool<MySql>, kphis_log: &str) -> Result<ExecuteResponse, AppError> {
        let sql = ["INSERT INTO ",kphis_log,".ipt_log (ipt_log_type, an, vn, hn, ward, create_datetime) VALUES (?,?,?,?,?,NOW());"].concat();
        sqlx::query(AssertSqlSafe(sql))
            .bind(&ipt_log.ipt_log_type)
            .bind(&ipt_log.an)
            .bind(&ipt_log.vn)
            .bind(&ipt_log.hn)
            .bind(&ipt_log.ward)
            .execute(pool).await
            .map(|result| ExecuteResponse::from_query_result(result, "InsertIptLog"))
            .map_err(|e| Source::SQLx.to_error(500, e, "InsertIptLog"))
    }

    #[tokio::test]
    #[ignore]
    async fn sqlx_any_an_exists_procedure_empty() {
        let tester = MySqlTester::new_kphis_and_kphis_extra().await;
        create_all_an(&tester).await;

        assert!(add_any_an_exists_procedure(&tester.db_pool, &tester.kphis, &tester.kphis_extra).await.is_ok());
        assert!(add_any_an_exists_procedure(&tester.db_pool, &tester.kphis, &tester.kphis_extra).await.is_ok());
        assert!(!test_any_an_exists_procedure("660001234", &tester.db_pool, &tester.kphis).await.unwrap());
    }

    #[tokio::test]
    #[ignore]
    async fn sqlx_any_an_exists_procedure_full() {
        let tester = MySqlTester::new_kphis_and_kphis_extra().await;
        create_all_an(&tester).await;
        insert_all_an(&tester).await;

        assert!(add_any_an_exists_procedure(&tester.db_pool, &tester.kphis, &tester.kphis_extra).await.is_ok());
        assert!(test_any_an_exists_procedure("660001234", &tester.db_pool, &tester.kphis).await.unwrap());
    }

    #[tokio::test]
    #[ignore]
    async fn sqlx_update_all_an_procedure() {
        let tester = MySqlTester::new_kphis_and_kphis_extra().await;
        create_all_an(&tester).await;
        insert_all_an(&tester).await;

        let mut an_tester = AnTester::default();

        let old_an = "660001234";
        an_tester.add_before(old_an, &tester).await;

        assert!(add_update_all_an_procedure(&tester.db_pool, &tester.kphis, &tester.kphis_extra).await.is_ok());
        assert!(add_update_all_an_procedure(&tester.db_pool, &tester.kphis, &tester.kphis_extra).await.is_ok());

        let new_an = "123456789012";
        assert!(call_update_all_an_procedure(old_an, new_an, &tester.db_pool, &tester.kphis).await.is_ok());

        an_tester.add_after(new_an, &tester).await;
        an_tester.compare_all();
    }

    #[tokio::test]
    #[ignore]
    async fn sqlx_add_ipt_insert_trigger() {
        let tester = MySqlTester::new_hosxp_and_kphis_log().await;

        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/hosxp/ipt.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis_log/ipt_log.sql")).execute(&tester.db_pool).await.unwrap();

        assert!(add_ipt_insert_trigger(&tester.db_pool, &tester.hosxp, &tester.kphis_log).await.is_ok());
        assert!(add_ipt_insert_trigger(&tester.db_pool, &tester.hosxp, &tester.kphis_log).await.is_ok());

        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/hosxp/ipt.sql")).execute(&tester.db_pool).await.unwrap();
        let ipt_logs = select_ipt_log(&tester.db_pool, &tester.kphis_log).await.unwrap();
        assert_eq!(ipt_logs.len(), 14);
    }

    #[tokio::test]
    #[ignore]
    async fn sqlx_select_exists_trg_kphis_ipt_log_insert() {
        let tester = MySqlTester::new_hosxp_and_kphis_log().await;

        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/hosxp/ipt.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis_log/ipt_log.sql")).execute(&tester.db_pool).await.unwrap();

        let not_exists = select_exists_trg_kphis_ipt_log_insert(&tester.db_pool, &tester.hosxp).await.unwrap();
        assert!(!not_exists);

        assert!(add_ipt_insert_trigger(&tester.db_pool, &tester.hosxp, &tester.kphis_log).await.is_ok());

        let exists = select_exists_trg_kphis_ipt_log_insert(&tester.db_pool, &tester.hosxp).await.unwrap();
        assert!(exists);
    }

    #[tokio::test]
    #[ignore]
    async fn sqlx_add_ipt_delete_trigger() {
        let tester = MySqlTester::new_hosxp_and_kphis_log().await;

        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/hosxp/ipt.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis_log/ipt_log.sql")).execute(&tester.db_pool).await.unwrap();

        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/hosxp/ipt.sql")).execute(&tester.db_pool).await.unwrap();

        assert!(add_ipt_delete_trigger(&tester.db_pool, &tester.hosxp, &tester.kphis_log).await.is_ok());
        assert!(add_ipt_delete_trigger(&tester.db_pool, &tester.hosxp, &tester.kphis_log).await.is_ok());

        let delete_sql = ["DELETE FROM ", &tester.hosxp, ".ipt;"].concat();
        let delete_result= sqlx::query(AssertSqlSafe(delete_sql)).execute(&tester.db_pool).await.unwrap();
        let ipt_logs = select_ipt_log(&tester.db_pool, &tester.kphis_log).await.unwrap();
        assert_eq!(ipt_logs.len() as u64, delete_result.rows_affected());
    }

    #[tokio::test]
    #[ignore]
    async fn sqlx_select_exists_trg_kphis_ipt_log_delete() {
        let tester = MySqlTester::new_hosxp_and_kphis_log().await;

        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/hosxp/ipt.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis_log/ipt_log.sql")).execute(&tester.db_pool).await.unwrap();

        let not_exists = select_exists_trg_kphis_ipt_log_delete(&tester.db_pool, &tester.hosxp).await.unwrap();
        assert!(!not_exists);

        assert!(add_ipt_delete_trigger(&tester.db_pool, &tester.hosxp, &tester.kphis_log).await.is_ok());

        let exists = select_exists_trg_kphis_ipt_log_delete(&tester.db_pool, &tester.hosxp).await.unwrap();
        assert!(exists);
    }

    #[tokio::test]
    #[ignore]
    async fn sqlx_add_ipt_log_insert_trigger() {
        let tester = MySqlTester::new_kphis_log().await;

        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis_log/ipt_log.sql")).execute(&tester.db_pool).await.unwrap();

        assert!(add_ipt_log_insert_trigger(&tester.db_pool, &tester.kphis, &tester.kphis_log).await.is_ok());
        assert!(add_ipt_log_insert_trigger(&tester.db_pool, &tester.kphis, &tester.kphis_log).await.is_ok());
    }

    #[tokio::test]
    #[ignore]
    async fn sqlx_insert_ipt_log_without_pre_admit() {
        let tester = MySqlTester::new_kphis_and_kphis_log().await;

        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis/ipd_pre_admit_master.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis_log/ipt_log.sql")).execute(&tester.db_pool).await.unwrap();

        assert!(add_ipt_log_insert_trigger(&tester.db_pool, &tester.kphis, &tester.kphis_log).await.is_ok());

        assert!(select_pre_admit_by_vn("661231235959", &tester.db_pool, &tester.kphis).await.unwrap().is_none());
        let new_i_log = IptLog {
            ipt_log_id: 0,
            ipt_log_type: String::from("I"),
            an: String::from("66006666"),
            vn: String::from("661231235959"),
            hn: Some(String::from("0001234")),
            ward: Some(String::from("01")),
        };
        let _ = insert_ipt_log(&new_i_log, &tester.db_pool, &tester.kphis_log).await.unwrap();
        let pre_admits = select_pre_admit_by_vn("661231235959", &tester.db_pool, &tester.kphis).await.unwrap();
        assert!(pre_admits.is_none());
        // not call Stored Procedure `update_all_an`
    }

    #[tokio::test]
    #[ignore]
    async fn sqlx_insert_ipt_log_with_new_pre_admit() {
        let tester = MySqlTester::new_kphis_and_kphis_log_and_kphis_extra().await;

        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis/ipd_pre_admit_master.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis_log/ipt_log.sql")).execute(&tester.db_pool).await.unwrap();

        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/kphis/ipd_pre_admit_master.sql")).execute(&tester.db_pool).await.unwrap();

        create_all_an(&tester).await;

        assert!(add_any_an_exists_procedure(&tester.db_pool, &tester.kphis, &tester.kphis_extra).await.is_ok());
        assert!(add_ipt_log_insert_trigger(&tester.db_pool, &tester.kphis, &tester.kphis_log).await.is_ok());

        if let Some(pre_admit) = select_pre_admit_by_vn("670111111111", &tester.db_pool, &tester.kphis).await.unwrap() {
            assert_eq!(pre_admit.an, None);
            assert_eq!(pre_admit.prev_an, None);
        }
        let new_i_log = IptLog {
            ipt_log_id: 0,
            ipt_log_type: String::from("I"),
            an: String::from("66006666"),
            vn: String::from("670111111111"),
            hn: Some(String::from("0001234")),
            ward: Some(String::from("01")),
        };
        let _ = insert_ipt_log(&new_i_log, &tester.db_pool, &tester.kphis_log).await.unwrap();
        if let Some(pre_admit) = select_pre_admit_by_vn("670111111111", &tester.db_pool, &tester.kphis).await.unwrap() {
            assert_eq!(pre_admit.an, Some(String::from("66006666"))); // changed
            assert_eq!(pre_admit.prev_an, None);
        }
        // not call Stored Procedure `update_all_an`
    }

    #[tokio::test]
    #[ignore]
    async fn sqlx_insert_ipt_log_with_old_pre_admit_without_data() {
        let tester = MySqlTester::new_kphis_and_kphis_log_and_kphis_extra().await;

        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis/ipd_pre_admit_master.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis_log/ipt_log.sql")).execute(&tester.db_pool).await.unwrap();

        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/kphis/ipd_pre_admit_master.sql")).execute(&tester.db_pool).await.unwrap();

        create_all_an(&tester).await;

        assert!(add_any_an_exists_procedure(&tester.db_pool, &tester.kphis, &tester.kphis_extra).await.is_ok());
        assert!(add_ipt_log_insert_trigger(&tester.db_pool, &tester.kphis, &tester.kphis_log).await.is_ok());

        if let Some(pre_admit) = select_pre_admit_by_vn("661231235959", &tester.db_pool, &tester.kphis).await.unwrap() {
            assert_eq!(pre_admit.an, Some(String::from("660001234")));
            assert_eq!(pre_admit.prev_an, Some(String::from("660001111")));
        }
        let new_i_log = IptLog {
            ipt_log_id: 0,
            ipt_log_type: String::from("I"),
            an: String::from("660009999"),
            vn: String::from("661231235959"),
            hn: Some(String::from("0001234")),
            ward: Some(String::from("01")),
        };
        let _ = insert_ipt_log(&new_i_log, &tester.db_pool, &tester.kphis_log).await.unwrap();
        if let Some(pre_admit) = select_pre_admit_by_vn("661231235959", &tester.db_pool, &tester.kphis).await.unwrap() {
            assert_eq!(pre_admit.an, Some(String::from("660009999"))); // changed
            assert_eq!(pre_admit.prev_an, Some(String::from("660001234"))); // changed
        }
        // not call Stored Procedure `update_all_an`
    }

    #[tokio::test]
    #[ignore]
    async fn sqlx_insert_ipt_log_with_old_pre_admit_with_data() {
        let tester = MySqlTester::new_kphis_and_kphis_log_and_kphis_extra().await;

        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis/ipd_pre_admit_master.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis_log/ipt_log.sql")).execute(&tester.db_pool).await.unwrap();

        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/kphis/ipd_pre_admit_master.sql")).execute(&tester.db_pool).await.unwrap();

        create_all_an(&tester).await;
        insert_all_an(&tester).await;

        assert!(add_update_all_an_procedure(&tester.db_pool, &tester.kphis, &tester.kphis_extra).await.is_ok());
        assert!(add_any_an_exists_procedure(&tester.db_pool, &tester.kphis, &tester.kphis_extra).await.is_ok());
        assert!(add_ipt_log_insert_trigger(&tester.db_pool, &tester.kphis, &tester.kphis_log).await.is_ok());

        assert!(call_update_all_an_procedure("660001234", "661231235959", &tester.db_pool, &tester.kphis).await.is_ok());

        let mut an_tester = AnTester::default();

        let old_an = "661231235959";
        an_tester.add_before(old_an, &tester).await;

        if let Some(pre_admit) = select_pre_admit_by_vn("661231235959", &tester.db_pool, &tester.kphis).await.unwrap() {
            assert_eq!(pre_admit.an, Some(String::from("660001234")));
            assert_eq!(pre_admit.prev_an, Some(String::from("660001111")));
        }
        let new_i_log = IptLog {
            ipt_log_id: 0,
            ipt_log_type: String::from("I"),
            an: String::from("660009999"),
            vn: String::from("661231235959"),
            hn: Some(String::from("0001234")),
            ward: Some(String::from("01")),
        };
        let _ = insert_ipt_log(&new_i_log, &tester.db_pool, &tester.kphis_log).await.unwrap();
        if let Some(pre_admit) = select_pre_admit_by_vn("661231235959", &tester.db_pool, &tester.kphis).await.unwrap() {
            assert_eq!(pre_admit.an, Some(String::from("660009999"))); // changed
            assert_eq!(pre_admit.prev_an, Some(String::from("660001234"))); // changed
        }

        let new_an = "660009999";
        an_tester.add_after(new_an, &tester).await;
        an_tester.compare_all();
    }

    #[tokio::test]
    #[ignore]
    async fn sqlx_delete_ipt_log_without_pre_admit_without_data() {
        let tester = MySqlTester::new_kphis_and_kphis_log_and_kphis_extra().await;

        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis/ipd_pre_admit_master.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis_log/ipt_log.sql")).execute(&tester.db_pool).await.unwrap();

        create_all_an(&tester).await;

        assert!(add_any_an_exists_procedure(&tester.db_pool, &tester.kphis, &tester.kphis_extra).await.is_ok());
        assert!(add_ipt_log_insert_trigger(&tester.db_pool, &tester.kphis, &tester.kphis_log).await.is_ok());

        assert!(select_pre_admit_by_vn("661231235959", &tester.db_pool, &tester.kphis).await.unwrap().is_none());
        let new_d_log = IptLog {
            ipt_log_id: 0,
            ipt_log_type: String::from("D"),
            an: String::from("660001234"),
            vn: String::from("661231235959"),
            hn: Some(String::from("0001234")),
            ward: Some(String::from("01")),
        };
        let _ = insert_ipt_log(&new_d_log, &tester.db_pool, &tester.kphis_log).await.unwrap();
        assert!(select_pre_admit_by_vn("661231235959", &tester.db_pool, &tester.kphis).await.unwrap().is_none());
        // not call Stored Procedure `update_all_an`
    }

    #[tokio::test]
    #[ignore]
    async fn sqlx_delete_ipt_log_without_pre_admit_with_data() {
        let tester = MySqlTester::new_kphis_and_kphis_log_and_kphis_extra().await;

        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis/ipd_pre_admit_master.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis_log/ipt_log.sql")).execute(&tester.db_pool).await.unwrap();

        create_all_an(&tester).await;
        insert_all_an(&tester).await;

        assert!(add_update_all_an_procedure(&tester.db_pool, &tester.kphis, &tester.kphis_extra).await.is_ok());
        assert!(add_any_an_exists_procedure(&tester.db_pool, &tester.kphis, &tester.kphis_extra).await.is_ok());
        assert!(add_ipt_log_insert_trigger(&tester.db_pool, &tester.kphis, &tester.kphis_log).await.is_ok());

        let mut an_tester = AnTester::default();

        let old_an = "660001234";
        an_tester.add_before(old_an, &tester).await;

        assert!(select_pre_admit_by_vn("661231235959", &tester.db_pool, &tester.kphis).await.unwrap().is_none());
        let new_d_log = IptLog {
            ipt_log_id: 0,
            ipt_log_type: String::from("D"),
            an: String::from("660001234"),
            vn: String::from("661231235959"),
            hn: Some(String::from("0001234")),
            ward: Some(String::from("01")),
        };
        let _ = insert_ipt_log(&new_d_log, &tester.db_pool, &tester.kphis_log).await.unwrap();
        if let Some(pre_admit) = select_pre_admit_by_vn("661231235959", &tester.db_pool, &tester.kphis).await.unwrap() {
            assert_eq!(pre_admit.an, None);
            assert_eq!(pre_admit.prev_an, Some(String::from("660001234")));
        }

        let new_an = "661231235959";
        an_tester.add_after(new_an, &tester).await;
        an_tester.compare_all();
    }

    #[tokio::test]
    #[ignore]
    async fn sqlx_delete_ipt_log_with_old_pre_admit_without_data() {
        let tester = MySqlTester::new_kphis_and_kphis_log_and_kphis_extra().await;

        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis/ipd_pre_admit_master.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis_log/ipt_log.sql")).execute(&tester.db_pool).await.unwrap();

        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/kphis/ipd_pre_admit_master.sql")).execute(&tester.db_pool).await.unwrap();

        create_all_an(&tester).await;

        assert!(add_any_an_exists_procedure(&tester.db_pool, &tester.kphis, &tester.kphis_extra).await.is_ok());
        assert!(add_ipt_log_insert_trigger(&tester.db_pool, &tester.kphis, &tester.kphis_log).await.is_ok());

        if let Some(pre_admit) = select_pre_admit_by_vn("661231235959", &tester.db_pool, &tester.kphis).await.unwrap() {
            assert_eq!(pre_admit.an, Some(String::from("660001234")));
            assert_eq!(pre_admit.prev_an,  Some(String::from("660001111")));
        }
        let new_d_log = IptLog {
            ipt_log_id: 0,
            ipt_log_type: String::from("D"),
            an: String::from("660001234"),
            vn: String::from("661231235959"),
            hn: Some(String::from("0001234")),
            ward: Some(String::from("01")),
        };
        let _ = insert_ipt_log(&new_d_log, &tester.db_pool, &tester.kphis_log).await.unwrap();
        if let Some(pre_admit) = select_pre_admit_by_vn("661231235959", &tester.db_pool, &tester.kphis).await.unwrap() {
            assert_eq!(pre_admit.an, None); // changed
            assert_eq!(pre_admit.prev_an, Some(String::from("660001234"))); // changed
        }
        // not call Stored Procedure `update_all_an`
    }

    #[tokio::test]
    #[ignore]
    async fn sqlx_delete_ipt_log_with_old_pre_admit_and_data() {
        let tester = MySqlTester::new_kphis_and_kphis_log_and_kphis_extra().await;

        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis/ipd_pre_admit_master.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis_log/ipt_log.sql")).execute(&tester.db_pool).await.unwrap();

        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/kphis/ipd_pre_admit_master.sql")).execute(&tester.db_pool).await.unwrap();

        create_all_an(&tester).await;
        insert_all_an(&tester).await;

        assert!(add_update_all_an_procedure(&tester.db_pool, &tester.kphis, &tester.kphis_extra).await.is_ok());
        assert!(add_any_an_exists_procedure(&tester.db_pool, &tester.kphis, &tester.kphis_extra).await.is_ok());
        assert!(add_ipt_log_insert_trigger(&tester.db_pool, &tester.kphis, &tester.kphis_log).await.is_ok());

        let mut an_tester = AnTester::default();

        let old_an = "660001234";
        an_tester.add_before(old_an, &tester).await;

        if let Some(pre_admit) = select_pre_admit_by_vn("661231235959", &tester.db_pool, &tester.kphis).await.unwrap() {
            assert_eq!(pre_admit.an, Some(String::from("660001234")));
            assert_eq!(pre_admit.prev_an,  Some(String::from("660001111")));
        }
        let new_d_log = IptLog {
            ipt_log_id: 0,
            ipt_log_type: String::from("D"),
            an: String::from("660001234"),
            vn: String::from("661231235959"),
            hn: Some(String::from("0001234")),
            ward: Some(String::from("01")),
        };
        let _ = insert_ipt_log(&new_d_log, &tester.db_pool, &tester.kphis_log).await.unwrap();
        if let Some(pre_admit) = select_pre_admit_by_vn("661231235959", &tester.db_pool, &tester.kphis).await.unwrap() {
            assert_eq!(pre_admit.an, None); // changed
            assert_eq!(pre_admit.prev_an, Some(String::from("660001234"))); // changed
        }

        let new_an = "661231235959";
        an_tester.add_after(new_an, &tester).await;
        an_tester.compare_all();
    }
}
