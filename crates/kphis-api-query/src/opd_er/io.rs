use sqlx::{
    AssertSqlSafe, FromRow, MySql, Pool, Row,
    mysql::{MySqlQueryResult, MySqlRow},
    types::time::{Date, Time},
};

use kphis_model::{
    fetch::ExecuteResponse,
    ipd::io::{IoDate, IoOnly, IoParams, IoShift},
    shift::NurseShift,
};
use kphis_sql::{
    data_history_utils::{KeyValue, SourceTable},
    opd_er::io,
};
use kphis_util::{
    error::{AppError, Source},
    util::zero_none,
};

use crate::log::insert_history_log;

// GET /opd-er/io-date/:opd_er_order_master_id
pub async fn get_io_date(opd_er_order_master_id: u32, pool: &Pool<MySql>, kphis: &str) -> Result<Vec<IoDate>, AppError> {
    let sql = io::select_io_date(kphis);
    let results = sqlx::query(AssertSqlSafe(sql))
        .bind(opd_er_order_master_id)
        .fetch_all(pool)
        .await
        .map_err(|e| Source::SQLx.to_error(500, e, "Select IoDate"))?
        .iter()
        .map(IoDate::from_row)
        .collect::<sqlx::Result<Vec<IoDate>>>()
        .map_err(|e| Source::SQLx.to_error(500, e, "Select IoDate"))?;

    Ok(results)
}

// odp-er-vital-sign-io-table.php
// GET /opd-er/io
pub async fn get_io_shift(
    params: &IoParams,
    shift_day_start: Time,
    shift_evening_start: Time,
    shift_night_start: Time,
    pool: &Pool<MySql>,
    hosxp: &str,
    kphis: &str,
) -> Result<Vec<IoShift>, AppError> {
    let sql = io::select_io(params, kphis, hosxp);
    let mut query = sqlx::query(AssertSqlSafe(sql));
    if let Some(io_id) = &params.io_id {
        query = query.bind(io_id);
    }
    if let Some(opd_er_order_master_id) = &params.opd_er_order_master_id {
        query = query.bind(opd_er_order_master_id);
    }
    if let Some(start_date) = &params.start_date {
        query = query.bind(start_date);
    }
    if let Some(end_date) = &params.end_date {
        query = query.bind(end_date);
    }
    let results = query
        .fetch_all(pool)
        .await
        .map_err(|e| Source::SQLx.to_error(500, e, "Select OpdErIoShift"))?
        .iter()
        .map(|row| from_row(row, shift_day_start, shift_evening_start, shift_night_start))
        .collect::<sqlx::Result<Vec<IoShift>>>()
        .map_err(|e| Source::SQLx.to_error(500, e, "Select OpdErIoShift"))?;

    Ok(results)
}
fn from_row(row: &MySqlRow, shift_day_start: Time, shift_evening_start: Time, shift_night_start: Time) -> sqlx::Result<IoShift> {
    let io_date: Date = row.try_get("opd_er_io_date")?;
    let io_time: Time = row.try_get("opd_er_io_time")?;
    let (shift_date, shift) = NurseShift::generate(shift_day_start, shift_evening_start, shift_night_start, io_date, io_time);
    Ok(IoShift {
        io_id: row.try_get("opd_er_io_id")?,
        io_date,
        io_time,
        io_parenteral_type: row.try_get("opd_er_io_parenteral_type")?,
        io_parenteral_name: row.try_get("opd_er_io_parenteral_name")?,
        io_parenteral_amount: row.try_get("opd_er_io_parenteral_amount")?,
        io_parenteral_absorb: row.try_get("opd_er_io_parenteral_absorb")?,
        io_parenteral_carry_forward: row.try_get("opd_er_io_parenteral_carry_forward")?,
        io_parenteral_remark: row.try_get("opd_er_io_parenteral_remark")?,
        io_oral_name: row.try_get("opd_er_io_oral_name")?,
        io_oral_amount: row.try_get("opd_er_io_oral_amount")?,
        io_oral_absorb: row.try_get("opd_er_io_oral_absorb")?,
        io_oral_carry_forward: row.try_get("opd_er_io_oral_carry_forward")?,
        io_oral_remark: row.try_get("opd_er_io_oral_remark")?,
        io_output_type: row.try_get("opd_er_io_output_type")?,
        io_output_amount: row.try_get("opd_er_io_output_amount")?,
        io_output_remark: row.try_get("opd_er_io_output_remark")?,
        version: row.try_get("version")?,
        user_name: row.try_get("user_name")?,
        entryposition: row.try_get("entryposition")?,
        shift_date: Some(shift_date),
        shift: Some(shift),
        an: None,
        opd_er_order_master_id: row.try_get("opd_er_order_master_id")?,
    })
}

