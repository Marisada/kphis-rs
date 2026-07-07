use sqlx::{MySql, Pool};
use std::{
    path::Path,
    sync::Arc, // RwLock},
};
use strum::IntoEnumIterator;
use tokio::sync::Mutex;

use kphis_model::{
    app::AppAsset,
    image::file_path::DocumentType,
    select_utils::{ColorSelectOption, SelectOption},
};
use kphis_sql::select_utils as sql;
use kphis_util::{datetime::get_timestamp_server, error::AppError}; // , util::hash_to_base64_string};

use crate::{search::searchbox::get_drugusage_searchbox, select_utils as query};

const SAVE_FILE: &str = "volume/app_assets.bin";

// rules
// 1. start server => load save file first, then database
// 2. cache expired => database first
// 3. always save to file after loading from database

// loading app assets from database take time 1-5 minutes
// so we load from save file first to decrease system downtime
/// if load from file, it will load from database in the next call after 5 minutes
pub async fn new_app_asset(
    // special item from config, may be bundle in the future
    fcnote_patient_types: &[ColorSelectOption],
    app_asset_cache_minutes: u64,
    pool: &Pool<MySql>,
    hosxp: &str,
    kphis: &str,
) -> Result<(u64, Arc<Mutex<AppAsset>>, Arc<Mutex<Vec<u8>>>), AppError> {
    let (app_asset, app_asset_bytes, is_from_file) = load_app_asset(false, fcnote_patient_types, pool, hosxp, kphis).await?;
    let exp_plus = if is_from_file { 300 } else { app_asset_cache_minutes * 60 };
    let exp = get_timestamp_server()? + exp_plus;
    // let etag = hash_to_base64_string(&app_asset_bytes);
    Ok((
        exp,
        // Arc::new(RwLock::new(etag)),
        Arc::new(Mutex::new(app_asset)),
        Arc::new(Mutex::new(app_asset_bytes)),
    ))
}

/// can serve AppAsset from database or saved file
/// return (AppAsset, is_from_file)
pub async fn load_app_asset(
    db_first: bool,
    // special item from config, may be bundle in the future
    fcnote_patient_types: &[ColorSelectOption],
    pool: &Pool<MySql>,
    hosxp: &str,
    kphis: &str,
) -> Result<(AppAsset, Vec<u8>, bool), AppError> {
    let from_file_opt = if db_first {
        tracing::info!("Updating app assets cache from database..");
        None
    } else if Path::new(SAVE_FILE).exists() {
        load_app_asset_from_file(SAVE_FILE).await
    } else {
        tracing::info!("Initiating app assets from database..");
        None
    };
    match from_file_opt {
        Some((asset, bytes)) => Ok((asset, bytes, true)),
        None => load_and_save_app_asset_from_db(fcnote_patient_types, pool, hosxp, kphis).await,
    }
}

pub async fn load_app_asset_from_file(file_path: &str) -> Option<(AppAsset, Vec<u8>)> {
    match tokio::fs::read(file_path).await {
        Ok(bytes) => match bitcode::decode(&bytes) {
            Ok(asset) => Some((asset, bytes)),
            Err(e) => {
                tracing::warn!("parsing saved app assets from {} failed: {}", file_path, e);
                None
            }
        },
        Err(e) => {
            tracing::warn!("loading saved app assets from {} failed: {}", file_path, e);
            None
        }
    }
}

