use sqlx::{AssertSqlSafe, MySql, Pool, Row, mysql::MySqlRow};

use kphis_model::{app::VisitTypeId, patient_info::PatientInfo};
use kphis_sql::opd_er::show_patient_main;
use kphis_util::error::{AppError, Source};

// opd-er-show-patient-main.php
pub async fn get_show_patient_main_id(opd_er_order_master_id: u32, pool: &Pool<MySql>, hosxp: &str, kphis: &str) -> Result<Option<PatientInfo>, AppError> {
    let sql = show_patient_main::select_show_patient_main("opd_er_order_master_id", hosxp, kphis);
    sqlx::query(AssertSqlSafe(sql))
        .bind(opd_er_order_master_id)
        .fetch_optional(pool)
        .await
        .map_err(|e| Source::SQLx.to_error(500, e.to_string(), "Select ShowPatientMainID"))?
        .as_ref()
        .map(opd_er_from_row)
        .transpose()
        .map_err(|e| Source::SQLx.to_error(500, e.to_string(), "Select ShowPatientMainID"))
}

pub async fn get_show_patient_main_vn(vn: &str, pool: &Pool<MySql>, hosxp: &str, kphis: &str) -> Result<Option<PatientInfo>, AppError> {
    let sql = show_patient_main::select_show_patient_main("vn", hosxp, kphis);
    sqlx::query(AssertSqlSafe(sql))
        .bind(vn)
        .fetch_optional(pool)
        .await
        .map_err(|e| Source::SQLx.to_error(500, e.to_string(), "Select ShowPatientMainVN"))?
        .as_ref()
        .map(pre_admit_from_row)
        .transpose()
        .map_err(|e| Source::SQLx.to_error(500, e.to_string(), "Select ShowPatientMainVN"))
}

fn opd_er_from_row(row: &MySqlRow) -> sqlx::Result<PatientInfo> {
    let vn: String = row.try_get("vn")?;
    let opd_er_order_master_id: Option<u32> = row.try_get("opd_er_order_master_id")?;
    let pre_admit_master_id: Option<u32> = row.try_get("pre_admit_master_id")?;
    let visit_type = if let Some(id) = opd_er_order_master_id {
        VisitTypeId::OpdEr(vn.clone(), id)
    } else {
        VisitTypeId::Visit(vn.clone())
    };
    from_row(vn, opd_er_order_master_id, pre_admit_master_id, visit_type, row)
}
fn pre_admit_from_row(row: &MySqlRow) -> sqlx::Result<PatientInfo> {
    let vn: String = row.try_get("vn")?;
    let opd_er_order_master_id: Option<u32> = row.try_get("opd_er_order_master_id")?;
    let pre_admit_master_id: Option<u32> = row.try_get("pre_admit_master_id")?;
    let visit_type = if pre_admit_master_id.is_some() {
        VisitTypeId::PreAdmit(vn.clone())
    } else {
        VisitTypeId::Visit(vn.clone())
    };
    from_row(vn, opd_er_order_master_id, pre_admit_master_id, visit_type, row)
}
fn from_row(vn: String, opd_er_order_master_id: Option<u32>, pre_admit_master_id: Option<u32>, visit_type: VisitTypeId, row: &MySqlRow) -> sqlx::Result<PatientInfo> {
    Ok(PatientInfo {
        visit_type,
        opd_er_order_master_id,
        pre_admit_master_id,
        // admission_note_id: None,
        hn: row.try_get("hn")?,
        an: row.try_get("an")?,
        vn: Some(vn),

        cid: row.try_get("cid")?,
        passport_no: row.try_get("passport_no")?,
        pname: row.try_get("pname")?,
        fname: row.try_get("fname")?,
        lname: row.try_get("lname")?,
        birthday: row.try_get("birthday")?,
        sex: row.try_get("sex")?,

        age_y: row.try_get("age_y")?,
        age_m: row.try_get("age_m")?,
        age_d: row.try_get("age_d")?,

        pttype: row.try_get("pttype")?,
        income: row.try_get("income")?,

        homeaddr: row.try_get("homeaddr")?,
        hometel: row.try_get("hometel")?,
        worktel: row.try_get("worktel")?,
        workaddr: row.try_get("workaddr")?,
        informtel: row.try_get("informtel")?,
        informaddr: row.try_get("informaddr")?,
        informname: row.try_get("informname")?,
        informrelation: row.try_get("informrelation")?,

        vstdate: row.try_get("vstdate")?,
        vsttime: row.try_get("vsttime")?,

        drugallergy: row.try_get("drugallergy")?,
        er_drugallergy_history: row.try_get("er_drugallergy_history")?,

        allergy_drug_history: None,
        allergy_drug_history_hosxp: None,
        allergy_drug_pharmacy_check_person: None,
        allergy_drug_pharmacy_check_datetime: None,

        regdate: None,
        regtime: None,
        admdate: None,
        leave_home_day: None,
        dchdate: None,
        dchtime: None,
        dchstts: None,
        dchtype: None,
        ward: None,
        spclty: None,
        bedno: None,
        birth_weight: None,
        bw: None,

        chief_complaints: None,
        g: None,
        p: None,
        last_child: None,
        lmp: None,
        edc: None,
        gestational_age: None,
        gestational_day: None,
        mem_ruptured_hours: None,

        dchstts_name: None,
        dchtype_name: None,
        ward_name: None,
        spclty_name: None,

        sex_name: row.try_get("sex_name")?,
        pttype_name: row.try_get("pttype_name")?,
        occupation_name: row.try_get("occupation_name")?,
        religion_name: row.try_get("religion_name")?,
        citizenship_name: row.try_get("citizenship_name")?,
        nationality_name: row.try_get("nationality_name")?,
        marrystatus_name: row.try_get("marrystatus_name")?,

        latest_height: row.try_get("latest_height")?,
        latest_bw: row.try_get("latest_bw")?,
        latest_bw_datetime: row.try_get("latest_bw_datetime")?,
        latest_vs_datetime: row.try_get("latest_vs_datetime")?,
    })
}

