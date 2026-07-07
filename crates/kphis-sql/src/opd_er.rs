pub mod dc_plan;
pub mod document;
pub mod focus_list;
pub mod focus_note;
pub mod hosxp_med;
pub mod index_action;
pub mod index_monitor;
pub mod index_plan;
pub mod io;
pub mod med_reconcile;
pub mod medical_history;
pub mod order;
pub mod order_master;
pub mod progress_note;
pub mod show_patient_main;
pub mod vital_sign;

use std::cmp::Ordering;

/// hn/vn/fullname
pub fn and_opd_patient(
    patient_opt: &Option<String>,
    hlen: usize,
    vlen: usize,
    hosxp: &str,
) -> Option<String> {
    patient_opt.as_ref().and_then(|s| urlencoding::decode(s).ok()).map(|patient| {
        let pt_len = patient.len();
        match patient.parse::<u64>().is_ok() {
            true => {
                if pt_len == 13 {
                    [
                        " AND (ovst.hn IN (SELECT hn FROM ",hosxp,".patient WHERE cid=?)) "
                    ].concat() // patient
                } else {
                    match hlen.cmp(&vlen) {
                        Ordering::Equal => " AND ovst.hn LIKE ? OR ovst.vn LIKE ? ", // patient, patient
                        Ordering::Less => match pt_len.cmp(&hlen) {
                            Ordering::Greater => " AND ovst.vn LIKE ? ", // patient
                            Ordering::Equal
                            | Ordering::Less => " AND ovst.hn LIKE ? ",                 // patient
                        },
                        Ordering::Greater => match pt_len.cmp(&vlen) {
                            Ordering::Greater => " AND ovst.hn LIKE ? ", // patient
                            Ordering::Equal
                            | Ordering::Less => " AND ovst.vn LIKE ? ",                 // patient
                        },
                    }
                    .to_owned()
                }
            }
            // false => " AND CONCAT(TRIM(patient.pname),TRIM(patient.fname),' ',TRIM(patient.lname)) LIKE ? ", // patient
            false => [
                " AND (ovst.hn IN (SELECT hn FROM ",hosxp,".patient WHERE CONCAT(TRIM(pname),TRIM(fname),' ',TRIM(lname)) LIKE ?)) "
            ].concat(), // patient
        }
    })
}

/// qn/hn/vn/fullname<br>
/// search text.len() < 5 = QN
pub fn or_opd_patient_or_qn(
    patient_opt: &Option<String>,
    hlen: usize,
    vlen: usize,
    hosxp: &str,
) -> Option<String> {
    patient_opt.as_ref().and_then(|s| urlencoding::decode(s).ok()).map(|patient| {
        let pt_len = patient.len();
        match patient.parse::<u64>().is_ok() {
            true => {
                if pt_len == 13 {
                    [
                        " OR (ovst.hn IN (SELECT hn FROM ",hosxp,".patient WHERE cid=?)) "
                    ].concat() // patient
                } else if pt_len < 5 {
                    " OR ovst.vstdate = DATE(NOW()) AND ovst.oqueue=? ".to_owned() // patient
                } else {
                    match hlen.cmp(&vlen) {
                        Ordering::Equal => " OR ovst.hn LIKE ? OR ovst.vn LIKE ? ", // patient, patient
                        Ordering::Less => match pt_len.cmp(&hlen) {
                            Ordering::Greater => " OR ovst.vn LIKE ? ", // patient
                            Ordering::Equal
                            | Ordering::Less => " OR ovst.hn LIKE ? ",                 // patient
                        },
                        Ordering::Greater => match pt_len.cmp(&vlen) {
                            Ordering::Greater => " OR ovst.hn LIKE ? ", // patient
                            Ordering::Equal
                            | Ordering::Less => " OR ovst.vn LIKE ? ",                 // patient
                        },
                    }.to_owned()
                }
            }
            // false => " OR CONCAT(TRIM(patient.pname),TRIM(patient.fname),' ',TRIM(patient.lname)) LIKE ? ", // patient
            false => [
                " OR (ovst.hn IN (SELECT hn FROM ",hosxp,".patient WHERE CONCAT(TRIM(pname),TRIM(fname),' ',TRIM(lname)) LIKE ?)) "
            ].concat(), // patient
        }
    })
}
