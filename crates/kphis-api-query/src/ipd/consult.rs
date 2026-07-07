use sqlx::{AssertSqlSafe, FromRow, MySql, Pool, Row, mysql::MySqlQueryResult, types::time::PrimitiveDateTime};

use kphis_model::{
    fetch::ExecuteResponse,
    ipd::consult::{Consult, ConsultSave, ConsultWithName, IpdConsultList, IpdConsultListParams},
};
use kphis_sql::{
    data_history_utils::{KeyValue, SourceTable},
    ipd::consult,
};
use kphis_util::{
    datetime::now,
    error::{AppError, Source},
    util::zero_none,
};

use crate::{log::insert_history_log, query_all, query1_all, query2_all, query3_all, query4_all, query5_all, query6_all};

// ipd-consult-list-table.php
pub async fn get_ipd_consult_list(request: IpdConsultListParams, hn_len: usize, an_len: usize, pool: &Pool<MySql>, hosxp: &str, kphis: &str) -> Result<Vec<IpdConsultList>, AppError> {
    let patient = request.patient.as_ref().and_then(|s| urlencoding::decode(s).map(|s| s.into_owned()).ok()).unwrap_or_default();
    let spclty = request.spclty.clone().unwrap_or_default();
    let c_status = request.search_consult_status.clone().unwrap_or_default();
    let c_search = request.consult_dr_search.clone().unwrap_or_default();
    let cr_search = request.consult_dr_reply_search.clone().unwrap_or_default();
    let c_emergency = request.search_consult_emergency.clone().unwrap_or_default();
    let patient_wildcard = ["%", &patient, "%"].concat();
    let (sql, filter) = consult::sql_and_filter(request.clone(), hn_len, an_len, hosxp, kphis);
    let rows = match (
        filter.has_patient,
        filter.pt_is_num,
        filter.anlen_eq_hnlen,
        filter.has_spclty,
        filter.has_search_consult_status,
        filter.has_consult,
        filter.has_consult_dr_reply_search,
        filter.has_search_consult_emergency,
    ) {
        (true, true, false, _, _, _, _, _) => query1_all(&patient, &sql, pool, "Select IpdConsultList-1").await,
        (true, false, true, _, _, _, _, _) => query2_all(&patient_wildcard, &patient_wildcard, &sql, pool, "Select IpdConsultList-2").await,
        (true, _, _, _, _, _, _, _) => query1_all(&patient_wildcard, &sql, pool, "Select IpdConsultList-3").await,
        (false, _, _, true, true, true, true, true) => query6_all(&spclty, &c_status, &c_search, &cr_search, &cr_search, &c_emergency, &sql, pool, "Select IpdConsultList-4").await,
        (false, _, _, true, true, true, true, false) => query5_all(&spclty, &c_status, &c_search, &cr_search, &cr_search, &sql, pool, "Select IpdConsultList-5").await,
        (false, _, _, true, true, true, false, true) => query4_all(&spclty, &c_status, &c_search, &c_emergency, &sql, pool, "Select IpdConsultList-6").await,
        (false, _, _, true, true, true, false, false) => query3_all(&spclty, &c_status, &c_search, &sql, pool, "Select IpdConsultList-7").await,
        (false, _, _, true, true, false, true, true) => query5_all(&spclty, &c_status, &cr_search, &cr_search, &c_emergency, &sql, pool, "Select IpdConsultList-8").await,
        (false, _, _, true, true, false, true, false) => query4_all(&spclty, &c_status, &cr_search, &cr_search, &sql, pool, "Select IpdConsultList-9").await,
        (false, _, _, true, true, false, false, true) => query3_all(&spclty, &c_status, &c_emergency, &sql, pool, "Select IpdConsultList-10").await,
        (false, _, _, true, true, false, false, false) => query2_all(&spclty, &c_status, &sql, pool, "Select IpdConsultList-11").await,
        (false, _, _, true, false, true, true, true) => query5_all(&spclty, &c_search, &cr_search, &cr_search, &c_emergency, &sql, pool, "Select IpdConsultList-12").await,
        (false, _, _, true, false, true, true, false) => query4_all(&spclty, &c_search, &cr_search, &cr_search, &sql, pool, "Select IpdConsultList-13").await,
        (false, _, _, true, false, true, false, true) => query3_all(&spclty, &c_search, &c_emergency, &sql, pool, "Select IpdConsultList-14").await,
        (false, _, _, true, false, true, false, false) => query2_all(&spclty, &c_search, &sql, pool, "Select IpdConsultList-15").await,
        (false, _, _, true, false, false, true, true) => query4_all(&spclty, &cr_search, &cr_search, &c_emergency, &sql, pool, "Select IpdConsultList-16").await,
        (false, _, _, true, false, false, true, false) => query3_all(&spclty, &cr_search, &cr_search, &sql, pool, "Select IpdConsultList-17").await,
        (false, _, _, true, false, false, false, true) => query2_all(&spclty, &c_emergency, &sql, pool, "Select IpdConsultList-18").await,
        (false, _, _, true, false, false, false, false) => query1_all(&spclty, &sql, pool, "Select IpdConsultList-19").await,
        (false, _, _, false, true, true, true, true) => query5_all(&c_status, &c_search, &cr_search, &cr_search, &c_emergency, &sql, pool, "Select IpdConsultList-20").await,
        (false, _, _, false, true, true, true, false) => query4_all(&c_status, &c_search, &cr_search, &cr_search, &sql, pool, "Select IpdConsultList-21").await,
        (false, _, _, false, true, true, false, true) => query3_all(&c_status, &c_search, &c_emergency, &sql, pool, "Select IpdConsultList-22").await,
        (false, _, _, false, true, true, false, false) => query2_all(&c_status, &c_search, &sql, pool, "Select IpdConsultList-23").await,
        (false, _, _, false, true, false, true, true) => query4_all(&c_status, &cr_search, &cr_search, &c_emergency, &sql, pool, "Select IpdConsultList-24").await,
        (false, _, _, false, true, false, true, false) => query3_all(&c_status, &cr_search, &cr_search, &sql, pool, "Select IpdConsultList-25").await,
        (false, _, _, false, true, false, false, true) => query2_all(&c_status, &c_emergency, &sql, pool, "Select IpdConsultList-26").await,
        (false, _, _, false, true, false, false, false) => query1_all(&c_status, &sql, pool, "Select IpdConsultList-27").await,
        (false, _, _, false, false, true, true, true) => query4_all(&c_search, &cr_search, &cr_search, &c_emergency, &sql, pool, "Select IpdConsultList-28").await,
        (false, _, _, false, false, true, true, false) => query3_all(&c_search, &cr_search, &cr_search, &sql, pool, "Select IpdConsultList-29").await,
        (false, _, _, false, false, true, false, true) => query2_all(&c_search, &c_emergency, &sql, pool, "Select IpdConsultList-30").await,
        (false, _, _, false, false, true, false, false) => query1_all(&c_search, &sql, pool, "Select IpdConsultList-31").await,
        (false, _, _, false, false, false, true, true) => query3_all(&cr_search, &cr_search, &c_emergency, &sql, pool, "Select IpdConsultList-32").await,
        (false, _, _, false, false, false, true, false) => query2_all(&cr_search, &cr_search, &sql, pool, "Select IpdConsultList-33").await,
        (false, _, _, false, false, false, false, true) => query1_all(&c_emergency, &sql, pool, "Select IpdConsultList-34").await,
        (false, _, _, false, false, false, false, false) => query_all(&sql, pool, "Select IpdConsultList-35").await,
    }?;

    rows.iter()
        .map(IpdConsultList::from_row)
        .collect::<sqlx::Result<Vec<IpdConsultList>>>()
        .map_err(|e| Source::SQLx.to_error(500, e, "Select IpdConsultList"))
}

