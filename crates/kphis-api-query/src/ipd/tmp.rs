use sqlx::{AssertSqlSafe, FromRow, MySql, Pool, Row, mysql::MySqlQueryResult};

use kphis_model::{
    fetch::ExecuteResponse,
    ipd::tmp::{TmpDlc, TmpFocus, TmpGoal, TmpGroup, TmpIntvt, TmpParams, TmpSubGroup},
};
use kphis_sql::ipd::tmp;
use kphis_util::error::{AppError, Source};

// setting-template-nurse-note-smp-data.php
pub async fn get_group(params: &TmpParams, pool: &Pool<MySql>, kphis: &str) -> Result<Vec<TmpGroup>, AppError> {
    let sql = tmp::select_group(params, kphis);
    let mut query = sqlx::query(AssertSqlSafe(sql));
    if let Some(smp_id) = params.smp_id {
        query = query.bind(smp_id);
    }
    query
        .fetch_all(pool)
        .await
        .map_err(|e| Source::SQLx.to_error(500, e, "Select Group"))?
        .iter()
        .map(TmpGroup::from_row)
        .collect::<sqlx::Result<Vec<TmpGroup>>>()
        .map_err(|e| Source::SQLx.to_error(500, e, "Select Group"))
}

// setting-template-nurse-note-smp-save.php
pub async fn post_group(save: &TmpGroup, user: &str, pool: &Pool<MySql>, kphis: &str) -> Result<ExecuteResponse, AppError> {
    let result = if save.smp_id > 0 {
        let update_result = update_group(save, user, pool, kphis).await?;
        ExecuteResponse::from_query_result(update_result, "Update Group")
    } else {
        let last_insert_id = insert_group(save, user, pool, kphis).await?.unwrap_or_default() as u64;
        ExecuteResponse {
            last_insert_id,
            rows_affected: if last_insert_id > 0 { 1 } else { 0 },
            error: None,
            action: Some(String::from("Insert Group")),
        }
    };

    Ok(result)
}

async fn update_group(save: &TmpGroup, user: &str, pool: &Pool<MySql>, kphis: &str) -> Result<MySqlQueryResult, AppError> {
    let update_sql = tmp::update_group(kphis);
    sqlx::query(AssertSqlSafe(update_sql))
        .bind(&save.smp_name)
        .bind(&save.smp_status)
        .bind(user)
        .bind(save.smp_id)
        .execute(pool)
        .await
        .map_err(|e| Source::SQLx.to_error(500, e, "Update Group"))
}

async fn insert_group(save: &TmpGroup, user: &str, pool: &Pool<MySql>, kphis: &str) -> Result<Option<u32>, AppError> {
    let insert_sql = tmp::insert_group(kphis);
    sqlx::query(AssertSqlSafe(insert_sql))
        .bind(&save.smp_name)
        .bind(&save.smp_status)
        .bind(user)
        .bind(user)
        .fetch_optional(pool)
        .await
        .map_err(|e| Source::SQLx.to_error(500, e, "Insert Group"))?
        .map(|row| row.try_get::<u32, usize>(0))
        .transpose()
        .map_err(|e| Source::SQLx.to_error(500, e, "Insert Group"))
}

// setting-template-nurse-note-smp-delete.php
pub async fn delete_group(params: &TmpParams, user: &str, pool: &Pool<MySql>, kphis: &str) -> Result<ExecuteResponse, AppError> {
    let sql = tmp::delete_group(kphis);
    let result = sqlx::query(AssertSqlSafe(sql))
        .bind(user)
        .bind(params.smp_id)
        .bind(params.smp_id)
        .bind(params.smp_id)
        .execute(pool)
        .await
        .map_err(|e| Source::SQLx.to_error(500, e, "Delete Group"))?;

    Ok(ExecuteResponse::from_query_result(result, "Delete Group"))
}

// setting-template-nurse-note-subgroup-data.php
pub async fn get_subgroup(params: &TmpParams, pool: &Pool<MySql>, kphis: &str) -> Result<Vec<TmpSubGroup>, AppError> {
    let sql = tmp::select_subgroup(params, kphis);
    let mut query = sqlx::query(AssertSqlSafe(sql));
    if let Some(smp_id) = params.smp_id {
        query = query.bind(smp_id);
    }
    if let Some(subgroup) = params.subgroup {
        query = query.bind(subgroup);
    }
    query
        .fetch_all(pool)
        .await
        .map_err(|e| Source::SQLx.to_error(500, e, "Get SubGroup"))?
        .iter()
        .map(TmpSubGroup::from_row)
        .collect::<sqlx::Result<Vec<TmpSubGroup>>>()
        .map_err(|e| Source::SQLx.to_error(500, e, "Get SubGroup"))
}

// setting-template-nurse-note-subgroup-save.php
pub async fn post_subgroup(save: &TmpSubGroup, user: &str, pool: &Pool<MySql>, kphis: &str) -> Result<ExecuteResponse, AppError> {
    let result = if save.subgroup > 0 {
        let update_result = update_subgroup(save, user, pool, kphis).await?;
        ExecuteResponse::from_query_result(update_result, "Update SubGroup")
    } else {
        let last_insert_id = insert_subgroup(save, user, pool, kphis).await?.unwrap_or_default() as u64;
        ExecuteResponse {
            last_insert_id,
            rows_affected: if last_insert_id > 0 { 1 } else { 0 },
            error: None,
            action: Some(String::from("Insert SubGroup")),
        }
    };

    Ok(result)
}

