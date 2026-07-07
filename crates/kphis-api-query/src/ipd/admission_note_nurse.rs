use sqlx::{
    FromRow, MySql, Pool, Row,
    mysql::{MySqlQueryResult, MySqlRow},
};

use kphis_model::ipd::admission_note_nurse::IpdNurseAdmissionNote;
use kphis_sql::{TABLE_CREATE_COLUMNS, TABLE_CREATE_PREPARED, TABLE_UPDATE_SET, ipd::admission_note_nurse};
use kphis_util::error::{AppError, Source};

use crate::query1_opt;

// ipd-nurse-admission-note-edit.php
pub async fn get_ipd_admission_note_nurse(an: &str, pool: &Pool<MySql>, hosxp: &str, kphis: &str) -> Result<Option<IpdNurseAdmissionNote>, AppError> {
    let admission_note = select_ipd_admission_note_nurse(an, pool, hosxp, kphis)
        .await?
        .or(select_cc_hpi_from_an(an, pool, hosxp).await?.map(|(chief_complaints, medical_history)| IpdNurseAdmissionNote {
            an: an.to_owned(),
            chief_complaints,
            medical_history,
            info_patient: Some(String::from("Y")),
            concious: Some(String::from("รู้สึกตัวดี")),
            normal_breath: Some(String::from("Y")),
            normal_hr: Some(String::from("Y")),
            normal_cir: Some(String::from("Y")),
            normal_skin: Some(String::from("Y")),
            pain: Some(String::from("ไม่มี")),
            normal_behav: Some(String::from("Y")),
            normal_emotional: Some(String::from("Y")),
            no_anxiety: Some(String::from("Y")),
            spiritual_no: Some(String::from("Y")),
            education: Some(String::from("ไม่ได้รับ")),
            income: Some(String::from("เพียงพอ")),
            self_value: Some(String::from("Y")),
            clinic: Some(String::from("Y")),
            no_risk: Some(String::from("Y")),
            diet_regular: Some(String::from("อาหารทั่วไป")),
            nutrition_risk: Some(String::from("Y")),
            normal_urine: Some(String::from("Y")),
            normal_feces: Some(String::from("Y")),
            activity1: Some(String::from("Y")),
            sleep_med_name: Some(String::from("ไม่เคย")),
            cognitive: Some(String::from("ตรง")),
            memory: Some(String::from("ปกติ")),
            hearing: Some(String::from("ปกติ")),
            vision: Some(String::from("ปกติ")),
            speech: Some(String::from("ปกติ")),
            self_image: Some(String::from("ไม่มี")),
            self_activity: Some(String::from("ไม่มี")),
            sickness_effect: Some(String::from("ไม่มี")),
            period: Some(String::from("ยังไม่มี")),
            breast: Some(String::from("ปกติ")),
            consult: Some(String::from("Y")),
            belief_sickness_behave: Some(String::from("Y")),
            belief_believe: Some(String::from("ไม่มี")),
            religious_activity: Some(String::from("ไม่ต้องการ")),
            ..Default::default()
        }));

    Ok(admission_note)
}

async fn select_ipd_admission_note_nurse(an: &str, pool: &Pool<MySql>, hosxp: &str, kphis: &str) -> Result<Option<IpdNurseAdmissionNote>, AppError> {
    let sql = admission_note_nurse::select_admission_note_from_an(hosxp, kphis);
    let admission_note = query1_opt(an, &sql, pool, "Select IpdNurseAdmissionNote")
        .await?
        .as_ref()
        .map(IpdNurseAdmissionNote::from_row)
        .transpose()
        .map_err(|e| Source::SQLx.to_error(500, e, "Select IpdNurseAdmissionNote"))?;

    Ok(admission_note)
}

async fn select_cc_hpi_from_an(an: &str, pool: &Pool<MySql>, hosxp: &str) -> Result<Option<(Option<String>, Option<String>)>, AppError> {
    let sql = admission_note_nurse::select_cc_hpi_from_an(hosxp);
    let cc_hpi = query1_opt(an, &sql, pool, "Select CcHpi")
        .await?
        .as_ref()
        .map(cc_hpi_from_row)
        .transpose()
        .map_err(|e| Source::SQLx.to_error(500, e, "Select CcHpi"))?;

    Ok(cc_hpi)
}
fn cc_hpi_from_row(row: &MySqlRow) -> sqlx::Result<(Option<String>, Option<String>)> {
    let cc: Option<String> = row.try_get("cc")?;
    let hpi: Option<String> = row.try_get("hpi")?;
    Ok((cc, hpi))
}

