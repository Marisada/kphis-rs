use sqlx::{AssertSqlSafe, FromRow, MySql, Pool};

use kphis_model::{
    fetch::ExecuteResponse,
    refer_out::{HisReferOut, HisReferOutData, HisReferOutSave, HisReferVitalSign},
};
use kphis_sql::refer_out;
use kphis_util::{
    error::{AppError, Source},
    util::opt_zero_none,
};

use crate::app::bump_and_get_serial;

pub async fn select_his_referout_data(vnan: &str, pool: &Pool<MySql>, hosxp: &str) -> Result<Vec<HisReferOutData>, AppError> {
    let referout_sql = refer_out::select_his_referout(hosxp);
    let referouts = sqlx::query(AssertSqlSafe(referout_sql))
        .bind(vnan)
        .fetch_all(pool)
        .await
        .map_err(|e| Source::SQLx.to_error(500, e, "Select ReferOut"))?
        .iter()
        .map(HisReferOut::from_row)
        .collect::<sqlx::Result<Vec<HisReferOut>>>()
        .map_err(|e| Source::SQLx.to_error(500, e, "Select ReferOut"))?;

    let mut results = Vec::new();
    for referout in referouts {
        let vital_sign_sql = refer_out::select_his_refer_vital_sign(hosxp);
        let vital_signs = sqlx::query(AssertSqlSafe(vital_sign_sql))
            .bind(referout.referout_id)
            .fetch_all(pool)
            .await
            .map_err(|e| Source::SQLx.to_error(500, e, "Select ReferOutVitalSign"))?
            .iter()
            .map(HisReferVitalSign::from_row)
            .collect::<sqlx::Result<Vec<HisReferVitalSign>>>()
            .map_err(|e| Source::SQLx.to_error(500, e, "Select ReferOutVitalSign"))?;

        results.push(HisReferOutData { referout, vital_signs })
    }

    Ok(results)
}

pub async fn post_his_referout(save: &HisReferOutSave, doctorcode: &Option<String>, pool: &Pool<MySql>, hosxp: &str) -> Result<Vec<ExecuteResponse>, AppError> {
    let mut results = Vec::new();

    // UPDATE
    if let Some(referout_id) = opt_zero_none(save.referout_id) {
        let update_referout_result = update_referout(referout_id, save, doctorcode, pool, hosxp).await?;
        results.push(update_referout_result);
        // with old vital_sign
        if let Some(refer_vital_sign_id) = opt_zero_none(save.refer_vital_sign_id) {
            let update_refer_vs_result = update_refer_vital_sign(refer_vital_sign_id, save, pool, hosxp).await?;
            results.push(update_refer_vs_result);
        // without old vital_sign
        } else if let Some(refer_vital_sign_id) = bump_and_get_serial("refer_vital_sign_id", pool, hosxp).await? {
            let insert_refer_vs_result = insert_refer_vital_sign(refer_vital_sign_id, referout_id, save, pool, hosxp).await?;
            results.push(insert_refer_vs_result);
        }
    // INSERT
    } else if let Some(referout_id) = bump_and_get_serial("referout_id", pool, hosxp).await? {
        let insert_referout_result = insert_referout(referout_id, save, doctorcode, pool, hosxp).await?;
        results.push(insert_referout_result);
        // save.refer_vital_sign_id MUST BE NULL here
        if save.refer_vital_sign_id.is_none() && (save.cc.is_some() || save.pe.is_some()) {
            if let Some(refer_vital_sign_id) = bump_and_get_serial("refer_vital_sign_id", pool, hosxp).await? {
                let insert_refer_vs_result = insert_refer_vital_sign(refer_vital_sign_id, referout_id, save, pool, hosxp).await?;
                results.push(insert_refer_vs_result);
            }
        }
    } else {
        return Err(AppError::new_server(500, "Cannot bump 'referout_id' serial", "Post HisReferOut"));
    }

    Ok(results)
}

