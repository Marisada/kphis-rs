use kphis_model::med_reconcile::MedReconciliationParams;

use crate::{TABLE_CREATE_COLUMNS, TABLE_CREATE_PREPARED, TABLE_UPDATE_SET};

// // ipd-dr-med-reconcile-data.php
// SELECT mr.med_reconciliation_id,mr.opd_er_order_master_id,mr.pharmacist,mr.note,mr.doctor,mr.med_reconciliation_datetime,
//     mr.phamacist_confirm_datetime,mr.doctor_confirm_datetime,pd.name AS pharmacist_name,dd.name AS doctor_name,
//     IF(mr.pharmacist=?,'Y','N') AS is_pharmacist_current_user_doctor
// FROM kphis.opd_er_med_reconciliation mr
//     LEFT JOIN hos.doctor pd ON pd.code=mr.pharmacist
//     LEFT JOIN hos.doctor dd ON dd.code=mr.doctor
// WHERE (
//     (mr.phamacist_confirm_datetime IS NULL AND mr.pharmacist=?) OR (mr.phamacist_confirm_datetime IS NOT NULL)
// )
// ORDER BY mr.med_reconciliation_id;
/// check_pharmacist==true(current_user_doctorcode, current_user_doctorcode), (med_reconciliation_id), (opd_er_order_master_id)
pub fn get_med_reconciliation(
    check_phamacist: bool,
    params: &MedReconciliationParams,
    hosxp: &str,
    kphis: &str,
) -> String {
    let med_reconciliation_id = if params.med_reconciliation_id.is_some() {" AND mr.med_reconciliation_id=? "} else {""};
    let opd_er_order_master_id = if params.opd_er_order_master_id.is_some() {" AND mr.opd_er_order_master_id=? "} else {""};
    let (is_pharmacist, where_phamacist) = if check_phamacist {(
        "IF(mr.pharmacist=?,'Y','N')",
        " ((mr.phamacist_confirm_datetime IS NULL AND mr.pharmacist=?) OR (mr.phamacist_confirm_datetime IS NOT NULL)) "
    )} else {(
        "'U'", " 1=1 "
    )};
    [
        "SELECT mr.med_reconciliation_id,mr.opd_er_order_master_id,mr.pharmacist,mr.note,mr.doctor,mr.med_reconciliation_datetime,\
            mr.phamacist_confirm_datetime,mr.doctor_confirm_datetime,pd.name AS pharmacist_name,dd.name AS doctor_name,",
            is_pharmacist," AS is_pharmacist_current_user_doctor \
        FROM ",kphis,".opd_er_med_reconciliation mr \
            LEFT JOIN ",hosxp,".doctor pd ON pd.code=mr.pharmacist \
            LEFT JOIN ",hosxp,".doctor dd ON dd.code=mr.doctor \
        WHERE ",where_phamacist,med_reconciliation_id,opd_er_order_master_id,
        "ORDER BY mr.med_reconciliation_id;"
    ].concat()
}

