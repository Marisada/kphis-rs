use sqlx::{AssertSqlSafe, FromRow, MySql, Pool, Row};

use kphis_model::{
    emr::{EmrDate, EmrVisit},
    image::scan_his::ScanHisExists,
};
use kphis_sql::{emr, image::scan_his};
use kphis_util::error::{AppError, Source};

use super::{prescription::select_next_app, query1_all, query1_opt, query2_all};

pub async fn get_emr_date(hn: &str, pool: &Pool<MySql>, hosxp: &str, kphis: &str) -> Result<Vec<EmrDate>, AppError> {
    let sql = emr::select_visits(hosxp, kphis);
    let result = query1_all(hn, &sql, pool, "Select EmrDate")
        .await?
        .iter()
        .map(EmrDate::from_row)
        .collect::<sqlx::Result<Vec<EmrDate>>>()
        .map_err(|e| Source::SQLx.to_error(500, e, "Select EmrDate"))?;

    Ok(result)
}

pub async fn get_emr_visit(vn: &str, pool: &Pool<MySql>, hosxp: &str) -> Result<Option<EmrVisit>, AppError> {
    let mut result = select_visit_detail(vn, pool, hosxp).await?;

    if let Some(visit) = result.as_mut() {
        visit.diagnoses = select_diagnosis(vn, pool, hosxp).await?;
        visit.drugs = select_drug(vn, false, pool, hosxp).await?;
        visit.nondrugs = select_nondrug(vn, false, pool, hosxp).await?;
        visit.next_app = select_next_app(&vn, pool, hosxp).await?;

        if let Some(an) = visit.an.as_ref() {
            visit.home_drugs = select_drug(an, true, pool, hosxp).await?;
            visit.home_nondrugs = select_nondrug(an, true, pool, hosxp).await?;
        }
        visit.image_exists = select_his_image_exists(&visit.vn, &visit.an, pool, hosxp).await?;
    }

    Ok(result)
}

async fn select_visit_detail(vn: &str, pool: &Pool<MySql>, hosxp: &str) -> Result<Option<EmrVisit>, AppError> {
    let visit_sql = emr::select_visit_detail(hosxp);
    query1_opt(vn, &visit_sql, pool, "Select EmrVisit")
        .await?
        .as_ref()
        .map(EmrVisit::from_row)
        .transpose()
        .map_err(|e| Source::SQLx.to_error(500, e, "Select EmrVisit"))
}

async fn select_diagnosis(vn: &str, pool: &Pool<MySql>, hosxp: &str) -> Result<Vec<String>, AppError> {
    let diagnoses_sql = emr::select_diagnosis(hosxp);
    query2_all(vn, vn, &diagnoses_sql, pool, "Select EmrDiagnoses")
        .await?
        .iter()
        .filter_map(|row| row.try_get("diagnosis").transpose())
        .collect::<sqlx::Result<Vec<String>>>()
        .map_err(|e| Source::SQLx.to_error(500, e, "Select EmrDiagnoses"))
}

/// is_home_med use an, else vn
async fn select_drug(vnan: &str, is_home_med: bool, pool: &Pool<MySql>, hosxp: &str) -> Result<Vec<String>, AppError> {
    let drugs_sql = emr::select_drug(hosxp, is_home_med);
    query1_all(vnan, &drugs_sql, pool, "Select EmrDrugs")
        .await?
        .iter()
        .filter_map(|row| row.try_get("drug").transpose())
        .collect::<sqlx::Result<Vec<String>>>()
        .map_err(|e| Source::SQLx.to_error(500, e, "Select EmrDrugs"))
}

/// is_home_med use an, else vn
async fn select_nondrug(vnan: &str, is_home_med: bool, pool: &Pool<MySql>, hosxp: &str) -> Result<Vec<String>, AppError> {
    let nondrugs_sql = emr::select_nondrug(hosxp, is_home_med);
    query1_all(vnan, &nondrugs_sql, pool, "Select EmrNonDrugs")
        .await?
        .iter()
        .filter_map(|row| row.try_get("nondrug").transpose())
        .collect::<sqlx::Result<Vec<String>>>()
        .map_err(|e| Source::SQLx.to_error(500, e, "Select EmrNonDrugs"))
}

async fn select_his_image_exists(vn: &str, an_opt: &Option<String>, pool: &Pool<MySql>, hosxp: &str) -> Result<ScanHisExists, AppError> {
    let image_exists_sql = scan_his::select_his_image_exists(an_opt.is_some(), hosxp);
    let mut query = sqlx::query(AssertSqlSafe(image_exists_sql)).bind(vn).bind(vn).bind(vn);
    if let Some(an) = an_opt {
        query = query.bind(an);
    }
    query
        .bind(vn)
        .fetch_one(pool)
        .await
        .as_ref()
        .map(ScanHisExists::from_row)
        .map_err(|e| Source::SQLx.to_error(500, e, "Select ScanHisExists"))?
        .map_err(|e| Source::SQLx.to_error(500, e, "Select ScanHisExists"))
}

#[cfg(test)]
#[rustfmt::skip]
mod tests {

    use super::*;
    use kphis_sqlx_tester::MySqlTester;

    #[tokio::test]
    #[ignore]
    async fn sqlx_get_emr_date() {
        let tester = MySqlTester::new_hosxp_and_kphis().await;
        sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/create/hosxp/ovst.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/create/hosxp/ipt.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/create/kphis/opd_er_order_master.sql")).execute(&tester.db_pool).await.unwrap();

        sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/insert/hosxp/ovst.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/insert/hosxp/ipt.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/insert/kphis/opd_er_order_master.sql")).execute(&tester.db_pool).await.unwrap();

        let found = get_emr_date("0001234", &tester.db_pool, &tester.hosxp, &tester.kphis).await.unwrap();
        assert_eq!(found.len(), 4);
        let not_found = get_emr_date("0006666", &tester.db_pool, &tester.hosxp, &tester.kphis).await.unwrap();
        assert!(not_found.is_empty());
    }

