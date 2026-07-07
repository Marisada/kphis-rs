use kphis_model::ipd::io::IoParams;

use crate::{TABLE_CREATE_PREPARED, TABLE_CREATE_COLUMNS};

// SELECT DISTINCT opd_er_io_date AS io_date, (opd_er_io_date = DATE(NOW())) AS is_today FROM kphis.opd_er_io WHERE opd_er_order_master_id=? ORDER BY opd_er_io_date DESC;
/// opd_er_order_master_id
pub fn select_io_date(kphis: &str) -> String {
    [
        "SELECT DISTINCT opd_er_io_date AS io_date, (opd_er_io_date = DATE(NOW())) AS is_today FROM ",kphis,".opd_er_io WHERE opd_er_order_master_id=? ORDER BY opd_er_io_date DESC;"
    ].concat()
}

// // www\opd-er-vital-sign-io-table.php
// // we select using date as ipd doing
// SELECT io.*,opduser.name AS user_name,opduser.entryposition,
//     CASE
//         WHEN (io.opd_er_io_date=DATE_SUB('2024-01-31', INTERVAL 1 DAY) AND io.opd_er_io_time BETWEEN '22:00:00.0' AND '23:59:59.999')
//             OR (io.opd_er_io_date='2024-01-31' AND io.opd_er_io_time BETWEEN '00:00:00.0' AND '05:59:59.999') THEN 'ดึก'
//         WHEN io.opd_er_io_date='2024-01-31' AND io.opd_er_io_time BETWEEN '06:00:00.0' AND '13:59:59.999' THEN 'เช้า'
//         WHEN io.opd_er_io_date='2024-01-31' AND io.opd_er_io_time BETWEEN '14:00:00.0' AND '21:59:59.999' THEN 'บ่าย'
//         ELSE NULL
//     END AS shift
// FROM kphis.opd_er_io io
// 	LEFT JOIN hos.opduser ON opduser.loginname=io.update_user
// WHERE io.opd_er_order_master_id=1 AND io.opd_er_io_date IN (DATE_SUB('2024-01-31', INTERVAL 1 DAY),'2024-01-31') HAVING shift IS NOT NULL
// ORDER BY io.opd_er_io_date,io.opd_er_io_time;
/// prefix = 'io.opd_er_io_'; shift_case(date x3 or x4), where_shift(date x1 or x2), opd_er_order_master_id
// pub fn select_io_signle_day(shift_case_sql: &str, where_shift: &str, kphis: &str, hosxp: &str) -> String {
//     [
//         "SELECT io.*,opduser.name AS user_name,opduser.entryposition,",shift_case_sql," AS shift \
//         FROM ",kphis,".opd_er_io io \
//             LEFT JOIN ",hosxp,".opduser ON opduser.loginname=io.update_user \
//         WHERE ",where_shift," AND io.opd_er_order_master_id=? HAVING shift IS NOT NULL \
//         ORDER BY io.opd_er_io_date,io.opd_er_io_time;"
//     )
// }
// // 31-05-2024 we change to calculate shift at client too
// // and we add date selection
// // with night shift can start before/after midnight
// // so we add 1 day before and 1 day after to date selection, and exclude outlier at client
// SELECT io.*,opduser.name AS user_name,opduser.entryposition
// FROM kphis.opd_er_io io
// 	LEFT JOIN hos.opduser ON opduser.loginname=io.update_user
// WHERE io.opd_er_order_master_id=1 AND io.opd_er_io_date IN (DATE_SUB('2024-01-31', INTERVAL 1 DAY),'2024-01-31')
// ORDER BY io.opd_er_io_date,io.opd_er_io_time;
/// (opd_er_io_id), (opd_er_order_master_id), (start_date), (end_date)
pub fn select_io(params: &IoParams, kphis: &str, hosxp: &str) -> String {
    let io_id = if params.io_id.is_some() {" AND io.opd_er_io_id=? "} else {""};
    let opd_er_order_master_id = if params.opd_er_order_master_id.is_some() {"  AND io.opd_er_order_master_id=? "} else {""};
    let start_date = if params.start_date.is_some() {" AND io.opd_er_io_date >= DATE_SUB(?, INTERVAL 1 DAY) "} else {""};
    let end_date = if params.end_date.is_some() {" AND io.opd_er_io_date <= DATE_ADD(?, INTERVAL 1 DAY) "} else {""};
    [
        "SELECT io.*,opduser.name AS user_name,opduser.entryposition \
        FROM ",kphis,".opd_er_io io \
            LEFT JOIN ",hosxp,".opduser ON opduser.loginname=io.update_user \
        WHERE 1=1 ",io_id,opd_er_order_master_id,start_date,end_date,
        "ORDER BY io.opd_er_io_date,io.opd_er_io_time;"
    ].concat()
}

/// opd_er_order_master_id
pub fn select_io_only(kphis: &str) -> String {
    [
        "SELECT io.* FROM ",kphis,".opd_er_io io WHERE io.opd_er_order_master_id=? ORDER BY io.opd_er_io_date,io.opd_er_io_time;"
    ].concat()
}

