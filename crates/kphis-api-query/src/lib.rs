#![allow(dead_code)]

pub mod app;
pub mod assets;
pub mod avatar;
pub mod binder;
pub mod drug_use_duration;
pub mod emr;
pub mod image;
pub mod ipd;
pub mod lab;
pub mod log;
pub mod med_reconciliation;
pub mod opd_er;
pub mod post_admit;
pub mod pre_admit;
pub mod pre_order;
pub mod prescription;
pub mod query_utils;
pub mod refer_note;
pub mod refer_out;
pub mod report;
pub mod schema_update;
pub mod search;
pub mod select_utils;
pub mod sse;
pub mod transform;
pub mod user;
pub mod xray;

use sqlx::{
    AssertSqlSafe, MySql, Pool,
    mysql::{MySqlQueryResult, MySqlRow},
};

use kphis_util::error::{AppError, Source};

//=========//
// execute //
//=========//
#[inline]
pub async fn execute(sql: &str, pool: &Pool<MySql>, action: &str) -> Result<MySqlQueryResult, AppError> {
    sqlx::query(AssertSqlSafe(sql)).execute(pool).await.map_err(|e| Source::SQLx.to_error(500, e, action))
}

#[inline]
pub async fn execute1(first: &str, sql: &str, pool: &Pool<MySql>, action: &str) -> Result<MySqlQueryResult, AppError> {
    sqlx::query(AssertSqlSafe(sql)).bind(first).execute(pool).await.map_err(|e| Source::SQLx.to_error(500, e, action))
}

#[inline]
pub async fn execute2(first: &str, second: &str, sql: &str, pool: &Pool<MySql>, action: &str) -> Result<MySqlQueryResult, AppError> {
    sqlx::query(AssertSqlSafe(sql))
        .bind(first)
        .bind(second)
        .execute(pool)
        .await
        .map_err(|e| Source::SQLx.to_error(500, e, action))
}

#[inline]
pub async fn execute3(first: &str, second: &str, third: &str, sql: &str, pool: &Pool<MySql>, action: &str) -> Result<MySqlQueryResult, AppError> {
    sqlx::query(AssertSqlSafe(sql))
        .bind(first)
        .bind(second)
        .bind(third)
        .execute(pool)
        .await
        .map_err(|e| Source::SQLx.to_error(500, e, action))
}

#[inline]
pub async fn execute4(first: &str, second: &str, third: &str, fourth: &str, sql: &str, pool: &Pool<MySql>, action: &str) -> Result<MySqlQueryResult, AppError> {
    sqlx::query(AssertSqlSafe(sql))
        .bind(first)
        .bind(second)
        .bind(third)
        .bind(fourth)
        .execute(pool)
        .await
        .map_err(|e| Source::SQLx.to_error(500, e, action))
}

#[allow(clippy::too_many_arguments)]
#[inline]
pub async fn execute5(first: &str, second: &str, third: &str, fourth: &str, fifth: &str, sql: &str, pool: &Pool<MySql>, action: &str) -> Result<MySqlQueryResult, AppError> {
    sqlx::query(AssertSqlSafe(sql))
        .bind(first)
        .bind(second)
        .bind(third)
        .bind(fourth)
        .bind(fifth)
        .execute(pool)
        .await
        .map_err(|e| Source::SQLx.to_error(500, e, action))
}

#[allow(clippy::too_many_arguments)]
#[inline]
pub async fn execute6(first: &str, second: &str, third: &str, fourth: &str, fifth: &str, sixth: &str, sql: &str, pool: &Pool<MySql>, action: &str) -> Result<MySqlQueryResult, AppError> {
    sqlx::query(AssertSqlSafe(sql))
        .bind(first)
        .bind(second)
        .bind(third)
        .bind(fourth)
        .bind(fifth)
        .bind(sixth)
        .execute(pool)
        .await
        .map_err(|e| Source::SQLx.to_error(500, e, action))
}

//===========//
// fetch all //
//===========//
#[inline]
pub async fn query_all(sql: &str, pool: &Pool<MySql>, action: &str) -> Result<Vec<MySqlRow>, AppError> {
    sqlx::query(AssertSqlSafe(sql)).fetch_all(pool).await.map_err(|e| Source::SQLx.to_error(500, e, action))
}

