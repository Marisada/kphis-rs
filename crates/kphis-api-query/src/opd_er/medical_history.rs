use sqlx::{AssertSqlSafe, FromRow, MySql, Pool, Row, mysql::MySqlQueryResult};

use kphis_model::{
    fetch::ExecuteResponse,
    opd_er::medical_history::{
        AllergyHistory, ConsultHistory, NurseScreeningHistory, OpdErMedicalHistory, OpdErMedicalHistoryParams, OpdScreenHistory, ScanHistory, SetFtHistory, TraumaHistory, VitalSignHistory,
    },
};
use kphis_sql::{
    data_history_utils::{KeyValue, SourceTable},
    opd_er::medical_history,
};
use kphis_util::error::{AppError, Source};

use crate::{log::insert_history_log, query1_all, query1_opt, query2_all};

// opd-er-medical-history-data.php
pub async fn get_medical_history(params: &OpdErMedicalHistoryParams, hospital_name: &str, pool: &Pool<MySql>, hosxp: &str, kphis: &str) -> Result<OpdErMedicalHistory, AppError> {
    let vn = params.vn.clone().unwrap_or_default();
    let opdscreen = get_opdscreens(&vn, pool, hosxp).await?;
    if params.only_opdscreen.is_some() {
        return Ok(OpdErMedicalHistory { opdscreen, ..Default::default() });
    }

    let hn = params.hn.clone().unwrap_or_default();
    let hosxp_drugallergy = get_hosxp_drugallergy(&hn, pool, hosxp).await?;
    let hosxp_operation_history = get_hosxp_operation_history(&hn, &params.visit_datetime, hospital_name, pool, hosxp).await?;
    let hosxp_diagnosis = get_hosxp_diagnosis(&vn, pool, hosxp).await?;
    let hosxp_drug_history = get_hosxp_drug_history(&vn, pool, hosxp).await?;
    let vs_kphis = if let Some(opd_er_order_master_id) = params.opd_er_order_master_id {
        get_vs_kphis(opd_er_order_master_id, pool, kphis).await?
    } else {
        None
    };

    Ok(OpdErMedicalHistory {
        opdscreen,
        hosxp_drugallergy,
        hosxp_operation_history,
        hosxp_diagnosis,
        hosxp_drug_history,
        vs_kphis,
    })
}

async fn get_opdscreens(vn: &str, pool: &Pool<MySql>, hosxp: &str) -> Result<Option<OpdScreenHistory>, AppError> {
    let opdscreen_sql = medical_history::get_opdscreens(hosxp);
    query1_opt(vn, &opdscreen_sql, pool, "Select OpdScreen")
        .await?
        .as_ref()
        .map(OpdScreenHistory::from_row)
        .transpose()
        .map_err(|e| Source::SQLx.to_error(500, e, "Select OpdScreen"))
}

async fn get_hosxp_drugallergy(hn: &str, pool: &Pool<MySql>, hosxp: &str) -> Result<Vec<String>, AppError> {
    let hosxp_drugallergy_sql = medical_history::get_hosxp_drugallergy(hosxp);
    query1_all(hn, &hosxp_drugallergy_sql, pool, "Select HosXpDrugAllergy")
        .await?
        .iter()
        .filter_map(|row| row.try_get("drugallergy").transpose())
        .collect::<sqlx::Result<Vec<String>>>()
        .map_err(|e| Source::SQLx.to_error(500, e, "Select HosXpDrugAllergy"))
}

async fn get_hosxp_operation_history(hn: &str, visit_datetime: &Option<String>, hospital_name: &str, pool: &Pool<MySql>, hosxp: &str) -> Result<Vec<String>, AppError> {
    let hosxp_operation_history_sql = medical_history::get_hosxp_operation_history(hospital_name, hosxp);
    query2_all(hn, &visit_datetime.clone().unwrap_or_default(), &hosxp_operation_history_sql, pool, "Select HosXpOperationHistory")
        .await?
        .iter()
        .filter_map(|row| row.try_get("operation_list").transpose())
        .collect::<sqlx::Result<Vec<String>>>()
        .map_err(|e| Source::SQLx.to_error(500, e, "Select HosXpOperationHistory"))
}

async fn get_hosxp_diagnosis(vn: &str, pool: &Pool<MySql>, hosxp: &str) -> Result<Vec<String>, AppError> {
    let hosxp_diagnosis_sql = medical_history::get_hosxp_diagnosis(hosxp);
    query1_all(vn, &hosxp_diagnosis_sql, pool, "Select HosXpDiagnosis")
        .await?
        .iter()
        .filter_map(|row| row.try_get("diagnosis").transpose())
        .collect::<sqlx::Result<Vec<String>>>()
        .map_err(|e| Source::SQLx.to_error(500, e, "Select HosXpDiagnosis"))
}

async fn get_hosxp_drug_history(vn: &str, pool: &Pool<MySql>, hosxp: &str) -> Result<Vec<String>, AppError> {
    let hosxp_drug_history_sql = medical_history::get_hosxp_drug_history(hosxp);
    let results = query1_all(vn, &hosxp_drug_history_sql, pool, "Select HosXpDrugHistory")
        .await?
        .iter()
        .map(|row| row.try_get("drug_history"))
        .collect::<sqlx::Result<Vec<Option<String>>>>()
        .map_err(|e| Source::SQLx.to_error(500, e, "Select HosXpDrugHistory"))?
        .into_iter()
        .flatten()
        .collect::<Vec<String>>();

    Ok(results)
}

async fn get_vs_kphis(opd_er_order_master_id: u32, pool: &Pool<MySql>, kphis: &str) -> Result<Option<VitalSignHistory>, AppError> {
    let vs_kphis_sql = medical_history::get_vs_kphis(kphis);
    sqlx::query(AssertSqlSafe(vs_kphis_sql))
        .bind(opd_er_order_master_id)
        .fetch_optional(pool)
        .await
        .map_err(|e| Source::SQLx.to_error(500, e, "Select VitalSign"))?
        .as_ref()
        .map(VitalSignHistory::from_row)
        .transpose()
        .map_err(|e| Source::SQLx.to_error(500, e, "Select VitalSign"))
}

