use kphis_model::score::CONCAT_SQL;

use crate::{TABLE_CREATE_COLUMNS, TABLE_UPDATE_SET};

// $an_parameters = ['an' => $an];
// $hn_parameters = ['hn' => $hn];
// $loginname = $_SESSION['loginname'];
// $values =['loginname'=>$loginname];

// SELECT * FROM kphis.ipd_dr_admission_note WHERE an = ?;
pub fn select_admission_note_from_an(kphis: &str, kphis_extra: &str) -> String {
    [
        "SELECT dan.*,soc.stage_of_change_name,(SELECT GROUP_CONCAT(i.`path` ORDER BY u.image_usage_id) FROM ",kphis_extra,".image AS i \
  		    LEFT JOIN ",kphis_extra,".image_usage AS u ON u.image_id=i.image_id WHERE u.usage_id=1 AND u.usage_key_id=dan.admission_note_id) AS imgs \
        FROM ",kphis,".ipd_dr_admission_note dan \
            LEFT JOIN ",kphis,".ipd_vs_stage_of_change soc ON soc.stage_of_change_id=dan.stage_of_change_id \
        WHERE dan.an=?;"
    ].concat()
}

// SELECT hn,CONCAT(vstdate,' ',vsttime) AS vstdatetime,bw,height,pain_score,IF(bpd=0,NULL,bpd) AS bpd,IF(bps=0,NULL,bps) AS bps,
//    IF(pulse=0,NULL,pulse) AS pulse,IF(rr=0,NULL,rr) AS rr,IF(temperature=0,NULL,temperature) AS temperature,
//    cc,hpi,pe_ga_text,pe_heent_text,pe_heart_text,pe_lung_text,pe_ab_text,pe_ext_text,pe_neuro_text,pe,
//    pe_skin_text,pe_chest_text,pe_gy_text,pe_gu_text,pe_head_text,pe_gi_text,pe_pv_text,pe_pr_text,pe_gen_text
// FROM hos.opdscreen WHERE vn=?;
pub fn select_opdscreen_pe_from_vn(hosxp: &str) -> String {
    [
        "SELECT hn,ADDTIME(CONVERT(vstdate,DATETIME),vsttime) AS vstdatetime,bw,height,pain_score,IF(bpd=0,NULL,bpd) AS bpd,IF(bps=0,NULL,bps) AS bps,\
            IF(pulse=0,NULL,pulse) AS pulse,IF(rr=0,NULL,rr) AS rr,IF(temperature=0,NULL,temperature) AS temperature,\
            cc,hpi,pe_ga_text,pe_heent_text,pe_heart_text,pe_lung_text,pe_ab_text,pe_ext_text,pe_neuro_text,pe \
        FROM ",hosxp,".opdscreen WHERE vn=?;"
    ].concat()
}