// ipd-dr-consult-data.php
pub async fn get_ipd_consult_by_an(an: &str, pool: &Pool<MySql>, hosxp: &str, kphis: &str, kphis_extra: &str) -> Result<Vec<ConsultWithName>, AppError> {
    let sql = consult::select_consult_by_an(hosxp, kphis, kphis_extra);

    query1_all(an, &&sql, pool, "Select ConsultWithName")
        .await?
        .iter()
        .map(ConsultWithName::from_row)
        .collect::<sqlx::Result<Vec<ConsultWithName>>>()
        .map_err(|e| Source::SQLx.to_error(500, e, "Select ConsultWithName"))
}

// ipd-dr-consult-edit.php
pub async fn get_ipd_consult_by_id(consult_id: u32, pool: &Pool<MySql>, hosxp: &str, kphis: &str) -> Result<Option<Consult>, AppError> {
    let sql = consult::select_consult_by_id(hosxp, kphis);

    sqlx::query(AssertSqlSafe(sql))
        .bind(consult_id)
        .fetch_optional(pool)
        .await
        .map_err(|e| Source::SQLx.to_error(500, e, "Select Consult"))?
        .as_ref()
        .map(Consult::from_row)
        .transpose()
        .map_err(|e| Source::SQLx.to_error(500, e, "Select Consult"))
}

