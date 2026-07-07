pub mod ipd_search_patient_dr;
pub mod ipd_search_patient_nurse;
pub mod ipd_search_patient_other;
pub mod ipd_search_patient_pharmacist;
pub mod searchbox;

use std::cmp::Ordering;

trait FilterPatient {
    fn has_patient(self) -> Self;
    fn pt_is_num(self) -> Self;
    fn anlen_eq_hnlen(self) -> Self;
}

trait FilterWard {
    fn has_ward(self) -> Self;
}

trait FilterDoctor {
    fn has_doctor(self) -> Self;
}

trait FilterConsult {
    fn has_consult(self) -> Self;
}

trait FilterPasscode {
    fn has_passcode(self) -> Self;
}

fn where_and_filter_patient<F>(patient: &str, hlen: usize, alen: usize) -> (String, F)
where
    F: FilterPatient + Default,
{
    let filter = F::default().has_patient();
    let (where_pt, filter) = match patient.parse::<u64>().is_ok() {
        true => {
            let pt_len = patient.len();
            if pt_len == 13 {
                (" WHERE patient.cid=?", filter.pt_is_num())
            } else {
                match hlen.cmp(&alen) {
                    Ordering::Equal => (
                        " WHERE ipt.hn LIKE ? OR ipt.an LIKE ?",
                        filter.anlen_eq_hnlen(),
                    ), // patient, patient
                    Ordering::Less => match pt_len.cmp(&hlen) {
                        Ordering::Greater => (" WHERE ipt.an LIKE ?", filter), // patient
                        Ordering::Equal
                            | Ordering::Less => (" WHERE ipt.hn LIKE ?", filter),                 // patient
                    },
                    Ordering::Greater => match pt_len.cmp(&alen) {
                        Ordering::Greater => (" WHERE ipt.hn LIKE ?", filter), // patient
                        Ordering::Equal
                            | Ordering::Less => (" WHERE ipt.an LIKE ?", filter),                 // patient
                    },
                }
            }
        }
        false => (
            " WHERE CONCAT(TRIM(patient.pname),TRIM(patient.fname),' ',TRIM(patient.lname)) LIKE ?",
            filter,
        ), // patient
    };
    (where_pt.to_owned(), filter)
}

fn where_no_patient() -> &'static str {
    " WHERE (om.opd_er_order_master_id IS NULL OR (om.opd_er_order_master_id IS NOT NULL AND om.er_patient_status_id=7)) AND ipt.dchstts IS NULL"
}

fn where_and_filter_ward<F>(ward: &Option<String>, sql: &str, filter: F) -> (String, F)
where
    F: FilterWard + Default,
{
    match ward {
        Some(_ward) => ([sql, " AND ipt.ward=?"].concat(), filter.has_ward()), // ward
        None => (sql.to_owned(), filter),
    }
}

fn where_and_filter_doctor<F>(
    doctor: &Option<String>,
    kphis: &str,
    sql: &str,
    filter: F,
) -> (String, F)
where
    F: FilterDoctor + Default,
{
    match doctor {
        Some(_doctor) => (
            [sql," AND ipt.an IN (SELECT an FROM ",kphis,".ipd_doctor_in_charge WHERE doctor=?)"].concat(),
            filter.has_doctor(),
        ), // doctor
        None => (sql.to_owned(), filter),
    }
}

fn where_and_filter_consult<F>(
    consult: &Option<String>,
    kphis: &str,
    sql: &str,
    filter: F,
) -> (String, F)
where
    F: FilterConsult + Default,
{
    match consult {
        Some(_consult) => (
            [
                sql," AND ipt.an IN (SELECT an FROM ",kphis,".ipd_dr_consult_signature_reply \
                WHERE consult_doctorcode_reply=? OR consult_doctorcode_reply_person2=?)"
            ].concat(),
            filter.has_consult(),
        ), // consult, consult
        None => (sql.to_owned(), filter),
    }
}

fn where_and_filter_passcode<F>(
    passcode: &Option<String>,
    kphis: &str,
    sql: &str,
    filter: F,
) -> (String, F)
where
    F: FilterPasscode + Default,
{
    match passcode {
        // original is " AND (wp.passcode = ? OR wp.passcode IS NULL OR ipt.dchstts IS NOT NULL)"
        Some(_passcode) => ([sql," AND (wp.passcode=? OR (wp.passcode IS NULL AND ipt.ward NOT IN (SELECT ward FROM ",kphis,".ipd_ward_passcode)))"].concat(), filter.has_passcode()), // passcode
        // original is " AND (wp.passcode IS NULL OR ipt.dchstts IS NOT NULL)"
        None => ([sql," AND wp.passcode IS NULL AND ipt.ward NOT IN (SELECT ward FROM ",kphis,".ipd_ward_passcode)"].concat(), filter),
    }
}

fn where_drug_allergy(drug_allergy_check: &str, kphis: &str) -> String {
    match drug_allergy_check {
        "no_admission_note" => String::from(" AND dan.admission_note_id IS NULL "),
        "waiting" => [
            " AND ((dan.admission_note_id IS NOT NULL AND dan.allergy_drug_history IS NOT NULL AND dan.allergy_drug_history <> '' AND dan.allergy_drug_pharmacy_check_person IS NULL) \
                OR (SELECT EXISTS(SELECT * FROM ",kphis,".opd_er_allergy_history ah JOIN ",kphis,".opd_er_order_master om ON ah.opd_er_order_master_id=om.opd_er_order_master_id WHERE om.vn=ipt.vn))) "
        ].concat(),
        "checked" => String::from(" AND dan.allergy_drug_pharmacy_check_person IS NOT NULL AND dan.allergy_drug_pharmacy_check_person <> '' "),
        _ => String::new(),
    }
}

fn group_by_patient(has_patient: bool, sql: &str) -> String {
    match has_patient {
        true => [sql, " GROUP BY ipt.an ORDER BY ipt.an DESC LIMIT 200;"].concat(),
        false => [sql, " GROUP BY ipt.an ORDER BY LEFT(iptadm.bedno,3),MID(iptadm.bedno,4,999),iptadm.bedno,ipt.regdate,ipt.regtime;"].concat(),
    }
}
