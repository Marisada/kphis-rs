use sqlx::{AssertSqlSafe, FromRow, MySql, Pool};

use kphis_model::image::{
    ImageBase64,
    scan_his::{ScanHis5, ScanHis10, ScanImage, ScanImageParams},
};
use kphis_sql::image::scan_his;
use kphis_util::error::{AppError, Source};

use super::image_from_row;
use crate::{query1_all, query1_opt};

pub async fn get_scan_his_image(params: &ScanImageParams, pool: &Pool<MySql>, hosxp: &str) -> Result<Vec<ScanImage>, AppError> {
    if let Some(key) = params.key.as_ref() {
        let vn = params.vn.clone().unwrap_or_default();
        match key.as_str() {
            "pe" => get_pe_image(&vn, pool, hosxp).await,
            "er" => get_er_image(&vn, pool, hosxp).await,
            "lab" => get_lab_image(&vn, &params.an, pool, hosxp).await,
            "opd" => get_opd_image(&vn, pool, hosxp).await,
            _ => Err(AppError::app_400("Select ScanHisImage")),
        }
    } else {
        Err(AppError::app_400("Select ScanHisImage"))
    }
}

// hos.patient_opd_scan
// - patient_opd_scan_id: u32
// - vn: Option<String>
// - image: Option<Vec<u8>>
pub async fn get_opd_image(vn: &str, pool: &Pool<MySql>, hosxp: &str) -> Result<Vec<ScanImage>, AppError> {
    let sql = scan_his::select_opd_scan_image(hosxp);
    let result = query1_all(vn, &sql, pool, "Select OpdScanImage")
        .await?
        .iter()
        .map(|row| {
            if let Ok(Some(image)) = image_from_row(row) {
                ScanImage { image, note: None }
            } else {
                ScanImage {
                    image: ImageBase64::new_warn(),
                    note: None,
                }
            }
        })
        .collect::<Vec<ScanImage>>();

    Ok(result)
}

// hos.pe_image
// - vn: String // UNIQUE
// - image1 .. image10 : Option<Vec<u8>>
// - image1_note .. image10_note : Option<String>
pub async fn get_pe_image(vn: &str, pool: &Pool<MySql>, hosxp: &str) -> Result<Vec<ScanImage>, AppError> {
    let sql = scan_his::select_pe_image(hosxp);
    let result = query1_opt(vn, &sql, pool, "Select PeImage")
        .await?
        .as_ref()
        .map(ScanHis10::from_row)
        .transpose()
        .map_err(|e| Source::SQLx.to_error(500, e, "Select PeImage"))?
        .map(|sc| sc.to_vec())
        .unwrap_or_default();

    Ok(result)
}

// hos.er_image
// - vn: String // UNIQUE
// - image1 .. image5 : Option<Vec<u8>>
// - image1_note .. image5_note : Option<String>
pub async fn get_er_image(vn: &str, pool: &Pool<MySql>, hosxp: &str) -> Result<Vec<ScanImage>, AppError> {
    let sql = scan_his::select_er_image(hosxp);
    let result = query1_opt(vn, &sql, pool, "Select ErImage")
        .await?
        .as_ref()
        .map(ScanHis5::from_row)
        .transpose()
        .map_err(|e| Source::SQLx.to_error(500, e, "Select ErImage"))?
        .map(|sc| sc.to_vec())
        .unwrap_or_default();

    Ok(result)
}

// hos.lab_order_image
// - lab_order_number: i32
// - image1 .. image5 : Option<Vec<u8>>
// - image1_note .. image5_note : Option<String>
pub async fn get_lab_image(vn: &str, an_opt: &Option<String>, pool: &Pool<MySql>, hosxp: &str) -> Result<Vec<ScanImage>, AppError> {
    let sql = scan_his::select_lab_image(an_opt.is_some(), hosxp);
    let mut query = sqlx::query(AssertSqlSafe(sql)).bind(vn);
    if let Some(an) = an_opt.as_ref() {
        query = query.bind(an);
    }
    let result = query
        .fetch_all(pool)
        .await
        .map_err(|e| Source::SQLx.to_error(500, e, "Select LabImage"))?
        .iter()
        .map(ScanHis5::from_row)
        .collect::<sqlx::Result<Vec<ScanHis5>>>()
        .map_err(|e| Source::SQLx.to_error(500, e, "Select LabImage"))?
        .iter()
        .flat_map(|sc| sc.to_vec())
        .collect::<Vec<ScanImage>>();

    Ok(result)
}