// ipd-nurse-admission-note-save.php
pub async fn post_ipd_admission_note_nurse(form: &IpdNurseAdmissionNote, user: &str, pool: &Pool<MySql>, kphis: &str) -> Result<MySqlQueryResult, AppError> {
    form.insert(Some("nurse_admission_note_id"), None, TABLE_CREATE_COLUMNS, TABLE_CREATE_PREPARED, &[user, user], pool, kphis)
        .await
        .map_err(|e| {
            if let sqlx::error::Error::Database(err) = &e
                && err.code().map(|c| c == "23000").unwrap_or_default()
            {
                AppError::app_403_duplicate("Insert IpdNurseAdmissionNote")
            } else {
                Source::SQLx.to_error(500, e, "Insert IpdNurseAdmissionNote")
            }
        })
}

// ipd-nurse-admission-note-update.php
pub async fn put_ipd_admission_note_nurse(form: &IpdNurseAdmissionNote, user: &str, pool: &Pool<MySql>, kphis: &str) -> Result<MySqlQueryResult, AppError> {
    form.update("nurse_admission_note_id", None, TABLE_UPDATE_SET, &[user], pool, kphis)
        .await
        .map_err(|e| Source::SQLx.to_error(500, e, "Update IpdNurseAdmissionNote"))
}

#[cfg(test)]
#[rustfmt::skip]
mod tests {

    use super::*;
    use kphis_sqlx_tester::MySqlTester;

    #[tokio::test]
    #[ignore]
    async fn sqlx_select_ipd_admission_note_nurse() {
        let tester = MySqlTester::new_hosxp_and_kphis().await;
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis/ipd_nurse_admission_note.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis/ipd_dr_admission_note.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/hosxp/opduser.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/hosxp/doctor.sql")).execute(&tester.db_pool).await.unwrap();

        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/kphis/ipd_nurse_admission_note.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/kphis/ipd_dr_admission_note.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/hosxp/opduser.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/hosxp/doctor.sql")).execute(&tester.db_pool).await.unwrap();

        let found = select_ipd_admission_note_nurse("660001234",&tester.db_pool,&tester.hosxp,&tester.kphis).await.unwrap();
        assert!(found.is_some());
        let not_found = select_ipd_admission_note_nurse("660006666",&tester.db_pool,&tester.hosxp,&tester.kphis).await.unwrap();
        assert!(not_found.is_none());
    }

    #[tokio::test]
    #[ignore]
    async fn sqlx_select_cc_hpi_from_an() {
        let tester = MySqlTester::new_hosxp().await;
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/hosxp/opdscreen.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/hosxp/ipt.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/hosxp/opdscreen.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/hosxp/ipt.sql")).execute(&tester.db_pool).await.unwrap();

        let found = select_cc_hpi_from_an("660001234", &tester.db_pool, &tester.hosxp).await.unwrap();
        assert!(found.is_some());
        let not_found = select_cc_hpi_from_an("660006666", &tester.db_pool, &tester.hosxp).await.unwrap();
        assert!(not_found.is_none());
    }

    #[tokio::test]
    #[ignore]
    async fn sqlx_post_ipd_admission_note_nurse() {
        let tester = MySqlTester::new_kphis().await;
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis/ipd_nurse_admission_note.sql")).execute(&tester.db_pool).await.unwrap();

        let success = post_ipd_admission_note_nurse(&IpdNurseAdmissionNote::demo(),"user",&tester.db_pool,&tester.kphis).await.unwrap();
        assert_eq!(success.rows_affected(), 1);
        let again_duplicate_an = post_ipd_admission_note_nurse(&IpdNurseAdmissionNote::demo(),"user",&tester.db_pool,&tester.kphis).await;
        if let Err(again_duplicate_an_error) = again_duplicate_an {
            assert_eq!(again_duplicate_an_error.status, 403);
        } else {
            panic!("MUST ERROR")
        }
    }

    #[tokio::test]
    #[ignore]
    async fn sqlx_put_ipd_admission_note_nurse() {
        let tester = MySqlTester::new_kphis().await;
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis/ipd_nurse_admission_note.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/kphis/ipd_nurse_admission_note.sql")).execute(&tester.db_pool).await.unwrap();

        let success = put_ipd_admission_note_nurse(&IpdNurseAdmissionNote::demo(),"user",&tester.db_pool,&tester.kphis).await.unwrap();
        assert_eq!(success.rows_affected(), 1);
        let again_success = put_ipd_admission_note_nurse(&IpdNurseAdmissionNote::demo(),"user",&tester.db_pool,&tester.kphis).await.unwrap();
        assert_eq!(again_success.rows_affected(), 1);
    }
}
