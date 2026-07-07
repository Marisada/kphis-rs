use sqlx::{AssertSqlSafe, Connection, Executor, MySql, MySqlConnection, Pool};
use std::thread;
use tokio::runtime::Runtime;

pub const TEST_URL: &str = "mysql://test_user:test_pass@localhost:3306";
pub const HOSXP_TEST: &str = "hos";
pub const KPHIS_TEST: &str = "kphis";
pub const KPHIS_LOG_TEST: &str = "kphis_log";
pub const KPHIS_EXTRA_TEST: &str = "kphis_extra";

/// Mocker that start with all data, clean on drop
pub struct MySqlMocker {
    has_hosxp: bool,
    has_kphis: bool,
    has_kphis_log: bool,
    pub has_kphis_extra: bool,
    pub hosxp: String,
    pub kphis: String,
    pub kphis_log: String,
    pub kphis_extra: String,
    pub db_pool: Pool<MySql>,
}

impl MySqlMocker {
    pub async fn new_all() -> Self {
        let has_hosxp = true;
        let has_kphis = true;
        let has_kphis_log = true;
        let has_kphis_extra = true;
        let db_pool = create_pool(has_hosxp, has_kphis, has_kphis_log, has_kphis_extra).await;

        let create_hosxp = include_dir::include_dir!("$CARGO_MANIFEST_DIR/test_sqls/create/hosxp");
        for file in create_hosxp.files() {
            if let Some(query) = file.contents_utf8() {
                db_pool.execute(query).await.expect("Error while querying the create table");
            }
        }
        let create_kphis = include_dir::include_dir!("$CARGO_MANIFEST_DIR/test_sqls/create/kphis");
        for file in create_kphis.files() {
            if let Some(query) = file.contents_utf8() {
                db_pool.execute(query).await.expect("Error while querying the create table");
            }
        }
        let create_kphis_extra = include_dir::include_dir!("$CARGO_MANIFEST_DIR/test_sqls/create/kphis_extra");
        for file in create_kphis_extra.files() {
            if let Some(query) = file.contents_utf8() {
                db_pool.execute(query).await.expect("Error while querying the create table");
            }
        }
        let create_kphis_log = include_dir::include_dir!("$CARGO_MANIFEST_DIR/test_sqls/create/kphis_log");
        for file in create_kphis_log.files() {
            if let Some(query) = file.contents_utf8() {
                db_pool.execute(query).await.expect("Error while querying the create table");
            }
        }
        let insert_hosxp = include_dir::include_dir!("$CARGO_MANIFEST_DIR/test_sqls/insert/hosxp");
        for file in insert_hosxp.files() {
            if let Some(query) = file.contents_utf8() {
                db_pool.execute(query).await.expect("Error while querying the insert table");
            }
        }
        let insert_kphis = include_dir::include_dir!("$CARGO_MANIFEST_DIR/test_sqls/insert/kphis");
        for file in insert_kphis.files() {
            if let Some(query) = file.contents_utf8() {
                db_pool.execute(query).await.expect("Error while querying the insert table");
            }
        }
        let insert_kphis_extra = include_dir::include_dir!("$CARGO_MANIFEST_DIR/test_sqls/insert/kphis_extra");
        for file in insert_kphis_extra.files() {
            if let Some(query) = file.contents_utf8() {
                db_pool.execute(query).await.expect("Error while querying the insert table");
            }
        }
        let insert_kphis_log = include_dir::include_dir!("$CARGO_MANIFEST_DIR/test_sqls/insert/kphis_log");
        for file in insert_kphis_log.files() {
            if let Some(query) = file.contents_utf8() {
                db_pool.execute(query).await.expect("Error while querying the insert table");
            }
        }

        Self {
            has_hosxp,
            has_kphis,
            has_kphis_log,
            has_kphis_extra,
            hosxp: HOSXP_TEST.to_owned(),
            kphis: KPHIS_TEST.to_owned(),
            kphis_log: KPHIS_LOG_TEST.to_owned(),
            kphis_extra: KPHIS_EXTRA_TEST.to_owned(),
            db_pool,
        }
    }

