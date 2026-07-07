use sqlx::{
    AssertSqlSafe, FromRow, MySql, Pool, Row,
    mysql::{MySqlQueryResult, MySqlRow},
};

use kphis_model::{
    app::VisitTypeId,
    fetch::ExecuteResponse,
    med_reconcile::{
        AdmissionNoteLastDose, MedReconciliation, MedReconciliationDetail, MedReconciliationItem, MedReconciliationItemPatch, MedReconciliationItemSave, MedReconciliationNote,
        MedReconciliationParams, ReMedMedication, ReMedVisit,
    },
};
use kphis_sql::ipd::med_reconcile;
use kphis_util::error::{AppError, Source};

// ipd-dr-med-reconcile-data.php
// GET /ipd/med-reconcile
pub async fn get_ipd_med_reconcile(params: &MedReconciliationParams, doctor_code: &Option<String>, pool: &Pool<MySql>, hosxp: &str, kphis: &str) -> Result<Vec<MedReconciliation>, AppError> {
    let mut recons = get_med_reconciliation(params, doctor_code, pool, hosxp, kphis).await?;

    if recons.is_empty() {
        Ok(Vec::new())
    } else {
        let ids = recons.iter().map(|r| r.med_reconciliation_id).collect::<Vec<u32>>();
        let items = get_med_reconciliation_item(&ids, params, pool, hosxp, kphis).await?;
        for recon in recons.iter_mut() {
            recon.med_reconciliation_items = items
                .iter()
                .filter(|i| i.med_reconciliation_id == Some(recon.med_reconciliation_id))
                .cloned()
                .collect::<Vec<MedReconciliationItem>>();
        }

        Ok(recons)
    }
}

async fn get_med_reconciliation(params: &MedReconciliationParams, doctor_code: &Option<String>, pool: &Pool<MySql>, hosxp: &str, kphis: &str) -> Result<Vec<MedReconciliation>, AppError> {
    let sql = med_reconcile::get_med_reconciliation(doctor_code.is_some(), params, hosxp, kphis);
    let mut query = sqlx::query(AssertSqlSafe(sql));
    if let Some(doctor) = doctor_code {
        query = query.bind(doctor).bind(doctor);
    }
    if let Some(med_reconciliation_id) = params.med_reconciliation_id.as_ref() {
        query = query.bind(med_reconciliation_id);
    }
    if let Some(an) = params.an.as_ref() {
        query = query.bind(an);
    }
    query
        .fetch_all(pool)
        .await
        .map_err(|e| Source::SQLx.to_error(500, e, "Select IpdMedReconcile"))?
        .iter()
        .map(med_rec_from_row)
        .collect::<sqlx::Result<Vec<MedReconciliation>>>()
        .map_err(|e| Source::SQLx.to_error(500, e, "Select IpdMedReconcile"))
}
fn med_rec_from_row(row: &MySqlRow) -> sqlx::Result<MedReconciliation> {
    let an: String = row.try_get("an")?;
    Ok(MedReconciliation {
        visit_type: VisitTypeId::Ipd(an.clone()),
        med_reconciliation_id: row.try_get("med_reconciliation_id")?,
        pharmacist: row.try_get("pharmacist")?,
        note: row.try_get("note")?,
        doctor: row.try_get("doctor")?,
        med_reconciliation_datetime: row.try_get("med_reconciliation_datetime")?,
        phamacist_confirm_datetime: row.try_get("phamacist_confirm_datetime")?,
        doctor_confirm_datetime: row.try_get("doctor_confirm_datetime")?,
        pharmacist_name: row.try_get("pharmacist_name")?,
        doctor_name: row.try_get("doctor_name")?,
        is_pharmacist_current_user_doctor: row.try_get("is_pharmacist_current_user_doctor")?,
        med_reconciliation_items: Vec::new(),
    })
}

async fn get_med_reconciliation_item(ids: &[u32], params: &MedReconciliationParams, pool: &Pool<MySql>, hosxp: &str, kphis: &str) -> Result<Vec<MedReconciliationItem>, AppError> {
    let hn = params.hn.clone().unwrap_or_default();
    let item_sql = med_reconcile::get_med_reconciliation_item(params, ids, hosxp, kphis);
    let mut item_query = sqlx::query(AssertSqlSafe(item_sql)).bind(&hn).bind(&hn);
    if let Some(used) = params.used.as_ref() {
        item_query = item_query.bind(used);
    }
    item_query
        .fetch_all(pool)
        .await
        .map_err(|e| Source::SQLx.to_error(500, e, "Select IpdMedReconcileItem"))?
        .iter()
        .map(med_rec_item_from_row)
        .collect::<sqlx::Result<Vec<MedReconciliationItem>>>()
        .map_err(|e| Source::SQLx.to_error(500, e, "Select IpdMedReconcileItem"))
}
fn med_rec_item_from_row(row: &MySqlRow) -> sqlx::Result<MedReconciliationItem> {
    let an: String = row.try_get("an")?;
    Ok(MedReconciliationItem {
        visit_type: VisitTypeId::Ipd(an.clone()),
        med_reconciliation_item_id: row.try_get("med_reconciliation_item_id")?,
        med_reconciliation_id: row.try_get("med_reconciliation_id")?,
        icode: row.try_get("icode")?,
        med_name: row.try_get("med_name")?,
        custom_med_name: row.try_get("custom_med_name")?,
        receive_from: row.try_get("receive_from")?,
        receive_date: row.try_get("receive_date")?,
        old_drugusage: row.try_get("old_drugusage")?,
        changed_drugusage: row.try_get("changed_drugusage")?,
        receive_qty: row.try_get("receive_qty")?,
        last_dose_taken_time: row.try_get("last_dose_taken_time")?,
        last_dose_taken_remark: row.try_get("last_dose_taken_remark")?,
        used: row.try_get("used")?,
        allergy_agent: row.try_get("allergy_agent")?,
        allergy_agent_symptom: row.try_get("allergy_agent_symptom")?,
        allergy_count_force_no_order: row.try_get("allergy_count_force_no_order")?,
        generic_name: row.try_get("generic_name")?,
        dosageform: row.try_get("dosageform")?,
        show_notify: row.try_get("show_notify")?,
        show_notify_text: row.try_get("show_notify_text")?,
        due_usage: row.try_get("due_usage")?,
        due_status: row.try_get("due_status")?,
        info: row.try_get("info")?,
        info_status: row.try_get("info_status")?,
    })
}