// opd-er-medical-history-dr-data.php
// GET /opd-er/medical-history-trauma
pub async fn get_trauma_history(opd_er_order_master_id: u32, pool: &Pool<MySql>, hosxp: &str, kphis: &str, kphis_extra: &str) -> Result<Option<TraumaHistory>, AppError> {
    let sql = medical_history::get_trauma_history(hosxp, kphis, kphis_extra);
    let result = sqlx::query(AssertSqlSafe(sql))
        .bind(opd_er_order_master_id)
        .fetch_optional(pool)
        .await
        .map_err(|e| Source::SQLx.to_error(500, e, "Select TraumaHistory"))?
        .as_ref()
        .map(TraumaHistory::from_row)
        .transpose()
        .map_err(|e| Source::SQLx.to_error(500, e, "Select TraumaHistory"))?;
    Ok(result)
}

// opd-er-medical-history-dr-save.php
// opd-er-medical-history-dr-update.php
// POST /opd-er/medical-history-trauma
pub async fn post_trauma_history(save: &TraumaHistory, doctor_code: &Option<String>, user: &str, pool: &Pool<MySql>, kphis: &str, kphis_log: &str) -> Result<(u32, Vec<ExecuteResponse>), AppError> {
    let id;
    let mut result = Vec::with_capacity(2);
    if save.opd_er_pe_id > 0 {
        // update
        id = save.opd_er_pe_id;
        let update_result = update_trauma_history(save, doctor_code, user, pool, kphis).await?;
        let is_update = update_result.rows_affected() > 0;
        result.push(ExecuteResponse::from_query_result(update_result, "Update TraumaHistory"));

        if is_update {
            let update_history_result = insert_history_log(SourceTable::OpdErDrPe, "U", user, &[KeyValue("opd_er_pe_id", id.to_string())], kphis, kphis_log, pool).await?;
            result.push(ExecuteResponse::from_query_result(update_history_result, "Update TraumaHistory History"));
        }
    } else {
        // insert
        let insert_result = insert_trauma_history(save, doctor_code, user, pool, kphis).await?;
        id = insert_result.last_insert_id() as u32;
        let is_insert = insert_result.rows_affected() > 0;
        result.push(ExecuteResponse::from_query_result(insert_result, "Insert TraumaHistory"));

        if is_insert {
            let insert_history_result = insert_history_log(SourceTable::OpdErDrPe, "I", user, &[KeyValue("opd_er_pe_id", id.to_string())], kphis, kphis_log, pool).await?;
            result.push(ExecuteResponse::from_query_result(insert_history_result, "Insert TraumaHistory History"));
        }
    }

    Ok((id, result))
}

async fn update_trauma_history(save: &TraumaHistory, doctor_code: &Option<String>, user: &str, pool: &Pool<MySql>, kphis: &str) -> Result<MySqlQueryResult, AppError> {
    let update_sql = medical_history::update_trauma_history(kphis);
    sqlx::query(AssertSqlSafe(update_sql))
        .bind(&save.arc)
        .bind(&save.arc_npc_text)
        .bind(&save.breathing_chest_wall)
        .bind(&save.breathing_lung)
        .bind(&save.circulation_shock)
        .bind(&save.circulation_shock_text)
        .bind(&save.circulation_other)
        .bind(&save.circulation_other_text)
        .bind(save.circulation_efast_date)
        .bind(save.circulation_efast_time)
        .bind(&save.circulation_doctor)
        .bind(&save.circulation)
        .bind(&save.circulation_positive_text)
        .bind(save.disability_e)
        .bind(&save.disability_v)
        .bind(save.disability_m)
        .bind(save.disability_pupil_rt)
        .bind(save.disability_pupil_lt)
        .bind(&save.disability_other)
        .bind(&save.exposure)
        .bind(doctor_code)
        .bind(user)
        .bind(save.opd_er_pe_id)
        .bind(save.version)
        .execute(pool)
        .await
        .map_err(|e| Source::SQLx.to_error(500, e, "Update TraumaHistory"))
}

async fn insert_trauma_history(save: &TraumaHistory, doctor_code: &Option<String>, user: &str, pool: &Pool<MySql>, kphis: &str) -> Result<MySqlQueryResult, AppError> {
    let insert_sql = medical_history::insert_trauma_history(kphis);
    sqlx::query(AssertSqlSafe(insert_sql))
        .bind(save.opd_er_order_master_id)
        .bind(&save.arc)
        .bind(&save.arc_npc_text)
        .bind(&save.breathing_chest_wall)
        .bind(&save.breathing_lung)
        .bind(&save.circulation_shock)
        .bind(&save.circulation_shock_text)
        .bind(&save.circulation_other)
        .bind(&save.circulation_other_text)
        .bind(save.circulation_efast_date)
        .bind(save.circulation_efast_time)
        .bind(&save.circulation_doctor)
        .bind(&save.circulation)
        .bind(&save.circulation_positive_text)
        .bind(save.disability_e)
        .bind(&save.disability_v)
        .bind(save.disability_m)
        .bind(save.disability_pupil_rt)
        .bind(save.disability_pupil_lt)
        .bind(&save.disability_other)
        .bind(&save.exposure)
        .bind(doctor_code)
        .bind(user)
        .bind(user)
        .execute(pool)
        .await
        .map_err(|e| Source::SQLx.to_error(500, e, "Insert TraumaHistory"))
}

// opd-er-allergy-history-edit.php
// GET /opd-er/medical-history-allergy
pub async fn get_allergy_history(opd_er_order_master_id: u32, pool: &Pool<MySql>, hosxp: &str, kphis: &str) -> Result<Vec<AllergyHistory>, AppError> {
    let sql = medical_history::get_allergy_history(hosxp, kphis);
    let result = sqlx::query(AssertSqlSafe(sql))
        .bind(opd_er_order_master_id)
        .fetch_all(pool)
        .await
        .map_err(|e| Source::SQLx.to_error(500, e, "Select AllergyHistory"))?
        .iter()
        .map(AllergyHistory::from_row)
        .collect::<sqlx::Result<Vec<AllergyHistory>>>()
        .map_err(|e| Source::SQLx.to_error(500, e, "Select AllergyHistory"))?;
    Ok(result)
}