// SELECT mri.med_reconciliation_item_id,mri.med_reconciliation_id,mri.opd_er_order_master_id,mri.icode,mri.med_name,mri.custom_med_name,mri.receive_from,mri.receive_date,
//     mri.old_drugusage,mri.changed_drugusage,mri.receive_qty,mri.last_dose_taken_time,mri.last_dose_taken_remark,mri.`use`,dud.`usage` AS due_usage,dud.info,dud.info_status,
//     GROUP_CONCAT(DISTINCT(allergy.agent) ORDER BY allergy.agent) AS allergy_agent,
//     GROUP_CONCAT(DISTINCT(CONCAT(allergy.agent,'=',IFNULL(allergy.symptom,''))) ORDER BY allergy.agent) AS allergy_agent_symptom,
//     SUM(IF(allergy.force_no_order='Y',1,0)) AS allergy_count_force_no_order,
//     CONCAT(IFNULL(du.name1,''),' ',IFNULL(du.name2,''),' ',IFNULL(du.name3,'')),di.generic_name,di.show_notify,di.show_notify_text
// FROM kphis.opd_er_med_reconciliation_item mri
//     LEFT JOIN kphis.kphis_drug_use_duration dud ON dud.icode=mri.icode AND dud.status='Y'
//     LEFT JOIN hos.drugitems di ON di.icode=mri.icode
//     LEFT JOIN hos.drugusage du ON di.drugusage=du.drugusage
//     LEFT JOIN hos.opd_allergy allergy ON
//     ((allergy.agent LIKE CONCAT('%',di.generic_name,'%') AND allergy.hn=? AND di.generic_name IS NOT NULL AND TRIM(di.generic_name) <> '')
//         OR (di.generic_name LIKE CONCAT('%',allergy.agent,'%') AND allergy.hn=? AND allergy.agent IS NOT NULL AND TRIM(allergy.agent) <> ''))
// WHERE mri.med_reconciliation_id=?
// GROUP BY mri.med_reconciliation_item_id ORDER BY mri.med_reconciliation_id;
/// hn, hn, (used)
pub fn get_med_reconciliation_item(
    params: &MedReconciliationParams,
    ids: &[u32],
    hosxp: &str,
    kphis: &str,
) -> String {
    let med_reconciliation_ids = if ids.is_empty() {
        String::from("0")
    } else {
        ids.iter().map(|i| i.to_string()).collect::<Vec<String>>().join(",")
    };
    let used = if params.used.is_some() {" AND mri.`use`=? "} else {""};
    [
        "SELECT mri.med_reconciliation_item_id,mri.med_reconciliation_id,mri.opd_er_order_master_id,mri.icode,mri.med_name,mri.custom_med_name,mri.receive_from,mri.receive_date,\
            mri.old_drugusage,mri.changed_drugusage,mri.receive_qty,mri.last_dose_taken_time,mri.last_dose_taken_remark,mri.`use` AS used,dud.`usage` AS due_usage,dud.`status` AS due_status,dud.info,dud.info_status,\
            GROUP_CONCAT(DISTINCT(allergy.agent) ORDER BY allergy.agent) AS allergy_agent,\
            GROUP_CONCAT(DISTINCT(CONCAT(allergy.agent,'=',IFNULL(allergy.symptom,''))) ORDER BY allergy.agent) AS allergy_agent_symptom,\
            SUM(IF(allergy.force_no_order='Y',1,0)) AS allergy_count_force_no_order,\
            di.generic_name,di.dosageform,di.show_notify,di.show_notify_text \
        FROM ",kphis,".opd_er_med_reconciliation_item mri \
            LEFT JOIN ",kphis,".kphis_drug_use_duration dud ON dud.icode=mri.icode \
            LEFT JOIN ",hosxp,".drugitems di ON di.icode=mri.icode \
            LEFT JOIN ",hosxp,".opd_allergy allergy ON \
            ((allergy.agent LIKE CONCAT('%',di.generic_name,'%') AND allergy.hn=? AND di.generic_name IS NOT NULL AND TRIM(di.generic_name) <> '') \
                OR (di.generic_name LIKE CONCAT('%',allergy.agent,'%') AND allergy.hn=? AND allergy.agent IS NOT NULL AND TRIM(allergy.agent) <> '')) \
        WHERE mri.med_reconciliation_id IN (",&med_reconciliation_ids,") ", used,
        "GROUP BY mri.med_reconciliation_item_id ORDER BY mri.med_reconciliation_id;"
    ].concat()
}

