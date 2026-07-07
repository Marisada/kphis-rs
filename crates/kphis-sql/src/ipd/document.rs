use crate::{TABLE_CREATE_COLUMNS, TABLE_CREATE_PREPARED};

// // ipd-document-main.php
// KphisQueryUtils::getDocumentSummary2($an)
// KphisQueryUtils::getDocumentAddmissionDoctor($an)
// KphisQueryUtils::getDocumentAddmissionNurse($an)
// KphisQueryUtils::getDocumentOrderProgressNote($an)
// KphisQueryUtils::getDocumentConsult($an)
// KphisQueryUtils::getDocumentFocusList($an)
// KphisQueryUtils::getDocumentFocusNote($an)
// KphisQueryUtils::getDocumentVitalSign($an)
// KphisQueryUtils::getDocumentIO($an)
// KphisQueryUtils::getDocumentIndex($an)
// KphisQueryUtils::getDocumentLab($an)
// KphisQueryUtils::getDocumentXray($an)
// KphisQueryUtils::getDocumentCTscan($an)
// KphisQueryUtils::getDocumentMRI($an)
// KphisQueryUtils::getDocumentMedReconciliation($an)
// KphisQueryUtils::getDocumentMedReconciliationHOSXP($an)
// KphisQueryUtils::getDocumentERFromOpdErMasterId($vn)
// KphisQueryUtils::getDocumentOperative($an)
// SELECT
//     (SELECT COUNT(summary_id) FROM kphis.ipd_summary_2 WHERE an=?) AS count_data_summary2,
//     (SELECT COUNT(admission_note_id) FROM kphis.ipd_dr_admission_note WHERE an=?) AS count_data_dr_admission_note,
//     (SELECT COUNT(nurse_admission_note_id) FROM kphis.ipd_nurse_admission_note WHERE an=?) AS count_data_nurse_admission_note,
//     (SELECT COUNT(order_id) FROM kphis.ipd_order WHERE an=?) AS count_data_order,\
//     (SELECT COUNT(progress_note_id) FROM kphis.ipd_progress_note WHERE an=?) AS count_data_progress_note,
//     (SELECT COUNT(consult_id) FROM kphis.ipd_dr_consult WHERE an=?) AS count_data_dr_consult,
//     (SELECT COUNT(fclist_id) FROM kphis.ipd_focus_list WHERE an=?) AS count_data_focus_list,
//     (SELECT COUNT(fcnote_id) FROM kphis.ipd_focus_note WHERE an=?) AS count_data_focus_note,
//     (SELECT COUNT(vs_id) FROM kphis.ipd_vs_vital_sign WHERE an=?) AS count_data_vital_sign,
//     (SELECT COUNT(io_id) FROM kphis.ipd_io WHERE an=?) AS count_data_io,
//     (SELECT COUNT(plan_id) FROM kphis.ipd_nurse_index_plan WHERE an=?) AS count_data_index_plan,
//     (SELECT COUNT(h.lab_order_number) FROM hos.lab_head h INNER JOIN hos.lab_order o ON h.lab_order_number=o.lab_order_number WHERE h.vn=? AND o.confirm='Y') AS count_data_lab,
//     (SELECT COUNT(xi.xray_items_code) FROM hos.xray_report x LEFT JOIN hos.xray_items xi ON xi.xray_items_code=x.xray_items_code WHERE x.an=? AND xi.xray_items_group=1) AS count_data_xray,
//     (SELECT COUNT(xi.xray_items_code) FROM hos.xray_report x LEFT JOIN hos.xray_items xi ON xi.xray_items_code=x.xray_items_code WHERE x.an=? AND xi.xray_items_group=3) AS count_data_ct,
//     (SELECT COUNT(xi.xray_items_code) FROM hos.xray_report x LEFT JOIN hos.xray_items xi ON xi.xray_items_code=x.xray_items_code WHERE x.an=? AND xi.xray_items_group=4) AS count_data_mri,
//     (SELECT COUNT(med_reconciliation_id) FROM kphis.ipd_med_reconciliation WHERE an=?) AS count_data_med_reconciliation,
//     (SELECT COUNT(medication_reconciliation_detail_id) FROM hos.medication_reconciliation_detail t2 JOIN hos.medication_reconciliation t1 ON t1.medication_reconciliation_id=t2.medication_reconciliation_id WHERE t1.an=?) AS count_data_med_reconciliation_hosxp,
//     (SELECT COUNT(operation_id) FROM hos.operation_list WHERE an=?) AS count_data_operation,
//     (SELECT EXISTS(SELECT * FROM hos.referout WHERE vn=?)) AS has_data_referout,
//     (SELECT EXISTS(SELECT * FROM kphis_extra.image_usage iu JOIN kphis_extra.ipd_document doc ON doc.document_id=iu.usage_key_id WHERE iu.usage_id=11 AND doc.document_type_id=1 AND an=?)) AS has_scan_consent,
//     (SELECT EXISTS(SELECT * FROM kphis_extra.image_usage iu JOIN kphis_extra.ipd_document doc ON doc.document_id=iu.usage_key_id WHERE iu.usage_id=11 AND doc.document_type_id=2 AND an=?)) AS has_scan_insure,
//     (SELECT EXISTS(SELECT * FROM kphis_extra.image_usage iu JOIN kphis_extra.ipd_document doc ON doc.document_id=iu.usage_key_id WHERE iu.usage_id=11 AND doc.document_type_id=3 AND an=?)) AS has_scan_refer_in,
//     (SELECT EXISTS(SELECT * FROM kphis_extra.image_usage iu JOIN kphis_extra.ipd_document doc ON doc.document_id=iu.usage_key_id WHERE iu.usage_id=11 AND doc.document_type_id=4 AND an=?)) AS has_scan_refer_out,
//     (SELECT EXISTS(SELECT * FROM kphis_extra.image_usage iu JOIN kphis_extra.ipd_document doc ON doc.document_id=iu.usage_key_id WHERE iu.usage_id=11 AND doc.document_type_id=5 AND an=?)) AS has_scan_culture,
//     (SELECT EXISTS(SELECT * FROM kphis_extra.image_usage iu JOIN kphis_extra.ipd_document doc ON doc.document_id=iu.usage_key_id WHERE iu.usage_id=11 AND doc.document_type_id=6 AND an=?)) AS has_scan_blood,
//     (SELECT EXISTS(SELECT * FROM kphis_extra.image_usage iu JOIN kphis_extra.ipd_document doc ON doc.document_id=iu.usage_key_id WHERE iu.usage_id=11 AND doc.document_type_id=7 AND an=?)) AS has_scan_special,
//     (SELECT EXISTS(SELECT * FROM kphis_extra.image_usage iu JOIN kphis_extra.ipd_document doc ON doc.document_id=iu.usage_key_id WHERE iu.usage_id=11 AND doc.document_type_id=8 AND an=?)) AS has_scan_ekg,
//     (SELECT EXISTS(SELECT * FROM kphis_extra.image_usage iu JOIN kphis_extra.ipd_document doc ON doc.document_id=iu.usage_key_id WHERE iu.usage_id=11 AND doc.document_type_id=9 AND an=?)) AS has_scan_xray,
//     (SELECT EXISTS(SELECT * FROM kphis_extra.image_usage iu JOIN kphis_extra.ipd_document doc ON doc.document_id=iu.usage_key_id WHERE iu.usage_id=11 AND doc.document_type_id=10 AND an=?)) AS has_scan_ct,
//     (SELECT EXISTS(SELECT * FROM kphis_extra.image_usage iu JOIN kphis_extra.ipd_document doc ON doc.document_id=iu.usage_key_id WHERE iu.usage_id=11 AND doc.document_type_id=11 AND an=?)) AS has_scan_mri,
//     (SELECT EXISTS(SELECT * FROM kphis_extra.image_usage iu JOIN kphis_extra.ipd_document doc ON doc.document_id=iu.usage_key_id WHERE iu.usage_id=11 AND doc.document_type_id=12 AND an=?)) AS has_scan_oper,
//     (SELECT EXISTS(SELECT * FROM kphis_extra.image_usage iu JOIN kphis_extra.ipd_document doc ON doc.document_id=iu.usage_key_id WHERE iu.usage_id=11 AND doc.document_type_id=13 AND an=?)) AS has_scan_anes,
//     (SELECT EXISTS(SELECT * FROM kphis_extra.image_usage iu JOIN kphis_extra.ipd_document doc ON doc.document_id=iu.usage_key_id WHERE iu.usage_id=11 AND doc.document_type_id=14 AND an=?)) AS has_scan_labour,
//     (SELECT EXISTS(SELECT * FROM kphis_extra.image_usage iu JOIN kphis_extra.ipd_document doc ON doc.document_id=iu.usage_key_id WHERE iu.usage_id=11 AND doc.document_type_id=15 AND an=?)) AS has_scan_physio,
//     (SELECT EXISTS(SELECT * FROM kphis_extra.image_usage iu JOIN kphis_extra.ipd_document doc ON doc.document_id=iu.usage_key_id WHERE iu.usage_id=11 AND doc.document_type_id=16 AND an=?)) AS has_scan_alt_med,
//     (SELECT EXISTS(SELECT * FROM kphis_extra.image_usage iu JOIN kphis_extra.ipd_document doc ON doc.document_id=iu.usage_key_id WHERE iu.usage_id=11 AND doc.document_type_id=17 AND an=?)) AS has_scan_nutrition,
//     (SELECT EXISTS(SELECT * FROM kphis_extra.image_usage iu JOIN kphis_extra.ipd_document doc ON doc.document_id=iu.usage_key_id WHERE iu.usage_id=11 AND doc.document_type_id=18 AND an=?)) AS has_scan_others,
//     (SELECT EXISTS(SELECT * FROM kphis_extra.image_usage iu JOIN kphis_extra.ipd_document doc ON doc.document_id=iu.usage_key_id WHERE iu.usage_id=11 AND doc.document_type_id=19 AND an=?)) AS has_scan_other_sp_clinic,
//     (SELECT EXISTS(SELECT * FROM kphis_extra.image_usage iu JOIN kphis_extra.ipd_document doc ON doc.document_id=iu.usage_key_id WHERE iu.usage_id=11 AND doc.document_type_id=20 AND an=?)) AS has_scan_opd_card,
//     (SELECT EXISTS(SELECT * FROM kphis_extra.image_usage iu JOIN kphis_extra.ipd_document doc ON doc.document_id=iu.usage_key_id WHERE iu.usage_id=11 AND doc.document_type_id=21 AND an=?)) AS has_scan_finance,
//     (SELECT COUNT(opd_er_order_master_id) FROM kphis.opd_er_order_master WHERE vn=? AND (delete_flag IS NULL OR delete_flag <> 'Y')) AS count_data_er_master_id;
/// an x40, vn
pub fn get_ipd_document_exists(hosxp: &str, kphis: &str, kphis_extra: &str) -> String {
    [
        "SELECT \
            (SELECT EXISTS(SELECT * FROM ",kphis,".ipd_summary_2 WHERE an=?)) AS has_data_summary2,\
            (SELECT EXISTS(SELECT * FROM ",kphis,".ipd_dr_admission_note WHERE an=?)) AS has_data_dr_admission_note,\
            (SELECT EXISTS(SELECT * FROM ",kphis,".ipd_nurse_admission_note WHERE an=?)) AS has_data_nurse_admission_note,\
            (SELECT EXISTS(SELECT * FROM ",kphis,".ipd_order WHERE an=?)) AS has_data_order,\
            (SELECT EXISTS(SELECT * FROM ",kphis,".ipd_progress_note WHERE an=?)) AS has_data_progress_note,\
            (SELECT EXISTS(SELECT * FROM ",kphis,".ipd_dr_consult WHERE an=?)) AS has_data_dr_consult,\
            (SELECT EXISTS(SELECT * FROM ",kphis,".ipd_focus_list WHERE an=?)) AS has_data_focus_list,\
            (SELECT EXISTS(SELECT * FROM ",kphis,".ipd_focus_note WHERE an=?)) AS has_data_focus_note,\
            (SELECT EXISTS(SELECT * FROM ",kphis,".ipd_vs_vital_sign WHERE an=?)) AS has_data_vital_sign,\
            (SELECT EXISTS(SELECT * FROM ",kphis,".ipd_io WHERE an=?)) AS has_data_io,\
            (SELECT EXISTS(SELECT * FROM ",kphis,".ipd_nurse_index_plan WHERE an=?)) AS has_data_index_plan,\
            (SELECT EXISTS(SELECT * FROM ",hosxp,".lab_head h INNER JOIN ",hosxp,".lab_order o ON h.lab_order_number=o.lab_order_number WHERE h.vn=? AND o.confirm='Y')) AS has_data_lab,\
            (SELECT EXISTS(SELECT * FROM ",hosxp,".xray_report x LEFT JOIN ",hosxp,".xray_items xi ON xi.xray_items_code=x.xray_items_code WHERE x.an=? AND xi.xray_items_group=1)) AS has_data_xray,\
            (SELECT EXISTS(SELECT * FROM ",hosxp,".xray_report x LEFT JOIN ",hosxp,".xray_items xi ON xi.xray_items_code=x.xray_items_code WHERE x.an=? AND xi.xray_items_group=3)) AS has_data_ct,\
            (SELECT EXISTS(SELECT * FROM ",hosxp,".xray_report x LEFT JOIN ",hosxp,".xray_items xi ON xi.xray_items_code=x.xray_items_code WHERE x.an=? AND xi.xray_items_group=4)) AS has_data_mri,\
            (SELECT EXISTS(SELECT * FROM ",kphis,".ipd_med_reconciliation WHERE an=?)) AS has_data_med_reconciliation,\
            (SELECT EXISTS(SELECT * FROM ",hosxp,".medication_reconciliation_detail t2 JOIN ",hosxp,".medication_reconciliation t1 ON t1.medication_reconciliation_id=t2.medication_reconciliation_id WHERE t1.an=?)) AS has_data_med_reconciliation_hosxp,\
            (SELECT EXISTS(SELECT * FROM ",hosxp,".operation_list WHERE an=?)) AS has_data_operation,\
            (SELECT EXISTS(SELECT * FROM ",hosxp,".referout WHERE vn=?)) AS has_data_refer_out,\
            (SELECT EXISTS(SELECT * FROM ",kphis_extra,".image_usage iu JOIN ",kphis_extra,".ipd_document doc ON doc.document_id=iu.usage_key_id WHERE iu.usage_id=11 AND doc.document_type_id=1 AND an=?)) AS has_scan_consent,\
            (SELECT EXISTS(SELECT * FROM ",kphis_extra,".image_usage iu JOIN ",kphis_extra,".ipd_document doc ON doc.document_id=iu.usage_key_id WHERE iu.usage_id=11 AND doc.document_type_id=2 AND an=?)) AS has_scan_insure,\
            (SELECT EXISTS(SELECT * FROM ",kphis_extra,".image_usage iu JOIN ",kphis_extra,".ipd_document doc ON doc.document_id=iu.usage_key_id WHERE iu.usage_id=11 AND doc.document_type_id=3 AND an=?)) AS has_scan_refer_in,\
            (SELECT EXISTS(SELECT * FROM ",kphis_extra,".image_usage iu JOIN ",kphis_extra,".ipd_document doc ON doc.document_id=iu.usage_key_id WHERE iu.usage_id=11 AND doc.document_type_id=4 AND an=?)) AS has_scan_refer_out,\
            (SELECT EXISTS(SELECT * FROM ",kphis_extra,".image_usage iu JOIN ",kphis_extra,".ipd_document doc ON doc.document_id=iu.usage_key_id WHERE iu.usage_id=11 AND doc.document_type_id=5 AND an=?)) AS has_scan_culture,\
            (SELECT EXISTS(SELECT * FROM ",kphis_extra,".image_usage iu JOIN ",kphis_extra,".ipd_document doc ON doc.document_id=iu.usage_key_id WHERE iu.usage_id=11 AND doc.document_type_id=6 AND an=?)) AS has_scan_blood,\
            (SELECT EXISTS(SELECT * FROM ",kphis_extra,".image_usage iu JOIN ",kphis_extra,".ipd_document doc ON doc.document_id=iu.usage_key_id WHERE iu.usage_id=11 AND doc.document_type_id=7 AND an=?)) AS has_scan_special,\
            (SELECT EXISTS(SELECT * FROM ",kphis_extra,".image_usage iu JOIN ",kphis_extra,".ipd_document doc ON doc.document_id=iu.usage_key_id WHERE iu.usage_id=11 AND doc.document_type_id=8 AND an=?)) AS has_scan_ekg,\
            (SELECT EXISTS(SELECT * FROM ",kphis_extra,".image_usage iu JOIN ",kphis_extra,".ipd_document doc ON doc.document_id=iu.usage_key_id WHERE iu.usage_id=11 AND doc.document_type_id=9 AND an=?)) AS has_scan_xray,\
            (SELECT EXISTS(SELECT * FROM ",kphis_extra,".image_usage iu JOIN ",kphis_extra,".ipd_document doc ON doc.document_id=iu.usage_key_id WHERE iu.usage_id=11 AND doc.document_type_id=10 AND an=?)) AS has_scan_ct,\
            (SELECT EXISTS(SELECT * FROM ",kphis_extra,".image_usage iu JOIN ",kphis_extra,".ipd_document doc ON doc.document_id=iu.usage_key_id WHERE iu.usage_id=11 AND doc.document_type_id=11 AND an=?)) AS has_scan_mri,\
            (SELECT EXISTS(SELECT * FROM ",kphis_extra,".image_usage iu JOIN ",kphis_extra,".ipd_document doc ON doc.document_id=iu.usage_key_id WHERE iu.usage_id=11 AND doc.document_type_id=12 AND an=?)) AS has_scan_oper,\
            (SELECT EXISTS(SELECT * FROM ",kphis_extra,".image_usage iu JOIN ",kphis_extra,".ipd_document doc ON doc.document_id=iu.usage_key_id WHERE iu.usage_id=11 AND doc.document_type_id=13 AND an=?)) AS has_scan_anes,\
            (SELECT EXISTS(SELECT * FROM ",kphis_extra,".image_usage iu JOIN ",kphis_extra,".ipd_document doc ON doc.document_id=iu.usage_key_id WHERE iu.usage_id=11 AND doc.document_type_id=14 AND an=?)) AS has_scan_labour,\
            (SELECT EXISTS(SELECT * FROM ",kphis_extra,".image_usage iu JOIN ",kphis_extra,".ipd_document doc ON doc.document_id=iu.usage_key_id WHERE iu.usage_id=11 AND doc.document_type_id=15 AND an=?)) AS has_scan_physio,\
            (SELECT EXISTS(SELECT * FROM ",kphis_extra,".image_usage iu JOIN ",kphis_extra,".ipd_document doc ON doc.document_id=iu.usage_key_id WHERE iu.usage_id=11 AND doc.document_type_id=16 AND an=?)) AS has_scan_alt_med,\
            (SELECT EXISTS(SELECT * FROM ",kphis_extra,".image_usage iu JOIN ",kphis_extra,".ipd_document doc ON doc.document_id=iu.usage_key_id WHERE iu.usage_id=11 AND doc.document_type_id=17 AND an=?)) AS has_scan_nutrition,\
            (SELECT EXISTS(SELECT * FROM ",kphis_extra,".image_usage iu JOIN ",kphis_extra,".ipd_document doc ON doc.document_id=iu.usage_key_id WHERE iu.usage_id=11 AND doc.document_type_id=18 AND an=?)) AS has_scan_others,\
            (SELECT EXISTS(SELECT * FROM ",kphis_extra,".image_usage iu JOIN ",kphis_extra,".ipd_document doc ON doc.document_id=iu.usage_key_id WHERE iu.usage_id=11 AND doc.document_type_id=19 AND an=?)) AS has_scan_other_sp_clinic,\
            (SELECT EXISTS(SELECT * FROM ",kphis_extra,".image_usage iu JOIN ",kphis_extra,".ipd_document doc ON doc.document_id=iu.usage_key_id WHERE iu.usage_id=11 AND doc.document_type_id=20 AND an=?)) AS has_scan_opd_card,\
            (SELECT EXISTS(SELECT * FROM ",kphis_extra,".image_usage iu JOIN ",kphis_extra,".ipd_document doc ON doc.document_id=iu.usage_key_id WHERE iu.usage_id=11 AND doc.document_type_id=21 AND an=?)) AS has_scan_finance,\
            (SELECT opd_er_order_master_id FROM ",kphis,".opd_er_order_master WHERE vn=? AND (delete_flag IS NULL OR delete_flag <> 'Y')) AS opd_er_order_master_id;"
    ].concat()
}

