use sqlx::{AssertSqlSafe, FromRow, MySql, Pool, mysql::MySqlQueryResult};

use kphis_model::{
    dc_plan::{DischargePlan, DischargePlanDietItemOnly, DischargePlanEnvItemOnly, DischargePlanMedItemOnly, DischargePlanOnly, DischargePlanSave, DischargePlanTxItemOnly},
    fetch::ExecuteResponse,
};
use kphis_sql::{TABLE_UPDATE_SET, opd_er::dc_plan};
use kphis_util::error::{AppError, Source};

pub async fn get_dc_plan(opd_er_order_master_id: u32, pool: &Pool<MySql>, hosxp: &str, kphis_extra: &str) -> Result<Vec<DischargePlan>, AppError> {
    let sql = dc_plan::select_dc_plan(hosxp, kphis_extra);
    sqlx::query(AssertSqlSafe(sql))
        .bind(opd_er_order_master_id)
        .fetch_all(pool)
        .await
        .map_err(|e| Source::SQLx.to_error(500, e, "Select DischargePlan"))?
        .iter()
        .map(DischargePlan::from_row)
        .collect::<sqlx::Result<Vec<DischargePlan>>>()
        .map_err(|e| Source::SQLx.to_error(500, e, "Select DischargePlan"))
}

pub async fn get_dc_plan_only_bundle(opd_er_order_master_id: u32, pool: &Pool<MySql>, kphis_extra: &str) -> Result<Vec<DischargePlanOnly>, AppError> {
    let mut dc_plans = get_dc_plan_only(opd_er_order_master_id, pool, kphis_extra).await?;
    for dc_plan in dc_plans.iter_mut() {
        let med_items = get_dc_plan_med_item_only(dc_plan.dc_plan_id, pool, kphis_extra).await?;
        dc_plan.dc_plan_med_items = med_items;
        let env_items = get_dc_plan_env_item_only(dc_plan.dc_plan_id, pool, kphis_extra).await?;
        dc_plan.dc_plan_env_items = env_items;
        let tx_items = get_dc_plan_tx_item_only(dc_plan.dc_plan_id, pool, kphis_extra).await?;
        dc_plan.dc_plan_tx_items = tx_items;
        let diet_items = get_dc_plan_diet_item_only(dc_plan.dc_plan_id, pool, kphis_extra).await?;
        dc_plan.dc_plan_diet_items = diet_items;
    }

    Ok(dc_plans)
}

async fn get_dc_plan_only(opd_er_order_master_id: u32, pool: &Pool<MySql>, kphis_extra: &str) -> Result<Vec<DischargePlanOnly>, AppError> {
    let sql = dc_plan::select_dc_plan_only(kphis_extra);
    sqlx::query(AssertSqlSafe(sql))
        .bind(opd_er_order_master_id)
        .fetch_all(pool)
        .await
        .map_err(|e| Source::SQLx.to_error(500, e, "Select DischargePlanOnly"))?
        .iter()
        .map(DischargePlanOnly::from_row)
        .collect::<sqlx::Result<Vec<DischargePlanOnly>>>()
        .map_err(|e| Source::SQLx.to_error(500, e, "Select DischargePlanOnly"))
}

async fn get_dc_plan_med_item_only(dc_plan_id: u32, pool: &Pool<MySql>, kphis_extra: &str) -> Result<Vec<DischargePlanMedItemOnly>, AppError> {
    let sql = dc_plan::select_dc_plan_med_item_only(kphis_extra);
    sqlx::query(AssertSqlSafe(sql))
        .bind(dc_plan_id)
        .fetch_all(pool)
        .await
        .map_err(|e| Source::SQLx.to_error(500, e, "Select DischargePlanMedItemOnly"))?
        .iter()
        .map(DischargePlanMedItemOnly::from_row)
        .collect::<sqlx::Result<Vec<DischargePlanMedItemOnly>>>()
        .map_err(|e| Source::SQLx.to_error(500, e, "Select DischargePlanMedItemOnly"))
}

async fn get_dc_plan_env_item_only(dc_plan_id: u32, pool: &Pool<MySql>, kphis_extra: &str) -> Result<Vec<DischargePlanEnvItemOnly>, AppError> {
    let sql = dc_plan::select_dc_plan_env_item_only(kphis_extra);
    sqlx::query(AssertSqlSafe(sql))
        .bind(dc_plan_id)
        .fetch_all(pool)
        .await
        .map_err(|e| Source::SQLx.to_error(500, e, "Select DischargePlanEnvItemOnly"))?
        .iter()
        .map(DischargePlanEnvItemOnly::from_row)
        .collect::<sqlx::Result<Vec<DischargePlanEnvItemOnly>>>()
        .map_err(|e| Source::SQLx.to_error(500, e, "Select DischargePlanEnvItemOnly"))
}

async fn get_dc_plan_tx_item_only(dc_plan_id: u32, pool: &Pool<MySql>, kphis_extra: &str) -> Result<Vec<DischargePlanTxItemOnly>, AppError> {
    let sql = dc_plan::select_dc_plan_tx_item_only(kphis_extra);
    sqlx::query(AssertSqlSafe(sql))
        .bind(dc_plan_id)
        .fetch_all(pool)
        .await
        .map_err(|e| Source::SQLx.to_error(500, e, "Select DischargePlanTxItemOnly"))?
        .iter()
        .map(DischargePlanTxItemOnly::from_row)
        .collect::<sqlx::Result<Vec<DischargePlanTxItemOnly>>>()
        .map_err(|e| Source::SQLx.to_error(500, e, "Select DischargePlanTxItemOnly"))
}