// opd-er-vital-sign-io-save.php
// INSERT INTO kphis.opd_er_io (opd_er_io_date,opd_er_io_time,opd_er_io_parenteral_type,opd_er_io_parenteral_name,opd_er_io_parenteral_amount,opd_er_io_parenteral_absorb,opd_er_io_parenteral_carry_forward,opd_er_io_parenteral_remark,
//     opd_er_io_oral_name,opd_er_io_oral_amount,opd_er_io_oral_absorb,opd_er_io_oral_carry_forward,opd_er_io_oral_remark,opd_er_io_output_type,opd_er_io_output_amount,opd_er_io_output_remark,opd_er_order_master_id,
//     create_user,create_datetime,update_user,update_datetime,version)
// VALUES (?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,NOW(),?,NOW(),1);
/// opd_er_io_date, opd_er_io_time, opd_er_io_parenteral_type, opd_er_io_parenteral_name, opd_er_io_parenteral_amount, opd_er_io_parenteral_absorb, opd_er_io_parenteral_carry_forward, opd_er_io_parenteral_remark,
/// opd_er_io_oral_name, opd_er_io_oral_amount, opd_er_io_oral_absorb, opd_er_io_oral_carry_forward, opd_er_io_oral_remark, opd_er_io_output_type, opd_er_io_output_amount, opd_er_io_output_remark, opd_er_order_master_id,
/// loginname, loginname
pub fn insert_io(kphis: &str) -> String {
    [
        "INSERT INTO ",kphis,".opd_er_io (opd_er_io_date,opd_er_io_time,opd_er_io_parenteral_type,opd_er_io_parenteral_name,opd_er_io_parenteral_amount,opd_er_io_parenteral_absorb,opd_er_io_parenteral_carry_forward,opd_er_io_parenteral_remark,\
            opd_er_io_oral_name,opd_er_io_oral_amount,opd_er_io_oral_absorb,opd_er_io_oral_carry_forward,opd_er_io_oral_remark,opd_er_io_output_type,opd_er_io_output_amount,opd_er_io_output_remark,opd_er_order_master_id",
        TABLE_CREATE_COLUMNS,") VALUES (?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?",TABLE_CREATE_PREPARED,");"
    ].concat()
}

/// opd_er_io_date,opd_er_io_time,opd_er_io_parenteral_type,opd_er_io_parenteral_name,opd_er_io_parenteral_amount,opd_er_io_parenteral_absorb,opd_er_io_parenteral_carry_forward,opd_er_io_parenteral_remark,
/// opd_er_io_oral_name,opd_er_io_oral_amount,opd_er_io_oral_absorb,opd_er_io_oral_carry_forward,opd_er_io_oral_remark,opd_er_io_output_type,opd_er_io_output_amount,opd_er_io_output_remark,opd_er_order_master_id,
/// create_user,create_datetime,update_user,update_datetime,version
pub fn insert_io_only(kphis: &str) -> String {
    [
        "INSERT INTO ",kphis,".opd_er_io (opd_er_io_date,opd_er_io_time,opd_er_io_parenteral_type,opd_er_io_parenteral_name,opd_er_io_parenteral_amount,opd_er_io_parenteral_absorb,opd_er_io_parenteral_carry_forward,opd_er_io_parenteral_remark,\
            opd_er_io_oral_name,opd_er_io_oral_amount,opd_er_io_oral_absorb,opd_er_io_oral_carry_forward,opd_er_io_oral_remark,opd_er_io_output_type,opd_er_io_output_amount,opd_er_io_output_remark,opd_er_order_master_id",
        TABLE_CREATE_COLUMNS,") VALUES (?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?);"
    ].concat()
}

// opd-er-vital-sign-io-update.php
// UPDATE kphis.opd_er_io
// SET opd_er_io_date=?,opd_er_io_time=?,opd_er_io_parenteral_type=?,opd_er_io_parenteral_name=?,opd_er_io_parenteral_amount=?,opd_er_io_parenteral_absorb=?,opd_er_io_parenteral_carry_forward=?,opd_er_io_parenteral_remark=?,
//     opd_er_io_oral_name=?,opd_er_io_oral_amount=?,opd_er_io_oral_absorb=?,opd_er_io_oral_carry_forward=?,opd_er_io_oral_remark=?,opd_er_io_output_type=?,opd_er_io_output_amount=?,opd_er_io_output_remark=?,
//     update_user=?,update_datetime=NOW(),version=(version + 1)
// WHERE opd_er_io_id=? AND version=?;
/// opd_er_io_date, opd_er_io_time, opd_er_io_parenteral_type, opd_er_io_parenteral_name, opd_er_io_parenteral_amount, opd_er_io_parenteral_absorb, opd_er_io_parenteral_carry_forward, opd_er_io_parenteral_remark,
/// opd_er_io_oral_name, opd_er_io_oral_amount, opd_er_io_oral_absorb, opd_er_io_oral_carry_forward, opd_er_io_oral_remark, opd_er_io_output_type, opd_er_io_output_amount, opd_er_io_output_remark, loginname, opd_er_io_id, version
pub fn update_io(kphis: &str) -> String {
    [
        "UPDATE ",kphis,".opd_er_io \
        SET opd_er_io_date=?,opd_er_io_time=?,opd_er_io_parenteral_type=?,opd_er_io_parenteral_name=?,opd_er_io_parenteral_amount=?,opd_er_io_parenteral_absorb=?,opd_er_io_parenteral_carry_forward=?,opd_er_io_parenteral_remark=?,\
            opd_er_io_oral_name=?,opd_er_io_oral_amount=?,opd_er_io_oral_absorb=?,opd_er_io_oral_carry_forward=?,opd_er_io_oral_remark=?,opd_er_io_output_type=?,opd_er_io_output_amount=?,opd_er_io_output_remark=?,\
            update_user=?,update_datetime=NOW(),version=(version + 1) \
        WHERE opd_er_io_id=? AND version=?;"
    ].concat()
}

// opd-er-vital-sign-io-delete.php
// DELETE FROM kphis.opd_er_io WHERE opd_er_io_id=? AND version=?;
/// io_id, version
pub fn delete_io(kphis: &str) -> String {
    [
        "DELETE FROM ",kphis,".opd_er_io WHERE opd_er_io_id=? AND version=?;"
    ].concat()
}
