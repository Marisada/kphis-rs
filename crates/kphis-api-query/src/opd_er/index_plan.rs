use sqlx::{
    AssertSqlSafe, FromRow, MySql, Pool, Row,
    mysql::{MySqlQueryResult, MySqlRow},
};

use kphis_model::{
    app::VisitTypeId,
    fetch::ExecuteResponse,
    index_plan::{IndexPlan, IndexPlanOnly, IndexPlanSave},
};
use kphis_sql::opd_er::index_plan;
use kphis_util::error::{AppError, Source};

// // opd-er-nurse-index-plan-data.php
// // opd-er-nurse-index-plan-list-data.php
// // opd-er-nurse-index-plan-monitor-data.php
// pub async fn get_index_plan_plus(params: &IndexPlanParams, hlen: usize, vlen: usize, pool: &Pool<MySql>, hosxp: &str, kphis: &str) -> Result<Vec<IndexPlanPlus>, AppError> {
//     let sql = index_plan::select_index_plan_plus(params, hlen, vlen, hosxp, kphis);
//     let mut query = sqlx::query(AssertSqlSafe(sql));
//     if let Some(plan_id) = params.plan_id.as_ref() {
//         query = query.bind(plan_id);
//     }
//     if let Some(order_item_id) = params.order_item_id.as_ref() {
//         query = query.bind(order_item_id);
//     }
//     if let Some(order_item_type) = params.order_item_type.as_ref() {
//         query = query.bind(order_item_type);
//     }
//     if let Some(nurse_assign) = params.nurse_assign.as_ref() {
//         query = query.bind(nurse_assign);
//     }
//     if let Some(opd_er_order_master_id) = params.opd_er_order_master_id.as_ref() {
//         query = query.bind(opd_er_order_master_id);
//     }
//     if let Some(plan_date) = params.plan_date.as_ref() {
//         query = query.bind(plan_date);
//     }
//     if let Some(start_plan_date) = params.start_plan_date.as_ref() {
//         query = query.bind(start_plan_date);
//     }
//     if let Some(end_plan_date) = params.end_plan_date.as_ref() {
//         query = query.bind(end_plan_date);
//     }
//     if let Some(plan_sch_type) = params.plan_sch_type.as_ref() {
//         query = query.bind(plan_sch_type);
//     }
//     if let Some(patient) = params.patient.as_ref().and_then(|s| urlencoding::decode(s).ok()) {
//         let wildcard = ["%", patient.trim(), "%"].concat();
//         match patient.parse::<u64>().is_ok() {
//             true => match hlen.cmp(&vlen) {
//                 Ordering::Equal => {
//                     query = query.bind(wildcard.clone()).bind(wildcard.clone());
//                 }
//                 _ => {
//                     query = query.bind(wildcard.clone());
//                 }
//             },
//             false => {
//                 query = query.bind(wildcard);
//             }
//         }
//     }
//     query
//         .fetch_all(pool)
//         .await
//         .map(|rows| {
//             rows.iter()
//                 .map(index_plan_plus_from_row)
//                 .collect::<sqlx::Result<Vec<IndexPlanPlus>>>()
//                 .map_err(|e| Source::SQLx.to_error(500, e, "Select IndexPlan"))
//         })
//         .map_err(|e| Source::SQLx.to_error(500, e, "Select IndexPlan"))?
// }
// fn index_plan_plus_from_row(row: &MySqlRow) -> sqlx::Result<IndexPlanPlus> {
//     let opd_er_order_master_id: u32 = row.try_get("opd_er_order_master_id")?;
//     let vn: Option<String> = row.try_get("vn")?;
//     Ok(IndexPlanPlus {
//         visit_type: VisitTypeId::OpdEr(vn.unwrap_or_default(), opd_er_order_master_id),
//         hn: row.try_get("hn")?,
//         plan_id: row.try_get("plan_id")?,
//         plan_detail: row.try_get("plan_detail")?,
//         plan_date: row.try_get("plan_date")?,
//         plan_time: row.try_get("plan_time")?,
//         plan_sch_type: row.try_get("plan_sch_type")?,
//         order_item_id: row.try_get("order_item_id")?,
//         order_id: row.try_get("order_id")?,
//         order_item_detail: row.try_get("order_item_detail")?,
//         stat: row.try_get("stat")?,
//         icode: row.try_get("icode")?,
//         off_order_item_id: row.try_get("off_order_item_id")?,
//         off_order_item_type: row.try_get("off_order_item_type")?,
//         off_order_item_detail: row.try_get("off_order_item_detail")?,
//         order_item_type: row.try_get("order_item_type")?,
//         nurse_assign: row.try_get("nurse_assign")?,
//         order_date: row.try_get("order_date")?,
//         order_time: row.try_get("order_time")?,
//         order_doctor: row.try_get("order_doctor")?,
//         order_type: row.try_get("order_type")?,
//         order_owner_type: row.try_get("order_owner_type")?,
//         order_confirm: row.try_get("order_confirm")?,
//         med_name: row.try_get("med_name")?,
//         dosageform: row.try_get("dosageform")?,
//         displaycolor: row.try_get("displaycolor")?,
//         off_med_name: row.try_get("off_med_name")?,
//         off_displaycolor: row.try_get("off_displaycolor")?,
//         off_by_datetime: row.try_get("off_by_datetime")?,
//         bedno: row.try_get("bedno")?,
//         patient_name: row.try_get("patient_name")?,
//         drugallergy: row.try_get("drugallergy")?,
//         er_drugallergy_history: row.try_get("er_drugallergy_history")?,
//         admission_note_id: None,
//         allergy_drug_history: None,
//         allergy_drug_history_hosxp: None,
//         allergy_drug_pharmacy_check_person: None,
//         allergy_drug_pharmacy_check_datetime: None,
//         note: row.try_get("note")?,
//         actions: Vec::new(),
//     })
// }

