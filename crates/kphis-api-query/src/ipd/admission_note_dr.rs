use sqlx::{AssertSqlSafe, FromRow, MySql, Pool, Row, mysql::MySqlQueryResult};

use kphis_model::ipd::admission_note_dr::{AdmissionNoteDoctor, IpdAdmissionNoteDrRaw, IpdDrAdmissionNote, Ipt, OpdErAllergyHistory, OpdscreenPe, Period, Vs};
use kphis_sql::{TABLE_CREATE_COLUMNS, TABLE_CREATE_PREPARED, TABLE_UPDATE_SET, ipd::admission_note_dr, query_utils};
use kphis_util::error::{AppError, Source};
use time::PrimitiveDateTime;

use crate::{query1_all, query1_opt};

// ipd-dr-admission-note-form.php
pub async fn get_ipd_admission_note_dr_from_an(an: &str, pool: &Pool<MySql>, hosxp: &str, kphis: &str, kphis_extra: &str) -> Result<IpdAdmissionNoteDrRaw, AppError> {
    // 1. get ipt -> we get hn, vn, regdatetime
    let ipt = select_ipt_from_an(an, pool, hosxp).await?;
    let hn_opt = ipt.as_ref().and_then(|i| i.hn.clone());
    let vn_opt = ipt.as_ref().and_then(|i| i.vn.clone());
    let regdatetime_opt = ipt.as_ref().and_then(|i| i.regdatetime.clone());

    // 2. get admission_note_dr_items, old_regdatetime, operation_list, vs, period
    let admission_note_doctors = select_admission_note_dr_items_from_an(an, pool, hosxp, kphis).await?;
    let vs = select_vs_from_an(an, pool, kphis).await?;
    let period = select_period_from_an(an, pool, kphis).await?;

    let operation_list = match (hn_opt.as_ref(), regdatetime_opt) {
        (Some(hn), Some(regdatetime)) => select_operation_list_from_hn(hn, &regdatetime, pool, hosxp).await?,
        _ => Vec::new(),
    };

    let old_regdatetime = match hn_opt.as_ref() {
        Some(hn) => select_old_regdatetime_from_hn_an(hn, an, pool, hosxp).await?,
        None => None,
    };

    // 3. get admission_note
    let admission_note = select_admission_note_from_an(an, pool, kphis, kphis_extra).await?;
    let admission_note_id = admission_note.as_ref().map(|note| note.admission_note_id);

    // 4. get opdscreen_pe
    let opdscreen_pe = if let Some(vn) = &vn_opt { select_opdscreen_pe_from_vn(vn, pool, hosxp).await? } else { None };

    // 5. get opd_er_allergy_history_list **if admission_note_id.is_none()
    let opd_er_allergy_histories = if admission_note_id.is_none()
        && let Some(vn) = &vn_opt
    {
        get_opd_er_allergy_list_by_vn(vn, pool, kphis).await?
    } else {
        Vec::new()
    };

    // 6. get get_opd_er_allergy_with_symptom
    let opd_er_allergy_history = if let Some(vn) = &vn_opt {
        get_opd_er_allergy_with_symptom_by_vn(vn, pool, kphis).await?
    } else {
        None
    };

    // 7. get doctor_in_charge
    let doctor_in_charge = select_ipd_doctor_in_charge(an, pool, hosxp, kphis).await?;

    Ok(IpdAdmissionNoteDrRaw {
        admission_note,
        admission_note_doctors,
        opdscreen_pe,
        // ipt,
        vs,
        period,
        operation_list,
        old_regdatetime,
        opd_er_allergy_histories,
        opd_er_allergy_history,
        doctor_in_charge,
    })
}

