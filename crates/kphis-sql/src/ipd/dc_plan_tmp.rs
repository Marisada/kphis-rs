use kphis_model::ipd::dc_plan_tmp::DcPlanTmpParams;

use crate::{TABLE_CREATE_COLUMNS, TABLE_CREATE_PREPARED, TABLE_UPDATE_SET};


// SELECT * FROM kphis_extra.ipd_dc_plan_tmp_dx WHERE dx_id=?;
/// (dx_id)
pub fn select_dx(params: &DcPlanTmpParams, kphis_extra: &str) -> String {
    let where_id = if params.id.is_some() {" WHERE dx_id=?"} else {""};
    [
        "SELECT * FROM ",kphis_extra,".ipd_dc_plan_tmp_dx",where_id,";"
    ].concat()
}

// INSERT INTO kphis_extra.ipd_dc_plan_tmp_dx (dx_name,dx_knowledge,dx_revisit,dx_prevention,create_user,create_datetime,update_user,update_datetime,version)
// VALUES (?,?,?,?,?,NOW(),?,NOW(),1);
/// dx_name, dx_knowledge, dx_revisit, dx_prevention, loginname, loginname
pub fn insert_dx(kphis_extra: &str) -> String {
    [
        "INSERT INTO ",kphis_extra,".ipd_dc_plan_tmp_dx (dx_name,dx_knowledge,dx_revisit,dx_prevention",TABLE_CREATE_COLUMNS,") \
        VALUES (?,?,?,?",TABLE_CREATE_PREPARED,");"
    ].concat()
}

// UPDATE kphis_extra.ipd_dc_plan_tmp_dx SET dx_name=?,dx_knowledge=?,dx_revisit=?,dx_prevention=?,update_user=?,update_datetime=NOW(),version=(version+1) WHERE dx_id=?;
/// dx_name, dx_knowledge, dx_revisit, dx_prevention, loginname, dx_id
pub fn update_dx(kphis_extra: &str) -> String {
    [
        "UPDATE ",kphis_extra,".ipd_dc_plan_tmp_dx SET dx_name=?,dx_knowledge=?,dx_revisit=?,dx_prevention=?",TABLE_UPDATE_SET," WHERE dx_id=?;"
    ].concat()
}

// DELETE FROM kphis_extra.ipd_dc_plan_tmp_dx WHERE update_user=? AND dx_id=?
//    AND NOT EXISTS(SELECT * FROM kphis_extra.ipd_dc_plan WHERE dx_id=?)
//    AND NOT EXISTS(SELECT * FROM kphis_extra.opd_er_dc_plan WHERE dx_id=?);"
/// loginname, dx_id, dx_id, dx_id
pub fn delete_dx(kphis_extra: &str) -> String {
    [
        "DELETE FROM ",kphis_extra,".ipd_dc_plan_tmp_dx WHERE update_user=? AND dx_id=? \
            AND NOT EXISTS(SELECT * FROM ",kphis_extra,".ipd_dc_plan WHERE dx_id=?) \
            AND NOT EXISTS(SELECT * FROM ",kphis_extra,".opd_er_dc_plan WHERE dx_id=?);"
    ].concat()
}

// SELECT * FROM kphis_extra.ipd_dc_plan_tmp_med WHERE med_id=?;
/// (med_id)
pub fn select_med(params: &DcPlanTmpParams, kphis_extra: &str) -> String {
    let where_id = if params.id.is_some() {" WHERE med_id=?"} else {""};
    [
        "SELECT * FROM ",kphis_extra,".ipd_dc_plan_tmp_med",where_id,";"
    ].concat()
}

// INSERT INTO kphis_extra.ipd_dc_plan_tmp_med (med_text,create_user,create_datetime,update_user,update_datetime,version)
//     VALUES (?,?,NOW(),?,NOW(),1);
/// med_text, loginname, loginname
pub fn insert_med(kphis_extra: &str) -> String {
    [
        "INSERT INTO ",kphis_extra,".ipd_dc_plan_tmp_med (med_text",TABLE_CREATE_COLUMNS,") \
            VALUES (?",TABLE_CREATE_PREPARED,");"
    ].concat()
}