async fn update_subgroup(save: &TmpSubGroup, user: &str, pool: &Pool<MySql>, kphis: &str) -> Result<MySqlQueryResult, AppError> {
    let update_sql = tmp::update_subgroup(kphis);
    sqlx::query(AssertSqlSafe(update_sql))
        .bind(&save.subgroup_name)
        .bind(&save.subgroup_status)
        .bind(user)
        .bind(save.smp_id)
        .bind(save.subgroup)
        .execute(pool)
        .await
        .map_err(|e| Source::SQLx.to_error(500, e, "Update SubGroup"))
}

async fn insert_subgroup(save: &TmpSubGroup, user: &str, pool: &Pool<MySql>, kphis: &str) -> Result<Option<u32>, AppError> {
    let insert_sql = tmp::insert_subgroup(kphis);
    sqlx::query(AssertSqlSafe(insert_sql))
        .bind(save.smp_id)
        .bind(save.smp_id)
        .bind(&save.subgroup_name)
        .bind(&save.subgroup_status)
        .bind(user)
        .bind(user)
        .fetch_optional(pool)
        .await
        .map_err(|e| Source::SQLx.to_error(500, e, "Insert SubGroup"))?
        .map(|row| row.try_get::<u32, usize>(0))
        .transpose()
        .map_err(|e| Source::SQLx.to_error(500, e, "Insert SubGroup"))
}

// setting-template-nurse-note-subgroup-delete.php
pub async fn delete_subgroup(params: &TmpParams, user: &str, pool: &Pool<MySql>, kphis: &str) -> Result<ExecuteResponse, AppError> {
    let sql = tmp::delete_subgroup(kphis);
    let result = sqlx::query(AssertSqlSafe(sql))
        .bind(user)
        .bind(params.smp_id)
        .bind(params.subgroup)
        .bind(params.smp_id)
        .bind(params.smp_id)
        .execute(pool)
        .await
        .map_err(|e| Source::SQLx.to_error(500, e, "Delete SubGroup"))?;

    Ok(ExecuteResponse::from_query_result(result, "Delete SubGroup"))
}

// setting-template-nurse-note-focus-data.php
pub async fn get_focus(params: &TmpParams, pool: &Pool<MySql>, kphis: &str) -> Result<Vec<TmpFocus>, AppError> {
    let sql = tmp::select_focus(params, kphis);
    let mut query = sqlx::query(AssertSqlSafe(sql));
    if let Some(smp_id) = params.smp_id {
        query = query.bind(smp_id);
    }
    if let Some(subgroup) = params.subgroup {
        query = query.bind(subgroup);
    }
    if let Some(focus_id) = params.id {
        query = query.bind(focus_id);
    }
    query
        .fetch_all(pool)
        .await
        .map_err(|e| Source::SQLx.to_error(500, e, "Get Focus"))?
        .iter()
        .map(TmpFocus::from_row)
        .collect::<sqlx::Result<Vec<TmpFocus>>>()
        .map_err(|e| Source::SQLx.to_error(500, e, "Get Focus"))
}

// setting-template-nurse-note-focus-save.php
pub async fn post_focus(save: &TmpFocus, user: &str, pool: &Pool<MySql>, kphis: &str) -> Result<ExecuteResponse, AppError> {
    let result = if save.focus_id > 0 {
        let update_result = update_focus(save, user, pool, kphis).await?;
        ExecuteResponse::from_query_result(update_result, "Update Focus")
    } else {
        let last_insert_id = insert_focus(save, user, pool, kphis).await?.unwrap_or_default() as u64;
        ExecuteResponse {
            last_insert_id,
            rows_affected: if last_insert_id > 0 { 1 } else { 0 },
            error: None,
            action: Some(String::from("Insert Focus")),
        }
    };

    Ok(result)
}

async fn update_focus(save: &TmpFocus, user: &str, pool: &Pool<MySql>, kphis: &str) -> Result<MySqlQueryResult, AppError> {
    let update_sql = tmp::update_focus(kphis);
    sqlx::query(AssertSqlSafe(update_sql))
        .bind(&save.focus_name)
        .bind(&save.focus_status)
        .bind(user)
        .bind(save.focus_id)
        .execute(pool)
        .await
        .map_err(|e| Source::SQLx.to_error(500, e, "Update Focus"))
}

async fn insert_focus(save: &TmpFocus, user: &str, pool: &Pool<MySql>, kphis: &str) -> Result<Option<u32>, AppError> {
    let insert_sql = tmp::insert_focus(kphis);
    sqlx::query(AssertSqlSafe(insert_sql))
        .bind(&save.focus_name)
        .bind(save.smp_id)
        .bind(save.subgroup)
        .bind(save.smp_id)
        .bind(save.smp_id)
        .bind(&save.focus_status)
        .bind(user)
        .bind(user)
        .fetch_optional(pool)
        .await
        .map_err(|e| Source::SQLx.to_error(500, e, "Insert Focus"))?
        .map(|row| row.try_get::<u32, usize>(0))
        .transpose()
        .map_err(|e| Source::SQLx.to_error(500, e, "Insert Focus"))
}

// setting-template-nurse-note-focus-delete.php
pub async fn delete_focus(params: &TmpParams, user: &str, pool: &Pool<MySql>, kphis: &str) -> Result<ExecuteResponse, AppError> {
    let sql = tmp::delete_focus(kphis);
    let result = sqlx::query(AssertSqlSafe(sql))
        .bind(user)
        .bind(params.id)
        .bind(params.id)
        .bind(params.id)
        .execute(pool)
        .await
        .map_err(|e| Source::SQLx.to_error(500, e, "Delete Focus"))?;

    Ok(ExecuteResponse::from_query_result(result, "Delete Focus"))
}

