use sqlx::{AssertSqlSafe, FromRow, MySql, Pool, Row};

use kphis_model::{
    fetch::ExecuteResponse,
    opd_er::order_master::{OpdErOrderMaster, OpdErOrderMasterCheck, OpdErOrderMasterList, OpdErOrderMasterParams, OpdErOrderMasterSave},
};
use kphis_sql::{opd_er::order_master, query_utils};
use kphis_util::error::{AppError, Source};

use crate::query1_all;

// opd-er-order-master-check.php
/// return Vec<(opd_er_order_master_id, order_date)>
pub async fn get_order_master_check(vn: &str, pool: &Pool<MySql>, kphis: &str) -> Result<Vec<OpdErOrderMasterCheck>, AppError> {
    let sql = order_master::select_master_check(kphis);
    query1_all(vn, &sql, pool, "Select OrderMasterCheck")
        .await?
        .iter()
        .map(OpdErOrderMasterCheck::from_row)
        .collect::<sqlx::Result<Vec<OpdErOrderMasterCheck>>>()
        .map_err(|e| Source::SQLx.to_error(500, e, "Select OrderMasterCheck"))
}

// opd-er-order-list-data.php
pub async fn get_order_master_list(params: &OpdErOrderMasterParams, intern_roles: &[String], pool: &Pool<MySql>, hosxp: &str, kphis: &str) -> Result<Vec<OpdErOrderMasterList>, AppError> {
    let sql = order_master::select_order_master_list(params, intern_roles, hosxp, kphis);

    let mut query = sqlx::query(AssertSqlSafe(sql));

    if let Some(hn) = params.hn.as_ref() {
        query = query.bind(["%", hn.trim()].concat());
    }
    if let Some(opd_er_order_master_id) = params.opd_er_order_master_id {
        query = query.bind(opd_er_order_master_id);
    }
    if let Some(vn) = params.vn.as_ref() {
        query = query.bind(vn);
    }
    if let Some(vstdate) = params.vstdate.as_ref() {
        query = query.bind(vstdate);
    }
    if let Some(qn) = params.qn.as_ref() {
        query = query.bind(qn);
    }
    if let Some(start_order_date) = params.start_order_date.as_ref() {
        query = query.bind(start_order_date);
    }
    if let Some(end_order_date) = params.end_order_date.as_ref() {
        query = query.bind(end_order_date);
    }
    if let Some(order_date) = params.order_date.as_ref() {
        query = query.bind(order_date);
    }
    if let Some(order_doctor) = params.order_doctor.as_ref() {
        query = query.bind(order_doctor);
    }
    if let Some(bedno) = params.bedno.as_ref() {
        query = query.bind(bedno);
    }
    // if let Some(vn) = params.vn.as_ref() {
    //     query = query.bind(vn);
    // }
    if let Some(status_id) = params.er_patient_status_id {
        if status_id == 7 {
            if let Some(er_dch_type_id) = params.er_dch_type_id {
                query = query.bind(er_dch_type_id);
            }
        }
    }

    query
        .fetch_all(pool)
        .await
        .map(|rows| {
            rows.iter()
                .map(OpdErOrderMasterList::from_row)
                .collect::<sqlx::Result<Vec<OpdErOrderMasterList>>>()
                .map_err(|e| Source::SQLx.to_error(500, e, "Select OrderMasterList"))
        })
        .map_err(|e| Source::SQLx.to_error(500, e, "Select OrderMasterList"))?
}

// opd-er-order-master-data.php
pub async fn get_order_master(opd_er_order_master_id: u32, intern_roles: &[String], pool: &Pool<MySql>, hosxp: &str, kphis: &str) -> Result<Option<OpdErOrderMaster>, AppError> {
    let sql = order_master::select_order_master(intern_roles, hosxp, kphis);
    sqlx::query(AssertSqlSafe(sql))
        .bind(opd_er_order_master_id)
        .fetch_optional(pool)
        .await
        .map_err(|e| Source::SQLx.to_error(500, e, "Select OrderMaster"))?
        .as_ref()
        .map(OpdErOrderMaster::from_row)
        .transpose()
        .map_err(|e| Source::SQLx.to_error(500, e, "Select OrderMaster"))
}

