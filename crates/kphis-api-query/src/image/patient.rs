use sqlx::{MySql, Pool, Row, mysql::MySqlRow};

use kphis_sql::image::patient;
use kphis_util::error::{AppError, Source};

use crate::query1_opt;

// get-patient-image.php
pub async fn get_patient_image(hn: &str, pool: &Pool<MySql>, hosxp: &str) -> Result<Option<Vec<u8>>, AppError> {
    let sql = patient::select_patient_image(hosxp);
    Ok(query1_opt(hn, &sql, pool, "Select PatientImage").await?.as_ref().map(patient_image_row).transpose()?.flatten())
}
fn patient_image_row(row: &MySqlRow) -> Result<Option<Vec<u8>>, AppError> {
    row.try_get(0).map_err(|e| Source::SQLx.to_error(500, e, "Select PatientImage"))
}

#[cfg(test)]
#[rustfmt::skip]
mod tests {

    use super::*;
    use kphis_sqlx_tester::MySqlTester;

    #[tokio::test]
    #[ignore]
    async fn sqlx_get_patient_image() {
        let tester = MySqlTester::new_hosxp().await;
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/hosxp/patient_image.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/hosxp/patient_image.sql")).execute(&tester.db_pool).await.unwrap();

        let found_image = get_patient_image("0001234", &tester.db_pool, &tester.hosxp).await.unwrap();
        assert!(found_image.is_some());
        let found_null = get_patient_image("0001111", &tester.db_pool, &tester.hosxp).await.unwrap();
        assert!(found_null.is_none());
        let not_found = get_patient_image("0006666", &tester.db_pool, &tester.hosxp).await.unwrap();
        assert!(not_found.is_none());
    }
}