pub async fn load_and_save_app_asset_from_db(
    // special item from config, may be bundle in the future
    fcnote_patient_types: &[ColorSelectOption],
    pool: &Pool<MySql>,
    hosxp: &str,
    kphis: &str,
) -> Result<(AppAsset, Vec<u8>, bool), AppError> {
    let er_bed_select_options = query::get_color_select_options(&sql::get_er_bed_select_option(kphis), pool).await?;
    let er_patient_status_select_options = query::get_select_option_u32_key(&sql::get_er_patient_status_select_option(kphis), pool).await?;
    let er_dch_type_select_options = query::get_select_option_u32_key(&sql::get_er_dch_type_select_option(kphis), pool).await?;
    let ward_select_option = query::get_select_option(&sql::get_ward_select_option(hosxp), pool).await?;
    let doctor_select_option = query::get_select_option(&sql::get_doctor_select_option(hosxp), pool).await?;
    let all_doctor_select_option = query::get_select_option(&sql::get_all_doctor_select_option(hosxp), pool).await?;
    let spclty_select_option = query::get_select_option(&sql::get_spclty_select_option(hosxp), pool).await?;
    let spclty_kphis_select_option = query::get_select_option_u32_key(&sql::get_kphis_spclty_select_option(kphis), pool).await?;
    let inscl_select_option = query::get_select_option(&sql::get_inscl_select_option(hosxp), pool).await?;
    let emergency_select_option = query::get_select_option_u32_key(&sql::get_emergency_select_option(kphis), pool).await?;
    let emergency_level_select_option = query::get_select_option_i32_key(&sql::get_emergency_level_select_option(hosxp), pool).await?;
    let consult_type_select_option = query::get_select_option_u32_key(&sql::get_consult_type_select_option(kphis), pool).await?;
    let conscious_select_option = query::get_select_option_u32_key(&sql::get_conscious_select_option(kphis), pool).await?;
    let urine_amount_select_option = query::get_select_option_u32_key(&sql::get_urine_amount_select_option(kphis), pool).await?;
    let urine_duration_select_option = query::get_select_option_u32_key(&sql::get_urine_duration_select_option(kphis), pool).await?;
    let line_select_option = query::get_select_option_u32_key(&sql::get_line_select_option(kphis), pool).await?;
    let cha_select_option = query::get_select_option_u32_key(&sql::get_cha_select_option(kphis), pool).await?;
    let va_select_option = query::get_select_option_u32_key(&sql::get_va_select_option(kphis), pool).await?;
    let mass_select_option = query::get_select_option_u32_key(&sql::get_mass_select_option(kphis), pool).await?;
    let motor_select_option = query::get_select_option_u32_key(&sql::get_lt_arm_select_option(kphis), pool).await?;
    let o2_select_option = query::get_select_option_u32_key(&sql::get_o2_select_option(kphis), pool).await?;
    let tube_select_option = query::get_select_option_u32_key(&sql::get_tube_select_option(kphis), pool).await?;
    // let intake_select_option = query::get_select_option_u32_key(&sql::get_intake_select_option(kphis), pool).await?;
    // let output_select_option = query::get_select_option_u32_key(&sql::get_output_select_option(kphis), pool).await?;
    let lr_sta_select_option = query::get_select_option_u32_key(&sql::get_lr_sta_select_option(kphis), pool).await?;
    let lr_mem_select_option = query::get_select_option_u32_key(&sql::get_lr_mem_select_option(kphis), pool).await?;
    let lr_moulding_select_option = query::get_select_option_u32_key(&sql::get_lr_moulding_select_option(kphis), pool).await?;
    let dipstick_select_option = query::get_select_option_u32_key(&sql::get_dipstick_select_option(kphis), pool).await?;
    let breathing_select_option = query::get_select_option_u32_key(&sql::get_breathing_select_option(kphis), pool).await?;
    let avpu_select_option = query::get_select_option_u32_key(&sql::get_avpu_select_option(kphis), pool).await?;
    let gut_feeling_select_option = query::get_select_option_u32_key(&sql::get_gut_feeling_select_option(kphis), pool).await?;
    let pops_other_select_option = query::get_select_option_u32_key(&sql::get_pops_other_select_option(kphis), pool).await?;
    let stage_of_change_select_option = query::get_select_option_u32_key(&sql::get_stage_of_change_select_option(kphis), pool).await?;
    let refer_type_select_option = query::get_select_option_i32_key(&sql::get_refer_type_select_option(hosxp), pool).await?;
    let refer_cause_select_option = query::get_select_option_i32_key(&sql::get_refer_cause_select_option(hosxp), pool).await?;
    let refer_point_select_option = query::get_select_option(&sql::get_refer_point_select_option(hosxp), pool).await?;
    let moph_refer_expire_type_select_option = query::get_select_option_i32_key(&sql::get_moph_refer_expire_type_select_option(hosxp), pool).await?;
    // let icd10_keywords = get_icd10_searchbox(false, pool, hosxp, kphis).await?;
    // let icd10_keywords_ext = get_icd10_searchbox(true, pool, hosxp, kphis).await?;
    let document_type_select_option = DocumentType::iter()
        .map(|dt| SelectOption {
            key: dt.as_str().to_owned(),
            value: dt.label().to_owned(),
        })
        .collect::<Vec<SelectOption>>();
    let drugusages = get_drugusage_searchbox(pool, hosxp).await?;

    let asset = AppAsset {
        fcnote_patient_type_select_options: fcnote_patient_types.to_vec(),
        er_bed_select_options,
        er_patient_status_select_options,
        er_dch_type_select_options,
        ward_select_option,
        doctor_select_option,
        all_doctor_select_option,
        spclty_select_option,
        spclty_kphis_select_option,
        inscl_select_option,
        emergency_select_option,
        emergency_level_select_option,
        consult_type_select_option,
        conscious_select_option,
        urine_amount_select_option,
        urine_duration_select_option,
        line_select_option,
        cha_select_option,
        va_select_option,
        mass_select_option,
        motor_select_option,
        o2_select_option,
        tube_select_option,
        // intake_select_option,
        // output_select_option,
        lr_sta_select_option,
        lr_mem_select_option,
        lr_moulding_select_option,
        dipstick_select_option,
        breathing_select_option,
        avpu_select_option,
        gut_feeling_select_option,
        pops_other_select_option,
        stage_of_change_select_option,
        refer_type_select_option,
        refer_cause_select_option,
        refer_point_select_option,
        moph_refer_expire_type_select_option,
        // icd10_keywords,
        // icd10_keywords_ext,
        document_type_select_option,
        drugusages,
    };

    let bytes = bitcode::encode(&asset);
    if let Err(e) = tokio::fs::write(SAVE_FILE, &bytes).await {
        tracing::warn!("Error save app assets to {} failure: {}", SAVE_FILE, e);
    } else {
        tracing::info!("App assets cached and saved to {}", SAVE_FILE);
    }

    Ok((asset, bytes, false))
}