// opd-er-allergy-history-save.php
// POST /opd-er/medical-history-allergy
#[allow(clippy::too_many_arguments)]
pub async fn post_allergy_history(
    saves: &[AllergyHistory],
    opd_er_order_master_id: u32,
    version: i32,
    doctor_code: &Option<String>,
    user: &str,
    pool: &Pool<MySql>,
    kphis: &str,
    kphis_log: &str,
) -> Result<Vec<ExecuteResponse>, AppError> {
    let mut result = Vec::with_capacity(3);
    let mut be_insert = true;
    if version > 0 {
        // update = delete then insert(below)
        let delete_result = delete_allergy_history(opd_er_order_master_id, version, pool, kphis).await?;
        be_insert = delete_result.rows_affected() > 0;
        result.push(ExecuteResponse::from_query_result(delete_result, "Update AllergyHistory"));
    }
    if be_insert {
        // insert
        let insert_result = insert_allergy_history(opd_er_order_master_id, saves, version, doctor_code, user, pool, kphis).await?;
        let is_insert = insert_result.rows_affected() > 0;
        result.push(ExecuteResponse::from_query_result(insert_result, "Insert AllergyHistory"));

        if is_insert {
            let flag = if version > 0 { "U" } else { "I" };
            let insert_history_result = insert_history_log(
                SourceTable::OpdErAllergyHistory,
                flag,
                user,
                &[KeyValue("opd_er_order_master_id", opd_er_order_master_id.to_string())],
                kphis,
                kphis_log,
                pool,
            )
            .await?;
            result.push(ExecuteResponse::from_query_result(insert_history_result, "Insert AllergyHistory History"));
        }

        Ok(result)
    } else {
        Err(AppError::app_404("Update AllergyHistory"))
    }
}

async fn delete_allergy_history(opd_er_order_master_id: u32, version: i32, pool: &Pool<MySql>, kphis: &str) -> Result<MySqlQueryResult, AppError> {
    let delete_sql = medical_history::delete_allergy_history(kphis);
    sqlx::query(AssertSqlSafe(delete_sql))
        .bind(opd_er_order_master_id)
        .bind(version)
        .execute(pool)
        .await
        .map_err(|e| Source::SQLx.to_error(500, e, "Update AllergyHistory"))
}

async fn insert_allergy_history(
    opd_er_order_master_id: u32,
    saves: &[AllergyHistory],
    version: i32,
    doctor_code: &Option<String>,
    user: &str,
    pool: &Pool<MySql>,
    kphis: &str,
) -> Result<MySqlQueryResult, AppError> {
    let insert_sql = medical_history::insert_allergy_history(saves.len(), kphis);
    let mut insert_query = sqlx::query(AssertSqlSafe(insert_sql));
    for save in saves {
        insert_query = insert_query
            .bind(opd_er_order_master_id)
            .bind(&save.er_allergy_history_agent)
            .bind(&save.er_allergy_history_symptom)
            .bind(doctor_code)
            .bind(user)
            .bind(user)
            .bind(version + 1);
    }
    insert_query.execute(pool).await.map_err(|e| Source::SQLx.to_error(500, e, "Insert AllergyHistory"))
}

// opd-er-medical-history-nurse-data.php
// GET /opd-er/medical-history-screen
pub async fn get_screen_history(opd_er_order_master_id: u32, pool: &Pool<MySql>, hosxp: &str, kphis: &str) -> Result<Option<NurseScreeningHistory>, AppError> {
    let sql = medical_history::get_nurse_screening_history(hosxp, kphis);
    let result = sqlx::query(AssertSqlSafe(sql))
        .bind(opd_er_order_master_id)
        .fetch_optional(pool)
        .await
        .map_err(|e| Source::SQLx.to_error(500, e, "Select ScreenHistory"))?
        .as_ref()
        .map(NurseScreeningHistory::from_row)
        .transpose()
        .map_err(|e| Source::SQLx.to_error(500, e, "Select ScreenHistory"))?;
    Ok(result)
}

// opd-er-medical-history-nurse-save.php
// opd-er-medical-history-nurse-update.php
// POST /opd-er/medical-history-screen
pub async fn post_screen_history(
    save: &NurseScreeningHistory,
    view_by: &str,
    doctor_code: &Option<String>,
    user: &str,
    pool: &Pool<MySql>,
    kphis: &str,
    kphis_log: &str,
) -> Result<(u32, Vec<ExecuteResponse>), AppError> {
    let id;
    let mut result = Vec::with_capacity(2);
    if save.opd_er_screening_id > 0 {
        // update
        id = save.opd_er_screening_id;
        let update_result = update_nurse_screening_history(save, view_by, doctor_code, user, pool, kphis).await?;
        let is_update = update_result.rows_affected() > 0;
        result.push(ExecuteResponse::from_query_result(update_result, "Update ScreenHistory"));

        if is_update {
            let update_history_result = insert_history_log(SourceTable::OpdErNurseScreening, "U", user, &[KeyValue("opd_er_screening_id", id.to_string())], kphis, kphis_log, pool).await?;
            result.push(ExecuteResponse::from_query_result(update_history_result, "Update ScreenHistory History"));
        }
    } else {
        // insert
        let insert_result = insert_nurse_screening_history(save, view_by, doctor_code, user, pool, kphis).await?;
        id = insert_result.last_insert_id() as u32;
        let is_insert = insert_result.rows_affected() > 0;
        result.push(ExecuteResponse::from_query_result(insert_result, "Insert ScreenHistory"));

        if is_insert {
            let insert_history_result = insert_history_log(SourceTable::OpdErNurseScreening, "I", user, &[KeyValue("opd_er_screening_id", id.to_string())], kphis, kphis_log, pool).await?;
            result.push(ExecuteResponse::from_query_result(insert_history_result, "Insert ScreenHistory History"));
        }
    }

    Ok((id, result))
}

async fn update_nurse_screening_history(save: &NurseScreeningHistory, view_by: &str, doctor_code: &Option<String>, user: &str, pool: &Pool<MySql>, kphis: &str) -> Result<MySqlQueryResult, AppError> {
    let update_sql = medical_history::update_nurse_screening_history(view_by, kphis);
    sqlx::query(AssertSqlSafe(update_sql))
        .bind(&save.screening_emergency_level)
        .bind(&save.screening_spclty)
        .bind(save.screening_arrive_date)
        .bind(save.screening_arrive_time)
        .bind(save.screening_date)
        .bind(save.screening_time)
        .bind(save.screening_report_date)
        .bind(save.screening_report_time)
        .bind(save.screening_see_doctor_date)
        .bind(save.screening_see_doctor_time)
        .bind(doctor_code)
        .bind(user)
        .bind(save.opd_er_screening_id)
        .bind(save.version)
        .execute(pool)
        .await
        .map_err(|e| Source::SQLx.to_error(500, e, "Update ScreenHistory"))
}