    #[tokio::test]
    #[ignore]
    async fn sqlx_select_visit_detail() {
        let tester = MySqlTester::new_hosxp().await;
        sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/create/hosxp/ovst.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/create/hosxp/ipt.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/create/hosxp/vn_stat.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/create/hosxp/doctor.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/create/hosxp/patient.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/create/hosxp/pttype.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/create/hosxp/opdscreen.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/create/hosxp/ovstist.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/create/hosxp/referout.sql")).execute(&tester.db_pool).await.unwrap();

        sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/insert/hosxp/ovst.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/insert/hosxp/ipt.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/insert/hosxp/vn_stat.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/insert/hosxp/doctor.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/insert/hosxp/patient.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/insert/hosxp/pttype.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/insert/hosxp/opdscreen.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/insert/hosxp/ovstist.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/insert/hosxp/referout.sql")).execute(&tester.db_pool).await.unwrap();

        let found = select_visit_detail("661231235959", &tester.db_pool, &tester.hosxp).await.unwrap();
        assert!(found.is_some());
        let not_found = select_visit_detail("991231235959", &tester.db_pool, &tester.hosxp).await.unwrap();
        assert!(not_found.is_none());
    }

    #[tokio::test]
    #[ignore]
    async fn sqlx_select_diagnosis() {
        let tester = MySqlTester::new_hosxp().await;
        sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/create/hosxp/ovstdiag.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/create/hosxp/icd101.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/create/hosxp/icd9cm1.sql")).execute(&tester.db_pool).await.unwrap();

        sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/insert/hosxp/ovstdiag.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/insert/hosxp/icd101.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/insert/hosxp/icd9cm1.sql")).execute(&tester.db_pool).await.unwrap();

        let found = select_diagnosis("661231235959", &tester.db_pool, &tester.hosxp).await.unwrap();
        assert_eq!(found.len(), 2);
        let not_found = select_diagnosis("991231235959", &tester.db_pool, &tester.hosxp).await.unwrap();
        assert!(not_found.is_empty());
    }

    #[tokio::test]
    #[ignore]
    async fn sqlx_select_drug() {
        let tester = MySqlTester::new_hosxp().await;
        sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/create/hosxp/opitemrece.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/create/hosxp/drugitems.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/create/hosxp/drugusage.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/create/hosxp/sp_use.sql")).execute(&tester.db_pool).await.unwrap();

        sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/insert/hosxp/opitemrece.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/insert/hosxp/drugitems.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/insert/hosxp/drugusage.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/insert/hosxp/sp_use.sql")).execute(&tester.db_pool).await.unwrap();

        let found_vn = select_drug("661231235959", false, &tester.db_pool, &tester.hosxp).await.unwrap();
        assert_eq!(found_vn.len(), 6);
        let found_an = select_drug("660001234", true, &tester.db_pool, &tester.hosxp).await.unwrap();
        assert_eq!(found_an.len(), 1);
        let not_found = select_drug("991231235959", false, &tester.db_pool, &tester.hosxp).await.unwrap();
        assert!(not_found.is_empty());
    }

    #[tokio::test]
    #[ignore]
    async fn sqlx_select_nondrug() {
        let tester = MySqlTester::new_hosxp().await;
        sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/create/hosxp/opitemrece.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/create/hosxp/nondrugitems.sql")).execute(&tester.db_pool).await.unwrap();

        sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/insert/hosxp/opitemrece.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/insert/hosxp/nondrugitems.sql")).execute(&tester.db_pool).await.unwrap();

        let found_vn = select_nondrug("661231235959", false, &tester.db_pool, &tester.hosxp).await.unwrap();
        assert_eq!(found_vn.len(), 4);
        let found_an = select_nondrug("660001234", true, &tester.db_pool, &tester.hosxp).await.unwrap();
        assert_eq!(found_an.len(), 1);
        let not_found = select_nondrug("991231235959", false, &tester.db_pool, &tester.hosxp).await.unwrap();
        assert!(not_found.is_empty());
    }

    #[tokio::test]
    #[ignore]
    async fn sqlx_select_his_image_exists() {
        let tester = MySqlTester::new_hosxp().await;
        sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/create/hosxp/pe_image.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/create/hosxp/er_image.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/create/hosxp/lab_order_image.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/create/hosxp/patient_opd_scan.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/create/hosxp/lab_head.sql")).execute(&tester.db_pool).await.unwrap();

        sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/insert/hosxp/pe_image.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/insert/hosxp/er_image.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/insert/hosxp/lab_order_image.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/insert/hosxp/patient_opd_scan.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/insert/hosxp/lab_head.sql")).execute(&tester.db_pool).await.unwrap();

        let found = select_his_image_exists("651231235959", &None, &tester.db_pool, &tester.hosxp).await.unwrap();
        assert!(found.has_scan);
        assert!(found.has_pe);
        assert!(found.has_er);
        assert!(found.has_lab);
        let found_ipd = select_his_image_exists("651231235959", &Some(String::from("650001234")), &tester.db_pool, &tester.hosxp).await.unwrap();
        assert!(found_ipd.has_scan);
        assert!(found_ipd.has_pe);
        assert!(found_ipd.has_er);
        assert!(found_ipd.has_lab);
        let not_found = select_his_image_exists("991231235959", &None, &tester.db_pool, &tester.hosxp).await.unwrap();
        assert!(!not_found.has_scan);
        assert!(!not_found.has_pe);
        assert!(!not_found.has_er);
        assert!(!not_found.has_lab);
    }
}
