use sqlx::{
    AssertSqlSafe, FromRow, MySql, Pool, Row,
    mysql::{MySqlQueryResult, MySqlRow},
};

use kphis_model::{
    app::VisitTypeId,
    fetch::ExecuteResponse,
    progress_note::{ProgressNote, ProgressNoteItem, ProgressNoteItemOnly, ProgressNoteItemSave, ProgressNoteOnly, ProgressNoteParams, ProgressNoteSave},
};
use kphis_sql::ipd::progress_note;
use kphis_util::{
    error::{AppError, Source},
    util::str_some,
};

use crate::{query_all, query4_all};

// ipd-dr-order-progress-note-previous-problem-list-data.php
pub async fn get_previous_progress(an: &str, progress_note_owner_type: &str, pool: &Pool<MySql>, kphis: &str) -> Result<Vec<ProgressNoteItem>, AppError> {
    let sql = progress_note::select_progress_previous(kphis);
    query4_all(an, progress_note_owner_type, an, progress_note_owner_type, &sql, pool, "Select PreviousProgressNote")
        .await?
        .iter()
        .map(progress_note_item_from_row)
        .collect::<sqlx::Result<Vec<ProgressNoteItem>>>()
        .map_err(|e| Source::SQLx.to_error(500, e, "Select PreviousProgressNote"))
}
fn progress_note_item_from_row(row: &MySqlRow) -> sqlx::Result<ProgressNoteItem> {
    let an: String = row.try_get("an")?;
    Ok(ProgressNoteItem {
        visit_type: VisitTypeId::Ipd(an.clone()),
        progress_note_item_id: row.try_get("progress_note_item_id")?,
        progress_note_id: row.try_get("progress_note_id")?,
        progress_note_item_type: row.try_get("progress_note_item_type")?,
        progress_note_item_detail: row.try_get("progress_note_item_detail")?,
        progress_note_item_detail_2: row.try_get("progress_note_item_detail_2")?,
    })
}

// ipd-dr-order-progress-note-data.php
pub async fn get_progress_note(params: &ProgressNoteParams, intern_roles: &[String], pool: &Pool<MySql>, hosxp: &str, kphis: &str, kphis_extra: &str) -> Result<Vec<ProgressNote>, AppError> {
    let sql = progress_note::select_progress(params, intern_roles, hosxp, kphis, kphis_extra);
    let mut query = sqlx::query(AssertSqlSafe(sql));
    if let Some(progress_note_id) = params.progress_note_id.as_ref() {
        query = query.bind(progress_note_id);
    }
    if let Some(an) = params.an.as_ref() {
        query = query.bind(an);
    }
    if let Some(progress_note_date) = params.progress_note_date.as_ref() {
        query = query.bind(progress_note_date);
    }
    query
        .fetch_all(pool)
        .await
        .map_err(|e| Source::SQLx.to_error(500, e, "Select ProgressNote"))?
        .iter()
        .map(progress_note_from_row)
        .collect::<sqlx::Result<Vec<ProgressNote>>>()
        .map_err(|e| Source::SQLx.to_error(500, e, "Select ProgressNote"))
}
fn progress_note_from_row(row: &MySqlRow) -> sqlx::Result<ProgressNote> {
    let an: String = row.try_get("an")?;
    Ok(ProgressNote {
        visit_type: VisitTypeId::Ipd(an.clone()),
        progress_note_id: row.try_get("progress_note_id")?,
        progress_note_date: row.try_get("progress_note_date")?,
        progress_note_time: row.try_get("progress_note_time")?,
        progress_note_owner_type: row.try_get("progress_note_owner_type")?,
        progress_note_doctor: row.try_get("progress_note_doctor")?,
        progress_note_enter_datetime: row.try_get("progress_note_enter_datetime")?,
        pre_order_progress_note_id: row.try_get("pre_order_progress_note_id")?,
        pre_order_progress_note_date: row.try_get("pre_order_progress_note_date")?,
        pre_order_progress_note_time: row.try_get("pre_order_progress_note_time")?,
        order_doctor_name: row.try_get("order_doctor_name")?,
        order_doctor_is_intern: row.try_get("order_doctor_is_intern")?,
        doctor_licenseno: row.try_get("doctor_licenseno")?,
        entryposition: row.try_get("entryposition")?,
        imgs: row.try_get("imgs")?,
        create_datetime: row.try_get("create_datetime")?,
        progress_note_item_types: Vec::new(),
    })
}

