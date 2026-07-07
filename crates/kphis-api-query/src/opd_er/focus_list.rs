use sqlx::{AssertSqlSafe, FromRow, MySql, Pool, mysql::MySqlQueryResult};

use kphis_model::{
    fetch::ExecuteResponse,
    focus_list::{FocusList, FocusListGoalItemOnly, FocusListOnly, FocusListParams, FocusListSave},
};
use kphis_sql::{
    data_history_utils::{KeyValue, SourceTable},
    opd_er::focus_list,
};
use kphis_util::error::{AppError, Source};

use crate::{app::get_exists, log::insert_history_log};

pub async fn get_focus_list(opd_er_order_master_id: u32, params: &FocusListParams, pool: &Pool<MySql>, hosxp: &str, kphis: &str) -> Result<Vec<FocusList>, AppError> {
    let sql = focus_list::select_focus_list(params, hosxp, kphis);
    let mut query = sqlx::query(AssertSqlSafe(sql)).bind(opd_er_order_master_id);
    if let Some(start_date) = params.start_date {
        query = query.bind(start_date);
    }
    if let Some(end_date) = params.end_date {
        query = query.bind(end_date);
    }
    if let Some(status) = &params.status {
        query = query.bind(status);
    }
    if let Some(fclist_id) = &params.fclist_id {
        query = query.bind(fclist_id);
    }
    query
        .fetch_all(pool)
        .await
        .map_err(|e| Source::SQLx.to_error(500, e, "Select FocusList"))?
        .iter()
        .map(FocusList::from_row)
        .collect::<sqlx::Result<Vec<FocusList>>>()
        .map_err(|e| Source::SQLx.to_error(500, e, "Select FocusList"))
}

pub async fn get_focus_list_only_bundle(opd_er_order_master_id: u32, pool: &Pool<MySql>, kphis: &str) -> Result<Vec<FocusListOnly>, AppError> {
    let mut focus_lists = get_focus_list_only(opd_er_order_master_id, pool, kphis).await?;
    for focus_list in focus_lists.iter_mut() {
        let focus_list_goal_items = get_focus_list_goal_item_only(focus_list.fclist_id, pool, kphis).await?;
        focus_list.focus_list_goal_items = focus_list_goal_items;
    }

    Ok(focus_lists)
}

async fn get_focus_list_only(opd_er_order_master_id: u32, pool: &Pool<MySql>, kphis: &str) -> Result<Vec<FocusListOnly>, AppError> {
    let sql = focus_list::select_focus_list_only(kphis);
    sqlx::query(AssertSqlSafe(sql))
        .bind(opd_er_order_master_id)
        .fetch_all(pool)
        .await
        .map_err(|e| Source::SQLx.to_error(500, e, "Select FocusListOnly"))?
        .iter()
        .map(FocusListOnly::from_row)
        .collect::<sqlx::Result<Vec<FocusListOnly>>>()
        .map_err(|e| Source::SQLx.to_error(500, e, "Select FocusListOnly"))
}

async fn get_focus_list_goal_item_only(fclist_id: u32, pool: &Pool<MySql>, kphis: &str) -> Result<Vec<FocusListGoalItemOnly>, AppError> {
    let sql = focus_list::select_focus_list_goal_item_only(kphis);
    sqlx::query(AssertSqlSafe(sql))
        .bind(fclist_id)
        .fetch_all(pool)
        .await
        .map_err(|e| Source::SQLx.to_error(500, e, "Select FocusListGoalItemOnly"))?
        .iter()
        .map(FocusListGoalItemOnly::from_row)
        .collect::<sqlx::Result<Vec<FocusListGoalItemOnly>>>()
        .map_err(|e| Source::SQLx.to_error(500, e, "Select FocusListGoalItemOnly"))
}

// opd-er-nurse-focus-list-save.php OR opd-er-nurse-focus-list-update.php
pub async fn post_focus_list(
    opd_er_order_master_id: u32,
    save: &FocusListSave,
    user: &str,
    pool: &Pool<MySql>,
    hosxp: &str,
    kphis: &str,
    kphis_log: &str,
) -> Result<(u32, Vec<ExecuteResponse>), AppError> {
    if let Some(fclist_id) = save.fclist_id {
        let results = update_focus_list_bundle(fclist_id, opd_er_order_master_id, save, user, pool, hosxp, kphis, kphis_log).await?;

        Ok((fclist_id, results))
    } else {
        insert_focus_list_bundle(opd_er_order_master_id, save, user, pool, kphis, kphis_log).await
    }
}

