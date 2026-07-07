// opd-er-medical-history.php

use dominator::{Dom, clone, events, html};
use futures_signals::{
    map_ref,
    signal::{Mutable, Signal, SignalExt, always, not},
    signal_vec::{MutableVec, SignalVecExt},
};
use rust_decimal::Decimal;
use std::{
    rc::Rc,
    sync::atomic::{AtomicU32, Ordering},
};
use web_sys::{HtmlButtonElement, HtmlInputElement, HtmlSelectElement};

use kphis_model::{
    app::VisitTypeId,
    endpoint::EndPoint,
    fetch::Method,
    image::file_path::ImageUsage,
    opd_er::medical_history::{AllergyHistory, ConsultHistory, NurseScreeningHistory, OpdErMedicalHistory, OpdErMedicalHistoryParams, OpdScreenHistory, ScanHistory, SetFtHistory, TraumaHistory},
    patient_info::PatientInfo,
    timer::Timeout,
};
use kphis_ui_app::App;
use kphis_ui_core::{class, doms, mixins};
use kphis_util::{
    datetime::{JsTime, date_8601, js_now, time_8601},
    util::{str_some, zero_none},
};

use crate::{gadget::image::ImageCpn, opd_er_medical_history::OpdErMedicalHistoryCpn};

static ID_COUNTER: AtomicU32 = AtomicU32::new(0);

/// - GET `EndPoint::OpdErMedicalHistory` (self/MedicalHistoryCpn)
/// - GET `EndPoint::OpdErMedicalHistoryTrauma`
/// - GET `EndPoint::OpdErMedicalHistoryAllergy`
/// - GET `EndPoint::OpdErMedicalHistoryScreen`
/// - GET `EndPoint::OpdErMedicalHistoryScan`
/// - GET `EndPoint::OpdErMedicalHistoryConsult`
/// - GET `EndPoint::OpdErMedicalHistoryFt`
/// - POST `EndPoint::OpdErMedicalHistoryTrauma` (guarded, remove 'Save' btn)
/// - POST `EndPoint::OpdErMedicalHistoryAllergy` (guarded, remove 'เพิ่มประวัติการแพ้ยา','Save' btn)
/// - POST `EndPoint::OpdErMedicalHistoryScreen` (guarded, remove 'Save' btn)
/// - POST `EndPoint::OpdErMedicalHistoryScan` (guarded, remove 'ผู้ป่วยมีเอกสาร' btn)
/// - POST `EndPoint::OpdErMedicalHistoryConsult` (guarded, remove 'เพิ่มฟอร์ม Consult','Save' btn)
/// - POST `EndPoint::OpdErMedicalHistoryFt` (guarded, remove 'Save' btn)
#[derive(Default)]
pub struct OpdErEmergencyCpn {
    patient: Mutable<Option<Rc<PatientInfo>>>,
    view_by: Mutable<String>,

    loaded: Mutable<bool>,
    medical_history: Mutable<Rc<OpdErMedicalHistory>>,

    // trauma = opd_er_dr_pe; opd_er_order_master_id is <u32>
    trauma_loaded: Mutable<bool>,
    trauma_visible: Mutable<bool>,
    trauma_changed: Mutable<bool>,

    opd_er_pe_id: Mutable<u32>,
    arc: Mutable<String>,
    arc_npc_text: Mutable<String>,
    breathing_chest_wall: Mutable<String>,
    breathing_lung: Mutable<String>,
    circulation_shock: Mutable<String>,
    circulation_shock_text: Mutable<String>,
    circulation_other: Mutable<String>,
    circulation_other_text: Mutable<String>,
    circulation_efast_date: Mutable<String>, // Date
    circulation_efast_time: Mutable<String>, // Time
    circulation_doctor: Mutable<String>,
    circulation: Mutable<String>,
    circulation_positive_text: Mutable<String>,
    disability_e: Mutable<String>, // i32
    disability_v: Mutable<String>,
    disability_m: Mutable<String>,        // i32
    disability_pupil_rt: Mutable<String>, // Decimal
    disability_pupil_lt: Mutable<String>, // Decimal
    disability_other: Mutable<String>,
    exposure: Mutable<String>,
    doctor_pe: Mutable<String>,

    trauma_version: Mutable<i32>,
    trauma_doctor_name: Mutable<String>,
    trauma_saving: Mutable<bool>,
    trauma_save_result: Mutable<Option<bool>>,

    // allergy; opd_er_order_master_id is Option<u32>
    allergy_loaded: Mutable<bool>,
    allergy_visible: Mutable<bool>,
    allergy_changed: Mutable<bool>,

    er_allergy_history_items: MutableVec<Rc<DrugAllergy>>,
    er_allergy_history_doctorcode: Mutable<String>,

    allergy_version: Mutable<i32>,
    allergy_doctor_name: Mutable<String>,
    allergy_saving: Mutable<bool>,
    allergy_save_result: Mutable<Option<bool>>,

    // nurse_screening; opd_er_order_master_id is <u32>
    screen_loaded: Mutable<bool>,
    // screen_visible: Mutable<bool>,
    screen_changed: Mutable<bool>,

    opd_er_screening_id: Mutable<u32>,
    screening_emergency_level: Mutable<String>,
    screening_spclty: Mutable<String>,
    screening_arrive_date: Mutable<String>,     // Date
    screening_arrive_time: Mutable<String>,     // Time
    screening_date: Mutable<String>,            // Date
    screening_time: Mutable<String>,            // Time
    screening_report_date: Mutable<String>,     // Date
    screening_report_time: Mutable<String>,     // Time
    screening_see_doctor_date: Mutable<String>, // Date
    screening_see_doctor_time: Mutable<String>, // Time
    screening_doctor_doctorcode: Mutable<String>,
    screening_nurse_doctorcode: Mutable<String>,

    screen_version: Mutable<i32>,
    screen_nurse_name: Mutable<String>,
    screen_doctor_name: Mutable<String>,
    screen_saving: Mutable<bool>,
    screen_save_result: Mutable<Option<bool>>,

    // has_scan; opd_er_order_master_id is <u32>
    scan_loaded: Mutable<bool>,

    opd_er_document_scan_id: Mutable<u32>,
    opd_er_document_scan: Mutable<String>,
    opd_er_document_scan_doctorcode: Mutable<String>,

    scan_version: Mutable<i32>,
    scan_saving: Mutable<bool>,
    scan_save_result: Mutable<Option<bool>>,

    // consult; opd_er_order_master_id is Option<u32>
    consult_loaded: Mutable<bool>,
    consult_visible: Mutable<bool>,
    consult_changed: Mutable<bool>,

    er_consult_items: MutableVec<Rc<ConsultItem>>,
    er_consult_doctorcode: Mutable<String>,

    consult_version: Mutable<i32>,
    consult_doctor_name: Mutable<String>,
    consult_saving: Mutable<bool>,
    consult_save_result: Mutable<Option<bool>>,

    // set_ft; opd_er_order_master_id is Option<u32>
    ft_loaded: Mutable<bool>,
    ft_visible: Mutable<bool>,
    ft_changed: Mutable<bool>,

    set_ft_id: Mutable<u32>,
    set_ft_date: Mutable<String>, // Date
    set_ft_time: Mutable<String>, // Time
    set_ft_doctorcode: Mutable<String>,

    ft_version: Mutable<i32>,
    ft_doctor_name: Mutable<String>,
    ft_saving: Mutable<bool>,
    ft_save_result: Mutable<Option<bool>>,
}

impl OpdErEmergencyCpn {
    pub fn new(patient: Mutable<Option<Rc<PatientInfo>>>, view_by: Mutable<String>) -> Rc<Self> {
        Rc::new(Self {
            patient,
            view_by,
            ..Default::default()
        })
    }

    fn is_doctor(&self) -> impl Signal<Item = bool> + use<> {
        self.view_by.signal_cloned().map(|view_by| view_by == "doctor")
    }

    fn is_nurse(&self) -> impl Signal<Item = bool> + use<> {
        self.view_by.signal_cloned().map(|view_by| view_by == "nurse")
    }

    fn is_nurse_screen_after_doctor(&self) -> impl Signal<Item = bool> + use<> {
        map_ref! {
            let is_nurse = self.is_nurse(),
            let screen_doctor_name = self.screen_doctor_name.signal_cloned() =>
            *is_nurse && !screen_doctor_name.is_empty()
        }
    }

    fn load(page: Rc<Self>, app: Rc<App>) {
        if let Some(patient) = page.patient.get_cloned() {
            if let VisitTypeId::OpdEr(_vn, opd_er_order_master_id) = patient.visit_type() {
                app.async_load(
                    true,
                    clone!(app, page => async move {
                        let visit_datetime = if let (Some(regdate), Some(regtime)) = (patient.regdate(), patient.regtime()) {
                            Some([regdate.to_string(), regtime.js_string()].join(" "))
                        } else {
                            None
                        };
                        let params = OpdErMedicalHistoryParams {
                            opd_er_order_master_id: zero_none(opd_er_order_master_id),
                            hn: patient.hn(),
                            vn: patient.vn(),
                            visit_datetime,
                            // age_y: zero_none(page.age_y.get()),
                            ..Default::default()
                        };
                        // GET `EndPoint::OpdErMedicalHistory`
                        match OpdErMedicalHistory::call_api_get(&params, app.state()).await {
                            Ok(response) => {
                                page.medical_history.set(Rc::new(response));
                            }
                            Err(e) => {
                                app.alert_app_error(&e).await;
                            }
                        }
                    }),
                )
            }
        }
    }

    fn new_trauma(page: Rc<Self>) {
        page.opd_er_pe_id.set_neq(0);
        page.arc.set_neq(String::new());
        page.arc_npc_text.set_neq(String::new());
        page.breathing_chest_wall.set_neq(String::new());
        page.breathing_lung.set_neq(String::new());
        page.circulation_shock.set_neq(String::new());
        page.circulation_shock_text.set_neq(String::new());
        page.circulation_other.set_neq(String::new());
        page.circulation_other_text.set_neq(String::new());
        page.circulation_efast_date.set_neq(String::new());
        page.circulation_efast_time.set_neq(String::new());
        page.circulation_doctor.set_neq(String::new());
        page.circulation.set_neq(String::new());
        page.circulation_positive_text.set_neq(String::new());
        page.disability_e.set_neq(String::new());
        page.disability_v.set_neq(String::new());
        page.disability_m.set_neq(String::new());
        page.disability_pupil_rt.set_neq(String::new());
        page.disability_pupil_lt.set_neq(String::new());
        page.disability_other.set_neq(String::new());
        page.exposure.set_neq(String::new());
        page.doctor_pe.set_neq(String::new());
        page.trauma_doctor_name.set_neq(String::new());
        page.trauma_version.set_neq(0);
        page.trauma_changed.set_neq(false);
    }

    fn load_trauma(page: Rc<Self>, app: Rc<App>) {
        let visit_type = page.patient.lock_ref().as_ref().map(|pt| pt.visit_type());
        if let Some(VisitTypeId::OpdEr(_vn, opd_er_order_master_id)) = visit_type {
            let params = OpdErMedicalHistoryParams {
                opd_er_order_master_id: zero_none(opd_er_order_master_id),
                ..Default::default()
            };
            app.async_load(
                true,
                clone!(app, page => async move {
                    // GET `EndPoint::OpdErMedicalHistoryTrauma`
                    match TraumaHistory::call_api_get(&params, app.state()).await {
                        Ok(Some(response)) => {
                            page.opd_er_pe_id.set_neq(response.opd_er_pe_id);
                            page.arc.set_neq(response.arc.clone().unwrap_or_default());
                            page.arc_npc_text.set_neq(response.arc_npc_text.clone().unwrap_or_default());
                            page.breathing_chest_wall.set_neq(response.breathing_chest_wall.clone().unwrap_or_default());
                            page.breathing_lung.set_neq(response.breathing_lung.clone().unwrap_or_default());
                            page.circulation_shock.set_neq(response.circulation_shock.clone().unwrap_or_default());
                            page.circulation_shock_text.set_neq(response.circulation_shock_text.clone().unwrap_or_default());
                            page.circulation_other.set_neq(response.circulation_other.clone().unwrap_or_default());
                            page.circulation_other_text.set_neq(response.circulation_other_text.clone().unwrap_or_default());
                            page.circulation_efast_date.set_neq(response.circulation_efast_date.map(|d| d.to_string()).unwrap_or_default());
                            page.circulation_efast_time.set_neq(response.circulation_efast_time.map(|t| t.js_string()).unwrap_or_default());
                            page.circulation_doctor.set_neq(response.circulation_doctor.clone().unwrap_or_default());
                            page.circulation.set_neq(response.circulation.clone().unwrap_or_default());
                            page.circulation_positive_text.set_neq(response.circulation_positive_text.clone().unwrap_or_default());
                            page.disability_e.set_neq(response.disability_e.map(|i| i.to_string()).unwrap_or_default());
                            page.disability_v.set_neq(response.disability_v.clone().unwrap_or_default());
                            page.disability_m.set_neq(response.disability_m.map(|i| i.to_string()).unwrap_or_default());
                            page.disability_pupil_rt.set_neq(response.disability_pupil_rt.map(|d| d.to_string()).unwrap_or_default());
                            page.disability_pupil_lt.set_neq(response.disability_pupil_lt.map(|d| d.to_string()).unwrap_or_default());
                            page.disability_other.set_neq(response.disability_other.clone().unwrap_or_default());
                            page.exposure.set_neq(response.exposure.clone().unwrap_or_default());
                            page.doctor_pe.set_neq(response.doctor_pe.clone().unwrap_or_default());
                            page.trauma_doctor_name.set_neq(response.doctor_name.clone().unwrap_or_default());
                            page.trauma_version.set_neq(response.version);
                            page.trauma_changed.set_neq(false);
                        }
                        Ok(None) => {
                            Self::new_trauma(page);
                        }
                        Err(e) => {
                            app.alert_app_error(&e).await;
                        }
                    }
                }),
            )
        }
    }