// opd-er-order-master-save.php
pub async fn post_order_master(save: &OpdErOrderMasterSave, user: &str, doctorcode: &Option<String>, pool: &Pool<MySql>, kphis: &str) -> Result<(u32, ExecuteResponse), AppError> {
    if let Some(vn) = save.vn.as_ref() {
        let exs = exists_order_master_by_vn(vn, save.opd_er_order_master_id, pool, kphis).await?;
        if exs {
            return Ok((0, Source::App.to_error(500, "VN Already In Used", "Select CountOrderMasterByVn").into()));
        }
    }
    if let Some(bedno) = save.bedno {
        let exs = exists_order_master_by_bedno(bedno, save.opd_er_order_master_id, pool, kphis).await?;
        if exs {
            return Ok((0, Source::App.to_error(500, "Bedno Already In Used", "Select CountOrderMasterByBedNo").into()));
        }
    }

    if let Some(opd_er_order_master_id) = save.opd_er_order_master_id {
        if opd_er_order_master_id > 0 {
            let is_now_pass_24_hrs_from_opd_er_dch = is_now_pass_24_hrs_from_opd_er_discharge(opd_er_order_master_id, pool, kphis).await?;
            if !is_now_pass_24_hrs_from_opd_er_dch {
                let result = update_order_master(save, opd_er_order_master_id, user, doctorcode, pool, kphis).await?;
                Ok((opd_er_order_master_id, result))
            } else {
                Ok((0, Source::App.to_error(500, "Already pass 24 hrs after discharge", "Select IsPass24Hr").into()))
            }
        } else {
            let result = insert_order_master(save, user, doctorcode, pool, kphis).await?;
            Ok((result.last_insert_id as u32, result))
        }
    } else {
        let result = insert_order_master(save, user, doctorcode, pool, kphis).await?;
        Ok((result.last_insert_id as u32, result))
    }
}

pub async fn exists_order_master_by_vn(vn: &str, exclude_opd_er_order_master_id: Option<u32>, pool: &Pool<MySql>, kphis: &str) -> Result<bool, AppError> {
    let exists_by_vn_sql = order_master::exists_order_master_by_vn(exclude_opd_er_order_master_id.is_some(), kphis);
    let mut query = sqlx::query(AssertSqlSafe(exists_by_vn_sql)).bind(vn);
    if let Some(opd_er_order_master_id) = exclude_opd_er_order_master_id {
        query = query.bind(opd_er_order_master_id);
    }
    query
        .fetch_one(pool)
        .await
        .map(|row| row.try_get(0).unwrap_or_default())
        .map_err(|e| Source::SQLx.to_error(500, e, "Select ExistsOrderMasterByVn"))
}

async fn exists_order_master_by_bedno(bedno: u32, exclude_opd_er_order_master_id: Option<u32>, pool: &Pool<MySql>, kphis: &str) -> Result<bool, AppError> {
    let exists_by_bedno_sql = order_master::exists_order_master_by_bedno(exclude_opd_er_order_master_id.is_some(), kphis);
    let mut query = sqlx::query(AssertSqlSafe(exists_by_bedno_sql)).bind(bedno);
    if let Some(opd_er_order_master_id) = exclude_opd_er_order_master_id {
        query = query.bind(opd_er_order_master_id);
    }
    query
        .fetch_one(pool)
        .await
        .map(|row| row.try_get(0).unwrap_or_default())
        .map_err(|e| Source::SQLx.to_error(500, e, "Select ExistsOrderMasterByBedNo"))
}