// ipd-dr-admission-note-form.php
pub async fn get_ipd_admission_note_dr_from_vn(vn: &str, pool: &Pool<MySql>, hosxp: &str, kphis: &str, kphis_extra: &str) -> Result<IpdAdmissionNoteDrRaw, AppError> {
    // 1. get opdscreen_pe
    let opdscreen_pe = select_opdscreen_pe_from_vn(vn, pool, hosxp).await?;

    let hn_opt = opdscreen_pe.as_ref().and_then(|ops| ops.hn.clone());
    let vstdatetime_opt = opdscreen_pe.as_ref().and_then(|ops| ops.vstdatetime.clone());

    // 2. get admission_note_dr_items, old_regdatetime, operation_list, vs, period
    // `AN` use for KPHIS here
    // only use hos.doctor here
    let admission_note_doctors = select_admission_note_dr_items_from_an(vn, pool, hosxp, kphis).await?;
    let vs = select_vs_from_an(vn, pool, kphis).await?;
    let period = select_period_from_an(vn, pool, kphis).await?;

    let operation_list = match (hn_opt.as_ref(), vstdatetime_opt) {
        (Some(hn), Some(vstdatetime)) => select_operation_list_from_hn(hn, &vstdatetime, pool, hosxp).await?,
        _ => Vec::new(),
    };

    let old_regdatetime = match hn_opt.as_ref() {
        Some(hn) => select_old_regdatetime_from_hn_an(hn, "", pool, hosxp).await?,
        None => None,
    };

    // 3. get admission_note
    // `AN` use for KPHIS here
    let admission_note = select_admission_note_from_an(vn, pool, kphis, kphis_extra).await?;
    let admission_note_id = admission_note.as_ref().map(|note| note.admission_note_id);

    // 5. get opd_er_allergy_history_list **if admission_note_id.is_none()
    let opd_er_allergy_histories = if admission_note_id.is_none() {
        get_opd_er_allergy_list_by_vn(vn, pool, kphis).await?
    } else {
        Vec::new()
    };

    // 6. get get_opd_er_allergy_with_symptom
    let opd_er_allergy_history = get_opd_er_allergy_with_symptom_by_vn(vn, pool, kphis).await?;

    // 7. get doctor_in_charge
    // only use hos.doctor here
    let doctor_in_charge = select_ipd_doctor_in_charge(vn, pool, hosxp, kphis).await?;

    Ok(IpdAdmissionNoteDrRaw {
        admission_note,
        admission_note_doctors,
        opdscreen_pe,
        // ipt: None,
        vs,
        period,
        operation_list,
        old_regdatetime,
        opd_er_allergy_histories,
        opd_er_allergy_history,
        doctor_in_charge,
    })
}

async fn select_ipt_from_an(an: &str, pool: &Pool<MySql>, hosxp: &str) -> Result<Option<Ipt>, AppError> {
    let sql = admission_note_dr::select_ipt_from_an(hosxp);
    query1_opt(an, &sql, pool, "Select Ipt")
        .await?
        .as_ref()
        .map(Ipt::from_row)
        .transpose()
        .map_err(|e| Source::SQLx.to_error(500, e, "Select Ipt"))
}

async fn select_admission_note_dr_items_from_an(an: &str, pool: &Pool<MySql>, hosxp: &str, kphis: &str) -> Result<Vec<AdmissionNoteDoctor>, AppError> {
    let sql = admission_note_dr::select_admission_note_dr_items_from_an(hosxp, kphis);
    query1_all(an, &sql, pool, "Select DrItem")
        .await?
        .iter()
        .map(AdmissionNoteDoctor::from_row)
        .collect::<sqlx::Result<Vec<AdmissionNoteDoctor>>>()
        .map_err(|e| Source::SQLx.to_error(500, e, "Select DrItem"))
}

async fn select_vs_from_an(an: &str, pool: &Pool<MySql>, kphis: &str) -> Result<Option<Vs>, AppError> {
    let sql = admission_note_dr::select_vs_from_an(kphis);
    query1_opt(an, &sql, pool, "Select Vs")
        .await?
        .as_ref()
        .map(Vs::from_row)
        .transpose()
        .map_err(|e| Source::SQLx.to_error(500, e, "Select Vs"))
}

