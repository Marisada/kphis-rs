use crate::{TABLE_CREATE_COLUMNS, TABLE_CREATE_PREPARED, TABLE_UPDATE_SET};

// // project\function\KphisQueryUtils.php::getCtData($an)
// // project\function\KphisQueryUtils.php::getMriData($an)
// SELECT xi.xray_items_group,xi.xray_items_name,x.examined_date,x.examined_time
// FROM hos.xray_report x
//     LEFT JOIN hos.xray_items xi ON xi.xray_items_code=x.xray_items_code
//     JOIN hos.ipt ON ipt.vn=x.vn
// WHERE ipt.an=? AND xi.xray_items_group IN (3)
// ORDER BY examined_date,examined_time;
/// hos.xray_items_group [X-Ray=1, Ultrasound=2, CT=3, MRI=4, Mammogram=5]<br>
/// an
pub fn select_xray_with_groups(groups: &[i32], hosxp: &str) -> String {
    let grps = groups.iter().map(|i| i.to_string()).collect::<Vec<String>>().join(",");
    [
        "SELECT xi.xray_items_group,xi.xray_items_name,x.examined_date,x.examined_time \
        FROM ",hosxp,".xray_report x \
            LEFT JOIN ",hosxp,".xray_items xi ON xi.xray_items_code=x.xray_items_code \
            JOIN ",hosxp,".ipt ON ipt.vn=x.vn \
        WHERE ipt.an=? AND xi.xray_items_group IN (",&grps,") \
        ORDER BY examined_date,examined_time;"
    ].concat()
}

// // ipd-summary-2-hosxp-ipt-data.php
// SELECT ipt.an,ipt.dchdate,ipt.dchtime,ipt.dchstts,dchstts.`name` AS dchstts_name,ipt.dchtype,dchtype.`name` AS dchtype_name
// FROM hos.ipt
//     LEFT JOIN hos.dchstts ON ipt.dchstts=dchstts.dchstts
//     LEFT JOIN hos.dchtype ON ipt.dchtype=dchtype.dchtype
// WHERE ipt.an=?;
/// an
pub fn select_hosxp_dch(hosxp: &str) -> String {
    [
        "SELECT ipt.an,ipt.dchdate,ipt.dchtime,ipt.dchstts,dchstts.`name` AS dchstts_name,ipt.dchtype,dchtype.`name` AS dchtype_name \
        FROM ",hosxp,".ipt \
            LEFT JOIN ",hosxp,".dchstts ON ipt.dchstts=dchstts.dchstts \
            LEFT JOIN ",hosxp,".dchtype ON ipt.dchtype=dchtype.dchtype \
        WHERE ipt.an=?;"
    ].concat()
}

