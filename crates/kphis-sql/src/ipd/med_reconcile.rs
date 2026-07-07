use kphis_model::med_reconcile::MedReconciliationParams;

use crate::{TABLE_CREATE_COLUMNS, TABLE_CREATE_PREPARED, TABLE_UPDATE_SET};

// // ipd-dr-med-reconcile-data.php
// SELECT mr.med_reconciliation_id,mr.an,mr.pharmacist,mr.note,mr.doctor,mr.med_reconciliation_datetime,
//     mr.phamacist_confirm_datetime,mr.doctor_confirm_datetime,pd.name AS pharmacist_name,dd.name AS doctor_name,
//     IF(mr.pharmacist=?,'Y','N') AS is_pharmacist_current_user_doctor
// FROM kphis.ipd_med_reconciliation mr
//     LEFT JOIN hos.doctor pd ON pd.code=mr.pharmacist
//     LEFT JOIN hos.doctor dd ON dd.code=mr.doctor
// WHERE (
//     (mr.phamacist_confirm_datetime IS NULL AND mr.pharmacist=?) OR (mr.phamacist_confirm_datetime IS NOT NULL)
// )
// ORDER BY mr.med_reconciliation_id;
/// check_pharmacist==true(current_user_doctorcode, current_user_doctorcode), (med_reconciliation_id), (an)
pub fn get_med_reconciliation(
    check_phamacist: bool,
    params: &MedReconciliationParams,
    hosxp: &str,
    kphis: &str,
) -> String {
    let med_reconciliation_id = if params.med_reconciliation_id.is_some() {" AND mr.med_reconciliation_id=? "} else {""};
    let an = if params.an.is_some() {" AND mr.an=? "} else {""};
    let (is_pharmacist, where_phamacist) = if check_phamacist {(
        "IF(mr.pharmacist=?,'Y','N')",
        " ((mr.phamacist_confirm_datetime IS NULL AND mr.pharmacist=?) OR (mr.phamacist_confirm_datetime IS NOT NULL)) "
    )} else {(
        "'U'", " 1=1 "
    )};
    [
        "SELECT mr.med_reconciliation_id,mr.an,mr.pharmacist,mr.note,mr.doctor,mr.med_reconciliation_datetime,\
            mr.phamacist_confirm_datetime,mr.doctor_confirm_datetime,pd.name AS pharmacist_name,dd.name AS doctor_name,",
            is_pharmacist," AS is_pharmacist_current_user_doctor \
        FROM ",kphis,".ipd_med_reconciliation mr \
            LEFT JOIN ",hosxp,".doctor pd ON pd.code=mr.pharmacist \
            LEFT JOIN ",hosxp,".doctor dd ON dd.code=mr.doctor \
        WHERE ",where_phamacist,med_reconciliation_id,an,
        "ORDER BY mr.med_reconciliation_id;"
    ].concat()
}

