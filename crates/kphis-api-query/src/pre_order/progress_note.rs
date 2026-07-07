use futures_util::stream::StreamExt;
use sqlx::{AssertSqlSafe, Executor, FromRow, MySql, Pool, Row, mysql::MySqlQueryResult};
use std::collections::HashMap;

use kphis_model::{
    fetch::ExecuteResponse,
    pre_order::progress_note::{PreProgressNote, PreProgressNoteItem, PreProgressNoteParams, PreProgressNoteSave},
    progress_note::ProgressNoteItemSave,
};
use kphis_sql::{ipd::progress_note, opd_er, pre_order};
use kphis_util::error::{AppError, Source};

use crate::query_all;

// ipd-dr-pre-order-progress-note-data.php
pub async fn get_progress_note(params: &PreProgressNoteParams, intern_roles: &[String], pool: &Pool<MySql>, hosxp: &str, kphis: &str) -> Result<Vec<PreProgressNote>, AppError> {
    if params.pre_order_master_id.is_none() && params.progress_note_id.is_none() {
        return Ok(Vec::new());
    }

    let sql = pre_order::progress_note::select_progress(params, intern_roles, hosxp, kphis);
    let mut query = sqlx::query(AssertSqlSafe(sql));
    if let Some(progress_note_id) = params.progress_note_id.as_ref() {
        query = query.bind(progress_note_id);
    }
    if let Some(pre_order_master_id) = params.pre_order_master_id.as_ref() {
        query = query.bind(pre_order_master_id);
    }
    query
        .fetch_all(pool)
        .await
        .map(|rows| {
            rows.iter()
                .map(PreProgressNote::from_row)
                .collect::<sqlx::Result<Vec<PreProgressNote>>>()
                .map_err(|e| Source::SQLx.to_error(500, e, "Select PreProgressNote"))
        })
        .map_err(|e| Source::SQLx.to_error(500, e, "Select PreProgressNote"))?
}

pub async fn get_progress_note_types(ids: &[u32], pool: &Pool<MySql>, kphis: &str) -> Result<Vec<(u32, String)>, AppError> {
    if ids.is_empty() {
        Ok(Vec::new())
    } else {
        let sql = pre_order::progress_note::select_progress_type(ids, kphis);
        query_all(&sql, pool, "Select ProgressNoteType").await.map(|rows| {
            rows.iter()
                .filter_map(|row| {
                    let item_type: Option<String> = row.try_get("progress_note_item_type").ok();
                    let id: Option<u32> = row.try_get("progress_note_id").ok();
                    id.zip(item_type)
                })
                .collect()
        })
    }
}

pub async fn get_progress_note_item(id: u32, item_type: &str, pool: &Pool<MySql>, kphis: &str) -> Result<Vec<PreProgressNoteItem>, AppError> {
    let sql = pre_order::progress_note::select_progress_item(id, item_type, kphis);
    query_all(&sql, pool, "Select PreProgressNoteItem")
        .await?
        .iter()
        .map(PreProgressNoteItem::from_row)
        .collect::<sqlx::Result<Vec<PreProgressNoteItem>>>()
        .map_err(|e| Source::SQLx.to_error(500, e, "Select PreProgressNoteItem"))
}

pub async fn select_progress_note_to(pre_order_master_id: u32, pool: &Pool<MySql>, kphis: &str) -> Result<Vec<PreProgressNoteSave>, AppError> {
    let select_note_sql = pre_order::progress_note::select_progress_note_to(kphis);
    sqlx::query(AssertSqlSafe(select_note_sql))
        .bind(pre_order_master_id)
        .fetch_all(pool)
        .await
        .map_err(|e| Source::SQLx.to_error(500, e, "Select PreProgressNote"))?
        .iter()
        .map(PreProgressNoteSave::from_row)
        .collect::<sqlx::Result<Vec<PreProgressNoteSave>>>()
        .map_err(|e| Source::SQLx.to_error(500, e, "Select PreProgressNote"))
}

