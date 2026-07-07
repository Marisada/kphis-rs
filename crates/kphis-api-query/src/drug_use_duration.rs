use sqlx::{AssertSqlSafe, FromRow, MySql, Pool};

use kphis_model::{
    drug_use_duration::{DrugUseDuration, DrugUseDurationParams},
    fetch::ExecuteResponse,
};
use kphis_sql::drug_use_duration;
use kphis_util::error::{AppError, Source};

pub async fn get_drug_use_duration(params: &DrugUseDurationParams, pool: &Pool<MySql>, hosxp: &str, kphis: &str) -> Result<Vec<DrugUseDuration>, AppError> {
    let sql = drug_use_duration::get_drug_use_duration(params, hosxp, kphis);
    let mut query = sqlx::query(AssertSqlSafe(sql));
    if let Some(icode) = &params.icode {
        query = query.bind(icode);
    }
    if let Some(due_status) = &params.due_status {
        query = query.bind(due_status);
    }
    if let Some(monitor_status) = &params.monitor_status {
        query = query.bind(monitor_status);
    }
    if let Some(med_name) = &params.med_name {
        query = query.bind(["%", med_name, "%"].concat());
    }
    let result = query
        .fetch_all(pool)
        .await
        .map_err(|e| Source::SQLx.to_error(500, e, "Select DrugUseDuration"))?
        .iter()
        .map(DrugUseDuration::from_row)
        .collect::<sqlx::Result<Vec<DrugUseDuration>>>()
        .map_err(|e| Source::SQLx.to_error(500, e, "Select DrugUseDuration"))?;

    Ok(result)
}

pub async fn post_drug_use_duration(payload: &DrugUseDuration, loginname: &str, pool: &Pool<MySql>, kphis: &str) -> Result<ExecuteResponse, AppError> {
    let sql = drug_use_duration::insert_duplicate_drug_use_duration(kphis);
    let result = sqlx::query(AssertSqlSafe(sql))
        .bind(&payload.icode)
        .bind(&payload.usage)
        .bind(payload.duration1)
        .bind(&payload.exceed_duration1_color)
        .bind(payload.duration2)
        .bind(&payload.exceed_duration2_color)
        .bind(payload.duration3)
        .bind(&payload.exceed_duration3_color)
        .bind(&payload.status)
        .bind(&payload.monitor)
        .bind(payload.monitor_count)
        .bind(payload.monitor_duration)
        .bind(&payload.monitor_status)
        .bind(&payload.info)
        .bind(&payload.info_status)
        .bind(loginname)
        .bind(loginname)
        .execute(pool)
        .await
        .map_err(|e| Source::SQLx.to_error(500, e, "InsertDuplicate DrugUseDuration"))?;

    Ok(ExecuteResponse::from_query_result(result, "InsertDuplicate DrugUseDuration"))
}

#[cfg(test)]
#[rustfmt::skip]
mod tests {

    use super::*;
    use kphis_sqlx_tester::MySqlTester;

    #[tokio::test]
    #[ignore]
    async fn sqlx_get_drug_use_duration() {
        let tester = MySqlTester::new_hosxp_and_kphis().await;
        sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/create/hosxp/drugitems.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/create/kphis/kphis_drug_use_duration.sql")).execute(&tester.db_pool).await.unwrap();

        sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/insert/hosxp/drugitems.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/insert/kphis/kphis_drug_use_duration.sql")).execute(&tester.db_pool).await.unwrap();

        let default = get_drug_use_duration(&DrugUseDurationParams::default(),&tester.db_pool,&tester.hosxp,&tester.kphis).await.unwrap();
        assert_eq!(default.len(),3);
        let icode_found = get_drug_use_duration(&DrugUseDurationParams{icode: Some(String::from("1000111")),..Default::default()},&tester.db_pool,&tester.hosxp,&tester.kphis).await.unwrap();
        assert_eq!(icode_found.len(),1);
        let med_name_found = get_drug_use_duration(&DrugUseDurationParams{med_name: Some(String::from("race")),..Default::default()},&tester.db_pool,&tester.hosxp,&tester.kphis).await.unwrap();
        assert_eq!(med_name_found.len(),1);
        let due_status_y_found = get_drug_use_duration(&DrugUseDurationParams{due_status: Some(String::from("Y")),..Default::default()},&tester.db_pool,&tester.hosxp,&tester.kphis).await.unwrap();
        assert_eq!(due_status_y_found.len(),1);
        let due_status_n_found = get_drug_use_duration(&DrugUseDurationParams{due_status: Some(String::from("N")),..Default::default()},&tester.db_pool,&tester.hosxp,&tester.kphis).await.unwrap();
        assert_eq!(due_status_n_found.len(),1);
        let monitor_status_y_found = get_drug_use_duration(&DrugUseDurationParams{monitor_status: Some(String::from("Y")),..Default::default()},&tester.db_pool,&tester.hosxp,&tester.kphis).await.unwrap();
        assert_eq!(monitor_status_y_found.len(),1);
        let monitor_status_n_found = get_drug_use_duration(&DrugUseDurationParams{monitor_status: Some(String::from("N")),..Default::default()},&tester.db_pool,&tester.hosxp,&tester.kphis).await.unwrap();
        assert_eq!(monitor_status_n_found.len(),1);
        let not_found = get_drug_use_duration(&DrugUseDurationParams{icode: Some(String::from("6666666")),..Default::default()},&tester.db_pool,&tester.hosxp,&tester.kphis).await.unwrap();
        assert!(not_found.is_empty());
    }

    #[tokio::test]
    #[ignore]
    async fn sqlx_post_drug_use_duration() {
        let tester = MySqlTester::new_kphis().await;
        sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/create/kphis/kphis_drug_use_duration.sql")).execute(&tester.db_pool).await.unwrap();

        let resp = post_drug_use_duration(&DrugUseDuration::demo(),"user",&tester.db_pool,&tester.kphis).await.unwrap();
        assert_eq!(resp.rows_affected,1);
        // INSERT DUPLICATE
        let resp_again = post_drug_use_duration(&DrugUseDuration::demo(),"user",&tester.db_pool,&tester.kphis).await.unwrap();
        assert_eq!(resp_again.rows_affected,2);
    }
}