#[cfg(test)]
#[rustfmt::skip]
mod tests {

    use super::*;
    use kphis_sqlx_tester::MySqlTester;

    #[tokio::test]
    #[ignore]
    async fn sqlx_get_er_bed_select_option() {
        let tester = MySqlTester::new_kphis().await;
        sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/create/kphis/opd_er_bed.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/create/kphis/opd_er_bed_type.sql")).execute(&tester.db_pool).await.unwrap();

        sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/insert/kphis/opd_er_bed.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/insert/kphis/opd_er_bed_type.sql")).execute(&tester.db_pool).await.unwrap();

        let found = query::get_color_select_options(&sql::get_er_bed_select_option(&tester.kphis),&tester.db_pool).await.unwrap();
        assert!(!found.is_empty());
    }

    #[tokio::test]
    #[ignore]
    async fn sqlx_get_er_patient_status_select_option() {
        let tester = MySqlTester::new_kphis().await;
        sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/create/kphis/opd_er_patient_status.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/insert/kphis/opd_er_patient_status.sql")).execute(&tester.db_pool).await.unwrap();

        let found = query::get_select_option_u32_key(&sql::get_er_patient_status_select_option(&tester.kphis),&tester.db_pool).await.unwrap();
        assert!(!found.is_empty());
    }

