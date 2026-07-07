use kphis_model::focus_note::{FocusNoteIntvtItemOnly, FocusNoteDlcItemOnly, FocusNoteParams};

use crate::{TABLE_CREATE_COLUMNS, TABLE_CREATE_PREPARED, TABLE_UPDATE_SET};

// // opd-er-nurse-focus-note-table-all-showdata.php
// SELECT f.*,k.name AS user_name,k.entryposition
//     FROM kphis.opd_er_focus_note f
//     LEFT JOIN hos.opduser k ON k.loginname = f.create_user
//     WHERE f.opd_er_order_master_id=:opd_er_order_master_id
//         AND f.fcnote_date = :search_startdate
//         AND f.fcnote_date = :search_enddate
//         AND f.fcnote_date BETWEEN :search_startdate AND :search_enddate
//     ORDER BY f.fcnote_date ASC, f.fcnote_time ASC, f.fcnote_id ASC
// SELECT opd_er_focus_list.focus_id  AS focus_id_tmp,opd_er_focus_list.focus_text
//     FROM kphis.opd_er_focus_list WHERE opd_er_focus_list.fclist_id = :fclist_id
// SELECT ipd_tmp_focus.focus_name
//     FROM kphis.ipd_tmp_focus WHERE ipd_tmp_focus.focus_id = :focus_id_tmp
// SELECT COUNT(*) AS count_data_item_intvt
//     FROM kphis.opd_er_focus_note_intvt_item WHERE fcnote_id =:fcnote_id
// SELECT item_in.intvt_id, tmp_in.intvt_name
//     FROM kphis.opd_er_focus_note_intvt_item item_in
//     LEFT JOIN kphis.ipd_tmp_intvt tmp_in ON tmp_in.intvt_id = item_in.intvt_id
//     WHERE item_in.fcnote_id =:fcnote_id ORDER BY item_in.intvt_id ASC
// SELECT COUNT(*) AS count_data_item_dlc
//     FROM kphis.opd_er_focus_note_dlc_item WHERE fcnote_id =:fcnote_id
// SELECT item_d.dlc_id, tmp_d.dlc_name
//     FROM kphis.opd_er_focus_note_dlc_item item_d
//     LEFT JOIN kphis.ipd_tmp_dlc tmp_d ON tmp_d.dlc_id = item_d.dlc_id
//     WHERE item_d.fcnote_id =:fcnote_id ORDER BY item_d.dlc_id ASC
// // opd-er-nurse-focus-note-table.php
// SELECT f.*,k.name AS user_name,k.entryposition
//     FROM kphis.opd_er_focus_note f
//     LEFT JOIN hos.opduser k ON k.loginname = f.create_user
//     WHERE f.opd_er_order_master_id=:opd_er_order_master_id AND f.fcnote_date=:date
//     ORDER BY f.fcnote_date, f.fcnote_time  DESC, f.fcnote_id DESC
// SELECT opd_er_focus_list.focus_id  AS focus_id_tmp,opd_er_focus_list.focus_text
//     FROM kphis.opd_er_focus_list WHERE opd_er_focus_list.fclist_id = :fclist_id
// SELECT ipd_tmp_focus.focus_name
//     FROM kphis.ipd_tmp_focus WHERE ipd_tmp_focus.focus_id = :focus_id_tmp
// SELECT COUNT(*) AS count_data_item_intvt
//     FROM kphis.opd_er_focus_note_intvt_item WHERE fcnote_id =:fcnote_id
// SELECT item_in.intvt_id, tmp_in.intvt_name
//     FROM kphis.opd_er_focus_note_intvt_item item_in
//     LEFT JOIN kphis.ipd_tmp_intvt tmp_in ON tmp_in.intvt_id = item_in.intvt_id
//     WHERE item_in.fcnote_id =:fcnote_id ORDER BY item_in.intvt_id ASC
// SELECT COUNT(*) AS count_data_item_dlc
//     FROM kphis.opd_er_focus_note_dlc_item WHERE fcnote_id =:fcnote_id
// SELECT item_d.dlc_id, tmp_d.dlc_name
//     FROM kphis.opd_er_focus_note_dlc_item item_d
//     LEFT JOIN kphis.ipd_tmp_dlc tmp_d ON tmp_d.dlc_id = item_d.dlc_id
//     WHERE item_d.fcnote_id =:fcnote_id ORDER BY item_d.dlc_id ASC
// // opd-er-nurse-focus-note-edit.php
// SELECT opd_er_focus_note.*, opd_er_focus_list.smp_id, opd_er_focus_list.focus_id, opd_er_focus_list.focus_text, ipd_tmp_focus.focus_name
//     FROM kphis.opd_er_focus_note
//     LEFT JOIN kphis.opd_er_focus_list ON opd_er_focus_note.fclist_id = opd_er_focus_list.fclist_id
//     LEFT JOIN kphis.ipd_tmp_focus ON opd_er_focus_list.focus_id = ipd_tmp_focus.focus_id
//     WHERE opd_er_focus_note.fcnote_id = :fcnote_id AND opd_er_focus_note.opd_er_order_master_id = :opd_er_order_master_id
// // we merge to one
// SELECT f.*,fcl.smp_id,fcl.focus_id,fcl.focus_text,tf.focus_name,k.`name` AS user_name,k.entryposition,k.doctorcode,
//     (SELECT GROUP_CONCAT(CONCAT(ti.intvt_id,'^',ti.intvt_name) ORDER BY ii.intvt_id ASC SEPARATOR '|')
//         FROM kphis.opd_er_focus_note_intvt_item ii LEFT JOIN kphis.ipd_tmp_intvt ti ON ti.intvt_id=ii.intvt_id
//         WHERE ii.fcnote_id=f.fcnote_id) AS intvts,
//     (SELECT GROUP_CONCAT(CONCAT(td.dlc_id,'^',td.dlc_name) ORDER BY di.dlc_id ASC SEPARATOR '|')
//         FROM kphis.opd_er_focus_note_dlc_item di LEFT JOIN kphis.ipd_tmp_dlc td ON td.dlc_id=di.dlc_id
//         WHERE di.fcnote_id=f.fcnote_id) AS dlcs
// FROM kphis.opd_er_focus_note f
//     LEFT JOIN kphis.opd_er_focus_list fcl ON f.fclist_id=fcl.fclist_id
//     LEFT JOIN kphis.ipd_tmp_focus tf ON fcl.focus_id=tf.focus_id
//     LEFT JOIN hos.opduser k ON k.loginname=f.create_user
// WHERE f.opd_er_order_master_id=? ORDER BY f.fcnote_date ASC,f.fcnote_time ASC,f.fcnote_id ASC;
/// opd_er_order_master_id, (start_date), (end_date), (fcnote_id)
pub fn select_focus_note(params: &FocusNoteParams, hosxp: &str, kphis: &str, kphis_extra: &str) -> String {
    let date = match (params.start_date.is_some(), params.end_date.is_some()) {
        (true, true) => " AND f.fcnote_date BETWEEN ? AND ? ",
        (false, false) => "",
        _ => " AND f.fcnote_date=? ",
    };
    let fclist_id = match params.fclist_id {
        Some(0) => String::from(" AND f.fclist_id IS NULL "),
        Some(u) => [" AND f.fclist_id=", &u.to_string(), " "].concat(),
        None => String::new(),
    };
    let fcnote_id = if params.fcnote_id.is_some() {" AND f.fcnote_id=? "} else {""};

    [
        "SELECT f.*,fcl.smp_id,fcl.focus_id,fcl.focus_text,tf.focus_name,d.`name` AS user_name,d.licenseno,k.entryposition,k.doctorcode,\
            (SELECT GROUP_CONCAT(CONCAT(ti.intvt_id,'^',ti.intvt_name) ORDER BY ii.intvt_id ASC SEPARATOR '|') \
                FROM ",kphis,".opd_er_focus_note_intvt_item ii LEFT JOIN ",kphis,".ipd_tmp_intvt ti ON ti.intvt_id=ii.intvt_id \
                WHERE ii.fcnote_id=f.fcnote_id) AS intvts,\
            (SELECT GROUP_CONCAT(CONCAT(td.dlc_id,'^',td.dlc_name) ORDER BY di.dlc_id ASC SEPARATOR '|') \
                FROM ",kphis,".opd_er_focus_note_dlc_item di LEFT JOIN ",kphis,".ipd_tmp_dlc td ON td.dlc_id=di.dlc_id \
                WHERE di.fcnote_id=f.fcnote_id) AS dlcs,\
            (SELECT GROUP_CONCAT(i.`path` ORDER BY u.image_usage_id) FROM ",kphis_extra,".image AS i \
  		        LEFT JOIN ",kphis_extra,".image_usage AS u ON u.image_id=i.image_id WHERE u.usage_id=6 AND u.usage_key_id=f.fcnote_id) AS a_imgs,\
            (SELECT GROUP_CONCAT(i.`path` ORDER BY u.image_usage_id) FROM ",kphis_extra,".image AS i \
  		        LEFT JOIN ",kphis_extra,".image_usage AS u ON u.image_id=i.image_id WHERE u.usage_id=8 AND u.usage_key_id=f.fcnote_id) AS e_imgs \
        FROM ",kphis,".opd_er_focus_note f \
            LEFT JOIN ",kphis,".opd_er_focus_list fcl ON f.fclist_id=fcl.fclist_id \
            LEFT JOIN ",kphis,".ipd_tmp_focus tf ON fcl.focus_id=tf.focus_id \
            LEFT JOIN ",hosxp,".opduser k ON k.loginname=f.create_user \
            LEFT JOIN ",hosxp,".doctor d ON d.`code`=k.doctorcode \
        WHERE f.opd_er_order_master_id=? ",date,&fclist_id,fcnote_id,
        "ORDER BY f.fcnote_date ASC,f.fcnote_time ASC,f.fcnote_id ASC;"
    ].concat()
}

