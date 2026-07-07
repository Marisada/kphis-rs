// // original version
// SELECT ipt.an,ipt.vn,patient.cid,patient.passport_no,patient.hn,patient.pname,patient.fname,patient.lname,patient.birthday,patient.hometel,patient.worktel,patient.workaddr,patient.informtel,patient.informaddr,patient.informname,patient.informrelation,
//     (SELECT GROUP_CONCAT(CONCAT(opd_allergy.agent,'=',IF(opd_allergy.symptom IS NULL,'',opd_allergy.symptom))) AS name
//         FROM hos.opd_allergy WHERE opd_allergy.hn = ipt.hn ORDER BY display_order) AS drugallergy,
//     (SELECT GROUP_CONCAT(CONCAT(er_allergy_history_agent,'=',IF(er_allergy_history_symptom IS NULL,'',er_allergy_history_symptom))) AS name
//         FROM kphis.opd_er_allergy_history WHERE opd_er_allergy_history.opd_er_order_master_id = om.opd_er_order_master_id ORDER BY er_allergy_history_id) AS er_drugallergy_history,
//     dan.admission_note_id,dan.allergy_drug_history,dan.allergy_drug_history_hosxp,dan.allergy_drug_pharmacy_check_person,dan.allergy_drug_pharmacy_check_datetime,an_stat.sex,an_stat.age_y,an_stat.age_m,an_stat.age_d,an_stat.income,
//     ipt.regdate,ipt.regtime,ipt.dchdate,ipt.dchtime,ipt.dchstts,dchstts.`name` AS dchstts_name,ipt.ward,ward.name AS ward_name,spclty.name AS spclty_name,ipt.pttype,pttype.`name` AS pttype_name,iptadm.bedno,
//     (SELECT vs.bw FROM kphis.ipd_vs_vital_sign vs WHERE vs.an = ipt.an AND vs.bw IS NOT NULL AND trim(vs.bw) <> '' ORDER BY vs_datetime DESC LIMIT 1) AS latest_bw,
//     (SELECT vs.vs_datetime FROM kphis.ipd_vs_vital_sign vs WHERE vs.an = ipt.an AND vs.bw IS NOT NULL AND trim(vs.bw) <> '' ORDER BY vs_datetime DESC LIMIT 1) AS latest_bw_datetime
// FROM hos.ipt
//     LEFT JOIN hos.an_stat ON an_stat.an = ipt.an
//     LEFT JOIN hos.patient ON patient.hn = ipt.hn
//     LEFT JOIN hos.ward ON ward.ward = ipt.ward
//     LEFT JOIN hos.spclty ON spclty.spclty = ipt.spclty
//     LEFT JOIN hos.pttype ON pttype.pttype = ipt.pttype
//     LEFT JOIN hos.iptadm ON iptadm.an = ipt.an
//     LEFT JOIN hos.dchstts ON dchstts.dchstts = ipt.dchstts
//     LEFT JOIN kphis.ipd_dr_admission_note dan ON dan.an = ipt.an
//     LEFT JOIN kphis.opd_er_order_master om ON om.vn = ipt.vn
// WHERE ipt.an = '670000800' ORDER BY ipt.an;
/// an
// pub fn select_show_patient_main(hosxp: &str, kphis: &str) -> String {
//     [
//         "SELECT ipt.an,ipt.vn,patient.cid,patient.passport_no,patient.hn,patient.pname,patient.fname,patient.lname,patient.birthday,patient.hometel,patient.worktel,patient.workaddr,patient.informtel,patient.informaddr,patient.informname,patient.informrelation,\
//             (SELECT GROUP_CONCAT(CONCAT(opd_allergy.agent,'=',IF(opd_allergy.symptom IS NULL,'',opd_allergy.symptom))) AS name \
//                 FROM ",hosxp,".opd_allergy WHERE opd_allergy.hn = ipt.hn ORDER BY display_order) AS drugallergy,\
//             (SELECT GROUP_CONCAT(CONCAT(er_allergy_history_agent,'=',IF(er_allergy_history_symptom IS NULL,'',er_allergy_history_symptom))) AS name \
//                 FROM ",kphis,".opd_er_allergy_history WHERE opd_er_allergy_history.opd_er_order_master_id = om.opd_er_order_master_id ORDER BY er_allergy_history_id) AS er_drugallergy_history,\
//             dan.admission_note_id,dan.allergy_drug_history,dan.allergy_drug_history_hosxp,dan.allergy_drug_pharmacy_check_person,dan.allergy_drug_pharmacy_check_datetime,an_stat.sex,an_stat.age_y,an_stat.age_m,an_stat.age_d,an_stat.income,\
//             ipt.regdate,ipt.regtime,ipt.dchdate,ipt.dchtime,ipt.dchstts,dchstts.`name` AS dchstts_name,ipt.ward,ward.name AS ward_name,spclty.name AS spclty_name,ipt.pttype,pttype.`name` AS pttype_name,iptadm.bedno,\
//             (SELECT vs.bw FROM ",kphis,".ipd_vs_vital_sign vs WHERE vs.an=ipt.an AND vs.bw IS NOT NULL AND trim(vs.bw) <> '' ORDER BY vs_datetime DESC LIMIT 1) AS latest_bw,\
//             (SELECT vs.vs_datetime FROM ",kphis,".ipd_vs_vital_sign vs WHERE vs.an=ipt.an AND vs.bw IS NOT NULL AND trim(vs.bw) <> '' ORDER BY vs_datetime DESC LIMIT 1) AS latest_bw_datetime \
//         FROM ",hosxp,".ipt \
//             LEFT JOIN ",hosxp,".an_stat ON an_stat.an=ipt.an \
//             LEFT JOIN ",hosxp,".patient ON patient.hn=ipt.hn \
//             LEFT JOIN ",hosxp,".ward ON ward.ward=ipt.ward \
//             LEFT JOIN ",hosxp,".spclty ON spclty.spclty=ipt.spclty \
//             LEFT JOIN ",hosxp,".pttype ON pttype.pttype=ipt.pttype \
//             LEFT JOIN ",hosxp,".iptadm ON iptadm.an=ipt.an \
//             LEFT JOIN ",hosxp,".dchstts ON dchstts.dchstts=ipt.dchstts \
//             LEFT JOIN ",kphis,".ipd_dr_admission_note dan ON dan.an=ipt.an \
//             LEFT JOIN ",kphis,".opd_er_order_master om ON om.vn=ipt.vn \
//         WHERE ipt.an=? ORDER BY ipt.an;"
//     )
// }
// // add Summary required data and calculate age_y,age_m_age_d as Summary do
// SELECT ipt.an,ipt.vn,patient.cid,patient.passport_no,patient.hn,patient.pname,patient.fname,patient.lname,patient.birthday,patient.sex,
//     CONCAT_WS(' ',p.addrpart,'หมู่',p.moopart,tambol.full_address_name) AS homeaddr,
//     patient.hometel,patient.worktel,patient.workaddr,patient.informtel,patient.informaddr,patient.informname,patient.informrelation,
//     (SELECT GROUP_CONCAT(CONCAT(opd_allergy.agent,'=',IF(opd_allergy.symptom IS NULL,'',opd_allergy.symptom))) AS name
//         FROM hos.opd_allergy WHERE opd_allergy.hn = ipt.hn ORDER BY display_order) AS drugallergy,
//     (SELECT GROUP_CONCAT(CONCAT(er_allergy_history_agent,'=',IF(er_allergy_history_symptom IS NULL,'',er_allergy_history_symptom))) AS name
//         FROM kphis.opd_er_allergy_history WHERE opd_er_allergy_history.opd_er_order_master_id = om.opd_er_order_master_id ORDER BY er_allergy_history_id) AS er_drugallergy_history,
//     dan.allergy_drug_history,dan.allergy_drug_history_hosxp,dan.allergy_drug_pharmacy_check_person,dan.allergy_drug_pharmacy_check_datetime,
//     timestampdiff(year,patient.birthday,ipt.regdate) AS age_y,
//     timestampdiff(month,patient.birthday,ipt.regdate)-(timestampdiff(year,patient.birthday,ipt.regdate)*12) AS age_m,
//     timestampdiff(day,date_add(patient.birthday,interval (timestampdiff(month,patient.birthday,ipt.regdate)) month),ipt.regdate) AS age_d,
//     iptadm.bedno,an_stat.income,an_stat.admdate,ovst.vstdate,ovst.vsttime,ipt_newborn.birth_weight,ipt.gravidity,ipt.parity,ipt.living_children,
//     ipt.bw,ipt.regdate,ipt.regtime,ipt.dchdate,ipt.dchtime,ipt.dchstts,dchstts.`name` AS dchstts_name,ipt.dchtype,dchtype.`name` AS dchtype_name,ipt.ward,ward.name AS ward_name,
//     ipt.spclty,spclty.name AS spclty_name,ipt.pttype,pttype.`name` AS pttype_name,ipt.leave_home_day,sex.`name` AS sex_name,occupation.`name` AS occupation_name,
//     religion.`name` AS religion_name,nationality.`name` AS citizenship_name,nationality2.`name` AS nationality_name,marrystatus.`name` AS marrystatus_name,
//     (SELECT vs.height FROM kphis.ipd_vs_vital_sign vs WHERE vs.an=ipt.an AND vs.height IS NOT NULL AND TRIM(vs.height) <> '' ORDER BY vs_datetime DESC LIMIT 1) AS latest_height,
//     (SELECT vs.bw FROM kphis.ipd_vs_vital_sign vs WHERE vs.an=ipt.an AND vs.bw IS NOT NULL AND TRIM(vs.bw) <> '' ORDER BY vs_datetime DESC LIMIT 1) AS latest_bw,
//     (SELECT vs.vs_datetime FROM kphis.ipd_vs_vital_sign vs WHERE vs.an=ipt.an AND vs.bw IS NOT NULL AND TRIM(vs.bw) <> '' ORDER BY vs_datetime DESC LIMIT 1) AS latest_bw_datetime,
//     (SELECT vs.vs_datetime FROM kphis.ipd_vs_vital_sign vs WHERE vs.an=ipt.an AND vs.vs_id IS NOT NULL ORDER BY vs_datetime DESC LIMIT 1) AS latest_vs_datetime
// FROM hos.ipt
//     LEFT JOIN hos.ovst ON ovst.vn=ipt.vn
//     LEFT JOIN hos.an_stat ON an_stat.an=ipt.an
//     LEFT JOIN hos.patient ON patient.hn=ipt.hn
//     LEFT JOIN hos.ward ON ward.ward=ipt.ward
//     LEFT JOIN hos.spclty ON spclty.spclty=ipt.spclty
//     LEFT JOIN hos.pttype ON pttype.pttype=ipt.pttype
//     LEFT JOIN hos.sex ON sex.code=patient.sex
//     LEFT JOIN hos.tambol ON tambol.tambol_code=CONCAT(p.chwpart,p.amppart,p.tmbpart)
//     LEFT JOIN hos.occupation ON occupation.occupation=patient.occupation
//     LEFT JOIN hos.religion ON religion.religion=patient.religion
//     LEFT JOIN hos.nationality ON nationality.nationality=patient.citizenship
//     LEFT JOIN hos.nationality AS nationality2 ON nationality2.nationality=patient.nationality
//     LEFT JOIN hos.marrystatus ON marrystatus.code=patient.marrystatus
//     LEFT JOIN hos.ipt_newborn ON ipt_newborn.an=ipt.an
//     LEFT JOIN hos.iptadm ON iptadm.an=ipt.an
//     LEFT JOIN hos.dchstts ON dchstts.dchstts=ipt.dchstts
//     LEFT JOIN hos.dchtype ON dchtype.dchtype=ipt.dchtype
//     LEFT JOIN kphis.ipd_dr_admission_note dan ON dan.an=ipt.an
//     LEFT JOIN kphis.opd_er_order_master om ON om.vn=ipt.vn
// WHERE ipt.an=?;
/// an
pub fn select_show_patient_main(hosxp: &str, kphis: &str) -> String {
    [
        "SELECT ipt.an,ipt.vn,p.cid,p.passport_no,p.hn,p.pname,p.fname,p.lname,p.birthday,p.sex,\
            CONCAT_WS(' ',p.addrpart,'หมู่',p.moopart,tambol.full_address_name) AS homeaddr,\
            p.hometel,p.worktel,p.workaddr,p.informtel,p.informaddr,p.informname,p.informrelation,\
            (SELECT GROUP_CONCAT(CONCAT(opd_allergy.agent,'=',IF(opd_allergy.symptom IS NULL,'',opd_allergy.symptom))) AS name \
                FROM ",hosxp,".opd_allergy WHERE opd_allergy.hn = ipt.hn ORDER BY display_order) AS drugallergy,\
            (SELECT GROUP_CONCAT(CONCAT(er_allergy_history_agent,'=',IF(er_allergy_history_symptom IS NULL,'',er_allergy_history_symptom))) AS name \
                FROM ",kphis,".opd_er_allergy_history WHERE opd_er_allergy_history.opd_er_order_master_id = om.opd_er_order_master_id ORDER BY er_allergy_history_id) AS er_drugallergy_history,\
            dan.allergy_drug_history,dan.allergy_drug_history_hosxp,dan.allergy_drug_pharmacy_check_person,dan.allergy_drug_pharmacy_check_datetime,\
            dan.chief_complaints,dan.g,dan.p,dan.last_child,dan.lmp,dan.edc,dan.gestational_age,dan.gestational_day,dan.mem_ruptured_hours,\
            timestampdiff(year,p.birthday,ipt.regdate) AS age_y,\
            timestampdiff(month,p.birthday,ipt.regdate)-(timestampdiff(year,p.birthday,ipt.regdate)*12) AS age_m,\
            timestampdiff(day,date_add(p.birthday,interval (timestampdiff(month,p.birthday,ipt.regdate)) month),ipt.regdate) AS age_d,\
            iptadm.bedno,an_stat.income,an_stat.admdate,ovst.vstdate,ovst.vsttime,ipt_newborn.birth_weight,\
            ipt.bw,ipt.regdate,ipt.regtime,ipt.dchdate,ipt.dchtime,ipt.dchstts,dchstts.`name` AS dchstts_name,ipt.dchtype,dchtype.`name` AS dchtype_name,ipt.ward,ward.name AS ward_name,\
            ipt.spclty,spclty.name AS spclty_name,ipt.pttype,pttype.`name` AS pttype_name,ipt.leave_home_day,sex.`name` AS sex_name,occupation.`name` AS occupation_name,\
            religion.`name` AS religion_name,nationality.`name` AS citizenship_name,nationality2.`name` AS nationality_name,marrystatus.`name` AS marrystatus_name,\
            (SELECT vs.height FROM ",kphis,".ipd_vs_vital_sign vs WHERE vs.an=ipt.an AND vs.height IS NOT NULL AND TRIM(vs.height) <> '' ORDER BY vs_datetime DESC LIMIT 1) AS latest_height,\
            (SELECT vs.bw FROM ",kphis,".ipd_vs_vital_sign vs WHERE vs.an=ipt.an AND vs.bw IS NOT NULL AND TRIM(vs.bw) <> '' ORDER BY vs_datetime DESC LIMIT 1) AS latest_bw,\
            (SELECT vs.vs_datetime FROM ",kphis,".ipd_vs_vital_sign vs WHERE vs.an=ipt.an AND vs.bw IS NOT NULL AND TRIM(vs.bw) <> '' ORDER BY vs_datetime DESC LIMIT 1) AS latest_bw_datetime,\
            (SELECT vs.vs_datetime FROM ",kphis,".ipd_vs_vital_sign vs WHERE vs.an=ipt.an AND vs.vs_id IS NOT NULL ORDER BY vs_datetime DESC LIMIT 1) AS latest_vs_datetime \
        FROM ",hosxp,".ipt \
            LEFT JOIN ",hosxp,".ovst ON ovst.vn=ipt.vn \
            LEFT JOIN ",hosxp,".an_stat ON an_stat.an=ipt.an \
            LEFT JOIN ",hosxp,".patient p ON p.hn=ipt.hn \
            LEFT JOIN ",hosxp,".ward ON ward.ward=ipt.ward \
            LEFT JOIN ",hosxp,".spclty ON spclty.spclty=ipt.spclty \
            LEFT JOIN ",hosxp,".pttype ON pttype.pttype=ipt.pttype \
            LEFT JOIN ",hosxp,".sex ON sex.code=p.sex \
            LEFT JOIN ",hosxp,".tambol ON tambol.tambol_code=CONCAT(p.chwpart,p.amppart,p.tmbpart) \
            LEFT JOIN ",hosxp,".occupation ON occupation.occupation=p.occupation \
            LEFT JOIN ",hosxp,".religion ON religion.religion=p.religion \
            LEFT JOIN ",hosxp,".nationality ON nationality.nationality=p.citizenship \
            LEFT JOIN ",hosxp,".nationality AS nationality2 ON nationality2.nationality=p.nationality \
            LEFT JOIN ",hosxp,".marrystatus ON marrystatus.code=p.marrystatus \
            LEFT JOIN ",hosxp,".ipt_newborn ON ipt_newborn.an=ipt.an \
            LEFT JOIN ",hosxp,".iptadm ON iptadm.an=ipt.an \
            LEFT JOIN ",hosxp,".dchstts ON dchstts.dchstts=ipt.dchstts \
            LEFT JOIN ",hosxp,".dchtype ON dchtype.dchtype=ipt.dchtype \
            LEFT JOIN ",kphis,".ipd_dr_admission_note dan ON dan.an=ipt.an \
            LEFT JOIN ",kphis,".opd_er_order_master om ON om.vn=ipt.vn \
        WHERE ipt.an=?;"
    ].concat()
}
