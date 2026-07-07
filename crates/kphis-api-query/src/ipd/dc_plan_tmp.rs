use sqlx::{AssertSqlSafe, FromRow, MySql, Pool, mysql::MySqlQueryResult};

use kphis_model::{
    fetch::ExecuteResponse,
    ipd::dc_plan_tmp::{DcPlanTmpDiet, DcPlanTmpDx, DcPlanTmpEnv, DcPlanTmpMed, DcPlanTmpParams, DcPlanTmpTx},
};
use kphis_sql::ipd::dc_plan_tmp;
use kphis_util::error::{AppError, Source};

pub async fn get_dx(params: &DcPlanTmpParams, pool: &Pool<MySql>, kphis_extra: &str) -> Result<Vec<DcPlanTmpDx>, AppError> {
    let sql = dc_plan_tmp::select_dx(params, kphis_extra);
    let mut query = sqlx::query(AssertSqlSafe(sql));
    if let Some(dx_id) = params.id {
        query = query.bind(dx_id);
    }
    query
        .fetch_all(pool)
        .await
        .map_err(|e| Source::SQLx.to_error(500, e, "Select DcPlanTmpDx"))?
        .iter()
        .map(DcPlanTmpDx::from_row)
        .collect::<sqlx::Result<Vec<DcPlanTmpDx>>>()
        .map_err(|e| Source::SQLx.to_error(500, e, "Select DcPlanTmpDx"))
}

pub async fn post_dx(save: &DcPlanTmpDx, user: &str, pool: &Pool<MySql>, kphis_extra: &str) -> Result<ExecuteResponse, AppError> {
    let result = if save.dx_id > 0 {
        let update_result = update_dx(save, user, pool, kphis_extra).await?;
        ExecuteResponse::from_query_result(update_result, "Update DcPlanTmpDx")
    } else {
        let insert_result = insert_dx(save, user, pool, kphis_extra).await?;
        ExecuteResponse::from_query_result(insert_result, "Insert DcPlanTmpDx")
    };

    Ok(result)
}

async fn update_dx(save: &DcPlanTmpDx, user: &str, pool: &Pool<MySql>, kphis_extra: &str) -> Result<MySqlQueryResult, AppError> {
    let update_sql = dc_plan_tmp::update_dx(kphis_extra);
    sqlx::query(AssertSqlSafe(update_sql))
        .bind(&save.dx_name)
        .bind(&save.dx_knowledge)
        .bind(&save.dx_revisit)
        .bind(&save.dx_prevention)
        .bind(user)
        .bind(save.dx_id)
        .execute(pool)
        .await
        .map_err(|e| Source::SQLx.to_error(500, e, "Update DcPlanTmpDx"))
}

async fn insert_dx(save: &DcPlanTmpDx, user: &str, pool: &Pool<MySql>, kphis_extra: &str) -> Result<MySqlQueryResult, AppError> {
    let insert_sql = dc_plan_tmp::insert_dx(kphis_extra);
    sqlx::query(AssertSqlSafe(insert_sql))
        .bind(&save.dx_name)
        .bind(&save.dx_knowledge)
        .bind(&save.dx_revisit)
        .bind(&save.dx_prevention)
        .bind(user)
        .bind(user)
        .execute(pool)
        .await
        .map_err(|e| Source::SQLx.to_error(500, e, "Insert DcPlanTmpDx"))
}

pub async fn delete_dx(params: &DcPlanTmpParams, user: &str, pool: &Pool<MySql>, kphis_extra: &str) -> Result<ExecuteResponse, AppError> {
    let sql = dc_plan_tmp::delete_dx(kphis_extra);
    let result = sqlx::query(AssertSqlSafe(sql))
        .bind(user)
        .bind(params.id)
        .bind(params.id)
        .bind(params.id)
        .execute(pool)
        .await
        .map_err(|e| Source::SQLx.to_error(500, e, "Delete DcPlanTmpDx"))?;

    Ok(ExecuteResponse::from_query_result(result, "Delete DcPlanTmpDx"))
}

pub async fn get_med(params: &DcPlanTmpParams, pool: &Pool<MySql>, kphis_extra: &str) -> Result<Vec<DcPlanTmpMed>, AppError> {
    let sql = dc_plan_tmp::select_med(params, kphis_extra);
    let mut query = sqlx::query(AssertSqlSafe(sql));
    if let Some(med_id) = params.id {
        query = query.bind(med_id);
    }
    query
        .fetch_all(pool)
        .await
        .map_err(|e| Source::SQLx.to_error(500, e, "Get DcPlanTmpMed"))?
        .iter()
        .map(DcPlanTmpMed::from_row)
        .collect::<sqlx::Result<Vec<DcPlanTmpMed>>>()
        .map_err(|e| Source::SQLx.to_error(500, e, "Get DcPlanTmpMed"))
}