pub async fn get_index_plan_by_order_item_id(order_item_id: u32, pool: &Pool<MySql>, kphis: &str) -> Result<Vec<IndexPlan>, AppError> {
    let sql = index_plan::select_index_plan_by_order_item_id(kphis);
    sqlx::query(AssertSqlSafe(sql))
        .bind(order_item_id)
        .fetch_all(pool)
        .await
        .map_err(|e| Source::SQLx.to_error(500, e, "Select OpdErIndexPlan"))?
        .iter()
        .map(index_plan_from_row)
        .collect::<sqlx::Result<Vec<IndexPlan>>>()
        .map_err(|e| Source::SQLx.to_error(500, e, "Select OpdErIndexPlan"))
}
fn index_plan_from_row(row: &MySqlRow) -> sqlx::Result<IndexPlan> {
    let opd_er_order_master_id: u32 = row.try_get("opd_er_order_master_id")?;
    Ok(IndexPlan {
        visit_type: VisitTypeId::OpdEr(String::new(), opd_er_order_master_id),
        plan_id: row.try_get("plan_id")?,
        plan_detail: row.try_get("plan_detail")?,
        plan_date: row.try_get("plan_date")?,
        plan_time: row.try_get("plan_time")?,
        plan_sch_type: row.try_get("plan_sch_type")?,
        order_item_id: row.try_get("order_item_id")?,
        order_type: None,
        order_date: None,
        order_time: None,
        off_by_datetime: None,
        actions: Vec::new(),
    })
}

pub async fn get_index_plan_only(order_item_id: u32, pool: &Pool<MySql>, kphis: &str) -> Result<Vec<IndexPlanOnly>, AppError> {
    let sql = index_plan::select_index_plan_only(kphis);
    sqlx::query(AssertSqlSafe(sql))
        .bind(order_item_id)
        .fetch_all(pool)
        .await
        .map_err(|e| Source::SQLx.to_error(500, e, "Select IndexPlanOnly"))?
        .iter()
        .map(IndexPlanOnly::from_row)
        .collect::<sqlx::Result<Vec<IndexPlanOnly>>>()
        .map_err(|e| Source::SQLx.to_error(500, e, "Select IndexPlanOnly"))
}

pub async fn get_index_plan_without_order_item_id(opd_er_order_master_id: u32, pool: &Pool<MySql>, kphis: &str) -> Result<Vec<IndexPlan>, AppError> {
    let sql = index_plan::select_index_plan_without_order_item_id(kphis);
    sqlx::query(AssertSqlSafe(sql))
        .bind(opd_er_order_master_id)
        .fetch_all(pool)
        .await
        .map_err(|e| Source::SQLx.to_error(500, e, "Select OpdErIndexPlan"))?
        .iter()
        .map(index_plan_from_row)
        .collect::<sqlx::Result<Vec<IndexPlan>>>()
        .map_err(|e| Source::SQLx.to_error(500, e, "Select OpdErIndexPlan"))
}

