// canAccessTable
// SELECT TABLE_NAME
// FROM information_schema.TABLES
// WHERE TABLE_SCHEMA = ? AND TABLE_NAME = ?;
/// db_name, table_name
pub fn can_access_table() -> &'static str {
    "SELECT TABLE_NAME FROM information_schema.TABLES WHERE TABLE_SCHEMA = ? AND TABLE_NAME = ?;"
}
// getOpdErAllergyWithSymptomByAn
// # IF($with_symptom){
// SELECT GROUP_CONCAT(CONCAT(er_allergy_history_agent,'=',IF(er_allergy_history_symptom IS NULL,',',er_allergy_history_symptom))) AS drugallergy
// FROM kphis.opd_er_allergy_history
// JOIN opd_er_order_master om ON om.opd_er_order_master_id = opd_er_allergy_history.opd_er_order_master_id
// WHERE om.vn = ?
// ORDER BY er_allergy_history_id;
// # ELSE
// SELECT GROUP_CONCAT(er_allergy_history_agent) AS drugallergy
// FROM kphis.opd_er_allergy_history
// JOIN opd_er_order_master om ON om.opd_er_order_master_id = opd_er_allergy_history.opd_er_order_master_id
// WHERE om.vn = ?
// ORDER BY er_allergy_history_id;
/// vn
pub fn get_opd_er_allergy_with_symptom_by_vn(
    with_symptom: bool,
    kphis: &str,
) -> String {
    let select = if with_symptom {
        "SELECT GROUP_CONCAT(CONCAT(ah.er_allergy_history_agent,'=',IF(ah.er_allergy_history_symptom IS NULL,',',ah.er_allergy_history_symptom))) AS drugallergy "
    } else {
        "SELECT GROUP_CONCAT(ah.er_allergy_history_agent) AS drugallergy "
    };
    [
        select,
        "FROM ",kphis,".opd_er_allergy_history ah \
            JOIN ",kphis,".opd_er_order_master om ON om.opd_er_order_master_id=ah.opd_er_order_master_id \
        WHERE om.vn=? \
        ORDER BY ah.er_allergy_history_id;"
    ].concat()
}
// getOpdErAllergyListByAn
// SELECT ah.er_allergy_history_agent,ah.er_allergy_history_symptom
// FROM kphis.opd_er_allergy_history ah
// JOIN kphis.opd_er_order_master om ON om.opd_er_order_master_id=ah.opd_er_order_master_id
// WHERE om.vn=?
// ORDER BY ah.er_allergy_history_id;
/// vn
pub fn get_opd_er_allergy_list_by_vn(kphis: &str) -> String {
    [
        "SELECT ah.er_allergy_history_agent,ah.er_allergy_history_symptom \
        FROM ",kphis,".opd_er_allergy_history ah \
            JOIN ",kphis,".opd_er_order_master om ON om.opd_er_order_master_id=ah.opd_er_order_master_id \
        WHERE om.vn=? \
        ORDER BY ah.er_allergy_history_id;"
    ].concat()
}
// isNowPass24HrsFromOpdErDischarge
// SELECT IF(om.discharge_date IS NULL OR om.discharge_time IS NULL,false,IF((DATE_ADD(NOW(),INTERVAL -1 DAY) > CONCAT(om.discharge_date,' ',om.discharge_time)),true,false)) AS pass_24_hour_from_dch
// FROM kphis.opd_er_order_master om
// WHERE opd_er_order_master_id=?;
/// opd_er_order_master_id
pub fn is_now_pass_24_hrs_from_opd_er_discharge(kphis: &str) -> String {
    [
        "SELECT IF(om.discharge_date IS NULL OR om.discharge_time IS NULL,false,IF((DATE_ADD(NOW(),INTERVAL -1 DAY) > CONCAT(om.discharge_date,' ',om.discharge_time)),true,false)) AS pass_24_hour_from_dch \
        FROM ",kphis,".opd_er_order_master om \
        WHERE opd_er_order_master_id=?;"
    ].concat()
}

