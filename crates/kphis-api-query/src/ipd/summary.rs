use sqlx::{AssertSqlSafe, FromRow, MySql, Pool, Row, mysql::MySqlQueryResult};

use kphis_model::{
    fetch::ExecuteResponse,
    ipd::summary::{
        DchData, DoctorData, DxData, LabAlertData, Summary, SummaryCodeSave, SummaryData, SummaryDataSave, SummaryNote, SummaryNoteSave, SummaryParams, SummarySave, SummaryStatus, XRayData,
    },
};
use kphis_sql::ipd::summary;
use kphis_util::{
    error::{AppError, Source},
    util::zero_none,
};

// ipd-summary-2-data.php
// ipd-summary-2-hosxp-ct-mri-data.php
// ipd-summary-2-hosxp-ipt-data.php
// ipd-summary-2-hosxp-or-data.php
// ipd-summary-2-lab-data.php
// ipd-summary-2-problem-list-data.php
// GET /ipd/summary
#[allow(clippy::too_many_arguments)]
pub async fn get_ipd_summary(
    params: &SummaryParams,
    doctorcode: &Option<String>,
    groupname: &Option<String>,
    alerts: &[(String, Vec<u64>, String)],
    operation_success: &[u64],
    pool: &Pool<MySql>,
    hosxp: &str,
    kphis: &str,
) -> Result<Summary, AppError> {
    // 1.get summary: Option<SummaryData>
    let summary = get_summary_data(params, pool, hosxp, kphis).await?;

    // 2. get summary's associated values
    let (dx_data, doctor_data) = get_dx_and_doctor_data(&summary.as_ref().map(|s| s.summary_id), pool, hosxp, kphis).await?;

    // 3. get an's associated data
    let (xray_data, dch_data, or_data, lab_alert_data, problem_list_data) = if let Some(an) = params.an.as_ref() {
        // 3.1 get xray_data: Vec<XRayData>, // xray_items_group 3,4
        let xray_result = select_xray_with_groups(an, &[3, 4], pool, hosxp).await?;
        // 3.2 get dch_data: Option<DchData>,
        let dch_result = select_hosxp_dch(an, pool, hosxp).await?;
        // 3.3 get or_data: Vec<OrData>,
        let or_result = super::his::get_operation_admit(an, operation_success, pool, hosxp).await?;
        // 3.4 get lab_alert_data: Vec<LabAlertData>,
        let lab_alert_result = if !alerts.is_empty() {
            select_lab_alert(an, alerts, doctorcode, groupname, pool, hosxp).await?
        } else {
            Vec::new()
        };
        // 3.5 get problem_list_data: Vec<String>,
        let problem_list_result = select_problem_list(an, pool, kphis).await?;
        (xray_result, dch_result, or_result, lab_alert_result, problem_list_result)
    } else {
        (Vec::new(), None, Vec::new(), Vec::new(), Vec::new())
    };

    Ok(Summary {
        summary,
        dx_data,
        doctor_data,
        xray_data,
        dch_data,
        or_data,
        lab_alert_data,
        problem_list_data,
    })
}

pub async fn get_summary_data(params: &SummaryParams, pool: &Pool<MySql>, hosxp: &str, kphis: &str) -> Result<Option<SummaryData>, AppError> {
    let summary_sql = summary::select_summary2_by(params.summary_id.is_some(), hosxp, kphis);
    let mut summary_query = sqlx::query(AssertSqlSafe(summary_sql));
    let mut valid_params = true;
    if let Some(summary_id) = params.summary_id.as_ref() {
        summary_query = summary_query.bind(summary_id);
    } else if let Some(an) = params.an.as_ref() {
        summary_query = summary_query.bind(an);
    } else {
        valid_params = false;
    }
    let result = if valid_params {
        summary_query
            .fetch_optional(pool)
            .await
            .map_err(|e| Source::SQLx.to_error(500, e, "Select IpdSummary"))?
            .as_ref()
            .map(SummaryData::from_row)
            .transpose()
            .map_err(|e| Source::SQLx.to_error(500, e, "Select IpdSummary"))?
    } else {
        None
    };

    Ok(result)
}

pub async fn get_summary_status(summary_id: u32, pool: &Pool<MySql>, kphis: &str) -> Result<Option<SummaryStatus>, AppError> {
    let sql = summary::select_summary2_status(kphis);
    sqlx::query(AssertSqlSafe(sql))
        .bind(summary_id)
        .fetch_optional(pool)
        .await
        .map_err(|e| Source::SQLx.to_error(500, e, "Select IpdSummaryStatus"))?
        .as_ref()
        .map(SummaryStatus::from_row)
        .transpose()
        .map_err(|e| Source::SQLx.to_error(500, e, "Select IpdSummaryStatus"))
}

pub async fn get_dx_and_doctor_data(summary_id: &Option<u32>, pool: &Pool<MySql>, hosxp: &str, kphis: &str) -> Result<(Vec<DxData>, Vec<DoctorData>), AppError> {
    let (dx_data, doctor_data) = if let Some(summary_id) = summary_id {
        // 2.1 get dx_data: Vec<DxData>, // ty 2-5
        let dx_result = select_dx2_5(*summary_id, pool, kphis).await?;
        // 2.2 get doctor_data: Vec<DoctorData>, // ty 1,2
        let doctor_result = select_attend_and_approve_doctor(*summary_id, pool, hosxp, kphis).await?;
        (dx_result, doctor_result)
    } else {
        (Vec::new(), Vec::new())
    };

    Ok((dx_data, doctor_data))
}

async fn select_dx2_5(summary_id: u32, pool: &Pool<MySql>, kphis: &str) -> Result<Vec<DxData>, AppError> {
    let dx_sql = summary::select_dx2_5(kphis);
    sqlx::query(AssertSqlSafe(dx_sql))
        .bind(summary_id)
        .bind(summary_id)
        .bind(summary_id)
        .bind(summary_id)
        .fetch_all(pool)
        .await
        .map_err(|e| Source::SQLx.to_error(500, e, "Select IpdSummaryDx"))?
        .iter()
        .map(DxData::from_row)
        .collect::<sqlx::Result<Vec<DxData>>>()
        .map_err(|e| Source::SQLx.to_error(500, e, "Select IpdSummaryDx"))
}