// SELECT pt.hn,pt.sex,pt.pname,pt.fname,pt.lname,ans.age_y,ans.age_m,ans.age_d,
//     ipt.vn,ipt.dchdate,ipt.dchtime,ipt.ward,w.name,ipt.pttype,iad.bedno,ty.`name` AS pttype_name,
//     concat(ipt.regdate,' ',ipt.regtime) AS regdatetime,
//     (SELECT GROUP_CONCAT(CONCAT(oa.agent,'=',IF(oa.symptom IS NULL,',',oa.symptom))) as name FROM hos.opd_allergy oa WHERE oa.hn = ipt.hn ORDER BY oa.display_order) AS drugallergy,
//     (SELECT vs.bw FROM kphis.ipd_vs_vital_sign vs WHERE vs.an = ipt.an AND vs.bw IS NOT NULL AND trim(vs.bw) <> '' ORDER BY vs.vs_datetime DESC LIMIT 1) AS latest_bw,
//     (SELECT vs.height FROM kphis.ipd_vs_vital_sign vs WHERE vs.an = ipt.an AND vs.bw IS NOT NULL AND trim(vs.bw) <> '' ORDER BY vs.vs_datetime DESC LIMIT 1) AS latest_height,
//     (SELECT vs.vs_datetime FROM kphis.ipd_vs_vital_sign vs WHERE vs.an = ipt.an AND vs.bw IS NOT NULL AND trim(vs.bw) <> '' ORDER BY vs.vs_datetime DESC LIMIT 1) AS latest_bw_datetime
// FROM hos.ipt
//     LEFT JOIN hos.an_stat ans ON ans.an=ipt.an
//     LEFT JOIN hos.patient pt ON pt.hn=ipt.hn
//     LEFT JOIN hos.ward w ON w.ward=ipt.ward
//     LEFT JOIN hos.pttype ty ON ty.pttype = ipt.pttype
//     LEFT JOIN hos.iptadm iad ON iad.an = ipt.an
// WHERE ipt.an=? ORDER BY ipt.an;
// pub fn select_ipt_from_an(hosxp: &str, kphis: &str) -> String {
//     [
//         "SELECT pt.hn,pt.sex,pt.pname,pt.fname,pt.lname,ans.age_y,ans.age_m,ans.age_d,\
//             ipt.vn,ipt.dchdate,ipt.dchtime,ipt.ward,w.name,ipt.pttype,iad.bedno,ty.`name` AS pttype_name,\
//             CONCAT(ipt.regdate,' ',ipt.regtime) AS regdatetime,\
//             (SELECT GROUP_CONCAT(CONCAT(oa.agent,'=',IF(oa.symptom IS NULL,',',oa.symptom))) as name FROM ",hosxp,".opd_allergy oa WHERE oa.hn = ipt.hn ORDER BY oa.display_order) AS drugallergy,\
//             (SELECT vs.bw FROM ",kphis,".ipd_vs_vital_sign vs WHERE vs.an = ipt.an AND vs.bw IS NOT NULL AND trim(vs.bw) <> '' ORDER BY vs.vs_datetime DESC LIMIT 1) AS latest_bw,\
//             (SELECT vs.height FROM ",kphis,".ipd_vs_vital_sign vs WHERE vs.an = ipt.an AND vs.bw IS NOT NULL AND trim(vs.bw) <> '' ORDER BY vs.vs_datetime DESC LIMIT 1) AS latest_height,\
//             (SELECT vs.vs_datetime FROM ",kphis,".ipd_vs_vital_sign vs WHERE vs.an = ipt.an AND vs.bw IS NOT NULL AND trim(vs.bw) <> '' ORDER BY vs.vs_datetime DESC LIMIT 1) AS latest_bw_datetime \
//         FROM ",hosxp,".ipt \
//             LEFT JOIN ",hosxp,".an_stat ans ON ans.an=ipt.an \
//             LEFT JOIN ",hosxp,".patient pt ON pt.hn=ipt.hn \
//             LEFT JOIN ",hosxp,".ward w ON w.ward=ipt.ward \
//             LEFT JOIN ",hosxp,".pttype ty ON ty.pttype=ipt.pttype \
//             LEFT JOIN ",hosxp,".iptadm iad ON iad.an=ipt.an \
//         WHERE ipt.an=? ORDER BY ipt.an;"
//     ].concat()
// }

// SELECT ipt.hn,ipt.vn,CONCAT(ipt.regdate,' ',ipt.regtime) AS regdatetime FROM hosxp.ipt WHERE ipt.an=?;
/// an
pub fn select_ipt_from_an(hosxp: &str) -> String {
    [
        "SELECT ipt.hn,ipt.vn,ADDTIME(CONVERT(ipt.regdate,DATETIME),ipt.regtime) AS regdatetime \
        FROM ",hosxp,".ipt \
        WHERE ipt.an=?;"
    ].concat()
}

// $regdatetime = $row_ipt["regdatetime"]

// SELECT i.admission_note_item_id,
//     i.admission_note_doctor,
//     d.`name` AS admission_note_doctorname,
//     d.licenseno
// FROM kphis.ipd_dr_admission_note_item i
//     LEFT JOIN hos.doctor d ON d.code = i.admission_note_doctor
// WHERE i.an=:an ORDER BY i.admission_note_item_id ASC;
pub fn select_admission_note_dr_items_from_an(hosxp: &str, kphis: &str) -> String {
    [
        "SELECT i.admission_note_item_id,i.admission_note_doctor,d.`name` AS admission_note_doctorname,d.licenseno,\
            (SELECT entryposition FROM ",hosxp,".opduser WHERE doctorcode=i.admission_note_doctor AND (account_disable IS NULL OR account_disable='N') LIMIT 1) AS entryposition \
        FROM ",kphis,".ipd_dr_admission_note_item i \
            LEFT JOIN ",hosxp,".doctor d ON d.code=i.admission_note_doctor \
        WHERE i.an=? ORDER BY i.admission_note_item_id ASC;"
    ].concat()
}
// $admission_note_count = rows.len()
// $admission_note_item_id = vec!(rows.admission_note_item_id)
// $admission_note_doctor = vec!(rows.admission_note_doctor)
// $admission_note_doctorname = vec!(rows.admission_note_doctorname)

