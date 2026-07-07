use crate::{TABLE_CREATE_COLUMNS, TABLE_CREATE_PREPARED};

// // ipd-lab-wbc-data.php
// SELECT lh.hn,lh.vn,lh.lab_order_number,lh.order_date,lh.report_date,lh.receive_date,lo.lab_order_result
//     FROM hos.lab_head lh
//     INNER JOIN hos.lab_order lo ON lh.lab_order_number=lo.lab_order_number
// WHERE lh.an=? AND lo.lab_items_code=5
// ORDER BY lh.lab_order_number DESC;
// // we change to get wbc and band
// SELECT lh.hn,lh.vn,lh.lab_order_number,lh.order_date,lh.report_date,lh.receive_date,
//     (SELECT lo.lab_order_result FROM hos.lab_order lo WHERE lh.lab_order_number=lo.lab_order_number AND lo.lab_items_code=5) AS wbc,
//     (SELECT lo.lab_order_result FROM hos.lab_order lo WHERE lh.lab_order_number=lo.lab_order_number AND lo.lab_items_code=20) AS band
// FROM hos.lab_head lh
// WHERE lh.an=? HAVING wbc IS NOT NULL
// ORDER BY lh.lab_order_number DESC;
/// wbc's lab_item_code(i64), band's lab_item_code(i64), [hn/vn]<br>
/// *NOTE*: 'vn' is 'vn' or 'an'
pub fn get_lab_wbc_band(key: &str, hosxp: &str) -> String {
    [
        "SELECT lh.hn,lh.vn,lh.lab_order_number,lh.order_date,lh.report_date,lh.receive_date,\
            (SELECT lo.lab_order_result FROM ",hosxp,".lab_order lo WHERE lh.lab_order_number=lo.lab_order_number AND lo.lab_items_code=?) AS wbc,\
            (SELECT lo.lab_order_result FROM ",hosxp,".lab_order lo WHERE lh.lab_order_number=lo.lab_order_number AND lo.lab_items_code=?) AS band \
        FROM ",hosxp,".lab_head lh \
        WHERE lh.",key,"=? HAVING wbc IS NOT NULL \
        ORDER BY lh.lab_order_number DESC;"
    ].concat()
}