async fn select_attend_and_approve_doctor(summary_id: u32, pool: &Pool<MySql>, hosxp: &str, kphis: &str) -> Result<Vec<DoctorData>, AppError> {
    let doctor_sql = summary::select_attend_and_approve_doctor(hosxp, kphis);
    sqlx::query(AssertSqlSafe(doctor_sql))
        .bind(summary_id)
        .bind(summary_id)
        .fetch_all(pool)
        .await
        .map_err(|e| Source::SQLx.to_error(500, e, "Select IpdSummaryDoctorData"))?
        .iter()
        .map(DoctorData::from_row)
        .collect::<sqlx::Result<Vec<DoctorData>>>()
        .map_err(|e| Source::SQLx.to_error(500, e, "Select IpdSummaryDoctorData"))
}

pub async fn select_xray_with_groups(an: &str, groups: &[i32], pool: &Pool<MySql>, hosxp: &str) -> Result<Vec<XRayData>, AppError> {
    let xray_sql = summary::select_xray_with_groups(groups, hosxp);
    sqlx::query(AssertSqlSafe(xray_sql))
        .bind(an)
        .fetch_all(pool)
        .await
        .map_err(|e| Source::SQLx.to_error(500, e, "Select IpdSummaryXRayData"))?
        .iter()
        .map(XRayData::from_row)
        .collect::<sqlx::Result<Vec<XRayData>>>()
        .map_err(|e| Source::SQLx.to_error(500, e, "Select IpdSummaryXRayData"))
}

async fn select_hosxp_dch(an: &str, pool: &Pool<MySql>, hosxp: &str) -> Result<Option<DchData>, AppError> {
    let dch_sql = summary::select_hosxp_dch(hosxp);
    sqlx::query(AssertSqlSafe(dch_sql))
        .bind(an)
        .fetch_optional(pool)
        .await
        .map_err(|e| Source::SQLx.to_error(500, e, "Select IpdSummaryDchData"))?
        .as_ref()
        .map(DchData::from_row)
        .transpose()
        .map_err(|e| Source::SQLx.to_error(500, e, "Select IpdSummaryDchData"))
}

async fn select_lab_alert(
    an: &str,
    alerts: &[(String, Vec<u64>, String)],
    doctorcode: &Option<String>,
    groupname: &Option<String>,
    pool: &Pool<MySql>,
    hosxp: &str,
) -> Result<Vec<LabAlertData>, AppError> {
    let lab_alert_sql = summary::select_lab_alert(alerts, hosxp);
    sqlx::query(AssertSqlSafe(lab_alert_sql))
        .bind(doctorcode)
        .bind(groupname)
        .bind(an)
        .fetch_all(pool)
        .await
        .map_err(|e| Source::SQLx.to_error(500, e, "Select IpdSummaryLabAlertData"))?
        .iter()
        .map(LabAlertData::from_row)
        .collect::<sqlx::Result<Vec<LabAlertData>>>()
        .map_err(|e| Source::SQLx.to_error(500, e, "Select IpdSummaryLabAlertData"))
}

async fn select_problem_list(an: &str, pool: &Pool<MySql>, kphis: &str) -> Result<Vec<String>, AppError> {
    let problem_list_sql = summary::select_problem_list(kphis);
    sqlx::query(AssertSqlSafe(problem_list_sql))
        .bind(an)
        .fetch_all(pool)
        .await
        .map_err(|e| Source::SQLx.to_error(500, e, "Select IpdSummaryProblemListData"))?
        .iter()
        .filter_map(|r| r.try_get("progress_note_item_detail").transpose())
        .collect::<sqlx::Result<Vec<String>>>()
        .map_err(|e| Source::SQLx.to_error(500, e, "Select IpdSummaryProblemListData"))
}

// // ipd-summary-2-save.php
// POST /ipd/summary
pub async fn post_ipd_summary(save: &SummarySave, doctorcode: &Option<String>, user: &str, is_doctor: bool, pool: &Pool<MySql>, kphis: &str) -> Result<(u32, Vec<ExecuteResponse>), AppError> {
    let id;
    let mut results = Vec::with_capacity(11);
    // 1. insert/update summary
    if let Some(summary_id) = zero_none(save.summary.summary_id) {
        id = summary_id;
        // 1.1 update
        let update_result = update_summary2(id, is_doctor, &save.summary, user, pool, kphis).await?;
        results.push(ExecuteResponse::from_query_result(update_result, "Update IpdSummary"));
        if is_doctor {
            // 1.1.1 delete dx 2-4
            let delete_dx2_result = delete_pre_admission_comorbidity(id, pool, kphis).await?;
            results.push(ExecuteResponse::from_query_result(delete_dx2_result, "Delete IpdSummaryDx2"));

            let delete_dx3_result = delete_post_admission_comorbidity(id, pool, kphis).await?;
            results.push(ExecuteResponse::from_query_result(delete_dx3_result, "Delete IpdSummaryDx3"));

            let delete_dx4_result = delete_other_diagnosis(id, pool, kphis).await?;
            results.push(ExecuteResponse::from_query_result(delete_dx4_result, "Delete IpdSummaryDx4"));
        }
        // 1.1.2 delete dx 5
        let delete_dx5_result = delete_external_cause(id, pool, kphis).await?;
        results.push(ExecuteResponse::from_query_result(delete_dx5_result, "Delete IpdSummaryDx5"));
    } else {
        // 1.2 insert
        let insert_result = insert_summary2(is_doctor, save, user, pool, kphis).await?;
        id = insert_result.last_insert_id() as u32;
        results.push(ExecuteResponse::from_query_result(insert_result, "Insert IpdSummary"));
    }

    // 2. insert attending/approve doctor
    if let Some(doctor) = doctorcode {
        if save.attending_doctor {
            let insert_attending_result = insert_attending_doctor(id, doctor, user, pool, kphis).await?;
            results.push(ExecuteResponse::from_query_result(insert_attending_result, "Insert IpdSummaryAttendingDoctor"));
        }
        if save.approve_doctor {
            let insert_approve_result = insert_approve_doctor(id, doctor, user, pool, kphis).await?;
            results.push(ExecuteResponse::from_query_result(insert_approve_result, "Insert IpdSummaryApproveDoctor"));
        }
    }

    // 3. insert dx 2-4
    if is_doctor {
        if !save.dx2_data.is_empty() {
            let insert_dx2_result = insert_pre_admission_comorbidity(id, &save.dx2_data, user, pool, kphis).await?;
            results.push(ExecuteResponse::from_query_result(insert_dx2_result, "Insert IpdSummaryDx2"));
        }
        if !save.dx3_data.is_empty() {
            let insert_dx3_result = insert_post_admission_comorbidity(id, &save.dx3_data, user, pool, kphis).await?;
            results.push(ExecuteResponse::from_query_result(insert_dx3_result, "Insert IpdSummaryDx3"));
        }
        if !save.dx4_data.is_empty() {
            let insert_dx4_result = insert_other_diagnosis(id, &save.dx4_data, user, pool, kphis).await?;
            results.push(ExecuteResponse::from_query_result(insert_dx4_result, "Insert IpdSummaryDx4"));
        }
    }

    // 4. insert dx 5
    if !save.dx5_data.is_empty() {
        let insert_dx5_result = insert_external_cause(id, &save.dx5_data, user, pool, kphis).await?;
        results.push(ExecuteResponse::from_query_result(insert_dx5_result, "Insert IpdSummaryDx5"));
    }

    Ok((id, results))
}