async fn update_focus_list_bundle(
    fclist_id: u32,
    opd_er_order_master_id: u32,
    save: &FocusListSave,
    user: &str,
    pool: &Pool<MySql>,
    hosxp: &str,
    kphis: &str,
    kphis_log: &str,
) -> Result<Vec<ExecuteResponse>, AppError> {
    let mut results = Vec::with_capacity(5);
    // 0. Check fclist_id is used in focus_note
    let used = get_exists("opd-er-focus-list-used", &fclist_id.to_string(), pool, hosxp, kphis).await?;
    // 1. Update focus_list
    let update_focus_list_result = if used {
        update_focus_list_used(fclist_id, opd_er_order_master_id, save, user, pool, kphis).await?
    } else {
        update_focus_list_new(fclist_id, opd_er_order_master_id, save, user, pool, kphis).await?
    };
    let is_update = update_focus_list_result.rows_affected() > 0;
    results.push(ExecuteResponse::from_query_result(update_focus_list_result, "Update FocusList"));
    // 2. Insert to history_ipd_focus_list
    if is_update {
        let insert_history_focus_list_result = insert_history_log(
            SourceTable::OpdErFocusList,
            "U",
            user,
            &[KeyValue("fclist_id", fclist_id.to_string()), KeyValue("version", (save.version + 1).to_string())],
            kphis,
            kphis_log,
            pool,
        )
        .await?;
        results.push(ExecuteResponse::from_query_result(insert_history_focus_list_result, "Insert FocusList History"));
    }
    // 3. Delete used goals
    let delete_goal_item_result = delete_goal_item(fclist_id, save.version, pool, kphis).await?;
    results.push(ExecuteResponse::from_query_result(delete_goal_item_result, "Delete GoalItem"));

    if !save.goal_ids.is_empty() {
        // 4. Insert goals item
        let insert_goal_item_result = insert_goal_items(fclist_id, &save.goal_ids, save.version + 1, user, pool, kphis).await?;
        results.push(ExecuteResponse::from_query_result(insert_goal_item_result, "Insert GoalItem"));
        // 5. Insert history goal_item
        let insert_history_goal_result = insert_history_log(SourceTable::OpdErFocusListGoalItem, "U", user, &[KeyValue("fclist_id", fclist_id.to_string())], kphis, kphis_log, pool).await?;
        results.push(ExecuteResponse::from_query_result(insert_history_goal_result, "Insert GoalItem History"));
    }

    Ok(results)
}

pub async fn insert_focus_list_bundle(
    opd_er_order_master_id: u32,
    save: &FocusListSave,
    user: &str,
    pool: &Pool<MySql>,
    kphis: &str,
    kphis_log: &str,
) -> Result<(u32, Vec<ExecuteResponse>), AppError> {
    let mut results = Vec::with_capacity(4);
    // 1. Insert focus_list
    let insert_focus_list_result = insert_focus_list(opd_er_order_master_id, save, user, pool, kphis).await?;
    let fclist_id = insert_focus_list_result.last_insert_id() as u32;
    results.push(ExecuteResponse::from_query_result(insert_focus_list_result, "Insert FocusList"));
    // 2. Insert to history_ipd_focus_list
    let insert_history_focus_list_result = insert_history_log(SourceTable::OpdErFocusList, "I", user, &[KeyValue("fclist_id", fclist_id.to_string())], kphis, kphis_log, pool).await?;
    results.push(ExecuteResponse::from_query_result(insert_history_focus_list_result, "Insert FocusList History"));
    if !save.goal_ids.is_empty() {
        // 3. Insert goals item
        let insert_goal_item_result = insert_goal_items(fclist_id, &save.goal_ids, 1, user, pool, kphis).await?;
        results.push(ExecuteResponse::from_query_result(insert_goal_item_result, "Insert GoalItem"));
        // 4. Insert history goal_item
        let insert_history_goal_result = insert_history_log(SourceTable::OpdErFocusListGoalItem, "I", user, &[KeyValue("fclist_id", fclist_id.to_string())], kphis, kphis_log, pool).await?;
        results.push(ExecuteResponse::from_query_result(insert_history_goal_result, "Insert GoalItem History"));
    }

    Ok((fclist_id, results))
}