// // ipd-dr-med-reconcile-save.php
// POST /ipd/med-reconcile
pub async fn post_ipd_med_reconcile(
    an: &str,
    items: &[MedReconciliationItemSave],
    doctor_code: &Option<String>,
    user: &str,
    pool: &Pool<MySql>,
    kphis: &str,
) -> Result<(u32, Vec<ExecuteResponse>), AppError> {
    let id;
    let mut results = Vec::with_capacity(2);
    // 1. get last unconfirm med_reconcile
    let last_id_opt = get_last_unconfirm_mr(an, doctor_code, pool, kphis).await?;
    if let Some(med_reconciliation_id) = last_id_opt {
        // 2.1 update last unconfirm med_reconcile
        id = med_reconciliation_id;
        let update_result = update_mr(med_reconciliation_id, user, pool, kphis).await?;
        results.push(ExecuteResponse::from_query_result(update_result, "Update IpdMedReconcile"));
    } else {
        // 2.2 insert med_reconcile
        let insert_result = insert_mr(an, doctor_code, user, pool, kphis).await?;
        id = insert_result.last_insert_id() as u32;
        results.push(ExecuteResponse::from_query_result(insert_result, "Insert IpdMedReconcile"));
    }
    // 3. insert med_reconcile_items
    if !items.is_empty() {
        let insert_result = insert_mri(id, an, items, user, pool, kphis).await?;
        results.push(ExecuteResponse::from_query_result(insert_result, "Insert IpdMedReconcileItem"));
    }

    Ok((id, results))
}

async fn get_last_unconfirm_mr(an: &str, doctor_code: &Option<String>, pool: &Pool<MySql>, kphis: &str) -> Result<Option<u32>, AppError> {
    let last_sql = med_reconcile::get_last_unconfirm_mr(kphis);
    sqlx::query(AssertSqlSafe(last_sql))
        .bind(an)
        .bind(doctor_code)
        .fetch_optional(pool)
        .await
        .map_err(|e| Source::SQLx.to_error(500, e, "Select LastIpdMedReconcile"))?
        .as_ref()
        .map(|row| row.try_get::<u32, &str>("med_reconciliation_id"))
        .transpose()
        .map_err(|e| Source::SQLx.to_error(500, e, "Select LastIpdMedReconcile"))
}

async fn update_mr(med_reconciliation_id: u32, user: &str, pool: &Pool<MySql>, kphis: &str) -> Result<MySqlQueryResult, AppError> {
    let update_sql = med_reconcile::update_mr(kphis);
    sqlx::query(AssertSqlSafe(update_sql))
        .bind(user)
        .bind(med_reconciliation_id)
        .execute(pool)
        .await
        .map_err(|e| Source::SQLx.to_error(500, e, "Update IpdMedReconcile"))
}

async fn insert_mr(an: &str, doctor_code: &Option<String>, user: &str, pool: &Pool<MySql>, kphis: &str) -> Result<MySqlQueryResult, AppError> {
    let insert_sql = med_reconcile::insert_mr(kphis);
    sqlx::query(AssertSqlSafe(insert_sql))
        .bind(an)
        .bind(doctor_code)
        .bind(user)
        .bind(user)
        .execute(pool)
        .await
        .map_err(|e| Source::SQLx.to_error(500, e, "Insert IpdMedReconcile"))
}

async fn insert_mri(med_reconciliation_id: u32, an: &str, items: &[MedReconciliationItemSave], user: &str, pool: &Pool<MySql>, kphis: &str) -> Result<MySqlQueryResult, AppError> {
    let insert_sql = med_reconcile::insert_mri(items.len(), kphis);
    let mut insert_query = sqlx::query(AssertSqlSafe(insert_sql));
    for item in items {
        insert_query = insert_query
            .bind(med_reconciliation_id)
            .bind(an)
            .bind(&item.icode)
            .bind(&item.med_name)
            .bind(&item.custom_med_name)
            .bind(&item.receive_from)
            .bind(item.receive_date)
            .bind(&item.old_drugusage)
            .bind(item.receive_qty)
            .bind(user)
            .bind(user);
    }
    insert_query.execute(pool).await.map_err(|e| Source::SQLx.to_error(500, e, "Insert IpdMedReconcileItem"))
}

