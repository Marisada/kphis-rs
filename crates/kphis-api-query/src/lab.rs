use sqlx::{AssertSqlSafe, FromRow, MySql, Pool, Row};

use kphis_model::{
    fetch::ExecuteResponse,
    lab::{LabHead, LabHeadParams, LabItem, LabItemParams, LabItemsGroup, LabWbcBand},
};
use kphis_sql::lab;
use kphis_util::error::{AppError, Source};

use crate::image::scan_his::get_lab_image_from_lab_order_number;

pub async fn get_wbc_band(key: &str, value: &str, wbc_code: i64, band_code: i64, pool: &Pool<MySql>, hosxp: &str) -> Result<Vec<LabWbcBand>, AppError> {
    let sql = lab::get_lab_wbc_band(key, hosxp);
    let result = sqlx::query(AssertSqlSafe(sql))
        .bind(wbc_code)
        .bind(band_code)
        .bind(value)
        .fetch_all(pool)
        .await
        .map_err(|e| Source::SQLx.to_error(500, e, "Select WBC Band"))?
        .iter()
        .map(LabWbcBand::from_row)
        .collect::<sqlx::Result<Vec<LabWbcBand>>>()
        .map_err(|e| Source::SQLx.to_error(500, e, "Select WBC Band"))?;

    Ok(result)
}

pub async fn get_lab_head(params: &LabHeadParams, doctorcode: &Option<String>, groupname: &Option<String>, pool: &Pool<MySql>, hosxp: &str, kphis: &str) -> Result<Vec<LabHead>, AppError> {
    let mut heads = get_lab_head_inner(params, pool, hosxp, kphis).await?;
    for head in heads.iter_mut() {
        head.lab_items_group = get_lab_item_group(head.lab_order_number, params.only_head.unwrap_or_default(), doctorcode, groupname, pool, hosxp).await?;
        if params.with_scan.unwrap_or_default() {
            head.scan_images = get_lab_image_from_lab_order_number(head.lab_order_number, pool, hosxp).await?;
        }
    }

    Ok(heads)
}

pub async fn get_lab_head_inner(params: &LabHeadParams, pool: &Pool<MySql>, hosxp: &str, kphis: &str) -> Result<Vec<LabHead>, AppError> {
    let result = if let (Some(hn), Some(lab_order_number)) = (&params.hn, params.id) {
        let ids = get_lab_ids_with_previous(hn, lab_order_number, params.prev.unwrap_or(1), pool, hosxp).await?;
        if !ids.is_empty() {
            let sql = lab::get_lab_head("lab_order_number", &ids, params.start_date.is_some(), params.end_date.is_some(), hosxp, kphis);
            sqlx::query(AssertSqlSafe(sql)).fetch_all(pool).await
        } else {
            Ok(Vec::new())
        }
    } else if let Some(hn) = &params.hn {
        let sql = lab::get_lab_head("hn", &[], params.start_date.is_some(), params.end_date.is_some(), hosxp, kphis);
        let mut query = sqlx::query(AssertSqlSafe(sql)).bind(hn);
        if let Some(start_date) = &params.start_date {
            query = query.bind(start_date);
        }
        if let Some(end_date) = &params.end_date {
            query = query.bind(end_date);
        }
        query.fetch_all(pool).await
    } else if let Some(vn) = &params.vn {
        let sql = lab::get_lab_head("vn", &[], params.start_date.is_some(), params.end_date.is_some(), hosxp, kphis);
        sqlx::query(AssertSqlSafe(sql)).bind(vn).fetch_all(pool).await
    } else {
        Ok(Vec::new())
    };

    result
        .map_err(|e| Source::SQLx.to_error(500, e, "Select LabHead"))?
        .iter()
        .map(LabHead::from_row)
        .collect::<sqlx::Result<Vec<LabHead>>>()
        .map_err(|e| Source::SQLx.to_error(500, e, "Select LabHead"))
}