    fn save_trauma(page: Rc<Self>, app: Rc<App>) {
        let visit_type = page.patient.lock_ref().as_ref().map(|pt| pt.visit_type());
        if let Some(VisitTypeId::OpdEr(_vn, opd_er_order_master_id)) = visit_type {
            let trauma = TraumaHistory {
                opd_er_pe_id: page.opd_er_pe_id.get(),
                opd_er_order_master_id,
                arc: str_some(page.arc.get_cloned()),
                arc_npc_text: str_some(page.arc_npc_text.get_cloned()),
                breathing_chest_wall: str_some(page.breathing_chest_wall.get_cloned()),
                breathing_lung: str_some(page.breathing_lung.get_cloned()),
                circulation_shock: str_some(page.circulation_shock.get_cloned()),
                circulation_shock_text: str_some(page.circulation_shock_text.get_cloned()),
                circulation_other: str_some(page.circulation_other.get_cloned()),
                circulation_other_text: str_some(page.circulation_other_text.get_cloned()),
                circulation_efast_date: date_8601(&page.circulation_efast_date.lock_ref()),
                circulation_efast_time: time_8601(&page.circulation_efast_time.lock_ref()),
                circulation_doctor: str_some(page.circulation_doctor.get_cloned()),
                circulation_doctor_name: None, // report use only
                circulation: str_some(page.circulation.get_cloned()),
                circulation_positive_text: str_some(page.circulation_positive_text.get_cloned()),
                disability_e: page.disability_e.lock_ref().parse::<i32>().ok(),
                disability_v: str_some(page.disability_v.get_cloned()),
                disability_m: page.disability_m.lock_ref().parse::<i32>().ok(),
                disability_pupil_rt: Decimal::from_str_exact(&page.disability_pupil_rt.lock_ref()).ok(),
                disability_pupil_lt: Decimal::from_str_exact(&page.disability_pupil_lt.lock_ref()).ok(),
                disability_other: str_some(page.disability_other.get_cloned()),
                exposure: str_some(page.exposure.get_cloned()),
                doctor_pe: str_some(page.doctor_pe.get_cloned()),
                doctor_name: str_some(page.trauma_doctor_name.get_cloned()),
                // not use when insert/update
                imgs: None,
                version: page.trauma_version.get(),
            };

            page.trauma_saving.set(true);
            app.async_load(
                true,
                clone!(app => async move {
                    // POST `EndPoint::OpdErMedicalHistoryTrauma`
                    match trauma.call_api_post(app.state()).await {
                        Ok((id, responses)) => {
                            if !responses.is_empty() && responses[0].rows_affected > 0 {
                                page.opd_er_pe_id.set_neq(id);
                                page.trauma_doctor_name.set_neq(app.doctor_name().unwrap_or_default());
                                page.trauma_changed.set_neq(false);
                                page.trauma_save_result.set(Some(true));
                            } else {
                                page.trauma_save_result.set(Some(false));
                            }
                        }
                        Err(e) => {
                            app.alert_app_error(&e).await;
                            page.trauma_save_result.set(Some(false));
                        }
                    }
                    page.trauma_saving.set(false);
                    Timeout::new(2000, move || {
                        page.trauma_save_result.set(None);
                    }).forget();
                }),
            )
        }
    }

    fn new_allergy(page: Rc<Self>) {
        page.er_allergy_history_items.lock_mut().clear();
        page.er_allergy_history_doctorcode.set_neq(String::new());
        page.allergy_doctor_name.set_neq(String::new());
        page.allergy_version.set_neq(0);
        page.allergy_changed.set_neq(false);
    }