async fn insert_nurse_screening_history(save: &NurseScreeningHistory, view_by: &str, doctor_code: &Option<String>, user: &str, pool: &Pool<MySql>, kphis: &str) -> Result<MySqlQueryResult, AppError> {
    let (doctor, nurse) = if view_by == "doctor" { (doctor_code, &None) } else { (&None, doctor_code) };
    let insert_sql = medical_history::insert_nurse_screening_history(kphis);
    sqlx::query(AssertSqlSafe(insert_sql))
        .bind(save.opd_er_order_master_id)
        .bind(&save.screening_emergency_level)
        .bind(&save.screening_spclty)
        .bind(save.screening_arrive_date)
        .bind(save.screening_arrive_time)
        .bind(save.screening_date)
        .bind(save.screening_time)
        .bind(save.screening_report_date)
        .bind(save.screening_report_time)
        .bind(save.screening_see_doctor_date)
        .bind(save.screening_see_doctor_time)
        .bind(doctor)
        .bind(nurse)
        .bind(user)
        .bind(user)
        .execute(pool)
        .await
        .map_err(|e| Source::SQLx.to_error(500, e, "Insert ScreenHistory"))
}

// opd-er-consult-dr-edit.php
// GET /opd-er/medical-history-consult
pub async fn get_consult_history(opd_er_order_master_id: u32, pool: &Pool<MySql>, hosxp: &str, kphis: &str) -> Result<Vec<ConsultHistory>, AppError> {
    let sql = medical_history::get_consult_history(hosxp, kphis);
    let result = sqlx::query(AssertSqlSafe(sql))
        .bind(opd_er_order_master_id)
        .fetch_all(pool)
        .await
        .map_err(|e| Source::SQLx.to_error(500, e, "Select ConsultHistory"))?
        .iter()
        .map(ConsultHistory::from_row)
        .collect::<sqlx::Result<Vec<ConsultHistory>>>()
        .map_err(|e| Source::SQLx.to_error(500, e, "Select ConsultHistory"))?;
    Ok(result)
}

// opd-er-consult-dr-save.php
// POST /opd-er/medical-history-consult
#[allow(clippy::too_many_arguments)]
pub async fn post_consult_history(
    saves: &[ConsultHistory],
    opd_er_order_master_id: u32,
    version: i32,
    doctor_code: &Option<String>,
    user: &str,
    pool: &Pool<MySql>,
    kphis: &str,
    kphis_log: &str,
) -> Result<Vec<ExecuteResponse>, AppError> {
    let mut result = Vec::with_capacity(3);
    let mut be_insert = true;
    if version > 0 {
        // update = delete then insert(below)
        let delete_result = delete_consult_history(opd_er_order_master_id, version, pool, kphis).await?;
        be_insert = delete_result.rows_affected() > 0;
        result.push(ExecuteResponse::from_query_result(delete_result, "Update ConsultHistory"));
    }
    if be_insert {
        // insert
        let insert_result = insert_consult_history(opd_er_order_master_id, saves, version, doctor_code, user, pool, kphis).await?;
        let is_insert = insert_result.rows_affected() > 0;
        result.push(ExecuteResponse::from_query_result(insert_result, "Insert ConsultHistory"));

        if is_insert {
            let flag = if version > 0 { "U" } else { "I" };
            let insert_history_result = insert_history_log(
                SourceTable::OpdErConsult,
                flag,
                user,
                &[KeyValue("opd_er_order_master_id", opd_er_order_master_id.to_string())],
                kphis,
                kphis_log,
                pool,
            )
            .await?;
            result.push(ExecuteResponse::from_query_result(insert_history_result, "Insert ConsultHistory History"));
        }

        Ok(result)
    } else {
        Err(AppError::app_404("Update ConsultHistory"))
    }
}

async fn delete_consult_history(opd_er_order_master_id: u32, version: i32, pool: &Pool<MySql>, kphis: &str) -> Result<MySqlQueryResult, AppError> {
    let delete_sql = medical_history::delete_consult_history(kphis);
    sqlx::query(AssertSqlSafe(delete_sql))
        .bind(opd_er_order_master_id)
        .bind(version)
        .execute(pool)
        .await
        .map_err(|e| Source::SQLx.to_error(500, e, "Update ConsultHistory"))
}

async fn insert_consult_history(
    opd_er_order_master_id: u32,
    saves: &[ConsultHistory],
    version: i32,
    doctor_code: &Option<String>,
    user: &str,
    pool: &Pool<MySql>,
    kphis: &str,
) -> Result<MySqlQueryResult, AppError> {
    let insert_sql = medical_history::insert_consult_history(saves.len(), kphis);
    let mut insert_query = sqlx::query(AssertSqlSafe(insert_sql));
    for save in saves {
        insert_query = insert_query
            .bind(opd_er_order_master_id)
            .bind(&save.er_consult_ward)
            .bind(save.er_consult_date)
            .bind(save.er_consult_time)
            .bind(&save.er_consult_doctor_reply)
            .bind(save.er_consult_date_reply)
            .bind(save.er_consult_time_reply)
            .bind(doctor_code)
            .bind(user)
            .bind(user)
            .bind(version + 1);
    }
    insert_query.execute(pool).await.map_err(|e| Source::SQLx.to_error(500, e, "Insert ConsultHistory"))
}

// opd-er-document-scan-data.php
// GET /opd-er/medical-history-scan
pub async fn get_scan_history(opd_er_order_master_id: u32, pool: &Pool<MySql>, kphis: &str) -> Result<Option<ScanHistory>, AppError> {
    let sql = medical_history::get_scan_history(kphis);
    let result = sqlx::query(AssertSqlSafe(sql))
        .bind(opd_er_order_master_id)
        .fetch_optional(pool)
        .await
        .map_err(|e| Source::SQLx.to_error(500, e, "Select ScanHistory"))?
        .as_ref()
        .map(ScanHistory::from_row)
        .transpose()
        .map_err(|e| Source::SQLx.to_error(500, e, "Select ScanHistory"))?;
    Ok(result)
}