// ipd-dr-med-reconcile-doctor-confirm.php
// ipd-dr-med-reconcile-pharmacist-confirm.php
// ipd-dr-med-reconcile-pharmacist-unconfirm.php
// ipd-dr-med-reconcile-last-dose-save.php
// PATCH /ipd/med-reconcile
pub async fn patch_ipd_med_reconcile(
    med_reconciliation_id: u32,
    patch: &str,
    items: &[MedReconciliationItemPatch],
    doctor_code: &Option<String>,
    user: &str,
    pool: &Pool<MySql>,
    kphis: &str,
) -> Result<Vec<ExecuteResponse>, AppError> {
    let mut results = Vec::with_capacity(2);
    match patch {
        "doctor" => {
            let mr_result = update_mr_doctor_confirm(med_reconciliation_id, doctor_code, user, pool, kphis).await?;
            let is_update = mr_result.rows_affected() > 0;
            results.push(ExecuteResponse::from_query_result(mr_result, "Update MRDoctorConfirm"));

            if is_update && !items.is_empty() {
                results.extend(update_mri_doctor_confirm(items, user, pool, kphis).await?);
            }

            Ok(results)
        }
        "pharm" => {
            let mr_result = update_mr_pharm_confirm(med_reconciliation_id, doctor_code, user, pool, kphis).await?;
            let is_update = mr_result.rows_affected() > 0;
            results.push(ExecuteResponse::from_query_result(mr_result, "Update MRPharmConfirm"));

            if is_update && !items.is_empty() {
                results.extend(update_mri_pharm_confirm(items, user, pool, kphis).await?);
            }

            Ok(results)
        }
        "unconfirm" => {
            let mr_result = update_mr_pharm_unconfirm(med_reconciliation_id, doctor_code, user, pool, kphis).await?;
            results.push(ExecuteResponse::from_query_result(mr_result, "Update MRPharmUnConfirm"));

            Ok(results)
        }
        "receive" => {
            if !items.is_empty() {
                results.extend(update_mri_receive(items, user, pool, kphis).await?);

                Ok(results)
            } else {
                Err(Source::App.to_error(500, "Empty Item", "Update MRIReceive"))
            }
        }
        "last" => {
            if !items.is_empty() {
                results.extend(update_mri_last_dose(items, user, pool, kphis).await?);

                Ok(results)
            } else {
                Err(Source::App.to_error(500, "Empty Item", "Update MRILastDose"))
            }
        }
        _ => Err(Source::App.to_error(500, "Invalid Patch", "Patch IpdMedReconcile")),
    }
}

async fn update_mr_doctor_confirm(med_reconciliation_id: u32, doctor_code: &Option<String>, user: &str, pool: &Pool<MySql>, kphis: &str) -> Result<MySqlQueryResult, AppError> {
    let mr_sql = med_reconcile::update_mr_doctor_confirm(kphis);
    sqlx::query(AssertSqlSafe(mr_sql))
        .bind(doctor_code)
        .bind(user)
        .bind(med_reconciliation_id)
        .execute(pool)
        .await
        .map_err(|e| Source::SQLx.to_error(500, e, "Update MRDoctorConfirm"))
}

async fn update_mri_doctor_confirm(items: &[MedReconciliationItemPatch], user: &str, pool: &Pool<MySql>, kphis: &str) -> Result<Vec<ExecuteResponse>, AppError> {
    let mut results = Vec::with_capacity(items.len());
    let mri_sql = med_reconcile::update_mri_doctor_confirm(kphis);
    for item in items {
        let mri_result = sqlx::query(AssertSqlSafe(mri_sql.clone()))
            .bind(&item.used)
            .bind(&item.changed_drugusage)
            .bind(item.last_dose_taken_time)
            .bind(&item.last_dose_taken_remark)
            .bind(user)
            .bind(item.med_reconciliation_item_id)
            .execute(pool)
            .await
            .map_err(|e| Source::SQLx.to_error(500, e, "Update MRIDoctorConfirm"))?;
        results.push(ExecuteResponse::from_query_result(mri_result, "Update MRIDoctorConfirm"));
    }
    Ok(results)
}

async fn update_mr_pharm_confirm(med_reconciliation_id: u32, doctor_code: &Option<String>, user: &str, pool: &Pool<MySql>, kphis: &str) -> Result<MySqlQueryResult, AppError> {
    let mr_sql = med_reconcile::update_mr_pharm_confirm(kphis);
    sqlx::query(AssertSqlSafe(mr_sql))
        .bind(user)
        .bind(med_reconciliation_id)
        .bind(doctor_code)
        .execute(pool)
        .await
        .map_err(|e| Source::SQLx.to_error(500, e, "Update MRPharmConfirm"))
}

async fn update_mri_pharm_confirm(items: &[MedReconciliationItemPatch], user: &str, pool: &Pool<MySql>, kphis: &str) -> Result<Vec<ExecuteResponse>, AppError> {
    let mut results = Vec::with_capacity(items.len());
    let mri_sql = med_reconcile::update_mri_pharm_confirm(kphis);
    for item in items {
        let mri_result = sqlx::query(AssertSqlSafe(mri_sql.clone()))
            .bind(&item.old_drugusage)
            .bind(item.receive_qty)
            .bind(&item.receive_from)
            .bind(item.receive_date)
            .bind(item.last_dose_taken_time)
            .bind(&item.last_dose_taken_remark)
            .bind(user)
            .bind(item.med_reconciliation_item_id)
            .execute(pool)
            .await
            .map_err(|e| Source::SQLx.to_error(500, e, "Update MRIPharmConfirm"))?;
        results.push(ExecuteResponse::from_query_result(mri_result, "Update MRIPharmConfirm"));
    }
    Ok(results)
}

async fn update_mr_pharm_unconfirm(med_reconciliation_id: u32, doctor_code: &Option<String>, user: &str, pool: &Pool<MySql>, kphis: &str) -> Result<MySqlQueryResult, AppError> {
    let mr_sql = med_reconcile::update_mr_pharm_unconfirm(kphis);
    sqlx::query(AssertSqlSafe(mr_sql))
        .bind(user)
        .bind(med_reconciliation_id)
        .bind(doctor_code)
        .execute(pool)
        .await
        .map_err(|e| Source::SQLx.to_error(500, e, "Update MRPharmUnConfirm"))
}

