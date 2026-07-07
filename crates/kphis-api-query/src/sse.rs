use sqlx::{AssertSqlSafe, FromRow, MySql, Pool};
use time::PrimitiveDateTime;

use kphis_model::{
    fetch::ExecuteResponse,
    sse::{SseData, SseMessage, SseMessageParams, SsePostMessage},
};
use kphis_sql::sse;
use kphis_util::error::{AppError, Source};

pub async fn get_sse_message(
    params: &SseMessageParams,
    user: &str,
    doctorcode: &Option<String>,
    wards: &[String],
    spclty_ids: &[u32],
    pool: &Pool<MySql>,
    kphis_log: &str,
) -> Result<Vec<SseMessage>, AppError> {
    let result = if let Some(my_code) = doctorcode {
        if let Some(cat) = &params.cat {
            match cat.as_str() {
                "global" => get_sse_message_global(50, params, user, my_code, pool, kphis_log).await?.collect(),
                "ward" => {
                    if !wards.is_empty() {
                        get_sse_message_ward(50, params, user, my_code, wards, pool, kphis_log).await?.collect()
                    } else {
                        Vec::new()
                    }
                }
                "spclty" => {
                    if !spclty_ids.is_empty() {
                        get_sse_message_spclty(50, params, user, my_code, spclty_ids, pool, kphis_log).await?.collect()
                    } else {
                        Vec::new()
                    }
                }
                "private" => get_sse_message_private(50, params, user, my_code, pool, kphis_log).await?.collect(),
                _ => Vec::new(),
            }
        } else {
            // all group
            let mut result = Vec::new();
            // global
            result.extend(get_sse_message_global(10, params, user, my_code, pool, kphis_log).await?);
            // ward
            if !wards.is_empty() {
                result.extend(get_sse_message_ward(10, params, user, my_code, wards, pool, kphis_log).await?);
            }
            // spclty
            if !spclty_ids.is_empty() {
                result.extend(get_sse_message_spclty(10, params, user, my_code, spclty_ids, pool, kphis_log).await?);
            }
            // private
            result.extend(get_sse_message_private(10, params, user, my_code, pool, kphis_log).await?);

            result
        }
    } else {
        Vec::new()
    };

    Ok(result)
}

async fn get_sse_message_global(limit: usize, params: &SseMessageParams, user: &str, doctorcode: &str, pool: &Pool<MySql>, kphis_log: &str) -> Result<impl Iterator<Item = SseMessage>, AppError> {
    let sql = sse::select_sse_message_global(params.min_id.is_some(), kphis_log, limit);
    let mut query = sqlx::query(AssertSqlSafe(sql)).bind(user).bind(doctorcode);
    if let Some(min_id) = params.min_id {
        query = query.bind(min_id);
    }
    let result = query
        .fetch_all(pool)
        .await
        .map_err(|e| Source::SQLx.to_error(500, e, "Select Global SseMessage"))?
        .iter()
        .map(SseData::from_row)
        .collect::<sqlx::Result<Vec<SseData>>>()
        .map_err(|e| Source::SQLx.to_error(500, e, "Select Global SseMessage"))?
        .into_iter()
        .map(SseMessage::GlobalMsg);

    Ok(result)
}

async fn get_sse_message_ward(
    limit: usize,
    params: &SseMessageParams,
    user: &str,
    doctorcode: &str,
    wards: &[String],
    pool: &Pool<MySql>,
    kphis_log: &str,
) -> Result<impl Iterator<Item = SseMessage>, AppError> {
    let sql = sse::select_sse_message_ward(wards, params.min_id.is_some(), kphis_log, limit);
    let mut query = sqlx::query(AssertSqlSafe(sql)).bind(user).bind(doctorcode);
    if let Some(min_id) = params.min_id {
        query = query.bind(min_id);
    }
    let result = query
        .fetch_all(pool)
        .await
        .map_err(|e| Source::SQLx.to_error(500, e, "Select Ward SseMessage"))?
        .iter()
        .map(SseData::from_row)
        .collect::<sqlx::Result<Vec<SseData>>>()
        .map_err(|e| Source::SQLx.to_error(500, e, "Select Ward SseMessage"))?
        .into_iter()
        .map(SseMessage::WardMsg);

    Ok(result)
}