// // ipd-summary-2-lab-data.php
// // NOTE: MySQL group by without aggregate fn will take only the first not-aggregate one to show (as ORDER BY x DESC LIMIT 1)
// SELECT x.*
// FROM (
//     SELECT h.hn,o.lab_items_code,i.display_order,o.lab_items_name_ref,o.lab_items_normal_value_ref,
//     IF(((SELECT (COUNT(*)=0) FROM hos.lab_items_doctor lid WHERE o.lab_items_code=lid.lab_items_code) AND
//         (SELECT (COUNT(*)=0) FROM hos.lab_items_visible liv WHERE o.lab_items_code=liv.lab_items_code))
//         OR
//         ((SELECT SUM(IF(doctor_code=?,1,0)) > 0 FROM hos.lab_items_doctor lid WHERE o.lab_items_code=lid.lab_items_code) OR
//         (SELECT SUM(IF(groupname=?,1,0)) > 0 FROM hos.lab_items_visible liv WHERE o.lab_items_code=liv.lab_items_code))
//     ,o.lab_order_result,'[[ปกปิด]]') AS lab_order_result,
//     i.lab_items_unit,i.range_check_min,i.range_check_max,o.staff_lock_result,o.lab_order_remark,i.lab_items_group,ig.lab_items_group_name,
//     h.lab_order_number,h.vn,h.order_date,h.order_time,h.receive_date,h.receive_time,
//     IF(h.report_date='1899-12-30',NULL,h.report_date) AS report_date,IF(h.report_date='1899-12-30',NULL,h.report_time) AS report_time
// FROM hos.lab_head h
//     INNER JOIN hos.lab_order o ON h.lab_order_number=o.lab_order_number
//     INNER JOIN hos.lab_items i ON o.lab_items_code=i.lab_items_code
//     LEFT JOIN hos.lab_items_group ig ON i.lab_items_group=ig.lab_items_group_code
// WHERE o.confirm = 'Y' AND (o.lab_order_result IS NOT NULL AND TRIM(o.lab_order_result) NOT IN ('','-'))
//     AND (
//         (o.lab_items_code = 4 AND o.lab_order_result < 30) -- Hct
//         or (o.lab_items_code = 10 and o.lab_order_result < 100) -- Plt Count (10^3/uL)
//         or (o.lab_items_code = 458 and o.lab_order_result > 1.2) -- INR
//         or (o.lab_items_code = 266 and o.lab_order_result > 34) -- PTT
//         or (o.lab_items_code in (78,983,987) and o.lab_order_result > 1.1) -- Cr
//         or (o.lab_items_code = 80 and (o.lab_order_result < 135 or o.lab_order_result > 150)) -- Na
//         or (o.lab_items_code = 81 and (o.lab_order_result < 3.5 or o.lab_order_result > 5)) -- K
//         or (o.lab_items_code = 82 and o.lab_order_result < 15) -- CO2
//         or (o.lab_items_code = 95 and o.lab_order_result < 3.5) -- Albumin
//         or (o.lab_items_code = 97 and o.lab_order_result > 1.2) -- Total Bilirubin
//         or (o.lab_items_code = 99 and o.lab_order_result > 40) -- AST
//         or (o.lab_items_code = 100 and o.lab_order_result > 40) -- ALT
//         or (o.lab_items_code = 101 and o.lab_order_result > 90) -- ALP
//         or (o.lab_items_code = 79 and o.lab_order_result > 8) -- Uric Acid
//         or (o.lab_items_code = 303 and o.lab_order_result < 1.9) -- Magnesium
//         or (o.lab_items_code = 302 and (o.lab_order_result < 8.8 or o.lab_order_result > 10.6)) -- Calcium
//         or (o.lab_items_code = 304 and (o.lab_order_result < 2.5 or o.lab_order_result > 4.5)) -- Phosphorus
//         or (o.lab_items_code = 53 and (o.lab_order_result is not null
//             and o.lab_order_result not in ('','-','UA น้อย ไม่พอตรวจ','0-1','1-2','2-3')) ) -- UA WBC > 3
//         or (o.lab_items_code = 52 and (o.lab_order_result is not null
//             and o.lab_order_result not in ('','-','UA น้อย ไม่พอตรวจ','0-1','1-2','2-3')) ) -- UA RBC > 3
//     )
//     AND h.vn=?
// ORDER BY IF(h.report_date='1899-12-30',h.order_date,h.report_date) DESC,
//     IF(h.report_date='1899-12-30',h.order_time,h.report_time) DESC,
//     h.order_date DESC,h.order_time DESC
// ) x
// GROUP BY x.lab_items_group_name,x.lab_items_code
// ORDER BY x.lab_items_group_name,x.display_order;
/// doctorcode, groupname, an
pub fn select_lab_alert(alerts: &[(String, Vec<u64>, String)], hosxp: &str) -> String {
    let labs = alerts.iter().map(|(_, codes, where_at)| {
        let code_comma = codes.iter().map(|u| u.to_string()).collect::<Vec<String>>().join(",");
        let replaced = where_at.replace('@', "o.lab_order_result");
        ["(o.lab_items_code IN (",&code_comma,") AND ",&replaced,")"].concat()
    }).collect::<Vec<String>>().join(" OR ");

    [
        "SELECT x.* \
        FROM ( \
            SELECT h.hn,o.lab_items_code,i.display_order,o.lab_items_name_ref,o.lab_items_normal_value_ref,\
            IF(((SELECT NOT EXISTS(SELECT * FROM ",hosxp,".lab_items_doctor lid WHERE o.lab_items_code=lid.lab_items_code)) \
                AND (SELECT NOT EXISTS(SELECT * FROM ",hosxp,".lab_items_visible liv WHERE o.lab_items_code=liv.lab_items_code))\
            ) OR ((SELECT SUM(IF(lid.doctor_code=?,1,0)) > 0 FROM ",hosxp,".lab_items_doctor lid WHERE o.lab_items_code=lid.lab_items_code) \
                OR (SELECT SUM(IF(liv.groupname=?,1,0)) > 0 FROM ",hosxp,".lab_items_visible liv WHERE o.lab_items_code=liv.lab_items_code)\
            ),o.lab_order_result,'[[ปกปิด]]') AS lab_order_result,\
            i.lab_items_unit,i.range_check_min,i.range_check_max,o.staff_lock_result,o.lab_order_remark,i.lab_items_group,ig.lab_items_group_name,\
            h.lab_order_number,h.vn,h.order_date,h.order_time,h.receive_date,h.receive_time,\
            IF(h.report_date='1899-12-30',NULL,h.report_date) AS report_date,IF(h.report_date='1899-12-30',NULL,h.report_time) AS report_time \
        FROM ",hosxp,".lab_head h \
            INNER JOIN ",hosxp,".lab_order o ON h.lab_order_number=o.lab_order_number \
            INNER JOIN ",hosxp,".lab_items i ON o.lab_items_code=i.lab_items_code \
            LEFT JOIN ",hosxp,".lab_items_group ig ON i.lab_items_group=ig.lab_items_group_code \
        WHERE o.confirm = 'Y' AND (o.lab_order_result IS NOT NULL AND TRIM(o.lab_order_result) NOT IN ('','-')) \
            AND (", &labs, ") AND h.vn=? \
        ORDER BY IF(h.report_date='1899-12-30',h.order_date,h.report_date) DESC,\
            IF(h.report_date='1899-12-30',h.order_time,h.report_time) DESC,\
            h.order_date DESC,h.order_time DESC \
        ) x \
        GROUP BY x.lab_items_group_name,x.lab_items_code \
        ORDER BY x.lab_items_group_name,x.display_order;"
    ].concat()
}