async fn update_mri_receive(items: &[MedReconciliationItemPatch], user: &str, pool: &Pool<MySql>, kphis: &str) -> Result<Vec<ExecuteResponse>, AppError> {
    let mut results = Vec::with_capacity(items.len());
    let mri_sql = med_reconcile::update_mri_receive(kphis);
    for item in items {
        let mri_result = sqlx::query(AssertSqlSafe(mri_sql.clone()))
            .bind(item.receive_qty)
            .bind(&item.receive_from)
            .bind(item.receive_date)
            .bind(user)
            .bind(item.med_reconciliation_item_id)
            .execute(pool)
            .await
            .map_err(|e| Source::SQLx.to_error(500, e, "Update MRIReceive"))?;
        results.push(ExecuteResponse::from_query_result(mri_result, "Update MRIReceive"));
    }
    Ok(results)
}

async fn update_mri_last_dose(items: &[MedReconciliationItemPatch], user: &str, pool: &Pool<MySql>, kphis: &str) -> Result<Vec<ExecuteResponse>, AppError> {
    let mut results = Vec::with_capacity(items.len());
    let mri_sql = med_reconcile::update_mri_last_dose(kphis);
    for item in items {
        let mri_result = sqlx::query(AssertSqlSafe(mri_sql.clone()))
            .bind(item.last_dose_taken_time)
            .bind(&item.last_dose_taken_remark)
            .bind(user)
            .bind(item.med_reconciliation_item_id)
            .execute(pool)
            .await
            .map_err(|e| Source::SQLx.to_error(500, e, "Update MRILastDose"))?;
        results.push(ExecuteResponse::from_query_result(mri_result, "Update MRILastDose"));
    }
    Ok(results)
}

// ipd-dr-med-reconcile-delete.php
// DELETE /ipd/med-reconcile
/// delete med_reconfile + items
pub async fn delete_ipd_med_reconcile(med_reconciliation_id: u32, pool: &Pool<MySql>, kphis: &str) -> Result<ExecuteResponse, AppError> {
    let sql = med_reconcile::delete_med_reconciliation(kphis);
    let result = sqlx::query(AssertSqlSafe(sql))
        .bind(med_reconciliation_id)
        .execute(pool)
        .await
        .map_err(|e| Source::SQLx.to_error(500, e, "Delete IpdMedReconcileosXp"))?;

    Ok(ExecuteResponse::from_query_result(result, "Delete IpdMedReconcile"))
}

// ipd-dr-med-reconcile-item-delete.php
// DELETE /ipd/med-reconcile
pub async fn delete_ipd_med_reconcile_item(med_reconciliation_item_id: u32, pool: &Pool<MySql>, kphis: &str) -> Result<ExecuteResponse, AppError> {
    let sql = med_reconcile::delete_med_reconciliation_item(kphis);
    let result = sqlx::query(AssertSqlSafe(sql))
        .bind(med_reconciliation_item_id)
        .execute(pool)
        .await
        .map_err(|e| Source::SQLx.to_error(500, e, "Delete IpdMedReconcileItem"))?;

    Ok(ExecuteResponse::from_query_result(result, "Delete IpdMedReconcileItem"))
}

// ipd-dr-med-reconcile-from-hosxp.php
// GET /ipd/med-reconcile-hosxp/:an
pub async fn get_ipd_med_reconcile_hosxp(an: &str, pool: &Pool<MySql>, hosxp: &str) -> Result<Vec<MedReconciliationDetail>, AppError> {
    let sql = med_reconcile::from_hosxp(hosxp);
    let result = sqlx::query(AssertSqlSafe(sql))
        .bind(an)
        .fetch_all(pool)
        .await
        .map_err(|e| Source::SQLx.to_error(500, e, "Select IpdMedReconcileHosXp"))?
        .iter()
        .map(MedReconciliationDetail::from_row)
        .collect::<sqlx::Result<Vec<MedReconciliationDetail>>>()
        .map_err(|e| Source::SQLx.to_error(500, e, "Select IpdMedReconcileHosXp"))?;

    Ok(result)
}

// ipd-dr-med-reconcile-dr-admission-note-last-dose.php
// GET /ipd/med-reconcile-last-dose/:an
pub async fn get_ipd_med_reconcile_last_dose(an: &str, pool: &Pool<MySql>, kphis: &str) -> Result<Option<AdmissionNoteLastDose>, AppError> {
    let sql = med_reconcile::get_last_dose(kphis);
    let result = sqlx::query(AssertSqlSafe(sql))
        .bind(an)
        .fetch_optional(pool)
        .await
        .map_err(|e| Source::SQLx.to_error(500, e, "Select IpdMedReconcileLastDose"))?
        .as_ref()
        .map(AdmissionNoteLastDose::from_row)
        .transpose()
        .map_err(|e| Source::SQLx.to_error(500, e, "Select IpdMedReconcileLastDose"))?;

    Ok(result)
}

// ipd-dr-med-reconcile-note-data.php
// GET /ipd/med-reconcile-note/:med_reconciliation_id
pub async fn get_ipd_med_reconcile_note(med_reconciliation_id: u32, pool: &Pool<MySql>, kphis: &str) -> Result<Option<MedReconciliationNote>, AppError> {
    let sql = med_reconcile::get_note(kphis);
    let result = sqlx::query(AssertSqlSafe(sql))
        .bind(med_reconciliation_id)
        .fetch_optional(pool)
        .await
        .map_err(|e| Source::SQLx.to_error(500, e, "Select IpdMedReconcileNote"))?
        .as_ref()
        .map(MedReconciliationNote::from_row)
        .transpose()
        .map_err(|e| Source::SQLx.to_error(500, e, "Select IpdMedReconcileNote"))?;

    Ok(result)
}