// SELECT mri.med_reconciliation_item_id,mri.med_reconciliation_id,mri.an,mri.icode,mri.med_name,mri.custom_med_name,mri.receive_from,mri.receive_date,
//     mri.old_drugusage,mri.changed_drugusage,mri.receive_qty,mri.last_dose_taken_time,mri.last_dose_taken_remark,mri.`use`,dud.`usage` AS due_usage,dud.info,dud.info_status,
//     GROUP_CONCAT(DISTINCT(allergy.agent) ORDER BY allergy.agent) AS allergy_agent,
//     GROUP_CONCAT(DISTINCT(CONCAT(allergy.agent,'=',IFNULL(allergy.symptom,''))) ORDER BY allergy.agent) AS allergy_agent_symptom,
//     SUM(IF(allergy.force_no_order='Y',1,0)) AS allergy_count_force_no_order,
//     CONCAT(IFNULL(du.name1,''),' ',IFNULL(du.name2,''),' ',IFNULL(du.name3,'')) AS `usage`,di.generic_name,di.show_notify,di.show_notify_text
// FROM kphis.ipd_med_reconciliation_item mri
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
        "SELECT mri.med_reconciliation_item_id,mri.med_reconciliation_id,mri.an,mri.icode,mri.med_name,mri.custom_med_name,mri.receive_from,mri.receive_date,\
            mri.old_drugusage,mri.changed_drugusage,mri.receive_qty,mri.last_dose_taken_time,mri.last_dose_taken_remark,mri.`use` AS used,dud.`usage` AS due_usage,dud.`status` AS due_status,dud.info,dud.info_status,\
            GROUP_CONCAT(DISTINCT(allergy.agent) ORDER BY allergy.agent) AS allergy_agent,\
            GROUP_CONCAT(DISTINCT(CONCAT(allergy.agent,'=',IFNULL(allergy.symptom,''))) ORDER BY allergy.agent) AS allergy_agent_symptom,\
            SUM(IF(allergy.force_no_order='Y',1,0)) AS allergy_count_force_no_order,\
            di.generic_name,di.dosageform,di.show_notify,di.show_notify_text \
        FROM ",kphis,".ipd_med_reconciliation_item mri \
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
// FROM kphis.ipd_med_reconciliation
// WHERE an=? AND pharmacist=? AND phamacist_confirm_datetime IS NULL AND doctor_confirm_datetime IS NULL
// ORDER BY med_reconciliation_id DESC LIMIT 1;
/// an, doctor_code
pub fn get_last_unconfirm_mr(kphis: &str) -> String {
    [
        "SELECT med_reconciliation_id \
        FROM ",kphis,".ipd_med_reconciliation \
        WHERE an=? AND pharmacist=? AND phamacist_confirm_datetime IS NULL AND doctor_confirm_datetime IS NULL \
        ORDER BY med_reconciliation_id DESC LIMIT 1;"
    ].concat()
}
// UPDATE kphis.ipd_med_reconciliation
// SET update_user=?,update_datetime=NOW(),version=(version+1)
// WHERE med_reconciliation_id=?;
/// loginname, med_reconciliation_id
pub fn update_mr(kphis: &str) -> String {
    [
        "UPDATE ",kphis,".ipd_med_reconciliation \
        SET update_user=?,update_datetime=NOW(),version=(version+1) \
        WHERE med_reconciliation_id=?;"
    ].concat()
}
// INSERT INTO kphis.ipd_med_reconciliation (an,pharmacist,med_reconciliation_datetime,create_user,create_datetime,update_user,update_datetime,version)
// VALUES (?,?,NOW(),?,NOW(),?,NOW(),1);
/// an, doctor_code, loginname, loginname
pub fn insert_mr(kphis: &str) -> String {
    [
        "INSERT INTO ",kphis,".ipd_med_reconciliation (an,pharmacist,med_reconciliation_datetime",TABLE_CREATE_COLUMNS,") \
        VALUES (?,?,NOW()",TABLE_CREATE_PREPARED,");"
    ].concat()
}
// INSERT INTO kphis.ipd_med_reconciliation_item (
//     med_reconciliation_id,an,icode,med_name,custom_med_name,receive_from,receive_date,old_drugusage,receive_qty,
//     create_user,create_datetime,update_user,update_datetime,version)
// VALUES (?,?,?,?,?,?,?,?,?,?,NOW(),?,NOW(),1);
/// med_reconciliation_id, an, icode, med_name, custom_med_name, receive_from, receive_date, old_drugusage, receive_qty, loginname, loginname
pub fn insert_mri(len: usize, kphis: &str) -> String {
    let values = vec!["(?,?,?,?,?,?,?,?,?,?,NOW(),?,NOW(),1)"; len].join(",");
    [
        "INSERT INTO ",kphis,".ipd_med_reconciliation_item (med_reconciliation_id,\
            an,icode,med_name,custom_med_name,receive_from,receive_date,old_drugusage,receive_qty",
        TABLE_CREATE_COLUMNS,") VALUES ", &values
    ].concat()
}