async fn update_summary2(summary_id: u32, is_doctor: bool, save: &SummaryDataSave, user: &str, pool: &Pool<MySql>, kphis: &str) -> Result<MySqlQueryResult, AppError> {
    let update_sql = summary::update_summary2(is_doctor, kphis);
    let mut update_query = sqlx::query(AssertSqlSafe(update_sql));
    if is_doctor {
        update_query = update_query.bind(&save.principal_diagnosis).bind(&save.principal_diagnosis_icd10);
    }
    update_query
        .bind(&save.operating_room)
        .bind(&save.tracheostomy)
        .bind(&save.mechanical_ventilation)
        .bind(&save.packed_redcells)
        .bind(&save.fresh_frozen_plasma)
        .bind(&save.platelets)
        .bind(&save.cryoprecipitate)
        .bind(&save.whole_blood)
        .bind(&save.computer_tomography)
        .bind(&save.computer_tomography_text)
        .bind(&save.chemotherapy)
        .bind(&save.mri)
        .bind(&save.mri_text)
        .bind(&save.hemodialysis)
        .bind(&save.non_or_other)
        .bind(&save.non_or_other_text)
        .bind(&save.special_other)
        .bind(&save.special_other_text)
        .bind(&save.discharge_status)
        .bind(&save.discharge_type)
        .bind(&save.hospital_refer)
        .bind(&save.status)
        .bind(user)
        .bind(summary_id)
        .bind(&save.an)
        .execute(pool)
        .await
        .map_err(|e| Source::SQLx.to_error(500, e, "Update IpdSummary"))
}

pub async fn update_summary2_code(save: &SummaryCodeSave, user: &str, pool: &Pool<MySql>, kphis: &str) -> Result<MySqlQueryResult, AppError> {
    let update_sql = summary::update_summary2_code(kphis);
    sqlx::query(AssertSqlSafe(update_sql))
        .bind(&save.coder_name)
        .bind(&save.principal_diagnosis_code)
        .bind(&save.pre_admission_comorbidity_codes)
        .bind(&save.post_admission_comorbidity_codes)
        .bind(&save.other_diagnosis_codes)
        .bind(&save.external_cause_codes)
        .bind(&save.main_procedure_code)
        .bind(&save.other_procedure_codes)
        .bind(&save.status)
        .bind(user)
        .bind(save.summary_id)
        .bind(&save.an)
        .execute(pool)
        .await
        .map_err(|e| Source::SQLx.to_error(500, e, "Update IpdSummaryCode"))
}

pub async fn update_summary2_status(status: &Option<String>, summary_id: u32, user: &str, pool: &Pool<MySql>, kphis: &str) -> Result<MySqlQueryResult, AppError> {
    let update_sql = summary::update_summary2_status(kphis);
    sqlx::query(AssertSqlSafe(update_sql))
        .bind(status)
        .bind(user)
        .bind(summary_id)
        .execute(pool)
        .await
        .map_err(|e| Source::SQLx.to_error(500, e, "Update IpdSummaryStatus"))
}

async fn delete_pre_admission_comorbidity(summary_id: u32, pool: &Pool<MySql>, kphis: &str) -> Result<MySqlQueryResult, AppError> {
    let delete_dx2_sql = summary::delete_pre_admission_comorbidity(kphis);
    sqlx::query(AssertSqlSafe(delete_dx2_sql))
        .bind(summary_id)
        .execute(pool)
        .await
        .map_err(|e| Source::SQLx.to_error(500, e, "Delete IpdSummaryDx2"))
}

async fn delete_post_admission_comorbidity(summary_id: u32, pool: &Pool<MySql>, kphis: &str) -> Result<MySqlQueryResult, AppError> {
    let delete_dx3_sql = summary::delete_post_admission_comorbidity(kphis);
    sqlx::query(AssertSqlSafe(delete_dx3_sql))
        .bind(summary_id)
        .execute(pool)
        .await
        .map_err(|e| Source::SQLx.to_error(500, e, "Delete IpdSummaryDx3"))
}

async fn delete_other_diagnosis(summary_id: u32, pool: &Pool<MySql>, kphis: &str) -> Result<MySqlQueryResult, AppError> {
    let delete_dx4_sql = summary::delete_other_diagnosis(kphis);
    sqlx::query(AssertSqlSafe(delete_dx4_sql))
        .bind(summary_id)
        .execute(pool)
        .await
        .map_err(|e| Source::SQLx.to_error(500, e, "Delete IpdSummaryDx4"))
}

async fn delete_external_cause(summary_id: u32, pool: &Pool<MySql>, kphis: &str) -> Result<MySqlQueryResult, AppError> {
    let delete_dx5_sql = summary::delete_external_cause(kphis);
    sqlx::query(AssertSqlSafe(delete_dx5_sql))
        .bind(summary_id)
        .execute(pool)
        .await
        .map_err(|e| Source::SQLx.to_error(500, e, "Delete IpdSummaryDx5"))
}