/// opd_er_order_master_id
pub fn select_focus_note_only(kphis: &str) -> String {
    [
        "SELECT * FROM ",kphis,".opd_er_focus_note WHERE opd_er_order_master_id=? ORDER BY fcnote_date ASC,fcnote_time ASC,fcnote_id ASC;"
    ].concat()
}

/// fcnote_id
pub fn select_focus_note_intvt_item_only(kphis: &str) -> String {
    [
        "SELECT * FROM ",kphis,".opd_er_focus_note_intvt_item WHERE fcnote_id=? ORDER BY fcnote_intvt_item_id ASC;"
    ].concat()
}

/// fcnote_id
pub fn select_focus_note_dlc_item_only(kphis: &str) -> String {
    [
        "SELECT * FROM ",kphis,".opd_er_focus_note_dlc_item WHERE fcnote_id=? ORDER BY fcnote_dlc_item_id ASC;"
    ].concat()
}

// // // opd-er-nurse-focus-note-edit.php

// SELECT item_in.intvt_id
//     FROM kphis.opd_er_focus_note_intvt_item item_in
//     WHERE item_in.fcnote_id =:fcnote_id ORDER BY item_in.intvt_id ASC

// SELECT item_d.dlc_id
//     FROM kphis.opd_er_focus_note_dlc_item item_d
// WHERE item_d.fcnote_id =:fcnote_id ORDER BY item_d.dlc_id ASC

