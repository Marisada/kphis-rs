use sqlx::{AssertSqlSafe, FromRow, MySql, Pool, Row, mysql::MySqlRow};

use kphis_model::{
    fetch::ExecuteResponse,
    image::file_path::DocumentType,
    ipd::document::{DocumentScan, IpdDocumentDatetime, IpdDocumentExists},
};
use kphis_sql::ipd::document;
use kphis_util::error::{AppError, Source};

pub async fn get_ipd_document_list(vn: &str, an: &str, pool: &Pool<MySql>, hosxp: &str, kphis: &str, kphis_extra: &str) -> Result<IpdDocumentExists, AppError> {
    let sql = document::get_ipd_document_exists(hosxp, kphis, kphis_extra);
    sqlx::query(AssertSqlSafe(sql))
        .bind(an)
        .bind(an)
        .bind(an)
        .bind(an)
        .bind(an)
        .bind(an)
        .bind(an)
        .bind(an)
        .bind(an)
        .bind(an)
        .bind(an)
        .bind(an)
        .bind(an)
        .bind(an)
        .bind(an)
        .bind(an)
        .bind(an)
        .bind(an)
        .bind(an)
        .bind(an)
        .bind(an)
        .bind(an)
        .bind(an)
        .bind(an)
        .bind(an)
        .bind(an)
        .bind(an)
        .bind(an)
        .bind(an)
        .bind(an)
        .bind(an)
        .bind(an)
        .bind(an)
        .bind(an)
        .bind(an)
        .bind(an)
        .bind(an)
        .bind(an)
        .bind(an)
        .bind(an)
        .bind(vn)
        .fetch_one(pool)
        .await
        .as_ref()
        .map(IpdDocumentExists::from_row)
        .map_err(|e| Source::SQLx.to_error(500, e, "Select IpdDocumentList"))?
        .map_err(|e| Source::SQLx.to_error(500, e, "Select IpdDocumentList"))
}

pub async fn get_ipd_document_datetime(an: &str, pool: &Pool<MySql>, kphis: &str) -> Result<IpdDocumentDatetime, AppError> {
    let sql = document::get_ipd_document_datetime(kphis);
    let row = sqlx::query(AssertSqlSafe(sql))
        .bind(an)
        .bind(an)
        .bind(an)
        .fetch_one(pool)
        .await
        .map_err(|e| Source::SQLx.to_error(500, e, "Select IpdDocumentDatetime"))?;

    IpdDocumentDatetime::from_row(&row).map_err(|e| Source::SQLx.to_error(500, e, "Select IpdDocumentDatetime"))
}

pub async fn get_ipd_document_types(an: &str, pool: &Pool<MySql>, kphis_extra: &str) -> Result<Vec<DocumentScan>, AppError> {
    let sql = document::get_ipd_document_types(kphis_extra);
    sqlx::query(AssertSqlSafe(sql))
        .bind(an)
        .fetch_all(pool)
        .await
        .map_err(|e| Source::SQLx.to_error(500, e, "Select IpdDocumentScan"))?
        .iter()
        .map(document_scan_from_row)
        .collect::<sqlx::Result<Vec<DocumentScan>>>()
        .map_err(|e| Source::SQLx.to_error(500, e, "Select IpdDocumentScan"))
}
pub fn document_scan_from_row(row: &MySqlRow) -> sqlx::Result<DocumentScan> {
    Ok(DocumentScan {
        document_id: row.try_get::<u32, &str>("document_id")?,
        document_type_id: row.try_get::<Option<u8>, &str>("document_type_id")?.map(DocumentType::new_from_u8).unwrap_or(DocumentType::Others),
        has_image: row.try_get::<bool, &str>("has_image")?,
    })
}

pub async fn post_ipd_document_type(an: &str, document_type_id: u8, user: &str, pool: &Pool<MySql>, kphis_extra: &str) -> Result<ExecuteResponse, AppError> {
    let sql = document::insert_ignore_ipd_document_type(kphis_extra);
    sqlx::query(AssertSqlSafe(sql))
        .bind(an)
        .bind(document_type_id)
        .bind(user)
        .bind(user)
        .execute(pool)
        .await
        .map(|res| ExecuteResponse::from_query_result(res, "Insert IpdDocumentType"))
        .map_err(|e| Source::SQLx.to_error(500, e, "Insert IpdDocumentType"))
}

pub async fn delete_ipd_document_type(an: &str, document_type_id: u8, pool: &Pool<MySql>, kphis_extra: &str) -> Result<ExecuteResponse, AppError> {
    let sql = document::delete_ipd_document_type(kphis_extra);
    sqlx::query(AssertSqlSafe(sql))
        .bind(an)
        .bind(document_type_id)
        .execute(pool)
        .await
        .map(|res| ExecuteResponse::from_query_result(res, "Delete IpdDocumentType"))
        .map_err(|e| Source::SQLx.to_error(500, e, "Delete IpdDocumentType"))
}

#[cfg(test)]
#[rustfmt::skip]
mod tests {

    use super::*;
    use kphis_sqlx_tester::MySqlTester;

