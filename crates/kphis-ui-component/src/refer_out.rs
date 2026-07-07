use dominator::{Dom, EventOptions, clone, events, html};
use futures_signals::{
    map_ref,
    signal::{Mutable, Signal, SignalExt, always, not},
    signal_vec::{MutableVec, SignalVecExt},
};
use std::{collections::BTreeMap, rc::Rc};
use time::{Duration, PrimitiveDateTime};
use web_sys::{HtmlButtonElement, HtmlInputElement, HtmlSelectElement, HtmlTextAreaElement};

use kphis_model::{
    app::VisitTypeId,
    endpoint::EndPoint,
    fetch::Method,
    ipd::admission_note_dr::IpdAdmissionNoteDrRaw,
    lab::{LabHead, LabHeadParams},
    med_reconcile::{MedReconciliation, MedReconciliationParams},
    order::{OrderItem, OrderParams},
    patient_info::PatientInfo,
    refer_note::{ReferNote, ReferNoteSave},
    refer_out::{HisReferOutData, HisReferOutSave},
    report::{SystemReport, TypstReport},
    search::searchbox::HospSearchBox,
};
use kphis_ui_app::App;
use kphis_ui_core::{class, doms, mixins};
use kphis_util::{
    datetime::{JsTime, date_8601, date_th, date_th_opt, datetime_from_opt, datetime_th, datetime_th_opt, js_now, time_8601},
    util::{decimal_rescale, explode, sanity_tis620, str_some, zero_none},
};

use crate::{
    gadget::{pdf_button::PdfButtons, searchbox::hosp::HospSearchboxCpn},
    lab,
    modal::{blank_modal, lab_selector::LabSelector, vs_selector::VsSelector},
};

#[derive(Clone, Default, PartialEq)]
enum Tab {
    #[default]
    ReferOut,
    ReferNote,
}

/// - GET `EndPoint::HisReferOutVnan`
/// - POST `EndPoint::HisReferOutVnan` (guarded, hide 'บันทึก' btn)
/// - GET `EndPoint::ReferNoteVnan`
/// - POST `EndPoint::ReferNoteVnan` (guarded, hide 'บันทึก' btn)
/// - GET `EndPoint::IpdAdmissionNoteDrAn` (guarded, hide 'คัดลอกจากบันทึกแรกรับผู้ป่วยใน' btn)
/// - GET `EndPoint::IpdOrderPrevious` (guarded, hide 'Rx' btn)
/// - GET `EndPoint::IpdMedReconcile` (guarded, not call API)
/// - GET `EndPoint::OpdErMedReconcile` (guarded, not call API)
/// - GET `EndPoint::LabHead` (guarded, hide 'Lab' btn)
/// - GET `EndPoint::IpdVitalSign` (VsSelector, guarded, hide `VS` btn)
/// - GET `EndPoint::OpdErVitalSign` (VsSelector, guarded, hide `VS` btn)
#[derive(Default)]
pub struct ReferOutCpn {
    active_tab: Mutable<Tab>,
    changed: Mutable<bool>,
    checked: Mutable<bool>,

    patient: Mutable<Option<Rc<PatientInfo>>>,

    referouts: MutableVec<Rc<HisReferOutData>>,
    refernotes: MutableVec<Rc<ReferNote>>,

    loaded: Mutable<bool>,
    reload_referout: Mutable<bool>,
    reload_refernote: Mutable<bool>,

    referout_id: Mutable<i32>,
    refernote_id: Mutable<u32>,
    docno: Mutable<String>,

    hospital_refer: Mutable<Option<Rc<HospSearchBox>>>,
    refer_date: Mutable<String>,
    refer_time: Mutable<String>,
    // due_date: Mutable<String>,
    expire_date: Mutable<String>,
    pre_diagnosis: Mutable<String>,
    // diagnosis_text: Mutable<String>,
    pmh: Mutable<String>,
    hpi: Mutable<String>,
    lab_text: Mutable<String>,
    treatment_text: Mutable<String>,
    other_text: Mutable<String>,
    request_text: Mutable<String>,

    department: Mutable<String>,
    refer_type: Mutable<String>,
    refer_cause: Mutable<String>,
    refer_point: Mutable<String>,
    moph_refer_expire_type_id: Mutable<String>,

    refer_vital_sign_id: Mutable<i32>,
    cc: Mutable<String>,
    pe: Mutable<String>,

    vs_selector_modal: Mutable<Option<Rc<VsSelector>>>,
    lab_selector_modal: Mutable<Option<Rc<LabSelector>>>,
}

impl ReferOutCpn {
    pub fn new(patient: Mutable<Option<Rc<PatientInfo>>>) -> Rc<Self> {
        Rc::new(Self { patient, ..Default::default() })
    }

    fn is_refernote_not_ready_signal(&self) -> impl Signal<Item = bool> + use<> {
        map_ref! {
            let is_changed = self.changed.signal(),
            let has_refer_date = self.refer_date.signal_ref(|s| !s.is_empty()),
            let has_diagnosis = self.pre_diagnosis.signal_ref(|s| !s.is_empty()) =>
            !is_changed || !has_refer_date || !has_diagnosis
        }
    }

    fn is_referout_not_ready_signal(&self) -> impl Signal<Item = bool> + use<> {
        map_ref! {
            let is_changed = self.changed.signal(),
            let has_hospital = self.hospital_refer.signal_ref(|opt| opt.is_some()),
            let has_refer_date = self.refer_date.signal_ref(|s| !s.is_empty()),
            let has_exp_date = self.expire_date.signal_ref(|s| !s.is_empty()),
            let has_diagnosis = self.pre_diagnosis.signal_ref(|s| !s.is_empty()) =>
            !is_changed || !has_hospital || !has_refer_date || !has_exp_date || !has_diagnosis
        }
    }

    fn load_all(page: Rc<Self>, app: Rc<App>) {
        let vnan_opt = page.patient.lock_ref().as_ref().map(|pt| pt.visit_type.vnan().to_owned());
        if let Some(vnan) = vnan_opt {
            app.async_load(
                true,
                clone!(app => async move {
                    Self::get_referout(true, &vnan, page.clone(), app.clone()).await;
                    Self::get_refernote(false, &vnan, page, app).await;
                }),
            );
        }
    }

    fn load_referout(page: Rc<Self>, app: Rc<App>) {
        let vnan_opt = page.patient.lock_ref().as_ref().map(|pt| pt.visit_type.vnan().to_owned());
        if let Some(vnan) = vnan_opt {
            app.async_load(
                true,
                clone!(app => async move {
                    Self::get_referout(true, &vnan, page, app).await;
                }),
            );
        }
    }

    async fn get_referout(auto_set: bool, vnan: &str, page: Rc<Self>, app: Rc<App>) {
        // GET `EndPoint::HisReferOutVnan`
        match HisReferOutData::call_api_get(vnan, app.state()).await {
            Ok(responses) => {
                let mut lock = page.referouts.lock_mut();
                lock.clear();
                let has_response = !responses.is_empty();
                page.checked.set_neq(has_response);
                if has_response {
                    if auto_set && let Some(first) = responses.first() {
                        page.set_referout(first);
                    }
                    lock.extend(responses.into_iter().map(Rc::new));
                } else {
                    page.set_new();
                }
            }
            Err(e) => {
                app.alert_app_error(&e).await;
            }
        }
    }

    fn load_refernote(page: Rc<Self>, app: Rc<App>) {
        let vnan_opt = page.patient.lock_ref().as_ref().map(|pt| pt.visit_type.vnan().to_owned());
        if let Some(vnan) = vnan_opt {
            app.async_load(
                true,
                clone!(app => async move {
                    Self::get_refernote(true, &vnan, page, app).await;
                }),
            );
        }
    }

    async fn get_refernote(auto_set: bool, vnan: &str, page: Rc<Self>, app: Rc<App>) {
        // GET `EndPoint::ReferNoteVnan`
        match ReferNote::call_api_get(&vnan, app.state()).await {
            Ok(responses) => {
                let mut lock = page.refernotes.lock_mut();
                lock.clear();
                let has_response = !responses.is_empty();
                page.checked.set_neq(has_response);
                if has_response {
                    if auto_set && let Some(first) = responses.first() {
                        page.set_refernote(first);
                    }
                    lock.extend(responses.into_iter().map(Rc::new));
                } else {
                    page.set_new();
                }
            }
            Err(e) => {
                app.alert_app_error(&e).await;
            }
        }
    }