pub async fn post_med(save: &DcPlanTmpMed, user: &str, pool: &Pool<MySql>, kphis_extra: &str) -> Result<ExecuteResponse, AppError> {
    let result = if save.med_id > 0 {
        let update_result = update_med(save, user, pool, kphis_extra).await?;
        ExecuteResponse::from_query_result(update_result, "Update DcPlanTmpMed")
    } else {
        let insert_result = insert_med(save, user, pool, kphis_extra).await?;
        ExecuteResponse::from_query_result(insert_result, "Update DcPlanTmpMed")
    };

    Ok(result)
}

async fn update_med(save: &DcPlanTmpMed, user: &str, pool: &Pool<MySql>, kphis_extra: &str) -> Result<MySqlQueryResult, AppError> {
    let update_sql = dc_plan_tmp::update_med(kphis_extra);
    sqlx::query(AssertSqlSafe(update_sql))
        .bind(&save.med_text)
        .bind(user)
        .bind(save.med_id)
        .execute(pool)
        .await
        .map_err(|e| Source::SQLx.to_error(500, e, "Update DcPlanTmpMed"))
}

async fn insert_med(save: &DcPlanTmpMed, user: &str, pool: &Pool<MySql>, kphis_extra: &str) -> Result<MySqlQueryResult, AppError> {
    let insert_sql = dc_plan_tmp::insert_med(kphis_extra);
    sqlx::query(AssertSqlSafe(insert_sql))
        .bind(&save.med_text)
        .bind(user)
        .bind(user)
        .execute(pool)
        .await
        .map_err(|e| Source::SQLx.to_error(500, e, "Insert DcPlanTmpMed"))
}

pub async fn delete_med(params: &DcPlanTmpParams, user: &str, pool: &Pool<MySql>, kphis_extra: &str) -> Result<ExecuteResponse, AppError> {
    let sql = dc_plan_tmp::delete_med(kphis_extra);
    let result = sqlx::query(AssertSqlSafe(sql))
        .bind(user)
        .bind(params.id)
        .bind(params.id)
        .bind(params.id)
        .execute(pool)
        .await
        .map_err(|e| Source::SQLx.to_error(500, e, "Delete DcPlanTmpMed"))?;

    Ok(ExecuteResponse::from_query_result(result, "Delete DcPlanTmpMed"))
}

pub async fn get_env(params: &DcPlanTmpParams, pool: &Pool<MySql>, kphis_extra: &str) -> Result<Vec<DcPlanTmpEnv>, AppError> {
    let sql = dc_plan_tmp::select_env(params, kphis_extra);
    let mut query = sqlx::query(AssertSqlSafe(sql));
    if let Some(env_id) = params.id {
        query = query.bind(env_id);
    }
    query
        .fetch_all(pool)
        .await
        .map_err(|e| Source::SQLx.to_error(500, e, "Get DcPlanTmpEnv"))?
        .iter()
        .map(DcPlanTmpEnv::from_row)
        .collect::<sqlx::Result<Vec<DcPlanTmpEnv>>>()
        .map_err(|e| Source::SQLx.to_error(500, e, "Get DcPlanTmpEnv"))
}

pub async fn post_env(save: &DcPlanTmpEnv, user: &str, pool: &Pool<MySql>, kphis_extra: &str) -> Result<ExecuteResponse, AppError> {
    let result = if save.env_id > 0 {
        let update_result = update_env(save, user, pool, kphis_extra).await?;
        ExecuteResponse::from_query_result(update_result, "Update DcPlanTmpEnv")
    } else {
        let insert_result = insert_env(save, user, pool, kphis_extra).await?;
        ExecuteResponse::from_query_result(insert_result, "Update DcPlanTmpEnv")
    };

    Ok(result)
}

async fn update_env(save: &DcPlanTmpEnv, user: &str, pool: &Pool<MySql>, kphis_extra: &str) -> Result<MySqlQueryResult, AppError> {
    let update_sql = dc_plan_tmp::update_env(kphis_extra);
    sqlx::query(AssertSqlSafe(update_sql))
        .bind(&save.env_text)
        .bind(user)
        .bind(save.env_id)
        .execute(pool)
        .await
        .map_err(|e| Source::SQLx.to_error(500, e, "Update DcPlanTmpEnv"))
}