async fn select_period_from_an(an: &str, pool: &Pool<MySql>, kphis: &str) -> Result<Option<Period>, AppError> {
    let sql = admission_note_dr::select_period_from_an(kphis);
    query1_opt(an, &sql, pool, "Select Period")
        .await?
        .as_ref()
        .map(Period::from_row)
        .transpose()
        .map_err(|e| Source::SQLx.to_error(500, e, "Select Period"))
}

async fn select_operation_list_from_hn(hn: &str, regdatetime: &PrimitiveDateTime, pool: &Pool<MySql>, hosxp: &str) -> Result<Vec<String>, AppError> {
    let sql = admission_note_dr::select_operation_list_from_hn(&regdatetime.to_string(), hosxp);
    query1_all(hn, &sql, pool, "Select OperationList")
        .await?
        .iter()
        .filter_map(|row| row.try_get("operation_list").transpose())
        .collect::<sqlx::Result<Vec<String>>>()
        .map_err(|e| Source::SQLx.to_error(500, e, "Select OperationList"))
}

async fn select_old_regdatetime_from_hn_an(hn: &str, an: &str, pool: &Pool<MySql>, hosxp: &str) -> Result<Option<PrimitiveDateTime>, AppError> {
    let sql = admission_note_dr::select_old_regdatetime_from_hn(!an.is_empty(), hosxp);
    let mut query = sqlx::query(AssertSqlSafe(sql)).bind(hn);
    if !an.is_empty() {
        query = query.bind(an);
    }
    query
        .fetch_optional(pool)
        .await
        .map_err(|e| Source::SQLx.to_error(500, e, "Select OldRegDateTime"))?
        .map(|row| row.try_get("old_regdatetime"))
        .transpose()
        .map(|opt| opt.flatten())
        .map_err(|e| Source::SQLx.to_error(500, e, "Select OldRegDateTime"))
}

async fn select_admission_note_from_an(an: &str, pool: &Pool<MySql>, kphis: &str, kphis_extra: &str) -> Result<Option<IpdDrAdmissionNote>, AppError> {
    let sql = admission_note_dr::select_admission_note_from_an(kphis, kphis_extra);
    query1_opt(an, &sql, pool, "Select IpdDrAdmissionNote")
        .await?
        .as_ref()
        .map(IpdDrAdmissionNote::from_row)
        .transpose()
        .map_err(|e| Source::SQLx.to_error(500, e, "Select IpdDrAdmissionNote"))
}

async fn select_opdscreen_pe_from_vn(vn: &str, pool: &Pool<MySql>, hosxp: &str) -> Result<Option<OpdscreenPe>, AppError> {
    let sql = admission_note_dr::select_opdscreen_pe_from_vn(hosxp);
    query1_opt(vn, &sql, pool, "Select OpdScreen")
        .await?
        .as_ref()
        .map(OpdscreenPe::from_row)
        .transpose()
        .map_err(|e| Source::SQLx.to_error(500, e, "Select OpdScreen"))
}

async fn get_opd_er_allergy_list_by_vn(vn: &str, pool: &Pool<MySql>, kphis: &str) -> Result<Vec<OpdErAllergyHistory>, AppError> {
    let sql = query_utils::get_opd_er_allergy_list_by_vn(kphis);
    query1_all(vn, &sql, pool, "Select OpdErAllergyHistory")
        .await?
        .iter()
        .map(OpdErAllergyHistory::from_row)
        .collect::<sqlx::Result<Vec<OpdErAllergyHistory>>>()
        .map_err(|e| Source::SQLx.to_error(500, e, "Select OpdErAllergyHistory"))
}

async fn get_opd_er_allergy_with_symptom_by_vn(vn: &str, pool: &Pool<MySql>, kphis: &str) -> Result<Option<String>, AppError> {
    let sql = query_utils::get_opd_er_allergy_with_symptom_by_vn(true, kphis);
    query1_opt(vn, &sql, pool, "Select DrugAllergy")
        .await?
        .map(|row| row.try_get("drugallergy"))
        .transpose()
        .map(|opt| opt.flatten())
        .map_err(|e| Source::SQLx.to_error(500, e, "Select DrugAllergy"))
}

