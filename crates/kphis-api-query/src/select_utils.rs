use sqlx::{Error, FromRow, MySql, Pool, Row, mysql::MySqlRow};

use kphis_model::select_utils::{ColorSelectOption, SelectOption};
use kphis_util::error::{AppError, Source};

use super::query_all;

// single use, maybe change!!
// SelectUtils::getColorSelectOption
pub async fn get_color_select_options(sql: &str, pool: &Pool<MySql>) -> Result<Vec<ColorSelectOption>, AppError> {
    let options = query_all(sql, pool, "Select Color Select Options").await?;

    options
        .iter()
        .map(|row| color_select_option_from_row_u64_key(row).map_err(|e| Source::SQLx.to_error(500, e, "Select Color Select Options-u64")))
        .collect()
}
fn color_select_option_from_row_u64_key(row: &MySqlRow) -> Result<ColorSelectOption, Error> {
    let key: u64 = row.try_get("key")?;
    Ok(ColorSelectOption {
        key: key.to_string(),
        value: row.try_get("value")?,
        color: row.try_get("color")?,
    })
}

// general use
// SelectUtils::getSelectOption
pub async fn get_select_option(sql: &str, pool: &Pool<MySql>) -> Result<Vec<SelectOption>, AppError> {
    let options = query_all(sql, pool, "Select Select Options").await?;

    options
        .iter()
        .map(SelectOption::from_row)
        .collect::<sqlx::Result<Vec<SelectOption>>>()
        .map_err(|e| Source::SQLx.to_error(500, e, "Select Select Options"))
}

pub async fn get_select_option_i8_key(sql: &str, pool: &Pool<MySql>) -> Result<Vec<SelectOption>, AppError> {
    let options = query_all(sql, pool, "Select Select Options-i8").await?;

    options
        .iter()
        .map(|row| select_option_from_row_i8_key(row).map_err(|e| Source::SQLx.to_error(500, e, "Select Select Options-i8")))
        .collect()
}
fn select_option_from_row_i8_key(row: &MySqlRow) -> Result<SelectOption, Error> {
    let key: i8 = row.try_get("key")?;
    Ok(SelectOption {
        key: key.to_string(),
        value: row.try_get("value")?,
    })
}

pub async fn get_select_option_i32_key(sql: &str, pool: &Pool<MySql>) -> Result<Vec<SelectOption>, AppError> {
    let options = query_all(sql, pool, "Select Select Options-i32").await?;

    options
        .iter()
        .map(|row| select_option_from_row_i32_key(row).map_err(|e| Source::SQLx.to_error(500, e, "Select Select Options-i32")))
        .collect()
}
fn select_option_from_row_i32_key(row: &MySqlRow) -> Result<SelectOption, Error> {
    let key: i32 = row.try_get("key")?;
    Ok(SelectOption {
        key: key.to_string(),
        value: row.try_get("value")?,
    })
}

pub async fn get_select_option_u32_key(sql: &str, pool: &Pool<MySql>) -> Result<Vec<SelectOption>, AppError> {
    let options = query_all(sql, pool, "Select Select Options-u32").await?;

    options
        .iter()
        .map(|row| select_option_from_row_u32_key(row).map_err(|e| Source::SQLx.to_error(500, e, "Select Select Options-u32")))
        .collect()
}
fn select_option_from_row_u32_key(row: &MySqlRow) -> Result<SelectOption, Error> {
    let key: u32 = row.try_get("key")?;
    Ok(SelectOption {
        key: key.to_string(),
        value: row.try_get("value")?,
    })
}