    pub async fn new_no_database() -> Self {
        let has_hosxp = false;
        let has_kphis = false;
        let has_kphis_log = false;
        let has_kphis_extra = false;
        let db_pool = create_pool(has_hosxp, has_kphis, has_kphis_log, has_kphis_extra).await;

        Self {
            has_hosxp,
            has_kphis,
            has_kphis_log,
            has_kphis_extra,
            hosxp: HOSXP_TEST.to_owned(),
            kphis: KPHIS_TEST.to_owned(),
            kphis_log: KPHIS_LOG_TEST.to_owned(),
            kphis_extra: KPHIS_EXTRA_TEST.to_owned(),
            db_pool,
        }
    }

    pub async fn new_hosxp() -> Self {
        let has_hosxp = true;
        let has_kphis = false;
        let has_kphis_log = false;
        let has_kphis_extra = false;
        let db_pool = create_pool(has_hosxp, has_kphis, has_kphis_log, has_kphis_extra).await;

        let create_hosxp = include_dir::include_dir!("$CARGO_MANIFEST_DIR/test_sqls/create/hosxp");
        for file in create_hosxp.files() {
            if let Some(query) = file.contents_utf8() {
                db_pool.execute(query).await.expect("Error while querying the create table");
            }
        }
        let insert_hosxp = include_dir::include_dir!("$CARGO_MANIFEST_DIR/test_sqls/insert/hosxp");
        for file in insert_hosxp.files() {
            if let Some(query) = file.contents_utf8() {
                db_pool.execute(query).await.expect("Error while querying the insert table");
            }
        }

        Self {
            has_hosxp,
            has_kphis,
            has_kphis_log,
            has_kphis_extra,
            hosxp: HOSXP_TEST.to_owned(),
            kphis: KPHIS_TEST.to_owned(),
            kphis_log: KPHIS_LOG_TEST.to_owned(),
            kphis_extra: KPHIS_EXTRA_TEST.to_owned(),
            db_pool,
        }
    }

    pub async fn new_kphis() -> Self {
        let has_hosxp = false;
        let has_kphis = true;
        let has_kphis_log = false;
        let has_kphis_extra = false;
        let db_pool = create_pool(has_hosxp, has_kphis, has_kphis_log, has_kphis_extra).await;

        let create_kphis = include_dir::include_dir!("$CARGO_MANIFEST_DIR/test_sqls/create/kphis");
        for file in create_kphis.files() {
            if let Some(query) = file.contents_utf8() {
                db_pool.execute(query).await.expect("Error while querying the create table");
            }
        }
        let insert_kphis = include_dir::include_dir!("$CARGO_MANIFEST_DIR/test_sqls/insert/kphis");
        for file in insert_kphis.files() {
            if let Some(query) = file.contents_utf8() {
                db_pool.execute(query).await.expect("Error while querying the insert table");
            }
        }

        Self {
            has_hosxp,
            has_kphis,
            has_kphis_log,
            has_kphis_extra,
            hosxp: HOSXP_TEST.to_owned(),
            kphis: KPHIS_TEST.to_owned(),
            kphis_log: KPHIS_LOG_TEST.to_owned(),
            kphis_extra: KPHIS_EXTRA_TEST.to_owned(),
            db_pool,
        }
    }