// setting-template-nurse-note-goal-data.php
pub async fn get_goal(params: &TmpParams, pool: &Pool<MySql>, kphis: &str) -> Result<Vec<TmpGoal>, AppError> {
    let sql = tmp::select_goal(params, kphis);
    let mut query = sqlx::query(AssertSqlSafe(sql));
    if let Some(smp_id) = params.smp_id {
        query = query.bind(smp_id);
    }
    if let Some(subgroup) = params.subgroup {
        query = query.bind(subgroup);
    }
    if let Some(goal_id) = params.id {
        query = query.bind(goal_id);
    }
    query
        .fetch_all(pool)
        .await
        .map_err(|e| Source::SQLx.to_error(500, e, "Get Goal"))?
        .iter()
        .map(TmpGoal::from_row)
        .collect::<sqlx::Result<Vec<TmpGoal>>>()
        .map_err(|e| Source::SQLx.to_error(500, e, "Get Goal"))
}

// setting-template-nurse-note-goal-save.php
pub async fn post_goal(save: &TmpGoal, user: &str, pool: &Pool<MySql>, kphis: &str) -> Result<ExecuteResponse, AppError> {
    let result = if save.goal_id > 0 {
        let update_result = update_goal(save, user, pool, kphis).await?;
        ExecuteResponse::from_query_result(update_result, "Update Goal")
    } else {
        let last_insert_id = insert_goal(save, user, pool, kphis).await?.unwrap_or_default() as u64;
        ExecuteResponse {
            last_insert_id,
            rows_affected: if last_insert_id > 0 { 1 } else { 0 },
            error: None,
            action: Some(String::from("Insert Goal")),
        }
    };

    Ok(result)
}

async fn update_goal(save: &TmpGoal, user: &str, pool: &Pool<MySql>, kphis: &str) -> Result<MySqlQueryResult, AppError> {
    let update_sql = tmp::update_goal(kphis);
    sqlx::query(AssertSqlSafe(update_sql))
        .bind(&save.goal_name)
        .bind(&save.goal_status)
        .bind(user)
        .bind(save.goal_id)
        .execute(pool)
        .await
        .map_err(|e| Source::SQLx.to_error(500, e, "Update Goal"))
}

async fn insert_goal(save: &TmpGoal, user: &str, pool: &Pool<MySql>, kphis: &str) -> Result<Option<u32>, AppError> {
    let insert_sql = tmp::insert_goal(kphis);
    sqlx::query(AssertSqlSafe(insert_sql))
        .bind(&save.goal_name)
        .bind(save.smp_id)
        .bind(save.subgroup)
        .bind(save.smp_id)
        .bind(save.smp_id)
        .bind(&save.goal_status)
        .bind(user)
        .bind(user)
        .fetch_optional(pool)
        .await
        .map_err(|e| Source::SQLx.to_error(500, e, "Insert Goal"))?
        .map(|row| row.try_get::<u32, usize>(0))
        .transpose()
        .map_err(|e| Source::SQLx.to_error(500, e, "Insert Goal"))
}

// setting-template-nurse-note-goal-delete.php
pub async fn delete_goal(params: &TmpParams, user: &str, pool: &Pool<MySql>, kphis: &str) -> Result<ExecuteResponse, AppError> {
    let sql = tmp::delete_goal(kphis);
    let result = sqlx::query(AssertSqlSafe(sql))
        .bind(user)
        .bind(params.id)
        .bind(params.id)
        .bind(params.id)
        .execute(pool)
        .await
        .map_err(|e| Source::SQLx.to_error(500, e, "Delete Goal"))?;

    Ok(ExecuteResponse::from_query_result(result, "Delete Goal"))
}

// setting-template-nurse-note-intvt-data.php
pub async fn get_intvt(params: &TmpParams, pool: &Pool<MySql>, kphis: &str) -> Result<Vec<TmpIntvt>, AppError> {
    let sql = tmp::select_intvt(params, kphis);
    let mut query = sqlx::query(AssertSqlSafe(sql));
    if let Some(smp_id) = params.smp_id {
        query = query.bind(smp_id);
    }
    if let Some(subgroup) = params.subgroup {
        query = query.bind(subgroup);
    }
    if let Some(intvt_id) = params.id {
        query = query.bind(intvt_id);
    }
    query
        .fetch_all(pool)
        .await
        .map_err(|e| Source::SQLx.to_error(500, e, "Get Intervention"))?
        .iter()
        .map(TmpIntvt::from_row)
        .collect::<sqlx::Result<Vec<TmpIntvt>>>()
        .map_err(|e| Source::SQLx.to_error(500, e, "Get Intervention"))
}

// setting-template-nurse-note-intvt-save.php
pub async fn post_intvt(save: &TmpIntvt, user: &str, pool: &Pool<MySql>, kphis: &str) -> Result<ExecuteResponse, AppError> {
    let result = if save.intvt_id > 0 {
        let update_result = update_intvt(save, user, pool, kphis).await?;
        ExecuteResponse::from_query_result(update_result, "Update Intervention")
    } else {
        let last_insert_id = insert_intvt(save, user, pool, kphis).await?.unwrap_or_default() as u64;
        ExecuteResponse {
            last_insert_id,
            rows_affected: if last_insert_id > 0 { 1 } else { 0 },
            error: None,
            action: Some(String::from("Insert Intervention")),
        }
    };

    Ok(result)
}