// // ipd-summary-2-problem-list-data.php
// SELECT DISTINCT pni.progress_note_item_detail
// FROM kphis.ipd_progress_note_item pni
//     JOIN kphis.ipd_progress_note pn ON pn.progress_note_id=pni.progress_note_id
// WHERE pni.progress_note_item_type='problem-list' AND pn.an=?
// ORDER BY pni.progress_note_id;
/// an
pub fn select_problem_list(kphis: &str) -> String {
    [
        "SELECT DISTINCT pni.progress_note_item_detail \
        FROM ",kphis,".ipd_progress_note_item pni \
            JOIN ",kphis,".ipd_progress_note pn ON pn.progress_note_id=pni.progress_note_id \
        WHERE pni.progress_note_item_type='problem-list' AND pn.an=? \
        ORDER BY pni.progress_note_id;"
    ].concat()
}

// // ipd-summary-2-data.php
// SELECT summary.*,h.hosptype,h.NAME AS hospname
// FROM kphis.ipd_summary_2 AS summary
//     LEFT JOIN hos.hospcode h ON h.hospcode=summary.hospital_refer
// WHERE summary.summary_id=?
// -- WHERE summary.an=?
// ORDER BY summary.summary_id DESC LIMIT 1;
/// summary_id | an
pub fn select_summary2_by(by_id: bool, hosxp: &str, kphis: &str) -> String {
    let by = if by_id { "summary_id=?" } else { "an=?" };
    [
        "SELECT summary.*,h.hosptype,h.NAME AS hospname \
        FROM ",kphis,".ipd_summary_2 AS summary \
            LEFT JOIN ",hosxp,".hospcode h ON h.hospcode=summary.hospital_refer \
        WHERE summary.",by," ORDER BY summary.summary_id DESC LIMIT 1;"
    ].concat()
}

// SELECT status FROM kphis.ipd_summary_2 WHERE summary_id=?;
/// summary_id
pub fn select_summary2_status(kphis: &str) -> String {
    ["SELECT status FROM ",kphis,".ipd_summary_2 WHERE summary_id=?;"].concat()
}

// // from 4 queries
// SELECT t1.* FROM kphis.ipd_summary_pre_admission_comorbidity t1
//     WHERE summary_id = :summary_id ORDER BY pre_admission_comorbidity_id;
// SELECT t1.* FROM kphis.ipd_summary_post_admission_comorbidity t1
//     WHERE summary_id = :summary_id ORDER BY post_admission_comorbidity_id;
// SELECT t1.* FROM kphis.ipd_summary_other_diagnosis t1
//     WHERE summary_id = :summary_id ORDER BY other_diagnosis_id;
// SELECT t1.* FROM kphis.ipd_summary_external_cause t1
//     WHERE summary_id = :summary_id ORDER BY external_cause_id;
// // we union 4 query and group at client by 'ty'
// (SELECT 2 AS ty, pre_admission_comorbidity_detail AS detail, pre_admission_comorbidity_icd10 AS icd10
//     FROM kphis.ipd_summary_pre_admission_comorbidity
//     WHERE summary_id=? ORDER BY pre_admission_comorbidity_id)
// UNION
//     (SELECT 3 AS ty, post_admission_comorbidity_detail AS detail, post_admission_comorbidity_icd10 AS icd10
//     FROM kphis.ipd_summary_post_admission_comorbidity
//     WHERE summary_id=? ORDER BY post_admission_comorbidity_id)
// UNION
//     (SELECT 4 AS ty, other_diagnosis_detail AS detail, other_diagnosis_icd10 AS icd10
//     FROM kphis.ipd_summary_other_diagnosis
//     WHERE summary_id=? ORDER BY other_diagnosis_id)
// UNION
//     (SELECT 5 AS ty, external_cause_detail AS detail,external_cause_icd10 AS icd10
//     FROM kphis.ipd_summary_external_cause
//     WHERE summary_id=? ORDER BY external_cause_id)
/// summary_id, summary_id, summary_id, summary_id
pub fn select_dx2_5(kphis: &str) -> String {
    [
        "(SELECT 2 AS ty, pre_admission_comorbidity_detail AS detail, pre_admission_comorbidity_icd10 AS icd \
            FROM ",kphis,".ipd_summary_pre_admission_comorbidity \
            WHERE summary_id=? ORDER BY pre_admission_comorbidity_id) \
        UNION \
            (SELECT 3 AS ty, post_admission_comorbidity_detail AS detail, post_admission_comorbidity_icd10 AS icd \
            FROM ",kphis,".ipd_summary_post_admission_comorbidity \
            WHERE summary_id=? ORDER BY post_admission_comorbidity_id) \
        UNION \
            (SELECT 4 AS ty, other_diagnosis_detail AS detail, other_diagnosis_icd10 AS icd \
            FROM ",kphis,".ipd_summary_other_diagnosis \
            WHERE summary_id=? ORDER BY other_diagnosis_id) \
        UNION \
            (SELECT 5 AS ty, external_cause_detail AS detail,external_cause_icd10 AS icd \
            FROM ",kphis,".ipd_summary_external_cause \
            WHERE summary_id=? ORDER BY external_cause_id)"
    ].concat()
}