pub async fn get_io_only(opd_er_order_master_id: u32, pool: &Pool<MySql>, kphis: &str) -> Result<Vec<IoOnly>, AppError> {
    let sql = io::select_io_only(kphis);
    let results = sqlx::query(AssertSqlSafe(sql))
        .bind(opd_er_order_master_id)
        .fetch_all(pool)
        .await
        .map_err(|e| Source::SQLx.to_error(500, e, "Select IoOnly"))?
        .iter()
        .map(from_row_only)
        .collect::<sqlx::Result<Vec<IoOnly>>>()
        .map_err(|e| Source::SQLx.to_error(500, e, "Select IoOnly"))?;

    Ok(results)
}
fn from_row_only(row: &MySqlRow) -> sqlx::Result<IoOnly> {
    Ok(IoOnly {
        io_id: row.try_get("opd_er_io_id")?,
        io_date: row.try_get("opd_er_io_date")?,
        io_time: row.try_get("opd_er_io_time")?,
        io_parenteral_type: row.try_get("opd_er_io_parenteral_type")?,
        io_parenteral_name: row.try_get("opd_er_io_parenteral_name")?,
        io_parenteral_amount: row.try_get("opd_er_io_parenteral_amount")?,
        io_parenteral_absorb: row.try_get("opd_er_io_parenteral_absorb")?,
        io_parenteral_carry_forward: row.try_get("opd_er_io_parenteral_carry_forward")?,
        io_parenteral_remark: row.try_get("opd_er_io_parenteral_remark")?,
        io_oral_name: row.try_get("opd_er_io_oral_name")?,
        io_oral_amount: row.try_get("opd_er_io_oral_amount")?,
        io_oral_absorb: row.try_get("opd_er_io_oral_absorb")?,
        io_oral_carry_forward: row.try_get("opd_er_io_oral_carry_forward")?,
        io_oral_remark: row.try_get("opd_er_io_oral_remark")?,
        io_output_type: row.try_get("opd_er_io_output_type")?,
        io_output_amount: row.try_get("opd_er_io_output_amount")?,
        io_output_remark: row.try_get("opd_er_io_output_remark")?,
        create_user: row.try_get("create_user")?,
        create_datetime: row.try_get("create_datetime")?,
        update_user: row.try_get("update_user")?,
        update_datetime: row.try_get("update_datetime")?,
        version: row.try_get("version")?,
    })
}

// opd-er-vital-sign-io-save.php
// opd-er-vital-sign-io-update.php
// POST /opd-er/io
pub async fn post_io_shift(form: &IoShift, user: &str, pool: &Pool<MySql>, kphis: &str, kphis_log: &str) -> Result<(u32, Vec<ExecuteResponse>), AppError> {
    if let Some(io_id) = zero_none(form.io_id) {
        let results = update_io_shift(io_id, form, user, pool, kphis, kphis_log).await?;

        Ok((io_id, results))
    } else {
        insert_io_shift(form, user, pool, kphis, kphis_log).await
    }
}

async fn update_io_shift(io_id: u32, form: &IoShift, user: &str, pool: &Pool<MySql>, kphis: &str, kphis_log: &str) -> Result<Vec<ExecuteResponse>, AppError> {
    let mut results = Vec::with_capacity(2);
    // 1. Update ipd_io
    let update_result = update_io(form, user, pool, kphis).await?;
    let is_update = update_result.rows_affected() > 0;
    results.push(ExecuteResponse::from_query_result(update_result, "Update OpdErIo"));
    // 2. Insert to history_ipd_focus_list
    if is_update {
        let insert_history_io_result = insert_history_log(SourceTable::OpdErIo, "U", user, &[KeyValue("opd_er_io_id", io_id.to_string())], kphis, kphis_log, pool).await?;
        results.push(ExecuteResponse::from_query_result(insert_history_io_result, "Insert OpdErIo History"));
    }

    Ok(results)
}

