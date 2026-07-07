use sqlx::{AssertSqlSafe, MySql, Pool, Row, mysql::MySqlRow};

use kphis_model::{app::VisitTypeId, med_reconcile::MedReconciliationHeader};
use kphis_sql::med_reconciliation;
use kphis_util::error::{AppError, Source};

pub async fn get_med_reconciliation_header(hn: &str, an_len: usize, pool: &Pool<MySql>, hosxp: &str, kphis: &str) -> Result<Vec<MedReconciliationHeader>, AppError> {
    let sql = med_reconciliation::select_med_reconcile_header_by_hn(hosxp, kphis);
    let result = sqlx::query(AssertSqlSafe(sql))
        .bind(hn)
        .bind(hn)
        .fetch_all(pool)
        .await
        .map_err(|e| Source::SQLx.to_error(500, e, "Select MedReconciliationHead"))?
        .iter()
        .map(|r| med_reconciliation_head_from_row(r, an_len))
        .collect::<sqlx::Result<Vec<MedReconciliationHeader>>>()
        .map_err(|e| Source::SQLx.to_error(500, e, "Select MedReconciliationHead"))?;

    Ok(result)
}
fn med_reconciliation_head_from_row(row: &MySqlRow, an_len: usize) -> sqlx::Result<MedReconciliationHeader> {
    let vnid_an: String = row.try_get("vnid_an")?;
    let ss = vnid_an.split(",").collect::<Vec<&str>>();
    let visit_type = if ss.len() > 1 {
        VisitTypeId::OpdEr(ss[0].to_owned(), ss[1].parse::<u32>().unwrap_or_default())
    } else if ss[0].len() > an_len {
        VisitTypeId::PreAdmit(ss[0].to_owned())
    } else {
        VisitTypeId::Ipd(ss[0].to_owned())
    };
    Ok(MedReconciliationHeader {
        visit_type,
        visit_datetime: row.try_get("visit_datetime")?,
        hn: row.try_get("hn")?,
    })
}

#[cfg(test)]
#[rustfmt::skip]
mod tests {

    use super::*;
    use kphis_sqlx_tester::MySqlTester;

    #[tokio::test]
    #[ignore]
    async fn sqlx_get_get_med_reconciliation_header() {
        let tester = MySqlTester::new_hosxp_and_kphis().await;
        sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/create/hosxp/ipt.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/create/hosxp/ovst.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/create/kphis/ipd_med_reconciliation.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/create/kphis/opd_er_med_reconciliation.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/create/kphis/opd_er_order_master.sql")).execute(&tester.db_pool).await.unwrap();

        sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/insert/hosxp/ipt.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/insert/hosxp/ovst.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/insert/kphis/ipd_med_reconciliation.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/insert/kphis/opd_er_med_reconciliation.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/insert/kphis/opd_er_order_master.sql")).execute(&tester.db_pool).await.unwrap();

        let hn_found = get_med_reconciliation_header("0001234",9,&tester.db_pool,&tester.hosxp,&tester.kphis).await.unwrap();
        assert_eq!(hn_found.len(),2);
        let hn_not_found = get_med_reconciliation_header("0006666",9,&tester.db_pool,&tester.hosxp,&tester.kphis).await.unwrap();
        assert!(hn_not_found.is_empty());
    }
}