// $row_opduser = SELECT opduser.entryposition,opduser.name
//     FROM hos.opduser
//     WHERE loginname = :loginname

// $hnan_para = ['an' => $an,'hn'=>$hn];

// SELECT concat(ipt.regdate,' ',ipt.regtime) AS old_regdatetime
// FROM  hos.ipt
// WHERE ipt.hn = :hn AND ipt.an < :an
// ORDER BY ipt.an DESC LIMIT 1;
/// hn, (an)
pub fn select_old_regdatetime_from_hn(has_an: bool, hosxp: &str) -> String {
    let where_an = if has_an {" AND ipt.an < ? "} else {""};
    [
        "SELECT ADDTIME(CONVERT(ipt.regdate,DATETIME),ipt.regtime) AS old_regdatetime \
        FROM ",hosxp,".ipt WHERE ipt.hn=? ",where_an," ORDER BY ipt.an DESC LIMIT 1;"
    ].concat()
}

// $reg_parameters =[
//     'hn' => $hn,
//     'regdatetime'=>$regdatetime,
//     'hospital_name'=>KphisConstant::KPHIS_HOSPITAL_NAME
// ];

// SELECT CONCAT(IFNULL(ol.enter_date,''),', ',IFNULL(ol.operation_name,''),', ',IFNULL(d.name,'')) AS operation_list
// FROM hos.operation_list ol LEFT JOIN hos.doctor d ON d.code = ol.request_doctor
// WHERE ol.hn = ? AND ol.status_id = 3 AND concat(ol.enter_date,' ',ol.enter_time) < '2023-01-12 12:25:54' ORDER BY ol.enter_date,ol.enter_time;
// *** remove hospital_name from operation_list, we will return it directly
pub fn select_operation_list_from_hn(regdatetime: &str, hosxp: &str) -> String {
    [
        "SELECT CONCAT(IFNULL(ol.enter_date,''),', ',IFNULL(ol.operation_name,''),', ',IFNULL(d.name,'')) AS operation_list \
        FROM ",hosxp,".operation_list ol LEFT JOIN ",hosxp,".doctor d ON d.code=ol.request_doctor \
        WHERE ol.hn=? AND ol.status_id=3 AND ADDTIME(CONVERT(ol.enter_date,DATETIME),ol.enter_time) < '",regdatetime,"' ORDER BY ol.enter_date,ol.enter_time;"
    ].concat()
}

// $operation_text = vec![
//     '<label>'.$row_ol["operation_list"].'</label><br>'s
// ]

// // use MySQL functions
// SELECT sbp,dbp,bt,pr,rr,eye,verbal,movement,braden,
//     kphis.score_total(kphis.score_bt(2,bt),kphis.score_pr(2,pr),kphis.score_rr(2,rr,respirator),kphis.score_sbp(2,sbp,inotrope),
//         kphis.score_conscious_id(2,conscious_id),kphis.score_urine(2,urine_amount, urine_duration)) AS kphis_mews
// FROM kphis.ipd_vs_vital_sign WHERE an=? GROUP BY vs_datetime ASC LIMIT 1;
/// an
// pub fn select_vs_from_an(age_y: i8, kphis: &str) -> String {
//     let age = age_y.to_owned();
//     [
//         "SELECT sbp,dbp,bt,pr,rr,eye,verbal,movement,braden,",
//             kphis,".score_total(",kphis,".score_bt(",age,",bt),",kphis,".score_pr(",age,",pr),",kphis,".score_rr(",age,",rr,respirator),",kphis,".score_sbp(",age,",sbp,inotrope),",
//             kphis,".score_conscious_id(",age,",conscious_id),",kphis,".score_urine(",age,",urine_amount,urine_duration)) AS kphis_mews ",
//         "FROM ",kphis,".ipd_vs_vital_sign WHERE an=? GROUP BY vs_datetime ASC LIMIT 1;"
//     )
// }
// // NOT use MySQL functions
/// an
pub fn select_vs_from_an(kphis: &str) -> String {
    [
        "SELECT vs_datetime,sbp,dbp,bt,pr,rr,eye,verbal,movement,braden,",CONCAT_SQL," AS ews_concat \
        FROM ",kphis,".ipd_vs_vital_sign \
        WHERE an=? ORDER BY vs_datetime ASC LIMIT 1;"
    ].concat()
}