async fn insert_env(save: &DcPlanTmpEnv, user: &str, pool: &Pool<MySql>, kphis_extra: &str) -> Result<MySqlQueryResult, AppError> {
    let insert_sql = dc_plan_tmp::insert_env(kphis_extra);
    sqlx::query(AssertSqlSafe(insert_sql))
        .bind(&save.env_text)
        .bind(user)
        .bind(user)
        .execute(pool)
        .await
        .map_err(|e| Source::SQLx.to_error(500, e, "Insert DcPlanTmpEnv"))
}

pub async fn delete_env(params: &DcPlanTmpParams, user: &str, pool: &Pool<MySql>, kphis_extra: &str) -> Result<ExecuteResponse, AppError> {
    let sql = dc_plan_tmp::delete_env(kphis_extra);
    let result = sqlx::query(AssertSqlSafe(sql))
        .bind(user)
        .bind(params.id)
        .bind(params.id)
        .bind(params.id)
        .execute(pool)
        .await
        .map_err(|e| Source::SQLx.to_error(500, e, "Delete DcPlanTmpEnv"))?;

    Ok(ExecuteResponse::from_query_result(result, "Delete DcPlanTmpEnv"))
}

pub async fn get_tx(params: &DcPlanTmpParams, pool: &Pool<MySql>, kphis_extra: &str) -> Result<Vec<DcPlanTmpTx>, AppError> {
    let sql = dc_plan_tmp::select_tx(params, kphis_extra);
    let mut query = sqlx::query(AssertSqlSafe(sql));
    if let Some(tx_id) = params.id {
        query = query.bind(tx_id);
    }
    query
        .fetch_all(pool)
        .await
        .map_err(|e| Source::SQLx.to_error(500, e, "Get DcPlanTmpTx"))?
        .iter()
        .map(DcPlanTmpTx::from_row)
        .collect::<sqlx::Result<Vec<DcPlanTmpTx>>>()
        .map_err(|e| Source::SQLx.to_error(500, e, "Get DcPlanTmpTx"))
}

pub async fn post_tx(save: &DcPlanTmpTx, user: &str, pool: &Pool<MySql>, kphis_extra: &str) -> Result<ExecuteResponse, AppError> {
    let result = if save.tx_id > 0 {
        let update_result = update_tx(save, user, pool, kphis_extra).await?;
        ExecuteResponse::from_query_result(update_result, "Update DcPlanTmpTx")
    } else {
        let insert_result = insert_tx(save, user, pool, kphis_extra).await?;
        ExecuteResponse::from_query_result(insert_result, "Update DcPlanTmpTx")
    };

    Ok(result)
}

async fn update_tx(save: &DcPlanTmpTx, user: &str, pool: &Pool<MySql>, kphis_extra: &str) -> Result<MySqlQueryResult, AppError> {
    let update_sql = dc_plan_tmp::update_tx(kphis_extra);
    sqlx::query(AssertSqlSafe(update_sql))
        .bind(&save.tx_text)
        .bind(user)
        .bind(save.tx_id)
        .execute(pool)
        .await
        .map_err(|e| Source::SQLx.to_error(500, e, "Update DcPlanTmpTx"))
}

async fn insert_tx(save: &DcPlanTmpTx, user: &str, pool: &Pool<MySql>, kphis_extra: &str) -> Result<MySqlQueryResult, AppError> {
    let insert_sql = dc_plan_tmp::insert_tx(kphis_extra);
    sqlx::query(AssertSqlSafe(insert_sql))
        .bind(&save.tx_text)
        .bind(user)
        .bind(user)
        .execute(pool)
        .await
        .map_err(|e| Source::SQLx.to_error(500, e, "Insert DcPlanTmpTx"))
}

pub async fn delete_tx(params: &DcPlanTmpParams, user: &str, pool: &Pool<MySql>, kphis_extra: &str) -> Result<ExecuteResponse, AppError> {
    let sql = dc_plan_tmp::delete_tx(kphis_extra);
    let result = sqlx::query(AssertSqlSafe(sql))
        .bind(user)
        .bind(params.id)
        .bind(params.id)
        .bind(params.id)
        .execute(pool)
        .await
        .map_err(|e| Source::SQLx.to_error(500, e, "Delete DcPlanTmpTx"))?;

    Ok(ExecuteResponse::from_query_result(result, "Delete DcPlanTmpTx"))
}