// opd-er-nurse-index-plan-action-save.php
pub async fn post_index_plan(save: &IndexPlanSave, user: &str, pool: &Pool<MySql>, kphis: &str) -> Result<(u32, Vec<ExecuteResponse>), AppError> {
    let is_update = match save.plan_id {
        Some(id) => id > 0,
        None => false,
    };
    let plan_id;
    let mut results = Vec::with_capacity(1);
    if is_update {
        plan_id = save.plan_id;

        let update_plan_result = update_index_plan(save, user, pool, kphis).await?;
        results.push(ExecuteResponse::from_query_result(update_plan_result, "Update IndexPlan"));
    } else {
        let insert_plan_result = insert_index_plan(save, user, pool, kphis).await?;
        plan_id = Some(insert_plan_result.last_insert_id() as u32);
        results.push(ExecuteResponse::from_query_result(insert_plan_result, "Insert IndexPlan"));
    }

    Ok((plan_id.unwrap_or_default(), results))
}

async fn update_index_plan(save: &IndexPlanSave, user: &str, pool: &Pool<MySql>, kphis: &str) -> Result<MySqlQueryResult, AppError> {
    let update_plan_sql = index_plan::update_index_plan(kphis);
    sqlx::query(AssertSqlSafe(update_plan_sql))
        .bind(&save.plan_detail)
        .bind(save.plan_date)
        .bind(save.plan_time)
        .bind(&save.plan_sch_type)
        .bind(user)
        .bind(save.plan_id.unwrap_or_default())
        .execute(pool)
        .await
        .map_err(|e| Source::SQLx.to_error(500, e, "Update IndexPlan"))
}

async fn insert_index_plan(save: &IndexPlanSave, user: &str, pool: &Pool<MySql>, kphis: &str) -> Result<MySqlQueryResult, AppError> {
    if let VisitTypeId::OpdEr(_, opd_er_order_master_id) = &save.visit_type {
        let insert_plan_sql = index_plan::insert_index_plan(kphis);
        sqlx::query(AssertSqlSafe(insert_plan_sql))
            .bind(save.order_item_id)
            .bind(opd_er_order_master_id)
            .bind(&save.plan_detail)
            .bind(save.plan_date)
            .bind(save.plan_time)
            .bind(&save.plan_sch_type)
            .bind(user)
            .bind(user)
            .execute(pool)
            .await
            .map_err(|e| Source::SQLx.to_error(500, e, "Insert IndexPlan"))
    } else {
        Err(AppError::app_400("Insert IndexPlan"))
    }
}

pub async fn insert_index_plan_only(order_item_id: u32, opd_er_order_master_id: u32, only: &mut IndexPlanOnly, pool: &Pool<MySql>, kphis: &str) -> Result<MySqlQueryResult, AppError> {
    only.order_item_id = Some(order_item_id);
    only.insert(
        Some("plan_id"),
        Some("opd_er_nurse_index_plan"),
        ",opd_er_order_master_id",
        ",?",
        &[&opd_er_order_master_id.to_string()],
        pool,
        kphis,
    )
    .await
    .map_err(|e| Source::SQLx.to_error(500, e, "Insert IndexPlansOnly"))
}

// pub async fn insert_index_plans_only(order_item_id: u32, opd_er_order_master_id: u32, index_plans_only: &[IndexPlanOnly], pool: &Pool<MySql>, kphis: &str) -> Result<MySqlQueryResult, AppError> {
//     let insert_plan_sql = index_plan::insert_index_plans_only(order_item_id, opd_er_order_master_id, index_plans_only, kphis);
//     sqlx::query(AssertSqlSafe(insert_plan_sql))
//         .execute(pool)
//         .await
//         .map_err(|e| Source::SQLx.to_error(500, e, "Insert IndexPlansOnly"))
// }

// opd-er-nurse-index-plan-action-delete.php
pub async fn delete_index_plan(plan_id: u32, pool: &Pool<MySql>, kphis: &str) -> Result<ExecuteResponse, AppError> {
    let sql = index_plan::delete_index_plan(kphis);
    let delete_result = sqlx::query(AssertSqlSafe(sql))
        .bind(plan_id)
        .execute(pool)
        .await
        .map_err(|e| Source::SQLx.to_error(500, e, "Delete IndexPlan"))?;

    Ok(ExecuteResponse::from_query_result(delete_result, "Delete IndexPlan"))
}