    fn load_admission_note(page: Rc<Self>, app: Rc<App>) {
        if let Some((an, pt_bw, pt_ht)) = page.patient.lock_ref().as_ref().map(|pt| (pt.visit_type.vnan(), pt.latest_bw, pt.latest_height)) {
            app.async_load(
                true,
                clone!(app, page, an => async move {
                    // GET `EndPoint::IpdAdmissionNoteDrAn`
                    match IpdAdmissionNoteDrRaw::call_api_get(&an, app.state()).await {
                        Ok(response) => {
                            if let Some(note) = &response.admission_note {
                                // CC
                                if let Some(cc) = &note.chief_complaints {
                                    page.cc.set(cc.to_owned());
                                }

                                // Physical Examination
                                let mut pe = String::new();
                                if let Some(bt) = note.t {
                                    pe.push_str("T: ");
                                    pe.push_str(&decimal_rescale(bt, 1).to_string());
                                    pe.push_str(" °C");
                                }
                                if let Some(pr) = note.pr {
                                    if !pe.is_empty() {pe.push(',')};
                                    pe.push_str(" P: ");
                                    pe.push_str(&pr.to_string());
                                    pe.push_str(" /min");
                                }
                                if let Some(rr) = note.rr {
                                    if !pe.is_empty() {pe.push(',')};
                                    pe.push_str(" R: ");
                                    pe.push_str(&rr.to_string());
                                    pe.push_str(" /min");
                                }
                                if let Some(bp) = &note.bp {
                                    if !pe.is_empty() {pe.push(',')};
                                    pe.push_str(" BP: ");
                                    pe.push_str(bp);
                                    pe.push_str(" mmHg");
                                }
                                if let (Some(e), Some(v), Some(m)) = (&note.e, &note.v, &note.m) {
                                    if !pe.is_empty() {pe.push(',')};
                                    pe.push_str(" GCS: E");
                                    pe.push_str(&e);
                                    pe.push('V');
                                    pe.push_str(&v);
                                    pe.push('M');
                                    pe.push_str(&m);
                                }
                                if let Some(bw) = pt_bw.map(|pbw| decimal_rescale(pbw, 3).to_string()).or(response.opdscreen_pe.as_ref().and_then(|ope| ope.bw).map(|obw| obw.to_string())) {
                                    if !pe.is_empty() {pe.push(',')};
                                    pe.push_str(" BW: ");
                                    pe.push_str(&bw);
                                    pe.push_str(" Kg");
                                }
                                if let Some(ht) = pt_ht.or(response.opdscreen_pe.as_ref().and_then(|ope| ope.height)).map(|h| h.to_string()) {
                                    if !pe.is_empty() {pe.push(',')};
                                    pe.push_str(" HT: ");
                                    pe.push_str(&ht);
                                    pe.push_str(" cm");
                                }
                                // OB/GYN
                                if let Some(hf_pos) = &note.hf_position {
                                    if !pe.is_empty() {pe.push('\n')};
                                    pe.push_str("- POS: ");
                                    pe.push_str(&hf_pos);
                                }
                                if let Some(hf) = &note.hf {
                                    if !pe.is_empty() {pe.push(',')};
                                    pe.push_str(" Ht of fundus: ");
                                    pe.push_str(&hf.to_string());
                                    pe.push_str(" cm");
                                }
                                if let Some(fhr) = &note.lr_fhr {
                                    if !pe.is_empty() {pe.push(',')};
                                    pe.push_str(" FHR: ");
                                    pe.push_str(&fhr.to_string());
                                    pe.push_str(" /min");
                                }
                                if note.lr_fhr_irrigular.as_ref().map(|s| s == "Y").unwrap_or_default() {
                                    if !pe.is_empty() {pe.push(',')};
                                    pe.push_str(" irregular");
                                }
                                if let Some(efw) = &note.lr_efw {
                                    if !pe.is_empty() {pe.push(',')};
                                    pe.push_str(" Estimate: ");
                                    pe.push_str(&efw.to_string());
                                    pe.push_str(" g");
                                }
                                if let Some(lr_int) = &note.lr_interval {
                                    if !pe.is_empty() {pe.push(',')};
                                    pe.push_str(" I: ");
                                    pe.push_str(lr_int);
                                }
                                if let Some(lr_dur) = &note.lr_duration {
                                    if !pe.is_empty() {pe.push(',')};
                                    pe.push_str(" D: ");
                                    pe.push_str(&lr_dur.to_string());
                                    pe.push('"');
                                }
                                if let Some(lr_intensity) = &note.lr_intensity {
                                    if !pe.is_empty() {pe.push(',')};
                                    pe.push_str(" Sev: ");
                                    pe.push_str(lr_intensity);
                                }
                                if let Some(lr_cx_dil) = &note.lr_cx_dilate {
                                    if !pe.is_empty() {pe.push(',')};
                                    pe.push_str(" Cx: ");
                                    pe.push_str(&lr_cx_dil.to_string());
                                    pe.push_str(" cm");
                                }
                                if let Some(lr_cx_eff) = &note.lr_cx_efface {
                                    if !pe.is_empty() {pe.push(',')};
                                    pe.push_str(" Eff: ");
                                    pe.push_str(&lr_cx_eff.to_string());
                                    pe.push_str(" %");
                                }
                                if let Some(lr_cx_st) = &note.lr_cx_station {
                                    if !pe.is_empty() {pe.push(',')};
                                    pe.push_str(" St: ");
                                    pe.push_str(&lr_cx_st.to_string());
                                }
                                if let Some(lr_cx_pos) = &note.lr_cx_position {
                                    if !pe.is_empty() {pe.push_str(", ")};
                                    pe.push_str(lr_cx_pos);
                                }
                                if let Some(lr_cx_con) = &note.lr_cx_consistency {
                                    if !pe.is_empty() {pe.push_str(", ")};
                                    pe.push_str(lr_cx_con);
                                }
                                if let Some(lr_cx_bis) = &note.lr_cx_bishop {
                                    if !pe.is_empty() {pe.push(',')};
                                    pe.push_str(" Bishop: ");
                                    pe.push_str(&lr_cx_bis.to_string());
                                }
                                if let Some(lr_mem) = &note.lr_membrane {
                                    if !pe.is_empty() {pe.push(',')};
                                    pe.push_str(" Mem: ");
                                    pe.push_str(lr_mem);
                                }
                                if let Some(lr_mem_col) = &note.lr_amniotic_color {
                                    if !pe.is_empty() {pe.push_str(", ")};
                                    pe.push_str(lr_mem_col);
                                }
                                if let Some(lr_mem_sm) = &note.lr_amniotic_smell {
                                    if !pe.is_empty() {pe.push_str(", ")};
                                    pe.push_str(lr_mem_sm);
                                    pe.push_str(" smell");
                                }

                                let old_pe = page.pe.get_cloned();
                                if old_pe.is_empty() {
                                    page.pe.set(pe.to_owned());
                                } else {
                                    page.pe.set([&old_pe, "\n", &pe].concat());
                                }

                                // Present Illness
                                let mut hpi = String::new();
                                // OB/GYN
                                if let Some(g) = note.g {
                                    hpi.push_str("- G");
                                    hpi.push_str(&g.to_string());
                                }
                                if let Some(p) = &note.p {
                                    hpi.push_str("P");
                                    hpi.push_str(p);
                                }
                                if let Some(gestational_age) = &note.gestational_age {
                                    if !hpi.is_empty() {hpi.push(',')};
                                    hpi.push_str(" GA: ");
                                    hpi.push_str(gestational_age);
                                }
                                if let Some(gestational_day) = &note.gestational_day {
                                    hpi.push('+');
                                    hpi.push_str(gestational_day);
                                }
                                if let Some(last_child) = note.last_child {
                                    hpi.push_str(" last: ");
                                    hpi.push_str(&last_child.to_string());
                                    hpi.push_str(" yr");
                                }
                                if let Some(lmp) = &note.lmp {
                                    if !hpi.is_empty() {hpi.push(',')};
                                    hpi.push_str(" LMP: ");
                                    hpi.push_str(&date_th(lmp));
                                }
                                if let Some(edc) = &note.edc {
                                    if !hpi.is_empty() {hpi.push(',')};
                                    hpi.push_str(" EDC: ");
                                    hpi.push_str(&date_th(edc));
                                }
                                // Addict
                                if let Some(addict_assist) = &note.addict_assist {
                                    let subs = explode(addict_assist, 2).iter().map(|chunk| {
                                        [&chunk[0], " = ", chunk[1].split(',').next().unwrap_or_default()].concat()
                                    }).collect::<Vec<String>>().join(", ");
                                    if !hpi.is_empty() {hpi.push('\n')};
                                    hpi.push_str("- V2: ");
                                    hpi.push_str(&subs);
                                }
                                if let Some(concat) = &note.amphetamine_awq {
                                    let mut iter = concat.split(',');
                                    let awq = iter.next();
                                    let awq_h = iter.next();
                                    let awq_a = iter.next();
                                    let awq_r = iter.next();
                                    if let Some(total) = awq {
                                        if !hpi.is_empty() {hpi.push(',')};
                                        hpi.push_str(" AWQv2: ");
                                        hpi.push_str(total);
                                    }
                                    if let (Some(h), Some(a), Some(r)) = (awq_h, awq_a, awq_r) {
                                        hpi.push_str(" (H");
                                        hpi.push_str(h);
                                        hpi.push('A');
                                        hpi.push_str(a);
                                        hpi.push('R');
                                        hpi.push_str(r);
                                        hpi.push(')');
                                    }
                                }
                                if let Some(aggression_oas) = note.aggression_oas.as_ref().and_then(|concat| concat.split(',').nth(0)) {
                                    if !hpi.is_empty() {hpi.push(',')};
                                    hpi.push_str(" OAS: ");
                                    hpi.push_str(&aggression_oas.to_string());
                                }
                                if let Some(motivation_scale) = &note.motivation_scale {
                                    if !hpi.is_empty() {hpi.push(',')};
                                    hpi.push_str(" Motivation: ");
                                    hpi.push_str(&motivation_scale.to_string());
                                }
                                if let Some(craving_scale) = &note.craving_scale {
                                    if !hpi.is_empty() {hpi.push(',')};
                                    hpi.push_str(" Craving: ");
                                    hpi.push_str(&craving_scale.to_string());
                                }
                                if let Some(stage_of_change_name) = &note.stage_of_change_name {
                                    if !hpi.is_empty() {hpi.push(',')};
                                    hpi.push_str(" SOC: ");
                                    hpi.push_str(stage_of_change_name);
                                }
                                if let Some(alcohol_audit) = note.alcohol_audit.as_ref().and_then(|concat| concat.split(',').nth(0)) {
                                    if !hpi.is_empty() {hpi.push(',')};
                                    hpi.push_str(" Alcohol audit: ");
                                    hpi.push_str(&alcohol_audit.to_string());
                                }
                                if let Some(alcohol_aws) = note.alcohol_aws.as_ref().and_then(|concat| concat.split(',').nth(0)) {
                                    if !hpi.is_empty() {hpi.push(',')};
                                    hpi.push_str(" AWS: ");
                                    hpi.push_str(&alcohol_aws.to_string());
                                }
                                if let Some(alcohol_ciwa) = note.alcohol_ciwa.as_ref().and_then(|concat| concat.split(',').nth(0)) {
                                    if !hpi.is_empty() {hpi.push(',')};
                                    hpi.push_str(" CIWA-Ar: ");
                                    hpi.push_str(&alcohol_ciwa.to_string());
                                }
                                if let Some(depress_2q) = note.depress_2q.as_ref().and_then(|concat| concat.split(',').nth(0)) {
                                    if !hpi.is_empty() {hpi.push(',')};
                                    hpi.push_str(" 2Q: ");
                                    hpi.push_str(&depress_2q.to_string());
                                }
                                if let Some(depress_9q) = note.depress_9q.as_ref().and_then(|concat| concat.split(',').nth(0)) {
                                    if !hpi.is_empty() {hpi.push(',')};
                                    hpi.push_str(" 9Q: ");
                                    hpi.push_str(&depress_9q.to_string());
                                }
                                if let Some(suicide_8q) = note.suicide_8q.as_ref().and_then(|concat| concat.split(',').nth(0)) {
                                    if !hpi.is_empty() {hpi.push(',')};
                                    hpi.push_str(" 8Q: ");
                                    hpi.push_str(&suicide_8q.to_string());
                                }
                                if let Some(stress_st5) = note.stress_st5.as_ref().and_then(|concat| concat.split(',').nth(0)) {
                                    if !hpi.is_empty() {hpi.push(',')};
                                    hpi.push_str(" ST-5: ");
                                    hpi.push_str(&stress_st5.to_string());
                                }
                                if let Some(depress_cdi) = note.depress_cdi.as_ref().and_then(|concat| concat.split(',').nth(0)) {
                                    if !hpi.is_empty() {hpi.push(',')};
                                    hpi.push_str(" CDI: ");
                                    hpi.push_str(&depress_cdi.to_string());
                                }
                                if let Some(depress_cesd) = note.depress_cesd.as_ref().and_then(|concat| concat.split(',').nth(0)) {
                                    if !hpi.is_empty() {hpi.push(',')};
                                    hpi.push_str(" CES-D: ");
                                    hpi.push_str(&depress_cesd.to_string());
                                }
                                if let Some(depress_phqa) = note.depress_phqa.as_ref().and_then(|concat| concat.split(',').nth(0)) {
                                    if !hpi.is_empty() {hpi.push(',')};
                                    hpi.push_str(" PHQ-A: ");
                                    hpi.push_str(&depress_phqa.to_string());
                                }
                                if let Some(nicotin_ftnd) = note.nicotin_ftnd.as_ref().and_then(|concat| concat.split(',').nth(0)) {
                                    if !hpi.is_empty() {hpi.push(',')};
                                    hpi.push_str(" FTND: ");
                                    hpi.push_str(&nicotin_ftnd.to_string());
                                }
                                if let Some(ptsd_screen) = note.ptsd_screen.as_ref().and_then(|concat| concat.split(',').nth(0)) {
                                    if !hpi.is_empty() {hpi.push(',')};
                                    hpi.push_str(" PTSD: ");
                                    hpi.push_str(&ptsd_screen.to_string());
                                }
                                if let Some(ptsd_pisces) = note.ptsd_pisces.as_ref().and_then(|concat| concat.split(',').nth(0)) {
                                    if !hpi.is_empty() {hpi.push(',')};
                                    hpi.push_str(" PISCES-10: ");
                                    hpi.push_str(&ptsd_pisces.to_string());
                                }
                                if let Some(ptsd_cries) = note.ptsd_cries.as_ref().and_then(|concat| concat.split(',').nth(0)) {
                                    if !hpi.is_empty() {hpi.push(',')};
                                    hpi.push_str(" CRIES-13: ");
                                    hpi.push_str(&ptsd_cries.to_string());
                                }
                                // HPI from note
                                if let Some(medical_history) = &note.medical_history {
                                    if !hpi.is_empty() {hpi.push('\n')};
                                    hpi.push_str(medical_history);
                                }

                                let old_hpi = page.hpi.get_cloned();
                                if old_hpi.is_empty() {
                                    page.hpi.set(hpi.to_owned());
                                } else {
                                    page.hpi.set([&old_hpi, "\n", &hpi].concat());
                                }

                                // Past Medical History
                                let mut pmh = String::new();
                                if let Some(disease_detail) = &note.disease_detail {
                                    let diseases = explode(disease_detail, 3).iter().map(|chunk| {
                                        ["- ", &chunk[0], " เป็นมา ", &chunk[1], " ปี รักษาที่ ", &chunk[2]].concat()
                                    }).collect::<Vec<String>>().join("\n");
                                    if !pmh.is_empty() {pmh.push('\n')};
                                    pmh.push_str(&diseases);
                                }
                                if let Some(operation) = &note.operation_history {
                                    if !pmh.is_empty() {pmh.push('\n')};
                                    pmh.push_str("- ผ่าตัด ");
                                    pmh.push_str(operation);
                                }
                                if let Some(drug_allergy) = &note.allergy_drug_history {
                                    let drugs = explode(drug_allergy, 2).iter().map(|chunk| {
                                        ["- แพ้ยา ", &chunk[0], " (", &chunk[1], ")"].concat()
                                    }).collect::<Vec<String>>().join("\n");
                                    if !pmh.is_empty() {pmh.push('\n')};
                                    pmh.push_str(&drugs);
                                }
                                if let Some(food_allergy) = &note.allergy_food_history {
                                    let foods = explode(food_allergy, 2).iter().map(|chunk| {
                                        ["- แพ้ ", &chunk[0], " (", &chunk[1], ")"].concat()
                                    }).collect::<Vec<String>>().join("\n");
                                    if !pmh.is_empty() {pmh.push('\n')};
                                    pmh.push_str(&foods);
                                }
                                if let Some(etc_allergy) = &note.allergy_etc_history {
                                    let etcs = explode(etc_allergy, 2).iter().map(|chunk| {
                                        ["- แพ้ ", &chunk[0], " (", &chunk[1], ")"].concat()
                                    }).collect::<Vec<String>>().join("\n");
                                    if !pmh.is_empty() {pmh.push('\n')};
                                    pmh.push_str(&etcs);
                                }

                                let old_pmh = page.pmh.get_cloned();
                                if old_pmh.is_empty() {
                                    page.pmh.set(pmh);
                                } else {
                                    page.pmh.set([&old_pmh, "\n", &pmh].concat());
                                }
                                page.changed.set_neq(true);
                            }
                        }
                        Err(e) => {
                            app.alert_app_error(&e).await;
                        }
                    }
                }),
            )
        }
    }