// // ipd-dr-med-reconcile-doctor-confirm.php
// UPDATE kphis.ipd_med_reconciliation
// SET doctor=?,doctor_confirm_datetime=NOW(),update_user=?,update_datetime=NOW(),version=(version+1)
// WHERE med_reconciliation_id=? AND doctor_confirm_datetime IS NULL AND phamacist_confirm_datetime IS NOT NULL;
/// doctor_code, loginname, med_reconciliation_id
pub fn update_mr_doctor_confirm(kphis: &str) -> String {
    [
        "UPDATE ",kphis,".ipd_med_reconciliation \
        SET doctor=?,doctor_confirm_datetime=NOW()",TABLE_UPDATE_SET,
        " WHERE med_reconciliation_id=? AND phamacist_confirm_datetime IS NOT NULL;"
    ].concat()
}
// UPDATE kphis.ipd_med_reconciliation_item
// SET `use`=?,changed_drugusage=?,last_dose_taken_time=?,last_dose_taken_remark=?,
//     update_user=?,update_datetime=NOW(),version=(version+1)
// WHERE med_reconciliation_item_id=?;
/// used, changed_drugusage, last_dose_taken_time, last_dose_taken_remark, loginname, med_reconciliation_item_id
pub fn update_mri_doctor_confirm(kphis: &str) -> String {
    [
        "UPDATE ",kphis,".ipd_med_reconciliation_item \
        SET `use`=?,changed_drugusage=?,last_dose_taken_time=?,last_dose_taken_remark=?",TABLE_UPDATE_SET,
        " WHERE med_reconciliation_item_id=?;"
    ].concat()
}

// // ipd-dr-med-reconcile-pharmacist-confirm.php
// UPDATE kphis.ipd_med_reconciliation
// SET phamacist_confirm_datetime=NOW(),update_user=?,update_datetime=NOW(),version=(version+1)
// WHERE med_reconciliation_id=? AND pharmacist=? AND phamacist_confirm_datetime IS NULL;
/// loginname, med_reconciliation_id, doctor_code
pub fn update_mr_pharm_confirm(kphis: &str) -> String {
    [
        "UPDATE ",kphis,".ipd_med_reconciliation \
        SET phamacist_confirm_datetime=NOW()",TABLE_UPDATE_SET,
        " WHERE med_reconciliation_id=? AND pharmacist=? AND phamacist_confirm_datetime IS NULL;"
    ].concat()
}
// UPDATE kphis.ipd_med_reconciliation_item
// SET old_drugusage=?,receive_qty=?,receive_from=?,receive_date=?,last_dose_taken_time=?,last_dose_taken_remark=?,update_user=?, update_datetime=NOW(), version=(version+1)
// WHERE med_reconciliation_item_id=?;
/// old_drugusage, receive_qty, receive_from, receive_date, last_dose_taken_time, last_dose_taken_remark, loginname, med_reconciliation_item_id
pub fn update_mri_pharm_confirm(kphis: &str) -> String {
    [
        "UPDATE ",kphis,".ipd_med_reconciliation_item \
        SET old_drugusage=?,receive_qty=?,receive_from=?,receive_date=?,last_dose_taken_time=?,last_dose_taken_remark=?",TABLE_UPDATE_SET,
        " WHERE med_reconciliation_item_id=?;"
    ].concat()
}

// // ipd-dr-med-reconcile-pharmacist-unconfirm.php
// UPDATE kphis.ipd_med_reconciliation
// SET phamacist_confirm_datetime=NULL,update_user=?,update_datetime=NOW(),version=(version+1)
// WHERE med_reconciliation_id=? AND pharmacist=? AND doctor_confirm_datetime IS NULL AND phamacist_confirm_datetime IS NOT NULL;
/// loginname, med_reconciliation_id, doctor_code
pub fn update_mr_pharm_unconfirm(kphis: &str) -> String {
    [
        "UPDATE ",kphis,".ipd_med_reconciliation \
        SET phamacist_confirm_datetime=NULL",TABLE_UPDATE_SET,
        " WHERE med_reconciliation_id=? AND pharmacist=? AND doctor_confirm_datetime IS NULL AND phamacist_confirm_datetime IS NOT NULL;"
    ].concat()
}

// UPDATE kphis.ipd_med_reconciliation_item
// SET receive_qty=?,receive_from=?,receive_date=?,update_user=?,update_datetime=NOW(),version=(version+1)
// WHERE med_reconciliation_item_id=?;
/// receive_qty, receive_from, receive_date, loginname, med_reconciliation_item_id
pub fn update_mri_receive(kphis: &str) -> String {
    [
        "UPDATE ",kphis,".ipd_med_reconciliation_item \
        SET receive_qty=?,receive_from=?,receive_date=?",TABLE_UPDATE_SET,
        " WHERE med_reconciliation_item_id=?;"
    ].concat()
}

