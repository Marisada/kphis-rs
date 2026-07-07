use std::collections::HashMap;

use kphis_model::{
    progress_note::{ProgressNoteParams, ProgressNoteItemOnly},
    pre_order::progress_note::{PreProgressNoteItem, PreProgressNoteSave},
};

use crate::{TABLE_CREATE_COLUMNS, TABLE_CREATE_PREPARED, TABLE_UPDATE_SET};

// // opd-er-order-progress-note-data.php
// SELECT n.*, d.name AS order_doctor_name, d.licenseno AS doctor_licenseno
// FROM kphis.opd_er_order_progress_note n
// LEFT JOIN hos.doctor d ON d.code = n.progress_note_doctor WHERE 1=1
// ORDER BY n.progress_note_date, n.progress_note_time, n.progress_note_id;
/// (progress_note_id), (opd_er_order_master_id), (progress_note_date)
pub fn select_progress(params: &ProgressNoteParams, intern_roles: &[String], hosxp: &str, kphis: &str, kphis_extra: &str) -> String {
    let progress_note_id = if params.progress_note_id.is_some() {" AND progress_note_id=? "} else {""};
    let opd_er_order_master_id = if params.opd_er_order_master_id.is_some() {" AND opd_er_order_master_id=? "} else {""};
    let progress_note_date = if params.progress_note_date.is_some() {" AND progress_note_date=? "} else {""};
    [
        "SELECT n.*, d.name AS order_doctor_name, d.licenseno AS doctor_licenseno,\
        (SELECT entryposition FROM ",hosxp,".opduser WHERE doctorcode=n.progress_note_doctor AND (account_disable IS NULL OR account_disable='N') LIMIT 1) AS entryposition,\
        (SELECT EXISTS(SELECT * FROM ",hosxp,".opduser ou LEFT JOIN ",kphis,".system_ac_role_user ru ON ou.loginname=ru.loginname \
            WHERE ou.doctorcode=n.progress_note_doctor AND (ou.account_disable IS NULL OR ou.account_disable='N') AND ru.role IN ('",&intern_roles.join("','"),"'))) AS order_doctor_is_intern,\
        (SELECT GROUP_CONCAT(i.`path` ORDER BY u.image_usage_id) FROM ",kphis_extra,".image AS i \
            LEFT JOIN ",kphis_extra,".image_usage AS u ON u.image_id=i.image_id WHERE u.usage_id=4 AND u.usage_key_id=n.progress_note_id) AS imgs \
        FROM ",kphis,".opd_er_order_progress_note n \
            LEFT JOIN ",hosxp,".doctor d ON d.code = n.progress_note_doctor \
        WHERE 1=1 ",progress_note_id,opd_er_order_master_id,progress_note_date,
        "ORDER BY n.progress_note_date, n.progress_note_time, n.progress_note_id;"
    ].concat()
}

/// opd_er_order_master_id
pub fn select_progress_only(kphis: &str) -> String {
    [
        "SELECT n.* FROM ",kphis,".opd_er_order_progress_note n WHERE opd_er_order_master_id=? ORDER BY n.progress_note_date,n.progress_note_time,n.progress_note_id;"
    ].concat()
}

// SELECT DISTINCT pni.progress_note_item_type, pni.progress_note_id
// FROM kphis.opd_er_order_progress_note_item pni
// JOIN kphis.opd_er_order_progress_note o ON o.progress_note_id = pni.progress_note_id
// LEFT JOIN kphis.ipd_progress_note_item_type pnit ON pni.progress_note_item_type = pnit.progress_note_item_type
// WHERE pni.progress_note_id IN ()
// ORDER BY pnit.display_order, pni.progress_note_item_id;
pub fn select_progress_type(ids: &[u32], kphis: &str) -> String {
    let in_c = ids.iter().map(|id| id.to_string()).collect::<Vec<String>>().join(",");
    [
        "SELECT DISTINCT pni.progress_note_item_type, pni.progress_note_id FROM ",kphis,".opd_er_order_progress_note_item pni JOIN ",kphis,".opd_er_order_progress_note o ON o.progress_note_id = pni.progress_note_id \
        LEFT JOIN ",kphis,".ipd_progress_note_item_type pnit ON pni.progress_note_item_type = pnit.progress_note_item_type \
        WHERE pni.progress_note_id IN (",&in_c,") ORDER BY pnit.display_order, pni.progress_note_item_id;"
    ].concat()
}

// SELECT pni.* FROM kphis.opd_er_order_progress_note_item pni
// JOIN kphis.opd_er_order_progress_note o ON o.progress_note_id = pni.progress_note_id
// WHERE pni.progress_note_id=? AND pni.progress_note_item_type=?
// ORDER BY pni.progress_note_item_id;
pub fn select_progress_item(id: u32, progress_note_item_type: &str, kphis: &str) -> String {
    [
        "SELECT pni.* FROM ",kphis,".opd_er_order_progress_note_item pni \
            JOIN ",kphis,".opd_er_order_progress_note o ON o.progress_note_id = pni.progress_note_id \
        WHERE pni.progress_note_id=",&id.to_string()," AND pni.progress_note_item_type='",progress_note_item_type,
        "' ORDER BY pni.progress_note_item_id;"
    ].concat()
}