// // from 2 queries
// SELECT t1.*, doctor.name as doctor_name
//     FROM kphis.ipd_summary_attending_doctor t1
//         LEFT JOIN hos.doctor on doctor.code = t1.summary_attending_doctor
//     WHERE t1.summary_id = :summary_id ORDER BY t1.create_datetime
// SELECT t1.*, doctor.name as doctor_name
//     FROM kphis.ipd_summary_approve_doctor t1
//         LEFT JOIN hos.doctor on doctor.code = t1.summary_approve_doctor
//     WHERE t1.summary_id = :summary_id ORDER BY t1.create_datetime
// // we union 2 query and group at client by 'ty'
// (SELECT 1 AS ty,summary_attending_doctor AS doctor,doctor.name AS doctor_name
//     FROM kphis.ipd_summary_attending_doctor
//         LEFT JOIN hos.doctor ON doctor.code=summary_attending_doctor
//     WHERE summary_id=? ORDER BY create_datetime)
// UNION
//     (SELECT 2 AS ty,summary_approve_doctor AS doctor,doctor.name AS doctor_name
//     FROM kphis.ipd_summary_approve_doctor
//         LEFT JOIN hos.doctor ON doctor.code=summary_approve_doctor
//     WHERE summary_id=? ORDER BY create_datetime)
/// summary_id, summary_id
pub fn select_attend_and_approve_doctor(hosxp: &str, kphis: &str) -> String {
    [
        "(SELECT 1 AS ty,summary_attending_doctor AS doctor,doctor.name AS doctor_name,doctor.licenseno \
            FROM ",kphis,".ipd_summary_attending_doctor \
                LEFT JOIN ",hosxp,".doctor ON doctor.code=summary_attending_doctor \
            WHERE summary_id=? ORDER BY create_datetime) \
        UNION \
            (SELECT 2 AS ty,summary_approve_doctor AS doctor,doctor.name AS doctor_name,doctor.licenseno \
            FROM ",kphis,".ipd_summary_approve_doctor \
                LEFT JOIN ",hosxp,".doctor ON doctor.code=summary_approve_doctor \
            WHERE summary_id=? ORDER BY create_datetime)"
    ].concat()
}

// // ipd-summary-2-save.php
// INSERT INTO kphis.ipd_summary_2 (principal_diagnosis,principal_diagnosis_icd10,
//     operating_room,tracheostomy,mechanical_ventilation,packed_redcells,fresh_frozen_plasma,platelets,cryoprecipitate,whole_blood,
//     computer_tomography,computer_tomography_text,chemotherapy,mri,mri_text,
//     hemodialysis,non_or_other,non_or_other_text,special_other,special_other_text,discharge_status,discharge_type,hospital_refer,status,an,
//     create_user,create_datetime,update_user,update_datetime,version)
// VALUES (?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,NOW(),?,NOW(),1);
/// doctor(principal_diagnosis, principal_diagnosis_icd10), operating_room, tracheostomy, mechanical_ventilation, packed_redcells, fresh_frozen_plasma, platelets,
/// cryoprecipitate, whole_blood, computer_tomography, computer_tomography_text, chemotherapy, mri, mri_text, hemodialysis, non_or_other, non_or_other_text,
/// special_other, special_other_text, discharge_status, discharge_type, hospital_refer, status, an, loginname, loginname
pub fn insert_summary2(is_doctor: bool, kphis: &str) -> String {
    let (into, value) = if is_doctor {
        ("principal_diagnosis,principal_diagnosis_icd10,", "?,?,")
    } else {
        ("", "")
    };
    [
        "INSERT INTO ",kphis,".ipd_summary_2 (",into,
            "operating_room,tracheostomy,mechanical_ventilation,packed_redcells,fresh_frozen_plasma,platelets,cryoprecipitate,whole_blood,\
            computer_tomography,computer_tomography_text,chemotherapy,mri,mri_text,\
            hemodialysis,non_or_other,non_or_other_text,special_other,special_other_text,discharge_status,discharge_type,hospital_refer,status,an",
        TABLE_CREATE_COLUMNS,") VALUES (",value,"?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?",TABLE_CREATE_PREPARED,");"
    ].concat()
}

