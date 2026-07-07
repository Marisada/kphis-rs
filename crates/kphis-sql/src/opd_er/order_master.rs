use kphis_model::{
    opd_er::order_master::OpdErOrderMasterParams,
    score::CONCAT_SQL,
};

use crate::{TABLE_CREATE_COLUMNS, TABLE_CREATE_PREPARED};

// SELECT opd_er_order_master_id, order_date
// FROM kphis.opd_er_order_master
// WHERE vn=? AND (delete_flag IS NULL OR delete_flag <> 'Y')
// ORDER BY opd_er_order_master_id;
/// vn
pub fn select_master_check(kphis: &str) -> String {
    [
        "SELECT opd_er_order_master_id, order_date FROM ",kphis,".opd_er_order_master \
        WHERE vn=? AND (delete_flag IS NULL OR delete_flag <> 'Y') ORDER BY opd_er_order_master_id;"
    ].concat()
}

// // opd-er-order-list-data.php
// SELECT om.opd_er_order_master_id,om.bedno,b.bedno AS display_bedno,b.bed_type,bt.bed_type_name,bt.bed_type_color,bt.display_order AS bed_type_display_order,
//     om.er_patient_status_id,ps.er_patient_status_name,ps.display_order AS opd_er_patient_status_display_order,om.er_dch_type_id,dt.er_dch_type_name,dt.display_order AS opd_er_dch_type_display_order,
//     om.discharge_date,om.discharge_time,CONCAT(om.discharge_date,' ',om.discharge_time) AS discharge_date_time,om.note,om.vn,ovst.oqueue,ovst.hn,ovst.an,ovst.vstdate,ovst.vsttime,CONCAT(ovst.vstdate,' ',ovst.vsttime) AS vstdate_time,
//     om.order_date,om.order_time,CONCAT(om.order_date,' ',om.order_time) AS order_date_time,om.order_doctor,d.`name` AS order_doctor_name,CONCAT(p.pname,p.fname,' ',p.lname) AS ptname,vn_stat.age_y,vn_stat.age_m,vn_stat.age_d,
//     (SELECT GROUP_CONCAT(DISTINCT d.`name` ORDER BY o.order_date DESC, o.order_time DESC SEPARATOR '\n') FROM kphis.opd_er_order o LEFT JOIN hos.doctor d ON o.order_doctor = d.`code`
//         WHERE o.opd_er_order_master_id = om.opd_er_order_master_id AND o.order_confirm = 'Y' GROUP BY o.opd_er_order_master_id) AS all_order_doctor_name,
//     (SELECT MAX(CONCAT(o.order_date,' ',o.order_time)) FROM kphis.opd_er_order o WHERE o.opd_er_order_master_id = om.opd_er_order_master_id AND o.order_confirm = 'Y'
//         GROUP BY o.opd_er_order_master_id) AS max_order_date_time,
//     (SELECT COUNT(*) FROM kphis.opd_er_order o WHERE o.opd_er_order_master_id=om.opd_er_order_master_id AND o.order_confirm = 'Y' AND o.order_owner_type = 'doctor'
//         AND o.nurse_accept_time IS NULL) AS count_nurse_not_accept,
//     (SELECT COUNT(DISTINCT o.order_id) FROM kphis.opd_er_order o JOIN kphis.opd_er_order_item oi ON o.order_id = oi.order_id AND oi.order_item_type = 'discharge'
//         WHERE o.opd_er_order_master_id=om.opd_er_order_master_id AND o.order_confirm = 'Y' AND o.order_owner_type = 'doctor') AS count_discharge_order,
//     (SELECT COUNT(DISTINCT o.order_id) FROM kphis.opd_er_order o JOIN kphis.opd_er_order_item oi ON o.order_id = oi.order_id AND oi.stat = 'Y'
//         WHERE o.opd_er_order_master_id=om.opd_er_order_master_id AND o.order_confirm = 'Y' AND o.order_owner_type = 'doctor' AND o.nurse_accept_time IS NULL) AS count_stat_order_nurse_not_accept,
//     (SELECT score_total(score_bt(vn_stat.age_y,vs.bt),score_pr(vn_stat.age_y,vs.pr),score_rr(vn_stat.age_y,vs.rr,vs.respirator),
//         score_sbp(vn_stat.age_y,vs.sbp,vs.inotrope),score_conscious_id(vn_stat.age_y,vs.conscious_id),score_urine(vn_stat.age_y,vs.urine_amount,vs.urine_duration))
//         FROM kphis.opd_er_vs_vital_sign vs
//         WHERE vs.opd_er_order_master_id = om.opd_er_order_master_id AND (
//             (bt IS NOT NULL AND TRIM(bt) <> '') OR (pr IS NOT NULL AND TRIM(pr) <> '') OR ((rr IS NOT NULL AND TRIM(rr) <> '') AND (respiratOR IS NOT NULL AND TRIM(respirator) <> ''))
//             OR ((sbp IS NOT NULL AND TRIM(sbp) <> '') AND (inotrope IS NOT NULL AND TRIM(inotrope) <> '')) OR (conscious_id IS NOT NULL AND TRIM(conscious_id) <> '')
//             OR ((urine_amount IS NOT NULL AND TRIM(urine_amount) <> '') AND (urine_duration IS NOT NULL AND TRIM(urine_duration) <> ''))
//         ) ORDER BY vs.vs_datetime DESC LIMIT 1) AS mews_score
// FROM kphis.opd_er_order_master om
//     LEFT JOIN kphis.opd_er_bed b ON b.opd_er_bed_id=om.bedno
//     LEFT JOIN kphis.opd_er_bed_type bt ON b.bed_type=bt.bed_type
//     LEFT JOIN kphis.opd_er_patient_status ps ON ps.er_patient_status_id=om.er_patient_status_id
//     LEFT JOIN kphis.opd_er_dch_type dt ON dt.er_dch_type_id=om.er_dch_type_id
//     LEFT JOIN hos.ovst ON ovst.vn=om.vn
//     LEFT JOIN hos.vn_stat ON ovst.vn=vn_stat.vn
//     LEFT JOIN hos.patient p ON p.hn=ovst.hn
//     LEFT JOIN hos.doctor d ON d.code=om.order_doctor
// WHERE (om.delete_flag IS NULL OR om.delete_flag <> 'Y')
// ORDER BY om.opd_er_order_master_id DESC;
/// (% + hn), (opd_er_order_master_id), (vn), (vstdate), (qn), (start_order_date), (end_order_date), (order_date), (order_doctor), (bedno),<br>
/// er_patient_status_id=7?{(er_dch_type_id)}
pub fn select_order_master_list(
    params: &OpdErOrderMasterParams,
    intern_roles: &[String], 
    hosxp: &str,
    kphis: &str,
) -> String {
    let hn = if params.hn.is_some() {" AND ovst.hn like ?"} else {""};
    let opd_er_order_master_id = if params.opd_er_order_master_id.is_some() {" AND om.opd_er_order_master_id=?"} else {""};
    let vn = if params.vn.is_some() {" AND om.vn=?"} else {""};
    let vstdate = if params.vstdate.is_some() {" AND ovst.vstdate=?"} else {""};
    let qn = if params.qn.is_some() {" AND ovst.oqueue=? AND ovst.vstdate = DATE(NOW())"} else {""};
    let start_order_date = if params.start_order_date.is_some() {" AND om.order_date>=?"} else {""};
    let end_order_date = if params.end_order_date.is_some() {" AND om.order_date<=?"} else {""};
    let order_date = if params.order_date.is_some() {" AND om.order_date=?"} else {""};
    let order_doctor = if params.order_doctor.is_some() {" AND om.order_doctor=?"} else {""};
    let bedno = if params.bedno.is_some() {" AND om.bedno=? "} else {""};
    let (er_patient_status_id, er_dch_type_id) = match params.er_patient_status_id {
        Some(status_id) => {
            if status_id != 7 {(
                " AND om.er_patient_status_id <> 7", ""
            )} else {(
                " AND om.er_patient_status_id=7", if params.er_dch_type_id.is_some() {" AND om.er_dch_type_id=?"} else {""},
            )}
        }
        None => ("", ""),
    };
    [
        "SELECT om.opd_er_order_master_id,om.bedno,b.bedno AS display_bedno,b.bed_type,bt.bed_type_name,bt.bed_type_color,bt.display_order AS bed_type_display_order,sex.`name` AS sex_name,ptt.pcode AS rtcode,ptt.name AS rtname,\
            om.er_patient_status_id,ps.er_patient_status_name,ps.display_order AS opd_er_patient_status_display_order,om.er_dch_type_id,dt.er_dch_type_name,dt.display_order AS opd_er_dch_type_display_order,\
            om.discharge_date,om.discharge_time,ADDTIME(CONVERT(om.discharge_date,DATETIME),om.discharge_time) AS discharge_date_time,om.note,om.vn,ovst.oqueue,ovst.hn,ovst.an,ovst.vstdate,ovst.vsttime,ADDTIME(CONVERT(ovst.vstdate,DATETIME),ovst.vsttime) AS vstdate_time,\
            om.order_date,om.order_time,ADDTIME(CONVERT(om.order_date,DATETIME),om.order_time) AS order_date_time,om.order_doctor,d.`name` AS order_doctor_name,CONCAT(p.pname,p.fname,' ',p.lname) AS ptname,p.birthday,vn_stat.age_y,vn_stat.age_m,vn_stat.age_d,\
            (SELECT EXISTS(SELECT * FROM ",hosxp,".opduser ou LEFT JOIN ",kphis,".system_ac_role_user ru ON ou.loginname=ru.loginname \
                WHERE ou.doctorcode=om.order_doctor AND (ou.account_disable IS NULL OR ou.account_disable='N') AND ru.role IN ('",&intern_roles.join("','"),"'))) AS order_doctor_is_intern,\
            (SELECT GROUP_CONCAT(DISTINCT d.`name` ORDER BY o.order_date DESC, o.order_time DESC SEPARATOR '\n') FROM ",kphis,".opd_er_order o LEFT JOIN ",hosxp,".doctor d ON o.order_doctor = d.`code` \
                WHERE o.opd_er_order_master_id = om.opd_er_order_master_id AND o.order_confirm = 'Y' GROUP BY o.opd_er_order_master_id) AS all_order_doctor_name,\
            (SELECT MAX(ADDTIME(CONVERT(o.order_date,DATETIME),o.order_time)) FROM ",kphis,".opd_er_order o WHERE o.opd_er_order_master_id = om.opd_er_order_master_id AND o.order_confirm = 'Y' \
                GROUP BY o.opd_er_order_master_id) AS max_order_date_time,\
            (SELECT COUNT(*) FROM ",kphis,".opd_er_order o WHERE o.opd_er_order_master_id=om.opd_er_order_master_id AND o.order_confirm = 'Y' AND o.order_owner_type = 'doctor' \
                AND o.nurse_accept_time IS NULL) AS count_nurse_not_accept,\
            (SELECT COUNT(DISTINCT o.order_id) FROM ",kphis,".opd_er_order o JOIN ",kphis,".opd_er_order_item oi ON o.order_id = oi.order_id AND oi.order_item_type = 'discharge' \
                WHERE o.opd_er_order_master_id=om.opd_er_order_master_id AND o.order_confirm = 'Y' AND o.order_owner_type = 'doctor') AS count_discharge_order,\
            (SELECT COUNT(DISTINCT o.order_id) FROM ",kphis,".opd_er_order o JOIN ",kphis,".opd_er_order_item oi ON o.order_id = oi.order_id AND oi.stat = 'Y' \
                WHERE o.opd_er_order_master_id=om.opd_er_order_master_id AND o.order_confirm = 'Y' AND o.order_owner_type = 'doctor' AND o.nurse_accept_time IS NULL) AS count_stat_order_nurse_not_accept,\
            (SELECT ",CONCAT_SQL," FROM ",kphis,".opd_er_vs_vital_sign WHERE opd_er_order_master_id=om.opd_er_order_master_id ORDER BY vs_datetime DESC LIMIT 1) AS ews_concat \
        FROM ",kphis,".opd_er_order_master om \
            LEFT JOIN ",kphis,".opd_er_bed b ON b.opd_er_bed_id=om.bedno \
            LEFT JOIN ",kphis,".opd_er_bed_type bt ON bt.bed_type=b.bed_type \
            LEFT JOIN ",kphis,".opd_er_patient_status ps ON ps.er_patient_status_id=om.er_patient_status_id \
            LEFT JOIN ",kphis,".opd_er_dch_type dt ON dt.er_dch_type_id=om.er_dch_type_id \
            LEFT JOIN ",hosxp,".ovst ON ovst.vn=om.vn \
            LEFT JOIN ",hosxp,".vn_stat ON ovst.vn=vn_stat.vn \
            LEFT JOIN ",hosxp,".patient p ON p.hn=ovst.hn \
            LEFT JOIN ",hosxp,".sex ON sex.code=p.sex \
            LEFT JOIN ",hosxp,".pttype ptt ON ptt.pttype=ovst.pttype \
            LEFT JOIN ",hosxp,".doctor d ON d.code=om.order_doctor \
        WHERE (om.delete_flag IS NULL OR om.delete_flag <> 'Y') ",hn,opd_er_order_master_id,vn,vstdate,qn,start_order_date,end_order_date,order_date,order_doctor,bedno,
        er_patient_status_id,er_dch_type_id," ORDER BY om.bedno LIMIT 100;"
    ].concat()
}