pub async fn insert_focus_list_only_bundle(opd_er_order_master_id: u32, only: &FocusListOnly, pool: &Pool<MySql>, kphis: &str, kphis_log: &str) -> Result<(u32, Vec<ExecuteResponse>), AppError> {
    let mut results = Vec::with_capacity(4);
    // 1. Insert focus_list
    let insert_focus_list_result = insert_focus_list_only(opd_er_order_master_id, only, pool, kphis).await?;
    let fclist_id = insert_focus_list_result.last_insert_id() as u32;
    results.push(ExecuteResponse::from_query_result(insert_focus_list_result, "Insert FocusListOnly"));
    // 2. Insert to history_ipd_focus_list
    let insert_history_focus_list_result = insert_history_log(SourceTable::OpdErFocusList, "I", "system", &[KeyValue("fclist_id", fclist_id.to_string())], kphis, kphis_log, pool).await?;
    results.push(ExecuteResponse::from_query_result(insert_history_focus_list_result, "Insert FocusListOnly History"));
    if !only.focus_list_goal_items.is_empty() {
        // 3. Insert goals item
        let insert_goal_item_result = insert_goal_items_only(fclist_id, &only.focus_list_goal_items, pool, kphis).await?;
        results.push(ExecuteResponse::from_query_result(insert_goal_item_result, "Insert GoalItemOnly"));
        // 4. Insert history goal_item
        let insert_history_goal_result = insert_history_log(
            SourceTable::OpdErFocusListGoalItem,
            "I",
            "system",
            &[KeyValue("fclist_id", fclist_id.to_string())],
            kphis,
            kphis_log,
            pool,
        )
        .await?;
        results.push(ExecuteResponse::from_query_result(insert_history_goal_result, "Insert GoalItemOnly History"));
    }

    Ok((fclist_id, results))
}

async fn update_focus_list_used(fclist_id: u32, opd_er_order_master_id: u32, save: &FocusListSave, user: &str, pool: &Pool<MySql>, kphis: &str) -> Result<MySqlQueryResult, AppError> {
    let update_focus_list_sql = focus_list::update_focus_list_used(kphis);
    sqlx::query(AssertSqlSafe(update_focus_list_sql))
        .bind(save.fclist_enddate)
        .bind(save.fclist_endtime)
        .bind(&save.fclist_status)
        .bind(opd_er_order_master_id)
        .bind(user)
        .bind(fclist_id)
        .bind(save.version)
        .execute(pool)
        .await
        .map_err(|e| Source::SQLx.to_error(500, e, "Update UsedFocusList"))
}

async fn update_focus_list_new(fclist_id: u32, opd_er_order_master_id: u32, save: &FocusListSave, user: &str, pool: &Pool<MySql>, kphis: &str) -> Result<MySqlQueryResult, AppError> {
    let update_focus_list_sql = focus_list::update_focus_list_new(kphis);
    sqlx::query(AssertSqlSafe(update_focus_list_sql))
        .bind(save.smp_id)
        .bind(save.focus_id)
        .bind(&save.focus_text)
        .bind(&save.goal_text)
        .bind(save.fclist_stdate)
        .bind(save.fclist_sttime)
        .bind(save.fclist_enddate)
        .bind(save.fclist_endtime)
        .bind(&save.fclist_status)
        .bind(opd_er_order_master_id)
        .bind(user)
        .bind(fclist_id)
        .bind(save.version)
        .execute(pool)
        .await
        .map_err(|e| Source::SQLx.to_error(500, e, "Update FocusList"))
}