async fn select_ipd_doctor_in_charge(an: &str, pool: &Pool<MySql>, hosxp: &str, kphis: &str) -> Result<Vec<String>, AppError> {
    let sql = admission_note_dr::select_ipd_doctor_in_charge(hosxp, kphis);
    query1_all(an, &sql, pool, "Select DoctorInCharge")
        .await?
        .iter()
        .filter_map(|row| row.try_get("name").transpose())
        .collect::<sqlx::Result<Vec<String>>>()
        .map_err(|e| Source::SQLx.to_error(500, e, "Select DoctorInCharge"))
}

// ipd-dr-admission-note-save.php
pub async fn post_ipd_admission_note_dr(form: &IpdDrAdmissionNote, user: &str, pool: &Pool<MySql>, kphis: &str) -> Result<MySqlQueryResult, AppError> {
    form.insert(Some("admission_note_id"), None, TABLE_CREATE_COLUMNS, TABLE_CREATE_PREPARED, &[user, user], pool, kphis)
        .await
        .map_err(|e| {
            if let sqlx::error::Error::Database(err) = &e
                && err.code().map(|c| c == "23000").unwrap_or_default()
            {
                AppError::app_403_duplicate("Insert IpdDrAdmissionNote")
            } else {
                Source::SQLx.to_error(500, e, "Insert IpdDrAdmissionNote")
            }
        })
}

// ipd-dr-admission-note-update.php
pub async fn put_ipd_admission_note_dr(form: &IpdDrAdmissionNote, user: &str, pool: &Pool<MySql>, kphis: &str) -> Result<MySqlQueryResult, AppError> {
    form.update("admission_note_id", None, TABLE_UPDATE_SET, &[user], pool, kphis)
        .await
        .map_err(|e| Source::SQLx.to_error(500, e, "Update IpdDrAdmissionNote"))
}

// ipd-dr-admission-note-save.php
pub async fn insert_ipd_admission_note_dr_items(items: &[String], admission_note_id: u32, an: &str, user: &str, pool: &Pool<MySql>, kphis: &str) -> Result<MySqlQueryResult, AppError> {
    let sql = admission_note_dr::insert_ipd_dr_admission_note_items(items.len(), kphis);
    let mut query = sqlx::query(AssertSqlSafe(sql));
    for item in items {
        query = query.bind(admission_note_id).bind(an).bind(item).bind(user).bind(user);
    }
    query.execute(pool).await.map_err(|e| Source::SQLx.to_error(500, e, "Insert IpdDrAdmissionNoteItem"))
}

pub async fn delete_ipd_admission_note_dr_items(an: &str, pool: &Pool<MySql>, kphis: &str) -> Result<MySqlQueryResult, AppError> {
    let sql = admission_note_dr::delete_ipd_dr_admission_note_item(kphis);
    sqlx::query(AssertSqlSafe(sql))
        .bind(an)
        .execute(pool)
        .await
        .map_err(|e| Source::SQLx.to_error(500, e, "Delete IpdDrAdmissionNoteItem"))
}

// ipd-dr-admission-note-pharmacy-check-save.php
pub async fn patch_pharamacy_check(an: &str, doctor_code: &Option<String>, loginname: &str, pool: &Pool<MySql>, kphis: &str) -> Result<MySqlQueryResult, AppError> {
    let sql = admission_note_dr::update_ipd_dr_pharmacy_check(kphis);
    sqlx::query(AssertSqlSafe(sql))
        .bind(doctor_code)
        .bind(loginname)
        .bind(an)
        .execute(pool)
        .await
        .map_err(|e| Source::SQLx.to_error(500, e, "Update PharmacyCheck"))
}

#[cfg(test)]
#[rustfmt::skip]
mod tests {

    use time::macros::datetime;

    use super::*;
    use kphis_sqlx_tester::MySqlTester;

    #[tokio::test]
    #[ignore]
    async fn sqlx_select_ipt_from_an() {
        let tester = MySqlTester::new_hosxp_and_kphis().await;
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/hosxp/ipt.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/hosxp/ipt.sql")).execute(&tester.db_pool).await.unwrap();

        let found = select_ipt_from_an("660001234", &tester.db_pool, &tester.hosxp).await.unwrap();
        assert!(found.is_some());
        let not_found =select_ipt_from_an("660006666", &tester.db_pool, &tester.hosxp).await.unwrap();
        assert!(not_found.is_none());
    }