// // ipd-dr-med-reconcile-last-dose-save.php
// UPDATE kphis.ipd_med_reconciliation_item
// SET last_dose_taken_time=?,last_dose_taken_remark=?,update_user=?,update_datetime=NOW(),version=(version+1)
// WHERE med_reconciliation_item_id=?;
/// last_dose_taken_time, last_dose_taken_remark, loginname, med_reconciliation_item_id
pub fn update_mri_last_dose(kphis: &str) -> String {
    [
        "UPDATE ",kphis,".ipd_med_reconciliation_item \
        SET last_dose_taken_time=?,last_dose_taken_remark=?",TABLE_UPDATE_SET,
        " WHERE med_reconciliation_item_id=?;"
    ].concat()
}

// // ipd-dr-med-reconcile-delete.php
// *** cannot use alias in delete `https://bugs.mysql.com/bug.php?id=82189` ***
// DELETE kphis.ipd_med_reconciliation,kphis.ipd_med_reconciliation_item
// FROM kphis.ipd_med_reconciliation
//     LEFT JOIN kphis.ipd_med_reconciliation_item ON ipd_med_reconciliation.med_reconciliation_id=ipd_med_reconciliation_item.med_reconciliation_id
// WHERE ipd_med_reconciliation.med_reconciliation_id=? AND ipd_med_reconciliation.phamacist_confirm_datetime IS NULL;
/// med_reconciliation_id
pub fn delete_med_reconciliation(kphis: &str) -> String {
    [
        "DELETE ",kphis,".ipd_med_reconciliation,",kphis,".ipd_med_reconciliation_item \
        FROM ",kphis,".ipd_med_reconciliation \
            LEFT JOIN ",kphis,".ipd_med_reconciliation_item ON ipd_med_reconciliation.med_reconciliation_id=ipd_med_reconciliation_item.med_reconciliation_id \
        WHERE ipd_med_reconciliation.med_reconciliation_id=? AND ipd_med_reconciliation.phamacist_confirm_datetime IS NULL;"
    ].concat()
}

// // ipd-dr-med-reconcile-item-delete.php
// DELETE kphis.ipd_med_reconciliation_item FROM kphis.ipd_med_reconciliation_item
// INNER JOIN kphis.ipd_med_reconciliation ON ipd_med_reconciliation.med_reconciliation_id=ipd_med_reconciliation_item.med_reconciliation_id
// WHERE ipd_med_reconciliation_item.med_reconciliation_item_id=? AND ipd_med_reconciliation.phamacist_confirm_datetime IS NULL;
/// med_reconciliation_item_id
pub fn delete_med_reconciliation_item(kphis: &str) -> String {
    [
        "DELETE ",kphis,".ipd_med_reconciliation_item FROM ",kphis,".ipd_med_reconciliation_item \
        INNER JOIN ",kphis,".ipd_med_reconciliation ON ipd_med_reconciliation.med_reconciliation_id=ipd_med_reconciliation_item.med_reconciliation_id \
        WHERE ipd_med_reconciliation_item.med_reconciliation_item_id=? AND ipd_med_reconciliation.phamacist_confirm_datetime IS NULL;"
    ].concat()
}

// // ipd-dr-med-reconcile-from-hosxp.php
// SELECT t2.* FROM hos.medication_reconciliation_detail t2
//     JOIN hos.medication_reconciliation t1 ON t1.medication_reconciliation_id=t2.medication_reconciliation_id
// WHERE t1.an=? ORDER BY medication_reconciliation_detail_id;
/// an
pub fn from_hosxp(hosxp: &str) -> String {
    [
        "SELECT t2.* FROM ",hosxp,".medication_reconciliation_detail t2 \
            JOIN ",hosxp,".medication_reconciliation t1 ON t1.medication_reconciliation_id=t2.medication_reconciliation_id \
        WHERE t1.an=? ORDER BY medication_reconciliation_detail_id;"
    ].concat()
}