async fn get_lab_ids_with_previous(hn: &str, lab_order_number: i32, limit: i32, pool: &Pool<MySql>, hosxp: &str) -> Result<Vec<i32>, AppError> {
    let sql = lab::get_lab_head_with_previous(hosxp);
    sqlx::query(AssertSqlSafe(sql))
        .bind(hn)
        .bind(lab_order_number)
        .bind(lab_order_number)
        .bind(lab_order_number)
        .bind(limit)
        .fetch_all(pool)
        .await
        .map_err(|e| Source::SQLx.to_error(500, e, "Select LabIdsPrevious"))?
        .iter()
        .map(|row| row.try_get("lab_order_number"))
        .collect::<sqlx::Result<Vec<i32>>>()
        .map_err(|e| Source::SQLx.to_error(500, e, "Select LabIdsPrevious"))
}

// GET /lab/item
pub async fn get_lab_item(params: &LabItemParams, doctorcode: &Option<String>, groupname: &Option<String>, pool: &Pool<MySql>, hosxp: &str) -> Result<Vec<LabItem>, AppError> {
    select_lab_items(None, None, true, params, doctorcode, groupname, pool, hosxp).await
}

pub async fn get_lab_item_group(
    lab_order_number: i32,
    only_group: bool,
    doctorcode: &Option<String>,
    groupname: &Option<String>,
    pool: &Pool<MySql>,
    hosxp: &str,
) -> Result<Vec<LabItemsGroup>, AppError> {
    let mut group_result = select_lab_detail_group(lab_order_number, pool, hosxp).await?;
    if !only_group && !group_result.is_empty() {
        for group in group_result.iter_mut() {
            let items_result = select_lab_items(
                Some(lab_order_number),
                Some(group.lab_items_group),
                false,
                &LabItemParams::default(),
                doctorcode,
                groupname,
                pool,
                hosxp,
            )
            .await?;
            group.lab_items = items_result;
        }
    }
    Ok(group_result)
}

async fn select_lab_detail_group(lab_order_number: i32, pool: &Pool<MySql>, hosxp: &str) -> Result<Vec<LabItemsGroup>, AppError> {
    let group_sql = lab::select_lab_detail_group(hosxp);
    sqlx::query(AssertSqlSafe(group_sql))
        .bind(lab_order_number)
        .fetch_all(pool)
        .await
        .map_err(|e| Source::SQLx.to_error(500, e, "Select LabItemGroup"))?
        .iter()
        .map(LabItemsGroup::from_row)
        .collect::<sqlx::Result<Vec<LabItemsGroup>>>()
        .map_err(|e| Source::SQLx.to_error(500, e, "Select LabItemGroup"))
}

#[allow(clippy::too_many_arguments)]
async fn select_lab_items(
    lab_order_number: Option<i32>,
    lab_items_group: Option<Option<i32>>,
    is_order_by_time: bool,
    params: &LabItemParams,
    doctorcode: &Option<String>,
    groupname: &Option<String>,
    pool: &Pool<MySql>,
    hosxp: &str,
) -> Result<Vec<LabItem>, AppError> {
    let items_sql = lab::select_lab_items(
        lab_order_number.is_some(),
        lab_items_group.map(|opt| opt.is_some()),
        params.hn.is_some(),
        params.vn.is_some(),
        params.lab_items_code.is_some(),
        is_order_by_time,
        hosxp,
    );
    let mut items_query = sqlx::query(AssertSqlSafe(items_sql)).bind(doctorcode).bind(groupname);
    if let Some(lab_order_num) = lab_order_number {
        items_query = items_query.bind(lab_order_num);
    }
    if let Some(lab_items_group) = lab_items_group.flatten() {
        items_query = items_query.bind(lab_items_group);
    }
    if let Some(hn) = params.hn.as_ref() {
        items_query = items_query.bind(hn);
    }
    if let Some(vn) = params.vn.as_ref() {
        items_query = items_query.bind(vn);
    }
    if let Some(lab_items_code) = params.lab_items_code {
        items_query = items_query.bind(lab_items_code);
    }

    items_query
        .fetch_all(pool)
        .await
        .map_err(|e| Source::SQLx.to_error(500, e, "Select LabItem"))?
        .iter()
        .map(LabItem::from_row)
        .collect::<sqlx::Result<Vec<LabItem>>>()
        .map_err(|e| Source::SQLx.to_error(500, e, "Select LabItem"))
}

