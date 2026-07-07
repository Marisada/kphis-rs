use sqlx::{AssertSqlSafe, FromRow, MySql, Pool, mysql::MySqlQueryResult};

use kphis_model::{
    fetch::ExecuteResponse,
    ipd::summary_audit::{SummaryAudit, SummaryAuditItem, SummaryAuditParams},
};
use kphis_sql::{TABLE_CREATE_COLUMNS, TABLE_CREATE_PREPARED, TABLE_UPDATE_SET, ipd::summary_audit};
use kphis_util::{
    error::{AppError, Source},
    util::zero_none,
};

pub async fn get_ipd_summary_audit(params: &SummaryAuditParams, pool: &Pool<MySql>, hosxp: &str, kphis: &str, kphis_extra: &str) -> Result<Vec<SummaryAudit>, AppError> {
    let mut summary_audits = get_summary_audit(params, pool, hosxp, kphis, kphis_extra).await?;

    for audit in summary_audits.iter_mut() {
        audit.summary_audit_items = select_summary_audit_item(audit.summary_audit_id, pool, kphis_extra).await?;
    }

    Ok(summary_audits)
}

async fn get_summary_audit(params: &SummaryAuditParams, pool: &Pool<MySql>, hosxp: &str, kphis: &str, kphis_extra: &str) -> Result<Vec<SummaryAudit>, AppError> {
    let summary_audit_sql = summary_audit::select_summary_audit_by(params.summary_id.is_some(), hosxp, kphis, kphis_extra);
    let mut summary_audit_query = sqlx::query(AssertSqlSafe(summary_audit_sql));
    let mut valid_params = true;
    if let Some(summary_id) = params.summary_id.as_ref() {
        summary_audit_query = summary_audit_query.bind(summary_id);
    } else if let Some(an) = params.an.as_ref() {
        summary_audit_query = summary_audit_query.bind(an);
    } else {
        valid_params = false;
    }
    let result = if valid_params {
        summary_audit_query
            .fetch_all(pool)
            .await
            .map_err(|e| Source::SQLx.to_error(500, e, "Select IpdSummaryAudit"))?
            .iter()
            .map(SummaryAudit::from_row)
            .collect::<sqlx::Result<Vec<SummaryAudit>>>()
            .map_err(|e| Source::SQLx.to_error(500, e, "Select IpdSummaryAudit"))?
    } else {
        Vec::new()
    };

    Ok(result)
}

async fn select_summary_audit_item(summary_audit_id: u32, pool: &Pool<MySql>, kphis_extra: &str) -> Result<Vec<SummaryAuditItem>, AppError> {
    let sql = summary_audit::select_summary_audit_item(kphis_extra);
    sqlx::query(AssertSqlSafe(sql))
        .bind(summary_audit_id)
        .fetch_all(pool)
        .await
        .map_err(|e| Source::SQLx.to_error(500, e, "Select IpdSummaryAuditItem"))?
        .iter()
        .map(SummaryAuditItem::from_row)
        .collect::<sqlx::Result<Vec<SummaryAuditItem>>>()
        .map_err(|e| Source::SQLx.to_error(500, e, "Select IpdSummaryAuditItem"))
}

pub async fn post_ipd_summary_audit(save: &SummaryAudit, user: &str, pool: &Pool<MySql>, kphis_extra: &str) -> Result<(u32, Vec<ExecuteResponse>), AppError> {
    let id;
    let mut results = Vec::with_capacity(11);
    let mut by_creator = true;
    // 1. insert/update summary_audit
    if let Some(summary_audit_id) = zero_none(save.summary_audit_id) {
        id = summary_audit_id;
        // 1.1.1 update
        let update_result = update_summary_audit(&save, user, pool, kphis_extra).await?;
        by_creator = update_result.rows_affected() != 0;
        results.push(ExecuteResponse::from_query_result(update_result, "Update IpdSummaryAudit"));
        if by_creator {
            // 1.1.2 delete items
            let delete_items_result = delete_summary_audit_items(id, pool, kphis_extra).await?;
            results.push(ExecuteResponse::from_query_result(delete_items_result, "Delete IpdSummaryAuditItems"));
        }
    } else {
        // 1.2 insert
        let insert_result = insert_summary_audit(save, user, pool, kphis_extra).await?;
        id = insert_result.last_insert_id() as u32;
        results.push(ExecuteResponse::from_query_result(insert_result, "Insert IpdSummaryAudit"));
    }

    // 2. insert items
    if by_creator && !save.summary_audit_items.is_empty() {
        let insert_items_result = insert_summary_audit_items(id, &save.summary_audit_items, user, pool, kphis_extra).await?;
        results.push(ExecuteResponse::from_query_result(insert_items_result, "Insert IpdSummaryAuditItems"));
    }

    Ok((id, results))
}