// // ipd-dr-med-reconcile-save.php
// SELECT med_reconciliation_id
// FROM kphis.opd_er_med_reconciliation
// WHERE opd_er_order_master_id=? AND pharmacist=? AND phamacist_confirm_datetime IS NULL AND doctor_confirm_datetime IS NULL
// ORDER BY med_reconciliation_id DESC LIMIT 1;
/// opd_er_order_master_id, doctor_code
pub fn get_last_unconfirm_mr(kphis: &str) -> String {
    [
        "SELECT med_reconciliation_id \
        FROM ",kphis,".opd_er_med_reconciliation \
        WHERE opd_er_order_master_id=? AND pharmacist=? AND phamacist_confirm_datetime IS NULL AND doctor_confirm_datetime IS NULL \
        ORDER BY med_reconciliation_id DESC LIMIT 1;"
    ].concat()
}
// UPDATE kphis.opd_er_med_reconciliation
// SET update_user=?,update_datetime=NOW(),version=(version+1)
// WHERE med_reconciliation_id=?;
/// loginname, med_reconciliation_id
pub fn update_mr(kphis: &str) -> String {
    [
        "UPDATE ",kphis,".opd_er_med_reconciliation \
        SET update_user=?,update_datetime=NOW(),version=(version+1) \
        WHERE med_reconciliation_id=?;"
    ].concat()
}
// INSERT INTO kphis.opd_er_med_reconciliation (opd_er_order_master_id,pharmacist,med_reconciliation_datetime,create_user,create_datetime,update_user,update_datetime,version)
// VALUES (?,?,NOW(),?,NOW(),?,NOW(),1);
/// opd_er_order_master_id, doctor_code, loginname, loginname
pub fn insert_mr(kphis: &str) -> String {
    [
        "INSERT INTO ",kphis,".opd_er_med_reconciliation (opd_er_order_master_id,pharmacist,med_reconciliation_datetime",TABLE_CREATE_COLUMNS,") \
        VALUES (?,?,NOW()",TABLE_CREATE_PREPARED,");"
    ].concat()
}
// INSERT INTO kphis.opd_er_med_reconciliation_item (
//     med_reconciliation_id,opd_er_order_master_id,icode,med_name,custom_med_name,receive_from,receive_date,old_drugusage,receive_qty,
//     create_user,create_datetime,update_user,update_datetime,version)
// VALUES (?,?,?,?,?,?,?,?,?,?,NOW(),?,NOW(),1);
/// med_reconciliation_id, opd_er_order_master_id, icode, med_name, custom_med_name, receive_from, receive_date, old_drugusage, receive_qty, loginname, loginname
pub fn insert_mri(len: usize, kphis: &str) -> String {
    let values = vec!["(?,?,?,?,?,?,?,?,?,?,NOW(),?,NOW(),1)"; len].join(",");
    [
        "INSERT INTO ",kphis,".opd_er_med_reconciliation_item (med_reconciliation_id,\
            opd_er_order_master_id,icode,med_name,custom_med_name,receive_from,receive_date,old_drugusage,receive_qty",
        TABLE_CREATE_COLUMNS,") VALUES ", &values
    ].concat()
}

// // ipd-dr-med-reconcile-doctor-confirm.php
// UPDATE kphis.opd_er_med_reconciliation
// SET doctor=?,doctor_confirm_datetime=NOW(),update_user=?,update_datetime=NOW(),version=(version+1)
// WHERE med_reconciliation_id=? AND doctor_confirm_datetime IS NULL AND phamacist_confirm_datetime IS NOT NULL;
/// doctor_code, loginname, med_reconciliation_id
pub fn update_mr_doctor_confirm(kphis: &str) -> String {
    [
        "UPDATE ",kphis,".opd_er_med_reconciliation \
        SET doctor=?,doctor_confirm_datetime=NOW()",TABLE_UPDATE_SET,
        " WHERE med_reconciliation_id=? AND phamacist_confirm_datetime IS NOT NULL;"
    ].concat()
}
// UPDATE kphis.opd_er_med_reconciliation_item
// SET `use`=?,changed_drugusage=?,last_dose_taken_time=?,last_dose_taken_remark=?,
//     update_user=?,update_datetime=NOW(),version=(version+1)
// WHERE med_reconciliation_item_id=?;
/// used, changed_drugusage, last_dose_taken_time, last_dose_taken_remark, loginname, med_reconciliation_item_id
pub fn update_mri_doctor_confirm(kphis: &str) -> String {
    [
        "UPDATE ",kphis,".opd_er_med_reconciliation_item \
        SET `use`=?,changed_drugusage=?,last_dose_taken_time=?,last_dose_taken_remark=?",TABLE_UPDATE_SET,
        " WHERE med_reconciliation_item_id=?;"
    ].concat()
}