// UPDATE kphis.ipd_summary_2 SET principal_diagnosis=?,principal_diagnosis_icd10=?,
//     operating_room=?,tracheostomy=?,mechanical_ventilation=?,packed_redcells=?,fresh_frozen_plasma=?,platelets=?,cryoprecipitate=?,whole_blood=?,
//     computer_tomography=?,computer_tomography_text=?,chemotherapy=?,mri=?,mri_text=?,hemodialysis=?,non_or_other=?,non_or_other_text=?,
//     discharge_status=?,discharge_type=?,hospital_refer=?,update_user=?,update_datetime=NOW()
// WHERE summary_id=? AND an=?;
/// doctor(principal_diagnosis, principal_diagnosis_icd10), operating_room, tracheostomy, mechanical_ventilation, packed_redcells, fresh_frozen_plasma, platelets,
/// cryoprecipitate, whole_blood, computer_tomography, computer_tomography_text, chemotherapy, mri, mri_text, hemodialysis, non_or_other, non_or_other_text,
/// special_other, special_other_text, discharge_status, discharge_type, hospital_refer, status, loginname, summary_id, an
pub fn update_summary2(is_doctor: bool, kphis: &str) -> String {
    let set = if is_doctor {"principal_diagnosis=?,principal_diagnosis_icd10=?,"} else {""};
    [
        "UPDATE ",kphis,".ipd_summary_2 SET ",set,
            "operating_room=?,tracheostomy=?,mechanical_ventilation=?,packed_redcells=?,fresh_frozen_plasma=?,platelets=?,cryoprecipitate=?,whole_blood=?,\
            computer_tomography=?,computer_tomography_text=?,chemotherapy=?,mri=?,mri_text=?,hemodialysis=?,non_or_other=?,non_or_other_text=?,\
            special_other=?,special_other_text=?,discharge_status=?,discharge_type=?,hospital_refer=?,status=?",TABLE_UPDATE_SET,
        " WHERE summary_id=? AND an=? AND (status IS NULL OR status NOT IN ('claim','done'));"
    ].concat()
}

// UPDATE kphis.ipd_summary_2 SET coder_name=?,principal_diagnosis_code=?,pre_admission_comorbidity_codes=?,post_admission_comorbidity_codes=?,
//     other_diagnosis_codes=?,external_cause_codes=?,main_procedure_code=?,other_procedure_codes=?,update_user=?,update_datetime=NOW()
// WHERE summary_id=? AND an=?;
/// coder_name, principal_diagnosis_code, pre_admission_comorbidity_codes, post_admission_comorbidity_codes, other_diagnosis_codes, external_cause_codes,
/// main_procedure_code, other_procedure_codes, status, loginname, summary_id, an
pub fn update_summary2_code(kphis: &str) -> String {
    [
        "UPDATE ",kphis,".ipd_summary_2 SET coder_name=?,principal_diagnosis_code=?,pre_admission_comorbidity_codes=?,post_admission_comorbidity_codes=?,\
            other_diagnosis_codes=?,external_cause_codes=?,main_procedure_code=?,other_procedure_codes=?,status=?",TABLE_UPDATE_SET,
        " WHERE summary_id=? AND an=?;"
    ].concat()
}

// UPDATE kphis.ipd_summary_2 SET status=?,update_user=?,update_datetime=NOW() WHERE summary_id=?;
/// status, loginname, summary_id
pub fn update_summary2_status(kphis: &str) -> String {
    ["UPDATE ",kphis,".ipd_summary_2 SET status=?",TABLE_UPDATE_SET," WHERE summary_id=?;"].concat()
}

// DELETE FROM kphis.ipd_summary_pre_admission_comorbidity WHERE summary_id=?;
/// summary_id
pub fn delete_pre_admission_comorbidity(kphis: &str) -> String {
    [
        "DELETE FROM ",kphis,".ipd_summary_pre_admission_comorbidity WHERE summary_id=?;"
    ].concat()
}