// // opd-er-order-master-data.php
// SELECT om.opd_er_order_master_id,om.bedno,b.bedno AS display_bedno,b.bed_type,bt.bed_type_name,bt.bed_type_color,bt.display_order AS bed_type_display_order,
//     om.er_patient_status_id,ps.er_patient_status_name,om.er_dch_type_id,dt.er_dch_type_name,om.discharge_date,om.discharge_time,
//     IF(om.discharge_date IS NULL OR om.discharge_time IS NULL,'N',IF((DATE_ADD(NOW(),INTERVAL -1 DAY) > CONCAT(om.discharge_date,' ',om.discharge_time)),'Y','N')) AS pass_24_hour_from_dch,
//     om.note,om.vn,ovst.oqueue,ovst.hn,ovst.an,ovst.vstdate,ovst.vsttime,CONCAT(ovst.vstdate,' ',ovst.vsttime) AS vstdate_time,
//     om.order_date,om.order_time,CONCAT(om.order_date,' ',om.order_time) AS order_date_time,om.order_doctor,d.`name` AS order_doctor_name,
//     CONCAT(p.pname,p.fname,' ',p.lname) AS ptname,vn_stat.age_y,vn_stat.age_m,vn_stat.age_d,
//     (SELECT GROUP_CONCAT(h.er_allergy_history_agent) FROM kphis.opd_er_allergy_history h
//         WHERE h.opd_er_order_master_id = om.opd_er_order_master_id ORDER BY h.er_allergy_history_id) AS er_drugallergy_history
// FROM kphis.opd_er_order_master om
//     LEFT JOIN kphis.opd_er_bed b ON b.opd_er_bed_id=om.bedno
//     LEFT JOIN kphis.opd_er_bed_type bt ON b.bed_type=bt.bed_type
//     LEFT JOIN kphis.opd_er_patient_status ps ON ps.er_patient_status_id=om.er_patient_status_id
//     LEFT JOIN kphis.opd_er_dch_type dt ON dt.er_dch_type_id=om.er_dch_type_id
//     LEFT JOIN hos.ovst ON ovst.vn=om.vn
//     LEFT JOIN hos.vn_stat ON ovst.vn=vn_stat.vn
//     LEFT JOIN hos.patient p ON p.hn=ovst.hn
//     LEFT JOIN hos.doctor d ON d.code=om.order_doctor
//     LEFT JOIN hos.ipt ON ipt.vn=ovst.vn
// WHERE om.opd_er_order_master_id=? ORDER BY om.opd_er_order_master_id;
/// (opd_er_order_master_id)
pub fn select_order_master(intern_roles: &[String], hosxp: &str, kphis: &str) -> String {
    [
        "SELECT om.opd_er_order_master_id,om.bedno,b.bedno AS display_bedno,b.bed_type,bt.bed_type_name,bt.bed_type_color,bt.display_order AS bed_type_display_order,\
            om.er_patient_status_id,ps.er_patient_status_name,om.er_dch_type_id,dt.er_dch_type_name,om.discharge_date,om.discharge_time,\
            IF(om.discharge_date IS NULL OR om.discharge_time IS NULL,'N',IF(DATE_ADD(NOW(),INTERVAL -1 DAY) > ADDTIME(CONVERT(om.discharge_date,DATETIME),om.discharge_time),'Y','N')) AS pass_24_hour_from_dch,\
            om.note,om.vn,ovst.oqueue,ovst.hn,ovst.an,ovst.vstdate,ovst.vsttime,ADDTIME(CONVERT(ovst.vstdate,DATETIME),ovst.vsttime) AS vstdate_time,\
            om.order_date,om.order_time,ADDTIME(CONVERT(om.order_date,DATETIME),om.order_time) AS order_date_time,om.order_doctor,d.`name` AS order_doctor_name,\
            (SELECT EXISTS(SELECT * FROM ",hosxp,".opduser ou LEFT JOIN ",kphis,".system_ac_role_user ru ON ou.loginname=ru.loginname \
                WHERE ou.doctorcode=om.order_doctor AND (ou.account_disable IS NULL OR ou.account_disable='N') AND ru.role IN ('",&intern_roles.join("','"),"'))) AS order_doctor_is_intern,\
            CONCAT(p.pname,p.fname,' ',p.lname) AS ptname,vn_stat.age_y,vn_stat.age_m,vn_stat.age_d,\
            (SELECT GROUP_CONCAT(h.er_allergy_history_agent) FROM ",kphis,".opd_er_allergy_history h \
                WHERE h.opd_er_order_master_id = om.opd_er_order_master_id ORDER BY h.er_allergy_history_id) AS er_drugallergy_history \
        FROM ",kphis,".opd_er_order_master om \
            LEFT JOIN ",kphis,".opd_er_bed b ON b.opd_er_bed_id=om.bedno \
            LEFT JOIN ",kphis,".opd_er_bed_type bt ON b.bed_type=bt.bed_type \
            LEFT JOIN ",kphis,".opd_er_patient_status ps ON ps.er_patient_status_id=om.er_patient_status_id \
            LEFT JOIN ",kphis,".opd_er_dch_type dt ON dt.er_dch_type_id=om.er_dch_type_id \
            LEFT JOIN ",hosxp,".ovst ON ovst.vn=om.vn \
            LEFT JOIN ",hosxp,".vn_stat ON ovst.vn=vn_stat.vn \
            LEFT JOIN ",hosxp,".patient p ON p.hn=ovst.hn \
            LEFT JOIN ",hosxp,".doctor d ON d.code=om.order_doctor \
            LEFT JOIN ",hosxp,".ipt ON ipt.vn=ovst.vn \
        WHERE om.opd_er_order_master_id=? ORDER BY om.opd_er_order_master_id;"
    ].concat()
}

