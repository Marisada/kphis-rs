use std::{cmp::Ordering, str::FromStr};

use kphis_model::{ipd::summary::AuditStatus, post_admit::PostAdmitParams};

// SELECT ipt.hn,ipt.an,ipt.regdate,ipt.regtime,CONCAT(p.pname,p.fname,' ',p.lname) AS fullname,
//     dt.name AS admdoctor_name,aa.age_y,aa.age_m,aa.age_d,dct.name AS dchtype_name,dcs.name AS dchstts_name,ipt.dchdate,ipt.dchtime,ds.name AS discharge_doctor_name,
//     (SELECT MAX(CONCAT(order_date,' ',order_time)) FROM kphis.ipd_order WHERE ipd_order.an=ipt.an AND ipd_order.order_confirm='Y' AND ipd_order.order_owner_type='doctor') AS max_order_datetime,
//     (SELECT MAX(CONCAT(progress_note_date,' ',progress_note_time)) FROM kphis.ipd_progress_note ipn WHERE ipn.an=ipt.an AND ipn.progress_note_owner_type='doctor') AS max_progress_note_datetime,
//     (SELECT MAX(vs_datetime) FROM kphis.ipd_vs_vital_sign vs WHERE vs.an=ipt.an) AS max_vs_datetime,
//     IF(wp.ward IS NOT NULL,1,0) AS wp_status,summary_2.status AS summary_status,
//     (SELECT EXISTS(SELECT * FROM kphis.ipd_dr_admission_note_item dani WHERE dani.admission_note_id=dan.admission_note_id)) AS dr_admission_note_exists,,
//     (SELECT EXISTS(SELECT * FROM kphis.ipd_summary_attending_doctor atd WHERE atd.summary_id=summary_2.summary_id)) AS attending_doctor_exists,
//     (SELECT EXISTS(SELECT * FROM kphis.ipd_summary_approve_doctor apd WHERE apd.summary_id=summary_2.summary_id)) AS approve_doctor_exists
// FROM hos.ipt
//     LEFT JOIN hos.patient p ON p.hn=ipt.hn
//     LEFT JOIN hos.doctor dt ON dt.code=ipt.admdoctor
//     LEFT JOIN hos.doctor ds ON ds.code=ipt.dch_doctor
//     LEFT JOIN hos.an_stat aa ON aa.an=ipt.an
//     LEFT JOIN hos.dchtype dct ON dct.dchtype=ipt.dchtype
//     LEFT JOIN hos.dchstts dcs ON dcs.dchstts=ipt.dchstts
//     LEFT JOIN kphis.ipd_ward_passcode wp ON wp.ward=ipt.ward
//     LEFT JOIN kphis.ipd_summary_2 summary_2 ON summary_2.an=ipt.an
//     LEFT JOIN kphis.ipd_dr_admission_note dan ON dan.an=ipt.an
// WHERE ipt.dchstts IS NOT NULL GROUP BY ipt.an ORDER BY ipt.dchdate DESC,ipt.an ASC LIMIT 100;
/// with patient: (passcode), (if hlan = alen: patient x2 else: patient x1)<br>
/// without patient: (passcode), (ward), (inscl), (adm_doctor), (dch_doctor), (start_dchdate), (end_dchdate)
pub fn select_post_admit_list(params: &PostAdmitParams, hlen: usize, alen: usize, hosxp: &str, kphis: &str, kphis_extra: &str) -> String {
    // params.summary_status is None => ""
    // params.summary_status is Some(valid) => " AND .."
    // params.summary_status is Some(invalid) => ""
    let (summary_status, having) = params.summary_status.as_ref().and_then(|s| {
        AuditStatus::from_str(s).map(|audit_status| audit_status.sql_where_having()).ok()
    }).unwrap_or_default();
    let ward = if params.ward.is_some() {" AND ipt.ward=?"} else {""};
    let inscl = if params.inscl.is_some() {" AND ptt.hipdata_code=?"} else {""};
    let adm_doctor = if params.adm_doctor.is_some() {" AND ipt.admdoctor=?"} else {""};
    let dch_doctor = if params.dch_doctor.is_some() {" AND ipt.dch_doctor=?"} else {""};
    let passcode = if params.passcode.is_some() {
        [" AND (wp.passcode=? OR (wp.passcode IS NULL AND ipt.ward NOT IN (SELECT ward FROM ",kphis,".ipd_ward_passcode)))"].concat()
    } else {
        [" AND wp.passcode IS NULL AND ipt.ward NOT IN (SELECT ward FROM ",kphis,".ipd_ward_passcode)"].concat()
    };
    let start_dchdate = if params.start_dchdate.is_some() {" AND ipt.dchdate>=?"} else {""};
    let end_dchdate = if params.end_dchdate.is_some() {" AND ipt.dchdate<=?"} else {""};

    let not_patient = [&summary_status, ward, inscl, adm_doctor, dch_doctor, start_dchdate, end_dchdate].concat();
    let where_str = params.patient.as_ref().map(|pt| where_patient(pt, hlen, alen)).unwrap_or(not_patient.as_str());

    [
        "SELECT ipt.hn,ipt.an,w.`name` AS ward_name,ipt.regdate,ipt.regtime,CONCAT(p.pname,p.fname,' ',p.lname) AS fullname,sex.`name` AS sex_name,ptt.pcode AS rtcode,ptt.name AS rtname,\
            dt.`name` AS admdoctor_name,aa.age_y,aa.age_m,aa.age_d,ipt.dchtype,ipt.dchstts,dct.`name` AS dchtype_name,dcs.`name` AS dchstts_name,ipt.dchdate,ipt.dchtime,ds.`name` AS dchdoctor_name,\
            (SELECT MAX(ADDTIME(CONVERT(order_date,DATETIME),order_time)) FROM ",kphis,".ipd_order WHERE ipd_order.an=ipt.an AND ipd_order.order_confirm='Y') AS max_order_datetime,\
            (SELECT MAX(ADDTIME(CONVERT(progress_note_date,DATETIME),progress_note_time)) FROM ",kphis,".ipd_progress_note ipn WHERE ipn.an=ipt.an AND ipn.progress_note_owner_type='doctor') AS max_progress_note_datetime,\
            IF(wp.ward IS NOT NULL,1,0) AS wp_status,summary_2.status AS summary_status,\
            (SELECT EXISTS(SELECT * FROM ",kphis,".ipd_dr_admission_note_item dani WHERE dani.admission_note_id=dan.admission_note_id)) AS dr_admission_note_exists,\
            (SELECT EXISTS(SELECT * FROM ",kphis,".ipd_summary_attending_doctor atd WHERE atd.summary_id=summary_2.summary_id)) AS attending_doctor_exists,\
            (SELECT EXISTS(SELECT * FROM ",kphis,".ipd_summary_approve_doctor apd WHERE apd.summary_id=summary_2.summary_id)) AS approve_doctor_exists,\
            (SELECT COUNT(*) FROM ",kphis_extra,".ipd_summary_audit isa WHERE isa.summary_id=summary_2.summary_id) AS summary_audit_count,\
            (SELECT COUNT(*) FROM ",kphis_extra,".ipd_mra mra WHERE mra.an=ipt.an) AS mra_count \
        FROM ",hosxp,".ipt \
            LEFT JOIN ",hosxp,".patient p ON p.hn=ipt.hn \
            LEFT JOIN ",hosxp,".sex ON sex.code=p.sex \
            LEFT JOIN ",hosxp,".doctor dt ON dt.code=ipt.admdoctor \
            LEFT JOIN ",hosxp,".doctor ds ON ds.code=ipt.dch_doctor \
            LEFT JOIN ",hosxp,".an_stat aa ON aa.an=ipt.an \
            LEFT JOIN ",hosxp,".ward w ON w.ward=ipt.ward \
            LEFT JOIN ",hosxp,".pttype ptt ON ptt.pttype=ipt.pttype \
            LEFT JOIN ",hosxp,".dchtype dct ON dct.dchtype=ipt.dchtype \
            LEFT JOIN ",hosxp,".dchstts dcs ON dcs.dchstts=ipt.dchstts \
            LEFT JOIN ",kphis,".ipd_ward_passcode wp ON wp.ward=ipt.ward \
            LEFT JOIN ",kphis,".ipd_summary_2 summary_2 ON summary_2.an=ipt.an \
            LEFT JOIN ",kphis,".ipd_dr_admission_note dan ON dan.an=ipt.an \
        WHERE ipt.dchstts IS NOT NULL ", &passcode, where_str,
        " GROUP BY ipt.an HAVING max_order_datetime IS NOT NULL ", having," ORDER BY ipt.dchdate ASC,ipt.an ASC LIMIT 500;"
    ].concat()
}