// // getHnByVn
// // SELECT hn FROM hos.ovst WHERE vn=?;"
// fn get_hn_by_vn(hosxp: &str) -> String {
//     ["SELECT hn FROM ", hosxp, ".ovst WHERE vn=?;"].concat()
// }
// // getHnByAn
// // SELECT hn FROM hos.ipt WHERE an=?;
// fn get_hn_by_an(hosxp: &str) -> String {
//     ["SELECT hn FROM ", hosxp, ".ipt WHERE an=?;"].concat()
// }
// // getPatientName
// // SELECT CONCAT(patient.pname,patient.fname,' ',patient.lname) AS patient_name FROM hos.patient WHERE hn=?;
// fn get_patient_name(hosxp: &str) -> String {
//     ["SELECT CONCAT(patient.pname,patient.fname,' ',patient.lname) AS patient_name FROM ",hosxp,".patient WHERE hn=?;"].concat()
// }
// // getPatientCidAndPassportNo
// // SELECT cid, passport_no FROM hos.patient WHERE hn=?;
// fn get_patient_cid_and_passport_no(hosxp: &str) -> String {
//     ["SELECT cid, passport_no FROM ",hosxp,".patient WHERE hn=?;"].concat()
// }
// // getPatientCid
// // SELECT cid FROM hos.patient WHERE hn=?;
// fn get_patient_cid(hosxp: &str) -> String {
//     ["SELECT cid FROM ", hosxp, ".patient WHERE hn=?;"].concat()
// }
// // getPatientPassportNo
// // SELECT passport_no FROM hos.patient WHERE hn=?;
// fn get_patient_passport_no(hosxp: &str) -> String {
//     ["SELECT passport_no FROM ", hosxp, ".patient WHERE hn=?;"].concat()
// }
// // getVnByAn
// // SELECT vn FROM hos.ipt WHERE an=?;
// fn get_vn_by_an(hosxp: &str) -> String {
//     ["SELECT vn FROM ", hosxp, ".ipt WHERE an=?;"].concat()
// }
// // getAnByVn
// // SELECT an FROM hos.ovst WHERE vn=?;
// fn get_an_by_vn(hosxp: &str) -> String {
//     ["SELECT an FROM ", hosxp, ".ovst WHERE vn=?;"].concat()
// }
// // getRegDateByAn
// // SELECT regdate FROM hos.ipt WHERE an=?;
// fn get_reg_date_by_an(hosxp: &str) -> String {
//     ["SELECT regdate FROM ", hosxp, ".ipt WHERE an=?;"].concat()
// }
// // getDchDateByAn
// // SELECT dchdate FROM hos.ipt WHERE an=?;
// fn get_dch_date_by_an(hosxp: &str) -> String {
//     ["SELECT dchdate FROM ", hosxp, ".ipt WHERE an=?;"].concat()
// }
// // getMinOpdErVsDate
// // SELECT MIN(DATE(vs_datetime)) AS min_vs_date FROM kphis.opd_er_vs_vital_sign WHERE opd_er_order_master_id=?;
// fn get_min_opd_er_vs_date(kphis: &str) -> String {
//     ["SELECT MIN(DATE(vs_datetime)) AS min_vs_date FROM ",kphis,".opd_er_vs_vital_sign WHERE opd_er_order_master_id=?;"].concat()
// }
// // getMaxOpdErVsDate
// // SELECT MAX(DATE(vs_datetime)) AS max_vs_date FROM kphis.opd_er_vs_vital_sign WHERE opd_er_order_master_id=?;
// fn get_max_opd_er_vs_date(kphis: &str) -> String {
//     ["SELECT MAX(DATE(vs_datetime)) AS max_vs_date FROM ",kphis,".opd_er_vs_vital_sign WHERE opd_er_order_master_id=?;"].concat()
// }
// // getHnByPreOrderMasterId
// // SELECT hn FROM kphis.ipd_pre_order_master WHERE pre_order_master_id=?;
// fn get_hn_by_pre_order_master_id(kphis: &str) -> String {
//     ["SELECT hn FROM ",kphis,".ipd_pre_order_master WHERE pre_order_master_id=?;"].concat()
// }
// // getVnByOpdErOrderMasterId
// // SELECT vn FROM kphis.opd_er_order_master WHERE opd_er_order_master_id=?;
// fn get_vn_by_opd_er_order_master_id(kphis: &str) -> String {
//     ["SELECT vn FROM ",kphis,".opd_er_order_master WHERE opd_er_order_master_id=?;"].concat()
// }
// // getDoctorName
// // SELECT `name` FROM hos.doctor WHERE `code` =?;
// fn get_doctor_name(hosxp: &str) -> String {
//     ["SELECT `name` FROM ", hosxp, ".doctor WHERE `code` =?;"].concat()
// }
// // isTodayNotPassDchDate
// // SELECT NOT(dchdate IS NOT NULL AND DATE(NOW()) > dchdate) AS isTodayNotPassDchDate FROM hos.ipt WHERE an=?;
// fn is_today_not_pass_dch_date(hosxp: &str) -> String {
//     ["SELECT NOT(dchdate IS NOT NULL AND DATE(NOW()) > dchdate) AS isTodayNotPassDchDate FROM ",hosxp,".ipt WHERE an=?;"].concat()
// }
// // getFromOpdErDischarge
// // SELECT IF(om.discharge_date IS NULL OR om.discharge_time IS NULL,false,true) AS opd_er_from_dch
// // FROM kphis.opd_er_order_master om
// // WHERE opd_er_order_master_id=?;
// fn get_from_opd_er_discharge(kphis: &str) -> String {
//     [
//         "SELECT IF(om.discharge_date IS NULL OR om.discharge_time IS NULL,false,true) AS opd_er_from_dch \
//         FROM ",kphis,".opd_er_order_master om \
//         WHERE opd_er_order_master_id=?;"
//     ].concat()
// }
// // getDateTimeFromOpdErDischarge
// // SELECT om.discharge_date, om.discharge_time
// // FROM kphis.opd_er_order_master om
// // WHERE opd_er_order_master_id=?;
// fn get_date_time_from_opd_er_discharge(kphis: &str) -> String {
//     [
//         "SELECT om.discharge_date, om.discharge_time \
//         FROM ",kphis,".opd_er_order_master om \
//         WHERE opd_er_order_master_id=?;"
//     ].concat()
// }
// // getOrderDateByOpdErOrderMasterId
// // SELECT order_date FROM kphis.opd_er_order_master WHERE opd_er_order_master_id=?;
// fn get_order_date_by_opd_er_order_master_id(kphis: &str) -> String {
//     ["SELECT order_date FROM ",kphis,".opd_er_order_master WHERE opd_er_order_master_id=?;"].concat()
// }
// // getDbTodayDate
// // SELECT DATE(NOW()) AS todayDate;
// fn get_db_today_date() -> &'static str {
//     "SELECT DATE(NOW()) AS todayDate;"
// }
// // isDbTodayDate
// // SELECT DATE(NOW()) = ? AS isDbTodayDate;
// fn is_db_today_date() -> &'static str {
//     "SELECT DATE(NOW()) = ? AS isDbTodayDate;"
// }
// // checkForeignKeyUsage
// // SELECT COUNT(*) AS cnt FROM $tablename WHERE $fieldname = ?;
// fn check_foreign_key_usage(tablename: &str, fieldname: &str) -> String {
//     ["SELECT COUNT(*) AS cnt FROM ",tablename," WHERE ",fieldname," = ?;"].concat()
// }
// // getOperationHis
// // SELECT CONCAT(operation_list.enter_date,', ',operation_list.operation_name,', ',doctor.name,', ',:hospital_name) AS operation_list,
// // 	TIMESTAMPDIFF(YEAR,operation_list.enter_date,NOW()) AS or_concat_year,
// // 	MOD(TIMESTAMPDIFF(MONTH,operation_list.enter_date,NOW()),12) AS or_concat_month,
// // 	TIMESTAMPDIFF(DAY,DATE_ADD(operation_list.enter_date,INTERVAL (TIMESTAMPDIFF(MONTH,operation_list.enter_date,NOW())) MONTH),NOW()) AS or_concat_day
// // FROM hos.operation_list
// // LEFT JOIN hos.doctor ON doctor.code = operation_list.request_doctor
// // LEFT JOIN hos.ipt ON ipt.an = operation_list.an
// // WHERE operation_list.hn=?
// // 	AND CONCAT(operation_list.enter_date,' ',operation_list.enter_time) < (SELECT CONCAT(ipt.regdate,' ',ipt.regtime) FROM hos.ipt WHERE an=?);
// fn get_operation_his(hosxp: &str) -> String {
//     [
//         "SELECT CONCAT(operation_list.enter_date,', ',operation_list.operation_name,', ',doctor.name,', ',:hospital_name) AS operation_list,\
//             TIMESTAMPDIFF(YEAR,operation_list.enter_date,NOW()) AS or_concat_year,\
//             MOD(TIMESTAMPDIFF(MONTH,operation_list.enter_date,NOW()),12) AS or_concat_month,\
//             TIMESTAMPDIFF(DAY,DATE_ADD(operation_list.enter_date,INTERVAL (TIMESTAMPDIFF(MONTH,operation_list.enter_date,NOW())) MONTH),NOW()) AS or_concat_day \
//         FROM ",hosxp,".operation_list \
//             LEFT JOIN ",hosxp,".doctor ON doctor.code = operation_list.request_doctor \
//             LEFT JOIN ",hosxp,".ipt ON ipt.an = operation_list.an \
//         WHERE operation_list.hn=? \
//             AND CONCAT(operation_list.enter_date,' ',operation_list.enter_time) < (SELECT CONCAT(ipt.regdate,' ',ipt.regtime) FROM ",hosxp,".ipt WHERE an=?);"
//     ].concat()
// }
// // getHosxpOpdAllergy
// // SELECT GROUP_CONCAT(opd_allergy.agent) AS drugallergy
// // FROM hos.opd_allergy
// // WHERE opd_allergy.hn = ? /*AND (opd_allergy.no_alert<>'Y' or opd_allergy.no_alert IS NULL)*/
// // ORDER BY display_order;
// fn get_hosxp_opd_allergy(hosxp: &str) -> String {
//     [
//         "SELECT GROUP_CONCAT(opd_allergy.agent) AS drugallergy \
//         FROM ",hosxp,".opd_allergy \
//         WHERE opd_allergy.hn = ? \
//         ORDER BY display_order;"
//     ].concat()
// }
// // getHosxpOpdAllergyWithSymptom
// // SELECT GROUP_CONCAT(CONCAT(opd_allergy.agent,'=',IF(opd_allergy.symptom IS NULL,',',opd_allergy.symptom)/*,' [',IF(note IS NULL,',',note),']'*/)) AS drugallergy
// // FROM hos.opd_allergy
// // WHERE opd_allergy.hn = ? /*AND (opd_allergy.no_alert<>'Y' or opd_allergy.no_alert IS NULL)*/
// // ORDER BY display_order;
// fn get_hosxp_opd_allergy_with_symptom(hosxp: &str) -> String {
//     [
//         "SELECT GROUP_CONCAT(CONCAT(opd_allergy.agent,'=',IF(opd_allergy.symptom IS NULL,',',opd_allergy.symptom))) AS drugallergy \
//         FROM ",hosxp,".opd_allergy \
//         WHERE opd_allergy.hn = ? \
//         ORDER BY display_order;"
//     ].concat()
// }
// // getDrAdmissionNoteDrugAllergy
// // SELECT allergy_drug_history,allergy_drug_pharmacy_check_person,allergy_drug_pharmacy_check_datetime
// // FROM kphis.ipd_dr_admission_note
// // WHERE an = ?;
// fn get_admission_note_dr_drug_allergy(kphis: &str) -> String {
//     [
//         "SELECT allergy_drug_history,allergy_drug_pharmacy_check_person,allergy_drug_pharmacy_check_datetime \
//         FROM ",kphis,".ipd_dr_admission_note \
//         WHERE an = ?;"
//     ].concat()
// }
// // getOpdErAllergyWithSymptom
// // SELECT GROUP_CONCAT(CONCAT(er_allergy_history_agent,'=',IF(er_allergy_history_symptom IS NULL,',',er_allergy_history_symptom))) AS drugallergy
// // FROM kphis.opd_er_allergy_history
// // WHERE opd_er_allergy_history.opd_er_order_master_id = ?
// // ORDER BY er_allergy_history_id;
// fn get_opd_er_allergy_with_symptom(kphis: &str) -> String {
//     [
//         "SELECT GROUP_CONCAT(CONCAT(er_allergy_history_agent,'=',IF(er_allergy_history_symptom IS NULL,',',er_allergy_history_symptom))) AS drugallergy \
//         FROM ",kphis,".opd_er_allergy_history \
//         WHERE opd_er_allergy_history.opd_er_order_master_id = ? \
//         ORDER BY er_allergy_history_id;"
//     ].concat()
// }
// // getOperationAdmit
// // SELECT CONCAT(IFNULL(operation_list.enter_date,''),', ',IFNULL(operation_list.operation_name,''),', ',doctor.name,', ':hospital_name) AS operation_list,
// // TIMESTAMPDIFF(YEAR,operation_list.enter_date,NOW()) AS or_concat_year,
// // MOD(TIMESTAMPDIFF(MONTH,operation_list.enter_date,NOW()),12) AS or_concat_month,
// // TIMESTAMPDIFF(DAY,DATE_ADD(operation_list.enter_date,INTERVAL (TIMESTAMPDIFF(MONTH,operation_list.enter_date,NOW())) MONTH),NOW()) AS or_concat_day
// // FROM hos.operation_list
// // LEFT JOIN hos.doctor ON doctor.code = operation_list.request_doctor
// // LEFT JOIN hos.ipt ON ipt.an = operation_list.an
// // WHERE operation_list.an= ? AND operation_list.status_id = 3
// // ORDER BY operation_list.enter_date,operation_list.enter_time;
// /// an
// fn get_operation_admit(hospital_name: &str, hosxp: &str) -> String {
//     [
//         "SELECT CONCAT(IFNULL(operation_list.enter_date,''),', ',IFNULL(operation_list.operation_name,''),', ',doctor.name,', ",hospital_name,"') AS operation_list,\
//             TIMESTAMPDIFF(YEAR,operation_list.enter_date,NOW()) AS or_concat_year,\
//             MOD(TIMESTAMPDIFF(MONTH,operation_list.enter_date,NOW()),12) AS or_concat_month,\
//             TIMESTAMPDIFF(DAY,DATE_ADD(operation_list.enter_date,INTERVAL (TIMESTAMPDIFF(MONTH,operation_list.enter_date,NOW())) MONTH),NOW()) AS or_concat_day \
//         FROM ",hosxp,".operation_list \
//             LEFT JOIN ",hosxp,".doctor ON doctor.code = operation_list.request_doctor \
//             LEFT JOIN ",hosxp,".ipt ON ipt.an = operation_list.an \
//         WHERE operation_list.an= ? AND operation_list.status_id = 3 \
//         ORDER BY operation_list.enter_date,operation_list.enter_time;"
//     ].concat()
// }
// // getDocumentAddmissionNurse
// // SELECT COUNT(*) AS count_data FROM kphis.ipd_nurse_admission_note WHERE an = ?;
// fn get_document_addmission_nurse(kphis: &str) -> String {
//     ["SELECT COUNT(*) AS count_data FROM ",kphis,".ipd_nurse_admission_note WHERE an = ?;"].concat()
// }
// // getDocumentAddmissionDoctor
// // SELECT COUNT(*) AS count_data FROM kphis.ipd_dr_admission_note WHERE an = ?;
// fn get_document_addmission_doctor(kphis: &str) -> String {
//     ["SELECT COUNT(*) AS count_data FROM ",kphis,".ipd_dr_admission_note WHERE an = ?;"].concat()
// }
// // getDocumentSummary
// // SELECT COUNT(*) AS count_data FROM kphis.ipd_summary WHERE an = ?;
// fn get_document_summary(kphis: &str) -> String {
//     ["SELECT COUNT(*) AS count_data FROM ",kphis,".ipd_summary WHERE an = ?;"].concat()
// }
// // getDocumentOrder
// // SELECT COUNT(*) AS count_data FROM kphis.ipd_order WHERE an = ?;
// fn get_document_order(kphis: &str) -> String {
//     ["SELECT COUNT(*) AS count_data FROM ",kphis,".ipd_order WHERE an = ?;"].concat()
// }
// // getDocumentOrderProgressNote
// // SELECT COUNT(*) AS count_data FROM kphis.ipd_progress_note WHERE an = ?;
// fn get_document_order_progress_note(kphis: &str) -> String {
//     ["SELECT COUNT(*) AS count_data FROM ",kphis,".ipd_progress_note WHERE an = ?;"].concat()
// }
// // getDocumentFocusList
// // SELECT COUNT(*) AS count_data FROM kphis.ipd_focus_list WHERE an = ?;
// fn get_document_focus_list(kphis: &str) -> String {
//     ["SELECT COUNT(*) AS count_data FROM ",kphis,".ipd_focus_list WHERE an = ?;"].concat()
// }
// // getDocumentFocusNote
// // SELECT COUNT(*) AS count_data FROM kphis.ipd_focus_note WHERE an = ?;
// fn get_document_focus_note(kphis: &str) -> String {
//     ["SELECT COUNT(*) AS count_data FROM ",kphis,".ipd_focus_note WHERE an = ?;"].concat()
// }
// // getDocumentVitalSign
// // SELECT COUNT(*) AS count_data FROM kphis.ipd_vs_vital_sign WHERE an = ?;
// fn get_document_vital_sign(kphis: &str) -> String {
//     ["SELECT COUNT(*) AS count_data FROM ",kphis,".ipd_vs_vital_sign WHERE an = ?;"].concat()
// }
// // getDocumentIo
// // SELECT COUNT(*) AS count_data FROM kphis.ipd_io WHERE an = ?;
// fn get_document_io(kphis: &str) -> String {
//     ["SELECT COUNT(*) AS count_data FROM ",kphis,".ipd_io WHERE an = ?;"].concat()
// }
// // getDocumentConsult
// // SELECT COUNT(*) AS count_data FROM kphis.ipd_dr_consult WHERE an = ?;
// fn get_document_consult(kphis: &str) -> String {
//     ["SELECT COUNT(*) AS count_data FROM ",kphis,".ipd_dr_consult WHERE an = ?;"].concat()
// }
// // getDocumentOperative
// // SELECT COUNT(*) count_data
// // FROM hos.operation_list ol
// // LEFT JOIN hos.an_stat an1 ON an1.an=ol.an
// // LEFT JOIN hos.ovst ov ON ov.an=ol.an
// // LEFT JOIN hos.vn_stat v ON v.vn=ov.vn
// // LEFT JOIN hos.patient pt ON  pt.hn=ol.hn
// // WHERE ov.an = ? AND ol.status_id=3;
// fn get_document_operative(hosxp: &str) -> String {
//     [
//         "SELECT COUNT(*) count_data \
//         FROM ",hosxp,".operation_list ol \
//             LEFT JOIN ",hosxp,".an_stat an1 ON an1.an=ol.an \
//             LEFT JOIN ",hosxp,".ovst ov ON ov.an=ol.an \
//             LEFT JOIN ",hosxp,".vn_stat v ON v.vn=ov.vn \
//             LEFT JOIN ",hosxp,".patient pt ON  pt.hn=ol.hn \
//         WHERE ov.an = ? AND ol.status_id=3;"
//     ].concat()
// }
// // getDocumentIndex
// // SELECT COUNT(*) AS count_data FROM kphis.ipd_nurse_index_plan WHERE an = ?;
// fn get_document_index(kphis: &str) -> String {
//     ["SELECT COUNT(*) AS count_data FROM ",kphis,".ipd_nurse_index_plan WHERE an = ?;"].concat()
// }
// // getDataDocumentAddmissionNurse
// // SELECT create_datetime AS create_datetimeAddmissionNurse, update_datetime AS update_datetimeAddmissionNurse
// // FROM kphis.ipd_nurse_admission_note
// // WHERE an = ?;
// fn get_data_document_addmission_nurse(kphis: &str) -> String {
//     [
//         "SELECT create_datetime AS create_datetimeAddmissionNurse, update_datetime AS update_datetimeAddmissionNurse \
//         FROM ",kphis,".ipd_nurse_admission_note \
//         WHERE an = ?;"
//     ].concat()
// }
// // getDataDocumentAddmissionDoctor
// // SELECT create_datetime AS create_datetimeAddmissionDoctor, update_datetime AS update_datetimeAddmissionDoctor
// // FROM kphis.ipd_dr_admission_note
// // WHERE an = ?;
// fn get_data_document_addmission_doctor(kphis: &str) -> String {
//     [
//         "SELECT create_datetime AS create_datetimeAddmissionDoctor, update_datetime AS update_datetimeAddmissionDoctor \
//         FROM ",kphis,".ipd_dr_admission_note \
//         WHERE an = ?;"
//     ].concat()
// }
// // getDataDocumentSummary
// // SELECT create_datetime AS create_datetimeSummary, update_datetime AS update_datetimeSummary
// // FROM kphis.ipd_summary
// // WHERE an = ?;
// fn get_data_document_summary(kphis: &str) -> String {
//     [
//         "SELECT create_datetime AS create_datetimeSummary, update_datetime AS update_datetimeSummary \
//         FROM ",kphis,".ipd_summary \
//         WHERE an = ?;"
//     ].concat()
// }
// // getDocumentLab
// // SELECT COUNT(*) AS count_data
// // FROM hos.lab_head h
// // INNER JOIN hos.lab_order o ON h.lab_order_number=o.lab_order_number
// // WHERE h.vn=? AND o.confirm = 'Y'
// // -- group by o.confirm = 'Y'
// fn get_document_lab(hosxp: &str) -> String {
//     [
//         "SELECT COUNT(*) AS count_data \
//         FROM ",hosxp,".lab_head h \
//         INNER JOIN ",hosxp,".lab_order o ON h.lab_order_number=o.lab_order_number \
//         WHERE h.vn=? AND o.confirm = 'Y';"
//     ].concat()
// }
// // getDocumentOpdLab
// // SELECT COUNT(*) AS count_data
// // FROM hos.lab_head h
// // INNER JOIN hos.lab_order o ON h.lab_order_number=o.lab_order_number
// // WHERE h.vn=? AND o.confirm = 'Y';
// // -- group by o.confirm = 'Y'
// fn get_document_opd_lab(hosxp: &str) -> String {
//     [
//         "SELECT COUNT(*) AS count_data \
//         FROM ",hosxp,".lab_head h \
//         INNER JOIN ",hosxp,".lab_order o ON h.lab_order_number=o.lab_order_number \
//         WHERE h.vn=? AND o.confirm = 'Y';"
//     ].concat()
// }
// // getDocumentMedReconciliation
// // SELECT COUNT(*) AS count_data FROM kphis.ipd_med_reconciliation WHERE an=?;
// fn get_document_med_reconciliation(kphis: &str) -> String {
//     ["SELECT COUNT(*) AS count_data FROM ",kphis,".ipd_med_reconciliation WHERE an=?;"].concat()
// }
// // getDocumentMedReconciliationHosxp
// // SELECT COUNT(*) AS count_data FROM hos.medication_reconciliation_detail t2
// // JOIN hos.medication_reconciliation t1 ON t1.medication_reconciliation_id = t2.medication_reconciliation_id
// // WHERE t1.an=?
// // ORDER BY medication_reconciliation_detail_id;
// fn get_document_med_reconciliation_hosxp(hosxp: &str) -> String {
//     [
//         "SELECT COUNT(*) AS count_data FROM ",hosxp,".medication_reconciliation_detail t2 \
//         JOIN ",hosxp,".medication_reconciliation t1 ON t1.medication_reconciliation_id = t2.medication_reconciliation_id \
//         WHERE t1.an=? \
//         ORDER BY medication_reconciliation_detail_id;"
//     ].concat()
// }
// // checkPatientAge
// // SELECT age_y FROM hos.an_stat WHERE an = ? LIMIT 1;
// fn check_patient_age(hosxp: &str) -> String {
//     ["SELECT age_y FROM ",hosxp,".an_stat WHERE an = ? LIMIT 1;"].concat()
// }
// // checkPatientAgeByVn
// // SELECT age_y FROM hos.vn_stat WHERE vn = ? LIMIT 1;
// fn check_patient_age_by_vn(hosxp: &str) -> String {
//     ["SELECT age_y FROM ",hosxp,".vn_stat WHERE vn = ? LIMIT 1;"].concat()
// }
// // getVnFromOpdErMasterId
// // SELECT vn FROM kphis.opd_er_order_master WHERE opd_er_order_master_id = ? LIMIT 1;
// fn get_vn_from_opd_er_master_id(kphis: &str) -> String {
//     ["SELECT vn FROM ",kphis,".opd_er_order_master WHERE opd_er_order_master_id = ? LIMIT 1;"].concat()
// }
// // getHnFromOpdErMasterId
// // SELECT ovst.hn
// // FROM kphis.opd_er_order_master
// // LEFT JOIN hos.ovst ON opd_er_order_master.vn = ovst.vn
// // WHERE opd_er_order_master_id = ? LIMIT 1;
// fn get_hn_from_opd_er_master_id(hosxp: &str, kphis: &str) -> String {
//     [
//         "SELECT ovst.hn \
//         FROM ",kphis,".opd_er_order_master \
//         LEFT JOIN ",hosxp,".ovst ON opd_er_order_master.vn = ovst.vn \
//         WHERE opd_er_order_master_id = ? LIMIT 1;"
//     ].concat()
// }
// // getDataHosxpWard
// // SELECT ward FROM hos.ipt WHERE an=? LIMIT 1;
// fn get_data_hosxp_ward(hosxp: &str) -> String {
//     ["SELECT ward FROM ", hosxp, ".ipt WHERE an=? LIMIT 1;"].concat()
// }
// // getVstdateTimeByVn
// // SELECT CONCAT(vstdate,' ',vsttime) AS vstdate_time FROM hos.opdscreen WHERE vn=?;
// fn get_vstdate_time_by_vn(hosxp: &str) -> String {
//     ["SELECT CONCAT(vstdate,' ',vsttime) AS vstdate_time FROM ",hosxp,".opdscreen WHERE vn=?;"].concat()
// }
// // getDocumentAnesthetic
// // SELECT COUNT(*) AS count_data
// // FROM hos.operation_list ol
// // LEFT JOIN hos.operation_anes os ON os.operation_id=ol.operation_id
// // WHERE ol.an=? AND os.operation_id IS NOT NULL;
// fn get_document_anesthetic(hosxp: &str) -> String {
//     [
//         "SELECT COUNT(*) AS count_data \
//         FROM ",hosxp,".operation_list ol \
//         LEFT JOIN ",hosxp,".operation_anes os ON os.operation_id=ol.operation_id \
//         WHERE ol.an=? AND os.operation_id IS NOT NULL;"
//     ].concat()
// }
// // getDocumentXray
// // SELECT COUNT(*) AS count_data
// // FROM hos.xray_report x
// // LEFT JOIN hos.xray_items xi ON xi.xray_items_code=x.xray_items_code
// // WHERE x.an =? AND xi.xray_items_group IN (1);
// fn get_document_xray(hosxp: &str) -> String {
//     [
//         "SELECT COUNT(*) AS count_data \
//         FROM ",hosxp,".xray_report x \
//         LEFT JOIN ",hosxp,".xray_items xi ON xi.xray_items_code=x.xray_items_code \
//         WHERE x.an =? AND xi.xray_items_group IN (1);"
//     ].concat()
// }
// // getDocumentCtScan
// // SELECT COUNT(*) AS count_data
// // FROM hos.xray_report x
// // LEFT JOIN hos.xray_items xi ON xi.xray_items_code=x.xray_items_code
// // WHERE x.an =? AND xi.xray_items_group IN (3);
// fn get_document_ct_scan(hosxp: &str) -> String {
//     [
//         "SELECT COUNT(*) AS count_data \
//         FROM ",hosxp,".xray_report x \
//         LEFT JOIN ",hosxp,".xray_items xi ON xi.xray_items_code=x.xray_items_code \
//         WHERE x.an =? AND xi.xray_items_group IN (3);"
//     ].concat()
// }
// // getDocumentMri
// // SELECT COUNT(*) AS count_data
// // FROM hos.xray_report x
// // LEFT JOIN hos.xray_items xi ON xi.xray_items_code=x.xray_items_code
// // WHERE x.an =? AND xi.xray_items_group IN (4);
// fn get_document_mri(hosxp: &str) -> String {
//     [
//         "SELECT COUNT(*) AS count_data \
//         FROM ",hosxp,".xray_report x \
//         LEFT JOIN ",hosxp,".xray_items xi ON xi.xray_items_code=x.xray_items_code \
//         WHERE x.an =? AND xi.xray_items_group IN (4);"
//     ].concat()
// }
// // getCovidLatestLabResult
// // SELECT sent_date, `status`, results, results_date, approve_date, approve_results
// // FROM kph_covid.lab_result_covid
// // WHERE false
// // 	# IF($cid != NULL && trim($cid) != ''){
// // OR cid=:cid
// // 	# IF($passport_no != NULL && trim($passport_no) != ''){
// // OR passport=:passport_no
// // ORDER BY sent_date DESC LIMIT 1
// fn get_covid_latest_lab_result(cid: &str, passport_no: &str) -> String {
//     let or = if !(cid.is_empty()) && !(passport_no.trim().is_empty()) {
//         ["OR cid='", cid, "' "].concat()
//     } else if !(passport_no.trim().is_empty()) {
//         ["OR passport='", passport_no, "' "].concat()
//     } else {
//         String::new()
//     };
//     [
//         "SELECT sent_date, `status`, results, results_date, approve_date, approve_results \
//         FROM kph_covid.lab_result_covid \
//         WHERE false ",&or,"ORDER BY sent_date DESC LIMIT 1;"
//     ].concat()
// }
// // getCovidLabResults
// // SELECT *
// // FROM kph_covid.lab_result_covid
// // WHERE false
// // 	# IF($cid != NULL && trim($cid) != ''){
// // OR cid=:cid
// // 	# IF($passport_no != NULL && trim($passport_no) != ''){
// // OR passport=:passport_no
// // ORDER BY sent_date DESC;
// fn get_covid_lab_results(cid: &str, passport_no: &str) -> String {
//     let or = if !(cid.trim().is_empty()) {
//         ["OR cid='", cid, "' "].concat()
//     } else if !(passport_no.trim().is_empty()) {
//         ["OR passport='", passport_no, "' "].concat()
//     } else {
//         String::new()
//     };
//     [
//         "SELECT * \
//         FROM kph_covid.lab_result_covid \
//         WHERE false ",&or,"ORDER BY sent_date DESC;"
//     ].concat()
// }
// // getDocumentErFromOpdErMasterId
// // SELECT COUNT(*) AS count_data
// // FROM kphis.opd_er_order_master
// // WHERE vn = ? AND (delete_flag IS NULL or delete_flag <> 'Y')
// // ORDER BY opd_er_order_master_id;
// fn get_document_er_from_opd_er_master_id(kphis: &str) -> String {
//     [
//         "SELECT COUNT(*) AS count_data \
//         FROM ",kphis,".opd_er_order_master \
//         WHERE vn = ? AND (delete_flag IS NULL or delete_flag <> 'Y') \
//         ORDER BY opd_er_order_master_id;"
//     ].concat()
// }
// // getDocumentErDoctorTrauma
// // SELECT COUNT(*) AS count_data
// // FROM kphis.opd_er_dr_pe
// // WHERE opd_er_order_master_id = ?;
// fn get_document_er_doctor_trauma(kphis: &str) -> String {
//     [
//         "SELECT COUNT(*) AS count_data \
//         FROM ",kphis,".opd_er_dr_pe \
//         WHERE opd_er_order_master_id = ?;"
//     ].concat()
// }
// // getDocumentErNurseScreening
// // SELECT COUNT(*) AS count_data
// // FROM kphis.opd_er_nurse_screening
// // WHERE opd_er_order_master_id = ?;
// fn get_document_er_nurse_screening(kphis: &str) -> String {
//     [
//         "SELECT COUNT(*) AS count_data \
//         FROM ",kphis,".opd_er_nurse_screening \
//         WHERE opd_er_order_master_id = ?;"
//     ].concat()
// }
// // getDocumentErConsult
// // SELECT COUNT(*) AS count_data
// // FROM kphis.opd_er_consult
// // WHERE opd_er_order_master_id = ?;
// fn get_document_er_consult(kphis: &str) -> String {
//     [
//         "SELECT COUNT(*) AS count_data \
//         FROM ",kphis,".opd_er_consult \
//         WHERE opd_er_order_master_id = ?;"
//     ].concat()
// }
// // getDocumentErSetFt
// // SELECT COUNT(*) AS count_data
// // FROM kphis.opd_er_set_fast_track
// // WHERE opd_er_order_master_id = ?;
// fn get_document_er_set_ft(kphis: &str) -> String {
//     [
//         "SELECT COUNT(*) AS count_data \
//         FROM ",kphis,".opd_er_set_fast_track \
//         WHERE opd_er_order_master_id = ?;"
//     ].concat()
// }
// // getDocumentErOrder
// // SELECT COUNT(*) AS count_data
// // FROM kphis.opd_er_order
// // WHERE opd_er_order_master_id = ?;
// fn get_document_er_order(kphis: &str) -> String {
//     [
//         "SELECT COUNT(*) AS count_data \
//         FROM ",kphis,".opd_er_order \
//         WHERE opd_er_order_master_id = ?;"
//     ].concat()
// }
// // getDocumentErNurseIndexPlan
// // SELECT COUNT(*) AS count_data
// // FROM kphis.opd_er_nurse_index_plan
// // WHERE opd_er_order_master_id = ?;
// fn get_document_er_nurse_index_plan(kphis: &str) -> String {
//     [
//         "SELECT COUNT(*) AS count_data \
//         FROM ",kphis,".opd_er_nurse_index_plan \
//         WHERE opd_er_order_master_id = ?;"
//     ].concat()
// }
// // getDocumentErVitalSign
// // SELECT COUNT(*) AS count_data
// // FROM kphis.opd_er_vs_vital_sign
// // WHERE opd_er_order_master_id = ?;
// fn get_document_er_vital_sign(kphis: &str) -> String {
//     [
//         "SELECT COUNT(*) AS count_data \
//         FROM ",kphis,".opd_er_vs_vital_sign \
//         WHERE opd_er_order_master_id = ?;"
//     ].concat()
// }
// // getDocumentErIo
// // SELECT COUNT(*) AS count_data
// // FROM kphis.opd_er_io
// // WHERE opd_er_order_master_id = ?;
// fn get_document_er_io(kphis: &str) -> String {
//     [
//         "SELECT COUNT(*) AS count_data \
//         FROM ",kphis,".opd_er_io \
//         WHERE opd_er_order_master_id = ?;"
//     ].concat()
// }
// // getDocumentErFocusList
// // SELECT COUNT(*) AS count_data
// // FROM kphis.opd_er_focus_list
// // WHERE opd_er_order_master_id = ?;
// fn get_document_er_focus_list(kphis: &str) -> String {
//     [
//         "SELECT COUNT(*) AS count_data \
//         FROM ",kphis,".opd_er_focus_list \
//         WHERE opd_er_order_master_id = ?;"
//     ].concat()
// }
// // getDocumentErFocusNote
// // SELECT COUNT(*) AS count_data
// // FROM kphis.opd_er_focus_note
// // WHERE opd_er_order_master_id = ?;
// fn get_document_er_focus_note(kphis: &str) -> String {
//     [
//         "SELECT COUNT(*) AS count_data \
//         FROM ",kphis,".opd_er_focus_note \
//         WHERE opd_er_order_master_id = ?;"
//     ].concat()
// }
// // getIpdCheckRowSmp
// // SELECT COUNT(*) AS count_data FROM kphis.ipd_focus_list WHERE smp_id = ?;
// fn get_ipd_check_row_smp(kphis: &str) -> String {
//     ["SELECT COUNT(*) AS count_data FROM ",kphis,".ipd_focus_list WHERE smp_id = ?;"].concat()
// }
// // getOpdErCheckRowSmp
// // SELECT COUNT(*) AS count_data FROM kphis.opd_er_focus_list WHERE smp_id = ?;
// fn get_opd_er_check_row_smp(kphis: &str) -> String {
//     ["SELECT COUNT(*) AS count_data FROM ",kphis,".opd_er_focus_list WHERE smp_id = ?;"].concat()
// }
// // getIpdCheckRowFocus
// // SELECT COUNT(*) AS count_data FROM kphis.ipd_focus_list WHERE focus_id = ?;
// fn get_ipd_check_row_focus(kphis: &str) -> String {
//     ["SELECT COUNT(*) AS count_data FROM ",kphis,".ipd_focus_list WHERE focus_id = ?;"].concat()
// }
// // getOpdErCheckRowFocus
// // SELECT COUNT(*) AS count_data FROM kphis.opd_er_focus_list WHERE focus_id = ?;
// fn get_opd_er_check_row_focus(kphis: &str) -> String {
//     ["SELECT COUNT(*) AS count_data FROM ",kphis,".opd_er_focus_list WHERE focus_id = ?;"].concat()
// }
// // getIpdCheckRowGoal
// // SELECT COUNT(*) AS count_data FROM kphis.ipd_focus_list_goal_item WHERE goal_id = ?;
// fn get_ipd_check_row_goal(kphis: &str) -> String {
//     ["SELECT COUNT(*) AS count_data FROM ",kphis,".ipd_focus_list_goal_item WHERE goal_id = ?;"].concat()
// }
// // getOpdErCheckRowGoal
// // SELECT COUNT(*) AS count_data FROM kphis.opd_er_focus_list_goal_item WHERE goal_id = ?;
// fn get_opd_er_check_row_goal(kphis: &str) -> String {
//     ["SELECT COUNT(*) AS count_data FROM ",kphis,".opd_er_focus_list_goal_item WHERE goal_id = ?;"].concat()
// }
// // getIpdCheckRowIntvt
// // SELECT COUNT(*) AS count_data FROM kphis.ipd_focus_note_intvt_item WHERE intvt_id = ?;
// fn get_ipd_check_row_intvt(kphis: &str) -> String {
//     ["SELECT COUNT(*) AS count_data FROM ",kphis,".ipd_focus_note_intvt_item WHERE intvt_id = ?;"].concat()
// }
// // getOpdErCheckRowIntvt
// // SELECT COUNT(*) AS count_data FROM kphis.opd_er_focus_note_intvt_item WHERE intvt_id = ?;
// fn get_opd_er_check_row_intvt(kphis: &str) -> String {
//     ["SELECT COUNT(*) AS count_data FROM ",kphis,".opd_er_focus_note_intvt_item WHERE intvt_id = ?;"].concat()
// }
// // getDocumentErAllergyHistory
// // SELECT COUNT(*) AS count_data
// // FROM kphis.opd_er_allergy_history
// // WHERE opd_er_order_master_id = ?;
// fn get_document_er_allergy_history(kphis: &str) -> String {
//     [
//         "SELECT COUNT(*) AS count_data \
//         FROM ",kphis,".opd_er_allergy_history \
//         WHERE opd_er_order_master_id = ?;"
//     ].concat()
// }