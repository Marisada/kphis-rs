use kphis_model::score::CONCAT_SQL;

use crate::{TABLE_CREATE_COLUMNS, TABLE_CREATE_PREPARED, TABLE_UPDATE_SET};

// // opd-er-medical-history-data.php
// SELECT os.vstdate,os.vsttime,os.bw,os.height,os.pain_score,IF(os.bpd=0,NULL,os.bpd) AS bpd,IF(os.bps=0,NULL,os.bps) AS bps,
//     IF(os.pulse=0,NULL,os.pulse) AS pulse,IF(os.rr=0,NULL,os.rr) AS rr,IF(os.temperature=0,NULL,os.temperature) AS temperature,
//     os.cc,os.hpi,os.pe_ga_text,os.pe_heent,os.pe_heent_text,os.pe_heart,os.pe_heart_text,os.pe_lung,os.pe_lung_text,
//     os.pe_ab,os.pe_ab_text,os.pe_ext,os.pe_ext_text,os.pe_neuro,os.pe_neuro_text,os.pe,pt.`name` AS er_pt_type_name,
//     er_et.name AS er_emergency_type_name,er_el.er_emergency_level_name,er_ed.gcs_e,er_ed.gcs_m, er_ed.gcs_v,spclty.`name` AS er_spclty_name,
//     kphis.score_total(kphis.score_bt(vn_stat.age_y,IF(os.temperature=0,NULL,os.temperature)),kphis.score_pr(vn_stat.age_y,IF(os.pulse=0,NULL,os.pulse)),
//         kphis.score_rr(vn_stat.age_y,IF(os.rr=0,NULL,os.rr),NULL),kphis.score_sbp(vn_stat.age_y,IF(os.bps=0,NULL,os.bps),NULL),0,0) AS mews
// FROM hos.opdscreen os
//     LEFT JOIN hos.er_regist er_r ON os.vn=er_r.vn
//     LEFT JOIN hos.er_pt_type pt ON er_r.er_pt_type=pt.er_pt_type
//     LEFT JOIN hos.er_emergency_type er_et ON er_r.er_emergency_type=er_et.er_emergency_type
//     LEFT JOIN hos.er_emergency_level er_el ON er_r.er_emergency_level_id=er_el.er_emergency_level_id
//     LEFT JOIN hos.er_nursing_detail er_ed ON os.vn=er_ed.vn
//     LEFT JOIN hos.vn_stat ON vn_stat.vn=os.vn
//     LEFT JOIN hos.ovst ON ovst.vn=os.vn
//     LEFT JOIN hos.spclty ON spclty.spclty=ovst.spclty
// WHERE os.vn=?;
/// vn
// pub fn get_opdscreens(hosxp: &str, kphis: &str) -> String {
//     [
//         "SELECT os.vstdate,os.vsttime,os.bw,os.height,os.pain_score,IF(os.bpd=0,NULL,os.bpd) AS bpd,IF(os.bps=0,NULL,os.bps) AS bps,",
//             "IF(os.pulse=0,NULL,os.pulse) AS pulse,IF(os.rr=0,NULL,os.rr) AS rr,IF(os.temperature=0,NULL,os.temperature) AS temperature,",
//             "os.cc,os.hpi,os.pe_ga_text,os.pe_heent,os.pe_heent_text,os.pe_heart,os.pe_heart_text,os.pe_lung,os.pe_lung_text,",
//             "os.pe_ab,os.pe_ab_text,os.pe_ext,os.pe_ext_text,os.pe_neuro,os.pe_neuro_text,os.pe,pt.`name` AS er_pt_type_name,",
//             "er_et.name AS er_emergency_type_name,er_el.er_emergency_level_name,er_ed.gcs_e,er_ed.gcs_m, er_ed.gcs_v,spclty.`name` AS er_spclty_name,",
//             kphis,".score_total(",kphis,".score_bt(vn_stat.age_y,IF(os.temperature=0,NULL,os.temperature)),",kphis,".score_pr(vn_stat.age_y,IF(os.pulse=0,NULL,os.pulse)),",
//             kphis,".score_rr(vn_stat.age_y,IF(os.rr=0,NULL,os.rr),NULL),",kphis,".score_sbp(vn_stat.age_y,IF(os.bps=0,NULL,os.bps),NULL),0,0) AS mews ",
//         "FROM ",hosxp,".opdscreen os ",
//             "LEFT JOIN ",hosxp,".er_regist er_r ON os.vn=er_r.vn ",
//             "LEFT JOIN ",hosxp,".er_pt_type pt ON er_r.er_pt_type=pt.er_pt_type ",
//             "LEFT JOIN ",hosxp,".er_emergency_type er_et ON er_r.er_emergency_type=er_et.er_emergency_type ",
//             "LEFT JOIN ",hosxp,".er_emergency_level er_el ON er_r.er_emergency_level_id=er_el.er_emergency_level_id ",
//             "LEFT JOIN ",hosxp,".er_nursing_detail er_ed ON os.vn=er_ed.vn ",
//             "LEFT JOIN ",hosxp,".vn_stat ON vn_stat.vn=os.vn ",
//             "LEFT JOIN ",hosxp,".ovst ON ovst.vn=os.vn ",
//             "LEFT JOIN ",hosxp,".spclty ON spclty.spclty=ovst.spclty ",
//         "WHERE os.vn=?;"
//     )
// }
// // remove MySQL functions
pub fn get_opdscreens(hosxp: &str) -> String {
    [
        "SELECT os.vstdate,os.vsttime,os.bw,os.height,os.pain_score,IF(os.bpd=0,NULL,os.bpd) AS bpd,IF(os.bps=0,NULL,os.bps) AS bps,\
            IF(os.pulse=0,NULL,os.pulse) AS pulse,IF(os.rr=0,NULL,os.rr) AS rr,IF(os.temperature=0,NULL,os.temperature) AS temperature,\
            os.cc,os.hpi,os.pe_ga_text,os.pe_heent,os.pe_heent_text,os.pe_heart,os.pe_heart_text,os.pe_lung,os.pe_lung_text,\
            os.pe_ab,os.pe_ab_text,os.pe_ext,os.pe_ext_text,os.pe_neuro,os.pe_neuro_text,os.pe,pt.`name` AS er_pt_type_name,\
            er_et.name AS er_emergency_type_name,er_el.er_emergency_level_name,er_ed.gcs_e,er_ed.gcs_m, er_ed.gcs_v,spclty.`name` AS er_spclty_name \
        FROM ",hosxp,".opdscreen os \
            LEFT JOIN ",hosxp,".er_regist er_r ON os.vn=er_r.vn \
            LEFT JOIN ",hosxp,".er_pt_type pt ON er_r.er_pt_type=pt.er_pt_type \
            LEFT JOIN ",hosxp,".er_emergency_type er_et ON er_r.er_emergency_type=er_et.er_emergency_type \
            LEFT JOIN ",hosxp,".er_emergency_level er_el ON er_r.er_emergency_level_id=er_el.er_emergency_level_id \
            LEFT JOIN ",hosxp,".er_nursing_detail er_ed ON os.vn=er_ed.vn \
            LEFT JOIN ",hosxp,".ovst ON ovst.vn=os.vn \
            LEFT JOIN ",hosxp,".spclty ON spclty.spclty=ovst.spclty \
        WHERE os.vn=?;"
    ].concat()
}