// DELETE FROM kphis.ipd_summary_post_admission_comorbidity WHERE summary_id=?;
/// summary_id
pub fn delete_post_admission_comorbidity(kphis: &str) -> String {
    [
        "DELETE FROM ",kphis,".ipd_summary_post_admission_comorbidity WHERE summary_id=?;"
    ].concat()
}

// DELETE FROM kphis.ipd_summary_other_diagnosis WHERE summary_id=?;
/// summary_id
pub fn delete_other_diagnosis(kphis: &str) -> String {
    [
        "DELETE FROM ",kphis,".ipd_summary_other_diagnosis WHERE summary_id=?;"
    ].concat()
}

// DELETE FROM kphis.ipd_summary_external_cause WHERE summary_id=?;
/// summary_id
pub fn delete_external_cause(kphis: &str) -> String {
    [
        "DELETE FROM ",kphis,".ipd_summary_external_cause WHERE summary_id=?;"
    ].concat()
}

// INSERT INTO kphis.ipd_summary_attending_doctor (summary_id,summary_attending_doctor,
//     create_user,create_datetime,update_user,update_datetime,version)
// VALUES (?,?,?,NOW(),?,NOW(),1);
/// summary_id, doctorcode, loginname, loginname
pub fn insert_attending_doctor(kphis: &str) -> String {
    [
        "INSERT INTO ",kphis,".ipd_summary_attending_doctor (summary_id,summary_attending_doctor",
        TABLE_CREATE_COLUMNS,") VALUES (?,?",TABLE_CREATE_PREPARED,");"
    ].concat()
}

// INSERT INTO kphis.ipd_summary_approve_doctor (summary_id,summary_approve_doctor,
//     create_user,create_datetime,update_user,update_datetime,version)
// VALUES (?,?,?,NOW(),?,NOW(),1);
/// summary_id, doctorcode, loginname, loginname
pub fn insert_approve_doctor(kphis: &str) -> String {
    [
        "INSERT INTO ",kphis,".ipd_summary_approve_doctor (summary_id,summary_approve_doctor",
        TABLE_CREATE_COLUMNS,") VALUES (?,?",TABLE_CREATE_PREPARED,");"
    ].concat()
}

// INSERT INTO kphis.ipd_summary_pre_admission_comorbidity (
//     summary_id,pre_admission_comorbidity_detail,pre_admission_comorbidity_icd10,create_user,create_datetime,update_user,update_datetime,version)
// VALUES (?,?,?,?,NOW(),?,NOW(),1);
/// summary_id, pre_admission_comorbidity_detail, pre_admission_comorbidity_icd10, loginname, loginname
pub fn insert_pre_admission_comorbidity(count: usize, kphis: &str) -> String {
    let values = vec!["(?,?,?,?,NOW(),?,NOW(),1)"; count].join(",");
    [
        "INSERT INTO ",kphis,".ipd_summary_pre_admission_comorbidity (\
            summary_id,pre_admission_comorbidity_detail,pre_admission_comorbidity_icd10",TABLE_CREATE_COLUMNS,") \
        VALUES ", &values
    ].concat()
}

// INSERT INTO kphis.ipd_summary_post_admission_comorbidity (
//     summary_id,post_admission_comorbidity_detail,post_admission_comorbidity_icd10,create_user,create_datetime,update_user,update_datetime,version)
// VALUES (?,?,?,?,NOW(),?,NOW(),1);
/// summary_id, post_admission_comorbidity_detail, post_admission_comorbidity_icd10, loginname, loginname
pub fn insert_post_admission_comorbidity(count: usize, kphis: &str) -> String {
    let values = vec!["(?,?,?,?,NOW(),?,NOW(),1)"; count].join(",");
    [
        "INSERT INTO ",kphis,".ipd_summary_post_admission_comorbidity (\
            summary_id,post_admission_comorbidity_detail,post_admission_comorbidity_icd10",TABLE_CREATE_COLUMNS,") \
        VALUES ", &values
    ].concat()
}

// INSERT INTO kphis.ipd_summary_other_diagnosis (
//     summary_id,other_diagnosis_detail,other_diagnosis_icd10,create_user,create_datetime,update_user,update_datetime,version)
// VALUES (?,?,?,?,NOW(),?,NOW(),1);
/// summary_id, other_diagnosis_detail, other_diagnosis_icd10, loginname, loginname
pub fn insert_other_diagnosis(count: usize, kphis: &str) -> String {
    let values = vec!["(?,?,?,?,NOW(),?,NOW(),1)"; count].join(",");
    [
        "INSERT INTO ",kphis,".ipd_summary_other_diagnosis (\
            summary_id,other_diagnosis_detail,other_diagnosis_icd10",TABLE_CREATE_COLUMNS,") \
        VALUES ", &values
    ].concat()
}

