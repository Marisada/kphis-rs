use sqlx::{AssertSqlSafe, FromRow, MySql, Pool, mysql::MySqlQueryResult};

use kphis_model::{
    fetch::ExecuteResponse,
    focus_note::{FocusNote, FocusNoteDlcItemOnly, FocusNoteIntvtItemOnly, FocusNoteOnly, FocusNoteParams, FocusNoteSave},
};
use kphis_sql::{
    data_history_utils::{KeyValue, SourceTable},
    ipd::focus_note,
};
use kphis_util::error::{AppError, Source};

use crate::log::insert_history_log;

pub async fn get_focus_note(an: &str, params: &FocusNoteParams, pool: &Pool<MySql>, hosxp: &str, kphis: &str, kphis_extra: &str) -> Result<Vec<FocusNote>, AppError> {
    let sql = focus_note::select_focus_note(params, hosxp, kphis, kphis_extra);
    let mut query = sqlx::query(AssertSqlSafe(sql)).bind(an);
    if let Some(start_date) = params.start_date {
        query = query.bind(start_date);
    }
    if let Some(end_date) = params.end_date {
        query = query.bind(end_date);
    }
    if let Some(fcnote_id) = &params.fcnote_id {
        query = query.bind(fcnote_id);
    }
    query
        .fetch_all(pool)
        .await
        .map_err(|e| Source::SQLx.to_error(500, e, "Select FocusNote"))?
        .iter()
        .map(FocusNote::from_row)
        .collect::<sqlx::Result<Vec<FocusNote>>>()
        .map_err(|e| Source::SQLx.to_error(500, e, "Select FocusNote"))
}

pub async fn get_focus_note_only_bundle(an: &str, pool: &Pool<MySql>, kphis: &str) -> Result<Vec<FocusNoteOnly>, AppError> {
    let mut focus_notes = get_focus_note_only(an, pool, kphis).await?;
    for focus_note in focus_notes.iter_mut() {
        let focus_note_intvt_items = get_focus_note_intvt_item_only(focus_note.fcnote_id, pool, kphis).await?;
        focus_note.focus_note_intvt_items = focus_note_intvt_items;
        let focus_note_dlc_items = get_focus_note_dlc_item_only(focus_note.fcnote_id, pool, kphis).await?;
        focus_note.focus_note_dlc_items = focus_note_dlc_items;
    }

    Ok(focus_notes)
}

async fn get_focus_note_only(an: &str, pool: &Pool<MySql>, kphis: &str) -> Result<Vec<FocusNoteOnly>, AppError> {
    let sql = focus_note::select_focus_note_only(kphis);
    sqlx::query(AssertSqlSafe(sql))
        .bind(an)
        .fetch_all(pool)
        .await
        .map_err(|e| Source::SQLx.to_error(500, e, "Select FocusNoteOnly"))?
        .iter()
        .map(FocusNoteOnly::from_row)
        .collect::<sqlx::Result<Vec<FocusNoteOnly>>>()
        .map_err(|e| Source::SQLx.to_error(500, e, "Select FocusNoteOnly"))
}

async fn get_focus_note_intvt_item_only(fcnote_id: u32, pool: &Pool<MySql>, kphis: &str) -> Result<Vec<FocusNoteIntvtItemOnly>, AppError> {
    let sql = focus_note::select_focus_note_intvt_item_only(kphis);
    sqlx::query(AssertSqlSafe(sql))
        .bind(fcnote_id)
        .fetch_all(pool)
        .await
        .map_err(|e| Source::SQLx.to_error(500, e, "Select FocusNoteIntvtItemOnly"))?
        .iter()
        .map(FocusNoteIntvtItemOnly::from_row)
        .collect::<sqlx::Result<Vec<FocusNoteIntvtItemOnly>>>()
        .map_err(|e| Source::SQLx.to_error(500, e, "Select FocusNoteIntvtItemOnly"))
}

async fn get_focus_note_dlc_item_only(fcnote_id: u32, pool: &Pool<MySql>, kphis: &str) -> Result<Vec<FocusNoteDlcItemOnly>, AppError> {
    let sql = focus_note::select_focus_note_dlc_item_only(kphis);
    sqlx::query(AssertSqlSafe(sql))
        .bind(fcnote_id)
        .fetch_all(pool)
        .await
        .map_err(|e| Source::SQLx.to_error(500, e, "Select FocusNoteDlcItemOnly"))?
        .iter()
        .map(FocusNoteDlcItemOnly::from_row)
        .collect::<sqlx::Result<Vec<FocusNoteDlcItemOnly>>>()
        .map_err(|e| Source::SQLx.to_error(500, e, "Select FocusNoteDlcItemOnly"))
}