pub async fn insert_io_shift(form: &IoShift, user: &str, pool: &Pool<MySql>, kphis: &str, kphis_log: &str) -> Result<(u32, Vec<ExecuteResponse>), AppError> {
    let mut results = Vec::with_capacity(2);
    // 1. Insert ipd_io
    let insert_io_result = insert_io(form, user, pool, kphis).await?;
    let is_insert = insert_io_result.rows_affected() > 0;
    let id = insert_io_result.last_insert_id() as u32;
    results.push(ExecuteResponse::from_query_result(insert_io_result, "Insert OpdErIo"));
    // 2. Insert to history_ipd_focus_list
    if is_insert {
        let insert_history_io_result = insert_history_log(SourceTable::OpdErIo, "I", user, &[KeyValue("opd_er_io_id", id.to_string())], kphis, kphis_log, pool).await?;
        results.push(ExecuteResponse::from_query_result(insert_history_io_result, "Insert OpdErIo History"));
    }

    Ok((id, results))
}

pub async fn insert_io_only_bundle(opd_er_order_master_id: u32, only: &IoOnly, pool: &Pool<MySql>, kphis: &str, kphis_log: &str) -> Result<(u32, Vec<ExecuteResponse>), AppError> {
    let mut results = Vec::with_capacity(2);
    // 1. Insert ipd_io
    let insert_io_result = insert_io_only(opd_er_order_master_id, only, pool, kphis).await?;
    let is_insert = insert_io_result.rows_affected() > 0;
    let id = insert_io_result.last_insert_id() as u32;
    results.push(ExecuteResponse::from_query_result(insert_io_result, "Insert OpdErIoOnly"));
    // 2. Insert to history_ipd_focus_list
    if is_insert {
        let insert_history_io_result = insert_history_log(SourceTable::OpdErIo, "I", "system", &[KeyValue("opd_er_io_id", id.to_string())], kphis, kphis_log, pool).await?;
        results.push(ExecuteResponse::from_query_result(insert_history_io_result, "Insert OpdErIoOnly History"));
    }

    Ok((id, results))
}

async fn update_io(form: &IoShift, user: &str, pool: &Pool<MySql>, kphis: &str) -> Result<MySqlQueryResult, AppError> {
    let update_io_sql = io::update_io(kphis);
    sqlx::query(AssertSqlSafe(update_io_sql))
        .bind(form.io_date)
        .bind(form.io_time)
        .bind(&form.io_parenteral_type)
        .bind(&form.io_parenteral_name)
        .bind(form.io_parenteral_amount)
        .bind(form.io_parenteral_absorb)
        .bind(form.io_parenteral_carry_forward)
        .bind(&form.io_parenteral_remark)
        .bind(&form.io_oral_name)
        .bind(form.io_oral_amount)
        .bind(form.io_oral_absorb)
        .bind(form.io_oral_carry_forward)
        .bind(&form.io_oral_remark)
        .bind(&form.io_output_type)
        .bind(form.io_output_amount)
        .bind(&form.io_output_remark)
        .bind(user)
        .bind(form.io_id)
        .bind(form.version)
        .execute(pool)
        .await
        .map_err(|e| Source::SQLx.to_error(500, e, "Update OpdErIo"))
}

async fn insert_io(form: &IoShift, user: &str, pool: &Pool<MySql>, kphis: &str) -> Result<MySqlQueryResult, AppError> {
    let insert_io_sql = io::insert_io(kphis);
    sqlx::query(AssertSqlSafe(insert_io_sql))
        .bind(form.io_date)
        .bind(form.io_time)
        .bind(&form.io_parenteral_type)
        .bind(&form.io_parenteral_name)
        .bind(form.io_parenteral_amount)
        .bind(form.io_parenteral_absorb)
        .bind(form.io_parenteral_carry_forward)
        .bind(&form.io_parenteral_remark)
        .bind(&form.io_oral_name)
        .bind(form.io_oral_amount)
        .bind(form.io_oral_absorb)
        .bind(form.io_oral_carry_forward)
        .bind(&form.io_oral_remark)
        .bind(&form.io_output_type)
        .bind(form.io_output_amount)
        .bind(&form.io_output_remark)
        .bind(form.opd_er_order_master_id)
        .bind(user)
        .bind(user)
        .execute(pool)
        .await
        .map_err(|e| Source::SQLx.to_error(500, e, "Insert OpdErIo"))
}