// // ipd-dr-med-reconcile-dr-admission-note-last-dose.php
// SELECT last_dose_taken_time,last_dose_taken_remark FROM kphis.ipd_dr_admission_note WHERE an=?;
/// an
pub fn get_last_dose(kphis: &str) -> String {
    [
        "SELECT last_dose_taken_time,last_dose_taken_remark FROM ",kphis,".ipd_dr_admission_note WHERE an=?;"
    ].concat()
}

// // ipd-dr-med-reconcile-note-data.php
// SELECT med_reconciliation_id,phamacist_confirm_datetime,note
//     FROM kphis.ipd_med_reconciliation
// WHERE med_reconciliation_id=?
// ORDER BY med_reconciliation_id DESC;
/// med_reconciliation_id
pub fn get_note(kphis: &str) -> String {
    [
        "SELECT med_reconciliation_id,phamacist_confirm_datetime,note \
            FROM ",kphis,".ipd_med_reconciliation \
        WHERE med_reconciliation_id=?;"
    ].concat()
}

// // ipd-dr-med-reconcile-note-save.php
// UPDATE kphis.ipd_med_reconciliation SET note=?,update_user=?,update_datetime=NOW(),version=(version+1)
// WHERE med_reconciliation_id=? AND phamacist_confirm_datetime IS NULL;
/// note, loginname, med_reconciliation_id
pub fn post_note(kphis: &str) -> String {
    [
        "UPDATE ",kphis,".ipd_med_reconciliation SET note=?",TABLE_UPDATE_SET,
        " WHERE med_reconciliation_id=? AND phamacist_confirm_datetime IS NULL;"
    ].concat()
}

// // ipd-dr-med-reconcile-remed-visit-data.php
// SELECT ovst.vn,ovst.an,ovst.vstdate,ovst.vsttime,
//     COUNT(opd_item.hos_guid) AS opd_item_count,COUNT(ipd_item.hos_guid) AS ipd_home_med_item_count
// FROM hos.ovst
//     LEFT JOIN hos.opitemrece opd_item ON opd_item.vn=ovst.vn AND opd_item.icode LIKE '1%'
//     LEFT JOIN hos.opitemrece ipd_item ON ipd_item.an=ovst.an AND ipd_item.icode LIKE '1%' AND ipd_item.item_type='H'
// WHERE ovst.hn=? GROUP BY vn HAVING opd_item_count > 0 OR ipd_home_med_item_count > 0
// ORDER BY vstdate DESC,vsttime DESC;
//
// // we change to SELECT EXISTS
// SELECT ovst.vn,ovst.an,ovst.vstdate,ovst.vsttime,
//   (SELECT EXISTS(SELECT * FROM hos.opitemrece opd_item WHERE ovst.vn != "" AND opd_item.vn=ovst.vn AND opd_item.icode LIKE '1%')) AS opd_item_exists,
//   (SELECT EXISTS(SELECT * FROM hos.opitemrece ipd_item WHERE ovst.an != "" AND ipd_item.an=ovst.an AND ipd_item.icode LIKE '1%' AND ipd_item.item_type='H')) AS ipd_home_med_item_exists
// FROM hos.ovst
// WHERE ovst.hn=? GROUP BY vn HAVING opd_item_exists OR ipd_home_med_item_exists
// ORDER BY vstdate DESC,vsttime DESC;
/// hn
pub fn get_remed_visit(hosxp: &str) -> String {
    [
        "SELECT ovst.vn,ovst.an,ovst.vstdate,ovst.vsttime,\
            (SELECT EXISTS(SELECT * FROM ",hosxp,".opitemrece opd_item WHERE ovst.vn != '' AND opd_item.vn=ovst.vn AND opd_item.icode LIKE '1%')) AS opd_item_exists,\
            (SELECT EXISTS(SELECT * FROM ",hosxp,".opitemrece ipd_item WHERE ovst.an != '' AND ipd_item.an=ovst.an AND ipd_item.icode LIKE '1%' AND ipd_item.item_type='H')) AS ipd_home_med_item_exists \
        FROM ",hosxp,".ovst \
        WHERE ovst.hn=? GROUP BY vn HAVING opd_item_exists OR ipd_home_med_item_exists \
        ORDER BY vstdate DESC,vsttime DESC;"
    ].concat()
}