pub async fn select_progress_note_item_to(note_ids: &[u32], pool: &Pool<MySql>, kphis: &str) -> Result<Vec<PreProgressNoteItem>, AppError> {
    let select_note_item_sql = pre_order::progress_note::select_progress_note_item_to(note_ids, kphis);
    sqlx::query(AssertSqlSafe(select_note_item_sql))
        .fetch_all(pool)
        .await
        .map_err(|e| Source::SQLx.to_error(500, e, "Select PreProgressNoteItem"))?
        .iter()
        .map(PreProgressNoteItem::from_row)
        .collect::<sqlx::Result<Vec<PreProgressNoteItem>>>()
        .map_err(|e| Source::SQLx.to_error(500, e, "Select PreProgressNoteItem"))
}

// ipd-dr-pre-order-progress-note-save.php
pub async fn post_progress_note(save: &PreProgressNoteSave, user: &str, pool: &Pool<MySql>, kphis: &str) -> Result<(u32, Vec<ExecuteResponse>), AppError> {
    if save.progress_note_items.is_empty() || save.pre_order_master_id == 0 {
        return Ok((0, Vec::new()));
    }

    let progress_note_id;
    let mut results = Vec::with_capacity(2 + save.progress_note_items.len());
    if save.progress_note_id > 0 {
        progress_note_id = save.progress_note_id;
        let update_result = update_progress_note_id(progress_note_id, save.pre_order_master_id, user, pool, kphis).await?;
        results.push(ExecuteResponse::from_query_result(update_result, "Update ProgressNote"));
        let delete_item_result = delete_progress_note_item(progress_note_id, pool, kphis).await?;
        results.push(ExecuteResponse::from_query_result(delete_item_result, "Delete ProgressNoteItem"));
    } else {
        let insert_result = insert_pre_order_progress_note(save.pre_order_master_id, save, user, pool, kphis).await?;
        progress_note_id = insert_result.last_insert_id() as u32;
        results.push(ExecuteResponse::from_query_result(insert_result, "Insert ProgressNote"));
    }
    if progress_note_id > 0 {
        for progress_note_item in save.progress_note_items.iter() {
            let insert_progress_note_item_result = insert_pre_order_progress_note_item(progress_note_id, save.pre_order_master_id, progress_note_item, user, pool, kphis).await?;
            results.push(ExecuteResponse::from_query_result(insert_progress_note_item_result, "Insert ProgressNoteItem"));
        }
    }

    Ok((progress_note_id, results))
}

async fn insert_pre_order_progress_note(pre_order_master_id: u32, save: &PreProgressNoteSave, user: &str, pool: &Pool<MySql>, kphis: &str) -> Result<MySqlQueryResult, AppError> {
    let insert_sql = pre_order::progress_note::insert_progress_note(kphis);
    sqlx::query(AssertSqlSafe(insert_sql))
        .bind(pre_order_master_id)
        .bind(&save.progress_note_owner_type)
        .bind(&save.progress_note_doctor)
        .bind(user)
        .bind(user)
        .execute(pool)
        .await
        .map_err(|e| Source::SQLx.to_error(500, e, "Insert ProgressNote"))
}

pub async fn insert_many_pre_order_progress_notes(
    notes: &[PreProgressNoteSave],
    pre_order_master_id: u32,
    loginname: &str,
    doctorcode: &str,
    pool: &Pool<MySql>,
    kphis: &str,
) -> Result<Vec<u64>, AppError> {
    let insert_note_sql = pre_order::progress_note::insert_many_progress_notes(notes, pre_order_master_id, loginname, doctorcode, kphis);
    let res = pool
        .execute_many(AssertSqlSafe(insert_note_sql))
        .collect::<Vec<_>>()
        .await
        .into_iter()
        .collect::<sqlx::Result<Vec<MySqlQueryResult>>>()
        .map_err(|e| Source::SQLx.to_error(500, e, "Insert PreOrderProgressNote"))?
        .into_iter()
        .map(|r| r.last_insert_id())
        .collect::<Vec<u64>>();
    Ok(res)
}