// // ipd-nurse-lab.php
// SELECT h.lab_order_number,h.vn,h.hn,h.order_date,h.order_time,h.department,h.receive_date,h.receive_time,
//     IF(h.report_date='1899-12-30',NULL,h.report_date) AS report_date,IF(h.report_date='1899-12-30',NULL,h.report_time) AS report_time,
//     h.confirm_report,d.`name` AS doctor_name,h.form_name,
//     GROUP_CONCAT(DISTINCT los.lab_name SEPARATOR ', ') AS lab_name_cc,GROUP_CONCAT(DISTINCT lsi.specimen_name SEPARATOR ', ') AS specimen_name_cc,
//     IF(h.confirm_report='Y','1',(SELECT IF((COUNT(*) > 0),'2','3') FROM hos.lab_order o WHERE h.lab_order_number=o.lab_order_number AND o.confirm='Y')) AS lab_confirm_state,
//     IF(lr.lab_order_number IS NOT NULL,'Y','N') AS lab_read_status,d3.`name` AS lab_read_user,lr.lab_read_datetime,
//     CONCAT('ทนพ',CAST(IF(d1.sex='2','ญ','') AS CHAR),'.',d1.fname,' ',d1.lname,' ทน.',REPLACE(d1.licenseno,'-','')) AS reporter_name,
//     CONCAT('ทนพ',CAST(IF(d2.sex='2','ญ','') AS CHAR),'.',d2.fname,' ',d2.lname,' ทน.',REPLACE(d2.licenseno,'-','')) AS approve_staff
// FROM hos.lab_head h
//     LEFT JOIN hos.doctor d ON d.`code`=h.doctor_code
//     LEFT JOIN hos.opduser u1 ON u1.loginname=h.reporter_name
//     LEFT JOIN hos.doctor d1 ON d1.`code`=u1.doctorcode
//     LEFT JOIN hos.opduser u2 ON u2.loginname=h.approve_staff
//     LEFT JOIN hos.doctor d2 ON d2.`code`=u2.doctorcode
//     LEFT JOIN kphis.ipd_lab_read lr ON lr.lab_order_number=h.lab_order_number
//     LEFT JOIN hos.opduser u3 ON u3.loginname=lr.lab_read_user
//     LEFT JOIN hos.doctor d3 ON d3.`code`=u3.doctorcode
//     LEFT JOIN hos.lab_order_service los ON los.lab_order_number=h.lab_order_number
//     LEFT JOIN hos.lab_order lo ON lo.lab_order_number=h.lab_order_number
//     LEFT JOIN hos.lab_items li ON lo.lab_items_code=li.lab_items_code
//     LEFT JOIN hos.lab_specimen_items lsi ON li.specimen_code=lsi.specimen_code
// WHERE h.vn=?
// GROUP BY h.lab_order_number;
// -- ORDER BY IF(h.receive_date='1899-12-30',h.order_date,h.receive_date) DESC,IF(h.receive_date='1899-12-30',h.order_time,h.receive_time) DESC,h.order_date DESC,h.order_time DESC;
// -- ORDER BY IF(h.report_date='1899-12-30',h.order_date,h.report_date) DESC,IF(h.report_date='1899-12-30',h.order_time,h.report_time) DESC,h.order_date DESC,h.order_time DESC;
/// key = ["hn", "vn", "lab_order_number"]<br>
/// if key = "lab_order_number" then ids MUST NOT empty<br>
/// NOTE: lab_head.vn is an/vn in the same column<br>
/// (hn|vn), (?hn: start_date), (?hn: end_date)
pub fn get_lab_head(key: &str, ids: &[i32], with_start_date: bool, with_end_date: bool, hosxp: &str, kphis: &str) -> String {
    let (where_key, start_date, end_date, order_by) = if key == "lab_order_number" {(
        [" IN (",&ids.iter().map(|i| i.to_string()).collect::<Vec<String>>().join(","),") "].concat(),
        "",
        "",
        " ORDER BY h.lab_order_number DESC;",
    )} else {(
        String::from("=? "),
        if key == "hn" && with_start_date {" AND h.order_date >= ? "} else {""},
        if key == "hn" && with_end_date {" AND h.order_date <= ? "} else {""},
        " ORDER BY IF(h.receive_date='1899-12-30',h.order_date,h.receive_date) DESC,\
            IF(h.receive_date='1899-12-30',h.order_time,h.receive_time) DESC,h.order_date DESC,h.order_time DESC;"
    )};
    // let limit = if with_limit { " LIMIT ?;" } else { ";" };

    [
        "SELECT h.lab_order_number,h.vn,h.hn,h.order_date,h.order_time,h.department,h.receive_date,h.receive_time,\
            IF(h.report_date='1899-12-30',NULL,h.report_date) AS report_date,IF(h.report_date='1899-12-30',NULL,h.report_time) AS report_time,\
            h.confirm_report,d.`name` AS doctor_name,h.form_name,\
            GROUP_CONCAT(DISTINCT los.lab_name SEPARATOR ', ') AS lab_name_cc,GROUP_CONCAT(DISTINCT lsi.specimen_name SEPARATOR ', ') AS specimen_name_cc,\
            IF(h.confirm_report='Y','1',IF((SELECT EXISTS(SELECT * FROM ",hosxp,".lab_order o WHERE h.lab_order_number=o.lab_order_number AND o.confirm='Y')),'2','3')) AS lab_confirm_state,\
            IF(lr.lab_order_number IS NOT NULL,'Y','N') AS lab_read_status,d3.`name` AS lab_read_user,lr.lab_read_datetime,\
            CONCAT('ทนพ',CAST(IF(d1.sex='2','ญ','') AS CHAR),'.',d1.fname,' ',d1.lname,' ทน.',REPLACE(d1.licenseno,'-','')) AS reporter_name,\
            CONCAT('ทนพ',CAST(IF(d2.sex='2','ญ','') AS CHAR),'.',d2.fname,' ',d2.lname,' ทน.',REPLACE(d2.licenseno,'-','')) AS approve_staff,\
            (SELECT EXISTS(SELECT * FROM ",hosxp,".lab_order_image WHERE lab_order_number=h.lab_order_number AND image1 IS NOT NULL)) AS has_scan \
        FROM ",hosxp,".lab_head h \
            LEFT JOIN ",hosxp,".doctor d ON d.`code`=h.doctor_code \
            LEFT JOIN ",hosxp,".opduser u1 ON u1.loginname=h.reporter_name \
            LEFT JOIN ",hosxp,".doctor d1 ON d1.`code`=u1.doctorcode \
            LEFT JOIN ",hosxp,".opduser u2 ON u2.loginname=h.approve_staff \
            LEFT JOIN ",hosxp,".doctor d2 ON d2.`code`=u2.doctorcode \
            LEFT JOIN ",kphis,".ipd_lab_read lr ON lr.lab_order_number=h.lab_order_number \
            LEFT JOIN ",hosxp,".opduser u3 ON u3.loginname=lr.lab_read_user \
            LEFT JOIN ",hosxp,".doctor d3 ON d3.`code`=u3.doctorcode \
            LEFT JOIN ",hosxp,".lab_order_service los ON los.lab_order_number=h.lab_order_number \
            LEFT JOIN ",hosxp,".lab_order lo ON lo.lab_order_number=h.lab_order_number \
            LEFT JOIN ",hosxp,".lab_items li ON lo.lab_items_code=li.lab_items_code \
            LEFT JOIN ",hosxp,".lab_specimen_items lsi ON li.specimen_code=lsi.specimen_code \
        WHERE h.",key,&where_key,start_date,end_date," GROUP BY h.lab_order_number",order_by
    ].concat()
}

