use sqlx::{AssertSqlSafe, MySql, Pool, mysql::MySqlQueryResult};

use kphis_sql::{data_history_utils, log};
use kphis_util::error::{AppError, Source};

use super::execute3;

pub async fn insert_history_log(
    source_table: data_history_utils::SourceTable,
    history_type: &str,
    user: &str,
    kvs: &[data_history_utils::KeyValue],
    kphis: &str,
    kphis_log: &str,
    pool: &Pool<MySql>,
) -> Result<MySqlQueryResult, AppError> {
    sqlx::query(AssertSqlSafe(data_history_utils::insert_history_log(&source_table, history_type, user, kvs, kphis, kphis_log)))
        .execute(pool)
        .await
        .map_err(|e| Source::SQLx.to_error(500, e, &["Insert ", source_table.table_name(), " History"].concat()))
}

pub async fn insert_access_log(user: &str, address: &str, access_detail: &str, pool: &Pool<MySql>, kphis_log: &str) -> Result<MySqlQueryResult, AppError> {
    let sql = log::insert_system_access_log(kphis_log);
    execute3(user, address, access_detail, &sql, pool, "Insert AccessLog").await
}

pub async fn delete_expired_access_log(days: i64, pool: &Pool<MySql>, kphis_log: &str) -> Result<MySqlQueryResult, AppError> {
    let access_sql = log::delete_expired_access_log(kphis_log);
    sqlx::query(AssertSqlSafe(access_sql))
        .bind(days)
        .execute(pool)
        .await
        .map_err(|e| Source::SQLx.to_error(500, e, "Delete AccessLog"))
}

pub async fn delete_expired_history_log(days: i64, pool: &Pool<MySql>, kphis_log: &str) -> Result<MySqlQueryResult, AppError> {
    let history_sql = log::delete_expired_history_log(kphis_log);
    sqlx::query(AssertSqlSafe(history_sql))
        .bind(days)
        .execute(pool)
        .await
        .map_err(|e| Source::SQLx.to_error(500, e, "Delete HistoryLog"))
}

pub async fn delete_expired_message(days: i64, pool: &Pool<MySql>, kphis_log: &str) -> Result<MySqlQueryResult, AppError> {
    let message_sql = log::delete_expired_message(kphis_log);
    sqlx::query(AssertSqlSafe(message_sql))
        .bind(days)
        .execute(pool)
        .await
        .map_err(|e| Source::SQLx.to_error(500, e, "Delete Message"))
}

#[cfg(test)]
#[rustfmt::skip]
mod tests {

    use super::*;
    use kphis_sqlx_tester::MySqlTester;
    use kphis_sql::data_history_utils::{SourceTable, KeyValue};

    #[tokio::test]
    #[ignore]
    async fn sqlx_insert_access_log() {
        let tester = MySqlTester::new_kphis_log().await;
        sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/create/kphis_log/system_access_log.sql")).execute(&tester.db_pool).await.unwrap();

        let success = insert_access_log("user", "127.0.0.1:11111", "{}", &tester.db_pool, &tester.kphis_log).await.unwrap();
        assert_eq!(success.rows_affected(), 1);
        let again = insert_access_log("user", "127.0.0.1:11111", "{}", &tester.db_pool, &tester.kphis_log).await.unwrap();
        assert_eq!(again.rows_affected(), 1);
    }

    #[tokio::test]
    #[ignore]
    async fn sqlx_delete_expired_access_log() {
        let tester = MySqlTester::new_kphis_log().await;
        sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/create/kphis_log/system_access_log.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/insert/kphis_log/system_access_log.sql")).execute(&tester.db_pool).await.unwrap();

        let success = delete_expired_access_log(1, &tester.db_pool, &tester.kphis_log).await.unwrap();
        assert_eq!(success.rows_affected(), 1);
        let again = delete_expired_access_log(1, &tester.db_pool, &tester.kphis_log).await.unwrap();
        assert_eq!(again.rows_affected(), 0);
    }

