// // opd-er-show-patient-main.php, opd-er-show-patient-main-sticky.php
// SELECT om.opd_er_order_master_id,ovst.vn,ovst.an,patient.cid,patient.passport_no,patient.hn,patient.pname,patient.fname,patient.lname,patient.birthday,patient.sex,
//     CONCAT_WS(' ',patient.addrpart,'หมู่',patient.moopart,tambol.full_address_name) AS homeaddr,
//     patient.hometel,patient.worktel,patient.workaddr,patient.informtel,patient.informaddr,patient.informname,patient.informrelation,
//     (SELECT GROUP_CONCAT(CONCAT(oa.agent,'=',IF(oa.symptom is null,',',oa.symptom)))
//         FROM hos.opd_allergy oa WHERE oa.hn = ovst.hn ORDER BY display_order) AS drugallergy,
//     (SELECT GROUP_CONCAT(CONCAT(oh.er_allergy_history_agent,'=',IF(oh.er_allergy_history_symptom IS NULL,',',oh.er_allergy_history_symptom)))
//         FROM kphis.opd_er_allergy_history oh WHERE oh.opd_er_order_master_id = om.opd_er_order_master_id
//         ORDER BY oh.er_allergy_history_id) AS er_drugallergy_history,
//     vn_stat.age_y,vn_stat.age_m,vn_stat.age_d,vn_stat.income,ovst.vstdate,ovst.vsttime,ovst.pttype, pttype.`name` AS pttype_name,sex.`name` AS sex_name,occupation.`name` AS occupation_name,
//     religion.`name` AS religion_name,nationality.`name` AS citizenship_name,nationality2.`name` AS nationality_name,marrystatus.`name` AS marrystatus_name,
//     (SELECT vs.height FROM kphis.opd_er_vs_vital_sign vs WHERE vs.opd_er_order_master_id = om.opd_er_order_master_id AND vs.height IS NOT NULL AND TRIM(vs.height) <> '' ORDER BY vs_datetime DESC LIMIT 1) AS latest_height,
//     (SELECT vs.bw FROM kphis.opd_er_vs_vital_sign vs WHERE vs.opd_er_order_master_id = om.opd_er_order_master_id AND vs.bw IS NOT NULL AND TRIM(vs.bw) <> '' ORDER BY vs_datetime DESC LIMIT 1) AS latest_bw,
//     (SELECT vs.vs_datetime FROM kphis.opd_er_vs_vital_sign vs WHERE vs.opd_er_order_master_id = om.opd_er_order_master_id AND vs.bw IS NOT NULL AND TRIM(vs.bw) <> '' ORDER BY vs_datetime DESC LIMIT 1) AS latest_bw_datetime,
//     (SELECT vs.vs_datetime FROM kphis.opd_er_vs_vital_sign vs WHERE vs.opd_er_order_master_id = om.opd_er_order_master_id AND vs.vs_id IS NOT NULL ORDER BY vs_datetime DESC LIMIT 1) AS latest_vs_datetime
// FROM hos.ovst
//     LEFT JOIN kphis.opd_er_order_master om ON ovst.vn=om.vn
//     LEFT JOIN hos.vn_stat ON vn_stat.vn=ovst.vn
//     LEFT JOIN hos.patient ON patient.hn=ovst.hn
//     LEFT JOIN hos.pttype ON pttype.pttype=ovst.pttype
//     LEFT JOIN hos.sex ON sex.code=patient.sex
//     LEFT JOIN hos.tambol ON tambol.tambol_code=CONCAT(patient.chwpart,patient.amppart,patient.tmbpart)
//     LEFT JOIN hos.occupation ON occupation.occupation=patient.occupation
//     LEFT JOIN hos.religion ON religion.religion=patient.religion
//     LEFT JOIN hos.nationality ON nationality.nationality=patient.citizenship
//     LEFT JOIN hos.nationality AS nationality2 ON nationality2.nationality=patient.nationality
//     LEFT JOIN hos.marrystatus ON marrystatus.code=patient.marrystatus
// WHERE om.opd_er_order_master_id=?;
/// key = [opd_er_order_master_id,vn]
pub fn select_show_patient_main(key: &str, hosxp: &str, kphis: &str) -> String {
    let where_key = if key == "vn" {"ovst.vn=?;"} else {"om.opd_er_order_master_id=?;"};
    [
        "SELECT om.opd_er_order_master_id,ipm.pre_admit_master_id,ovst.vn,ovst.an,p.cid,p.passport_no,p.hn,p.pname,p.fname,p.lname,p.birthday,p.sex,\
            CONCAT_WS(' ',p.addrpart,'หมู่',p.moopart,tambol.full_address_name) AS homeaddr,
            p.hometel,p.worktel,p.workaddr,p.informtel,p.informaddr,p.informname,p.informrelation,\
            (SELECT GROUP_CONCAT(CONCAT(oa.agent,'=',IF(oa.symptom is null,',',oa.symptom))) \
                FROM ",hosxp,".opd_allergy oa WHERE oa.hn = ovst.hn ORDER BY display_order) AS drugallergy,\
            (SELECT GROUP_CONCAT(CONCAT(oh.er_allergy_history_agent,'=',IF(oh.er_allergy_history_symptom IS NULL,',',oh.er_allergy_history_symptom))) \
                FROM ",kphis,".opd_er_allergy_history oh WHERE oh.opd_er_order_master_id = om.opd_er_order_master_id \
                ORDER BY oh.er_allergy_history_id) AS er_drugallergy_history,\
            vn_stat.age_y,vn_stat.age_m,vn_stat.age_d,vn_stat.income,ovst.vstdate,ovst.vsttime,ovst.pttype,pttype.`name` AS pttype_name,sex.`name` AS sex_name,occupation.`name` AS occupation_name,\
            religion.`name` AS religion_name,nationality.`name` AS citizenship_name,nationality2.`name` AS nationality_name,marrystatus.`name` AS marrystatus_name,\
            (SELECT vs.height FROM ",kphis,".opd_er_vs_vital_sign vs WHERE vs.opd_er_order_master_id = om.opd_er_order_master_id AND vs.height IS NOT NULL AND TRIM(vs.height) <> '' ORDER BY vs_datetime DESC LIMIT 1) AS latest_height,\
            (SELECT vs.bw FROM ",kphis,".opd_er_vs_vital_sign vs WHERE vs.opd_er_order_master_id = om.opd_er_order_master_id AND vs.bw IS NOT NULL AND TRIM(vs.bw) <> '' ORDER BY vs_datetime DESC LIMIT 1) AS latest_bw,\
            (SELECT vs.vs_datetime FROM ",kphis,".opd_er_vs_vital_sign vs WHERE vs.opd_er_order_master_id = om.opd_er_order_master_id AND vs.bw IS NOT NULL AND TRIM(vs.bw) <> '' ORDER BY vs_datetime DESC LIMIT 1) AS latest_bw_datetime,\
            (SELECT vs.vs_datetime FROM ",kphis,".opd_er_vs_vital_sign vs WHERE vs.opd_er_order_master_id = om.opd_er_order_master_id AND vs.vs_id IS NOT NULL ORDER BY vs_datetime DESC LIMIT 1) AS latest_vs_datetime \
        FROM ",hosxp,".ovst \
            LEFT JOIN ",kphis,".opd_er_order_master om ON om.vn=ovst.vn \
            LEFT JOIN ",kphis,".ipd_pre_admit_master ipm ON ipm.vn=ovst.vn AND ipm.an IS NULL \
            LEFT JOIN ",hosxp,".vn_stat ON vn_stat.vn=ovst.vn \
            LEFT JOIN ",hosxp,".patient p ON p.hn=ovst.hn \
            LEFT JOIN ",hosxp,".pttype ON pttype.pttype=ovst.pttype \
            LEFT JOIN ",hosxp,".sex ON sex.code=p.sex \
            LEFT JOIN ",hosxp,".tambol ON tambol.tambol_code=CONCAT(p.chwpart,p.amppart,p.tmbpart) \
            LEFT JOIN ",hosxp,".occupation ON occupation.occupation=p.occupation \
            LEFT JOIN ",hosxp,".religion ON religion.religion=p.religion \
            LEFT JOIN ",hosxp,".nationality ON nationality.nationality=p.citizenship \
            LEFT JOIN ",hosxp,".nationality AS nationality2 ON nationality2.nationality=p.nationality \
            LEFT JOIN ",hosxp,".marrystatus ON marrystatus.code=p.marrystatus \
        WHERE ",where_key
    ].concat()
}