    #[tokio::test]
    #[ignore]
    async fn sqlx_get_er_dch_type_select_option() {
        let tester = MySqlTester::new_kphis().await;
        sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/create/kphis/opd_er_dch_type.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/insert/kphis/opd_er_dch_type.sql")).execute(&tester.db_pool).await.unwrap();

        let found = query::get_select_option_u32_key(&sql::get_er_dch_type_select_option(&tester.kphis),&tester.db_pool).await.unwrap();
        assert!(!found.is_empty());
    }

    #[tokio::test]
    #[ignore]
    async fn sqlx_get_ward_select_option() {
        let tester = MySqlTester::new_hosxp().await;
        sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/create/hosxp/ward.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/insert/hosxp/ward.sql")).execute(&tester.db_pool).await.unwrap();

        let found =
            query::get_select_option(&sql::get_ward_select_option(&tester.hosxp), &tester.db_pool).await.unwrap();
        assert!(!found.is_empty());
    }

    #[tokio::test]
    #[ignore]
    async fn sqlx_get_doctor_select_option() {
        let tester = MySqlTester::new_hosxp().await;
        sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/create/hosxp/doctor.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/insert/hosxp/doctor.sql")).execute(&tester.db_pool).await.unwrap();

        let found = query::get_select_option(&sql::get_doctor_select_option(&tester.hosxp),&tester.db_pool).await.unwrap();
        assert!(!found.is_empty());
    }

    #[tokio::test]
    #[ignore]
    async fn sqlx_get_all_doctor_select_option() {
        let tester = MySqlTester::new_hosxp().await;
        sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/create/hosxp/doctor.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/insert/hosxp/doctor.sql")).execute(&tester.db_pool).await.unwrap();

        let found = query::get_select_option(&sql::get_all_doctor_select_option(&tester.hosxp),&tester.db_pool).await.unwrap();
        assert!(!found.is_empty());
    }

    #[tokio::test]
    #[ignore]
    async fn sqlx_get_spclty_select_option() {
        let tester = MySqlTester::new_hosxp().await;
        sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/create/hosxp/spclty.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/insert/hosxp/spclty.sql")).execute(&tester.db_pool).await.unwrap();

        let found = query::get_select_option(&sql::get_spclty_select_option(&tester.hosxp),&tester.db_pool).await.unwrap();
        assert!(!found.is_empty());
    }

    #[tokio::test]
    #[ignore]
    async fn sqlx_get_kphis_spclty_select_option() {
        let tester = MySqlTester::new_kphis().await;
        sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/create/kphis/kphis_spclty.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/insert/kphis/kphis_spclty.sql")).execute(&tester.db_pool).await.unwrap();

        let found = query::get_select_option_u32_key(&sql::get_kphis_spclty_select_option(&tester.kphis),&tester.db_pool).await.unwrap();
        assert!(!found.is_empty());
    }

    #[tokio::test]
    #[ignore]
    async fn sqlx_get_inscl_select_option() {
        let tester = MySqlTester::new_hosxp().await;
        sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/create/hosxp/nhso_inscl_code.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/insert/hosxp/nhso_inscl_code.sql")).execute(&tester.db_pool).await.unwrap();

        let found = query::get_select_option(&sql::get_inscl_select_option(&tester.hosxp),&tester.db_pool).await.unwrap();
        assert!(!found.is_empty());
    }

    #[tokio::test]
    #[ignore]
    async fn sqlx_get_emergency_select_option() {
        let tester = MySqlTester::new_kphis().await;
        sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/create/kphis/ipd_emergency.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/insert/kphis/ipd_emergency.sql")).execute(&tester.db_pool).await.unwrap();

        let found = query::get_select_option_u32_key(&sql::get_emergency_select_option(&tester.kphis),&tester.db_pool).await.unwrap();
        assert!(!found.is_empty());
    }

    #[tokio::test]
    #[ignore]
    async fn sqlx_get_emergency_level_select_option() {
        let tester = MySqlTester::new_hosxp().await;
        sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/create/hosxp/er_emergency_level.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/insert/hosxp/er_emergency_level.sql")).execute(&tester.db_pool).await.unwrap();

        let found = query::get_select_option_i32_key(&sql::get_emergency_level_select_option(&tester.hosxp),&tester.db_pool).await.unwrap();
        assert!(!found.is_empty());
    }