    #[tokio::test]
    #[ignore]
    async fn sqlx_delete_expired_history_log() {
        let tester = MySqlTester::new_kphis_log().await;
        sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/create/kphis_log/history_log.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/insert/kphis_log/history_log.sql")).execute(&tester.db_pool).await.unwrap();

        let success = delete_expired_history_log(1, &tester.db_pool, &tester.kphis_log).await.unwrap();
        assert_eq!(success.rows_affected(), 1);
        let again = delete_expired_history_log(1, &tester.db_pool, &tester.kphis_log).await.unwrap();
        assert_eq!(again.rows_affected(), 0);
    }

    #[tokio::test]
    #[ignore]
    async fn sqlx_delete_expired_message() {
        let tester = MySqlTester::new_kphis_log().await;
        sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/create/kphis_log/message.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/insert/kphis_log/message.sql")).execute(&tester.db_pool).await.unwrap();

        let success = delete_expired_message(1, &tester.db_pool, &tester.kphis_log).await.unwrap();
        assert_eq!(success.rows_affected(), 6);
        let again = delete_expired_message(1, &tester.db_pool, &tester.kphis_log).await.unwrap();
        assert_eq!(again.rows_affected(), 0);
    }

    #[tokio::test]
    #[ignore]
    async fn sqlx_insert_history_log_ipd_dr_consult() {
        let tester = MySqlTester::new_kphis_and_kphis_log().await;
        sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/create/kphis_log/history_log.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/create/kphis/ipd_dr_consult.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/insert/kphis/ipd_dr_consult.sql")).execute(&tester.db_pool).await.unwrap();

        let success = insert_history_log(SourceTable::IpdDrConsult,"I","user",&[KeyValue("consult_id",String::from("1"))],&tester.kphis, &tester.kphis_log, &tester.db_pool).await.unwrap();
        assert_eq!(success.rows_affected(), 1);
    }

    #[tokio::test]
    #[ignore]
    async fn sqlx_insert_history_log_ipd_dr_consult_signature_request() {
        let tester = MySqlTester::new_kphis_and_kphis_log().await;
        sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/create/kphis_log/history_log.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/create/kphis/ipd_dr_consult_signature_request.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/insert/kphis/ipd_dr_consult_signature_request.sql")).execute(&tester.db_pool).await.unwrap();

        let success = insert_history_log(SourceTable::IpdDrConsultSignatureRequest,"I","user",&[KeyValue("consult_signature_id",String::from("1"))],&tester.kphis, &tester.kphis_log, &tester.db_pool).await.unwrap();
        assert_eq!(success.rows_affected(), 1);
    }

    #[tokio::test]
    #[ignore]
    async fn sqlx_insert_history_log_ipd_dr_consult_signature_reply() {
        let tester = MySqlTester::new_kphis_and_kphis_log().await;
        sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/create/kphis_log/history_log.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/create/kphis/ipd_dr_consult_signature_reply.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/insert/kphis/ipd_dr_consult_signature_reply.sql")).execute(&tester.db_pool).await.unwrap();

        let success = insert_history_log(SourceTable::IpdDrConsultSignatureReply,"I","user",&[KeyValue("consult_reply_id",String::from("1"))],&tester.kphis, &tester.kphis_log, &tester.db_pool).await.unwrap();
        assert_eq!(success.rows_affected(), 1);
    }

    #[tokio::test]
    #[ignore]
    async fn sqlx_insert_history_log_ipd_doctor_in_charge() {
        let tester = MySqlTester::new_kphis_and_kphis_log().await;
        sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/create/kphis_log/history_log.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/create/kphis/ipd_doctor_in_charge.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/insert/kphis/ipd_doctor_in_charge.sql")).execute(&tester.db_pool).await.unwrap();

        let success = insert_history_log(SourceTable::IpdDoctorInCharge,"I","user",&[KeyValue("doctor_in_charge_id",String::from("1"))],&tester.kphis, &tester.kphis_log, &tester.db_pool).await.unwrap();
        assert_eq!(success.rows_affected(), 1);
    }