    #[tokio::test]
    #[ignore]
    async fn sqlx_select_admission_note_dr_items_from_an() {
        let tester = MySqlTester::new_hosxp_and_kphis().await;
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/hosxp/doctor.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/hosxp/opduser.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis/ipd_dr_admission_note_item.sql")).execute(&tester.db_pool).await.unwrap();

        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/hosxp/doctor.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/hosxp/opduser.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/kphis/ipd_dr_admission_note_item.sql")).execute(&tester.db_pool).await.unwrap();

        let found = select_admission_note_dr_items_from_an("660001234",&tester.db_pool,&tester.hosxp,&tester.kphis).await.unwrap();
        assert_eq!(found.len(), 1);
        let not_found = select_admission_note_dr_items_from_an("660006666",&tester.db_pool,&tester.hosxp,&tester.kphis).await.unwrap();
        assert!(not_found.is_empty());
    }

    #[tokio::test]
    #[ignore]
    async fn sqlx_select_vs_from_an() {
        let tester = MySqlTester::new_kphis().await;
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis/ipd_vs_vital_sign.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/kphis/ipd_vs_vital_sign.sql")).execute(&tester.db_pool).await.unwrap();

        let found = select_vs_from_an("660001234", &tester.db_pool, &tester.kphis).await.unwrap();
        assert!(found.is_some());
        let not_found = select_vs_from_an("660006666", &tester.db_pool, &tester.kphis).await.unwrap();
        assert!(not_found.is_none());
    }

    #[tokio::test]
    #[ignore]
    async fn sqlx_select_period_from_an() {
        let tester = MySqlTester::new_kphis().await;
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis/ipd_nurse_admission_note.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/kphis/ipd_nurse_admission_note.sql")).execute(&tester.db_pool).await.unwrap();

        let found = select_period_from_an("660001234", &tester.db_pool, &tester.kphis).await.unwrap();
        assert!(found.is_some());
        let not_found = select_period_from_an("660006666", &tester.db_pool, &tester.kphis).await.unwrap();
        assert!(not_found.is_none());
    }

    #[tokio::test]
    #[ignore]
    async fn sqlx_select_operation_list_from_hn() {
        let tester = MySqlTester::new_hosxp().await;
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/hosxp/operation_list.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/hosxp/doctor.sql")).execute(&tester.db_pool).await.unwrap();

        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/hosxp/operation_list.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/hosxp/doctor.sql")).execute(&tester.db_pool).await.unwrap();

        let found = select_operation_list_from_hn("0001234",&datetime!(2024-11-11 11:11:11),&tester.db_pool,&tester.hosxp).await.unwrap();
        assert_eq!(found.len(), 1);
        let not_found_before = select_operation_list_from_hn("0001234",&datetime!(2023-12-31 23:59:59),&tester.db_pool,&tester.hosxp).await.unwrap();
        assert!(not_found_before.is_empty());
        let not_found = select_operation_list_from_hn("0006666",&datetime!(2024-11-11 11:11:11),&tester.db_pool,&tester.hosxp).await.unwrap();
        assert!(not_found.is_empty());
    }

    #[tokio::test]
    #[ignore]
    async fn sqlx_select_old_regdatetime_from_hn_an() {
        let tester = MySqlTester::new_hosxp().await;
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/hosxp/ipt.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/hosxp/ipt.sql")).execute(&tester.db_pool).await.unwrap();

        let found = select_old_regdatetime_from_hn_an("0001234","670001234",&tester.db_pool,&tester.hosxp).await.unwrap();
        assert!(found.is_some());
        let found_without_an = select_old_regdatetime_from_hn_an("0001234","",&tester.db_pool,&tester.hosxp).await.unwrap();
        assert!(found_without_an.is_some());
        let not_found_before_an = select_old_regdatetime_from_hn_an("0001234","660001234",&tester.db_pool,&tester.hosxp).await.unwrap();
        assert!(not_found_before_an.is_none());
        let not_found = select_old_regdatetime_from_hn_an("0006666","670001234",&tester.db_pool,&tester.hosxp).await.unwrap();
        assert!(not_found.is_none());
    }

