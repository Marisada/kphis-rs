use sqlx::{AssertSqlSafe, FromRow, MySql, Pool};

use kphis_model::search::searchbox::{
    DrugCheckParams, DrugDuplicateCheck, DrugInteractionCheck, DrugUsage, HospSearchBox, IvfluidSearchbox, LabSearchbox, MedSearchbox, OpdVisitSearchType, OpdVisitSearchbox, PatientSearchbox,
    XraySearchbox,
};
use kphis_sql::search::searchbox;
use kphis_util::error::{AppError, Source};

use crate::{query1_all, query2_all, query4_all};

// common-searchbox-lab-data.php
pub async fn get_lab_searchbox(search_text: &str, pool: &Pool<MySql>, hosxp: &str) -> Result<Vec<LabSearchbox>, AppError> {
    let sql = searchbox::select_lab_searchbox(hosxp);
    let rows = query1_all(&["%", &search_text.trim().replace('%', "\\%"), "%"].concat(), &sql, pool, "Select LabSearchbox").await?;

    rows.iter()
        .map(LabSearchbox::from_row)
        .collect::<sqlx::Result<Vec<LabSearchbox>>>()
        .map_err(|e| Source::SQLx.to_error(500, e, "Select LabSearchbox"))
}

// common-searchbox-xray-data.php
pub async fn get_xray_searchbox(search_text: &str, pool: &Pool<MySql>, hosxp: &str) -> Result<Vec<XraySearchbox>, AppError> {
    let sql = searchbox::select_xray_searchbox(hosxp);
    let search = ["%", &search_text.trim().replace('%', "\\%"), "%"].concat();
    let rows = query2_all(&search, &search, &sql, pool, "Select XraySearchbox").await?;

    rows.iter()
        .map(XraySearchbox::from_row)
        .collect::<sqlx::Result<Vec<XraySearchbox>>>()
        .map_err(|e| Source::SQLx.to_error(500, e, "Select XraySearchbox"))
}

// common-searchbox-ivfluid-data.php
pub async fn get_ivfluid_searchbox(search_text: &str, ivfluid: &str, pool: &Pool<MySql>, hosxp: &str) -> Result<Vec<IvfluidSearchbox>, AppError> {
    let sql = searchbox::select_ivfluid_searchbox(ivfluid, hosxp);
    let search = ["%", &search_text.trim().replace('%', "\\%"), "%"].concat();
    let rows = query2_all(&search, &search, &sql, pool, "Select IvfluidSearchbox").await?;

    rows.iter()
        .map(IvfluidSearchbox::from_row)
        .collect::<sqlx::Result<Vec<IvfluidSearchbox>>>()
        .map_err(|e| Source::SQLx.to_error(500, e, "Select IvfluidSearchbox"))
}

// common-searchbox-med-data.php
pub async fn get_med_searchbox(hn: &str, search_text: &str, pool: &Pool<MySql>, hosxp: &str, kphis: &str) -> Result<Vec<MedSearchbox>, AppError> {
    let sql = searchbox::select_med_searchbox(hosxp, kphis);
    let search = ["%", &search_text.trim().replace('%', "\\%"), "%"].concat();
    let rows = query4_all(hn, hn, &search, &search, &sql, pool, "Select MedSearchbox").await?;

    rows.iter()
        .map(MedSearchbox::from_row)
        .collect::<sqlx::Result<Vec<MedSearchbox>>>()
        .map_err(|e| Source::SQLx.to_error(500, e, "Select MedSearchbox"))
}

/// WARNING! : This function NOT DETECTING DRUG ALLERGY!!!
pub async fn get_med_searchbox_without_hn(search_text: &str, pool: &Pool<MySql>, hosxp: &str, kphis: &str) -> Result<Vec<MedSearchbox>, AppError> {
    let sql = searchbox::select_med_searchbox_without_hn(hosxp, kphis);
    let search = ["%", &search_text.trim().replace('%', "\\%"), "%"].concat();
    let rows = query2_all(&search, &search, &sql, pool, "Select MedSearchboxNoHn").await?;

    rows.iter()
        .map(MedSearchbox::from_row)
        .collect::<sqlx::Result<Vec<MedSearchbox>>>()
        .map_err(|e| Source::SQLx.to_error(500, e, "Select MedSearchboxNoHn"))
}