// // ipd-dr-med-reconcile-remed-med-data.php
// SELECT o.hos_guid,o.icode,o.item_no,o.item_type,CONCAT(s.NAME,' ',s.strength,' ',s.units) AS item_name,
//     IF(NOT(o.sp_use IS NULL OR TRIM(o.sp_use)=''),
//         CONCAT(IFNULL(u.name1,''),' ',IFNULL(u.name2,''),' ',IFNULL(u.name3,'')),
//         CONCAT(IFNULL(d.name1,''),' ',IFNULL(d.name2,''),' ',IFNULL(d.name3,''))
//     ) AS `usage`,
//     o.qty,d.shortlist,s.displaycolor,u.name1,u.name2,u.name3,
//     o.sum_price,o.unitprice,o.paidst,o.sp_use,o.rxdate
// FROM
//     hos.opitemrece o
//     LEFT JOIN hos.s_drugitems s ON s.icode=o.icode
//     LEFT JOIN hos.drugusage d ON d.drugusage=o.drugusage
//     LEFT JOIN hos.sp_use u ON u.sp_use=o.sp_use
//     LEFT JOIN hos.drugitems i ON i.icode=o.icode
// WHERE o.icode LIKE '1%'
// ORDER BY o.item_no;
/// (vn),(hn)
pub fn get_remed_med(params: &MedReconciliationParams, hosxp: &str) -> String {
    let where_str = match (params.vn.is_some(), params.an.is_some()) {
        (true, true) => " AND (o.vn=? OR (o.item_type='H' AND o.an=?)) ",
        (true, false) => " AND o.vn=? ",
        (false, true) => " AND o.item_type='H' AND o.an=? ",
        (false, false) => "",
    };
    [
        "SELECT o.hos_guid,o.icode,o.item_no,o.item_type,CONCAT(s.NAME,' ',s.strength,' ',s.units) AS item_name,\
            IF(NOT(o.sp_use IS NULL OR TRIM(o.sp_use)=''),\
                CONCAT(IFNULL(u.name1,''),' ',IFNULL(u.name2,''),' ',IFNULL(u.name3,'')),\
                CONCAT(IFNULL(d.name1,''),' ',IFNULL(d.name2,''),' ',IFNULL(d.name3,'')) \
            ) AS `usage`,\
            o.qty,d.shortlist,s.displaycolor,u.name1,u.name2,u.name3,\
            o.sum_price,o.unitprice,o.paidst,o.sp_use,o.rxdate \
        FROM ",hosxp,".opitemrece o \
            LEFT JOIN ",hosxp,".s_drugitems s ON s.icode=o.icode \
            LEFT JOIN ",hosxp,".drugusage d ON d.drugusage=o.drugusage \
            LEFT JOIN ",hosxp,".sp_use u ON u.sp_use=o.sp_use \
            LEFT JOIN ",hosxp,".drugitems i ON i.icode=o.icode \
        WHERE o.icode LIKE '1%' ",where_str,
        "ORDER BY o.item_no;"
    ].concat()
}

// // GET /count/ipd-med-reconcile/:an
// // ipd-dr-med-reconcile-check.php
// SELECT COUNT(*) AS med_reconciliation_count FROM kphis.ipd_med_reconciliation WHERE an=?;
/// an
pub fn get_med_reconcile_exists(kphis: &str) -> String {
    [
        "SELECT EXISTS(SELECT * FROM ",kphis,".ipd_med_reconciliation WHERE an=? AND phamacist_confirm_datetime IS NOT NULL) AS exs;"
    ].concat()
}

// // GET /count/ipd-med-reconcile-dr-unconfirm/:an
// // ipd-dr-med-reconcile-check-doctor-unconfirm.php
// SELECT COUNT(*) AS doctor_unconfirm_med_reconciliation_count
// FROM kphis.ipd_med_reconciliation
// WHERE an=? AND phamacist_confirm_datetime IS NOT NULL AND doctor_confirm_datetime IS NULL;
/// an
pub fn get_med_reconcile_doctor_unconfirm_exists(kphis: &str) -> String {
    [
        "SELECT EXISTS(SELECT * FROM ",kphis,".ipd_med_reconciliation \
        WHERE an=? AND phamacist_confirm_datetime IS NOT NULL AND doctor_confirm_datetime IS NULL) AS exs;"
    ].concat()
}