    fn load_med_rec(page: Rc<Self>, app: Rc<App>) {
        if let Some(patient) = page.patient.get_cloned() {
            let (is_ipd, is_pre_admit) = patient.visit_type.is_ipd_and_is_pre_admit();
            let is_allow = if is_ipd {
                app.endpoint_is_allow(&Method::GET, &EndPoint::IpdMedReconcile, is_pre_admit)
            } else {
                app.endpoint_is_allow(&Method::GET, &EndPoint::OpdErMedReconcile, false)
            };
            if is_allow {
                app.async_load(
                    true,
                    clone!(app => async move {
                        let result_opt = match &patient.visit_type {
                            VisitTypeId::Ipd(an)
                            | VisitTypeId::PreAdmit(an) => {
                                let params = MedReconciliationParams {
                                    hn: patient.hn(),
                                    an: str_some(an.to_owned()),
                                    ..Default::default()
                                };
                                // GET `EndPoint::IpdMedReconcile`
                                Some(MedReconciliation::call_api_get(true, &params, app.state()).await)
                            }
                            VisitTypeId::OpdEr(_, opd_er_order_master_id) => {
                                let params = MedReconciliationParams {
                                    hn: patient.hn(),
                                    opd_er_order_master_id: zero_none(*opd_er_order_master_id),
                                    ..Default::default()
                                };
                                // GET `EndPoint::OpdErMedReconcile`
                                Some(MedReconciliation::call_api_get(false, &params, app.state()).await)
                            }
                            VisitTypeId::Visit(_) => None,
                        };

                        if let Some(result) = result_opt {
                            match result {
                                Ok(responses) => {
                                    let mut med_rec = responses.into_iter().flat_map(|mr| mr.med_reconciliation_items).filter_map(|item| {
                                        item.med_name.as_ref().or(item.custom_med_name.as_ref()).map(|med_name| {
                                            let receive = if item.receive_from.is_some() || item.receive_date.is_some() || item.receive_qty.is_some() {
                                                [" [", &item.receive_from.clone().unwrap_or_default(), " ", &date_th_opt(&item.receive_date), " #", &item.receive_qty.unwrap_or_default().to_string(),"]"].concat()
                                            } else {
                                                String::new()
                                            };
                                            ["- ", med_name, " ", &item.old_drugusage.clone().unwrap_or_default(), &receive].concat()
                                        })
                                    }).collect::<Vec<String>>();
                                    med_rec.sort();
                                    if !med_rec.is_empty() {
                                        let with_header = ["ยาเดิมผู้ป่วย\n", &med_rec.join("\n")].concat();
                                        let old_pmh = page.pmh.get_cloned();
                                        if old_pmh.is_empty() {
                                            page.pmh.set(with_header);
                                        } else {
                                            page.pmh.set([&old_pmh, "\n\n", &with_header].concat());
                                        }
                                        page.changed.set_neq(true);
                                    }
                                }
                                Err(e) => {
                                    app.alert_app_error(&e).await;
                                }
                            }
                        }
                    }),
                );
            }
        }
    }