async fn insert_summary2(is_doctor: bool, save: &SummarySave, user: &str, pool: &Pool<MySql>, kphis: &str) -> Result<MySqlQueryResult, AppError> {
    let insert_sql = summary::insert_summary2(is_doctor, kphis);
    let mut insert_query = sqlx::query(AssertSqlSafe(insert_sql));
    if is_doctor {
        insert_query = insert_query.bind(&save.summary.principal_diagnosis).bind(&save.summary.principal_diagnosis_icd10);
    }
    insert_query
        .bind(&save.summary.operating_room)
        .bind(&save.summary.tracheostomy)
        .bind(&save.summary.mechanical_ventilation)
        .bind(&save.summary.packed_redcells)
        .bind(&save.summary.fresh_frozen_plasma)
        .bind(&save.summary.platelets)
        .bind(&save.summary.cryoprecipitate)
        .bind(&save.summary.whole_blood)
        .bind(&save.summary.computer_tomography)
        .bind(&save.summary.computer_tomography_text)
        .bind(&save.summary.chemotherapy)
        .bind(&save.summary.mri)
        .bind(&save.summary.mri_text)
        .bind(&save.summary.hemodialysis)
        .bind(&save.summary.non_or_other)
        .bind(&save.summary.non_or_other_text)
        .bind(&save.summary.special_other)
        .bind(&save.summary.special_other_text)
        .bind(&save.summary.discharge_status)
        .bind(&save.summary.discharge_type)
        .bind(&save.summary.hospital_refer)
        .bind(&save.summary.status)
        .bind(&save.summary.an)
        .bind(user)
        .bind(user)
        .execute(pool)
        .await
        .map_err(|e| {
            if let sqlx::error::Error::Database(err) = &e
                && err.code().map(|c| c == "23000").unwrap_or_default()
            {
                AppError::app_403_duplicate("Insert IpdSummary")
            } else {
                Source::SQLx.to_error(500, e, "Insert IpdSummary")
            }
        })
}

async fn insert_attending_doctor(summary_id: u32, doctorcode: &str, user: &str, pool: &Pool<MySql>, kphis: &str) -> Result<MySqlQueryResult, AppError> {
    let insert_attending_sql = summary::insert_attending_doctor(kphis);
    sqlx::query(AssertSqlSafe(insert_attending_sql))
        .bind(summary_id)
        .bind(doctorcode)
        .bind(user)
        .bind(user)
        .execute(pool)
        .await
        .map_err(|e| {
            if let sqlx::error::Error::Database(err) = &e
                && err.code().map(|c| c == "23000").unwrap_or_default()
            {
                AppError::app_403_duplicate("Insert IpdSummaryAttendingDoctor")
            } else {
                Source::SQLx.to_error(500, e, "Insert IpdSummaryAttendingDoctor")
            }
        })
}

async fn insert_approve_doctor(summary_id: u32, doctorcode: &str, user: &str, pool: &Pool<MySql>, kphis: &str) -> Result<MySqlQueryResult, AppError> {
    let insert_approve_sql = summary::insert_approve_doctor(kphis);
    sqlx::query(AssertSqlSafe(insert_approve_sql))
        .bind(summary_id)
        .bind(doctorcode)
        .bind(user)
        .bind(user)
        .execute(pool)
        .await
        .map_err(|e| {
            if let sqlx::error::Error::Database(err) = &e
                && err.code().map(|c| c == "23000").unwrap_or_default()
            {
                AppError::app_403_duplicate("Insert IpdSummaryApproveDoctor")
            } else {
                Source::SQLx.to_error(500, e, "Insert IpdSummaryApproveDoctor")
            }
        })
}

async fn insert_pre_admission_comorbidity(summary_id: u32, dx_data: &[DxData], user: &str, pool: &Pool<MySql>, kphis: &str) -> Result<MySqlQueryResult, AppError> {
    let insert_dx2_sql = summary::insert_pre_admission_comorbidity(dx_data.len(), kphis);
    let mut insert_dx2_query = sqlx::query(AssertSqlSafe(insert_dx2_sql));
    for dx2 in dx_data.iter() {
        insert_dx2_query = insert_dx2_query.bind(summary_id).bind(&dx2.detail).bind(&dx2.icd).bind(user).bind(user);
    }
    insert_dx2_query.execute(pool).await.map_err(|e| Source::SQLx.to_error(500, e, "Insert IpdSummaryDx2"))
}

async fn insert_post_admission_comorbidity(summary_id: u32, dx_data: &[DxData], user: &str, pool: &Pool<MySql>, kphis: &str) -> Result<MySqlQueryResult, AppError> {
    let insert_dx3_sql = summary::insert_post_admission_comorbidity(dx_data.len(), kphis);
    let mut insert_dx3_query = sqlx::query(AssertSqlSafe(insert_dx3_sql));
    for dx3 in dx_data.iter() {
        insert_dx3_query = insert_dx3_query.bind(summary_id).bind(&dx3.detail).bind(&dx3.icd).bind(user).bind(user);
    }
    insert_dx3_query.execute(pool).await.map_err(|e| Source::SQLx.to_error(500, e, "Insert IpdSummaryDx3"))
}

async fn insert_other_diagnosis(summary_id: u32, dx_data: &[DxData], user: &str, pool: &Pool<MySql>, kphis: &str) -> Result<MySqlQueryResult, AppError> {
    let insert_dx4_sql = summary::insert_other_diagnosis(dx_data.len(), kphis);
    let mut insert_dx4_query = sqlx::query(AssertSqlSafe(insert_dx4_sql));
    for dx4 in dx_data.iter() {
        insert_dx4_query = insert_dx4_query.bind(summary_id).bind(&dx4.detail).bind(&dx4.icd).bind(user).bind(user);
    }
    insert_dx4_query.execute(pool).await.map_err(|e| Source::SQLx.to_error(500, e, "Insert IpdSummaryDx4"))
}

async fn insert_external_cause(summary_id: u32, dx_data: &[DxData], user: &str, pool: &Pool<MySql>, kphis: &str) -> Result<MySqlQueryResult, AppError> {
    let insert_dx5_sql = summary::insert_external_cause(dx_data.len(), kphis);
    let mut insert_dx5_query = sqlx::query(AssertSqlSafe(insert_dx5_sql));
    for dx5 in dx_data.iter() {
        insert_dx5_query = insert_dx5_query.bind(summary_id).bind(&dx5.detail).bind(&dx5.icd).bind(user).bind(user);
    }
    insert_dx5_query.execute(pool).await.map_err(|e| Source::SQLx.to_error(500, e, "Insert IpdSummaryDx5"))
}

pub async fn select_summary_note(summary_id: u32, pool: &Pool<MySql>, hosxp: &str, kphis: &str) -> Result<Vec<SummaryNote>, AppError> {
    let note_sql = summary::select_summary_note(hosxp, kphis);
    sqlx::query(AssertSqlSafe(note_sql))
        .bind(summary_id)
        .fetch_all(pool)
        .await
        .map_err(|e| Source::SQLx.to_error(500, e, "Select IpdSummaryNote"))?
        .iter()
        .map(SummaryNote::from_row)
        .collect::<sqlx::Result<Vec<SummaryNote>>>()
        .map_err(|e| Source::SQLx.to_error(500, e, "Select IpdSummaryNote"))
}