// UPDATE kphis_extra.ipd_dc_plan_tmp_med SET med_text=?,update_user=?,update_datetime=NOW(),version=(version+1) WHERE med_id=?;
/// med_text, loginname, med_id
pub fn update_med(kphis_extra: &str) -> String {
    [
        "UPDATE ",kphis_extra,".ipd_dc_plan_tmp_med SET med_text=?",TABLE_UPDATE_SET," WHERE med_id=?;"
    ].concat()
}

// DELETE FROM kphis_extra.ipd_dc_plan_tmp_med WHERE update_user=? AND med_id=?
//    AND NOT EXISTS(SELECT * FROM kphis_extra.ipd_dc_plan_med_item WHERE med_id=?)
//    AND NOT EXISTS(SELECT * FROM kphis_extra.opd_er_dc_plan_med_item WHERE med_id=?);
/// loginname, med_id, med_id, med_id
pub fn delete_med(kphis_extra: &str) -> String {
    [
        "DELETE FROM ",kphis_extra,".ipd_dc_plan_tmp_med WHERE update_user=? AND med_id=? \
            AND NOT EXISTS(SELECT * FROM ",kphis_extra,".ipd_dc_plan_med_item WHERE med_id=?) \
            AND NOT EXISTS(SELECT * FROM ",kphis_extra,".opd_er_dc_plan_med_item WHERE med_id=?);"
    ].concat()
}

// SELECT * FROM kphis_extra.ipd_dc_plan_tmp_env WHERE env_id=?;
/// (env_id)
pub fn select_env(params: &DcPlanTmpParams, kphis_extra: &str) -> String {
    let where_id = if params.id.is_some() {" WHERE env_id=?"} else {""};
    [
        "SELECT * FROM ",kphis_extra,".ipd_dc_plan_tmp_env",where_id,";"
    ].concat()
}

// INSERT INTO kphis_extra.ipd_dc_plan_tmp_env (env_text,create_user,create_datetime,update_user,update_datetime,version)
//     VALUES (?,?,NOW(),?,NOW(),1);
/// env_text, loginname, loginname
pub fn insert_env(kphis_extra: &str) -> String {
    [
        "INSERT INTO ",kphis_extra,".ipd_dc_plan_tmp_env (env_text",TABLE_CREATE_COLUMNS,") \
            VALUES (?",TABLE_CREATE_PREPARED,");"
    ].concat()
}

// UPDATE kphis_extra.ipd_dc_plan_tmp_env SET env_text=?,update_user=?,update_datetime=NOW(),version=(version+1) WHERE env_id=?;
/// env_text, loginname, env_id
pub fn update_env(kphis_extra: &str) -> String {
    [
        "UPDATE ",kphis_extra,".ipd_dc_plan_tmp_env SET env_text=?",TABLE_UPDATE_SET," WHERE env_id=?;"
    ].concat()
}

// DELETE FROM kphis_extra.ipd_dc_plan_tmp_env WHERE update_user=? AND env_id=?
//    AND NOT EXISTS(SELECT * FROM kphis_extra.ipd_dc_plan_env_item WHERE env_id=?)
//    AND NOT EXISTS(SELECT * FROM kphis_extra.opd_er_dc_plan_env_item WHERE env_id=?);
/// loginname, env_id, env_id, env_id
pub fn delete_env(kphis_extra: &str) -> String {
    [
        "DELETE FROM ",kphis_extra,".ipd_dc_plan_tmp_env WHERE update_user=? AND env_id=? \
            AND NOT EXISTS(SELECT * FROM ",kphis_extra,".ipd_dc_plan_env_item WHERE env_id=?) \
            AND NOT EXISTS(SELECT * FROM ",kphis_extra,".opd_er_dc_plan_env_item WHERE env_id=?);"
    ].concat()
}

// SELECT * FROM kphis_extra.ipd_dc_plan_tmp_tx WHERE tx_id=?;
/// (tx_id)
pub fn select_tx(params: &DcPlanTmpParams, kphis_extra: &str) -> String {
    let where_id = if params.id.is_some() {" WHERE tx_id=?"} else {""};
    [
        "SELECT * FROM ",kphis_extra,".ipd_dc_plan_tmp_tx",where_id,";"
    ].concat()
}

