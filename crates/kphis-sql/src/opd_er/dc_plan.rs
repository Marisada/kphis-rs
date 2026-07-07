use kphis_model::dc_plan::{DischargePlanMedItemOnly, DischargePlanEnvItemOnly, DischargePlanTxItemOnly, DischargePlanDietItemOnly};

use crate::TABLE_CREATE_COLUMNS;

// SELECT dp.*,tdx.dx_name,tdx.dx_knowledge,tdx.dx_revisit,tdx.dx_prevention,
// 	d1.`name` AS dx_user_name,d2.`name` AS med_user_name,d3.`name` AS env_user_name,d4.`name` AS tx_user_name,
//  d5.`name` AS health_user_name,d6.`name` AS out_user_name,d7.`name` AS diet_user_name,
// 	u1.entryposition AS dx_entryposition,u2.entryposition AS med_entryposition,u3.entryposition AS env_entryposition,u4.entryposition AS tx_entryposition,
// 	u5.entryposition AS health_entryposition,u6.entryposition AS out_entryposition,u7.entryposition AS diet_entryposition,
// 	(SELECT GROUP_CONCAT(CONCAT(tm.med_id,'^',tm.med_text) ORDER BY mi.med_id ASC SEPARATOR '|')
//        FROM kphis_extra.opd_er_dc_plan_med_item mi LEFT JOIN kphis_extra.ipd_dc_plan_tmp_med tm ON tm.med_id=mi.med_id
//        WHERE mi.dc_plan_id=dp.dc_plan_id) AS meds,
// 	(SELECT GROUP_CONCAT(CONCAT(te.env_id,'^',te.env_text) ORDER BY ei.env_id ASC SEPARATOR '|')
//        FROM kphis_extra.opd_er_dc_plan_env_item ei LEFT JOIN kphis_extra.ipd_dc_plan_tmp_env te ON te.env_id=ei.env_id
//        WHERE ei.dc_plan_id=dp.dc_plan_id) AS envs,
// 	(SELECT GROUP_CONCAT(CONCAT(tt.tx_id,'^',tt.tx_text) ORDER BY ti.tx_id ASC SEPARATOR '|')
//        FROM kphis_extra.opd_er_dc_plan_tx_item ti LEFT JOIN kphis_extra.ipd_dc_plan_tmp_tx tt ON tt.tx_id=ti.tx_id
//        WHERE ti.dc_plan_id=dp.dc_plan_id) AS txs,
//    (SELECT GROUP_CONCAT(CONCAT(td.diet_id,'^',td.diet_text) ORDER BY di.diet_id ASC SEPARATOR '|')
//        FROM kphis_extra.opd_er_dc_plan_diet_item di LEFT JOIN kphis_extra.ipd_dc_plan_tmp_diet td ON td.diet_id=di.diet_id
//        WHERE di.dc_plan_id=dp.dc_plan_id) AS diets
// FROM kphis_extra.opd_er_dc_plan dp
// 	LEFT JOIN kphis_extra.ipd_dc_plan_tmp_dx tdx ON tdx.dx_id=dp.dx_id
// 	LEFT JOIN hos.doctor d1 ON d1.code=dp.dx_doctor
// 	LEFT JOIN hos.doctor d2 ON d2.code=dp.med_doctor
// 	LEFT JOIN hos.doctor d3 ON d3.code=dp.env_doctor
// 	LEFT JOIN hos.doctor d4 ON d4.code=dp.tx_doctor
// 	LEFT JOIN hos.doctor d5 ON d5.code=dp.health_doctor
// 	LEFT JOIN hos.doctor d6 ON d6.code=dp.out_doctor
// 	LEFT JOIN hos.doctor d7 ON d7.code=dp.diet_doctor
// 	LEFT JOIN hos.opduser u1 ON u1.doctorcode=dp.dx_doctor AND (u1.account_disable IS NULL OR u1.account_disable='N')
// 	LEFT JOIN hos.opduser u2 ON u2.doctorcode=dp.med_doctor AND (u2.account_disable IS NULL OR u2.account_disable='N')
// 	LEFT JOIN hos.opduser u3 ON u3.doctorcode=dp.env_doctor AND (u3.account_disable IS NULL OR u3.account_disable='N')
// 	LEFT JOIN hos.opduser u4 ON u4.doctorcode=dp.tx_doctor AND (u4.account_disable IS NULL OR u4.account_disable='N')
// 	LEFT JOIN hos.opduser u5 ON u5.doctorcode=dp.health_doctor AND (u5.account_disable IS NULL OR u5.account_disable='N')
// 	LEFT JOIN hos.opduser u6 ON u6.doctorcode=dp.out_doctor AND (u6.account_disable IS NULL OR u6.account_disable='N')
// 	LEFT JOIN hos.opduser u7 ON u7.doctorcode=dp.diet_doctor AND (u7.account_disable IS NULL OR u7.account_disable='N')
// WHERE dp.opd_er_order_master_id=?;
/// opd_er_order_master_id
pub fn select_dc_plan(hosxp: &str, kphis_extra: &str) -> String {
    [
        "SELECT dp.*,tdx.dx_name,tdx.dx_knowledge,tdx.dx_revisit,tdx.dx_prevention,\
            d1.`name` AS dx_user_name,d2.`name` AS med_user_name,d3.`name` AS env_user_name,d4.`name` AS tx_user_name,\
            d5.`name` AS health_user_name,d6.`name` AS out_user_name,d7.`name` AS diet_user_name,\
            d1.licenseno AS dx_licenseno,d2.licenseno AS med_licenseno,d3.licenseno AS env_licenseno,d4.licenseno AS tx_licenseno,\
            d5.licenseno AS health_licenseno,d6.licenseno AS out_licenseno,d7.licenseno AS diet_licenseno,\
            (SELECT entryposition FROM ",hosxp,".opduser WHERE doctorcode=dp.dx_doctor AND (account_disable IS NULL OR account_disable='N') LIMIT 1) AS dx_entryposition,\
            (SELECT entryposition FROM ",hosxp,".opduser WHERE doctorcode=dp.med_doctor AND (account_disable IS NULL OR account_disable='N') LIMIT 1) AS med_entryposition,\
            (SELECT entryposition FROM ",hosxp,".opduser WHERE doctorcode=dp.env_doctor AND (account_disable IS NULL OR account_disable='N') LIMIT 1) AS env_entryposition,\
            (SELECT entryposition FROM ",hosxp,".opduser WHERE doctorcode=dp.tx_doctor AND (account_disable IS NULL OR account_disable='N') LIMIT 1) AS tx_entryposition,\
            (SELECT entryposition FROM ",hosxp,".opduser WHERE doctorcode=dp.health_doctor AND (account_disable IS NULL OR account_disable='N') LIMIT 1) AS health_entryposition,\
            (SELECT entryposition FROM ",hosxp,".opduser WHERE doctorcode=dp.out_doctor AND (account_disable IS NULL OR account_disable='N') LIMIT 1) AS out_entryposition,\
            (SELECT entryposition FROM ",hosxp,".opduser WHERE doctorcode=dp.diet_doctor AND (account_disable IS NULL OR account_disable='N') LIMIT 1) AS diet_entryposition,\
            (SELECT GROUP_CONCAT(CONCAT(tm.med_id,'^',tm.med_text) ORDER BY mi.med_id ASC SEPARATOR '|')\
            FROM ",kphis_extra,".opd_er_dc_plan_med_item mi LEFT JOIN ",kphis_extra,".ipd_dc_plan_tmp_med tm ON tm.med_id=mi.med_id \
            WHERE mi.dc_plan_id=dp.dc_plan_id) AS meds,\
            (SELECT GROUP_CONCAT(CONCAT(te.env_id,'^',te.env_text) ORDER BY ei.env_id ASC SEPARATOR '|') \
            FROM ",kphis_extra,".opd_er_dc_plan_env_item ei LEFT JOIN ",kphis_extra,".ipd_dc_plan_tmp_env te ON te.env_id=ei.env_id \
            WHERE ei.dc_plan_id=dp.dc_plan_id) AS envs,\
            (SELECT GROUP_CONCAT(CONCAT(tt.tx_id,'^',tt.tx_text) ORDER BY ti.tx_id ASC SEPARATOR '|') \
            FROM ",kphis_extra,".opd_er_dc_plan_tx_item ti LEFT JOIN ",kphis_extra,".ipd_dc_plan_tmp_tx tt ON tt.tx_id=ti.tx_id \
            WHERE ti.dc_plan_id=dp.dc_plan_id) AS txs,\
        (SELECT GROUP_CONCAT(CONCAT(td.diet_id,'^',td.diet_text) ORDER BY di.diet_id ASC SEPARATOR '|') \
            FROM ",kphis_extra,".opd_er_dc_plan_diet_item di LEFT JOIN ",kphis_extra,".ipd_dc_plan_tmp_diet td ON td.diet_id=di.diet_id \
            WHERE di.dc_plan_id=dp.dc_plan_id) AS diets \
        FROM ",kphis_extra,".opd_er_dc_plan dp \
            LEFT JOIN ",kphis_extra,".ipd_dc_plan_tmp_dx tdx ON tdx.dx_id=dp.dx_id \
            LEFT JOIN ",hosxp,".doctor d1 ON d1.code=dp.dx_doctor \
            LEFT JOIN ",hosxp,".doctor d2 ON d2.code=dp.med_doctor \
            LEFT JOIN ",hosxp,".doctor d3 ON d3.code=dp.env_doctor \
            LEFT JOIN ",hosxp,".doctor d4 ON d4.code=dp.tx_doctor \
            LEFT JOIN ",hosxp,".doctor d5 ON d5.code=dp.health_doctor \
            LEFT JOIN ",hosxp,".doctor d6 ON d6.code=dp.out_doctor \
            LEFT JOIN ",hosxp,".doctor d7 ON d7.code=dp.diet_doctor \
        WHERE dp.opd_er_order_master_id=?;"
    ].concat()
}