// ipd-nurse-focus-note-save.php OR ipd-nurse-focus-note-update.php
#[allow(clippy::too_many_arguments)]
pub async fn post_focus_note(an: &str, hn: &str, ward: &str, save: &FocusNoteSave, user: &str, pool: &Pool<MySql>, kphis: &str, kphis_log: &str) -> Result<(u32, Vec<ExecuteResponse>), AppError> {
    if let Some(fcnote_id) = save.fcnote_id {
        let results = update_focus_note_bundle(fcnote_id, an, hn, save, user, pool, kphis, kphis_log).await?;

        Ok((fcnote_id, results))
    } else {
        insert_focus_note_bundle(an, hn, ward, save, user, pool, kphis, kphis_log).await
    }
}

#[allow(clippy::too_many_arguments)]
pub async fn update_focus_note_bundle(
    fcnote_id: u32,
    an: &str,
    hn: &str,
    save: &FocusNoteSave,
    user: &str,
    pool: &Pool<MySql>,
    kphis: &str,
    kphis_log: &str,
) -> Result<Vec<ExecuteResponse>, AppError> {
    let mut results = Vec::with_capacity(8);
    // 1. Update focus_note
    let update_focus_note_result = update_focus_note(fcnote_id, an, hn, save, user, pool, kphis).await?;
    let is_update = update_focus_note_result.rows_affected() > 0;
    results.push(ExecuteResponse::from_query_result(update_focus_note_result, "Update FocusNote"));
    // 2. Insert to history_ipd_focus_note
    if is_update {
        let insert_history_focus_note_result = insert_history_log(
            SourceTable::IpdFocusNote,
            "U",
            user,
            &[KeyValue("fcnote_id", fcnote_id.to_string()), KeyValue("version", (save.version + 1).to_string())],
            kphis,
            kphis_log,
            pool,
        )
        .await?;
        results.push(ExecuteResponse::from_query_result(insert_history_focus_note_result, "Insert FocusNote History"));
    }
    // 3. Delete used intvts
    let delete_intvt_item_result = delete_intvt_item(fcnote_id, save.version, pool, kphis).await?;
    results.push(ExecuteResponse::from_query_result(delete_intvt_item_result, "Delete FocusNoteIntvtItem"));
    if !save.intvt_ids.is_empty() {
        // 4. Insert intvts item
        let insert_intvt_item_result = insert_intvt_items(fcnote_id, &save.intvt_ids, save.version + 1, user, pool, kphis).await?;
        results.push(ExecuteResponse::from_query_result(insert_intvt_item_result, "Insert FocusNoteIntvtItem"));
        // 5. Insert history goal_item
        let insert_history_intvt_result = insert_history_log(SourceTable::IpdFocusNoteIntvtItem, "U", user, &[KeyValue("fcnote_id", fcnote_id.to_string())], kphis, kphis_log, pool).await?;
        results.push(ExecuteResponse::from_query_result(insert_history_intvt_result, "Insert FocusNoteIntvtItem History"));
    }
    // 6. Delete used dlcs
    let delete_dlc_item_result = delete_dlc_item(fcnote_id, save.version, pool, kphis).await?;
    results.push(ExecuteResponse::from_query_result(delete_dlc_item_result, "Delete FocusNoteDlcItem"));

    if !save.dlc_ids.is_empty() {
        // 7. Insert dlcs item
        let insert_dlc_item_result = insert_dlc_items(fcnote_id, &save.dlc_ids, save.version + 1, user, pool, kphis).await?;
        results.push(ExecuteResponse::from_query_result(insert_dlc_item_result, "Insert FocusNoteDlcItem"));
        // 8. Insert history dlc_item
        let insert_history_dlc_result = insert_history_log(SourceTable::IpdFocusNoteDlcItem, "U", user, &[KeyValue("fcnote_id", fcnote_id.to_string())], kphis, kphis_log, pool).await?;
        results.push(ExecuteResponse::from_query_result(insert_history_dlc_result, "Insert FocusNoteDlcItem History"));
    }

    Ok(results)
}