pub async fn insert_summary_note(summary_id: u32, save: &SummaryNoteSave, doctorcode: &Option<String>, user: &str, pool: &Pool<MySql>, kphis: &str) -> Result<MySqlQueryResult, AppError> {
    let insert_note_sql = summary::insert_summary_note(kphis);
    sqlx::query(AssertSqlSafe(insert_note_sql))
        .bind(summary_id)
        .bind(&save.note)
        .bind(doctorcode)
        .bind(user)
        .bind(user)
        .execute(pool)
        .await
        .map_err(|e| Source::SQLx.to_error(500, e, "Insert IpdSummaryNote"))
}

pub async fn update_summary_note(summary_id: u32, save: &SummaryNoteSave, doctorcode: &Option<String>, user: &str, pool: &Pool<MySql>, kphis: &str) -> Result<MySqlQueryResult, AppError> {
    let update_note_sql = summary::update_summary_note(kphis);
    sqlx::query(AssertSqlSafe(update_note_sql))
        .bind(&save.note)
        .bind(user)
        .bind(save.note_id)
        .bind(summary_id)
        .bind(doctorcode)
        .execute(pool)
        .await
        .map_err(|e| Source::SQLx.to_error(500, e, "Update IpdSummaryNote"))
}

pub async fn delete_summary_note(summary_id: u32, save: &SummaryNoteSave, doctorcode: &Option<String>, pool: &Pool<MySql>, kphis: &str) -> Result<MySqlQueryResult, AppError> {
    let delete_note_sql = summary::delete_summary_note(kphis);
    sqlx::query(AssertSqlSafe(delete_note_sql))
        .bind(save.note_id)
        .bind(summary_id)
        .bind(doctorcode)
        .execute(pool)
        .await
        .map_err(|e| Source::SQLx.to_error(500, e, "Delete IpdSummaryNote"))
}

#[cfg(test)]
#[rustfmt::skip]
mod tests {

    use super::*;
    use kphis_sqlx_tester::MySqlTester;

    #[tokio::test]
    #[ignore]
    async fn sqlx_get_summary_data() {
        let tester = MySqlTester::new_hosxp_and_kphis().await;
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis/ipd_summary_2.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/hosxp/hospcode.sql")).execute(&tester.db_pool).await.unwrap();

        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/kphis/ipd_summary_2.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/hosxp/hospcode.sql")).execute(&tester.db_pool).await.unwrap();

        // MUST have 'summary_id' or 'an' in params
        let default = get_summary_data(&SummaryParams::default(),&tester.db_pool,&tester.hosxp,&tester.kphis).await.unwrap();
        assert!(default.is_none());
        let found_summary_id = get_summary_data(&SummaryParams {summary_id: Some(1),..Default::default()},&tester.db_pool,&tester.hosxp,&tester.kphis).await.unwrap();
        assert!(found_summary_id.is_some());
        let found_an = get_summary_data(&SummaryParams {an: Some(String::from("660001234")),..Default::default()},&tester.db_pool,&tester.hosxp,&tester.kphis).await.unwrap();
        assert!(found_an.is_some());
        let not_found = get_summary_data(&SummaryParams {an: Some(String::from("1234")),..Default::default()},&tester.db_pool,&tester.hosxp,&tester.kphis).await.unwrap();
        assert!(not_found.is_none());
    }

    #[tokio::test]
    #[ignore]
    async fn sqlx_get_summary_status() {
        let tester = MySqlTester::new_kphis().await;
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis/ipd_summary_2.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/kphis/ipd_summary_2.sql")).execute(&tester.db_pool).await.unwrap();

        let found_summary_id = get_summary_status(1,&tester.db_pool,&tester.kphis).await.unwrap();
        assert!(found_summary_id.is_some());
        let not_found = get_summary_status(99,&tester.db_pool,&tester.kphis).await.unwrap();
        assert!(not_found.is_none());
    }

    #[tokio::test]
    #[ignore]
    async fn sqlx_select_xray_with_groups() {
        let tester = MySqlTester::new_hosxp().await;
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/hosxp/xray_report.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/hosxp/xray_items.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/hosxp/ipt.sql")).execute(&tester.db_pool).await.unwrap();

        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/hosxp/xray_report.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/hosxp/xray_items.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/hosxp/ipt.sql")).execute(&tester.db_pool).await.unwrap();

        let found = select_xray_with_groups("660001234", &[3, 4], &tester.db_pool, &tester.hosxp).await.unwrap();
        assert_eq!(found.len(), 2);
        let not_found = select_xray_with_groups("660006666", &[3, 4], &tester.db_pool, &tester.hosxp).await.unwrap();
        assert!(not_found.is_empty());
    }

    #[tokio::test]
    #[ignore]
    async fn sqlx_select_hosxp_dch() {
        let tester = MySqlTester::new_hosxp().await;
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/hosxp/ipt.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/hosxp/dchstts.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/hosxp/dchtype.sql")).execute(&tester.db_pool).await.unwrap();

        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/hosxp/ipt.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/hosxp/dchstts.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/hosxp/dchtype.sql")).execute(&tester.db_pool).await.unwrap();

        let found = select_hosxp_dch("660001234", &tester.db_pool, &tester.hosxp).await.unwrap();
        assert!(found.is_some());
        let not_found = select_hosxp_dch("660006666", &tester.db_pool, &tester.hosxp).await.unwrap();
        assert!(not_found.is_none());
    }