// SELECT DISTINCT h.lab_order_number
// FROM hos.lab_head h
//     LEFT JOIN hos.lab_order lo ON lo.lab_order_number=h.lab_order_number
// WHERE h.hn = '0001234'
//     AND h.confirm_report='Y'
//     AND h.lab_order_number <= 425080
//     AND lo.lab_items_code IN (SELECT lab_items_code FROM hos.lab_order WHERE lab_order_number = 425080)
// ORDER BY h.lab_order_number DESC LIMIT 10;
/// hn, lab_order_number x3, limit
pub fn get_lab_head_with_previous(hosxp: &str) -> String {
    [
        "SELECT DISTINCT h.lab_order_number \
            FROM ",hosxp,".lab_head h LEFT JOIN ",hosxp,".lab_order lo ON lo.lab_order_number=h.lab_order_number \
        WHERE h.hn=? \
            AND (h.lab_order_number=? OR (h.lab_order_number <? AND h.confirm_report='Y')) \
            AND lo.lab_items_code IN (SELECT lab_items_code FROM ",hosxp,".lab_order WHERE lab_order_number=?) \
        ORDER BY h.lab_order_number DESC LIMIT ?;"
    ].concat()
}

// pub fn get_lab_head(is_confirm: bool, hosxp: &str, kphis: &str) -> String {
//     let (confirm, order) = if is_confirm {
//         (" AND h.confirm_report='Y' ", "IF(h.report_date='1899-12-30',h.order_date,h.report_date) DESC,IF(h.report_date='1899-12-30',h.order_time,h.report_time) DESC,h.order_date DESC,h.order_time DESC;")
//     } else {
//         ("", "IF(h.receive_date='1899-12-30',h.order_date,h.receive_date) DESC,IF(h.receive_date='1899-12-30',h.order_time,h.receive_time) DESC,h.order_date DESC,h.order_time DESC;")
//     };
//     [
//         "SELECT h.lab_order_number,h.vn,h.hn,h.order_date,h.order_time,h.department,h.receive_date,h.receive_time,\
//             IF(h.report_date='1899-12-30',NULL,h.report_date) AS report_date,IF(h.report_date='1899-12-30',NULL,h.report_time) AS report_time,\
//             h.confirm_report,d.name AS doctor_name,h.form_name,\
//             GROUP_CONCAT(DISTINCT los.lab_name SEPARATOR ', ') AS lab_name_cc,GROUP_CONCAT(DISTINCT lsi.specimen_name SEPARATOR ', ') AS specimen_name_cc,\
//             IF(h.confirm_report='Y','1',(SELECT IF((COUNT(*) > 0),'2','3') FROM ",hosxp,".lab_order o WHERE h.lab_order_number=o.lab_order_number AND o.confirm='Y')) AS lab_confirm_state,\
//             IF(lr.lab_order_number IS NOT NULL,'Y','N') AS lab_read_status,lr.lab_read_user,lr.lab_read_datetime \
//         FROM ",hosxp,".lab_head h \
//             LEFT JOIN ",hosxp,".doctor d ON d.code=h.doctor_code \
//             LEFT JOIN ",kphis,".ipd_lab_read lr ON lr.lab_order_number=h.lab_order_number \
//             LEFT JOIN ",hosxp,".lab_order_service los ON los.lab_order_number=h.lab_order_number \
//             LEFT JOIN ",hosxp,".lab_order lo ON lo.lab_order_number=h.lab_order_number \
//             LEFT JOIN ",hosxp,".lab_items li ON lo.lab_items_code=li.lab_items_code \
//             LEFT JOIN ",hosxp,".lab_specimen_items lsi ON li.specimen_code=lsi.specimen_code \
//         WHERE h.hn=?",confirm,"GROUP BY h.lab_order_number ORDER BY ",order
//     )
// }