// ipd-dr-order-item-drug-duplication-check.php
pub async fn get_drug_duplicate_check(params: &DrugCheckParams, pool: &Pool<MySql>, hosxp: &str, kphis: &str) -> Result<Vec<DrugDuplicateCheck>, AppError> {
    let sql = searchbox::select_drug_duplication_check(params, hosxp, kphis);
    let mut query = sqlx::query(AssertSqlSafe(sql));
    if let Some(an) = params.an.as_ref() {
        query = query.bind(an);
    } else {
        return Ok(Vec::new());
    }
    if let Some(exclude_order_id) = params.exclude_order_id {
        query = query.bind(exclude_order_id);
    }
    if let Some(generic_name) = params.generic_name.as_ref() {
        query = query.bind(generic_name);
    } else {
        return Ok(Vec::new());
    }

    query
        .fetch_all(pool)
        .await
        .map(|rows| {
            rows.iter()
                .map(DrugDuplicateCheck::from_row)
                .collect::<sqlx::Result<Vec<DrugDuplicateCheck>>>()
                .map_err(|e| Source::SQLx.to_error(500, e, "Select DrugDuplicate"))
        })
        .map_err(|e| Source::SQLx.to_error(500, e, "Select DrugDuplicate"))?
}

// ipd-dr-order-item-drug-interaction-check.php
pub async fn get_drug_interaction_check(params: &DrugCheckParams, pool: &Pool<MySql>, hosxp: &str, kphis: &str) -> Result<Vec<DrugInteractionCheck>, AppError> {
    let sql = searchbox::select_drug_interaction_check(params, hosxp, kphis);
    let mut query = sqlx::query(AssertSqlSafe(sql));
    if let Some(generic_name) = &params.generic_name {
        query = query.bind(generic_name);
    } else {
        return Ok(Vec::new());
    }
    if let Some(an) = &params.an {
        query = query.bind(an);
    } else {
        return Ok(Vec::new());
    }
    if let Some(exclude_order_id) = params.exclude_order_id {
        query = query.bind(exclude_order_id);
    }
    if let Some(generic_name) = &params.generic_name {
        query = query.bind(generic_name);
    }
    if let Some(an) = &params.an {
        query = query.bind(an);
    }
    if let Some(exclude_order_id) = params.exclude_order_id {
        query = query.bind(exclude_order_id);
    }

    query
        .fetch_all(pool)
        .await
        .map(|rows| {
            rows.iter()
                .map(DrugInteractionCheck::from_row)
                .collect::<sqlx::Result<Vec<DrugInteractionCheck>>>()
                .map_err(|e| Source::SQLx.to_error(500, e, "Select DrugInteraction"))
        })
        .map_err(|e| Source::SQLx.to_error(500, e, "Select DrugInteraction"))?
}

// common-searchbox-patient-data.php
pub async fn get_patient_searchbox(search_text: &str, hosxp_hn_len: usize, pool: &Pool<MySql>, hosxp: &str) -> Result<Vec<PatientSearchbox>, AppError> {
    let text = search_text.trim();
    let sql = searchbox::select_patient_searchbox(&text, hosxp_hn_len, hosxp);
    let rows = query1_all(&["%", &text, "%"].concat(), &sql, pool, "Select PatientSearchbox").await?;

    rows.iter()
        .map(PatientSearchbox::from_row)
        .collect::<sqlx::Result<Vec<PatientSearchbox>>>()
        .map_err(|e| Source::SQLx.to_error(500, e, "Select PatientSearchbox"))
}

// common-searchbox-opd_visit-data.php
pub async fn get_opd_visit_searchbox(mode_text: &str, search_text: &str, pool: &Pool<MySql>, hosxp: &str) -> Result<Vec<OpdVisitSearchbox>, AppError> {
    let mode = OpdVisitSearchType::new(mode_text);
    let text = mode.wildcard(search_text);
    let sql = searchbox::select_opd_visit_searchbox(&mode, hosxp);
    let mut query = sqlx::query(AssertSqlSafe(sql));
    if mode.not_all() {
        query = query.bind(text);
    }
    let rows = query.fetch_all(pool).await.map_err(|e| Source::SQLx.to_error(500, e, "Select OpdVisitSearchbox"))?;

    rows.iter()
        .map(OpdVisitSearchbox::from_row)
        .collect::<sqlx::Result<Vec<OpdVisitSearchbox>>>()
        .map_err(|e| Source::SQLx.to_error(500, e, "Select OpdVisitSearchbox"))
}