    #[tokio::test]
    #[ignore]
    async fn sqlx_select_lab_alert() {
        let tester = MySqlTester::new_hosxp().await;
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/hosxp/lab_head.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/hosxp/lab_order.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/hosxp/lab_items.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/hosxp/lab_items_group.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/hosxp/lab_items_doctor.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/hosxp/lab_items_visible.sql")).execute(&tester.db_pool).await.unwrap();

        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/hosxp/lab_head.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/hosxp/lab_order.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/hosxp/lab_items.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/hosxp/lab_items_group.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/hosxp/lab_items_doctor.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/hosxp/lab_items_visible.sql")).execute(&tester.db_pool).await.unwrap();

        let alerts = [(String::from("Cr > 1.1 mg/dL"),vec![78],String::from("@ > 1.1"))];

        let default = select_lab_alert("660001234",&alerts,&None,&None,&tester.db_pool,&tester.hosxp).await.unwrap();
        assert_eq!(default.len(), 1);
        assert_eq!(default.first().and_then(|lab| lab.lab_order_result.clone()).unwrap_or_default(),String::from("[[ปกปิด]]"));
        let found = select_lab_alert("660001234",&alerts,&Some(String::from("007")),&Some(String::from("BIOCHEMISTRY")),&tester.db_pool,&tester.hosxp).await.unwrap();
        assert_eq!(found.len(), 1);
        assert_eq!(found.first().and_then(|lab| lab.lab_order_result.clone()).unwrap_or_default(),String::from("1.2"));

        // lab_items_visible has 'item code matched' + 'group NOT match' will NOT mark result as '[[ปกปิด]]'
        let found_visible_masked = select_lab_alert("660001234",&alerts,&Some(String::from("007")),&Some(String::from("HEMATOLOGY")),&tester.db_pool,&tester.hosxp).await.unwrap();
        assert_eq!(found_visible_masked.len(), 1);
        assert_eq!(found_visible_masked.first().and_then(|lab| lab.lab_order_result.clone()).unwrap_or_default(),String::from("[[ปกปิด]]"));

        // lab_items_doctor has 'item code matched' + 'doctorcode NOT match' will mark result as '[[ปกปิด]]'
        let alerts = [(String::from("eFGR < 30"),vec![364],String::from("@ < 30.0"))];
        let found_doctor_not_masked = select_lab_alert("660001234",&alerts,&Some(String::from("009")),&Some(String::from("BIOCHEMISTRY")),&tester.db_pool,&tester.hosxp).await.unwrap();
        assert_eq!(found_doctor_not_masked.len(), 1);
        assert_eq!(found_doctor_not_masked.first().and_then(|lab| lab.lab_order_result.clone()).unwrap_or_default(),String::from("22.22"));
        let found_doctor_masked = select_lab_alert("660001234",&alerts,&Some(String::from("007")),&Some(String::from("BIOCHEMISTRY")),&tester.db_pool,&tester.hosxp).await.unwrap();
        assert_eq!(found_doctor_masked.len(), 1);
        assert_eq!(found_doctor_masked.first().and_then(|lab| lab.lab_order_result.clone()).unwrap_or_default(),String::from("[[ปกปิด]]"));
    }

    #[tokio::test]
    #[ignore]
    async fn sqlx_select_problem_list() {
        let tester = MySqlTester::new_kphis().await;
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis/ipd_progress_note.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis/ipd_progress_note_item.sql")).execute(&tester.db_pool).await.unwrap();

        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/kphis/ipd_progress_note.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/kphis/ipd_progress_note_item.sql")).execute(&tester.db_pool).await.unwrap();

        let found = select_problem_list("660001234", &tester.db_pool, &tester.kphis).await.unwrap();
        assert_eq!(found.len(), 1);
        let not_found = select_problem_list("660006666", &tester.db_pool, &tester.kphis).await.unwrap();
        assert!(not_found.is_empty());
    }

    #[tokio::test]
    #[ignore]
    async fn sqlx_select_dx2_5() {
        let tester = MySqlTester::new_kphis().await;
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis/ipd_summary_pre_admission_comorbidity.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis/ipd_summary_post_admission_comorbidity.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis/ipd_summary_other_diagnosis.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis/ipd_summary_external_cause.sql")).execute(&tester.db_pool).await.unwrap();

        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/kphis/ipd_summary_pre_admission_comorbidity.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/kphis/ipd_summary_post_admission_comorbidity.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/kphis/ipd_summary_other_diagnosis.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/kphis/ipd_summary_external_cause.sql")).execute(&tester.db_pool).await.unwrap();

        let found = select_dx2_5(1, &tester.db_pool, &tester.kphis).await.unwrap();
        assert_eq!(found.len(), 4);
        let not_found = select_dx2_5(999, &tester.db_pool, &tester.kphis).await.unwrap();
        assert!(not_found.is_empty());
    }

    #[tokio::test]
    #[ignore]
    async fn sqlx_select_attend_and_approve_doctor() {
        let tester = MySqlTester::new_hosxp_and_kphis().await;
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis/ipd_summary_attending_doctor.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis/ipd_summary_approve_doctor.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/hosxp/doctor.sql")).execute(&tester.db_pool).await.unwrap();

        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/kphis/ipd_summary_attending_doctor.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/kphis/ipd_summary_approve_doctor.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/hosxp/doctor.sql")).execute(&tester.db_pool).await.unwrap();

        let found =select_attend_and_approve_doctor(1, &tester.db_pool, &tester.hosxp, &tester.kphis).await.unwrap();
        assert_eq!(found.len(), 2);
        let not_found =select_attend_and_approve_doctor(999, &tester.db_pool, &tester.hosxp, &tester.kphis).await.unwrap();
        assert!(not_found.is_empty());
    }

    #[tokio::test]
    #[ignore]
    async fn sqlx_insert_summary2() {
        let tester = MySqlTester::new_kphis().await;
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis/ipd_summary_2.sql")).execute(&tester.db_pool).await.unwrap();

        let success = insert_summary2(true,&SummarySave::demo(),"user",&tester.db_pool,&tester.kphis).await.unwrap();
        assert_eq!(success.rows_affected(), 1);
        let again_duplicate_an = insert_summary2(false,&SummarySave::demo(),"user",&tester.db_pool,&tester.kphis).await;
        if let Err(again_duplicate_an_error) = again_duplicate_an {
            assert_eq!(again_duplicate_an_error.status, 403);
        } else {
            panic!("MUST ERROR")
        }
    }

    #[tokio::test]
    #[ignore]
    async fn sqlx_insert_attending_doctor() {
        let tester = MySqlTester::new_kphis().await;
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis/ipd_summary_attending_doctor.sql")).execute(&tester.db_pool).await.unwrap();

        let success = insert_attending_doctor(1,"007","user",&tester.db_pool,&tester.kphis).await.unwrap();
        assert_eq!(success.rows_affected(), 1);
        // duplicate (summary_id + summary_attending_doctor) will error
        let again_duplicate = insert_attending_doctor(1,"007","user",&tester.db_pool,&tester.kphis).await;
        if let Err(again_duplicate_error) = again_duplicate {
            assert_eq!(again_duplicate_error.status, 403);
        } else {
            panic!("MUST ERROR")
        }
    }

