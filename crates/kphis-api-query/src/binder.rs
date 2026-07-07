use sqlx::{AssertSqlSafe, MySql, Pool, mysql::MySqlQueryResult};

use kphis_sql::{TABLE_CREATE_COLUMNS, TABLE_CREATE_PREPARED, TABLE_UPDATE_SET};
use kphis_util::error::{AppError, Source};

// TODO apply to MysqlBinder crate
pub trait MysqlBinder<B: Binder> {
    fn get_struct_name_snake(&self) -> String;
    fn get_field_names(&self) -> Vec<&str>;
    fn get_field_enums(&self) -> Vec<B>;
}

// TODO apply to MysqlBinder crate
pub trait Binder {
    fn bind(self, query: sqlx::query::Query<'_, sqlx::MySql, sqlx::mysql::MySqlArguments>) -> sqlx::query::Query<'_, sqlx::MySql, sqlx::mysql::MySqlArguments>;
}

pub async fn insert_binder<T: MysqlBinder<B>, B: Binder>(binder: &T, user: &str, pool: &Pool<MySql>, kphis: &str) -> Result<MySqlQueryResult, AppError> {
    let tbname = binder.get_struct_name_snake();
    let keys = binder.get_field_names();
    let params = binder.get_field_enums();
    let sql = [
        "INSERT INTO ",
        kphis,
        ".",
        &tbname,
        " (",
        &keys.join(","),
        TABLE_CREATE_COLUMNS,
        ") VALUE (",
        &vec!["?"; keys.len()].join(","),
        TABLE_CREATE_PREPARED,
        ");",
    ]
    .concat();

    let mut query = sqlx::query(AssertSqlSafe(sql));
    for param in params {
        query = param.bind(query);
    }
    query.bind(user).bind(user).execute(pool).await.map_err(|e| Source::SQLx.to_error(500, e, "Binder Insert"))
}

pub async fn update_binder<T: MysqlBinder<B>, B: Binder>(index: &str, binder: &T, user: &str, pool: &Pool<MySql>, kphis: &str) -> Result<MySqlQueryResult, AppError> {
    let tbname = binder.get_struct_name_snake();
    let mut keys = binder.get_field_names();
    let mut params = binder.get_field_enums();

    let position = keys.iter().position(|k| *k == index).ok_or_else(|| Source::App.to_error(500, "Table Key Not Found", "Binder Update"))?;
    let removed_keys = keys.swap_remove(position);
    let removed_param = params.swap_remove(position);

    let sql = [
        "UPDATE ",
        kphis,
        ".",
        &tbname,
        " SET ",
        &keys.iter().map(|k| [k, "=?"].concat()).collect::<Vec<String>>().join(","),
        TABLE_UPDATE_SET,
        " WHERE ",
        removed_keys,
        "=?;",
    ]
    .concat();

    let mut query = sqlx::query(AssertSqlSafe(sql));
    for param in params {
        query = param.bind(query);
    }
    query = removed_param.bind(query.bind(user));
    query.execute(pool).await.map_err(|e| Source::SQLx.to_error(500, e, "Binder Update"))
}