// // ipd-summary-2-dx-data.php
// pub async fn get_icd10_searchbox(is_external_cause: bool, pool: &Pool<MySql>, hosxp: &str, kphis: &str) -> Result<Vec<Icd10Keywords>, AppError> {
//     let sql = searchbox::select_icd10_searchbox(is_external_cause, hosxp, kphis);
//     let results = sqlx::query(AssertSqlSafe(sql))
//         .fetch_all(pool)
//         .await
//         .map_err(|e| Source::SQLx.to_error(500, e, "Select Icd10Keywords"))?
//         .iter()
//         .map(Icd10Keywords::from_row)
//         .collect::<sqlx::Result<Vec<Icd10Keywords>>>()
//         .map_err(|e| Source::SQLx.to_error(500, e, "Select Icd10Keywords"))?;

//     Ok(results)
// }

// ipd-summary-2-hospcode-data.php
pub async fn get_hosp_searchbox(search_text: &str, pool: &Pool<MySql>, hosxp: &str) -> Result<Vec<HospSearchBox>, AppError> {
    let text = ["%", &search_text.trim().replace('%', "\\%"), "%"].concat();
    let sql = searchbox::select_hosp_searchbox(hosxp);
    let results = sqlx::query(AssertSqlSafe(sql))
        .bind(&text)
        .bind(&text)
        .bind(&text)
        .bind(100)
        .bind(0)
        .fetch_all(pool)
        .await
        .map_err(|e| Source::SQLx.to_error(500, e, "Select HospSearchbox"))?
        .iter()
        .map(HospSearchBox::from_row)
        .collect::<sqlx::Result<Vec<HospSearchBox>>>()
        .map_err(|e| Source::SQLx.to_error(500, e, "Select HospSearchbox"))?;

    Ok(results)
}

pub async fn get_drugusage_searchbox(pool: &Pool<MySql>, hosxp: &str) -> Result<Vec<DrugUsage>, AppError> {
    let sql = searchbox::select_drugusage_searchbox(hosxp);
    let results = sqlx::query(AssertSqlSafe(sql))
        .fetch_all(pool)
        .await
        .map_err(|e| Source::SQLx.to_error(500, e, "Select DrugUsageSearchbox"))?
        .iter()
        .map(DrugUsage::from_row)
        .collect::<sqlx::Result<Vec<DrugUsage>>>()
        .map_err(|e| Source::SQLx.to_error(500, e, "Select DrugUsageSearchbox"))?;

    Ok(results)
}

#[cfg(test)]
#[rustfmt::skip]
mod tests {

    use super::*;
    use kphis_sqlx_tester::MySqlTester;
    use kphis_util::util::str_some;

    #[tokio::test]
    #[ignore]
    async fn sqlx_get_lab_searchbox() {
        let tester = MySqlTester::new_hosxp().await;
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/hosxp/lab_form.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/hosxp/lab_form_head.sql")).execute(&tester.db_pool).await.unwrap();

        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/hosxp/lab_form.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/hosxp/lab_form_head.sql")).execute(&tester.db_pool).await.unwrap();

        let found = get_lab_searchbox("tinin", &tester.db_pool, &tester.hosxp).await.unwrap();
        assert_eq!(found.len(), 1);
        let not_found = get_lab_searchbox("xxxx", &tester.db_pool, &tester.hosxp).await.unwrap();
        assert!(not_found.is_empty());
    }

    #[tokio::test]
    #[ignore]
    async fn sqlx_get_xray_searchbox() {
        let tester = MySqlTester::new_hosxp().await;
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/hosxp/xray_items.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/hosxp/xray_items_group.sql")).execute(&tester.db_pool).await.unwrap();

        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/hosxp/xray_items.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/hosxp/xray_items_group.sql")).execute(&tester.db_pool).await.unwrap();

        let found_item = get_xray_searchbox("PA", &tester.db_pool, &tester.hosxp).await.unwrap();
        assert_eq!(found_item.len(), 1);
        let found_group = get_xray_searchbox("Ra", &tester.db_pool, &tester.hosxp).await.unwrap();
        assert_eq!(found_group.len(), 1);
        let not_found = get_xray_searchbox("xxxx", &tester.db_pool, &tester.hosxp).await.unwrap();
        assert!(not_found.is_empty());
    }