#[inline]
pub async fn query1_all(first: &str, sql: &str, pool: &Pool<MySql>, action: &str) -> Result<Vec<MySqlRow>, AppError> {
    sqlx::query(AssertSqlSafe(sql)).bind(first).fetch_all(pool).await.map_err(|e| Source::SQLx.to_error(500, e, action))
}

#[inline]
pub async fn query2_all(first: &str, second: &str, sql: &str, pool: &Pool<MySql>, action: &str) -> Result<Vec<MySqlRow>, AppError> {
    sqlx::query(AssertSqlSafe(sql))
        .bind(first)
        .bind(second)
        .fetch_all(pool)
        .await
        .map_err(|e| Source::SQLx.to_error(500, e, action))
}

#[inline]
pub async fn query3_all(first: &str, second: &str, third: &str, sql: &str, pool: &Pool<MySql>, action: &str) -> Result<Vec<MySqlRow>, AppError> {
    sqlx::query(AssertSqlSafe(sql))
        .bind(first)
        .bind(second)
        .bind(third)
        .fetch_all(pool)
        .await
        .map_err(|e| Source::SQLx.to_error(500, e, action))
}

#[inline]
pub async fn query4_all(first: &str, second: &str, third: &str, fourth: &str, sql: &str, pool: &Pool<MySql>, action: &str) -> Result<Vec<MySqlRow>, AppError> {
    sqlx::query(AssertSqlSafe(sql))
        .bind(first)
        .bind(second)
        .bind(third)
        .bind(fourth)
        .fetch_all(pool)
        .await
        .map_err(|e| Source::SQLx.to_error(500, e, action))
}

#[allow(clippy::too_many_arguments)]
#[inline]
pub async fn query5_all(first: &str, second: &str, third: &str, fourth: &str, fifth: &str, sql: &str, pool: &Pool<MySql>, action: &str) -> Result<Vec<MySqlRow>, AppError> {
    sqlx::query(AssertSqlSafe(sql))
        .bind(first)
        .bind(second)
        .bind(third)
        .bind(fourth)
        .bind(fifth)
        .fetch_all(pool)
        .await
        .map_err(|e| Source::SQLx.to_error(500, e, action))
}

#[allow(clippy::too_many_arguments)]
#[inline]
pub async fn query6_all(first: &str, second: &str, third: &str, fourth: &str, fifth: &str, sixth: &str, sql: &str, pool: &Pool<MySql>, action: &str) -> Result<Vec<MySqlRow>, AppError> {
    sqlx::query(AssertSqlSafe(sql))
        .bind(first)
        .bind(second)
        .bind(third)
        .bind(fourth)
        .bind(fifth)
        .bind(sixth)
        .fetch_all(pool)
        .await
        .map_err(|e| Source::SQLx.to_error(500, e, action))
}

//================//
// fetch optional //
//================//
#[inline]
pub async fn query_opt(sql: &str, pool: &Pool<MySql>, action: &str) -> Result<Option<MySqlRow>, AppError> {
    sqlx::query(AssertSqlSafe(sql)).fetch_optional(pool).await.map_err(|e| Source::SQLx.to_error(500, e, action))
}

#[inline]
pub async fn query1_opt(first: &str, sql: &str, pool: &Pool<MySql>, action: &str) -> Result<Option<MySqlRow>, AppError> {
    sqlx::query(AssertSqlSafe(sql))
        .bind(first)
        .fetch_optional(pool)
        .await
        .map_err(|e| Source::SQLx.to_error(500, e, action))
}

#[inline]
pub async fn query2_opt(first: &str, second: &str, sql: &str, pool: &Pool<MySql>, action: &str) -> Result<Option<MySqlRow>, AppError> {
    sqlx::query(AssertSqlSafe(sql))
        .bind(first)
        .bind(second)
        .fetch_optional(pool)
        .await
        .map_err(|e| Source::SQLx.to_error(500, e, action))
}

#[inline]
pub async fn query3_opt(first: &str, second: &str, third: &str, sql: &str, pool: &Pool<MySql>, action: &str) -> Result<Option<MySqlRow>, AppError> {
    sqlx::query(AssertSqlSafe(sql))
        .bind(first)
        .bind(second)
        .bind(third)
        .fetch_optional(pool)
        .await
        .map_err(|e| Source::SQLx.to_error(500, e, action))
}
