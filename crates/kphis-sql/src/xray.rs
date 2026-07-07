use crate::{TABLE_CREATE_COLUMNS, TABLE_CREATE_PREPARED};

// SELECT xr.xn,xr.hn,xr.vn,xr.an,xr.confirm,xi.xray_items_name,
//   xr.request_date,xr.request_time,xr.examined_date,xr.examined_time,
//   d1.`name` AS technician_name, d2.`name` AS request_doctor_name,
//   IF(r.xn IS NOT NULL,'Y','N') AS xray_read_status,d3.`name` AS xray_read_user,r.xray_read_datetime
// FROM hos.xray_report xr
//     LEFT JOIN hos.xray_items xi ON xi.xray_items_code=xr.xray_items_code
//     LEFT JOIN hos.doctor d1 ON d1.`code`=xr.doctor
//     LEFT JOIN hos.doctor d2 ON d2.`code`=xr.request_doctor
//     LEFT JOIN kphis.ipd_xray_read r ON r.xn=xr.xn
//     LEFT JOIN hos.opduser u ON u.loginname=r.xray_read_user
//     LEFT JOIN hos.doctor d3 ON d3.`code`=u.doctorcode
// WHERE xr.hn='0001234' ORDER BY xr.request_date DESC,xr.request_time DESC;
/// hn
pub fn get_xray_report(hosxp: &str, kphis: &str) -> String {
    ["SELECT xr.xn,xr.hn,xr.vn,xr.an,xr.confirm,xi.xray_items_name,\
        xr.request_date,xr.request_time,xr.examined_date,xr.examined_time,\
        d1.`name` AS technician_name, d2.`name` AS request_doctor_name,\
        IF(r.xn IS NOT NULL,'Y','N') AS xray_read_status,d3.`name` AS xray_read_user,r.xray_read_datetime \
        FROM ",hosxp,".xray_report xr \
            LEFT JOIN ",hosxp,".xray_items xi ON xi.xray_items_code=xr.xray_items_code \
            LEFT JOIN ",hosxp,".doctor d1 ON d1.`code`=xr.doctor \
            LEFT JOIN ",hosxp,".doctor d2 ON d2.`code`=xr.request_doctor \
            LEFT JOIN ",kphis,".ipd_xray_read r ON r.xn=xr.xn \
            LEFT JOIN ",hosxp,".opduser u ON u.loginname=r.xray_read_user \
            LEFT JOIN ",hosxp,".doctor d3 ON d3.`code`=u.doctorcode \
        WHERE xr.hn=? ORDER BY xr.request_date DESC,xr.request_time DESC;"].concat()
}

// INSERT IGNORE INTO kphis.ipd_xray_read (xn,xray_read_user,xray_read_datetime,create_user,create_datetime,update_user,update_datetime,version)
//     VALUES (?,?,NOW(),?,NOW(),?,NOW(),1);
/// xn, loginname, loginname, loginname
pub fn insert_ignore_xray_read(kphis: &str) -> String {
    [
        "INSERT IGNORE INTO ",kphis,".ipd_xray_read (xn,xray_read_user,xray_read_datetime",TABLE_CREATE_COLUMNS,") \
            VALUES (?,?,NOW()",TABLE_CREATE_PREPARED,");"
    ].concat()
}

// DELETE FROM kphis.ipd_xray_read WHERE xn=?;
/// xn
pub fn delete_xray_read(kphis: &str) -> String {
    [
        "DELETE FROM ",kphis,".ipd_xray_read WHERE xn=?;"
    ].concat()
}

// // GET /exists-key-id/ipd-xray-unread/{an}
// SELECT EXISTS(SELECT * FROM hos.xray_report xr LEFT JOIN kphis.ipd_xray_read r ON r.xn=xr.xn WHERE xr.an=? AND xr.confirm='Y' AND r.xn IS NULL) AS exs;
/// an
pub fn get_xray_unread_an_exists(hosxp: &str, kphis: &str) -> String {
    [
        "SELECT EXISTS(SELECT * FROM ",hosxp,".xray_report xr LEFT JOIN ",kphis,".ipd_xray_read r ON r.xn=xr.xn WHERE xr.an=? AND xr.confirm='Y' AND r.xn IS NULL) AS exs;"
    ].concat()
}

// // GET /exists-key-id/opd-er-xray-unread/{vn}
// SELECT EXISTS(SELECT * FROM hos.xray_report xr LEFT JOIN kphis.ipd_xray_read r ON r.xn=xr.xn WHERE xr.vn=? AND xr.confirm='Y' AND r.xn IS NULL) AS exs;
/// an
pub fn get_xray_unread_vn_exists(hosxp: &str, kphis: &str) -> String {
    [
        "SELECT EXISTS(SELECT * FROM ",hosxp,".xray_report xr LEFT JOIN ",kphis,".ipd_xray_read r ON r.xn=xr.xn WHERE xr.vn=? AND xr.confirm='Y' AND r.xn IS NULL) AS exs;"
    ].concat()
}