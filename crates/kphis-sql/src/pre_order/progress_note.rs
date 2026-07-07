use std::collections::HashMap;

use kphis_model::pre_order::progress_note::{PreProgressNoteItem, PreProgressNoteParams, PreProgressNoteSave};

use crate::{TABLE_CREATE_COLUMNS, TABLE_CREATE_PREPARED, TABLE_UPDATE_SET};

// // ipd-dr-pre-order-progress-note-data.php
// SELECT n.*, d.name AS order_doctor_name, d.licenseno AS doctor_licenseno FROM kphis.ipd_pre_order_progress_note n
// LEFT JOIN hos.doctor d ON d.code = n.progress_note_doctor WHERE 1=1
// ORDER BY n.progress_note_date, n.progress_note_time, n.progress_note_id;
/// (progress_note_id),(pre_order_master_id)
/// require pre_order_master_id OR progress_note_id
pub fn select_progress(params: &PreProgressNoteParams, intern_roles: &[String], hosxp: &str, kphis: &str) -> String {
    let progress_note_id = if params.progress_note_id.is_some() {" AND progress_note_id=? "} else {""};
    let pre_order_master_id = if params.pre_order_master_id.is_some() {" AND pre_order_master_id=? "} else {""};
    [
        "SELECT n.*, d.name AS order_doctor_name, d.licenseno AS doctor_licenseno,\
        (SELECT entryposition FROM ",hosxp,".opduser WHERE doctorcode=n.progress_note_doctor AND (account_disable IS NULL OR account_disable='N') LIMIT 1) AS entryposition,\
        (SELECT EXISTS(SELECT * FROM ",hosxp,".opduser ou LEFT JOIN ",kphis,".system_ac_role_user ru ON ou.loginname=ru.loginname \
            WHERE ou.doctorcode=n.progress_note_doctor AND (ou.account_disable IS NULL OR ou.account_disable='N') AND ru.role IN ('",&intern_roles.join("','"),"'))) AS order_doctor_is_intern \
        FROM ",kphis,".ipd_pre_order_progress_note n \
            LEFT JOIN ",hosxp,".doctor d ON d.code = n.progress_note_doctor \
        WHERE 1=1 ",progress_note_id,pre_order_master_id,
        "ORDER BY n.progress_note_date, n.progress_note_time, n.progress_note_id;"
    ].concat()
}

// SELECT DISTINCT pni.progress_note_item_type, pni.progress_note_id
// FROM kphis.ipd_pre_order_progress_note_item pni
// JOIN kphis.ipd_pre_order_progress_note o ON o.progress_note_id = pni.progress_note_id
// LEFT JOIN kphis.ipd_progress_note_item_type pnit ON pni.progress_note_item_type = pnit.progress_note_item_type
// WHERE pni.progress_note_id IN (1,2)
// ORDER BY pnit.display_order, pni.progress_note_item_id;
pub fn select_progress_type(ids: &[u32], kphis: &str) -> String {
    let in_c = ids.iter().map(|id| id.to_string()).collect::<Vec<String>>().join(",");
    [
        "SELECT DISTINCT pni.progress_note_item_type, pni.progress_note_id \
        FROM ",kphis,".ipd_pre_order_progress_note_item pni \
            JOIN ",kphis,".ipd_pre_order_progress_note o ON o.progress_note_id = pni.progress_note_id \
            LEFT JOIN ",kphis,".ipd_progress_note_item_type pnit ON pni.progress_note_item_type = pnit.progress_note_item_type \
        WHERE pni.progress_note_id IN (",&in_c,") ORDER BY pnit.display_order, pni.progress_note_item_id;"
    ].concat()
}

// SELECT pni.* FROM kphis.ipd_pre_order_progress_note_item pni
// JOIN kphis.ipd_pre_order_progress_note o ON o.progress_note_id = pni.progress_note_id
// WHERE pni.progress_note_id=1 AND pni.progress_note_item_type='oneday'
// ORDER BY pni.progress_note_item_id;
pub fn select_progress_item(id: u32, progress_note_item_type: &str, kphis: &str) -> String {
    [
        "SELECT pni.* FROM ",kphis,".ipd_pre_order_progress_note_item pni \
            JOIN ",kphis,".ipd_pre_order_progress_note o ON o.progress_note_id = pni.progress_note_id \
        WHERE pni.progress_note_id=",&id.to_string()," AND pni.progress_note_item_type='",progress_note_item_type,
        "' ORDER BY pni.progress_note_item_id;"
    ].concat()
}

