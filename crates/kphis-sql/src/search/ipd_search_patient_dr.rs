use kphis_model::{score::CONCAT_SQL, search::ipd_search_patient_dr::IpdSearchPatientDrRequest};

#[derive(Debug, Default)]
pub struct FilterDr {
    pub has_patient: bool,
    pub pt_is_num: bool,
    pub anlen_eq_hnlen: bool,
    pub has_ward: bool,
    pub has_doctor: bool,
    pub has_consult: bool,
    pub has_passcode: bool,
}

impl super::FilterPatient for FilterDr {
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

impl super::FilterWard for FilterDr {
    fn has_ward(self) -> Self {
        Self { has_ward: true, ..self }
    }
}

impl super::FilterDoctor for FilterDr {
    fn has_doctor(self) -> Self {
        Self { has_doctor: true, ..self }
    }
}

impl super::FilterConsult for FilterDr {
    fn has_consult(self) -> Self {
        Self { has_consult: true, ..self }
    }
}

impl super::FilterPasscode for FilterDr {
    fn has_passcode(self) -> Self {
        Self { has_passcode: true, ..self }
    }
}

pub fn sql_and_filter(params: IpdSearchPatientDrRequest, hlen: usize, alen: usize, hosxp: &str, kphis: &str) -> (String, FilterDr) {
    let not_empty = params.not_empty();
    let has_patient = not_empty.patient.is_some();
    let select = select(hosxp, kphis);

    let (where_patient, filter) = match not_empty.patient.as_ref().and_then(|s| urlencoding::decode(s).ok()) {
        Some(patient) => super::where_and_filter_patient(&patient, hlen, alen),
        None => {
            let where_no_pt = super::where_no_patient().to_owned();
            let filter = FilterDr::default();
            let (where_no_pt, filter) = super::where_and_filter_ward(&not_empty.ward, &where_no_pt, filter);
            let (where_no_pt, filter) = super::where_and_filter_doctor(&not_empty.doctor_in_charge, kphis, &where_no_pt, filter);
            super::where_and_filter_consult(&not_empty.consult_dr_search, kphis, &where_no_pt, filter)
        }
    };
    let (where_passcode, filter) = super::where_and_filter_passcode(&not_empty.passcode, kphis, &where_patient, filter);
    let where_with_order = super::group_by_patient(has_patient, &where_passcode);

    ([select, where_with_order].concat(), filter)
}

// // ipd-dr-search-patient-table.php
// SELECT ipt.hn,ipt.an,SUBSTRING(CONCAT(spclty.name,' - ',w.name),1,200) AS sname,w.name AS ward_name,iptadm.bedno,
// CONCAT(patient.pname,patient.fname,' ',patient.lname) AS fullname,aa.income AS income,ptt.pcode AS rtcode,ptt.name AS rtname,
// dt.name AS admdoctor_name,aa.admdate,aa.age_y,aa.age_m,aa.age_d,dc1.name AS dchtype_name,dc2.name AS dchstts_name,di.name AS incharge_doctor_name,
// GROUP_CONCAT(d3.name SEPARATOR ', ') AS kphis_incharge_doctor_name,
// (SELECT MAX(vs_datetime) FROM kphis.ipd_vs_vital_sign vs WHERE vs.an=ipt.an) AS max_vs_datetime,
// (SELECT MAX(CONCAT(fcnote_date,' ',fcnote_time)) FROM kphis.ipd_focus_note fcnote WHERE fcnote.an=ipt.an) AS max_fcnote_datetime,
// (SELECT MAX(CONCAT(order_date,' ',order_time)) FROM kphis.ipd_order WHERE ipd_order.an=ipt.an AND ipd_order.order_confirm='Y' AND ipd_order.order_owner_type='doctor') AS max_order_datetime,
// (SELECT GROUP_CONCAT(CONCAT(doctor_reply.`name`,', ',doctor_reply2.`name`) ORDER BY idc_reply.consult_reply_id ASC SEPARATOR '|')
//     FROM kphis.ipd_dr_consult idc
//         LEFT JOIN kphis.ipd_dr_consult_signature_reply idc_reply ON idc_reply.consult_id=idc.consult_id
//         LEFT JOIN hos.doctor doctor_reply ON doctor_reply.`code`=idc_reply.consult_doctorcode_reply
//         LEFT JOIN hos.doctor doctor_reply2 ON doctor_reply2.`code`=idc_reply.consult_doctorcode_reply_person2
//     WHERE idc.an=ipt.an) AS consult_reply_name,
// IF(wp.ward IS NOT NULL,1,0) AS wp_status,
// (SELECT EXISTS(SELECT * FROM kphis.ipd_dr_admission_note_item dani WHERE dani.admission_note_id=dan.admission_note_id)) AS dr_admission_note_exists,
// (SELECT EXISTS(SELECT * FROM kphis.ipd_summary_attending_doctor sad WHERE sad.summary_id=summary_2.summary_id)) AS summary_2_attending_doctor_exists,
// (SELECT EXISTS(SELECT * FROM hos.lab_head WHERE vn=ipt.an AND report_date IS NULL AND confirm_report<>'Y')) AS lab_unreported_exists,
// (SELECT EXISTS(SELECT * FROM hos.lab_head h LEFT JOIN kphis.ipd_lab_read lr ON lr.lab_order_number=h.lab_order_number WHERE h.vn=ipt.an AND h.confirm_report='Y' AND lr.lab_order_number IS NULL)) AS lab_unreaded_exists,
// (SELECT EXISTS(SELECT * FROM hos.xray_report xr LEFT JOIN kphis.ipd_xray_read r ON r.xn=xr.xn WHERE xr.an=ipt.an AND xr.confirm='Y' AND r.xn IS NULL)) AS xray_unreaded_exists,
// (SELECT EXISTS(SELECT * FROM kphis.ipd_med_reconciliation mr WHERE mr.an=ipt.an AND mr.phamacist_confirm_datetime IS NOT NULL AND mr.doctor_confirm_datetime IS NULL)) AS mr_unconfirmed_exists,
// (SELECT EXISTS(SELECT * FROM kphis.ipd_med_reconciliation mr WHERE mr.an=ipt.an AND mr.phamacist_confirm_datetime IS NOT NULL AND mr.doctor_confirm_datetime IS NOT NULL)) AS mr_confirmed_exists
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
// LEFT JOIN kphis.ipd_dr_admission_note dan ON dan.an=ipt.an
// LEFT JOIN kphis.ipd_summary summary ON summary.an=ipt.an
// LEFT JOIN kphis.ipd_summary_2 summary_2 ON summary_2.an=ipt.an
fn select(hosxp: &str, kphis: &str) -> String {
    [
        "SELECT ipt.hn,ipt.an,SUBSTRING(CONCAT(spclty.name,' - ',w.name),1,200) AS sname,w.name AS ward_name,iptadm.bedno,\
            ipt.regdate,ipt.regtime,ADDTIME(CONVERT(ipt.regdate,DATETIME),ipt.regtime) AS regdatetime,\
            CONCAT(patient.pname,patient.fname,' ',patient.lname) AS fullname,patient.birthday,aa.income AS income,ptt.pcode AS rtcode,ptt.name AS rtname,sex.`name` AS sex_name,\
            dt.name AS admdoctor_name,aa.admdate,aa.age_y,aa.age_m,aa.age_d,dc1.name AS dchtype_name,dc2.name AS dchstts_name,di.name AS incharge_doctor_name,\
            GROUP_CONCAT(d3.name SEPARATOR ', ') AS kphis_incharge_doctor_name,\
            (SELECT fcnote_patient_type FROM ",kphis,".ipd_focus_note fcnote WHERE fcnote.an=ipt.an ORDER BY fcnote.fcnote_date DESC,fcnote.fcnote_time DESC LIMIT 1) AS max_fcnote_patient_type,\
            (SELECT MAX(ADDTIME(CONVERT(order_date,DATETIME),order_time)) FROM ",kphis,".ipd_order WHERE ipd_order.an=ipt.an AND ipd_order.order_confirm='Y' AND ipd_order.order_owner_type='doctor') AS max_order_datetime,\
            (SELECT GROUP_CONCAT(CONCAT(doctor_reply.`name`,', ',doctor_reply2.`name`) ORDER BY idc_reply.consult_reply_id ASC SEPARATOR '|') \
                FROM ",kphis,".ipd_dr_consult idc \
                    LEFT JOIN ",kphis,".ipd_dr_consult_signature_reply idc_reply ON idc_reply.consult_id=idc.consult_id \
                    LEFT JOIN ",hosxp,".doctor doctor_reply ON doctor_reply.`code`=idc_reply.consult_doctorcode_reply \
                    LEFT JOIN ",hosxp,".doctor doctor_reply2 ON doctor_reply2.`code`=idc_reply.consult_doctorcode_reply_person2 \
                WHERE idc.an=ipt.an) AS consult_reply_name,\
            (SELECT ",CONCAT_SQL," FROM ",kphis,".ipd_vs_vital_sign WHERE an=ipt.an ORDER BY vs_datetime DESC LIMIT 1) AS ews_concat,\
            IF(wp.ward IS NOT NULL,1,0) AS wp_status,\
            (SELECT EXISTS(SELECT * FROM ",kphis,".ipd_dr_admission_note_item dani WHERE dani.admission_note_id=dan.admission_note_id)) AS dr_admission_note_exists,\
            (SELECT EXISTS(SELECT * FROM ",kphis,".ipd_summary_attending_doctor sad WHERE sad.summary_id=summary_2.summary_id)) AS summary_2_attending_doctor_exists,\
            (SELECT EXISTS(SELECT * FROM ",hosxp,".lab_head WHERE vn=ipt.an AND report_date IS NULL AND confirm_report<>'Y')) AS lab_unreported_exists,\
            (SELECT EXISTS(SELECT * FROM ",hosxp,".lab_head h LEFT JOIN ",kphis,".ipd_lab_read lr ON lr.lab_order_number=h.lab_order_number WHERE h.vn=ipt.an AND h.confirm_report='Y' AND lr.lab_order_number IS NULL)) AS lab_unreaded_exists,\
            (SELECT EXISTS(SELECT * FROM ",hosxp,".xray_report xr LEFT JOIN ",kphis,".ipd_xray_read r ON r.xn=xr.xn WHERE xr.an=ipt.an AND xr.confirm='Y' AND r.xn IS NULL)) AS xray_unreaded_exists,\
            (SELECT EXISTS(SELECT * FROM ",kphis,".ipd_med_reconciliation mr WHERE mr.an=ipt.an AND mr.phamacist_confirm_datetime IS NOT NULL AND mr.doctor_confirm_datetime IS NULL)) AS mr_unconfirmed_exists,\
            (SELECT EXISTS(SELECT * FROM ",kphis,".ipd_med_reconciliation mr WHERE mr.an=ipt.an AND mr.phamacist_confirm_datetime IS NOT NULL AND mr.doctor_confirm_datetime IS NOT NULL)) AS mr_confirmed_exists \
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
            LEFT JOIN ",kphis,".opd_er_order_master om ON om.vn=ipt.vn \
            LEFT JOIN ",kphis,".ipd_dr_admission_note dan ON dan.an=ipt.an \
            LEFT JOIN ",kphis,".ipd_summary_2 summary_2 ON summary_2.an=ipt.an"
    ].concat()
}