async fn is_now_pass_24_hrs_from_opd_er_discharge(opd_er_order_master_id: u32, pool: &Pool<MySql>, kphis: &str) -> Result<bool, AppError> {
    let dch_over_24_sql = query_utils::is_now_pass_24_hrs_from_opd_er_discharge(kphis);
    sqlx::query(AssertSqlSafe(dch_over_24_sql))
        .bind(opd_er_order_master_id)
        .fetch_optional(pool)
        .await
        .map_err(|e| Source::SQLx.to_error(500, e, "Select IsPass24Hr"))?
        .map(|row| row.try_get::<bool, &str>("pass_24_hour_from_dch"))
        .transpose()
        .map(|opt| opt.unwrap_or_default())
        .map_err(|e| Source::SQLx.to_error(500, e, "Select IsPass24Hr"))
}

pub async fn insert_order_master(
    save: &OpdErOrderMasterSave,
    // admit_flag: &Option<String>,
    user: &str,
    doctorcode: &Option<String>,
    pool: &Pool<MySql>,
    kphis: &str,
) -> Result<ExecuteResponse, AppError> {
    let sql = order_master::insert_order_master(kphis);
    let result = sqlx::query(AssertSqlSafe(sql))
        .bind(&save.note)
        .bind(&save.vn)
        // .bind(&save.an)
        .bind(save.bedno)
        .bind(doctorcode.clone().unwrap_or_default())
        // .bind(admit_flag)
        .bind(user)
        .bind(user)
        .execute(pool)
        .await
        .map_err(|e| Source::SQLx.to_error(500, e, "Insert OrderMasterSave"))?;

    Ok(ExecuteResponse::from_query_result(result, "Insert OrderMasterSave"))
}

async fn update_order_master(save: &OpdErOrderMasterSave, opd_er_order_master_id: u32, user: &str, doctorcode: &Option<String>, pool: &Pool<MySql>, kphis: &str) -> Result<ExecuteResponse, AppError> {
    let sql = order_master::update_order_master(kphis);
    let result = sqlx::query(AssertSqlSafe(sql))
        .bind(&save.note)
        .bind(&save.vn)
        // .bind(&save.an)
        .bind(save.bedno)
        .bind(save.er_patient_status_id)
        .bind(save.er_dch_type_id)
        .bind(doctorcode.clone().unwrap_or_default())
        .bind(user)
        .bind(save.discharge_date)
        .bind(save.discharge_time)
        .bind(opd_er_order_master_id)
        .execute(pool)
        .await
        .map_err(|e| Source::SQLx.to_error(500, e, "Update OrderMasterSave"))?;

    Ok(ExecuteResponse::from_query_result(result, "Update OrderMasterSave"))
}

#[cfg(test)]
#[rustfmt::skip]
mod tests {

    use super::*;
    use kphis_sqlx_tester::MySqlTester;
    use kphis_util::datetime::date_8601;

    #[tokio::test]
    #[ignore]
    async fn sqlx_get_order_master_check() {
        let tester = MySqlTester::new_kphis().await;
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis/opd_er_order_master.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/kphis/opd_er_order_master.sql")).execute(&tester.db_pool).await.unwrap();

        // delete_flag <> 'Y'
        let found = get_order_master_check("670111111111", &tester.db_pool, &tester.kphis).await.unwrap();
        assert_eq!(found.len(), 1);
        let not_found = get_order_master_check("991231235959", &tester.db_pool, &tester.kphis).await.unwrap();
        assert!(not_found.is_empty());
    }