    #[tokio::test]
    #[ignore]
    async fn sqlx_get_ivfluid_searchbox() {
        let tester = MySqlTester::new_hosxp().await;
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/hosxp/drugitems.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/hosxp/drugitems.sql")).execute(&tester.db_pool).await.unwrap();

        let found_item = get_ivfluid_searchbox("NS", "INTRAVENOUS SOLUTION", &tester.db_pool, &tester.hosxp).await.unwrap();
        assert_eq!(found_item.len(), 1);
        let found_drugnote = get_ivfluid_searchbox("0.9","INTRAVENOUS SOLUTION",&tester.db_pool,&tester.hosxp).await.unwrap();
        assert_eq!(found_drugnote.len(), 1);
        let not_found_iv = get_ivfluid_searchbox("NS", "IV SOLUTION", &tester.db_pool, &tester.hosxp).await.unwrap();
        assert!(not_found_iv.is_empty());
        let not_found = get_ivfluid_searchbox("xxxx","INTRAVENOUS SOLUTION",&tester.db_pool,&tester.hosxp).await.unwrap();
        assert!(not_found.is_empty());
    }

    #[tokio::test]
    #[ignore]
    async fn sqlx_get_med_searchbox() {
        let tester = MySqlTester::new_hosxp_and_kphis().await;
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/hosxp/drugitems.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/hosxp/drugusage.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/hosxp/opd_allergy.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis/kphis_drug_use_duration.sql")).execute(&tester.db_pool).await.unwrap();

        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/hosxp/drugitems.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/hosxp/drugusage.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/hosxp/opd_allergy.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/kphis/kphis_drug_use_duration.sql")).execute(&tester.db_pool).await.unwrap();


        let found_item = get_med_searchbox("0001234", "RACE", &tester.db_pool, &tester.hosxp, &tester.kphis).await.unwrap();
        assert_eq!(found_item.len(), 1);
        assert!(found_item.into_iter().all(|m| m.allergy_agent.and_then(str_some).is_some()));
        let found_note = get_med_searchbox("0001234", "para", &tester.db_pool, &tester.hosxp, &tester.kphis).await.unwrap();
        assert_eq!(found_note.len(), 1);
        assert!(found_note.into_iter().all(|m| m.allergy_agent.and_then(str_some).is_some()));
        let found_no_allergy = get_med_searchbox("0001234", "FAR", &tester.db_pool, &tester.hosxp, &tester.kphis).await.unwrap();
        assert_eq!(found_no_allergy.len(), 1);
        assert!(found_no_allergy.into_iter().all(|m| m.allergy_agent.and_then(str_some).is_none()));
        let not_found = get_med_searchbox("0001234", "xxxx", &tester.db_pool, &tester.hosxp, &tester.kphis).await.unwrap();
        assert!(not_found.is_empty());
    }

    #[tokio::test]
    #[ignore]
    async fn sqlx_get_med_searchbox_without_hn() {
        let tester = MySqlTester::new_hosxp_and_kphis().await;
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/hosxp/drugitems.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/hosxp/drugusage.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis/kphis_drug_use_duration.sql")).execute(&tester.db_pool).await.unwrap();

        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/hosxp/drugitems.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/hosxp/drugusage.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/kphis/kphis_drug_use_duration.sql")).execute(&tester.db_pool).await.unwrap();

        let found_item = get_med_searchbox_without_hn("RACE", &tester.db_pool, &tester.hosxp, &tester.kphis).await.unwrap();
        assert_eq!(found_item.len(), 1);
        // this function NOT DETECTING DRUG ALLERGY!!!
        assert!(found_item.into_iter().all(|m| m.allergy_agent.and_then(str_some).is_none()));
        let found_note = get_med_searchbox_without_hn("para", &tester.db_pool, &tester.hosxp, &tester.kphis).await.unwrap();
        assert_eq!(found_note.len(), 1);
        // this function NOT DETECTING DRUG ALLERGY!!!
        assert!(found_note.into_iter().all(|m| m.allergy_agent.and_then(str_some).is_none()));
        let found_no_allergy = get_med_searchbox_without_hn("FAR", &tester.db_pool, &tester.hosxp, &tester.kphis).await.unwrap();
        assert_eq!(found_no_allergy.len(), 1);
        assert!(found_no_allergy.into_iter().all(|m| m.allergy_agent.and_then(str_some).is_none()));
        let not_found = get_med_searchbox_without_hn("xxxx", &tester.db_pool, &tester.hosxp, &tester.kphis).await.unwrap();
        assert!(not_found.is_empty());
    }