async fn insert_focus_list(opd_er_order_master_id: u32, save: &FocusListSave, user: &str, pool: &Pool<MySql>, kphis: &str) -> Result<MySqlQueryResult, AppError> {
    let insert_focus_list_sql = focus_list::insert_focus_list(kphis);
    sqlx::query(AssertSqlSafe(insert_focus_list_sql))
        .bind(save.smp_id)
        .bind(save.focus_id)
        .bind(&save.focus_text)
        .bind(&save.goal_text)
        .bind(save.fclist_stdate)
        .bind(save.fclist_sttime)
        .bind(save.fclist_enddate)
        .bind(save.fclist_endtime)
        .bind(&save.fclist_status)
        .bind(opd_er_order_master_id)
        .bind(user)
        .bind(user)
        .execute(pool)
        .await
        .map_err(|e| Source::SQLx.to_error(500, e, "Insert FocusList"))
}

async fn insert_focus_list_only(opd_er_order_master_id: u32, only: &FocusListOnly, pool: &Pool<MySql>, kphis: &str) -> Result<MySqlQueryResult, AppError> {
    let insert_focus_list_sql = focus_list::insert_focus_list_only(kphis);
    sqlx::query(AssertSqlSafe(insert_focus_list_sql))
        .bind(only.smp_id)
        .bind(only.focus_id)
        .bind(&only.focus_text)
        .bind(&only.goal_text)
        .bind(only.fclist_stdate)
        .bind(only.fclist_sttime)
        .bind(only.fclist_enddate)
        .bind(only.fclist_endtime)
        .bind(&only.fclist_status)
        .bind(opd_er_order_master_id)
        .bind(&only.create_user)
        .bind(only.create_datetime)
        .bind(&only.update_user)
        .bind(only.update_datetime)
        .bind(only.version)
        .execute(pool)
        .await
        .map_err(|e| Source::SQLx.to_error(500, e, "Insert FocusListOnly"))
}

async fn insert_goal_items(fclist_id: u32, goal_ids: &[u32], version: i32, user: &str, pool: &Pool<MySql>, kphis: &str) -> Result<MySqlQueryResult, AppError> {
    let insert_goal_item_sql = focus_list::insert_goal_items(goal_ids, fclist_id, user, version, kphis);
    sqlx::query(AssertSqlSafe(insert_goal_item_sql)).execute(pool).await.map_err(|e| {
        if let sqlx::error::Error::Database(err) = &e
            && err.code().map(|c| c == "23000").unwrap_or_default()
        {
            AppError::app_403_duplicate("Insert FocusListGoalItem")
        } else {
            Source::SQLx.to_error(500, e, "Insert FocusListGoalItem")
        }
    })
}

async fn insert_goal_items_only(fclist_id: u32, goals: &[FocusListGoalItemOnly], pool: &Pool<MySql>, kphis: &str) -> Result<MySqlQueryResult, AppError> {
    let insert_goal_item_sql = focus_list::insert_goal_items_only(goals, fclist_id, kphis);
    sqlx::query(AssertSqlSafe(insert_goal_item_sql)).execute(pool).await.map_err(|e| {
        if let sqlx::error::Error::Database(err) = &e
            && err.code().map(|c| c == "23000").unwrap_or_default()
        {
            AppError::app_403_duplicate("Insert FocusListGoalItemOnly")
        } else {
            Source::SQLx.to_error(500, e, "Insert FocusListGoalItemOnly")
        }
    })
}