pub async fn get_diet(params: &DcPlanTmpParams, pool: &Pool<MySql>, kphis_extra: &str) -> Result<Vec<DcPlanTmpDiet>, AppError> {
    let sql = dc_plan_tmp::select_diet(params, kphis_extra);
    let mut query = sqlx::query(AssertSqlSafe(sql));
    if let Some(diet_id) = params.id {
        query = query.bind(diet_id);
    }
    query
        .fetch_all(pool)
        .await
        .map_err(|e| Source::SQLx.to_error(500, e, "Get DcPlanTmpDiet"))?
        .iter()
        .map(DcPlanTmpDiet::from_row)
        .collect::<sqlx::Result<Vec<DcPlanTmpDiet>>>()
        .map_err(|e| Source::SQLx.to_error(500, e, "Get DcPlanTmpDiet"))
}

pub async fn post_diet(save: &DcPlanTmpDiet, user: &str, pool: &Pool<MySql>, kphis_extra: &str) -> Result<ExecuteResponse, AppError> {
    let result = if save.diet_id > 0 {
        let update_result = update_diet(save, user, pool, kphis_extra).await?;
        ExecuteResponse::from_query_result(update_result, "Update DcPlanTmpDiet")
    } else {
        let insert_result = insert_diet(save, user, pool, kphis_extra).await?;
        ExecuteResponse::from_query_result(insert_result, "Update DcPlanTmpDiet")
    };

    Ok(result)
}

async fn update_diet(save: &DcPlanTmpDiet, user: &str, pool: &Pool<MySql>, kphis_extra: &str) -> Result<MySqlQueryResult, AppError> {
    let update_sql = dc_plan_tmp::update_diet(kphis_extra);
    sqlx::query(AssertSqlSafe(update_sql))
        .bind(&save.diet_text)
        .bind(user)
        .bind(save.diet_id)
        .execute(pool)
        .await
        .map_err(|e| Source::SQLx.to_error(500, e, "Update DcPlanTmpDiet"))
}

async fn insert_diet(save: &DcPlanTmpDiet, user: &str, pool: &Pool<MySql>, kphis_extra: &str) -> Result<MySqlQueryResult, AppError> {
    let insert_sql = dc_plan_tmp::insert_diet(kphis_extra);
    sqlx::query(AssertSqlSafe(insert_sql))
        .bind(&save.diet_text)
        .bind(user)
        .bind(user)
        .execute(pool)
        .await
        .map_err(|e| Source::SQLx.to_error(500, e, "Insert DcPlanTmpDiet"))
}

pub async fn delete_diet(params: &DcPlanTmpParams, user: &str, pool: &Pool<MySql>, kphis_extra: &str) -> Result<ExecuteResponse, AppError> {
    let sql = dc_plan_tmp::delete_diet(kphis_extra);
    let result = sqlx::query(AssertSqlSafe(sql))
        .bind(user)
        .bind(params.id)
        .bind(params.id)
        .bind(params.id)
        .execute(pool)
        .await
        .map_err(|e| Source::SQLx.to_error(500, e, "Delete DcPlanTmpDiet"))?;

    Ok(ExecuteResponse::from_query_result(result, "Delete DcPlanTmpDiet"))
}

#[cfg(test)]
#[rustfmt::skip]
mod tests {

    use super::*;
    use kphis_sqlx_tester::MySqlTester;

    #[tokio::test]
    #[ignore]
    async fn sqlx_get_dx() {
        let tester = MySqlTester::new_kphis_extra().await;
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis_extra/ipd_dc_plan_tmp_dx.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/kphis_extra/ipd_dc_plan_tmp_dx.sql")).execute(&tester.db_pool).await.unwrap();

        let default = get_dx(&DcPlanTmpParams::default(), &tester.db_pool, &tester.kphis_extra).await.unwrap();
        assert_eq!(default.len(), 2);
        let found_dx_id = get_dx(&DcPlanTmpParams {id: Some(1)},&tester.db_pool,&tester.kphis_extra).await.unwrap();
        assert_eq!(found_dx_id.len(), 1);
        let not_found = get_dx(&DcPlanTmpParams {id: Some(888)},&tester.db_pool,&tester.kphis_extra).await.unwrap();
        assert!(not_found.is_empty());
    }