// INSERT INTO kphis_extra.ipd_dc_plan_tmp_tx (tx_text,create_user,create_datetime,update_user,update_datetime,version)
//     VALUES (?,?,NOW(),?,NOW(),1);
/// tx_text, loginname, loginname
pub fn insert_tx(kphis_extra: &str) -> String {
    [
        "INSERT INTO ",kphis_extra,".ipd_dc_plan_tmp_tx (tx_text",TABLE_CREATE_COLUMNS,") \
            VALUES (?",TABLE_CREATE_PREPARED,");"
    ].concat()
}

// UPDATE kphis_extra.ipd_dc_plan_tmp_tx SET tx_text=?,update_user=?,update_datetime=NOW(),version=(version+1) WHERE tx_id=?;
/// tx_text, loginname, tx_id
pub fn update_tx(kphis_extra: &str) -> String {
    [
        "UPDATE ",kphis_extra,".ipd_dc_plan_tmp_tx SET tx_text=?",TABLE_UPDATE_SET," WHERE tx_id=?;"
    ].concat()
}

// DELETE FROM kphis_extra.ipd_dc_plan_tmp_tx WHERE update_user=? AND tx_id=?
//    AND NOT EXISTS(SELECT * FROM kphis_extra.ipd_dc_plan_tx_item WHERE tx_id=?)
//    AND NOT EXISTS(SELECT * FROM kphis_extra.opd_er_dc_plan_tx_item WHERE tx_id=?);
/// loginname, tx_id, tx_id, tx_id
pub fn delete_tx(kphis_extra: &str) -> String {
    [
        "DELETE FROM ",kphis_extra,".ipd_dc_plan_tmp_tx WHERE update_user=? AND tx_id=? \
            AND NOT EXISTS(SELECT * FROM ",kphis_extra,".ipd_dc_plan_tx_item WHERE tx_id=?) \
            AND NOT EXISTS(SELECT * FROM ",kphis_extra,".opd_er_dc_plan_tx_item WHERE tx_id=?);"
    ].concat()
}

// SELECT * FROM kphis_extra.ipd_dc_plan_tmp_diet WHERE diet_id=?;
/// (diet_id)
pub fn select_diet(params: &DcPlanTmpParams, kphis_extra: &str) -> String {
    let where_id = if params.id.is_some() {" WHERE diet_id=?"} else {""};
    [
        "SELECT * FROM ",kphis_extra,".ipd_dc_plan_tmp_diet",where_id,";"
    ].concat()
}

// INSERT INTO kphis_extra.ipd_dc_plan_tmp_diet (diet_text,create_user,create_datetime,update_user,update_datetime,version)
//     VALUES (?,?,NOW(),?,NOW(),1);
/// diet_text, loginname, loginname
pub fn insert_diet(kphis_extra: &str) -> String {
    [
        "INSERT INTO ",kphis_extra,".ipd_dc_plan_tmp_diet (diet_text",TABLE_CREATE_COLUMNS,") \
            VALUES (?",TABLE_CREATE_PREPARED,");"
    ].concat()
}

// UPDATE kphis_extra.ipd_dc_plan_tmp_diet SET diet_text=?,update_user=?,update_datetime=NOW(),version=(version+1) WHERE diet_id=?;
/// diet_text, loginname, diet_id
pub fn update_diet(kphis_extra: &str) -> String {
    [
        "UPDATE ",kphis_extra,".ipd_dc_plan_tmp_diet SET diet_text=?",TABLE_UPDATE_SET," WHERE diet_id=?;"
    ].concat()
}

// DELETE FROM kphis_extra.ipd_dc_plan_tmp_diet WHERE update_user=? AND diet_id=?
//    AND NOT EXISTS(SELECT * FROM kphis_extra.ipd_dc_plan_diet_item WHERE diet_id=?)
//    AND NOT EXISTS(SELECT * FROM kphis_extra.opd_er_dc_plan_diet_item WHERE diet_id=?);
/// loginname, diet_id, diet_id, diet_id
pub fn delete_diet(kphis_extra: &str) -> String {
    [
        "DELETE FROM ",kphis_extra,".ipd_dc_plan_tmp_diet WHERE update_user=? AND diet_id=? \
            AND NOT EXISTS(SELECT * FROM ",kphis_extra,".ipd_dc_plan_diet_item WHERE diet_id=?) \
            AND NOT EXISTS(SELECT * FROM ",kphis_extra,".opd_er_dc_plan_diet_item WHERE diet_id=?);"
    ].concat()
}