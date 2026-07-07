use kphis_model::ipd::io::IoParams;

use crate::{TABLE_CREATE_COLUMNS, TABLE_CREATE_PREPARED};

// // ipd-vital-sign-io-select-date.php
// SELECT DISTINCT io_date, (io_date = DATE(NOW())) AS is_today FROM kphis.ipd_io WHERE an=? ORDER BY io_date DESC;
/// an
pub fn select_io_date(kphis: &str) -> String {
    [
        "SELECT DISTINCT io_date, (io_date = DATE(NOW())) AS is_today FROM ",kphis,".ipd_io WHERE an=? ORDER BY io_date DESC;"
    ].concat()
}

// // ipd-vital-sign-io-table.php
// // get 8hr data
// SELECT io_date,SUM(io_parenteral_absorb) AS sum8_io_parenteral_absorb,
//     SUM(io_oral_absorb) AS sum8_io_oral_absorb,SUM(io_output_amount) AS sum8_io_output_amount,
//     CASE
//         WHEN io_time BETWEEN '00:00:00.000' AND '07:59:59' THEN 'ดึก'
//         WHEN io_time BETWEEN '08:00:00.000' AND '15:59:59' THEN 'เช้า'
//         WHEN io_time BETWEEN '16:00:00.000' AND '23:59:59' THEN 'บ่าย'
//     ELSE NULL
//     END AS shift
// FROM kphis.ipd_io
// WHERE io_date=? AND an=?
// GROUP BY io_date,shift ORDER BY io_date,io_time;
// // get each data + user_name
// /// date, an
// SELECT io.*,kphuser.name AS user_name,kphuser.entryposition
// FROM kphis.ipd_io io
//     LEFT JOIN hos.opduser kphuser ON kphuser.loginname=io.update_user
// WHERE io.an=? AND io.io_date=? ORDER BY io.io_time;
// /// an, date
// // get 24hr data
// SELECT
//     SUM(io.io_parenteral_absorb) AS sum24_io_parenteral_absorb,
//     SUM(io.io_oral_absorb) AS sum24_io_oral_absorb,
//     SUM(io.io_output_amount) AS sum24_io_output_amount,
//     SUM(IFNULL(io.io_parenteral_absorb,0)+IFNULL(io.io_oral_absorb,0)) AS sum24_parenteral_oral_absorb
// FROM kphis.ipd_io io
// WHERE io.an=? AND io.io_date=?;
// /// an, date, shift_day 'io.io_'()
// // we get each data + user_name + shift and sum at client
// SELECT io.*,opduser.name AS user_name,opduser.entryposition,
//     CASE
//         WHEN (io.io_date=DATE_SUB('2024-01-31', INTERVAL 1 DAY) AND io.io_time BETWEEN '22:00:00.0' AND '23:59:59.999')
//             OR (io.io_date='2024-01-31' AND io.io_time BETWEEN '00:00:00.0' AND '05:59:59.999') THEN 'ดึก'
//         WHEN io.io_date='2024-01-31' AND io.io_time BETWEEN '06:00:00.0' AND '13:59:59.999' THEN 'เช้า'
//         WHEN io.io_date='2024-01-31' AND io.io_time BETWEEN '14:00:00.0' AND '21:59:59.999' THEN 'บ่าย'
//         ELSE NULL
//     END AS shift
// FROM kphis.ipd_io io
// 	LEFT JOIN hos.opduser ON opduser.loginname=io.update_user
// WHERE io.an='660001509' AND io.io_date IN (DATE_SUB('2024-01-31', INTERVAL 1 DAY),'2024-01-31') HAVING shift IS NOT NULL
// ORDER BY io.io_date,io.io_time;
/// prefix = 'io.io_'; shift_case(date x3 or x4), where_shift(date x1 or x2), an
// pub fn select_io_single_day(shift_case_sql: &str, where_shift: &str, kphis: &str, hosxp: &str) -> String {
//     [
//         "SELECT io.*,opduser.name AS user_name,opduser.entryposition,",shift_case_sql," AS shift \
//         FROM ",kphis,".ipd_io io \
//             LEFT JOIN ",hosxp,".opduser ON opduser.loginname=io.update_user \
//         WHERE ",where_shift," AND io.an=? HAVING shift IS NOT NULL \
//         ORDER BY io.io_date,io.io_time;"
//     )
// }
// // 31-05-2024 we change to calculate shift at client too
// // and we add date selection
// // with night shift can start before/after midnight
// // so we add 1 day before and 1 day after to date selection, and exclude outlier at client
// SELECT io.*,opduser.name AS user_name,opduser.entryposition
// FROM kphis.ipd_io io
// 	LEFT JOIN hos.opduser ON opduser.loginname=io.update_user
// WHERE io.an='660001509' AND io.io_date IN (DATE_SUB('2024-01-31', INTERVAL 1 DAY),'2024-01-31')
// ORDER BY io.io_date,io.io_time;
/// (io_id), (an), (start_date), (end_date)
pub fn select_io(params: &IoParams, kphis: &str, hosxp: &str) -> String {
    let io_id = if params.io_id.is_some() {" AND io.io_id=? "} else {""};
    let an = if params.an.is_some() {"  AND io.an=? "} else {""};
    let start_date = if params.start_date.is_some() {" AND io.io_date >= DATE_SUB(?, INTERVAL 1 DAY) "} else {""};
    let end_date = if params.end_date.is_some() {" AND io.io_date <= DATE_ADD(?, INTERVAL 1 DAY) "} else {""};
    [
        "SELECT io.*,opduser.name AS user_name,opduser.entryposition \
        FROM ",kphis,".ipd_io io \
            LEFT JOIN ",hosxp,".opduser ON opduser.loginname=io.update_user \
        WHERE 1=1 ",io_id,an,start_date,end_date,
        "ORDER BY io.io_date,io.io_time;"
    ].concat()
}

