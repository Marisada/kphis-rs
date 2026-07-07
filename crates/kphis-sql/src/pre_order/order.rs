use std::collections::HashMap;

use kphis_model::{
    order::OrderItemSave,
    pre_order::order::{PreOrderParams, PreOrderSave},
};

use crate::{TABLE_CREATE_COLUMNS, TABLE_CREATE_PREPARED, TABLE_UPDATE_SET};

// // ipd-dr-pre-order-one-day-data.php
// SELECT i.*,d.name AS order_doctor_name,d.licenseno AS doctor_licenseno, na.name AS nurse_accept_name,pa.name AS pharmacist_accept_name,pc.name AS pharmacist_check_name,pd.name AS pharmacist_done_name
// FROM kphis.ipd_pre_order i
//     LEFT JOIN hos.doctor d ON d.code = i.order_doctor
//     LEFT JOIN hos.doctor na ON na.code = i.nurse_accept
//     LEFT JOIN hos.doctor pa ON pa.code = i.pharmacist_accept
//     LEFT JOIN hos.doctor pc ON pc.code = i.pharmacist_check
//     LEFT JOIN hos.doctor pd ON pd.code = i.pharmacist_done
// WHERE order_type='oneday'
// ORDER BY i.order_date, i.order_time, i.order_id;
/// (order_id),(order_confirm)
/// require pre_order_master_id OR order_id
pub fn select_order(params: &PreOrderParams, intern_roles: &[String], hosxp: &str, kphis: &str) -> String {
    let order_type = params.order_type.as_ref().map(|t| {
        if ["oneday", "continuous"].contains(&t.as_str()) {
            [" AND i.order_type ='", t, "' "].concat()
        } else {
            String::new()
        }
    }).unwrap_or_default();
    let order_id = if params.order_id.is_some() {" AND i.order_id=?"} else {""};
    let master_id = params.pre_order_master_id.map(|id| [" AND i.pre_order_master_id=", &id.to_string()].concat()).unwrap_or_default();
    let order_confirm = if params.order_confirm.is_some() {" AND i.order_confirm=?"} else {""};
    let order_owner_types = if let Some(owner_types) = params.order_owner_type.clone() {
        let split = owner_types.split(',').collect::<Vec<&str>>();
        if split.is_empty() {String::new()} else {[" AND i.order_owner_type IN ('", &split.join("','"), "')"].concat()}
    } else {
        String::new()
    };
    let view_by = params.view_by.as_ref().map(|vb| {
        match vb.as_str() {
            "doctor" => " AND i.order_owner_type = 'doctor'",
            "nurse" => " AND ((i.order_owner_type = 'doctor' AND i.order_confirm = 'Y') OR (i.order_owner_type = 'nurse'))",
            "pharmacist" | "other" => " AND ((i.order_owner_type = 'doctor' OR i.order_owner_type = 'nurse') AND i.order_confirm = 'Y')",
            _ => " AND false ",
        }
    }).unwrap_or_default();

    [
        "SELECT i.*,d.name AS order_doctor_name,d.licenseno AS doctor_licenseno,na.name AS nurse_accept_name,pa.name AS pharmacist_accept_name,pc.name AS pharmacist_check_name,pd.name AS pharmacist_done_name,\
        (SELECT EXISTS(SELECT * FROM ",hosxp,".opduser ou LEFT JOIN ",kphis,".system_ac_role_user ru ON ou.loginname=ru.loginname \
            WHERE ou.doctorcode=i.order_doctor AND (ou.account_disable IS NULL OR ou.account_disable='N') AND ru.role IN ('",&intern_roles.join("','"),"'))) AS order_doctor_is_intern \
        FROM ",kphis,".ipd_pre_order i \
            LEFT JOIN ",hosxp,".doctor d ON d.code = i.order_doctor \
            LEFT JOIN ",hosxp,".doctor na ON na.code = i.nurse_accept \
            LEFT JOIN ",hosxp,".doctor pa ON pa.code = i.pharmacist_accept \
            LEFT JOIN ",hosxp,".doctor pc ON pc.code = i.pharmacist_check \
            LEFT JOIN ",hosxp,".doctor pd ON pd.code = i.pharmacist_done \
        WHERE 1=1 ",&order_type, order_id, &master_id, order_confirm, &order_owner_types, view_by,
        " ORDER BY i.order_date, i.order_time, i.order_id;"
    ].concat()
}