    #[tokio::test]
    #[ignore]
    async fn sqlx_insert_history_log_ipd_focus_list() {
        let tester = MySqlTester::new_kphis_and_kphis_log().await;
        sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/create/kphis_log/history_log.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/create/kphis/ipd_focus_list.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/insert/kphis/ipd_focus_list.sql")).execute(&tester.db_pool).await.unwrap();

        let success = insert_history_log(SourceTable::IpdFocusList,"I","user",&[KeyValue("fclist_id",String::from("1"))],&tester.kphis, &tester.kphis_log, &tester.db_pool).await.unwrap();
        assert_eq!(success.rows_affected(), 1);
    }

    #[tokio::test]
    #[ignore]
    async fn sqlx_insert_history_log_ipd_focus_list_goal_item() {
        let tester = MySqlTester::new_kphis_and_kphis_log().await;
        sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/create/kphis_log/history_log.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/create/kphis/ipd_focus_list_goal_item.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/insert/kphis/ipd_focus_list_goal_item.sql")).execute(&tester.db_pool).await.unwrap();

        let success = insert_history_log(SourceTable::IpdFocusListGoalItem,"I","user",&[KeyValue("fclist_item_id",String::from("1"))],&tester.kphis, &tester.kphis_log, &tester.db_pool).await.unwrap();
        assert_eq!(success.rows_affected(), 1);
    }

    #[tokio::test]
    #[ignore]
    async fn sqlx_insert_history_log_ipd_focus_note() {
        let tester = MySqlTester::new_kphis_and_kphis_log().await;
        sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/create/kphis_log/history_log.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/create/kphis/ipd_focus_note.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/insert/kphis/ipd_focus_note.sql")).execute(&tester.db_pool).await.unwrap();

        let success = insert_history_log(SourceTable::IpdFocusNote,"I","user",&[KeyValue("fcnote_id",String::from("1"))],&tester.kphis, &tester.kphis_log, &tester.db_pool).await.unwrap();
        assert_eq!(success.rows_affected(), 1);
    }

    #[tokio::test]
    #[ignore]
    async fn sqlx_insert_history_log_ipd_focus_note_intvt_item() {
        let tester = MySqlTester::new_kphis_and_kphis_log().await;
        sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/create/kphis_log/history_log.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/create/kphis/ipd_focus_note_intvt_item.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/insert/kphis/ipd_focus_note_intvt_item.sql")).execute(&tester.db_pool).await.unwrap();

        let success = insert_history_log(SourceTable::IpdFocusNoteIntvtItem,"I","user",&[KeyValue("fcnote_intvt_item_id",String::from("1"))],&tester.kphis, &tester.kphis_log, &tester.db_pool).await.unwrap();
        assert_eq!(success.rows_affected(), 1);
    }

    #[tokio::test]
    #[ignore]
    async fn sqlx_insert_history_log_ipd_focus_note_dlc_item() {
        let tester = MySqlTester::new_kphis_and_kphis_log().await;
        sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/create/kphis_log/history_log.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/create/kphis/ipd_focus_note_dlc_item.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/insert/kphis/ipd_focus_note_dlc_item.sql")).execute(&tester.db_pool).await.unwrap();

        let success = insert_history_log(SourceTable::IpdFocusNoteDlcItem,"I","user",&[KeyValue("fcnote_dlc_item_id",String::from("1"))],&tester.kphis, &tester.kphis_log, &tester.db_pool).await.unwrap();
        assert_eq!(success.rows_affected(), 1);
    }

    #[tokio::test]
    #[ignore]
    async fn sqlx_insert_history_log_ipd_io() {
        let tester = MySqlTester::new_kphis_and_kphis_log().await;
        sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/create/kphis_log/history_log.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/create/kphis/ipd_io.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/insert/kphis/ipd_io.sql")).execute(&tester.db_pool).await.unwrap();

        let success = insert_history_log(SourceTable::IpdIo,"I","user",&[KeyValue("io_id",String::from("1"))],&tester.kphis, &tester.kphis_log, &tester.db_pool).await.unwrap();
        assert_eq!(success.rows_affected(), 1);
    }