    pub async fn new_kphis_log() -> Self {
        let has_hosxp = false;
        let has_kphis = false;
        let has_kphis_log = true;
        let has_kphis_extra = false;
        let db_pool = create_pool(has_hosxp, has_kphis, has_kphis_log, has_kphis_extra).await;

        let create_kphis_log = include_dir::include_dir!("$CARGO_MANIFEST_DIR/test_sqls/create/kphis_log");
        for file in create_kphis_log.files() {
            if let Some(query) = file.contents_utf8() {
                db_pool.execute(query).await.expect("Error while querying the create table");
            }
        }
        let insert_kphis_log = include_dir::include_dir!("$CARGO_MANIFEST_DIR/test_sqls/insert/kphis_log");
        for file in insert_kphis_log.files() {
            if let Some(query) = file.contents_utf8() {
                db_pool.execute(query).await.expect("Error while querying the insert table");
            }
        }

        Self {
            has_hosxp,
            has_kphis,
            has_kphis_log,
            has_kphis_extra,
            hosxp: HOSXP_TEST.to_owned(),
            kphis: KPHIS_TEST.to_owned(),
            kphis_log: KPHIS_LOG_TEST.to_owned(),
            kphis_extra: KPHIS_EXTRA_TEST.to_owned(),
            db_pool,
        }
    }

    pub async fn new_kphis_extra() -> Self {
        let has_hosxp = false;
        let has_kphis = false;
        let has_kphis_log = false;
        let has_kphis_extra = true;
        let db_pool = create_pool(has_hosxp, has_kphis, has_kphis_log, has_kphis_extra).await;

        let create_kphis_extra = include_dir::include_dir!("$CARGO_MANIFEST_DIR/test_sqls/create/kphis_extra");
        for file in create_kphis_extra.files() {
            if let Some(query) = file.contents_utf8() {
                db_pool.execute(query).await.expect("Error while querying the create table");
            }
        }
        let insert_kphis_extra = include_dir::include_dir!("$CARGO_MANIFEST_DIR/test_sqls/insert/kphis_extra");
        for file in insert_kphis_extra.files() {
            if let Some(query) = file.contents_utf8() {
                db_pool.execute(query).await.expect("Error while querying the insert table");
            }
        }

        Self {
            has_hosxp,
            has_kphis,
            has_kphis_log,
            has_kphis_extra,
            hosxp: HOSXP_TEST.to_owned(),
            kphis: KPHIS_TEST.to_owned(),
            kphis_log: KPHIS_LOG_TEST.to_owned(),
            kphis_extra: KPHIS_EXTRA_TEST.to_owned(),
            db_pool,
        }
    }
}

impl Drop for MySqlMocker {
    fn drop(&mut self) {
        let has_hosxp = self.has_hosxp;
        let has_kphis = self.has_kphis;
        let has_kphis_log = self.has_kphis_log;
        let has_kphis_extra = self.has_kphis_extra;
        thread::spawn(move || {
            let rt = Runtime::new().expect("Error while initiating async runtime");
            rt.block_on(async move {
                let mut conn = MySqlConnection::connect(TEST_URL).await.expect("Error while connecting to database");
                if has_hosxp {
                    conn.execute(AssertSqlSafe(["DROP DATABASE ", HOSXP_TEST, ";"].concat()))
                        .await
                        .expect("Error while querying the drop database");
                }
                if has_kphis {
                    conn.execute(AssertSqlSafe(["DROP DATABASE ", KPHIS_TEST, ";"].concat()))
                        .await
                        .expect("Error while querying the drop database");
                }
                if has_kphis_log {
                    conn.execute(AssertSqlSafe(["DROP DATABASE ", KPHIS_LOG_TEST, ";"].concat()))
                        .await
                        .expect("Error while querying the drop database");
                }
                if has_kphis_extra {
                    conn.execute(AssertSqlSafe(["DROP DATABASE ", KPHIS_EXTRA_TEST, ";"].concat()))
                        .await
                        .expect("Error while querying the drop database");
                }
            });
        })
        .join()
        .expect("failed to drop database");
    }
}

/// Tester that start with empty database(s), clean on drop
#[derive(Clone, Debug)]
pub struct MySqlTester {
    has_hosxp: bool,
    has_kphis: bool,
    has_kphis_log: bool,
    has_kphis_extra: bool,
    pub hosxp: String,
    pub kphis: String,
    pub kphis_log: String,
    pub kphis_extra: String,
    pub db_pool: Pool<MySql>,
}