/// progress_note_id
pub fn select_progress_item_only(kphis: &str) -> String {
    [
        "SELECT pni.* FROM ",kphis,".opd_er_order_progress_note_item pni WHERE pni.progress_note_id=? ORDER BY pni.progress_note_item_id;"
    ].concat()
}

// INSERT INTO kphis.opd_er_order_progress_note (opd_er_order_master_id,progress_note_date,progress_note_time,progress_note_owner_type,progress_note_doctor,progress_note_enter_datetime,pre_order_progress_note_id,pre_order_progress_note_date,pre_order_progress_note_time,create_user,create_datetime,update_user,update_datetime,version)
// VALUES (?,DATE(NOW()),TIME(NOW()),?,?,?,NOW(),?,NOW(),1);"
/// opd_er_order_master_id, is_past_time(progress_note_for_past_time),
/// progress_note_owner_type, progress_note_doctor, loginname, loginname
pub fn insert_progress_note(is_past_time: bool, kphis: &str) -> String {
    let progress_note_time = if is_past_time {"?"} else {"NOW()"};
    [
        "INSERT INTO ",kphis,".opd_er_order_progress_note (opd_er_order_master_id,progress_note_date,progress_note_time,progress_note_owner_type,progress_note_doctor,progress_note_enter_datetime",TABLE_CREATE_COLUMNS,") \
        VALUES (?,NOW(),",progress_note_time,",?,?,NOW(),?,NOW(),?,NOW(),1);"
    ].concat()
}

/// opd_er_order_master_id,progress_note_date,progress_note_time,progress_note_owner_type,progress_note_doctor,progress_note_enter_datetime,pre_order_progress_note_id,pre_order_progress_note_date,pre_order_progress_note_time,create_user,create_datetime,update_user,update_datetime,version
pub fn insert_progress_note_only(kphis: &str) -> String {
    [
        "INSERT INTO ",kphis,".opd_er_order_progress_note (opd_er_order_master_id,progress_note_date,progress_note_time,progress_note_owner_type,progress_note_doctor,progress_note_enter_datetime,pre_order_progress_note_id,pre_order_progress_note_date,pre_order_progress_note_time",TABLE_CREATE_COLUMNS,") \
        VALUES (?,?,?,?,?,?,?,?,?,?,?,?,?,?);"
    ].concat()
}

// INSERT INTO kphis.opd_er_order_progress_note_item (progress_note_id,opd_er_order_master_id,progress_note_item_type,progress_note_item_detail,progress_note_item_detail_2,create_user,create_datetime,update_user,update_datetime,version)
// VALUES (?,?,?,?,?,now(),?,now(),1);
/// progress_note_id, opd_er_order_master_id, progress_note_item_type, progress_note_item_detail, loginname, loginname
pub fn insert_progress_note_item(kphis: &str) -> String {
    [
        "INSERT INTO ",kphis,".opd_er_order_progress_note_item (progress_note_id,opd_er_order_master_id,progress_note_item_type,progress_note_item_detail,progress_note_item_detail_2",TABLE_CREATE_COLUMNS,") \
        VALUES (?,?,?,?,?",TABLE_CREATE_PREPARED,");"
    ].concat()
}

pub fn insert_progress_note_items_only(progress_note_id: u32, opd_er_order_master_id: u32, progress_note_items: &[ProgressNoteItemOnly], kphis: &str) -> String {
    let values = progress_note_items.iter().map(|item| {
        [
            "(",&progress_note_id.to_string(),",",
            &opd_er_order_master_id.to_string(),",",
            &item.progress_note_item_type.as_ref().map(|s| ["'", s, "'"].concat()).unwrap_or(String::from("NULL")),",",
            &item.progress_note_item_detail.as_ref().map(|s| ["'", s, "'"].concat()).unwrap_or(String::from("NULL")),",",
            &item.progress_note_item_detail_2.as_ref().map(|s| ["'", s, "'"].concat()).unwrap_or(String::from("NULL")),",'",
            &item.create_user,"','",
            &item.create_datetime.to_string(),"','",
            &item.update_user,"','",
            &item.update_datetime.to_string(),"',",
            &item.version.to_string(),")"
        ].concat()
    }).collect::<Vec<String>>().join(",");
    [
        "INSERT INTO ",kphis,".opd_er_order_progress_note_item (progress_note_id,opd_er_order_master_id,progress_note_item_type,progress_note_item_detail,progress_note_item_detail_2",TABLE_CREATE_COLUMNS,") \
        VALUES ",&values,";"
    ].concat()
}