    fn load_current_treatment(page: Rc<Self>, app: Rc<App>) {
        let vnan_opt = page.patient.lock_ref().as_ref().map(|pt| (pt.visit_type.vnan().to_owned(), pt.is_ipd()));
        if let Some((an, is_ipd)) = vnan_opt
            && is_ipd
        {
            app.async_load(
                true,
                clone!(app, page => async move {
                    let params = OrderParams {
                        an: str_some(an),
                        order_type: Some(String::from("continuous")),
                        current_date: Some(js_now().date()),
                        view_by: Some(String::from("doctor")),
                        with_offed: Some(String::from("Y")),
                        ..Default::default()
                    };
                    // GET `EndPoint::IpdOrderPrevious`
                    match OrderItem::call_api_get_ipd_previous(&params, app.state()).await {
                        Ok(order_items) => {
                            let (med_inj, non_meds): (Vec<OrderItem>, Vec<OrderItem>) = order_items
                                .into_iter()
                                .partition(|item| ["med", "injection"].contains(&item.order_item_type.clone().unwrap_or_default().as_str()));
                            let (injections, meds): (Vec<OrderItem>, Vec<OrderItem>) = med_inj.into_iter().partition(|item| item.order_item_type.clone().unwrap_or_default().as_str() == "injection");
                            let mut result = String::new();
                            for non_med in non_meds {
                                // NOT include OFFed and Note
                                if non_med.off_by_datetime.is_none() && non_med.order_item_type != Some(String::from("note")) && let Some(detail) = &non_med.order_item_detail {
                                    result.push_str("\r\n- ");
                                    result.push_str(detail);
                                }
                            }
                            for injection in injections {
                                // include OFFed
                                if let Some(med_name) = &injection.med_name {
                                    result.push_str("\r\n- ");
                                    result.push_str(med_name);
                                    if let Some(detail) = &injection.order_item_detail {
                                        result.push_str(" : ");
                                        result.push_str(detail);
                                    }
                                    let mut dts = injection.index_plans.iter().flat_map(|plan| plan.actions.iter().filter_map(|action| {
                                        datetime_from_opt(action.action_date, action.action_time)
                                    })).collect::<Vec<PrimitiveDateTime>>();
                                    // Show time range, use order_date/order_time + off_by_datetime
                                    if dts.is_empty() {
                                        let order_datetime = datetime_from_opt(injection.order_date, injection.order_time);
                                        match (order_datetime.is_some(), injection.off_by_datetime.is_some()) {
                                            (true, true) => {
                                                result.push_str(" (order ");
                                                result.push_str(&datetime_th_opt(&order_datetime));
                                                result.push_str(", off ");
                                                result.push_str(&datetime_th_opt(&injection.off_by_datetime));
                                                result.push(')');
                                            }
                                            (true, false) => {
                                                result.push_str(" (order ");
                                                result.push_str(&datetime_th_opt(&order_datetime));
                                                result.push(')');
                                            }
                                            (false, true) => {
                                                result.push_str(" (off ");
                                                result.push_str(&datetime_th_opt(&injection.off_by_datetime));
                                                result.push(')');
                                            }
                                            (false, false) => {}
                                        }
                                    // Show time range, use min/max action
                                    } else {
                                        dts.sort();
                                        if let (Some(min), Some(max)) = (dts.first(), dts.last()) {
                                            result.push_str(" (");
                                            result.push_str(&datetime_th(min));
                                            result.push_str(" - ");
                                            result.push_str(&datetime_th(max));
                                            result.push(')');
                                        }
                                    }
                                }
                            }
                            for med in meds {
                                // NOT include OFFed
                                if med.off_by_datetime.is_none() && let Some(med_name) = &med.med_name {
                                    result.push_str("\r\n- ");
                                    result.push_str(med_name);
                                    if let Some(detail) = &med.order_item_detail {
                                        result.push_str(" : ");
                                        result.push_str(detail);
                                    }
                                }
                            }
                            let treatment_text = page.treatment_text.get_cloned();
                            if treatment_text.is_empty() {
                                page.treatment_text.set(result.trim_start().to_owned())
                            } else {
                                page.treatment_text.set([treatment_text, result].concat());
                            }
                            page.changed.set_neq(true);
                        }
                        Err(e) => {
                            app.alert_app_error(&e).await;
                        }
                    }
                }),
            );
        }
    }

    fn load_all_lab(page: Rc<Self>, app: Rc<App>) {
        if let Some((Some(hn), Some(regdate))) = page.patient.lock_ref().as_ref().map(|pt| (pt.hn(), pt.regdate())) {
            app.async_load(
                true,
                clone!(app, page => async move {
                    let params = LabHeadParams {
                        hn: Some(hn),
                        start_date: Some(regdate),
                        end_date: Some(js_now().date()),
                        ..Default::default()
                    };
                    // GET `EndPoint::LabHead`
                    match LabHead::call_api_get(&params, app.state()).await {
                        Ok(items) => {
                            let mut groups = BTreeMap::new();
                            for item in items.into_iter().filter_map(|head| {
                                (head.confirm_report == Some(String::from("Y"))).then(|| {
                                    datetime_from_opt(head.report_date.or(head.order_date), head.report_time.or(head.order_time)).map(move |dt| (dt, head))
                                }).flatten()
                            }) {
                                groups.entry(item.1.lab_name_cc.clone()).or_insert(Vec::new()).push(item);
                            }
                            let result = groups.iter_mut().map(|(k, lhs)| {
                                let title = k.clone().unwrap_or(String::from("อื่นๆ"));
                                lhs.sort_by(|a, b| a.0.cmp(&b.0));
                                let inner = lhs.iter().map(|(dt, head)| {
                                    ["- [", &datetime_th(dt), "] ", &lab::full_text(head)].concat()
                                }).collect::<Vec<String>>().join("\n");
                                [title, inner].join("\n")
                            }).collect::<Vec<String>>().join("\n\n");

                            page.lab_text.set(result);
                            page.changed.set_neq(true);
                        }
                        Err(e) => {
                            app.alert_app_error(&e).await;
                        }
                    }
                }),
            )
        }
    }

    fn set_new_not_refernote(&self) {
        let now = js_now();
        let department = if self.patient.lock_ref().as_ref().map(|pt| pt.is_admited()).unwrap_or_default() {
            "IPD"
        } else {
            "OPD"
        };

        self.referout_id.set_neq(0);
        self.refernote_id.set_neq(0);

        self.docno.set_neq(String::new());
        // self.due_date.set_neq(now.date().to_string());
        self.expire_date.set_neq(now.date().to_string());

        self.department.set_neq(String::from(department));
        self.refer_type.set_neq(String::from("1"));
        self.refer_cause.set_neq(String::from("1"));
        self.refer_point.set_neq(String::from("ER"));
        self.moph_refer_expire_type_id.set_neq(String::from("2"));

        self.refer_vital_sign_id.set_neq(0);
    }

    fn set_new(&self) {
        let now = js_now();
        self.hospital_refer.set(None);
        self.refer_date.set_neq(now.date().to_string());
        self.refer_time.set_neq(now.time().js_string());
        self.docno.set_neq(String::new());
        self.pre_diagnosis.set_neq(String::new());
        // self.diagnosis_text.set_neq(String::new());

        self.pmh.set_neq(String::new());
        self.hpi.set_neq(String::new());
        self.lab_text.set_neq(String::new());
        self.treatment_text.set_neq(String::new());
        self.other_text.set_neq(String::new());
        self.request_text.set_neq(String::new());

        self.cc.set_neq(String::new());
        self.pe.set_neq(String::new());

        self.set_new_not_refernote();
        self.changed.set_neq(false);
    }

    fn set_referout(&self, data: &HisReferOutData) {
        self.referout_id.set_neq(data.referout.referout_id);

        self.hospital_refer.set(HospSearchBox::new(&data.referout.refer_hospcode, &None, &data.referout.refer_hospcode_name));
        self.refer_date.set(data.referout.refer_date.map(|d| d.to_string()).unwrap_or_default());
        self.refer_time.set(data.referout.refer_time.map(|t| t.js_string()).unwrap_or_default());
        self.docno.set_neq(String::new());
        // self.due_date.set(data.referout.due_date.map(|d| d.to_string()).unwrap_or_default());
        self.expire_date.set(data.referout.expire_date.or(data.referout.due_date).map(|d| d.to_string()).unwrap_or_default());
        self.pre_diagnosis.set(data.referout.diagnosis_text.clone().or(data.referout.pre_diagnosis.clone()).unwrap_or_default());
        // self.diagnosis_text.set(data.referout.diagnosis_text.clone().unwrap_or_default());

        self.department.set(data.referout.department.clone().unwrap_or_default());
        self.refer_type.set(data.referout.refer_type.map(|i| i.to_string()).unwrap_or_default());
        self.refer_cause.set(data.referout.refer_cause.map(|i| i.to_string()).unwrap_or_default());
        self.refer_point.set(data.referout.refer_point.clone().unwrap_or_default());
        self.moph_refer_expire_type_id.set(data.referout.moph_refer_expire_type_id.map(|i| i.to_string()).unwrap_or_default());

        self.pmh.set(data.referout.pmh.clone().unwrap_or_default());
        self.hpi.set(data.referout.hpi.clone().unwrap_or_default());
        self.lab_text.set(data.referout.lab_text.clone().unwrap_or_default());
        self.treatment_text.set(data.referout.treatment_text.clone().unwrap_or_default());
        self.other_text.set(data.referout.other_text.clone().unwrap_or_default());
        self.request_text.set(data.referout.request_text.clone().unwrap_or_default());

        if let Some(vs) = data.vital_signs.first() {
            self.refer_vital_sign_id.set_neq(vs.refer_vital_sign_id);
            self.cc.set(vs.cc.clone().unwrap_or_default());
            self.pe.set(vs.pe.clone().unwrap_or_default());
        } else {
            self.refer_vital_sign_id.set_neq(0);
            self.cc.set_neq(String::new());
            self.pe.set_neq(String::new());
        }
        self.changed.set_neq(false);
    }

    fn set_refernote(&self, note: &ReferNote) {
        self.refernote_id.set_neq(note.refernote_id);

        self.hospital_refer.set(HospSearchBox::new(&note.refer_hospcode, &None, &note.refer_hospcode_name));
        self.refer_date.set(note.refer_date.map(|d| d.to_string()).unwrap_or_default());
        self.refer_time.set(note.refer_time.map(|t| t.js_string()).unwrap_or_default());
        self.docno.set(note.docno.clone().unwrap_or_default());
        self.expire_date.set(String::new());
        self.pre_diagnosis.set(note.diagnosis_text.clone().unwrap_or_default());

        self.department.set(String::new());
        self.refer_type.set(String::new());
        self.refer_cause.set(String::new());
        self.refer_point.set(String::new());
        self.moph_refer_expire_type_id.set(String::new());

        self.pmh.set(note.pmh.clone().unwrap_or_default());
        self.hpi.set(note.hpi.clone().unwrap_or_default());
        self.lab_text.set(note.lab_text.clone().unwrap_or_default());
        self.treatment_text.set(note.treatment_text.clone().unwrap_or_default());
        self.other_text.set(note.other_text.clone().unwrap_or_default());
        self.request_text.set(note.request_text.clone().unwrap_or_default());

        self.refer_vital_sign_id.set_neq(0);
        self.cc.set(note.cc.clone().unwrap_or_default());
        self.pe.set(note.pe.clone().unwrap_or_default());

        self.changed.set_neq(false);
    }