    #[tokio::test]
    #[ignore]
    async fn sqlx_get_consult_type_select_option() {
        let tester = MySqlTester::new_kphis().await;
        sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/create/kphis/ipd_dr_consult_type.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/insert/kphis/ipd_dr_consult_type.sql")).execute(&tester.db_pool).await.unwrap();

        let found = query::get_select_option_u32_key(&sql::get_consult_type_select_option(&tester.kphis),&tester.db_pool).await.unwrap();
        assert!(!found.is_empty());
    }

    #[tokio::test]
    #[ignore]
    async fn sqlx_get_conscious_select_option() {
        let tester = MySqlTester::new_kphis().await;
        sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/create/kphis/ipd_vs_conscious.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/insert/kphis/ipd_vs_conscious.sql")).execute(&tester.db_pool).await.unwrap();

        let found = query::get_select_option_u32_key(&sql::get_conscious_select_option(&tester.kphis),&tester.db_pool).await.unwrap();
        assert!(!found.is_empty());
    }

    #[tokio::test]
    #[ignore]
    async fn sqlx_get_urine_amount_select_option() {
        let tester = MySqlTester::new_kphis().await;
        sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/create/kphis/ipd_vs_urine_amount.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/insert/kphis/ipd_vs_urine_amount.sql")).execute(&tester.db_pool).await.unwrap();

        let found = query::get_select_option_u32_key(&sql::get_urine_amount_select_option(&tester.kphis),&tester.db_pool).await.unwrap();
        assert!(!found.is_empty());
    }

    #[tokio::test]
    #[ignore]
    async fn sqlx_get_urine_duration_select_option() {
        let tester = MySqlTester::new_kphis().await;
        sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/create/kphis/ipd_vs_urine_duration.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/insert/kphis/ipd_vs_urine_duration.sql")).execute(&tester.db_pool).await.unwrap();

        let found = query::get_select_option_u32_key(&sql::get_urine_duration_select_option(&tester.kphis),&tester.db_pool).await.unwrap();
        assert!(!found.is_empty());
    }

    #[tokio::test]
    #[ignore]
    async fn sqlx_get_line_select_option() {
        let tester = MySqlTester::new_kphis().await;
        sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/create/kphis/ipd_vs_line.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/insert/kphis/ipd_vs_line.sql")).execute(&tester.db_pool).await.unwrap();

        let found = query::get_select_option_u32_key(&sql::get_line_select_option(&tester.kphis),&tester.db_pool).await.unwrap();
        assert!(!found.is_empty());
    }

    #[tokio::test]
    #[ignore]
    async fn sqlx_get_cha_select_option() {
        let tester = MySqlTester::new_kphis().await;
        sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/create/kphis/ipd_vs_cha.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/insert/kphis/ipd_vs_cha.sql")).execute(&tester.db_pool).await.unwrap();

        let found = query::get_select_option_u32_key(&sql::get_cha_select_option(&tester.kphis),&tester.db_pool).await.unwrap();
        assert!(!found.is_empty());
    }

    #[tokio::test]
    #[ignore]
    async fn sqlx_get_va_select_option() {
        let tester = MySqlTester::new_kphis().await;
        sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/create/kphis/ipd_vs_va.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/insert/kphis/ipd_vs_va.sql")).execute(&tester.db_pool).await.unwrap();

        let found = query::get_select_option_u32_key(&sql::get_va_select_option(&tester.kphis),&tester.db_pool).await.unwrap();
        assert!(!found.is_empty());
    }

    #[tokio::test]
    #[ignore]
    async fn sqlx_get_mass_select_option() {
        let tester = MySqlTester::new_kphis().await;
        sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/create/kphis/ipd_vs_mass.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/insert/kphis/ipd_vs_mass.sql")).execute(&tester.db_pool).await.unwrap();

        let found = query::get_select_option_u32_key(&sql::get_mass_select_option(&tester.kphis),&tester.db_pool).await.unwrap();
        assert!(!found.is_empty());
    }