#[cfg(test)]
#[rustfmt::skip]
mod tests {

    use super::*;
    use kphis_sqlx_tester::MySqlTester;

    // #[tokio::test]
    // #[ignore]
    // async fn sqlx_get_index_plan_plus() {
    //     let tester = MySqlTester::new_hosxp_and_kphis().await;
    //     sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/hosxp/ovst.sql")).execute(&tester.db_pool).await.unwrap();
    //     sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/hosxp/patient.sql")).execute(&tester.db_pool).await.unwrap();
    //     sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/hosxp/drugitems.sql")).execute(&tester.db_pool).await.unwrap();
    //     sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/hosxp/opd_allergy.sql")).execute(&tester.db_pool).await.unwrap();
    //     sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis/opd_er_order.sql")).execute(&tester.db_pool).await.unwrap();
    //     sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis/opd_er_order_item.sql")).execute(&tester.db_pool).await.unwrap();
    //     sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis/opd_er_nurse_index_plan.sql")).execute(&tester.db_pool).await.unwrap();
    //     sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis/opd_er_order_master.sql")).execute(&tester.db_pool).await.unwrap();
    //     sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis/opd_er_allergy_history.sql")).execute(&tester.db_pool).await.unwrap();
    //     sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis/opd_er_bed.sql")).execute(&tester.db_pool).await.unwrap();

    //     sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/hosxp/ovst.sql")).execute(&tester.db_pool).await.unwrap();
    //     sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/hosxp/patient.sql")).execute(&tester.db_pool).await.unwrap();
    //     sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/hosxp/drugitems.sql")).execute(&tester.db_pool).await.unwrap();
    //     sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/hosxp/opd_allergy.sql")).execute(&tester.db_pool).await.unwrap();
    //     sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/kphis/opd_er_order.sql")).execute(&tester.db_pool).await.unwrap();
    //     sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/kphis/opd_er_order_item.sql")).execute(&tester.db_pool).await.unwrap();
    //     sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/kphis/opd_er_nurse_index_plan.sql")).execute(&tester.db_pool).await.unwrap();
    //     sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/kphis/opd_er_order_master.sql")).execute(&tester.db_pool).await.unwrap();
    //     sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/kphis/opd_er_allergy_history.sql")).execute(&tester.db_pool).await.unwrap();
    //     sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/kphis/opd_er_bed.sql")).execute(&tester.db_pool).await.unwrap();