    fn save_referout(page: Rc<Self>, app: Rc<App>) {
        if let Some(pt) = page.patient.get_cloned() {
            let hn = pt.hn();
            let vnan = pt.visit_type.vnan().to_owned();
            let saver = HisReferOutSave {
                referout_id: zero_none(page.referout_id.get()),
                vn: vnan.clone(),
                hn,
                refer_hospcode: page.hospital_refer.get_cloned().map(|hosp| hosp.id.clone()),
                refer_date: date_8601(&page.refer_date.lock_ref()),
                refer_time: time_8601(&page.refer_time.lock_ref()),
                // use expire_date
                due_date: date_8601(&page.expire_date.lock_ref()),
                expire_date: date_8601(&page.expire_date.lock_ref()),
                pre_diagnosis: str_some(sanity_tis620(&page.pre_diagnosis.lock_ref())),
                // use pre_diagnosis
                diagnosis_text: str_some(sanity_tis620(&page.pre_diagnosis.lock_ref())),
                pmh: str_some(sanity_tis620(&page.pmh.lock_ref())),
                hpi: str_some(sanity_tis620(&page.hpi.lock_ref())),
                lab_text: str_some(sanity_tis620(&page.lab_text.lock_ref())),
                treatment_text: str_some(sanity_tis620(&page.treatment_text.lock_ref())),
                other_text: str_some(sanity_tis620(&page.other_text.lock_ref())),
                request_text: str_some(sanity_tis620(&page.request_text.lock_ref())),

                department: str_some(page.department.get_cloned()).or(Some(String::from(if pt.is_admited() { "IPD" } else { "OPD" }))),
                pttype: pt.pttype.clone(),
                spclty: pt.spclty.clone(),
                refer_type: page.refer_type.lock_ref().parse::<i8>().ok(),
                refer_cause: page.refer_cause.lock_ref().parse::<i8>().ok(),
                refer_point: str_some(page.refer_point.get_cloned()),
                moph_refer_expire_type_id: page.moph_refer_expire_type_id.get_cloned().parse::<i32>().ok(),

                refer_vital_sign_id: zero_none(page.refer_vital_sign_id.get()),
                cc: str_some(sanity_tis620(&page.cc.lock_ref())),
                pe: str_some(sanity_tis620(&page.pe.lock_ref())),
            };
            app.async_load(
                true,
                clone!(app => async move {
                    // POST `EndPoint::HisReferOutVnan`
                    match saver.call_api_post(&vnan, app.state()).await {
                        Ok(responses) => {
                            app.alert_execute_responses(&responses, clone!(app, page => async move {
                                page.reload_referout.set(true);
                            })).await;
                        }
                        Err(e) => {
                            app.alert_app_error(&e).await;
                        }
                    }
                }),
            )
        }
    }

    fn save_refernote(page: Rc<Self>, app: Rc<App>) {
        if let Some(pt) = page.patient.get_cloned() {
            let hn = pt.hn();
            let vnan = pt.visit_type.vnan().to_owned();
            let saver = ReferNoteSave {
                refernote_id: zero_none(page.refernote_id.get()),
                vn: vnan.clone(),
                hn,
                refer_hospcode: page.hospital_refer.get_cloned().map(|hosp| hosp.id.clone()),
                refer_date: date_8601(&page.refer_date.lock_ref()),
                refer_time: time_8601(&page.refer_time.lock_ref()),
                docno: str_some(page.docno.get_cloned()),
                pre_diagnosis: str_some(page.pre_diagnosis.get_cloned()),
                // use pre_diagnosis
                diagnosis_text: str_some(page.pre_diagnosis.get_cloned()),
                pmh: str_some(page.pmh.get_cloned()),
                hpi: str_some(page.hpi.get_cloned()),
                lab_text: str_some(page.lab_text.get_cloned()),
                treatment_text: str_some(page.treatment_text.get_cloned()),
                other_text: str_some(page.other_text.get_cloned()),
                request_text: str_some(page.request_text.get_cloned()),

                cc: str_some(page.cc.get_cloned()),
                pe: str_some(page.pe.get_cloned()),
            };
            app.async_load(
                true,
                clone!(app => async move {
                    // POST `EndPoint::ReferNoteVnan`
                    match saver.call_api_post(&vnan, app.state()).await {
                        Ok(response) => {
                            app.alert_execute_response(&response, clone!(app, page => async move {
                                page.reload_refernote.set(true);
                            })).await;
                        }
                        Err(e) => {
                            app.alert_app_error(&e).await;
                        }
                    }
                }),
            )
        }
    }

    pub fn render(page: Rc<Self>, app: Rc<App>) -> Dom {
        html!("div", {
            .future(map_ref!(
                let busy = app.loader_is_loading(),
                let loaded = page.loaded.signal() =>
                !busy && !loaded
            ).for_each(clone!(app, page => move |ready| {
                if ready {
                    Self::load_all(page.clone(), app.clone());
                    page.loaded.set(true);
                }
                async {}
            })))
            .future(map_ref!(
                let busy = app.loader_is_loading(),
                let reload = page.reload_referout.signal() =>
                !busy && *reload
            ).for_each(clone!(app, page => move |ready| {
                if ready {
                    Self::load_referout(page.clone(), app.clone());
                    page.reload_referout.set(false);
                }
                async {}
            })))
            .future(map_ref!(
                let busy = app.loader_is_loading(),
                let reload = page.reload_refernote.signal() =>
                !busy && *reload
            ).for_each(clone!(app, page => move |ready| {
                if ready {
                    Self::load_refernote(page.clone(), app.clone());
                    page.reload_refernote.set(false);
                }
                async {}
            })))
            .children([
                html!("nav", {
                    .child(html!("div", {
                        .class(class::NAV_TABS_T)
                        .attr("role","tablist")
                        .children([
                            html!("a", {
                                .class(class::NAV_ITEM_LINK_P2)
                                .class_signal("active", page.active_tab.signal_ref(|tab| matches!(tab, Tab::ReferOut)))
                                .attr("id", "nav-refer-out-tab")
                                .attr("data-bs-toggle","pill")
                                .attr("href","#")
                                .text("Refer Out")
                                .event_with_options(&EventOptions::preventable(), clone!(page => move |event: events::Click| {
                                    event.prevent_default();
                                    page.active_tab.set_neq(Tab::ReferOut);
                                    if let Some(first) = page.referouts.lock_ref().first() {
                                        page.set_referout(&first);
                                    } else {
                                        page.set_new();
                                    }
                                }))
                            }),
                            html!("a", {
                                .class(class::NAV_ITEM_LINK_P2)
                                .class_signal("active", page.active_tab.signal_ref(|tab| matches!(tab, Tab::ReferNote)))
                                .attr("id", "nav-refer-note-tab")
                                .attr("data-bs-toggle","pill")
                                .attr("href","#")
                                .text("บันทึกข้อความ")
                                .event_with_options(&EventOptions::preventable(), clone!(page => move |event: events::Click| {
                                    event.prevent_default();
                                    page.active_tab.set_neq(Tab::ReferNote);
                                    if let Some(first) = page.refernotes.lock_ref().first() {
                                        page.set_refernote(&first);
                                    } else {
                                        page.set_new();
                                    }
                                }))
                            }),
                        ])
                    }))
                }),
                html!("div", {
                    .class(class::ROW_TC)
                    .child_signal(page.active_tab.signal_cloned().map(clone!(app, page => move |tab| {
                        Some(match tab {
                            Tab::ReferOut => {
                                Self::render_refer_out(page.clone(), app.clone())
                            }
                            Tab::ReferNote => {
                                Self::render_refer_note(page.clone(), app.clone())
                            }
                        })
                    })))
                }),
                html!("div", {
                    .class("modal")
                    .attr("id", "vsSelectorModal")
                    .attr("role", "dialog")
                    .attr("tabindex", "-1")
                    .child_signal(page.vs_selector_modal.signal_cloned().map(clone!(app => move |opt| {
                        opt.as_ref().map(clone!(app => move |modal| {
                            VsSelector::render(modal.clone(), app)
                        })).or(Some(blank_modal()))
                    })))
                }),
                html!("div", {
                    .class("modal")
                    .attr("id", "labSelectorModal")
                    .attr("role", "dialog")
                    .attr("tabindex", "-1")
                    .child_signal(page.lab_selector_modal.signal_cloned().map(clone!(app => move |opt| {
                        opt.as_ref().map(clone!(app => move |modal| {
                            LabSelector::render(modal.clone(), app)
                        })).or(Some(blank_modal()))
                    })))
                }),
            ])
        })
    }

    fn render_refer_note(page: Rc<Self>, app: Rc<App>) -> Dom {
        html!("div", {
            .children([
                html!("div", {
                    .class(class::ROW_T)
                    .child(html!("div", {
                        .class("col")
                        .child_signal(page.patient.signal_cloned().map(clone!(app, page => move |opt| {
                            opt.map(|patient| {
                                let vnan = patient.visit_type.vnan().to_owned();
                                html!("div",{
                                    .class(class::FLOAT_RR)
                                    .children(PdfButtons::buttons(
                                        PdfButtons::new(
                                            TypstReport::from_system_with_coercion(SystemReport::ReferNote, &app.state().report_coercions()),
                                            Mutable::new(vnan.clone()),
                                            page.checked.clone(),
                                            page.changed.clone(),
                                            clone!(page => move || {serde_json::json!({
                                                "id": vnan,
                                                "patient": patient,
                                                "refernote": page.refernotes.lock_ref().to_vec(),
                                            }).to_string()})
                                        ), "PDF", None, app.clone()
                                    ))
                                })
                            })
                        })))
                        .child_signal(page.refernote_id.signal().map(clone!(page => move |refernote_id| {
                            (refernote_id > 0).then(|| {
                                html!("div", {
                                    .class(class::FLOAT_RR)
                                    .child(html!("button", {
                                        .attr("type", "button")
                                        .class(class::BTN_L_GRAY)
                                        .child(html!("i", {.class(class::FA_CLIPBOARD)}))
                                        .text(" คัดลอกเป็นใบ Refer ใหม่")
                                        .event(clone!(page => move |_: events::Click| {
                                            page.active_tab.set(Tab::ReferOut);
                                            page.set_new_not_refernote();
                                            page.changed.set_neq(true);
                                        }))
                                    }))
                                })
                            })
                        })))
                        .child_signal(page.refernote_id.signal().map(clone!(page => move |refernote_id| {
                            (refernote_id > 0).then(|| {
                                html!("div", {
                                    .class(class::FLOAT_RR)
                                    .child(html!("button", {
                                        .attr("type", "button")
                                        .class(class::BTN_L_GRAY)
                                        .child(html!("i", {.class(class::FA_CLIPBOARD)}))
                                        .text(" เพิ่มบันทึกข้อความใหม่")
                                        .event(clone!(page => move |_: events::Click| {
                                            page.set_new();
                                        }))
                                    }))
                                })
                            })
                        })))
                        .children_signal_vec(page.refernotes.signal_vec_cloned().map(clone!(page => move |note| {
                            let refernote_id = note.refernote_id;
                            html!("button", {
                                .attr("type","button")
                                .class(class::BTN_L)
                                .class_signal("btn-primary", page.refernote_id.signal().map(move |id| id == refernote_id))
                                .class_signal("btn-secondary", page.refernote_id.signal().map(move |id| id != refernote_id))
                                .text(&note.refer_hospcode_name.clone().unwrap_or(String::from("ไม่ระบุ รพ.")))
                                .event(clone!(page => move |_:events::Click| {
                                    page.set_refernote(&note);
                                }))
                            })
                        })))
                    }))
                }),
                html!("div", {
                    .class("mb-3")
                    .style("column-width","720px")
                    .style("column-gap","8px")
                    .children([
                        Self::render_general_refernote(page.clone(), app.clone()),
                        Self::render_hx_pe(page.clone(), app.clone()),
                        Self::render_detail(page.clone(), app.clone()),
                    ])
                }),
                html!("div", {
                    .class("row")
                    .child(html!("div", {
                        .class(class::COL_R)
                        .apply_if(app.endpoint_is_allow(&Method::POST, &EndPoint::ReferNoteVnan, false), |dom| dom
                            .child(html!("button" => HtmlButtonElement, {
                                .attr("type", "button")
                                .class(class::BTN_R)
                                .class_signal("btn-primary", page.changed.signal())
                                .class_signal("btn-secondary", not(page.changed.signal()))
                                .child(html!("i", {.class(class::FA_SAVE)}))
                                .text(" บันทึก")
                                .apply(mixins::click_with_loader_checked_or_true_disable_signal(clone!(app, page => move || {
                                    Self::save_refernote(page.clone(), app.clone());
                                }), page.is_refernote_not_ready_signal(), app.state()))
                            }))
                        )
                    }))
                }),
            ])
        })
    }

