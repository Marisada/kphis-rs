use crate::{TABLE_CREATE_COLUMNS, TABLE_CREATE_PREPARED};

// // opd-document-main-data.php
// SELECT
// (SELECT EXISTS(SELECT * FROM kphis.opd_er_order_master WHERE opd_er_order_master_id=?)) AS has_data_er_master_id,
// (SELECT EXISTS(SELECT * FROM kphis.opd_er_nurse_index_plan WHERE opd_er_order_master_id=?)) AS has_data_index_plan,
// (SELECT EXISTS(SELECT * FROM kphis.opd_er_vs_vital_sign WHERE opd_er_order_master_id=?)) AS has_data_vital_sign,
// (SELECT EXISTS(SELECT * FROM kphis.opd_er_order WHERE opd_er_order_master_id=?)) AS has_data_order,
// (SELECT EXISTS(SELECT * FROM kphis.opd_er_order_progress_note WHERE opd_er_order_master_id=?)) AS has_data_progress_note,
// (SELECT EXISTS(SELECT * FROM kphis.opd_er_focus_list WHERE opd_er_order_master_id=?)) AS has_data_focus_list,
// (SELECT EXISTS(SELECT * FROM kphis.opd_er_focus_note WHERE opd_er_order_master_id=?)) AS has_data_focus_note,
// (SELECT EXISTS(SELECT * FROM kphis.opd_er_io WHERE opd_er_order_master_id=?)) AS has_data_io,
// (SELECT EXISTS(SELECT * FROM kphis.opd_er_med_reconciliation WHERE opd_er_order_master_id=?)) AS has_data_med_reconciliation,
// (SELECT EXISTS(SELECT * FROM kphis_extra.image_usage iu JOIN kphis_extra.opd_er_document doc ON doc.document_id=iu.usage_key_id WHERE iu.usage_id=12 AND doc.document_type_id=1 AND opd_er_order_master_id=?)) AS has_scan_consent,
// (SELECT EXISTS(SELECT * FROM kphis_extra.image_usage iu JOIN kphis_extra.opd_er_document doc ON doc.document_id=iu.usage_key_id WHERE iu.usage_id=12 AND doc.document_type_id=2 AND opd_er_order_master_id=?)) AS has_scan_insure,
// (SELECT EXISTS(SELECT * FROM kphis_extra.image_usage iu JOIN kphis_extra.opd_er_document doc ON doc.document_id=iu.usage_key_id WHERE iu.usage_id=12 AND doc.document_type_id=3 AND opd_er_order_master_id=?)) AS has_scan_refer_in,
// (SELECT EXISTS(SELECT * FROM kphis_extra.image_usage iu JOIN kphis_extra.opd_er_document doc ON doc.document_id=iu.usage_key_id WHERE iu.usage_id=12 AND doc.document_type_id=4 AND opd_er_order_master_id=?)) AS has_scan_refer_out,
// (SELECT EXISTS(SELECT * FROM kphis_extra.image_usage iu JOIN kphis_extra.opd_er_document doc ON doc.document_id=iu.usage_key_id WHERE iu.usage_id=12 AND doc.document_type_id=5 AND opd_er_order_master_id=?)) AS has_scan_culture,
// (SELECT EXISTS(SELECT * FROM kphis_extra.image_usage iu JOIN kphis_extra.opd_er_document doc ON doc.document_id=iu.usage_key_id WHERE iu.usage_id=12 AND doc.document_type_id=6 AND opd_er_order_master_id=?)) AS has_scan_blood,
// (SELECT EXISTS(SELECT * FROM kphis_extra.image_usage iu JOIN kphis_extra.opd_er_document doc ON doc.document_id=iu.usage_key_id WHERE iu.usage_id=12 AND doc.document_type_id=7 AND opd_er_order_master_id=?)) AS has_scan_special,
// (SELECT EXISTS(SELECT * FROM kphis_extra.image_usage iu JOIN kphis_extra.opd_er_document doc ON doc.document_id=iu.usage_key_id WHERE iu.usage_id=12 AND doc.document_type_id=8 AND opd_er_order_master_id=?)) AS has_scan_ekg,
// (SELECT EXISTS(SELECT * FROM kphis_extra.image_usage iu JOIN kphis_extra.opd_er_document doc ON doc.document_id=iu.usage_key_id WHERE iu.usage_id=12 AND doc.document_type_id=9 AND opd_er_order_master_id=?)) AS has_scan_xray,
// (SELECT EXISTS(SELECT * FROM kphis_extra.image_usage iu JOIN kphis_extra.opd_er_document doc ON doc.document_id=iu.usage_key_id WHERE iu.usage_id=12 AND doc.document_type_id=10 AND opd_er_order_master_id=?)) AS has_scan_ct,
// (SELECT EXISTS(SELECT * FROM kphis_extra.image_usage iu JOIN kphis_extra.opd_er_document doc ON doc.document_id=iu.usage_key_id WHERE iu.usage_id=12 AND doc.document_type_id=11 AND opd_er_order_master_id=?)) AS has_scan_mri,
// (SELECT EXISTS(SELECT * FROM kphis_extra.image_usage iu JOIN kphis_extra.opd_er_document doc ON doc.document_id=iu.usage_key_id WHERE iu.usage_id=12 AND doc.document_type_id=12 AND opd_er_order_master_id=?)) AS has_scan_oper,
// (SELECT EXISTS(SELECT * FROM kphis_extra.image_usage iu JOIN kphis_extra.opd_er_document doc ON doc.document_id=iu.usage_key_id WHERE iu.usage_id=12 AND doc.document_type_id=13 AND opd_er_order_master_id=?)) AS has_scan_anes,
// (SELECT EXISTS(SELECT * FROM kphis_extra.image_usage iu JOIN kphis_extra.opd_er_document doc ON doc.document_id=iu.usage_key_id WHERE iu.usage_id=12 AND doc.document_type_id=14 AND opd_er_order_master_id=?)) AS has_scan_labour,
// (SELECT EXISTS(SELECT * FROM kphis_extra.image_usage iu JOIN kphis_extra.opd_er_document doc ON doc.document_id=iu.usage_key_id WHERE iu.usage_id=12 AND doc.document_type_id=15 AND opd_er_order_master_id=?)) AS has_scan_physio,
// (SELECT EXISTS(SELECT * FROM kphis_extra.image_usage iu JOIN kphis_extra.opd_er_document doc ON doc.document_id=iu.usage_key_id WHERE iu.usage_id=12 AND doc.document_type_id=16 AND opd_er_order_master_id=?)) AS has_scan_alt_med,
// (SELECT EXISTS(SELECT * FROM kphis_extra.image_usage iu JOIN kphis_extra.opd_er_document doc ON doc.document_id=iu.usage_key_id WHERE iu.usage_id=12 AND doc.document_type_id=17 AND opd_er_order_master_id=?)) AS has_scan_nutrition,
// (SELECT EXISTS(SELECT * FROM kphis_extra.image_usage iu JOIN kphis_extra.opd_er_document doc ON doc.document_id=iu.usage_key_id WHERE iu.usage_id=12 AND doc.document_type_id=18 AND opd_er_order_master_id=?)) AS has_scan_others,
// (SELECT EXISTS(SELECT * FROM kphis_extra.image_usage iu JOIN kphis_extra.opd_er_document doc ON doc.document_id=iu.usage_key_id WHERE iu.usage_id=12 AND doc.document_type_id=19 AND opd_er_order_master_id=?)) AS has_scan_other_sp_clinic,
// (SELECT EXISTS(SELECT * FROM kphis_extra.image_usage iu JOIN kphis_extra.opd_er_document doc ON doc.document_id=iu.usage_key_id WHERE iu.usage_id=12 AND doc.document_type_id=20 AND opd_er_order_master_id=?)) AS has_scan_opd_card,
// (SELECT EXISTS(SELECT * FROM kphis_extra.image_usage iu JOIN kphis_extra.opd_er_document doc ON doc.document_id=iu.usage_key_id WHERE iu.usage_id=12 AND doc.document_type_id=21 AND opd_er_order_master_id=?)) AS has_scan_finance,
// (SELECT EXISTS(SELECT * FROM hos.lab_head lh INNER JOIN hosxp.lab_order lo ON lh.lab_order_number=lo.lab_order_number WHERE lh.vn=? AND lo.confirm = 'Y')) AS has_data_lab,
// (SELECT EXISTS(SELECT * FROM hos.referout WHERE vn=?)) AS has_data_refer_out;
/// opd_er_order_master_id(u32 x30), vnx2
pub fn get_opd_er_document_exists(hosxp: &str, kphis: &str, kphis_extra: &str) -> String {
    [
        "SELECT \
            (SELECT EXISTS(SELECT * FROM ",kphis,".opd_er_order_master WHERE opd_er_order_master_id=?)) AS has_data_er_master_id,\
            (SELECT EXISTS(SELECT * FROM ",kphis,".opd_er_nurse_index_plan WHERE opd_er_order_master_id=?)) AS has_data_index_plan,\
            (SELECT EXISTS(SELECT * FROM ",kphis,".opd_er_vs_vital_sign WHERE opd_er_order_master_id=?)) AS has_data_vital_sign,\
            (SELECT EXISTS(SELECT * FROM ",kphis,".opd_er_order WHERE opd_er_order_master_id=?)) AS has_data_order,\
            (SELECT EXISTS(SELECT * FROM ",kphis,".opd_er_order_progress_note WHERE opd_er_order_master_id=?)) AS has_data_progress_note,\
            (SELECT EXISTS(SELECT * FROM ",kphis,".opd_er_focus_list WHERE opd_er_order_master_id=?)) AS has_data_focus_list,\
            (SELECT EXISTS(SELECT * FROM ",kphis,".opd_er_focus_note WHERE opd_er_order_master_id=?)) AS has_data_focus_note,\
            (SELECT EXISTS(SELECT * FROM ",kphis,".opd_er_io WHERE opd_er_order_master_id=?)) AS has_data_io,\
            (SELECT EXISTS(SELECT * FROM ",kphis,".opd_er_med_reconciliation WHERE opd_er_order_master_id=?)) AS has_data_med_reconciliation,\
            (SELECT EXISTS(SELECT * FROM ",kphis_extra,".image_usage iu JOIN ",kphis_extra,".opd_er_document doc ON doc.document_id=iu.usage_key_id WHERE iu.usage_id=12 AND doc.document_type_id=1 AND opd_er_order_master_id=?)) AS has_scan_consent,\
            (SELECT EXISTS(SELECT * FROM ",kphis_extra,".image_usage iu JOIN ",kphis_extra,".opd_er_document doc ON doc.document_id=iu.usage_key_id WHERE iu.usage_id=12 AND doc.document_type_id=2 AND opd_er_order_master_id=?)) AS has_scan_insure,\
            (SELECT EXISTS(SELECT * FROM ",kphis_extra,".image_usage iu JOIN ",kphis_extra,".opd_er_document doc ON doc.document_id=iu.usage_key_id WHERE iu.usage_id=12 AND doc.document_type_id=3 AND opd_er_order_master_id=?)) AS has_scan_refer_in,\
            (SELECT EXISTS(SELECT * FROM ",kphis_extra,".image_usage iu JOIN ",kphis_extra,".opd_er_document doc ON doc.document_id=iu.usage_key_id WHERE iu.usage_id=12 AND doc.document_type_id=4 AND opd_er_order_master_id=?)) AS has_scan_refer_out,\
            (SELECT EXISTS(SELECT * FROM ",kphis_extra,".image_usage iu JOIN ",kphis_extra,".opd_er_document doc ON doc.document_id=iu.usage_key_id WHERE iu.usage_id=12 AND doc.document_type_id=5 AND opd_er_order_master_id=?)) AS has_scan_culture,\
            (SELECT EXISTS(SELECT * FROM ",kphis_extra,".image_usage iu JOIN ",kphis_extra,".opd_er_document doc ON doc.document_id=iu.usage_key_id WHERE iu.usage_id=12 AND doc.document_type_id=6 AND opd_er_order_master_id=?)) AS has_scan_blood,\
            (SELECT EXISTS(SELECT * FROM ",kphis_extra,".image_usage iu JOIN ",kphis_extra,".opd_er_document doc ON doc.document_id=iu.usage_key_id WHERE iu.usage_id=12 AND doc.document_type_id=7 AND opd_er_order_master_id=?)) AS has_scan_special,\
            (SELECT EXISTS(SELECT * FROM ",kphis_extra,".image_usage iu JOIN ",kphis_extra,".opd_er_document doc ON doc.document_id=iu.usage_key_id WHERE iu.usage_id=12 AND doc.document_type_id=8 AND opd_er_order_master_id=?)) AS has_scan_ekg,\
            (SELECT EXISTS(SELECT * FROM ",kphis_extra,".image_usage iu JOIN ",kphis_extra,".opd_er_document doc ON doc.document_id=iu.usage_key_id WHERE iu.usage_id=12 AND doc.document_type_id=9 AND opd_er_order_master_id=?)) AS has_scan_xray,\
            (SELECT EXISTS(SELECT * FROM ",kphis_extra,".image_usage iu JOIN ",kphis_extra,".opd_er_document doc ON doc.document_id=iu.usage_key_id WHERE iu.usage_id=12 AND doc.document_type_id=10 AND opd_er_order_master_id=?)) AS has_scan_ct,\
            (SELECT EXISTS(SELECT * FROM ",kphis_extra,".image_usage iu JOIN ",kphis_extra,".opd_er_document doc ON doc.document_id=iu.usage_key_id WHERE iu.usage_id=12 AND doc.document_type_id=11 AND opd_er_order_master_id=?)) AS has_scan_mri,\
            (SELECT EXISTS(SELECT * FROM ",kphis_extra,".image_usage iu JOIN ",kphis_extra,".opd_er_document doc ON doc.document_id=iu.usage_key_id WHERE iu.usage_id=12 AND doc.document_type_id=12 AND opd_er_order_master_id=?)) AS has_scan_oper,\
            (SELECT EXISTS(SELECT * FROM ",kphis_extra,".image_usage iu JOIN ",kphis_extra,".opd_er_document doc ON doc.document_id=iu.usage_key_id WHERE iu.usage_id=12 AND doc.document_type_id=13 AND opd_er_order_master_id=?)) AS has_scan_anes,\
            (SELECT EXISTS(SELECT * FROM ",kphis_extra,".image_usage iu JOIN ",kphis_extra,".opd_er_document doc ON doc.document_id=iu.usage_key_id WHERE iu.usage_id=12 AND doc.document_type_id=14 AND opd_er_order_master_id=?)) AS has_scan_labour,\
            (SELECT EXISTS(SELECT * FROM ",kphis_extra,".image_usage iu JOIN ",kphis_extra,".opd_er_document doc ON doc.document_id=iu.usage_key_id WHERE iu.usage_id=12 AND doc.document_type_id=15 AND opd_er_order_master_id=?)) AS has_scan_physio,\
            (SELECT EXISTS(SELECT * FROM ",kphis_extra,".image_usage iu JOIN ",kphis_extra,".opd_er_document doc ON doc.document_id=iu.usage_key_id WHERE iu.usage_id=12 AND doc.document_type_id=16 AND opd_er_order_master_id=?)) AS has_scan_alt_med,\
            (SELECT EXISTS(SELECT * FROM ",kphis_extra,".image_usage iu JOIN ",kphis_extra,".opd_er_document doc ON doc.document_id=iu.usage_key_id WHERE iu.usage_id=12 AND doc.document_type_id=17 AND opd_er_order_master_id=?)) AS has_scan_nutrition,\
            (SELECT EXISTS(SELECT * FROM ",kphis_extra,".image_usage iu JOIN ",kphis_extra,".opd_er_document doc ON doc.document_id=iu.usage_key_id WHERE iu.usage_id=12 AND doc.document_type_id=18 AND opd_er_order_master_id=?)) AS has_scan_others,\
            (SELECT EXISTS(SELECT * FROM ",kphis_extra,".image_usage iu JOIN ",kphis_extra,".opd_er_document doc ON doc.document_id=iu.usage_key_id WHERE iu.usage_id=12 AND doc.document_type_id=19 AND opd_er_order_master_id=?)) AS has_scan_other_sp_clinic,\
            (SELECT EXISTS(SELECT * FROM ",kphis_extra,".image_usage iu JOIN ",kphis_extra,".opd_er_document doc ON doc.document_id=iu.usage_key_id WHERE iu.usage_id=12 AND doc.document_type_id=20 AND opd_er_order_master_id=?)) AS has_scan_opd_card,\
            (SELECT EXISTS(SELECT * FROM ",kphis_extra,".image_usage iu JOIN ",kphis_extra,".opd_er_document doc ON doc.document_id=iu.usage_key_id WHERE iu.usage_id=12 AND doc.document_type_id=21 AND opd_er_order_master_id=?)) AS has_scan_finance,\
            (SELECT EXISTS(SELECT * FROM ",hosxp,".lab_head lh INNER JOIN ",hosxp,".lab_order lo ON lh.lab_order_number=lo.lab_order_number WHERE lh.vn=? AND lo.confirm = 'Y')) AS has_data_lab,\
            (SELECT EXISTS(SELECT * FROM ",hosxp,".referout WHERE vn=?)) AS has_data_refer_out;"
    ].concat()
}