    #[tokio::test]
    #[ignore]
    async fn sqlx_get_lt_arm_select_option() {
        let tester = MySqlTester::new_kphis().await;
        sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/create/kphis/ipd_vs_lt_arm.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/insert/kphis/ipd_vs_lt_arm.sql")).execute(&tester.db_pool).await.unwrap();

        let found = query::get_select_option_u32_key(&sql::get_lt_arm_select_option(&tester.kphis),&tester.db_pool).await.unwrap();
        assert!(!found.is_empty());
    }

    #[tokio::test]
    #[ignore]
    async fn sqlx_get_o2_select_option() {
        let tester = MySqlTester::new_kphis().await;
        sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/create/kphis/ipd_vs_o2.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/insert/kphis/ipd_vs_o2.sql")).execute(&tester.db_pool).await.unwrap();

        let found = query::get_select_option_u32_key(&sql::get_o2_select_option(&tester.kphis),&tester.db_pool).await.unwrap();
        assert!(!found.is_empty());
    }

    #[tokio::test]
    #[ignore]
    async fn sqlx_get_tube_select_option() {
        let tester = MySqlTester::new_kphis().await;
        sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/create/kphis/ipd_vs_tube.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/insert/kphis/ipd_vs_tube.sql")).execute(&tester.db_pool).await.unwrap();

        let found = query::get_select_option_u32_key(&sql::get_tube_select_option(&tester.kphis),&tester.db_pool).await.unwrap();
        assert!(!found.is_empty());
    }

    #[tokio::test]
    #[ignore]
    async fn sqlx_get_intake_select_option() {
        let tester = MySqlTester::new_kphis().await;
        sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/create/kphis/ipd_vs_intake.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/insert/kphis/ipd_vs_intake.sql")).execute(&tester.db_pool).await.unwrap();

        let found = query::get_select_option_u32_key(&sql::get_intake_select_option(&tester.kphis),&tester.db_pool).await.unwrap();
        assert!(!found.is_empty());
    }

    #[tokio::test]
    #[ignore]
    async fn sqlx_get_output_select_option() {
        let tester = MySqlTester::new_kphis().await;
        sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/create/kphis/ipd_vs_output.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/insert/kphis/ipd_vs_output.sql")).execute(&tester.db_pool).await.unwrap();

        let found = query::get_select_option_u32_key(&sql::get_output_select_option(&tester.kphis),&tester.db_pool).await.unwrap();
        assert!(!found.is_empty());
    }

    #[tokio::test]
    #[ignore]
    async fn sqlx_get_lr_sta_select_option() {
        let tester = MySqlTester::new_kphis().await;
        sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/create/kphis/ipd_vs_lr_sta.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/insert/kphis/ipd_vs_lr_sta.sql")).execute(&tester.db_pool).await.unwrap();

        let found = query::get_select_option_u32_key(&sql::get_lr_sta_select_option(&tester.kphis),&tester.db_pool).await.unwrap();
        assert!(!found.is_empty());
    }

    #[tokio::test]
    #[ignore]
    async fn sqlx_get_lr_mem_select_option() {
        let tester = MySqlTester::new_kphis().await;
        sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/create/kphis/ipd_vs_lr_mem.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/insert/kphis/ipd_vs_lr_mem.sql")).execute(&tester.db_pool).await.unwrap();

        let found = query::get_select_option_u32_key(&sql::get_lr_mem_select_option(&tester.kphis),&tester.db_pool).await.unwrap();
        assert!(!found.is_empty());
    }