// // ipd-dr-pre-order-progress-note-save.php
// INSERT INTO kphis.ipd_pre_order_progress_note (pre_order_master_id,progress_note_date,progress_note_time,progress_note_owner_type,progress_note_doctor,create_user,create_datetime,update_user,update_datetime,version)
// VALUES (?,DATE(NOW()),TIME(NOW()),?,?,?,NOW(),?,NOW(),1);
/// pre_order_master_id, progress_note_owner_type, progress_note_doctor, loginname, loginname
pub fn insert_progress_note(kphis: &str) -> String {
    [
        "INSERT INTO ",kphis,".ipd_pre_order_progress_note (pre_order_master_id,progress_note_date,progress_note_time,progress_note_owner_type,progress_note_doctor",TABLE_CREATE_COLUMNS,") \
        VALUES (?,DATE(NOW()),TIME(NOW()),?,?",TABLE_CREATE_PREPARED,");"
    ].concat()
}

// // UPDATE kphis.ipd_pre_order_progress_note SET pre_order_master_id=?, progress_note_date=?, progress_note_time=?, update_user=?, update_datetime=NOW(), version=1
// // WHERE progress_note_id=?;
// /// pre_order_master_id, progress_note_date, progress_note_time, loginname, progress_note_id
// we not 'progress_note_date' and 'progress_note_time'
// UPDATE kphis.ipd_pre_order_progress_note SET pre_order_master_id=?, update_user=?, update_datetime=NOW(), version=1
// WHERE progress_note_id=?;
/// pre_order_master_id, loginname, progress_note_id
pub fn update_progress_note_id(kphis: &str) -> String {
    [
        "UPDATE ",kphis,".ipd_pre_order_progress_note SET pre_order_master_id=?",TABLE_UPDATE_SET," WHERE progress_note_id=?;"
    ].concat()
}

// DELETE kphis.ipd_pre_order_progress_note, kphis.ipd_pre_order_progress_note_item FROM kphis.ipd_pre_order_progress_note JOIN kphis.ipd_pre_order_progress_note_item ON ipd_pre_order_progress_note.progress_note_id = ipd_pre_order_progress_note_item.progress_note_id WHERE ipd_pre_order_progress_note.progress_note_id=?;
/// progress_note_id
pub fn delete_progress_note(kphis: &str) -> String {
    [
        "DELETE ",kphis,".ipd_pre_order_progress_note, ",kphis,".ipd_pre_order_progress_note_item FROM ",kphis,".ipd_pre_order_progress_note JOIN ",kphis,".ipd_pre_order_progress_note_item ON ipd_pre_order_progress_note.progress_note_id = ipd_pre_order_progress_note_item.progress_note_id WHERE ipd_pre_order_progress_note.progress_note_id=?;"
    ].concat()
}

// DELETE FROM kphis.ipd_pre_order_progress_note_item WHERE progress_note_id=?;
/// progress_note_id
pub fn delete_progress_note_item(kphis: &str) -> String {
    [
        "DELETE FROM ",kphis,".ipd_pre_order_progress_note_item WHERE progress_note_id=?;"
    ].concat()
}

// INSERT INTO kphis.ipd_pre_order_progress_note_item (progress_note_id,pre_order_master_id,progress_note_item_type,progress_note_item_detail,progress_note_item_detail_2,create_user,create_datetime,update_user,update_datetime,version)
// VALUES (?,?,?,?,?,?,NOW(),?,NOW(),1);
/// progress_note_id, pre_order_master_id, progress_note_item_type, progress_note_item_detail, loginname, loginname
pub fn insert_progress_note_item(kphis: &str) -> String {
    [
        "INSERT INTO ",kphis,".ipd_pre_order_progress_note_item (progress_note_id,pre_order_master_id,progress_note_item_type,progress_note_item_detail,progress_note_item_detail_2",TABLE_CREATE_COLUMNS,") \
        VALUES (?,?,?,?,?",TABLE_CREATE_PREPARED,");"
    ].concat()
}

// DELETE FROM kphis.ipd_pre_order_progress_note_item WHERE pre_order_master_id=?;
/// pre_order_master_id
pub fn delete_progress_note_item_by_master_id(kphis: &str) -> String {
    [
        "DELETE FROM ",kphis,".ipd_pre_order_progress_note_item WHERE pre_order_master_id=?;"
    ].concat()
}