async fn insert_referout(referout_id: i32, save: &HisReferOutSave, doctorcode: &Option<String>, pool: &Pool<MySql>, hosxp: &str) -> Result<ExecuteResponse, AppError> {
    let sql = refer_out::insert_his_referout(hosxp);
    sqlx::query(AssertSqlSafe(sql))
        .bind(referout_id)
        .bind(&save.vn)
        .bind(&save.hn)
        .bind(&save.refer_hospcode)
        .bind(save.refer_date)
        .bind(save.refer_time)
        .bind(save.due_date)
        .bind(save.expire_date)
        .bind(&save.pre_diagnosis)
        .bind(&save.pmh)
        .bind(&save.hpi)
        .bind(&save.lab_text)
        .bind(&save.treatment_text)
        .bind(&save.other_text)
        .bind(&save.diagnosis_text)
        .bind(&save.request_text)
        .bind(&save.department)
        .bind(&save.pttype)
        .bind(&save.spclty)
        .bind(save.refer_type)
        .bind(save.refer_cause)
        .bind(&save.refer_point)
        .bind(save.moph_refer_expire_type_id)
        .bind(doctorcode)
        .execute(pool)
        .await
        .map(|result| ExecuteResponse::from_query_result(result, "Insert HisReferOutSave"))
        .map_err(|e| Source::SQLx.to_error(500, e, "Insert HisReferOutSave"))
}

async fn update_referout(referout_id: i32, save: &HisReferOutSave, doctorcode: &Option<String>, pool: &Pool<MySql>, hosxp: &str) -> Result<ExecuteResponse, AppError> {
    let sql = refer_out::update_his_referout(hosxp);
    sqlx::query(AssertSqlSafe(sql))
        .bind(&save.refer_hospcode)
        .bind(save.refer_date)
        .bind(save.refer_time)
        .bind(save.due_date)
        .bind(save.expire_date)
        .bind(&save.pre_diagnosis)
        .bind(&save.pmh)
        .bind(&save.hpi)
        .bind(&save.lab_text)
        .bind(&save.treatment_text)
        .bind(&save.other_text)
        .bind(&save.diagnosis_text)
        .bind(&save.request_text)
        .bind(&save.department)
        .bind(&save.pttype)
        .bind(&save.spclty)
        .bind(save.refer_type)
        .bind(save.refer_cause)
        .bind(&save.refer_point)
        .bind(save.moph_refer_expire_type_id)
        .bind(doctorcode)
        .bind(referout_id)
        .execute(pool)
        .await
        .map(|result| ExecuteResponse::from_query_result(result, "Update HisReferOutSave"))
        .map_err(|e| Source::SQLx.to_error(500, e, "Update HisReferOutSave"))
}

async fn insert_refer_vital_sign(refer_vital_sign_id: i32, referout_id: i32, save: &HisReferOutSave, pool: &Pool<MySql>, hosxp: &str) -> Result<ExecuteResponse, AppError> {
    let sql = refer_out::insert_his_refer_vital_sign(hosxp);
    sqlx::query(AssertSqlSafe(sql))
        .bind(refer_vital_sign_id)
        .bind(referout_id)
        .bind(&save.cc)
        .bind(&save.pe)
        .bind(&save.pre_diagnosis)
        .execute(pool)
        .await
        .map(|result| ExecuteResponse::from_query_result(result, "Insert HisReferOutVitalSign"))
        .map_err(|e| Source::SQLx.to_error(500, e, "Insert HisReferOutVitalSign"))
}

async fn update_refer_vital_sign(refer_vital_sign_id: i32, save: &HisReferOutSave, pool: &Pool<MySql>, hosxp: &str) -> Result<ExecuteResponse, AppError> {
    let sql = refer_out::update_his_refer_vital_sign(hosxp);
    sqlx::query(AssertSqlSafe(sql))
        .bind(&save.cc)
        .bind(&save.pe)
        .bind(&save.pre_diagnosis)
        .bind(refer_vital_sign_id)
        .execute(pool)
        .await
        .map(|result| ExecuteResponse::from_query_result(result, "Update HisReferOutVitalSign"))
        .map_err(|e| Source::SQLx.to_error(500, e, "Update HisReferOutVitalSign"))
}

#[cfg(test)]
#[rustfmt::skip]
mod tests {