    fn render_refer_out(page: Rc<Self>, app: Rc<App>) -> Dom {
        html!("div", {
            .children([
                html!("div", {
                    .class(class::ROW_T)
                    .child(html!("div", {
                        .class("col")
                        .child_signal(page.patient.signal_cloned().map(clone!(app, page => move |opt| {
                            opt.map(|patient| {
                                let vnan = patient.visit_type.vnan().to_owned();
                                html!("div",{
                                    .class(class::FLOAT_RR)
                                    .children(PdfButtons::buttons(
                                        PdfButtons::new(
                                            TypstReport::from_system_with_coercion(SystemReport::LabSummary, &app.state().report_coercions()),
                                            Mutable::new(vnan.clone()),
                                            Mutable::new(true),
                                            Mutable::new(false),
                                            clone!(page => move || {serde_json::json!({
                                                "id": vnan,
                                                "patient": patient,
                                                "lab": null,
                                            }).to_string()})
                                        ), "Lab Summary", None, app.clone()
                                    ))
                                })
                            })
                        })))
                        .child_signal(page.patient.signal_cloned().map(clone!(app, page => move |opt| {
                            opt.map(|patient| {
                                let vnan = patient.visit_type.vnan().to_owned();
                                html!("div",{
                                    .class(class::FLOAT_RR)
                                    .children(PdfButtons::buttons(
                                        PdfButtons::new(
                                            TypstReport::from_system_with_coercion(SystemReport::ReferOut, &app.state().report_coercions()),
                                            Mutable::new(vnan.clone()),
                                            page.checked.clone(),
                                            page.changed.clone(),
                                            clone!(page => move || {serde_json::json!({
                                                "id": vnan,
                                                "patient": patient,
                                                "referout": page.referouts.lock_ref().to_vec(),
                                            }).to_string()})
                                        ), "Refer Out", None, app.clone()
                                    ))
                                })
                            })
                        })))
                        .child_signal(page.referout_id.signal().map(clone!(page => move |referout_id| {
                            (referout_id > 0).then(|| {
                                html!("div", {
                                    .class(class::FLOAT_RR)
                                    .child(html!("button", {
                                        .attr("type", "button")
                                        .class(class::BTN_L_GRAY)
                                        .child(html!("i", {.class(class::FA_CLIPBOARD)}))
                                        .text(" คัดลอกเป็นบันทึกข้อความใหม่")
                                        .event(clone!(page => move |_: events::Click| {
                                            page.active_tab.set(Tab::ReferNote);
                                            page.refernote_id.set_neq(0);
                                            page.changed.set_neq(true);
                                        }))
                                    }))
                                })
                            })
                        })))
                        .child_signal(page.referout_id.signal().map(clone!(page => move |referout_id| {
                            (referout_id > 0).then(|| {
                                html!("div", {
                                    .class(class::FLOAT_RR)
                                    .child(html!("button", {
                                        .attr("type", "button")
                                        .class(class::BTN_L_GRAY)
                                        .child(html!("i", {.class(class::FA_CLIPBOARD)}))
                                        .text(" เพิ่มใบ Refer ใหม่")
                                        .event(clone!(page => move |_: events::Click| {
                                            page.set_new();
                                        }))
                                    }))
                                })
                            })
                        })))
                        .children_signal_vec(page.referouts.signal_vec_cloned().map(clone!(page => move |data| {
                            let referout_id = data.referout.referout_id;
                            html!("button", {
                                .attr("type","button")
                                .class(class::BTN_L)
                                .class_signal("btn-primary", page.referout_id.signal().map(move |id| id == referout_id))
                                .class_signal("btn-secondary", page.referout_id.signal().map(move |id| id != referout_id))
                                .text(&data.referout.refer_number.clone().unwrap_or_default())
                                .apply_if(data.referout.refer_number.is_some(), |dom| dom.text(" "))
                                .text(&data.referout.refer_hospcode_name.clone().unwrap_or(String::from("ไม่ระบุ รพ.")))
                                .apply(|dom| {
                                    if data.referout.issued_moph_refer.as_ref().map(|issued_moph_refer| issued_moph_refer.as_str() == "Y").unwrap_or_default() {
                                        dom.child(html!("i", {.class(class::FA_CHECK_CIRCLE_GREEN).class("ms-1")}))
                                    } else {
                                        dom.child(html!("i", {.class(class::FA_X_CIRCLE_RED).class("ms-1")}))
                                    }
                                })
                                .event(clone!(page => move |_:events::Click| {
                                    page.set_referout(&data);
                                }))
                            })
                        })))
                    }))
                }),
                html!("div", {
                    .class("mb-3")
                    .style("column-width","720px")
                    .style("column-gap","8px")
                    .children([
                        Self::render_general_referout(page.clone(), app.clone()),
                        Self::render_hx_pe(page.clone(), app.clone()),
                        Self::render_detail(page.clone(), app.clone()),
                    ])
                }),
                html!("div", {
                    .class("row")
                    .child(html!("div", {
                        .class(class::COL_R)
                        .apply_if(app.endpoint_is_allow(&Method::POST, &EndPoint::HisReferOutVnan, false), |dom| dom
                            .child(html!("button" => HtmlButtonElement, {
                                .attr("type", "button")
                                .class(class::BTN_R)
                                .class_signal("btn-primary", page.changed.signal())
                                .class_signal("btn-secondary", not(page.changed.signal()))
                                .child(html!("i", {.class(class::FA_SAVE)}))
                                .text(" บันทึก")
                                .apply(mixins::click_with_loader_checked_or_true_disable_signal(clone!(app, page => move || {
                                    Self::save_referout(page.clone(), app.clone());
                                }), page.is_referout_not_ready_signal(), app.state()))
                            }))
                        )
                    }))
                }),
            ])
        })
    }

    fn render_general_refernote(page: Rc<Self>, app: Rc<App>) -> Dom {
        html!("div", {
            .class(class::CARD)
            .style("break-inside","avoid")
            .child(html!("div", {
                .class("card-body")
                .child(html!("div", {
                    .children([
                        html!("div", {
                            .class("row")
                            .children([
                                html!("div", {
                                    .class(class::COL_SM3_R)
                                    .child(html!("span", {
                                        .class("fw-bold")
                                        .text("ถึงสถานพยาบาล")
                                    }))
                                }),
                                html!("div", {
                                    .class("col-sm-9")
                                    .child(HospSearchboxCpn::render(HospSearchboxCpn::new(page.hospital_refer.clone()), app.clone(), page.changed.clone(), true, false))
                                }),
                            ])
                        }),
                        html!("div", {
                            .class(class::ROW_T)
                            .children([
                                html!("div", {
                                    .class(class::COL_SM3_R)
                                    .child(html!("span", {
                                        .class("fw-bold")
                                        .text("เลขที่")
                                    }))
                                }),
                                html!("div", {
                                    .class("col-sm-9")
                                    .child(html!("input" => HtmlInputElement, {
                                        .attr("type","text")
                                        .class(class::FORM_CTRL_SM)
                                        .apply(mixins::string_value(page.docno.clone(), page.changed.clone()))
                                    }))
                                }),
                            ])
                        }),
                        html!("div", {
                            .class(class::ROW_T)
                            .children([
                                html!("div", {
                                    .class(class::COL_SM3_R)
                                    .child(html!("span", {
                                        .class("fw-bold")
                                        .text("วันและเวลา")
                                    }))
                                }),
                                html!("div", {
                                    .class("col-sm-9")
                                    .child(html!("div", {
                                        .class(class::INPUT_GROUP_SM)
                                        .children([
                                            doms::date_picker(
                                                page.refer_date.clone(),
                                                page.changed.clone(), always(false), None,
                                                |d| d.class(class::FLEX_GROW1).style("min-width","120px"),
                                                clone!(page => move |d| d.class(class::FORM_CTRL_ONLY_SM_R0_R).class_signal("border-danger", page.refer_date.signal_ref(|s| s.is_empty()))),
                                                |d| d.class(class::FORM_CTRL_ONLY_SM_R0_R),
                                                |s| s, always(None),
                                            ),
                                            doms::time_picker(
                                                page.refer_time.clone(),
                                                page.changed.clone(), always(false), None,
                                                |d| d.class(class::FLEX_GROW1).style("min-width","95px"),
                                                |d| d.class(class::FORM_CTRL_ONLY_SM_R0_R),
                                                |d| d.class(class::FORM_CTRL_ONLY_SM_R0_R),
                                                |s| s, always(None),
                                            ),
                                            html!("button", {
                                                .class(class::BTN_GRAY)
                                                .attr("type", "button")
                                                .text("ปัจจุบัน")
                                                .event(clone!(page => move |_:events::Click| {
                                                    let now = js_now();
                                                    page.refer_date.set_neq(now.date().to_string());
                                                    page.refer_time.set_neq(now.time().js_string());
                                                    page.changed.set_neq(true);
                                                }))
                                            }),
                                        ])
                                    }))
                                }),
                            ])
                        }),
                    ])
                }))
            }))
        })
    }

