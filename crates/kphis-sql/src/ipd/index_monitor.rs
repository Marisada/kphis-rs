use kphis_model::index_monitor::IndexMonitorOnly;

use crate::{TABLE_CREATE_COLUMNS, TABLE_CREATE_PREPARED, TABLE_UPDATE_SET};

// SELECT m.*,d.`name` AS monitor_doctor_name
// FROM kphis_extra.ipd_nurse_index_monitor m
//     LEFT JOIN hos.doctor d ON d.code=m.monitor_doctor
// WHERE m.action_id=? ORDER BY m.monitor_datetime;
/// action_id
pub fn select_index_monitor(hosxp: &str, kphis_extra: &str) -> String {
    [
        "SELECT m.*,d.`name` AS monitor_doctor_name \
        FROM ",kphis_extra,".ipd_nurse_index_monitor m \
            LEFT JOIN ",hosxp,".doctor d ON d.code=m.monitor_doctor \
        WHERE m.action_id=? ORDER BY m.monitor_datetime;"
    ].concat()
}

/// action_id
pub fn select_index_monitor_only(kphis_extra: &str) -> String {
    [
        "SELECT * FROM ",kphis_extra,".ipd_nurse_index_monitor WHERE action_id=? ORDER BY monitor_id;"
    ].concat()
}

// INSERT INTO kphis_extra.ipd_nurse_index_monitor (action_id,an,monitor_datetime,monitor_doctor,monitor_abnormal,monitor_result,monitor_remark,create_user,create_datetime,update_user,update_datetime,version)
// VALUES (?,?,?,?,?,?,?,?,NOW(),?,NOW(),1);
/// action_id, an, monitor_datetime, monitor_doctor, monitor_abnormal, monitor_result, monitor_remark, loginname, loginname
pub fn insert_index_monitor(kphis_extra: &str) -> String {
    [
        "INSERT INTO ",kphis_extra,".ipd_nurse_index_monitor (action_id,an,monitor_datetime,monitor_doctor,monitor_abnormal,monitor_result,monitor_remark",TABLE_CREATE_COLUMNS,") \
        VALUES (?,?,?,?,?,?,?",TABLE_CREATE_PREPARED,");"
    ].concat()
}

pub fn insert_index_monitors_only(action_id: u32, an: &str, index_monitors_only: &[IndexMonitorOnly], kphis_extra: &str) -> String {
    let values = index_monitors_only.iter().map(|item| {
        [
            "(",&action_id.to_string(),",'",
            an,"',",
            &item.monitor_datetime.map(|s| ["'", &s.to_string(), "'"].concat()).unwrap_or(String::from("NULL")),",'",
            &item.monitor_doctor,"',",
            &item.monitor_abnormal.as_ref().map(|s| ["'", s, "'"].concat()).unwrap_or(String::from("NULL")),",",
            &item.monitor_result.as_ref().map(|s| ["'", s, "'"].concat()).unwrap_or(String::from("NULL")),",",
            &item.monitor_remark.as_ref().map(|s| ["'", s, "'"].concat()).unwrap_or(String::from("NULL")),",'",
            &item.create_user,"','",
            &item.create_datetime.to_string(),"','",
            &item.update_user,"','",
            &item.update_datetime.to_string(),"',",
            &item.version.to_string(),")"
        ].concat()
    }).collect::<Vec<String>>().join(",");
    [
        "INSERT INTO ",kphis_extra,".ipd_nurse_index_monitor (action_id,an,monitor_datetime,monitor_doctor,monitor_abnormal,monitor_result,monitor_remark",TABLE_CREATE_COLUMNS,") \
        VALUES ", &values,";"
    ].concat()
}

// UPDATE kphis_extra.ipd_nurse_index_monitor
//  SET monitor_datetime=?,monitor_doctor=?,monitor_abnormal=?,monitor_result=?,monitor_remark=?,update_user=?,update_datetime=NOW(),version=(version+1)
// WHERE monitor_id=?;
/// monitor_datetime, monitor_doctor, monitor_abnormal, monitor_result, monitor_remark, loginname, monitor_id
pub fn update_index_monitor(kphis_extra: &str) -> String {
    [
        "UPDATE ",kphis_extra,".ipd_nurse_index_monitor \
        SET monitor_datetime=?,monitor_doctor=?,monitor_abnormal=?,monitor_result=?,monitor_remark=?",TABLE_UPDATE_SET,
        " WHERE monitor_id=?;"
    ].concat()
}

// DELETE FROM kphis_extra.ipd_nurse_index_monitor WHERE monitor_id=?;
/// monitor_id
pub fn delete_index_monitor(kphis_extra: &str) -> String {
    [
        "DELETE FROM ",kphis_extra,".ipd_nurse_index_monitor WHERE monitor_id=?;"
    ].concat()
}