// ipd-dr-consult-save.php
// ipd-dr-consult-update.php
pub async fn post_ipd_consult(save: &ConsultSave, loginname: &str, pool: &Pool<MySql>, kphis: &str, kphis_log: &str) -> Result<(u32, Vec<ExecuteResponse>), AppError> {
    let consult_id;
    let mut results = Vec::new();

    if let Some(id) = save.consult_id.and_then(zero_none) {
        consult_id = id;
        // ipd-dr-consult-update.php
        match save.consult_mode.as_str() {
            "edit" => {
                let update_consult_result = update_consult_request(consult_id, save, loginname, pool, kphis).await?;
                let update_affectd = update_consult_result.rows_affected();
                results.push(ExecuteResponse::from_query_result(update_consult_result, "Update ConsultRequest"));

                if update_affectd > 0 {
                    let insert_consult_history_result =
                        insert_history_log(SourceTable::IpdDrConsult, "U", loginname, &[KeyValue("consult_id", consult_id.to_string())], kphis, kphis_log, pool).await?;
                    results.push(ExecuteResponse::from_query_result(insert_consult_history_result, "Insert Consult History"));

                    let delete_sig_request_result = delete_consult_signature_request(consult_id, pool, kphis).await?;
                    results.push(ExecuteResponse::from_query_result(delete_sig_request_result, "Delete ConsultSignatureRequest"));

                    results.extend(insert_consult_signature_request(consult_id, save, loginname, pool, kphis).await?);

                    let insert_sig_request_history_result = insert_history_log(
                        SourceTable::IpdDrConsultSignatureRequest,
                        "U",
                        loginname,
                        &[KeyValue("consult_id", consult_id.to_string())],
                        kphis,
                        kphis_log,
                        pool,
                    )
                    .await?;
                    results.push(ExecuteResponse::from_query_result(insert_sig_request_history_result, "Insert ConsultSignatureRequest History"));
                }
            }
            "reply" => {
                let consult_status = if save.consult_finding.is_some() || save.consult_recommendation.is_some() { "Y" } else { "N" };
                let consult_datetime = select_consult_datetime(consult_id, pool, kphis).await?;
                let update_consult_result = update_consult_reply(consult_id, consult_datetime, consult_status, save, loginname, pool, kphis).await?;
                let update_affected = update_consult_result.rows_affected();
                results.push(ExecuteResponse::from_query_result(update_consult_result, "Update ConsultReply"));

                if update_affected > 0 {
                    let insert_consult_history_result =
                        insert_history_log(SourceTable::IpdDrConsult, "U", loginname, &[KeyValue("consult_id", consult_id.to_string())], kphis, kphis_log, pool).await?;
                    results.push(ExecuteResponse::from_query_result(insert_consult_history_result, "Insert Consult History"));

                    let delete_sig_reply_result = delete_consult_signature_reply(consult_id, pool, kphis).await?;
                    results.push(ExecuteResponse::from_query_result(delete_sig_reply_result, "Delete ConsultSignatureReply"));

                    results.extend(insert_consult_signature_reply(consult_id, save, loginname, pool, kphis).await?);

                    let insert_sig_reply_history_result = insert_history_log(
                        SourceTable::IpdDrConsultSignatureReply,
                        "U",
                        loginname,
                        &[KeyValue("consult_id", consult_id.to_string())],
                        kphis,
                        kphis_log,
                        pool,
                    )
                    .await?;
                    results.push(ExecuteResponse::from_query_result(insert_sig_reply_history_result, "Insert ConsultSignatureReply History"));
                }
            }
            _ => {}
        }
    } else {
        // ipd-dr-consult-save.php
        let insert_consult_result = insert_consult_request(save, loginname, pool, kphis).await?;
        consult_id = insert_consult_result.last_insert_id() as u32;
        results.push(ExecuteResponse::from_query_result(insert_consult_result, "Insert ConsultRequest"));

        let insert_consult_history_result = insert_history_log(SourceTable::IpdDrConsult, "I", loginname, &[KeyValue("consult_id", consult_id.to_string())], kphis, kphis_log, pool).await?;
        results.push(ExecuteResponse::from_query_result(insert_consult_history_result, "Insert Consult History"));

        results.extend(insert_consult_signature_request(consult_id, save, loginname, pool, kphis).await?);

        let insert_sig_request_history_result = insert_history_log(
            SourceTable::IpdDrConsultSignatureRequest,
            "I",
            loginname,
            &[KeyValue("consult_id", consult_id.to_string())],
            kphis,
            kphis_log,
            pool,
        )
        .await?;
        results.push(ExecuteResponse::from_query_result(insert_sig_request_history_result, "Insert ConsultSignatureRequest History"));
    }

    Ok((consult_id, results))
}