/// opd_er_order_master_id
pub fn select_dc_plan_only(kphis_extra: &str) -> String {
    [
        "SELECT * FROM ",kphis_extra,".opd_er_dc_plan WHERE opd_er_order_master_id=? ORDER BY dc_plan_id ASC;"
    ].concat()
}

/// dc_plan_id
pub fn select_dc_plan_med_item_only(kphis_extra: &str) -> String {
    [
        "SELECT * FROM ",kphis_extra,".opd_er_dc_plan_med_item WHERE dc_plan_id=? ORDER BY med_item_id ASC;"
    ].concat()
}

/// dc_plan_id
pub fn select_dc_plan_env_item_only(kphis_extra: &str) -> String {
    [
        "SELECT * FROM ",kphis_extra,".opd_er_dc_plan_env_item WHERE dc_plan_id=? ORDER BY env_item_id ASC;"
    ].concat()
}

/// dc_plan_id
pub fn select_dc_plan_tx_item_only(kphis_extra: &str) -> String {
    [
        "SELECT * FROM ",kphis_extra,".opd_er_dc_plan_tx_item WHERE dc_plan_id=? ORDER BY tx_item_id ASC;"
    ].concat()
}

/// dc_plan_id
pub fn select_dc_plan_diet_item_only(kphis_extra: &str) -> String {
    [
        "SELECT * FROM ",kphis_extra,".opd_er_dc_plan_diet_item WHERE dc_plan_id=? ORDER BY diet_item_id ASC;"
    ].concat()
}