    #[tokio::test]
    #[ignore]
    async fn sqlx_insert_history_log_log_opd_er_allergy_history() {
        let tester = MySqlTester::new_kphis_and_kphis_log().await;
        sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/create/kphis_log/history_log.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/create/kphis/opd_er_allergy_history.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/insert/kphis/opd_er_allergy_history.sql")).execute(&tester.db_pool).await.unwrap();

        let success = insert_history_log(SourceTable::OpdErAllergyHistory,"I","user",&[KeyValue("er_allergy_history_id",String::from("1"))],&tester.kphis, &tester.kphis_log, &tester.db_pool).await.unwrap();
        assert_eq!(success.rows_affected(), 1);
    }

    #[tokio::test]
    #[ignore]
    async fn sqlx_insert_history_log_log_opd_er_consult() {
        let tester = MySqlTester::new_kphis_and_kphis_log().await;
        sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/create/kphis_log/history_log.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/create/kphis/opd_er_consult.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/insert/kphis/opd_er_consult.sql")).execute(&tester.db_pool).await.unwrap();

        let success = insert_history_log(SourceTable::OpdErConsult,"I","user",&[KeyValue("er_consult_id",String::from("1"))],&tester.kphis, &tester.kphis_log, &tester.db_pool).await.unwrap();
        assert_eq!(success.rows_affected(), 1);
    }

    #[tokio::test]
    #[ignore]
    async fn sqlx_insert_history_log_log_opd_er_document_scan() {
        let tester = MySqlTester::new_kphis_and_kphis_log().await;
        sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/create/kphis_log/history_log.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/create/kphis/opd_er_document_scan.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/insert/kphis/opd_er_document_scan.sql")).execute(&tester.db_pool).await.unwrap();

        let success = insert_history_log(SourceTable::OpdErDocumentScan,"I","user",&[KeyValue("opd_er_document_scan_id",String::from("1"))],&tester.kphis, &tester.kphis_log, &tester.db_pool).await.unwrap();
        assert_eq!(success.rows_affected(), 1);
    }

    #[tokio::test]
    #[ignore]
    async fn sqlx_insert_history_log_log_opd_er_dr_pe() {
        let tester = MySqlTester::new_kphis_and_kphis_log().await;
        sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/create/kphis_log/history_log.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/create/kphis/opd_er_dr_pe.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/insert/kphis/opd_er_dr_pe.sql")).execute(&tester.db_pool).await.unwrap();

        let success = insert_history_log(SourceTable::OpdErDrPe,"I","user",&[KeyValue("opd_er_pe_id",String::from("1"))],&tester.kphis, &tester.kphis_log, &tester.db_pool).await.unwrap();
        assert_eq!(success.rows_affected(), 1);
    }

    #[tokio::test]
    #[ignore]
    async fn sqlx_insert_history_log_log_opd_er_nurse_screening() {
        let tester = MySqlTester::new_kphis_and_kphis_log().await;
        sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/create/kphis_log/history_log.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/create/kphis/opd_er_nurse_screening.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/insert/kphis/opd_er_nurse_screening.sql")).execute(&tester.db_pool).await.unwrap();

        let success = insert_history_log(SourceTable::OpdErNurseScreening,"I","user",&[KeyValue("opd_er_screening_id",String::from("1"))],&tester.kphis, &tester.kphis_log, &tester.db_pool).await.unwrap();
        assert_eq!(success.rows_affected(), 1);
    }

    #[tokio::test]
    #[ignore]
    async fn sqlx_insert_history_log_log_opd_er_focus_list() {
        let tester = MySqlTester::new_kphis_and_kphis_log().await;
        sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/create/kphis_log/history_log.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/create/kphis/opd_er_focus_list.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/insert/kphis/opd_er_focus_list.sql")).execute(&tester.db_pool).await.unwrap();

        let success = insert_history_log(SourceTable::OpdErFocusList,"I","user",&[KeyValue("fclist_id",String::from("1"))],&tester.kphis, &tester.kphis_log, &tester.db_pool).await.unwrap();
        assert_eq!(success.rows_affected(), 1);
    }