// opd-er-document-scan-save.php
// opd-er-document-scan-update.php
// POST /opd-er/medical-history-scan
pub async fn post_scan_history(save: &ScanHistory, doctor_code: &Option<String>, user: &str, pool: &Pool<MySql>, kphis: &str, kphis_log: &str) -> Result<(u32, Vec<ExecuteResponse>), AppError> {
    let id;
    let mut result = Vec::with_capacity(2);
    if save.opd_er_document_scan_id > 0 {
        // update
        id = save.opd_er_document_scan_id;
        let update_result = update_scan_history(save, doctor_code, user, pool, kphis).await?;
        let is_update = update_result.rows_affected() > 0;
        result.push(ExecuteResponse::from_query_result(update_result, "Update ScanHistory"));

        if is_update {
            let update_history_result = insert_history_log(
                SourceTable::OpdErDocumentScan,
                "U",
                user,
                &[KeyValue("opd_er_document_scan_id", id.to_string())],
                kphis,
                kphis_log,
                pool,
            )
            .await?;
            result.push(ExecuteResponse::from_query_result(update_history_result, "Update ScanHistory History"));
        }
    } else {
        // insert
        let insert_result = insert_scan_history(save, doctor_code, user, pool, kphis).await?;
        id = insert_result.last_insert_id() as u32;
        let is_insert = insert_result.rows_affected() > 0;
        result.push(ExecuteResponse::from_query_result(insert_result, "Insert ScanHistory"));

        if is_insert {
            let insert_history_result = insert_history_log(
                SourceTable::OpdErDocumentScan,
                "I",
                user,
                &[KeyValue("opd_er_document_scan_id", id.to_string())],
                kphis,
                kphis_log,
                pool,
            )
            .await?;
            result.push(ExecuteResponse::from_query_result(insert_history_result, "Insert ScanHistory History"));
        }
    }

    Ok((id, result))
}

async fn update_scan_history(save: &ScanHistory, doctor_code: &Option<String>, user: &str, pool: &Pool<MySql>, kphis: &str) -> Result<MySqlQueryResult, AppError> {
    let update_sql = medical_history::update_scan_history(kphis);
    sqlx::query(AssertSqlSafe(update_sql))
        .bind(save.opd_er_order_master_id)
        .bind(&save.opd_er_document_scan)
        .bind(doctor_code)
        .bind(user)
        .bind(save.opd_er_document_scan_id)
        .bind(save.version)
        .execute(pool)
        .await
        .map_err(|e| Source::SQLx.to_error(500, e, "Update ScanHistory"))
}

async fn insert_scan_history(save: &ScanHistory, doctor_code: &Option<String>, user: &str, pool: &Pool<MySql>, kphis: &str) -> Result<MySqlQueryResult, AppError> {
    let insert_sql = medical_history::insert_scan_history(kphis);
    sqlx::query(AssertSqlSafe(insert_sql))
        .bind(save.opd_er_order_master_id)
        .bind(&save.opd_er_document_scan)
        .bind(doctor_code)
        .bind(user)
        .bind(user)
        .execute(pool)
        .await
        .map_err(|e| Source::SQLx.to_error(500, e, "Insert ScanHistory"))
}

// opd-er-set-ft-data.php
// GET /opd-er/medical-history-ft
pub async fn get_ft_history(opd_er_order_master_id: u32, pool: &Pool<MySql>, hosxp: &str, kphis: &str) -> Result<Option<SetFtHistory>, AppError> {
    let sql = medical_history::get_set_ft_history(hosxp, kphis);
    let result = sqlx::query(AssertSqlSafe(sql))
        .bind(opd_er_order_master_id)
        .fetch_optional(pool)
        .await
        .map_err(|e| Source::SQLx.to_error(500, e, "Select SetFtHistory"))?
        .as_ref()
        .map(SetFtHistory::from_row)
        .transpose()
        .map_err(|e| Source::SQLx.to_error(500, e, "Select SetFtHistory"))?;
    Ok(result)
}

// opd-er-set-ft-save.php
// opd-er-set-ft-update.php
// POST /opd-er/medical-history-ft
pub async fn post_ft_history(save: &SetFtHistory, doctor_code: &Option<String>, user: &str, pool: &Pool<MySql>, kphis: &str, kphis_log: &str) -> Result<(u32, Vec<ExecuteResponse>), AppError> {
    let id;
    let mut result = Vec::with_capacity(2);
    if save.set_ft_id > 0 {
        // update
        id = save.set_ft_id;
        let update_result = update_ft_history(save, doctor_code, user, pool, kphis).await?;
        let is_update = update_result.rows_affected() > 0;
        result.push(ExecuteResponse::from_query_result(update_result, "Update SetFtHistory"));

        if is_update {
            let update_history_result = insert_history_log(SourceTable::OpdErSetFastTrack, "U", user, &[KeyValue("set_ft_id", id.to_string())], kphis, kphis_log, pool).await?;
            result.push(ExecuteResponse::from_query_result(update_history_result, "Update SetFtHistory History"));
        }
    } else {
        // insert
        let insert_result = insert_ft_history(save, doctor_code, user, pool, kphis).await?;
        id = insert_result.last_insert_id() as u32;
        let is_insert = insert_result.rows_affected() > 0;
        result.push(ExecuteResponse::from_query_result(insert_result, "Insert SetFtHistory"));

        if is_insert {
            let insert_history_result = insert_history_log(SourceTable::OpdErSetFastTrack, "I", user, &[KeyValue("set_ft_id", id.to_string())], kphis, kphis_log, pool).await?;
            result.push(ExecuteResponse::from_query_result(insert_history_result, "Insert SetFtHistory History"));
        }
    }

    Ok((id, result))
}
async fn update_ft_history(save: &SetFtHistory, doctor_code: &Option<String>, user: &str, pool: &Pool<MySql>, kphis: &str) -> Result<MySqlQueryResult, AppError> {
    let update_sql = medical_history::update_ft_history(kphis);
    sqlx::query(AssertSqlSafe(update_sql))
        .bind(save.opd_er_order_master_id)
        .bind(save.set_ft_date)
        .bind(save.set_ft_time)
        .bind(doctor_code)
        .bind(user)
        .bind(save.set_ft_id)
        .bind(save.version)
        .execute(pool)
        .await
        .map_err(|e| Source::SQLx.to_error(500, e, "Update SetFtHistory"))
}