// opd-er-nurse-focus-list-delete.php
pub async fn delete_focus_list(fclist_id: u32, version: i32, user: &str, pool: &Pool<MySql>, hosxp: &str, kphis: &str, kphis_log: &str) -> Result<Vec<ExecuteResponse>, AppError> {
    let mut results = Vec::with_capacity(4);
    // 0. Check fclist_id is used in focus_note
    let used = get_exists("opd-er-focus-list-used", &fclist_id.to_string(), pool, hosxp, kphis).await?;
    if used {
        results.push(Source::App.to_error(500, "Used", "Delete FocusList").into());
    } else {
        // 1. Insert to history_ipd_focus_list
        let insert_history_focus_list_result = insert_history_log(
            SourceTable::OpdErFocusList,
            "D",
            user,
            &[KeyValue("fclist_id", fclist_id.to_string()), KeyValue("version", version.to_string())],
            kphis,
            kphis_log,
            pool,
        )
        .await?;
        let will_delete = insert_history_focus_list_result.rows_affected() > 0;
        results.push(ExecuteResponse::from_query_result(insert_history_focus_list_result, "Insert FocusList History"));
        if will_delete {
            // 2. Delete focus list
            let delete_focus_list_result = delete_focus_list_inner(fclist_id, version, pool, kphis).await?;
            results.push(ExecuteResponse::from_query_result(delete_focus_list_result, "Delete FocusList"));

            // 3. Insert history goal_item
            let insert_history_goal_result = insert_history_log(
                SourceTable::OpdErFocusListGoalItem,
                "D",
                user,
                &[KeyValue("fclist_id", fclist_id.to_string()), KeyValue("version", version.to_string())],
                kphis,
                kphis_log,
                pool,
            )
            .await?;
            let will_delete_goal = insert_history_goal_result.rows_affected() > 0;
            results.push(ExecuteResponse::from_query_result(insert_history_goal_result, "Insert GoalItem History"));
            if will_delete_goal {
                // 4. Delete goals
                let delete_goal_item_result = delete_goal_item(fclist_id, version, pool, kphis).await?;
                results.push(ExecuteResponse::from_query_result(delete_goal_item_result, "Delete GoalItem"));
            }
        }
    }

    Ok(results)
}

async fn delete_focus_list_inner(fclist_id: u32, version: i32, pool: &Pool<MySql>, kphis: &str) -> Result<MySqlQueryResult, AppError> {
    let delete_focus_list_sql = focus_list::delete_focus_list(kphis);
    sqlx::query(AssertSqlSafe(delete_focus_list_sql))
        .bind(fclist_id)
        .bind(version)
        .execute(pool)
        .await
        .map_err(|e| Source::SQLx.to_error(500, e, "Delete FocusList"))
}

async fn delete_goal_item(fclist_id: u32, version: i32, pool: &Pool<MySql>, kphis: &str) -> Result<MySqlQueryResult, AppError> {
    let delete_goal_item_sql = focus_list::delete_goal_item(kphis);
    sqlx::query(AssertSqlSafe(delete_goal_item_sql))
        .bind(fclist_id)
        .bind(version)
        .execute(pool)
        .await
        .map_err(|e| Source::SQLx.to_error(500, e, "Delete GoalItem"))
}

#[cfg(test)]
#[rustfmt::skip]
mod tests {

    use super::*;
    use kphis_sqlx_tester::MySqlTester;
    use kphis_util::datetime::date_8601;

    #[tokio::test]
    #[ignore]
    async fn sqlx_get_focus_list() {
        let tester = MySqlTester::new_hosxp_and_kphis().await;
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis/opd_er_focus_list.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis/opd_er_focus_list_goal_item.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis/opd_er_focus_note.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis/opd_er_order_master.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis/opd_er_dch_type.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis/ipd_tmp_focus.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis/ipd_tmp_goal.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/hosxp/opduser.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/hosxp/doctor.sql")).execute(&tester.db_pool).await.unwrap();

        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/kphis/opd_er_focus_list.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/kphis/opd_er_focus_list_goal_item.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/kphis/opd_er_focus_note.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/kphis/opd_er_order_master.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/kphis/opd_er_dch_type.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/kphis/ipd_tmp_focus.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/kphis/ipd_tmp_goal.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/hosxp/opduser.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/hosxp/doctor.sql")).execute(&tester.db_pool).await.unwrap();

        let default = get_focus_list(1,&FocusListParams::default(),&tester.db_pool,&tester.hosxp,&tester.kphis).await.unwrap();
        assert_eq!(default.len(), 2);
        let found_start_end = get_focus_list(1,&FocusListParams {start_date: date_8601("2024-01-01"),end_date: date_8601("2024-02-01"),..Default::default()},&tester.db_pool,&tester.hosxp,&tester.kphis).await.unwrap();
        assert_eq!(found_start_end.len(), 2);
        let found_start_alone = get_focus_list(1,&FocusListParams {start_date: date_8601("2024-01-01"),..Default::default()},&tester.db_pool,&tester.hosxp,&tester.kphis).await.unwrap();
        assert_eq!(found_start_alone.len(), 1);
        // will treat end_date as start_date
        let found_end_alone = get_focus_list(1,&FocusListParams {end_date: date_8601("2024-01-01"),..Default::default()},&tester.db_pool,&tester.hosxp,&tester.kphis).await.unwrap();
        assert_eq!(found_end_alone.len(), 1);
        let found_status_fin = get_focus_list(1,&FocusListParams {status: Some(String::from("2")),..Default::default()},&tester.db_pool,&tester.hosxp,&tester.kphis).await.unwrap();
        assert_eq!(found_status_fin.len(), 1);
        let found_status_unfin = get_focus_list(1,&FocusListParams {status: Some(String::from("1")),..Default::default()},&tester.db_pool,&tester.hosxp,&tester.kphis).await.unwrap();
        assert_eq!(found_status_unfin.len(), 1);
        let found_id = get_focus_list(1,&FocusListParams {fclist_id: Some(1),..Default::default()},&tester.db_pool,&tester.hosxp,&tester.kphis).await.unwrap();
        assert_eq!(found_id.len(), 1);
        let not_found = get_focus_list(999,&FocusListParams {fclist_id: Some(1),..Default::default()},&tester.db_pool,&tester.hosxp,&tester.kphis).await.unwrap();
        assert!(not_found.is_empty());
    }

