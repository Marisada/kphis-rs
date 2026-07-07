use sqlx::{AssertSqlSafe, FromRow, MySql, Pool, Row};

use kphis_model::post_admit::{PostAdmitList, PostAdmitParams};
use kphis_sql::post_admit;
use kphis_util::error::{AppError, Source};

pub async fn get_post_admit_list(params: &PostAdmitParams, hlen: usize, alen: usize, pool: &Pool<MySql>, hosxp: &str, kphis: &str, kphis_extra: &str) -> Result<Vec<PostAdmitList>, AppError> {
    let sql = post_admit::select_post_admit_list(params, hlen, alen, hosxp, kphis, kphis_extra);

    let mut query = sqlx::query(AssertSqlSafe(sql));
    if let Some(passcode) = &params.passcode {
        query = query.bind(passcode);
    }
    if let Some(patient) = params.patient.as_ref().and_then(|s| urlencoding::decode(s).ok()) {
        let patient_wildcard = ["%", &patient, "%"].concat();
        if patient.len() == 13 {
            query = query.bind(patient)
        } else if hlen == alen {
            query = query.bind(patient_wildcard.clone()).bind(patient_wildcard);
        } else {
            query = query.bind(patient_wildcard)
        }
    } else {
        if let Some(ward) = &params.ward {
            query = query.bind(ward);
        }
        if let Some(inscl) = &params.inscl {
            query = query.bind(inscl);
        }
        if let Some(adm_doctor) = &params.adm_doctor {
            query = query.bind(adm_doctor);
        }
        if let Some(dch_doctor) = &params.dch_doctor {
            query = query.bind(dch_doctor);
        }
        if let Some(start_dchdate) = &params.start_dchdate {
            query = query.bind(start_dchdate);
        }
        if let Some(end_dchdate) = &params.end_dchdate {
            query = query.bind(end_dchdate);
        }
    }

    query
        .fetch_all(pool)
        .await
        .map(|rows| {
            rows.iter()
                .map(PostAdmitList::from_row)
                .collect::<sqlx::Result<Vec<PostAdmitList>>>()
                .map_err(|e| Source::SQLx.to_error(500, e, "Select PostAdmitList"))
        })
        .map_err(|e| Source::SQLx.to_error(500, e, "Select PostAdmitList"))?
}

pub async fn get_post_admit_count(doctorcode: &str, pool: &Pool<MySql>, hosxp: &str, kphis: &str) -> Result<i64, AppError> {
    let sql = post_admit::select_post_admit_count(hosxp, kphis);

    sqlx::query(AssertSqlSafe(sql))
        .bind(doctorcode)
        .fetch_one(pool)
        .await
        .map(|row| row.try_get(0))
        .map_err(|e| Source::SQLx.to_error(500, e, "Select PostAdmitCount"))?
        .map_err(|e| Source::SQLx.to_error(500, e, "Select PostAdmitCount"))
}

#[cfg(test)]
#[rustfmt::skip]
mod tests {

    use super::*;
    use kphis_sqlx_tester::MySqlTester;
    use time::macros::date;