    #[tokio::test]
    #[ignore]
    async fn sqlx_get_order_master_list() {
        let tester = MySqlTester::new_hosxp_and_kphis().await;
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis/opd_er_order_master.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis/opd_er_order.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis/opd_er_order_item.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis/opd_er_vs_vital_sign.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis/opd_er_bed.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis/opd_er_bed_type.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis/opd_er_patient_status.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis/opd_er_dch_type.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis/system_ac_role.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis/system_acc_role_user.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/hosxp/ovst.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/hosxp/vn_stat.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/hosxp/doctor.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/hosxp/patient.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/hosxp/sex.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/hosxp/pttype.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/hosxp/opduser.sql")).execute(&tester.db_pool).await.unwrap();

        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/kphis/opd_er_order_master.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/kphis/opd_er_order.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/kphis/opd_er_order_item.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/kphis/opd_er_vs_vital_sign.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/kphis/opd_er_bed.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/kphis/opd_er_bed_type.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/kphis/opd_er_patient_status.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/kphis/opd_er_dch_type.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/kphis/system_ac_role.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/kphis/system_acc_role_user.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/hosxp/ovst.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/hosxp/vn_stat.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/hosxp/doctor.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/hosxp/patient.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/hosxp/sex.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/hosxp/pttype.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/hosxp/opduser.sql")).execute(&tester.db_pool).await.unwrap();

        // delete_flag <> 'Y'
        let default = get_order_master_list(&OpdErOrderMasterParams::default(),&[String::from("DOCTOR_INTERN")],&tester.db_pool,&tester.hosxp,&tester.kphis).await.unwrap();
        assert_eq!(default.len(), 4);
        let found_opd_er_order_master_id = get_order_master_list(&OpdErOrderMasterParams {opd_er_order_master_id: Some(1),..Default::default()},&[String::from("DOCTOR_INTERN")],&tester.db_pool,&tester.hosxp,&tester.kphis).await.unwrap();
        assert_eq!(found_opd_er_order_master_id.len(), 1);

        let found_hn = get_order_master_list(&OpdErOrderMasterParams {hn: Some(String::from("0001234")),..Default::default()},&[String::from("DOCTOR_INTERN")],&tester.db_pool,&tester.hosxp,&tester.kphis).await.unwrap();
        assert_eq!(found_hn.len(), 4);
        let found_vn = get_order_master_list(&OpdErOrderMasterParams {vn: Some(String::from("670111111111")),..Default::default()},&[String::from("DOCTOR_INTERN")],&tester.db_pool,&tester.hosxp,&tester.kphis).await.unwrap();
        assert_eq!(found_vn.len(), 1);
        let found_qn = get_order_master_list(&OpdErOrderMasterParams {qn: Some(String::from("1")),..Default::default()},&[String::from("DOCTOR_INTERN")],&tester.db_pool,&tester.hosxp,&tester.kphis).await.unwrap();
        assert_eq!(found_qn.len(), 1);

        let found_vstdate = get_order_master_list(&OpdErOrderMasterParams {vstdate: date_8601("2024-01-01"),..Default::default()},&[String::from("DOCTOR_INTERN")],&tester.db_pool,&tester.hosxp,&tester.kphis).await.unwrap();
        assert_eq!(found_vstdate.len(), 1);
        let found_start_order_date = get_order_master_list(&OpdErOrderMasterParams {start_order_date: date_8601("2024-01-11"),..Default::default()},&[String::from("DOCTOR_INTERN")],&tester.db_pool,&tester.hosxp,&tester.kphis).await.unwrap();
        assert_eq!(found_start_order_date.len(), 2);
        let found_end_order_date = get_order_master_list(&OpdErOrderMasterParams {end_order_date: date_8601("2024-01-11"),..Default::default()},&[String::from("DOCTOR_INTERN")],&tester.db_pool,&tester.hosxp,&tester.kphis).await.unwrap();
        assert_eq!(found_end_order_date.len(), 3);
        let found_between_order_date = get_order_master_list(&OpdErOrderMasterParams {start_order_date: date_8601("2024-01-11"),end_order_date: date_8601("2024-01-11"),..Default::default()},&[String::from("DOCTOR_INTERN")],&tester.db_pool,&tester.hosxp,&tester.kphis).await.unwrap();
        assert_eq!(found_between_order_date.len(), 1);
        let found_order_date = get_order_master_list(&OpdErOrderMasterParams {order_date: date_8601("2024-01-11"),..Default::default()},&[String::from("DOCTOR_INTERN")],&tester.db_pool,&tester.hosxp,&tester.kphis).await.unwrap();
        assert_eq!(found_order_date.len(), 1);

        let found_order_doctor = get_order_master_list(&OpdErOrderMasterParams {order_doctor: Some(String::from("007")),..Default::default()},&[String::from("DOCTOR_INTERN")],&tester.db_pool,&tester.hosxp,&tester.kphis).await.unwrap();
        assert_eq!(found_order_doctor.len(), 4);
        let found_bedno = get_order_master_list(&OpdErOrderMasterParams {bedno: Some(String::from("109")),..Default::default()},&[String::from("DOCTOR_INTERN")],&tester.db_pool,&tester.hosxp,&tester.kphis).await.unwrap();
        assert_eq!(found_bedno.len(), 4);

        let found_er_patient_status_id_not_7 = get_order_master_list(&OpdErOrderMasterParams {er_patient_status_id: Some(1),..Default::default()},&[String::from("DOCTOR_INTERN")],&tester.db_pool,&tester.hosxp,&tester.kphis).await.unwrap();
        assert_eq!(found_er_patient_status_id_not_7.len(), 2);
        let found_er_patient_status_id_7 = get_order_master_list(&OpdErOrderMasterParams {er_patient_status_id: Some(7),..Default::default()},&[String::from("DOCTOR_INTERN")],&tester.db_pool,&tester.hosxp,&tester.kphis).await.unwrap();
        assert_eq!(found_er_patient_status_id_7.len(), 2);
        let found_er_patient_status_id_7_with_er_dch_type_id = get_order_master_list(&OpdErOrderMasterParams {er_patient_status_id: Some(7),er_dch_type_id: Some(1),..Default::default()},&[String::from("DOCTOR_INTERN")],&tester.db_pool,&tester.hosxp,&tester.kphis).await.unwrap();
        assert_eq!(found_er_patient_status_id_7_with_er_dch_type_id.len(), 1);

        let not_found = get_order_master_list(&OpdErOrderMasterParams {opd_er_order_master_id: Some(999),..Default::default()},&[String::from("DOCTOR_INTERN")],&tester.db_pool,&tester.hosxp,&tester.kphis).await.unwrap();
        assert!(not_found.is_empty());
    }