#[allow(clippy::too_many_arguments)]
pub async fn insert_focus_note_bundle(
    an: &str,
    hn: &str,
    ward: &str,
    save: &FocusNoteSave,
    user: &str,
    pool: &Pool<MySql>,
    kphis: &str,
    kphis_log: &str,
) -> Result<(u32, Vec<ExecuteResponse>), AppError> {
    let mut results = Vec::with_capacity(6);
    // 1. Insert focus_note
    let insert_focus_note_result = insert_focus_note(an, hn, ward, save, user, pool, kphis).await?;
    let fcnote_id = insert_focus_note_result.last_insert_id() as u32;
    results.push(ExecuteResponse::from_query_result(insert_focus_note_result, "Insert FocusNote"));
    // 2. Insert to history_ipd_focus_note
    let insert_history_focus_note_result = insert_history_log(SourceTable::IpdFocusNote, "I", user, &[KeyValue("fcnote_id", fcnote_id.to_string())], kphis, kphis_log, pool).await?;
    results.push(ExecuteResponse::from_query_result(insert_history_focus_note_result, "Insert FocusNote History"));

    if !save.intvt_ids.is_empty() {
        // 3. Insert intvts item
        let insert_intvt_item_result = insert_intvt_items(fcnote_id, &save.intvt_ids, 1, user, pool, kphis).await?;
        results.push(ExecuteResponse::from_query_result(insert_intvt_item_result, "Insert FocusNoteIntvtItem"));
        // 4. Insert history intvt_item
        let insert_history_intvt_result = insert_history_log(SourceTable::IpdFocusNoteIntvtItem, "I", user, &[KeyValue("fcnote_id", fcnote_id.to_string())], kphis, kphis_log, pool).await?;
        results.push(ExecuteResponse::from_query_result(insert_history_intvt_result, "Insert FocusNoteIntvtItem History"));
    }
    if !save.dlc_ids.is_empty() {
        // 5. Insert dlcs item
        let insert_dlc_item_result = insert_dlc_items(fcnote_id, &save.dlc_ids, 1, user, pool, kphis).await?;
        results.push(ExecuteResponse::from_query_result(insert_dlc_item_result, "Insert FocusNoteDlcItem"));
        // 6. Insert history dlc_item
        let insert_history_dlc_result = insert_history_log(SourceTable::IpdFocusNoteDlcItem, "I", user, &[KeyValue("fcnote_id", fcnote_id.to_string())], kphis, kphis_log, pool).await?;
        results.push(ExecuteResponse::from_query_result(insert_history_dlc_result, "Insert FocusNoteDlcItem History"));
    }

    Ok((fcnote_id, results))
}

pub async fn insert_focus_note_only_bundle(an: &str, hn: &str, ward: &str, only: &FocusNoteOnly, pool: &Pool<MySql>, kphis: &str, kphis_log: &str) -> Result<(u32, Vec<ExecuteResponse>), AppError> {
    let mut results = Vec::with_capacity(6);
    // 1. Insert focus_note
    let insert_focus_note_result = insert_focus_note_only(an, hn, ward, only, pool, kphis).await?;
    let fcnote_id = insert_focus_note_result.last_insert_id() as u32;
    results.push(ExecuteResponse::from_query_result(insert_focus_note_result, "Insert FocusNoteOnly"));
    // 2. Insert to history_ipd_focus_note
    let insert_history_focus_note_result = insert_history_log(SourceTable::IpdFocusNote, "I", "system", &[KeyValue("fcnote_id", fcnote_id.to_string())], kphis, kphis_log, pool).await?;
    results.push(ExecuteResponse::from_query_result(insert_history_focus_note_result, "Insert FocusNoteOnly History"));

    if !only.focus_note_intvt_items.is_empty() {
        // 3. Insert intvts item
        let insert_intvt_item_result = insert_intvt_items_only(fcnote_id, &only.focus_note_intvt_items, pool, kphis).await?;
        results.push(ExecuteResponse::from_query_result(insert_intvt_item_result, "Insert FocusNoteIntvtItemOnly"));
        // 4. Insert history intvt_item
        let insert_history_intvt_result = insert_history_log(
            SourceTable::IpdFocusNoteIntvtItem,
            "I",
            "system",
            &[KeyValue("fcnote_id", fcnote_id.to_string())],
            kphis,
            kphis_log,
            pool,
        )
        .await?;
        results.push(ExecuteResponse::from_query_result(insert_history_intvt_result, "Insert FocusNoteIntvtItemOnly History"));
    }
    if !only.focus_note_dlc_items.is_empty() {
        // 5. Insert dlcs item
        let insert_dlc_item_result = insert_dlc_items_only(fcnote_id, &only.focus_note_dlc_items, pool, kphis).await?;
        results.push(ExecuteResponse::from_query_result(insert_dlc_item_result, "Insert FocusNoteDlcItemOnly"));
        // 6. Insert history dlc_item
        let insert_history_dlc_result = insert_history_log(SourceTable::IpdFocusNoteDlcItem, "I", "system", &[KeyValue("fcnote_id", fcnote_id.to_string())], kphis, kphis_log, pool).await?;
        results.push(ExecuteResponse::from_query_result(insert_history_dlc_result, "Insert FocusNoteDlcItemOnly History"));
    }

    Ok((fcnote_id, results))
}