async fn insert_ft_history(save: &SetFtHistory, doctor_code: &Option<String>, user: &str, pool: &Pool<MySql>, kphis: &str) -> Result<MySqlQueryResult, AppError> {
    let insert_sql = medical_history::insert_ft_history(kphis);
    sqlx::query(AssertSqlSafe(insert_sql))
        .bind(save.opd_er_order_master_id)
        .bind(save.set_ft_date)
        .bind(save.set_ft_time)
        .bind(doctor_code)
        .bind(user)
        .bind(user)
        .execute(pool)
        .await
        .map_err(|e| Source::SQLx.to_error(500, e, "Insert SetFtHistory"))
}

#[cfg(test)]
#[rustfmt::skip]
mod tests {

    use super::*;
    use kphis_sqlx_tester::MySqlTester;

    #[tokio::test]
    #[ignore]
    async fn sqlx_get_opdscreens() {
        let tester = MySqlTester::new_hosxp().await;
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/hosxp/opdscreen.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/hosxp/er_regist.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/hosxp/er_pt_type.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/hosxp/er_emergency_type.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/hosxp/er_emergency_level.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/hosxp/er_nursing_detail.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/hosxp/ovst.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/hosxp/spclty.sql")).execute(&tester.db_pool).await.unwrap();

        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/hosxp/opdscreen.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/hosxp/er_regist.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/hosxp/er_pt_type.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/hosxp/er_emergency_type.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/hosxp/er_emergency_level.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/hosxp/er_nursing_detail.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/hosxp/ovst.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/hosxp/spclty.sql")).execute(&tester.db_pool).await.unwrap();

        let found = get_opdscreens("661231235959", &tester.db_pool, &tester.hosxp).await.unwrap();
        assert!(found.is_some());
        let not_found = get_opdscreens("991231235959", &tester.db_pool, &tester.hosxp).await.unwrap();
        assert!(not_found.is_none());
    }

    #[tokio::test]
    #[ignore]
    async fn sqlx_get_hosxp_drugallergy() {
        let tester = MySqlTester::new_hosxp().await;
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/hosxp/opd_allergy.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/hosxp/opd_allergy.sql")).execute(&tester.db_pool).await.unwrap();

        let found = get_hosxp_drugallergy("0001234", &tester.db_pool, &tester.hosxp).await.unwrap();
        assert_eq!(found.len(), 1);
        let not_found = get_hosxp_drugallergy("0006666", &tester.db_pool, &tester.hosxp).await.unwrap();
        assert!(not_found.is_empty());
    }

    #[tokio::test]
    #[ignore]
    async fn sqlx_get_hosxp_operation_history() {
        let tester = MySqlTester::new_hosxp().await;
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/hosxp/operation_list.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/hosxp/doctor.sql")).execute(&tester.db_pool).await.unwrap();

        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/hosxp/operation_list.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/hosxp/doctor.sql")).execute(&tester.db_pool).await.unwrap();

        // show only item that enter_date + enter_time < 'visit_datetime'
        let no_visit_datetime = get_hosxp_operation_history("0001234",&None,"Hospital",&tester.db_pool,&tester.hosxp).await.unwrap();
        assert!(no_visit_datetime.is_empty());
        let found = get_hosxp_operation_history("0001234",&Some(String::from("2024-05-05 23:59:59")),"Hospital",&tester.db_pool,&tester.hosxp).await.unwrap();
        assert_eq!(found.len(), 1);
        let not_found = get_hosxp_operation_history("0001234",&Some(String::from("2023-12-31 23:59:59")),"Hospital",&tester.db_pool,&tester.hosxp).await.unwrap();
        assert!(not_found.is_empty());
    }

    #[tokio::test]
    #[ignore]
    async fn sqlx_get_hosxp_diagnosis() {
        let tester = MySqlTester::new_hosxp().await;
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/hosxp/ovstdiag.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/hosxp/icd101.sql")).execute(&tester.db_pool).await.unwrap();

        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/hosxp/ovstdiag.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/hosxp/icd101.sql")).execute(&tester.db_pool).await.unwrap();

        let found = get_hosxp_diagnosis("661231235959", &tester.db_pool, &tester.hosxp).await.unwrap();
        assert_eq!(found.len(), 1);
        let not_found = get_hosxp_diagnosis("991231235959", &tester.db_pool, &tester.hosxp).await.unwrap();
        assert!(not_found.is_empty());
    }

    #[tokio::test]
    #[ignore]
    async fn sqlx_get_hosxp_drug_history() {
        let tester = MySqlTester::new_hosxp().await;
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/hosxp/opitemrece.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/hosxp/drugitems.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/hosxp/drugusage.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/hosxp/sp_use.sql")).execute(&tester.db_pool).await.unwrap();

        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/hosxp/opitemrece.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/hosxp/drugitems.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/hosxp/drugusage.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/hosxp/sp_use.sql")).execute(&tester.db_pool).await.unwrap();

        let found = get_hosxp_drug_history("661231235959", &tester.db_pool, &tester.hosxp).await.unwrap();
        assert_eq!(found.len(), 6);
        let not_found = get_hosxp_drug_history("991231235959", &tester.db_pool, &tester.hosxp).await.unwrap();
        assert!(not_found.is_empty());
    }

    #[tokio::test]
    #[ignore]
    async fn sqlx_get_vs_kphis() {
        let tester = MySqlTester::new_kphis().await;
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis/opd_er_vs_vital_sign.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/kphis/opd_er_vs_vital_sign.sql")).execute(&tester.db_pool).await.unwrap();

        let found = get_vs_kphis(1, &tester.db_pool, &tester.kphis).await.unwrap();
        assert!(found.is_some());
        let not_found = get_vs_kphis(999, &tester.db_pool, &tester.kphis).await.unwrap();
        assert!(not_found.is_none());
    }