// SELECT GROUP_CONCAT(CONCAT(opd_allergy.agent,'=',IF(opd_allergy.symptom IS NULL,',',opd_allergy.symptom)),'\n') AS drugallergy
// FROM hos.opd_allergy WHERE opd_allergy.hn = ? ORDER BY display_order;
/// hn
pub fn get_hosxp_drugallergy(hosxp: &str) -> String {
    [
        "SELECT CONCAT(opd_allergy.agent,'=',IF(opd_allergy.symptom IS NULL,',',opd_allergy.symptom)) AS drugallergy \
        FROM ",hosxp,".opd_allergy WHERE opd_allergy.hn = ? ORDER BY display_order;"
    ].concat()
}

// SELECT CONCAT(IFNULL(ol.enter_date,''),', ',IFNULL(ol.operation_name,''),', ',IFNULL(d.name,''),', ',':hospital_name','\n') AS operation_list
// FROM hos.operation_list ol LEFT JOIN hos.doctor d ON d.code = ol.request_doctor
// WHERE ol.hn=? AND ol.status_id = 3 AND CONCAT(ol.enter_date,' ',ol.enter_time) < ? ORDER BY ol.enter_date,ol.enter_time;
/// hn, vstdate_time
pub fn get_hosxp_operation_history(hospital_name: &str, hosxp: &str) -> String {
    [
        "SELECT CONCAT(IFNULL(ol.enter_date,''),', ',IFNULL(ol.operation_name,''),', ',IFNULL(d.name,''),', ','",hospital_name,"','\n') AS operation_list \
        FROM ",hosxp,".operation_list ol LEFT JOIN ",hosxp,".doctor d ON d.code = ol.request_doctor \
        WHERE ol.hn=? AND ol.status_id = 3 AND CONCAT(ol.enter_date,' ',ol.enter_time) < ? ORDER BY ol.enter_date,ol.enter_time;"
    ].concat()
}