async fn insert_io_only(opd_er_order_master_id: u32, only: &IoOnly, pool: &Pool<MySql>, kphis: &str) -> Result<MySqlQueryResult, AppError> {
    let insert_io_sql = io::insert_io_only(kphis);
    sqlx::query(AssertSqlSafe(insert_io_sql))
        .bind(only.io_date)
        .bind(only.io_time)
        .bind(&only.io_parenteral_type)
        .bind(&only.io_parenteral_name)
        .bind(only.io_parenteral_amount)
        .bind(only.io_parenteral_absorb)
        .bind(only.io_parenteral_carry_forward)
        .bind(&only.io_parenteral_remark)
        .bind(&only.io_oral_name)
        .bind(only.io_oral_amount)
        .bind(only.io_oral_absorb)
        .bind(only.io_oral_carry_forward)
        .bind(&only.io_oral_remark)
        .bind(&only.io_output_type)
        .bind(only.io_output_amount)
        .bind(&only.io_output_remark)
        .bind(opd_er_order_master_id)
        .bind(&only.create_user)
        .bind(only.create_datetime)
        .bind(&only.update_user)
        .bind(only.update_datetime)
        .bind(only.version)
        .execute(pool)
        .await
        .map_err(|e| Source::SQLx.to_error(500, e, "Insert IoOnly"))
}

// opd-er-vital-sign-io-delete.php
// DELETE /opd-er/io
pub async fn delete_io_shift(opd_er_io_id: u32, version: i32, user: &str, pool: &Pool<MySql>, kphis: &str, kphis_log: &str) -> Result<Vec<ExecuteResponse>, AppError> {
    let mut results = Vec::with_capacity(2);
    // 1. Insert to history
    let insert_history_io_result = insert_history_log(
        SourceTable::OpdErIo,
        "D",
        user,
        &[KeyValue("opd_er_io_id", opd_er_io_id.to_string()), KeyValue("version", version.to_string())],
        kphis,
        kphis_log,
        pool,
    )
    .await?;
    let will_delete = insert_history_io_result.rows_affected() > 0;
    results.push(ExecuteResponse::from_query_result(insert_history_io_result, "Insert OpdErIo History"));

    // 2. Delete focus list
    if will_delete {
        let delete_io_result = delete_io(opd_er_io_id, version, pool, kphis).await?;
        results.push(ExecuteResponse::from_query_result(delete_io_result, "Delete OpdErIo"));
    }

    Ok(results)
}

async fn delete_io(opd_er_io_id: u32, version: i32, pool: &Pool<MySql>, kphis: &str) -> Result<MySqlQueryResult, AppError> {
    let delete_io_sql = io::delete_io(kphis);
    sqlx::query(AssertSqlSafe(delete_io_sql))
        .bind(opd_er_io_id)
        .bind(version)
        .execute(pool)
        .await
        .map_err(|e| Source::SQLx.to_error(500, e, "Delete OpdErIo"))
}

#[cfg(test)]
#[rustfmt::skip]
mod tests {

    use super::*;
    use kphis_sqlx_tester::MySqlTester;
    use time::macros::time;
    use kphis_util::datetime::date_8601;

    #[tokio::test]
    #[ignore]
    async fn sqlx_get_io_date() {
        let tester = MySqlTester::new_kphis().await;
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis/opd_er_io.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/kphis/opd_er_io.sql")).execute(&tester.db_pool).await.unwrap();

        let found = get_io_date(1, &tester.db_pool, &tester.kphis).await.unwrap();
        assert_eq!(found.len(), 3);
        let not_found = get_io_date(999, &tester.db_pool, &tester.kphis).await.unwrap();
        assert!(not_found.is_empty());
    }