async fn get_dc_plan_diet_item_only(dc_plan_id: u32, pool: &Pool<MySql>, kphis_extra: &str) -> Result<Vec<DischargePlanDietItemOnly>, AppError> {
    let sql = dc_plan::select_dc_plan_diet_item_only(kphis_extra);
    sqlx::query(AssertSqlSafe(sql))
        .bind(dc_plan_id)
        .fetch_all(pool)
        .await
        .map_err(|e| Source::SQLx.to_error(500, e, "Select DischargePlanDietItemOnly"))?
        .iter()
        .map(DischargePlanDietItemOnly::from_row)
        .collect::<sqlx::Result<Vec<DischargePlanDietItemOnly>>>()
        .map_err(|e| Source::SQLx.to_error(500, e, "Select DischargePlanDietItemOnly"))
}

pub async fn post_dc_plan(opd_er_order_master_id: u32, save: &DischargePlanSave, user: &str, pool: &Pool<MySql>, kphis_extra: &str) -> Result<(u32, Vec<ExecuteResponse>), AppError> {
    if save.dc_plan_id > 0 {
        let results = update_dc_plan_bundle(save.dc_plan_id, opd_er_order_master_id, save, user, pool, kphis_extra).await?;

        Ok((save.dc_plan_id, results))
    } else {
        insert_dc_plan_bundle(opd_er_order_master_id, save, user, pool, kphis_extra).await
    }
}

pub async fn update_dc_plan_bundle(
    dc_plan_id: u32,
    opd_er_order_master_id: u32,
    save: &DischargePlanSave,
    user: &str,
    pool: &Pool<MySql>,
    kphis_extra: &str,
) -> Result<Vec<ExecuteResponse>, AppError> {
    let mut results = Vec::with_capacity(9);

    let update_dc_plan_result = update_dc_plan(opd_er_order_master_id, save, user, pool, kphis_extra).await?;
    let is_updated = update_dc_plan_result.rows_affected() > 0;
    results.push(ExecuteResponse::from_query_result(update_dc_plan_result, "Update DcPlan"));

    if is_updated {
        let delete_med_item_result = delete_med_item(dc_plan_id, save.version, pool, kphis_extra).await?;
        results.push(ExecuteResponse::from_query_result(delete_med_item_result, "Delete DcPlanMedItem"));
        if !save.med_ids.is_empty() {
            let insert_med_item_result = insert_med_items(dc_plan_id, &save.med_ids, save.version + 1, user, pool, kphis_extra).await?;
            results.push(ExecuteResponse::from_query_result(insert_med_item_result, "Insert DcPlanMedItem"));
        }

        let delete_env_item_result = delete_env_item(dc_plan_id, save.version, pool, kphis_extra).await?;
        results.push(ExecuteResponse::from_query_result(delete_env_item_result, "Delete DcPlanEnvItem"));
        if !save.env_ids.is_empty() {
            let insert_env_item_result = insert_env_items(dc_plan_id, &save.env_ids, save.version + 1, user, pool, kphis_extra).await?;
            results.push(ExecuteResponse::from_query_result(insert_env_item_result, "Insert DcPlanEnvItem"));
        }

        let delete_tx_item_result = delete_tx_item(dc_plan_id, save.version, pool, kphis_extra).await?;
        results.push(ExecuteResponse::from_query_result(delete_tx_item_result, "Delete DcPlanTxItem"));
        if !save.tx_ids.is_empty() {
            let insert_tx_item_result = insert_tx_items(dc_plan_id, &save.tx_ids, save.version + 1, user, pool, kphis_extra).await?;
            results.push(ExecuteResponse::from_query_result(insert_tx_item_result, "Insert DcPlanTxItem"));
        }

        let delete_diet_item_result = delete_diet_item(dc_plan_id, save.version, pool, kphis_extra).await?;
        results.push(ExecuteResponse::from_query_result(delete_diet_item_result, "Delete DcPlanDietItem"));
        if !save.diet_ids.is_empty() {
            let insert_diet_item_result = insert_diet_items(dc_plan_id, &save.diet_ids, save.version + 1, user, pool, kphis_extra).await?;
            results.push(ExecuteResponse::from_query_result(insert_diet_item_result, "Insert DcPlanDietItem"));
        }
    }

    Ok(results)
}

pub async fn insert_dc_plan_bundle(opd_er_order_master_id: u32, save: &DischargePlanSave, user: &str, pool: &Pool<MySql>, kphis_extra: &str) -> Result<(u32, Vec<ExecuteResponse>), AppError> {
    let mut results = Vec::with_capacity(5);

    let insert_dc_plan_result = insert_dc_plan(opd_er_order_master_id, save, user, pool, kphis_extra).await?;
    let dc_plan_id = insert_dc_plan_result.last_insert_id() as u32;
    results.push(ExecuteResponse::from_query_result(insert_dc_plan_result, "Insert DcPlan"));

    if !save.med_ids.is_empty() {
        let insert_med_item_result = insert_med_items(dc_plan_id, &save.med_ids, 1, user, pool, kphis_extra).await?;
        results.push(ExecuteResponse::from_query_result(insert_med_item_result, "Insert DcPlanMedItem"));
    }
    if !save.env_ids.is_empty() {
        let insert_env_item_result = insert_env_items(dc_plan_id, &save.env_ids, 1, user, pool, kphis_extra).await?;
        results.push(ExecuteResponse::from_query_result(insert_env_item_result, "Insert DcPlanEnvItem"));
    }
    if !save.tx_ids.is_empty() {
        let insert_tx_item_result = insert_tx_items(dc_plan_id, &save.tx_ids, 1, user, pool, kphis_extra).await?;
        results.push(ExecuteResponse::from_query_result(insert_tx_item_result, "Insert DcPlanTxItem"));
    }
    if !save.diet_ids.is_empty() {
        let insert_diet_item_result = insert_diet_items(dc_plan_id, &save.diet_ids, 1, user, pool, kphis_extra).await?;
        results.push(ExecuteResponse::from_query_result(insert_diet_item_result, "Insert DcPlanDietItem"));
    }

    Ok((dc_plan_id, results))
}