    #[tokio::test]
    #[ignore]
    async fn sqlx_insert_approve_doctor() {
        let tester = MySqlTester::new_kphis().await;
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis/ipd_summary_approve_doctor.sql")).execute(&tester.db_pool).await.unwrap();

        let success = insert_approve_doctor(1,"007","user",&tester.db_pool,&tester.kphis).await.unwrap();
        assert_eq!(success.rows_affected(), 1);
        // duplicate (summary_id + summary_approve_doctor) will error
        let again_duplicate = insert_approve_doctor(1,"007","user",&tester.db_pool,&tester.kphis).await;
        if let Err(again_duplicate_error) = again_duplicate {
            assert_eq!(again_duplicate_error.status, 403);
        } else {
            panic!("MUST ERROR")
        }
    }

    #[tokio::test]
    #[ignore]
    async fn sqlx_insert_pre_admission_comorbidity() {
        let tester = MySqlTester::new_kphis().await;
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis/ipd_summary_pre_admission_comorbidity.sql")).execute(&tester.db_pool).await.unwrap();

        let success = insert_pre_admission_comorbidity(1,&[DxData::demo()],"user",&tester.db_pool,&tester.kphis).await.unwrap();
        assert_eq!(success.rows_affected(), 1);
        let again_success = insert_pre_admission_comorbidity(1,&[DxData::demo()],"user",&tester.db_pool,&tester.kphis).await.unwrap();
        assert_eq!(again_success.rows_affected(), 1);
    }

    #[tokio::test]
    #[ignore]
    async fn sqlx_insert_post_admission_comorbidity() {
        let tester = MySqlTester::new_kphis().await;
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis/ipd_summary_post_admission_comorbidity.sql")).execute(&tester.db_pool).await.unwrap();

        let success = insert_post_admission_comorbidity(1,&[DxData::demo()],"user",&tester.db_pool,&tester.kphis).await.unwrap();
        assert_eq!(success.rows_affected(), 1);
        let again_success = insert_post_admission_comorbidity(1,&[DxData::demo()],"user",&tester.db_pool,&tester.kphis).await.unwrap();
        assert_eq!(again_success.rows_affected(), 1);
    }

    #[tokio::test]
    #[ignore]
    async fn sqlx_insert_other_diagnosis() {
        let tester = MySqlTester::new_kphis().await;
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis/ipd_summary_other_diagnosis.sql")).execute(&tester.db_pool).await.unwrap();

        let success = insert_other_diagnosis(1, &[DxData::demo()], "user", &tester.db_pool, &tester.kphis).await.unwrap();
        assert_eq!(success.rows_affected(), 1);
        let again_success = insert_other_diagnosis(1, &[DxData::demo()], "user", &tester.db_pool, &tester.kphis).await.unwrap();
        assert_eq!(again_success.rows_affected(), 1);
    }

    #[tokio::test]
    #[ignore]
    async fn sqlx_insert_external_cause() {
        let tester = MySqlTester::new_kphis().await;
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis/ipd_summary_external_cause.sql")).execute(&tester.db_pool).await.unwrap();

        let success = insert_external_cause(1, &[DxData::demo()], "user", &tester.db_pool, &tester.kphis).await.unwrap();
        assert_eq!(success.rows_affected(), 1);
        let again_success = insert_external_cause(1, &[DxData::demo()], "user", &tester.db_pool, &tester.kphis).await.unwrap();
        assert_eq!(again_success.rows_affected(), 1);
    }

    #[tokio::test]
    #[ignore]
    async fn sqlx_update_summary2() {
        let tester = MySqlTester::new_kphis().await;
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis/ipd_summary_2.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/kphis/ipd_summary_2.sql")).execute(&tester.db_pool).await.unwrap();

        let mut save = SummaryDataSave::demo();
        let success = update_summary2(1,true,&save,"user",&tester.db_pool,&tester.kphis).await.unwrap();
        assert_eq!(success.rows_affected(), 1);

        // now summary_id=1 has statis=Some(review)
        save.status = None;
        let again_success = update_summary2(1,false,&save,"user",&tester.db_pool,&tester.kphis).await.unwrap();
        assert_eq!(again_success.rows_affected(), 1);

        // now summary_id=1 has statis=None
        save.status = Some(String::from("code"));
        let status_none = update_summary2(1,true,&save,"user",&tester.db_pool,&tester.kphis).await.unwrap();
        assert_eq!(status_none.rows_affected(), 1);

        // now summary_id=1 has statis=Some(code)
        save.status = Some(String::from("audit"));
        let status_none = update_summary2(1,true,&save,"user",&tester.db_pool,&tester.kphis).await.unwrap();
        assert_eq!(status_none.rows_affected(), 1);

        // now summary_id=1 has statis=Some(audit)
        save.status = Some(String::from("appeal"));
        let status_none = update_summary2(1,true,&save,"user",&tester.db_pool,&tester.kphis).await.unwrap();
        assert_eq!(status_none.rows_affected(), 1);

        // now summary_id=1 has statis=Some(appeal)
        save.status = Some(String::from("claim"));
        let status_none = update_summary2(1,true,&save,"user",&tester.db_pool,&tester.kphis).await.unwrap();
        assert_eq!(status_none.rows_affected(), 1);

        // now summary_id=1 has statis=Some(claim)
        save.status = Some(String::from("review"));
        let status_claim = update_summary2(1,true,&save,"user",&tester.db_pool,&tester.kphis).await.unwrap();
        assert_eq!(status_claim.rows_affected(), 0);
    }

    #[tokio::test]
    #[ignore]
    async fn sqlx_update_summary2_done() {
        let tester = MySqlTester::new_kphis().await;
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis/ipd_summary_2.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/kphis/ipd_summary_2.sql")).execute(&tester.db_pool).await.unwrap();

        let mut save = SummaryDataSave::demo();
        save.status = Some(String::from("done"));
        let success = update_summary2(1,true,&save,"user",&tester.db_pool,&tester.kphis).await.unwrap();
        assert_eq!(success.rows_affected(), 1);

        // now summary_id=1 has statis=Some(done)
        save.status = Some(String::from("review"));
        let status_done = update_summary2(1,true,&save,"user",&tester.db_pool,&tester.kphis).await.unwrap();
        assert_eq!(status_done.rows_affected(), 0);
    }

    #[tokio::test]
    #[ignore]
    async fn sqlx_update_summary2_code() {
        let tester = MySqlTester::new_kphis().await;
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis/ipd_summary_2.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/kphis/ipd_summary_2.sql")).execute(&tester.db_pool).await.unwrap();

        let success = update_summary2_code(&SummaryCodeSave::demo(),"user",&tester.db_pool,&tester.kphis).await.unwrap();
        assert_eq!(success.rows_affected(), 1);
        let again_success = update_summary2_code(&SummaryCodeSave::demo(),"user",&tester.db_pool,&tester.kphis).await.unwrap();
        assert_eq!(again_success.rows_affected(), 1);
    }