async fn insert_pre_order_progress_note_item(
    progress_note_id: u32,
    pre_order_master_id: u32,
    progress_note_item: &ProgressNoteItemSave,
    user: &str,
    pool: &Pool<MySql>,
    kphis: &str,
) -> Result<MySqlQueryResult, AppError> {
    let insert_progress_note_item_sql = pre_order::progress_note::insert_progress_note_item(kphis);
    sqlx::query(AssertSqlSafe(insert_progress_note_item_sql))
        .bind(progress_note_id)
        .bind(pre_order_master_id)
        .bind(&progress_note_item.progress_note_item_type)
        .bind(&progress_note_item.progress_note_item_detail)
        .bind(&progress_note_item.progress_note_item_detail_2)
        .bind(user)
        .bind(user)
        .execute(pool)
        .await
        .map_err(|e| Source::SQLx.to_error(500, e, "Insert ProgressNoteItem"))
}

pub async fn insert_pre_order_progress_note_items(
    note_items: &[PreProgressNoteItem],
    note_id_map: &HashMap<u32, u64>,
    pre_order_master_id: u32,
    loginname: &str,
    pool: &Pool<MySql>,
    kphis: &str,
) -> Result<MySqlQueryResult, AppError> {
    let insert_note_item_sql = pre_order::progress_note::insert_progress_note_items(note_items, note_id_map, pre_order_master_id, loginname, kphis);
    sqlx::query(AssertSqlSafe(insert_note_item_sql))
        .execute(pool)
        .await
        .map_err(|e| Source::SQLx.to_error(500, e, "Insert PreOrderProgressNoteItem"))
}

pub async fn insert_many_progress_notes_from_pre_order(notes: &[PreProgressNoteSave], an: &str, loginname: &str, pool: &Pool<MySql>, kphis: &str) -> Result<Vec<u64>, AppError> {
    let insert_note_sql = progress_note::insert_many_progress_notes_from_pre_order(notes, an, loginname, kphis);
    let res = pool
        .execute_many(AssertSqlSafe(insert_note_sql))
        .collect::<Vec<_>>()
        .await
        .into_iter()
        .collect::<sqlx::Result<Vec<MySqlQueryResult>>>()
        .map_err(|e| Source::SQLx.to_error(500, e, "Insert PreProgressNote to ProgressNote"))?
        .into_iter()
        .map(|r| r.last_insert_id())
        .collect::<Vec<u64>>();
    Ok(res)
}

pub async fn insert_many_progress_notes_from_template(notes: &[PreProgressNoteSave], an: &str, loginname: &str, doctorcode: &str, pool: &Pool<MySql>, kphis: &str) -> Result<Vec<u64>, AppError> {
    let insert_note_sql = progress_note::insert_many_progress_notes_from_template(notes, an, loginname, doctorcode, kphis);
    let res = pool
        .execute_many(AssertSqlSafe(insert_note_sql))
        .collect::<Vec<_>>()
        .await
        .into_iter()
        .collect::<sqlx::Result<Vec<MySqlQueryResult>>>()
        .map_err(|e| Source::SQLx.to_error(500, e, "Insert TemplateProgressNote to ProgressNote"))?
        .into_iter()
        .map(|r| r.last_insert_id())
        .collect::<Vec<u64>>();
    Ok(res)
}

pub async fn insert_ipd_progress_note_items(
    note_items: &[PreProgressNoteItem],
    note_id_map: &HashMap<u32, u64>,
    an: &str,
    loginname: &str,
    pool: &Pool<MySql>,
    kphis: &str,
) -> Result<MySqlQueryResult, AppError> {
    let insert_note_item_sql = progress_note::insert_progress_note_items(note_items, note_id_map, an, loginname, kphis);
    sqlx::query(AssertSqlSafe(insert_note_item_sql))
        .execute(pool)
        .await
        .map_err(|e| Source::SQLx.to_error(500, e, "Insert PreProgressNoteItem to ProgressNoteItem"))
}