pub async fn insert_dc_plan_only_bundle(opd_er_order_master_id: u32, only: &DischargePlanOnly, pool: &Pool<MySql>, kphis_extra: &str) -> Result<(u32, Vec<ExecuteResponse>), AppError> {
    let mut results = Vec::with_capacity(5);

    let insert_dc_plan_result = insert_dc_plan_only(opd_er_order_master_id, only, pool, kphis_extra).await?;
    let dc_plan_id = insert_dc_plan_result.last_insert_id() as u32;
    results.push(ExecuteResponse::from_query_result(insert_dc_plan_result, "Insert DcPlanOnly"));

    if !only.dc_plan_med_items.is_empty() {
        let insert_med_item_result = insert_med_items_only(dc_plan_id, &only.dc_plan_med_items, pool, kphis_extra).await?;
        results.push(ExecuteResponse::from_query_result(insert_med_item_result, "Insert DcPlanMedItemOnly"));
    }
    if !only.dc_plan_env_items.is_empty() {
        let insert_env_item_result = insert_env_items_only(dc_plan_id, &only.dc_plan_env_items, pool, kphis_extra).await?;
        results.push(ExecuteResponse::from_query_result(insert_env_item_result, "Insert DcPlanEnvItemOnly"));
    }
    if !only.dc_plan_tx_items.is_empty() {
        let insert_tx_item_result = insert_tx_items_only(dc_plan_id, &only.dc_plan_tx_items, pool, kphis_extra).await?;
        results.push(ExecuteResponse::from_query_result(insert_tx_item_result, "Insert DcPlanTxItemOnly"));
    }
    if !only.dc_plan_diet_items.is_empty() {
        let insert_diet_item_result = insert_diet_items_only(dc_plan_id, &only.dc_plan_diet_items, pool, kphis_extra).await?;
        results.push(ExecuteResponse::from_query_result(insert_diet_item_result, "Insert DcPlanDietItemOnly"));
    }

    Ok((dc_plan_id, results))
}

pub async fn insert_dc_plan(opd_er_order_master_id: u32, form: &DischargePlanSave, user: &str, pool: &Pool<MySql>, kphis_extra: &str) -> Result<MySqlQueryResult, AppError> {
    form.insert(
        Some("dc_plan_id"),
        Some("opd_er_dc_plan"),
        ",opd_er_order_master_id,create_user,create_datetime,update_user,update_datetime,version",
        ",?,?,NOW(),?,NOW(),1",
        &[&opd_er_order_master_id.to_string(), user, user],
        pool,
        kphis_extra,
    )
    .await
    .map_err(|e| {
        if let sqlx::error::Error::Database(err) = &e
            && err.code().map(|c| c == "23000").unwrap_or_default()
        {
            AppError::app_403_duplicate("Insert OpdErDischargePlan")
        } else {
            Source::SQLx.to_error(500, e, "Insert OpdErDischargePlan")
        }
    })
}

pub async fn insert_dc_plan_only(opd_er_order_master_id: u32, only: &DischargePlanOnly, pool: &Pool<MySql>, kphis_extra: &str) -> Result<MySqlQueryResult, AppError> {
    only.insert(
        Some("dc_plan_id"),
        Some("opd_er_dc_plan"),
        ",opd_er_order_master_id",
        ",?",
        &[&opd_er_order_master_id.to_string()],
        pool,
        kphis_extra,
    )
    .await
    .map_err(|e| {
        if let sqlx::error::Error::Database(err) = &e
            && err.code().map(|c| c == "23000").unwrap_or_default()
        {
            AppError::app_403_duplicate("Insert OpdErDischargePlanOnly")
        } else {
            Source::SQLx.to_error(500, e, "Insert OpdErDischargePlanOnly")
        }
    })
}

pub async fn update_dc_plan(opd_er_order_master_id: u32, form: &DischargePlanSave, user: &str, pool: &Pool<MySql>, kphis_extra: &str) -> Result<MySqlQueryResult, AppError> {
    form.update(
        "dc_plan_id",
        Some("opd_er_dc_plan"),
        &[",opd_er_order_master_id=?", TABLE_UPDATE_SET].concat(),
        &[&opd_er_order_master_id.to_string(), user],
        pool,
        kphis_extra,
    )
    .await
    .map_err(|e| Source::SQLx.to_error(500, e, "Update OpdErDischargePlan"))
}

async fn insert_med_items(dc_plan_id: u32, med_ids: &[u32], version: i32, user: &str, pool: &Pool<MySql>, kphis_extra: &str) -> Result<MySqlQueryResult, AppError> {
    let insert_med_item_sql = dc_plan::insert_med_items(med_ids, dc_plan_id, user, version, kphis_extra);
    sqlx::query(AssertSqlSafe(insert_med_item_sql)).execute(pool).await.map_err(|e| {
        if let sqlx::error::Error::Database(err) = &e
            && err.code().map(|c| c == "23000").unwrap_or_default()
        {
            AppError::app_403_duplicate("Insert DcPlanMedItem")
        } else {
            Source::SQLx.to_error(500, e, "Insert DcPlanMedItem")
        }
    })
}