    #[tokio::test]
    #[ignore]
    async fn sqlx_insert_history_log_log_opd_er_focus_list_goal_item() {
        let tester = MySqlTester::new_kphis_and_kphis_log().await;
        sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/create/kphis_log/history_log.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/create/kphis/opd_er_focus_list_goal_item.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/insert/kphis/opd_er_focus_list_goal_item.sql")).execute(&tester.db_pool).await.unwrap();

        let success = insert_history_log(SourceTable::OpdErFocusListGoalItem,"I","user",&[KeyValue("fclist_item_id",String::from("1"))],&tester.kphis, &tester.kphis_log, &tester.db_pool).await.unwrap();
        assert_eq!(success.rows_affected(), 1);
    }

    #[tokio::test]
    #[ignore]
    async fn sqlx_insert_history_log_log_opd_er_focus_note() {
        let tester = MySqlTester::new_kphis_and_kphis_log().await;
        sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/create/kphis_log/history_log.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/create/kphis/opd_er_focus_note.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/insert/kphis/opd_er_focus_note.sql")).execute(&tester.db_pool).await.unwrap();

        let success = insert_history_log(SourceTable::OpdErFocusNote,"I","user",&[KeyValue("fcnote_id",String::from("1"))],&tester.kphis, &tester.kphis_log, &tester.db_pool).await.unwrap();
        assert_eq!(success.rows_affected(), 1);
    }

    #[tokio::test]
    #[ignore]
    async fn sqlx_insert_history_log_log_opd_er_focus_note_intvt_item() {
        let tester = MySqlTester::new_kphis_and_kphis_log().await;
        sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/create/kphis_log/history_log.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/create/kphis/opd_er_focus_note_intvt_item.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/insert/kphis/opd_er_focus_note_intvt_item.sql")).execute(&tester.db_pool).await.unwrap();

        let success = insert_history_log(SourceTable::OpdErFocusNoteIntvtItem,"I","user",&[KeyValue("fcnote_intvt_item_id",String::from("1"))],&tester.kphis, &tester.kphis_log, &tester.db_pool).await.unwrap();
        assert_eq!(success.rows_affected(), 1);
    }

    #[tokio::test]
    #[ignore]
    async fn sqlx_insert_history_log_log_opd_er_focus_note_dlc_item() {
        let tester = MySqlTester::new_kphis_and_kphis_log().await;
        sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/create/kphis_log/history_log.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/create/kphis/opd_er_focus_note_dlc_item.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/insert/kphis/opd_er_focus_note_dlc_item.sql")).execute(&tester.db_pool).await.unwrap();

        let success = insert_history_log(SourceTable::OpdErFocusNoteDlcItem,"I","user",&[KeyValue("fcnote_dlc_item_id",String::from("1"))],&tester.kphis, &tester.kphis_log, &tester.db_pool).await.unwrap();
        assert_eq!(success.rows_affected(), 1);
    }

    #[tokio::test]
    #[ignore]
    async fn sqlx_insert_history_log_log_opd_er_set_fast_track() {
        let tester = MySqlTester::new_kphis_and_kphis_log().await;
        sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/create/kphis_log/history_log.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/create/kphis/opd_er_set_fast_track.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/insert/kphis/opd_er_set_fast_track.sql")).execute(&tester.db_pool).await.unwrap();

        let success = insert_history_log(SourceTable::OpdErSetFastTrack,"I","user",&[KeyValue("set_ft_id",String::from("1"))],&tester.kphis, &tester.kphis_log, &tester.db_pool).await.unwrap();
        assert_eq!(success.rows_affected(), 1);
    }

    #[tokio::test]
    #[ignore]
    async fn sqlx_insert_history_log_log_opd_er_io() {
        let tester = MySqlTester::new_kphis_and_kphis_log().await;
        sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/create/kphis_log/history_log.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/create/kphis/opd_er_io.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/insert/kphis/opd_er_io.sql")).execute(&tester.db_pool).await.unwrap();

        let success = insert_history_log(SourceTable::OpdErIo,"I","user",&[KeyValue("opd_er_io_id",String::from("1"))],&tester.kphis, &tester.kphis_log, &tester.db_pool).await.unwrap();
        assert_eq!(success.rows_affected(), 1);
    }
}