// ipd-dr-med-reconcile-note-save.php
// POST /ipd/med-reconcile-note/:med_reconciliation_id
pub async fn post_ipd_med_reconcile_note(med_reconciliation_id: u32, note: &str, user: &str, pool: &Pool<MySql>, kphis: &str) -> Result<ExecuteResponse, AppError> {
    let sql = med_reconcile::post_note(kphis);
    let result = sqlx::query(AssertSqlSafe(sql))
        .bind(note)
        .bind(user)
        .bind(med_reconciliation_id)
        .execute(pool)
        .await
        .map_err(|e| Source::SQLx.to_error(500, e, "Update IpdMedReconcileNote"))?;

    Ok(ExecuteResponse::from_query_result(result, "Update IpdMedReconcileNote"))
}

// ipd-dr-med-reconcile-remed-visit-data.php
// GET /ipd/med-reconcile-remed-visit/:hn
pub async fn get_ipd_med_reconcile_remed_visit(hn: &str, pool: &Pool<MySql>, hosxp: &str) -> Result<Vec<ReMedVisit>, AppError> {
    let sql = med_reconcile::get_remed_visit(hosxp);
    let result = sqlx::query(AssertSqlSafe(sql))
        .bind(hn)
        .fetch_all(pool)
        .await
        .map_err(|e| Source::SQLx.to_error(500, e, "Select IpdMedReconcileRemedVisit"))?
        .iter()
        .map(ReMedVisit::from_row)
        .collect::<sqlx::Result<Vec<ReMedVisit>>>()
        .map_err(|e| Source::SQLx.to_error(500, e, "Select IpdMedReconcileRemedVisit"))?;

    Ok(result)
}

// ipd-dr-med-reconcile-remed-med-data.php
// GET /ipd/med-reconcile-remed-med
pub async fn get_ipd_med_reconcile_remed_med(params: &MedReconciliationParams, pool: &Pool<MySql>, hosxp: &str) -> Result<Vec<ReMedMedication>, AppError> {
    let sql = med_reconcile::get_remed_med(params, hosxp);
    let mut query = sqlx::query(AssertSqlSafe(sql));
    match (params.vn.as_ref(), params.an.as_ref()) {
        (Some(vn), Some(an)) => {
            query = query.bind(vn).bind(an);
        }
        (Some(vn), None) => {
            query = query.bind(vn);
        }
        (None, Some(an)) => {
            query = query.bind(an);
        }
        (None, None) => {}
    }
    let result = query
        .fetch_all(pool)
        .await
        .map_err(|e| Source::SQLx.to_error(500, e, "Select IpdMedReconcileRemedMed"))?
        .iter()
        .map(ReMedMedication::from_row)
        .collect::<sqlx::Result<Vec<ReMedMedication>>>()
        .map_err(|e| Source::SQLx.to_error(500, e, "Select IpdMedReconcileRemedMed"))?;

    Ok(result)
}

#[cfg(test)]
#[rustfmt::skip]
mod tests {

    use super::*;
    use kphis_sqlx_tester::MySqlTester;

    #[tokio::test]
    #[ignore]
    async fn sqlx_get_med_reconciliation() {
        let tester = MySqlTester::new_hosxp_and_kphis().await;
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis/ipd_med_reconciliation.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/hosxp/doctor.sql")).execute(&tester.db_pool).await.unwrap();

        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/kphis/ipd_med_reconciliation.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/hosxp/doctor.sql")).execute(&tester.db_pool).await.unwrap();

        let default = get_med_reconciliation(&MedReconciliationParams::default(),&None,&tester.db_pool,&tester.hosxp,&tester.kphis).await.unwrap();
        assert_eq!(default.len(), 3);
        // (pharmacist not comfirmed + match pharmacist) + pharmacist comfirmed(2)
        let found_doctor_unconfirm = get_med_reconciliation(&MedReconciliationParams::default(),&Some(String::from("009")),&tester.db_pool,&tester.hosxp,&tester.kphis).await.unwrap();
        assert_eq!(found_doctor_unconfirm.len(), 3);
        let not_found_doctor_unconfirm = get_med_reconciliation(&MedReconciliationParams::default(),&Some(String::from("007")),&tester.db_pool,&tester.hosxp,&tester.kphis).await.unwrap();
        assert_eq!(not_found_doctor_unconfirm.len(), 2);
        let found_an = get_med_reconciliation(&MedReconciliationParams {an: Some(String::from("660001234")),..Default::default()},&None,&tester.db_pool,&tester.hosxp,&tester.kphis).await.unwrap();
        assert_eq!(found_an.len(), 3);
        let found_id = get_med_reconciliation(&MedReconciliationParams {med_reconciliation_id: Some(1),..Default::default()},&None,&tester.db_pool,&tester.hosxp,&tester.kphis).await.unwrap();
        assert_eq!(found_id.len(), 1);
        let not_found = get_med_reconciliation(&MedReconciliationParams {med_reconciliation_id: Some(999),..Default::default()},&None,&tester.db_pool,&tester.hosxp,&tester.kphis).await.unwrap();
        assert!(not_found.is_empty());
    }