async fn insert_med_items_only(dc_plan_id: u32, meds: &[DischargePlanMedItemOnly], pool: &Pool<MySql>, kphis_extra: &str) -> Result<MySqlQueryResult, AppError> {
    let insert_med_item_sql = dc_plan::insert_med_items_only(meds, dc_plan_id, kphis_extra);
    sqlx::query(AssertSqlSafe(insert_med_item_sql)).execute(pool).await.map_err(|e| {
        if let sqlx::error::Error::Database(err) = &e
            && err.code().map(|c| c == "23000").unwrap_or_default()
        {
            AppError::app_403_duplicate("Insert DcPlanMedItemOnly")
        } else {
            Source::SQLx.to_error(500, e, "Insert DcPlanMedItemOnly")
        }
    })
}

async fn insert_env_items(dc_plan_id: u32, env_ids: &[u32], version: i32, user: &str, pool: &Pool<MySql>, kphis_extra: &str) -> Result<MySqlQueryResult, AppError> {
    let insert_env_item_sql = dc_plan::insert_env_items(env_ids, dc_plan_id, user, version, kphis_extra);
    sqlx::query(AssertSqlSafe(insert_env_item_sql)).execute(pool).await.map_err(|e| {
        if let sqlx::error::Error::Database(err) = &e
            && err.code().map(|c| c == "23000").unwrap_or_default()
        {
            AppError::app_403_duplicate("Insert DcPlanEnvItem")
        } else {
            Source::SQLx.to_error(500, e, "Insert DcPlanEnvItem")
        }
    })
}

async fn insert_env_items_only(dc_plan_id: u32, envs: &[DischargePlanEnvItemOnly], pool: &Pool<MySql>, kphis_extra: &str) -> Result<MySqlQueryResult, AppError> {
    let insert_env_item_sql = dc_plan::insert_env_items_only(envs, dc_plan_id, kphis_extra);
    sqlx::query(AssertSqlSafe(insert_env_item_sql)).execute(pool).await.map_err(|e| {
        if let sqlx::error::Error::Database(err) = &e
            && err.code().map(|c| c == "23000").unwrap_or_default()
        {
            AppError::app_403_duplicate("Insert DcPlanEnvItemOnly")
        } else {
            Source::SQLx.to_error(500, e, "Insert DcPlanEnvItemOnly")
        }
    })
}

async fn insert_tx_items(dc_plan_id: u32, tx_ids: &[u32], version: i32, user: &str, pool: &Pool<MySql>, kphis_extra: &str) -> Result<MySqlQueryResult, AppError> {
    let insert_tx_item_sql = dc_plan::insert_tx_items(tx_ids, dc_plan_id, user, version, kphis_extra);
    sqlx::query(AssertSqlSafe(insert_tx_item_sql)).execute(pool).await.map_err(|e| {
        if let sqlx::error::Error::Database(err) = &e
            && err.code().map(|c| c == "23000").unwrap_or_default()
        {
            AppError::app_403_duplicate("Insert DcPlanTxItem")
        } else {
            Source::SQLx.to_error(500, e, "Insert DcPlanTxItem")
        }
    })
}

async fn insert_tx_items_only(dc_plan_id: u32, txs: &[DischargePlanTxItemOnly], pool: &Pool<MySql>, kphis_extra: &str) -> Result<MySqlQueryResult, AppError> {
    let insert_tx_item_sql = dc_plan::insert_tx_items_only(txs, dc_plan_id, kphis_extra);
    sqlx::query(AssertSqlSafe(insert_tx_item_sql)).execute(pool).await.map_err(|e| {
        if let sqlx::error::Error::Database(err) = &e
            && err.code().map(|c| c == "23000").unwrap_or_default()
        {
            AppError::app_403_duplicate("Insert DcPlanTxItemOnly")
        } else {
            Source::SQLx.to_error(500, e, "Insert DcPlanTxItemOnly")
        }
    })
}

async fn insert_diet_items(dc_plan_id: u32, diet_ids: &[u32], version: i32, user: &str, pool: &Pool<MySql>, kphis_extra: &str) -> Result<MySqlQueryResult, AppError> {
    let insert_diet_item_sql = dc_plan::insert_diet_items(diet_ids, dc_plan_id, user, version, kphis_extra);
    sqlx::query(AssertSqlSafe(insert_diet_item_sql)).execute(pool).await.map_err(|e| {
        if let sqlx::error::Error::Database(err) = &e
            && err.code().map(|c| c == "23000").unwrap_or_default()
        {
            AppError::app_403_duplicate("Insert DcPlanDietItem")
        } else {
            Source::SQLx.to_error(500, e, "Insert DcPlanDietItem")
        }
    })
}

async fn insert_diet_items_only(dc_plan_id: u32, diets: &[DischargePlanDietItemOnly], pool: &Pool<MySql>, kphis_extra: &str) -> Result<MySqlQueryResult, AppError> {
    let insert_diet_item_sql = dc_plan::insert_diet_items_only(diets, dc_plan_id, kphis_extra);
    sqlx::query(AssertSqlSafe(insert_diet_item_sql)).execute(pool).await.map_err(|e| {
        if let sqlx::error::Error::Database(err) = &e
            && err.code().map(|c| c == "23000").unwrap_or_default()
        {
            AppError::app_403_duplicate("Insert DcPlanDietItemOnly")
        } else {
            Source::SQLx.to_error(500, e, "Insert DcPlanDietItemOnly")
        }
    })
}