    fn render_general_referout(page: Rc<Self>, app: Rc<App>) -> Dom {
        let (refer_type_select_option, refer_cause_select_option, refer_point_select_option, moph_refer_expire_type_select_option) = app
            .app_asset
            .lock_ref()
            .as_ref()
            .map(|asset| {
                (
                    asset.refer_type_select_option.clone(),
                    asset.refer_cause_select_option.clone(),
                    asset.refer_point_select_option.clone(),
                    asset.moph_refer_expire_type_select_option.clone(),
                )
            })
            .unwrap_or_default();

        html!("div", {
            .class(class::CARD)
            .style("break-inside","avoid")
            .child(html!("div", {
                .class("card-body")
                .child(html!("div", {
                    .children([
                        html!("div", {
                            .class("row")
                            .children([
                                html!("div", {
                                    .class(class::COL_SM3_R)
                                    .child(html!("span", {
                                        .class("fw-bold")
                                        .text("ส่งไปสถานพยาบาล")
                                    }))
                                }),
                                html!("div", {
                                    .class("col-sm-9")
                                    .child(HospSearchboxCpn::render(HospSearchboxCpn::new(page.hospital_refer.clone()), app.clone(), page.changed.clone(), true, true))
                                }),
                            ])
                        }),
                        html!("div", {
                            .class(class::ROW_T)
                            .children([
                                html!("div", {
                                    .class(class::COL_SM3_R)
                                    .child(html!("span", {
                                        .class("fw-bold")
                                        .text("ประเภทผู้ป่วย")
                                    }))
                                }),
                                html!("div", {
                                    .class(class::COL_SM3_R)
                                    .child(html!("select" => HtmlSelectElement, {
                                        .class(class::FORM_SELECT_SM)
                                        .children([
                                            html!("option", {.attr("value", "OPD").text("OPD")}),
                                            html!("option", {.attr("value", "IPD").text("IPD")})
                                        ])
                                        .apply(mixins::string_value_select(page.department.clone(), page.changed.clone()))
                                    }))
                                }),
                                html!("div", {
                                    .class(class::COL_SM3_R)
                                    .child(html!("span", {
                                        .class("fw-bold")
                                        .text("จุดรับส่งต่อ")
                                    }))
                                }),
                                html!("div", {
                                    .class("col-sm-3")
                                    .child(html!("select" => HtmlSelectElement, {
                                        .class(class::FORM_SELECT_SM)
                                        .child(html!("option", {.attr("value", "").text("เลือก")}))
                                        .children(refer_point_select_option.iter().map(|option| {
                                            doms::select_option(option, "")
                                        }))
                                        .apply(mixins::string_value_select(page.refer_point.clone(), page.changed.clone()))
                                    }))
                                }),
                            ])
                        }),
                        html!("div", {
                            .class(class::ROW_T)
                            .children([
                                html!("div", {
                                    .class(class::COL_SM3_R)
                                    .child(html!("span", {
                                        .class("fw-bold")
                                        .text("วันและเวลาส่งตัว")
                                    }))
                                }),
                                html!("div", {
                                    .class("col-sm-9")
                                    .child(html!("div", {
                                        .class(class::INPUT_GROUP_SM)
                                        .children([
                                            doms::date_picker(
                                                page.refer_date.clone(),
                                                page.changed.clone(), always(false), None,
                                                |d| d.class(class::FLEX_GROW1).style("min-width","120px"),
                                                clone!(page => move |d| d.class(class::FORM_CTRL_ONLY_SM_R0_R).class_signal("border-danger", page.refer_date.signal_ref(|s| s.is_empty()))),
                                                |d| d.class(class::FORM_CTRL_ONLY_SM_R0_R),
                                                |s| s, always(None),
                                            ),
                                            doms::time_picker(
                                                page.refer_time.clone(),
                                                page.changed.clone(), always(false), None,
                                                |d| d.class(class::FLEX_GROW1).style("min-width","95px"),
                                                |d| d.class(class::FORM_CTRL_ONLY_SM_R0_R),
                                                |d| d.class(class::FORM_CTRL_ONLY_SM_R0_R),
                                                |s| s, always(None),
                                            ),
                                            html!("button", {
                                                .class(class::BTN_GRAY)
                                                .attr("type", "button")
                                                .text("ปัจจุบัน")
                                                .event(clone!(page => move |_:events::Click| {
                                                    let now = js_now();
                                                    page.refer_date.set_neq(now.date().to_string());
                                                    page.refer_time.set_neq(now.time().js_string());
                                                    page.changed.set_neq(true);
                                                }))
                                            }),
                                        ])
                                    }))
                                }),
                            ])
                        }),
                        html!("div", {
                            .class(class::ROW_T)
                            .children([
                                html!("div", {
                                    .class(class::COL_SM3_R)
                                    .child(html!("span", {
                                        .class("fw-bold")
                                        .text("การหมดอายุ")
                                    }))
                                }),
                                html!("div", {
                                    .class("col-sm-9")
                                    .child(html!("select" => HtmlSelectElement, {
                                        .class(class::FORM_SELECT_SM)
                                        .child(html!("option", {.attr("value", "").text("เลือก")}))
                                        .children(moph_refer_expire_type_select_option.iter().map(|option| {
                                            doms::select_option(option, "")
                                        }))
                                        .apply(mixins::string_value_select(page.moph_refer_expire_type_id.clone(), page.changed.clone()))
                                    }))
                                }),
                            ])
                        }),
                        html!("div", {
                            .class(class::ROW_T)
                            .children([
                                html!("div", {
                                    .class(class::COL_SM3_R)
                                    .child(html!("span", {
                                        .class("fw-bold")
                                        .text("วันหมดอายุ")
                                    }))
                                }),
                                html!("div", {
                                    .class("col-sm-9")
                                    .child(html!("div", {
                                        .class(class::INPUT_GROUP_SM)
                                        .children([
                                            doms::date_picker(
                                                page.expire_date.clone(),
                                                page.changed.clone(), always(false), None,
                                                |d| d.class(class::FLEX_GROW1).style("min-width","120px"),
                                                clone!(page => move |d| d.class(class::FORM_CTRL_ONLY_SM_R0_R).class_signal("border-danger", page.expire_date.signal_ref(|s| s.is_empty()))),
                                                |d| d.class(class::FORM_CTRL_ONLY_SM_R0_R),
                                                |s| s, always(None),
                                            ),
                                            html!("button", {
                                                .class(class::BTN_GRAY)
                                                .attr("type", "button")
                                                .text("วันนี้")
                                                .event(clone!(page => move |_:events::Click| {
                                                    page.expire_date.set_neq(js_now().date().to_string());
                                                    page.moph_refer_expire_type_id.set_neq(String::from("2"));
                                                    page.changed.set_neq(true);
                                                }))
                                            }),
                                            html!("button", {
                                                .class(class::BTN_CYAN)
                                                .attr("type", "button")
                                                .text("1 เดือน")
                                                .event(clone!(page => move |_:events::Click| {
                                                    let dt = js_now() + Duration::days(30);
                                                    page.expire_date.set_neq(dt.date().to_string());
                                                    page.moph_refer_expire_type_id.set_neq(String::from("1"));
                                                    page.changed.set_neq(true);
                                                }))
                                            }),
                                            html!("button", {
                                                .class(class::BTN_GOLD)
                                                .attr("type", "button")
                                                .text("3 เดือน")
                                                .event(clone!(page => move |_:events::Click| {
                                                    let dt = js_now() + Duration::days(90);
                                                    page.expire_date.set_neq(dt.date().to_string());
                                                    page.moph_refer_expire_type_id.set_neq(String::from("1"));
                                                    page.changed.set_neq(true);
                                                }))
                                            }),
                                            html!("button", {
                                                .class(class::BTN_RED75)
                                                .attr("type", "button")
                                                .text("6 เดือน")
                                                .event(clone!(page => move |_:events::Click| {
                                                    let dt = js_now() + Duration::days(180);
                                                    page.expire_date.set_neq(dt.date().to_string());
                                                    page.moph_refer_expire_type_id.set_neq(String::from("1"));
                                                    page.changed.set_neq(true);
                                                }))
                                            }),
                                        ])
                                    }))
                                }),
                            ])
                        }),
                        html!("div", {
                            .class(class::ROW_T)
                            .children([
                                html!("div", {
                                    .class(class::COL_SM3_R)
                                    .child(html!("span", {
                                        .class("fw-bold")
                                        .text("ส่งตัวเพื่อ")
                                    }))
                                }),
                                html!("div", {
                                    .class(class::COL_SM3_R)
                                    .child(html!("select" => HtmlSelectElement, {
                                        .class(class::FORM_SELECT_SM)
                                        .child(html!("option", {.attr("value", "").text("เลือก")}))
                                        .children(refer_cause_select_option.iter().map(|option| {
                                            doms::select_option(option, "")
                                        }))
                                        .apply(mixins::string_value_select(page.refer_cause.clone(), page.changed.clone()))
                                    }))
                                }),
                                html!("div", {
                                    .class(class::COL_SM3_R)
                                    .child(html!("span", {
                                        .class("fw-bold")
                                        .text("ขอบเขตการรับส่ง")
                                    }))
                                }),
                                html!("div", {
                                    .class("col-sm-3")
                                    .child(html!("select" => HtmlSelectElement, {
                                        .class(class::FORM_SELECT_SM)
                                        .child(html!("option", {.attr("value", "").text("เลือก")}))
                                        .children(refer_type_select_option.iter().map(|option| {
                                            doms::select_option(option, "")
                                        }))
                                        .apply(mixins::string_value_select(page.refer_type.clone(), page.changed.clone()))
                                    }))
                                }),
                            ])
                        }),
                    ])
                }))
            }))
        })
    }