async fn update_consult_request(id: u32, save: &ConsultSave, loginname: &str, pool: &Pool<MySql>, kphis: &str) -> Result<MySqlQueryResult, AppError> {
    let update_consult_sql = consult::update_consult_request(kphis);
    sqlx::query(AssertSqlSafe(update_consult_sql))
        .bind(save.consult_type)
        .bind(&save.consult_ward)
        .bind(&save.consult_emergency)
        .bind(&save.consult_doctorcode_mention)
        .bind(&save.consult_spclty)
        .bind(save.consult_date)
        .bind(save.consult_time)
        .bind(&save.consult_data)
        .bind(loginname)
        .bind(id)
        .bind(save.version)
        .execute(pool)
        .await
        .map_err(|e| Source::SQLx.to_error(500, e, "Update ConsultRequest"))
}

async fn delete_consult_signature_request(id: u32, pool: &Pool<MySql>, kphis: &str) -> Result<MySqlQueryResult, AppError> {
    let delete_sig_request_sql = consult::delete_consult_signature_request(kphis);
    sqlx::query(AssertSqlSafe(delete_sig_request_sql))
        .bind(id)
        .execute(pool)
        .await
        .map_err(|e| Source::SQLx.to_error(500, e, "Delete ConsultSignatureRequest"))
}

async fn insert_consult_signature_request(id: u32, save: &ConsultSave, loginname: &str, pool: &Pool<MySql>, kphis: &str) -> Result<Vec<ExecuteResponse>, AppError> {
    let insert_sig_request_sql = consult::insert_consult_signature_request(kphis);
    let mut results = Vec::with_capacity(save.consult_doctorcode_requests.len());
    for request in save.consult_doctorcode_requests.iter() {
        let insert_sig_request_result = sqlx::query(AssertSqlSafe(insert_sig_request_sql.clone()))
            .bind(id)
            .bind(&request.person1)
            .bind(&request.person2)
            .bind(&save.an)
            .bind(loginname)
            .bind(loginname)
            .execute(pool)
            .await
            .map_err(|e| Source::SQLx.to_error(500, e, "Insert ConsultSignatureRequest"))?;
        results.push(ExecuteResponse::from_query_result(insert_sig_request_result, "Insert ConsultSignatureRequest"));
    }
    Ok(results)
}

async fn select_consult_datetime(id: u32, pool: &Pool<MySql>, kphis: &str) -> Result<Option<PrimitiveDateTime>, AppError> {
    sqlx::query(AssertSqlSafe(["SELECT consult_datetime_create_reply FROM ", kphis, ".ipd_dr_consult WHERE consult_id=?;"].concat()))
        .bind(id)
        .fetch_optional(pool)
        .await
        .map_err(|e| Source::SQLx.to_error(500, e, "Select CreateReplyDateTime"))?
        .map(|row| row.try_get::<Option<PrimitiveDateTime>, &str>("consult_datetime_create_reply"))
        .transpose()
        .map(|opt| opt.flatten())
        .map_err(|e| Source::Time.to_error(500, e, "Select CreateReplyDateTime"))
}

async fn update_consult_reply(
    id: u32,
    consult_datetime: Option<PrimitiveDateTime>,
    consult_status: &str,
    save: &ConsultSave,
    loginname: &str,
    pool: &Pool<MySql>,
    kphis: &str,
) -> Result<MySqlQueryResult, AppError> {
    let consult_datetime_update_reply = consult_datetime.is_some().then(now);
    let consult_datetime_create_reply = consult_datetime.or(Some(now()));
    let update_consult_sql = consult::update_consult_reply(kphis);
    sqlx::query(AssertSqlSafe(update_consult_sql))
        .bind(consult_datetime_create_reply)
        .bind(consult_datetime_update_reply)
        .bind(&save.consult_finding)
        .bind(&save.consult_diagnosis)
        .bind(&save.consult_recommendation)
        .bind(consult_status)
        .bind(loginname)
        .bind(id)
        .bind(save.version)
        .execute(pool)
        .await
        .map_err(|e| Source::SQLx.to_error(500, e, "Update ConsultReply"))
}

async fn delete_consult_signature_reply(id: u32, pool: &Pool<MySql>, kphis: &str) -> Result<MySqlQueryResult, AppError> {
    let delete_sig_reply_sql = consult::delete_consult_signature_reply(kphis);
    sqlx::query(AssertSqlSafe(delete_sig_reply_sql))
        .bind(id)
        .execute(pool)
        .await
        .map_err(|e| Source::SQLx.to_error(500, e, "Delete ConsultSignatureReply"))
}

