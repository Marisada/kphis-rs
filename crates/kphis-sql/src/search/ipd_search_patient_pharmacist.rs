use kphis_model::search::ipd_search_patient_pharmacist::IpdSearchPatientPharmacistRequest;

#[derive(Debug, Default)]
pub struct FilterPharmacist {
    pub has_patient: bool,
    pub pt_is_num: bool,
    pub anlen_eq_hnlen: bool,
    pub has_ward: bool,
    pub has_doctor: bool,
}

impl super::FilterPatient for FilterPharmacist {
    fn has_patient(self) -> Self {
        Self { has_patient: true, ..self }
    }
    fn pt_is_num(self) -> Self {
        Self { pt_is_num: true, ..self }
    }
    fn anlen_eq_hnlen(self) -> Self {
        Self { anlen_eq_hnlen: true, ..self }
    }
}

impl super::FilterWard for FilterPharmacist {
    fn has_ward(self) -> Self {
        Self { has_ward: true, ..self }
    }
}

impl super::FilterDoctor for FilterPharmacist {
    fn has_doctor(self) -> Self {
        Self { has_doctor: true, ..self }
    }
}


pub fn sql_and_filter(params: IpdSearchPatientPharmacistRequest, hlen: usize, alen: usize, hosxp: &str, kphis: &str, kphis_extra: &str) -> (String, FilterPharmacist) {
    let not_empty = params.not_empty();
    let has_patient = not_empty.patient.is_some();
    let select = select(hosxp, kphis, kphis_extra);

    let (where_patient, filter) = match not_empty.patient.as_ref().and_then(|s| urlencoding::decode(s).ok()) {
        Some(patient) => super::where_and_filter_patient(&patient, hlen, alen),
        None => {
            let where_no_pt = super::where_no_patient().to_owned();
            let filter = FilterPharmacist::default();
            let (where_no_pt, filter) = super::where_and_filter_ward(&not_empty.ward, &where_no_pt, filter);
            let (where_no_pt, filter) = super::where_and_filter_doctor(&not_empty.doctor_in_charge, kphis, &where_no_pt, filter);
            match not_empty.drug_allergy_check {
                Some(drug_allergy_check) => ([where_no_pt, super::where_drug_allergy(&drug_allergy_check, kphis)].concat(), filter),
                None => (where_no_pt, filter),
            }
        }
    };

    let where_with_order = super::group_by_patient(has_patient, &where_patient);

    ([select, where_with_order].concat(), filter)
}