async fn update_focus_note(fcnote_id: u32, an: &str, hn: &str, save: &FocusNoteSave, user: &str, pool: &Pool<MySql>, kphis: &str) -> Result<MySqlQueryResult, AppError> {
    let update_focus_note_sql = focus_note::update_focus_note(kphis);
    sqlx::query(AssertSqlSafe(update_focus_note_sql))
        .bind(&save.general_symptoms)
        .bind(save.fclist_id)
        .bind(&save.assessment)
        .bind(&save.intvt_text)
        .bind(&save.evalution)
        .bind(&save.dlc_text)
        .bind(&save.other)
        .bind(an)
        .bind(hn)
        .bind(save.fcnote_date)
        .bind(save.fcnote_time)
        .bind(&save.fcnote_patient_type)
        .bind(user)
        .bind(fcnote_id)
        .bind(save.version)
        .execute(pool)
        .await
        .map_err(|e| Source::SQLx.to_error(500, e, "Update FocusNote"))
}

async fn insert_focus_note(an: &str, hn: &str, ward: &str, save: &FocusNoteSave, user: &str, pool: &Pool<MySql>, kphis: &str) -> Result<MySqlQueryResult, AppError> {
    let insert_focus_note_sql = focus_note::insert_focus_note(kphis);
    sqlx::query(AssertSqlSafe(insert_focus_note_sql))
        .bind(&save.general_symptoms)
        .bind(save.fclist_id)
        .bind(&save.assessment)
        .bind(&save.intvt_text)
        .bind(&save.evalution)
        .bind(&save.dlc_text)
        .bind(&save.other)
        .bind(an)
        .bind(hn)
        .bind(save.fcnote_date)
        .bind(save.fcnote_time)
        .bind(&save.fcnote_patient_type)
        .bind(ward)
        .bind(user)
        .bind(user)
        .execute(pool)
        .await
        .map_err(|e| Source::SQLx.to_error(500, e, "Insert FocusNote"))
}

async fn insert_focus_note_only(an: &str, hn: &str, ward: &str, only: &FocusNoteOnly, pool: &Pool<MySql>, kphis: &str) -> Result<MySqlQueryResult, AppError> {
    let insert_focus_note_sql = focus_note::insert_focus_note_only(kphis);
    sqlx::query(AssertSqlSafe(insert_focus_note_sql))
        .bind(&only.general_symptoms)
        .bind(only.fclist_id)
        .bind(&only.assessment)
        .bind(&only.intvt_text)
        .bind(&only.evalution)
        .bind(&only.dlc_text)
        .bind(&only.other)
        .bind(an)
        .bind(hn)
        .bind(only.fcnote_date)
        .bind(only.fcnote_time)
        .bind(&only.fcnote_patient_type)
        .bind(ward)
        .bind(&only.create_user)
        .bind(only.create_datetime)
        .bind(&only.update_user)
        .bind(only.update_datetime)
        .bind(only.version)
        .execute(pool)
        .await
        .map_err(|e| Source::SQLx.to_error(500, e, "Insert FocusNoteOnly"))
}

async fn insert_intvt_items(fcnote_id: u32, intvt_ids: &[u32], version: i32, user: &str, pool: &Pool<MySql>, kphis: &str) -> Result<MySqlQueryResult, AppError> {
    let insert_intvt_item_sql = focus_note::insert_intvt_items(intvt_ids, fcnote_id, user, version, kphis);
    sqlx::query(AssertSqlSafe(insert_intvt_item_sql)).execute(pool).await.map_err(|e| {
        if let sqlx::error::Error::Database(err) = &e
            && err.code().map(|c| c == "23000").unwrap_or_default()
        {
            AppError::app_403_duplicate("Insert InterventionItem")
        } else {
            Source::SQLx.to_error(500, e, "Insert InterventionItem")
        }
    })
}

async fn insert_intvt_items_only(fcnote_id: u32, intvts: &[FocusNoteIntvtItemOnly], pool: &Pool<MySql>, kphis: &str) -> Result<MySqlQueryResult, AppError> {
    let insert_intvt_item_sql = focus_note::insert_intvt_items_only(intvts, fcnote_id, kphis);
    sqlx::query(AssertSqlSafe(insert_intvt_item_sql)).execute(pool).await.map_err(|e| {
        if let sqlx::error::Error::Database(err) = &e
            && err.code().map(|c| c == "23000").unwrap_or_default()
        {
            AppError::app_403_duplicate("Insert InterventionItemOnly")
        } else {
            Source::SQLx.to_error(500, e, "Insert InterventionItemOnly")
        }
    })
}