async fn insert_consult_signature_reply(id: u32, save: &ConsultSave, loginname: &str, pool: &Pool<MySql>, kphis: &str) -> Result<Vec<ExecuteResponse>, AppError> {
    let insert_sig_reply_sql = consult::insert_consult_signature_reply(kphis);
    let mut results = Vec::with_capacity(save.consult_doctorcode_replies.len());
    for reply in save.consult_doctorcode_replies.iter() {
        let insert_sig_reply_result = sqlx::query(AssertSqlSafe(insert_sig_reply_sql.clone()))
            .bind(id)
            .bind(&reply.person1)
            .bind(&reply.person2)
            .bind(&save.an)
            .bind(loginname)
            .bind(loginname)
            .execute(pool)
            .await
            .map_err(|e| Source::SQLx.to_error(500, e, "Insert ConsultSignatureReply"))?;
        results.push(ExecuteResponse::from_query_result(insert_sig_reply_result, "Insert ConsultSignatureReply"));
    }
    Ok(results)
}

async fn insert_consult_request(save: &ConsultSave, loginname: &str, pool: &Pool<MySql>, kphis: &str) -> Result<MySqlQueryResult, AppError> {
    let insert_consult_sql = consult::insert_consult_request(kphis);
    sqlx::query(AssertSqlSafe(insert_consult_sql))
        .bind(save.consult_type)
        .bind(&save.consult_ward)
        .bind(&save.consult_emergency)
        .bind(&save.consult_doctorcode_mention)
        .bind(&save.consult_spclty)
        .bind(save.consult_date)
        .bind(save.consult_time)
        .bind(&save.consult_data)
        .bind("N")
        .bind(&save.an)
        .bind(loginname)
        .bind(loginname)
        .execute(pool)
        .await
        .map_err(|e| Source::SQLx.to_error(500, e, "Insert ConsultRequest"))
}

// ipd-dr-consult-delete.php
pub async fn delete_ipd_consult_by_id(consult_id: u32, version: i32, loginname: &str, pool: &Pool<MySql>, kphis: &str, kphis_log: &str) -> Result<Vec<ExecuteResponse>, AppError> {
    let mut results = Vec::with_capacity(4);
    let insert_history_result = insert_history_log(
        SourceTable::IpdDrConsult,
        "D",
        loginname,
        &[KeyValue("consult_id", consult_id.to_string()), KeyValue("version", version.to_string())],
        kphis,
        kphis_log,
        pool,
    )
    .await?;
    let inserted = insert_history_result.rows_affected();
    results.push(ExecuteResponse::from_query_result(insert_history_result, "Insert Consult History"));

    if inserted > 0 {
        let delete_result = delete_consult(consult_id, pool, kphis).await?;
        results.push(ExecuteResponse::from_query_result(delete_result, "Delete Consult"));

        let insert_sig_request_result = insert_history_log(
            SourceTable::IpdDrConsultSignatureRequest,
            "D",
            loginname,
            &[KeyValue("consult_id", consult_id.to_string()), KeyValue("version", version.to_string())],
            kphis,
            kphis_log,
            pool,
        )
        .await?;
        let inserted_sig = insert_sig_request_result.rows_affected();
        results.push(ExecuteResponse::from_query_result(insert_sig_request_result, "Insert ConsultSignatureRequest History"));

        if inserted_sig > 0 {
            let delete_sig_request_result = delete_consult_signature_request(consult_id, pool, kphis).await?;
            results.push(ExecuteResponse::from_query_result(delete_sig_request_result, "Delete ConsultSignatureRequest"));
        }
    }

    Ok(results)
}

async fn delete_consult(consult_id: u32, pool: &Pool<MySql>, kphis: &str) -> Result<MySqlQueryResult, AppError> {
    let delete_sql = consult::delete_consult(kphis);
    sqlx::query(AssertSqlSafe(delete_sql))
        .bind(consult_id)
        .execute(pool)
        .await
        .map_err(|e| Source::SQLx.to_error(500, e, "Delete Colsult"))
}

#[cfg(test)]
#[rustfmt::skip]
mod tests {

    use super::*;
    use kphis_sqlx_tester::MySqlTester;