// POST /lab/read-id/{lab_order_number}
pub async fn post_lab_read(lab_order_number: i32, loginname: &str, pool: &Pool<MySql>, kphis: &str) -> Result<ExecuteResponse, AppError> {
    let sql = lab::insert_ignore_lab_read(kphis);
    let result = sqlx::query(AssertSqlSafe(sql))
        .bind(lab_order_number)
        .bind(loginname)
        .bind(loginname)
        .bind(loginname)
        .execute(pool)
        .await
        .map_err(|e| Source::SQLx.to_error(500, e, "Insert LabRead"))?;

    Ok(ExecuteResponse::from_query_result(result, "Insert LabRead"))
}

// DELETE /lab/read-id/{lab_order_number}
pub async fn delete_lab_read(lab_order_number: i32, pool: &Pool<MySql>, kphis: &str) -> Result<ExecuteResponse, AppError> {
    let sql = lab::delete_lab_read(kphis);
    let result = sqlx::query(AssertSqlSafe(sql))
        .bind(lab_order_number)
        .execute(pool)
        .await
        .map_err(|e| Source::SQLx.to_error(500, e, "Delete LabRead"))?;

    Ok(ExecuteResponse::from_query_result(result, "Delete LabRead"))
}

#[cfg(test)]
#[rustfmt::skip]
mod tests {

    use super::*;
    use time::macros::date;
    use kphis_sqlx_tester::MySqlTester;

    #[tokio::test]
    #[ignore]
    async fn sqlx_get_wbc_band() {
        let tester = MySqlTester::new_hosxp().await;
        sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/create/hosxp/lab_head.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/create/hosxp/lab_order.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/insert/hosxp/lab_head.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/insert/hosxp/lab_order.sql")).execute(&tester.db_pool).await.unwrap();

        let hn_found = get_wbc_band("hn","0001234",78,364,&tester.db_pool,&tester.hosxp).await.unwrap();
        assert_eq!(hn_found.len(),4);
        let hn_not_found = get_wbc_band("hn","0006666",78,364,&tester.db_pool,&tester.hosxp).await.unwrap();
        assert!(hn_not_found.is_empty());
        let vn_found_vn = get_wbc_band("vn","661231235959",78,364,&tester.db_pool,&tester.hosxp).await.unwrap();
        assert_eq!(vn_found_vn.len(),1);
        let vn_found_an = get_wbc_band("vn","660001234",78,364,&tester.db_pool,&tester.hosxp).await.unwrap();
        assert_eq!(vn_found_an.len(),1);
        let vn_not_found = get_wbc_band("vn","6666",78,364,&tester.db_pool,&tester.hosxp).await.unwrap();
        assert!(vn_not_found.is_empty());
        // test key not used in app
        let value_parsable = get_wbc_band("lab_order_number","1",78,364,&tester.db_pool,&tester.hosxp).await.unwrap();
        assert_eq!(value_parsable.len(),1);
        // 'lab_head' column name(key) that value cannot parse will be default of column type
        let value_to_default = get_wbc_band("lab_order_number","A",78,364,&tester.db_pool,&tester.hosxp).await.unwrap();
        assert!(value_to_default.is_empty());
        // no column named 'xxx' will 500
        let key_error = get_wbc_band("xxx","1",78,364,&tester.db_pool,&tester.hosxp).await;
        assert!(key_error.is_err());
    }

