use std::cmp::Ordering;

use kphis_model::pre_admit::PreAdmitParams;

use crate::{TABLE_CREATE_COLUMNS, TABLE_CREATE_PREPARED, TABLE_UPDATE_SET};

// SELECT pam.vn,pam.an,w.name AS ward_name,vn_stat.age_y,vn_stat.age_m,vn_stat.age_d,ovst.hn,ovst.vstdate,ovst.vsttime,
// 	CONCAT(p.pname,p.fname,' ',p.lname) AS fullname,
// 	(SELECT GROUP_CONCAT(DISTINCT d.`name` ORDER BY o.order_date DESC, o.order_time DESC SEPARATOR ', ') FROM kphis.ipd_order o LEFT JOIN hos.doctor d ON o.order_doctor = d.`code`
//    	WHERE o.an=pam.an AND o.order_confirm='Y' GROUP BY o.an) AS all_order_doctor_name,
// 	(SELECT MAX(CONCAT(order_date,' ',order_time)) FROM kphis.ipd_order WHERE ipd_order.an=ipt.an AND ipd_order.order_confirm='Y' AND ipd_order.order_owner_type='doctor') AS max_order_datetime,
// 	IF(((SELECT COUNT(*) FROM kphis.ipd_dr_admission_note_item dani WHERE dani.admission_note_id=dan.admission_note_id) > 0),1,NULL) AS dr_admission_note_count
// FROM kphis.ipd_pre_admit_master pam
// 	LEFT JOIN hos.ovst ON ovst.vn=pam.vn
// 	LEFT JOIN hos.vn_stat ON vn_stat.vn=pam.vn
// 	LEFT JOIN hos.patient p ON p.hn=ovst.hn
// 	LEFT JOIN hos.ipt ON ipt.an=pam.an
// 	LEFT JOIN hos.ward w ON w.ward=ipt.ward
// 	LEFT JOIN kphis.ipd_dr_admission_note dan ON dan.an=pam.vn
/// (doctor_in_charge), (if hlan = alen: patient x2 else: patient x1)
pub fn select_pre_admit(params: &PreAdmitParams, hlen: usize, alen: usize, hosxp: &str, kphis: &str) -> String {
    let is_all = params.all.as_ref().map(|s| !s.is_empty()).unwrap_or_default();
    let (status, pam_col, vn_from) = params.status.as_ref().map(|s| {
        match s.as_str() {
            "admited" => (if is_all {" AND pam.an IS NOT NULL"} else {" AND pam.an IS NOT NULL AND ipt.dchstts IS NULL"}, "an", "ipt"),
            "revoked" => (" AND pam.an IS NULL AND pam.prev_an IS NOT NULL", "vn", "pam"),
            "pre" => (if is_all {" AND pam.an IS NULL AND pam.prev_an IS NULL"} else {" AND pam.an IS NULL AND pam.prev_an IS NULL AND DATE_ADD(pam.update_datetime, INTERVAL 3 DAY) > NOW()"}, "vn", "pam"),
            _ => (" AND 1=0", "vn", "pam"),
        }
    }).unwrap_or((" AND 1=0", "vn", "pam"));
    let doctor_in_charge = if params.doctor_in_charge.is_some() {
        [" AND (SELECT EXISTS(SELECT * FROM kphis.ipd_order o WHERE o.an=pam.",pam_col," AND o.order_confirm='Y' AND o.order_doctor=?))"].concat()
    } else {String::new()};
    let patient = params.patient.as_ref().map(|pt| where_patient(pt, hlen, alen)).unwrap_or_default();
    [
        "SELECT pam.vn,pam.an,w.name AS ward_name,vn_stat.age_y,vn_stat.age_m,vn_stat.age_d,ovst.hn,ovst.vstdate,ovst.vsttime,\
            CONCAT(p.pname,p.fname,' ',p.lname) AS fullname,ptt.pcode AS rtcode,ptt.name AS rtname,sex.`name` AS sex_name,\
            (SELECT GROUP_CONCAT(DISTINCT d.`name` ORDER BY o.order_date DESC, o.order_time DESC SEPARATOR ', ') FROM ",kphis,".ipd_order o LEFT JOIN ",hosxp,".doctor d ON o.order_doctor = d.`code` \
                WHERE o.an=pam.",pam_col," AND o.order_confirm='Y' GROUP BY o.an) AS all_order_doctor_name,\
            (SELECT MAX(ADDTIME(CONVERT(order_date,DATETIME),order_time)) FROM ",kphis,".ipd_order WHERE ipd_order.an=pam.",pam_col," AND ipd_order.order_confirm='Y' AND ipd_order.order_owner_type='doctor') AS max_order_datetime,\
            (SELECT EXISTS(SELECT * FROM ",kphis,".ipd_dr_admission_note_item dani WHERE dani.admission_note_id=dan.admission_note_id)) AS dr_admission_note_exists,\
            (SELECT COUNT(*) FROM ",kphis,".ipd_med_reconciliation mr WHERE mr.an=pam.",pam_col," AND mr.phamacist_confirm_datetime IS NOT NULL AND mr.doctor_confirm_datetime IS NULL GROUP BY mr.an) AS mr_unconfirmed_count,\
            (SELECT COUNT(*) FROM ",kphis,".ipd_med_reconciliation mr WHERE mr.an=pam.",pam_col," AND mr.phamacist_confirm_datetime IS NOT NULL AND mr.doctor_confirm_datetime IS NOT NULL GROUP BY mr.an) AS mr_confirmed_count \
        FROM ",kphis,".ipd_pre_admit_master pam \
            LEFT JOIN ",hosxp,".ipt ON ipt.an=pam.an \
            LEFT JOIN ",hosxp,".ovst ON ovst.vn=",vn_from,".vn \
            LEFT JOIN ",hosxp,".vn_stat ON vn_stat.vn=",vn_from,".vn \
            LEFT JOIN ",hosxp,".patient p ON p.hn=ovst.hn \
            LEFT JOIN ",hosxp,".sex ON sex.code=p.sex \
            LEFT JOIN ",hosxp,".pttype ptt ON ptt.pttype=ovst.pttype \
            LEFT JOIN ",hosxp,".ward w ON w.ward=ipt.ward \
            LEFT JOIN ",kphis,".ipd_dr_admission_note dan ON dan.an=pam.",pam_col,
        // visit is not removed
        " WHERE ovst.hn IS NOT NULL", status, &doctor_in_charge, patient, " ORDER BY ovst.vstdate DESC,ovst.vsttime DESC LIMIT 100;"
    ].concat()
}