    #[tokio::test]
    #[ignore]
    async fn sqlx_select_admission_note_from_an() {
        let tester = MySqlTester::new_kphis_and_kphis_extra().await;
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis/ipd_dr_admission_note.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis/ipd_vs_stage_of_change.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis_extra/image.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis_extra/image_usage.sql")).execute(&tester.db_pool).await.unwrap();

        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/kphis/ipd_dr_admission_note.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/kphis/ipd_vs_stage_of_change.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/kphis_extra/image.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/kphis_extra/image_usage.sql")).execute(&tester.db_pool).await.unwrap();

        let found = select_admission_note_from_an("660001234", &tester.db_pool, &tester.kphis, &tester.kphis_extra).await.unwrap();
        assert!(found.is_some());
        let not_found = select_admission_note_from_an("660006666", &tester.db_pool, &tester.kphis, &tester.kphis_extra).await.unwrap();
        assert!(not_found.is_none());
    }

    #[tokio::test]
    #[ignore]
    async fn sqlx_select_opdscreen_pe_from_vn() {
        let tester = MySqlTester::new_hosxp().await;
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/hosxp/opdscreen.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/hosxp/opdscreen.sql")).execute(&tester.db_pool).await.unwrap();

        let found = select_opdscreen_pe_from_vn("661231235959", &tester.db_pool, &tester.hosxp).await.unwrap();
        assert!(found.is_some());
        let not_found = select_opdscreen_pe_from_vn("671111111111", &tester.db_pool, &tester.hosxp).await.unwrap();
        assert!(not_found.is_none());
    }

    #[tokio::test]
    #[ignore]
    async fn sqlx_get_opd_er_allergy_list_by_vn() {
        let tester = MySqlTester::new_kphis().await;
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis/opd_er_allergy_history.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis/opd_er_order_master.sql")).execute(&tester.db_pool).await.unwrap();

        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/kphis/opd_er_allergy_history.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/kphis/opd_er_order_master.sql")).execute(&tester.db_pool).await.unwrap();

        let found = get_opd_er_allergy_list_by_vn("661231235959",&tester.db_pool,&tester.kphis).await.unwrap();
        assert_eq!(found.len(), 1);
        let not_found = get_opd_er_allergy_list_by_vn("991231235959",&tester.db_pool,&tester.kphis).await.unwrap();
        assert!(not_found.is_empty());
    }

    #[tokio::test]
    #[ignore]
    async fn sqlx_get_opd_er_allergy_with_symptom_by_vn() {
        let tester = MySqlTester::new_kphis().await;
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis/opd_er_allergy_history.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis/opd_er_order_master.sql")).execute(&tester.db_pool).await.unwrap();

        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/kphis/opd_er_allergy_history.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/kphis/opd_er_order_master.sql")).execute(&tester.db_pool).await.unwrap();

        let found = get_opd_er_allergy_with_symptom_by_vn("661231235959",&tester.db_pool,&tester.kphis).await.unwrap();
        assert!(found.is_some());
        let not_found = get_opd_er_allergy_with_symptom_by_vn("991231235959",&tester.db_pool,&tester.kphis).await.unwrap();
        assert!(not_found.is_none());
    }

    #[tokio::test]
    #[ignore]
    async fn sqlx_select_ipd_doctor_in_charge() {
        let tester = MySqlTester::new_hosxp_and_kphis().await;
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/hosxp/doctor.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis/ipd_doctor_in_charge.sql")).execute(&tester.db_pool).await.unwrap();

        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/hosxp/doctor.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/kphis/ipd_doctor_in_charge.sql")).execute(&tester.db_pool).await.unwrap();

        let found =select_ipd_doctor_in_charge("660001234", &tester.db_pool, &tester.hosxp, &tester.kphis).await.unwrap();
        assert_eq!(found.len(), 1);
        let not_found = select_ipd_doctor_in_charge("660006666", &tester.db_pool, &tester.hosxp, &tester.kphis).await.unwrap();
        assert!(not_found.is_empty());
    }