// INSERT INTO kphis.ipd_summary_external_cause (
//     summary_id,external_cause_detail,external_cause_icd10,create_user,create_datetime,update_user,update_datetime,version)
// VALUES (?,?,?,?,NOW(),?,NOW(),1);
/// summary_id, external_cause_detail, external_cause_icd10, loginname, loginname
pub fn insert_external_cause(count: usize, kphis: &str) -> String {
    let values = vec!["(?,?,?,?,NOW(),?,NOW(),1)"; count].join(",");
    [
        "INSERT INTO ",kphis,".ipd_summary_external_cause (\
            summary_id,external_cause_detail,external_cause_icd10",TABLE_CREATE_COLUMNS,") \
        VALUES ", &values
    ].concat()
}

// // ipd-summary-2-pdf.php
// SELECT ipt.an,ipt.hn AS ipt_hn,patient.pname,patient.fname,patient.lname,patient.drugallergy,patient.cid,patient.passport_no,
//     patient.birthday,patient.informaddr,patient.informname,patient.informrelation,patient.informtel,patient.hometel,
//     timestampdiff(year,patient.birthday,ipt.regdate) AS age_y,
//     timestampdiff(month,patient.birthday,ipt.regdate)-(timestampdiff(year,patient.birthday,ipt.regdate)*12) AS age_m,
//     timestampdiff(day,date_add(patient.birthday,interval (timestampdiff(month,patient.birthday,ipt.regdate)) month),ipt.regdate) AS age_d,
//     an_stat.admdate,ipt.regdate,ipt.regtime,ipt.ward,ipt.dchdate,ipt.dchtime,ipt.pttype,ipt.gravidity,ipt.parity,ipt.living_children,
//     pttype.`name` AS pttype_name,ward.name ward_name, ward.shortname, spclty.`name` AS spclty_name,
//     occupation.`name` AS occupation_name,sex.`name` AS sex_name,religion.`name` AS religion_name,
//     nationality.`name` AS citizenship_name,nationality2.`name` AS nationality_name,marrystatus.`name` AS marrystatus_name,
//     opduser.name AS user_create_user,opduser.entryposition,doctor.licenseno,ipt_newborn.birth_weight,ipt.bw,
//     CONCAT(h.hosptype,' ',h.NAME) AS hospital_refer_name,ipd_summary_2.*
// FROM hos.ipt
//     LEFT JOIN hos.an_stat ON an_stat.an=ipt.an
//     LEFT JOIN hos.patient ON patient.hn=ipt.hn
//     LEFT JOIN hos.ward ON ward.ward=ipt.ward
//     LEFT JOIN hos.pttype ON pttype.pttype=ipt.pttype
//     LEFT JOIN hos.occupation ON occupation.occupation=patient.occupation
//     LEFT JOIN hos.sex ON sex.code=patient.sex
//     LEFT JOIN hos.religion ON religion.religion=patient.religion
//     LEFT JOIN hos.nationality ON nationality.nationality=patient.citizenship
//     LEFT JOIN hos.nationality AS nationality2 ON nationality2.nationality=patient.nationality
//     LEFT JOIN hos.marrystatus ON marrystatus.code=patient.marrystatus
//     LEFT JOIN hos.spclty ON spclty.spclty=an_stat.spclty
//     LEFT JOIN kphis.ipd_summary_2 ON ipd_summary_2.an=an_stat.an
//     LEFT JOIN hos.opduser ON opduser.loginname=ipd_summary_2.create_user
//     LEFT JOIN hos.doctor ON doctor.code=opduser.doctorcode AND doctor.licenseno !='-99999'
//     LEFT JOIN hos.ipt_newborn ON ipt_newborn.an=ipd_summary_2.an
//     LEFT JOIN hos.hospcode h ON h.hospcode=ipd_summary_2.hospital_refer
// WHERE ipt.an=? ORDER BY summary_id DESC;
// /// summary_id | an
// pub fn select_summary_report_by(by_id: bool, hosxp: &str, kphis: &str) -> String {
//     let by = if by_id {"ipd_summary_2.summary_id=?"} else {"ipt.an=?"};
//     [
//         "SELECT ipt.hn,patient.pname,patient.fname,patient.lname,patient.drugallergy,patient.cid,patient.passport_no,\
//             patient.birthday,patient.informaddr,patient.informname,patient.informrelation,patient.informtel,patient.hometel,\
//             timestampdiff(year,patient.birthday,ipt.regdate) AS age_y,\
//             timestampdiff(month,patient.birthday,ipt.regdate)-(timestampdiff(year,patient.birthday,ipt.regdate)*12) AS age_m,\
//             timestampdiff(day,date_add(patient.birthday,interval (timestampdiff(month,patient.birthday,ipt.regdate)) month),ipt.regdate) AS age_d,\
//             an_stat.admdate,ipt.regdate,ipt.regtime,ipt.ward,ipt.dchdate,ipt.dchtime,ipt.pttype,ipt.gravidity,ipt.parity,ipt.living_children,\
//             pttype.`name` AS pttype_name,ward.name ward_name, ward.shortname, spclty.`name` AS spclty_name,\
//             occupation.`name` AS occupation_name,sex.`name` AS sex_name,religion.`name` AS religion_name,\
//             nationality.`name` AS citizenship_name,nationality2.`name` AS nationality_name,marrystatus.`name` AS marrystatus_name,\
//             opduser.name AS user_create_user,opduser.entryposition,doctor.licenseno,ipt_newborn.birth_weight,ipt.bw,\
//             CONCAT(h.hosptype,' ',h.NAME) AS hospital_refer_name,ipd_summary_2.* \
//         FROM ",hosxp,".ipt \
//             LEFT JOIN ",hosxp,".an_stat ON an_stat.an=ipt.an \
//             LEFT JOIN ",hosxp,".patient ON patient.hn=ipt.hn \
//             LEFT JOIN ",hosxp,".ward ON ward.ward=ipt.ward \
//             LEFT JOIN ",hosxp,".pttype ON pttype.pttype=ipt.pttype \
//             LEFT JOIN ",hosxp,".occupation ON occupation.occupation=patient.occupation \
//             LEFT JOIN ",hosxp,".sex ON sex.code=patient.sex \
//             LEFT JOIN ",hosxp,".religion ON religion.religion=patient.religion \
//             LEFT JOIN ",hosxp,".nationality ON nationality.nationality=patient.citizenship \
//             LEFT JOIN ",hosxp,".nationality AS nationality2 ON nationality2.nationality=patient.nationality \
//             LEFT JOIN ",hosxp,".marrystatus ON marrystatus.code=patient.marrystatus \
//             LEFT JOIN ",hosxp,".spclty ON spclty.spclty=an_stat.spclty \
//             LEFT JOIN ",kphis,".ipd_summary_2 ON ipd_summary_2.an=an_stat.an \
//             LEFT JOIN ",hosxp,".opduser ON opduser.loginname=ipd_summary_2.create_user \
//             LEFT JOIN ",hosxp,".doctor ON doctor.code=opduser.doctorcode AND doctor.licenseno !='-99999' \
//             LEFT JOIN ",hosxp,".ipt_newborn ON ipt_newborn.an=ipd_summary_2.an \
//             LEFT JOIN ",hosxp,".hospcode h ON h.hospcode=ipd_summary_2.hospital_refer \
//         WHERE ",by ," ORDER BY summary_id DESC;"
//     )
// }