pub async fn insert_many_opd_er_progress_notes(
    notes: &[PreProgressNoteSave],
    opd_er_order_master_id: u32,
    loginname: &str,
    doctorcode: &str,
    pool: &Pool<MySql>,
    kphis: &str,
) -> Result<Vec<u64>, AppError> {
    let insert_note_sql = opd_er::progress_note::insert_many_progress_notes(notes, opd_er_order_master_id, loginname, doctorcode, kphis);
    let res = pool
        .execute_many(AssertSqlSafe(insert_note_sql))
        .collect::<Vec<_>>()
        .await
        .into_iter()
        .collect::<sqlx::Result<Vec<MySqlQueryResult>>>()
        .map_err(|e| Source::SQLx.to_error(500, e, "Insert TemplateProgressNote to ProgressNote"))?
        .into_iter()
        .map(|r| r.last_insert_id())
        .collect::<Vec<u64>>();
    Ok(res)
}

pub async fn insert_opd_er_progress_note_items(
    note_items: &[PreProgressNoteItem],
    note_id_map: &HashMap<u32, u64>,
    opd_er_order_master_id: u32,
    loginname: &str,
    pool: &Pool<MySql>,
    kphis: &str,
) -> Result<MySqlQueryResult, AppError> {
    let insert_note_item_sql = opd_er::progress_note::insert_progress_note_items(note_items, note_id_map, opd_er_order_master_id, loginname, kphis);
    sqlx::query(AssertSqlSafe(insert_note_item_sql))
        .execute(pool)
        .await
        .map_err(|e| Source::SQLx.to_error(500, e, "Insert OpdEr ProgresNoteItems"))
}

async fn update_progress_note_id(progress_note_id: u32, pre_order_master_id: u32, user: &str, pool: &Pool<MySql>, kphis: &str) -> Result<MySqlQueryResult, AppError> {
    let update_an_sql = pre_order::progress_note::update_progress_note_id(kphis);
    sqlx::query(AssertSqlSafe(update_an_sql))
        .bind(pre_order_master_id)
        .bind(user)
        .bind(progress_note_id)
        .execute(pool)
        .await
        .map_err(|e| Source::SQLx.to_error(500, e, "Update ProgressNote"))
}

// ipd-dr-pre-order-progress-note-delete.php
/// delete ipd_pre_order_progress_note and ipd_pre_order_progress_note_item
pub async fn delete_progress_note(progress_note_id: u32, pool: &Pool<MySql>, kphis: &str) -> Result<ExecuteResponse, AppError> {
    let sql = pre_order::progress_note::delete_progress_note(kphis);
    let delete_result = sqlx::query(AssertSqlSafe(sql))
        .bind(progress_note_id)
        .execute(pool)
        .await
        .map_err(|e| Source::SQLx.to_error(500, e, "Delete ProgressNote"))?;

    Ok(ExecuteResponse::from_query_result(delete_result, "Delete ProgressNote"))
}

pub async fn delete_progress_note_by_master_id(pre_order_master_id: u32, pool: &Pool<MySql>, kphis: &str) -> Result<MySqlQueryResult, AppError> {
    let delete_progress_note_sql = pre_order::progress_note::delete_progress_note_by_master_id(kphis);
    sqlx::query(AssertSqlSafe(delete_progress_note_sql))
        .bind(pre_order_master_id)
        .execute(pool)
        .await
        .map_err(|e| Source::SQLx.to_error(500, e, "Delete ProgressNote"))
}

pub async fn delete_progress_note_item_by_master_id(pre_order_master_id: u32, pool: &Pool<MySql>, kphis: &str) -> Result<MySqlQueryResult, AppError> {
    let delete_progress_note_item_sql = pre_order::progress_note::delete_progress_note_item_by_master_id(kphis);
    sqlx::query(AssertSqlSafe(delete_progress_note_item_sql))
        .bind(pre_order_master_id)
        .execute(pool)
        .await
        .map_err(|e| Source::SQLx.to_error(500, e, "Delete ProgressNoteItem"))
}