/// an
pub fn select_io_only(kphis: &str) -> String {
    [
        "SELECT io.* FROM ",kphis,".ipd_io io WHERE io.an=? ORDER BY io.io_date,io.io_time;"
    ].concat()
}

// ipd-vital-sign-io-save.php
// INSERT INTO kphis.ipd_io (io_date,io_time,io_parenteral_type,io_parenteral_name,io_parenteral_amount,io_parenteral_absorb,io_parenteral_carry_forward,io_parenteral_remark,
//     io_oral_name,io_oral_amount,io_oral_absorb,io_oral_carry_forward,io_oral_remark,io_output_type,io_output_amount,io_output_remark,an,create_user,create_datetime,update_user,update_datetime,version)
// VALUES (?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,NOW(),?,NOW(),1);
/// io_date, io_time, io_parenteral_type, io_parenteral_name, io_parenteral_amount, io_parenteral_absorb, io_parenteral_carry_forward, io_parenteral_remark,
/// io_oral_name, io_oral_amount, io_oral_absorb, io_oral_carry_forward, io_oral_remark, io_output_type, io_output_amount, io_output_remark, an,
/// loginname, loginname
pub fn insert_io(kphis: &str) -> String {
    [
        "INSERT INTO ",kphis,".ipd_io (io_date,io_time,io_parenteral_type,io_parenteral_name,io_parenteral_amount,io_parenteral_absorb,io_parenteral_carry_forward,io_parenteral_remark,\
            io_oral_name,io_oral_amount,io_oral_absorb,io_oral_carry_forward,io_oral_remark,io_output_type,io_output_amount,io_output_remark,an",TABLE_CREATE_COLUMNS,") \
        VALUES (?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?",TABLE_CREATE_PREPARED,");"
    ].concat()
}

/// io_date,io_time,io_parenteral_type,io_parenteral_name,io_parenteral_amount,io_parenteral_absorb,io_parenteral_carry_forward,io_parenteral_remark,
/// io_oral_name,io_oral_amount,io_oral_absorb,io_oral_carry_forward,io_oral_remark,io_output_type,io_output_amount,io_output_remark,an,
/// create_user,create_datetime,update_user,update_datetime,version
pub fn insert_io_only(kphis: &str) -> String {
    [
        "INSERT INTO ",kphis,".ipd_io (io_date,io_time,io_parenteral_type,io_parenteral_name,io_parenteral_amount,io_parenteral_absorb,io_parenteral_carry_forward,io_parenteral_remark,\
            io_oral_name,io_oral_amount,io_oral_absorb,io_oral_carry_forward,io_oral_remark,io_output_type,io_output_amount,io_output_remark,an",TABLE_CREATE_COLUMNS,") \
        VALUES (?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?);"
    ].concat()
}

// ipd-vital-sign-io-update.php
// UPDATE kphis.ipd_io
// SET io_date=?,io_time=?,io_parenteral_type=?,io_parenteral_name=?,io_parenteral_amount=?,io_parenteral_absorb=?,io_parenteral_carry_forward=?,io_parenteral_remark=?,
//     io_oral_name=?,io_oral_amount=?,io_oral_absorb=?,io_oral_carry_forward=?,io_oral_remark=?,io_output_type=?,io_output_amount=?,io_output_remark=?,
//     update_user=?,update_datetime=NOW(),version=(version + 1)
// WHERE io_id=? AND version=?;
/// io_date, io_time, io_parenteral_type, io_parenteral_name, io_parenteral_amount, io_parenteral_absorb, io_parenteral_carry_forward, io_parenteral_remark,
/// io_oral_name, io_oral_amount, io_oral_absorb, io_oral_carry_forward, io_oral_remark, io_output_type, io_output_amount, io_output_remark, loginname, io_id, version
pub fn update_io(kphis: &str) -> String {
    [
        "UPDATE ",kphis,".ipd_io \
        SET io_date=?,io_time=?,io_parenteral_type=?,io_parenteral_name=?,io_parenteral_amount=?,io_parenteral_absorb=?,io_parenteral_carry_forward=?,io_parenteral_remark=?,\
            io_oral_name=?,io_oral_amount=?,io_oral_absorb=?,io_oral_carry_forward=?,io_oral_remark=?,io_output_type=?,io_output_amount=?,io_output_remark=?,\
            update_user=?,update_datetime=NOW(),version=(version + 1) \
        WHERE io_id=? AND version=?;"
    ].concat()
}

// ipd-vital-sign-io-delete.php
// DELETE FROM kphis.ipd_io WHERE io_id=? AND version=?;
/// io_id, version
pub fn delete_io(kphis: &str) -> String {
    [
        "DELETE FROM ",kphis,".ipd_io WHERE io_id=? AND version=?;"
    ].concat()
}