    #[tokio::test]
    #[ignore]
    async fn sqlx_get_med() {
        let tester = MySqlTester::new_kphis_extra().await;
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis_extra/ipd_dc_plan_tmp_med.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/kphis_extra/ipd_dc_plan_tmp_med.sql")).execute(&tester.db_pool).await.unwrap();

        let default = get_med(&DcPlanTmpParams::default(), &tester.db_pool, &tester.kphis_extra).await.unwrap();
        assert_eq!(default.len(), 2);
        let found_med_id = get_med(&DcPlanTmpParams {id: Some(1)},&tester.db_pool,&tester.kphis_extra).await.unwrap();
        assert_eq!(found_med_id.len(), 1);
        let not_found = get_med(&DcPlanTmpParams {id: Some(888)},&tester.db_pool,&tester.kphis_extra).await.unwrap();
        assert!(not_found.is_empty());
    }

    #[tokio::test]
    #[ignore]
    async fn sqlx_get_env() {
        let tester = MySqlTester::new_kphis_extra().await;
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis_extra/ipd_dc_plan_tmp_env.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/kphis_extra/ipd_dc_plan_tmp_env.sql")).execute(&tester.db_pool).await.unwrap();

        let default = get_env(&DcPlanTmpParams::default(), &tester.db_pool, &tester.kphis_extra).await.unwrap();
        assert_eq!(default.len(), 2);
        let found_env_id = get_env(&DcPlanTmpParams {id: Some(1)},&tester.db_pool,&tester.kphis_extra).await.unwrap();
        assert_eq!(found_env_id.len(), 1);
        let not_found = get_env(&DcPlanTmpParams {id: Some(888)},&tester.db_pool,&tester.kphis_extra).await.unwrap();
        assert!(not_found.is_empty());
    }

    #[tokio::test]
    #[ignore]
    async fn sqlx_get_tx() {
        let tester = MySqlTester::new_kphis_extra().await;
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis_extra/ipd_dc_plan_tmp_tx.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/kphis_extra/ipd_dc_plan_tmp_tx.sql")).execute(&tester.db_pool).await.unwrap();

        let default = get_tx(&DcPlanTmpParams::default(), &tester.db_pool, &tester.kphis_extra).await.unwrap();
        assert_eq!(default.len(), 2);
        let found_tx_id = get_tx(&DcPlanTmpParams {id: Some(1)},&tester.db_pool,&tester.kphis_extra).await.unwrap();
        assert_eq!(found_tx_id.len(), 1);
        let not_found = get_tx(&DcPlanTmpParams {id: Some(888)},&tester.db_pool,&tester.kphis_extra).await.unwrap();
        assert!(not_found.is_empty());
    }

    #[tokio::test]
    #[ignore]
    async fn sqlx_get_diet() {
        let tester = MySqlTester::new_kphis_extra().await;
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis_extra/ipd_dc_plan_tmp_diet.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/kphis_extra/ipd_dc_plan_tmp_diet.sql")).execute(&tester.db_pool).await.unwrap();

        let default = get_diet(&DcPlanTmpParams::default(), &tester.db_pool, &tester.kphis_extra).await.unwrap();
        assert_eq!(default.len(), 2);
        let found_diet_id = get_diet(&DcPlanTmpParams {id: Some(1)},&tester.db_pool,&tester.kphis_extra).await.unwrap();
        assert_eq!(found_diet_id.len(), 1);
        let not_found = get_diet(&DcPlanTmpParams {id: Some(888)},&tester.db_pool,&tester.kphis_extra).await.unwrap();
        assert!(not_found.is_empty());
    }

    #[tokio::test]
    #[ignore]
    async fn sqlx_insert_dx() {
        let tester = MySqlTester::new_kphis_extra().await;
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis_extra/ipd_dc_plan_tmp_dx.sql")).execute(&tester.db_pool).await.unwrap();

        let success = insert_dx(&DcPlanTmpDx::demo(), "user", &tester.db_pool, &tester.kphis_extra).await.unwrap();
        assert_eq!(success.rows_affected(), 1);
        let again_success = insert_dx(&DcPlanTmpDx::demo(), "user", &tester.db_pool, &tester.kphis_extra).await.unwrap();
        assert_eq!(again_success.rows_affected(), 1);
    }

