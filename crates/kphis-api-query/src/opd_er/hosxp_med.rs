use sqlx::{FromRow, MySql, Pool};

use kphis_model::opd_er::hosxp_med::OpdMed;
use kphis_sql::opd_er::hosxp_med;
use kphis_util::error::{AppError, Source};

use crate::query2_all;

// opd-er-hosxp-med-data.php
pub async fn get_opd_med(vn: &str, pool: &Pool<MySql>, hosxp: &str) -> Result<Vec<OpdMed>, AppError> {
    let sql = hosxp_med::select_opd_med(hosxp);
    query2_all(vn, vn, &sql, pool, "Select OpdMed")
        .await?
        .iter()
        .map(OpdMed::from_row)
        .collect::<sqlx::Result<Vec<OpdMed>>>()
        .map_err(|e| Source::SQLx.to_error(500, e, "Select OpdMed"))
}

#[cfg(test)]
#[rustfmt::skip]
mod tests {

    use super::*;
    use kphis_sqlx_tester::MySqlTester;

    #[tokio::test]
    #[ignore]
    async fn sqlx_get_opd_med() {
        let tester = MySqlTester::new_hosxp().await;
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/hosxp/ovst.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/hosxp/opitemrece.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/hosxp/drugitems.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/hosxp/s_drugitems.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/hosxp/drugusage.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/hosxp/sp_use.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/hosxp/ipt_order_no.sql")).execute(&tester.db_pool).await.unwrap();

        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/hosxp/ovst.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/hosxp/opitemrece.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/hosxp/drugitems.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/hosxp/s_drugitems.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/hosxp/drugusage.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/hosxp/sp_use.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/hosxp/ipt_order_no.sql")).execute(&tester.db_pool).await.unwrap();

        let found = get_opd_med("661231235959", &tester.db_pool, &tester.hosxp).await.unwrap();
        assert_eq!(found.len(), 3);
        let not_found = get_opd_med("991231235959", &tester.db_pool, &tester.hosxp).await.unwrap();
        assert!(not_found.is_empty());
    }
}