    #[tokio::test]
    #[ignore]
    async fn sqlx_select_lab_head_inner() {
        let tester = MySqlTester::new_hosxp_and_kphis().await;
        sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/create/hosxp/lab_head.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/create/hosxp/lab_order.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/create/hosxp/lab_order_image.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/create/hosxp/lab_order_service.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/create/hosxp/lab_items.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/create/hosxp/lab_specimen_items.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/create/hosxp/doctor.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/create/hosxp/opduser.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/create/kphis/ipd_lab_read.sql")).execute(&tester.db_pool).await.unwrap();

        sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/insert/hosxp/lab_head.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/insert/hosxp/lab_order.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/insert/hosxp/lab_order_image.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/insert/hosxp/lab_order_service.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/insert/hosxp/lab_items.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/insert/hosxp/lab_specimen_items.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/insert/hosxp/doctor.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/insert/hosxp/opduser.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/insert/kphis/ipd_lab_read.sql")).execute(&tester.db_pool).await.unwrap();

        let default = get_lab_head_inner(&LabHeadParams::default(),&tester.db_pool,&tester.hosxp,&tester.kphis).await.unwrap();
        assert!(default.is_empty());

        let hn_found = get_lab_head_inner(&LabHeadParams {hn: Some(String::from("0001234")),..Default::default()},&tester.db_pool,&tester.hosxp,&tester.kphis).await.unwrap();
        assert_eq!(hn_found.len(), 4);

        let hn_found_before = get_lab_head_inner(&LabHeadParams {hn: Some(String::from("0001234")),end_date: Some(date!(2023-01-01)),..Default::default()},&tester.db_pool,&tester.hosxp,&tester.kphis).await.unwrap();
        assert_eq!(hn_found_before.len(), 2);

        let hn_found_after = get_lab_head_inner(&LabHeadParams {hn: Some(String::from("0001234")),start_date: Some(date!(2024-01-01)),..Default::default()},&tester.db_pool,&tester.hosxp,&tester.kphis).await.unwrap();
        assert_eq!(hn_found_after.len(), 2);

        let hn_found_between = get_lab_head_inner(&LabHeadParams {hn: Some(String::from("0001234")),start_date: Some(date!(2024-01-01)),end_date: Some(date!(2024-01-01)),..Default::default()},&tester.db_pool,&tester.hosxp,&tester.kphis).await.unwrap();
        assert_eq!(hn_found_between.len(), 2);

        let hn_not_found = get_lab_head_inner(&LabHeadParams {hn: Some(String::from("0006666")),..Default::default()},&tester.db_pool,&tester.hosxp,&tester.kphis).await.unwrap();
        assert!(hn_not_found.is_empty());

        let vn_found_vn = get_lab_head_inner(&LabHeadParams {vn: Some(String::from("661231235959")),..Default::default()},&tester.db_pool,&tester.hosxp,&tester.kphis).await.unwrap();
        assert_eq!(vn_found_vn.len(), 1);

        let vn_found_an = get_lab_head_inner(&LabHeadParams {vn: Some(String::from("660001234")),..Default::default()},&tester.db_pool,&tester.hosxp,&tester.kphis).await.unwrap();
        assert_eq!(vn_found_an.len(), 1);

        let vn_not_found = get_lab_head_inner(&LabHeadParams {vn: Some(String::from("6666")),..Default::default()},&tester.db_pool,&tester.hosxp,&tester.kphis).await.unwrap();
        assert!(vn_not_found.is_empty());

        let id_found = get_lab_head_inner(&LabHeadParams {hn: Some(String::from("0001234")),id: Some(3),..Default::default()},&tester.db_pool,&tester.hosxp,&tester.kphis).await.unwrap();
        assert_eq!(id_found.len(), 1);

        let id_found_with_limit = get_lab_head_inner(&LabHeadParams {hn: Some(String::from("0001234")),id: Some(3),prev: Some(2),..Default::default()},&tester.db_pool,&tester.hosxp,&tester.kphis).await.unwrap();
        assert_eq!(id_found_with_limit.len(), 2);

        let id_not_found = get_lab_head_inner(&LabHeadParams {hn: Some(String::from("0001234")),id: Some(999),..Default::default()},&tester.db_pool,&tester.hosxp,&tester.kphis).await.unwrap();
        assert!(id_not_found.is_empty());
    }

