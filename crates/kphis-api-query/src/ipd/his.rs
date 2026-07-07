use sqlx::{AssertSqlSafe, FromRow, MySql, Pool};

use kphis_model::ipd::his::{HisIptDiag, HisIptOprt, HisMedPlanIpd, HisOperationAdmit};
use kphis_sql::ipd::his;
use kphis_util::error::{AppError, Source};

pub async fn get_operation_admit(an: &str, operation_success: &[u64], pool: &Pool<MySql>, hosxp: &str) -> Result<Vec<HisOperationAdmit>, AppError> {
    let or_sql = his::select_hosxp_operation(operation_success, hosxp);
    let or_result = sqlx::query(AssertSqlSafe(or_sql))
        .bind(an)
        .fetch_all(pool)
        .await
        .map_err(|e| Source::SQLx.to_error(500, e, "Select HisIpdOperationAdmitData"))?
        .iter()
        .map(HisOperationAdmit::from_row)
        .collect::<sqlx::Result<Vec<HisOperationAdmit>>>()
        .map_err(|e| Source::SQLx.to_error(500, e, "Select HisIpdOperationAdmitData"))?;

    Ok(or_result)
}

pub async fn get_medplan_ipd_remains(an: &str, pool: &Pool<MySql>, hosxp: &str) -> Result<Vec<HisMedPlanIpd>, AppError> {
    let or_sql = his::select_hosxp_medplan_ipd_remains(hosxp);
    let or_result = sqlx::query(AssertSqlSafe(or_sql))
        .bind(an)
        .fetch_all(pool)
        .await
        .map_err(|e| Source::SQLx.to_error(500, e, "Select HisMedPlanIpd"))?
        .iter()
        .map(HisMedPlanIpd::from_row)
        .collect::<sqlx::Result<Vec<HisMedPlanIpd>>>()
        .map_err(|e| Source::SQLx.to_error(500, e, "Select HisMedPlanIpd"))?;

    Ok(or_result)
}

pub async fn get_ipt_diag(an: &str, pool: &Pool<MySql>, hosxp: &str) -> Result<Vec<HisIptDiag>, AppError> {
    let sql = his::select_hosxp_ipt_diag(hosxp);
    let result = sqlx::query(AssertSqlSafe(sql))
        .bind(an)
        .fetch_all(pool)
        .await
        .map_err(|e| Source::SQLx.to_error(500, e, "Select HisIptDiag"))?
        .iter()
        .map(HisIptDiag::from_row)
        .collect::<sqlx::Result<Vec<HisIptDiag>>>()
        .map_err(|e| Source::SQLx.to_error(500, e, "Select HisIptDiag"))?;

    Ok(result)
}

pub async fn get_ipt_oprt(an: &str, pool: &Pool<MySql>, hosxp: &str) -> Result<Vec<HisIptOprt>, AppError> {
    let sql = his::select_hosxp_ipt_oprt(hosxp);
    let result = sqlx::query(AssertSqlSafe(sql))
        .bind(an)
        .fetch_all(pool)
        .await
        .map_err(|e| Source::SQLx.to_error(500, e, "Select HisIptOprt"))?
        .iter()
        .map(HisIptOprt::from_row)
        .collect::<sqlx::Result<Vec<HisIptOprt>>>()
        .map_err(|e| Source::SQLx.to_error(500, e, "Select HisIptOprt"))?;

    Ok(result)
}

#[cfg(test)]
#[rustfmt::skip]
mod tests {

    use super::*;
    use kphis_sqlx_tester::MySqlTester;

    #[tokio::test]
    #[ignore]
    async fn sqlx_get_operation_admit() {
        let tester = MySqlTester::new_hosxp().await;
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/hosxp/operation_list.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/hosxp/operation_detail.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/hosxp/operation_item.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/hosxp/doctor.sql")).execute(&tester.db_pool).await.unwrap();

        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/hosxp/operation_list.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/hosxp/operation_detail.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/hosxp/operation_item.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/hosxp/doctor.sql")).execute(&tester.db_pool).await.unwrap();

        let found = get_operation_admit("660001234", &[3], &tester.db_pool, &tester.hosxp).await.unwrap();
        assert_eq!(found.len(), 1);
        let not_found = get_operation_admit("660006666", &[3], &tester.db_pool, &tester.hosxp).await.unwrap();
        assert!(not_found.is_empty());
    }

    #[tokio::test]
    #[ignore]
    async fn sqlx_get_medplan_ipd_remains() {
        let tester = MySqlTester::new_hosxp().await;
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/hosxp/medplan_ipd.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/hosxp/drugitems.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/hosxp/nondrugitems.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/hosxp/drugusage.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/hosxp/sp_use.sql")).execute(&tester.db_pool).await.unwrap();

        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/hosxp/medplan_ipd.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/hosxp/drugitems.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/hosxp/nondrugitems.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/hosxp/drugusage.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/hosxp/sp_use.sql")).execute(&tester.db_pool).await.unwrap();

        let found = get_medplan_ipd_remains("660001234", &tester.db_pool, &tester.hosxp).await.unwrap();
        assert_eq!(found.len(), 1);
        let not_found = get_medplan_ipd_remains("660006666", &tester.db_pool, &tester.hosxp).await.unwrap();
        assert!(not_found.is_empty());
    }

    #[tokio::test]
    #[ignore]
    async fn sqlx_get_ipt_diag() {
        let tester = MySqlTester::new_hosxp().await;
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/hosxp/iptdiag.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/hosxp/iptdiag.sql")).execute(&tester.db_pool).await.unwrap();

        let found = get_ipt_diag("660001234", &tester.db_pool, &tester.hosxp).await.unwrap();
        assert_eq!(found.len(), 4);
        let not_found = get_ipt_diag("660006666", &tester.db_pool, &tester.hosxp).await.unwrap();
        assert!(not_found.is_empty());
    }

    #[tokio::test]
    #[ignore]
    async fn sqlx_get_ipt_oprt() {
        let tester = MySqlTester::new_hosxp().await;
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/hosxp/iptoprt.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/hosxp/iptoprt.sql")).execute(&tester.db_pool).await.unwrap();

        let found = get_ipt_oprt("660001234", &tester.db_pool, &tester.hosxp).await.unwrap();
        assert_eq!(found.len(), 2);
        let not_found = get_ipt_oprt("660006666", &tester.db_pool, &tester.hosxp).await.unwrap();
        assert!(not_found.is_empty());
    }
}
