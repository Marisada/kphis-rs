use sqlx::{AssertSqlSafe, MySql, Pool, Row, mysql::MySqlRow};

use kphis_model::{app::VisitTypeId, patient_info::PatientInfo};
use kphis_sql::ipd::show_patient_main;
use kphis_util::error::{AppError, Source};

pub async fn get_show_patient_main(an: &str, pool: &Pool<MySql>, hosxp: &str, kphis: &str) -> Result<Option<PatientInfo>, AppError> {
    let sql = show_patient_main::select_show_patient_main(hosxp, kphis);
    sqlx::query(AssertSqlSafe(sql))
        .bind(an)
        .fetch_optional(pool)
        .await
        .map_err(|e| Source::SQLx.to_error(500, e.to_string(), "Select ShowPatientMainAN"))?
        .as_ref()
        .map(from_row)
        .transpose()
        .map_err(|e| Source::SQLx.to_error(500, e.to_string(), "Select ShowPatientMainAN"))
}

fn from_row(row: &MySqlRow) -> sqlx::Result<PatientInfo> {
    let an: String = row.try_get("an")?;
    Ok(PatientInfo {
        visit_type: VisitTypeId::Ipd(an.clone()),
        opd_er_order_master_id: None,
        pre_admit_master_id: None,
        // admission_note_id: row.try_get("admission_note_id")?,
        hn: row.try_get("hn")?,
        an: Some(an),
        vn: row.try_get("vn")?,

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

        allergy_drug_history: row.try_get("allergy_drug_history")?,
        allergy_drug_history_hosxp: row.try_get("allergy_drug_history_hosxp")?,
        allergy_drug_pharmacy_check_person: row.try_get("allergy_drug_pharmacy_check_person")?,
        allergy_drug_pharmacy_check_datetime: row.try_get("allergy_drug_pharmacy_check_datetime")?,

        regdate: row.try_get("regdate")?,
        regtime: row.try_get("regtime")?,
        admdate: row.try_get("admdate")?,
        leave_home_day: row.try_get("leave_home_day")?,
        dchdate: row.try_get("dchdate")?,
        dchtime: row.try_get("dchtime")?,
        dchstts: row.try_get("dchstts")?,
        dchtype: row.try_get("dchtype")?,
        ward: row.try_get("ward")?,
        spclty: row.try_get("spclty")?,
        bedno: row.try_get("bedno")?,
        birth_weight: row.try_get("birth_weight")?,
        bw: row.try_get("bw")?,

        chief_complaints: row.try_get("chief_complaints")?,
        g: row.try_get("g")?,
        p: row.try_get("p")?,
        last_child: row.try_get("last_child")?,
        lmp: row.try_get("lmp")?,
        edc: row.try_get("edc")?,
        gestational_age: row.try_get("gestational_age")?,
        gestational_day: row.try_get("gestational_day")?,
        mem_ruptured_hours: row.try_get("mem_ruptured_hours")?,

        dchstts_name: row.try_get("dchstts_name")?,
        dchtype_name: row.try_get("dchtype_name")?,
        ward_name: row.try_get("ward_name")?,
        spclty_name: row.try_get("spclty_name")?,

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
    async fn sqlx_get_show_patient_main() {
        let tester = MySqlTester::new_hosxp_and_kphis().await;

        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/hosxp/ipt.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/hosxp/an_stat.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/hosxp/ovst.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/hosxp/patient.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/hosxp/ward.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/hosxp/spclty.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/hosxp/pttype.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/hosxp/sex.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/hosxp/tambol.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/hosxp/occupation.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/hosxp/religion.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/hosxp/nationality.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/hosxp/marrystatus.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/hosxp/ipt_newborn.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/hosxp/iptadm.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/hosxp/dchstts.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/hosxp/dchtype.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/hosxp/opd_allergy.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis/ipd_dr_admission_note.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis/opd_er_order_master.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis/opd_er_allergy_history.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis/ipd_vs_vital_sign.sql")).execute(&tester.db_pool).await.unwrap();

        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/hosxp/ipt.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/hosxp/an_stat.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/hosxp/ovst.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/hosxp/patient.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/hosxp/ward.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/hosxp/spclty.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/hosxp/pttype.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/hosxp/sex.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/hosxp/tambol.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/hosxp/occupation.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/hosxp/religion.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/hosxp/nationality.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/hosxp/marrystatus.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/hosxp/ipt_newborn.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/hosxp/iptadm.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/hosxp/dchstts.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/hosxp/dchtype.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/hosxp/opd_allergy.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/kphis/ipd_dr_admission_note.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/kphis/opd_er_order_master.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/kphis/opd_er_allergy_history.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/kphis/ipd_vs_vital_sign.sql")).execute(&tester.db_pool).await.unwrap();

        let found = get_show_patient_main("660001234", &tester.db_pool, &tester.hosxp, &tester.kphis).await.unwrap();
        assert!(found.is_some());
        let not_found = get_show_patient_main("660006666", &tester.db_pool, &tester.hosxp, &tester.kphis).await.unwrap();
        assert!(not_found.is_none());
    }
}