    #[tokio::test]
    #[ignore]
    async fn sqlx_get_lab_ids_with_previous() {
        let tester = MySqlTester::new_hosxp().await;
        sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/create/hosxp/lab_head.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/create/hosxp/lab_order.sql")).execute(&tester.db_pool).await.unwrap();

        sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/insert/hosxp/lab_head.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/insert/hosxp/lab_order.sql")).execute(&tester.db_pool).await.unwrap();

        let found = get_lab_ids_with_previous("0001234",3,2,&tester.db_pool,&tester.hosxp).await.unwrap();
        assert_eq!(found.len(),2);

        let not_found = get_lab_ids_with_previous("0006666",3,2,&tester.db_pool,&tester.hosxp).await.unwrap();
        assert!(not_found.is_empty());
    }

    #[tokio::test]
    #[ignore]
    async fn sqlx_select_lab_detail_group() {
        let tester = MySqlTester::new_hosxp().await;
        sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/create/hosxp/lab_head.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/create/hosxp/lab_order.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/create/hosxp/lab_items.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/create/hosxp/lab_items_group.sql")).execute(&tester.db_pool).await.unwrap();

        sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/insert/hosxp/lab_head.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/insert/hosxp/lab_order.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/insert/hosxp/lab_items.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/insert/hosxp/lab_items_group.sql")).execute(&tester.db_pool).await.unwrap();

        let found = select_lab_detail_group(1,&tester.db_pool,&tester.hosxp).await.unwrap();
        assert_eq!(found.len(),1);
        let not_found = select_lab_detail_group(999,&tester.db_pool,&tester.hosxp).await.unwrap();
        assert!(not_found.is_empty());
    }