// // // opd-er-nurse-focus-select-date.php

// SELECT DISTINCT fn.fcnote_date
//     FROM kphis.opd_er_focus_note fn WHERE fn.opd_er_order_master_id = :opd_er_order_master_id
// UNION
//     SELECT DATE(NOW()) AS fcnote_date ORDER BY fcnote_date DESC

// // // opd-er-nurse-focus-note-patient_type.php

// SELECT fcnote_patient_type
//     FROM kphis.opd_er_focus_note fcn
// WHERE opd_er_order_master_id=:opd_er_order_master_id AND fcn.fcnote_patient_type IS NOT NULL
// ORDER BY fcn.fcnote_date DESC, fcn.fcnote_time DESC LIMIT 1

// // // opd-er-nurse-focus-note-save.php

// INSERT INTO kphis.opd_er_focus_note (general_symptoms,fclist_id,assessment,intvt_text,evalution,dlc_text,other,opd_er_order_master_id,
//     fcnote_date,fcnote_time,fcnote_patient_type,
//     create_user,create_datetime,update_user,update_datetime,version)
// VALUES (?,?,?,?,?,?,?,?,?,?,?,?,NOW(),?,NOW(),?);
/// general_symptoms, fclist_id, assessment, intvt_text, evalution, dlc_text, other, opd_er_order_master_id, fcnote_date, fcnote_time, fcnote_patient_type, loginname, loginname
pub fn insert_focus_note(kphis: &str) -> String {
    [
        "INSERT INTO ",kphis,".opd_er_focus_note (general_symptoms,fclist_id,assessment,intvt_text,evalution,dlc_text,other,opd_er_order_master_id,\
            fcnote_date,fcnote_time,fcnote_patient_type",TABLE_CREATE_COLUMNS,") \
        VALUES (?,?,?,?,?,?,?,?,?,?,?",TABLE_CREATE_PREPARED,");"
    ].concat()
}