async fn insert_dlc_items(fcnote_id: u32, dlc_ids: &[u32], version: i32, user: &str, pool: &Pool<MySql>, kphis: &str) -> Result<MySqlQueryResult, AppError> {
    let insert_dlc_item_sql = focus_note::insert_dlc_items(dlc_ids, fcnote_id, user, version, kphis);
    sqlx::query(AssertSqlSafe(insert_dlc_item_sql)).execute(pool).await.map_err(|e| {
        if let sqlx::error::Error::Database(err) = &e
            && err.code().map(|c| c == "23000").unwrap_or_default()
        {
            AppError::app_403_duplicate("Insert DailyCareItem")
        } else {
            Source::SQLx.to_error(500, e, "Insert DailyCareItem")
        }
    })
}

async fn insert_dlc_items_only(fcnote_id: u32, dlcs: &[FocusNoteDlcItemOnly], pool: &Pool<MySql>, kphis: &str) -> Result<MySqlQueryResult, AppError> {
    let insert_dlc_item_sql = focus_note::insert_dlc_items_only(dlcs, fcnote_id, kphis);
    sqlx::query(AssertSqlSafe(insert_dlc_item_sql)).execute(pool).await.map_err(|e| {
        if let sqlx::error::Error::Database(err) = &e
            && err.code().map(|c| c == "23000").unwrap_or_default()
        {
            AppError::app_403_duplicate("Insert FocusNoteDlcItemOnly")
        } else {
            Source::SQLx.to_error(500, e, "Insert FocusNoteDlcItemOnly")
        }
    })
}

// ipd-nurse-focus-note-delete.php
pub async fn delete_focus_note(fcnote_id: u32, version: i32, user: &str, pool: &Pool<MySql>, kphis: &str, kphis_log: &str) -> Result<Vec<ExecuteResponse>, AppError> {
    let mut results = Vec::with_capacity(6);

    // 1. Insert to history_ipd_focus_note
    let insert_history_focus_note_result = insert_history_log(
        SourceTable::IpdFocusNote,
        "D",
        user,
        &[KeyValue("fcnote_id", fcnote_id.to_string()), KeyValue("version", version.to_string())],
        kphis,
        kphis_log,
        pool,
    )
    .await?;
    let will_delete = insert_history_focus_note_result.rows_affected() > 0;
    results.push(ExecuteResponse::from_query_result(insert_history_focus_note_result, "Insert FocusNote History"));
    if will_delete {
        // 2. Delete focus note
        let delete_focus_note_result = delete_focus_note_inner(fcnote_id, version, pool, kphis).await?;
        results.push(ExecuteResponse::from_query_result(delete_focus_note_result, "Delete FocusNote"));

        // 3. Insert history intvt_item
        let insert_history_intvt_result = insert_history_log(
            SourceTable::IpdFocusNoteIntvtItem,
            "D",
            user,
            &[KeyValue("fcnote_id", fcnote_id.to_string()), KeyValue("version", version.to_string())],
            kphis,
            kphis_log,
            pool,
        )
        .await?;
        let will_delete_intvt = insert_history_intvt_result.rows_affected() > 0;
        results.push(ExecuteResponse::from_query_result(insert_history_intvt_result, "Insert InterventionItem History"));
        if will_delete_intvt {
            // 4. Delete intvts
            let delete_intvt_item_result = delete_intvt_item(fcnote_id, version, pool, kphis).await?;
            results.push(ExecuteResponse::from_query_result(delete_intvt_item_result, "Delete InterventionItem"));
        }

        // 5. Insert history dlc_item
        let insert_history_dlc_result = insert_history_log(
            SourceTable::IpdFocusNoteDlcItem,
            "D",
            user,
            &[KeyValue("fcnote_id", fcnote_id.to_string()), KeyValue("version", version.to_string())],
            kphis,
            kphis_log,
            pool,
        )
        .await?;
        let will_delete_dlc = insert_history_dlc_result.rows_affected() > 0;
        results.push(ExecuteResponse::from_query_result(insert_history_dlc_result, "Insert DailyCareItem History"));
        if will_delete_dlc {
            // 6. Delete dlcs
            let delete_dlc_item_result = delete_dlc_item(fcnote_id, version, pool, kphis).await?;
            results.push(ExecuteResponse::from_query_result(delete_dlc_item_result, "Delete DailyCareItem"));
        }
    }

    Ok(results)
}

async fn delete_focus_note_inner(fcnote_id: u32, version: i32, pool: &Pool<MySql>, kphis: &str) -> Result<MySqlQueryResult, AppError> {
    let delete_focus_note_sql = focus_note::delete_focus_note(kphis);
    sqlx::query(AssertSqlSafe(delete_focus_note_sql))
        .bind(fcnote_id)
        .bind(version)
        .execute(pool)
        .await
        .map_err(|e| Source::SQLx.to_error(500, e, "Delete FocusNote"))
}