// SELECT GROUP_CONCAT(IF(v.diagtype=1,CONCAT(v.icd10,':',icd101.name,'(PDX)'),CONCAT(v.icd10,':',icd101.name)) SEPARATOR '\n') AS diagnosis_concat
// FROM hos.ovstdiag v LEFT JOIN hos.icd101 ON icd101.code=v.icd10 WHERE vn=?;
/// vn
pub fn get_hosxp_diagnosis(hosxp: &str) -> String {
    [
        "SELECT IF(v.diagtype=1,CONCAT(v.icd10,' : ',icd101.name,' (PDX)'),CONCAT(v.icd10,' : ',icd101.name)) AS diagnosis \
        FROM ",hosxp,".ovstdiag v LEFT JOIN ",hosxp,".icd101 ON icd101.code=v.icd10 WHERE vn=?;"
    ].concat()
}

// SELECT GROUP_CONCAT(CONCAT(d.name,' ',d.strength,' ',du.shortlist,' X ',o1.qty) SEPARATOR '\n') AS drug_history_concat
// FROM hos.opitemrece o1 INNER JOIN hos.drugitems d ON o1.icode=d.icode
// LEFT JOIN hos.drugusage du ON du.drugusage=o1.drugusage LEFT JOIN hos.sp_use u ON u.sp_use = o1.sp_use
// WHERE o1.vn=?;
/// vn
pub fn get_hosxp_drug_history(hosxp: &str) -> String {
    [
        "SELECT CONCAT(d.name,' ',d.strength,' ',du.shortlist,' X ',o1.qty) AS drug_history \
        FROM ",hosxp,".opitemrece o1 INNER JOIN ",hosxp,".drugitems d ON o1.icode=d.icode \
        LEFT JOIN ",hosxp,".drugusage du ON du.drugusage=o1.drugusage LEFT JOIN ",hosxp,".sp_use u ON u.sp_use = o1.sp_use \
        WHERE o1.vn=?;"
    ].concat()
}