    //     // if params not has 'opd_er_order_master_id' or 'patient' => only 'er_patient_status_id <> 7' will selected
    //     let default = get_index_plan_plus(&IndexPlanParams::default(),7,9,&tester.db_pool,&tester.hosxp,&tester.kphis).await.unwrap();
    //     assert_eq!(default.len(), 4);
    //     let found_plan_id = get_index_plan_plus(&IndexPlanParams {plan_id: Some(4),..Default::default()},7,9,&tester.db_pool,&tester.hosxp,&tester.kphis).await.unwrap();
    //     assert_eq!(found_plan_id.len(), 1);
    //     let found_order_item_id = get_index_plan_plus(&IndexPlanParams {order_item_id: Some(11),..Default::default()},7,9,&tester.db_pool,&tester.hosxp,&tester.kphis).await.unwrap();
    //     assert_eq!(found_order_item_id.len(), 3);
    //     let found_without_order = get_index_plan_plus(&IndexPlanParams {without_order: Some(String::from("Y")),..Default::default()},7,9,&tester.db_pool,&tester.hosxp,&tester.kphis).await.unwrap();
    //     assert_eq!(found_without_order.len(), 1);
    //     let found_order_item_type = get_index_plan_plus(&IndexPlanParams {order_item_type: Some(String::from("med")),..Default::default()},7,9,&tester.db_pool,&tester.hosxp,&tester.kphis).await.unwrap();
    //     assert_eq!(found_order_item_type.len(), 3);
    //     let found_nurse_assign = get_index_plan_plus(&IndexPlanParams {nurse_assign: Some(String::from("Incharge")),..Default::default()},7,9,&tester.db_pool,&tester.hosxp,&tester.kphis).await.unwrap();
    //     assert_eq!(found_nurse_assign.len(), 3);
    //     let found_an = get_index_plan_plus(&IndexPlanParams {opd_er_order_master_id: Some(3),..Default::default()},7,9,&tester.db_pool,&tester.hosxp,&tester.kphis).await.unwrap();
    //     assert_eq!(found_an.len(), 4);
    //     let found_plan_date = get_index_plan_plus(&IndexPlanParams {plan_date: date_8601("2024-11-11"),..Default::default()},7,9,&tester.db_pool,&tester.hosxp,&tester.kphis).await.unwrap();
    //     assert_eq!(found_plan_date.len(), 1);
    //     let found_plan_date_between = get_index_plan_plus(&IndexPlanParams {start_plan_date: date_8601("2024-11-01"),end_plan_date: date_8601("2024-11-31"),..Default::default()},7,9,&tester.db_pool,&tester.hosxp,&tester.kphis).await.unwrap();
    //     assert_eq!(found_plan_date_between.len(), 3);
    //     let found_plan_date_after_eq = get_index_plan_plus(&IndexPlanParams {start_plan_date: date_8601("2024-11-11"),..Default::default()},7,9,&tester.db_pool,&tester.hosxp,&tester.kphis).await.unwrap();
    //     assert_eq!(found_plan_date_after_eq.len(), 2);
    //     let found_plan_date_before_eq = get_index_plan_plus(&IndexPlanParams {end_plan_date: date_8601("2024-11-11"),..Default::default()},7,9,&tester.db_pool,&tester.hosxp,&tester.kphis).await.unwrap();
    //     assert_eq!(found_plan_date_before_eq.len(), 3);
    //     let found_plan_sch_type = get_index_plan_plus(&IndexPlanParams {plan_sch_type: Some(String::from("stat")),..Default::default()},7,9,&tester.db_pool,&tester.hosxp,&tester.kphis).await.unwrap();
    //     assert_eq!(found_plan_sch_type.len(), 1);
    //     let found_plan_patient_hn = get_index_plan_plus(&IndexPlanParams {patient: Some(String::from("1234")),..Default::default()},7,9,&tester.db_pool,&tester.hosxp,&tester.kphis).await.unwrap();
    //     assert_eq!(found_plan_patient_hn.len(), 6);
    //     let found_plan_patient_an = get_index_plan_plus(&IndexPlanParams {patient: Some(String::from("70111111")),..Default::default()},7,9,&tester.db_pool,&tester.hosxp,&tester.kphis).await.unwrap();
    //     assert_eq!(found_plan_patient_an.len(), 4);
    //     let found_plan_patient_name = get_index_plan_plus(&IndexPlanParams {patient: Some(String::from("มุติ")),..Default::default()},7,9,&tester.db_pool,&tester.hosxp,&tester.kphis).await.unwrap();
    //     assert_eq!(found_plan_patient_name.len(), 6);
    //     let not_found = get_index_plan_plus(&IndexPlanParams {patient: Some(String::from("6666")),..Default::default()},7,9,&tester.db_pool,&tester.hosxp,&tester.kphis).await.unwrap();
    //     assert!(not_found.is_empty());
    // }

    #[tokio::test]
    #[ignore]
    async fn sqlx_get_index_plan_by_order_item_id() {
        let tester = MySqlTester::new_kphis().await;
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis/opd_er_nurse_index_plan.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/kphis/opd_er_nurse_index_plan.sql")).execute(&tester.db_pool).await.unwrap();

        let found = get_index_plan_by_order_item_id(11, &tester.db_pool, &tester.kphis).await.unwrap();
        assert_eq!(found.len(), 3);
        let not_found = get_index_plan_by_order_item_id(999, &tester.db_pool, &tester.kphis).await.unwrap();
        assert!(not_found.is_empty());
    }

    #[tokio::test]
    #[ignore]
    async fn sqlx_get_index_plan_only() {
        let tester = MySqlTester::new_kphis().await;
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis/opd_er_nurse_index_plan.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/kphis/opd_er_nurse_index_plan.sql")).execute(&tester.db_pool).await.unwrap();

        let found = get_index_plan_only(11, &tester.db_pool, &tester.kphis).await.unwrap();
        assert_eq!(found.len(), 3);
        let not_found = get_index_plan_only(999, &tester.db_pool, &tester.kphis).await.unwrap();
        assert!(not_found.is_empty());
    }