// // SELECT sbp,dbp,bt,pr,rr,eye,verbal,movement,braden FROM kphis.ipd_vs_vital_sign WHERE an=? GROUP BY vs_datetime ASC LIMIT 1;
// pub fn select_vs_from_an(kphis: &str) -> String {
//     [
//         "SELECT sbp,dbp,bt,pr,rr,eye,verbal,movement,braden FROM ",kphis,".ipd_vs_vital_sign WHERE an=? GROUP BY vs_datetime ASC LIMIT 1;"
//     )
// }

// SELECT period,period_normal,period_disorders,period_lmp,period_menopause,occupation,no_risk,smoking,smoke_year,smoke_frequency,smoke_stopped,
// alcohol,alc_year,alc_frequency,alc_stopped,medication_used,med_name,med_year,med_frequency,med_stopped
// FROM kphis.ipd_nurse_admission_note WHERE an=?;
/// from kphis.ipd_nurse_admission_note
pub fn select_period_from_an(kphis: &str) -> String {
    [
        "SELECT period,period_normal,period_disorders,period_lmp,period_menopause,occupation,no_risk,smoking,smoke_year,smoke_frequency,smoke_stopped,\
            alcohol,alc_year,alc_frequency,alc_stopped,medication_used,med_name,med_year,med_frequency,med_stopped \
        FROM ",kphis,".ipd_nurse_admission_note WHERE an=?;"
    ].concat()
}

// UPDATE kphis.ipd_dr_admission_note SET
//     allergy_drug_pharmacy_check_person=?,
//     allergy_drug_pharmacy_check_datetime=NOW(),
//     update_user=?, update_datetime=NOW(), version=(version+1)
// WHERE an=?;
pub fn update_ipd_dr_pharmacy_check(kphis: &str) -> String {
    [
        "UPDATE ",kphis,".ipd_dr_admission_note SET allergy_drug_pharmacy_check_person=?,allergy_drug_pharmacy_check_datetime=NOW()",TABLE_UPDATE_SET," WHERE an=?;"
    ].concat()
}

// SELECT d.name FROM kphis.ipd_doctor_in_charge ipd_dr LEFT JOIN hos.doctor d ON d.`code` = ipd_dr.doctor
// WHERE ipd_dr.an=? AND ipd_dr.activated = 'on' ORDER BY ipd_dr.status DESC;
/// an
pub fn select_ipd_doctor_in_charge(hosxp: &str, kphis: &str) -> String {
    [
        "SELECT d.name FROM ",kphis,".ipd_doctor_in_charge ipd_dr \
            LEFT JOIN ",hosxp,".doctor d ON d.`code`=ipd_dr.doctor \
        WHERE ipd_dr.an=? AND ipd_dr.activated='on' ORDER BY ipd_dr.status DESC;"
    ].concat()
}

// INSERT INTO kphis.ipd_dr_admission_note_item (admission_note_id,an,admission_note_doctor,create_user,create_datetime,update_user,update_datetime,version)
// VALUES (?,?,?,?,NOW(),?,NOW(),1)
/// (admission_note_id, an, admission_note_doctor, loginname, loginname) x len
pub fn insert_ipd_dr_admission_note_items(len: usize, kphis: &str) -> String {
    let values = vec!["(?,?,?,?,NOW(),?,NOW(),1)"; len].join(",");
    [
        "INSERT INTO ",kphis,".ipd_dr_admission_note_item (admission_note_id,an,admission_note_doctor",TABLE_CREATE_COLUMNS,") \
        VALUES ", &values
    ].concat()
}

// DELETE FROM kphis.ipd_dr_admission_note_item WHERE an=?;
/// an
pub fn delete_ipd_dr_admission_note_item(kphis: &str) -> String {
    [
        "DELETE FROM ",kphis,".ipd_dr_admission_note_item WHERE an=?;"
    ].concat()
}