async fn delete_intvt_item(fcnote_id: u32, version: i32, pool: &Pool<MySql>, kphis: &str) -> Result<MySqlQueryResult, AppError> {
    let delete_intvt_item_sql = focus_note::delete_intvt_item(kphis);
    sqlx::query(AssertSqlSafe(delete_intvt_item_sql))
        .bind(fcnote_id)
        .bind(version)
        .execute(pool)
        .await
        .map_err(|e| Source::SQLx.to_error(500, e, "Delete InterventionItem"))
}

async fn delete_dlc_item(fcnote_id: u32, version: i32, pool: &Pool<MySql>, kphis: &str) -> Result<MySqlQueryResult, AppError> {
    let delete_dlc_item_sql = focus_note::delete_dlc_item(kphis);
    sqlx::query(AssertSqlSafe(delete_dlc_item_sql))
        .bind(fcnote_id)
        .bind(version)
        .execute(pool)
        .await
        .map_err(|e| Source::SQLx.to_error(500, e, "Delete DailyCareItem"))
}

#[cfg(test)]
#[rustfmt::skip]
mod tests {

    use super::*;
    use kphis_sqlx_tester::MySqlTester;
    use kphis_util::datetime::date_8601;

    #[tokio::test]
    #[ignore]
    async fn sqlx_get_focus_note() {
        let tester = MySqlTester::new_hosxp_and_kphis_and_kphis_extra().await;
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis/ipd_focus_list.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis/ipd_focus_note.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis/ipd_tmp_focus.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis/ipd_tmp_intvt.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis/ipd_tmp_dlc.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis/ipd_focus_note_intvt_item.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis/ipd_focus_note_dlc_item.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/hosxp/opduser.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/hosxp/doctor.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis_extra/image.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis_extra/image_usage.sql")).execute(&tester.db_pool).await.unwrap();

        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/kphis/ipd_focus_list.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/kphis/ipd_focus_note.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/kphis/ipd_tmp_focus.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/kphis/ipd_tmp_intvt.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/kphis/ipd_tmp_dlc.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/kphis/ipd_focus_note_intvt_item.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/kphis/ipd_focus_note_dlc_item.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/hosxp/opduser.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/hosxp/doctor.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/kphis_extra/image.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/kphis_extra/image_usage.sql")).execute(&tester.db_pool).await.unwrap();

        let default = get_focus_note("660001234",&FocusNoteParams::default(),&tester.db_pool,&tester.hosxp,&tester.kphis,&tester.kphis_extra).await.unwrap();
        assert_eq!(default.len(), 3);
        let found_start_end = get_focus_note("660001234",&FocusNoteParams {start_date: date_8601("2024-01-01"),end_date: date_8601("2024-02-01"),..Default::default()},&tester.db_pool,&tester.hosxp,&tester.kphis,&tester.kphis_extra).await.unwrap();
        assert_eq!(found_start_end.len(), 2);
        let found_start_alone = get_focus_note("660001234",&FocusNoteParams {start_date: date_8601("2024-01-01"),..Default::default()},&tester.db_pool,&tester.hosxp,&tester.kphis,&tester.kphis_extra).await.unwrap();
        assert_eq!(found_start_alone.len(), 1);
        // will treat end_date as start_date
        let found_end_alone = get_focus_note("660001234",&FocusNoteParams {end_date: date_8601("2024-01-01"),..Default::default()},&tester.db_pool,&tester.hosxp,&tester.kphis,&tester.kphis_extra).await.unwrap();
        assert_eq!(found_end_alone.len(), 1);
        let found_list_id = get_focus_note("660001234",&FocusNoteParams {fclist_id: Some(1),..Default::default()},&tester.db_pool,&tester.hosxp,&tester.kphis,&tester.kphis_extra).await.unwrap();
        assert_eq!(found_list_id.len(), 2);
        let found_null_list_id = get_focus_note("660001234",&FocusNoteParams {fclist_id: Some(0),..Default::default()},&tester.db_pool,&tester.hosxp,&tester.kphis,&tester.kphis_extra).await.unwrap();
        assert_eq!(found_null_list_id.len(), 1);
        let found_note_id = get_focus_note("660001234",&FocusNoteParams {fcnote_id: Some(1),..Default::default()},&tester.db_pool,&tester.hosxp,&tester.kphis,&tester.kphis_extra).await.unwrap();
        assert_eq!(found_note_id.len(), 1);
        let not_found = get_focus_note("660006666",&FocusNoteParams {fcnote_id: Some(1),..Default::default()},&tester.db_pool,&tester.hosxp,&tester.kphis,&tester.kphis_extra).await.unwrap();
        assert!(not_found.is_empty());
    }