    #[tokio::test]
    #[ignore]
    async fn sqlx_get_ipd_consult_list() {
        let tester = MySqlTester::new_hosxp_and_kphis().await;
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/hosxp/ipt.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/hosxp/iptadm.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/hosxp/patient.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/hosxp/an_stat.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/hosxp/doctor.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/hosxp/ward.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/hosxp/sex.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/hosxp/pttype.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis/ipd_doctor_in_charge.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis/ipd_dr_consult.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis/kphis_spclty.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis/ipd_dr_consult_signature_reply.sql")).execute(&tester.db_pool).await.unwrap();

        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/hosxp/ipt.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/hosxp/iptadm.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/hosxp/patient.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/hosxp/an_stat.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/hosxp/doctor.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/hosxp/ward.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/hosxp/sex.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/hosxp/pttype.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/kphis/ipd_doctor_in_charge.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/kphis/ipd_dr_consult.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/kphis/kphis_spclty.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/kphis/ipd_dr_consult_signature_reply.sql")).execute(&tester.db_pool).await.unwrap();

        // total is 3, but only (ipt.dchstts IS NULL + NOT patient) will match
        let default = get_ipd_consult_list(IpdConsultListParams::default(),7,9,&tester.db_pool,&tester.hosxp,&tester.kphis).await.unwrap();
        assert_eq!(default.len(), 2);

        let params_not_found = IpdConsultListParams {patient: Some(String::from("6666")),..Default::default()};
        let not_found = get_ipd_consult_list(params_not_found,7,9,&tester.db_pool,&tester.hosxp,&tester.kphis).await.unwrap();
        assert!(not_found.is_empty());

        let params_hn = IpdConsultListParams {patient: Some(String::from("1234")),..Default::default()};
        let found_patient_hn = get_ipd_consult_list(params_hn,7,9,&tester.db_pool,&tester.hosxp,&tester.kphis).await.unwrap();
        assert_eq!(found_patient_hn.len(), 3);

        let params_an = IpdConsultListParams {patient: Some(String::from("670001234")),..Default::default()};
        let found_patient_an = get_ipd_consult_list(params_an,7,9,&tester.db_pool,&tester.hosxp,&tester.kphis).await.unwrap();
        assert_eq!(found_patient_an.len(), 2);

        let params_an_dc = IpdConsultListParams {patient: Some(String::from("660001234")),..Default::default()};
        let found_patient_an_dc = get_ipd_consult_list(params_an_dc,7,9,&tester.db_pool,&tester.hosxp,&tester.kphis).await.unwrap();
        assert_eq!(found_patient_an_dc.len(), 1);

        let params_cid = IpdConsultListParams {patient: Some(String::from("1111111111111")),..Default::default()};
        let found_patient_cid = get_ipd_consult_list(params_cid,7,9,&tester.db_pool,&tester.hosxp,&tester.kphis).await.unwrap();
        assert_eq!(found_patient_cid.len(), 3);

        let params_name = IpdConsultListParams {patient: Some(String::from("มุติ")),..Default::default()};
        let found_patient_name = get_ipd_consult_list(params_name,7,9,&tester.db_pool,&tester.hosxp,&tester.kphis).await.unwrap();
        assert_eq!(found_patient_name.len(), 3);

        let params_spclty = IpdConsultListParams {spclty: Some(String::from("1")),..Default::default()};
        let found_spclty = get_ipd_consult_list(params_spclty,7,9,&tester.db_pool,&tester.hosxp,&tester.kphis).await.unwrap();
        assert_eq!(found_spclty.len(), 2);

        let params_status_y = IpdConsultListParams {search_consult_status: Some(String::from("Y")),..Default::default()};
        let found_status_y = get_ipd_consult_list(params_status_y,7,9,&tester.db_pool,&tester.hosxp,&tester.kphis).await.unwrap();
        assert_eq!(found_status_y.len(), 1);

        let params_status_n = IpdConsultListParams {search_consult_status: Some(String::from("N")),..Default::default()};
        let found_status_n = get_ipd_consult_list(params_status_n,7,9,&tester.db_pool,&tester.hosxp,&tester.kphis).await.unwrap();
        assert_eq!(found_status_n.len(), 1);

        let params_mention = IpdConsultListParams {consult_dr_search: Some(String::from("007")),..Default::default()};
        let found_mention = get_ipd_consult_list(params_mention,7,9,&tester.db_pool,&tester.hosxp,&tester.kphis).await.unwrap();
        assert_eq!(found_mention.len(), 2);

        let params_reply_1 = IpdConsultListParams {consult_dr_reply_search: Some(String::from("008")),..Default::default()};
        let found_reply_1 = get_ipd_consult_list(params_reply_1,7,9,&tester.db_pool,&tester.hosxp,&tester.kphis).await.unwrap();
        assert_eq!(found_reply_1.len(), 1);

        let params_reply_2 = IpdConsultListParams {consult_dr_reply_search: Some(String::from("009")),..Default::default()};
        let found_reply_2 = get_ipd_consult_list(params_reply_2,7,9,&tester.db_pool,&tester.hosxp,&tester.kphis).await.unwrap();
        assert_eq!(found_reply_2.len(), 1);

        let params_emer_1 = IpdConsultListParams {search_consult_emergency: Some(String::from("1")),..Default::default()};
        let found_emer_1 = get_ipd_consult_list(params_emer_1,7,9,&tester.db_pool,&tester.hosxp,&tester.kphis).await.unwrap();
        assert_eq!(found_emer_1.len(), 1);

        let params_emer_2 = IpdConsultListParams {search_consult_emergency: Some(String::from("2")),..Default::default()};
        let found_emer_2 = get_ipd_consult_list(params_emer_2,7,9,&tester.db_pool,&tester.hosxp,&tester.kphis).await.unwrap();
        assert_eq!(found_emer_2.len(), 1);
    }

