use sqlx::{FromRow, MySql, Pool};

use kphis_model::search::ipd_search_patient_pharmacist::{IpdSearchPatientPharmacistRequest, IpdSearchPatientPharmacistResponse};
use kphis_sql::search::ipd_search_patient_pharmacist::sql_and_filter;
use kphis_util::error::{AppError, Source};

use crate::{query_all, query1_all, query2_all};

// ipd-pharmacy-search-patient-table.php
pub async fn get_ipd_pharmacist_search_patient(
    request: IpdSearchPatientPharmacistRequest,
    hn_len: usize,
    an_len: usize,
    pool: &Pool<MySql>,
    hosxp: &str,
    kphis: &str,
    kphis_extra: &str,
) -> Result<Vec<IpdSearchPatientPharmacistResponse>, AppError> {
    let patient = request
        .patient
        .as_ref()
        .and_then(|patient| urlencoding::decode(patient).map(|s| s.into_owned()).ok())
        .unwrap_or_default();
    let ward = request.ward.clone().unwrap_or_default();
    let doctor = request.doctor_in_charge.clone().unwrap_or_default();
    let patient_wildcard = ["%", &patient, "%"].concat();
    let (sql, filter) = sql_and_filter(request, hn_len, an_len, hosxp, kphis, kphis_extra);
    let rows = match (filter.has_patient, filter.pt_is_num, filter.anlen_eq_hnlen, filter.has_ward, filter.has_doctor) {
        (true, true, false, _, _) => query1_all(&patient, &sql, pool, "Select IpdPharmacistSearchPatient-1").await,
        (true, false, true, _, _) => query2_all(&patient_wildcard, &patient_wildcard, &sql, pool, "Select IpdPharmacistSearchPatient-2").await,
        (true, _, _, _, _) => query1_all(&patient_wildcard, &sql, pool, "Select IpdPharmacistSearchPatient-3").await,
        (false, _, _, true, true) => query2_all(&ward, &doctor, &sql, pool, "Select IpdPharmacistSearchPatient-4").await,
        (false, _, _, true, false) => query1_all(&ward, &sql, pool, "Select IpdPharmacistSearchPatient-5").await,
        (false, _, _, false, true) => query1_all(&doctor, &sql, pool, "Select IpdPharmacistSearchPatient-6").await,
        (false, _, _, false, false) => query_all(&sql, pool, "Select IpdPharmacistSearchPatient-7").await,
    }?;

    rows.iter()
        .map(IpdSearchPatientPharmacistResponse::from_row)
        .collect::<sqlx::Result<Vec<IpdSearchPatientPharmacistResponse>>>()
        .map_err(|e| Source::SQLx.to_error(500, e, "Select IpdPharmacistSearchPatient"))
}

#[cfg(test)]
#[rustfmt::skip]
mod tests {

    use super::*;
    use kphis_sqlx_tester::MySqlTester;