async fn update_intvt(save: &TmpIntvt, user: &str, pool: &Pool<MySql>, kphis: &str) -> Result<MySqlQueryResult, AppError> {
    let update_sql = tmp::update_intvt(kphis);
    sqlx::query(AssertSqlSafe(update_sql))
        .bind(&save.intvt_name)
        .bind(&save.intvt_status)
        .bind(user)
        .bind(save.intvt_id)
        .execute(pool)
        .await
        .map_err(|e| Source::SQLx.to_error(500, e, "Update Intervention"))
}

async fn insert_intvt(save: &TmpIntvt, user: &str, pool: &Pool<MySql>, kphis: &str) -> Result<Option<u32>, AppError> {
    let insert_sql = tmp::insert_intvt(kphis);
    sqlx::query(AssertSqlSafe(insert_sql))
        .bind(&save.intvt_name)
        .bind(save.smp_id)
        .bind(save.subgroup)
        .bind(save.smp_id)
        .bind(save.smp_id)
        .bind(&save.intvt_status)
        .bind(user)
        .bind(user)
        .fetch_optional(pool)
        .await
        .map_err(|e| Source::SQLx.to_error(500, e, "Insert Intervention"))?
        .map(|row| row.try_get::<u32, usize>(0))
        .transpose()
        .map_err(|e| Source::SQLx.to_error(500, e, "Insert Intervention"))
}

// setting-template-nurse-note-intvt-delete.php
pub async fn delete_intvt(params: &TmpParams, user: &str, pool: &Pool<MySql>, kphis: &str) -> Result<ExecuteResponse, AppError> {
    let sql = tmp::delete_intvt(kphis);
    let result = sqlx::query(AssertSqlSafe(sql))
        .bind(user)
        .bind(params.id)
        .bind(params.id)
        .bind(params.id)
        .execute(pool)
        .await
        .map_err(|e| Source::SQLx.to_error(500, e, "Delete Intervention"))?;

    Ok(ExecuteResponse::from_query_result(result, "Delete Intervention"))
}

pub async fn get_dlc(params: &TmpParams, pool: &Pool<MySql>, kphis: &str) -> Result<Vec<TmpDlc>, AppError> {
    let sql = tmp::select_dlc(params, kphis);
    let mut query = sqlx::query(AssertSqlSafe(sql));
    if let Some(dlc_id) = params.id {
        query = query.bind(dlc_id);
    }
    query
        .fetch_all(pool)
        .await
        .map_err(|e| Source::SQLx.to_error(500, e, "Get DailyCare"))?
        .iter()
        .map(TmpDlc::from_row)
        .collect::<sqlx::Result<Vec<TmpDlc>>>()
        .map_err(|e| Source::SQLx.to_error(500, e, "Get DailyCare"))
}

pub async fn post_dlc(save: &TmpDlc, user: &str, pool: &Pool<MySql>, kphis: &str) -> Result<ExecuteResponse, AppError> {
    let result = if save.dlc_id > 0 {
        let update_result = update_dlc(save, user, pool, kphis).await?;
        ExecuteResponse::from_query_result(update_result, "Update DailyCare")
    } else {
        let insert_result = insert_dlc(save, user, pool, kphis).await?;
        ExecuteResponse::from_query_result(insert_result, "Insert DailyCare")
    };

    Ok(result)
}

async fn update_dlc(save: &TmpDlc, user: &str, pool: &Pool<MySql>, kphis: &str) -> Result<MySqlQueryResult, AppError> {
    let update_sql = tmp::update_dlc(kphis);
    sqlx::query(AssertSqlSafe(update_sql))
        .bind(&save.dlc_name)
        .bind(user)
        .bind(save.dlc_id)
        .execute(pool)
        .await
        .map_err(|e| Source::SQLx.to_error(500, e, "Update DailyCare"))
}

async fn insert_dlc(save: &TmpDlc, user: &str, pool: &Pool<MySql>, kphis: &str) -> Result<MySqlQueryResult, AppError> {
    let insert_sql = tmp::insert_dlc(kphis);
    sqlx::query(AssertSqlSafe(insert_sql))
        .bind(&save.dlc_name)
        .bind(user)
        .bind(user)
        .execute(pool)
        .await
        .map_err(|e| Source::SQLx.to_error(500, e, "Insert DailyCare"))
}

pub async fn delete_dlc(params: &TmpParams, user: &str, pool: &Pool<MySql>, kphis: &str) -> Result<ExecuteResponse, AppError> {
    let sql = tmp::delete_dlc(kphis);
    let result = sqlx::query(AssertSqlSafe(sql))
        .bind(user)
        .bind(params.id)
        .bind(params.id)
        .bind(params.id)
        .execute(pool)
        .await
        .map_err(|e| Source::SQLx.to_error(500, e, "Delete DailyCare"))?;

    Ok(ExecuteResponse::from_query_result(result, "Delete DailyCare"))
}

#[cfg(test)]
#[rustfmt::skip]
mod tests {

    use super::*;
    use kphis_sqlx_tester::MySqlTester;

    #[tokio::test]
    #[ignore]
    async fn sqlx_get_group() {
        let tester = MySqlTester::new_kphis().await;
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis/ipd_tmp_group_smp.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/kphis/ipd_tmp_group_smp.sql")).execute(&tester.db_pool).await.unwrap();

        let default = get_group(&TmpParams::default(), &tester.db_pool, &tester.kphis).await.unwrap();
        assert_eq!(default.len(), 3);
        let found = get_group(&TmpParams {smp_id: Some(1),..Default::default()},&tester.db_pool,&tester.kphis).await.unwrap();
        assert_eq!(found.len(), 1);
        let not_found = get_group(&TmpParams {smp_id: Some(888),..Default::default()},&tester.db_pool,&tester.kphis).await.unwrap();
        assert!(not_found.is_empty());
    }