async fn get_sse_message_spclty(
    limit: usize,
    params: &SseMessageParams,
    user: &str,
    doctorcode: &str,
    spclty_ids: &[u32],
    pool: &Pool<MySql>,
    kphis_log: &str,
) -> Result<impl Iterator<Item = SseMessage>, AppError> {
    let sql = sse::select_sse_message_spclty(&spclty_ids, params.min_id.is_some(), kphis_log, limit);
    let mut query = sqlx::query(AssertSqlSafe(sql)).bind(user).bind(doctorcode);
    if let Some(min_id) = params.min_id {
        query = query.bind(min_id);
    }
    let result = query
        .fetch_all(pool)
        .await
        .map_err(|e| Source::SQLx.to_error(500, e, "Select Spclty SseMessage"))?
        .iter()
        .map(SseData::from_row)
        .collect::<sqlx::Result<Vec<SseData>>>()
        .map_err(|e| Source::SQLx.to_error(500, e, "Select Spclty SseMessage"))?
        .into_iter()
        .map(SseMessage::SpcltyMsg);

    Ok(result)
}

async fn get_sse_message_private(limit: usize, params: &SseMessageParams, user: &str, doctorcode: &str, pool: &Pool<MySql>, kphis_log: &str) -> Result<impl Iterator<Item = SseMessage>, AppError> {
    let sql = sse::select_sse_message_private(params.min_id.is_some(), kphis_log, limit);
    let mut query = sqlx::query(AssertSqlSafe(sql)).bind(user).bind(doctorcode);
    if let Some(min_id) = params.min_id {
        query = query.bind(min_id);
    }
    let result = query
        .fetch_all(pool)
        .await
        .map_err(|e| Source::SQLx.to_error(500, e, "Select Private SseMessage"))?
        .iter()
        .map(SseData::from_row)
        .collect::<sqlx::Result<Vec<SseData>>>()
        .map_err(|e| Source::SQLx.to_error(500, e, "Select Private SseMessage"))?
        .into_iter()
        .map(SseMessage::DirectMsg);

    Ok(result)
}

pub async fn post_sse_message(
    message: &SsePostMessage,
    message_datetime: PrimitiveDateTime,
    sender_code: &Option<String>,
    sender_name: &str,
    pool: &Pool<MySql>,
    kphis_log: &str,
) -> Result<ExecuteResponse, AppError> {
    let sql = sse::insert_sse_message(kphis_log);
    let reference = message.reference.as_ref().and_then(|data| serde_json::to_string(&data).ok());
    let result = sqlx::query(AssertSqlSafe(sql))
        .bind(message_datetime)
        .bind(&message.message)
        .bind(sender_code.clone().unwrap_or_default())
        .bind(sender_name)
        .bind(&message.person)
        .bind(&message.ward)
        .bind(message.spclty_id)
        .bind(message.route.as_ref().map(|route| route.string()))
        .bind(reference)
        .execute(pool)
        .await
        .map_err(|e| Source::SQLx.to_error(500, e, "Insert SseMessage"))?;

    Ok(ExecuteResponse::from_query_result(result, "Insert SseMessage"))
}

// PATCH /sse-message
pub async fn patch_sse_messages(user: &str, message_ids: &[u32], pool: &Pool<MySql>, kphis_log: &str) -> Result<ExecuteResponse, AppError> {
    if message_ids.is_empty() {
        return Ok(ExecuteResponse::default().with_action("Update SseMessageReaded"));
    }
    let sql = sse::insert_dup_sse_message_read(user, message_ids, kphis_log);
    let result = sqlx::query(AssertSqlSafe(sql))
        .execute(pool)
        .await
        .map_err(|e| Source::SQLx.to_error(500, e, "Update SseMessageReaded"))?;

    Ok(ExecuteResponse::from_query_result(result, "Update SseMessageReaded"))
}

#[cfg(test)]
#[rustfmt::skip]
mod tests {

    use super::*;
    use kphis_model::route::Route;
    use kphis_sqlx_tester::MySqlTester;
    use kphis_util::datetime::now;