async fn delete_progress_note_item(progress_note_id: u32, pool: &Pool<MySql>, kphis: &str) -> Result<MySqlQueryResult, AppError> {
    let delete_item_sql = pre_order::progress_note::delete_progress_note_item(kphis);
    sqlx::query(AssertSqlSafe(delete_item_sql))
        .bind(progress_note_id)
        .execute(pool)
        .await
        .map_err(|e| Source::SQLx.to_error(500, e, "Delete ProgressNoteItem"))
}

#[cfg(test)]
#[rustfmt::skip]
mod tests {

    use super::*;
    use kphis_sqlx_tester::MySqlTester;

    #[tokio::test]
    #[ignore]
    async fn sqlx_get_progress_note() {
        let tester = MySqlTester::new_hosxp_and_kphis().await;
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis/ipd_pre_order_progress_note.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis/system_ac_role.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis/system_acc_role_user.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/hosxp/doctor.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/hosxp/opduser.sql")).execute(&tester.db_pool).await.unwrap();

        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/kphis/ipd_pre_order_progress_note.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/kphis/system_ac_role.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/kphis/system_acc_role_user.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/hosxp/doctor.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/hosxp/opduser.sql")).execute(&tester.db_pool).await.unwrap();

        // MUST have 'progress_note_id' or 'pre_order_master_id' in params
        let default = get_progress_note(&PreProgressNoteParams::default(),&[String::from("DOCTOR_INTERN")],&tester.db_pool,&tester.hosxp,&tester.kphis).await.unwrap();
        assert!(default.is_empty());
        let found_progress_note_id = get_progress_note(&PreProgressNoteParams {progress_note_id: Some(1),..Default::default()},&[String::from("DOCTOR_INTERN")],&tester.db_pool,&tester.hosxp,&tester.kphis).await.unwrap();
        assert_eq!(found_progress_note_id.len(), 1);
        let found_pre_order_master_id = get_progress_note(&PreProgressNoteParams {pre_order_master_id: Some(1),..Default::default()},&[String::from("DOCTOR_INTERN")],&tester.db_pool,&tester.hosxp,&tester.kphis).await.unwrap();
        assert_eq!(found_pre_order_master_id.len(), 4);
    }

    #[tokio::test]
    #[ignore]
    async fn sqlx_get_progress_note_types() {
        let tester = MySqlTester::new_kphis().await;
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis/ipd_pre_order_progress_note.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis/ipd_pre_order_progress_note_item.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis/ipd_progress_note_item_type.sql")).execute(&tester.db_pool).await.unwrap();

        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/kphis/ipd_pre_order_progress_note.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/kphis/ipd_pre_order_progress_note_item.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/kphis/ipd_progress_note_item_type.sql")).execute(&tester.db_pool).await.unwrap();

        let found = get_progress_note_types(&[1], &tester.db_pool, &tester.kphis).await.unwrap();
        assert_eq!(found.len(), 6);
        let not_found = get_progress_note_types(&[], &tester.db_pool, &tester.kphis).await.unwrap();
        assert!(not_found.is_empty());
    }

    #[tokio::test]
    #[ignore]
    async fn sqlx_get_progress_note_item() {
        let tester = MySqlTester::new_kphis().await;
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis/ipd_pre_order_progress_note.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis/ipd_pre_order_progress_note_item.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis/ipd_progress_note_item_type.sql")).execute(&tester.db_pool).await.unwrap();

        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/kphis/ipd_pre_order_progress_note.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/kphis/ipd_pre_order_progress_note_item.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/kphis/ipd_progress_note_item_type.sql")).execute(&tester.db_pool).await.unwrap();

        let found = get_progress_note_item(1, "note", &tester.db_pool, &tester.kphis).await.unwrap();
        assert_eq!(found.len(), 1);
        let not_found = get_progress_note_item(999, "note", &tester.db_pool, &tester.kphis).await.unwrap();
        assert!(not_found.is_empty());
    }