    #[tokio::test]
    #[ignore]
    async fn sqlx_get_subgroup() {
        let tester = MySqlTester::new_kphis().await;
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis/ipd_tmp_subgroup.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/kphis/ipd_tmp_subgroup.sql")).execute(&tester.db_pool).await.unwrap();

        let default = get_subgroup(&TmpParams::default(), &tester.db_pool, &tester.kphis).await.unwrap();
        assert_eq!(default.len(), 2);
        let found_smp_id = get_subgroup(&TmpParams {smp_id: Some(1),..Default::default()},&tester.db_pool,&tester.kphis).await.unwrap();
        assert_eq!(found_smp_id.len(), 1);
        let found_subgroup = get_subgroup(&TmpParams {subgroup: Some(1),..Default::default()},&tester.db_pool,&tester.kphis).await.unwrap();
        assert_eq!(found_subgroup.len(), 2);
        let not_found = get_subgroup(&TmpParams {smp_id: Some(888),..Default::default()},&tester.db_pool,&tester.kphis).await.unwrap();
        assert!(not_found.is_empty());
    }

    #[tokio::test]
    #[ignore]
    async fn sqlx_get_focus() {
        let tester = MySqlTester::new_kphis().await;
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis/ipd_tmp_focus.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/kphis/ipd_tmp_focus.sql")).execute(&tester.db_pool).await.unwrap();

        let default = get_focus(&TmpParams::default(), &tester.db_pool, &tester.kphis).await.unwrap();
        assert_eq!(default.len(), 3);
        let found_smp_id = get_focus(&TmpParams {smp_id: Some(1),..Default::default()},&tester.db_pool,&tester.kphis).await.unwrap();
        assert_eq!(found_smp_id.len(), 2);
        let found_subgroup = get_focus(&TmpParams {subgroup: Some(1),..Default::default()},&tester.db_pool,&tester.kphis).await.unwrap();
        assert_eq!(found_subgroup.len(), 3);
        let found_subgroup_strict = get_focus(&TmpParams {subgroup: Some(1),strict: Some(true),..Default::default()},&tester.db_pool,&tester.kphis).await.unwrap();
        assert_eq!(found_subgroup_strict.len(), 2);
        let found_id = get_focus(&TmpParams {id: Some(1),..Default::default()},&tester.db_pool,&tester.kphis).await.unwrap();
        assert_eq!(found_id.len(), 1);
        let not_found = get_focus(&TmpParams {smp_id: Some(888),..Default::default()},&tester.db_pool,&tester.kphis).await.unwrap();
        assert!(not_found.is_empty());
    }

    #[tokio::test]
    #[ignore]
    async fn sqlx_get_goal() {
        let tester = MySqlTester::new_kphis().await;
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis/ipd_tmp_goal.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/kphis/ipd_tmp_goal.sql")).execute(&tester.db_pool).await.unwrap();

        let default = get_goal(&TmpParams::default(), &tester.db_pool, &tester.kphis).await.unwrap();
        assert_eq!(default.len(), 3);
        let found_smp_id = get_goal(&TmpParams {smp_id: Some(1),..Default::default()},&tester.db_pool,&tester.kphis).await.unwrap();
        assert_eq!(found_smp_id.len(), 2);
        let found_subgroup = get_goal(&TmpParams {subgroup: Some(1),..Default::default()},&tester.db_pool,&tester.kphis).await.unwrap();
        assert_eq!(found_subgroup.len(), 3);
        let found_subgroup_strict = get_goal(&TmpParams {subgroup: Some(1),strict: Some(true),..Default::default()},&tester.db_pool,&tester.kphis).await.unwrap();
        assert_eq!(found_subgroup_strict.len(), 2);
        let found_id = get_goal(&TmpParams {id: Some(1),..Default::default()},&tester.db_pool,&tester.kphis).await.unwrap();
        assert_eq!(found_id.len(), 1);
        let not_found = get_goal(&TmpParams {smp_id: Some(888),..Default::default()},&tester.db_pool,&tester.kphis).await.unwrap();
        assert!(not_found.is_empty());
    }

    #[tokio::test]
    #[ignore]
    async fn sqlx_get_intvt() {
        let tester = MySqlTester::new_kphis().await;
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis/ipd_tmp_intvt.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/kphis/ipd_tmp_intvt.sql")).execute(&tester.db_pool).await.unwrap();

        let default = get_intvt(&TmpParams::default(), &tester.db_pool, &tester.kphis).await.unwrap();
        assert_eq!(default.len(), 3);
        let found_smp_id = get_intvt(&TmpParams {smp_id: Some(1),..Default::default()},&tester.db_pool,&tester.kphis).await.unwrap();
        assert_eq!(found_smp_id.len(), 2);
        let found_subgroup = get_intvt(&TmpParams {subgroup: Some(1),..Default::default()},&tester.db_pool,&tester.kphis).await.unwrap();
        assert_eq!(found_subgroup.len(), 3);
        let found_subgroup_strict = get_intvt(&TmpParams {subgroup: Some(1),strict: Some(true),..Default::default()},&tester.db_pool,&tester.kphis).await.unwrap();
        assert_eq!(found_subgroup_strict.len(), 2);
        let found_id = get_intvt(&TmpParams {id: Some(1),..Default::default()},&tester.db_pool,&tester.kphis).await.unwrap();
        assert_eq!(found_id.len(), 1);
        let not_found = get_intvt(&TmpParams {smp_id: Some(888),..Default::default()},&tester.db_pool,&tester.kphis).await.unwrap();
        assert!(not_found.is_empty());
    }