/// general_symptoms,fclist_id,assessment,intvt_text,evalution,dlc_text,other,opd_er_order_master_id,fcnote_date,fcnote_time,fcnote_patient_type,create_user,create_datetime,update_user,update_datetime,version
pub fn insert_focus_note_only(kphis: &str) -> String {
    [
        "INSERT INTO ",kphis,".opd_er_focus_note (general_symptoms,fclist_id,assessment,intvt_text,evalution,dlc_text,other,opd_er_order_master_id,\
            fcnote_date,fcnote_time,fcnote_patient_type",TABLE_CREATE_COLUMNS,") \
        VALUES (?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?);"
    ].concat()
}

// INSERT INTO history_opd_er_focus_note
//     SELECT NULL,NOW(),'I',:update_user, opd_er_focus_note.*
//     FROM opd_er_focus_note WHERE fcnote_id = :fcnote_id

// INSERT INTO kphis.opd_er_focus_note_intvt_item (fcnote_id,intvt_id,create_user,create_datetime,update_user,update_datetime,version)
//     VALUES (:fcnote_id,:intvt_id,:create_user,NOW(),:update_user,NOW(),:version)
pub fn insert_intvt_items(
    intvts: &[u32],
    fcnote_id: u32,
    user: &str,
    version: i32,
    kphis: &str,
) -> String {
    let intvt = intvts.iter().map(|id| ["(",&fcnote_id.to_string(),",",&id.to_string(),",'",user,"',NOW(),'",user,"',NOW(),",&version.to_string(),")"].concat()).collect::<Vec<String>>().join(",");
    [
        "INSERT INTO ",kphis,".opd_er_focus_note_intvt_item (fcnote_id,intvt_id",TABLE_CREATE_COLUMNS,") \
        VALUES ",&intvt
    ].concat()
}

pub fn insert_intvt_items_only(
    intvts: &[FocusNoteIntvtItemOnly],
    fcnote_id: u32,
    kphis: &str,
) -> String {
    let values = intvts.iter().map(|item| {
        [
            "(",&fcnote_id.to_string(),",",
            &item.intvt_id.map(|s| s.to_string()).unwrap_or(String::from("NULL")),",'",
            &item.create_user,"','",
            &item.create_datetime.to_string(),"','",
            &item.update_user,"','",
            &item.update_datetime.to_string(),"',",
            &item.version.to_string(),")"
        ].concat()
    }).collect::<Vec<String>>().join(",");
    [
        "INSERT INTO ",kphis,".opd_er_focus_note_intvt_item (fcnote_id,intvt_id",TABLE_CREATE_COLUMNS,") \
        VALUES ",&values
    ].concat()
}