async fn insert_summary_audit(form: &SummaryAudit, user: &str, pool: &Pool<MySql>, kphis_extra: &str) -> Result<MySqlQueryResult, AppError> {
    form.insert(
        Some("summary_audit_id"),
        Some("ipd_summary_audit"),
        TABLE_CREATE_COLUMNS,
        TABLE_CREATE_PREPARED,
        &[user, user],
        pool,
        kphis_extra,
    )
    .await
    .map_err(|e| Source::SQLx.to_error(500, e, "Insert IpdSummaryAudit"))
}

async fn update_summary_audit(form: &SummaryAudit, user: &str, pool: &Pool<MySql>, kphis_extra: &str) -> Result<MySqlQueryResult, AppError> {
    form.update_by_creator("summary_audit_id", Some("ipd_summary_audit"), TABLE_UPDATE_SET, &[user], user, pool, kphis_extra)
        .await
        .map_err(|e| Source::SQLx.to_error(500, e, "Update IpdSummaryAudit"))
}

async fn insert_summary_audit_items(summary_audit_id: u32, items: &[SummaryAuditItem], user: &str, pool: &Pool<MySql>, kphis_extra: &str) -> Result<MySqlQueryResult, AppError> {
    let sql = summary_audit::insert_summary_audit_items(items.len(), kphis_extra);
    let mut query = sqlx::query(AssertSqlSafe(sql));
    for item in items.iter() {
        query = query
            .bind(summary_audit_id)
            .bind(item.summary_id)
            .bind(&item.ty)
            .bind(&item.sum_dx)
            .bind(&item.sum_icd)
            .bind(&item.com_icd)
            .bind(&item.rev_dx)
            .bind(&item.rev_icd)
            .bind(&item.sa)
            .bind(&item.ca)
            .bind(&item.remark)
            .bind(user)
            .bind(user);
    }
    query.execute(pool).await.map_err(|e| Source::SQLx.to_error(500, e, "Insert IpdSummaryAuditItems"))
}

async fn delete_summary_audit_items(summary_audit_id: u32, pool: &Pool<MySql>, kphis_extra: &str) -> Result<MySqlQueryResult, AppError> {
    let sql = summary_audit::delete_summary_audit_items(kphis_extra);
    sqlx::query(AssertSqlSafe(sql))
        .bind(summary_audit_id)
        .execute(pool)
        .await
        .map_err(|e| Source::SQLx.to_error(500, e, "Delete IpdSummaryAuditItems"))
}

pub async fn delete_summary_audit(summary_audit_id: u32, user: &str, pool: &Pool<MySql>, kphis_extra: &str) -> Result<MySqlQueryResult, AppError> {
    let sql = summary_audit::delete_summary_audit(kphis_extra);

    sqlx::query(AssertSqlSafe(sql))
        .bind(summary_audit_id)
        .bind(user)
        .execute(pool)
        .await
        .map_err(|e| Source::SQLx.to_error(500, e, "Delete IpdSummaryAudit"))
}

#[cfg(test)]
#[rustfmt::skip]
mod tests {

    use super::*;
    use kphis_sqlx_tester::MySqlTester;

    #[tokio::test]
    #[ignore]
    async fn sqlx_get_summary_audit() {
        let tester = MySqlTester::new_hosxp_and_kphis_and_kphis_extra().await;
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis/ipd_summary_2.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis_extra/ipd_summary_audit.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/hosxp/opduser.sql")).execute(&tester.db_pool).await.unwrap();

        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/kphis/ipd_summary_2.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/kphis_extra/ipd_summary_audit.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/hosxp/opduser.sql")).execute(&tester.db_pool).await.unwrap();

        // MUST have 'summary_id' or 'an' in params
        let default = get_summary_audit(&SummaryAuditParams::default(),&tester.db_pool,&tester.hosxp,&tester.kphis,&tester.kphis_extra).await.unwrap();
        assert!(default.is_empty());
        let found_summary_id = get_summary_audit(&SummaryAuditParams {summary_id: Some(1),..Default::default()},&tester.db_pool,&tester.hosxp,&tester.kphis,&tester.kphis_extra).await.unwrap();
        assert_eq!(found_summary_id.len(), 1);
        let found_an = get_summary_audit(&SummaryAuditParams {an: Some(String::from("660001234")),..Default::default()},&tester.db_pool,&tester.hosxp,&tester.kphis,&tester.kphis_extra).await.unwrap();
        assert_eq!(found_an.len(), 1);
        let not_found = get_summary_audit(&SummaryAuditParams {an: Some(String::from("1234")),..Default::default()},&tester.db_pool,&tester.hosxp,&tester.kphis,&tester.kphis_extra).await.unwrap();
        assert!(not_found.is_empty());
    }