// DELETE FROM kphis.ipd_pre_order_progress_note WHERE pre_order_master_id=?;
/// pre_order_master_id
pub fn delete_progress_note_by_master_id(kphis: &str) -> String {
    [
        "DELETE FROM ",kphis,".ipd_pre_order_progress_note WHERE pre_order_master_id=?;"
    ].concat()
}

// SELECT * FROM kphis.ipd_pre_order_progress_note WHERE pre_order_master_id = ? ORDER BY progress_note_id;
/// pre_order_master_id
pub fn select_progress_note_to(kphis: &str) -> String {
    [
        "SELECT * FROM ",kphis,".ipd_pre_order_progress_note \
        WHERE pre_order_master_id = ? ORDER BY progress_note_id;"
    ].concat()
}

// SELECT * FROM kphis.ipd_pre_order_progress_note_item WHERE progress_note_id = ? ORDER BY progress_note_item_id;
pub fn select_progress_note_item_to(progress_note_ids: &[u32], kphis: &str) -> String {
    let in_c = if progress_note_ids.is_empty() {
        String::from("0")
    } else {
        progress_note_ids.iter().map(|id| id.to_string()).collect::<Vec<String>>().join(",")
    };
    [
        "SELECT * FROM ",kphis,".ipd_pre_order_progress_note_item \
        WHERE progress_note_id IN (",&in_c,") ORDER BY progress_note_item_id;"
    ].concat()
}

// INSERT INTO kphis.ipd_pre_order_progress_note (pre_order_master_id,progress_note_date,progress_note_time,progress_note_owner_type,progress_note_doctor,create_user,create_datetime,update_user,update_datetime,version)
// VALUES (?,DATE(NOW()),TIME(NOW()),?,?,?,NOW(),?,NOW(),1);
/// pre_order_master_id, progress_note_owner_type, progress_note_doctor, loginname, loginname
pub fn insert_many_progress_notes(
    progress_notes: &[PreProgressNoteSave],
    pre_order_master_id: u32,
    loginname: &str,
    doctorcode: &str,
    kphis: &str,
) -> String {
    progress_notes.iter().map(|note| {
        [
            "INSERT INTO ",kphis,".ipd_pre_order_progress_note (pre_order_master_id,progress_note_date,progress_note_time,progress_note_owner_type,progress_note_doctor",TABLE_CREATE_COLUMNS,") \
                VALUES (",&pre_order_master_id.to_string(),",DATE(NOW()),TIME(NOW()),'",
                &note.progress_note_owner_type,"','",
                &doctorcode,"','",
                loginname, "',NOW(),'",loginname,"',NOW(),1);"
        ].concat()
    }).collect::<Vec<String>>().concat()
}

// INSERT INTO kphis.ipd_pre_order_progress_note_item (progress_note_id,pre_order_master_id,progress_note_item_type,progress_note_item_detail,progress_note_item_detail_2,create_user,create_datetime,update_user,update_datetime,version)
// VALUES (?,?,?,?,?,?,NOW(),?,NOW(),1);
pub fn insert_progress_note_items(
    progress_note_items: &[PreProgressNoteItem],
    progress_note_id_map: &HashMap<u32, u64>,
    pre_order_master_id: u32,
    loginname: &str,
    kphis: &str,
) -> String {
    let values = progress_note_items.iter().map(|item| {
        [
            "(",&progress_note_id_map.get(&(item.progress_note_id.unwrap_or_default())).map(|id| id.to_string()).unwrap_or(String::from("NULL")),",",
            &pre_order_master_id.to_string(),",",
            &item.progress_note_item_type.as_ref().map(|s| ["'", s, "'"].concat()).unwrap_or(String::from("NULL")),",",
            &item.progress_note_item_detail.as_ref().map(|s| ["'", s, "'"].concat()).unwrap_or(String::from("NULL")),",",
            &item.progress_note_item_detail_2.as_ref().map(|s| ["'", s, "'"].concat()).unwrap_or(String::from("NULL")),",'",
            loginname,"',NOW(),'",loginname,"',NOW(),1)"
        ].concat()
    }).collect::<Vec<String>>().join(",");
    [
        "INSERT INTO ",kphis,".ipd_pre_order_progress_note_item (progress_note_id,pre_order_master_id,progress_note_item_type,progress_note_item_detail,progress_note_item_detail_2",TABLE_CREATE_COLUMNS,") \
        VALUES ",&values,";"
    ].concat()
}