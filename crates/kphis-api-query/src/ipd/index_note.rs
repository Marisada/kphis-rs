use sqlx::{AssertSqlSafe, FromRow, MySql, Pool, Row, mysql::MySqlQueryResult};

use kphis_model::{
    fetch::ExecuteResponse,
    ipd::index_note::{IndexNote, IndexNoteParams},
};
use kphis_sql::ipd::index_note;
use kphis_util::{
    error::{AppError, Source},
    util::zero_none,
};

pub async fn get_index_note(params: &IndexNoteParams, pool: &Pool<MySql>, kphis: &str) -> Result<Vec<IndexNote>, AppError> {
    let sql = index_note::select_ipd_nurse_index_note(params, kphis);
    let mut query = sqlx::query(AssertSqlSafe(sql));
    if let Some(nurse_index_note_id) = params.nurse_index_note_id {
        query = query.bind(nurse_index_note_id);
    }
    if let Some(an) = &params.an {
        query = query.bind(an);
    }
    query
        .fetch_all(pool)
        .await
        .map_err(|e| Source::SQLx.to_error(500, e, "Select IndexNote"))?
        .iter()
        .map(IndexNote::from_row)
        .collect::<sqlx::Result<Vec<IndexNote>>>()
        .map_err(|e| Source::SQLx.to_error(500, e, "Select IndexNote"))
}

pub async fn get_index_note_only(an: &str, pool: &Pool<MySql>, kphis: &str) -> Result<Vec<Option<String>>, AppError> {
    let sql = index_note::select_ipd_nurse_index_note_only(kphis);
    sqlx::query(AssertSqlSafe(sql))
        .bind(an)
        .fetch_all(pool)
        .await
        .map_err(|e| Source::SQLx.to_error(500, e, "Select IndexNoteOnly"))?
        .iter()
        .map(|row| row.try_get("nurse_index_note"))
        .collect::<sqlx::Result<Vec<Option<String>>>>()
        .map_err(|e| Source::SQLx.to_error(500, e, "Select IndexNoteOnly"))
}

// ipd-nurse-index-note-save.php
pub async fn post_index_note(save: &IndexNote, user: &str, pool: &Pool<MySql>, kphis: &str) -> Result<(u32, ExecuteResponse), AppError> {
    if let Some(id) = zero_none(save.nurse_index_note_id) {
        let update_note_result = update_ipd_nurse_index_note(id, save, user, pool, kphis).await?;
        Ok((id, ExecuteResponse::from_query_result(update_note_result, "Update IpdNurseIndexNote")))
    } else {
        let insert_note_result = insert_ipd_nurse_index_note(&save.an, &save.nurse_index_note, user, pool, kphis).await?;
        Ok((
            insert_note_result.last_insert_id() as u32,
            ExecuteResponse::from_query_result(insert_note_result, "Insert IpdNurseIndexNote"),
        ))
    }
}

async fn update_ipd_nurse_index_note(nurse_index_note_id: u32, save: &IndexNote, user: &str, pool: &Pool<MySql>, kphis: &str) -> Result<MySqlQueryResult, AppError> {
    let update_note_sql = index_note::update_ipd_nurse_index_note(kphis);
    sqlx::query(AssertSqlSafe(update_note_sql))
        .bind(&save.nurse_index_note)
        .bind(user)
        .bind(nurse_index_note_id)
        .execute(pool)
        .await
        .map_err(|e| Source::SQLx.to_error(500, e, "Update IpdNurseIndexNote"))
}

async fn insert_ipd_nurse_index_note(an: &Option<String>, nurse_index_note: &Option<String>, user: &str, pool: &Pool<MySql>, kphis: &str) -> Result<MySqlQueryResult, AppError> {
    let insert_note_sql = index_note::insert_ipd_nurse_index_note(kphis);
    sqlx::query(AssertSqlSafe(insert_note_sql))
        .bind(an)
        .bind(nurse_index_note)
        .bind(user)
        .bind(user)
        .execute(pool)
        .await
        .map_err(|e| Source::SQLx.to_error(500, e, "Insert IpdNurseIndexNote"))
}