// SELECT vs.vs_datetime,vs.bw,vs.height,vs.pain,vs.sat,vs.right_pupil, vs.left_pupil,vs.eye,vs.verbal,vs.movement,
//     vs.hct,vs.dtx,vs.bl,vs.bt,vs.pr,vs.rr,vs.sbp,vs.dbp,vs.respirator,vs.inotrope,vs.conscious_id,vs.urine_amount,vs.urine_duration,
//     kphis.score_total(kphis.score_bt(2,vs.bt),kphis.score_pr(2,vs.pr),kphis.score_rr(2,vs.rr,vs.respirator),kphis.score_sbp(2,vs.sbp,vs.inotrope),
//         kphis.score_conscious_id(2,vs.conscious_id),kphis.score_urine(2,vs.urine_amount, vs.urine_duration)) AS kphis_mews
// FROM kphis.opd_er_vs_vital_sign vs WHERE vs.opd_er_order_master_id = ? ORDER BY vs.vs_datetime ASC LIMIT 1;
/// opd_er_order_master_id
// pub fn get_vs_kphis(age_y: i8, kphis: &str) -> String {
//     let age = age_y.to_owned();
//     [
//         "SELECT vs.vs_datetime,vs.bw,vs.height,vs.pain,vs.sat,vs.right_pupil, vs.left_pupil,vs.eye,vs.verbal,vs.movement,",
//             "vs.hct,vs.dtx,vs.bl,vs.bt,vs.pr,vs.rr,vs.sbp,vs.dbp,vs.respirator,vs.inotrope,vs.conscious_id,vs.urine_amount,vs.urine_duration,",
//             kphis,".score_total(",kphis,".score_bt(",age,",vs.bt),",kphis,".score_pr(",age,",vs.pr),",kphis,".score_rr(",age,",vs.rr,vs.respirator),",kphis,".score_sbp(",age,",vs.sbp,vs.inotrope),",
//                 kphis,".score_conscious_id(",age,",vs.conscious_id),",kphis,".score_urine(",age,",vs.urine_amount, vs.urine_duration)) AS kphis_mews ",
//         "FROM ",kphis,".opd_er_vs_vital_sign vs WHERE vs.opd_er_order_master_id = ? ORDER BY vs.vs_datetime ASC LIMIT 1;"
//     )
// }
// // remove MySQL functions
pub fn get_vs_kphis(kphis: &str) -> String {
    [
        "SELECT vs.vs_datetime,vs.bw,vs.height,vs.pain,vs.sat,vs.right_pupil, vs.left_pupil,vs.eye,vs.verbal,vs.movement,\
            vs.hct,vs.dtx,vs.bl,vs.bt,vs.pr,vs.rr,vs.sbp,vs.dbp,",
            // vs.respirator,vs.inotrope,vs.conscious_id,vs.urine_amount,vs.urine_duration,",
            CONCAT_SQL," AS ews_concat \
        FROM ",kphis,".opd_er_vs_vital_sign vs WHERE vs.opd_er_order_master_id = ? ORDER BY vs.vs_datetime ASC LIMIT 1;"
    ].concat()
}

// // opd-er-medical-history-dr-data.php
// SELECT dr_pe.*,d1.`name` AS doctor_name,d2.`name` AS circulation_doctor_name
// FROM kphis.opd_er_dr_pe dr_pe
//   LEFT JOIN hos.doctor d1 ON d1.`code`=dr_pe.doctor_pe
//   LEFT JOIN hos.doctor d2 ON d2.`code`=dr_pe.circulation_doctor
// WHERE dr_pe.opd_er_order_master_id=?;
/// opd_er_order_master_id
pub fn get_trauma_history(hosxp: &str, kphis: &str, kphis_extra: &str) -> String {
    [
        "SELECT dr_pe.*,d1.`name` AS doctor_name,d2.`name` AS circulation_doctor_name,\
            (SELECT GROUP_CONCAT(i.`path` ORDER BY u.image_usage_id) FROM ",kphis_extra,".image AS i \
  		        LEFT JOIN ",kphis_extra,".image_usage AS u ON u.image_id=i.image_id WHERE u.usage_id=2 AND u.usage_key_id=dr_pe.opd_er_order_master_id) AS imgs \
        FROM ",kphis,".opd_er_dr_pe dr_pe \
            LEFT JOIN ",hosxp,".doctor d1 ON d1.`code`=dr_pe.doctor_pe \
            LEFT JOIN ",hosxp,".doctor d2 ON d2.`code`=dr_pe.circulation_doctor \
        WHERE dr_pe.opd_er_order_master_id=?;"
    ].concat()
}

