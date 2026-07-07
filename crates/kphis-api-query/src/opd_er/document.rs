use sqlx::{AssertSqlSafe, FromRow, MySql, Pool};

use kphis_model::{fetch::ExecuteResponse, ipd::document::DocumentScan, opd_er::document::OpdErDocumentExists};
use kphis_sql::opd_er::document;
use kphis_util::error::{AppError, Source};

use crate::ipd::document::document_scan_from_row;

pub async fn get_opd_er_document_list(vn: &str, opd_er_order_master_id: u32, pool: &Pool<MySql>, hosxp: &str, kphis: &str, kphis_extra: &str) -> Result<OpdErDocumentExists, AppError> {
    let sql = document::get_opd_er_document_exists(hosxp, kphis, kphis_extra);
    sqlx::query(AssertSqlSafe(sql))
        .bind(opd_er_order_master_id)
        .bind(opd_er_order_master_id)
        .bind(opd_er_order_master_id)
        .bind(opd_er_order_master_id)
        .bind(opd_er_order_master_id)
        .bind(opd_er_order_master_id)
        .bind(opd_er_order_master_id)
        .bind(opd_er_order_master_id)
        .bind(opd_er_order_master_id)
        .bind(opd_er_order_master_id)
        .bind(opd_er_order_master_id)
        .bind(opd_er_order_master_id)
        .bind(opd_er_order_master_id)
        .bind(opd_er_order_master_id)
        .bind(opd_er_order_master_id)
        .bind(opd_er_order_master_id)
        .bind(opd_er_order_master_id)
        .bind(opd_er_order_master_id)
        .bind(opd_er_order_master_id)
        .bind(opd_er_order_master_id)
        .bind(opd_er_order_master_id)
        .bind(opd_er_order_master_id)
        .bind(opd_er_order_master_id)
        .bind(opd_er_order_master_id)
        .bind(opd_er_order_master_id)
        .bind(opd_er_order_master_id)
        .bind(opd_er_order_master_id)
        .bind(opd_er_order_master_id)
        .bind(opd_er_order_master_id)
        .bind(opd_er_order_master_id)
        .bind(vn)
        .bind(vn)
        .fetch_one(pool)
        .await
        .as_ref()
        .map(OpdErDocumentExists::from_row)
        .map_err(|e| Source::SQLx.to_error(500, e, "Select OpdErDocumentExists"))?
        .map_err(|e| Source::SQLx.to_error(500, e, "Select OpdErDocumentExists"))
}

pub async fn get_opd_er_document_types(opd_er_order_master_id: u32, pool: &Pool<MySql>, kphis_extra: &str) -> Result<Vec<DocumentScan>, AppError> {
    let sql = document::get_opd_er_document_types(kphis_extra);
    sqlx::query(AssertSqlSafe(sql))
        .bind(opd_er_order_master_id)
        .fetch_all(pool)
        .await
        .map_err(|e| Source::SQLx.to_error(500, e, "Select OpdErDocumentScan"))?
        .iter()
        .map(document_scan_from_row)
        .collect::<sqlx::Result<Vec<DocumentScan>>>()
        .map_err(|e| Source::SQLx.to_error(500, e, "Select OpdErDocumentScan"))
}

pub async fn post_opd_er_document_type(opd_er_order_master_id: u32, document_type_id: u8, user: &str, pool: &Pool<MySql>, kphis_extra: &str) -> Result<ExecuteResponse, AppError> {
    let sql = document::insert_ignore_opd_er_document_type(kphis_extra);
    sqlx::query(AssertSqlSafe(sql))
        .bind(opd_er_order_master_id)
        .bind(document_type_id)
        .bind(user)
        .bind(user)
        .execute(pool)
        .await
        .map(|res| ExecuteResponse::from_query_result(res, "Insert OpdErDocumentType"))
        .map_err(|e| Source::SQLx.to_error(500, e, "Insert OpdErDocumentType"))
}

pub async fn delete_opd_er_document_type(opd_er_order_master_id: u32, document_type_id: u8, pool: &Pool<MySql>, kphis_extra: &str) -> Result<ExecuteResponse, AppError> {
    let sql = document::delete_opd_er_document_type(kphis_extra);
    sqlx::query(AssertSqlSafe(sql))
        .bind(opd_er_order_master_id)
        .bind(document_type_id)
        .execute(pool)
        .await
        .map(|res| ExecuteResponse::from_query_result(res, "Delete OpdErDocumentType"))
        .map_err(|e| Source::SQLx.to_error(500, e, "Delete OpdErDocumentType"))
}