    #[tokio::test]
    #[ignore]
    async fn sqlx_get_sse_message_new() {
        let tester = MySqlTester::new_kphis_log().await;
        sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/create/kphis_log/message.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/create/kphis_log/message_read.sql")).execute(&tester.db_pool).await.unwrap();

        sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/insert/kphis_log/message.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/insert/kphis_log/message_read.sql")).execute(&tester.db_pool).await.unwrap();

        let params_all = SseMessageParams {cat: None,min_id: Some(5)};
        let params_global = SseMessageParams {cat: Some(String::from("global")),min_id: Some(5)};
        let params_ward = SseMessageParams {cat: Some(String::from("ward")),min_id: Some(5)};
        let params_splcty = SseMessageParams {cat: Some(String::from("spclty")),min_id: Some(5)};
        let params_private = SseMessageParams {cat: Some(String::from("private")),min_id: Some(5)};
        let params_unknown = SseMessageParams {cat: Some(String::from("xxx")),min_id: Some(5)};
        let doctorcode = Some(String::from("009"));

        let no_doctorcode = get_sse_message(&params_global,"user",&None,&vec![String::from("01")],&vec![1],&tester.db_pool,&tester.kphis_log).await.unwrap();
        assert!(no_doctorcode.is_empty());
        let all = get_sse_message(&params_all,"user",&doctorcode,&vec![String::from("01")],&vec![1],&tester.db_pool,&tester.kphis_log).await.unwrap();
        assert_eq!(all.len(), 5); // global 1 + ward 1 + spclty 1 + private 2
        let global = get_sse_message(&params_global,"user",&doctorcode,&vec![String::from("01")],&vec![1],&tester.db_pool,&tester.kphis_log).await.unwrap();
        assert_eq!(global.len(), 1);
        let ward_with_group = get_sse_message(&params_ward,"user",&doctorcode,&vec![String::from("01")],&vec![1],&tester.db_pool,&tester.kphis_log).await.unwrap();
        assert_eq!(ward_with_group.len(), 1);
        let ward_without_group = get_sse_message(&params_ward,"user",&doctorcode,&Vec::new(),&Vec::new(),&tester.db_pool,&tester.kphis_log).await.unwrap();
        assert!(ward_without_group.is_empty());
        let spclty_with_group = get_sse_message(&params_splcty,"user",&doctorcode,&vec![String::from("01")],&vec![1],&tester.db_pool,&tester.kphis_log).await.unwrap();
        assert_eq!(spclty_with_group.len(), 1);
        let spclty_without_group = get_sse_message(&params_splcty,"user",&doctorcode,&Vec::new(),&Vec::new(),&tester.db_pool,&tester.kphis_log).await.unwrap();
        assert!(spclty_without_group.is_empty());
        let private = get_sse_message(&params_private,"user",&doctorcode,&vec![String::from("01")],&vec![1],&tester.db_pool,&tester.kphis_log).await.unwrap();
        assert_eq!(private.len(), 2); // include self to self
        let unknown = get_sse_message(&params_unknown,"user",&doctorcode,&vec![String::from("01")],&vec![1],&tester.db_pool,&tester.kphis_log).await.unwrap();
        assert!(unknown.is_empty());
    }