    #[tokio::test]
    #[ignore]
    async fn sqlx_post_ipd_admission_note_dr() {
        let tester = MySqlTester::new_kphis().await;
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis/ipd_dr_admission_note.sql")).execute(&tester.db_pool).await.unwrap();

        let success = post_ipd_admission_note_dr(&IpdDrAdmissionNote::demo(),"user",&tester.db_pool,&tester.kphis).await.unwrap();
        assert_eq!(success.rows_affected(), 1);
        let again_duplicate_an = post_ipd_admission_note_dr(&IpdDrAdmissionNote::demo(),"user",&tester.db_pool,&tester.kphis).await;
        if let Err(again_duplicate_an_error) = again_duplicate_an {
            assert_eq!(again_duplicate_an_error.status, 403);
        } else {
            panic!("MUST ERROR")
        }
    }

    #[tokio::test]
    #[ignore]
    async fn sqlx_put_ipd_admission_note_dr() {
        let tester = MySqlTester::new_kphis().await;
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis/ipd_dr_admission_note.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/kphis/ipd_dr_admission_note.sql")).execute(&tester.db_pool).await.unwrap();

        let success = put_ipd_admission_note_dr(&IpdDrAdmissionNote::demo(),"user",&tester.db_pool,&tester.kphis).await.unwrap();
        assert_eq!(success.rows_affected(), 1);
        let again_success = put_ipd_admission_note_dr(&IpdDrAdmissionNote::demo(),"user",&tester.db_pool,&tester.kphis).await.unwrap();
        assert_eq!(again_success.rows_affected(), 1);
    }

    #[tokio::test]
    #[ignore]
    async fn sqlx_insert_ipd_admission_note_dr_items() {
        let tester = MySqlTester::new_kphis().await;
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis/ipd_dr_admission_note_item.sql")).execute(&tester.db_pool).await.unwrap();

        let success = insert_ipd_admission_note_dr_items(&[String::from("007")],1,"660001234","user",&tester.db_pool,&tester.kphis).await.unwrap();
        assert_eq!(success.rows_affected(), 1);
        // duplicate (admission_note_id + admission_note_doctor) will error
        let again_error_unique = insert_ipd_admission_note_dr_items(&[String::from("007")],1,"660001234","user",&tester.db_pool,&tester.kphis).await;
        assert!(again_error_unique.is_err());
    }

    #[tokio::test]
    #[ignore]
    async fn sqlx_delete_ipd_admission_note_dr_items() {
        let tester = MySqlTester::new_kphis().await;
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis/ipd_dr_admission_note_item.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/kphis/ipd_dr_admission_note_item.sql")).execute(&tester.db_pool).await.unwrap();

        let success = delete_ipd_admission_note_dr_items("660001234", &tester.db_pool, &tester.kphis).await.unwrap();
        assert_eq!(success.rows_affected(), 1);
        let again_not_found = delete_ipd_admission_note_dr_items("660001234", &tester.db_pool, &tester.kphis).await.unwrap();
        assert_eq!(again_not_found.rows_affected(), 0);
    }

    #[tokio::test]
    #[ignore]
    async fn sqlx_patch_pharamacy_check() {
        let tester = MySqlTester::new_kphis().await;
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis/ipd_dr_admission_note.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/kphis/ipd_dr_admission_note.sql")).execute(&tester.db_pool).await.unwrap();

        let success = patch_pharamacy_check("660001234",&Some(String::from("007")),"user",&tester.db_pool,&tester.kphis).await.unwrap();
        assert_eq!(success.rows_affected(), 1);
        let success_again = patch_pharamacy_check("660001234",&Some(String::from("007")),"user",&tester.db_pool,&tester.kphis).await.unwrap();
        assert_eq!(success_again.rows_affected(), 1);
    }
}