// // ipd-pharmacy-search-patient-table.php
// SELECT ipt.hn,ipt.an,SUBSTRING(CONCAT(spclty.name,' - ',w.name),1,200) AS sname,w.name AS ward_name,iptadm.bedno,
// ipt.regdate,ipt.regtime,CONCAT(ipt.regdate,' ',ipt.regtime) AS regdatetime,
// CONCAT(patient.pname,patient.fname,' ',patient.lname) AS fullname,aa.income AS income,ptt.pcode AS rtcode,ptt.name AS rtname,
// dt.name AS admdoctor_name,aa.admdate,aa.age_y,aa.age_m,aa.age_d,dc1.name AS dchtype_name,dc2.name AS dchstts_name,di.name AS incharge_doctor_name,
// GROUP_CONCAT(d3.name ORDER BY status DESC, doctor_in_charge_id ASC SEPARATOR ', ') AS kphis_incharge_doctor_name,
// dan.admission_note_id,dan.allergy_drug_history,dan.allergy_drug_history_hosxp,dan.allergy_drug_pharmacy_check_person,dan.allergy_drug_pharmacy_check_datetime,
// IF(dan.admission_note_id IS NULL,'ยังไม่มีบันทึกแรกรับ',
//     IF((dan.admission_note_id IS NOT NULL AND dan.allergy_drug_history IS NOT NULL AND dan.allergy_drug_history <> '' AND dan.allergy_drug_pharmacy_check_person IS NULL)
//         OR ((SELECT COUNT(*) FROM kphis.opd_er_allergy_history ah JOIN kphis.opd_er_order_master om ON ah.opd_er_order_master_id=om.opd_er_order_master_id WHERE om.vn=ipt.vn) > 0),'รอประเมิน',
//     IF(dan.allergy_drug_pharmacy_check_person IS NOT NULL AND dan.allergy_drug_pharmacy_check_person <> '','ประเมินแล้ว',''))) AS drug_allergy_check_status,
// d4.name AS allergy_drug_pharmacy_check_person_name
// FROM hos.ipt
// LEFT JOIN hos.spclty ON spclty.spclty=ipt.spclty
// LEFT JOIN hos.iptadm ON iptadm.an=ipt.an
// LEFT JOIN hos.patient ON patient.hn=ipt.hn
// LEFT JOIN hos.doctor dt ON dt.code=ipt.admdoctor
// LEFT JOIN hos.roomno ON roomno.roomno=iptadm.roomno
// LEFT JOIN hos.an_stat aa ON aa.an=ipt.an
// LEFT JOIN hos.ward w ON w.ward=ipt.ward
// LEFT JOIN hos.dchtype dc1 ON dc1.dchtype=ipt.dchtype
// LEFT JOIN hos.dchstts dc2 ON dc2.dchstts=ipt.dchstts
// LEFT JOIN hos.doctor di ON di.code=ipt.incharge_doctor
// LEFT JOIN hos.pttype ptt ON ptt.pttype=ipt.pttype
// LEFT JOIN kphis.ipd_doctor_in_charge ipd_dr ON ipd_dr.an=ipt.an AND ipd_dr.activated='on'
// LEFT JOIN hos.doctor d3 ON d3.`code`=ipd_dr.doctor
// LEFT JOIN kphis.ipd_dr_admission_note dan ON dan.an=ipt.an
// LEFT JOIN kphis.opd_er_order_master om ON om.vn=ipt.vn
// LEFT JOIN hos.doctor d4 ON d4.code=dan.allergy_drug_pharmacy_check_person
fn select(hosxp: &str, kphis: &str, kphis_extra: &str) -> String {
    [
        "SELECT ipt.hn,ipt.an,SUBSTRING(CONCAT(spclty.name,' - ',w.name),1,200) AS sname,w.name AS ward_name,iptadm.bedno,\
            ipt.regdate,ipt.regtime,ADDTIME(CONVERT(ipt.regdate,DATETIME),ipt.regtime) AS regdatetime,\
            CONCAT(patient.pname,patient.fname,' ',patient.lname) AS fullname,aa.income AS income,ptt.pcode AS rtcode,ptt.name AS rtname,sex.`name` AS sex_name,\
            dt.name AS admdoctor_name,aa.admdate,aa.age_y,aa.age_m,aa.age_d,dc1.name AS dchtype_name,dc2.name AS dchstts_name,di.name AS incharge_doctor_name,\
            GROUP_CONCAT(d3.name ORDER BY status DESC, doctor_in_charge_id ASC SEPARATOR ', ') AS kphis_incharge_doctor_name,\
            dan.admission_note_id,dan.allergy_drug_history,dan.allergy_drug_history_hosxp,dan.allergy_drug_pharmacy_check_person,dan.allergy_drug_pharmacy_check_datetime,\
            IF(dan.admission_note_id IS NULL,'ยังไม่มีบันทึกแรกรับ',\
                IF((dan.admission_note_id IS NOT NULL AND dan.allergy_drug_history IS NOT NULL AND dan.allergy_drug_history <> '' AND dan.allergy_drug_pharmacy_check_person IS NULL) \
                    OR ((SELECT COUNT(*) FROM ",kphis,".opd_er_allergy_history ah JOIN ",kphis,".opd_er_order_master om ON ah.opd_er_order_master_id=om.opd_er_order_master_id WHERE om.vn=ipt.vn) > 0),'รอประเมิน',\
                IF(dan.allergy_drug_pharmacy_check_person IS NOT NULL AND dan.allergy_drug_pharmacy_check_person <> '','ประเมินแล้ว',''))) AS drug_allergy_check_status,\
            d4.name AS allergy_drug_pharmacy_check_person_name,\
            (SELECT EXISTS(SELECT * FROM ",kphis,".ipd_med_reconciliation mr WHERE mr.an=ipt.an AND mr.phamacist_confirm_datetime IS NOT NULL AND mr.doctor_confirm_datetime IS NULL GROUP BY mr.an)) AS mr_unconfirmed_exists,\
            (SELECT EXISTS(SELECT * FROM ",kphis,".ipd_med_reconciliation mr WHERE mr.an=ipt.an AND mr.phamacist_confirm_datetime IS NOT NULL AND mr.doctor_confirm_datetime IS NOT NULL GROUP BY mr.an)) AS mr_confirmed_exists,\
            (SELECT EXISTS(SELECT ia.action_id,dud.monitor_status,dud.monitor_count,dud.monitor_duration,COUNT(im.monitor_id) AS m_count,TIMESTAMPDIFF(MINUTE,TIMESTAMP(ia.action_date, ia.action_time),MAX(im.monitor_datetime)) AS m_duration,\
                (SELECT monitor_abnormal FROM ",kphis_extra,".ipd_nurse_index_monitor WHERE action_id=ia.action_id AND monitor_abnormal IS NOT NULL AND monitor_datetime IS NOT NULL ORDER BY monitor_datetime DESC LIMIT 1) AS monitor_abnormal \
                FROM ",kphis,".ipd_nurse_index_action ia \
                    LEFT JOIN ",kphis_extra,".ipd_nurse_index_monitor im ON im.action_id=ia.action_id AND im.monitor_abnormal IS NOT NULL AND im.monitor_datetime IS NOT NULL \
                    LEFT JOIN ",kphis,".ipd_nurse_index_plan ip ON ip.plan_id=ia.plan_id \
                    LEFT JOIN ",kphis,".ipd_order_item oi ON oi.order_item_id=ip.order_item_id \
                    LEFT JOIN ",kphis,".kphis_drug_use_duration dud ON dud.icode=oi.icode \
                WHERE ia.an=ipt.an AND ia.action_date IS NOT NULL AND ia.action_time IS NOT NULL \
                GROUP BY ia.action_id \
                HAVING monitor_abnormal='Y' OR (monitor_status='Y' AND ((monitor_count > 0 AND m_count=0) \
                    OR (monitor_abnormal='N' AND (monitor_count > m_count OR monitor_duration > m_duration)))))) AS need_monitor \
        FROM ",hosxp,".ipt \
            LEFT JOIN ",hosxp,".spclty ON spclty.spclty=ipt.spclty \
            LEFT JOIN ",hosxp,".iptadm ON iptadm.an=ipt.an \
            LEFT JOIN ",hosxp,".patient ON patient.hn=ipt.hn \
            LEFT JOIN ",hosxp,".sex ON sex.code=patient.sex \
            LEFT JOIN ",hosxp,".doctor dt ON dt.code=ipt.admdoctor \
            LEFT JOIN ",hosxp,".roomno ON roomno.roomno=iptadm.roomno \
            LEFT JOIN ",hosxp,".an_stat aa ON aa.an=ipt.an \
            LEFT JOIN ",hosxp,".ward w ON w.ward=ipt.ward \
            LEFT JOIN ",hosxp,".dchtype dc1 ON dc1.dchtype=ipt.dchtype \
            LEFT JOIN ",hosxp,".dchstts dc2 ON dc2.dchstts=ipt.dchstts \
            LEFT JOIN ",hosxp,".doctor di ON di.code=ipt.incharge_doctor \
            LEFT JOIN ",hosxp,".pttype ptt ON ptt.pttype=ipt.pttype \
            LEFT JOIN ",kphis,".ipd_doctor_in_charge ipd_dr ON ipd_dr.an=ipt.an AND ipd_dr.activated='on' \
            LEFT JOIN ",hosxp,".doctor d3 ON d3.`code`=ipd_dr.doctor \
            LEFT JOIN ",kphis,".ipd_dr_admission_note dan ON dan.an=ipt.an \
            LEFT JOIN ",kphis,".opd_er_order_master om ON om.vn=ipt.vn \
            LEFT JOIN ",hosxp,".doctor d4 ON d4.code=dan.allergy_drug_pharmacy_check_person"
    ].concat()
}