    #[tokio::test]
    #[ignore]
    async fn sqlx_get_sse_message_all() {
        let tester = MySqlTester::new_kphis_log().await;
        sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/create/kphis_log/message.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/create/kphis_log/message_read.sql")).execute(&tester.db_pool).await.unwrap();

        sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/insert/kphis_log/message.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/insert/kphis_log/message_read.sql")).execute(&tester.db_pool).await.unwrap();

        let params_all = SseMessageParams {cat: None,min_id: None};
        let params_global = SseMessageParams {cat: Some(String::from("global")),min_id: None};
        let params_ward = SseMessageParams {cat: Some(String::from("ward")),min_id: None};
        let params_splcty = SseMessageParams {cat: Some(String::from("spclty")),min_id: None};
        let params_private = SseMessageParams {cat: Some(String::from("private")),min_id: None};
        let params_unknown = SseMessageParams {cat: Some(String::from("xxx")),min_id: None};
        let doctorcode = Some(String::from("009"));

        let no_doctorcode = get_sse_message(&params_global,"user",&None,&vec![String::from("01")],&vec![1],&tester.db_pool,&tester.kphis_log).await.unwrap();
        assert!(no_doctorcode.is_empty());
        let all = get_sse_message(&params_all,"user",&doctorcode,&vec![String::from("01")],&vec![1],&tester.db_pool,&tester.kphis_log).await.unwrap();
        assert_eq!(all.len(), 9); // global 2 + ward 2 + spclty 2 + private 3
        let global = get_sse_message(&params_global,"user",&doctorcode,&vec![String::from("01")],&vec![1],&tester.db_pool,&tester.kphis_log).await.unwrap();
        assert_eq!(global.len(), 2);
        let ward_with_group = get_sse_message(&params_ward,"user",&doctorcode,&vec![String::from("01")],&vec![1],&tester.db_pool,&tester.kphis_log).await.unwrap();
        assert_eq!(ward_with_group.len(), 2);
        let ward_without_group = get_sse_message(&params_ward,"user",&doctorcode,&Vec::new(),&Vec::new(),&tester.db_pool,&tester.kphis_log).await.unwrap();
        assert!(ward_without_group.is_empty());
        let spclty_with_group = get_sse_message(&params_splcty,"user",&doctorcode,&vec![String::from("01")],&vec![1],&tester.db_pool,&tester.kphis_log).await.unwrap();
        assert_eq!(spclty_with_group.len(), 2);
        let spclty_without_group = get_sse_message(&params_splcty,"user",&doctorcode,&Vec::new(),&Vec::new(),&tester.db_pool,&tester.kphis_log).await.unwrap();
        assert!(spclty_without_group.is_empty());
        let private = get_sse_message(&params_private,"user",&doctorcode,&vec![String::from("01")],&vec![1],&tester.db_pool,&tester.kphis_log).await.unwrap();
        assert_eq!(private.len(), 3); // include self to self
        let unknown = get_sse_message(&params_unknown,"user",&doctorcode,&vec![String::from("01")],&vec![1],&tester.db_pool,&tester.kphis_log).await.unwrap();
        assert!(unknown.is_empty());
    }

    #[tokio::test]
    #[ignore]
    async fn sqlx_post_sse_message() {
        let tester = MySqlTester::new_kphis_log().await;
        sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/create/kphis_log/message.sql")).execute(&tester.db_pool).await.unwrap();

        let datetime = now();
        let sender_code = String::from("009");
        let sender_name = String::from("sender");
        let message_none = SsePostMessage {message: String::from("message"),..Default::default()};

        let resp_1 = post_sse_message(&message_none,datetime,&None,&sender_name,&tester.db_pool,&tester.kphis_log).await.unwrap();
        assert_eq!(resp_1.last_insert_id, 1);
        let message_all = SsePostMessage {message: String::from("message"),person: Some(String::from("007")),ward: Some(String::from("01")),spclty_id: Some(1),route: Some(Route::Info),reference: Some(SseData::demo())};
        let resp_2 = post_sse_message(&message_all,datetime,&Some(sender_code),&sender_name,&tester.db_pool,&tester.kphis_log).await.unwrap();
        assert_eq!(resp_2.last_insert_id, 2);
    }

    #[tokio::test]
    #[ignore]
    async fn sqlx_patch_sse_messages() {
        let tester = MySqlTester::new_kphis_log().await;
        sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/create/kphis_log/message_read.sql")).execute(&tester.db_pool).await.unwrap();

        let resp_ok = patch_sse_messages("user", &[1,2], &tester.db_pool, &tester.kphis_log).await.unwrap();
        assert_eq!(resp_ok.rows_affected, 2);
        let resp_again = patch_sse_messages("user", &[1,2], &tester.db_pool, &tester.kphis_log).await.unwrap();
        assert_eq!(resp_again.rows_affected, 2);
        let resp_none = patch_sse_messages("user", &[], &tester.db_pool, &tester.kphis_log).await.unwrap();
        assert_eq!(resp_none.rows_affected, 0);
        // let rows = sqlx::query("SELECT * FROM kphis_log.message_read;").fetch_all(&tester.db_pool).await.unwrap();
        // assert_eq!(rows.len(), 2);
    }
}