    #[tokio::test]
    #[ignore]
    async fn sqlx_get_drug_duplicate_check() {
        let tester = MySqlTester::new_hosxp_and_kphis().await;
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/hosxp/drugitems.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis/ipd_order.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis/ipd_order_item.sql")).execute(&tester.db_pool).await.unwrap();

        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/hosxp/drugitems.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/kphis/ipd_order.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/kphis/ipd_order_item.sql")).execute(&tester.db_pool).await.unwrap();

        let params = DrugCheckParams {an: Some(String::from("660001234")),generic_name: Some(String::from("WARFARIN")),..Default::default()};
        let found = get_drug_duplicate_check(&params, &tester.db_pool, &tester.hosxp, &tester.kphis).await.unwrap();
        assert_eq!(found.len(), 1);
        let default = get_drug_duplicate_check(&DrugCheckParams::default(),&tester.db_pool,&tester.hosxp,&tester.kphis).await.unwrap();
        assert!(default.is_empty());

        let mut params_ex_order_id = params.clone();
        params_ex_order_id.exclude_order_id = Some(2);
        let ex_order_id = get_drug_duplicate_check(&params_ex_order_id,&tester.db_pool,&tester.hosxp,&tester.kphis).await.unwrap();
        assert!(ex_order_id.is_empty());

        let mut params_ex_off_order_item_ids = params.clone();
        params_ex_off_order_item_ids.off_order_item_ids = Some(String::from("2,9"));
        let ex_off_order_item_ids = get_drug_duplicate_check(&params_ex_off_order_item_ids,&tester.db_pool,&tester.hosxp,&tester.kphis).await.unwrap();
        assert!(ex_off_order_item_ids.is_empty());
    }

    #[tokio::test]
    #[ignore]
    async fn sqlx_get_drug_interaction_check() {
        let tester = MySqlTester::new_hosxp_and_kphis().await;
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/hosxp/drugitems.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/hosxp/drug_interaction.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis/ipd_order.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis/ipd_order_item.sql")).execute(&tester.db_pool).await.unwrap();

        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/hosxp/drugitems.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/hosxp/drug_interaction.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/kphis/ipd_order.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/kphis/ipd_order_item.sql")).execute(&tester.db_pool).await.unwrap();

        let params = DrugCheckParams {an: Some(String::from("660001234")),generic_name: Some(String::from("PARACETAMOL")),..Default::default()};
        let found = get_drug_interaction_check(&params, &tester.db_pool, &tester.hosxp, &tester.kphis).await.unwrap();
        assert_eq!(found.len(), 1);
        let default = get_drug_interaction_check(&DrugCheckParams::default(),&tester.db_pool,&tester.hosxp,&tester.kphis).await.unwrap();
        assert!(default.is_empty());

        let mut params_ex_order_id = params.clone();
        params_ex_order_id.exclude_order_id = Some(2);
        let ex_order_id = get_drug_interaction_check(&params_ex_order_id,&tester.db_pool,&tester.hosxp,&tester.kphis).await.unwrap();
        assert!(ex_order_id.is_empty());

        let mut params_ex_off_order_item_ids = params.clone();
        params_ex_off_order_item_ids.off_order_item_ids = Some(String::from("2,9"));
        let ex_off_order_item_ids = get_drug_interaction_check(&params_ex_off_order_item_ids,&tester.db_pool,&tester.hosxp,&tester.kphis).await.unwrap();
        assert!(ex_off_order_item_ids.is_empty());
    }

    #[tokio::test]
    #[ignore]
    async fn sqlx_get_patient_searchbox() {
        let tester = MySqlTester::new_hosxp().await;
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/hosxp/patient.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/hosxp/patient.sql")).execute(&tester.db_pool).await.unwrap();

        let found_hn = get_patient_searchbox("123", 7, &tester.db_pool, &tester.hosxp).await.unwrap();
        assert_eq!(found_hn.len(), 1);
        let found_cid = get_patient_searchbox("111111111", 7, &tester.db_pool, &tester.hosxp).await.unwrap();
        assert_eq!(found_cid.len(), 1);
        let found_fullname = get_patient_searchbox("มุติ", 7, &tester.db_pool, &tester.hosxp).await.unwrap();
        assert_eq!(found_fullname.len(), 1);
        let not_found = get_patient_searchbox("9999999", 7, &tester.db_pool, &tester.hosxp).await.unwrap();
        assert!(not_found.is_empty());
    }