    #[tokio::test]
    #[ignore]
    async fn sqlx_get_order_master() {
        let tester = MySqlTester::new_hosxp_and_kphis().await;
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis/opd_er_order_master.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis/opd_er_bed.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis/opd_er_bed_type.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis/opd_er_patient_status.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis/opd_er_dch_type.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis/opd_er_allergy_history.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis/system_ac_role.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis/system_acc_role_user.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/hosxp/ovst.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/hosxp/vn_stat.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/hosxp/patient.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/hosxp/doctor.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/hosxp/ipt.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/hosxp/opduser.sql")).execute(&tester.db_pool).await.unwrap();

        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/kphis/opd_er_order_master.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/kphis/opd_er_bed.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/kphis/opd_er_bed_type.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/kphis/opd_er_patient_status.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/kphis/opd_er_dch_type.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/kphis/opd_er_allergy_history.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/kphis/system_ac_role.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/kphis/system_acc_role_user.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/hosxp/ovst.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/hosxp/vn_stat.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/hosxp/patient.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/hosxp/doctor.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/hosxp/ipt.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/hosxp/opduser.sql")).execute(&tester.db_pool).await.unwrap();

        let found = get_order_master(1, &[String::from("DOCTOR_INTERN")],&tester.db_pool, &tester.hosxp, &tester.kphis).await.unwrap();
        assert!(found.is_some());
        let not_found = get_order_master(999, &[String::from("DOCTOR_INTERN")],&tester.db_pool, &tester.hosxp, &tester.kphis).await.unwrap();
        assert!(not_found.is_none());
    }