    #[tokio::test]
    #[ignore]
    async fn sqlx_select_lab_items() {
        let tester = MySqlTester::new_hosxp().await;
        sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/create/hosxp/lab_head.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/create/hosxp/lab_order.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/create/hosxp/lab_items.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/create/hosxp/lab_items_doctor.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/create/hosxp/lab_items_group.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/create/hosxp/lab_items_visible.sql")).execute(&tester.db_pool).await.unwrap();

        sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/insert/hosxp/lab_head.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/insert/hosxp/lab_order.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/insert/hosxp/lab_items.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/insert/hosxp/lab_items_group.sql")).execute(&tester.db_pool).await.unwrap();

        let allow_doctorcode = Some(String::from("007"));
        let allow_groupname = Some(String::from("BIOCHEMISTRY"));

        let num_found = select_lab_items(Some(1),None,true,&LabItemParams::default(),&allow_doctorcode,&allow_groupname,&tester.db_pool,&tester.hosxp).await.unwrap();
        assert_eq!(num_found.len(), 2);
        let num_not_found = select_lab_items(Some(999),None,true,&LabItemParams::default(),&allow_doctorcode,&allow_groupname,&tester.db_pool,&tester.hosxp).await.unwrap();
        assert!(num_not_found.is_empty());
        let group_found_some = select_lab_items(None,Some(Some(3)),true,&LabItemParams::default(),&allow_doctorcode,&allow_groupname,&tester.db_pool,&tester.hosxp).await.unwrap();
        assert_eq!(group_found_some.len(), 8);
        let group_not_found_some = select_lab_items(None,Some(Some(999)),true,&LabItemParams::default(),&allow_doctorcode,&allow_groupname,&tester.db_pool,&tester.hosxp).await.unwrap();
        assert!(group_not_found_some.is_empty());
        let group_found_none = select_lab_items(None,Some(None),true,&LabItemParams::default(),&allow_doctorcode,&allow_groupname,&tester.db_pool,&tester.hosxp).await.unwrap();
        assert!(group_found_none.is_empty());
        let params_found_hn = select_lab_items(None,None,true,&LabItemParams {hn: Some(String::from("0001234")),..Default::default()},&allow_doctorcode,&allow_groupname,&tester.db_pool,&tester.hosxp).await.unwrap();
        assert_eq!(params_found_hn.len(), 8);
        let params_not_found_hn = select_lab_items(None,None,true,&LabItemParams {hn: Some(String::from("0006666")),..Default::default()},&allow_doctorcode,&allow_groupname,&tester.db_pool,&tester.hosxp).await.unwrap();
        assert!(params_not_found_hn.is_empty());
        let params_found_vn = select_lab_items(None,None,true,&LabItemParams {vn: Some(String::from("661231235959")),..Default::default()},&allow_doctorcode,&allow_groupname,&tester.db_pool,&tester.hosxp).await.unwrap();
        assert_eq!(params_found_vn.len(), 2);
        let params_not_found_vn = select_lab_items(None,None,true,&LabItemParams {vn: Some(String::from("991231235959")),..Default::default()},&allow_doctorcode,&allow_groupname,&tester.db_pool,&tester.hosxp).await.unwrap();
        assert!(params_not_found_vn.is_empty());
        let params_found_code = select_lab_items(None,None,true,&LabItemParams {lab_items_code: Some(78),..Default::default()},&allow_doctorcode,&allow_groupname,&tester.db_pool,&tester.hosxp).await.unwrap();
        assert_eq!(params_found_code.len(), 4);
        let params_not_found_code = select_lab_items(None,None,true,&LabItemParams {lab_items_code: Some(999),..Default::default()},&allow_doctorcode,&allow_groupname,&tester.db_pool,&tester.hosxp).await.unwrap();
        assert!(params_not_found_code.is_empty());

        // lab_items_visible has 'item code matched' + 'group NOT match' will NOT mark result as '[[ปกปิด]]'
        sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/insert/hosxp/lab_items_visible.sql")).execute(&tester.db_pool).await.unwrap();
        let group_found_some_visible = select_lab_items(None,Some(Some(3)),true,&LabItemParams::default(),&allow_doctorcode,&Some(String::from("HEMATOLOGY")),&tester.db_pool,&tester.hosxp).await.unwrap();
        assert_eq!(group_found_some_visible.iter().filter(|li| li.lab_order_result.as_ref().map(|res| res.as_str() == "[[ปกปิด]]").unwrap_or_default()).count(),4);

        // lab_items_doctor has 'item code matched' + 'doctorcode NOT match' will mark result as '[[ปกปิด]]'
        sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/insert/hosxp/lab_items_doctor.sql")).execute(&tester.db_pool).await.unwrap();
        let group_found_some_doctor = select_lab_items(None,Some(Some(3)),true,&LabItemParams::default(),&allow_doctorcode,&Some(String::from("HEMATOLOGY")),&tester.db_pool,&tester.hosxp).await.unwrap();
        assert_eq!(group_found_some_doctor.iter().filter(|li| li.lab_order_result.as_ref().map(|res| res.as_str() == "[[ปกปิด]]").unwrap_or_default()).count(),8);
    }

    #[tokio::test]
    #[ignore]
    async fn sqlx_post_lab_read() {
        let tester = MySqlTester::new_kphis().await;
        sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/create/kphis/ipd_lab_read.sql")).execute(&tester.db_pool).await.unwrap();

        let resp = post_lab_read(1,"user",&tester.db_pool,&tester.kphis).await.unwrap();
        assert_eq!(resp.rows_affected,1);
        // INSERT IGNORE test with same PK
        let resp_again = post_lab_read(1,"user",&tester.db_pool,&tester.kphis).await.unwrap();
        assert_eq!(resp_again.rows_affected,0);
    }

    #[tokio::test]
    #[ignore]
    async fn sqlx_delete_lab_read() {
        let tester = MySqlTester::new_kphis().await;
        sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/create/kphis/ipd_lab_read.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/insert/kphis/ipd_lab_read.sql")).execute(&tester.db_pool).await.unwrap();

        let resp = delete_lab_read(1,&tester.db_pool,&tester.kphis).await.unwrap();
        assert_eq!(resp.rows_affected,1);
        let resp_again = delete_lab_read(1,&tester.db_pool,&tester.kphis).await.unwrap();
        assert_eq!(resp_again.rows_affected,0);
    }
}