    #[tokio::test]
    #[ignore]
    async fn sqlx_update_summary2_status() {
        let tester = MySqlTester::new_kphis().await;
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis/ipd_summary_2.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/kphis/ipd_summary_2.sql")).execute(&tester.db_pool).await.unwrap();

        let success = update_summary2_status(&Some(String::from("done")),1,"user",&tester.db_pool,&tester.kphis).await.unwrap();
        assert_eq!(success.rows_affected(), 1);
        let again_success = update_summary2_status(&Some(String::from("done")),1,"user",&tester.db_pool,&tester.kphis).await.unwrap();
        assert_eq!(again_success.rows_affected(), 1);
    }

    #[tokio::test]
    #[ignore]
    async fn sqlx_delete_pre_admission_comorbidity() {
        let tester = MySqlTester::new_kphis().await;
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis/ipd_summary_pre_admission_comorbidity.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/kphis/ipd_summary_pre_admission_comorbidity.sql")).execute(&tester.db_pool).await.unwrap();

        let success = delete_pre_admission_comorbidity(1, &tester.db_pool, &tester.kphis).await.unwrap();
        assert_eq!(success.rows_affected(), 1);
        let again_not_found = delete_pre_admission_comorbidity(1, &tester.db_pool, &tester.kphis).await.unwrap();
        assert_eq!(again_not_found.rows_affected(), 0);
    }

    #[tokio::test]
    #[ignore]
    async fn sqlx_delete_post_admission_comorbidity() {
        let tester = MySqlTester::new_kphis().await;
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis/ipd_summary_post_admission_comorbidity.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/kphis/ipd_summary_post_admission_comorbidity.sql")).execute(&tester.db_pool).await.unwrap();

        let success = delete_post_admission_comorbidity(1, &tester.db_pool, &tester.kphis).await.unwrap();
        assert_eq!(success.rows_affected(), 1);
        let again_not_found = delete_post_admission_comorbidity(1, &tester.db_pool, &tester.kphis).await.unwrap();
        assert_eq!(again_not_found.rows_affected(), 0);
    }

    #[tokio::test]
    #[ignore]
    async fn sqlx_delete_other_diagnosis() {
        let tester = MySqlTester::new_kphis().await;
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis/ipd_summary_other_diagnosis.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/kphis/ipd_summary_other_diagnosis.sql")).execute(&tester.db_pool).await.unwrap();

        let success = delete_other_diagnosis(1, &tester.db_pool, &tester.kphis).await.unwrap();
        assert_eq!(success.rows_affected(), 1);
        let again_not_found = delete_other_diagnosis(1, &tester.db_pool, &tester.kphis).await.unwrap();
        assert_eq!(again_not_found.rows_affected(), 0);
    }

    #[tokio::test]
    #[ignore]
    async fn sqlx_delete_external_cause() {
        let tester = MySqlTester::new_kphis().await;
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis/ipd_summary_external_cause.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/kphis/ipd_summary_external_cause.sql")).execute(&tester.db_pool).await.unwrap();

        let success = delete_external_cause(1, &tester.db_pool, &tester.kphis).await.unwrap();
        assert_eq!(success.rows_affected(), 1);
        let again_not_found = delete_external_cause(1, &tester.db_pool, &tester.kphis).await.unwrap();
        assert_eq!(again_not_found.rows_affected(), 0);
    }

    #[tokio::test]
    #[ignore]
    async fn sqlx_select_summary_note() {
        let tester = MySqlTester::new_hosxp_and_kphis().await;
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis/ipd_summary_note.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/hosxp/doctor.sql")).execute(&tester.db_pool).await.unwrap();

        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/kphis/ipd_summary_note.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/hosxp/doctor.sql")).execute(&tester.db_pool).await.unwrap();

        let found = select_summary_note(1, &tester.db_pool, &tester.hosxp, &tester.kphis).await.unwrap();
        assert_eq!(found.len(), 1);
        let not_found = select_summary_note(999, &tester.db_pool, &tester.hosxp, &tester.kphis).await.unwrap();
        assert!(not_found.is_empty());
    }

    #[tokio::test]
    #[ignore]
    async fn sqlx_insert_summary_note() {
        let tester = MySqlTester::new_kphis().await;
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis/ipd_summary_note.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/kphis/ipd_summary_note.sql")).execute(&tester.db_pool).await.unwrap();

        let success = insert_summary_note(1, &SummaryNoteSave::demo(), &Some(String::from("007")), "user", &tester.db_pool, &tester.kphis).await.unwrap();
        assert_eq!(success.rows_affected(), 1);
        let again_success = insert_summary_note(1, &SummaryNoteSave::demo(), &Some(String::from("007")), "user", &tester.db_pool, &tester.kphis).await.unwrap();
        assert_eq!(again_success.rows_affected(), 1);
    }

    #[tokio::test]
    #[ignore]
    async fn sqlx_update_summary_note() {
        let tester = MySqlTester::new_kphis().await;
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis/ipd_summary_note.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/kphis/ipd_summary_note.sql")).execute(&tester.db_pool).await.unwrap();

        let success = update_summary_note(1, &SummaryNoteSave::demo(), &Some(String::from("007")), "user", &tester.db_pool, &tester.kphis).await.unwrap();
        assert_eq!(success.rows_affected(), 1);
        let again_success = update_summary_note(1, &SummaryNoteSave::demo(), &Some(String::from("007")), "user", &tester.db_pool, &tester.kphis).await.unwrap();
        assert_eq!(again_success.rows_affected(), 1);
    }

    #[tokio::test]
    #[ignore]
    async fn sqlx_delete_summary_note() {
        let tester = MySqlTester::new_kphis().await;
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis/ipd_summary_note.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/kphis/ipd_summary_note.sql")).execute(&tester.db_pool).await.unwrap();

        let success = delete_summary_note(1, &SummaryNoteSave::demo(), &Some(String::from("007")), &tester.db_pool, &tester.kphis).await.unwrap();
        assert_eq!(success.rows_affected(), 1);
        let again_success = delete_summary_note(1, &SummaryNoteSave::demo(), &Some(String::from("007")), &tester.db_pool, &tester.kphis).await.unwrap();
        assert_eq!(again_success.rows_affected(), 0);
    }
}