    #[tokio::test]
    #[ignore]
    async fn sqlx_select_progress_note_to() {
        let tester = MySqlTester::new_kphis().await;
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis/ipd_pre_order_progress_note.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/kphis/ipd_pre_order_progress_note.sql")).execute(&tester.db_pool).await.unwrap();

        let found = select_progress_note_to(1, &tester.db_pool, &tester.kphis).await.unwrap();
        assert_eq!(found.len(), 4);
        let not_found = select_progress_note_to(999, &tester.db_pool, &tester.kphis).await.unwrap();
        assert!(not_found.is_empty());
    }

    #[tokio::test]
    #[ignore]
    async fn sqlx_select_progress_note_item_to() {
        let tester = MySqlTester::new_kphis().await;
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis/ipd_pre_order_progress_note_item.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/kphis/ipd_pre_order_progress_note_item.sql")).execute(&tester.db_pool).await.unwrap();

        let found = select_progress_note_item_to(&[1], &tester.db_pool, &tester.kphis).await.unwrap();
        assert_eq!(found.len(), 6);
        let not_found = select_progress_note_item_to(&[], &tester.db_pool, &tester.kphis).await.unwrap();
        assert!(not_found.is_empty());
    }

    #[tokio::test]
    #[ignore]
    async fn sqlx_insert_pre_order_progress_note() {
        let tester = MySqlTester::new_kphis().await;
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis/ipd_pre_order_progress_note.sql")).execute(&tester.db_pool).await.unwrap();

        let success = insert_pre_order_progress_note(1,&PreProgressNoteSave::demo(),"user",&tester.db_pool,&tester.kphis).await.unwrap();
        assert_eq!(success.rows_affected(), 1);
        let again_success = insert_pre_order_progress_note(1,&PreProgressNoteSave::demo(),"user",&tester.db_pool,&tester.kphis).await.unwrap();
        assert_eq!(again_success.rows_affected(), 1);
    }

    #[tokio::test]
    #[ignore]
    async fn sqlx_insert_many_pre_order_progress_notes() {
        let tester = MySqlTester::new_kphis().await;
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis/ipd_pre_order_progress_note.sql")).execute(&tester.db_pool).await.unwrap();

        let success = insert_many_pre_order_progress_notes(&[PreProgressNoteSave::demo(),PreProgressNoteSave::demo()],1,"user","007",&tester.db_pool,&tester.kphis).await.unwrap();
        assert_eq!(success.len(), 2);
        let again_success = insert_many_pre_order_progress_notes(&[PreProgressNoteSave::demo(),PreProgressNoteSave::demo()],1,"user","007",&tester.db_pool,&tester.kphis).await.unwrap();
        assert_eq!(again_success.len(), 2);
    }

    #[tokio::test]
    #[ignore]
    async fn sqlx_insert_pre_order_progress_note_item() {
        let tester = MySqlTester::new_kphis().await;
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis/ipd_pre_order_progress_note_item.sql")).execute(&tester.db_pool).await.unwrap();

        let success = insert_pre_order_progress_note_item(1,1,&ProgressNoteItemSave::demo(),"user",&tester.db_pool,&tester.kphis).await.unwrap();
        assert_eq!(success.rows_affected(), 1);
        let again_success = insert_pre_order_progress_note_item(1,1,&ProgressNoteItemSave::demo(),"user",&tester.db_pool,&tester.kphis).await.unwrap();
        assert_eq!(again_success.rows_affected(), 1);
    }

    #[tokio::test]
    #[ignore]
    async fn sqlx_insert_pre_order_progress_note_items() {
        let tester = MySqlTester::new_kphis().await;
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis/ipd_pre_order_progress_note_item.sql")).execute(&tester.db_pool).await.unwrap();

        let mut note_id_map = HashMap::new();
        note_id_map.insert(1, 1);
        let success = insert_pre_order_progress_note_items(&[PreProgressNoteItem::demo()],&note_id_map,1,"user",&tester.db_pool,&tester.kphis).await.unwrap();
        assert_eq!(success.rows_affected(), 1);
        let again_success = insert_pre_order_progress_note_items(&[PreProgressNoteItem::demo()],&note_id_map,1,"user",&tester.db_pool,&tester.kphis).await.unwrap();
        assert_eq!(again_success.rows_affected(), 1);
    }

