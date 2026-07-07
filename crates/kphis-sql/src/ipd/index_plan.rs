use crate::{TABLE_CREATE_COLUMNS, TABLE_CREATE_PREPARED, TABLE_UPDATE_SET};

// SELECT DISTINCT plan_date, (plan_date = DATE(NOW())) AS is_today FROM kphis.ipd_nurse_index_plan WHERE an=? ORDER BY plan_date ASC;
// an
pub fn select_index_plan_date(is_asc: bool, kphis: &str) -> String {
    let order_by = if is_asc { "ASC;" } else { "DESC;" };
    [
        "SELECT DISTINCT plan_date, (plan_date = DATE(NOW())) AS is_today \
        FROM ",kphis,".ipd_nurse_index_plan WHERE an=? \
        ORDER BY plan_date ",order_by
    ].concat()
}

// SELECT plan_id,order_item_id,an,plan_detail,plan_date,plan_time,plan_sch_type
// FROM kphis.ipd_nurse_index_plan
// WHERE order_item_id=? ORDER BY plan_date,plan_time;
/// order_item_id
pub fn select_index_plan_by_order_item_id(kphis: &str) -> String {
    [
        "SELECT plan_id,order_item_id,an,plan_detail,plan_date,plan_time,plan_sch_type \
        FROM ",kphis,".ipd_nurse_index_plan \
        WHERE order_item_id=? ORDER BY plan_date,plan_time;"
    ].concat()
}

/// order_item_id
pub fn select_index_plan_only(kphis: &str) -> String {
    [
        "SELECT * FROM ",kphis,".ipd_nurse_index_plan WHERE order_item_id=? ORDER BY plan_id;"
    ].concat()
}

// SELECT plan_id,order_item_id,an,plan_detail,plan_date,plan_time,plan_sch_type
// FROM kphis.ipd_nurse_index_plan
// WHERE an=? AND order_item_id IS NULL ORDER BY plan_date,plan_time;
/// an, (plan_date)
pub fn select_index_plan_without_order_item_id(with_plan_date: bool, kphis: &str) -> String {
    let plan_date = if with_plan_date {" AND plan_date=?"} else {""};
    [
        "SELECT plan_id,order_item_id,an,plan_detail,plan_date,plan_time,plan_sch_type \
        FROM ",kphis,".ipd_nurse_index_plan \
        WHERE an=? AND order_item_id IS NULL", plan_date," ORDER BY plan_date,plan_time;"
    ].concat()
}

// // ipd-nurse-index-plan-action-delete.php
// DELETE kphis.ipd_nurse_index_plan, kphis.ipd_nurse_index_action
// FROM kphis.ipd_nurse_index_plan LEFT JOIN kphis.ipd_nurse_index_action ON kphis.ipd_nurse_index_plan.plan_id = kphis.ipd_nurse_index_action.plan_id
// WHERE kphis.ipd_nurse_index_plan.plan_id = ?;
// *** cannot use alias in delete `https://bugs.mysql.com/bug.php?id=82189` ***
/// plan_id
pub fn delete_index_plan(kphis: &str) -> String {
    [
        "DELETE ",kphis,".ipd_nurse_index_plan, ",kphis,".ipd_nurse_index_action \
        FROM ",kphis,".ipd_nurse_index_plan \
            LEFT JOIN ",kphis,".ipd_nurse_index_action ON ",kphis,".ipd_nurse_index_plan.plan_id = ",kphis,".ipd_nurse_index_action.plan_id \
        WHERE ",kphis,".ipd_nurse_index_plan.plan_id = ?;"
    ].concat()
}