// Conditions
// - Discharged in HOSxP (has ipt.dchstts)
// - Has KPHIS confirmed order (any)
// - Not summarized (no summary_2.status and no signing of attending doctor)
// - Need review (summary_2.status == review)
/// doctorcode
pub fn select_post_admit_count(hosxp: &str, kphis: &str) -> String {
    [
        "SELECT COUNT(ipt.an)
        FROM ",hosxp,".ipt \
            LEFT JOIN ",kphis,".ipd_summary_2 summary_2 ON summary_2.an=ipt.an \
        WHERE ipt.dch_doctor=? AND ipt.dchstts IS NOT NULL AND EXISTS(SELECT * FROM ",kphis,".ipd_order WHERE ipd_order.an=ipt.an AND ipd_order.order_confirm='Y') AND (\
            (summary_2.status IS NULL AND NOT EXISTS(SELECT * FROM ",kphis,".ipd_summary_attending_doctor atd WHERE atd.summary_id=summary_2.summary_id)) \
            OR summary_2.status='review');"
    ].concat()
            // // for `discharged` (not offed) in KPHIS. Against advice will not included
            // JOIN ",kphis,".ipd_order_item oi ON ipd_order.order_id=oi.order_id AND oi.order_item_type='discharge' \
            //     LEFT JOIN ",kphis,".ipd_order_item ooi ON ooi.order_item_id=oi.off_order_item_id \
            // WHERE ipd_order.an=ipt.an AND ipd_order.order_confirm='Y' AND ooi.order_item_id IS NULL
}

/// patient x 1 (except hlen == alen need patient x2)
pub fn where_patient(patient: &str, hlen: usize, alen: usize) -> &'static str {
    match patient.parse::<u64>().is_ok() {
        true => {
            let pt_len = patient.len();
            if pt_len == 13 {
                " AND p.cid=?"
            } else {
                match hlen.cmp(&alen) {
                    Ordering::Equal => " AND ipt.hn LIKE ? OR ipt.an LIKE ?",
                    Ordering::Less => match pt_len.cmp(&hlen) {
                        Ordering::Greater => " AND ipt.an LIKE ?",
                        Ordering::Equal
                            | Ordering::Less => " AND ipt.hn LIKE ?",
                    },
                    Ordering::Greater => match pt_len.cmp(&alen) {
                        Ordering::Greater => " AND ipt.hn LIKE ?",
                        Ordering::Equal
                            | Ordering::Less => " AND ipt.an LIKE ?",
                    },
                }
            }
        }
        false => " AND CONCAT(TRIM(p.pname),TRIM(p.fname),' ',TRIM(p.lname)) LIKE ?",
    }
}