pub async fn delete_dc_plan(dc_plan_id: u32, version: i32, pool: &Pool<MySql>, kphis_extra: &str) -> Result<Vec<ExecuteResponse>, AppError> {
    let mut results = Vec::with_capacity(5);

    let delete_dc_plan_result = delete_dc_plan_inner(dc_plan_id, version, pool, kphis_extra).await?;
    results.push(ExecuteResponse::from_query_result(delete_dc_plan_result, "Delete DcPlan"));

    let delete_med_item_result = delete_med_item(dc_plan_id, version, pool, kphis_extra).await?;
    results.push(ExecuteResponse::from_query_result(delete_med_item_result, "Delete DcPlanMedItem"));

    let delete_env_item_result = delete_env_item(dc_plan_id, version, pool, kphis_extra).await?;
    results.push(ExecuteResponse::from_query_result(delete_env_item_result, "Delete DcPlanEnvItem"));

    let delete_tx_item_result = delete_tx_item(dc_plan_id, version, pool, kphis_extra).await?;
    results.push(ExecuteResponse::from_query_result(delete_tx_item_result, "Delete DcPlanTxItem"));

    let delete_diet_item_result = delete_diet_item(dc_plan_id, version, pool, kphis_extra).await?;
    results.push(ExecuteResponse::from_query_result(delete_diet_item_result, "Delete DcPlanDietItem"));

    Ok(results)
}

async fn delete_dc_plan_inner(dc_plan_id: u32, version: i32, pool: &Pool<MySql>, kphis_extra: &str) -> Result<MySqlQueryResult, AppError> {
    let delete_dc_plan_sql = dc_plan::delete_dc_plan(kphis_extra);
    sqlx::query(AssertSqlSafe(delete_dc_plan_sql))
        .bind(dc_plan_id)
        .bind(version)
        .execute(pool)
        .await
        .map_err(|e| Source::SQLx.to_error(500, e, "Delete DcPlan"))
}

async fn delete_med_item(dc_plan_id: u32, version: i32, pool: &Pool<MySql>, kphis_extra: &str) -> Result<MySqlQueryResult, AppError> {
    let delete_med_item_sql = dc_plan::delete_med_item(kphis_extra);
    sqlx::query(AssertSqlSafe(delete_med_item_sql))
        .bind(dc_plan_id)
        .bind(version)
        .execute(pool)
        .await
        .map_err(|e| Source::SQLx.to_error(500, e, "Delete DcPlanMedItem"))
}

async fn delete_env_item(dc_plan_id: u32, version: i32, pool: &Pool<MySql>, kphis_extra: &str) -> Result<MySqlQueryResult, AppError> {
    let delete_env_item_sql = dc_plan::delete_env_item(kphis_extra);
    sqlx::query(AssertSqlSafe(delete_env_item_sql))
        .bind(dc_plan_id)
        .bind(version)
        .execute(pool)
        .await
        .map_err(|e| Source::SQLx.to_error(500, e, "Delete DcPlanEnvItem"))
}

async fn delete_tx_item(dc_plan_id: u32, version: i32, pool: &Pool<MySql>, kphis_extra: &str) -> Result<MySqlQueryResult, AppError> {
    let delete_tx_item_sql = dc_plan::delete_tx_item(kphis_extra);
    sqlx::query(AssertSqlSafe(delete_tx_item_sql))
        .bind(dc_plan_id)
        .bind(version)
        .execute(pool)
        .await
        .map_err(|e| Source::SQLx.to_error(500, e, "Delete DcPlanTxItem"))
}

async fn delete_diet_item(dc_plan_id: u32, version: i32, pool: &Pool<MySql>, kphis_extra: &str) -> Result<MySqlQueryResult, AppError> {
    let delete_diet_item_sql = dc_plan::delete_diet_item(kphis_extra);
    sqlx::query(AssertSqlSafe(delete_diet_item_sql))
        .bind(dc_plan_id)
        .bind(version)
        .execute(pool)
        .await
        .map_err(|e| Source::SQLx.to_error(500, e, "Delete DcPlanDietItem"))
}

#[cfg(test)]
#[rustfmt::skip]
mod tests {

    use super::*;
    use kphis_sqlx_tester::MySqlTester;

    #[tokio::test]
    #[ignore]
    async fn sqlx_get_dc_plan() {
        let tester = MySqlTester::new_hosxp_and_kphis_extra().await;
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis_extra/opd_er_dc_plan.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis_extra/opd_er_dc_plan_med_item.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis_extra/opd_er_dc_plan_env_item.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis_extra/opd_er_dc_plan_tx_item.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis_extra/opd_er_dc_plan_diet_item.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis_extra/ipd_dc_plan_tmp_dx.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis_extra/ipd_dc_plan_tmp_med.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis_extra/ipd_dc_plan_tmp_env.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis_extra/ipd_dc_plan_tmp_tx.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis_extra/ipd_dc_plan_tmp_diet.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/hosxp/doctor.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/hosxp/opduser.sql")).execute(&tester.db_pool).await.unwrap();

        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/kphis_extra/opd_er_dc_plan.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/kphis_extra/opd_er_dc_plan_med_item.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/kphis_extra/opd_er_dc_plan_env_item.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/kphis_extra/opd_er_dc_plan_tx_item.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/kphis_extra/opd_er_dc_plan_diet_item.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/kphis_extra/ipd_dc_plan_tmp_dx.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/kphis_extra/ipd_dc_plan_tmp_med.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/kphis_extra/ipd_dc_plan_tmp_env.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/kphis_extra/ipd_dc_plan_tmp_tx.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/kphis_extra/ipd_dc_plan_tmp_diet.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/hosxp/doctor.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/hosxp/opduser.sql")).execute(&tester.db_pool).await.unwrap();

        let found = get_dc_plan(1,&tester.db_pool,&tester.hosxp,&tester.kphis_extra).await.unwrap();
        assert_eq!(found.len(), 1);
        let not_found = get_dc_plan(999,&tester.db_pool,&tester.hosxp,&tester.kphis_extra).await.unwrap();
        assert!(not_found.is_empty());
    }