// // opd-er-order-master-save.php
// SELECT COUNT(*) AS cnt FROM kphis.opd_er_order_master WHERE vn = ? AND opd_er_order_master_id <> ?;
/// vn,(opd_er_order_master_id)
/// return cnt:i64
pub fn exists_order_master_by_vn(has_exclude_id: bool, kphis: &str) -> String {
    let opd_er_order_master_id = if has_exclude_id {" AND opd_er_order_master_id <> ?"} else {""};
    [
        "SELECT EXISTS(SELECT * FROM ",kphis,".opd_er_order_master WHERE vn = ?",opd_er_order_master_id,") AS exs;"
    ].concat()
}

// SELECT COUNT(*) AS cnt FROM kphis.opd_er_order_master WHERE bedno = ? AND er_patient_status_id <> 7 AND opd_er_order_master_id <> ?;
/// bedno,(opd_er_order_master_id)
/// return cnt:i64
pub fn exists_order_master_by_bedno(has_exclude_id: bool, kphis: &str) -> String {
    let opd_er_order_master_id = if has_exclude_id {" AND opd_er_order_master_id <> ?"} else {""};
    [
        "SELECT EXISTS(SELECT * FROM ",kphis,".opd_er_order_master WHERE bedno = ?",opd_er_order_master_id,") AS exs;"
    ].concat()
}