    #[tokio::test]
    #[ignore]
    async fn sqlx_select_summary_audit_item() {
        let tester = MySqlTester::new_kphis_extra().await;
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis_extra/ipd_summary_audit_item.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/kphis_extra/ipd_summary_audit_item.sql")).execute(&tester.db_pool).await.unwrap();

        let found = select_summary_audit_item(1, &tester.db_pool, &tester.kphis_extra).await.unwrap();
        assert_eq!(found.len(), 4);
        let not_found = select_summary_audit_item(999, &tester.db_pool, &tester.kphis_extra).await.unwrap();
        assert!(not_found.is_empty());
    }


    #[tokio::test]
    #[ignore]
    async fn sqlx_insert_summary_audit() {
        let tester = MySqlTester::new_kphis_extra().await;
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis_extra/ipd_summary_audit.sql")).execute(&tester.db_pool).await.unwrap();

        let success = insert_summary_audit(&SummaryAudit::demo(),"user",&tester.db_pool,&tester.kphis_extra).await.unwrap();
        assert_eq!(success.rows_affected(), 1);
        let again_success = insert_summary_audit(&SummaryAudit::demo(),"user",&tester.db_pool,&tester.kphis_extra).await.unwrap();
        assert_eq!(again_success.rows_affected(), 1);
    }

    #[tokio::test]
    #[ignore]
    async fn sqlx_update_summary_audit() {
        let tester = MySqlTester::new_kphis_extra().await;
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis_extra/ipd_summary_audit.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/kphis_extra/ipd_summary_audit.sql")).execute(&tester.db_pool).await.unwrap();
    
        let not_creator = update_summary_audit(&SummaryAudit::demo(),"anyone",&tester.db_pool,&tester.kphis_extra).await.unwrap();
        assert_eq!(not_creator.rows_affected(), 0);
        let success = update_summary_audit(&SummaryAudit::demo(),"user",&tester.db_pool,&tester.kphis_extra).await.unwrap();
        assert_eq!(success.rows_affected(), 1);
        let again_success = update_summary_audit(&SummaryAudit::demo(),"user",&tester.db_pool,&tester.kphis_extra).await.unwrap();
        assert_eq!(again_success.rows_affected(), 1);
    }

    #[tokio::test]
    #[ignore]
    async fn sqlx_insert_summary_audit_items() {
        let tester = MySqlTester::new_kphis_extra().await;
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis_extra/ipd_summary_audit_item.sql")).execute(&tester.db_pool).await.unwrap();
    
        let success = insert_summary_audit_items(1, &[SummaryAuditItem::demo()],"user",&tester.db_pool,&tester.kphis_extra).await.unwrap();
        assert_eq!(success.rows_affected(), 1);
        let again_success = insert_summary_audit_items(1, &[SummaryAuditItem::demo()],"user",&tester.db_pool,&tester.kphis_extra).await.unwrap();
        assert_eq!(again_success.rows_affected(), 1);
    }

    #[tokio::test]
    #[ignore]
    async fn sqlx_delete_summary_audit_items() {
        let tester = MySqlTester::new_kphis_extra().await;
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis_extra/ipd_summary_audit_item.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/kphis_extra/ipd_summary_audit_item.sql")).execute(&tester.db_pool).await.unwrap();
    
        let success = delete_summary_audit_items(1,&tester.db_pool,&tester.kphis_extra).await.unwrap();
        assert_eq!(success.rows_affected(), 4);
        let again_success = delete_summary_audit_items(1,&tester.db_pool,&tester.kphis_extra).await.unwrap();
        assert_eq!(again_success.rows_affected(), 0);
    }

    #[tokio::test]
    #[ignore]
    async fn sqlx_delete_summary_audit() {
        let tester = MySqlTester::new_kphis_extra().await;
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis_extra/ipd_summary_audit.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis_extra/ipd_summary_audit_item.sql")).execute(&tester.db_pool).await.unwrap();

        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/kphis_extra/ipd_summary_audit.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/kphis_extra/ipd_summary_audit_item.sql")).execute(&tester.db_pool).await.unwrap();

        let not_creator = delete_summary_audit(1,"anyone",&tester.db_pool,&tester.kphis_extra).await.unwrap();
        assert_eq!(not_creator.rows_affected(), 0);
        let success = delete_summary_audit(1,"user",&tester.db_pool,&tester.kphis_extra).await.unwrap();
        assert_eq!(success.rows_affected(), 5);
        let again_success = delete_summary_audit(1,"user",&tester.db_pool,&tester.kphis_extra).await.unwrap();
        assert_eq!(again_success.rows_affected(), 0);
    }

}