    #[tokio::test]
    #[ignore]
    async fn sqlx_insert_many_progress_notes_from_pre_order() {
        let tester = MySqlTester::new_kphis().await;
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis/ipd_progress_note.sql")).execute(&tester.db_pool).await.unwrap();

        let success = insert_many_progress_notes_from_pre_order(&[PreProgressNoteSave::demo(),PreProgressNoteSave::demo()],"660001234","user",&tester.db_pool,&tester.kphis).await.unwrap();
        assert_eq!(success.len(), 2);
        let again_success = insert_many_progress_notes_from_pre_order(&[PreProgressNoteSave::demo(),PreProgressNoteSave::demo()],"660001234","user",&tester.db_pool,&tester.kphis).await.unwrap();
        assert_eq!(again_success.len(), 2);
    }

    #[tokio::test]
    #[ignore]
    async fn sqlx_insert_many_progress_notes_from_template() {
        let tester = MySqlTester::new_kphis().await;
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis/ipd_progress_note.sql")).execute(&tester.db_pool).await.unwrap();

        let success = insert_many_progress_notes_from_template(&[PreProgressNoteSave::demo(),PreProgressNoteSave::demo()],"660001234","user","007",&tester.db_pool,&tester.kphis).await.unwrap();
        assert_eq!(success.len(), 2);
        let again_success = insert_many_progress_notes_from_template(&[PreProgressNoteSave::demo(),PreProgressNoteSave::demo()],"660001234","user","007",&tester.db_pool,&tester.kphis).await.unwrap();
        assert_eq!(again_success.len(), 2);
    }

    #[tokio::test]
    #[ignore]
    async fn sqlx_insert_ipd_progress_note_items() {
        let tester = MySqlTester::new_kphis().await;
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis/ipd_progress_note_item.sql")).execute(&tester.db_pool).await.unwrap();

        let mut note_id_map = HashMap::new();
        note_id_map.insert(1, 1);
        let success = insert_ipd_progress_note_items(&[PreProgressNoteItem::demo()],&note_id_map,"660001234","user",&tester.db_pool,&tester.kphis).await.unwrap();
        assert_eq!(success.rows_affected(), 1);
        let again_success = insert_ipd_progress_note_items(&[PreProgressNoteItem::demo()],&note_id_map,"660001234","user",&tester.db_pool,&tester.kphis).await.unwrap();
        assert_eq!(again_success.rows_affected(), 1);
    }

    #[tokio::test]
    #[ignore]
    async fn sqlx_insert_many_opd_er_progress_notes() {
        let tester = MySqlTester::new_kphis().await;
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis/opd_er_order_progress_note.sql")).execute(&tester.db_pool).await.unwrap();

        let success = insert_many_opd_er_progress_notes(&[PreProgressNoteSave::demo(),PreProgressNoteSave::demo()],1,"user","007",&tester.db_pool,&tester.kphis).await.unwrap();
        assert_eq!(success.len(), 2);
        let again_success = insert_many_opd_er_progress_notes(&[PreProgressNoteSave::demo(),PreProgressNoteSave::demo()],1,"user","007",&tester.db_pool,&tester.kphis).await.unwrap();
        assert_eq!(again_success.len(), 2);
    }