    #[tokio::test]
    #[ignore]
    async fn sqlx_get_ipd_document_list() {
        let tester = MySqlTester::new_hosxp_and_kphis_and_kphis_extra().await;
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis/ipd_summary_2.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis/ipd_dr_admission_note.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis/ipd_nurse_admission_note.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis/ipd_order.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis/ipd_progress_note.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis/ipd_dr_consult.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis/ipd_focus_list.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis/ipd_focus_note.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis/ipd_vs_vital_sign.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis/ipd_io.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis/ipd_nurse_index_plan.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis/ipd_med_reconciliation.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis/opd_er_order_master.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/hosxp/lab_head.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/hosxp/lab_order.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/hosxp/xray_report.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/hosxp/xray_items.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/hosxp/operation_list.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/hosxp/medication_reconciliation.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/hosxp/medication_reconciliation_detail.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/hosxp/referout.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis_extra/image_usage.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis_extra/ipd_document.sql")).execute(&tester.db_pool).await.unwrap();

        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/kphis/ipd_summary_2.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/kphis/ipd_dr_admission_note.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/kphis/ipd_nurse_admission_note.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/kphis/ipd_order.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/kphis/ipd_progress_note.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/kphis/ipd_dr_consult.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/kphis/ipd_focus_list.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/kphis/ipd_focus_note.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/kphis/ipd_vs_vital_sign.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/kphis/ipd_io.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/kphis/ipd_nurse_index_plan.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/kphis/ipd_med_reconciliation.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/kphis/opd_er_order_master.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/hosxp/lab_head.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/hosxp/lab_order.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/hosxp/xray_report.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/hosxp/xray_items.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/hosxp/operation_list.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/hosxp/medication_reconciliation.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/hosxp/medication_reconciliation_detail.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/hosxp/referout.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/kphis_extra/image_usage.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/kphis_extra/ipd_document.sql")).execute(&tester.db_pool).await.unwrap();

        let found = get_ipd_document_list("661231235959","660001234",&tester.db_pool,&tester.hosxp,&tester.kphis,&tester.kphis_extra).await;
        assert!(found.is_ok());
        let not_found = get_ipd_document_list("991231235959","990001234",&tester.db_pool,&tester.hosxp,&tester.kphis,&tester.kphis_extra).await;
        assert!(not_found.is_ok());
    }

    #[tokio::test]
    #[ignore]
    async fn sqlx_get_ipd_document_datetime() {
        let tester = MySqlTester::new_kphis().await;
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis/ipd_nurse_admission_note.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis/ipd_dr_admission_note.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis/ipd_summary_2.sql")).execute(&tester.db_pool).await.unwrap();

        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/kphis/ipd_nurse_admission_note.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/kphis/ipd_dr_admission_note.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/kphis/ipd_summary_2.sql")).execute(&tester.db_pool).await.unwrap();

        let found = get_ipd_document_datetime("660001234", &tester.db_pool, &tester.kphis).await;
        assert!(found.is_ok());
        let not_found = get_ipd_document_datetime("660006666", &tester.db_pool, &tester.kphis).await;
        assert!(not_found.is_ok());
    }

    #[tokio::test]
    #[ignore]
    async fn sqlx_get_ipd_document_types() {
        let tester = MySqlTester::new_kphis_extra().await;
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis_extra/ipd_document.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis_extra/image_usage.sql")).execute(&tester.db_pool).await.unwrap();

        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/kphis_extra/ipd_document.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/kphis_extra/image_usage.sql")).execute(&tester.db_pool).await.unwrap();

        let found = get_ipd_document_types("660001234", &tester.db_pool, &tester.kphis_extra).await.unwrap();
        assert_eq!(found.len(), 1);
        let not_found = get_ipd_document_types("660006666", &tester.db_pool, &tester.kphis_extra).await.unwrap();
        assert!(not_found.is_empty());
    }

    #[tokio::test]
    #[ignore]
    async fn sqlx_post_ipd_document_type() {
        let tester = MySqlTester::new_kphis_extra().await;
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis_extra/ipd_document.sql")).execute(&tester.db_pool).await.unwrap();

        let success = post_ipd_document_type("660001234", 1, "user", &tester.db_pool, &tester.kphis_extra).await.unwrap();
        assert_eq!(success.rows_affected, 1);
        let again_ignore = post_ipd_document_type("660001234", 1, "user", &tester.db_pool, &tester.kphis_extra).await.unwrap();
        assert_eq!(again_ignore.rows_affected, 0);
    }

    #[tokio::test]
    #[ignore]
    async fn sqlx_delete_ipd_document_type() {
        let tester = MySqlTester::new_kphis_extra().await;
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis_extra/ipd_document.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis_extra/image_usage.sql")).execute(&tester.db_pool).await.unwrap();

        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/kphis_extra/ipd_document.sql")).execute(&tester.db_pool).await.unwrap();

        let success = delete_ipd_document_type("660001234", 1, &tester.db_pool, &tester.kphis_extra).await.unwrap();
        assert_eq!(success.rows_affected, 1);
        let again_not_found = delete_ipd_document_type("660001234", 1, &tester.db_pool, &tester.kphis_extra).await.unwrap();
        assert_eq!(again_not_found.rows_affected, 0);

        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/kphis_extra/ipd_document.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/kphis_extra/image_usage.sql")).execute(&tester.db_pool).await.unwrap();

        let not_empty = delete_ipd_document_type("660001234", 1, &tester.db_pool, &tester.kphis_extra).await.unwrap();
        assert_eq!(not_empty.rows_affected, 0);
    }
}