    #[tokio::test]
    #[ignore]
    async fn sqlx_get_lr_moulding_select_option() {
        let tester = MySqlTester::new_kphis().await;
        sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/create/kphis/ipd_vs_lr_moulding.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/insert/kphis/ipd_vs_lr_moulding.sql")).execute(&tester.db_pool).await.unwrap();

        let found = query::get_select_option_u32_key(&sql::get_lr_moulding_select_option(&tester.kphis),&tester.db_pool).await.unwrap();
        assert!(!found.is_empty());
    }

    #[tokio::test]
    #[ignore]
    async fn sqlx_get_dipstick_select_option() {
        let tester = MySqlTester::new_kphis().await;
        sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/create/kphis/ipd_vs_dipstick.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/insert/kphis/ipd_vs_dipstick.sql")).execute(&tester.db_pool).await.unwrap();

        let found = query::get_select_option_u32_key(&sql::get_dipstick_select_option(&tester.kphis),&tester.db_pool).await.unwrap();
        assert!(!found.is_empty());
    }

    #[tokio::test]
    #[ignore]
    async fn sqlx_get_breathing_select_option() {
        let tester = MySqlTester::new_kphis().await;
        sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/create/kphis/ipd_vs_breathing.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/insert/kphis/ipd_vs_breathing.sql")).execute(&tester.db_pool).await.unwrap();

        let found = query::get_select_option_u32_key(&sql::get_breathing_select_option(&tester.kphis),&tester.db_pool).await.unwrap();
        assert!(!found.is_empty());
    }

    #[tokio::test]
    #[ignore]
    async fn sqlx_get_avpu_select_option() {
        let tester = MySqlTester::new_kphis().await;
        sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/create/kphis/ipd_vs_avpu.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/insert/kphis/ipd_vs_avpu.sql")).execute(&tester.db_pool).await.unwrap();

        let found = query::get_select_option_u32_key(&sql::get_avpu_select_option(&tester.kphis),&tester.db_pool).await.unwrap();
        assert!(!found.is_empty());
    }

    #[tokio::test]
    #[ignore]
    async fn sqlx_get_gut_feeling_select_option() {
        let tester = MySqlTester::new_kphis().await;
        sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/create/kphis/ipd_vs_gut_feeling.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/insert/kphis/ipd_vs_gut_feeling.sql")).execute(&tester.db_pool).await.unwrap();

        let found = query::get_select_option_u32_key(&sql::get_gut_feeling_select_option(&tester.kphis),&tester.db_pool).await.unwrap();
        assert!(!found.is_empty());
    }

    #[tokio::test]
    #[ignore]
    async fn sqlx_get_pops_other_select_option() {
        let tester = MySqlTester::new_kphis().await;
        sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/create/kphis/ipd_vs_pops_other.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/insert/kphis/ipd_vs_pops_other.sql")).execute(&tester.db_pool).await.unwrap();

        let found = query::get_select_option_u32_key(&sql::get_pops_other_select_option(&tester.kphis),&tester.db_pool).await.unwrap();
        assert!(!found.is_empty());
    }

    #[tokio::test]
    #[ignore]
    async fn sqlx_get_stage_of_change_select_option() {
        let tester = MySqlTester::new_kphis().await;
        sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/create/kphis/ipd_vs_stage_of_change.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/insert/kphis/ipd_vs_stage_of_change.sql")).execute(&tester.db_pool).await.unwrap();

        let found = query::get_select_option_u32_key(&sql::get_stage_of_change_select_option(&tester.kphis),&tester.db_pool).await.unwrap();
        assert!(!found.is_empty());
    }

    #[tokio::test]
    #[ignore]
    async fn sqlx_get_moph_refer_expire_type_select_option() {
        let tester = MySqlTester::new_hosxp().await;
        sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/create/hosxp/moph_refer_expire_type.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../kphis-sqlx-tester/test_sqls/insert/hosxp/moph_refer_expire_type.sql")).execute(&tester.db_pool).await.unwrap();

        let found = query::get_select_option_i32_key(&sql::get_moph_refer_expire_type_select_option(&tester.hosxp),&tester.db_pool).await.unwrap();
        assert!(!found.is_empty());
    }
}