// // ipd-dr-med-reconcile-pharmacist-confirm.php
// UPDATE kphis.opd_er_med_reconciliation
// SET phamacist_confirm_datetime=NOW(),update_user=?,update_datetime=NOW(),version=(version+1)
// WHERE med_reconciliation_id=? AND pharmacist=? AND phamacist_confirm_datetime IS NULL;
/// loginname, med_reconciliation_id, doctor_code
pub fn update_mr_pharm_confirm(kphis: &str) -> String {
    [
        "UPDATE ",kphis,".opd_er_med_reconciliation \
        SET phamacist_confirm_datetime=NOW()",TABLE_UPDATE_SET,
        " WHERE med_reconciliation_id=? AND pharmacist=? AND phamacist_confirm_datetime IS NULL;"
    ].concat()
}
// UPDATE kphis.opd_er_med_reconciliation_item
// SET old_drugusage=?,receive_qty=?,receive_from=?,receive_date=?,last_dose_taken_time=?,last_dose_taken_remark=?,update_user=?,update_datetime=NOW(),version=(version+1)
// WHERE med_reconciliation_item_id=?;
/// old_drugusage, receive_qty, receive_from, receive_date, last_dose_taken_time, last_dose_taken_remark, loginname, med_reconciliation_item_id
pub fn update_mri_pharm_confirm(kphis: &str) -> String {
    [
        "UPDATE ",kphis,".opd_er_med_reconciliation_item \
        SET old_drugusage=?,receive_qty=?,receive_from=?,receive_date=?,last_dose_taken_time=?,last_dose_taken_remark=?",TABLE_UPDATE_SET,
        " WHERE med_reconciliation_item_id=?;"
    ].concat()
}

// // ipd-dr-med-reconcile-pharmacist-unconfirm.php
// UPDATE kphis.opd_er_med_reconciliation
// SET phamacist_confirm_datetime=NULL,update_user=?,update_datetime=NOW(),version=(version+1)
// WHERE med_reconciliation_id=? AND pharmacist=? AND doctor_confirm_datetime IS NULL AND phamacist_confirm_datetime IS NOT NULL;
/// loginname, med_reconciliation_id, doctor_code
pub fn update_mr_pharm_unconfirm(kphis: &str) -> String {
    [
        "UPDATE ",kphis,".opd_er_med_reconciliation \
        SET phamacist_confirm_datetime=NULL",TABLE_UPDATE_SET,
        " WHERE med_reconciliation_id=? AND pharmacist=? AND doctor_confirm_datetime IS NULL AND phamacist_confirm_datetime IS NOT NULL;"
    ].concat()
}

// UPDATE kphis.opd_er_med_reconciliation_item
// SET receive_qty=?,receive_from=?,receive_date=?,update_user=?,update_datetime=NOW(),version=(version+1)
// WHERE med_reconciliation_item_id=?;
/// receive_qty, receive_from, receive_date, loginname, med_reconciliation_item_id
pub fn update_mri_receive(kphis: &str) -> String {
    [
        "UPDATE ",kphis,".opd_er_med_reconciliation_item \
        SET receive_qty=?,receive_from=?,receive_date=?",TABLE_UPDATE_SET,
        " WHERE med_reconciliation_item_id=?;"
    ].concat()
}

// // ipd-dr-med-reconcile-last-dose-save.php
// UPDATE kphis.opd_er_med_reconciliation_item
// SET last_dose_taken_time=?,last_dose_taken_remark=?,update_user=?,update_datetime=NOW(),version=(version+1)
// WHERE med_reconciliation_item_id=?;
/// last_dose_taken_time, last_dose_taken_remark, loginname, med_reconciliation_item_id
pub fn update_mri_last_dose(kphis: &str) -> String {
    [
        "UPDATE ",kphis,".opd_er_med_reconciliation_item \
        SET last_dose_taken_time=?,last_dose_taken_remark=?",TABLE_UPDATE_SET,
        " WHERE med_reconciliation_item_id=?;"
    ].concat()
}

// // ipd-dr-med-reconcile-delete.php
// *** cannot use alias in delete `https://bugs.mysql.com/bug.php?id=82189` ***
// DELETE kphis.opd_er_med_reconciliation,kphis.opd_er_med_reconciliation_item
// FROM kphis.opd_er_med_reconciliation
//     LEFT JOIN kphis.opd_er_med_reconciliation_item ON opd_er_med_reconciliation.med_reconciliation_id=opd_er_med_reconciliation_item.med_reconciliation_id
// WHERE opd_er_med_reconciliation.med_reconciliation_id=? AND opd_er_med_reconciliation.phamacist_confirm_datetime IS NULL;
/// med_reconciliation_id
pub fn delete_med_reconciliation(kphis: &str) -> String {
    [
        "DELETE ",kphis,".opd_er_med_reconciliation,",kphis,".opd_er_med_reconciliation_item \
        FROM ",kphis,".opd_er_med_reconciliation \
            LEFT JOIN ",kphis,".opd_er_med_reconciliation_item ON opd_er_med_reconciliation.med_reconciliation_id=opd_er_med_reconciliation_item.med_reconciliation_id \
        WHERE opd_er_med_reconciliation.med_reconciliation_id=? AND opd_er_med_reconciliation.phamacist_confirm_datetime IS NULL;"
    ].concat()
}

