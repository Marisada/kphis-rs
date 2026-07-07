use sqlx::{AssertSqlSafe, FromRow, MySql, Pool, mysql::MySqlQueryResult};

use kphis_model::ipd::mra::{IpdMra, MraParams};
use kphis_sql::{TABLE_CREATE_COLUMNS, TABLE_CREATE_PREPARED, TABLE_UPDATE_SET, ipd::mra};
use kphis_util::error::{AppError, Source};

pub async fn get_ipd_mra(params: &MraParams, pool: &Pool<MySql>, kphis_extra: &str) -> Result<Vec<IpdMra>, AppError> {
    let sql = mra::select_mra(params, kphis_extra);

    let mut query = sqlx::query(AssertSqlSafe(sql));

    if let Some(an) = params.an.as_ref() {
        query = query.bind(an);
    }

    query
        .fetch_all(pool)
        .await
        .map_err(|e| Source::SQLx.to_error(500, e, "Select IpdMra"))?
        .iter()
        .map(IpdMra::from_row)
        .collect::<sqlx::Result<Vec<IpdMra>>>()
        .map_err(|e| Source::SQLx.to_error(500, e, "Select IpdMra"))
}

pub async fn post_ipd_mra(form: &IpdMra, user: &str, pool: &Pool<MySql>, kphis_extra: &str) -> Result<MySqlQueryResult, AppError> {
    form.insert(Some("mra_id"), None, TABLE_CREATE_COLUMNS, TABLE_CREATE_PREPARED, &[user, user], pool, kphis_extra)
        .await
        .map_err(|e| Source::SQLx.to_error(500, e, "Insert IpdMra"))
}

pub async fn put_ipd_mra(form: &IpdMra, user: &str, pool: &Pool<MySql>, kphis_extra: &str) -> Result<MySqlQueryResult, AppError> {
    form.update("mra_id", None, TABLE_UPDATE_SET, &[user], pool, kphis_extra)
        .await
        .map_err(|e| Source::SQLx.to_error(500, e, "Update IpdMra"))
}

pub async fn delete_ipd_mra(mra_id: u32, pool: &Pool<MySql>, kphis_extra: &str) -> Result<MySqlQueryResult, AppError> {
    let sql = mra::delete_mra(kphis_extra);

    sqlx::query(AssertSqlSafe(sql))
        .bind(mra_id)
        .execute(pool)
        .await
        .map_err(|e| Source::SQLx.to_error(500, e, "Delete IpdMra"))
}

#[cfg(test)]
#[rustfmt::skip]
mod tests {

    use super::*;
    use kphis_sqlx_tester::MySqlTester;

    #[tokio::test]
    #[ignore]
    async fn sqlx_get_ipd_mra() {
        let tester = MySqlTester::new_kphis_extra().await;
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis_extra/ipd_mra.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/kphis_extra/ipd_mra.sql")).execute(&tester.db_pool).await.unwrap();

        let default = get_ipd_mra(&MraParams::default(),&tester.db_pool,&tester.kphis_extra).await.unwrap();
        assert_eq!(default.len(), 1);
        let found_an = get_ipd_mra(&MraParams {an: Some(String::from("660001234")),..Default::default()},&tester.db_pool,&tester.kphis_extra).await.unwrap();
        assert_eq!(found_an.len(), 1);
        let not_found = get_ipd_mra(&MraParams {an: Some(String::from("660006666")),..Default::default()},&tester.db_pool,&tester.kphis_extra).await.unwrap();
        assert!(not_found.is_empty());
    }

    #[tokio::test]
    #[ignore]
    async fn sqlx_post_ipd_mra() {
        let tester = MySqlTester::new_kphis_extra().await;
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis_extra/ipd_mra.sql")).execute(&tester.db_pool).await.unwrap();

        let success = post_ipd_mra(&IpdMra::demo(),"user",&tester.db_pool,&tester.kphis_extra).await.unwrap();
        assert_eq!(success.rows_affected(), 1);
        let again_success = post_ipd_mra(&IpdMra::demo(),"user",&tester.db_pool,&tester.kphis_extra).await.unwrap();
        assert_eq!(again_success.rows_affected(), 1);
    }

    #[tokio::test]
    #[ignore]
    async fn sqlx_put_ipd_mra() {
        let tester = MySqlTester::new_kphis_extra().await;
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis_extra/ipd_mra.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/kphis_extra/ipd_mra.sql")).execute(&tester.db_pool).await.unwrap();

        let success = put_ipd_mra(&IpdMra::demo(),"user",&tester.db_pool,&tester.kphis_extra).await.unwrap();
        assert_eq!(success.rows_affected(), 1);
        let again_success = put_ipd_mra(&IpdMra::demo(),"user",&tester.db_pool,&tester.kphis_extra).await.unwrap();
        assert_eq!(again_success.rows_affected(), 1);
    }

    #[tokio::test]
    #[ignore]
    async fn sqlx_delete_ipd_mra() {
        let tester = MySqlTester::new_kphis_extra().await;
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis_extra/ipd_mra.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/kphis_extra/ipd_mra.sql")).execute(&tester.db_pool).await.unwrap();

        let success = delete_ipd_mra(1, &tester.db_pool, &tester.kphis_extra).await.unwrap();
        assert_eq!(success.rows_affected(), 1);
        let again_not_found = delete_ipd_mra(1, &tester.db_pool, &tester.kphis_extra).await.unwrap();
        assert_eq!(again_not_found.rows_affected(), 0);
    }
}