// SELECT * FROM ipt_pre_admit_master WHERE an=?;
/// an
pub fn select_pre_admit_by_an(kphis: &str) -> String {
    ["SELECT * FROM ",kphis,".ipd_pre_admit_master WHERE an=?;"].concat()
}

// SELECT * FROM ipt_pre_admit_master WHERE vn=?;
/// an
pub fn select_pre_admit_by_vn(kphis: &str) -> String {
    ["SELECT * FROM ",kphis,".ipd_pre_admit_master WHERE vn=?;"].concat()
}

// INSERT IGNORE INTO kphis.ipd_pre_admit_master (vn,create_user,create_datetime,update_user,update_datetime,version) VALUES (?,?,NOW(),?,NOW(),1);
/// vn, loginname, loginname
pub fn insert_pre_admit(kphis: &str) -> String {
    ["INSERT IGNORE INTO ",kphis,".ipd_pre_admit_master (vn",TABLE_CREATE_COLUMNS,") VALUES (?",TABLE_CREATE_PREPARED,");"].concat()
}

// INSERT IGNORE INTO kphis.ipd_pre_admit_master (vn,prev_an,create_user,create_datetime,update_user,update_datetime,version) VALUES (?,?,?,NOW(),?,NOW(),1);
/// vn, revoked_an, loginname, loginname
pub fn insert_revoked_pre_admit(kphis: &str) -> String {
    ["INSERT IGNORE INTO `",kphis,"`.`ipd_pre_admit_master` (vn,prev_an",TABLE_CREATE_COLUMNS,") VALUES (?,?",TABLE_CREATE_PREPARED,");"].concat()
}

// UPDATE `kphis`.`ipd_pre_admit_master` SET an=?,prev_an=?,update_user=?,update_datetime=NOW(),version=(version+1) WHERE vn=?;
/// an, prev_an, loginname, where_vn
pub fn update_pre_admit(kphis: &str) -> String {
    ["UPDATE `",kphis,"`.`ipd_pre_admit_master` SET an=?,prev_an=?",TABLE_UPDATE_SET," WHERE vn=?;"].concat()
} 

/// patient x 1 (except hlen == alen need patient x2)
fn where_patient(patient: &str, hlen: usize, alen: usize) -> &'static str {
    match patient.parse::<u64>().is_ok() {
        true => {
            let pt_len = patient.len();
            if pt_len == 13 {
                " AND p.cid=?"
            } else {
                match hlen.cmp(&alen) {
                    Ordering::Equal => " AND ovst.hn LIKE ? OR pam.an LIKE ?",
                    Ordering::Less => match pt_len.cmp(&hlen) {
                        Ordering::Greater => " AND pam.an LIKE ?",
                        Ordering::Equal
                            | Ordering::Less => " AND ovst.hn LIKE ?",
                    },
                    Ordering::Greater => match pt_len.cmp(&alen) {
                        Ordering::Greater => " AND ovst.hn LIKE ?",
                        Ordering::Equal
                            | Ordering::Less => " AND pam.an LIKE ?",
                    },
                }    
            }
        }
        false => " AND CONCAT(TRIM(p.pname),TRIM(p.fname),' ',TRIM(p.lname)) LIKE ?",
    }
}