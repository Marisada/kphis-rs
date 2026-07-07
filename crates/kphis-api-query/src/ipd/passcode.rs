use sqlx::{Error, MySql, Pool, Row, mysql::MySqlRow};

use kphis_model::ipd::passcode::ConfigIpdWardPasscode;
use kphis_sql::ipd::passcode;
use kphis_util::error::{AppError, Source};

use crate::{execute1, execute4, query1_all};

// kphis-config-ipd-ward-passcode-data.php
pub async fn get_ward_passcode(loginname: &str, pool: &Pool<MySql>, hosxp: &str, kphis: &str) -> Result<Vec<ConfigIpdWardPasscode>, AppError> {
    let sql = passcode::select_config_ipd_ward_passcode_data(hosxp, kphis);
    query1_all(loginname, &sql, pool, "Select WardPascode")
        .await?
        .iter()
        .map(ward_passcode_from_row)
        .collect::<sqlx::Result<Vec<ConfigIpdWardPasscode>>>()
        .map_err(|e| Source::SQLx.to_error(500, e, "Select WardPascode"))
}
fn ward_passcode_from_row(row: &MySqlRow) -> Result<ConfigIpdWardPasscode, Error> {
    // database store 'Y' or 'N'
    let using_passcode_string: String = row.try_get("using_passcode")?;
    let using_passcode = using_passcode_string == "Y";
    Ok(ConfigIpdWardPasscode {
        ward: row.try_get("ward")?,
        ward_name: row.try_get("ward_name")?,
        using_passcode,
    })
}

// // kphis-config-ipd-ward-passcode-gen.php
// pub async fn get_specific_ward_passcode(
//     ward: &str,
//     loginname: &str,
//     pool: &Pool<MySql>, kphis: &str,
// ) -> Result<Option<ConfigIpdWardPasscode>, AppError> {
//     let sql = passcode::select_can_change_specific_ward_passcode(kphis);
//     query2_opt(ward, loginname, &sql, pool, "Select SpecificWardPasscode").await?
//         .as_ref().map(ward_passcode_from_row).transpose()
//         .map_err(|e| Source::SQLx.to_error(500, e, "Select SpecificWardPasscode"))
// }

// kphis-config-ipd-ward-passcode-gen.php
pub async fn post_ward_passcode(ward: &str, passcode: &str, loginname: &str, pool: &Pool<MySql>, kphis: &str) -> Result<u64, AppError> {
    let sql = passcode::replace_ward_passcode(kphis);
    let result = execute4(ward, passcode, loginname, loginname, &sql, pool, "Replace WardPasscode").await?;

    Ok(result.rows_affected())
}

// kphis-config-ipd-ward-passcode-gen.php
pub async fn delete_ward_passcode(ward: &str, pool: &Pool<MySql>, kphis: &str) -> Result<u64, AppError> {
    let sql = passcode::delete_ward_passcode(kphis);
    let result = execute1(ward, &sql, pool, "Delete WardPasscode").await?;

    Ok(result.rows_affected())
}

#[cfg(test)]
#[rustfmt::skip]
mod tests {

    use super::*;
    use kphis_sqlx_tester::MySqlTester;

    #[tokio::test]
    #[ignore]
    async fn sqlx_get_ward_passcode() {
        let tester = MySqlTester::new_hosxp_and_kphis().await;
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/hosxp/ward.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis/ipd_ward_passcode.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis/ipd_ward_passcode_user.sql")).execute(&tester.db_pool).await.unwrap();

        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/hosxp/ward.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/kphis/ipd_ward_passcode.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/kphis/ipd_ward_passcode_user.sql")).execute(&tester.db_pool).await.unwrap();

        let found = get_ward_passcode("user", &tester.db_pool, &tester.hosxp, &tester.kphis).await.unwrap();
        assert_eq!(found.len(), 1);
        let not_found = get_ward_passcode("xxxx", &tester.db_pool, &tester.hosxp, &tester.kphis).await.unwrap();
        assert!(not_found.is_empty());
    }

    #[tokio::test]
    #[ignore]
    async fn sqlx_post_ward_passcode() {
        let tester = MySqlTester::new_kphis().await;
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis/ipd_ward_passcode.sql")).execute(&tester.db_pool).await.unwrap();

        let success = post_ward_passcode("01", "1234", "user", &tester.db_pool, &tester.kphis).await.unwrap();
        assert_eq!(success, 1);
        let again_success = post_ward_passcode("01", "1234", "user", &tester.db_pool, &tester.kphis).await.unwrap();
        assert_eq!(again_success, 1);
    }

    #[tokio::test]
    #[ignore]
    async fn sqlx_delete_ward_passcode() {
        let tester = MySqlTester::new_kphis().await;
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis/ipd_ward_passcode.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/kphis/ipd_ward_passcode.sql")).execute(&tester.db_pool).await.unwrap();

        let success = delete_ward_passcode("01", &tester.db_pool, &tester.kphis).await.unwrap();
        assert_eq!(success, 1);
        let again_not_found = delete_ward_passcode("01", &tester.db_pool, &tester.kphis).await.unwrap();
        assert_eq!(again_not_found, 0);
    }
}