// // ipd-nurse-lab-detail.php
// SELECT h.vn,h.department,h.result_rtf,h.order_date,h.order_time,h.receive_date,h.receive_time,
//     IF(h.report_date='1899-12-30',NULL,h.report_date) AS report_date,IF(h.report_date='1899-12-30',NULL,h.report_time) AS report_time,
//     h.form_name,IF(lr.lab_order_number IS NOT NULL,'Y','N') AS lab_read_status,lr.lab_read_user,lr.lab_read_datetime,
//     GROUP_CONCAT(DISTINCT los.lab_name SEPARATOR ', ') AS lab_name_cc,
//     GROUP_CONCAT(DISTINCT lsi.specimen_name SEPARATOR ', ') AS specimen_name_cc
// FROM hos.lab_head h
//     LEFT JOIN kphis.ipd_lab_read lr ON lr.lab_order_number = h.lab_order_number
//     LEFT JOIN hos.lab_order_service los ON los.lab_order_number=h.lab_order_number
//     LEFT JOIN hos.lab_order lo ON lo.lab_order_number=h.lab_order_number
//     LEFT JOIN hos.lab_items li ON lo.lab_items_code=li.lab_items_code
//     LEFT JOIN hos.lab_specimen_items lsi ON li.specimen_code=lsi.specimen_code
// WHERE h.lab_order_number=? GROUP BY h.lab_order_number;
// /// lab_order_number
// pub fn select_lab_detail(hosxp: &str, kphis: &str) -> String {
//     [
//         "SELECT h.vn,h.department,h.result_rtf,h.order_date,h.order_time,h.receive_date,h.receive_time,\
//             IF(h.report_date='1899-12-30',NULL,h.report_date) AS report_date,IF(h.report_date='1899-12-30',NULL,h.report_time) AS report_time,\
//             h.form_name,IF(lr.lab_order_number IS NOT NULL,'Y','N') AS lab_read_status,lr.lab_read_user,lr.lab_read_datetime,\
//             GROUP_CONCAT(DISTINCT los.lab_name SEPARATOR ', ') AS lab_name_cc,\
//             GROUP_CONCAT(DISTINCT lsi.specimen_name SEPARATOR ', ') AS specimen_name_cc \
//         FROM ",hosxp,".lab_head h \
//             LEFT JOIN ",kphis,".ipd_lab_read lr ON lr.lab_order_number = h.lab_order_number \
//             LEFT JOIN ",hosxp,".lab_order_service los ON los.lab_order_number=h.lab_order_number \
//             LEFT JOIN ",hosxp,".lab_order lo ON lo.lab_order_number=h.lab_order_number \
//             LEFT JOIN ",hosxp,".lab_items li ON lo.lab_items_code=li.lab_items_code \
//             LEFT JOIN ",hosxp,".lab_specimen_items lsi ON li.specimen_code=lsi.specimen_code \
//         WHERE h.lab_order_number=? GROUP BY h.lab_order_number;"
//     )
// }

// SELECT i.lab_items_group,ig.lab_items_group_name
// FROM hos.lab_head h
//     INNER JOIN hos.lab_order o ON h.lab_order_number=o.lab_order_number
//     INNER JOIN hos.lab_items i ON o.lab_items_code=i.lab_items_code
//     LEFT JOIN hos.lab_items_group ig ON i.lab_items_group=ig.lab_items_group_code
// WHERE o.lab_order_number=?
// GROUP BY i.lab_items_group
// ORDER BY MIN(i.display_order);
/// lab_order_number
pub fn select_lab_detail_group(hosxp: &str) -> String {
    [
        "SELECT i.lab_items_group,ig.lab_items_group_name \
        FROM ",hosxp,".lab_head h \
            INNER JOIN ",hosxp,".lab_order o ON h.lab_order_number=o.lab_order_number \
            INNER JOIN ",hosxp,".lab_items i ON o.lab_items_code=i.lab_items_code \
            LEFT JOIN ",hosxp,".lab_items_group ig ON i.lab_items_group=ig.lab_items_group_code \
        WHERE o.lab_order_number=? GROUP BY i.lab_items_group \
        ORDER BY MIN(i.display_order);"
    ].concat()
}