// // opd-er-medical-history-dr-save.php
// INSERT INTO kphis.opd_er_dr_pe (opd_er_order_master_id,arc,arc_npc_text,breathing_chest_wall,breathing_lung,circulation_shock,circulation_shock_text,circulation_other,circulation_other_text,circulation_efast_date,
//     circulation_efast_time,circulation_doctor,circulation,circulation_positive_text,disability_e,disability_v,disability_m,disability_pupil_rt,disability_pupil_lt,disability_other,
//     exposure,doctor_pe,create_user,create_datetime,update_user,update_datetime,version)
// VALUES (?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,NOW(),?,NOW(),1);
/// opd_er_order_master_id, arc, arc_npc_text, breathing_chest_wall, breathing_lung, circulation_shock, circulation_shock_text, circulation_other, circulation_other_text,
/// circulation_efast_date, circulation_efast_time, circulation_doctor, circulation, circulation_positive_text, disability_e, disability_v, disability_m, disability_pupil_rt,
/// disability_pupil_lt, disability_other, exposure, doctor_pe, loginname, loginname
pub fn insert_trauma_history(kphis: &str) -> String {
    [
        "INSERT INTO ",kphis,".opd_er_dr_pe (opd_er_order_master_id,arc,arc_npc_text,breathing_chest_wall,breathing_lung,circulation_shock,circulation_shock_text,\
            circulation_other,circulation_other_text,circulation_efast_date,circulation_efast_time,circulation_doctor,circulation,circulation_positive_text,\
            disability_e,disability_v,disability_m,disability_pupil_rt,disability_pupil_lt,disability_other,exposure,doctor_pe",TABLE_CREATE_COLUMNS,") \
        VALUES (?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?",TABLE_CREATE_PREPARED,");"
    ].concat()
}

// // opd-er-medical-history-dr-update.php
// UPDATE kphis.opd_er_dr_pe SET arc=?,arc_npc_text=?,breathing_chest_wall=?,breathing_lung=?,circulation_shock=?,circulation_shock_text=?,
//     circulation_other=?,circulation_other_text=?,circulation_efast_date=?,circulation_efast_time=?,circulation_doctor=?,circulation=?,circulation_positive_text=?,
//     disability_e=?,disability_v=?,disability_m=?,disability_pupil_rt=?,disability_pupil_lt=?,disability_other=?,exposure=?,doctor_pe=?,
//     update_user=?,update_datetime=NOW(),version=(version+1)
// WHERE opd_er_pe_id=? AND version=?;
/// arc, arc_npc_text, breathing_chest_wall, breathing_lung, circulation_shock, circulation_shock_text, circulation_other, circulation_other_text, circulation_efast_date,
/// circulation_efast_time, circulation_doctor, circulation, circulation_positive_text, disability_e, disability_v, disability_m, disability_pupil_rt, disability_pupil_lt,
/// disability_other, exposure, doctor_pe, loginname, opd_er_pe_id, version
pub fn update_trauma_history(kphis: &str) -> String {
    [
        "UPDATE ",kphis,".opd_er_dr_pe SET arc=?,arc_npc_text=?,breathing_chest_wall=?,breathing_lung=?,circulation_shock=?,circulation_shock_text=?,\
            circulation_other=?,circulation_other_text=?,circulation_efast_date=?,circulation_efast_time=?,circulation_doctor=?,circulation=?,circulation_positive_text=?,\
            disability_e=?,disability_v=?,disability_m=?,disability_pupil_rt=?,disability_pupil_lt=?,disability_other=?,exposure=?,doctor_pe=?",TABLE_UPDATE_SET,
        " WHERE opd_er_pe_id=? AND version=?;"
    ].concat()
}

// // opd-er-allergy-history-edit.php
// SELECT er_a.*,doctor.`name`
// FROM kphis.opd_er_allergy_history er_a
//     LEFT JOIN hos.doctor ON doctor.`code`=er_a.er_allergy_history_doctorcode
// WHERE opd_er_order_master_id=? ORDER BY er_a.er_allergy_history_id ASC;
/// opd_er_order_master_id
pub fn get_allergy_history(hosxp: &str, kphis: &str) -> String {
    [
        "SELECT er_a.*,doctor.`name` AS doctor_name \
        FROM ",kphis,".opd_er_allergy_history er_a \
            LEFT JOIN ",hosxp,".doctor ON doctor.`code`=er_a.er_allergy_history_doctorcode \
        WHERE opd_er_order_master_id=? ORDER BY er_a.er_allergy_history_id ASC;"
    ].concat()
}

// // opd-er-allergy-history-save.php
// DELETE FROM kphis.opd_er_allergy_history WHERE opd_er_order_master_id=? AND version=?;
/// opd_er_order_master_id, version
pub fn delete_allergy_history(kphis: &str) -> String {
    [
        "DELETE FROM ",kphis,".opd_er_allergy_history WHERE opd_er_order_master_id=? AND version=?;"
    ].concat()
}