    #[tokio::test]
    #[ignore]
    async fn sqlx_get_focus_note_only() {
        let tester = MySqlTester::new_kphis().await;
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis/ipd_focus_note.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/kphis/ipd_focus_note.sql")).execute(&tester.db_pool).await.unwrap();

        let found = get_focus_note_only("660001234",&tester.db_pool,&tester.kphis).await.unwrap();
        assert_eq!(found.len(), 3);
        let not_found = get_focus_note_only("660006666",&tester.db_pool,&tester.kphis).await.unwrap();
        assert!(not_found.is_empty());
    }

    #[tokio::test]
    #[ignore]
    async fn sqlx_get_focus_note_intvt_item_only() {
        let tester = MySqlTester::new_kphis().await;
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis/ipd_focus_note_intvt_item.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/kphis/ipd_focus_note_intvt_item.sql")).execute(&tester.db_pool).await.unwrap();

        let found = get_focus_note_intvt_item_only(1, &tester.db_pool,&tester.kphis).await.unwrap();
        assert_eq!(found.len(), 1);
        let not_found = get_focus_note_intvt_item_only(999,&tester.db_pool,&tester.kphis).await.unwrap();
        assert!(not_found.is_empty());
    }

    #[tokio::test]
    #[ignore]
    async fn sqlx_get_focus_note_dlc_item_only() {
        let tester = MySqlTester::new_kphis().await;
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis/ipd_focus_note_dlc_item.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/kphis/ipd_focus_note_dlc_item.sql")).execute(&tester.db_pool).await.unwrap();

        let found = get_focus_note_dlc_item_only(1, &tester.db_pool,&tester.kphis).await.unwrap();
        assert_eq!(found.len(), 1);
        let not_found = get_focus_note_dlc_item_only(999,&tester.db_pool,&tester.kphis).await.unwrap();
        assert!(not_found.is_empty());
    }

    #[tokio::test]
    #[ignore]
    async fn sqlx_insert_focus_note() {
        let tester = MySqlTester::new_kphis().await;
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis/ipd_focus_note.sql")).execute(&tester.db_pool).await.unwrap();

        let success = insert_focus_note("660001234","0001234","01",&FocusNoteSave::demo(),"user",&tester.db_pool,&tester.kphis).await.unwrap();
        assert_eq!(success.rows_affected(), 1);
        let again_success = insert_focus_note("660001234","0001234","01",&FocusNoteSave::demo(),"user",&tester.db_pool,&tester.kphis).await.unwrap();
        assert_eq!(again_success.rows_affected(), 1);
    }

    #[tokio::test]
    #[ignore]
    async fn sqlx_insert_focus_note_only() {
        let tester = MySqlTester::new_kphis().await;
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis/ipd_focus_note.sql")).execute(&tester.db_pool).await.unwrap();

        let success = insert_focus_note_only("660001234","0001234","01",&FocusNoteOnly::demo(),&tester.db_pool,&tester.kphis).await.unwrap();
        assert_eq!(success.rows_affected(), 1);
        let again_success = insert_focus_note_only("660001234","0001234","01",&FocusNoteOnly::demo(),&tester.db_pool,&tester.kphis).await.unwrap();
        assert_eq!(again_success.rows_affected(), 1);
    }

    #[tokio::test]
    #[ignore]
    async fn sqlx_insert_intvt_items() {
        let tester = MySqlTester::new_kphis().await;
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis/ipd_focus_note_intvt_item.sql")).execute(&tester.db_pool).await.unwrap();

        let success = insert_intvt_items(1, &[1, 9999], 1, "user", &tester.db_pool, &tester.kphis).await.unwrap();
        assert_eq!(success.rows_affected(), 2);
        // duplicate (fcnote_id + intvt_id) will error
        let again_duplicate = insert_intvt_items(1, &[1, 9999], 1, "user", &tester.db_pool, &tester.kphis).await;
        if let Err(again_duplicate_error) = again_duplicate {
            assert_eq!(again_duplicate_error.status, 403);
        } else {
            panic!("MUST ERROR")
        }
    }

    #[tokio::test]
    #[ignore]
    async fn sqlx_insert_intvt_items_only() {
        let tester = MySqlTester::new_kphis().await;
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis/ipd_focus_note_intvt_item.sql")).execute(&tester.db_pool).await.unwrap();

        let success = insert_intvt_items_only(1, &[FocusNoteIntvtItemOnly::demo()], &tester.db_pool, &tester.kphis).await.unwrap();
        assert_eq!(success.rows_affected(), 1);
        // duplicate (fcnote_id + intvt_id) will error
        let again_duplicate = insert_intvt_items_only(1, &[FocusNoteIntvtItemOnly::demo()], &tester.db_pool, &tester.kphis).await;
        if let Err(again_duplicate_error) = again_duplicate {
            assert_eq!(again_duplicate_error.status, 403);
        } else {
            panic!("MUST ERROR")
        }
    }

