use kphis_model::index_action::IndexActionOnly;

use crate::{TABLE_CREATE_COLUMNS, TABLE_CREATE_PREPARED, TABLE_UPDATE_SET};

// SELECT a.*,vs.vs_id,d1.`name` AS action_person_1_doctor_name,d2.`name` AS action_person_2_doctor_name,
//     u1.entryposition AS action_person_1_entryposition,u2.entryposition AS action_person_2_entryposition,
//     d3.`name` AS check_person_name,u3.entryposition AS check_person_entryposition,
//     (SELECT EXISTS(SELECT * FROM kphis_extra.ipd_nurse_index_monitor WHERE action_id=a.action_id)) AS has_monitor
// FROM kphis.ipd_nurse_index_action a
//    LEFT JOIN kphis.ipd_vs_vital_sign vs ON vs.action_id=a.action_id
//    LEFT JOIN hos.doctor d1 ON d1.code=a.action_person_1
//    LEFT JOIN hos.doctor d2 ON d2.code=a.action_person_2
//    LEFT JOIN hos.doctor d3 ON d3.code=a.check_person
//    LEFT JOIN hos.opduser u1 ON u1.doctorcode=a.action_person_1 AND (u1.account_disable IS NULL OR u1.account_disable='N')
//    LEFT JOIN hos.opduser u2 ON u2.doctorcode=a.action_person_2 AND (u2.account_disable IS NULL OR u2.account_disable='N')
//    LEFT JOIN hos.opduser u3 ON u3.doctorcode=a.check_person AND (u3.account_disable IS NULL OR u3.account_disable='N')
// WHERE a.plan_id=? ORDER BY a.action_date,a.action_time;
/// plan_id
pub fn select_index_action(hosxp: &str, kphis: &str, kphis_extra: &str) -> String {
    [
        "SELECT a.*,vs.vs_id,d1.`name` AS action_person_1_name,d2.`name` AS action_person_2_name,d3.`name` AS check_person_name,\
            (SELECT entryposition FROM ",hosxp,".opduser WHERE doctorcode=a.action_person_1 AND (account_disable IS NULL OR account_disable='N') LIMIT 1) AS action_person_1_entryposition,\
            (SELECT entryposition FROM ",hosxp,".opduser WHERE doctorcode=a.action_person_2 AND (account_disable IS NULL OR account_disable='N') LIMIT 1) AS action_person_2_entryposition,\
            (SELECT entryposition FROM ",hosxp,".opduser WHERE doctorcode=a.check_person AND (account_disable IS NULL OR account_disable='N') LIMIT 1) AS check_person_entryposition,\
            (SELECT EXISTS(SELECT * FROM ",kphis_extra,".ipd_nurse_index_monitor WHERE action_id=a.action_id)) AS has_monitor \
        FROM ",kphis,".ipd_nurse_index_action a \
            LEFT JOIN ",kphis,".ipd_vs_vital_sign vs ON vs.action_id=a.action_id \
            LEFT JOIN ",hosxp,".doctor d1 ON d1.code=a.action_person_1 \
            LEFT JOIN ",hosxp,".doctor d2 ON d2.code=a.action_person_2 \
            LEFT JOIN ",hosxp,".doctor d3 ON d3.code=a.check_person \
        WHERE a.plan_id=? ORDER BY a.action_date,a.action_time;"
    ].concat()
}

/// plan_id
pub fn select_index_action_only(kphis: &str, kphis_extra: &str) -> String {
    [
        "SELECT a.*,(SELECT EXISTS(SELECT * FROM ",kphis_extra,".ipd_nurse_index_monitor WHERE action_id=a.action_id)) AS has_monitor \
        FROM ",kphis,".ipd_nurse_index_action a WHERE a.plan_id=? ORDER BY a.action_id;"
    ].concat()
}

