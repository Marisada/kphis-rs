use sqlx::{AssertSqlSafe, FromRow, MySql, Pool};

use kphis_model::{fetch::ExecuteResponse, xray::XrayReport};
use kphis_sql::xray;
use kphis_util::error::{AppError, Source};

// GET /xray/report-hn/{hn}
pub async fn get_xray_report(hn: &str, pool: &Pool<MySql>, hosxp: &str, kphis: &str) -> Result<Vec<XrayReport>, AppError> {
    let sql = xray::get_xray_report(hosxp, kphis);
    sqlx::query(AssertSqlSafe(sql))
        .bind(hn)
        .fetch_all(pool)
        .await
        .map_err(|e| Source::SQLx.to_error(500, e, "Select XrayReport"))?
        .iter()
        .map(XrayReport::from_row)
        .collect::<sqlx::Result<Vec<XrayReport>>>()
        .map_err(|e| Source::SQLx.to_error(500, e, "Select XrayReport"))
}

// POST /xray/read-id/{xn}
pub async fn post_xray_read(xn: i32, loginname: &str, pool: &Pool<MySql>, kphis: &str) -> Result<ExecuteResponse, AppError> {
    let sql = xray::insert_ignore_xray_read(kphis);
    let result = sqlx::query(AssertSqlSafe(sql))
        .bind(xn)
        .bind(loginname)
        .bind(loginname)
        .bind(loginname)
        .execute(pool)
        .await
        .map_err(|e| Source::SQLx.to_error(500, e, "Insert XrayRead"))?;

    Ok(ExecuteResponse::from_query_result(result, "Insert XrayRead"))
}

// DELETE /xray/read-id/{xn}
pub async fn delete_xray_read(xn: i32, pool: &Pool<MySql>, kphis: &str) -> Result<ExecuteResponse, AppError> {
    let sql = xray::delete_xray_read(kphis);
    let result = sqlx::query(AssertSqlSafe(sql))
        .bind(xn)
        .execute(pool)
        .await
        .map_err(|e| Source::SQLx.to_error(500, e, "Delete XrayRead"))?;

    Ok(ExecuteResponse::from_query_result(result, "Delete XrayRead"))
}

#[cfg(test)]
#[rustfmt::skip]
mod tests {

    use super::*;
    use kphis_sqlx_tester::MySqlTester;

    #[tokio::test]
    #[ignore]
    async fn sqlx_select_xray_report() {
        let tester = MySqlTester::new_hosxp_and_kphis().await;
        sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/create/hosxp/xray_report.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/create/hosxp/xray_items.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/create/hosxp/doctor.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/create/hosxp/opduser.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/create/kphis/ipd_xray_read.sql")).execute(&tester.db_pool).await.unwrap();

        sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/insert/hosxp/xray_report.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/insert/hosxp/xray_items.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/insert/hosxp/doctor.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/insert/hosxp/opduser.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/insert/kphis/ipd_xray_read.sql")).execute(&tester.db_pool).await.unwrap();

        let hn_found = get_xray_report("0001234",&tester.db_pool,&tester.hosxp,&tester.kphis).await.unwrap();
        assert_eq!(hn_found.len(), 3);

        let not_found = get_xray_report("0006666",&tester.db_pool,&tester.hosxp,&tester.kphis).await.unwrap();
        assert!(not_found.is_empty());
    }

    #[tokio::test]
    #[ignore]
    async fn sqlx_post_xray_read() {
        let tester = MySqlTester::new_kphis().await;
        sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/create/kphis/ipd_xray_read.sql")).execute(&tester.db_pool).await.unwrap();

        let resp = post_xray_read(1,"user",&tester.db_pool,&tester.kphis).await.unwrap();
        assert_eq!(resp.rows_affected,1);
        // INSERT IGNORE test with same PK
        let resp_again = post_xray_read(1,"user",&tester.db_pool,&tester.kphis).await.unwrap();
        assert_eq!(resp_again.rows_affected,0);
    }

    #[tokio::test]
    #[ignore]
    async fn sqlx_delete_xray_read() {
        let tester = MySqlTester::new_kphis().await;
        sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/create/kphis/ipd_xray_read.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/insert/kphis/ipd_xray_read.sql")).execute(&tester.db_pool).await.unwrap();

        let resp = delete_xray_read(1,&tester.db_pool,&tester.kphis).await.unwrap();
        assert_eq!(resp.rows_affected,1);
        let resp_again = delete_xray_read(1,&tester.db_pool,&tester.kphis).await.unwrap();
        assert_eq!(resp_again.rows_affected,0);
    }
}