// hos.lab_order_image
// - lab_order_number: i32
// - image1 .. image5 : Option<Vec<u8>>
// - image1_note .. image5_note : Option<String>
pub async fn get_lab_image_from_lab_order_number(lab_order_number: i32, pool: &Pool<MySql>, hosxp: &str) -> Result<Vec<ScanImage>, AppError> {
    let sql = scan_his::select_lab_image_from_lab_order_number(hosxp);
    let result = sqlx::query(AssertSqlSafe(sql))
        .bind(lab_order_number)
        .fetch_all(pool)
        .await
        .map_err(|e| Source::SQLx.to_error(500, e, "Select LabImageByID"))?
        .iter()
        .map(ScanHis5::from_row)
        .collect::<sqlx::Result<Vec<ScanHis5>>>()
        .map_err(|e| Source::SQLx.to_error(500, e, "Select LabImageByID"))?
        .iter()
        .flat_map(|sc| sc.to_vec())
        .collect::<Vec<ScanImage>>();

    Ok(result)
}

#[cfg(test)]
#[rustfmt::skip]
mod tests {

    use super::*;
    use kphis_sqlx_tester::MySqlTester;

    #[tokio::test]
    #[ignore]
    async fn sqlx_get_opd_image() {
        let tester = MySqlTester::new_hosxp().await;
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/hosxp/patient_opd_scan.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/hosxp/patient_opd_scan.sql")).execute(&tester.db_pool).await.unwrap();

        let not_found = get_opd_image("670101111111", &tester.db_pool, &tester.hosxp).await.unwrap();
        assert!(not_found.is_empty());
        let found = get_opd_image("651231235959", &tester.db_pool, &tester.hosxp).await.unwrap();
        assert_eq!(found.len(), 1);
    }

    #[tokio::test]
    #[ignore]
    async fn sqlx_get_pe_image() {
        let tester = MySqlTester::new_hosxp().await;
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/hosxp/pe_image.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/hosxp/pe_image.sql")).execute(&tester.db_pool).await.unwrap();

        let not_found = get_pe_image("670101111111", &tester.db_pool, &tester.hosxp).await.unwrap();
        assert!(not_found.is_empty());
        // test image is invalid, but function need to return warn image
        let found = get_pe_image("651231235959", &tester.db_pool, &tester.hosxp).await.unwrap();
        assert!(found.len() == 1);
    }

    #[tokio::test]
    #[ignore]
    async fn sqlx_get_er_image() {
        let tester = MySqlTester::new_hosxp().await;
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/hosxp/er_image.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/hosxp/er_image.sql")).execute(&tester.db_pool).await.unwrap();

        let not_found = get_er_image("670101111111", &tester.db_pool, &tester.hosxp).await.unwrap();
        assert!(not_found.is_empty());
        // test image is invalid, but function need to return warn image
        let found = get_er_image("651231235959", &tester.db_pool, &tester.hosxp).await.unwrap();
        assert!(found.len() == 1);
    }

    #[tokio::test]
    #[ignore]
    async fn sqlx_get_lab_image() {
        let tester = MySqlTester::new_hosxp().await;
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/hosxp/lab_head.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/hosxp/lab_order_image.sql")).execute(&tester.db_pool).await.unwrap();

        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/hosxp/lab_head.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/hosxp/lab_order_image.sql")).execute(&tester.db_pool).await.unwrap();

        let not_found = get_lab_image("670101111111", &Some(String::from("670000001")), &tester.db_pool, &tester.hosxp).await.unwrap();
        assert!(not_found.is_empty());
        let no_image = get_lab_image("661231235959", &Some(String::from("660001234")), &tester.db_pool, &tester.hosxp).await.unwrap();
        assert!(no_image.is_empty());
        // test image is invalid, but function need to return warn image
        let found_opd = get_lab_image("651231235959", &None, &tester.db_pool, &tester.hosxp).await.unwrap();
        assert!(found_opd.len() == 1);
        let found_ipd = get_lab_image("651231235959", &Some(String::from("650001234")), &tester.db_pool, &tester.hosxp).await.unwrap();
        assert!(found_ipd.len() == 2);
    }
}