// INSERT INTO kphis_extra.opd_er_dc_plan_med_item (dc_plan_id,med_id,create_user,create_datetime,update_user,update_datetime,version)
//     VALUES (:dc_plan_id,:med_id,:create_user,NOW(),:update_user,NOW(),:version)
pub fn insert_med_items(
    meds: &[u32],
    dc_plan_id: u32,
    user: &str,
    version: i32,
    kphis_extra: &str,
) -> String {
    let med = meds.iter().map(|id| ["(",&dc_plan_id.to_string(),",",&id.to_string(),",'",user,"',NOW(),'",user,"',NOW(),",&version.to_string(),")"].concat()).collect::<Vec<String>>().join(",");
    [
        "INSERT INTO ",kphis_extra,".opd_er_dc_plan_med_item (dc_plan_id,med_id",TABLE_CREATE_COLUMNS,") VALUES ",&med
    ].concat()
}

pub fn insert_med_items_only(
    meds: &[DischargePlanMedItemOnly],
    dc_plan_id: u32,
    kphis_extra: &str,
) -> String {
    let values = meds.iter().map(|item| {
        [
            "(",&dc_plan_id.to_string(),",",
            &item.med_id.map(|s| s.to_string()).unwrap_or(String::from("NULL")),",'",
            &item.create_user,"','",
            &item.create_datetime.to_string(),"','",
            &item.update_user,"','",
            &item.update_datetime.to_string(),"',",
            &item.version.to_string(),")"
        ].concat()
    }).collect::<Vec<String>>().join(",");
    [
        "INSERT INTO ",kphis_extra,".opd_er_dc_plan_med_item (dc_plan_id,med_id",TABLE_CREATE_COLUMNS,") \
        VALUES ",&values
    ].concat()
}