// SELECT DISTINCT oi.order_item_type, oi.order_id
// FROM kphis.ipd_pre_order_item oi
// JOIN kphis.ipd_pre_order o ON o.order_id = oi.order_id
// LEFT JOIN kphis.ipd_order_item_type oit ON oi.order_item_type = oit.order_item_type AND o.order_type = oit.order_type
// WHERE o.order_type='oneday' AND oi.order_id IN (1,2)
// ORDER BY oit.display_order, oi.order_item_id;
pub fn select_order_types(ids: &[u32], order_type: &str, kphis: &str) -> String {
    let in_c = ids.iter().map(|id| id.to_string()).collect::<Vec<String>>().join(",");
    [
        "SELECT DISTINCT oi.order_item_type, oi.order_id \
        FROM ",kphis,".ipd_pre_order_item oi \
            JOIN ",kphis,".ipd_pre_order o ON o.order_id = oi.order_id \
            LEFT JOIN ",kphis,".ipd_order_item_type oit ON oi.order_item_type = oit.order_item_type AND o.order_type = oit.order_type \
        WHERE o.order_type='",order_type,"' AND oi.order_id IN (",&in_c,") ORDER BY oit.display_order, oi.order_item_id;"
    ].concat()
}

// SELECT oi.*, ooi.order_item_detail AS off_order_item_detail, null AS off_by_order_item_id,
// CONCAT(di.`name`, ' ', di.strength, ' ',di.units) AS med_name, di.displaycolor, di.generic_name, ooi.icode AS off_icode,
// CONCAT(off_di.`name`, ' ', off_di.strength, ' ',off_di.units) AS off_med_name, off_di.displaycolor AS off_displaycolor,
// GROUP_CONCAT(DISTINCT(CONCAT(allergy.agent,'=',IFNULL(allergy.symptom,''))) ORDER BY allergy.agent) AS allergy_agent_symptom
// FROM kphis.ipd_pre_order_item oi JOIN kphis.ipd_pre_order o ON o.order_id = oi.order_id
// LEFT JOIN kphis.ipd_pre_order_item ooi ON ooi.order_item_id = oi.off_order_item_id
// LEFT JOIN hos.drugitems di ON di.icode = oi.icode
// LEFT JOIN hos.drugitems off_di ON off_di.icode = ooi.icode
// LEFT JOIN kphis.ipd_pre_order_master pom ON o.pre_order_master_id = pom.pre_order_master_id
// LEFT JOIN hos.opd_allergy allergy ON (
//     (allergy.agent LIKE CONCAT('%',di.generic_name,'%') AND allergy.hn=pom.hn AND di.generic_name IS NOT NULL AND TRIM(di.generic_name) <> '')
//     OR (di.generic_name LIKE CONCAT('%',allergy.agent,'%') AND allergy.hn=pom.hn AND allergy.agent IS NOT NULL AND TRIM(allergy.agent) <> ''))
// WHERE oi.order_id=? AND oi.order_item_type=? AND o.order_type='oneday' GROUP BY oi.order_item_id ORDER BY oi.order_item_id;
pub fn select_order_item(
    order_id: u32,
    order_item_type: &str,
    order_type: &str,
    hosxp: &str,
    kphis: &str,
) -> String {
    [
        "SELECT oi.*,ooi.order_item_detail AS off_order_item_detail,null AS off_by_order_item_id,\
        CONCAT(di.`name`,' ',di.strength,' ',di.units) AS med_name,di.displaycolor,di.generic_name,di.dosageform,ooi.icode AS off_icode,\
        CONCAT(off_di.`name`,' ',off_di.strength,' ',off_di.units) AS off_med_name,off_di.displaycolor AS off_displaycolor,\
        GROUP_CONCAT(DISTINCT(CONCAT(allergy.agent,'=',IFNULL(allergy.symptom,''))) ORDER BY allergy.agent) AS allergy_agent_symptom \
        FROM ",kphis,".ipd_pre_order_item oi JOIN ",kphis,".ipd_pre_order o ON o.order_id=oi.order_id \
            LEFT JOIN ",kphis,".ipd_pre_order_item ooi ON ooi.order_item_id=oi.off_order_item_id \
            LEFT JOIN ",hosxp,".drugitems di ON di.icode=oi.icode \
            LEFT JOIN ",hosxp,".drugitems off_di ON off_di.icode=ooi.icode \
            LEFT JOIN ",kphis,".ipd_pre_order_master pom ON o.pre_order_master_id=pom.pre_order_master_id \
            LEFT JOIN ",hosxp,".opd_allergy allergy ON (\
                (allergy.agent LIKE CONCAT('%',di.generic_name,'%') AND allergy.hn=pom.hn AND di.generic_name IS NOT NULL AND TRIM(di.generic_name) <> '') \
                OR (di.generic_name LIKE CONCAT('%',allergy.agent,'%') AND allergy.hn=pom.hn AND allergy.agent IS NOT NULL AND TRIM(allergy.agent) <> '')) \
        WHERE oi.order_id=",&order_id.to_string()," AND oi.order_item_type='",order_item_type,"' AND o.order_type='",order_type,"' GROUP BY oi.order_item_id ORDER BY oi.order_item_id;"
    ].concat()
}