    #[tokio::test]
    #[ignore]
    async fn sqlx_get_dc_plan_only() {
        let tester = MySqlTester::new_kphis_extra().await;
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis_extra/opd_er_dc_plan.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/kphis_extra/opd_er_dc_plan.sql")).execute(&tester.db_pool).await.unwrap();

        let found = get_dc_plan_only(1,&tester.db_pool,&tester.kphis_extra).await.unwrap();
        assert_eq!(found.len(), 1);
        let not_found = get_dc_plan_only(999,&tester.db_pool,&tester.kphis_extra).await.unwrap();
        assert!(not_found.is_empty());
    }

    #[tokio::test]
    #[ignore]
    async fn sqlx_get_dc_plan_med_item_only() {
        let tester = MySqlTester::new_kphis_extra().await;
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis_extra/opd_er_dc_plan_med_item.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/kphis_extra/opd_er_dc_plan_med_item.sql")).execute(&tester.db_pool).await.unwrap();

        let found = get_dc_plan_med_item_only(1, &tester.db_pool,&tester.kphis_extra).await.unwrap();
        assert_eq!(found.len(), 1);
        let not_found = get_dc_plan_med_item_only(999,&tester.db_pool,&tester.kphis_extra).await.unwrap();
        assert!(not_found.is_empty());
    }

    #[tokio::test]
    #[ignore]
    async fn sqlx_get_dc_plan_env_item_only() {
        let tester = MySqlTester::new_kphis_extra().await;
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis_extra/opd_er_dc_plan_env_item.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/kphis_extra/opd_er_dc_plan_env_item.sql")).execute(&tester.db_pool).await.unwrap();

        let found = get_dc_plan_env_item_only(1, &tester.db_pool,&tester.kphis_extra).await.unwrap();
        assert_eq!(found.len(), 1);
        let not_found = get_dc_plan_env_item_only(999,&tester.db_pool,&tester.kphis_extra).await.unwrap();
        assert!(not_found.is_empty());
    }

    #[tokio::test]
    #[ignore]
    async fn sqlx_get_dc_plan_tx_item_only() {
        let tester = MySqlTester::new_kphis_extra().await;
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis_extra/opd_er_dc_plan_tx_item.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/kphis_extra/opd_er_dc_plan_tx_item.sql")).execute(&tester.db_pool).await.unwrap();

        let found = get_dc_plan_tx_item_only(1, &tester.db_pool,&tester.kphis_extra).await.unwrap();
        assert_eq!(found.len(), 1);
        let not_found = get_dc_plan_tx_item_only(999,&tester.db_pool,&tester.kphis_extra).await.unwrap();
        assert!(not_found.is_empty());
    }

    #[tokio::test]
    #[ignore]
    async fn sqlx_get_dc_plan_diet_item_only() {
        let tester = MySqlTester::new_kphis_extra().await;
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis_extra/opd_er_dc_plan_diet_item.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/kphis_extra/opd_er_dc_plan_diet_item.sql")).execute(&tester.db_pool).await.unwrap();

        let found = get_dc_plan_diet_item_only(1, &tester.db_pool,&tester.kphis_extra).await.unwrap();
        assert_eq!(found.len(), 1);
        let not_found = get_dc_plan_diet_item_only(999,&tester.db_pool,&tester.kphis_extra).await.unwrap();
        assert!(not_found.is_empty());
    }

    #[tokio::test]
    #[ignore]
    async fn sqlx_insert_dc_plan() {
        let tester = MySqlTester::new_kphis_extra().await;
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis_extra/opd_er_dc_plan.sql")).execute(&tester.db_pool).await.unwrap();

        let success = insert_dc_plan(1, &DischargePlanSave::demo(),"user",&tester.db_pool,&tester.kphis_extra).await.unwrap();
        assert_eq!(success.rows_affected(), 1);
        // duplicate (opd_er_order_master_id + dx_id) will error
        let again_duplicate = insert_dc_plan(1, &DischargePlanSave::demo(),"user",&tester.db_pool,&tester.kphis_extra).await;
        if let Err(again_duplicate_error) = again_duplicate {
            assert_eq!(again_duplicate_error.status, 403);
        } else {
            panic!("MUST ERROR")
        }
    }

    #[tokio::test]
    #[ignore]
    async fn sqlx_insert_dc_plan_only() {
        let tester = MySqlTester::new_kphis_extra().await;
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis_extra/opd_er_dc_plan.sql")).execute(&tester.db_pool).await.unwrap();

        let success = insert_dc_plan_only(1, &DischargePlanOnly::demo(),&tester.db_pool,&tester.kphis_extra).await.unwrap();
        assert_eq!(success.rows_affected(), 1);
        // duplicate (opd_er_order_master_id + dx_id) will error
        let again_duplicate = insert_dc_plan_only(1, &DischargePlanOnly::demo(),&tester.db_pool,&tester.kphis_extra).await;
        if let Err(again_duplicate_error) = again_duplicate {
            assert_eq!(again_duplicate_error.status, 403);
        } else {
            panic!("MUST ERROR")
        }
    }

