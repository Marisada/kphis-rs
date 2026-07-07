use sqlx::{AssertSqlSafe, FromRow, MySql, Pool};

use kphis_model::{
    fetch::ExecuteResponse,
    refer_note::{ReferNote, ReferNoteSave},
};
use kphis_sql::refer_note;
use kphis_util::{
    error::{AppError, Source},
    util::opt_zero_none,
};

pub async fn select_refernote(vnan: &str, pool: &Pool<MySql>, hosxp: &str, kphis_extra: &str) -> Result<Vec<ReferNote>, AppError> {
    let refernote_sql = refer_note::select_refer_note(hosxp, kphis_extra);
    sqlx::query(AssertSqlSafe(refernote_sql))
        .bind(vnan)
        .fetch_all(pool)
        .await
        .map_err(|e| Source::SQLx.to_error(500, e, "Select ReferNote"))?
        .iter()
        .map(ReferNote::from_row)
        .collect::<sqlx::Result<Vec<ReferNote>>>()
        .map_err(|e| Source::SQLx.to_error(500, e, "Select ReferNote"))
}

pub async fn post_refernote(save: &ReferNoteSave, doctorcode: &Option<String>, user: &str, pool: &Pool<MySql>, kphis_extra: &str) -> Result<ExecuteResponse, AppError> {
    // UPDATE
    let result = if let Some(refernote_id) = opt_zero_none(save.refernote_id) {
        update_refernote(refernote_id, save, doctorcode, user, pool, kphis_extra).await?
    // INSERT
    } else {
        insert_refernote(save, doctorcode, user, pool, kphis_extra).await?
    };

    Ok(result)
}

async fn insert_refernote(save: &ReferNoteSave, doctorcode: &Option<String>, user: &str, pool: &Pool<MySql>, kphis_extra: &str) -> Result<ExecuteResponse, AppError> {
    let sql = refer_note::insert_refer_note(kphis_extra);
    sqlx::query(AssertSqlSafe(sql))
        .bind(&save.vn)
        .bind(&save.hn)
        .bind(&save.refer_hospcode)
        .bind(save.refer_date)
        .bind(save.refer_time)
        .bind(&save.docno)
        .bind(&save.pmh)
        .bind(&save.hpi)
        .bind(&save.lab_text)
        .bind(&save.treatment_text)
        .bind(&save.other_text)
        .bind(&save.diagnosis_text)
        .bind(&save.request_text)
        .bind(doctorcode)
        .bind(&save.cc)
        .bind(&save.pe)
        .bind(user)
        .bind(user)
        .execute(pool)
        .await
        .map(|result| ExecuteResponse::from_query_result(result, "Insert ReferNoteSave"))
        .map_err(|e| Source::SQLx.to_error(500, e, "Insert ReferNoteSave"))
}

async fn update_refernote(refernote_id: u32, save: &ReferNoteSave, doctorcode: &Option<String>, user: &str, pool: &Pool<MySql>, kphis_extra: &str) -> Result<ExecuteResponse, AppError> {
    let sql = refer_note::update_refer_note(kphis_extra);
    sqlx::query(AssertSqlSafe(sql))
        .bind(&save.vn)
        .bind(&save.hn)
        .bind(&save.refer_hospcode)
        .bind(save.refer_date)
        .bind(save.refer_time)
        .bind(&save.docno)
        .bind(&save.pmh)
        .bind(&save.hpi)
        .bind(&save.lab_text)
        .bind(&save.treatment_text)
        .bind(&save.other_text)
        .bind(&save.diagnosis_text)
        .bind(&save.request_text)
        .bind(doctorcode)
        .bind(&save.cc)
        .bind(&save.pe)
        .bind(user)
        .bind(refernote_id)
        .execute(pool)
        .await
        .map(|result| ExecuteResponse::from_query_result(result, "Update ReferNoteSave"))
        .map_err(|e| Source::SQLx.to_error(500, e, "Update ReferNoteSave"))
}

#[cfg(test)]
#[rustfmt::skip]
mod tests {

    use super::*;
    use kphis_sqlx_tester::MySqlTester;

    #[tokio::test]
    #[ignore]
    async fn sqlx_select_refernote() {
        let tester = MySqlTester::new_hosxp_and_kphis_extra().await;
        sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/create/hosxp/hospcode.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/create/hosxp/doctor.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/create/kphis_extra/refer_note.sql")).execute(&tester.db_pool).await.unwrap();

        sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/insert/hosxp/hospcode.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/insert/hosxp/doctor.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/insert/kphis_extra/refer_note.sql")).execute(&tester.db_pool).await.unwrap();

        let found = select_refernote("660001234", &tester.db_pool, &tester.hosxp, &tester.kphis_extra).await.unwrap();
        assert_eq!(found.len(), 1);
        let not_found = select_refernote("990001234", &tester.db_pool, &tester.hosxp, &tester.kphis_extra).await.unwrap();
        assert!(not_found.is_empty());
    }

    #[tokio::test]
    #[ignore]
    async fn sqlx_get_insert_refernote() {
        let tester = MySqlTester::new_kphis_extra().await;
        sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/create/kphis_extra/refer_note.sql")).execute(&tester.db_pool).await.unwrap();

        let success = insert_refernote(&ReferNoteSave::demo(), &Some(String::from("009")), "user", &tester.db_pool, &tester.kphis_extra).await.unwrap();
        assert_eq!(success.rows_affected, 1);
        let again_success = insert_refernote(&ReferNoteSave::demo(), &Some(String::from("009")), "user", &tester.db_pool, &tester.kphis_extra).await.unwrap();
        assert_eq!(again_success.rows_affected, 1);
    }

    #[tokio::test]
    #[ignore]
    async fn sqlx_get_update_refernote() {
        let tester = MySqlTester::new_kphis_extra().await;
        sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/create/kphis_extra/refer_note.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/insert/kphis_extra/refer_note.sql")).execute(&tester.db_pool).await.unwrap();

        let success = update_refernote(1, &ReferNoteSave::demo(), &Some(String::from("009")), "user", &tester.db_pool, &tester.kphis_extra).await.unwrap();
        assert_eq!(success.rows_affected, 1);
        let again_success = update_refernote(1, &ReferNoteSave::demo(), &Some(String::from("009")), "user", &tester.db_pool, &tester.kphis_extra).await.unwrap();
        assert_eq!(again_success.rows_affected, 1);
    }
}