    #[tokio::test]
    #[ignore]
    async fn sqlx_get_dlc() {
        let tester = MySqlTester::new_kphis().await;
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis/ipd_tmp_dlc.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/kphis/ipd_tmp_dlc.sql")).execute(&tester.db_pool).await.unwrap();

        let default = get_dlc(&TmpParams::default(), &tester.db_pool, &tester.kphis).await.unwrap();
        assert_eq!(default.len(), 2);
        let found_id = get_dlc(&TmpParams {id: Some(1),..Default::default()},&tester.db_pool,&tester.kphis).await.unwrap();
        assert_eq!(found_id.len(), 1);
        let not_found = get_dlc(&TmpParams {id: Some(888),..Default::default()},&tester.db_pool,&tester.kphis).await.unwrap();
        assert!(not_found.is_empty());
    }

    #[tokio::test]
    #[ignore]
    async fn sqlx_insert_group() {
        let tester = MySqlTester::new_kphis().await;
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis/ipd_tmp_group_smp.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/kphis/ipd_tmp_group_smp.sql")).execute(&tester.db_pool).await.unwrap();

        let success = insert_group(&TmpGroup::demo(), "user", &tester.db_pool, &tester.kphis).await.unwrap();
        assert_eq!(success, Some(98));
        let again_success = insert_group(&TmpGroup::demo(), "user", &tester.db_pool, &tester.kphis).await.unwrap();
        assert_eq!(again_success, Some(100));
    }

    #[tokio::test]
    #[ignore]
    async fn sqlx_insert_subgroup() {
        let tester = MySqlTester::new_kphis().await;
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis/ipd_tmp_subgroup.sql")).execute(&tester.db_pool).await.unwrap();

        let success = insert_subgroup(&TmpSubGroup::demo(), "user", &tester.db_pool, &tester.kphis).await.unwrap();
        assert!(success.is_some());
        let again_success = insert_subgroup(&TmpSubGroup::demo(), "user", &tester.db_pool, &tester.kphis).await.unwrap();
        assert!(again_success.is_some());
    }

    #[tokio::test]
    #[ignore]
    async fn sqlx_insert_focus() {
        let tester = MySqlTester::new_kphis().await;
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis/ipd_tmp_focus.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/kphis/ipd_tmp_focus.sql")).execute(&tester.db_pool).await.unwrap();

        let success = insert_focus(&TmpFocus::demo(), "user", &tester.db_pool, &tester.kphis).await.unwrap();
        assert_eq!(success, Some(998));
        let again_success = insert_focus(&TmpFocus::demo(), "user", &tester.db_pool, &tester.kphis).await.unwrap();
        assert_eq!(again_success, Some(1000));
    }

    #[tokio::test]
    #[ignore]
    async fn sqlx_insert_goal() {
        let tester = MySqlTester::new_kphis().await;
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis/ipd_tmp_goal.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/kphis/ipd_tmp_goal.sql")).execute(&tester.db_pool).await.unwrap();

        let success = insert_goal(&TmpGoal::demo(), "user", &tester.db_pool, &tester.kphis).await.unwrap();
        assert_eq!(success, Some(998));
        let again_success = insert_goal(&TmpGoal::demo(), "user", &tester.db_pool, &tester.kphis).await.unwrap();
        assert_eq!(again_success, Some(1000));
    }

    #[tokio::test]
    #[ignore]
    async fn sqlx_insert_intvt() {
        let tester = MySqlTester::new_kphis().await;
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis/ipd_tmp_intvt.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/kphis/ipd_tmp_intvt.sql")).execute(&tester.db_pool).await.unwrap();

        let success = insert_intvt(&TmpIntvt::demo(), "user", &tester.db_pool, &tester.kphis).await.unwrap();
        assert_eq!(success, Some(9998));
        let again_success = insert_intvt(&TmpIntvt::demo(), "user", &tester.db_pool, &tester.kphis).await.unwrap();
        assert_eq!(again_success, Some(10000));
    }

    #[tokio::test]
    #[ignore]
    async fn sqlx_insert_dlc() {
        let tester = MySqlTester::new_kphis().await;
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis/ipd_tmp_dlc.sql")).execute(&tester.db_pool).await.unwrap();

        let success = insert_dlc(&TmpDlc::demo(), "user", &tester.db_pool, &tester.kphis).await.unwrap();
        assert_eq!(success.rows_affected(), 1);
        let again_success = insert_dlc(&TmpDlc::demo(), "user", &tester.db_pool, &tester.kphis).await.unwrap();
        assert_eq!(again_success.rows_affected(), 1);
    }

    #[tokio::test]
    #[ignore]
    async fn sqlx_update_group() {
        let tester = MySqlTester::new_kphis().await;
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis/ipd_tmp_group_smp.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/kphis/ipd_tmp_group_smp.sql")).execute(&tester.db_pool).await.unwrap();

        let success = update_group(&TmpGroup::demo(), "user", &tester.db_pool, &tester.kphis).await.unwrap();
        assert_eq!(success.rows_affected(), 1);
        let again_success = update_group(&TmpGroup::demo(), "user", &tester.db_pool, &tester.kphis).await.unwrap();
        assert_eq!(again_success.rows_affected(), 1);
    }