    #[tokio::test]
    #[ignore]
    async fn sqlx_insert_opd_er_progress_note_items() {
        let tester = MySqlTester::new_kphis().await;
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis/opd_er_order_progress_note_item.sql")).execute(&tester.db_pool).await.unwrap();

        let mut note_id_map = HashMap::new();
        note_id_map.insert(1, 1);
        let success = insert_opd_er_progress_note_items(&[PreProgressNoteItem::demo()],&note_id_map,1,"user",&tester.db_pool,&tester.kphis).await.unwrap();
        assert_eq!(success.rows_affected(), 1);
        let again_success = insert_opd_er_progress_note_items(&[PreProgressNoteItem::demo()],&note_id_map,1,"user",&tester.db_pool,&tester.kphis).await.unwrap();
        assert_eq!(again_success.rows_affected(), 1);
    }

    #[tokio::test]
    #[ignore]
    async fn sqlx_update_progress_note_id() {
        let tester = MySqlTester::new_kphis().await;
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis/ipd_pre_order_progress_note.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/kphis/ipd_pre_order_progress_note.sql")).execute(&tester.db_pool).await.unwrap();

        let success = update_progress_note_id(1, 1, "user", &tester.db_pool, &tester.kphis).await.unwrap();
        assert_eq!(success.rows_affected(), 1);
        let again_success = update_progress_note_id(1, 1, "user", &tester.db_pool, &tester.kphis).await.unwrap();
        assert_eq!(again_success.rows_affected(), 1);
    }

    #[tokio::test]
    #[ignore]
    async fn sqlx_delete_progress_note() {
        let tester = MySqlTester::new_kphis().await;
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis/ipd_pre_order_progress_note.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis/ipd_pre_order_progress_note_item.sql")).execute(&tester.db_pool).await.unwrap();

        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/kphis/ipd_pre_order_progress_note.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/kphis/ipd_pre_order_progress_note_item.sql")).execute(&tester.db_pool).await.unwrap();

        let success = delete_progress_note(1, &tester.db_pool, &tester.kphis).await.unwrap();
        assert_eq!(success.rows_affected, 7);
        let again_not_found = delete_progress_note(1, &tester.db_pool, &tester.kphis).await.unwrap();
        assert_eq!(again_not_found.rows_affected, 0);
    }

    #[tokio::test]
    #[ignore]
    async fn sqlx_delete_progress_note_by_master_id() {
        let tester = MySqlTester::new_kphis().await;
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis/ipd_pre_order_progress_note.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/kphis/ipd_pre_order_progress_note.sql")).execute(&tester.db_pool).await.unwrap();

        let success = delete_progress_note_by_master_id(1, &tester.db_pool, &tester.kphis).await.unwrap();
        assert_eq!(success.rows_affected(), 4);
        let again_not_found = delete_progress_note_by_master_id(1, &tester.db_pool, &tester.kphis).await.unwrap();
        assert_eq!(again_not_found.rows_affected(), 0);
    }

    #[tokio::test]
    #[ignore]
    async fn sqlx_delete_progress_note_item_by_master_id() {
        let tester = MySqlTester::new_kphis().await;
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis/ipd_pre_order_progress_note_item.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/kphis/ipd_pre_order_progress_note_item.sql")).execute(&tester.db_pool).await.unwrap();

        let success = delete_progress_note_item_by_master_id(1, &tester.db_pool, &tester.kphis).await.unwrap();
        assert_eq!(success.rows_affected(), 6);
        let again_not_found = delete_progress_note_item_by_master_id(1, &tester.db_pool, &tester.kphis).await.unwrap();
        assert_eq!(again_not_found.rows_affected(), 0);
    }

    #[tokio::test]
    #[ignore]
    async fn sqlx_delete_progress_note_item() {
        let tester = MySqlTester::new_kphis().await;
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis/ipd_pre_order_progress_note_item.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/kphis/ipd_pre_order_progress_note_item.sql")).execute(&tester.db_pool).await.unwrap();

        let success = delete_progress_note_item(1, &tester.db_pool, &tester.kphis).await.unwrap();
        assert_eq!(success.rows_affected(), 6);
        let again_not_found = delete_progress_note_item(1, &tester.db_pool, &tester.kphis).await.unwrap();
        assert_eq!(again_not_found.rows_affected(), 0);
    }
}