// // ipd-nurse-document_Table_DocumentEdit.php
// KphisQueryUtils::getDataDocumentAddmissionNurse($an)
// KphisQueryUtils::getDataDocumentAddmissionDoctor($an)
// KphisQueryUtils::getDataDocumentSummary($an)
// KphisQueryUtils::getDataDocumentSummary2($an)
// SELECT create_datetime AS create_datetimeAddmissionNurse, update_datetime AS update_datetimeAddmissionNurse FROM kphis.ipd_nurse_admission_note WHERE an = :an
// SELECT create_datetime AS create_datetimeAddmissionDoctor, update_datetime AS update_datetimeAddmissionDoctor FROM kphis.ipd_dr_admission_note WHERE an = :an
// SELECT create_datetime AS create_datetimeSummary, update_datetime AS update_datetimeSummary FROM kphis.ipd_summary_2 WHERE an = :an
// // into one
// SELECT
// (SELECT CONCAT(create_datetime,'|',update_datetime) FROM kphis.ipd_nurse_admission_note WHERE an=?) AS nurse_admission_note,
// (SELECT CONCAT(create_datetime,'|',update_datetime) FROM kphis.ipd_dr_admission_note WHERE an=?) AS dr_admission_note,
// (SELECT CONCAT(create_datetime,'|',update_datetime) FROM kphis.ipd_summary_2 WHERE an=?) AS summary2;
/// an x3
pub fn get_ipd_document_datetime(kphis: &str) -> String {
    [
        "SELECT \
            (SELECT CONCAT(create_datetime,'|',update_datetime) FROM ",kphis,".ipd_nurse_admission_note WHERE an=?) AS nurse_admission_note,\
            (SELECT CONCAT(create_datetime,'|',update_datetime) FROM ",kphis,".ipd_dr_admission_note WHERE an=?) AS dr_admission_note,\
            (SELECT CONCAT(create_datetime,'|',update_datetime) FROM ",kphis,".ipd_summary_2 WHERE an=?) AS summary2;"
    ].concat()
}