// INSERT INTO kphis.opd_er_allergy_history (opd_er_order_master_id,er_allergy_history_agent,er_allergy_history_symptom,
//     er_allergy_history_doctorcode,create_user,create_datetime,update_user,update_datetime,version)
// VALUES (?,?,?,?,?,NOW(),?,NOW(),?),..
/// [opd_er_order_master_id, er_allergy_history_agent, er_allergy_history_symptom, er_allergy_history_doctorcode,
/// loginname, loginname, version+1]
pub fn insert_allergy_history(len: usize, kphis: &str) -> String {
    let values = vec!["(?,?,?,?,?,NOW(),?,NOW(),?)"; len].join(",");
    [
        "INSERT INTO ",kphis,".opd_er_allergy_history (opd_er_order_master_id,er_allergy_history_agent,er_allergy_history_symptom,\
            er_allergy_history_doctorcode",TABLE_CREATE_COLUMNS,") \
        VALUES ",&values
    ].concat()
}

// // opd-er-medical-history-nurse-data.php
// SELECT nurse_s.*,doctor_d.`name` AS doctor_name,doctor_n.`name` AS nurse_name
// FROM kphis.opd_er_nurse_screening nurse_s
//     LEFT JOIN hos.doctor doctor_d ON doctor_d.`code`=nurse_s.screening_doctor_doctorcode
//     LEFT JOIN hos.doctor doctor_n ON doctor_n.`code`=nurse_s.screening_nurse_doctorcode
// WHERE opd_er_order_master_id=?;
/// opd_er_order_master_id
pub fn get_nurse_screening_history(hosxp: &str, kphis: &str) -> String {
    [
        "SELECT nurse_s.*,doctor_d.`name` AS doctor_name,doctor_n.`name` AS nurse_name \
        FROM ",kphis,".opd_er_nurse_screening nurse_s \
            LEFT JOIN ",hosxp,".doctor doctor_d ON doctor_d.`code`=nurse_s.screening_doctor_doctorcode \
            LEFT JOIN ",hosxp,".doctor doctor_n ON doctor_n.`code`=nurse_s.screening_nurse_doctorcode \
        WHERE opd_er_order_master_id=?;"
    ].concat()
}

// // opd-er-medical-history-nurse-save.php
// INSERT INTO kphis.opd_er_nurse_screening (opd_er_order_master_id,screening_emergency_level,screening_spclty,
//     screening_arrive_date,screening_arrive_time,screening_date,screening_time,screening_report_date,screening_report_time,
//     screening_see_doctor_date,screening_see_doctor_time,screening_doctor_doctorcode,screening_nurse_doctorcode,
//     create_user,create_datetime,update_user,update_datetime,version)
// VALUES (?,?,?,?,?,?,?,?,?,?,?,?,?,?,NOW(),?,NOW(),1);
/// opd_er_order_master_id, screening_emergency_level, screening_spclty, screening_arrive_date, screening_arrive_time,
/// screening_date, screening_time, screening_report_date, screening_report_time, screening_see_doctor_date,
/// screening_see_doctor_time, screening_doctor_doctorcode, screening_nurse_doctorcode, loginname, loginname
pub fn insert_nurse_screening_history(kphis: &str) -> String {
    [
        "INSERT INTO ",kphis,".opd_er_nurse_screening (opd_er_order_master_id,screening_emergency_level,screening_spclty,\
            screening_arrive_date,screening_arrive_time,screening_date,screening_time,screening_report_date,screening_report_time,\
            screening_see_doctor_date,screening_see_doctor_time,screening_doctor_doctorcode,screening_nurse_doctorcode",
        TABLE_CREATE_COLUMNS,") VALUES (?,?,?,?,?,?,?,?,?,?,?,?,?",TABLE_CREATE_PREPARED,");"
    ].concat()
}

