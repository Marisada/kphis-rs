use crate::{TABLE_CREATE_COLUMNS, TABLE_CREATE_PREPARED, TABLE_UPDATE_SET};

// SELECT plan_id,order_item_id,opd_er_order_master_id,plan_detail,plan_date,plan_time,plan_sch_type
// FROM kphis.opd_er_nurse_index_plan
// WHERE order_item_id=? ORDER BY plan_date,plan_time;
/// order_item_id
pub fn select_index_plan_by_order_item_id(kphis: &str) -> String {
    [
        "SELECT plan_id,order_item_id,opd_er_order_master_id,plan_detail,plan_date,plan_time,plan_sch_type \
        FROM ",kphis,".opd_er_nurse_index_plan \
        WHERE order_item_id=? ORDER BY plan_date,plan_time;"
    ].concat()
}

// SELECT plan_id,order_item_id,opd_er_order_master_id,plan_detail,plan_date,plan_time,plan_sch_type
// FROM kphis.opd_er_nurse_index_plan
// WHERE opd_er_order_master_id=? AND order_item_id IS NULL ORDER BY plan_date,plan_time;
/// opd_er_order_master_id
pub fn select_index_plan_without_order_item_id(kphis: &str) -> String {
    [
        "SELECT plan_id,order_item_id,opd_er_order_master_id,plan_detail,plan_date,plan_time,plan_sch_type \
        FROM ",kphis,".opd_er_nurse_index_plan \
        WHERE opd_er_order_master_id=? AND order_item_id IS NULL ORDER BY plan_date,plan_time;"
    ].concat()
}

/// order_item_id
pub fn select_index_plan_only(kphis: &str) -> String {
    [
        "SELECT * FROM ",kphis,".opd_er_nurse_index_plan WHERE order_item_id=? ORDER BY plan_id;"
    ].concat()
}

// // opd-er-nurse-index-plan-action-delete.php
// DELETE kphis.opd_er_nurse_index_plan, kphis.opd_er_nurse_index_action
// FROM kphis.opd_er_nurse_index_plan LEFT JOIN kphis.opd_er_nurse_index_action ON kphis.opd_er_nurse_index_plan.plan_id = kphis.opd_er_nurse_index_action.plan_id
// WHERE kphis.opd_er_nurse_index_plan.plan_id = ?;
// *** cannot use alias in delete `https://bugs.mysql.com/bug.php?id=82189` ***
/// plan_id
pub fn delete_index_plan(kphis: &str) -> String {
    [
        "DELETE ",kphis,".opd_er_nurse_index_plan, ",kphis,".opd_er_nurse_index_action \
        FROM ",kphis,".opd_er_nurse_index_plan \
            LEFT JOIN ",kphis,".opd_er_nurse_index_action ON ",kphis,".opd_er_nurse_index_plan.plan_id = ",kphis,".opd_er_nurse_index_action.plan_id \
        WHERE ",kphis,".opd_er_nurse_index_plan.plan_id = ?;"
    ].concat()
}

// // opd-er-nurse-index-plan-action-save.php
// INSERT INTO kphis.opd_er_nurse_index_plan (order_item_id,opd_er_order_master_id,plan_detail,plan_date,plan_time,plan_sch_type,create_user,create_datetime,update_user,update_datetime,version)
// VALUES (?,?,?,?,?,?,?,NOW(),?,NOW(),1);
/// order_item_id, opd_er_order_master_id, plan_detail, plan_date, plan_time, plan_sch_type, loginname, loginname
pub fn insert_index_plan(kphis: &str) -> String {
    [
        "INSERT INTO ",kphis,".opd_er_nurse_index_plan (order_item_id,opd_er_order_master_id,plan_detail,plan_date,plan_time,plan_sch_type",TABLE_CREATE_COLUMNS,") \
        VALUES (?,?,?,?,?,?",TABLE_CREATE_PREPARED,");"
    ].concat()
}

// pub fn insert_index_plans_only(order_item_id: u32, opd_er_order_master_id: u32, index_plans_only: &[IndexPlanOnly], kphis: &str) -> String {
//     let values = index_plans_only.iter().map(|item| {
//         [
//             "(",&order_item_id.to_owned(),",",
//             &opd_er_order_master_id.to_owned(),",",
//             &item.plan_detail.as_ref().map(|s| ["'", s, "'"].concat()).unwrap_or(String::from("NULL")),",",
//             &item.plan_date.map(|s| ["'", &s.to_owned(), "'"].concat()).unwrap_or(String::from("NULL")),",",
//             &item.plan_time.map(|s| ["'", &s.to_owned(), "'"].concat()).unwrap_or(String::from("NULL")),",",
//             &item.plan_sch_type.as_ref().map(|s| ["'", s, "'"].concat()).unwrap_or(String::from("NULL")),",'",
//             &item.create_user,"','",
//             &item.create_datetime.to_owned(),"','",
//             &item.update_user,"','",
//             &item.update_datetime.to_owned(),"',",
//             &item.version.to_owned(),")"
//         ].concat()
//     }).collect::<Vec<String>>().join(",");
//     [
//         "INSERT INTO ",kphis,".opd_er_nurse_index_plan (order_item_id,opd_er_order_master_id,plan_detail,plan_date,plan_time,plan_sch_type,create_user,create_datetime,update_user,update_datetime,version) \
//         VALUES ", &values,";"
//     ].concat()
// }

// UPDATE kphis.opd_er_nurse_index_plan
// SET plan_detail=?,plan_date=?,plan_time=?,plan_sch_type=?,update_user=?,update_datetime=NOW(),version=(version+1)
// WHERE plan_id=?;
/// plan_detail, plan_date, plan_time, plan_sch_type, loginname, plan_id
pub fn update_index_plan(kphis: &str) -> String {
    [
        "UPDATE ",kphis,".opd_er_nurse_index_plan \
        SET plan_detail=?,plan_date=?,plan_time=?,plan_sch_type=?",TABLE_UPDATE_SET,
        " WHERE plan_id=?;"
    ].concat()
}