    #[tokio::test]
    #[ignore]
    async fn sqlx_ipd_pharmacist_search_patient() {
        let tester = MySqlTester::new_hosxp_and_kphis_and_kphis_extra().await;

        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis/ipd_doctor_in_charge.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis/ipd_dr_admission_note.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis/ipd_med_reconciliation.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis/ipd_order_item.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis/ipd_nurse_index_plan.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis/ipd_nurse_index_action.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis/kphis_drug_use_duration.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis/opd_er_allergy_history.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis_extra/ipd_nurse_index_monitor.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis/opd_er_order_master.sql")).execute(&tester.db_pool).await.unwrap();
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

        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/kphis/ipd_doctor_in_charge.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/kphis/ipd_dr_admission_note.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/kphis/ipd_med_reconciliation.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/kphis/ipd_order_item.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/kphis/ipd_nurse_index_plan.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/kphis/ipd_nurse_index_action.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/kphis/kphis_drug_use_duration.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/kphis/opd_er_order_master.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/kphis/opd_er_allergy_history.sql")).execute(&tester.db_pool).await.unwrap();
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

        // not discharged
        let default = get_ipd_pharmacist_search_patient(IpdSearchPatientPharmacistRequest::default(), 7, 9, &tester.db_pool, &tester.hosxp, &tester.kphis, &tester.kphis_extra).await.unwrap();
        assert_eq!(default.len(), 13);

        // include discharged
        let found_hn = get_ipd_pharmacist_search_patient(IpdSearchPatientPharmacistRequest {patient: Some(String::from("1234")), ..Default::default()}, 7, 9, &tester.db_pool, &tester.hosxp, &tester.kphis, &tester.kphis_extra).await.unwrap();
        assert_eq!(found_hn.len(), 2);
        let found_an = get_ipd_pharmacist_search_patient(IpdSearchPatientPharmacistRequest {patient: Some(String::from("60001234")), ..Default::default()}, 7, 9, &tester.db_pool, &tester.hosxp, &tester.kphis, &tester.kphis_extra).await.unwrap();
        assert_eq!(found_an.len(), 1);
        let found_cid = get_ipd_pharmacist_search_patient(IpdSearchPatientPharmacistRequest {patient: Some(String::from("1111111111111")), ..Default::default()}, 7, 9, &tester.db_pool, &tester.hosxp, &tester.kphis, &tester.kphis_extra).await.unwrap();
        assert_eq!(found_cid.len(), 2);
        let found_fullname = get_ipd_pharmacist_search_patient(IpdSearchPatientPharmacistRequest {patient: Some(String::from("มมุติ")), ..Default::default()}, 7, 9, &tester.db_pool, &tester.hosxp, &tester.kphis, &tester.kphis_extra).await.unwrap();
        assert_eq!(found_fullname.len(), 2);

        // not discharged
        let found_ward = get_ipd_pharmacist_search_patient(IpdSearchPatientPharmacistRequest {ward: Some(String::from("01")), ..Default::default()}, 7, 9, &tester.db_pool, &tester.hosxp, &tester.kphis, &tester.kphis_extra).await.unwrap();
        assert_eq!(found_ward.len(), 1);
        let found_doctor = get_ipd_pharmacist_search_patient(IpdSearchPatientPharmacistRequest {doctor_in_charge: Some(String::from("007")), ..Default::default()}, 7, 9, &tester.db_pool, &tester.hosxp, &tester.kphis, &tester.kphis_extra).await.unwrap();
        assert_eq!(found_doctor.len(), 1);
        let found_drug_allergy = get_ipd_pharmacist_search_patient(IpdSearchPatientPharmacistRequest {drug_allergy_check: Some(String::from("007")), ..Default::default()}, 7, 9, &tester.db_pool, &tester.hosxp, &tester.kphis, &tester.kphis_extra).await.unwrap();
        assert_eq!(found_drug_allergy.len(), 13);
        let found_drug_allergy_no_note = get_ipd_pharmacist_search_patient(IpdSearchPatientPharmacistRequest {drug_allergy_check: Some(String::from("no_admission_note")), ..Default::default()}, 7, 9, &tester.db_pool, &tester.hosxp, &tester.kphis, &tester.kphis_extra).await.unwrap();
        assert_eq!(found_drug_allergy_no_note.len(), 13);
        let found_drug_allergy_waiting = get_ipd_pharmacist_search_patient(IpdSearchPatientPharmacistRequest {drug_allergy_check: Some(String::from("waiting")), ..Default::default()}, 7, 9, &tester.db_pool, &tester.hosxp, &tester.kphis, &tester.kphis_extra).await.unwrap();
        assert!(found_drug_allergy_waiting.is_empty());
        let found_drug_allergy_checked = get_ipd_pharmacist_search_patient(IpdSearchPatientPharmacistRequest {drug_allergy_check: Some(String::from("checked")), ..Default::default()}, 7, 9, &tester.db_pool, &tester.hosxp, &tester.kphis, &tester.kphis_extra).await.unwrap();
        assert!(found_drug_allergy_checked.is_empty());
    }
}