    #[tokio::test]
    #[ignore]
    async fn sqlx_get_ipd_consult_by_an() {
        let tester = MySqlTester::new_hosxp_and_kphis_and_kphis_extra().await;
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis/ipd_dr_consult.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis/ipd_dr_consult_signature_request.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis/ipd_dr_consult_signature_reply.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis/kphis_spclty.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis/ipd_dr_consult_type.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis/ipd_emergency.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/hosxp/doctor.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis_extra/image.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis_extra/image_usage.sql")).execute(&tester.db_pool).await.unwrap();

        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/kphis/ipd_dr_consult.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/kphis/ipd_dr_consult_signature_request.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/kphis/ipd_dr_consult_signature_reply.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/kphis/kphis_spclty.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/kphis/ipd_dr_consult_type.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/kphis/ipd_emergency.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/hosxp/doctor.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/kphis_extra/image.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/kphis_extra/image_usage.sql")).execute(&tester.db_pool).await.unwrap();

        let found = get_ipd_consult_by_an("660001234", &tester.db_pool, &tester.hosxp, &tester.kphis, &tester.kphis_extra).await.unwrap();
        assert_eq!(found.len(), 1);
        let not_found = get_ipd_consult_by_an("660006666", &tester.db_pool, &tester.hosxp, &tester.kphis, &tester.kphis_extra).await.unwrap();
        assert!(not_found.is_empty());
    }

    #[tokio::test]
    #[ignore]
    async fn sqlx_get_ipd_consult_by_id() {
        let tester = MySqlTester::new_hosxp_and_kphis().await;
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis/ipd_dr_consult.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis/ipd_dr_consult_signature_request.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis/ipd_dr_consult_signature_reply.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/hosxp/doctor.sql")).execute(&tester.db_pool).await.unwrap();

        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/kphis/ipd_dr_consult.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/kphis/ipd_dr_consult_signature_request.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/kphis/ipd_dr_consult_signature_reply.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/hosxp/doctor.sql")).execute(&tester.db_pool).await.unwrap();

        let found = get_ipd_consult_by_id(1, &tester.db_pool, &tester.hosxp, &tester.kphis).await.unwrap();
        assert!(found.is_some());
        let not_found = get_ipd_consult_by_id(9, &tester.db_pool, &tester.hosxp, &tester.kphis).await.unwrap();
        assert!(not_found.is_none());
    }

    #[tokio::test]
    #[ignore]
    async fn sqlx_select_consult_datetime() {
        let tester = MySqlTester::new_kphis().await;
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis/ipd_dr_consult.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/kphis/ipd_dr_consult.sql")).execute(&tester.db_pool).await.unwrap();

        let found = select_consult_datetime(1, &tester.db_pool, &tester.kphis).await.unwrap();
        assert!(found.is_some());
        let not_found = select_consult_datetime(9, &tester.db_pool, &tester.kphis).await.unwrap();
        assert!(not_found.is_none());
    }

    #[tokio::test]
    #[ignore]
    async fn sqlx_insert_consult_request() {
        let tester = MySqlTester::new_kphis().await;
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis/ipd_dr_consult.sql")).execute(&tester.db_pool).await.unwrap();

        let success = insert_consult_request(&ConsultSave::demo(), "user", &tester.db_pool, &tester.kphis).await.unwrap();
        assert_eq!(success.rows_affected(), 1);
        let again_success = insert_consult_request(&ConsultSave::demo(), "user", &tester.db_pool, &tester.kphis).await.unwrap();
        assert_eq!(again_success.rows_affected(), 1);
    }