// // opd-er-medical-history-nurse-update.php
// UPDATE kphis.opd_er_nurse_screening SET screening_emergency_level=?,screening_spclty=?,screening_arrive_date=?,screening_arrive_time=?,
//     screening_date=?,screening_time=?,screening_report_date=?,screening_report_time=?,screening_see_doctor_date=?,screening_see_doctor_time=?,
//     screening_nurse_doctorcode=?,update_user=?,update_datetime=NOW(),version=(version+1)
// WHERE opd_er_screening_id=? AND version=?;
/// screening_emergency_level, screening_spclty, screening_arrive_date, screening_arrive_time,
/// screening_date, screening_time, screening_report_date, screening_report_time,
/// screening_see_doctor_date, screening_see_doctor_time, screening_nurse_doctorcode, loginname, opd_er_screening_id, version
pub fn update_nurse_screening_history(view_by: &str, kphis: &str) -> String {
    [
        "UPDATE ",kphis,".opd_er_nurse_screening SET screening_emergency_level=?,screening_spclty=?,screening_arrive_date=?,screening_arrive_time=?,\
            screening_date=?,screening_time=?,screening_report_date=?,screening_report_time=?,screening_see_doctor_date=?,screening_see_doctor_time=?,\
            screening_",view_by,"_doctorcode=?",TABLE_UPDATE_SET,
        " WHERE opd_er_screening_id=? AND version=?;"
    ].concat()
}

// // opd-er-consult-dr-edit.php
// SELECT er_c.*,doctor.`name` AS doctor_name,ks.spclty_name AS er_consult_ward_name
// FROM kphis.opd_er_consult er_c
//     LEFT JOIN hos.doctor ON doctor.`code`=er_c.er_consult_doctorcode
//     LEFT JOIN kphis.kphis_spclty ks ON ks.spclty_id = er_c.er_consult_ward
// WHERE opd_er_order_master_id=? ORDER BY er_consult_id ASC;
/// opd_er_order_master_id
pub fn get_consult_history(hosxp: &str, kphis: &str) -> String {
    [
        "SELECT er_c.*,doctor.`name` AS doctor_name,ks.spclty_name AS er_consult_ward_name \
        FROM ",kphis,".opd_er_consult er_c \
            LEFT JOIN ",hosxp,".doctor ON doctor.`code`=er_c.er_consult_doctorcode \
            LEFT JOIN ",kphis,".kphis_spclty ks ON ks.spclty_id = er_c.er_consult_ward \
        WHERE opd_er_order_master_id=? ORDER BY er_consult_id ASC;"
    ].concat()
}

// // opd-er-consult-dr-save.php
// DELETE FROM kphis.opd_er_consult WHERE opd_er_order_master_id=? AND version=?;
/// opd_er_order_master_id, version
pub fn delete_consult_history(kphis: &str) -> String {
    [
        "DELETE FROM ",kphis,".opd_er_consult WHERE opd_er_order_master_id=? AND version=?;"
    ].concat()
}

// INSERT INTO kphis.opd_er_consult (opd_er_order_master_id,er_consult_ward,er_consult_date,er_consult_time,
//     er_consult_doctor_reply,er_consult_date_reply,er_consult_time_reply,er_consult_doctorcode,
//     create_user,create_datetime,update_user,update_datetime,version)
// VALUES (?,?,?,?,?,?,?,?,?,NOW(),?,NOW(),?),..
/// [opd_er_order_master_id, er_consult_ward, er_consult_date, er_consult_time, er_consult_doctor_reply,
/// er_consult_date_reply, er_consult_time_reply, er_consult_doctorcode, loginname, loginname, version+1 ]
pub fn insert_consult_history(len: usize, kphis: &str) -> String {
    let values = vec!["(?,?,?,?,?,?,?,?,?,NOW(),?,NOW(),?)"; len].join(",");
    [
        "INSERT INTO ",kphis,".opd_er_consult (opd_er_order_master_id,er_consult_ward,er_consult_date,er_consult_time,\
            er_consult_doctor_reply,er_consult_date_reply,er_consult_time_reply,er_consult_doctorcode",
        TABLE_CREATE_COLUMNS,") VALUES ",&values
    ].concat()
}

// // opd-er-document-scan-data.php
// SELECT * FROM kphis.opd_er_document_scan WHERE opd_er_order_master_id=?;
/// opd_er_order_master_id
pub fn get_scan_history(kphis: &str) -> String {
    [
        "SELECT * FROM ",kphis,".opd_er_document_scan WHERE opd_er_order_master_id=?;"
    ].concat()
}