impl MySqlTester {
    pub async fn new_all() -> Self {
        let has_hosxp = true;
        let has_kphis = true;
        let has_kphis_log = true;
        let has_kphis_extra = true;
        let db_pool = create_pool(has_hosxp, has_kphis, has_kphis_log, has_kphis_extra).await;
        Self {
            has_hosxp,
            has_kphis,
            has_kphis_log,
            has_kphis_extra,
            hosxp: HOSXP_TEST.to_owned(),
            kphis: KPHIS_TEST.to_owned(),
            kphis_log: KPHIS_LOG_TEST.to_owned(),
            kphis_extra: KPHIS_EXTRA_TEST.to_owned(),
            db_pool,
        }
    }
    pub async fn new_hosxp() -> Self {
        let has_hosxp = true;
        let has_kphis = false;
        let has_kphis_log = false;
        let has_kphis_extra = false;
        let db_pool = create_pool(has_hosxp, has_kphis, has_kphis_log, has_kphis_extra).await;
        Self {
            has_hosxp,
            has_kphis,
            has_kphis_log,
            has_kphis_extra,
            hosxp: HOSXP_TEST.to_owned(),
            kphis: KPHIS_TEST.to_owned(),
            kphis_log: KPHIS_LOG_TEST.to_owned(),
            kphis_extra: KPHIS_EXTRA_TEST.to_owned(),
            db_pool,
        }
    }
    pub async fn new_kphis() -> Self {
        let has_hosxp = false;
        let has_kphis = true;
        let has_kphis_log = false;
        let has_kphis_extra = false;
        let db_pool = create_pool(has_hosxp, has_kphis, has_kphis_log, has_kphis_extra).await;
        Self {
            has_hosxp,
            has_kphis,
            has_kphis_log,
            has_kphis_extra,
            hosxp: HOSXP_TEST.to_owned(),
            kphis: KPHIS_TEST.to_owned(),
            kphis_log: KPHIS_LOG_TEST.to_owned(),
            kphis_extra: KPHIS_EXTRA_TEST.to_owned(),
            db_pool,
        }
    }
    pub async fn new_kphis_log() -> Self {
        let has_hosxp = false;
        let has_kphis = false;
        let has_kphis_log = true;
        let has_kphis_extra = false;
        let db_pool = create_pool(has_hosxp, has_kphis, has_kphis_log, has_kphis_extra).await;
        Self {
            has_hosxp,
            has_kphis,
            has_kphis_log,
            has_kphis_extra,
            hosxp: HOSXP_TEST.to_owned(),
            kphis: KPHIS_TEST.to_owned(),
            kphis_log: KPHIS_LOG_TEST.to_owned(),
            kphis_extra: KPHIS_EXTRA_TEST.to_owned(),
            db_pool,
        }
    }
    pub async fn new_kphis_extra() -> Self {
        let has_hosxp = false;
        let has_kphis = false;
        let has_kphis_log = false;
        let has_kphis_extra = true;
        let db_pool = create_pool(has_hosxp, has_kphis, has_kphis_log, has_kphis_extra).await;
        Self {
            has_hosxp,
            has_kphis,
            has_kphis_log,
            has_kphis_extra,
            hosxp: HOSXP_TEST.to_owned(),
            kphis: KPHIS_TEST.to_owned(),
            kphis_log: KPHIS_LOG_TEST.to_owned(),
            kphis_extra: KPHIS_EXTRA_TEST.to_owned(),
            db_pool,
        }
    }
    pub async fn new_hosxp_and_kphis() -> Self {
        let has_hosxp = true;
        let has_kphis = true;
        let has_kphis_log = false;
        let has_kphis_extra = false;
        let db_pool = create_pool(has_hosxp, has_kphis, has_kphis_log, has_kphis_extra).await;
        Self {
            has_hosxp,
            has_kphis,
            has_kphis_log,
            has_kphis_extra,
            hosxp: HOSXP_TEST.to_owned(),
            kphis: KPHIS_TEST.to_owned(),
            kphis_log: KPHIS_LOG_TEST.to_owned(),
            kphis_extra: KPHIS_EXTRA_TEST.to_owned(),
            db_pool,
        }
    }
    pub async fn new_hosxp_and_kphis_log() -> Self {
        let has_hosxp = true;
        let has_kphis = false;
        let has_kphis_log = true;
        let has_kphis_extra = false;
        let db_pool = create_pool(has_hosxp, has_kphis, has_kphis_log, has_kphis_extra).await;
        Self {
            has_hosxp,
            has_kphis,
            has_kphis_log,
            has_kphis_extra,
            hosxp: HOSXP_TEST.to_owned(),
            kphis: KPHIS_TEST.to_owned(),
            kphis_log: KPHIS_LOG_TEST.to_owned(),
            kphis_extra: KPHIS_EXTRA_TEST.to_owned(),
            db_pool,
        }
    }
    pub async fn new_hosxp_and_kphis_extra() -> Self {
        let has_hosxp = true;
        let has_kphis = false;
        let has_kphis_log = false;
        let has_kphis_extra = true;
        let db_pool = create_pool(has_hosxp, has_kphis, has_kphis_log, has_kphis_extra).await;
        Self {
            has_hosxp,
            has_kphis,
            has_kphis_log,
            has_kphis_extra,
            hosxp: HOSXP_TEST.to_owned(),
            kphis: KPHIS_TEST.to_owned(),
            kphis_log: KPHIS_LOG_TEST.to_owned(),
            kphis_extra: KPHIS_EXTRA_TEST.to_owned(),
            db_pool,
        }
    }
    pub async fn new_hosxp_and_kphis_and_kphis_extra() -> Self {
        let has_hosxp = true;
        let has_kphis = true;
        let has_kphis_log = false;
        let has_kphis_extra = true;
        let db_pool = create_pool(has_hosxp, has_kphis, has_kphis_log, has_kphis_extra).await;
        Self {
            has_hosxp,
            has_kphis,
            has_kphis_log,
            has_kphis_extra,
            hosxp: HOSXP_TEST.to_owned(),
            kphis: KPHIS_TEST.to_owned(),
            kphis_log: KPHIS_LOG_TEST.to_owned(),
            kphis_extra: KPHIS_EXTRA_TEST.to_owned(),
            db_pool,
        }
    }
    pub async fn new_hosxp_and_kphis_and_kphis_log() -> Self {
        let has_hosxp = true;
        let has_kphis = true;
        let has_kphis_log = true;
        let has_kphis_extra = false;
        let db_pool = create_pool(has_hosxp, has_kphis, has_kphis_log, has_kphis_extra).await;
        Self {
            has_hosxp,
            has_kphis,
            has_kphis_log,
            has_kphis_extra,
            hosxp: HOSXP_TEST.to_owned(),
            kphis: KPHIS_TEST.to_owned(),
            kphis_log: KPHIS_LOG_TEST.to_owned(),
            kphis_extra: KPHIS_EXTRA_TEST.to_owned(),
            db_pool,
        }
    }
    pub async fn new_kphis_and_kphis_log() -> Self {
        let has_hosxp = false;
        let has_kphis = true;
        let has_kphis_log = true;
        let has_kphis_extra = false;
        let db_pool = create_pool(has_hosxp, has_kphis, has_kphis_log, has_kphis_extra).await;
        Self {
            has_hosxp,
            has_kphis,
            has_kphis_log,
            has_kphis_extra,
            hosxp: HOSXP_TEST.to_owned(),
            kphis: KPHIS_TEST.to_owned(),
            kphis_log: KPHIS_LOG_TEST.to_owned(),
            kphis_extra: KPHIS_EXTRA_TEST.to_owned(),
            db_pool,
        }
    }
    pub async fn new_kphis_and_kphis_extra() -> Self {
        let has_hosxp = false;
        let has_kphis = true;
        let has_kphis_log = false;
        let has_kphis_extra = true;
        let db_pool = create_pool(has_hosxp, has_kphis, has_kphis_log, has_kphis_extra).await;
        Self {
            has_hosxp,
            has_kphis,
            has_kphis_log,
            has_kphis_extra,
            hosxp: HOSXP_TEST.to_owned(),
            kphis: KPHIS_TEST.to_owned(),
            kphis_log: KPHIS_LOG_TEST.to_owned(),
            kphis_extra: KPHIS_EXTRA_TEST.to_owned(),
            db_pool,
        }
    }
    pub async fn new_kphis_and_kphis_log_and_kphis_extra() -> Self {
        let has_hosxp = false;
        let has_kphis = true;
        let has_kphis_log = true;
        let has_kphis_extra = true;
        let db_pool = create_pool(has_hosxp, has_kphis, has_kphis_log, has_kphis_extra).await;
        Self {
            has_hosxp,
            has_kphis,
            has_kphis_log,
            has_kphis_extra,
            hosxp: HOSXP_TEST.to_owned(),
            kphis: KPHIS_TEST.to_owned(),
            kphis_log: KPHIS_LOG_TEST.to_owned(),
            kphis_extra: KPHIS_EXTRA_TEST.to_owned(),
            db_pool,
        }
    }
}

