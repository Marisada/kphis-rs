use sqlx::{FromRow, MySql, Pool};

use kphis_model::search::ipd_search_patient_nurse::{IpdSearchPatientNurseRequest, IpdSearchPatientNurseResponse};
use kphis_sql::search::ipd_search_patient_nurse::sql_and_filter;
use kphis_util::error::{AppError, Source};

use crate::{query_all, query1_all, query2_all, query3_all};

// ipd-nurse-search-patient-table.php
pub async fn get_ipd_nurse_search_patient(
    request: IpdSearchPatientNurseRequest,
    hn_len: usize,
    an_len: usize,
    pool: &Pool<MySql>,
    hosxp: &str,
    kphis: &str,
    kphis_extra: &str,
) -> Result<Vec<IpdSearchPatientNurseResponse>, AppError> {
    let patient = request
        .patient
        .as_ref()
        .and_then(|patient| urlencoding::decode(&patient).map(|s| s.into_owned()).ok())
        .unwrap_or_default();
    let ward = request.ward.clone().unwrap_or_default();
    let doctor = request.doctor_in_charge.clone().unwrap_or_default();
    let passcode = request.passcode.clone().unwrap_or_default();
    let patient_wildcard = ["%", &patient, "%"].concat();
    let (sql, filter) = sql_and_filter(request.clone(), hn_len, an_len, hosxp, kphis, kphis_extra);
    let rows = match (filter.has_patient, filter.pt_is_num, filter.anlen_eq_hnlen, filter.has_ward, filter.has_doctor, filter.has_passcode) {
        (true, true, false, _, _, true) => query2_all(&patient, &passcode, &sql, pool, "Select IpdNurseSearchPatient-1").await,
        (true, true, false, _, _, false) => query1_all(&patient, &sql, pool, "Select IpdNurseSearchPatient-2").await,
        (true, false, true, _, _, true) => query3_all(&patient_wildcard, &patient_wildcard, &passcode, &sql, pool, "Select IpdNurseSearchPatient-3").await,
        (true, false, true, _, _, false) => query2_all(&patient_wildcard, &patient_wildcard, &sql, pool, "Select IpdNurseSearchPatient-4").await,
        (true, _, _, _, _, true) => query2_all(&patient_wildcard, &passcode, &sql, pool, "Select IpdNurseSearchPatient-5").await,
        (true, _, _, _, _, false) => query1_all(&patient_wildcard, &sql, pool, "Select IpdNurseSearchPatient-6").await,
        (false, _, _, true, true, true) => query3_all(&ward, &doctor, &passcode, &sql, pool, "Select IpdNurseSearchPatient-7").await,
        (false, _, _, true, true, false) => query2_all(&ward, &doctor, &sql, pool, "Select IpdNurseSearchPatient-8").await,
        (false, _, _, true, false, true) => query2_all(&ward, &passcode, &sql, pool, "Select IpdNurseSearchPatient-9").await,
        (false, _, _, true, false, false) => query1_all(&ward, &sql, pool, "Select IpdNurseSearchPatient-10").await,
        (false, _, _, false, true, true) => query2_all(&doctor, &passcode, &sql, pool, "Select IpdNurseSearchPatient-11").await,
        (false, _, _, false, true, false) => query1_all(&doctor, &sql, pool, "Select IpdNurseSearchPatient-12").await,
        (false, _, _, false, false, true) => query1_all(&passcode, &sql, pool, "Select IpdNurseSearchPatient-13").await,
        (false, _, _, false, false, false) => query_all(&sql, pool, "Select IpdNurseSearchPatient-14").await,
    }?;

    rows.iter()
        .map(IpdSearchPatientNurseResponse::from_row)
        .collect::<sqlx::Result<Vec<IpdSearchPatientNurseResponse>>>()
        .map_err(|e| Source::SQLx.to_error(500, e, "Select IpdNurseSearchPatient"))
}

#[cfg(test)]
#[rustfmt::skip]
mod tests {

    use super::*;
    use kphis_sqlx_tester::MySqlTester;