    #[tokio::test]
    #[ignore]
    async fn sqlx_get_med_reconciliation_item() {
        let tester = MySqlTester::new_hosxp_and_kphis().await;
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis/ipd_med_reconciliation_item.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis/kphis_drug_use_duration.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/hosxp/drugitems.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/hosxp/opd_allergy.sql")).execute(&tester.db_pool).await.unwrap();

        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/kphis/ipd_med_reconciliation_item.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/kphis/kphis_drug_use_duration.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/hosxp/drugitems.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/hosxp/opd_allergy.sql")).execute(&tester.db_pool).await.unwrap();

        let default = get_med_reconciliation_item(&[],&MedReconciliationParams::default(),&tester.db_pool,&tester.hosxp,&tester.kphis).await.unwrap();
        assert!(default.is_empty());
        let found = get_med_reconciliation_item(&[1, 2, 3],&MedReconciliationParams::default(),&tester.db_pool,&tester.hosxp,&tester.kphis).await.unwrap();
        assert_eq!(found.len(), 3);
        let found_with_hn = get_med_reconciliation_item(&[1, 2, 3],&MedReconciliationParams {hn: Some(String::from("660001234")),..Default::default()},&tester.db_pool,&tester.hosxp,&tester.kphis).await.unwrap();
        assert_eq!(found_with_hn.len(), 3);
        let found_use_y = get_med_reconciliation_item(&[1, 2, 3],&MedReconciliationParams {used: Some(String::from("Y")),..Default::default()},&tester.db_pool,&tester.hosxp,&tester.kphis).await.unwrap();
        assert_eq!(found_use_y.len(), 1);
        let found_use_h = get_med_reconciliation_item(&[1, 2, 3],&MedReconciliationParams {used: Some(String::from("H")),..Default::default()},&tester.db_pool,&tester.hosxp,&tester.kphis).await.unwrap();
        assert_eq!(found_use_h.len(), 1);
        let found_use_n = get_med_reconciliation_item(&[1, 2, 3],&MedReconciliationParams {used: Some(String::from("N")),..Default::default()},&tester.db_pool,&tester.hosxp,&tester.kphis).await.unwrap();
        assert_eq!(found_use_n.len(), 1);
    }

    #[tokio::test]
    #[ignore]
    async fn sqlx_get_last_unconfirm_mr() {
        let tester = MySqlTester::new_kphis().await;
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis/ipd_med_reconciliation.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/kphis/ipd_med_reconciliation.sql")).execute(&tester.db_pool).await.unwrap();

        // match both an and pharmacist + phamacist_confirm_datetime IS NULL + doctor_confirm_datetime IS NULL
        let found = get_last_unconfirm_mr("660001234",&Some(String::from("009")),&tester.db_pool,&tester.kphis).await.unwrap();
        assert!(found.is_some());
        let no_pharmacist = get_last_unconfirm_mr("660001234", &None, &tester.db_pool, &tester.kphis).await.unwrap();
        assert!(no_pharmacist.is_none());
        let not_found = get_last_unconfirm_mr("660006666",&Some(String::from("009")),&tester.db_pool,&tester.kphis).await.unwrap();
        assert!(not_found.is_none());
    }

    #[tokio::test]
    #[ignore]
    async fn sqlx_get_ipd_med_reconcile_hosxp() {
        let tester = MySqlTester::new_hosxp().await;
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/hosxp/medication_reconciliation.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/hosxp/medication_reconciliation_detail.sql")).execute(&tester.db_pool).await.unwrap();

        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/hosxp/medication_reconciliation.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/hosxp/medication_reconciliation_detail.sql")).execute(&tester.db_pool).await.unwrap();

        let found = get_ipd_med_reconcile_hosxp("660001234", &tester.db_pool, &tester.hosxp).await.unwrap();
        assert_eq!(found.len(), 1);
        let not_found = get_ipd_med_reconcile_hosxp("660006666", &tester.db_pool, &tester.hosxp).await.unwrap();
        assert!(not_found.is_empty());
    }

    #[tokio::test]
    #[ignore]
    async fn sqlx_get_ipd_med_reconcile_last_dose() {
        let tester = MySqlTester::new_kphis().await;
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis/ipd_dr_admission_note.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/kphis/ipd_dr_admission_note.sql")).execute(&tester.db_pool).await.unwrap();

        let found = get_ipd_med_reconcile_last_dose("660001234", &tester.db_pool, &tester.kphis).await.unwrap();
        assert!(found.is_some());
        let not_found = get_ipd_med_reconcile_last_dose("660006666", &tester.db_pool, &tester.kphis).await.unwrap();
        assert!(not_found.is_none());
    }

    #[tokio::test]
    #[ignore]
    async fn sqlx_get_ipd_med_reconcile_note() {
        let tester = MySqlTester::new_kphis().await;
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis/ipd_med_reconciliation.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/kphis/ipd_med_reconciliation.sql")).execute(&tester.db_pool).await.unwrap();

        let found = get_ipd_med_reconcile_note(1, &tester.db_pool, &tester.kphis).await.unwrap();
        assert!(found.is_some());
        let not_found = get_ipd_med_reconcile_note(999, &tester.db_pool, &tester.kphis).await.unwrap();
        assert!(not_found.is_none());
    }

    #[tokio::test]
    #[ignore]
    async fn sqlx_get_ipd_med_reconcile_remed_visit() {
        let tester = MySqlTester::new_hosxp().await;
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/hosxp/ovst.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/hosxp/opitemrece.sql")).execute(&tester.db_pool).await.unwrap();

        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/hosxp/ovst.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/hosxp/opitemrece.sql")).execute(&tester.db_pool).await.unwrap();

        let found = get_ipd_med_reconcile_remed_visit("0001234", &tester.db_pool, &tester.hosxp).await.unwrap();
        assert_eq!(found.len(), 2);
        let not_found = get_ipd_med_reconcile_remed_visit("0006666", &tester.db_pool, &tester.hosxp).await.unwrap();
        assert!(not_found.is_empty());
    }

