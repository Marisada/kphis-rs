use kphis_model::search::ipd_search_patient_other::IpdSearchPatientOtherRequest;

#[derive(Debug, Default)]
pub struct FilterOther {
    pub has_patient: bool,
    pub pt_is_num: bool,
    pub anlen_eq_hnlen: bool,
    pub has_ward: bool,
    pub has_doctor: bool,
    pub has_passcode: bool,
}

impl super::FilterPatient for FilterOther {
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

impl super::FilterWard for FilterOther {
    fn has_ward(self) -> Self {
        Self { has_ward: true, ..self }
    }
}

impl super::FilterDoctor for FilterOther {
    fn has_doctor(self) -> Self {
        Self { has_doctor: true, ..self }
    }
}

impl super::FilterPasscode for FilterOther {
    fn has_passcode(self) -> Self {
        Self { has_passcode: true, ..self }
    }
}


pub fn sql_and_filter(params: IpdSearchPatientOtherRequest, hlen: usize, alen: usize, hosxp: &str, kphis: &str) -> (String, FilterOther) {
    let not_empty = params.not_empty();
    let has_patient = not_empty.patient.is_some();
    let select = select(hosxp, kphis);

    let (where_patient, filter) = match not_empty.patient.as_ref().and_then(|s| urlencoding::decode(s).ok()) {
        Some(patient) => super::where_and_filter_patient(&patient, hlen, alen),
        None => {
            let where_no_pt = super::where_no_patient().to_owned();
            let filter = FilterOther::default();
            let (where_no_pt, filter) = super::where_and_filter_ward(&not_empty.ward, &where_no_pt, filter);
            super::where_and_filter_doctor(&not_empty.doctor_in_charge, kphis, &where_no_pt, filter)
        }
    };
    let (where_passcode, filter) = super::where_and_filter_passcode(&not_empty.passcode, kphis, &where_patient, filter);
    let where_with_order = super::group_by_patient(has_patient, &where_passcode);

    ([select, where_with_order].concat(), filter)
}

// // ipd-other-search-patient-table.php
// SELECT ipt.hn,ipt.an,SUBSTRING(CONCAT(spclty.name,' - ',w.name),1,200) AS sname,w.name AS ward_name,iptadm.bedno,
// CONCAT(patient.pname,patient.fname,' ',patient.lname) AS fullname,aa.income AS income,ptt.pcode AS rtcode,ptt.name AS rtname,
// dt.name AS admdoctor_name,aa.admdate,aa.age_y,aa.age_m,aa.age_d,dc1.name AS dchtype_name,dc2.name AS dchstts_name,di.name AS incharge_doctor_name,
// GROUP_CONCAT(d3.name ORDER BY status DESC,doctor_in_charge_id ASC SEPARATOR ', ') AS kphis_incharge_doctor_name,
// (SELECT MAX(vs_datetime) FROM kphis.ipd_vs_vital_sign vs WHERE vs.an=ipt.an) AS max_vs_datetime,
// (SELECT MAX(CONCAT(fcnote_date,' ',fcnote_time)) FROM kphis.ipd_focus_note fcnote WHERE fcnote.an=ipt.an) AS max_fcnote_datetime,
// (SELECT MAX(CONCAT(order_date,' ',order_time)) FROM kphis.ipd_order WHERE ipd_order.an=ipt.an AND ipd_order.order_confirm='Y' AND ipd_order.order_owner_type='doctor') AS max_order_datetime,
// (SELECT COUNT(*) FROM kphis.ipd_order WHERE ipd_order.an=ipt.an AND ipd_order.order_confirm='Y' AND ipd_order.order_owner_type='doctor'
//     AND ipd_order.nurse_accept_time IS NULL) AS count_nurse_not_accept,
// (SELECT COUNT(distinct ipd_order.order_id) FROM kphis.ipd_order JOIN kphis.ipd_order_item oi ON ipd_order.order_id=oi.order_id AND oi.order_item_type='discharge'
//     WHERE ipd_order.an=ipt.an AND ipd_order.order_confirm ='Y' AND ipd_order.order_owner_type='doctor') AS count_discharge_order,
// IF(wp.ward IS NOT NULL,1,0) AS wp_status
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
// LEFT JOIN kphis.ipd_ward_passcode wp ON wp.ward=ipt.ward
// LEFT JOIN kphis.opd_er_order_master om ON om.vn=ipt.vn
fn select(hosxp: &str, kphis: &str) -> String {
    [
        "SELECT ipt.hn,ipt.an,SUBSTRING(CONCAT(spclty.name,' - ',w.name),1,200) AS sname,w.name AS ward_name,iptadm.bedno,\
            CONCAT(patient.pname,patient.fname,' ',patient.lname) AS fullname,aa.income AS income,ptt.pcode AS rtcode,ptt.name AS rtname,sex.`name` AS sex_name,\
            dt.name AS admdoctor_name,aa.admdate,aa.age_y,aa.age_m,aa.age_d,dc1.name AS dchtype_name,dc2.name AS dchstts_name,di.name AS incharge_doctor_name,\
            GROUP_CONCAT(d3.name ORDER BY status DESC,doctor_in_charge_id ASC SEPARATOR ', ') AS kphis_incharge_doctor_name,\
            (SELECT MAX(vs_datetime) FROM ",kphis,".ipd_vs_vital_sign vs WHERE vs.an=ipt.an) AS max_vs_datetime,\
            (SELECT MAX(ADDTIME(CONVERT(fcnote_date,DATETIME),fcnote_time)) FROM ",kphis,".ipd_focus_note fcnote WHERE fcnote.an=ipt.an) AS max_fcnote_datetime,\
            (SELECT MAX(ADDTIME(CONVERT(order_date,DATETIME),order_time)) FROM ",kphis,".ipd_order WHERE ipd_order.an=ipt.an AND ipd_order.order_confirm='Y' AND ipd_order.order_owner_type='doctor') AS max_order_datetime,\
            IF(wp.ward IS NOT NULL,1,0) AS wp_status,
            (SELECT EXISTS(SELECT * FROM ",kphis,".ipd_order \
                WHERE ipd_order.an=ipt.an AND ipd_order.order_confirm='Y' AND ipd_order.order_owner_type='doctor' AND ipd_order.nurse_accept_time IS NULL)) AS nurse_not_accept_exists,\
            (SELECT EXISTS(SELECT * FROM ",kphis,".ipd_order \
                JOIN ",kphis,".ipd_order_item oi ON ipd_order.order_id=oi.order_id AND oi.order_item_type='discharge' \
                LEFT JOIN ",kphis,".ipd_order_item ooi ON ooi.order_item_id=oi.off_order_item_id \
                WHERE ipd_order.an=ipt.an AND ipd_order.order_confirm='Y' AND ooi.order_item_id IS NULL)) AS discharge_order_exists \
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
            LEFT JOIN ",kphis,".ipd_ward_passcode wp ON wp.ward=ipt.ward \
            LEFT JOIN ",kphis,".opd_er_order_master om ON om.vn=ipt.vn"
    ].concat()
}