    #[tokio::test]
    #[ignore]
    async fn sqlx_get_index_plan_without_order_item_id() {
        let tester = MySqlTester::new_kphis().await;
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis/opd_er_nurse_index_plan.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/kphis/opd_er_nurse_index_plan.sql")).execute(&tester.db_pool).await.unwrap();

        let found = get_index_plan_without_order_item_id(1, &tester.db_pool, &tester.kphis).await.unwrap();
        assert_eq!(found.len(), 2);
        let not_found = get_index_plan_without_order_item_id(999, &tester.db_pool, &tester.kphis).await.unwrap();
        assert!(not_found.is_empty());
    }

    #[tokio::test]
    #[ignore]
    async fn sqlx_insert_index_plan() {
        let tester = MySqlTester::new_kphis().await;
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis/opd_er_nurse_index_plan.sql")).execute(&tester.db_pool).await.unwrap();

        let mut index_plan = IndexPlanSave::demo();
        index_plan.visit_type = VisitTypeId::OpdEr(String::new(), 1);

        let success = insert_index_plan(&index_plan,"user",&tester.db_pool,&tester.kphis).await.unwrap();
        assert_eq!(success.rows_affected(), 1);
        let again_success = insert_index_plan(&index_plan,"user",&tester.db_pool,&tester.kphis).await.unwrap();
        assert_eq!(again_success.rows_affected(), 1);
    }

    #[tokio::test]
    #[ignore]
    async fn sqlx_insert_index_plan_only() {
        let tester = MySqlTester::new_kphis().await;
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis/opd_er_nurse_index_plan.sql")).execute(&tester.db_pool).await.unwrap();

        let success = insert_index_plan_only(1,1,&mut IndexPlanOnly::demo(),&tester.db_pool,&tester.kphis).await.unwrap();
        assert_eq!(success.rows_affected(), 1);
        let again_success = insert_index_plan_only(1,1,&mut IndexPlanOnly::demo(),&tester.db_pool,&tester.kphis).await.unwrap();
        assert_eq!(again_success.rows_affected(), 1);
    }

    // #[tokio::test]
    // #[ignore]
    // async fn sqlx_insert_index_plans_only() {
    //     let tester = MySqlTester::new_kphis().await;
    //     sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis/opd_er_nurse_index_plan.sql")).execute(&tester.db_pool).await.unwrap();

    //     let success = insert_index_plans_only(1,1,&[IndexPlanOnly::demo()],&tester.db_pool,&tester.kphis).await.unwrap();
    //     assert_eq!(success.rows_affected(), 1);
    //     let again_success = insert_index_plans_only(1,1,&[IndexPlanOnly::demo()],&tester.db_pool,&tester.kphis).await.unwrap();
    //     assert_eq!(again_success.rows_affected(), 1);
    // }

    #[tokio::test]
    #[ignore]
    async fn sqlx_update_index_plan() {
        let tester = MySqlTester::new_kphis().await;
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis/opd_er_nurse_index_plan.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/kphis/opd_er_nurse_index_plan.sql")).execute(&tester.db_pool).await.unwrap();

        let success = update_index_plan(&IndexPlanSave::demo(),"user",&tester.db_pool,&tester.kphis).await.unwrap();
        assert_eq!(success.rows_affected(), 1);
        let again_success = update_index_plan(&IndexPlanSave::demo(),"user",&tester.db_pool,&tester.kphis).await.unwrap();
        assert_eq!(again_success.rows_affected(), 1);
    }

    #[tokio::test]
    #[ignore]
    async fn sqlx_delete_index_plan() {
        let tester = MySqlTester::new_kphis().await;
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis/opd_er_nurse_index_plan.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis/opd_er_nurse_index_action.sql")).execute(&tester.db_pool).await.unwrap();

        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/kphis/opd_er_nurse_index_plan.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/kphis/opd_er_nurse_index_action.sql")).execute(&tester.db_pool).await.unwrap();

        let success = delete_index_plan(1, &tester.db_pool, &tester.kphis).await.unwrap();
        assert_eq!(success.rows_affected, 2);
        let again_not_found = delete_index_plan(1, &tester.db_pool, &tester.kphis).await.unwrap();
        assert_eq!(again_not_found.rows_affected, 0);
    }
}