    #[tokio::test]
    #[ignore]
    async fn sqlx_get_focus_list_only() {
        let tester = MySqlTester::new_kphis().await;
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis/opd_er_focus_list.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/kphis/opd_er_focus_list.sql")).execute(&tester.db_pool).await.unwrap();

        let found = get_focus_list_only(1,&tester.db_pool,&tester.kphis).await.unwrap();
        assert_eq!(found.len(), 2);
        let not_found = get_focus_list_only(999,&tester.db_pool,&tester.kphis).await.unwrap();
        assert!(not_found.is_empty());
    }

    #[tokio::test]
    #[ignore]
    async fn sqlx_get_focus_list_goal_item_only() {
        let tester = MySqlTester::new_kphis().await;
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis/opd_er_focus_list_goal_item.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/kphis/opd_er_focus_list_goal_item.sql")).execute(&tester.db_pool).await.unwrap();

        let found = get_focus_list_goal_item_only(1, &tester.db_pool,&tester.kphis).await.unwrap();
        assert_eq!(found.len(), 1);
        let not_found = get_focus_list_goal_item_only(999,&tester.db_pool,&tester.kphis).await.unwrap();
        assert!(not_found.is_empty());
    }

    #[tokio::test]
    #[ignore]
    async fn sqlx_insert_focus_list() {
        let tester = MySqlTester::new_kphis().await;
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis/opd_er_focus_list.sql")).execute(&tester.db_pool).await.unwrap();

        let success = insert_focus_list(1,&FocusListSave::demo(),"user",&tester.db_pool,&tester.kphis).await.unwrap();
        assert_eq!(success.rows_affected(), 1);
        let again_success = insert_focus_list(1,&FocusListSave::demo(),"user",&tester.db_pool,&tester.kphis).await.unwrap();
        assert_eq!(again_success.rows_affected(), 1);
    }

    #[tokio::test]
    #[ignore]
    async fn sqlx_insert_focus_list_only() {
        let tester = MySqlTester::new_kphis().await;
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis/opd_er_focus_list.sql")).execute(&tester.db_pool).await.unwrap();

        let success = insert_focus_list_only(1,&FocusListOnly::demo(),&tester.db_pool,&tester.kphis).await.unwrap();
        assert_eq!(success.rows_affected(), 1);
        let again_success = insert_focus_list_only(1,&FocusListOnly::demo(),&tester.db_pool,&tester.kphis).await.unwrap();
        assert_eq!(again_success.rows_affected(), 1);
    }

    #[tokio::test]
    #[ignore]
    async fn sqlx_insert_goal_items() {
        let tester = MySqlTester::new_kphis().await;
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis/opd_er_focus_list_goal_item.sql")).execute(&tester.db_pool).await.unwrap();

        let success = insert_goal_items(1, &[1, 999], 1, "user", &tester.db_pool, &tester.kphis).await.unwrap();
        assert_eq!(success.rows_affected(), 2);
        // duplicate (fclist_id + goal_id) will error
        let again_duplicate = insert_goal_items(1, &[1, 999], 1, "user", &tester.db_pool, &tester.kphis).await;
        if let Err(again_duplicate_error) = again_duplicate {
            assert_eq!(again_duplicate_error.status, 403);
        } else {
            panic!("MUST ERROR")
        }
    }