    use super::*;
    use kphis_sqlx_tester::MySqlTester;

    #[tokio::test]
    #[ignore]
    async fn sqlx_select_his_referout_data() {
        let tester = MySqlTester::new_hosxp().await;
        sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/create/hosxp/referout.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/create/hosxp/refer_vital_sign.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/create/hosxp/hospcode.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/create/hosxp/refer_cause.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/create/hosxp/moph_refer_expire_type.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/create/hosxp/doctor.sql")).execute(&tester.db_pool).await.unwrap();

        sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/insert/hosxp/referout.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/insert/hosxp/refer_vital_sign.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/insert/hosxp/hospcode.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/insert/hosxp/refer_cause.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/insert/hosxp/moph_refer_expire_type.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/insert/hosxp/doctor.sql")).execute(&tester.db_pool).await.unwrap();

        // let timer = std::time::Instant::now();
        let found = select_his_referout_data("660001234", &tester.db_pool, &tester.hosxp).await.unwrap();
        assert_eq!(found.len(), 1);
        let not_found = select_his_referout_data("990001234", &tester.db_pool, &tester.hosxp).await.unwrap();
        assert!(not_found.is_empty());
        // let duration = timer.elapsed();
        // println!("Time elapsed in expensive_function() is: {:?}", duration);
    }

    #[tokio::test]
    #[ignore]
    async fn sqlx_get_insert_referout() {
        let tester = MySqlTester::new_hosxp().await;
        sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/create/hosxp/referout.sql")).execute(&tester.db_pool).await.unwrap();

        let success = insert_referout(1, &HisReferOutSave::demo(), &Some(String::from("009")), &tester.db_pool, &tester.hosxp).await.unwrap();
        assert_eq!(success.rows_affected, 1);
        let again_duplicate = insert_referout(1, &HisReferOutSave::demo(), &Some(String::from("009")), &tester.db_pool, &tester.hosxp).await;
        assert!(again_duplicate.is_err());
    }

    #[tokio::test]
    #[ignore]
    async fn sqlx_get_update_referout() {
        let tester = MySqlTester::new_hosxp().await;
        sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/create/hosxp/referout.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/insert/hosxp/referout.sql")).execute(&tester.db_pool).await.unwrap();

        let success = update_referout(1, &HisReferOutSave::demo(), &Some(String::from("009")), &tester.db_pool, &tester.hosxp).await.unwrap();
        assert_eq!(success.rows_affected, 1);
        let again_success = update_referout(1, &HisReferOutSave::demo(), &Some(String::from("009")), &tester.db_pool, &tester.hosxp).await.unwrap();
        assert_eq!(again_success.rows_affected, 1);
    }

    #[tokio::test]
    #[ignore]
    async fn sqlx_get_insert_refer_vital_sign() {
        let tester = MySqlTester::new_hosxp().await;
        sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/create/hosxp/refer_vital_sign.sql")).execute(&tester.db_pool).await.unwrap();

        let success = insert_refer_vital_sign(1, 1, &HisReferOutSave::demo(), &tester.db_pool, &tester.hosxp).await.unwrap();
        assert_eq!(success.rows_affected, 1);
        let again_duplicate = insert_refer_vital_sign(1, 1, &HisReferOutSave::demo(), &tester.db_pool, &tester.hosxp).await;
        assert!(again_duplicate.is_err());
    }

    #[tokio::test]
    #[ignore]
    async fn sqlx_get_update_refer_vital_sign() {
        let tester = MySqlTester::new_hosxp().await;
        sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/create/hosxp/refer_vital_sign.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/insert/hosxp/refer_vital_sign.sql")).execute(&tester.db_pool).await.unwrap();

        let success = update_refer_vital_sign(1, &HisReferOutSave::demo(), &tester.db_pool, &tester.hosxp).await.unwrap();
        assert_eq!(success.rows_affected, 1);
        let again_success = update_refer_vital_sign(1, &HisReferOutSave::demo(), &tester.db_pool, &tester.hosxp).await.unwrap();
        assert_eq!(again_success.rows_affected, 1);
    }
}