// INSERT INTO history_opd_er_focus_note_intvt_item
//     SELECT NULL,NOW(),'I',:update_user, opd_er_focus_note_intvt_item.*
//     FROM kphis.opd_er_focus_note_intvt_item
//     WHERE fcnote_intvt_item_id = :fcnote_intvt_item_id

// INSERT INTO kphis.opd_er_focus_note_dlc_item (fcnote_id,dlc_id,create_user,create_datetime,update_user,update_datetime,version)
//     VALUES (:fcnote_id,:dlc_id,:create_user,NOW(),:update_user,NOW(),:version)
pub fn insert_dlc_items(
    dlcs: &[u32],
    fcnote_id: u32,
    user: &str,
    version: i32,
    kphis: &str,
) -> String {
    let dlc = dlcs.iter().map(|id| ["(",&fcnote_id.to_string(),",",&id.to_string(),",'",user,"',NOW(),'",user,"',NOW(),",&version.to_string(),")"].concat()).collect::<Vec<String>>().join(",");
    [
        "INSERT INTO ",kphis,".opd_er_focus_note_dlc_item (fcnote_id,dlc_id",TABLE_CREATE_COLUMNS,") \
        VALUES ",&dlc
    ].concat()
}

pub fn insert_dlc_items_only(
    dlcs: &[FocusNoteDlcItemOnly],
    fcnote_id: u32,
    kphis: &str,
) -> String {
    let values = dlcs.iter().map(|item| {
        [
            "(",&fcnote_id.to_string(),",",
            &item.dlc_id.map(|s| s.to_string()).unwrap_or(String::from("NULL")),",'",
            &item.create_user,"','",
            &item.create_datetime.to_string(),"','",
            &item.update_user,"','",
            &item.update_datetime.to_string(),"',",
            &item.version.to_string(),")"
        ].concat()
    }).collect::<Vec<String>>().join(",");
    [
        "INSERT INTO ",kphis,".opd_er_focus_note_dlc_item (fcnote_id,dlc_id",TABLE_CREATE_COLUMNS,") \
        VALUES ",&values
    ].concat()
}

// INSERT INTO history_opd_er_focus_note_dlc_item
//     SELECT NULL,NOW(),'I',:update_user, opd_er_focus_note_dlc_item.*
//     FROM kphis.opd_er_focus_note_dlc_item
//     WHERE fcnote_dlc_item_id = :fcnote_dlc_item_id

// // // opd-er-nurse-focus-note-update.php

// SELECT opd_er_focus_note.version FROM kphis.opd_er_focus_note WHERE fcnote_id = :fcnote_id

// UPDATE kphis.opd_er_focus_note SET general_symptoms=?,fclist_id=?,assessment=?,intvt_text=?,evalution=?,dlc_text=?,other=?,opd_er_order_master_id=?,
//     fcnote_date=?,fcnote_time=?,fcnote_patient_type=?,update_user=?,update_datetime=NOW(),version=? WHERE fcnote_id=?;
/// general_symptoms, fclist_id, assessment, intvt_text, evalution, dlc_text, other, opd_er_order_master_id, fcnote_date, fcnote_time, fcnote_patient_type, loginname, fcnote_id, version
pub fn update_focus_note(kphis: &str) -> String {
    [
        "UPDATE ",kphis,".opd_er_focus_note SET general_symptoms=?,fclist_id=?,assessment=?,intvt_text=?,evalution=?,dlc_text=?,other=?,opd_er_order_master_id=?,\
        fcnote_date=?,fcnote_time=?,fcnote_patient_type=?",TABLE_UPDATE_SET," WHERE fcnote_id=? AND version=?;"
    ].concat()
}