    #[tokio::test]
    #[ignore]
    async fn sqlx_insert_goal_items_only() {
        let tester = MySqlTester::new_kphis().await;
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis/opd_er_focus_list_goal_item.sql")).execute(&tester.db_pool).await.unwrap();

        let success = insert_goal_items_only(1, &[FocusListGoalItemOnly::demo()], &tester.db_pool, &tester.kphis).await.unwrap();
        assert_eq!(success.rows_affected(), 1);
        // duplicate (fclist_id + goal_id) will error
        let again_duplicate = insert_goal_items_only(1, &[FocusListGoalItemOnly::demo()], &tester.db_pool, &tester.kphis).await;
        if let Err(again_duplicate_error) = again_duplicate {
            assert_eq!(again_duplicate_error.status, 403);
        } else {
            panic!("MUST ERROR")
        }
    }

    #[tokio::test]
    #[ignore]
    async fn sqlx_update_focus_list_new() {
        let tester = MySqlTester::new_kphis().await;
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis/opd_er_focus_list.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/kphis/opd_er_focus_list.sql")).execute(&tester.db_pool).await.unwrap();

        let success = update_focus_list_new(1,1,&FocusListSave::demo(),"user",&tester.db_pool,&tester.kphis).await.unwrap();
        assert_eq!(success.rows_affected(), 1);
        let again_fail_version = update_focus_list_new(1,1,&FocusListSave::demo(),"user",&tester.db_pool,&tester.kphis).await.unwrap();
        assert_eq!(again_fail_version.rows_affected(), 0);
    }

    #[tokio::test]
    #[ignore]
    async fn sqlx_update_focus_list_used() {
        let tester = MySqlTester::new_kphis().await;
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis/opd_er_focus_list.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/kphis/opd_er_focus_list.sql")).execute(&tester.db_pool).await.unwrap();

        let success = update_focus_list_used(1,1,&FocusListSave::demo(),"user",&tester.db_pool,&tester.kphis).await.unwrap();
        assert_eq!(success.rows_affected(), 1);
        let again_fail_version = update_focus_list_used(1,1,&FocusListSave::demo(),"user",&tester.db_pool,&tester.kphis).await.unwrap();
        assert_eq!(again_fail_version.rows_affected(), 0);
    }

    #[tokio::test]
    #[ignore]
    async fn sqlx_delete_focus_list_inner() {
        let tester = MySqlTester::new_kphis().await;
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis/opd_er_focus_list.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/kphis/opd_er_focus_list.sql")).execute(&tester.db_pool).await.unwrap();

        let version_not_found = delete_focus_list_inner(1, 9, &tester.db_pool, &tester.kphis).await.unwrap();
        assert_eq!(version_not_found.rows_affected(), 0);
        let success = delete_focus_list_inner(1, 1, &tester.db_pool, &tester.kphis).await.unwrap();
        assert_eq!(success.rows_affected(), 1);
        let again_not_found = delete_focus_list_inner(1, 1, &tester.db_pool, &tester.kphis).await.unwrap();
        assert_eq!(again_not_found.rows_affected(), 0);
    }

    #[tokio::test]
    #[ignore]
    async fn sqlx_delete_goal_item() {
        let tester = MySqlTester::new_kphis().await;
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis/opd_er_focus_list_goal_item.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/kphis/opd_er_focus_list_goal_item.sql")).execute(&tester.db_pool).await.unwrap();

        let version_not_found = delete_goal_item(1, 9, &tester.db_pool, &tester.kphis).await.unwrap();
        assert_eq!(version_not_found.rows_affected(), 0);
        let success = delete_goal_item(1, 1, &tester.db_pool, &tester.kphis).await.unwrap();
        assert_eq!(success.rows_affected(), 1);
        let again_not_found = delete_goal_item(1, 1, &tester.db_pool, &tester.kphis).await.unwrap();
        assert_eq!(again_not_found.rows_affected(), 0);
    }
}