// INSERT INTO kphis.opd_er_order_progress_note (opd_er_order_master_id,progress_note_date,progress_note_time,pre_order_progress_note_id,pre_order_progress_note_date,pre_order_progress_note_time,progress_note_owner_type,progress_note_doctor,create_user,create_datetime,update_user,update_datetime,version)
// VALUES (?, DATE(NOW()), TIME(NOW()),?,?,?,NOW(),?,NOW(),1);
/// opd_er_order_master_id, progress_note_owner_type, progress_note_doctor, loginname, loginname
pub fn insert_many_progress_notes(
    progress_notes: &[PreProgressNoteSave],
    opd_er_order_master_id: u32,
    loginname: &str,
    doctorcode: &str,
    kphis: &str,
) -> String {
    progress_notes.iter().map(|note| {
        [
            "INSERT INTO ",kphis,".opd_er_order_progress_note (opd_er_order_master_id,progress_note_date,progress_note_time,pre_order_progress_note_id,pre_order_progress_note_date,pre_order_progress_note_time,progress_note_owner_type,progress_note_doctor",TABLE_CREATE_COLUMNS,") \
                VALUES (",&opd_er_order_master_id.to_string(),", DATE(NOW()), TIME(NOW()),",
                &note.progress_note_id.to_string(),",'",
                &note.progress_note_date.to_string(),"','",
                &note.progress_note_time.to_string(),"','",
                &note.progress_note_owner_type,"','",
                &doctorcode,"','",
                loginname, "',NOW(),'",loginname,"',NOW(),1);"
        ].concat()
    }).collect::<Vec<String>>().concat()
}

// INSERT INTO kphis.opd_er_order_progress_note_item (progress_note_id,opd_er_order_master_id,progress_note_item_type,progress_note_item_detail,create_user,create_datetime,update_user,update_datetime,version)
// VALUES (?,?,?,?,?,NOW(),?,NOW(),1);
/// :progress_note_id, :opd_er_order_master_id, :progress_note_item_type, :progress_note_item_detail, loginname, loginname
pub fn insert_progress_note_items(
    progress_note_items: &[PreProgressNoteItem],
    progress_note_id_map: &HashMap<u32, u64>,
    opd_er_order_master_id: u32,
    loginname: &str,
    kphis: &str,
) -> String {
    let values = progress_note_items.iter().map(|item| {
        [
            "(",&progress_note_id_map.get(&(item.progress_note_id.unwrap_or_default())).map(|id| id.to_string()).unwrap_or(String::from("NULL")),",",
            &opd_er_order_master_id.to_string(),",",
            &item.progress_note_item_type.as_ref().map(|s| ["'", s, "'"].concat()).unwrap_or(String::from("NULL")),",",
            &item.progress_note_item_detail.as_ref().map(|s| ["'", s, "'"].concat()).unwrap_or(String::from("NULL")),",'",
            loginname,"',NOW(),'",loginname,"',NOW(),1)"
        ].concat()
    }).collect::<Vec<String>>().join(",");
    [
        "INSERT INTO ",kphis,".opd_er_order_progress_note_item (progress_note_id,opd_er_order_master_id,progress_note_item_type,progress_note_item_detail",TABLE_CREATE_COLUMNS,") \
        VALUES ",&values,";"
    ].concat()
}

// // ipd-dr-order-progress-note-save.php
// UPDATE kphis.opd_er_order_progress_note SET opd_er_order_master_id=?, update_user=?, update_datetime=NOW(), version=1 WHERE progress_note_id=?;
/// opd_er_order_master_id, (progress_note_for_past_time), loginname, progress_note_id
pub fn update_progress_note_id(with_time: bool, kphis: &str) -> String {
    let progress_note_time = if with_time {",progress_note_time=?"} else {""};
    [
        "UPDATE ",kphis,".opd_er_order_progress_note SET opd_er_order_master_id=?",progress_note_time,TABLE_UPDATE_SET,
        " WHERE progress_note_id=?;"
    ].concat()
}

// DELETE kphis.opd_er_order_progress_note, kphis.opd_er_order_progress_note_item
// FROM kphis.opd_er_order_progress_note
//     LEFT JOIN kphis.opd_er_order_progress_note_item ON opd_er_order_progress_note.progress_note_id = opd_er_order_progress_note_item.progress_note_id
// WHERE opd_er_order_progress_note.progress_note_id=?;
// *** canNOT use alias IN delete `https://bugs.mysql.com/bug.php?id=82189` ***
/// progress_note_id
pub fn delete_progress_note(kphis: &str) -> String {
    [
        "DELETE ",kphis,".opd_er_order_progress_note, ",kphis,".opd_er_order_progress_note_item FROM ",kphis,".opd_er_order_progress_note \
            LEFT JOIN ",kphis,".opd_er_order_progress_note_item ON opd_er_order_progress_note.progress_note_id = opd_er_order_progress_note_item.progress_note_id \
        WHERE opd_er_order_progress_note.progress_note_id=?;"
    ].concat()
}

// DELETE FROM kphis.opd_er_order_progress_note_item WHERE progress_note_id=?;
/// progress_note_id
pub fn delete_progress_note_item(kphis: &str) -> String {
    [
        "DELETE FROM ",kphis,".opd_er_order_progress_note_item WHERE progress_note_id=?;"
    ].concat()
}