    #[tokio::test]
    #[ignore]
    async fn sqlx_insert_med() {
        let tester = MySqlTester::new_kphis_extra().await;
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis_extra/ipd_dc_plan_tmp_med.sql")).execute(&tester.db_pool).await.unwrap();

        let success = insert_med(&DcPlanTmpMed::demo(), "user", &tester.db_pool, &tester.kphis_extra).await.unwrap();
        assert_eq!(success.rows_affected(), 1);
        let again_success = insert_med(&DcPlanTmpMed::demo(), "user", &tester.db_pool, &tester.kphis_extra).await.unwrap();
        assert_eq!(again_success.rows_affected(), 1);
    }

    #[tokio::test]
    #[ignore]
    async fn sqlx_insert_env() {
        let tester = MySqlTester::new_kphis_extra().await;
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis_extra/ipd_dc_plan_tmp_env.sql")).execute(&tester.db_pool).await.unwrap();

        let success = insert_env(&DcPlanTmpEnv::demo(), "user", &tester.db_pool, &tester.kphis_extra).await.unwrap();
        assert_eq!(success.rows_affected(), 1);
        let again_success = insert_env(&DcPlanTmpEnv::demo(), "user", &tester.db_pool, &tester.kphis_extra).await.unwrap();
        assert_eq!(again_success.rows_affected(), 1);
    }

    #[tokio::test]
    #[ignore]
    async fn sqlx_insert_tx() {
        let tester = MySqlTester::new_kphis_extra().await;
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis_extra/ipd_dc_plan_tmp_tx.sql")).execute(&tester.db_pool).await.unwrap();

        let success = insert_tx(&DcPlanTmpTx::demo(), "user", &tester.db_pool, &tester.kphis_extra).await.unwrap();
        assert_eq!(success.rows_affected(), 1);
        let again_success = insert_tx(&DcPlanTmpTx::demo(), "user", &tester.db_pool, &tester.kphis_extra).await.unwrap();
        assert_eq!(again_success.rows_affected(), 1);
    }

    #[tokio::test]
    #[ignore]
    async fn sqlx_insert_diet() {
        let tester = MySqlTester::new_kphis_extra().await;
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis_extra/ipd_dc_plan_tmp_diet.sql")).execute(&tester.db_pool).await.unwrap();

        let success = insert_diet(&DcPlanTmpDiet::demo(), "user", &tester.db_pool, &tester.kphis_extra).await.unwrap();
        assert_eq!(success.rows_affected(), 1);
        let again_success = insert_diet(&DcPlanTmpDiet::demo(), "user", &tester.db_pool, &tester.kphis_extra).await.unwrap();
        assert_eq!(again_success.rows_affected(), 1);
    }


    #[tokio::test]
    #[ignore]
    async fn sqlx_update_dx() {
        let tester = MySqlTester::new_kphis_extra().await;
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis_extra/ipd_dc_plan_tmp_dx.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/kphis_extra/ipd_dc_plan_tmp_dx.sql")).execute(&tester.db_pool).await.unwrap();

        let success = update_dx(&DcPlanTmpDx::demo(), "user", &tester.db_pool, &tester.kphis_extra).await.unwrap();
        assert_eq!(success.rows_affected(), 1);
        let again_success = update_dx(&DcPlanTmpDx::demo(), "user", &tester.db_pool, &tester.kphis_extra).await.unwrap();
        assert_eq!(again_success.rows_affected(), 1);
    }

    #[tokio::test]
    #[ignore]
    async fn sqlx_update_med() {
        let tester = MySqlTester::new_kphis_extra().await;
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis_extra/ipd_dc_plan_tmp_med.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/kphis_extra/ipd_dc_plan_tmp_med.sql")).execute(&tester.db_pool).await.unwrap();

        let success = update_med(&DcPlanTmpMed::demo(), "user", &tester.db_pool, &tester.kphis_extra).await.unwrap();
        assert_eq!(success.rows_affected(), 1);
        let again_success = update_med(&DcPlanTmpMed::demo(), "user", &tester.db_pool, &tester.kphis_extra).await.unwrap();
        assert_eq!(again_success.rows_affected(), 1);
    }

    #[tokio::test]
    #[ignore]
    async fn sqlx_update_env() {
        let tester = MySqlTester::new_kphis_extra().await;
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis_extra/ipd_dc_plan_tmp_env.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/kphis_extra/ipd_dc_plan_tmp_env.sql")).execute(&tester.db_pool).await.unwrap();

        let success = update_env(&DcPlanTmpEnv::demo(), "user", &tester.db_pool, &tester.kphis_extra).await.unwrap();
        assert_eq!(success.rows_affected(), 1);
        let again_success = update_env(&DcPlanTmpEnv::demo(), "user", &tester.db_pool, &tester.kphis_extra).await.unwrap();
        assert_eq!(again_success.rows_affected(), 1);
    }