    #[tokio::test]
    #[ignore]
    async fn sqlx_get_post_admit_list() {
        let tester = MySqlTester::new_hosxp_and_kphis_and_kphis_extra().await;

        sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/create/kphis/ipd_order.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/create/kphis/ipd_progress_note.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/create/kphis/ipd_dr_admission_note.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/create/kphis/ipd_dr_admission_note_item.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/create/kphis/ipd_summary_2.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/create/kphis/ipd_summary_attending_doctor.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/create/kphis/ipd_summary_approve_doctor.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/create/kphis/ipd_ward_passcode.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/create/hosxp/ipt.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/create/hosxp/an_stat.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/create/hosxp/patient.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/create/hosxp/ward.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/create/hosxp/doctor.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/create/hosxp/sex.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/create/hosxp/pttype.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/create/hosxp/dchtype.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/create/hosxp/dchstts.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/create/kphis_extra/ipd_mra.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/create/kphis_extra/ipd_summary_audit.sql")).execute(&tester.db_pool).await.unwrap();

        sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/insert/kphis/ipd_order.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/insert/kphis/ipd_progress_note.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/insert/kphis/ipd_dr_admission_note.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/insert/kphis/ipd_dr_admission_note_item.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/insert/kphis/ipd_summary_2.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/insert/kphis/ipd_summary_attending_doctor.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/insert/kphis/ipd_summary_approve_doctor.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/insert/hosxp/ipt.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/insert/hosxp/an_stat.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/insert/hosxp/patient.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/insert/hosxp/ward.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/insert/hosxp/doctor.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/insert/hosxp/sex.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/insert/hosxp/pttype.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/insert/hosxp/dchtype.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/insert/hosxp/dchstts.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/insert/kphis_extra/ipd_mra.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/insert/kphis_extra/ipd_summary_audit.sql")).execute(&tester.db_pool).await.unwrap();

        // one of two still admited
        let default = get_post_admit_list(&PostAdmitParams::default(),7,9,&tester.db_pool,&tester.hosxp,&tester.kphis,&tester.kphis_extra).await.unwrap();
        assert_eq!(default.len(),1);

        let found_patient_hn = get_post_admit_list(&PostAdmitParams {patient: Some(String::from("1234")),..Default::default()},7,9,&tester.db_pool,&tester.hosxp,&tester.kphis,&tester.kphis_extra).await.unwrap();
        assert_eq!(found_patient_hn.len(), 1);
        let found_patient_an = get_post_admit_list(&PostAdmitParams {patient: Some(String::from("60001234")),..Default::default()},7,9,&tester.db_pool,&tester.hosxp,&tester.kphis,&tester.kphis_extra).await.unwrap();
        assert_eq!(found_patient_an.len(), 1);
        let found_patient_cid = get_post_admit_list(&PostAdmitParams {patient: Some(String::from("1111111111111")),..Default::default()},7,9,&tester.db_pool,&tester.hosxp,&tester.kphis,&tester.kphis_extra).await.unwrap();
        assert_eq!(found_patient_cid.len(), 1);
        let found_patient_fullname = get_post_admit_list(&PostAdmitParams {patient: Some(String::from("สมมุ")),..Default::default()},7,9,&tester.db_pool,&tester.hosxp,&tester.kphis,&tester.kphis_extra).await.unwrap();
        assert_eq!(found_patient_fullname.len(), 1);
        let not_found_patient = get_post_admit_list(&PostAdmitParams {patient: Some(String::from("6666")),..Default::default()},7,9,&tester.db_pool,&tester.hosxp,&tester.kphis,&tester.kphis_extra).await.unwrap();
        assert!(not_found_patient.is_empty());

        let found_ward = get_post_admit_list(&PostAdmitParams {ward: Some(String::from("01")),..Default::default()},7,9,&tester.db_pool,&tester.hosxp,&tester.kphis,&tester.kphis_extra).await.unwrap();
        assert_eq!(found_ward.len(), 1);
        let not_found_ward = get_post_admit_list(&PostAdmitParams {ward: Some(String::from("09")),..Default::default()},7,9,&tester.db_pool,&tester.hosxp,&tester.kphis,&tester.kphis_extra).await.unwrap();
        assert!(not_found_ward.is_empty());

        let found_inscl = get_post_admit_list(&PostAdmitParams {inscl: Some(String::from("UCS")),..Default::default()},7,9,&tester.db_pool,&tester.hosxp,&tester.kphis,&tester.kphis_extra).await.unwrap();
        assert_eq!(found_inscl.len(), 1);
        let not_found_inscl = get_post_admit_list(&PostAdmitParams {inscl: Some(String::from("SSS")),..Default::default()},7,9,&tester.db_pool,&tester.hosxp,&tester.kphis,&tester.kphis_extra).await.unwrap();
        assert!(not_found_inscl.is_empty());

        let found_adm_doctor = get_post_admit_list(&PostAdmitParams {adm_doctor: Some(String::from("007")),..Default::default()},7,9,&tester.db_pool,&tester.hosxp,&tester.kphis,&tester.kphis_extra).await.unwrap();
        assert_eq!(found_adm_doctor.len(), 1);
        let not_found_adm_doctor = get_post_admit_list(&PostAdmitParams {adm_doctor: Some(String::from("999")),..Default::default()},7,9,&tester.db_pool,&tester.hosxp,&tester.kphis,&tester.kphis_extra).await.unwrap();
        assert!(not_found_adm_doctor.is_empty());

        let found_dch_doctor = get_post_admit_list(&PostAdmitParams {dch_doctor: Some(String::from("008")),..Default::default()},7,9,&tester.db_pool,&tester.hosxp,&tester.kphis,&tester.kphis_extra).await.unwrap();
        assert_eq!(found_dch_doctor.len(), 1);
        let not_found_dch_doctor = get_post_admit_list(&PostAdmitParams {adm_doctor: Some(String::from("999")),..Default::default()},7,9,&tester.db_pool,&tester.hosxp,&tester.kphis,&tester.kphis_extra).await.unwrap();
        assert!(not_found_dch_doctor.is_empty());

        let found_before_dchdate = get_post_admit_list(&PostAdmitParams {end_dchdate: Some(date!(2024-01-01)),..Default::default()},7,9,&tester.db_pool,&tester.hosxp,&tester.kphis,&tester.kphis_extra).await.unwrap();
        assert_eq!(found_before_dchdate.len(), 1);
        let not_found_before_dchdate = get_post_admit_list(&PostAdmitParams {end_dchdate: Some(date!(2023-12-31)),..Default::default()},7,9,&tester.db_pool,&tester.hosxp,&tester.kphis,&tester.kphis_extra).await.unwrap();
        assert!(not_found_before_dchdate.is_empty());

        let found_after_dchdate = get_post_admit_list(&PostAdmitParams {start_dchdate: Some(date!(2024-01-01)),..Default::default()},7,9,&tester.db_pool,&tester.hosxp,&tester.kphis,&tester.kphis_extra).await.unwrap();
        assert_eq!(found_after_dchdate.len(), 1);
        let not_found_after_dchdate = get_post_admit_list(&PostAdmitParams {start_dchdate: Some(date!(2024-01-02)),..Default::default()},7,9,&tester.db_pool,&tester.hosxp,&tester.kphis,&tester.kphis_extra).await.unwrap();
        assert!(not_found_after_dchdate.is_empty());

        let found_between_dchdate = get_post_admit_list(&PostAdmitParams {start_dchdate: Some(date!(2024-01-01)),end_dchdate: Some(date!(2024-01-01)),..Default::default()},7,9,&tester.db_pool,&tester.hosxp,&tester.kphis,&tester.kphis_extra).await.unwrap();
        assert_eq!(found_between_dchdate.len(), 1);

        let found_status_null = get_post_admit_list(&PostAdmitParams {summary_status: Some(String::from("null")),..Default::default()},7,9,&tester.db_pool,&tester.hosxp,&tester.kphis,&tester.kphis_extra).await.unwrap();
        assert!(found_status_null.is_empty());
        let found_status_approve = get_post_admit_list(&PostAdmitParams {summary_status: Some(String::from("approve")),..Default::default()},7,9,&tester.db_pool,&tester.hosxp,&tester.kphis,&tester.kphis_extra).await.unwrap();
        assert!(found_status_approve.is_empty());
        let found_status_review = get_post_admit_list(&PostAdmitParams {summary_status: Some(String::from("review")),..Default::default()},7,9,&tester.db_pool,&tester.hosxp,&tester.kphis,&tester.kphis_extra).await.unwrap();
        assert_eq!(found_status_review.len(), 1);
        let found_status_code = get_post_admit_list(&PostAdmitParams {summary_status: Some(String::from("code")),..Default::default()},7,9,&tester.db_pool,&tester.hosxp,&tester.kphis,&tester.kphis_extra).await.unwrap();
        assert!(found_status_code.is_empty());
        let found_status_audit = get_post_admit_list(&PostAdmitParams {summary_status: Some(String::from("audit")),..Default::default()},7,9,&tester.db_pool,&tester.hosxp,&tester.kphis,&tester.kphis_extra).await.unwrap();
        assert!(found_status_audit.is_empty());
        let found_status_claim = get_post_admit_list(&PostAdmitParams {summary_status: Some(String::from("claim")),..Default::default()},7,9,&tester.db_pool,&tester.hosxp,&tester.kphis,&tester.kphis_extra).await.unwrap();
        assert!(found_status_claim.is_empty());
        let found_status_appeal = get_post_admit_list(&PostAdmitParams {summary_status: Some(String::from("appeal")),..Default::default()},7,9,&tester.db_pool,&tester.hosxp,&tester.kphis,&tester.kphis_extra).await.unwrap();
        assert!(found_status_appeal.is_empty());
        let found_status_done = get_post_admit_list(&PostAdmitParams {summary_status: Some(String::from("done")),..Default::default()},7,9,&tester.db_pool,&tester.hosxp,&tester.kphis,&tester.kphis_extra).await.unwrap();
        assert!(found_status_done.is_empty());

        let no_passcode = get_post_admit_list(&PostAdmitParams {passcode: Some(String::from("1234")), ..Default::default()}, 7, 9, &tester.db_pool, &tester.hosxp, &tester.kphis,&tester.kphis_extra).await.unwrap();
        assert_eq!(no_passcode.len(), 1);
        // with passcode
        sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/insert/kphis/ipd_ward_passcode.sql")).execute(&tester.db_pool).await.unwrap();
        let true_passcode = get_post_admit_list(&PostAdmitParams {passcode: Some(String::from("1234")), ..Default::default()}, 7, 9, &tester.db_pool, &tester.hosxp, &tester.kphis,&tester.kphis_extra).await.unwrap();
        assert_eq!(true_passcode.len(), 1);
        let false_passcode = get_post_admit_list(&PostAdmitParams {passcode: Some(String::from("6666")), ..Default::default()}, 7, 9, &tester.db_pool, &tester.hosxp, &tester.kphis,&tester.kphis_extra).await.unwrap();
        assert!(false_passcode.is_empty());
    }

    #[tokio::test]
    #[ignore]
    async fn sqlx_get_post_admit_count() {
        let tester = MySqlTester::new_hosxp_and_kphis().await;
        sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/create/kphis/ipd_order.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/create/kphis/ipd_summary_2.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/create/kphis/ipd_summary_attending_doctor.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/create/hosxp/ipt.sql")).execute(&tester.db_pool).await.unwrap();

        sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/insert/kphis/ipd_order.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/insert/kphis/ipd_summary_2.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/insert/kphis/ipd_summary_attending_doctor.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/insert/hosxp/ipt.sql")).execute(&tester.db_pool).await.unwrap();

        let found = get_post_admit_count("008", &tester.db_pool, &tester.hosxp, &tester.kphis).await.unwrap();
        assert_eq!(found, 1);
        let not_found = get_post_admit_count("007", &tester.db_pool, &tester.hosxp, &tester.kphis).await.unwrap();
        assert_eq!(not_found, 0);
    }
}