    fn render_hx_pe(page: Rc<Self>, app: Rc<App>) -> Dom {
        let (allow_admission_note, allow_med_rec, allow_vs) = page
            .patient
            .get_cloned()
            .map(|pt| pt.visit_type.is_ipd_and_is_pre_admit())
            .map(|(is_ipd, is_pre_admit)| {
                let allow_admission_note = if is_ipd {
                    app.endpoint_is_allow(&Method::GET, &EndPoint::IpdAdmissionNoteDrAn, is_pre_admit)
                } else {
                    false
                };
                let allow_med_rec = if is_ipd {
                    app.endpoint_is_allow(&Method::GET, &EndPoint::IpdMedReconcile, is_pre_admit)
                } else {
                    app.endpoint_is_allow(&Method::GET, &EndPoint::OpdErMedReconcile, false)
                };
                let allow_vs = if is_ipd {
                    app.endpoint_is_allow(&Method::GET, &EndPoint::IpdVitalSign, is_pre_admit)
                } else {
                    app.endpoint_is_allow(&Method::GET, &EndPoint::OpdErVitalSign, false)
                };
                (allow_admission_note, allow_med_rec, allow_vs)
            })
            .unwrap_or_default();

        html!("div", {
            .class(class::CARD)
            .style("break-inside","avoid")
            .child(html!("div", {
                .class("card-body")
                .children([
                    html!("div", {
                        .style("height","20px")
                        .apply_if(allow_med_rec, |dom| dom
                            .child(html!("div", {
                                .child(html!("button", {
                                    .attr("type", "button")
                                    .class(class::BTN_FR_R_GRAY)
                                    .child(html!("i", {.class(class::FA_PILLS)}))
                                    .text(" คัดลอกจาก Med Reconciliation")
                                    .event(clone!(app, page => move |_: events::Click| {
                                        Self::load_med_rec(page.clone(), app.clone());
                                    }))
                                }))
                            }))
                        )
                        .apply_if(allow_admission_note, |dom| dom
                            .child(html!("div", {
                                .child(html!("button", {
                                    .attr("type", "button")
                                    .class(class::BTN_FR_R_GRAY)
                                    .child(html!("i", {.class(class::FA_CLIPBOARD)}))
                                    .text(" คัดลอกจากบันทึกการรับใหม่ผู้ป่วยใน")
                                    .event(clone!(app, page => move |_: events::Click| {
                                        Self::load_admission_note(page.clone(), app.clone());
                                    }))
                                }))
                            }))
                        )
                    }),
                    html!("div", {
                        .class("mb-2")
                        .child(html!("span", {
                            .class("fw-bold")
                            .text("Chief Complaint")
                        }))
                    }),
                    html!("div", {
                        .class("mb-2")
                        .child(html!("input" => HtmlInputElement, {
                            .attr("type","text")
                            .class(class::FORM_CTRL_SM)
                            .apply(mixins::string_value(page.cc.clone(), page.changed.clone()))
                        }))
                    }),
                    html!("div", {
                        .class("mb-2")
                        .child(html!("span", {
                            .class("fw-bold")
                            .text("Physical Examination")
                        }))
                    }),
                    html!("div", {
                        .class("mb-2")
                        .child(html!("textarea" => HtmlTextAreaElement, {
                            .class(class::FORM_CTRL_SM)
                            .apply(mixins::textarea_value_auto_expand(page.pe.clone(), page.changed.clone()))
                        }))
                    }),
                    html!("div", {
                        .class("mb-2")
                        .child(html!("span", {
                            .class("fw-bold")
                            .text("ประวัติการเจ็บป่วยในอดีต")
                        }))
                    }),
                    html!("div", {
                        .class("mb-2")
                        .child(html!("textarea" => HtmlTextAreaElement, {
                            .class(class::FORM_CTRL_SM)
                            .apply(mixins::textarea_value_auto_expand(page.pmh.clone(), page.changed.clone()))
                        }))
                    }),
                    html!("div", {
                        .child(html!("span", {
                            .class(class::BOLD_L2)
                            .text("ประวัติการเจ็บป่วยในปัจจุบัน")
                        }))
                        .apply(|dom| {
                            if allow_vs {
                                dom.child(html!("button", {
                                    .attr("type", "button")
                                    .class(class::BTN_SM_RT_BLUE)
                                    .attr("data-bs-toggle", "modal")
                                    .attr("data-bs-target", "#vsSelectorModal")
                                    .child(html!("i", {.class(class::FA_HEARTBEAT)}))
                                    .event(clone!(page => move |_: events::Click| {
                                        page.vs_selector_modal.set(Some(VsSelector::new(
                                            true,
                                            page.patient.clone(),
                                            page.hpi.clone(),
                                            page.changed.clone(),
                                        )));
                                    }))
                                }))
                            } else {
                                dom.class("mb-2")
                            }
                        })
                    }),
                    html!("div", {
                        .class("mb-2")
                        .child(html!("textarea" => HtmlTextAreaElement, {
                            .class(class::FORM_CTRL_SM)
                            .apply(mixins::textarea_value_auto_expand(page.hpi.clone(), page.changed.clone()))
                        }))
                    }),
                ])
            }))
        })
    }

    fn render_detail(page: Rc<Self>, app: Rc<App>) -> Dom {
        let allow_lab = app.endpoint_is_allow(&Method::GET, &EndPoint::LabHead, false);
        let allow_rx = app.endpoint_is_allow(&Method::GET, &EndPoint::IpdOrderPrevious, false);
        html!("div", {
            .class(class::CARD)
            .style("break-inside","avoid")
            .child(html!("div", {
                .class("card-body")
                .child(html!("div", {
                    .children([
                        html!("div", {
                            .child(html!("span", {
                                .class(class::BOLD_L2)
                                .text("ผลการตรวจทางห้องปฏิบัติการ")
                            }))
                            .apply(|dom| {
                                if allow_lab {
                                    dom.children([
                                        html!("button", {
                                            .attr("type", "button")
                                            .class(class::BTN_SM_RT_BLUE)
                                            .attr("data-bs-toggle", "modal")
                                            .attr("data-bs-target", "#labSelectorModal")
                                            .child(html!("i", {.class(class::FA_FLASK)}))
                                            .event(clone!(page => move |_: events::Click| {
                                                page.lab_selector_modal.set(Some(LabSelector::new(
                                                    true,
                                                    page.patient.clone(),
                                                    page.lab_text.clone(),
                                                    page.changed.clone(),
                                                )));
                                            }))
                                        }),
                                        html!("button", {
                                            .attr("type", "button")
                                            .class(class::BTN_SM_RT_CYAN)
                                            .text("ALL ")
                                            .child(html!("i", {.class(class::FA_FLASK)}))
                                            .event(clone!(app, page => move |_: events::Click| {
                                                Self::load_all_lab(page.clone(), app.clone());
                                            }))
                                        }),
                                    ])
                                } else {
                                    dom.class("mb-2")
                                }
                            })
                        }),
                        html!("div", {
                            .class("mb-2")
                            .child(html!("textarea" => HtmlTextAreaElement, {
                                .attr("rows","5")
                                .class(class::FORM_CTRL_SM)
                                .apply(mixins::textarea_value_auto_expand(page.lab_text.clone(), page.changed.clone()))
                            }))
                        }),
                        html!("div", {
                            .class("mb-2")
                            .child(html!("span", {
                                .class("fw-bold")
                                .text("การวินิจฉัยโรคขั้นต้น")
                            }))
                        }),
                        html!("div", {
                            .class("mb-2")
                            .child(html!("input" => HtmlInputElement, {
                                .attr("type","text")
                                .class(class::FORM_CTRL_SM)
                                .class_signal("border-danger", page.pre_diagnosis.signal_ref(|s| s.is_empty()))
                                .apply(mixins::string_value(page.pre_diagnosis.clone(), page.changed.clone()))
                            }))
                        }),
                        html!("div", {
                            .child(html!("span", {
                                .class(class::BOLD_L2)
                                .text("การรักษาที่ได้ให้ไว้แล้ว (ยา/หัตถการ)")
                            }))
                            .apply(|dom| {
                                if allow_rx { dom
                                    // for keeping vertical alignment without Rx button
                                    .class_signal("mb-2", not(page.patient.signal_cloned().map(|opt| opt.map(|pt| pt.is_ipd()).unwrap_or_default())))
                                    .child_signal(page.patient.signal_cloned().map(|opt| opt.map(|pt| pt.is_ipd()).unwrap_or_default()).map(clone!(app, page => move |is_ipd| {
                                        is_ipd.then(|| {
                                            html!("button", {
                                                .attr("type", "button")
                                                .class(class::BTN_SM_RT_BLUE)
                                                .child(html!("i", {.class(class::FA_RX)}))
                                                .attr("title","สร้างรายการ Continuous order ในปัจจุบันเท่านั้น\nยกเว้น Injections ที่จะแสดงรายการที่ off ไปแล้วด้วย\nร่วมกับแสดงวันเริ่มต้น/สิ้นสุด จาก action ของพยาบาล\n(หากไม่มี action จะใช้วันที่ order และวันที่ off แทน)")
                                                .event(clone!(app, page => move |_: events::Click| {
                                                    Self::load_current_treatment(page.clone(), app.clone());
                                                }))
                                            })
                                        })
                                    })))
                                } else {
                                    dom.class("mb-2")
                                }
                            })
                        }),
                        html!("div", {
                            .class("mb-2")
                            .child(html!("textarea" => HtmlTextAreaElement, {
                                .attr("rows","5")
                                .class(class::FORM_CTRL_SM)
                                .apply(mixins::textarea_value_auto_expand(page.treatment_text.clone(), page.changed.clone()))
                            }))
                        }),
                        html!("div", {
                            .class("mb-2")
                            .child(html!("span", {
                                .class("fw-bold")
                                .text("สาเหตุที่ส่ง")
                            }))
                        }),
                        html!("div", {
                            .class("mb-2")
                            .child(html!("input" => HtmlInputElement, {
                                .attr("type","text")
                                .class(class::FORM_CTRL_SM)
                                .apply(mixins::string_value(page.request_text.clone(), page.changed.clone()))
                            }))
                        }),
                        html!("div", {
                            .class("mb-2")
                            .child(html!("span", {
                                .class("fw-bold")
                                .text("รายละเอียดอื่นๆ")
                            }))
                        }),
                        html!("div", {
                            .class("mb-2")
                            .child(html!("textarea" => HtmlTextAreaElement, {
                                .attr("rows","5")
                                .class(class::FORM_CTRL_SM)
                                .apply(mixins::textarea_value_auto_expand(page.other_text.clone(), page.changed.clone()))
                            }))
                        }),
                    ])
                }))
            }))
        })
    }
}