    #[tokio::test]
    #[ignore]
    async fn sqlx_update_tx() {
        let tester = MySqlTester::new_kphis_extra().await;
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis_extra/ipd_dc_plan_tmp_tx.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/kphis_extra/ipd_dc_plan_tmp_tx.sql")).execute(&tester.db_pool).await.unwrap();

        let success = update_tx(&DcPlanTmpTx::demo(), "user", &tester.db_pool, &tester.kphis_extra).await.unwrap();
        assert_eq!(success.rows_affected(), 1);
        let again_success = update_tx(&DcPlanTmpTx::demo(), "user", &tester.db_pool, &tester.kphis_extra).await.unwrap();
        assert_eq!(again_success.rows_affected(), 1);
    }

    #[tokio::test]
    #[ignore]
    async fn sqlx_update_diet() {
        let tester = MySqlTester::new_kphis_extra().await;
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis_extra/ipd_dc_plan_tmp_diet.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/kphis_extra/ipd_dc_plan_tmp_diet.sql")).execute(&tester.db_pool).await.unwrap();

        let success = update_diet(&DcPlanTmpDiet::demo(), "user", &tester.db_pool, &tester.kphis_extra).await.unwrap();
        assert_eq!(success.rows_affected(), 1);
        let again_success = update_diet(&DcPlanTmpDiet::demo(), "user", &tester.db_pool, &tester.kphis_extra).await.unwrap();
        assert_eq!(again_success.rows_affected(), 1);
    }

    #[tokio::test]
    #[ignore]
    async fn sqlx_delete_dx() {
        let tester = MySqlTester::new_kphis_extra().await;
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis_extra/ipd_dc_plan_tmp_dx.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis_extra/ipd_dc_plan.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis_extra/opd_er_dc_plan.sql")).execute(&tester.db_pool).await.unwrap();

        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/kphis_extra/ipd_dc_plan_tmp_dx.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/kphis_extra/ipd_dc_plan.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/kphis_extra/opd_er_dc_plan.sql")).execute(&tester.db_pool).await.unwrap();

        let default = delete_dx(&DcPlanTmpParams::default(),"user",&tester.db_pool,&tester.kphis_extra).await.unwrap();
        assert_eq!(default.rows_affected, 0);
        let used = delete_dx(&DcPlanTmpParams {id: Some(1)},"user",&tester.db_pool,&tester.kphis_extra).await.unwrap();
        assert_eq!(used.rows_affected, 0);
        let success = delete_dx(&DcPlanTmpParams {id: Some(2)},"user",&tester.db_pool,&tester.kphis_extra).await.unwrap();
        assert_eq!(success.rows_affected, 1);
        let again_not_found = delete_dx(&DcPlanTmpParams {id: Some(2)},"user",&tester.db_pool,&tester.kphis_extra).await.unwrap();
        assert_eq!(again_not_found.rows_affected, 0);
    }

    #[tokio::test]
    #[ignore]
    async fn sqlx_delete_med() {
        let tester = MySqlTester::new_kphis_extra().await;
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis_extra/ipd_dc_plan_tmp_med.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis_extra/ipd_dc_plan_med_item.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis_extra/opd_er_dc_plan_med_item.sql")).execute(&tester.db_pool).await.unwrap();

        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/kphis_extra/ipd_dc_plan_tmp_med.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/kphis_extra/ipd_dc_plan_med_item.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/kphis_extra/opd_er_dc_plan_med_item.sql")).execute(&tester.db_pool).await.unwrap();

        let default = delete_med(&DcPlanTmpParams::default(),"user",&tester.db_pool,&tester.kphis_extra).await.unwrap();
        assert_eq!(default.rows_affected, 0);
        let used = delete_med(&DcPlanTmpParams {id: Some(1)},"user",&tester.db_pool,&tester.kphis_extra).await.unwrap();
        assert_eq!(used.rows_affected, 0);
        let success = delete_med(&DcPlanTmpParams {id: Some(2)},"user",&tester.db_pool,&tester.kphis_extra).await.unwrap();
        assert_eq!(success.rows_affected, 1);
        let again_not_found = delete_med(&DcPlanTmpParams {id: Some(2)},"user",&tester.db_pool,&tester.kphis_extra).await.unwrap();
        assert_eq!(again_not_found.rows_affected, 0);
    }