    #[tokio::test]
    #[ignore]
    async fn sqlx_update_dc_plan() {
        let tester = MySqlTester::new_kphis_extra().await;
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis_extra/opd_er_dc_plan.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/kphis_extra/opd_er_dc_plan.sql")).execute(&tester.db_pool).await.unwrap();

        let success = update_dc_plan(1, &DischargePlanSave::demo(),"user",&tester.db_pool,&tester.kphis_extra).await.unwrap();
        assert_eq!(success.rows_affected(), 1);
        let again_success = update_dc_plan(1, &DischargePlanSave::demo(),"user",&tester.db_pool,&tester.kphis_extra).await.unwrap();
        assert_eq!(again_success.rows_affected(), 1);
    }

    #[tokio::test]
    #[ignore]
    async fn sqlx_insert_med_items() {
        let tester = MySqlTester::new_kphis_extra().await;
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis_extra/opd_er_dc_plan_med_item.sql")).execute(&tester.db_pool).await.unwrap();

        let success = insert_med_items(1, &[1, 2], 1, "user", &tester.db_pool, &tester.kphis_extra).await.unwrap();
        assert_eq!(success.rows_affected(), 2);
        // duplicate (dc_plan_id + med_id) will error
        let again_duplicate = insert_med_items(1, &[1, 2], 1, "user", &tester.db_pool, &tester.kphis_extra).await;
        if let Err(again_duplicate_error) = again_duplicate {
            assert_eq!(again_duplicate_error.status, 403);
        } else {
            panic!("MUST ERROR")
        }
    }

    #[tokio::test]
    #[ignore]
    async fn sqlx_insert_med_items_only() {
        let tester = MySqlTester::new_kphis_extra().await;
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis_extra/opd_er_dc_plan_med_item.sql")).execute(&tester.db_pool).await.unwrap();

        let success = insert_med_items_only(1, &[DischargePlanMedItemOnly::demo()], &tester.db_pool, &tester.kphis_extra).await.unwrap();
        assert_eq!(success.rows_affected(), 1);
        // duplicate (dc_plan_id + med_id) will error
        let again_duplicate = insert_med_items_only(1, &[DischargePlanMedItemOnly::demo()], &tester.db_pool, &tester.kphis_extra).await;
        if let Err(again_duplicate_error) = again_duplicate {
            assert_eq!(again_duplicate_error.status, 403);
        } else {
            panic!("MUST ERROR")
        }
    }

    #[tokio::test]
    #[ignore]
    async fn sqlx_insert_env_items() {
        let tester = MySqlTester::new_kphis_extra().await;
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis_extra/opd_er_dc_plan_env_item.sql")).execute(&tester.db_pool).await.unwrap();

        let success = insert_env_items(1, &[1, 2], 1, "user", &tester.db_pool, &tester.kphis_extra).await.unwrap();
        assert_eq!(success.rows_affected(), 2);
        // duplicate (dc_plan_id + env_id) will error
        let again_duplicate = insert_env_items(1, &[1, 2], 1, "user", &tester.db_pool, &tester.kphis_extra).await;
        if let Err(again_duplicate_error) = again_duplicate {
            assert_eq!(again_duplicate_error.status, 403);
        } else {
            panic!("MUST ERROR")
        }
    }

    #[tokio::test]
    #[ignore]
    async fn sqlx_insert_env_items_only() {
        let tester = MySqlTester::new_kphis_extra().await;
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis_extra/opd_er_dc_plan_env_item.sql")).execute(&tester.db_pool).await.unwrap();

        let success = insert_env_items_only(1, &[DischargePlanEnvItemOnly::demo()], &tester.db_pool, &tester.kphis_extra).await.unwrap();
        assert_eq!(success.rows_affected(), 1);
        // duplicate (dc_plan_id + env_id) will error
        let again_duplicate = insert_env_items_only(1, &[DischargePlanEnvItemOnly::demo()], &tester.db_pool, &tester.kphis_extra).await;
        if let Err(again_duplicate_error) = again_duplicate {
            assert_eq!(again_duplicate_error.status, 403);
        } else {
            panic!("MUST ERROR")
        }
    }

    #[tokio::test]
    #[ignore]
    async fn sqlx_insert_tx_items() {
        let tester = MySqlTester::new_kphis_extra().await;
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis_extra/opd_er_dc_plan_tx_item.sql")).execute(&tester.db_pool).await.unwrap();

        let success = insert_tx_items(1, &[1, 2], 1, "user", &tester.db_pool, &tester.kphis_extra).await.unwrap();
        assert_eq!(success.rows_affected(), 2);
        // duplicate (dc_plan_id + tx_id) will error
        let again_duplicate = insert_tx_items(1, &[1, 2], 1, "user", &tester.db_pool, &tester.kphis_extra).await;
        if let Err(again_duplicate_error) = again_duplicate {
            assert_eq!(again_duplicate_error.status, 403);
        } else {
            panic!("MUST ERROR")
        }
    }

    #[tokio::test]
    #[ignore]
    async fn sqlx_insert_tx_items_only() {
        let tester = MySqlTester::new_kphis_extra().await;
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis_extra/opd_er_dc_plan_tx_item.sql")).execute(&tester.db_pool).await.unwrap();

        let success = insert_tx_items_only(1, &[DischargePlanTxItemOnly::demo()], &tester.db_pool, &tester.kphis_extra).await.unwrap();
        assert_eq!(success.rows_affected(), 1);
        // duplicate (dc_plan_id + tx_id) will error
        let again_duplicate = insert_tx_items_only(1, &[DischargePlanTxItemOnly::demo()], &tester.db_pool, &tester.kphis_extra).await;
        if let Err(again_duplicate_error) = again_duplicate {
            assert_eq!(again_duplicate_error.status, 403);
        } else {
            panic!("MUST ERROR")
        }
    }

