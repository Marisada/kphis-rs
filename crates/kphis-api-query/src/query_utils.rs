use sqlx::{MySql, Pool, Row};

use kphis_sql::query_utils;
use kphis_util::error::{AppError, Source};

use super::{query1_opt, query2_opt};

pub async fn can_access_table(table_schema: &str, table_name: &str, pool: &Pool<MySql>) -> Result<bool, AppError> {
    let sql = query_utils::can_access_table();
    let result = query2_opt(table_schema, table_name, sql, pool, "Can Access").await?;

    Ok(result.is_some())
}

pub async fn get_vn_by_an(an: &str, pool: &Pool<MySql>, hosxp: &str) -> Result<Option<String>, AppError> {
    let sql = query_utils::get_vn_by_an(hosxp);
    query1_opt(an, &sql, pool, "GetVnByAn")
        .await?
        .map(|row| row.try_get::<String, usize>(0))
        .transpose()
        .map_err(|e| Source::SQLx.to_error(500, e, "GetVnByAn"))
}

#[cfg(test)]
#[rustfmt::skip]
mod tests {

    use super::*;
    use kphis_sqlx_tester::MySqlTester;

    #[tokio::test]
    #[ignore]
    async fn sqlx_can_access_table() {
        let tester = MySqlTester::new_kphis().await;
        sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/create/kphis/ipd_ward_passcode_user.sql")).execute(&tester.db_pool).await.unwrap();
        let can = can_access_table("kphis", "ipd_ward_passcode_user", &tester.db_pool).await.unwrap();
        assert!(can);
        let cannot = can_access_table("hos", "opduser", &tester.db_pool).await.unwrap();
        assert!(!cannot);
    }

    #[tokio::test]
    #[ignore]
    async fn sqlx_get_vn_by_an() {
        let tester = MySqlTester::new_hosxp().await;
        sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/create/hosxp/ipt.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/insert/hosxp/ipt.sql")).execute(&tester.db_pool).await.unwrap();

        let found = get_vn_by_an("660001234", &tester.db_pool, &tester.hosxp).await.unwrap();
        assert_eq!(found, Some(String::from("661231235959")));
        let not_found = get_vn_by_an("666666666", &tester.db_pool, &tester.hosxp).await.unwrap();
        assert!(not_found.is_none());
    }
}