// // ipd-nurse-lab-detail.php
// SELECT o.lab_items_name_ref,o.lab_items_normal_value_ref,
//     IF( ((SELECT (COUNT(*)=0) FROM hos.lab_items_doctor lid WHERE o.lab_items_code=lid.lab_items_code) AND
//         (SELECT (COUNT(*)=0) FROM hos.lab_items_visible liv WHERE o.lab_items_code=liv.lab_items_code)) OR
//         ((SELECT SUM(IF(doctor_code=?,1,0)) > 0 FROM hos.lab_items_doctor lid WHERE o.lab_items_code=lid.lab_items_code) OR
//         (SELECT SUM(IF(groupname=?,1,0)) > 0 FROM hos.lab_items_visible liv WHERE o.lab_items_code=liv.lab_items_code))
//     ,o.lab_order_result,'[[ปกปิด]]') AS lab_order_result,o.staff_lock_result,o.lab_order_remark,
//     i.lab_items_group,ig.lab_items_group_name,
//     o.lab_items_code,i.lab_items_unit,h.hn
// FROM hos.lab_head h
//     INNER JOIN hos.lab_order o ON h.lab_order_number=o.lab_order_number
//     INNER JOIN hos.lab_items i ON o.lab_items_code=i.lab_items_code
//     LEFT JOIN hos.lab_items_group ig ON i.lab_items_group=ig.lab_items_group_code
// WHERE o.confirm='Y' AND o.lab_order_number=?
// ORDER BY i.display_order,o.lab_items_name_ref;
// // ipd-nurse-lab-history-data.php
// SELECT o.lab_items_name_ref,o.lab_items_normal_value_ref,
//   IF( ((SELECT (COUNT(*)=0) FROM hos.lab_items_doctor lid WHERE o.lab_items_code=lid.lab_items_code) AND
//       (SELECT (COUNT(*)=0) FROM hos.lab_items_visible liv WHERE o.lab_items_code=liv.lab_items_code)) OR
//       ((SELECT SUM(IF(doctor_code=?,1,0)) > 0 FROM hos.lab_items_doctor lid WHERE o.lab_items_code=lid.lab_items_code) OR
//       (SELECT SUM(IF(groupname=?,1,0)) > 0 FROM hos.lab_items_visible liv WHERE o.lab_items_code=liv.lab_items_code))
//   ,o.lab_order_result,'[[ปกปิด]]') AS lab_order_result,o.staff_lock_result,o.lab_order_remark,
//   i.lab_items_group,ig.lab_items_group_name,
//   h.lab_order_number,h.vn,h.order_date,h.order_time,h.receive_date,h.receive_time,
//   IF(h.report_date='1899-12-30',NULL,h.report_date) AS report_date,IF(h.report_date='1899-12-30',NULL,h.report_time) AS report_time
// FROM hos.lab_head h
//     INNER JOIN hos.lab_order o ON h.lab_order_number=o.lab_order_number
//     INNER JOIN hos.lab_items i ON o.lab_items_code=i.lab_items_code
//     LEFT JOIN hos.lab_items_group ig ON i.lab_items_group=ig.lab_items_group_code
// WHERE o.confirm='Y' AND h.hn=? AND h.vn=? AND i.lab_items_code=?
// ORDER BY IF(h.report_date='1899-12-30',h.order_date,h.report_date) DESC,IF(h.report_date='1899-12-30',h.order_time,h.report_time) DESC,h.order_date DESC,h.order_time DESC;
/// doctorcode, groupname, (lab_order_number), (lab_items_group), (hn), (vn), (lab_items_code)
pub fn select_lab_items(
    has_lab_order_number: bool,
    has_lab_items_group: Option<bool>,
    has_hn: bool,
    has_vn: bool,
    has_lab_items_code: bool,
    is_order_by_time: bool,
    hosxp: &str,
) -> String {
    // detail parameters
    let lab_order_number = if has_lab_order_number {" AND o.lab_order_number=? "} else {""};
    let lab_items_group = has_lab_items_group.map(|has_group| {if has_group {" AND i.lab_items_group=? "} else {" AND i.lab_items_group IS NULL "}}).unwrap_or_default();
    // history parameters
    let hn = if has_hn { " AND h.hn=? " } else { "" };
    let vn = if has_vn { " AND h.vn=? " } else { "" };
    let lab_items_code = if has_lab_items_code {" AND i.lab_items_code=? "} else {""};
    let order = if is_order_by_time {
        " IF(h.report_date='1899-12-30',h.order_date,h.report_date) DESC,IF(h.report_date='1899-12-30',h.order_time,h.report_time) DESC,h.order_date DESC,h.order_time DESC;"
    } else {
        " i.display_order,o.lab_items_name_ref;"
    };
    [
        "SELECT o.lab_items_name_ref,o.lab_items_normal_value_ref,\
            IF(	((SELECT NOT EXISTS(SELECT * FROM ",hosxp,".lab_items_doctor lid WHERE o.lab_items_code=lid.lab_items_code)) AND \
                (SELECT NOT EXISTS(SELECT * FROM ",hosxp,".lab_items_visible liv WHERE o.lab_items_code=liv.lab_items_code))) OR \
                ((SELECT SUM(IF(lid.doctor_code=?,1,0)) > 0 FROM ",hosxp,".lab_items_doctor lid WHERE o.lab_items_code=lid.lab_items_code) OR \
                (SELECT SUM(IF(liv.groupname=?,1,0)) > 0 FROM ",hosxp,".lab_items_visible liv WHERE o.lab_items_code=liv.lab_items_code)),\
                o.lab_order_result,'[[ปกปิด]]') AS lab_order_result,\
            o.staff_lock_result,o.lab_order_remark,\
            i.lab_items_group,ig.lab_items_group_name,o.lab_items_code,i.lab_items_unit,h.hn,\
            h.lab_order_number,h.vn,h.order_date,h.order_time,h.receive_date,h.receive_time,\
            IF(h.report_date='1899-12-30',NULL,h.report_date) AS report_date,IF(h.report_date='1899-12-30',NULL,h.report_time) AS report_time \
        FROM ",hosxp,".lab_head h \
            INNER JOIN ",hosxp,".lab_order o ON h.lab_order_number=o.lab_order_number \
            INNER JOIN ",hosxp,".lab_items i ON o.lab_items_code=i.lab_items_code \
            LEFT JOIN ",hosxp,".lab_items_group ig ON i.lab_items_group=ig.lab_items_group_code \
        WHERE o.confirm='Y' ", lab_order_number, lab_items_group, hn, vn, lab_items_code,
        "ORDER BY ", order
    ].concat()
}