#[cfg(test)]
#[rustfmt::skip]
mod tests {

    use super::*;
    use kphis_sqlx_tester::MySqlTester;

    #[tokio::test]
    #[ignore]
    async fn sqlx_get_opd_er_document_list() {
        let tester = MySqlTester::new_hosxp_and_kphis_and_kphis_extra().await;
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis/opd_er_order_master.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis/opd_er_nurse_index_plan.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis/opd_er_vs_vital_sign.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis/opd_er_order.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis/opd_er_order_progress_note.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis/opd_er_focus_list.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis/opd_er_focus_note.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis/opd_er_io.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis/opd_er_med_reconciliation.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/hosxp/lab_head.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/hosxp/lab_order.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/hosxp/referout.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis_extra/image_usage.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis_extra/opd_er_document.sql")).execute(&tester.db_pool).await.unwrap();

        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/kphis/opd_er_order_master.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/kphis/opd_er_nurse_index_plan.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/kphis/opd_er_vs_vital_sign.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/kphis/opd_er_order.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/kphis/opd_er_order_progress_note.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/kphis/opd_er_focus_list.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/kphis/opd_er_focus_note.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/kphis/opd_er_io.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/kphis/opd_er_med_reconciliation.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/hosxp/lab_head.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/hosxp/lab_order.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/hosxp/referout.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/kphis_extra/image_usage.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/kphis_extra/opd_er_document.sql")).execute(&tester.db_pool).await.unwrap();

        let found = get_opd_er_document_list("661231235959",1,&tester.db_pool,&tester.hosxp,&tester.kphis,&tester.kphis_extra).await;
        assert!(found.is_ok());
        let not_found = get_opd_er_document_list("991231235959",1,&tester.db_pool,&tester.hosxp,&tester.kphis,&tester.kphis_extra).await;
        assert!(not_found.is_ok());
    }

    #[tokio::test]
    #[ignore]
    async fn sqlx_get_opd_er_document_types() {
        let tester = MySqlTester::new_kphis_extra().await;
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis_extra/opd_er_document.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis_extra/image_usage.sql")).execute(&tester.db_pool).await.unwrap();

        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/kphis_extra/opd_er_document.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/kphis_extra/image_usage.sql")).execute(&tester.db_pool).await.unwrap();

        let found = get_opd_er_document_types(1, &tester.db_pool, &tester.kphis_extra).await.unwrap();
        assert_eq!(found.len(), 1);
        let not_found = get_opd_er_document_types(999, &tester.db_pool, &tester.kphis_extra).await.unwrap();
        assert!(not_found.is_empty());
    }

    #[tokio::test]
    #[ignore]
    async fn sqlx_post_opd_er_document_type() {
        let tester = MySqlTester::new_kphis_extra().await;
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis_extra/opd_er_document.sql")).execute(&tester.db_pool).await.unwrap();

        let success = post_opd_er_document_type(1, 1, "user", &tester.db_pool, &tester.kphis_extra).await.unwrap();
        assert_eq!(success.rows_affected, 1);
        let again_ignore = post_opd_er_document_type(1, 1, "user", &tester.db_pool, &tester.kphis_extra).await.unwrap();
        assert_eq!(again_ignore.rows_affected, 0);
    }

    #[tokio::test]
    #[ignore]
    async fn sqlx_delete_opd_er_document_type() {
        let tester = MySqlTester::new_kphis_extra().await;
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis_extra/opd_er_document.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis_extra/image_usage.sql")).execute(&tester.db_pool).await.unwrap();

        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/kphis_extra/opd_er_document.sql")).execute(&tester.db_pool).await.unwrap();

        let success = delete_opd_er_document_type(1, 1, &tester.db_pool, &tester.kphis_extra).await.unwrap();
        assert_eq!(success.rows_affected, 1);
        let again_not_found = delete_opd_er_document_type(1, 1, &tester.db_pool, &tester.kphis_extra).await.unwrap();
        assert_eq!(again_not_found.rows_affected, 0);

        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/kphis_extra/opd_er_document.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/kphis_extra/image_usage.sql")).execute(&tester.db_pool).await.unwrap();

        let not_empty = delete_opd_er_document_type(1, 1, &tester.db_pool, &tester.kphis_extra).await.unwrap();
        assert_eq!(not_empty.rows_affected, 0);
    }
}