    #[tokio::test]
    #[ignore]
    async fn sqlx_insert_consult_signature_request() {
        let tester = MySqlTester::new_kphis().await;
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis/ipd_dr_consult_signature_request.sql")).execute(&tester.db_pool).await.unwrap();

        let success = insert_consult_signature_request(1,&ConsultSave::demo(),"user",&tester.db_pool,&tester.kphis).await.unwrap();
        assert_eq!(success.len(), 1);
        let again_success = insert_consult_signature_request(1,&ConsultSave::demo(),"user",&tester.db_pool,&tester.kphis).await.unwrap();
        assert_eq!(again_success.len(), 1);
    }

    #[tokio::test]
    #[ignore]
    async fn sqlx_insert_consult_signature_reply() {
        let tester = MySqlTester::new_kphis().await;
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis/ipd_dr_consult_signature_reply.sql")).execute(&tester.db_pool).await.unwrap();

        let success = insert_consult_signature_reply(1,&ConsultSave::demo(),"user",&tester.db_pool,&tester.kphis).await.unwrap();
        assert_eq!(success.len(), 1);
        let success_again = insert_consult_signature_reply(1,&ConsultSave::demo(),"user",&tester.db_pool,&tester.kphis).await.unwrap();
        assert_eq!(success_again.len(), 1);
    }

    #[tokio::test]
    #[ignore]
    async fn sqlx_update_consult_request() {
        let tester = MySqlTester::new_kphis().await;
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis/ipd_dr_consult.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/kphis/ipd_dr_consult.sql")).execute(&tester.db_pool).await.unwrap();

        let replied_fail = update_consult_request(1,&ConsultSave::demo(),"user",&tester.db_pool,&tester.kphis).await.unwrap();
        assert_eq!(replied_fail.rows_affected(), 0);
        let success = update_consult_request(3,&ConsultSave::demo(),"user",&tester.db_pool,&tester.kphis).await.unwrap();
        assert_eq!(success.rows_affected(), 1);
        let again_version_fail = update_consult_request(3,&ConsultSave::demo(),"user",&tester.db_pool,&tester.kphis).await.unwrap();
        assert_eq!(again_version_fail.rows_affected(), 0);
    }

    #[tokio::test]
    #[ignore]
    async fn sqlx_update_consult_reply() {
        let tester = MySqlTester::new_kphis().await;
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis/ipd_dr_consult.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/kphis/ipd_dr_consult.sql")).execute(&tester.db_pool).await.unwrap();

        let success = update_consult_reply(1,Some(now()),"Y",&ConsultSave::demo(),"user",&tester.db_pool,&tester.kphis).await.unwrap();
        assert_eq!(success.rows_affected(), 1);
        let again_version_fail = update_consult_reply(1,Some(now()),"Y",&ConsultSave::demo(),"user",&tester.db_pool,&tester.kphis).await.unwrap();
        assert_eq!(again_version_fail.rows_affected(), 0);
    }

    #[tokio::test]
    #[ignore]
    async fn sqlx_delete_consult() {
        let tester = MySqlTester::new_kphis().await;
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis/ipd_dr_consult.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/kphis/ipd_dr_consult.sql")).execute(&tester.db_pool).await.unwrap();

        let success = delete_consult(1, &tester.db_pool, &tester.kphis).await.unwrap();
        assert_eq!(success.rows_affected(), 1);
        let again_fail = delete_consult(1, &tester.db_pool, &tester.kphis).await.unwrap();
        assert_eq!(again_fail.rows_affected(), 0);
    }

    #[tokio::test]
    #[ignore]
    async fn sqlx_delete_consult_signature_request() {
        let tester = MySqlTester::new_kphis().await;
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis/ipd_dr_consult_signature_request.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/kphis/ipd_dr_consult_signature_request.sql")).execute(&tester.db_pool).await.unwrap();

        let success = delete_consult_signature_request(1, &tester.db_pool, &tester.kphis).await.unwrap();
        assert_eq!(success.rows_affected(), 1);
        let again_fail = delete_consult_signature_request(1, &tester.db_pool, &tester.kphis).await.unwrap();
        assert_eq!(again_fail.rows_affected(), 0);
    }

    #[tokio::test]
    #[ignore]
    async fn sqlx_delete_consult_signature_reply() {
        let tester = MySqlTester::new_kphis().await;
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis/ipd_dr_consult_signature_reply.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/kphis/ipd_dr_consult_signature_reply.sql")).execute(&tester.db_pool).await.unwrap();

        let success = delete_consult_signature_reply(1, &tester.db_pool, &tester.kphis).await.unwrap();
        assert_eq!(success.rows_affected(), 1);
        let again_fail = delete_consult_signature_reply(1, &tester.db_pool, &tester.kphis).await.unwrap();
        assert_eq!(again_fail.rows_affected(), 0);
    }
}