    #[tokio::test]
    #[ignore]
    async fn sqlx_get_trauma_history() {
        let tester = MySqlTester::new_hosxp_and_kphis_and_kphis_extra().await;
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis/opd_er_dr_pe.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/hosxp/doctor.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis_extra/image.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis_extra/image_usage.sql")).execute(&tester.db_pool).await.unwrap();

        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/kphis/opd_er_dr_pe.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/hosxp/doctor.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/kphis_extra/image.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/kphis_extra/image_usage.sql")).execute(&tester.db_pool).await.unwrap();

        let found = get_trauma_history(1, &tester.db_pool, &tester.hosxp, &tester.kphis, &tester.kphis_extra).await.unwrap();
        assert!(found.is_some());
        let not_found = get_trauma_history(999, &tester.db_pool, &tester.hosxp, &tester.kphis, &tester.kphis_extra).await.unwrap();
        assert!(not_found.is_none());
    }

    #[tokio::test]
    #[ignore]
    async fn sqlx_get_allergy_history() {
        let tester = MySqlTester::new_hosxp_and_kphis().await;
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis/opd_er_allergy_history.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/hosxp/doctor.sql")).execute(&tester.db_pool).await.unwrap();

        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/kphis/opd_er_allergy_history.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/hosxp/doctor.sql")).execute(&tester.db_pool).await.unwrap();

        let found = get_allergy_history(1, &tester.db_pool, &tester.hosxp, &tester.kphis).await.unwrap();
        assert_eq!(found.len(), 1);
        let not_found = get_allergy_history(999, &tester.db_pool, &tester.hosxp, &tester.kphis).await.unwrap();
        assert!(not_found.is_empty());
    }

    #[tokio::test]
    #[ignore]
    async fn sqlx_get_screen_history() {
        let tester = MySqlTester::new_hosxp_and_kphis().await;
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis/opd_er_nurse_screening.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/hosxp/doctor.sql")).execute(&tester.db_pool).await.unwrap();

        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/kphis/opd_er_nurse_screening.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/hosxp/doctor.sql")).execute(&tester.db_pool).await.unwrap();

        let found = get_screen_history(1, &tester.db_pool, &tester.hosxp, &tester.kphis).await    .unwrap();
        assert!(found.is_some());
        let not_found = get_screen_history(999, &tester.db_pool, &tester.hosxp, &tester.kphis).await.unwrap();
        assert!(not_found.is_none());
    }

    #[tokio::test]
    #[ignore]
    async fn sqlx_get_consult_history() {
        let tester = MySqlTester::new_hosxp_and_kphis().await;
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis/opd_er_consult.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis/kphis_spclty.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/hosxp/doctor.sql")).execute(&tester.db_pool).await.unwrap();

        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/kphis/opd_er_consult.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/kphis/kphis_spclty.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/hosxp/doctor.sql")).execute(&tester.db_pool).await.unwrap();

        let found = get_consult_history(1, &tester.db_pool, &tester.hosxp, &tester.kphis).await.unwrap();
        assert_eq!(found.len(), 1);
        let not_found = get_consult_history(999, &tester.db_pool, &tester.hosxp, &tester.kphis).await.unwrap();
        assert!(not_found.is_empty());
    }

    #[tokio::test]
    #[ignore]
    async fn sqlx_get_scan_history() {
        let tester = MySqlTester::new_kphis().await;
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis/opd_er_document_scan.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/kphis/opd_er_document_scan.sql")).execute(&tester.db_pool).await.unwrap();

        let found = get_scan_history(1, &tester.db_pool, &tester.kphis).await.unwrap();
        assert!(found.is_some());
        let not_found = get_scan_history(999, &tester.db_pool, &tester.kphis).await.unwrap();
        assert!(not_found.is_none());
    }

    #[tokio::test]
    #[ignore]
    async fn sqlx_get_ft_history() {
        let tester = MySqlTester::new_hosxp_and_kphis().await;
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis/opd_er_set_fast_track.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/hosxp/doctor.sql")).execute(&tester.db_pool).await.unwrap();

        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/kphis/opd_er_set_fast_track.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/hosxp/doctor.sql")).execute(&tester.db_pool).await.unwrap();

        let found = get_ft_history(1, &tester.db_pool, &tester.hosxp, &tester.kphis).await.unwrap();
        assert!(found.is_some());
        let not_found = get_ft_history(999, &tester.db_pool, &tester.hosxp, &tester.kphis).await.unwrap();
        assert!(not_found.is_none());
    }

    #[tokio::test]
    #[ignore]
    async fn sqlx_insert_trauma_history() {
        let tester = MySqlTester::new_kphis().await;
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis/opd_er_dr_pe.sql")).execute(&tester.db_pool).await.unwrap();

        let success = insert_trauma_history(&TraumaHistory::demo(),&None,"user",&tester.db_pool,&tester.kphis).await.unwrap();
        assert_eq!(success.rows_affected(), 1);
        // duplicate opd_er_order_master_id will error
        let again_error_duplicate_unique = insert_trauma_history(&TraumaHistory::demo(),&None,"user",&tester.db_pool,&tester.kphis).await;
        assert!(again_error_duplicate_unique.is_err());
    }

    #[tokio::test]
    #[ignore]
    async fn sqlx_insert_allergy_history() {
        let tester = MySqlTester::new_kphis().await;
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis/opd_er_allergy_history.sql")).execute(&tester.db_pool).await.unwrap();

        let success = insert_allergy_history(1,&[AllergyHistory::demo()],1,&Some(String::from("007")),"user",&tester.db_pool,&tester.kphis).await.unwrap();
        assert_eq!(success.rows_affected(), 1);
        let again_success = insert_allergy_history(1,&[AllergyHistory::demo()],1,&None,"user",&tester.db_pool,&tester.kphis).await.unwrap();
        assert_eq!(again_success.rows_affected(), 1);
    }

    #[tokio::test]
    #[ignore]
    async fn sqlx_insert_nurse_screening_history() {
        let tester = MySqlTester::new_kphis().await;
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis/opd_er_nurse_screening.sql")).execute(&tester.db_pool).await.unwrap();

        let mut hx = NurseScreeningHistory::demo();
        let success_doctor = insert_nurse_screening_history(&hx,"doctor",&Some(String::from("007")),"user",&tester.db_pool,&tester.kphis).await.unwrap();
        assert_eq!(success_doctor.rows_affected(), 1);
        hx.opd_er_order_master_id = 2;
        let success_nurse = insert_nurse_screening_history(&hx,"nurse",&None,"user",&tester.db_pool,&tester.kphis).await.unwrap();
        assert_eq!(success_nurse.rows_affected(), 1);
        // duplicate opd_er_order_master_id will error
        let again_error_duplicate_pk = insert_nurse_screening_history(&hx,"nurse",&None,"user",&tester.db_pool,&tester.kphis).await;
        assert!(again_error_duplicate_pk.is_err());
    }