    #[tokio::test]
    #[ignore]
    async fn sqlx_get_ipd_nurse_search_patient() {
        let tester = MySqlTester::new_hosxp_and_kphis_and_kphis_extra().await;

        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis/ipd_doctor_in_charge.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis/ipd_dr_consult.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis/ipd_dr_consult_signature_reply.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis/ipd_dr_admission_note.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis/ipd_nurse_admission_note.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis/ipd_med_reconciliation.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis/ipd_focus_note.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis/ipd_order.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis/ipd_order_item.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis/ipd_nurse_index_plan.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis/ipd_nurse_index_action.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis/ipd_ward_passcode.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis/ipd_vs_vital_sign.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis/ipd_lab_read.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis/ipd_xray_read.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis/opd_er_order_master.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis/kphis_drug_use_duration.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis_extra/ipd_nurse_index_monitor.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/hosxp/ipt.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/hosxp/doctor.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/hosxp/spclty.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/hosxp/iptadm.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/hosxp/patient.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/hosxp/sex.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/hosxp/roomno.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/hosxp/an_stat.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/hosxp/ward.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/hosxp/dchtype.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/hosxp/dchstts.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/hosxp/pttype.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/hosxp/lab_head.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/hosxp/xray_report.sql")).execute(&tester.db_pool).await.unwrap();

        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/kphis/ipd_doctor_in_charge.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/kphis/ipd_dr_consult.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/kphis/ipd_dr_consult_signature_reply.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/kphis/ipd_dr_admission_note.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/kphis/ipd_nurse_admission_note.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/kphis/ipd_med_reconciliation.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/kphis/ipd_focus_note.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/kphis/ipd_order.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/kphis/ipd_order_item.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/kphis/ipd_nurse_index_plan.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/kphis/ipd_nurse_index_action.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/kphis/ipd_vs_vital_sign.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/kphis/ipd_lab_read.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/kphis/ipd_xray_read.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/kphis/kphis_drug_use_duration.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/kphis/opd_er_order_master.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/kphis_extra/ipd_nurse_index_monitor.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/hosxp/ipt.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/hosxp/doctor.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/hosxp/spclty.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/hosxp/iptadm.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/hosxp/patient.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/hosxp/sex.sql")).execute(&tester.db_pool).await.unwrap();
        // sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/hosxp/roomno.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/hosxp/an_stat.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/hosxp/ward.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/hosxp/dchtype.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/hosxp/dchstts.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/hosxp/pttype.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/hosxp/lab_head.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/hosxp/xray_report.sql")).execute(&tester.db_pool).await.unwrap();

        // not discharged
        let default = get_ipd_nurse_search_patient(IpdSearchPatientNurseRequest::default(), 7, 9, &tester.db_pool, &tester.hosxp, &tester.kphis, &tester.kphis_extra).await.unwrap();
        assert_eq!(default.len(), 13);

        // include discharged
        let found_hn = get_ipd_nurse_search_patient(IpdSearchPatientNurseRequest {patient: Some(String::from("1234")), ..Default::default()}, 7, 9, &tester.db_pool, &tester.hosxp, &tester.kphis, &tester.kphis_extra).await.unwrap();
        assert_eq!(found_hn.len(), 2);
        let found_an = get_ipd_nurse_search_patient(IpdSearchPatientNurseRequest {patient: Some(String::from("60001234")), ..Default::default()}, 7, 9, &tester.db_pool, &tester.hosxp, &tester.kphis, &tester.kphis_extra).await.unwrap();
        assert_eq!(found_an.len(), 1);
        let found_cid = get_ipd_nurse_search_patient(IpdSearchPatientNurseRequest {patient: Some(String::from("1111111111111")), ..Default::default()}, 7, 9, &tester.db_pool, &tester.hosxp, &tester.kphis, &tester.kphis_extra).await.unwrap();
        assert_eq!(found_cid.len(), 2);
        let found_fullname = get_ipd_nurse_search_patient(IpdSearchPatientNurseRequest {patient: Some(String::from("มมุติ")), ..Default::default()}, 7, 9, &tester.db_pool, &tester.hosxp, &tester.kphis, &tester.kphis_extra).await.unwrap();
        assert_eq!(found_fullname.len(), 2);

        // not discharged
        let found_ward = get_ipd_nurse_search_patient(IpdSearchPatientNurseRequest {ward: Some(String::from("01")), ..Default::default()}, 7, 9, &tester.db_pool, &tester.hosxp, &tester.kphis, &tester.kphis_extra).await.unwrap();
        assert_eq!(found_ward.len(), 1);
        let found_doctor = get_ipd_nurse_search_patient(IpdSearchPatientNurseRequest {doctor_in_charge: Some(String::from("007")), ..Default::default()}, 7, 9, &tester.db_pool, &tester.hosxp, &tester.kphis, &tester.kphis_extra).await.unwrap();
        assert_eq!(found_doctor.len(), 1);

        // not discharged
        let no_passcode = get_ipd_nurse_search_patient(IpdSearchPatientNurseRequest {passcode: Some(String::from("1234")), ..Default::default()}, 7, 9, &tester.db_pool, &tester.hosxp, &tester.kphis, &tester.kphis_extra).await.unwrap();
        assert_eq!(no_passcode.len(), 13);
        // with passcode
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/kphis/ipd_ward_passcode.sql")).execute(&tester.db_pool).await.unwrap();
        let true_passcode = get_ipd_nurse_search_patient(IpdSearchPatientNurseRequest {passcode: Some(String::from("1234")), ..Default::default()}, 7, 9, &tester.db_pool, &tester.hosxp, &tester.kphis, &tester.kphis_extra).await.unwrap();
        assert_eq!(true_passcode.len(), 13);
        let false_passcode = get_ipd_nurse_search_patient(IpdSearchPatientNurseRequest {passcode: Some(String::from("6666")), ..Default::default()}, 7, 9, &tester.db_pool, &tester.hosxp, &tester.kphis, &tester.kphis_extra).await.unwrap();
        assert_eq!(false_passcode.len(), 12);
    }
}