// SELECT doc.document_id,doc.document_type_id, 
//     (SELECT EXISTS(SELECT * FROM kphis_extra.image_usage iu WHERE iu.usage_id=12 AND doc.document_id=iu.usage_key_id)) AS has_image
// FROM kphis_extra.opd_er_document doc WHERE opd_er_order_master_id=?;
/// opd_er_order_master_id
pub fn get_opd_er_document_types(kphis_extra: &str) -> String {
    [
        "SELECT doc.document_id,doc.document_type_id,\
            (SELECT EXISTS(SELECT * FROM ",kphis_extra,".image_usage iu WHERE iu.usage_id=12 AND doc.document_id=iu.usage_key_id)) AS has_image \
        FROM ",kphis_extra,".opd_er_document doc WHERE opd_er_order_master_id=?;"
    ].concat()
}

// INSERT IGNORE INTO kphis_extra.opd_er_document (opd_er_order_master_id,document_type_id,create_user,create_datetime,update_user,update_datetime,version) VALUES (?,?,?,NOW(),?,NOW(),1);
/// opd_er_order_master_id, document_type_id, loginname, loginname
pub fn insert_ignore_opd_er_document_type(kphis_extra: &str) -> String {
    [
        "INSERT IGNORE INTO ",kphis_extra,".opd_er_document (opd_er_order_master_id,document_type_id",TABLE_CREATE_COLUMNS,") VALUES (?,?",TABLE_CREATE_PREPARED,");"
    ].concat()
}

// DELETE FROM kphis_extra.opd_er_document WHERE opd_er_order_master_id=? AND document_type_id=? AND NOT (SELECT EXISTS(SELECT * FROM kphis_extra.image_usage iu WHERE iu.usage_id=12 AND document_id=iu.usage_key_id));
/// opd_er_order_master_id, document_type_id
pub fn delete_opd_er_document_type(kphis_extra: &str) -> String {
    [
        "DELETE FROM ",kphis_extra,".opd_er_document WHERE opd_er_order_master_id=? AND document_type_id=? AND NOT (SELECT EXISTS(SELECT * FROM ",kphis_extra,".image_usage iu WHERE iu.usage_id=12 AND document_id=iu.usage_key_id));"
    ].concat()
}