    #[tokio::test]
    #[ignore]
    async fn sqlx_insert_consult_history() {
        let tester = MySqlTester::new_kphis().await;
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis/opd_er_consult.sql")).execute(&tester.db_pool).await.unwrap();

        let success = insert_consult_history(1,&[ConsultHistory::demo()],1,&Some(String::from("007")),"user",&tester.db_pool,&tester.kphis).await.unwrap();
        assert_eq!(success.rows_affected(), 1);
        let again_success = insert_consult_history(1,&[ConsultHistory::demo()],1,&None,"user",&tester.db_pool,&tester.kphis).await.unwrap();
        assert_eq!(again_success.rows_affected(), 1);
    }

    #[tokio::test]
    #[ignore]
    async fn sqlx_insert_scan_history() {
        let tester = MySqlTester::new_kphis().await;
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis/opd_er_document_scan.sql")).execute(&tester.db_pool).await.unwrap();

        let success = insert_scan_history(&ScanHistory::demo(),&None,"user",&tester.db_pool,&tester.kphis).await.unwrap();
        assert_eq!(success.rows_affected(), 1);
        // duplicate opd_er_order_master_id will error
        let again_error_duplicate_pk = insert_scan_history(&ScanHistory::demo(),&None,"user",&tester.db_pool,&tester.kphis).await;
        assert!(again_error_duplicate_pk.is_err());
    }

    #[tokio::test]
    #[ignore]
    async fn sqlx_insert_ft_history() {
        let tester = MySqlTester::new_kphis().await;
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis/opd_er_set_fast_track.sql")).execute(&tester.db_pool).await.unwrap();

        let success = insert_ft_history(&SetFtHistory::demo(),&None,"user",&tester.db_pool,&tester.kphis).await.unwrap();
        assert_eq!(success.rows_affected(), 1);
        // duplicate opd_er_order_master_id will error
        let again_error_duplicate_pk = insert_ft_history(&SetFtHistory::demo(),&None,"user",&tester.db_pool,&tester.kphis).await;
        assert!(again_error_duplicate_pk.is_err());
    }

    #[tokio::test]
    #[ignore]
    async fn sqlx_update_trauma_history() {
        let tester = MySqlTester::new_kphis().await;
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis/opd_er_dr_pe.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/kphis/opd_er_dr_pe.sql")).execute(&tester.db_pool).await.unwrap();

        let success = update_trauma_history(&TraumaHistory::demo(),&None,"user",&tester.db_pool,&tester.kphis).await.unwrap();
        assert_eq!(success.rows_affected(), 1);
        let again_fail_version = update_trauma_history(&TraumaHistory::demo(),&None,"user",&tester.db_pool,&tester.kphis).await.unwrap();
        assert_eq!(again_fail_version.rows_affected(), 0);
    }

    #[tokio::test]
    #[ignore]
    async fn sqlx_update_nurse_screening_history() {
        let tester = MySqlTester::new_kphis().await;
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis/opd_er_nurse_screening.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/kphis/opd_er_nurse_screening.sql")).execute(&tester.db_pool).await.unwrap();

        let success = update_nurse_screening_history(&NurseScreeningHistory::demo(),"doctor",&None,"user",&tester.db_pool,&tester.kphis).await.unwrap();
        assert_eq!(success.rows_affected(), 1);
        let again_fail_version = update_nurse_screening_history(&NurseScreeningHistory::demo(),"doctor",&None,"user",&tester.db_pool,&tester.kphis).await.unwrap();
        assert_eq!(again_fail_version.rows_affected(), 0);
    }

    #[tokio::test]
    #[ignore]
    async fn sqlx_update_scan_history() {
        let tester = MySqlTester::new_kphis().await;
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis/opd_er_document_scan.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/kphis/opd_er_document_scan.sql")).execute(&tester.db_pool).await.unwrap();

        let success = update_scan_history(&ScanHistory::demo(),&None,"user",&tester.db_pool,&tester.kphis).await.unwrap();
        assert_eq!(success.rows_affected(), 1);
        let again_fail_version = update_scan_history(&ScanHistory::demo(),&None,"user",&tester.db_pool,&tester.kphis).await.unwrap();
        assert_eq!(again_fail_version.rows_affected(), 0);
    }

    #[tokio::test]
    #[ignore]
    async fn sqlx_update_ft_history() {
        let tester = MySqlTester::new_kphis().await;
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis/opd_er_set_fast_track.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/kphis/opd_er_set_fast_track.sql")).execute(&tester.db_pool).await.unwrap();

        let success = update_ft_history(&SetFtHistory::demo(),&None,"user",&tester.db_pool,&tester.kphis).await.unwrap();
        assert_eq!(success.rows_affected(), 1);
        let again_fail_version = update_ft_history(&SetFtHistory::demo(),&None,"user",&tester.db_pool,&tester.kphis).await.unwrap();
        assert_eq!(again_fail_version.rows_affected(), 0);
    }

    #[tokio::test]
    #[ignore]
    async fn sqlx_delete_allergy_history() {
        let tester = MySqlTester::new_kphis().await;
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis/opd_er_allergy_history.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/kphis/opd_er_allergy_history.sql")).execute(&tester.db_pool).await.unwrap();

        let success = delete_allergy_history(1, 1, &tester.db_pool, &tester.kphis).await.unwrap();
        assert_eq!(success.rows_affected(), 1);
        let again_not_found = delete_allergy_history(1, 1, &tester.db_pool, &tester.kphis).await.unwrap();
        assert_eq!(again_not_found.rows_affected(), 0);
    }

    #[tokio::test]
    #[ignore]
    async fn sqlx_delete_consult_history() {
        let tester = MySqlTester::new_kphis().await;
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis/opd_er_consult.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/kphis/opd_er_consult.sql")).execute(&tester.db_pool).await.unwrap();

        let success = delete_consult_history(1, 1, &tester.db_pool, &tester.kphis).await.unwrap();
        assert_eq!(success.rows_affected(), 1);
        let again_not_found = delete_consult_history(1, 1, &tester.db_pool, &tester.kphis).await.unwrap();
        assert_eq!(again_not_found.rows_affected(), 0);
    }
}