    #[tokio::test]
    #[ignore]
    async fn sqlx_exists_order_master_by_vn() {
        let tester = MySqlTester::new_kphis().await;
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis/opd_er_order_master.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/kphis/opd_er_order_master.sql")).execute(&tester.db_pool).await.unwrap();

        let found_all = exists_order_master_by_vn("670111111111", None, &tester.db_pool, &tester.kphis).await.unwrap();
        assert!(found_all);
        let found_exclude = exists_order_master_by_vn("670111111111", Some(3), &tester.db_pool, &tester.kphis).await.unwrap();
        assert!(found_exclude);
        let not_found = exists_order_master_by_vn("990111111111", None, &tester.db_pool, &tester.kphis).await.unwrap();
        assert!(!not_found);
    }
    #[tokio::test]
    #[ignore]
    async fn sqlx_exists_order_master_by_bedno() {
        let tester = MySqlTester::new_kphis().await;
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis/opd_er_order_master.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/kphis/opd_er_order_master.sql")).execute(&tester.db_pool).await.unwrap();

        let found_all = exists_order_master_by_bedno(109, None, &tester.db_pool, &tester.kphis).await.unwrap();
        assert!(found_all);
        let found_exclude = exists_order_master_by_bedno(109, Some(3), &tester.db_pool, &tester.kphis).await.unwrap();
        assert!(found_exclude);
        let not_found = exists_order_master_by_bedno(999, None, &tester.db_pool, &tester.kphis).await.unwrap();
        assert!(!not_found);
    }
    #[tokio::test]
    #[ignore]
    async fn sqlx_is_now_pass_24_hrs_from_opd_er_discharge() {
        let tester = MySqlTester::new_kphis().await;
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis/opd_er_order_master.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/kphis/opd_er_order_master.sql")).execute(&tester.db_pool).await.unwrap();

        let found_not_discharge = is_now_pass_24_hrs_from_opd_er_discharge(5, &tester.db_pool, &tester.kphis).await.unwrap();
        assert!(!found_not_discharge);
        let found_discharge_over_24hr = is_now_pass_24_hrs_from_opd_er_discharge(1, &tester.db_pool, &tester.kphis).await.unwrap();
        assert!(found_discharge_over_24hr);
        let found_discharge_under_24hr = is_now_pass_24_hrs_from_opd_er_discharge(4, &tester.db_pool, &tester.kphis).await.unwrap();
        assert!(!found_discharge_under_24hr);
        let not_found = is_now_pass_24_hrs_from_opd_er_discharge(999, &tester.db_pool, &tester.kphis).await.unwrap();
        assert!(!not_found);
    }

    #[tokio::test]
    #[ignore]
    async fn sqlx_insert_order_master() {
        let tester = MySqlTester::new_kphis().await;
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis/opd_er_order_master.sql")).execute(&tester.db_pool).await.unwrap();

        let success = insert_order_master(&OpdErOrderMasterSave::demo(), "user",&None,&tester.db_pool,&tester.kphis).await.unwrap();
        assert_eq!(success.rows_affected, 1);
        let again_success = insert_order_master(&OpdErOrderMasterSave::demo(), "user",&Some(String::from("007")),&tester.db_pool,&tester.kphis).await.unwrap();
        assert_eq!(again_success.rows_affected, 1);
    }

    #[tokio::test]
    #[ignore]
    async fn sqlx_update_order_master() {
        let tester = MySqlTester::new_kphis().await;
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis/opd_er_order_master.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/kphis/opd_er_order_master.sql")).execute(&tester.db_pool).await.unwrap();

        let success = update_order_master(&OpdErOrderMasterSave::demo(),1,"user",&None,&tester.db_pool,&tester.kphis).await.unwrap();
        assert_eq!(success.rows_affected, 1);
        let again_success = update_order_master(&OpdErOrderMasterSave::demo(),1,"user",&None,&tester.db_pool,&tester.kphis).await.unwrap();
        assert_eq!(again_success.rows_affected, 1);
    }
}