// // opd-er-document-scan-save.php
// INSERT INTO kphis.opd_er_document_scan (opd_er_order_master_id,opd_er_document_scan,opd_er_document_scan_doctorcode,
//     create_user,create_datetime,update_user,update_datetime,version)
// VALUES (?,?,?,?,NOW(),?,NOW(),1);
/// opd_er_order_master_id, opd_er_document_scan, opd_er_document_scan_doctorcode, loginname, loginname
pub fn insert_scan_history(kphis: &str) -> String {
    [
        "INSERT INTO ",kphis,".opd_er_document_scan (opd_er_order_master_id,opd_er_document_scan,opd_er_document_scan_doctorcode",
        TABLE_CREATE_COLUMNS,") VALUES (?,?,?",TABLE_CREATE_PREPARED,");"
    ].concat()
}

// // opd-er-document-scan-update.php
// UPDATE kphis.opd_er_document_scan SET opd_er_order_master_id=?,opd_er_document_scan=?,opd_er_document_scan_doctorcode=?,
//     update_user=?,update_datetime=NOW(),version=(version+1)
// WHERE opd_er_document_scan_id=? AND version=?;
/// opd_er_order_master_id, opd_er_document_scan, opd_er_document_scan_doctorcode, loginname, opd_er_document_scan_id, version
pub fn update_scan_history(kphis: &str) -> String {
    [
        "UPDATE ",kphis,".opd_er_document_scan SET opd_er_order_master_id=?,opd_er_document_scan=?,opd_er_document_scan_doctorcode=?",TABLE_UPDATE_SET,
        " WHERE opd_er_document_scan_id=? AND version=?;"
    ].concat()
}

// // opd-er-set-ft-data.php
// SELECT set_ft.*,doctor.`name`
// FROM kphis.opd_er_set_fast_track set_ft
//     LEFT JOIN hos.doctor ON doctor.`code`=set_ft.set_ft_doctorcode
// WHERE opd_er_order_master_id=?;
/// opd_er_order_master_id
pub fn get_set_ft_history(hosxp: &str, kphis: &str) -> String {
    [
        "SELECT set_ft.*,doctor.`name` AS doctor_name \
        FROM ",kphis,".opd_er_set_fast_track set_ft \
            LEFT JOIN ",hosxp,".doctor ON doctor.`code`=set_ft.set_ft_doctorcode \
        WHERE opd_er_order_master_id=?;"
    ].concat()
}

// // opd-er-set-ft-save.php
// INSERT INTO kphis.opd_er_set_fast_track (opd_er_order_master_id,set_ft_date,set_ft_time,set_ft_doctorcode,
//     create_user,create_datetime,update_user,update_datetime,version)
// VALUES (?,?,?,?,?,NOW(),?,NOW(),1);
/// opd_er_order_master_id, set_ft_date, set_ft_time, set_ft_doctorcode, loginname, loginname
pub fn insert_ft_history(kphis: &str) -> String {
    [
        "INSERT INTO ",kphis,".opd_er_set_fast_track (opd_er_order_master_id,set_ft_date,set_ft_time,set_ft_doctorcode",
        TABLE_CREATE_COLUMNS,") VALUES (?,?,?,?",TABLE_CREATE_PREPARED,");"
    ].concat()
}

// // opd-er-set-ft-update.php
// UPDATE kphis.opd_er_set_fast_track SET opd_er_order_master_id=?,set_ft_date=?,set_ft_time=?,set_ft_doctorcode=?,
//     update_user=?,update_datetime=NOW(),version=(version+1)
// WHERE set_ft_id=? AND version=?;
/// opd_er_order_master_id, set_ft_date, set_ft_time, set_ft_doctorcode, loginname, set_ft_id, version
pub fn update_ft_history(kphis: &str) -> String {
    [
        "UPDATE ",kphis,".opd_er_set_fast_track SET opd_er_order_master_id=?,set_ft_date=?,set_ft_time=?,set_ft_doctorcode=?",TABLE_UPDATE_SET,
        " WHERE set_ft_id=? AND version=?;"
    ].concat()
}