// INSERT INTO kphis_extra.opd_er_dc_plan_env_item (dc_plan_id,env_id,create_user,create_datetime,update_user,update_datetime,version)
//     VALUES (:dc_plan_id,:env_id,:create_user,NOW(),:update_user,NOW(),:version)
pub fn insert_env_items(
    envs: &[u32],
    dc_plan_id: u32,
    user: &str,
    version: i32,
    kphis_extra: &str,
) -> String {
    let env = envs.iter().map(|id| ["(",&dc_plan_id.to_string(),",",&id.to_string(),",'",user,"',NOW(),'",user,"',NOW(),",&version.to_string(),")"].concat()).collect::<Vec<String>>().join(",");
    [
        "INSERT INTO ",kphis_extra,".opd_er_dc_plan_env_item (dc_plan_id,env_id",TABLE_CREATE_COLUMNS,") VALUES ",&env
    ].concat()
}

pub fn insert_env_items_only(
    envs: &[DischargePlanEnvItemOnly],
    dc_plan_id: u32,
    kphis_extra: &str,
) -> String {
    let values = envs.iter().map(|item| {
        [
            "(",&dc_plan_id.to_string(),",",
            &item.env_id.map(|s| s.to_string()).unwrap_or(String::from("NULL")),",'",
            &item.create_user,"','",
            &item.create_datetime.to_string(),"','",
            &item.update_user,"','",
            &item.update_datetime.to_string(),"',",
            &item.version.to_string(),")"
        ].concat()
    }).collect::<Vec<String>>().join(",");
    [
        "INSERT INTO ",kphis_extra,".opd_er_dc_plan_env_item (dc_plan_id,env_id",TABLE_CREATE_COLUMNS,") \
        VALUES ",&values
    ].concat()
}

// INSERT INTO kphis_extra.opd_er_dc_plan_tx_item (dc_plan_id,tx_id,create_user,create_datetime,update_user,update_datetime,version)
//     VALUES (:dc_plan_id,:tx_id,:create_user,NOW(),:update_user,NOW(),:version)
pub fn insert_tx_items(
    txs: &[u32],
    dc_plan_id: u32,
    user: &str,
    version: i32,
    kphis_extra: &str,
) -> String {
    let tx = txs.iter().map(|id| ["(",&dc_plan_id.to_string(),",",&id.to_string(),",'",user,"',NOW(),'",user,"',NOW(),",&version.to_string(),")"].concat()).collect::<Vec<String>>().join(",");
    [
        "INSERT INTO ",kphis_extra,".opd_er_dc_plan_tx_item (dc_plan_id,tx_id",TABLE_CREATE_COLUMNS,") VALUES ",&tx
    ].concat()
}

pub fn insert_tx_items_only(
    txs: &[DischargePlanTxItemOnly],
    dc_plan_id: u32,
    kphis_extra: &str,
) -> String {
    let values = txs.iter().map(|item| {
        [
            "(",&dc_plan_id.to_string(),",",
            &item.tx_id.map(|s| s.to_string()).unwrap_or(String::from("NULL")),",'",
            &item.create_user,"','",
            &item.create_datetime.to_string(),"','",
            &item.update_user,"','",
            &item.update_datetime.to_string(),"',",
            &item.version.to_string(),")"
        ].concat()
    }).collect::<Vec<String>>().join(",");
    [
        "INSERT INTO ",kphis_extra,".opd_er_dc_plan_tx_item (dc_plan_id,tx_id",TABLE_CREATE_COLUMNS,") \
        VALUES ",&values
    ].concat()
}