// // ipd-dr-pre-order-one-day-save.php
// INSERT INTO kphis.ipd_pre_order (pre_order_master_id,order_date,order_time,order_doctor,order_type,order_owner_type,order_confirm,create_user,create_datetime,update_user,update_datetime,version)
// VALUES (?,DATE(NOW()),TIME(NOW()),?,?,?,'N',?,NOW(),?,NOW(),1);
/// pre_order_master_id, order_doctor, order_type, order_owner_type, loginname, loginname
pub fn insert_pre_order(kphis: &str) -> String {
    [
        "INSERT INTO ",kphis,".ipd_pre_order (pre_order_master_id,order_date,order_time,order_doctor,order_type,order_owner_type,order_confirm",TABLE_CREATE_COLUMNS,") \
        VALUES (?,DATE(NOW()),TIME(NOW()),?,?,?,'N'",TABLE_CREATE_PREPARED,");"
    ].concat()
}

// UPDATE kphis.ipd_pre_order SET order_date=DATE(NOW()), order_time=TIME(NOW()),
// order_doctor=?, update_user=?, update_datetime=NOW(), version=(version+1) WHERE order_id=?;
/// order_doctor, loginname, order_id
pub fn update_pre_order(kphis: &str) -> String {
    [
        "UPDATE ",kphis,".ipd_pre_order SET order_date=DATE(NOW()), order_time=TIME(NOW()),order_doctor=?",TABLE_UPDATE_SET," WHERE order_id=?;"
    ].concat()
}

// DELETE FROM kphis.ipd_pre_order_item WHERE order_id=?;
/// order_id
pub fn delete_pre_order_item(kphis: &str) -> String {
    [
        "DELETE FROM ",kphis,".ipd_pre_order_item WHERE order_id=?;"
    ].concat()
}

// INSERT INTO kphis.ipd_pre_order_item (order_id,pre_order_master_id,order_item_type,order_item_detail,stat,off_order_item_id,icode,create_user,create_datetime,update_user,update_datetime,version)
// VALUES (?,?,?,?,?,?,?,?,NOW(),?,NOW(),1);
/// order_id, pre_order_master_id, order_item_type, order_item_detail, stat, off_order_item_id, icode, loginname, loginname
pub fn insert_pre_order_item(kphis: &str) -> String {
    [
        "INSERT INTO ",kphis,".ipd_pre_order_item (order_id,pre_order_master_id,order_item_type,order_item_detail,stat,off_order_item_id,icode",TABLE_CREATE_COLUMNS,") \
        VALUES (?,?,?,?,?,?,?",TABLE_CREATE_PREPARED,");"
    ].concat()
}

// DELETE FROM kphis.ipd_pre_order_item WHERE pre_order_master_id=?;
/// pre_order_master_id
pub fn delete_pre_order_item_by_master_id(kphis: &str) -> String {
    [
        "DELETE FROM ",kphis,".ipd_pre_order_item WHERE pre_order_master_id=?;"
    ].concat()
}

// DELETE FROM kphis.ipd_pre_order WHERE pre_order_master_id=?;
/// pre_order_master_id
pub fn delete_pre_order_by_master_id(kphis: &str) -> String {
    [
        "DELETE FROM ",kphis,".ipd_pre_order WHERE pre_order_master_id=?;"
    ].concat()
}

// // ipd-dr-pre-order-one-day-delete.php, ipd-dr-pre-order-continuous-delete.php
// DELETE kphis.ipd_pre_order, kphis.ipd_pre_order_item FROM kphis.ipd_pre_order
//    LEFT JOIN kphis.ipd_pre_order_item ON ipd_pre_order_item.order_id = ipd_pre_order.order_id WHERE ipd_pre_order.order_id = ?;
// *** cannot use alias in delete `https://bugs.mysql.com/bug.php?id=82189` ***
/// order_id
pub fn delete_pre_order(kphis: &str) -> String {
    [
        "DELETE ",kphis,".ipd_pre_order, ",kphis,".ipd_pre_order_item \
        FROM ",kphis,".ipd_pre_order \
            LEFT JOIN ",kphis,".ipd_pre_order_item ON ipd_pre_order_item.order_id = ipd_pre_order.order_id \
        WHERE ipd_pre_order.order_id = ?;"
    ].concat()
}