// ipd-nurse-index-note-delete.php
pub async fn delete_index_note(nurse_index_note_id: u32, pool: &Pool<MySql>, kphis: &str) -> Result<ExecuteResponse, AppError> {
    let sql = index_note::delete_ipd_nurse_index_note(kphis);
    let delete_result = sqlx::query(AssertSqlSafe(sql))
        .bind(nurse_index_note_id)
        .execute(pool)
        .await
        .map_err(|e| Source::SQLx.to_error(500, e, "Delete IndexNote"))?;

    Ok(ExecuteResponse::from_query_result(delete_result, "Delete IndexNote"))
}

#[cfg(test)]
#[rustfmt::skip]
mod tests {

    use super::*;
    use kphis_sqlx_tester::MySqlTester;

    #[tokio::test]
    #[ignore]
    async fn sqlx_get_index_note() {
        let tester = MySqlTester::new_kphis().await;
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis/ipd_nurse_index_note.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/kphis/ipd_nurse_index_note.sql")).execute(&tester.db_pool).await.unwrap();

        let default = get_index_note(&IndexNoteParams::default(), &tester.db_pool, &tester.kphis).await.unwrap();
        assert_eq!(default.len(), 1);
        let found_an = get_index_note(&IndexNoteParams {an: Some(String::from("660001234")),..Default::default()},&tester.db_pool,&tester.kphis).await.unwrap();
        assert_eq!(found_an.len(), 1);
        let found_id = get_index_note(&IndexNoteParams {nurse_index_note_id: Some(1),..Default::default()},&tester.db_pool,&tester.kphis).await.unwrap();
        assert_eq!(found_id.len(), 1);
        let not_found = get_index_note(&IndexNoteParams {nurse_index_note_id: Some(999),..Default::default()},&tester.db_pool,&tester.kphis).await.unwrap();
        assert!(not_found.is_empty());
    }

    #[tokio::test]
    #[ignore]
    async fn sqlx_get_index_note_only() {
        let tester = MySqlTester::new_kphis().await;
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis/ipd_nurse_index_note.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/kphis/ipd_nurse_index_note.sql")).execute(&tester.db_pool).await.unwrap();

        let found = get_index_note_only("660001234", &tester.db_pool, &tester.kphis).await.unwrap();
        assert_eq!(found.len(), 1);
        let not_found = get_index_note_only("660006666",&tester.db_pool,&tester.kphis).await.unwrap();
        assert!(not_found.is_empty());
    }

    #[tokio::test]
    #[ignore]
    async fn sqlx_insert_ipd_nurse_index_note() {
        let tester = MySqlTester::new_kphis().await;
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis/ipd_nurse_index_note.sql")).execute(&tester.db_pool).await.unwrap();

        let note = IndexNote::demo();
        let success = insert_ipd_nurse_index_note(&note.an, &note.nurse_index_note, "user", &tester.db_pool, &tester.kphis).await.unwrap();
        assert_eq!(success.rows_affected(), 1);
        let again_error_duplicate_an = insert_ipd_nurse_index_note(&note.an, &note.nurse_index_note, "user", &tester.db_pool, &tester.kphis).await;
        assert!(again_error_duplicate_an.is_err());
    }

    #[tokio::test]
    #[ignore]
    async fn sqlx_update_ipd_nurse_index_note() {
        let tester = MySqlTester::new_kphis().await;
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis/ipd_nurse_index_note.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/kphis/ipd_nurse_index_note.sql")).execute(&tester.db_pool).await.unwrap();

        let success = update_ipd_nurse_index_note(1,&IndexNote::demo(),"user",&tester.db_pool,&tester.kphis).await.unwrap();
        assert_eq!(success.rows_affected(), 1);
        let again_success = update_ipd_nurse_index_note(1,&IndexNote::demo(),"user",&tester.db_pool,&tester.kphis).await.unwrap();
        assert_eq!(again_success.rows_affected(), 1);
    }

    #[tokio::test]
    #[ignore]
    async fn sqlx_delete_index_note() {
        let tester = MySqlTester::new_kphis().await;
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis/ipd_nurse_index_note.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/kphis/ipd_nurse_index_note.sql")).execute(&tester.db_pool).await.unwrap();

        let success = delete_index_note(1, &tester.db_pool, &tester.kphis).await.unwrap();
        assert_eq!(success.rows_affected, 1);
        let again_not_found = delete_index_note(1, &tester.db_pool, &tester.kphis).await.unwrap();
        assert_eq!(again_not_found.rows_affected, 0);
    }
}