    #[tokio::test]
    #[ignore]
    async fn sqlx_get_ipd_med_reconcile_remed_med() {
        let tester = MySqlTester::new_hosxp().await;
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/hosxp/opitemrece.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/hosxp/drugitems.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/hosxp/s_drugitems.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/hosxp/drugusage.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/hosxp/sp_use.sql")).execute(&tester.db_pool).await.unwrap();

        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/hosxp/opitemrece.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/hosxp/drugitems.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/hosxp/s_drugitems.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/hosxp/drugusage.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/hosxp/sp_use.sql")).execute(&tester.db_pool).await.unwrap();

        let default = get_ipd_med_reconcile_remed_med(&MedReconciliationParams::default(),&tester.db_pool,&tester.hosxp).await.unwrap();
        assert_eq!(default.len(), 7);
        // only o.item_type='H'
        let found_an = get_ipd_med_reconcile_remed_med(&MedReconciliationParams {an: Some(String::from("660001234")),..Default::default()},&tester.db_pool,&tester.hosxp).await.unwrap();
        assert_eq!(found_an.len(), 1);
        let found_vn = get_ipd_med_reconcile_remed_med(&MedReconciliationParams {vn: Some(String::from("661231235959")),..Default::default()},&tester.db_pool,&tester.hosxp).await.unwrap();
        assert_eq!(found_vn.len(), 6);
        let not_found = get_ipd_med_reconcile_remed_med(&MedReconciliationParams {an: Some(String::from("660006666")),..Default::default()},&tester.db_pool,&tester.hosxp).await.unwrap();
        assert!(not_found.is_empty());
    }

    #[tokio::test]
    #[ignore]
    async fn sqlx_insert_mr() {
        let tester = MySqlTester::new_kphis().await;
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis/ipd_med_reconciliation.sql")).execute(&tester.db_pool).await.unwrap();

        let success = insert_mr("660001234",&Some(String::from("007")),"user",&tester.db_pool,&tester.kphis).await.unwrap();
        assert_eq!(success.rows_affected(), 1);
        let again_success = insert_mr("660001234", &None, "user", &tester.db_pool, &tester.kphis).await.unwrap();
        assert_eq!(again_success.rows_affected(), 1);
    }

    #[tokio::test]
    #[ignore]
    async fn sqlx_insert_mri() {
        let tester = MySqlTester::new_kphis().await;
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis/ipd_med_reconciliation_item.sql")).execute(&tester.db_pool).await.unwrap();

        let success = insert_mri(1,"660001234",&[MedReconciliationItemSave::demo()],"user",&tester.db_pool,&tester.kphis).await.unwrap();
        assert_eq!(success.rows_affected(), 1);
        let again_success = insert_mri(1,"660001234",&[MedReconciliationItemSave::demo()],"user",&tester.db_pool,&tester.kphis).await.unwrap();
        assert_eq!(again_success.rows_affected(), 1);
    }

    #[tokio::test]
    #[ignore]
    async fn sqlx_update_mr() {
        let tester = MySqlTester::new_kphis().await;
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis/ipd_med_reconciliation.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/kphis/ipd_med_reconciliation.sql")).execute(&tester.db_pool).await.unwrap();

        let success = update_mr(1, "user", &tester.db_pool, &tester.kphis).await.unwrap();
        assert_eq!(success.rows_affected(), 1);
        let again_success = update_mr(1, "user", &tester.db_pool, &tester.kphis).await.unwrap();
        assert_eq!(again_success.rows_affected(), 1);
    }

    #[tokio::test]
    #[ignore]
    async fn sqlx_update_mr_doctor_confirm() {
        let tester = MySqlTester::new_kphis().await;
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis/ipd_med_reconciliation.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/kphis/ipd_med_reconciliation.sql")).execute(&tester.db_pool).await.unwrap();

        let success = update_mr_doctor_confirm(2,&Some(String::from("007")),"user",&tester.db_pool,&tester.kphis).await.unwrap();
        assert_eq!(success.rows_affected(), 1);
        // fixed to be able to confirm again
        let again_already_confirmed = update_mr_doctor_confirm(2,&Some(String::from("007")),"user",&tester.db_pool,&tester.kphis).await.unwrap();
        assert_eq!(again_already_confirmed.rows_affected(), 1);
    }

    #[tokio::test]
    #[ignore]
    async fn sqlx_update_mr_pharm_confirm() {
        let tester = MySqlTester::new_kphis().await;
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis/ipd_med_reconciliation.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/kphis/ipd_med_reconciliation.sql")).execute(&tester.db_pool).await.unwrap();

        let not_found = update_mr_pharm_confirm(3,&Some(String::from("007")),"user",&tester.db_pool,&tester.kphis).await.unwrap();
        assert_eq!(not_found.rows_affected(), 0);
        let success = update_mr_pharm_confirm(3,&Some(String::from("009")),"user",&tester.db_pool,&tester.kphis).await.unwrap();
        assert_eq!(success.rows_affected(), 1);
        let again_already_confirmed = update_mr_pharm_confirm(3,&Some(String::from("009")),"user",&tester.db_pool,&tester.kphis).await.unwrap();
        assert_eq!(again_already_confirmed.rows_affected(), 0);
    }

    #[tokio::test]
    #[ignore]
    async fn sqlx_update_mr_pharm_unconfirm() {
        let tester = MySqlTester::new_kphis().await;
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis/ipd_med_reconciliation.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/kphis/ipd_med_reconciliation.sql")).execute(&tester.db_pool).await.unwrap();

        let not_found = update_mr_pharm_unconfirm(2,&Some(String::from("009")),"user",&tester.db_pool,&tester.kphis).await.unwrap();
        assert_eq!(not_found.rows_affected(), 0);
        let success = update_mr_pharm_unconfirm(2,&Some(String::from("007")),"user",&tester.db_pool,&tester.kphis).await.unwrap();
        assert_eq!(success.rows_affected(), 1);
        let again_already_unconfirmed = update_mr_pharm_unconfirm(2,&Some(String::from("007")),"user",&tester.db_pool,&tester.kphis).await.unwrap();
        assert_eq!(again_already_unconfirmed.rows_affected(), 0);
    }