// SELECT * FROM kphis.ipd_pre_order WHERE pre_order_master_id = ? ORDER BY order_id;
/// pre_order_master_id
pub fn select_pre_order_to(kphis: &str) -> String {
    [
        "SELECT * FROM ",kphis,".ipd_pre_order WHERE pre_order_master_id = ? ORDER BY order_id;"
    ].concat()
}

// SELECT * FROM kphis.ipd_pre_order_item WHERE order_id IN (1,2) ORDER BY order_item_id;
pub fn select_pre_order_item_to(order_ids: &[u32], kphis: &str) -> String {
    let in_c = if order_ids.is_empty() {
        String::from("0")
    } else {
        order_ids.iter().map(|id| id.to_string()).collect::<Vec<String>>().join(",")
    };
    [
        "SELECT * FROM ",kphis,".ipd_pre_order_item \
        WHERE order_id IN (",&in_c,") ORDER BY order_item_id;"
    ].concat()
}

// INSERT INTO kphis.ipd_pre_order (pre_order_master_id,order_date,order_time,order_doctor,order_type,order_owner_type,order_confirm,create_user,create_datetime,update_user,update_datetime,version)
// VALUES (?,DATE(NOW()),TIME(NOW()),?,?,?,'N',?,NOW(),?,NOW(),1);
/// pre_order_master_id, order_doctor, order_type, order_owner_type, loginname, loginname
pub fn insert_many_pre_orders(
    pre_orders: &[PreOrderSave],
    loginname: &str,
    pre_order_master_id: u32,
    doctorcode: &str,
    kphis: &str,
) -> String {
    pre_orders.iter().map(|order| {
        [
            "INSERT INTO ",kphis,".ipd_pre_order (pre_order_master_id,order_date,order_time,order_doctor,order_type,order_owner_type,order_confirm",TABLE_CREATE_COLUMNS,") \
                VALUES (",&pre_order_master_id.to_string(),",DATE(NOW()),TIME(NOW()),'",
                &doctorcode,"','",
                &order.order_type,"','",
                &order.order_owner_type,"','N','",
                loginname, "',NOW(),'",loginname,"',NOW(),1);"
        ].concat()
    }).collect::<Vec<String>>().concat()
}

// INSERT INTO kphis.ipd_pre_order_item (order_id,pre_order_master_id,order_item_type,order_item_detail,stat,off_order_item_id,icode,create_user,create_datetime,update_user,update_datetime,version)
// VALUES (?,?,?,?,?,?,?,?,NOW(),?,NOW(),1);
/// order_id, pre_order_master_id, order_item_type, order_item_detail, stat, off_order_item_id, icode, loginname, loginname
pub fn insert_pre_order_items(
    order_items: &[OrderItemSave],
    order_id_map: &HashMap<u32, u64>,
    pre_order_master_id: u32,
    loginname: &str,
    kphis: &str,
) -> String {
    let values = order_items.iter().map(|item| {
        [
            "(",&order_id_map.get(&(item.order_id.unwrap_or_default())).map(|id| id.to_string()).unwrap_or(String::from("NULL")),",",
            &pre_order_master_id.to_string(),",",
            &item.order_item_type.as_ref().map(|s| ["'", s, "'"].concat()).unwrap_or(String::from("NULL")),",",
            &item.order_item_detail.as_ref().map(|s| ["'", s, "'"].concat()).unwrap_or(String::from("NULL")),",",
            &item.stat.as_ref().map(|s| ["'", s, "'"].concat()).unwrap_or(String::from("NULL")),",",
            &item.off_order_item_id.map(|id| id.to_string()).unwrap_or(String::from("NULL")),",",
            &item.icode.as_ref().map(|s| ["'", s, "'"].concat()).unwrap_or(String::from("NULL")),",'",
            loginname,"',NOW(),'",loginname,"',NOW(),1)"
        ].concat()
    }).collect::<Vec<String>>().join(",");
    [
        "INSERT INTO ",kphis,".ipd_pre_order_item (order_id,pre_order_master_id,order_item_type,order_item_detail,stat,off_order_item_id,icode",TABLE_CREATE_COLUMNS,") \
        VALUES ",&values,";"
    ].concat()
}