    #[tokio::test]
    #[ignore]
    async fn sqlx_update_subgroup() {
        let tester = MySqlTester::new_kphis().await;
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis/ipd_tmp_subgroup.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/kphis/ipd_tmp_subgroup.sql")).execute(&tester.db_pool).await.unwrap();

        let success = update_subgroup(&TmpSubGroup::demo(), "user", &tester.db_pool, &tester.kphis).await.unwrap();
        assert_eq!(success.rows_affected(), 1);
        let again_success = update_subgroup(&TmpSubGroup::demo(), "user", &tester.db_pool, &tester.kphis).await.unwrap();
        assert_eq!(again_success.rows_affected(), 1);
    }

    #[tokio::test]
    #[ignore]
    async fn sqlx_update_focus() {
        let tester = MySqlTester::new_kphis().await;
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis/ipd_tmp_focus.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/kphis/ipd_tmp_focus.sql")).execute(&tester.db_pool).await.unwrap();

        let success = update_focus(&TmpFocus::demo(), "user", &tester.db_pool, &tester.kphis).await.unwrap();
        assert_eq!(success.rows_affected(), 1);
        let again_success = update_focus(&TmpFocus::demo(), "user", &tester.db_pool, &tester.kphis).await.unwrap();
        assert_eq!(again_success.rows_affected(), 1);
    }

    #[tokio::test]
    #[ignore]
    async fn sqlx_update_goal() {
        let tester = MySqlTester::new_kphis().await;
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis/ipd_tmp_goal.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/kphis/ipd_tmp_goal.sql")).execute(&tester.db_pool).await.unwrap();

        let success = update_goal(&TmpGoal::demo(), "user", &tester.db_pool, &tester.kphis).await.unwrap();
        assert_eq!(success.rows_affected(), 1);
        let again_success = update_goal(&TmpGoal::demo(), "user", &tester.db_pool, &tester.kphis).await.unwrap();
        assert_eq!(again_success.rows_affected(), 1);
    }

    #[tokio::test]
    #[ignore]
    async fn sqlx_update_intvt() {
        let tester = MySqlTester::new_kphis().await;
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis/ipd_tmp_intvt.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/kphis/ipd_tmp_intvt.sql")).execute(&tester.db_pool).await.unwrap();

        let success = update_intvt(&TmpIntvt::demo(), "user", &tester.db_pool, &tester.kphis).await.unwrap();
        assert_eq!(success.rows_affected(), 1);
        let again_success = update_intvt(&TmpIntvt::demo(), "user", &tester.db_pool, &tester.kphis).await.unwrap();
        assert_eq!(again_success.rows_affected(), 1);
    }

    #[tokio::test]
    #[ignore]
    async fn sqlx_update_dlc() {
        let tester = MySqlTester::new_kphis().await;
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis/ipd_tmp_dlc.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/kphis/ipd_tmp_dlc.sql")).execute(&tester.db_pool).await.unwrap();

        let success = update_dlc(&TmpDlc::demo(), "user", &tester.db_pool, &tester.kphis).await.unwrap();
        assert_eq!(success.rows_affected(), 1);
        let again_success = update_dlc(&TmpDlc::demo(), "user", &tester.db_pool, &tester.kphis).await.unwrap();
        assert_eq!(again_success.rows_affected(), 1);
    }

    #[tokio::test]
    #[ignore]
    async fn sqlx_delete_group() {
        let tester = MySqlTester::new_kphis().await;
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis/ipd_tmp_group_smp.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis/ipd_focus_list.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis/opd_er_focus_list.sql")).execute(&tester.db_pool).await.unwrap();

        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/kphis/ipd_tmp_group_smp.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/kphis/ipd_focus_list.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/kphis/opd_er_focus_list.sql")).execute(&tester.db_pool).await.unwrap();

        let default = delete_group(&TmpParams::default(),"cha582",&tester.db_pool,&tester.kphis).await.unwrap();
        assert_eq!(default.rows_affected, 0);
        let used = delete_group(&TmpParams {smp_id: Some(1),..Default::default()},"cha582",&tester.db_pool,&tester.kphis).await.unwrap();
        assert_eq!(used.rows_affected, 0);
        let success = delete_group(&TmpParams {smp_id: Some(99),..Default::default()},"cha582",&tester.db_pool,&tester.kphis).await.unwrap();
        assert_eq!(success.rows_affected, 1);
        let again_not_found = delete_group(&TmpParams {smp_id: Some(99),..Default::default()},"cha582",&tester.db_pool,&tester.kphis).await.unwrap();
        assert_eq!(again_not_found.rows_affected, 0);
    }

    #[tokio::test]
    #[ignore]
    async fn sqlx_delete_subgroup() {
        let tester = MySqlTester::new_kphis().await;
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis/ipd_tmp_subgroup.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis/ipd_focus_list.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis/opd_er_focus_list.sql")).execute(&tester.db_pool).await.unwrap();

        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/kphis/ipd_tmp_subgroup.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/kphis/ipd_focus_list.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/kphis/opd_er_focus_list.sql")).execute(&tester.db_pool).await.unwrap();

        let default = delete_subgroup(&TmpParams::default(),"cha582",&tester.db_pool,&tester.kphis).await.unwrap();
        assert_eq!(default.rows_affected, 0);
        let used = delete_subgroup(&TmpParams {smp_id: Some(1),..Default::default()},"cha582",&tester.db_pool,&tester.kphis).await.unwrap();
        assert_eq!(used.rows_affected, 0);
        let success = delete_subgroup(&TmpParams {smp_id: Some(99),subgroup: Some(1),..Default::default()},"cha582",&tester.db_pool,&tester.kphis).await.unwrap();
        assert_eq!(success.rows_affected, 1);
        let again_not_found = delete_subgroup(&TmpParams {smp_id: Some(99),subgroup: Some(1),..Default::default()},"cha582",&tester.db_pool,&tester.kphis).await.unwrap();
        assert_eq!(again_not_found.rows_affected, 0);
    }