    #[tokio::test]
    #[ignore]
    async fn sqlx_update_mri_doctor_confirm() {
        let tester = MySqlTester::new_kphis().await;
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis/ipd_med_reconciliation_item.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/kphis/ipd_med_reconciliation_item.sql")).execute(&tester.db_pool).await.unwrap();

        let success = update_mri_doctor_confirm(&[MedReconciliationItemPatch::demo()],"user",&tester.db_pool,&tester.kphis).await.unwrap();
        assert_eq!(success.len(), 1);
        let again_success = update_mri_doctor_confirm(&[MedReconciliationItemPatch::demo()],"user",&tester.db_pool,&tester.kphis).await.unwrap();
        assert_eq!(again_success.len(), 1);
    }

    #[tokio::test]
    #[ignore]
    async fn sqlx_update_mri_pharm_confirm() {
        let tester = MySqlTester::new_kphis().await;
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis/ipd_med_reconciliation_item.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/kphis/ipd_med_reconciliation_item.sql")).execute(&tester.db_pool).await.unwrap();

        let success = update_mri_pharm_confirm(&[MedReconciliationItemPatch::demo()],"user",&tester.db_pool,&tester.kphis).await.unwrap();
        assert_eq!(success.len(), 1);
        let again_success = update_mri_pharm_confirm(&[MedReconciliationItemPatch::demo()],"user",&tester.db_pool,&tester.kphis).await.unwrap();
        assert_eq!(again_success.len(), 1);
    }

    #[tokio::test]
    #[ignore]
    async fn sqlx_update_mri_receive() {
        let tester = MySqlTester::new_kphis().await;
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis/ipd_med_reconciliation_item.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/kphis/ipd_med_reconciliation_item.sql")).execute(&tester.db_pool).await.unwrap();

        let success = update_mri_receive(&[MedReconciliationItemPatch::demo()],"user",&tester.db_pool,&tester.kphis).await.unwrap();
        assert_eq!(success.len(), 1);
        let again_success = update_mri_receive(&[MedReconciliationItemPatch::demo()],"user",&tester.db_pool,&tester.kphis).await.unwrap();
        assert_eq!(again_success.len(), 1);
    }

    #[tokio::test]
    #[ignore]
    async fn sqlx_update_mri_last_dose() {
        let tester = MySqlTester::new_kphis().await;
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis/ipd_med_reconciliation_item.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/kphis/ipd_med_reconciliation_item.sql")).execute(&tester.db_pool).await.unwrap();

        let success = update_mri_last_dose(&[MedReconciliationItemPatch::demo()],"user",&tester.db_pool,&tester.kphis).await.unwrap();
        assert_eq!(success.len(), 1);
        let again_success = update_mri_last_dose(&[MedReconciliationItemPatch::demo()],"user",&tester.db_pool,&tester.kphis).await.unwrap();
        assert_eq!(again_success.len(), 1);
    }

    #[tokio::test]
    #[ignore]
    async fn sqlx_post_ipd_med_reconcile_note() {
        let tester = MySqlTester::new_kphis().await;
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis/ipd_med_reconciliation.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/kphis/ipd_med_reconciliation.sql")).execute(&tester.db_pool).await.unwrap();

        let not_null = post_ipd_med_reconcile_note(1, "NOTE", "user", &tester.db_pool, &tester.kphis).await.unwrap();
        assert_eq!(not_null.rows_affected, 0);
        // phamacist_confirm_datetime IS NULL
        let success = post_ipd_med_reconcile_note(3, "NOTE", "user", &tester.db_pool, &tester.kphis).await.unwrap();
        assert_eq!(success.rows_affected, 1);
        let again_success = post_ipd_med_reconcile_note(3, "NOTE", "user", &tester.db_pool, &tester.kphis).await.unwrap();
        assert_eq!(again_success.rows_affected, 1);
    }

    #[tokio::test]
    #[ignore]
    async fn sqlx_delete_ipd_med_reconcile() {
        let tester = MySqlTester::new_kphis().await;
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis/ipd_med_reconciliation.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis/ipd_med_reconciliation_item.sql")).execute(&tester.db_pool).await.unwrap();

        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/kphis/ipd_med_reconciliation.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/kphis/ipd_med_reconciliation_item.sql")).execute(&tester.db_pool).await.unwrap();

        let not_null = delete_ipd_med_reconcile(1, &tester.db_pool, &tester.kphis).await.unwrap();
        assert_eq!(not_null.rows_affected, 0);
        // phamacist_confirm_datetime IS NULL
        let success = delete_ipd_med_reconcile(3, &tester.db_pool, &tester.kphis).await.unwrap();
        assert_eq!(success.rows_affected, 2);
        let again_not_found = delete_ipd_med_reconcile(3, &tester.db_pool, &tester.kphis).await.unwrap();
        assert_eq!(again_not_found.rows_affected, 0);
    }

    #[tokio::test]
    #[ignore]
    async fn sqlx_delete_ipd_med_reconcile_item() {
        let tester = MySqlTester::new_kphis().await;
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis/ipd_med_reconciliation.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis/ipd_med_reconciliation_item.sql")).execute(&tester.db_pool).await.unwrap();

        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/kphis/ipd_med_reconciliation.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/kphis/ipd_med_reconciliation_item.sql")).execute(&tester.db_pool).await.unwrap();

        let not_null = delete_ipd_med_reconcile_item(1, &tester.db_pool, &tester.kphis).await.unwrap();
        assert_eq!(not_null.rows_affected, 0);
        // phamacist_confirm_datetime IS NULL
        let success = delete_ipd_med_reconcile_item(3, &tester.db_pool, &tester.kphis).await.unwrap();
        assert_eq!(success.rows_affected, 1);
        let again_not_found = delete_ipd_med_reconcile_item(3, &tester.db_pool, &tester.kphis).await.unwrap();
        assert_eq!(again_not_found.rows_affected, 0);
    }
}