// SELECT note.*,doctor.name AS doctor_name
// FROM kphis.ipd_summary_note AS note
//     LEFT JOIN hosxp.doctor ON doctor.code=note.doctor
// WHERE note.summary_id=? ORDER BY note.summary_note_id DESC;
/// summary_id
pub fn select_summary_note(hosxp: &str, kphis: &str) -> String {
    [
        "SELECT note.*,doctor.name AS doctor_name \
        FROM ",kphis,".ipd_summary_note AS note \
            LEFT JOIN ",hosxp,".doctor ON doctor.code=note.doctor \
        WHERE note.summary_id=? ORDER BY note.summary_note_id;"
    ].concat()
}

// INSERT INTO kphis.ipd_summary_note (summary_id,note,doctor,create_user,create_datetime,update_user,update_datetime,version)
// VALUES (?,?,?,?,NOW(),?,NOW(),1);
/// summary_id, note, doctorcode, loginname, loginname
pub fn insert_summary_note(kphis: &str) -> String {
    [
        "INSERT INTO ",kphis,".ipd_summary_note (summary_id,note,doctor",
        TABLE_CREATE_COLUMNS,") VALUES (?,?,?",TABLE_CREATE_PREPARED,");"
    ].concat()
}

// UPDATE kphis.ipd_summary_note SET note=?,update_user=?,update_datetime=NOW()
// WHERE summary_note_id=? AND summary_id=? AND doctor=?;
/// note, loginname, summary_note_id, summary_id, doctor_code
pub fn update_summary_note(kphis: &str) -> String {
    [
        "UPDATE ",kphis,".ipd_summary_note SET note=?", TABLE_UPDATE_SET,
        " WHERE summary_note_id=? AND summary_id=? AND doctor=?;"
    ].concat()
}

// DELETE FROM kphis.ipd_summary_note WHERE summary_note_id=? AND summary_id=? AND doctor=?;
/// summary_note_id, summary_id, doctor_code
pub fn delete_summary_note(kphis: &str) -> String {
    [
        "DELETE FROM ",kphis,".ipd_summary_note WHERE summary_note_id=? AND summary_id=? AND doctor=?;",
    ].concat()
}