use sqlx::{MySql, Pool};

use kphis_sql::query_utils;
use kphis_util::error::AppError;

use super::query2_opt;

pub async fn can_access_table(table_schema: &str, table_name: &str, pool: &Pool<MySql>) -> Result<bool, AppError> {
    let sql = query_utils::can_access_table();
    let result = query2_opt(table_schema, table_name, sql, pool, "Can Access").await?;

    Ok(result.is_some())
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
}