#[cfg(test)]
#[rustfmt::skip]
mod tests {

    use super::*;
    use kphis_sqlx_tester::MySqlTester;

    #[tokio::test]
    #[ignore]
    async fn sqlx_get_show_patient_main_id() {
        let tester = MySqlTester::new_hosxp_and_kphis().await;

        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/hosxp/ovst.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/hosxp/vn_stat.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/hosxp/patient.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/hosxp/pttype.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/hosxp/sex.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/hosxp/tambol.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/hosxp/occupation.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/hosxp/religion.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/hosxp/nationality.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/hosxp/marrystatus.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/hosxp/opd_allergy.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis/opd_er_order_master.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis/opd_er_allergy_history.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis/opd_er_vs_vital_sign.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis/ipd_pre_admit_master.sql")).execute(&tester.db_pool).await.unwrap();

        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/hosxp/ovst.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/hosxp/vn_stat.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/hosxp/patient.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/hosxp/pttype.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/hosxp/sex.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/hosxp/tambol.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/hosxp/occupation.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/hosxp/religion.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/hosxp/nationality.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/hosxp/marrystatus.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/hosxp/opd_allergy.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/kphis/opd_er_order_master.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/kphis/opd_er_allergy_history.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/kphis/opd_er_vs_vital_sign.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/kphis/ipd_pre_admit_master.sql")).execute(&tester.db_pool).await.unwrap();

        let found = get_show_patient_main_id(1, &tester.db_pool, &tester.hosxp, &tester.kphis).await.unwrap();
        assert!(found.is_some());
        let not_found = get_show_patient_main_id(999, &tester.db_pool, &tester.hosxp, &tester.kphis).await.unwrap();
        assert!(not_found.is_none());
    }

    #[tokio::test]
    #[ignore]
    async fn sqlx_get_show_patient_main_vn() {
        let tester = MySqlTester::new_hosxp_and_kphis().await;

        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/hosxp/ovst.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/hosxp/vn_stat.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/hosxp/patient.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/hosxp/pttype.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/hosxp/sex.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/hosxp/tambol.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/hosxp/occupation.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/hosxp/religion.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/hosxp/nationality.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/hosxp/marrystatus.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/hosxp/opd_allergy.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis/opd_er_order_master.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis/opd_er_allergy_history.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis/opd_er_vs_vital_sign.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis/ipd_pre_admit_master.sql")).execute(&tester.db_pool).await.unwrap();

        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/hosxp/ovst.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/hosxp/vn_stat.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/hosxp/patient.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/hosxp/pttype.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/hosxp/sex.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/hosxp/tambol.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/hosxp/occupation.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/hosxp/religion.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/hosxp/nationality.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/hosxp/marrystatus.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/hosxp/opd_allergy.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/kphis/opd_er_order_master.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/kphis/opd_er_allergy_history.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/kphis/opd_er_vs_vital_sign.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/kphis/ipd_pre_admit_master.sql")).execute(&tester.db_pool).await.unwrap();

        let found = get_show_patient_main_vn("661231235959",&tester.db_pool,&tester.hosxp,&tester.kphis,).await.unwrap();
        assert!(found.is_some());
        let not_found = get_show_patient_main_vn("991231235959",&tester.db_pool,&tester.hosxp,&tester.kphis,).await.unwrap();
        assert!(not_found.is_none());
    }
}