    #[tokio::test]
    #[ignore]
    async fn sqlx_delete_env() {
        let tester = MySqlTester::new_kphis_extra().await;
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis_extra/ipd_dc_plan_tmp_env.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis_extra/ipd_dc_plan_env_item.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis_extra/opd_er_dc_plan_env_item.sql")).execute(&tester.db_pool).await.unwrap();

        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/kphis_extra/ipd_dc_plan_tmp_env.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/kphis_extra/ipd_dc_plan_env_item.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/kphis_extra/opd_er_dc_plan_env_item.sql")).execute(&tester.db_pool).await.unwrap();

        let default = delete_env(&DcPlanTmpParams::default(),"user",&tester.db_pool,&tester.kphis_extra).await.unwrap();
        assert_eq!(default.rows_affected, 0);
        let used = delete_env(&DcPlanTmpParams {id: Some(1)},"user",&tester.db_pool,&tester.kphis_extra).await.unwrap();
        assert_eq!(used.rows_affected, 0);
        let success = delete_env(&DcPlanTmpParams {id: Some(2)},"user",&tester.db_pool,&tester.kphis_extra).await.unwrap();
        assert_eq!(success.rows_affected, 1);
        let again_not_found = delete_env(&DcPlanTmpParams {id: Some(2)},"user",&tester.db_pool,&tester.kphis_extra).await.unwrap();
        assert_eq!(again_not_found.rows_affected, 0);
    }

    #[tokio::test]
    #[ignore]
    async fn sqlx_delete_tx() {
        let tester = MySqlTester::new_kphis_extra().await;
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis_extra/ipd_dc_plan_tmp_tx.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis_extra/ipd_dc_plan_tx_item.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis_extra/opd_er_dc_plan_tx_item.sql")).execute(&tester.db_pool).await.unwrap();

        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/kphis_extra/ipd_dc_plan_tmp_tx.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/kphis_extra/ipd_dc_plan_tx_item.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/kphis_extra/opd_er_dc_plan_tx_item.sql")).execute(&tester.db_pool).await.unwrap();

        let default = delete_tx(&DcPlanTmpParams::default(),"user",&tester.db_pool,&tester.kphis_extra).await.unwrap();
        assert_eq!(default.rows_affected, 0);
        let used = delete_tx(&DcPlanTmpParams {id: Some(1)},"user",&tester.db_pool,&tester.kphis_extra).await.unwrap();
        assert_eq!(used.rows_affected, 0);
        let success = delete_tx(&DcPlanTmpParams {id: Some(2)},"user",&tester.db_pool,&tester.kphis_extra).await.unwrap();
        assert_eq!(success.rows_affected, 1);
        let again_not_found = delete_tx(&DcPlanTmpParams {id: Some(2)},"user",&tester.db_pool,&tester.kphis_extra).await.unwrap();
        assert_eq!(again_not_found.rows_affected, 0);
    }

    #[tokio::test]
    #[ignore]
    async fn sqlx_delete_diet() {
        let tester = MySqlTester::new_kphis_extra().await;
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis_extra/ipd_dc_plan_tmp_diet.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis_extra/ipd_dc_plan_diet_item.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis_extra/opd_er_dc_plan_diet_item.sql")).execute(&tester.db_pool).await.unwrap();

        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/kphis_extra/ipd_dc_plan_tmp_diet.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/kphis_extra/ipd_dc_plan_diet_item.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/kphis_extra/opd_er_dc_plan_diet_item.sql")).execute(&tester.db_pool).await.unwrap();

        let default = delete_diet(&DcPlanTmpParams::default(),"user",&tester.db_pool,&tester.kphis_extra).await.unwrap();
        assert_eq!(default.rows_affected, 0);
        let used = delete_diet(&DcPlanTmpParams {id: Some(1)},"user",&tester.db_pool,&tester.kphis_extra).await.unwrap();
        assert_eq!(used.rows_affected, 0);
        let success = delete_diet(&DcPlanTmpParams {id: Some(2)},"user",&tester.db_pool,&tester.kphis_extra).await.unwrap();
        assert_eq!(success.rows_affected, 1);
        let again_not_found = delete_diet(&DcPlanTmpParams {id: Some(2)},"user",&tester.db_pool,&tester.kphis_extra).await.unwrap();
        assert_eq!(again_not_found.rows_affected, 0);
    }
}