pub async fn get_progress_note_only_bundle(an: &str, pool: &Pool<MySql>, kphis: &str) -> Result<Vec<ProgressNoteOnly>, AppError> {
    let mut notes = get_progress_note_only(an, pool, kphis).await?;
    for note in notes.iter_mut() {
        let items = get_progress_note_item_only(note.progress_note_id, pool, kphis).await?;
        note.progress_note_items = items;
    }

    Ok(notes)
}

async fn get_progress_note_only(an: &str, pool: &Pool<MySql>, kphis: &str) -> Result<Vec<ProgressNoteOnly>, AppError> {
    let sql = progress_note::select_progress_only(kphis);
    sqlx::query(AssertSqlSafe(sql))
        .bind(an)
        .fetch_all(pool)
        .await
        .map_err(|e| Source::SQLx.to_error(500, e, "Select ProgressNoteOnly"))?
        .iter()
        .map(ProgressNoteOnly::from_row)
        .collect::<sqlx::Result<Vec<ProgressNoteOnly>>>()
        .map_err(|e| Source::SQLx.to_error(500, e, "Select ProgressNoteOnly"))
}

pub async fn get_progress_note_types(ids: &[u32], pool: &Pool<MySql>, kphis: &str) -> Result<Vec<(u32, String)>, AppError> {
    if ids.is_empty() {
        Ok(Vec::new())
    } else {
        let sql = progress_note::select_progress_type(ids, kphis);
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

pub async fn get_progress_note_item(id: u32, item_type: &str, pool: &Pool<MySql>, kphis: &str) -> Result<Vec<ProgressNoteItem>, AppError> {
    let sql = progress_note::select_progress_item(id, item_type, kphis);
    query_all(&sql, pool, "Select ProgressNoteItem")
        .await?
        .iter()
        .map(progress_note_item_from_row)
        .collect::<sqlx::Result<Vec<ProgressNoteItem>>>()
        .map_err(|e| Source::SQLx.to_error(500, e, "Select ProgressNoteItem"))
}

async fn get_progress_note_item_only(progress_note_id: u32, pool: &Pool<MySql>, kphis: &str) -> Result<Vec<ProgressNoteItemOnly>, AppError> {
    let sql = progress_note::select_progress_item_only(kphis);
    sqlx::query(AssertSqlSafe(sql))
        .bind(progress_note_id)
        .fetch_all(pool)
        .await
        .map_err(|e| Source::SQLx.to_error(500, e, "Select ProgressNoteItemOnly"))?
        .iter()
        .map(ProgressNoteItemOnly::from_row)
        .collect::<sqlx::Result<Vec<ProgressNoteItemOnly>>>()
        .map_err(|e| Source::SQLx.to_error(500, e, "Select ProgressNoteItemOnly"))
}

// ipd-dr-order-progress-note-save.php
pub async fn post_progress_note(save: &ProgressNoteSave, user: &str, pool: &Pool<MySql>, kphis: &str) -> Result<(u32, Vec<ExecuteResponse>), AppError> {
    if save.progress_note_items.is_empty() {
        return Ok((0, Vec::new()));
    }

    match &save.visit_type {
        VisitTypeId::Ipd(an) | VisitTypeId::PreAdmit(an) => {
            let is_update = match save.progress_note_id {
                Some(id) => id > 0,
                None => false,
            };
            let progress_note_id;
            let mut results = Vec::with_capacity(2 + save.progress_note_items.len());
            if is_update {
                // safe to unwrap here because we checked at is_update above
                progress_note_id = save.progress_note_id.unwrap_or_default();

                let update_result = update_progress_note_an(an, save, user, pool, kphis).await?;
                results.push(ExecuteResponse::from_query_result(update_result, "Update ProgressNote"));

                let delete_item_result = delete_progress_note_item(progress_note_id, pool, kphis).await?;
                results.push(ExecuteResponse::from_query_result(delete_item_result, "Delete ProgressNoteItem"));
            } else {
                let insert_result = insert_progress_note(an, save, user, pool, kphis).await?;
                progress_note_id = insert_result.last_insert_id() as u32;
                results.push(ExecuteResponse::from_query_result(insert_result, "Insert ProgressNote"));
            }
            if progress_note_id > 0 {
                for progress_note_item in save.progress_note_items.iter() {
                    let insert_progress_note_item_result = insert_progress_note_item(progress_note_id, &str_some(an.to_owned()), progress_note_item, user, pool, kphis).await?;
                    results.push(ExecuteResponse::from_query_result(insert_progress_note_item_result, "Insert ProgressNoteItem"));
                }
            }

            Ok((progress_note_id, results))
        }
        VisitTypeId::OpdEr(_, _) | VisitTypeId::Visit(_) => Err(AppError::app_400("Post ProgressNoteSave")),
    }
}

async fn insert_progress_note(an: &str, save: &ProgressNoteSave, user: &str, pool: &Pool<MySql>, kphis: &str) -> Result<MySqlQueryResult, AppError> {
    let insert_sql = progress_note::insert_progress_note(save.progress_note_for_past_date.is_some(), save.progress_note_for_past_time.is_some(), kphis);
    let mut query = sqlx::query(AssertSqlSafe(insert_sql)).bind(an);
    if let Some(past_date) = save.progress_note_for_past_date {
        query = query.bind(past_date);
    }
    if let Some(past_time) = save.progress_note_for_past_time {
        query = query.bind(past_time);
    }
    query
        .bind(&save.progress_note_owner_type)
        .bind(&save.progress_note_doctor)
        .bind(user)
        .bind(user)
        .execute(pool)
        .await
        .map_err(|e| Source::SQLx.to_error(500, e, "Insert ProgressNote"))
}

pub async fn insert_progress_note_only(an: &str, only: &ProgressNoteOnly, pool: &Pool<MySql>, kphis: &str) -> Result<MySqlQueryResult, AppError> {
    let insert_sql = progress_note::insert_progress_note_only(kphis);
    sqlx::query(AssertSqlSafe(insert_sql))
        .bind(an)
        .bind(only.progress_note_date)
        .bind(only.progress_note_time)
        .bind(&only.progress_note_owner_type)
        .bind(&only.progress_note_doctor)
        .bind(only.progress_note_enter_datetime)
        .bind(only.pre_order_progress_note_id)
        .bind(only.pre_order_progress_note_date)
        .bind(only.pre_order_progress_note_time)
        .bind(&only.create_user)
        .bind(only.create_datetime)
        .bind(&only.update_user)
        .bind(only.update_datetime)
        .bind(only.version)
        .execute(pool)
        .await
        .map_err(|e| Source::SQLx.to_error(500, e, "Insert ProgressNoteOnly"))
}

async fn insert_progress_note_item(
    progress_note_id: u32,
    an: &Option<String>,
    progress_note_item: &ProgressNoteItemSave,
    user: &str,
    pool: &Pool<MySql>,
    kphis: &str,
) -> Result<MySqlQueryResult, AppError> {
    let insert_progress_note_item_sql = progress_note::insert_progress_note_item(kphis);
    sqlx::query(AssertSqlSafe(insert_progress_note_item_sql))
        .bind(progress_note_id)
        .bind(an)
        .bind(&progress_note_item.progress_note_item_type)
        .bind(&progress_note_item.progress_note_item_detail)
        .bind(&progress_note_item.progress_note_item_detail_2)
        .bind(user)
        .bind(user)
        .execute(pool)
        .await
        .map_err(|e| Source::SQLx.to_error(500, e, "Insert ProgressNoteItem"))
}

pub async fn insert_progress_note_items_only(progress_note_id: u32, an: &str, progress_note_items: &[ProgressNoteItemOnly], pool: &Pool<MySql>, kphis: &str) -> Result<MySqlQueryResult, AppError> {
    let insert_progress_note_item_sql = progress_note::insert_progress_note_items_only(progress_note_id, an, progress_note_items, kphis);
    sqlx::query(AssertSqlSafe(insert_progress_note_item_sql))
        .execute(pool)
        .await
        .map_err(|e| Source::SQLx.to_error(500, e, "Insert ProgressNoteItemOnly"))
}

async fn update_progress_note_an(an: &str, save: &ProgressNoteSave, user: &str, pool: &Pool<MySql>, kphis: &str) -> Result<MySqlQueryResult, AppError> {
    let update_an_sql = progress_note::update_progress_note_an(save.progress_note_for_past_time.is_some(), kphis);
    let mut query = sqlx::query(AssertSqlSafe(update_an_sql)).bind(an);
    if let Some(past_time) = save.progress_note_for_past_time {
        query = query.bind(past_time);
    }
    query
        .bind(user)
        .bind(save.progress_note_id)
        .execute(pool)
        .await
        .map_err(|e| Source::SQLx.to_error(500, e, "Update ProgressNoteSave"))
}

// ipd-dr-order-progress-note-delete.php
/// delete progress_note and progress_note_item
pub async fn delete_progress_note(progress_note_id: u32, pool: &Pool<MySql>, kphis: &str) -> Result<ExecuteResponse, AppError> {
    let sql = progress_note::delete_progress_note(kphis);
    let delete_result = sqlx::query(AssertSqlSafe(sql))
        .bind(progress_note_id)
        .execute(pool)
        .await
        .map_err(|e| Source::SQLx.to_error(500, e, "Delete ProgressNote"))?;

    Ok(ExecuteResponse::from_query_result(delete_result, "Delete ProgressNote"))
}

async fn delete_progress_note_item(progress_note_id: u32, pool: &Pool<MySql>, kphis: &str) -> Result<MySqlQueryResult, AppError> {
    let delete_item_sql = progress_note::delete_progress_note_item(kphis);
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
    use kphis_util::datetime::date_8601;
    use time::Time;

    #[tokio::test]
    #[ignore]
    async fn sqlx_get_previous_progress() {
        let tester = MySqlTester::new_kphis().await;
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis/ipd_progress_note.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis/ipd_progress_note_item.sql")).execute(&tester.db_pool).await.unwrap();

        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/kphis/ipd_progress_note.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/kphis/ipd_progress_note_item.sql")).execute(&tester.db_pool).await.unwrap();

        let found_doctor = get_previous_progress("660001234", "doctor", &tester.db_pool, &tester.kphis).await.unwrap();
        assert_eq!(found_doctor.len(), 3);
        let found_nurse = get_previous_progress("660001234", "nurse", &tester.db_pool, &tester.kphis).await.unwrap();
        assert_eq!(found_nurse.len(), 1);
        let found_pharm = get_previous_progress("660001234", "pharmacist", &tester.db_pool, &tester.kphis).await.unwrap();
        assert_eq!(found_pharm.len(), 1);
        let found_other = get_previous_progress("660001234", "other", &tester.db_pool, &tester.kphis).await.unwrap();
        assert_eq!(found_other.len(), 1);
        let not_found = get_previous_progress("660006666", "doctor", &tester.db_pool, &tester.kphis).await.unwrap();
        assert!(not_found.is_empty());
    }

    #[tokio::test]
    #[ignore]
    async fn sqlx_get_progress_note() {
        let tester = MySqlTester::new_hosxp_and_kphis_and_kphis_extra().await;
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis/ipd_progress_note.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis/system_ac_role.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis/system_acc_role_user.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/hosxp/doctor.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/hosxp/opduser.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis_extra/image.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis_extra/image_usage.sql")).execute(&tester.db_pool).await.unwrap();

        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/kphis/ipd_progress_note.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/kphis/system_ac_role.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/kphis/system_acc_role_user.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/hosxp/doctor.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/hosxp/opduser.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/kphis_extra/image.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/kphis_extra/image_usage.sql")).execute(&tester.db_pool).await.unwrap();

        let default = get_progress_note(&ProgressNoteParams::default(),&[String::from("DOCTOR_INTERN")],&tester.db_pool,&tester.hosxp,&tester.kphis,&tester.kphis_extra).await.unwrap();
        assert_eq!(default.len(), 5);
        let found_progress_note_id = get_progress_note(&ProgressNoteParams {progress_note_id: Some(1),..Default::default()},&[String::from("DOCTOR_INTERN")],&tester.db_pool,&tester.hosxp,&tester.kphis,&tester.kphis_extra).await.unwrap();
        assert_eq!(found_progress_note_id.len(), 1);
        let found_an = get_progress_note(&ProgressNoteParams {an: Some(String::from("660001234")),..Default::default()},&[String::from("DOCTOR_INTERN")],&tester.db_pool,&tester.hosxp,&tester.kphis,&tester.kphis_extra).await.unwrap();
        assert_eq!(found_an.len(), 4);
        let found_progress_note_date = get_progress_note(&ProgressNoteParams {progress_note_date: date_8601("2024-01-01"),..Default::default()},&[String::from("DOCTOR_INTERN")],&tester.db_pool,&tester.hosxp,&tester.kphis,&tester.kphis_extra).await.unwrap();
        assert_eq!(found_progress_note_date.len(), 4);
        let not_found = get_progress_note(&ProgressNoteParams {progress_note_id: Some(999),..Default::default()},&[String::from("DOCTOR_INTERN")],&tester.db_pool,&tester.hosxp,&tester.kphis,&tester.kphis_extra).await.unwrap();
        assert!(not_found.is_empty());
    }

    #[tokio::test]
    #[ignore]
    async fn sqlx_get_progress_note_only() {
        let tester = MySqlTester::new_kphis().await;
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis/ipd_progress_note.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/kphis/ipd_progress_note.sql")).execute(&tester.db_pool).await.unwrap();

        let found = get_progress_note_only("660001234",&tester.db_pool,&tester.kphis).await.unwrap();
        assert_eq!(found.len(), 4);
        let not_found = get_progress_note_only("660006666",&tester.db_pool,&tester.kphis).await.unwrap();
        assert!(not_found.is_empty());
    }

    #[tokio::test]
    #[ignore]
    async fn sqlx_get_progress_note_types() {
        let tester = MySqlTester::new_kphis().await;
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis/ipd_progress_note.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis/ipd_progress_note_item.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis/ipd_progress_note_item_type.sql")).execute(&tester.db_pool).await.unwrap();

        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/kphis/ipd_progress_note.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/kphis/ipd_progress_note_item.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/kphis/ipd_progress_note_item_type.sql")).execute(&tester.db_pool).await.unwrap();

        let found = get_progress_note_types(&[1], &tester.db_pool, &tester.kphis).await.unwrap();
        assert_eq!(found.len(), 3);
        let not_found = get_progress_note_types(&[], &tester.db_pool, &tester.kphis).await.unwrap();
        assert!(not_found.is_empty());
    }

    #[tokio::test]
    #[ignore]
    async fn sqlx_get_progress_note_item() {
        let tester = MySqlTester::new_kphis().await;
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis/ipd_progress_note.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis/ipd_progress_note_item.sql")).execute(&tester.db_pool).await.unwrap();

        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/kphis/ipd_progress_note.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/kphis/ipd_progress_note_item.sql")).execute(&tester.db_pool).await.unwrap();

        let found = get_progress_note_item(1, "problem-list", &tester.db_pool, &tester.kphis).await.unwrap();
        assert_eq!(found.len(), 1);
        let not_found = get_progress_note_item(999, "problem-list", &tester.db_pool, &tester.kphis).await.unwrap();
        assert!(not_found.is_empty());
    }

    #[tokio::test]
    #[ignore]
    async fn sqlx_get_progress_note_item_only() {
        let tester = MySqlTester::new_kphis().await;
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis/ipd_progress_note_item.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/kphis/ipd_progress_note_item.sql")).execute(&tester.db_pool).await.unwrap();

        let found = get_progress_note_item_only(1, &tester.db_pool, &tester.kphis).await.unwrap();
        assert_eq!(found.len(), 3);
        let not_found = get_progress_note_item_only(999, &tester.db_pool, &tester.kphis).await.unwrap();
        assert!(not_found.is_empty());
    }

    #[tokio::test]
    #[ignore]
    async fn sqlx_insert_progress_note() {
        let tester = MySqlTester::new_kphis().await;
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis/ipd_progress_note.sql")).execute(&tester.db_pool).await.unwrap();

        let mut save = ProgressNoteSave::demo();
        let success = insert_progress_note("660001234",&save,"user",&tester.db_pool,&tester.kphis).await.unwrap();
        assert_eq!(success.rows_affected(), 1);
        let again_success = insert_progress_note("660001234",&save,"user",&tester.db_pool,&tester.kphis).await.unwrap();
        assert_eq!(again_success.rows_affected(), 1);
        save.progress_note_for_past_time = None;
        let again_time_now = insert_progress_note("660001234",&save,"user",&tester.db_pool,&tester.kphis).await.unwrap();
        assert_eq!(again_time_now.rows_affected(), 1);
        save.progress_note_for_past_date = None;
        save.progress_note_for_past_time = Some(Time::MIDNIGHT);
        let again_date_now = insert_progress_note("660001234",&save,"user",&tester.db_pool,&tester.kphis).await.unwrap();
        assert_eq!(again_date_now.rows_affected(), 1);
        save.progress_note_for_past_time = None;
        let again_datetime_now = insert_progress_note("660001234",&save,"user",&tester.db_pool,&tester.kphis).await.unwrap();
        assert_eq!(again_datetime_now.rows_affected(), 1);
    }

    #[tokio::test]
    #[ignore]
    async fn sqlx_insert_progress_note_only() {
        let tester = MySqlTester::new_kphis().await;
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis/ipd_progress_note.sql")).execute(&tester.db_pool).await.unwrap();

        let success = insert_progress_note_only("660001234", &ProgressNoteOnly::demo(),&tester.db_pool,&tester.kphis).await.unwrap();
        assert_eq!(success.rows_affected(), 1);
        let again_success = insert_progress_note_only("660001234", &ProgressNoteOnly::demo(),&tester.db_pool,&tester.kphis).await.unwrap();
        assert_eq!(again_success.rows_affected(), 1);
    }

    #[tokio::test]
    #[ignore]
    async fn sqlx_insert_progress_note_item() {
        let tester = MySqlTester::new_kphis().await;
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis/ipd_progress_note_item.sql")).execute(&tester.db_pool).await.unwrap();

        let success = insert_progress_note_item(1,&Some(String::from("660001234")),&ProgressNoteItemSave::demo(),"user",&tester.db_pool,&tester.kphis).await.unwrap();
        assert_eq!(success.rows_affected(), 1);
        let again_success = insert_progress_note_item(1,&None,&ProgressNoteItemSave::demo(),"user",&tester.db_pool,&tester.kphis).await.unwrap();
        assert_eq!(again_success.rows_affected(), 1);
    }

    #[tokio::test]
    #[ignore]
    async fn sqlx_insert_progress_note_items_only() {
        let tester = MySqlTester::new_kphis().await;
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis/ipd_progress_note_item.sql")).execute(&tester.db_pool).await.unwrap();

        let success = insert_progress_note_items_only(1,"660001234",&[ProgressNoteItemOnly::demo()],&tester.db_pool,&tester.kphis).await.unwrap();
        assert_eq!(success.rows_affected(), 1);
        let again_success = insert_progress_note_items_only(1,"",&[ProgressNoteItemOnly::demo()],&tester.db_pool,&tester.kphis).await.unwrap();
        assert_eq!(again_success.rows_affected(), 1);
    }

    #[tokio::test]
    #[ignore]
    async fn sqlx_update_progress_note_an() {
        let tester = MySqlTester::new_kphis().await;
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis/ipd_progress_note.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/kphis/ipd_progress_note.sql")).execute(&tester.db_pool).await.unwrap();

        let mut save = ProgressNoteSave::demo();
        let success = update_progress_note_an("660001234",&save,"user",&tester.db_pool,&tester.kphis).await.unwrap();
        assert_eq!(success.rows_affected(), 1);
        let again_success = update_progress_note_an("660001234",&save,"user",&tester.db_pool,&tester.kphis).await.unwrap();
        assert_eq!(again_success.rows_affected(), 1);
        save.progress_note_for_past_time = None;
        let not_with_time_success = update_progress_note_an("660001234",&save,"user",&tester.db_pool,&tester.kphis).await.unwrap();
        assert_eq!(not_with_time_success.rows_affected(), 1);
    }

    #[tokio::test]
    #[ignore]
    async fn sqlx_delete_progress_note() {
        let tester = MySqlTester::new_kphis().await;
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis/ipd_progress_note.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis/ipd_progress_note_item.sql")).execute(&tester.db_pool).await.unwrap();

        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/kphis/ipd_progress_note.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/kphis/ipd_progress_note_item.sql")).execute(&tester.db_pool).await.unwrap();

        let success = delete_progress_note(1, &tester.db_pool, &tester.kphis).await.unwrap();
        assert_eq!(success.rows_affected, 4);
        let again_not_found = delete_progress_note(1, &tester.db_pool, &tester.kphis).await.unwrap();
        assert_eq!(again_not_found.rows_affected, 0);
    }

    #[tokio::test]
    #[ignore]
    async fn sqlx_delete_progress_note_item() {
        let tester = MySqlTester::new_kphis().await;
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis/ipd_progress_note_item.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/kphis/ipd_progress_note_item.sql")).execute(&tester.db_pool).await.unwrap();

        let success = delete_progress_note_item(1, &tester.db_pool, &tester.kphis).await.unwrap();
        assert_eq!(success.rows_affected(), 3);
        let again_not_found = delete_progress_note_item(1, &tester.db_pool, &tester.kphis).await.unwrap();
        assert_eq!(again_not_found.rows_affected(), 0);
    }
}