impl Drop for MySqlTester {
    fn drop(&mut self) {
        let has_hosxp = self.has_hosxp;
        let has_kphis = self.has_kphis;
        let has_kphis_log = self.has_kphis_log;
        let has_kphis_extra = self.has_kphis_extra;
        thread::spawn(move || {
            let rt = Runtime::new().expect("Error while initiating async runtime");
            rt.block_on(async move {
                let mut conn = MySqlConnection::connect(TEST_URL).await.expect("Error while connecting to database");
                if has_hosxp {
                    conn.execute(AssertSqlSafe(["DROP DATABASE ", HOSXP_TEST, ";"].concat()))
                        .await
                        .expect("Error while querying the drop database");
                }
                if has_kphis {
                    conn.execute(AssertSqlSafe(["DROP DATABASE ", KPHIS_TEST, ";"].concat()))
                        .await
                        .expect("Error while querying the drop database");
                }
                if has_kphis_log {
                    conn.execute(AssertSqlSafe(["DROP DATABASE ", KPHIS_LOG_TEST, ";"].concat()))
                        .await
                        .expect("Error while querying the drop database");
                }
                if has_kphis_extra {
                    conn.execute(AssertSqlSafe(["DROP DATABASE ", KPHIS_EXTRA_TEST, ";"].concat()))
                        .await
                        .expect("Error while querying the drop database");
                }
            });
        })
        .join()
        .expect("failed to drop database");
    }
}