// INSERT INTO kphis_extra.opd_er_dc_plan_diet_item (dc_plan_id,diet_id,create_user,create_datetime,update_user,update_datetime,version)
//     VALUES (:dc_plan_id,:diet_id,:create_user,NOW(),:update_user,NOW(),:version)
pub fn insert_diet_items(
    diets: &[u32],
    dc_plan_id: u32,
    user: &str,
    version: i32,
    kphis_extra: &str,
) -> String {
    let diet = diets.iter().map(|id| ["(",&dc_plan_id.to_string(),",",&id.to_string(),",'",user,"',NOW(),'",user,"',NOW(),",&version.to_string(),")"].concat()).collect::<Vec<String>>().join(",");
    [
        "INSERT INTO ",kphis_extra,".opd_er_dc_plan_diet_item (dc_plan_id,diet_id",TABLE_CREATE_COLUMNS,") VALUES ",&diet
    ].concat()
}

pub fn insert_diet_items_only(
    diets: &[DischargePlanDietItemOnly],
    dc_plan_id: u32,
    kphis_extra: &str,
) -> String {
    let values = diets.iter().map(|item| {
        [
            "(",&dc_plan_id.to_string(),",",
            &item.diet_id.map(|s| s.to_string()).unwrap_or(String::from("NULL")),",'",
            &item.create_user,"','",
            &item.create_datetime.to_string(),"','",
            &item.update_user,"','",
            &item.update_datetime.to_string(),"',",
            &item.version.to_string(),")"
        ].concat()
    }).collect::<Vec<String>>().join(",");
    [
        "INSERT INTO ",kphis_extra,".opd_er_dc_plan_diet_item (dc_plan_id,diet_id",TABLE_CREATE_COLUMNS,") \
        VALUES ",&values
    ].concat()
}

// DELETE FROM kphis_Extra.opd_er_dc_plan WHERE dc_plan_id = ?
/// dc_plan_id, version
pub fn delete_dc_plan(kphis_extra: &str) -> String {
    [
        "DELETE FROM ",kphis_extra,".opd_er_dc_plan WHERE dc_plan_id=? AND version=?;"
    ].concat()
}

// DELETE FROM kphis_extra.opd_er_dc_plan_med_item WHERE dc_plan_id=? AND version=?;
/// dc_plan_id, version
pub fn delete_med_item(kphis_extra: &str) -> String {
    [
        "DELETE FROM ",kphis_extra,".opd_er_dc_plan_med_item WHERE dc_plan_id=? AND version=?;"
    ].concat()
}

// DELETE FROM kphis_extra.opd_er_dc_plan_env_item WHERE dc_plan_id=? AND version=?;
/// dc_plan_id, version
pub fn delete_env_item(kphis_extra: &str) -> String {
    [
        "DELETE FROM ",kphis_extra,".opd_er_dc_plan_env_item WHERE dc_plan_id=? AND version=?;"
    ].concat()
}

// DELETE FROM kphis_extra.opd_er_dc_plan_tx_item WHERE dc_plan_id=? AND version=?;
/// dc_plan_id, version
pub fn delete_tx_item(kphis_extra: &str) -> String {
    [
        "DELETE FROM ",kphis_extra,".opd_er_dc_plan_tx_item WHERE dc_plan_id=? AND version=?;"
    ].concat()
}

// DELETE FROM kphis_extra.opd_er_dc_plan_diet_item WHERE dc_plan_id=? AND version=?;
/// dc_plan_id, version
pub fn delete_diet_item(kphis_extra: &str) -> String {
    [
        "DELETE FROM ",kphis_extra,".opd_er_dc_plan_diet_item WHERE dc_plan_id=? AND version=?;"
    ].concat()
}