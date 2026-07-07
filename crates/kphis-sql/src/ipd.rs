pub mod admission_note_dr;
pub mod admission_note_nurse;
pub mod consult;
pub mod dc_plan;
pub mod dc_plan_tmp;
pub mod doctor_in_charge;
pub mod document;
pub mod focus_list;
pub mod focus_note;
pub mod his;
pub mod index_action;
pub mod index_monitor;
pub mod index_note;
pub mod index_plan;
pub mod io;
pub mod med_reconcile;
pub mod mra;
pub mod order;
pub mod passcode;
pub mod progress_note;
pub mod show_patient_main;
pub mod summary;
pub mod summary_audit;
pub mod tmp;
pub mod vital_sign;

use std::cmp::Ordering;

pub fn and_ipt_patient(
    patient_opt: &Option<String>,
    hlen: usize,
    alen: usize,
    hosxp: &str,
) -> Option<String> {
    patient_opt.as_ref().and_then(|s| urlencoding::decode(s).ok()).map(|patient| {
        let pt_len = patient.len();
        match patient.parse::<u64>().is_ok() {
            true => {
                if pt_len == 13 {
                    [
                        " AND (ipt.hn IN (SELECT hn FROM ",hosxp,".patient WHERE cid=?)) "
                    ].concat() // patient
                } else {
                    match hlen.cmp(&alen) {
                        Ordering::Equal => " AND ipt.hn LIKE ? OR ipt.an LIKE ? ", // patient, patient
                        Ordering::Less => match pt_len.cmp(&hlen) {
                            Ordering::Greater => " AND ipt.an LIKE ? ", // patient
                            Ordering::Equal
                            |Ordering::Less => " AND ipt.hn LIKE ? ",                 // patient
                        },
                        Ordering::Greater => match pt_len.cmp(&alen) {
                            Ordering::Greater => " AND ipt.hn LIKE ? ", // patient
                            Ordering::Equal
                            | Ordering::Less => " AND ipt.an LIKE ? ",                 // patient
                        },
                    }
                    .to_owned()
                }
            }
            // false => " AND CONCAT(TRIM(patient.pname),TRIM(patient.fname),' ',TRIM(patient.lname)) LIKE ? ", // patient
            false => [
                " AND (ipt.hn IN (SELECT hn FROM ",hosxp,".patient WHERE CONCAT(TRIM(pname),TRIM(fname),' ',TRIM(lname)) LIKE ?)) "
            ].concat(), // patient
        }
    })
}

// from HosXp v3 'ระบบผู้ป่วยใน -> ลงผลการวินิจฉัย/การทำหัตถการ -> รายชื่อผู้ป่วยใน'
// SELECT
//   ipt.*,
//   substring(concat(spclty.name,' - ',w.name),1,200) AS sname,
//   iptadm.bedno, iptadm.bedtype, roomno.name AS roomname,
//   iptadm.roomno, iptdiag.icd10, concat(iptdiag.icd10,' - ',i1.name) AS icdname,
//   concat(patient.pname,patient.fname,' ',patient.lname) AS pname,
//   aa.income AS income, ptt.pcode AS rtcode, ptt.name AS rtname, dt.name AS admdoctor_name,
//   ft.name AS finance_status_name, aa.admdate, aa.age_y, aa.age_m, aa.age_d, aa.paid_money,
//   aa.rcpt_money, fs.finance_status, dc1.name AS dchtype_name, dc2.name AS dchstts_name,
//   aa.paid_money-aa.rcpt_money AS wait_paid_money, aa.rcpt_money AS ipt_rcpt_money,
//   di.name AS incharge_doctor_name,
//   IF(ipt.dw_hhc_list_id>0,'Y','N') AS hhc_send_status
// FROM ipt
//   LEFT JOIN spclty ON spclty.spclty=ipt.spclty
//   LEFT JOIN iptadm ON iptadm.an=ipt.an
//   LEFT JOIN patient ON patient.hn=ipt.hn
//   LEFT JOIN doctor dt ON dt.code = ipt.admdoctor
//   LEFT JOIN roomno ON roomno.roomno=iptadm.roomno
//   LEFT JOIN iptdiag ON iptdiag.an=ipt.an AND iptdiag.diagtype='1'
//   LEFT JOIN icd101 i1 ON i1.code=substring(iptdiag.icd10,1,3)
//   LEFT JOIN an_stat aa ON aa.an=ipt.an
//   LEFT JOIN ward w ON w.ward=ipt.ward
//   LEFT JOIN dchtype dc1 ON dc1.dchtype=ipt.dchtype
//   LEFT JOIN dchstts dc2 ON dc2.dchstts=ipt.dchstts
//   LEFT JOIN ipt_finance_status fs ON fs.an=ipt.an
//   LEFT JOIN finance_status ft ON ft.finance_status=fs.finance_status
//   LEFT JOIN doctor di ON di.code=ipt.incharge_doctor
//   left outer join pttype ptt ON ptt.pttype=ipt.pttype
// WHERE ipt.ward = '01' AND ipt.dchstts IS NULL
// ORDER BY ipt.regdate, ipt.regtime;