// // ipd-dr-med-reconcile-item-delete.php
// DELETE kphis.opd_er_med_reconciliation_item FROM kphis.opd_er_med_reconciliation_item
// INNER JOIN kphis.opd_er_med_reconciliation ON opd_er_med_reconciliation.med_reconciliation_id=opd_er_med_reconciliation_item.med_reconciliation_id
// WHERE opd_er_med_reconciliation_item.med_reconciliation_item_id=? AND opd_er_med_reconciliation.phamacist_confirm_datetime IS NULL;
/// med_reconciliation_item_id
pub fn delete_med_reconciliation_item(kphis: &str) -> String {
    [
        "DELETE ",kphis,".opd_er_med_reconciliation_item FROM ",kphis,".opd_er_med_reconciliation_item \
        INNER JOIN ",kphis,".opd_er_med_reconciliation ON opd_er_med_reconciliation.med_reconciliation_id=opd_er_med_reconciliation_item.med_reconciliation_id \
        WHERE opd_er_med_reconciliation_item.med_reconciliation_item_id=? AND opd_er_med_reconciliation.phamacist_confirm_datetime IS NULL;"
    ].concat()
}

// // ipd-dr-med-reconcile-note-data.php
// SELECT med_reconciliation_id,phamacist_confirm_datetime,note
//     FROM kphis.opd_er_med_reconciliation
// WHERE med_reconciliation_id=?
// ORDER BY med_reconciliation_id DESC;
/// med_reconciliation_id
pub fn get_note(kphis: &str) -> String {
    [
        "SELECT med_reconciliation_id,phamacist_confirm_datetime,note \
            FROM ",kphis,".opd_er_med_reconciliation \
        WHERE med_reconciliation_id=?;"
    ].concat()
}

// // ipd-dr-med-reconcile-note-save.php
// UPDATE kphis.opd_er_med_reconciliation SET note=?,update_user=?,update_datetime=NOW(),version=(version+1)
// WHERE med_reconciliation_id=? AND phamacist_confirm_datetime IS NULL;
/// note, loginname, med_reconciliation_id
pub fn post_note(kphis: &str) -> String {
    [
        "UPDATE ",kphis,".opd_er_med_reconciliation SET note=?",TABLE_UPDATE_SET,
        " WHERE med_reconciliation_id=? AND phamacist_confirm_datetime IS NULL;"
    ].concat()
}

// // GET /exists-key-id/opd-er-med-reconcile/:opd_er_order_master_id
// SELECT COUNT(*) AS med_reconciliation_count FROM kphis.opd_er_med_reconciliation WHERE an=?;
/// opd_er_order_master_id
pub fn get_med_reconcile_exists(kphis: &str) -> String {
    [
        "SELECT EXISTS(SELECT * FROM ",kphis,".opd_er_med_reconciliation WHERE opd_er_order_master_id=? AND phamacist_confirm_datetime IS NOT NULL) AS exs;"
    ].concat()
}

// // GET /exists-key-id/opd-er-med-reconcile-dr-unconfirm/:opd_er_order_master_id
// SELECT COUNT(*) AS doctor_unconfirm_med_reconciliation_count
// FROM kphis.opd_er_med_reconciliation
// WHERE opd_er_order_master_id=? AND phamacist_confirm_datetime IS NOT NULL AND doctor_confirm_datetime IS NULL;
/// opd_er_order_master_id
pub fn get_med_reconcile_doctor_unconfirm_exists(kphis: &str) -> String {
    [
        "SELECT EXISTS(SELECT * FROM ",kphis,".opd_er_med_reconciliation \
        WHERE opd_er_order_master_id=? AND phamacist_confirm_datetime IS NOT NULL AND doctor_confirm_datetime IS NULL) AS exs;"
    ].concat()
}