    fn load_allergy(page: Rc<Self>, app: Rc<App>) {
        let visit_type = page.patient.lock_ref().as_ref().map(|pt| pt.visit_type());
        if let Some(VisitTypeId::OpdEr(_vn, opd_er_order_master_id)) = visit_type {
            let params = OpdErMedicalHistoryParams {
                opd_er_order_master_id: zero_none(opd_er_order_master_id),
                ..Default::default()
            };
            app.async_load(
                true,
                clone!(app, page => async move {
                    // GET `EndPoint::OpdErMedicalHistoryAllergy`
                    match AllergyHistory::call_api_get(&params, app.state()).await {
                        Ok(responses) => {
                            if let Some(item) = responses.last() {
                                let allergies = responses.iter().map(DrugAllergy::from).map(Rc::new);
                                {
                                    let mut lock = page.er_allergy_history_items.lock_mut();
                                    lock.clear();
                                    lock.extend(allergies);
                                }
                                page.er_allergy_history_doctorcode.set_neq(item.er_allergy_history_doctorcode.clone().unwrap_or_default());
                                page.allergy_doctor_name.set_neq(item.doctor_name.clone().unwrap_or_default());
                                page.allergy_version.set_neq(item.version);
                                page.allergy_changed.set_neq(false);
                            } else {
                                Self::new_allergy(page);
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

    fn save_allergy(page: Rc<Self>, app: Rc<App>) {
        let visit_type = page.patient.lock_ref().as_ref().map(|pt| pt.visit_type());
        if let Some(VisitTypeId::OpdEr(_vn, opd_er_order_master_id)) = visit_type {
            let er_allergy_history_doctorcode = str_some(page.er_allergy_history_doctorcode.get_cloned());
            let doctor_name = str_some(page.allergy_doctor_name.get_cloned());
            let version = page.allergy_version.get();

            let allergies = page
                .er_allergy_history_items
                .lock_ref()
                .iter()
                .map(|r| AllergyHistory {
                    er_allergy_history_id: 0,
                    opd_er_order_master_id: zero_none(opd_er_order_master_id),
                    er_allergy_history_agent: str_some(r.agent.get_cloned()),
                    er_allergy_history_symptom: str_some(r.symptom.get_cloned()),
                    er_allergy_history_doctorcode: er_allergy_history_doctorcode.clone(),
                    doctor_name: doctor_name.clone(),
                    version,
                })
                .collect::<Vec<AllergyHistory>>();

            page.allergy_saving.set(true);
            app.async_load(
                true,
                clone!(app => async move {
                    // POST `EndPoint::OpdErMedicalHistoryAllergy`
                    match AllergyHistory::call_api_post(&allergies, app.state()).await {
                        Ok(responses) => {
                            if !responses.is_empty() && responses[0].rows_affected > 0 {
                                page.allergy_doctor_name.set_neq(app.doctor_name().unwrap_or_default());
                                page.allergy_changed.set_neq(false);
                                page.allergy_save_result.set(Some(true));
                            } else {
                                page.allergy_save_result.set(Some(false));
                            }
                        }
                        Err(e) => {
                            app.alert_app_error(&e).await;
                            page.allergy_save_result.set(Some(false));
                        }
                    }
                    page.allergy_saving.set(false);
                    Timeout::new(2000, move || {
                        page.allergy_save_result.set(None);
                    }).forget();
                }),
            )
        }
    }

    fn new_screen(page: Rc<Self>) {
        page.opd_er_screening_id.set_neq(0);
        page.screening_emergency_level.set_neq(String::new());
        page.screening_spclty.set_neq(String::new());
        page.screening_arrive_date.set_neq(String::new());
        page.screening_arrive_time.set_neq(String::new());
        page.screening_date.set_neq(String::new());
        page.screening_time.set_neq(String::new());
        page.screening_report_date.set_neq(String::new());
        page.screening_report_time.set_neq(String::new());
        page.screening_see_doctor_date.set_neq(String::new());
        page.screening_see_doctor_time.set_neq(String::new());
        page.screening_doctor_doctorcode.set_neq(String::new());
        page.screening_nurse_doctorcode.set_neq(String::new());
        page.screen_doctor_name.set_neq(String::new());
        page.screen_nurse_name.set_neq(String::new());
        page.screen_version.set_neq(0);
        page.screen_changed.set_neq(false);
    }

    fn load_screen(page: Rc<Self>, app: Rc<App>) {
        let visit_type = page.patient.lock_ref().as_ref().map(|pt| pt.visit_type());
        if let Some(VisitTypeId::OpdEr(_vn, opd_er_order_master_id)) = visit_type {
            let params = OpdErMedicalHistoryParams {
                opd_er_order_master_id: zero_none(opd_er_order_master_id),
                ..Default::default()
            };
            app.async_load(
                true,
                clone!(app, page => async move {
                    // GET `EndPoint::OpdErMedicalHistoryScreen`
                    match NurseScreeningHistory::call_api_get(&params, app.state()).await {
                        Ok(Some(response)) => {
                            page.opd_er_screening_id.set_neq(response.opd_er_screening_id);
                            page.screening_emergency_level.set_neq(response.screening_emergency_level.clone().unwrap_or_default());
                            page.screening_spclty.set_neq(response.screening_spclty.clone().unwrap_or_default());
                            page.screening_arrive_date.set_neq(response.screening_arrive_date.map(|d| d.to_string()).unwrap_or_default());
                            page.screening_arrive_time.set_neq(response.screening_arrive_time.map(|t| t.js_string()).unwrap_or_default());
                            page.screening_date.set_neq(response.screening_date.map(|d| d.to_string()).unwrap_or_default());
                            page.screening_time.set_neq(response.screening_time.map(|t| t.js_string()).unwrap_or_default());
                            page.screening_report_date.set_neq(response.screening_report_date.map(|d| d.to_string()).unwrap_or_default());
                            page.screening_report_time.set_neq(response.screening_report_time.map(|t| t.js_string()).unwrap_or_default());
                            page.screening_see_doctor_date.set_neq(response.screening_see_doctor_date.map(|d| d.to_string()).unwrap_or_default());
                            page.screening_see_doctor_time.set_neq(response.screening_see_doctor_time.map(|t| t.js_string()).unwrap_or_default());
                            page.screening_doctor_doctorcode.set_neq(response.screening_doctor_doctorcode.clone().unwrap_or_default());
                            page.screening_nurse_doctorcode.set_neq(response.screening_nurse_doctorcode.clone().unwrap_or_default());
                            page.screen_doctor_name.set_neq(response.doctor_name.clone().unwrap_or_default());
                            page.screen_nurse_name.set_neq(response.nurse_name.clone().unwrap_or_default());
                            page.screen_version.set_neq(response.version);
                            page.screen_changed.set_neq(false);
                        }
                        Ok(None) => {
                            Self::new_screen(page);
                        }
                        Err(e) => {
                            app.alert_app_error(&e).await;
                        }
                    }
                }),
            )
        }
    }

    fn save_screen(page: Rc<Self>, app: Rc<App>) {
        let visit_type = page.patient.lock_ref().as_ref().map(|pt| pt.visit_type());
        if let Some(VisitTypeId::OpdEr(_vn, opd_er_order_master_id)) = visit_type {
            if let Some(view_by) = str_some(page.view_by.get_cloned()) {
                if ["doctor", "nurse"].contains(&view_by.as_str()) {
                    let params = OpdErMedicalHistoryParams {
                        view_by: Some(view_by.clone()),
                        ..Default::default()
                    };
                    let screen = NurseScreeningHistory {
                        opd_er_screening_id: page.opd_er_screening_id.get(),
                        opd_er_order_master_id,
                        screening_emergency_level: str_some(page.screening_emergency_level.get_cloned()),
                        screening_spclty: str_some(page.screening_spclty.get_cloned()),
                        screening_arrive_date: date_8601(&page.screening_arrive_date.lock_ref()),
                        screening_arrive_time: time_8601(&page.screening_arrive_time.lock_ref()),
                        screening_date: date_8601(&page.screening_date.lock_ref()),
                        screening_time: time_8601(&page.screening_time.lock_ref()),
                        screening_report_date: date_8601(&page.screening_report_date.lock_ref()),
                        screening_report_time: time_8601(&page.screening_report_time.lock_ref()),
                        screening_see_doctor_date: date_8601(&page.screening_see_doctor_date.lock_ref()),
                        screening_see_doctor_time: time_8601(&page.screening_see_doctor_time.lock_ref()),
                        screening_doctor_doctorcode: str_some(page.screening_doctor_doctorcode.get_cloned()),
                        screening_nurse_doctorcode: str_some(page.screening_nurse_doctorcode.get_cloned()),
                        nurse_name: str_some(page.screen_nurse_name.get_cloned()),
                        doctor_name: str_some(page.screen_doctor_name.get_cloned()),
                        version: page.screen_version.get(),
                    };

                    page.screen_saving.set(true);
                    app.async_load(
                        true,
                        clone!(app => async move {
                            // POST `EndPoint::OpdErMedicalHistoryScreen`
                            match screen.call_api_post(&params, app.state()).await {
                                Ok((id, responses)) => {
                                    if !responses.is_empty() && responses[0].rows_affected > 0 {
                                        page.opd_er_screening_id.set_neq(id);
                                        if view_by == "doctor" {
                                            page.screen_doctor_name.set_neq(app.doctor_name().unwrap_or_default());
                                        } else {
                                            page.screen_nurse_name.set_neq(app.doctor_name().unwrap_or_default());
                                        }
                                        page.screen_changed.set_neq(false);
                                        page.screen_save_result.set(Some(true));
                                    } else {
                                        page.screen_save_result.set(Some(false));
                                    }
                                }
                                Err(e) => {
                                    app.alert_app_error(&e).await;
                                    page.screen_save_result.set(Some(false));
                                }
                            }
                            page.screen_saving.set(false);
                            Timeout::new(2000, move || {
                                page.screen_save_result.set(None);
                            }).forget();
                        }),
                    );
                }
            }
        }
    }

    fn new_scan(page: Rc<Self>) {
        page.opd_er_document_scan_id.set_neq(0);
        page.opd_er_document_scan.set_neq(String::new());
        page.opd_er_document_scan_doctorcode.set_neq(String::new());
        page.scan_version.set_neq(0);
    }

    fn load_scan(page: Rc<Self>, app: Rc<App>) {
        let visit_type = page.patient.lock_ref().as_ref().map(|pt| pt.visit_type());
        if let Some(VisitTypeId::OpdEr(_vn, opd_er_order_master_id)) = visit_type {
            let params = OpdErMedicalHistoryParams {
                opd_er_order_master_id: zero_none(opd_er_order_master_id),
                ..Default::default()
            };
            app.async_load(
                true,
                clone!(app, page => async move {
                    // GET `EndPoint::OpdErMedicalHistoryScan`
                    match ScanHistory::call_api_get(&params, app.state()).await {
                        Ok(Some(response)) => {
                            page.opd_er_document_scan_id.set_neq(response.opd_er_document_scan_id);
                            page.opd_er_document_scan.set_neq(response.opd_er_document_scan.clone().unwrap_or_default());
                            page.opd_er_document_scan_doctorcode.set_neq(response.opd_er_document_scan_doctorcode.clone().unwrap_or_default());
                            page.scan_version.set_neq(response.version);
                        }
                        Ok(None) => {
                            Self::new_scan(page);
                        }
                        Err(e) => {
                            app.alert_app_error(&e).await;
                        }
                    }
                }),
            )
        }
    }

    fn save_scan(page: Rc<Self>, app: Rc<App>) {
        let visit_type = page.patient.lock_ref().as_ref().map(|pt| pt.visit_type());
        if let Some(VisitTypeId::OpdEr(_vn, opd_er_order_master_id)) = visit_type {
            let scan = ScanHistory {
                opd_er_document_scan_id: page.opd_er_document_scan_id.get(),
                opd_er_order_master_id,
                opd_er_document_scan: str_some(page.opd_er_document_scan.get_cloned()),
                opd_er_document_scan_doctorcode: str_some(page.opd_er_document_scan_doctorcode.get_cloned()),
                version: page.scan_version.get(),
            };

            // page.scan_saving.set(true);
            app.async_load(
                true,
                clone!(app => async move {
                    // POST `EndPoint::OpdErMedicalHistoryScan`
                    match scan.call_api_post(app.state()).await {
                        Ok((id, responses)) => {
                            if !responses.is_empty() && responses[0].rows_affected > 0 {
                                page.opd_er_document_scan_id.set_neq(id);
                                page.scan_save_result.set(Some(true));
                            } else {
                                page.scan_save_result.set(Some(false));
                            }
                        }
                        Err(e) => {
                            app.alert_app_error(&e).await;
                            page.scan_save_result.set(Some(false));
                        }
                    }
                    page.scan_saving.set(false);
                    Timeout::new(2000, move || {
                        page.scan_save_result.set(None);
                    }).forget();
                }),
            );
        }
    }

    fn new_consult(page: Rc<Self>) {
        page.er_consult_items.lock_mut().clear();
        page.er_consult_doctorcode.set_neq(String::new());
        page.consult_doctor_name.set_neq(String::new());
        page.consult_version.set_neq(0);
        page.consult_changed.set_neq(false);
    }

    fn load_consult(page: Rc<Self>, app: Rc<App>) {
        let visit_type = page.patient.lock_ref().as_ref().map(|pt| pt.visit_type());
        if let Some(VisitTypeId::OpdEr(_vn, opd_er_order_master_id)) = visit_type {
            let params = OpdErMedicalHistoryParams {
                opd_er_order_master_id: zero_none(opd_er_order_master_id),
                ..Default::default()
            };
            app.async_load(
                true,
                clone!(app, page => async move {
                    // GET `EndPoint::OpdErMedicalHistoryConsult`
                    match ConsultHistory::call_api_get(&params, app.state()).await {
                        Ok(responses) => {
                            if let Some(item) = responses.last() {
                                let consults = responses.iter().map(ConsultItem::from).map(Rc::new);
                                {
                                    let mut lock = page.er_consult_items.lock_mut();
                                    lock.clear();
                                    lock.extend(consults);
                                }
                                page.er_consult_doctorcode.set_neq(item.er_consult_doctorcode.clone().unwrap_or_default());
                                page.consult_doctor_name.set_neq(item.doctor_name.clone().unwrap_or_default());
                                page.consult_version.set_neq(item.version);
                                page.consult_changed.set_neq(false);
                            } else {
                                Self::new_consult(page);
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

    fn save_consult(page: Rc<Self>, app: Rc<App>) {
        let visit_type = page.patient.lock_ref().as_ref().map(|pt| pt.visit_type());
        if let Some(VisitTypeId::OpdEr(_vn, opd_er_order_master_id)) = visit_type {
            let er_consult_doctorcode = str_some(page.er_consult_doctorcode.get_cloned());
            let doctor_name = str_some(page.consult_doctor_name.get_cloned());
            let version = page.consult_version.get();

            let consults = page
                .er_consult_items
                .lock_ref()
                .iter()
                .map(|c| {
                    ConsultHistory {
                        er_consult_id: 0,
                        opd_er_order_master_id: zero_none(opd_er_order_master_id),
                        er_consult_ward: str_some(c.er_consult_ward.get_cloned()),
                        er_consult_ward_name: None, // report use only
                        er_consult_date: date_8601(&c.er_consult_date.lock_ref()),
                        er_consult_time: time_8601(&c.er_consult_time.lock_ref()),
                        er_consult_doctor_reply: str_some(c.er_consult_doctor_reply.get_cloned()),
                        er_consult_date_reply: date_8601(&c.er_consult_date_reply.lock_ref()),
                        er_consult_time_reply: time_8601(&c.er_consult_time_reply.lock_ref()),
                        er_consult_doctorcode: er_consult_doctorcode.clone(),
                        doctor_name: doctor_name.clone(),
                        version,
                    }
                })
                .collect::<Vec<ConsultHistory>>();

            page.consult_saving.set(true);
            app.async_load(
                true,
                clone!(app => async move {
                    // POST `EndPoint::OpdErMedicalHistoryConsult`
                    match ConsultHistory::call_api_post(&consults, app.state()).await {
                        Ok(responses) => {
                            if !responses.is_empty() && responses[0].rows_affected > 0 {
                                page.consult_doctor_name.set_neq(app.doctor_name().unwrap_or_default());
                                page.consult_changed.set_neq(false);
                                page.consult_save_result.set(Some(true));
                            } else {
                                page.consult_save_result.set(Some(false));
                            }
                        }
                        Err(e) => {
                            app.alert_app_error(&e).await;
                            page.consult_save_result.set(Some(false));
                        }
                    }
                    page.consult_saving.set(false);
                    Timeout::new(2000, move || {
                        page.consult_save_result.set(None);
                    }).forget();
                }),
            )
        }
    }

    fn new_ft(page: Rc<Self>) {
        page.set_ft_id.set_neq(0);
        page.set_ft_date.set_neq(String::new()); // Date
        page.set_ft_time.set_neq(String::new()); // Time
        page.set_ft_doctorcode.set_neq(String::new());
        page.ft_doctor_name.set_neq(String::new());
        page.ft_version.set_neq(0);
        page.ft_changed.set_neq(false);
    }

    fn load_ft(page: Rc<Self>, app: Rc<App>) {
        let visit_type = page.patient.lock_ref().as_ref().map(|pt| pt.visit_type());
        if let Some(VisitTypeId::OpdEr(_vn, opd_er_order_master_id)) = visit_type {
            let params = OpdErMedicalHistoryParams {
                opd_er_order_master_id: zero_none(opd_er_order_master_id),
                ..Default::default()
            };
            app.async_load(
                true,
                clone!(app, page => async move {
                    // GET `EndPoint::OpdErMedicalHistoryFt`
                    match SetFtHistory::call_api_get(&params, app.state()).await {
                        Ok(Some(response)) => {
                            page.set_ft_id.set_neq(response.set_ft_id);
                            page.set_ft_date.set_neq(response.set_ft_date.map(|d| d.to_string()).unwrap_or_default()); // Date
                            page.set_ft_time.set_neq(response.set_ft_time.map(|t| t.js_string()).unwrap_or_default()); // Time
                            page.set_ft_doctorcode.set_neq(response.set_ft_doctorcode.clone().unwrap_or_default());
                            page.ft_doctor_name.set_neq(response.doctor_name.clone().unwrap_or_default());
                            page.ft_version.set_neq(response.version);
                            page.ft_changed.set_neq(false);
                        }
                        Ok(None) => {
                            Self::new_ft(page);
                        }
                        Err(e) => {
                            app.alert_app_error(&e).await;
                        }
                    }
                }),
            )
        }
    }

    fn save_ft(page: Rc<Self>, app: Rc<App>) {
        let visit_type = page.patient.lock_ref().as_ref().map(|pt| pt.visit_type());
        if let Some(VisitTypeId::OpdEr(_vn, opd_er_order_master_id)) = visit_type {
            let scan = SetFtHistory {
                set_ft_id: page.set_ft_id.get(),
                opd_er_order_master_id: zero_none(opd_er_order_master_id),
                set_ft_date: date_8601(&page.set_ft_date.lock_ref()),
                set_ft_time: time_8601(&page.set_ft_time.lock_ref()),
                set_ft_doctorcode: str_some(page.set_ft_doctorcode.get_cloned()),
                doctor_name: str_some(page.ft_doctor_name.get_cloned()),
                version: page.ft_version.get(),
            };

            page.ft_saving.set(true);
            app.async_load(
                true,
                clone!(app => async move {
                    // POST `EndPoint::OpdErMedicalHistoryFt`
                    match scan.call_api_post(app.state()).await {
                        Ok((id, responses)) => {
                            if !responses.is_empty() && responses[0].rows_affected > 0 {
                                page.set_ft_id.set_neq(id);
                                page.ft_doctor_name.set_neq(app.doctor_name().unwrap_or_default());
                                page.ft_changed.set_neq(false);
                                page.ft_save_result.set(Some(true));
                            } else {
                                page.ft_save_result.set(Some(false));
                            }
                        }
                        Err(e) => {
                            app.alert_app_error(&e).await;
                            page.ft_save_result.set(Some(false));
                        }
                    }
                    page.ft_saving.set(false);
                    Timeout::new(2000, move || {
                        page.ft_save_result.set(None);
                    }).forget();
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
                    Self::load(page.clone(), app.clone());
                    page.loaded.set_neq(true);
                }
                async {}
            })))
            .future(map_ref!(
                let busy = app.loader_is_loading(),
                let loaded = page.trauma_loaded.signal() =>
                !busy && !loaded
            ).for_each(clone!(app, page => move |ready| {
                if ready {
                    Self::load_trauma(page.clone(), app.clone());
                    page.trauma_loaded.set_neq(true);
                }
                async {}
            })))
            .future(map_ref!(
                let busy = app.loader_is_loading(),
                let loaded = page.allergy_loaded.signal() =>
                !busy && !loaded
            ).for_each(clone!(app, page => move |ready| {
                if ready {
                    Self::load_allergy(page.clone(), app.clone());
                    page.allergy_loaded.set_neq(true);
                }
                async {}
            })))
            .future(map_ref!(
                let busy = app.loader_is_loading(),
                let loaded = page.screen_loaded.signal() =>
                !busy && !loaded
            ).for_each(clone!(app, page => move |ready| {
                if ready {
                    Self::load_screen(page.clone(), app.clone());
                    page.screen_loaded.set_neq(true);
                }
                async {}
            })))
            .future(map_ref!(
                let busy = app.loader_is_loading(),
                let loaded = page.scan_loaded.signal() =>
                !busy && !loaded
            ).for_each(clone!(app, page => move |ready| {
                if ready {
                    Self::load_scan(page.clone(), app.clone());
                    page.scan_loaded.set_neq(true);
                }
                async {}
            })))
            .future(map_ref!(
                let busy = app.loader_is_loading(),
                let loaded = page.consult_loaded.signal() =>
                !busy && !loaded
            ).for_each(clone!(app, page => move |ready| {
                if ready {
                    Self::load_consult(page.clone(), app.clone());
                    page.consult_loaded.set_neq(true);
                }
                async {}
            })))
            .future(map_ref!(
                let busy = app.loader_is_loading(),
                let loaded = page.ft_loaded.signal() =>
                !busy && !loaded
            ).for_each(clone!(app, page => move |ready| {
                if ready {
                    Self::load_ft(page.clone(), app.clone());
                    page.ft_loaded.set_neq(true);
                }
                async {}
            })))
            .child(html!("div", {
                .style("column-width","720px")
                .style("column-gap","8px")
                .child(OpdErMedicalHistoryCpn::render(OpdErMedicalHistoryCpn::new(
                    page.patient.clone(),
                    Some(page.medical_history.clone()),
                ), None, app.clone()))
                .child_signal(page.medical_history.signal_cloned().map(|hx| hx.opdscreen.as_ref().map(Self::render_pe_hosxp)))
                .child(Self::render_trauma(page.clone(), app.clone()))
                .child_signal(page.patient.signal_cloned().map(clone!(page, app => move |opt| {
                    opt.and_then(|pt| {
                        if let VisitTypeId::OpdEr(_vn, opd_er_order_master_id) = pt.visit_type() {
                            // now ER freely edit image
                            Some(html!("div", {
                                .child(ImageCpn::render("300px", ImageCpn::new_with_key(
                                    ImageUsage::OpdErMedicalHistory,
                                    opd_er_order_master_id,
                                    true, // allow everyone to edit
                                    page.patient.clone(),
                                    page.patient.lock_ref().as_ref().map(|pt| pt.visit_type.vnan().to_owned()),
                                    "", // will use ImageUsage internally, so we add nothing here
                                ), app.clone()))
                            }))
                        } else {
                            None
                        }
                    })
                })))
                .children([
                    Self::render_allergy(page.clone(), app.clone()),
                    Self::render_nurse_screening(page.clone(), app.clone()),
                    Self::render_has_scan(page.clone(), app.clone()),
                    Self::render_consult(page.clone(), app.clone()),
                    Self::render_set_ft(page.clone(), app.clone()),
                ])
            }))
        })
    }

    // pe_hosxp - read
    pub fn render_pe_hosxp(os: &OpdScreenHistory) -> Dom {
        html!("div", {
            .class(class::CARD)
            .style("break-inside","avoid")
            .child(html!("div", {
                .class("card-body")
                .children([
                    html!("div", {
                        .class(class::BOLD_T3L)
                        .child(html!("i", {.class(class::FA_CLIPBOARD)}))
                        .text(" Physical Exmination (HOSxP)")
                    }),
                    html!("div", {
                        .class("ms-3")
                        .children([
                            html!("span", {.class("fw-bold").text("GA : ")}),
                            html!("span", {.text(&os.pe_ga_text.clone().unwrap_or_default())}),
                        ])
                    }),
                    render_pe("HEENT", &os.pe_heent, &os.pe_heent_text),
                    render_pe("Heart", &os.pe_heart, &os.pe_heart_text),
                    render_pe("Lung", &os.pe_lung, &os.pe_lung_text),
                    render_pe("Abdomen", &os.pe_ab, &os.pe_ab_text),
                    render_pe("Ext", &os.pe_ext, &os.pe_ext_text),
                    render_pe("Neuro", &os.pe_neuro, &os.pe_neuro_text),
                    html!("div", {
                        .class("ms-3")
                        .children([
                            html!("span", {.class("fw-bold").text("Note : ")}),
                            html!("span", {.style("white-space","pre-wrap").text(&os.pe.clone().unwrap_or_default())}),
                        ])
                    }),
                ])
            }))
        })
    }

    // trauma - only doctor can save
    pub fn render_trauma(page: Rc<Self>, app: Rc<App>) -> Dom {
        let doctor_select_option = app.app_asset.lock_ref().as_ref().map(|opt| opt.doctor_select_option.clone()).unwrap_or_default();

        html!("div", {
            .class(class::CARD)
            .style("break-inside","avoid")
            .child(html!("div", {
                .class("card-body")
                .children([
                    html!("div", {
                        .class(class::ROW)
                        .child(html!("div", {
                            .class("col-md-12")
                            .children([
                                html!("lable", {
                                    .class("fw-bold")
                                    .child(html!("i", {.class(class::FA_CLIPBOARD)}))
                                    .text(" Trauma")
                                }),
                                html!("button", {
                                    .attr("type", "button")
                                    .class(class::BTN_R_BLUE)
                                    .child(html!("i", {.class(class::FA_EYE)}))
                                    .event(clone!(page => move |_:events::Click| {
                                        page.trauma_visible.set(!page.trauma_visible.get());
                                    }))
                                }),
                            ])
                        }))
                    }),
                    html!("div", {
                        // .attr("id", "opd_er_dr_pe")
                        .children([
                            html!("div", {
                                // .attr("id", "trauma_div")
                                .visible_signal(page.trauma_visible.signal())
                                .children([
                                    html!("div", {
                                        .class(class::ROW)
                                        .child(html!("div", {
                                            .class("col-md-12")
                                            .child(html!("lable", {
                                                .text("A: Airway & restriction c-spine")
                                            }))
                                        }))
                                    }),
                                    html!("div", {
                                        .class(class::ROW)
                                        .children([
                                            html!("div", {
                                                .class(class::COL_MD2_OFSM1_Y1)
                                                .child(html!("div", {
                                                    .class("form-check")
                                                    .children([
                                                        html!("input" => HtmlInputElement, {
                                                            .class("form-check-input")
                                                            .attr("type", "radio")
                                                            .attr("id", "arc_patent")
                                                            .attr("value", "1")
                                                            .apply(mixins::other_true_signal_disable(not(page.is_doctor())))
                                                            .apply(mixins::radio_match(page.arc.clone(), page.trauma_changed.clone(), "1"))
                                                        }),
                                                        doms::label_check_for("arc_patent","Patent"),
                                                    ])
                                                }))
                                            }),
                                            html!("div", {
                                                .class(class::COLA_MY1)
                                                .child(html!("div", {
                                                    .class("form-check")
                                                    .children([
                                                        html!("input" => HtmlInputElement, {
                                                            .class("form-check-input")
                                                            .attr("type", "radio")
                                                            .attr("id", "arc_csp")
                                                            .attr("value", "2")
                                                            .apply(mixins::other_true_signal_disable(not(page.is_doctor())))
                                                            .apply(mixins::radio_match(page.arc.clone(), page.trauma_changed.clone(), "2"))
                                                        }),
                                                        doms::label_check_for("arc_csp","C-spine protection"),
                                                    ])
                                                }))
                                            }),
                                        ])
                                    }),
                                    html!("div", {
                                        .class(class::ROW)
                                        .children([
                                            html!("div", {
                                                .class(class::COL_MD4_OFSM1_Y1)
                                                .child(html!("div", {
                                                    .class("form-check")
                                                    .children([
                                                        html!("input" => HtmlInputElement, {
                                                            .class("form-check-input")
                                                            .attr("type", "radio")
                                                            .attr("id", "arc_npc")
                                                            .attr("value", "3")
                                                            .apply(mixins::other_true_signal_disable(not(page.is_doctor())))
                                                            .apply(mixins::radio_match(page.arc.clone(), page.trauma_changed.clone(), "3"))
                                                        }),
                                                        doms::label_check_for("arc_npc","Non-patent : Clinical"),
                                                    ])
                                                }))
                                            }),
                                            html!("div", {
                                                .class("col-md-7")
                                                .child(html!("input" => HtmlInputElement, {
                                                    .attr("type", "text")
                                                    .class("form-control")
                                                    .apply(mixins::other_true_signal_disable(map_ref!{
                                                        let is_doctor = page.is_doctor(),
                                                        let arc = page.arc.signal_cloned() =>
                                                        !is_doctor || arc.as_str() != "3"
                                                    }))
                                                    .apply(mixins::string_value(page.arc_npc_text.clone(), page.trauma_changed.clone()))
                                                    .future(page.arc.signal_cloned().for_each(clone!(page => move |arc| {
                                                        if arc.as_str() != "3" {
                                                            page.arc_npc_text.set_neq(String::new());
                                                        }
                                                        async {}
                                                    })))
                                                }))
                                            }),
                                        ])
                                    }),
                                    html!("div", {
                                        .class(class::ROW)
                                        .child(html!("div", {
                                            .class("col-md-12")
                                            .child(html!("lable", {
                                                .text("B: Breathing")
                                            }))
                                        }))
                                    }),
                                    html!("div", {
                                        .class(class::ROW)
                                        .children([
                                            html!("div", {
                                                .class(class::COL_MD2_OFSM1)
                                                .child(html!("lable", {
                                                    .text("Chest wall")
                                                }))
                                            }),
                                            html!("div", {
                                                .class("col-md-9")
                                                .child(html!("input" => HtmlInputElement, {
                                                    .attr("type", "text")
                                                    .class("form-control")
                                                    .apply(mixins::other_true_signal_disable(not(page.is_doctor())))
                                                    .apply(mixins::string_value(page.breathing_chest_wall.clone(), page.trauma_changed.clone()))
                                                }))
                                            }),
                                        ])
                                    }),
                                    html!("div", {
                                        .class(class::ROW)
                                        .children([
                                            html!("div", {
                                                .class(class::COL_MD2_OFSM1)
                                                .child(html!("lable", {
                                                    .text("Lung")
                                                }))
                                            }),
                                            html!("div", {
                                                .class("col-md-9")
                                                .child(html!("input" => HtmlInputElement, {
                                                    .attr("type", "text")
                                                    .class("form-control")
                                                    .apply(mixins::other_true_signal_disable(not(page.is_doctor())))
                                                    .apply(mixins::string_value(page.breathing_lung.clone(), page.trauma_changed.clone()))
                                                }))
                                            }),
                                        ])
                                    }),
                                    html!("div", {
                                        .class(class::ROW)
                                        .child(html!("div", {
                                            .class("col-md-4")
                                            .child(html!("lable", {
                                                .text("C: Circulation")
                                            }))
                                        }))
                                    }),
                                    html!("div", {
                                        .class(class::ROW)
                                        .children([
                                            html!("div", {
                                                .class(class::COL_MD2_OFSM1_Y1)
                                                .child(html!("div", {
                                                    .class("form-check")
                                                    .children([
                                                        html!("input" => HtmlInputElement, {
                                                            .class("form-check-input")
                                                            .attr("type", "radio")
                                                            .attr("id", "circulation_stable")
                                                            .attr("value", "1")
                                                            .apply(mixins::other_true_signal_disable(not(page.is_doctor())))
                                                            .apply(mixins::radio_match(page.circulation_shock.clone(), page.trauma_changed.clone(), "1"))
                                                        }),
                                                        doms::label_check_for("circulation_stable","Stable"),
                                                    ])
                                                }))
                                            }),
                                            html!("div", {
                                                .class(class::COL_MD2_Y1)
                                                .child(html!("div", {
                                                    .class("form-check")
                                                    .children([
                                                        html!("input" => HtmlInputElement, {
                                                            .class("form-check-input")
                                                            .attr("type", "radio")
                                                            .attr("id", "circulation_shock")
                                                            .attr("value", "2")
                                                            .apply(mixins::other_true_signal_disable(not(page.is_doctor())))
                                                            .apply(mixins::radio_match(page.circulation_shock.clone(), page.trauma_changed.clone(), "2"))
                                                        }),
                                                        doms::label_check_for("circulation_shock","Shock"),
                                                    ])
                                                }))
                                            }),
                                            html!("div", {
                                                .class("col-md-7")
                                                .child(html!("input" => HtmlInputElement, {
                                                    .attr("type", "text")
                                                    .class("form-control")
                                                    .apply(mixins::other_true_signal_disable(map_ref!{
                                                        let is_doctor = page.is_doctor(),
                                                        let circulation_shock = page.circulation_shock.signal_cloned() =>
                                                        !is_doctor || circulation_shock.as_str() != "2"
                                                    }))
                                                    .apply(mixins::string_value(page.circulation_shock_text.clone(), page.trauma_changed.clone()))
                                                    .future(page.circulation_shock.signal_cloned().for_each(clone!(page => move |circulation_shock| {
                                                        if circulation_shock.as_str() != "2" {
                                                            page.circulation_shock_text.set_neq(String::new());
                                                        }
                                                        async {}
                                                    })))
                                                }))
                                            }),
                                        ])
                                    }),
                                    html!("div", {
                                        .class(class::ROW)
                                        .children([
                                            html!("div", {
                                                .class(class::COL_MD2_OFSM1_Y1)
                                                .child(html!("div", {
                                                    .class("form-check")
                                                    .children([
                                                        html!("input" => HtmlInputElement, {
                                                            .class("form-check-input")
                                                            .attr("type", "checkbox")
                                                            .attr("id", "circulation_other")
                                                            .apply(mixins::other_true_signal_disable(not(page.is_doctor())))
                                                            .apply(mixins::checkbox_toggle(page.circulation_other.clone(), page.trauma_changed.clone(), "Y", ""))
                                                        }),
                                                        doms::label_check_for("circulation_other","อื่นๆ"),
                                                    ])
                                                }))
                                            }),
                                            html!("div", {
                                                .class("col-md-9")
                                                .child(html!("input" => HtmlInputElement, {
                                                    .attr("type", "text")
                                                    .class("form-control")
                                                    .apply(mixins::other_true_signal_disable(map_ref!{
                                                        let is_doctor = page.is_doctor(),
                                                        let circulation_other = page.circulation_other.signal_cloned() =>
                                                        !is_doctor || circulation_other.as_str() != "Y"
                                                    }))
                                                    .apply(mixins::string_value(page.circulation_other_text.clone(), page.trauma_changed.clone()))
                                                    .future(page.circulation_other.signal_cloned().for_each(clone!(page => move |circulation_other| {
                                                        if circulation_other.as_str() != "Y" {
                                                            page.circulation_other_text.set_neq(String::new());
                                                        }
                                                        async {}
                                                    })))
                                                }))
                                            }),
                                        ])
                                    }),
                                    html!("div", {
                                        .class(class::ROW)
                                        .children([
                                            html!("div", {
                                                .class(class::COL_MD2_OFSM1_Y1)
                                                .child(html!("lable", {
                                                    .text("eFAST Time")
                                                }))
                                            }),
                                            html!("div", {
                                                .class("col-sm-6")
                                                .child(html!("div", {
                                                    .class(class::INPUT_GROUP)
                                                    .children([
                                                        doms::date_picker(
                                                            page.circulation_efast_date.clone(),
                                                            page.trauma_changed.clone(), not(page.is_doctor()), None,
                                                            |d| d.class(class::FLEX_GROW1).style("min-width","120px"),
                                                            |d| d.class(class::COL_SM8_RE0),
                                                            |d| d.class(class::COL_SM8_RE0),
                                                            |s| s, always(None),
                                                        ),
                                                        doms::time_picker(
                                                            page.circulation_efast_time.clone(),
                                                            page.trauma_changed.clone(), not(page.is_doctor()), None,
                                                            |d| d.class(class::FLEX_GROW1).style("min-width","95px"),
                                                            |d| d.class(class::COL_SM4_RE0),
                                                            |d| d.class(class::COL_SM4_RE0),
                                                            |s| s, always(None),
                                                        ),
                                                        html!("button" => HtmlButtonElement, {
                                                            .class(class::BTN_GRAY)
                                                            .attr("type", "button")
                                                            .attr("title", "ใช้วันที่-เวลา ปัจจุบัน")
                                                            .child(html!("i", {.class(class::FA_CLOCK)}))
                                                            .apply(mixins::other_true_signal_disable(not(page.is_doctor())))
                                                            .event(clone!(page => move |_:events::Click| {
                                                                let now = js_now();
                                                                page.circulation_efast_date.set(now.date().to_string());
                                                                page.circulation_efast_time.set(now.time().js_string());
                                                                page.trauma_changed.set_neq(true);
                                                            }))
                                                        }),
                                                    ])
                                                }))
                                            }),
                                        ])
                                    }),
                                    html!("div", {
                                        .class(class::ROW)
                                        .children([
                                            html!("div", {
                                                .class(class::COL_MD2_OFSM1_Y1)
                                                .child(html!("lable", {
                                                    .text("eFAST Doctor")
                                                }))
                                            }),
                                            html!("div", {
                                                .class("col-md-9")
                                                .child(html!("div", {
                                                    .class("mb-3")
                                                    .child(html!("select" => HtmlSelectElement, {
                                                        .class("form-select")
                                                        .child(html!("option", {.attr("value", "").text("เลือก")}))
                                                        .children(doctor_select_option.iter().map(|option| {
                                                            doms::select_option(option, "")
                                                        }))
                                                        .apply(mixins::other_true_signal_disable(not(page.is_doctor())))
                                                        .apply(mixins::string_value_select(page.circulation_doctor.clone(), page.trauma_changed.clone()))
                                                    }))
                                                }))
                                            }),
                                        ])
                                    }),
                                    html!("div", {
                                        .class(class::ROW)
                                        .children([
                                            html!("div", {
                                                .class(class::COL_MD2_OFSM1_Y1)
                                                .child(html!("div", {
                                                    .class("form-check")
                                                    .children([
                                                        html!("input" => HtmlInputElement, {
                                                            .class("form-check-input")
                                                            .attr("type", "radio")
                                                            .attr("id", "circulation_negative")
                                                            .attr("value", "N")
                                                            .apply(mixins::other_true_signal_disable(not(page.is_doctor())))
                                                            .apply(mixins::radio_match(page.circulation.clone(), page.trauma_changed.clone(), "N"))
                                                        }),
                                                        doms::label_check_for("circulation_negative","Negative"),
                                                    ])
                                                }))
                                            }),
                                            html!("div", {
                                                .class(class::COL_MD2_Y1)
                                                .child(html!("div", {
                                                    .class("form-check")
                                                    .children([
                                                        html!("input" => HtmlInputElement, {
                                                            .class("form-check-input")
                                                            .attr("type", "radio")
                                                            .attr("id", "circulation_positive")
                                                            .attr("value", "P")
                                                            .apply(mixins::other_true_signal_disable(not(page.is_doctor())))
                                                            .apply(mixins::radio_match(page.circulation.clone(), page.trauma_changed.clone(), "P"))
                                                            // .attr("onchange", "onchangeDisabledInput_circulation('P')")
                                                        }),
                                                        doms::label_check_for("circulation_positive","Positive at"),
                                                    ])
                                                }))
                                            }),
                                            html!("div", {
                                                .class("col-md-7")
                                                .child(html!("input" => HtmlInputElement, {
                                                    .attr("type", "text")
                                                    .class("form-control")
                                                    .apply(mixins::other_true_signal_disable(map_ref!{
                                                        let is_doctor = page.is_doctor(),
                                                        let circulation = page.circulation.signal_cloned() =>
                                                        !is_doctor || circulation.as_str() != "P"
                                                    }))
                                                    .apply(mixins::string_value(page.circulation_positive_text.clone(), page.trauma_changed.clone()))
                                                    .future(page.circulation.signal_cloned().for_each(clone!(page => move |circulation| {
                                                        if circulation.as_str() != "P" {
                                                            page.circulation_positive_text.set_neq(String::new());
                                                        }
                                                        async {}
                                                    })))
                                                }))
                                            }),
                                        ])
                                    }),
                                    html!("div", {
                                        .class(class::ROW)
                                        .child(html!("div", {
                                            .class("col-md-3")
                                            .child(html!("lable", {
                                                .text("D: Disability")
                                            }))
                                        }))
                                    }),
                                    html!("div", {
                                        .class(class::ROW)
                                        .children([
                                            html!("div", {
                                                .class(class::COL_MD2_OFSM1_Y1)
                                                .child(html!("lable", {
                                                    .text("E")
                                                }))
                                            }),
                                            html!("div", {
                                                .class("col-md-2")
                                                .child(html!("input" => HtmlInputElement, {
                                                    .attr("type", "text")
                                                    .class("form-control")
                                                    .apply(mixins::other_true_signal_disable(not(page.is_doctor())))
                                                    .apply(mixins::string_value(page.disability_e.clone(), page.trauma_changed.clone()))
                                                }))
                                            }),
                                            html!("div", {
                                                .class(class::COL_MD1_Y1)
                                                .child(html!("lable", {
                                                    .text("V")
                                                }))
                                            }),
                                            html!("div", {
                                                .class("col-md-2")
                                                .child(html!("input" => HtmlInputElement, {
                                                    .attr("type", "text")
                                                    .class("form-control")
                                                    .apply(mixins::other_true_signal_disable(not(page.is_doctor())))
                                                    .apply(mixins::string_value(page.disability_v.clone(), page.trauma_changed.clone()))
                                                }))
                                            }),
                                            html!("div", {
                                                .class(class::COL_MD1_Y1)
                                                .child(html!("lable", {
                                                    .text("M")
                                                }))
                                            }),
                                            html!("div", {
                                                .class("col-md-2")
                                                .child(html!("input" => HtmlInputElement, {
                                                    .attr("type", "text")
                                                    .class("form-control")
                                                    .apply(mixins::other_true_signal_disable(not(page.is_doctor())))
                                                    .apply(mixins::string_value(page.disability_m.clone(), page.trauma_changed.clone()))
                                                }))
                                            }),
                                        ])
                                    }),
                                    html!("div", {
                                        .class(class::ROW)
                                        .children([
                                            html!("div", {
                                                .class(class::COL_MD2_OFSM1_Y1)
                                                .child(html!("lable", {
                                                    .text("Pupil Rt")
                                                }))
                                            }),
                                            html!("div", {
                                                .class("col-md-2")
                                                .child(html!("input" => HtmlInputElement, {
                                                    .attr("type", "text")
                                                    .class("form-control")
                                                    .apply(mixins::other_true_signal_disable(not(page.is_doctor())))
                                                    .apply(mixins::string_value(page.disability_pupil_rt.clone(), page.trauma_changed.clone()))
                                                }))
                                            }),
                                            html!("div", {
                                                .class(class::COL_MD2_Y1)
                                                .child(html!("lable", {
                                                    .text("mm")
                                                }))
                                            }),
                                        ])
                                    }),
                                    html!("div", {
                                        .class(class::ROW)
                                        .children([
                                            html!("div", {
                                                .class(class::COL_MD2_OFSM1_Y1)
                                                .child(html!("lable", {
                                                    .text("Pupil Lt")
                                                }))
                                            }),
                                            html!("div", {
                                                .class("col-md-2")
                                                .child(html!("input" => HtmlInputElement, {
                                                    .attr("type", "text")
                                                    .class("form-control")
                                                    .apply(mixins::other_true_signal_disable(not(page.is_doctor())))
                                                    .apply(mixins::string_value(page.disability_pupil_lt.clone(), page.trauma_changed.clone()))
                                                }))
                                            }),
                                            html!("div", {
                                                .class(class::COLA_MY1)
                                                .child(html!("lable", {
                                                    .text("mm")
                                                }))
                                            }),
                                        ])
                                    }),
                                    html!("div", {
                                        .class(class::ROW)
                                        .children([
                                            html!("div", {
                                                .class(class::COL_MD2_OFSM1_Y1)
                                                .child(html!("lable", {
                                                    .text("อื่นๆ")
                                                }))
                                            }),
                                            html!("div", {
                                                .class("col-md-9")
                                                .child(html!("input" => HtmlInputElement, {
                                                    .attr("type", "text")
                                                    .class("form-control")
                                                    .apply(mixins::other_true_signal_disable(not(page.is_doctor())))
                                                    .apply(mixins::string_value(page.disability_other.clone(), page.trauma_changed.clone()))
                                                }))
                                            }),
                                        ])
                                    }),
                                    html!("div", {
                                        .class(class::ROW)
                                        .children([
                                            html!("div", {
                                                .class("col-md-3")
                                                .child(html!("lable", {
                                                    .text("E: Exposure")
                                                }))
                                            }),
                                            html!("div", {
                                                .class("col-md-9")
                                                .child(html!("input" => HtmlInputElement, {
                                                    .attr("type", "text")
                                                    .class("form-control")
                                                    .apply(mixins::other_true_signal_disable(not(page.is_doctor())))
                                                    .apply(mixins::string_value(page.exposure.clone(), page.trauma_changed.clone()))
                                                }))
                                            }),
                                        ])
                                    }),
                                ])
                                .child_signal(page.trauma_doctor_name.signal_cloned().map(|name| {
                                    (!name.is_empty()).then(|| {
                                        html!("div", {
                                            .class(class::ROW)
                                            .child(html!("label", {
                                                .class(class::COL_MD12_R)
                                                // .attr("id", "opd_er_pe_doctor_name")
                                                .text("บันทึกโดย : ")
                                                .text(&name)
                                            }))
                                        })
                                    })
                                }))
                                .child_signal(page.trauma_save_result.signal_cloned().map(|result| {
                                    result.map(|success| {
                                        let class = if success {"text-success"} else {"text-danger"};
                                        let text = if success {"บันทึกข้อมูลสำเร็จ"} else {"บันทึกข้อมูลไม่สำเร็จ"};
                                        html!("div", {
                                            .class(class::ROW)
                                            .child(html!("div", {
                                                .class("col-md-12")
                                                .children([
                                                    html!("small", {
                                                        .class("fw-bold")
                                                        .class(class)
                                                        // .attr("id", "lable_show_save_er_DR_PE")
                                                        .child(html!("i", {.class(class::FA_CHECK_CIRCLE)}))
                                                        .text(text)
                                                    }),
                                                ])
                                            }))
                                        })
                                    })
                                }))
                                .apply_if(app.endpoint_is_allow(&Method::POST, &EndPoint::OpdErMedicalHistoryTrauma, false), |dom| dom
                                    .child(html!("div", {
                                        .class("row")
                                        .visible_signal(page.is_doctor())
                                        .child(html!("div", {
                                            .class(class::COL_MD12_R)
                                            .children([
                                                html!("button" => HtmlButtonElement, {
                                                    .attr("type", "button")
                                                    // .attr("id", "SaveDR_PE")
                                                    .class("btn")
                                                    .class_signal("btn-primary", page.trauma_changed.signal())
                                                    .class_signal("btn-secondary", not(page.trauma_changed.signal()))
                                                    .child(html!("i", {.class(class::FA_SAVE)}))
                                                    .text(" Save ")
                                                    .child(html!("span", {
                                                        .class(class::SPIN_SM_BLUE)
                                                        .attr("role", "status")
                                                        // .attr("id", "spinnerER_SaveDR_PE")
                                                        .visible_signal(map_ref!{
                                                            let loading = app.loader_is_loading(),
                                                            let saving = page.trauma_saving.signal() =>
                                                            *loading && *saving
                                                        })
                                                    }))
                                                    .apply(mixins::click_with_loader_checked_or_true_disable_signal(clone!(app, page => move || {
                                                        Self::save_trauma(page.clone(), app.clone());
                                                    }), not(page.trauma_changed.signal()), app.state()))
                                                }),
                                                html!("button", {
                                                    .attr("type", "button")
                                                    // .attr("id", "CancalDR_PE")
                                                    .class(class::BTN_R_GRAY)
                                                    .child(html!("i", {.class(class::FA_X)}))
                                                    .text(" Cancel")
                                                    .event(clone!(page => move |_:events::Click| {
                                                        if page.trauma_changed.get() {
                                                            page.trauma_loaded.set(false);
                                                        }
                                                    }))
                                                }),
                                            ])
                                        }))
                                    }))
                                )
                            }),
                        ])
                    }),
                ])
            }))
        })
    }

    // triage_hosxp - read
    pub fn render_triage(os: &OpdScreenHistory) -> Dom {
        html!("div", {
            .class(class::CARD)
            .style("break-inside","avoid")
            .child(html!("div", {
                .class("card-body")
                .children([
                    html!("div", {
                        .class(class::BOLD_T3L)
                        .child(html!("i", {.class(class::FA_CLIPBOARD)}))
                        .text(" Triage (HOSxP)")
                    }),
                    html!("div", {
                        .class("ms-3")
                        .children([
                            html!("span", {.class("fw-bold").text("ประเภทผู้ป่วย : ")}),
                            html!("span", {.text(&os.er_pt_type_name.clone().unwrap_or(String::from("ไม่ระบุ")))}),
                            html!("span", {.class("fw-bold").text(" ความเร่งด่วน : ")}),
                            html!("span", {.text(&os.er_emergency_type_name.clone().unwrap_or(String::from("ไม่ระบุ")))}),
                        ])
                    }),
                    html!("div", {
                        .class("ms-3")
                        .children([
                            html!("span", {.class("fw-bold").text("ระดับความฉุกเฉิน : ")}),
                            html!("span", {.text(&os.er_emergency_level_name.clone().unwrap_or(String::from("ไม่ระบุ")))}),
                            html!("span", {.class("fw-bold").text(" ประเภททางคลินิก : ")}),
                            html!("span", {.text(&os.er_spclty_name.clone().unwrap_or(String::from("ไม่ระบุ")))}),
                        ])
                    }),
                ])
            }))
        })
    }
    // drug_allergy - save
    pub fn render_allergy(page: Rc<Self>, app: Rc<App>) -> Dom {
        html!("div", {
            .class(class::CARD)
            .style("break-inside","avoid")
            .child(html!("div", {
                .class("card-body")
                .child(html!("div", {
                    // .attr("id", "opd_er_allergy_history")
                    .children([
                        html!("div", {
                            .class(class::ROW)
                            .child(html!("div", {
                                .class("col-md-12")
                                .children([
                                    html!("lable", {
                                        .class("fw-bold")
                                        .child(html!("i", {.class(class::FA_CAPSULE)}))
                                        .text(" ประวัติการแพ้ยา")
                                    }),
                                    html!("button", {
                                        .attr("type", "button")
                                        .class(class::BTN_R_BLUE)
                                        .child(html!("i", {.class(class::FA_EYE)}))
                                        .event(clone!(page => move |_:events::Click| {
                                            page.allergy_visible.set(!page.allergy_visible.get());
                                        }))
                                    }),
                                ])
                            }))
                        }),
                        html!("div", {
                            // .attr("id", "er_allergy_history_div")
                            .visible_signal(page.allergy_visible.signal())
                            .apply_if(app.endpoint_is_allow(&Method::POST, &EndPoint::OpdErMedicalHistoryAllergy, false), |dom| dom
                                .child(html!("div", {
                                    .class(class::ROW)
                                    .child(html!("div", {
                                        .class("col-md-12")
                                        .children([
                                            html!("lable", {.text("เพิ่มประวัติการแพ้ยา")}),
                                            html!("button", {
                                                .attr("type", "button")
                                                .class(class::BTN_R_BLUE)
                                                .child(html!("i", {.class(class::FA_PLUS)}))
                                                .event(clone!(page => move |_: events::Click| {
                                                    page.er_allergy_history_items.lock_mut().push_cloned(DrugAllergy::new());
                                                    page.allergy_changed.set_neq(true);
                                                }))
                                                // .attr("onclick", "addER_Allergy_History()")
                                            }),
                                        ])
                                    }))
                                }))
                            )
                            .children([
                                html!("div", {
                                    .class(class::ROW)
                                    .children([
                                        html!("div", {
                                            .class("col-md-5")
                                            .child(html!("label", {.text("ชื่อยา")}))
                                        }),
                                        html!("div", {
                                            .class("col-md-5")
                                            .child(html!("label", {.text("อาการที่แพ้")}))
                                        }),
                                    ])
                                }),
                                html!("div", {
                                    .class(class::ROW)
                                    .child(html!("div", {
                                        .class("col-md-12")
                                        .child(html!("div", {
                                            // .attr("id", "er-allergy-history-input-div")
                                            .children_signal_vec(page.er_allergy_history_items.signal_vec_cloned().map(clone!(page => move |ad| {
                                                DrugAllergy::render(ad, page.clone())
                                            })))
                                        }))
                                    }))
                                }),
                            ])
                            .child_signal(page.allergy_doctor_name.signal_cloned().map(|name| {
                                (!name.is_empty()).then(|| {
                                    html!("div", {
                                        .class(class::ROW)
                                        .child(html!("div", {
                                            .class(class::COL_MD12_R)
                                            // .attr("id", "er_allergy_history_doctorcode_text")
                                            .text("บันทึกโดย : ")
                                            .text(&name)
                                        }))
                                    })
                                })
                            }))
                            .child_signal(page.allergy_save_result.signal_cloned().map(|result| {
                                result.map(|success| {
                                    let class = if success {"text-success"} else {"text-danger"};
                                    let text = if success {"บันทึกข้อมูลสำเร็จ"} else {"บันทึกข้อมูลไม่สำเร็จ"};
                                    html!("div", {
                                        .class(class::ROW)
                                        .child(html!("div", {
                                            .class("col-md-12")
                                            .children([
                                                html!("small", {
                                                    .class("fw-bold")
                                                    .class(class)
                                                    // .attr("id", "lable_show_save_er_allergy_history")
                                                    .child(html!("i", {.class(class::FA_CHECK_CIRCLE)}))
                                                    .text(text)
                                                }),
                                            ])
                                        }))
                                    })
                                })
                            }))
                            .apply_if(app.endpoint_is_allow(&Method::POST, &EndPoint::OpdErMedicalHistoryAllergy, false), |dom| dom
                                .child(html!("div", {
                                    .class("row")
                                    .child(html!("div", {
                                        .class(class::COL_MD12_R)
                                        .children([
                                            html!("button" => HtmlButtonElement, {
                                                .attr("type", "button")
                                                // .attr("id", "SaveER_Allergy_History")
                                                .class("btn")
                                                .class_signal("btn-primary", page.allergy_changed.signal())
                                                .class_signal("btn-secondary", not(page.allergy_changed.signal()))
                                                .child(html!("i", {.class(class::FA_SAVE)}))
                                                .text(" Save ")
                                                .child(html!("span", {
                                                    .class(class::SPIN_SM_BLUE)
                                                    .attr("role", "status")
                                                    // .attr("id", "spinnerER_Allergy_History")
                                                    .visible_signal(map_ref!{
                                                        let loading = app.loader_is_loading(),
                                                        let saving = page.allergy_saving.signal() =>
                                                        *loading && *saving
                                                    })
                                                }))
                                                .apply(mixins::click_with_loader_checked_or_true_disable_signal(clone!(app, page => move || {
                                                    Self::save_allergy(page.clone(), app.clone());
                                                }), not(page.allergy_changed.signal()), app.state()))
                                            }),
                                            html!("button", {
                                                .attr("type", "button")
                                                // .attr("id", "CancalER_Allergy_History")
                                                .class(class::BTN_R_GRAY)
                                                .child(html!("i", {.class(class::FA_X)}))
                                                .text(" Cancel")
                                                .event(clone!(page => move |_:events::Click| {
                                                    if page.allergy_changed.get() {
                                                        page.allergy_loaded.set(false);
                                                    }
                                                }))
                                            }),
                                        ])
                                    }))
                                }))
                            )
                        }),
                    ])
                }))
            }))
        })
    }
    // record_datetime - only nurse can save
    pub fn render_nurse_screening(page: Rc<Self>, app: Rc<App>) -> Dom {
        let (emergency_level_select_option, spclty_select_option) = app
            .app_asset
            .lock_ref()
            .as_ref()
            .map(|opt| (opt.emergency_level_select_option.clone(), opt.spclty_select_option.clone()))
            .unwrap_or((Vec::new(), Vec::new()));

        html!("div", {
            .class(class::CARD)
            .style("break-inside","avoid")
            .child(html!("div", {
                .class("card-body")
                .child(html!("div", {
                    // .attr("id", "opd_er_nurse_screening")
                    .children([
                        html!("div", {
                            .class("row")
                            .children([
                                html!("div", {
                                    .class("col-sm-4")
                                    .child(html!("div", {
                                        .class("mb-3")
                                        .children([
                                            html!("label", {
                                                .attr("for", "screening_emergency_level")
                                                .text("ระดับความฉุกเฉิน")
                                            }),
                                            html!("select" => HtmlSelectElement, {
                                                .class("form-select")
                                                .attr("id", "screening_emergency_level")
                                                .child(html!("option", {.attr("value", "").text("เลือก")}))
                                                .children(emergency_level_select_option.iter().map(|option| {
                                                    doms::select_option(option, "")
                                                }))
                                                .apply(mixins::other_true_signal_disable(page.is_nurse_screen_after_doctor()))
                                                .apply(mixins::string_value_select(page.screening_emergency_level.clone(), page.screen_changed.clone()))
                                                 // kphis.opd_er_emergency_level contains(R, E, U, Semi-U, Non-U)
                                                 // hos.er_emergency_level contains(1: Resuscitate, 2:Emergency, 3:Urgency, 4:Semi Urgency, 5: Non Urgency)
                                            }),
                                        ])
                                    }))
                                }),
                                html!("div", {
                                    .class("col-sm-4")
                                    .child(html!("div", {
                                        .class("mb-3")
                                        .children([
                                            html!("label", {
                                                .attr("for", "screening_spclty")
                                                .text("ประเภททางคลินิก")
                                            }),
                                            html!("select" => HtmlSelectElement, {
                                                .class("form-select")
                                                .attr("style", "width: 100%")
                                                .attr("id", "screening_spclty")
                                                .child(html!("option", {.attr("value", "").text("เลือก")}))
                                                .children(spclty_select_option.iter().map(|option| {
                                                    doms::select_option(option, "")
                                                }))
                                                .apply(mixins::other_true_signal_disable(page.is_nurse_screen_after_doctor()))
                                                .apply(mixins::string_value_select(page.screening_spclty.clone(), page.screen_changed.clone()))
                                            }),
                                        ])
                                    }))
                                }),
                            ])
                        }),
                        html!("div", {
                            .class("row")
                            .children([
                                html!("div", {
                                    .class("col-sm-6")
                                    .child(html!("label", {
                                        .text("เวลาที่มาถึง ER")
                                    }))
                                }),
                                html!("div", {
                                    .class("col-sm-6")
                                    .child(html!("label", {
                                        .text("เวลาคัดกรอง")
                                    }))
                                }),
                            ])
                        }),
                        html!("div", {
                            .class(class::ROW)
                            .children([
                                html!("div", {
                                    .class("col-sm-6")
                                    .child(html!("div", {
                                        .class(class::INPUT_GROUP)
                                        .children([
                                            doms::date_picker(
                                                page.screening_arrive_date.clone(),
                                                page.screen_changed.clone(), page.is_nurse_screen_after_doctor(), None,
                                                |d| d.class(class::FLEX_GROW1).style("min-width","120px"),
                                                |d| d.class(class::COL_SM8_RE0),
                                                |d| d.class(class::COL_SM8_RE0),
                                                |s| s, always(None),
                                            ),
                                            doms::time_picker(
                                                page.screening_arrive_time.clone(),
                                                page.screen_changed.clone(), page.is_nurse_screen_after_doctor(), None,
                                                |d| d.class(class::FLEX_GROW1).style("min-width","95px"),
                                                |d| d.class(class::COL_SM4_RE0),
                                                |d| d.class(class::COL_SM4_RE0),
                                                |s| s, always(None),
                                            ),
                                            html!("button" => HtmlButtonElement, {
                                                .class(class::BTN_GRAY)
                                                .attr("type", "button")
                                                .attr("title", "ใช้วันที่-เวลา ปัจจุบัน")
                                                .child(html!("i", {.class(class::FA_CLOCK)}))
                                                .apply(mixins::other_true_signal_disable(page.is_nurse_screen_after_doctor()))
                                                .event(clone!(page => move |_:events::Click| {
                                                    let now = js_now();
                                                    page.screening_arrive_date.set_neq(now.date().to_string());
                                                    page.screening_arrive_time.set_neq(now.time().js_string());
                                                    page.screen_changed.set_neq(true);
                                                }))
                                            }),
                                        ])
                                    }))
                                }),
                                html!("div", {
                                    .class("col-sm-6")
                                    .child(html!("div", {
                                        .class(class::INPUT_GROUP)
                                        .children([
                                            doms::date_picker(
                                                page.screening_date.clone(),
                                                page.screen_changed.clone(), page.is_nurse_screen_after_doctor(), None,
                                                |d| d.class(class::FLEX_GROW1).style("min-width","120px"),
                                                |d| d.class(class::COL_SM8_RE0),
                                                |d| d.class(class::COL_SM8_RE0),
                                                |s| s, always(None),
                                            ),
                                            doms::time_picker(
                                                page.screening_time.clone(),
                                                page.screen_changed.clone(), page.is_nurse_screen_after_doctor(), None,
                                                |d| d.class(class::FLEX_GROW1).style("min-width","95px"),
                                                |d| d.class(class::COL_SM4_RE0),
                                                |d| d.class(class::COL_SM4_RE0),
                                                |s| s, always(None),
                                            ),
                                            html!("button" => HtmlButtonElement, {
                                                .class(class::BTN_GRAY)
                                                .attr("type", "button")
                                                .attr("title", "ใช้วันที่-เวลา ปัจจุบัน")
                                                .child(html!("i", {.class(class::FA_CLOCK)}))
                                                .apply(mixins::other_true_signal_disable(page.is_nurse_screen_after_doctor()))
                                                .event(clone!(page => move |_:events::Click| {
                                                    let now = js_now();
                                                    page.screening_date.set_neq(now.date().to_string());
                                                    page.screening_time.set_neq(now.time().js_string());
                                                    page.screen_changed.set_neq(true);
                                                }))
                                            }),
                                        ])
                                    }))
                                }),
                            ])
                        }),
                        html!("div", {
                            .class("row")
                            .children([
                                html!("div", {
                                    .class("col-sm-6")
                                    .child(html!("label", {
                                        .text("เวลาที่รายงาน")
                                    }))
                                }),
                                html!("div", {
                                    .class("col-sm-6")
                                    .child(html!("label", {
                                        .text("เวลาที่พบแพทย์")
                                    }))
                                }),
                            ])
                        }),
                        html!("div", {
                            .class(class::ROW)
                            .children([
                                html!("div", {
                                    .class("col-sm-6")
                                    .child(html!("div", {
                                        .class(class::INPUT_GROUP)
                                        .children([
                                            doms::date_picker(
                                                page.screening_report_date.clone(),
                                                page.screen_changed.clone(), page.is_nurse_screen_after_doctor(), None,
                                                |d| d.class(class::FLEX_GROW1).style("min-width","120px"),
                                                |d| d.class(class::COL_SM8_RE0),
                                                |d| d.class(class::COL_SM8_RE0),
                                                |s| s, always(None),
                                            ),
                                            doms::time_picker(
                                                page.screening_report_time.clone(),
                                                page.screen_changed.clone(), page.is_nurse_screen_after_doctor(), None,
                                                |d| d.class(class::FLEX_GROW1).style("min-width","95px"),
                                                |d| d.class(class::COL_SM4_RE0),
                                                |d| d.class(class::COL_SM4_RE0),
                                                |s| s, always(None),
                                            ),
                                            html!("button" => HtmlButtonElement, {
                                                .class(class::BTN_GRAY)
                                                .attr("type", "button")
                                                .attr("title", "ใช้วันที่-เวลา ปัจจุบัน")
                                                .child(html!("i", {.class(class::FA_CLOCK)}))
                                                .apply(mixins::other_true_signal_disable(page.is_nurse_screen_after_doctor()))
                                                .event(clone!(page => move |_:events::Click| {
                                                    let now = js_now();
                                                    page.screening_report_date.set_neq(now.date().to_string());
                                                    page.screening_report_time.set_neq(now.time().js_string());
                                                    page.screen_changed.set_neq(true);
                                                }))
                                            }),
                                        ])
                                    }))
                                }),
                                html!("div", {
                                    .class("col-sm-6")
                                    .child(html!("div", {
                                        .class(class::INPUT_GROUP)
                                        .children([
                                            doms::date_picker(
                                                page.screening_see_doctor_date.clone(),
                                                page.screen_changed.clone(), page.is_nurse_screen_after_doctor(), None,
                                                |d| d.class(class::FLEX_GROW1).style("min-width","120px"),
                                                |d| d.class(class::COL_SM8_RE0),
                                                |d| d.class(class::COL_SM8_RE0),
                                                |s| s, always(None),
                                            ),
                                            doms::time_picker(
                                                page.screening_see_doctor_time.clone(),
                                                page.screen_changed.clone(), page.is_nurse_screen_after_doctor(), None,
                                                |d| d.class(class::FLEX_GROW1).style("min-width","95px"),
                                                |d| d.class(class::COL_SM4_RE0),
                                                |d| d.class(class::COL_SM4_RE0),
                                                |s| s, always(None),
                                            ),
                                            html!("button" => HtmlButtonElement, {
                                                .class(class::BTN_GRAY)
                                                .attr("type", "button")
                                                .attr("title", "ใช้วันที่-เวลา ปัจจุบัน")
                                                .child(html!("i", {.class(class::FA_CLOCK)}))
                                                .apply(mixins::other_true_signal_disable(page.is_nurse_screen_after_doctor()))
                                                .event(clone!(page => move |_:events::Click| {
                                                    let now = js_now();
                                                    page.screening_see_doctor_date.set_neq(now.date().to_string());
                                                    page.screening_see_doctor_time.set_neq(now.time().js_string());
                                                    page.screen_changed.set_neq(true);
                                                }))
                                            }),
                                        ])
                                    }))
                                }),
                            ])
                        }),
                    ])
                    .child_signal(page.screen_nurse_name.signal_cloned().map(|name| {
                        (!name.is_empty()).then(|| {
                            html!("div", {
                                .class(class::ROW)
                                .child(html!("div", {
                                    .class(class::COL_MD12_R)
                                    // .attr("id", "screening_nurse_doctorcode")
                                    .text("บันทึกโดย : ")
                                    .text(&name)
                                }))
                            })
                        })
                    }))
                    .child_signal(page.screen_doctor_name.signal_cloned().map(|name| {
                        (!name.is_empty()).then(|| {
                            html!("div", {
                                .class(class::ROW)
                                .child(html!("div", {
                                    .class(class::COL_MD12_R)
                                    // .attr("id", "screening_doctor_doctorcode")
                                    .text("บันทึกโดย : ")
                                    .text(&name)
                                }))
                            })
                        })
                    }))
                    .child_signal(page.screen_save_result.signal_cloned().map(|result| {
                        result.map(|success| {
                            let class = if success {"text-success"} else {"text-danger"};
                            let text = if success {"บันทึกข้อมูลสำเร็จ"} else {"บันทึกข้อมูลไม่สำเร็จ"};
                            html!("div", {
                                .class(class::ROW)
                                .child(html!("div", {
                                    .class("col-md-12")
                                    .children([
                                        html!("small", {
                                            .class("fw-bold")
                                            .class(class)
                                            // .attr("id", "lable_show_save_er_screening")
                                            .child(html!("i", {.class(class::FA_CHECK_CIRCLE)}))
                                            .text(text)
                                        }),
                                    ])
                                }))
                            })
                        })
                    }))
                    .apply_if(app.endpoint_is_allow(&Method::POST, &EndPoint::OpdErMedicalHistoryScreen, false), |dom| dom
                        .child(html!("div", {
                            .class("row")
                            .visible_signal(not(page.is_nurse_screen_after_doctor()))
                            .child(html!("div", {
                                .class(class::COL_MD12_R)
                                .children([
                                    html!("button" => HtmlButtonElement, {
                                        .attr("type", "button")
                                        // .attr("id", "SaveNurse_Screening")
                                        .class("btn")
                                        .class_signal("btn-primary", page.screen_changed.signal())
                                        .class_signal("btn-secondary", not(page.screen_changed.signal()))
                                        .child(html!("i", {.class(class::FA_SAVE)}))
                                        .text(" Save")
                                        .child(html!("span", {
                                            .class(class::SPIN_SM_BLUE)
                                            .attr("role", "status")
                                            // .attr("id", "spinnerER_SaveNurse_Screening")
                                            .visible_signal(map_ref!{
                                                let loading = app.loader_is_loading(),
                                                let saving = page.screen_saving.signal() =>
                                                *loading && *saving
                                            })
                                        }))
                                        .apply(mixins::click_with_loader_checked_or_true_disable_signal(clone!(app, page => move || {
                                            Self::save_screen(page.clone(), app.clone());
                                        }), not(page.screen_changed.signal()), app.state()))
                                    }),
                                    html!("button", {
                                        .attr("type", "button")
                                        // .attr("id", "CancalNurse_Screening")
                                        .class(class::BTN_R_GRAY)
                                        .child(html!("i", {.class(class::FA_X)}))
                                        .text(" Cancel")
                                        .event(clone!(page => move |_:events::Click| {
                                            if page.screen_changed.get() {
                                                page.screen_loaded.set(false);
                                            }
                                        }))
                                    }),
                                ])
                            }))
                        }))
                    )
                }))
            }))
        })
    }
    // has_scan - checkbox
    pub fn render_has_scan(page: Rc<Self>, app: Rc<App>) -> Dom {
        let scan_saved = Mutable::new(true);
        html!("div", {
            .future(map_ref!{
                let busy = app.loader_is_loading(),
                let saved = scan_saved.signal() =>
                !busy && !saved
            }.for_each(clone!(app, page, scan_saved => move |saving| {
                if saving {
                    scan_saved.set(true);
                    Self::save_scan(page.clone(), app.clone());
                }
                async {}
            })))
            .class("row")
            .apply_if(app.endpoint_is_allow(&Method::POST, &EndPoint::OpdErMedicalHistoryScan, false), |dom| dom
                .child(html!("div", {
                    .class("col-md-12")
                    .child(html!("div", {
                        .class(class::FORM_CHK_T)
                        .children([
                            html!("input" => HtmlInputElement, {
                                .attr("type", "checkbox")
                                .class("form-check-input")
                                .attr("id", "opd_er_document_scan")
                                .apply(mixins::checkbox_toggle(page.opd_er_document_scan.clone(), page.scan_saving.clone(), "Y", ""))
                                .apply(mixins::other_true_signal_disable(app.loader_is_loading()))
                                .future(page.scan_saving.signal().for_each(clone!(scan_saved => move |saving| {
                                    if saving {
                                        scan_saved.set(false);
                                    }
                                    async {}
                                })))
                            }),
                            html!("label", {
                                .class(class::FORM_CHK_LBL_BOLD)
                                .attr("for", "opd_er_document_scan")
                                .style("user-select","none")
                                .text("ผู้ป่วยมีเอกสาร")
                            }),
                            html!("small", {
                                .class("fw-bold")
                                // .attr("id", "lable_show_save_er_document_scan")
                            }),
                        ])
                        .child_signal(page.trauma_save_result.signal_cloned().map(|result| {
                            result.map(|success| {
                                let class = if success {"text-success"} else {"text-danger"};
                                let text = if success {"บันทึกข้อมูลสำเร็จ"} else {"บันทึกข้อมูลไม่สำเร็จ"};
                                html!("small", {
                                    .class("fw-bold")
                                    .class(class)
                                    // .attr("id", "lable_show_save_er_document_scan")
                                    .child(html!("i", {.class(class::FA_CHECK_CIRCLE)}))
                                    .text(text)
                                })
                            })
                        }))
                    }))
                }))
            )
        })
    }
    // consult - save
    pub fn render_consult(page: Rc<Self>, app: Rc<App>) -> Dom {
        html!("div", {
            .class(class::CARD)
            .style("break-inside","avoid")
            .child(html!("div", {
                .class("card-body")
                .child(html!("div", {
                    // .attr("id", "opd_er_consult")
                    .children([
                        html!("div", {
                            .class(class::ROW)
                            .child(html!("div", {
                                .class("col-md-12")
                                .children([
                                    html!("lable", {
                                        .class("fw-bold")
                                        .child(html!("i", {.class(class::FA_CLIPBOARD)}))
                                        .text(" Consult")
                                    }),
                                    html!("button", {
                                        .attr("type", "button")
                                        .class(class::BTN_R_BLUE)
                                        .child(html!("i", {.class(class::FA_EYE)}))
                                        .event(clone!(page => move |_:events::Click| {
                                            page.consult_visible.set(!page.consult_visible.get());
                                        }))
                                    }),
                                ])
                            }))
                        }),
                        html!("div", {
                            // .attr("id", "consult_card")
                            .visible_signal(page.consult_visible.signal())
                            .apply_if(app.endpoint_is_allow(&Method::POST, &EndPoint::OpdErMedicalHistoryConsult, false), |dom| dom
                                .child(html!("div", {
                                    .class(class::ROW)
                                    .child(html!("div", {
                                        .class("col-md-12")
                                        .children([
                                            html!("lable", {
                                                .text("เพิ่มฟอร์ม Consult")
                                            }),
                                            html!("button", {
                                                .attr("type", "button")
                                                .class(class::BTN_R_BLUE)
                                                .child(html!("i", {.class(class::FA_PLUS)}))
                                                .event(clone!(page => move |_: events::Click| {
                                                    page.er_consult_items.lock_mut().push_cloned(ConsultItem::new());
                                                    page.consult_changed.set_neq(true);
                                                }))
                                            }),
                                        ])
                                    }))
                                }))
                            )
                            .child(html!("div", {
                                .class(class::ROW)
                                .child(html!("div", {
                                    .class("col-md-12")
                                    // .child(html!("div", {
                                    //     .attr("id", "er-consult-input-div")
                                    // }))
                                    .children_signal_vec(page.er_consult_items.signal_vec_cloned().map(clone!(app, page => move |ci| {
                                        ConsultItem::render(ci, page.clone(), app.clone())
                                    })))
                                }))
                            }))
                            .child_signal(page.consult_doctor_name.signal_cloned().map(|name| {
                                (!name.is_empty()).then(|| {
                                    html!("div", {
                                        .class(class::ROW)
                                        .child(html!("label", {
                                            .class(class::COL_MD12_R)
                                            // .attr("id", "er_consult_doctorcode_name")
                                            .text("บันทึกโดย : ")
                                            .text(&name)
                                        }))
                                    })
                                })
                            }))
                            .child_signal(page.consult_save_result.signal_cloned().map(|result| {
                                result.map(|success| {
                                    let class = if success {"text-success"} else {"text-danger"};
                                    let text = if success {"บันทึกข้อมูลสำเร็จ"} else {"บันทึกข้อมูลไม่สำเร็จ"};
                                    html!("div", {
                                        .class(class::ROW)
                                        .child(html!("div", {
                                            .class("col-md-12")
                                            .children([
                                                html!("small", {
                                                    .class("fw-bold")
                                                    .class(class)
                                                    // .attr("id", "lable_show_save_er_dr_consult")
                                                    .child(html!("i", {.class(class::FA_CHECK_CIRCLE)}))
                                                    .text(text)
                                                }),
                                            ])
                                        }))
                                    })
                                })
                            }))
                            .apply_if(app.endpoint_is_allow(&Method::POST, &EndPoint::OpdErMedicalHistoryConsult, false), |dom| dom
                                .child(html!("div", {
                                    .class("row")
                                    .child(html!("div", {
                                        .class(class::COL_MD12_R)
                                        .children([
                                            html!("button" => HtmlButtonElement, {
                                                .attr("type", "button")
                                                // .attr("id", "SaveER_Consult")
                                                .class("btn")
                                                .class_signal("btn-primary", page.consult_changed.signal())
                                                .class_signal("btn-secondary", not(page.consult_changed.signal()))
                                                .child(html!("i", {.class(class::FA_SAVE)}))
                                                .text(" Save")
                                                .child(html!("span", {
                                                    .class(class::SPIN_SM_BLUE)
                                                    .attr("role", "status")
                                                    // .attr("id", "spinnerER_Consult")
                                                    .visible_signal(map_ref!{
                                                        let loading = app.loader_is_loading(),
                                                        let saving = page.consult_saving.signal() =>
                                                        *loading && *saving
                                                    })
                                                }))
                                                .apply(mixins::click_with_loader_checked_or_true_disable_signal(clone!(app, page => move || {
                                                    Self::save_consult(page.clone(), app.clone());
                                                }), not(page.consult_changed.signal()), app.state()))
                                            }),
                                            html!("button", {
                                                .attr("type", "button")
                                                // .attr("id", "CancalER_Consult")
                                                .class(class::BTN_R_GRAY)
                                                .child(html!("i", {.class(class::FA_X)}))
                                                .text(" Cancel")
                                                .event(clone!(page => move |_:events::Click| {
                                                    if page.consult_changed.get() {
                                                        page.consult_loaded.set(false);
                                                    }
                                                }))
                                            }),
                                        ])
                                    }))
                                }))
                            )
                        }),
                    ])
                }))
            }))
        })
    }
    // set_ft - save
    pub fn render_set_ft(page: Rc<Self>, app: Rc<App>) -> Dom {
        html!("div", {
            .class(class::CARD)
            .style("break-inside","avoid")
            .child(html!("div", {
                .class("card-body")
                .children([
                    html!("div", {
                        .class(class::ROW)
                        .child(html!("div", {
                            .class("col-md-12")
                            .children([
                                html!("lable", {
                                    .class("fw-bold")
                                    .child(html!("i", {.class(class::FA_CLIPBOARD)}))
                                    .text(" ประสานงาน Set Fast Track")
                                }),
                                html!("button", {
                                    .attr("type", "button")
                                    .class(class::BTN_R_BLUE)
                                    .child(html!("i", {.class(class::FA_EYE)}))
                                    .event(clone!(page => move |_:events::Click| {
                                        page.ft_visible.set(!page.ft_visible.get());
                                    }))
                                }),
                            ])
                        }))
                    }),
                    html!("div", {
                        // .attr("id", "opd_er_set_fact_track")
                        .visible_signal(page.ft_visible.signal())
                        .children([
                            html!("div", {
                                // .attr("id", "set_ft_card")
                                .children([
                                    html!("div", {
                                        .class(class::ROW)
                                        .child(html!("div", {
                                            .class("col-sm-6")
                                            .child(html!("div", {
                                                .class(class::INPUT_GROUP)
                                                .children([
                                                    doms::date_picker(
                                                        page.set_ft_date.clone(),
                                                        page.ft_changed.clone(), always(false), None,
                                                        |d| d.class(class::FLEX_GROW1).style("min-width","120px"),
                                                        |d| d.class(class::COL_SM8_RE0),
                                                        |d| d.class(class::COL_SM8_RE0),
                                                        |s| s, always(None),
                                                    ),
                                                    doms::time_picker(
                                                        page.set_ft_time.clone(),
                                                        page.ft_changed.clone(), always(false), None,
                                                        |d| d.class(class::FLEX_GROW1).style("min-width","95px"),
                                                        |d| d.class(class::COL_SM4_RE0),
                                                        |d| d.class(class::COL_SM4_RE0),
                                                        |s| s, always(None),
                                                    ),
                                                    html!("button", {
                                                        .class(class::BTN_GRAY)
                                                        .attr("type", "button")
                                                        .attr("title", "ใช้วันที่-เวลา ปัจจุบัน")
                                                        .child(html!("i", {.class(class::FA_CLOCK)}))
                                                        .event(clone!(page => move |_:events::Click| {
                                                            let now = js_now();
                                                            page.set_ft_date.set_neq(now.date().to_string());
                                                            page.set_ft_time.set_neq(now.time().js_string());
                                                            page.ft_changed.set_neq(true);
                                                        }))
                                                    }),
                                                ])
                                            }))
                                        }))
                                    }),
                                ])
                                .child_signal(page.ft_doctor_name.signal_cloned().map(|name| {
                                    (!name.is_empty()).then(|| {
                                        html!("div", {
                                            .class(class::ROW)
                                            .child(html!("label", {
                                                .class(class::COL_MD12_R)
                                                // .attr("id", "set_ft_doctor_name")
                                                .text("บันทึกโดย : ")
                                                .text(&name)
                                            }))
                                        })
                                    })
                                }))
                                .child_signal(page.ft_save_result.signal_cloned().map(|result| {
                                    result.map(|success| {
                                        let class = if success {"text-success"} else {"text-danger"};
                                        let text = if success {"บันทึกข้อมูลสำเร็จ"} else {"บันทึกข้อมูลไม่สำเร็จ"};
                                        html!("div", {
                                            .class(class::ROW)
                                            .child(html!("div", {
                                                .class("col-md-12")
                                                .children([
                                                    html!("small", {
                                                        .class("fw-bold")
                                                        .class(class)
                                                        // .attr("id", "lable_show_save_er_Set_FT")
                                                        .child(html!("i", {.class(class::FA_CHECK_CIRCLE)}))
                                                        .text(text)
                                                    }),
                                                ])
                                            }))
                                        })
                                    })
                                }))
                                .apply_if(app.endpoint_is_allow(&Method::POST, &EndPoint::OpdErMedicalHistoryFt, false), |dom| dom
                                    .child(html!("div", {
                                        .class("row")
                                        .child(html!("div", {
                                            .class(class::COL_MD12_R)
                                            .children([
                                                html!("button" => HtmlButtonElement, {
                                                    .attr("type", "button")
                                                    // .attr("id", "SaveSet_FT")
                                                    .class("btn")
                                                    .class_signal("btn-primary", page.ft_changed.signal())
                                                    .class_signal("btn-secondary", not(page.ft_changed.signal()))
                                                    .child(html!("i", {.class(class::FA_SAVE)}))
                                                    .text(" Save")
                                                    .child(html!("span", {
                                                        .class(class::SPIN_SM_BLUE)
                                                        .attr("role", "status")
                                                        // .attr("id", "spinnerER_Set_FT")
                                                        .visible_signal(map_ref!{
                                                            let loading = app.loader_is_loading(),
                                                            let saving = page.ft_saving.signal() =>
                                                            *loading && *saving
                                                        })
                                                    }))
                                                    .apply(mixins::click_with_loader_checked_or_true_disable_signal(clone!(app, page => move || {
                                                        Self::save_ft(page.clone(), app.clone());
                                                    }), not(page.ft_changed.signal()), app.state()))
                                                }),
                                                html!("button", {
                                                    .attr("type", "button")
                                                    // .attr("id", "CancalSet_FT")
                                                    .class(class::BTN_R_GRAY)
                                                    .child(html!("i", {.class(class::FA_X)}))
                                                    .text(" Cancel")
                                                    .event(clone!(page => move |_:events::Click| {
                                                        if page.ft_changed.get() {
                                                            page.ft_loaded.set(false);
                                                        }
                                                    }))
                                                }),
                                            ])
                                        }))
                                    }))
                                )
                            }),
                        ])
                    }),
                ])
            }))
        })
    }
}

#[derive(Clone, Default)]
struct DrugAllergy {
    id: u32,
    agent: Mutable<String>,
    symptom: Mutable<String>,
}

impl PartialEq<DrugAllergy> for DrugAllergy {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl From<&AllergyHistory> for DrugAllergy {
    fn from(item: &AllergyHistory) -> Self {
        Self {
            id: ID_COUNTER.fetch_add(1, Ordering::SeqCst),
            agent: Mutable::new(item.er_allergy_history_agent.clone().unwrap_or_default()),
            symptom: Mutable::new(item.er_allergy_history_symptom.clone().unwrap_or_default()),
        }
    }
}

impl DrugAllergy {
    fn new() -> Rc<Self> {
        Rc::new(Self {
            id: ID_COUNTER.fetch_add(1, Ordering::SeqCst),
            ..Default::default()
        })
    }

    fn render(da: Rc<Self>, page: Rc<OpdErEmergencyCpn>) -> Dom {
        html!("div", {
            .class("er_allergy_history_input_div")
            .child(html!("div", {
                .class("row")
                .children([
                    html!("div", {
                        .class("col-md-5")
                        .child(html!("div", {
                            .class("mb-3")
                            .child(html!("input" => HtmlInputElement, {
                                .attr("type", "text")
                                .style("color","#E60000")
                                .class(class::FORM_CTRL_COL_MD12)
                                .apply(mixins::string_value(da.agent.clone(), page.allergy_changed.clone()))
                            }))
                        }))
                    }),
                    html!("div", {
                        .class("col-md-5")
                        .child(html!("div", {
                            .class("mb-3")
                            .child(html!("input" => HtmlInputElement, {
                                .attr("type", "text")
                                .style("color","#E60000")
                                .class(class::FORM_CTRL_COL_MD12)
                                .apply(mixins::string_value(da.symptom.clone(), page.allergy_changed.clone()))
                            }))
                        }))
                    }),
                    html!("div", {
                        .class("col-md-1")
                        .child(html!("div", {
                            .class("mb-3")
                            .child(html!("button", {
                                .class(class::BTN_RED)
                                .attr("type", "button")
                                .child(html!("i", {.class(class::FA_TRASH)}))
                                .event(clone!(page => move |_: events::Click| {
                                    page.er_allergy_history_items.lock_mut().retain(|x| *x != da);
                                }))
                                // .attr("onclick", "removeER_Allergy_History(event,this)")
                            }))
                        }))
                    }),
                ])
            }))
        })
    }
}

#[derive(Clone, Default)]
struct ConsultItem {
    id: u32,
    er_consult_ward: Mutable<String>,
    er_consult_date: Mutable<String>, // Date
    er_consult_time: Mutable<String>, // Time
    er_consult_doctor_reply: Mutable<String>,
    er_consult_date_reply: Mutable<String>, // Date
    er_consult_time_reply: Mutable<String>, // Time,
}

impl PartialEq<ConsultItem> for ConsultItem {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl From<&ConsultHistory> for ConsultItem {
    fn from(item: &ConsultHistory) -> Self {
        Self {
            id: ID_COUNTER.fetch_add(1, Ordering::SeqCst),
            er_consult_ward: Mutable::new(item.er_consult_ward.clone().unwrap_or_default()),
            er_consult_date: Mutable::new(item.er_consult_date.map(|d| d.to_string()).unwrap_or_default()), // Date
            er_consult_time: Mutable::new(item.er_consult_time.map(|t| t.js_string()).unwrap_or_default()), // Time
            er_consult_doctor_reply: Mutable::new(item.er_consult_doctor_reply.clone().unwrap_or_default()),
            er_consult_date_reply: Mutable::new(item.er_consult_date_reply.map(|d| d.to_string()).unwrap_or_default()), // Date
            er_consult_time_reply: Mutable::new(item.er_consult_time_reply.map(|t| t.js_string()).unwrap_or_default()), // Time,
        }
    }
}

impl ConsultItem {
    fn new() -> Rc<Self> {
        Rc::new(Self {
            id: ID_COUNTER.fetch_add(1, Ordering::SeqCst),
            ..Default::default()
        })
    }

    fn render(ci: Rc<Self>, page: Rc<OpdErEmergencyCpn>, app: Rc<App>) -> Dom {
        let (doctor_select_option, spclty_kphis_select_option) = app
            .app_asset
            .lock_ref()
            .as_ref()
            .map(|opt| (opt.doctor_select_option.clone(), opt.spclty_kphis_select_option.clone()))
            .unwrap_or((Vec::new(), Vec::new()));

        html!("div", {
            .class("er_consult_input_div")
            .child(html!("div", {
                .class(class::CARD)
                .child(html!("div", {
                    .class("card-body")
                    .children([
                        html!("div", {
                            .class("row")
                            .child(html!("label", {
                                .text("แผนก")
                            }))
                        }),
                        html!("div", {
                            .class(class::ROW)
                            .children([
                                html!("div", {
                                    .class("col-sm-4")
                                    .child(html!("select" => HtmlSelectElement, {
                                        .class("form-select")
                                        .child(html!("option", {.attr("value", "").text("เลือก")}))
                                        .children(spclty_kphis_select_option.iter().map(|option| {
                                            doms::select_option(option, "")
                                        }))
                                        .apply(mixins::string_value_select(ci.er_consult_ward.clone(), page.consult_changed.clone()))
                                    }))
                                }),
                                html!("div", {
                                    .class(class::COL_SM7_PX0)
                                    .child(html!("div", {
                                        .class(class::INPUT_GROUP)
                                        .children([
                                            doms::date_picker(
                                                ci.er_consult_date.clone(),
                                                page.consult_changed.clone(), always(false), None,
                                                |d| d.class(class::FLEX_GROW1).style("min-width","120px"),
                                                |d| d.class(class::COL_SM8_RE0),
                                                |d| d.class(class::COL_SM8_RE0),
                                                |s| s, always(None),
                                            ),
                                            doms::time_picker(
                                                ci.er_consult_time.clone(),
                                                page.consult_changed.clone(), always(false), None,
                                                |d| d.class(class::FLEX_GROW1).style("min-width","95px"),
                                                |d| d.class(class::COL_SM4_RE0),
                                                |d| d.class(class::COL_SM4_RE0),
                                                |s| s, always(None),
                                            ),
                                            html!("button", {
                                                .class(class::BTN_GRAY)
                                                .attr("type", "button")
                                                .attr("title", "ใช้วันที่-เวลา ปัจจุบัน")
                                                .child(html!("i", {.class(class::FA_CLOCK)}))
                                                .event(clone!(page, ci => move |_: events::Click| {
                                                    let now = js_now();
                                                    ci.er_consult_date.set_neq(now.date().to_string());
                                                    ci.er_consult_time.set_neq(now.time().js_string());
                                                    page.consult_changed.set_neq(true);
                                                }))
                                                // .attr("onclick", "er_consult_ward_date_time(event,this)")
                                            }),
                                        ])
                                    }))
                                }),
                                html!("div", {
                                    .class("col-sm-1")
                                    .child(html!("button", {
                                        .class(class::BTN_RED)
                                        .attr("type", "button")
                                        .child(html!("i", {.class(class::FA_TRASH)}))
                                        .event(clone!(page, ci => move |_: events::Click| {
                                            page.er_consult_items.lock_mut().retain(|x| *x != ci);
                                        }))
                                        // .attr("onclick", "removeDoctorER_Consult(event,this)")
                                    }))
                                }),
                            ])
                        }),
                        html!("div", {
                            .class("row")
                            .child(html!("label", {
                                .text("แพทย์เวรที่มาดู")
                            }))
                        }),
                        html!("div", {
                            .class(class::ROW)
                            .children([
                                html!("div", {
                                    .class("col-sm-5")
                                    .child(html!("select" => HtmlSelectElement, {
                                        .class("form-select")
                                        .child(html!("option", {.attr("value", "").text("เลือก")}))
                                        .children(doctor_select_option.iter().map(|option| {
                                            doms::select_option(option, "")
                                        }))
                                        .apply(mixins::string_value_select(ci.er_consult_doctor_reply.clone(), page.consult_changed.clone()))
                                    }))
                                }),
                                html!("div", {
                                    .class(class::COL_SM7_PX0)
                                    .child(html!("div", {
                                        .class(class::INPUT_GROUP)
                                        .children([
                                            doms::date_picker(
                                                ci.er_consult_date_reply.clone(),
                                                page.consult_changed.clone(), always(false), None,
                                                |d| d.class(class::FLEX_GROW1).style("min-width","120px"),
                                                |d| d.class(class::COL_SM8_RE0),
                                                |d| d.class(class::COL_SM8_RE0),
                                                |s| s, always(None),
                                            ),
                                            doms::time_picker(
                                                ci.er_consult_time_reply.clone(),
                                                page.consult_changed.clone(), always(false), None,
                                                |d| d.class(class::FLEX_GROW1).style("min-width","95px"),
                                                |d| d.class(class::COL_SM4_RE0),
                                                |d| d.class(class::COL_SM4_RE0),
                                                |s| s, always(None),
                                            ),
                                            html!("button", {
                                                .class(class::BTN_GRAY)
                                                .attr("type", "button")
                                                .attr("title", "ใช้วันที่-เวลา ปัจจุบัน")
                                                .child(html!("i", {.class(class::FA_CLOCK)}))
                                                .event(clone!(page => move |_: events::Click| {
                                                    let now = js_now();
                                                    ci.er_consult_date_reply.set_neq(now.date().to_string());
                                                    ci.er_consult_time_reply.set_neq(now.time().js_string());
                                                    page.consult_changed.set_neq(true);
                                                }))
                                                // .attr("onclick", "er_consult_date_time_reply(event,this)")
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
}

fn render_pe(label: &str, yn_opt: &Option<String>, pe_text: &Option<String>) -> Dom {
    html!("div", {
        .class("ms-3")
        .children([
            html!("span", {.class("fw-bold").text(label).text(" : ")}),
            html!("span", {.text(render_is_normal(yn_opt)).text(&pe_text.clone().unwrap_or_default())})
        ])
    })
}

fn render_is_normal(yn_opt: &Option<String>) -> &'static str {
    match yn_opt.as_ref() {
        Some(yn) => match yn.as_str() {
            "Y" => "ผิดปกติ ",
            "N" => "ปกติ ",
            _ => "",
        },
        None => "",
    }
}