    #[tokio::test]
    #[ignore]
    async fn sqlx_get_io_shift() {
        let tester = MySqlTester::new_hosxp_and_kphis().await;
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis/opd_er_io.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/hosxp/opduser.sql")).execute(&tester.db_pool).await.unwrap();

        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/kphis/opd_er_io.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/hosxp/opduser.sql")).execute(&tester.db_pool).await.unwrap();

        let default = get_io_shift(&IoParams::default(),time!(08:00),time!(16:00),time!(00:00),&tester.db_pool,&tester.hosxp,&tester.kphis).await.unwrap();
        assert_eq!(default.len(), 3);
        let find_io_id = get_io_shift(&IoParams {io_id: Some(1),..Default::default()},time!(08:00),time!(16:00),time!(00:00),&tester.db_pool,&tester.hosxp,&tester.kphis).await.unwrap();
        assert_eq!(find_io_id.len(), 1);
        let find_an = get_io_shift(&IoParams {opd_er_order_master_id: Some(1),..Default::default()},time!(08:00),time!(16:00),time!(00:00),&tester.db_pool,&tester.hosxp,&tester.kphis).await.unwrap();
        assert_eq!(find_an.len(), 3);
        let find_between_date = get_io_shift(&IoParams {start_date: date_8601("2024-01-01"),end_date: date_8601("2024-01-21"),..Default::default()},time!(08:00),time!(16:00),time!(00:00),&tester.db_pool,&tester.hosxp,&tester.kphis).await.unwrap();
        assert_eq!(find_between_date.len(), 3);
        let find_after_start = get_io_shift(&IoParams {start_date: date_8601("2024-01-11"),..Default::default()},time!(08:00),time!(16:00),time!(00:00),&tester.db_pool,&tester.hosxp,&tester.kphis).await.unwrap();
        assert_eq!(find_after_start.len(), 2);
        let find_before_end = get_io_shift(&IoParams {end_date: date_8601("2024-01-11"),..Default::default()},time!(08:00),time!(16:00),time!(00:00),&tester.db_pool,&tester.hosxp,&tester.kphis).await.unwrap();
        assert_eq!(find_before_end.len(), 2);
        let not_found = get_io_shift(&IoParams {opd_er_order_master_id: Some(999),..Default::default()},time!(08:00),time!(16:00),time!(00:00),&tester.db_pool,&tester.hosxp,&tester.kphis).await.unwrap();
        assert!(not_found.is_empty());
    }

    #[tokio::test]
    #[ignore]
    async fn sqlx_get_io_only() {
        let tester = MySqlTester::new_kphis().await;
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis/opd_er_io.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/kphis/opd_er_io.sql")).execute(&tester.db_pool).await.unwrap();

        let found = get_io_only(1,&tester.db_pool,&tester.kphis).await.unwrap();
        assert_eq!(found.len(), 3);
        let not_found = get_io_only(999,&tester.db_pool,&tester.kphis).await.unwrap();
        assert!(not_found.is_empty());
    }

    #[tokio::test]
    #[ignore]
    async fn sqlx_insert_io() {
        let tester = MySqlTester::new_kphis().await;
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis/opd_er_io.sql")).execute(&tester.db_pool).await.unwrap();

        let success = insert_io(&IoShift::demo(), "user", &tester.db_pool, &tester.kphis).await.unwrap();
        assert_eq!(success.rows_affected(), 1);
        let again_success = insert_io(&IoShift::demo(), "user", &tester.db_pool, &tester.kphis).await.unwrap();
        assert_eq!(again_success.rows_affected(), 1);
    }

    #[tokio::test]
    #[ignore]
    async fn sqlx_insert_io_only() {
        let tester = MySqlTester::new_kphis().await;
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis/opd_er_io.sql")).execute(&tester.db_pool).await.unwrap();

        let success = insert_io_only(1, &IoOnly::demo(), &tester.db_pool, &tester.kphis).await.unwrap();
        assert_eq!(success.rows_affected(), 1);
        let again_success = insert_io_only(1, &IoOnly::demo(), &tester.db_pool, &tester.kphis).await.unwrap();
        assert_eq!(again_success.rows_affected(), 1);
    }

    #[tokio::test]
    #[ignore]
    async fn sqlx_update_io() {
        let tester = MySqlTester::new_kphis().await;
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis/opd_er_io.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/kphis/opd_er_io.sql")).execute(&tester.db_pool).await.unwrap();

        let success = update_io(&IoShift::demo(), "user", &tester.db_pool, &tester.kphis).await.unwrap();
        assert_eq!(success.rows_affected(), 1);
        let again_fail_version =update_io(&IoShift::demo(), "user", &tester.db_pool, &tester.kphis).await.unwrap();
        assert_eq!(again_fail_version.rows_affected(), 0);
    }

    #[tokio::test]
    #[ignore]
    async fn sqlx_delete_io() {
        let tester = MySqlTester::new_kphis().await;
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis/opd_er_io.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/kphis/opd_er_io.sql")).execute(&tester.db_pool).await.unwrap();

        let success = delete_io(1, 1, &tester.db_pool, &tester.kphis).await.unwrap();
        assert_eq!(success.rows_affected(), 1);
        let again_not_found = delete_io(1, 1, &tester.db_pool, &tester.kphis).await.unwrap();
        assert_eq!(again_not_found.rows_affected(), 0);
    }
}