// if test database has remaining database, please run `sqlx_clean_tester()` below
async fn create_pool(has_hosxp: bool, has_kphis: bool, has_kphis_log: bool, has_kphis_extra: bool) -> Pool<MySql> {
    // let pool = PoolOptions::new().test_before_acquire(false).connect(TEST_URL).await.expect("Error while connecting to database");
    let pool = Pool::connect(TEST_URL).await.expect("Error while connecting to database");
    if has_kphis {
        pool.execute(AssertSqlSafe(["CREATE DATABASE ", KPHIS_TEST, ";"].concat()))
            .await
            .expect("Error while querying the create database");
    }
    if has_hosxp {
        pool.execute(AssertSqlSafe(["CREATE DATABASE ", HOSXP_TEST, " CHARACTER SET = 'tis620' COLLATE = 'tis620_thai_ci';"].concat()))
            .await
            .expect("Error while querying the create database");
    }
    if has_kphis_log {
        pool.execute(AssertSqlSafe(["CREATE DATABASE ", KPHIS_LOG_TEST, ";"].concat()))
            .await
            .expect("Error while querying the create database");
    }
    if has_kphis_extra {
        pool.execute(AssertSqlSafe(["CREATE DATABASE ", KPHIS_EXTRA_TEST, ";"].concat()))
            .await
            .expect("Error while querying the create database");
    }
    pool
}