// SELECT doc.document_id,doc.document_type_id, 
//     (SELECT EXISTS(SELECT * FROM kphis_extra.image_usage iu WHERE iu.usage_id=11 AND doc.document_id=iu.usage_key_id)) AS has_image
// FROM kphis_extra.ipd_document doc WHERE doc.an=?;
/// an
pub fn get_ipd_document_types(kphis_extra: &str) -> String {
    [
        "SELECT doc.document_id,doc.document_type_id,\
            (SELECT EXISTS(SELECT * FROM ",kphis_extra,".image_usage iu WHERE iu.usage_id=11 AND doc.document_id=iu.usage_key_id)) AS has_image \
        FROM ",kphis_extra,".ipd_document doc WHERE doc.an=?;"
    ].concat()
}

// INSERT IGNORE INTO kphis_extra.ipd_document (an,document_type_id,create_user,create_datetime,update_user,update_datetime,version) VALUES (?,?,?,NOW(),?,NOW(),1);
/// an, document_type_id, loginname, loginname
pub fn insert_ignore_ipd_document_type(kphis_extra: &str) -> String {
    [
        "INSERT IGNORE INTO ",kphis_extra,".ipd_document (an,document_type_id",TABLE_CREATE_COLUMNS,") VALUES (?,?",TABLE_CREATE_PREPARED,");"
    ].concat()
}

// DELETE FROM kphis_extra.ipd_document WHERE an=? AND document_type_id=? AND NOT (SELECT EXISTS(SELECT * FROM kphis_extra.image_usage iu WHERE iu.usage_id=11 AND document_id=iu.usage_key_id));
/// an, document_type_id
pub fn delete_ipd_document_type(kphis_extra: &str) -> String {
    [
        "DELETE FROM ",kphis_extra,".ipd_document WHERE an=? AND document_type_id=? AND NOT (SELECT EXISTS(SELECT * FROM ",kphis_extra,".image_usage iu WHERE iu.usage_id=11 AND document_id=iu.usage_key_id));"
    ].concat()
}