// // ipd-nurse-index-plan-action-save.php
// INSERT INTO kphis.ipd_nurse_index_plan (order_item_id,an,plan_detail,plan_date,plan_time,plan_sch_type,create_user,create_datetime,update_user,update_datetime,version)
// VALUES (?,?,?,?,?,?,?,NOW(),?,NOW(),1);
/// order_item_id, an, plan_detail, plan_date, plan_time, plan_sch_type, loginname, loginname
pub fn insert_index_plan(kphis: &str) -> String {
    [
        "INSERT INTO ",kphis,".ipd_nurse_index_plan (order_item_id,an,plan_detail,plan_date,plan_time,plan_sch_type",TABLE_CREATE_COLUMNS,") \
        VALUES (?,?,?,?,?,?",TABLE_CREATE_PREPARED,");"
    ].concat()
}

// pub fn insert_index_plans_only(order_item_id: u32, an: &str, index_plans_only: &[IndexPlanOnly], kphis: &str) -> String {
//     let values = index_plans_only.iter().map(|item| {
//         [
//             "(",&order_item_id.to_owned(),",'",
//             an,"',",
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
//         "INSERT INTO ",kphis,".ipd_nurse_index_plan (order_item_id,an,plan_detail,plan_date,plan_time,plan_sch_type,create_user,create_datetime,update_user,update_datetime,version) \
//         VALUES ", &values,";"
//     ].concat()
// }

// UPDATE kphis.ipd_nurse_index_plan
// SET plan_detail=?,plan_date=?,plan_time=?,plan_sch_type=?,update_user=?,update_datetime=NOW(),version=(version+1)
// WHERE plan_id=?;
/// plan_detail, plan_date, plan_time, plan_sch_type, loginname, plan_id
pub fn update_index_plan(kphis: &str) -> String {
    [
        "UPDATE ",kphis,".ipd_nurse_index_plan \
        SET plan_detail=?,plan_date=?,plan_time=?,plan_sch_type=?",TABLE_UPDATE_SET,
        " WHERE plan_id=?;"
    ].concat()
}

// SELECT pl.icode,py.med_order_qty,py.med_pay_qty,py.pay_flag,py.order_date_time,py.pay_date_time,u.`name` AS entryuser,u.entryposition
// FROM hos.medplan_ipd pl
//     LEFT JOIN hos.medpay_ipd py ON py.med_plan_number=pl.med_plan_number
//     LEFT JOIN hos.ipt_order_no od ON od.order_no=py.med_rx_number
//     LEFT JOIN hos.opduser u ON u.loginname=od.entry_staff
// WHERE pl.an=? AND pl.icode LIKE '1%'
// ORDER BY pl.icode,py.order_date;
// 
// NOTE: some hos.medpay_ipd.med_plan_number = 0 
// 
// SELECT py.icode,CONCAT(di.`name`,' ',di.strength,' ',di.units) AS med_name,di.displaycolor,di.dosageform,
//     py.med_order_qty,py.med_pay_qty,py.pay_flag,py.order_date_time,py.pay_date_time,u.`name` AS entryuser,u.entryposition
// FROM hos.ipt_order_no od
//     LEFT JOIN hos.medpay_ipd py ON py.med_rx_number=od.order_no
//     LEFT JOIN hos.drugitems di ON di.icode=py.icode
//     LEFT JOIN hos.opduser u ON u.loginname=od.entry_staff
// WHERE od.an=? AND py.icode LIKE '1%'
// ORDER BY py.icode,py.order_date;
/// an
pub fn select_index_med_pay_hosxp(hosxp: &str) -> String {
    [
        "SELECT py.icode,CONCAT(di.`name`,' ',di.strength,' ',di.units) AS med_name,di.displaycolor,di.dosageform,\
            py.med_order_qty,py.med_pay_qty,py.pay_flag,py.order_date_time,py.pay_date_time,u.`name` AS entryuser,u.entryposition \
        FROM ",hosxp,".ipt_order_no od \
            LEFT JOIN ",hosxp,".medpay_ipd py ON py.med_rx_number=od.order_no \
            LEFT JOIN ",hosxp,".drugitems di ON di.icode=py.icode \
            LEFT JOIN ",hosxp,".opduser u ON u.loginname=od.entry_staff \
        WHERE od.an=? AND py.icode LIKE '1%' \
        ORDER BY py.icode,py.order_date;"
    ].concat()
}