    #[tokio::test]
    #[ignore]
    async fn sqlx_delete_focus() {
        let tester = MySqlTester::new_kphis().await;
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis/ipd_tmp_focus.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis/ipd_focus_list.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis/opd_er_focus_list.sql")).execute(&tester.db_pool).await.unwrap();

        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/kphis/ipd_tmp_focus.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/kphis/ipd_focus_list.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/kphis/opd_er_focus_list.sql")).execute(&tester.db_pool).await.unwrap();

        let default = delete_focus(&TmpParams::default(),"cha582",&tester.db_pool,&tester.kphis).await.unwrap();
        assert_eq!(default.rows_affected, 0);
        let used = delete_focus(&TmpParams {id: Some(1),..Default::default()},"cha582",&tester.db_pool,&tester.kphis).await.unwrap();
        assert_eq!(used.rows_affected, 0);
        let success = delete_focus(&TmpParams {id: Some(999),..Default::default()},"cha582",&tester.db_pool,&tester.kphis).await.unwrap();
        assert_eq!(success.rows_affected, 1);
        let again_not_found = delete_focus(&TmpParams {id: Some(999),..Default::default()},"cha582",&tester.db_pool,&tester.kphis).await.unwrap();
        assert_eq!(again_not_found.rows_affected, 0);
    }

    #[tokio::test]
    #[ignore]
    async fn sqlx_delete_goal() {
        let tester = MySqlTester::new_kphis().await;
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis/ipd_tmp_goal.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis/ipd_focus_list_goal_item.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis/opd_er_focus_list_goal_item.sql")).execute(&tester.db_pool).await.unwrap();

        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/kphis/ipd_tmp_goal.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/kphis/ipd_focus_list_goal_item.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/kphis/opd_er_focus_list_goal_item.sql")).execute(&tester.db_pool).await.unwrap();

        let default = delete_goal(&TmpParams::default(),"cha582",&tester.db_pool,&tester.kphis).await.unwrap();
        assert_eq!(default.rows_affected, 0);
        let used = delete_goal(&TmpParams {id: Some(1),..Default::default()},"cha582",&tester.db_pool,&tester.kphis).await.unwrap();
        assert_eq!(used.rows_affected, 0);
        let success = delete_goal(&TmpParams {id: Some(999),..Default::default()},"cha582",&tester.db_pool,&tester.kphis).await.unwrap();
        assert_eq!(success.rows_affected, 1);
        let again_not_found = delete_goal(&TmpParams {id: Some(999),..Default::default()},"cha582",&tester.db_pool,&tester.kphis).await.unwrap();
        assert_eq!(again_not_found.rows_affected, 0);
    }

    #[tokio::test]
    #[ignore]
    async fn sqlx_delete_intvt() {
        let tester = MySqlTester::new_kphis().await;
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis/ipd_tmp_intvt.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis/ipd_focus_note_intvt_item.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis/opd_er_focus_note_intvt_item.sql")).execute(&tester.db_pool).await.unwrap();

        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/kphis/ipd_tmp_intvt.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/kphis/ipd_focus_note_intvt_item.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/kphis/opd_er_focus_note_intvt_item.sql")).execute(&tester.db_pool).await.unwrap();

        let default = delete_intvt(&TmpParams::default(),"cha582",&tester.db_pool,&tester.kphis).await.unwrap();
        assert_eq!(default.rows_affected, 0);
        let used = delete_intvt(&TmpParams {id: Some(1),..Default::default()},"cha582",&tester.db_pool,&tester.kphis).await.unwrap();
        assert_eq!(used.rows_affected, 0);
        let success = delete_intvt(&TmpParams {id: Some(9999),..Default::default()},"cha582",&tester.db_pool,&tester.kphis).await.unwrap();
        assert_eq!(success.rows_affected, 1);
        let again_not_found = delete_intvt(&TmpParams {id: Some(9999),..Default::default()},"cha582",&tester.db_pool,&tester.kphis).await.unwrap();
        assert_eq!(again_not_found.rows_affected, 0);
    }

    #[tokio::test]
    #[ignore]
    async fn sqlx_delete_dlc() {
        let tester = MySqlTester::new_kphis().await;
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis/ipd_tmp_dlc.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis/ipd_focus_note_dlc_item.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis/opd_er_focus_note_dlc_item.sql")).execute(&tester.db_pool).await.unwrap();

        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/kphis/ipd_tmp_dlc.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/kphis/ipd_focus_note_dlc_item.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/kphis/opd_er_focus_note_dlc_item.sql")).execute(&tester.db_pool).await.unwrap();

        let default = delete_dlc(&TmpParams::default(),"cha582",&tester.db_pool,&tester.kphis).await.unwrap();
        assert_eq!(default.rows_affected, 0);
        let used = delete_dlc(&TmpParams {id: Some(1),..Default::default()},"cha582",&tester.db_pool,&tester.kphis).await.unwrap();
        assert_eq!(used.rows_affected, 0);
        let success = delete_dlc(&TmpParams {id: Some(2),..Default::default()},"cha582",&tester.db_pool,&tester.kphis).await.unwrap();
        assert_eq!(success.rows_affected, 1);
        let again_not_found = delete_dlc(&TmpParams {id: Some(2),..Default::default()},"cha582",&tester.db_pool,&tester.kphis).await.unwrap();
        assert_eq!(again_not_found.rows_affected, 0);
    }
}