// INSERT INTO history_opd_er_focus_note
//     SELECT NULL,NOW(),'U',:update_user, opd_er_focus_note.* FROM opd_er_focus_note
//     WHERE fcnote_id = :fcnote_id

// SELECT COUNT(*) AS count_item_intvt FROM kphis.opd_er_focus_note_intvt_item WHERE fcnote_id=:fcnote_id

// DELETE FROM kphis.opd_er_focus_note_intvt_item WHERE fcnote_id=:fcnote_id
/// fcnote_id, version
pub fn delete_intvt_item(kphis: &str) -> String {
    [
        "DELETE FROM ",kphis,".opd_er_focus_note_intvt_item WHERE fcnote_id=? AND version=?;"
    ].concat()
}

// INSERT INTO kphis.opd_er_focus_note_intvt_item (fcnote_id,intvt_id,create_user,create_datetime,update_user,update_datetime,version)
//     VALUES (:fcnote_id,:intvt_id,:create_user,NOW(),:update_user,NOW(),:version)

// INSERT INTO history_opd_er_focus_note_intvt_item
//     SELECT NULL,NOW(),'U',:update_user, opd_er_focus_note_intvt_item.*
//     FROM kphis.opd_er_focus_note_intvt_item
//     WHERE fcnote_intvt_item_id = :fcnote_intvt_item_id

// SELECT COUNT(*) AS count_item_dlc FROM kphis.opd_er_focus_note_dlc_item WHERE fcnote_id=:fcnote_id

// DELETE FROM kphis.opd_er_focus_note_dlc_item WHERE fcnote_id=:fcnote_id
/// fcnote_id, version
pub fn delete_dlc_item(kphis: &str) -> String {
    [
        "DELETE FROM ",kphis,".opd_er_focus_note_dlc_item WHERE fcnote_id=? AND version=?;"
    ].concat()
}

// INSERT INTO kphis.opd_er_focus_note_dlc_item (fcnote_id,dlc_id,create_user,create_datetime,update_user,update_datetime,version)
//     VALUES (:fcnote_id,:dlc_id,:create_user,NOW(),:update_user,NOW(),:version)

// INSERT INTO history_opd_er_focus_note_dlc_item
//     SELECT NULL,NOW(),'U',:update_user, opd_er_focus_note_dlc_item.*
//     FROM kphis.opd_er_focus_note_dlc_item
//     WHERE fcnote_dlc_item_id = :fcnote_dlc_item_id

// // // opd-er-nurse-focus-note-delete.php

// SELECT opd_er_focus_note.version FROM kphis.opd_er_focus_note WHERE fcnote_id = :fcnote_id

// INSERT INTO kphis.history_opd_er_focus_note
//     SELECT NULL,NOW(),'D',:update_user, opd_er_focus_note.*
//     FROM kphis.opd_er_focus_note
//     WHERE opd_er_focus_note.fcnote_id = :fcnote_id

// DELETE FROM kphis.opd_er_focus_note WHERE fcnote_id = :fcnote_id
/// fcnote_id, version
pub fn delete_focus_note(kphis: &str) -> String {
    [
        "DELETE FROM ",kphis,".opd_er_focus_note WHERE fcnote_id=? AND version=?;"
    ].concat()
}

// INSERT INTO history_opd_er_focus_note_intvt_item
//     SELECT NULL,NOW(),'D',:update_user, opd_er_focus_note_intvt_item.*
//     FROM kphis.opd_er_focus_note_intvt_item
//     WHERE fcnote_id = :fcnote_id

// DELETE FROM kphis.opd_er_focus_note_intvt_item WHERE fcnote_id=:fcnote_id

// INSERT INTO history_opd_er_focus_note_dlc_item
//     SELECT NULL,NOW(),'D',:update_user, opd_er_focus_note_dlc_item.*
//     FROM kphis.opd_er_focus_note_dlc_item
//     WHERE fcnote_id = :fcnote_id

// DELETE FROM kphis.opd_er_focus_note_dlc_item WHERE fcnote_id=:fcnote_id