// // ipd-lab-read-save.php
// INSERT IGNORE INTO kphis.ipd_lab_read (lab_order_number,lab_read_user,lab_read_datetime,create_user,create_datetime,update_user,update_datetime,version)
//     VALUES (?,?,NOW(),?,NOW(),?,NOW(),1);
/// lab_order_number, loginname, loginname, loginname
pub fn insert_ignore_lab_read(kphis: &str) -> String {
    [
        "INSERT IGNORE INTO ",kphis,".ipd_lab_read (lab_order_number,lab_read_user,lab_read_datetime",TABLE_CREATE_COLUMNS,") \
            VALUES (?,?,NOW()",TABLE_CREATE_PREPARED,");"
    ].concat()
}

// // ipd-lab-read-delete.php
// DELETE FROM kphis.ipd_lab_read WHERE lab_order_number=?;
/// lab_order_number
pub fn delete_lab_read(kphis: &str) -> String {
    [
        "DELETE FROM ",kphis,".ipd_lab_read WHERE lab_order_number=?;"
    ].concat()
}

// // GET /exists-key-id/lab-unread/:vnan
// SELECT EXISTS(SELECT * FROM hos.lab_head h LEFT JOIN kphis.ipd_lab_read lr ON lr.lab_order_number=h.lab_order_number WHERE h.vn=? AND h.confirm_report='Y' AND lr.lab_order_number IS NULL) AS exs;
/// vnan
pub fn get_lab_unread_exists(hosxp: &str, kphis: &str) -> String {
    [
        "SELECT EXISTS(SELECT * FROM ",hosxp,".lab_head h LEFT JOIN ",kphis,".ipd_lab_read lr ON lr.lab_order_number=h.lab_order_number WHERE h.vn=? AND h.report_date IS NOT NULL AND h.confirm_report='Y' AND lr.lab_order_number IS NULL) AS exs;"
    ].concat()
}

// // GET /exists-key-id/lab-unreport/:vnan
// SELECT EXISTS(SELECT * FROM hos.lab_head WHERE vn=? AND report_date IS NULL AND confirm_report<>'Y') AS exs;
/// vnan
pub fn get_lab_unreport_exists(hosxp: &str) -> String {
    [
        "SELECT EXISTS(SELECT * FROM ",hosxp,".lab_head WHERE vn=? AND (report_date IS NULL OR confirm_report <> 'Y')) AS exs;"
    ].concat()
}