// INSERT INTO kphis.opd_er_order_master (note,vn,bedno,er_patient_status_id,order_date,order_time,order_doctor,create_user,create_datetime,update_user,update_datetime,version)
// VALUES (?,?,?,1,NOW(),NOW(),?,?,NOW(),?,NOW(),1)
/// note, vn, bedno, order_doctor, loginname, loginname
pub fn insert_order_master(kphis: &str) -> String {
    [
        "INSERT INTO ",kphis,".opd_er_order_master (note,vn,bedno,er_patient_status_id,order_date,order_time,order_doctor",TABLE_CREATE_COLUMNS,") \
        VALUES (?,?,?,1,NOW(),NOW(),?",TABLE_CREATE_PREPARED,")"
    ].concat()
}

// UPDATE kphis.opd_er_order_master SET note=?,vn=?,bedno=?,er_patient_status_id=?,er_dch_type_id=?,order_doctor=?,update_user=?,
// discharge_date=?,discharge_time=?,update_datetime=NOW(),version=(version+1) WHERE opd_er_order_master_id=?;
/// note, vn, bedno, er_patient_status_id, er_dch_type_id, order_doctor, loginname, discharge_date, discharge_time, opd_er_order_master_id
pub fn update_order_master(kphis: &str) -> String {
    [
        "UPDATE ",kphis,".opd_er_order_master SET note=?,vn=?,bedno=?,er_patient_status_id=?,er_dch_type_id=?,order_doctor=?,update_user=?,\
        discharge_date=?,discharge_time=?,update_datetime=NOW(),version=(version+1) WHERE opd_er_order_master_id=?;"
    ].concat()
}