    #[tokio::test]
    #[ignore]
    async fn sqlx_get_opd_visit_searchbox() {
        let tester = MySqlTester::new_hosxp().await;
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/hosxp/patient.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/hosxp/ovst.sql")).execute(&tester.db_pool).await.unwrap();

        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/hosxp/patient.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/hosxp/ovst.sql")).execute(&tester.db_pool).await.unwrap();

        // mode_text not match will be 'all'
        let default = get_opd_visit_searchbox("", "", &tester.db_pool, &tester.hosxp).await.unwrap();
        assert_eq!(default.len(), 4);
        let found_all = get_opd_visit_searchbox("all", "", &tester.db_pool, &tester.hosxp).await.unwrap();
        assert_eq!(found_all.len(), 4);
        // %search_text
        let found_hn = get_opd_visit_searchbox("hn", "1234", &tester.db_pool, &tester.hosxp).await.unwrap();
        assert_eq!(found_hn.len(), 4);
        // %search_text
        let found_vn = get_opd_visit_searchbox("vn", "1235959", &tester.db_pool, &tester.hosxp).await.unwrap();
        assert_eq!(found_vn.len(), 1);
        let found_qn = get_opd_visit_searchbox("qn", "1", &tester.db_pool, &tester.hosxp).await.unwrap();
        assert_eq!(found_qn.len(), 1);
        // %search_text%
        let found_pt_name = get_opd_visit_searchbox("pt-name", "มุติ", &tester.db_pool, &tester.hosxp).await.unwrap();
        assert_eq!(found_pt_name.len(), 4);
        // %search_text%
        let found_cid = get_opd_visit_searchbox("cid", "111111111", &tester.db_pool, &tester.hosxp).await.unwrap();
        assert_eq!(found_cid.len(), 4);
    }

    // #[tokio::test]
    // #[ignore]
    // async fn sqlx_et_icd10_searchbox() {
    //     let tester = MySqlTester::new_hosxp_and_kphis().await;
    //     sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/hosxp/icd101.sql")).execute(&tester.db_pool).await.unwrap();
    //     sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/hosxp/icd_codemap.sql")).execute(&tester.db_pool).await.unwrap();
    //     sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/kphis/ipd_summary_dx.sql")).execute(&tester.db_pool).await.unwrap();

    //     sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/hosxp/icd101.sql")).execute(&tester.db_pool).await.unwrap();
    //     sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/hosxp/icd_codemap.sql")).execute(&tester.db_pool).await.unwrap();
    //     sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/kphis/ipd_summary_dx.sql")).execute(&tester.db_pool).await.unwrap();

    //     let not_ext = get_icd10_searchbox(false, &tester.db_pool, &tester.hosxp, &tester.kphis).await.unwrap();
    //     assert_eq!(not_ext.len(), 2);
    //     let ext = get_icd10_searchbox(true, &tester.db_pool, &tester.hosxp, &tester.kphis).await.unwrap();
    //     assert_eq!(ext.len(), 1);
    // }

    #[tokio::test]
    #[ignore]
    async fn sqlx_get_hosp_searchbox() {
        let tester = MySqlTester::new_hosxp().await;
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/hosxp/hospcode.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/hosxp/thaiaddress.sql")).execute(&tester.db_pool).await.unwrap();

        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/hosxp/hospcode.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/hosxp/thaiaddress.sql")).execute(&tester.db_pool).await.unwrap();

        let found_name = get_hosp_searchbox("ทราบ", &tester.db_pool, &tester.hosxp).await.unwrap();
        assert_eq!(found_name.len(), 1);
        let found_part = get_hosp_searchbox("000", &tester.db_pool, &tester.hosxp).await.unwrap();
        assert_eq!(found_part.len(), 1);
        let found_code = get_hosp_searchbox("999", &tester.db_pool, &tester.hosxp).await.unwrap();
        assert_eq!(found_code.len(), 1);
        let not_found = get_hosp_searchbox("xxxx", &tester.db_pool, &tester.hosxp).await.unwrap();
        assert!(not_found.is_empty());
    }

    #[tokio::test]
    #[ignore]
    async fn sqlx_get_drugusage_searchbox() {
        let tester = MySqlTester::new_hosxp().await;
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/create/hosxp/drugusage.sql")).execute(&tester.db_pool).await.unwrap();
        sqlx::query(include_str!("../../../kphis-sqlx-tester/test_sqls/insert/hosxp/drugusage.sql")).execute(&tester.db_pool).await.unwrap();

        let found = get_drugusage_searchbox(&tester.db_pool, &tester.hosxp).await.unwrap();
        assert_eq!(found.len(), 2);
    }
}