    #[tokio::test]
    #[ignore]
    async fn sqlx_insert_dlc_items() {
        let tester = MySqlTester::new_kphis().await;
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis/ipd_focus_note_dlc_item.sql")).execute(&tester.db_pool).await.unwrap();

        let success = insert_dlc_items(1, &[1, 2], 1, "user", &tester.db_pool, &tester.kphis).await.unwrap();
        assert_eq!(success.rows_affected(), 2);
        // duplicate (fcnote_id + dlc_id) will error
        let again_duplicate = insert_dlc_items(1, &[1, 2], 1, "user", &tester.db_pool, &tester.kphis).await;
        if let Err(again_duplicate_error) = again_duplicate {
            assert_eq!(again_duplicate_error.status, 403);
        } else {
            panic!("MUST ERROR")
        }
    }

    #[tokio::test]
    #[ignore]
    async fn sqlx_insert_dlc_items_only() {
        let tester = MySqlTester::new_kphis().await;
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis/ipd_focus_note_dlc_item.sql")).execute(&tester.db_pool).await.unwrap();

        let success = insert_dlc_items_only(1, &[FocusNoteDlcItemOnly::demo()], &tester.db_pool, &tester.kphis).await.unwrap();
        assert_eq!(success.rows_affected(), 1);
        // duplicate (fcnote_id + dlc_id) will error
        let again_duplicate = insert_dlc_items_only(1, &[FocusNoteDlcItemOnly::demo()], &tester.db_pool, &tester.kphis).await;
        if let Err(again_duplicate_error) = again_duplicate {
            assert_eq!(again_duplicate_error.status, 403);
        } else {
            panic!("MUST ERROR")
        }
    }

    #[tokio::test]
    #[ignore]
    async fn sqlx_update_focus_note() {
        let tester = MySqlTester::new_kphis().await;
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis/ipd_focus_note.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/kphis/ipd_focus_note.sql")).execute(&tester.db_pool).await.unwrap();

        let success = update_focus_note(1,"660001234","0001234",&FocusNoteSave::demo(),"user",&tester.db_pool,&tester.kphis).await.unwrap();
        assert_eq!(success.rows_affected(), 1);
        let again_fail_version = update_focus_note(1,"660001234","0001234",&FocusNoteSave::demo(),"user",&tester.db_pool,&tester.kphis).await.unwrap();
        assert_eq!(again_fail_version.rows_affected(), 0);
    }

    #[tokio::test]
    #[ignore]
    async fn sqlx_delete_focus_note_inner() {
        let tester = MySqlTester::new_kphis().await;
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis/ipd_focus_note.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/kphis/ipd_focus_note.sql")).execute(&tester.db_pool).await.unwrap();

        let success = delete_focus_note_inner(1, 1, &tester.db_pool, &tester.kphis).await.unwrap();
        assert_eq!(success.rows_affected(), 1);
        let again_not_found = delete_focus_note_inner(1, 1, &tester.db_pool, &tester.kphis).await.unwrap();
        assert_eq!(again_not_found.rows_affected(), 0);
    }

    #[tokio::test]
    #[ignore]
    async fn sqlx_delete_intvt_item() {
        let tester = MySqlTester::new_kphis().await;
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis/ipd_focus_note_intvt_item.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/kphis/ipd_focus_note_intvt_item.sql")).execute(&tester.db_pool).await.unwrap();

        let success = delete_intvt_item(1, 1, &tester.db_pool, &tester.kphis).await.unwrap();
        assert_eq!(success.rows_affected(), 1);
        let again_not_found = delete_intvt_item(1, 1, &tester.db_pool, &tester.kphis).await.unwrap();
        assert_eq!(again_not_found.rows_affected(), 0);
    }

    #[tokio::test]
    #[ignore]
    async fn sqlx_delete_dlc_item() {
        let tester = MySqlTester::new_kphis().await;
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis/ipd_focus_note_dlc_item.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/kphis/ipd_focus_note_dlc_item.sql")).execute(&tester.db_pool).await.unwrap();

        let success = delete_dlc_item(1, 1, &tester.db_pool, &tester.kphis).await.unwrap();
        assert_eq!(success.rows_affected(), 1);
        let again_not_found = delete_dlc_item(1, 1, &tester.db_pool, &tester.kphis).await.unwrap();
        assert_eq!(again_not_found.rows_affected(), 0);
    }
}