// for demonstrate test
#[allow(dead_code)]
async fn insert_user(pool: &Pool<MySql>, title: &str, kphis_log: &str) -> sqlx::Result<()> {
    sqlx::query(AssertSqlSafe(["INSERT INTO ", kphis_log, ".todos (title) VALUES (?)"].concat()))
        .bind(title)
        .execute(pool)
        .await?;
    Ok(())
}

// for demonstrate test
#[allow(dead_code)]
async fn get_user(pool: &Pool<MySql>, kphis_log: &str) -> sqlx::Result<(u32, String)> {
    sqlx::query_as::<_, (u32, String)>(AssertSqlSafe(["SELECT id, title FROM ", kphis_log, ".todos;"].concat()))
        .fetch_one(pool)
        .await
}

#[cfg(test)]
#[rustfmt::skip]
mod tests {

    use crate::*;

    #[tokio::test]
    #[ignore]
    async fn sqlx_clean_tester() {
        let pool: Pool<MySql> = Pool::connect(TEST_URL).await.expect("Error while connecting to database");
        pool.execute(AssertSqlSafe(["DROP DATABASE IF EXISTS ", KPHIS_TEST, ";"].concat())).await.expect("Error while querying the drop database");
        pool.execute(AssertSqlSafe(["DROP DATABASE IF EXISTS ", HOSXP_TEST, ";"].concat())).await.expect("Error while querying the drop database");
        pool.execute(AssertSqlSafe(["DROP DATABASE IF EXISTS ", KPHIS_LOG_TEST, ";"].concat())).await.expect("Error while querying the drop database");
        pool.execute(AssertSqlSafe(["DROP DATABASE IF EXISTS ", KPHIS_EXTRA_TEST, ";"].concat())).await.expect("Error while querying the drop database");
    }

    #[tokio::test]
    #[ignore]
    async fn sqlx_mysql_tester_should_create_and_drop() {
        {
            let scope_tester = MySqlTester::new_kphis_log().await;
            sqlx::query(include_str!("../test_sqls/create/test_todos.sql")).execute(&scope_tester.db_pool).await.unwrap();
            insert_user(&scope_tester.db_pool, "test", &scope_tester.kphis_log).await.unwrap();
            let (id, title) = get_user(&scope_tester.db_pool, &scope_tester.kphis_log).await.unwrap();
            assert_eq!(id, 1);
            assert_eq!(title, "test");
        }

        let tester = MySqlTester::new_kphis_log().await;
        sqlx::query(include_str!("../test_sqls/create/test_todos.sql")).execute(&tester.db_pool).await.unwrap();
        insert_user(&tester.db_pool, "test", &tester.kphis_log).await.unwrap();
        let (id, title) = get_user(&tester.db_pool, &tester.kphis_log).await.unwrap();
        assert_eq!(id, 1);
        assert_eq!(title, "test");
    }

    #[tokio::test]
    #[ignore]
    async fn sqlx_mysql_mocker_should_create_and_drop() {
        {
            let scope_tester = MySqlMocker::new_all().await;
            sqlx::query(include_str!("../test_sqls/create/test_todos.sql")).execute(&scope_tester.db_pool).await.unwrap();
            insert_user(&scope_tester.db_pool, "test", KPHIS_LOG_TEST).await.unwrap();
            let (id, title) = get_user(&scope_tester.db_pool, KPHIS_LOG_TEST).await.unwrap();
            assert_eq!(id, 1);
            assert_eq!(title, "test");
        }

        let tester = MySqlMocker::new_all().await;
        sqlx::query(include_str!("../test_sqls/create/test_todos.sql")).execute(&tester.db_pool).await.unwrap();
        insert_user(&tester.db_pool, "test", KPHIS_LOG_TEST).await.unwrap();
        let (id, title) = get_user(&tester.db_pool, KPHIS_LOG_TEST).await.unwrap();
        assert_eq!(id, 1);
        assert_eq!(title, "test");
    }
}