// INSERT INTO kphis.ipd_nurse_index_action (plan_id,an,action_result,action_remark,action_date,action_time,action_report_back,action_blood_had,action_person_1,action_person_2,create_user,create_datetime,update_user,update_datetime,version)
// VALUES (?,?,?,?,?,?,?,?,?,?,?,?,?,NOW(),?,NOW(),1);
/// plan_id, an, check_datetime, check_person, action_result, action_remark, action_date, action_time, action_report_back, action_blood_had, action_person_1, action_person_2, loginname, loginname
pub fn insert_index_action(kphis: &str) -> String {
    [
        "INSERT INTO ",kphis,".ipd_nurse_index_action (plan_id,an,check_datetime,check_person,action_result,action_remark,action_date,action_time,action_report_back,action_blood_had,action_person_1,action_person_2",TABLE_CREATE_COLUMNS,") \
        VALUES (?,?,?,?,?,?,?,?,?,?,?,?",TABLE_CREATE_PREPARED,");"
    ].concat()
}

pub fn insert_index_actions_only(plan_id: u32, an: &str, index_actions_only: &[IndexActionOnly], kphis: &str) -> String {
    let values = index_actions_only.iter().map(|item| {
        [
            "(",&plan_id.to_string(),",'",
            an,"',",
            &item.check_datetime.map(|s| ["'", &s.to_string(), "'"].concat()).unwrap_or(String::from("NULL")),",",
            &item.check_person.as_ref().map(|s| ["'", s, "'"].concat()).unwrap_or(String::from("NULL")),",",
            &item.action_result.as_ref().map(|s| ["'", s, "'"].concat()).unwrap_or(String::from("NULL")),",",
            &item.action_remark.as_ref().map(|s| ["'", s, "'"].concat()).unwrap_or(String::from("NULL")),",",
            &item.action_date.map(|s| ["'", &s.to_string(), "'"].concat()).unwrap_or(String::from("NULL")),",",
            &item.action_time.map(|s| ["'", &s.to_string(), "'"].concat()).unwrap_or(String::from("NULL")),",",
            &item.action_report_back.as_ref().map(|s| ["'", s, "'"].concat()).unwrap_or(String::from("NULL")),",",
            &item.action_blood_had.as_ref().map(|s| ["'", s, "'"].concat()).unwrap_or(String::from("NULL")),",",
            &item.action_person_1.as_ref().map(|s| ["'", s, "'"].concat()).unwrap_or(String::from("NULL")),",",
            &item.action_person_2.as_ref().map(|s| ["'", s, "'"].concat()).unwrap_or(String::from("NULL")),",'",
            &item.create_user,"','",
            &item.create_datetime.to_string(),"','",
            &item.update_user,"','",
            &item.update_datetime.to_string(),"',",
            &item.version.to_string(),")"
        ].concat()
    }).collect::<Vec<String>>().join(",");
    [
        "INSERT INTO ",kphis,".ipd_nurse_index_action (plan_id,an,check_datetime,check_person,action_result,action_remark,action_date,action_time,action_report_back,action_blood_had,action_person_1,action_person_2",TABLE_CREATE_COLUMNS,") \
        VALUES ", &values,";"
    ].concat()
}

// UPDATE kphis.ipd_nurse_index_action
// SET check_datetime=?,check_person=?,action_result=?,action_remark=?,action_date=?,action_time=?,action_report_back=?,action_blood_had=?,action_person_1=?,action_person_2=?,update_user=?,update_datetime=NOW(),version=(version+1)
// WHERE action_id=?;
/// check_datetime, check_person, action_result, action_remark, action_date, action_time, action_report_back, action_blood_had, action_person_1, action_person_2, loginname, action_id
pub fn update_index_action(kphis: &str) -> String {
    [
        "UPDATE ",kphis,".ipd_nurse_index_action \
        SET check_datetime=?,check_person=?,action_result=?,action_remark=?,action_date=?,action_time=?,action_report_back=?,action_blood_had=?,action_person_1=?,action_person_2=?",TABLE_UPDATE_SET,
        " WHERE action_id=?;"
    ].concat()
}

// DELETE FROM ",kphis,".ipd_nurse_index_action WHERE action_id=?;
/// action_id
pub fn delete_index_action(kphis: &str) -> String {
    [
        "DELETE FROM ",kphis,".ipd_nurse_index_action WHERE action_id=?;"
    ].concat()
}