    #[tokio::test]
    #[ignore]
    async fn sqlx_insert_diet_items() {
        let tester = MySqlTester::new_kphis_extra().await;
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis_extra/opd_er_dc_plan_diet_item.sql")).execute(&tester.db_pool).await.unwrap();

        let success = insert_diet_items(1, &[1, 2], 1, "user", &tester.db_pool, &tester.kphis_extra).await.unwrap();
        assert_eq!(success.rows_affected(), 2);
        // duplicate (dc_plan_id + diet_id) will error
        let again_duplicate = insert_diet_items(1, &[1, 2], 1, "user", &tester.db_pool, &tester.kphis_extra).await;
        if let Err(again_duplicate_error) = again_duplicate {
            assert_eq!(again_duplicate_error.status, 403);
        } else {
            panic!("MUST ERROR")
        }
    }

    #[tokio::test]
    #[ignore]
    async fn sqlx_insert_diet_items_only() {
        let tester = MySqlTester::new_kphis_extra().await;
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis_extra/opd_er_dc_plan_diet_item.sql")).execute(&tester.db_pool).await.unwrap();

        let success = insert_diet_items_only(1, &[DischargePlanDietItemOnly::demo()], &tester.db_pool, &tester.kphis_extra).await.unwrap();
        assert_eq!(success.rows_affected(), 1);
        // duplicate (dc_plan_id + diet_id) will error
        let again_duplicate = insert_diet_items_only(1, &[DischargePlanDietItemOnly::demo()], &tester.db_pool, &tester.kphis_extra).await;
        if let Err(again_duplicate_error) = again_duplicate {
            assert_eq!(again_duplicate_error.status, 403);
        } else {
            panic!("MUST ERROR")
        }
    }

    #[tokio::test]
    #[ignore]
    async fn sqlx_delete_dc_plan_inner() {
        let tester = MySqlTester::new_kphis_extra().await;
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis_extra/opd_er_dc_plan.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/kphis_extra/opd_er_dc_plan.sql")).execute(&tester.db_pool).await.unwrap();

        let success = delete_dc_plan_inner(1, 1, &tester.db_pool, &tester.kphis_extra).await.unwrap();
        assert_eq!(success.rows_affected(), 1);
        let again_not_found = delete_dc_plan_inner(1, 1, &tester.db_pool, &tester.kphis_extra).await.unwrap();
        assert_eq!(again_not_found.rows_affected(), 0);
    }

    #[tokio::test]
    #[ignore]
    async fn sqlx_delete_med_item() {
        let tester = MySqlTester::new_kphis_extra().await;
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis_extra/opd_er_dc_plan_med_item.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/kphis_extra/opd_er_dc_plan_med_item.sql")).execute(&tester.db_pool).await.unwrap();

        let success = delete_med_item(1, 1, &tester.db_pool, &tester.kphis_extra).await.unwrap();
        assert_eq!(success.rows_affected(), 1);
        let again_not_found = delete_med_item(1, 1, &tester.db_pool, &tester.kphis_extra).await.unwrap();
        assert_eq!(again_not_found.rows_affected(), 0);
    }

    #[tokio::test]
    #[ignore]
    async fn sqlx_delete_env_item() {
        let tester = MySqlTester::new_kphis_extra().await;
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis_extra/opd_er_dc_plan_env_item.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/kphis_extra/opd_er_dc_plan_env_item.sql")).execute(&tester.db_pool).await.unwrap();

        let success = delete_env_item(1, 1, &tester.db_pool, &tester.kphis_extra).await.unwrap();
        assert_eq!(success.rows_affected(), 1);
        let again_not_found = delete_env_item(1, 1, &tester.db_pool, &tester.kphis_extra).await.unwrap();
        assert_eq!(again_not_found.rows_affected(), 0);
    }

    #[tokio::test]
    #[ignore]
    async fn sqlx_delete_tx_item() {
        let tester = MySqlTester::new_kphis_extra().await;
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis_extra/opd_er_dc_plan_tx_item.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/kphis_extra/opd_er_dc_plan_tx_item.sql")).execute(&tester.db_pool).await.unwrap();

        let success = delete_tx_item(1, 1, &tester.db_pool, &tester.kphis_extra).await.unwrap();
        assert_eq!(success.rows_affected(), 1);
        let again_not_found = delete_tx_item(1, 1, &tester.db_pool, &tester.kphis_extra).await.unwrap();
        assert_eq!(again_not_found.rows_affected(), 0);
    }

    #[tokio::test]
    #[ignore]
    async fn sqlx_delete_diet_item() {
        let tester = MySqlTester::new_kphis_extra().await;
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis_extra/opd_er_dc_plan_diet_item.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/kphis_extra/opd_er_dc_plan_diet_item.sql")).execute(&tester.db_pool).await.unwrap();

        let success = delete_diet_item(1, 1, &tester.db_pool, &tester.kphis_extra).await.unwrap();
        assert_eq!(success.rows_affected(), 1);
        let again_not_found = delete_diet_item(1, 1, &tester.db_pool, &tester.kphis_extra).await.unwrap();
        assert_eq!(again_not_found.rows_affected(), 0);
    }
}
