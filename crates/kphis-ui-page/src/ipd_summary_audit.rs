use dominator::{Dom, DomBuilder, EventOptions, clone, events, html, window_size, with_node};
use futures_signals::{
    map_ref,
    signal::{Mutable, ReadOnlyMutable, Signal, SignalExt, always, not, option},
    signal_vec::{MutableVec, SignalVecExt},
};
use std::{
    collections::HashSet,
    rc::Rc,
    sync::{
        Arc,
        atomic::{AtomicU32, Ordering},
    },
};
use time::{PrimitiveDateTime, Time};
use web_sys::{HtmlButtonElement, HtmlInputElement, HtmlSelectElement};

use kphis_drg_worker::drg::model::{GrouperInput, GrouperOutput, I10vx};
use kphis_model::{
    SCREEN_WIDTH_EXTRA,
    app::DragStartState,
    endpoint::EndPoint,
    fetch::Method,
    ipd::{
        his::{HisIptDiag, HisIptOprt},
        summary::{AuditStatus, DoctorData, DxData, LabAlertData, Summary, SummaryData, SummaryParams, SummaryStatus},
        summary_audit::{SummaryAudit, SummaryAuditItem, SummaryAuditParams},
    },
    report::{SystemReport, TypstReport},
    route::Route,
    sse::SsePostMessage,
};
use kphis_ui_app::{App, DaggerAsteriskState};
use kphis_ui_component::{
    gadget::{
        aside_resizer::AsideResizerCpn,
        pdf_button::PdfButtons,
        searchbox::{dx::DxSearchboxCpn, proc::ProcSearchboxCpn},
    },
    modal::{blank_modal, lab_history::LabHistory},
    show_patient_main::ShowPatientMainCpn,
    summary_note::{SummaryNoteCpn, render_lab_alert, render_problem_list},
};
use kphis_ui_core::{class, doms, mixins};
use kphis_util::{
    datetime::{JsTime, date_8601, datetime_8601, datetime_th_opt_relative},
    util::{f32_rescale, f64_rescale, icd10_dot, los_f32_to_u32, str_some, zero_none},
};

const SEX_ITEMS: [(&str, &str); 2] = [("1", "Male"), ("2", "Female")];

const DCH_STTS_ITEMS: [(&str, &str); 9] = [
    ("01", "Complete Recovery"),
    ("02", "Improved"),
    ("03", "Not Improved"),
    ("04", "Normal Delivery"),
    ("05", "Un-Delivery"),
    ("06", "Normal Child Discharged with Mother"),
    ("07", "Normal Child Discharged separately"),
    ("08", "Dead Still Birth"),
    ("09", "Dead"),
];

const DCH_TYPE_ITEMS: [(&str, &str); 7] = [
    ("01", "With Approval"),
    ("02", "Against Advice"),
    ("03", "By Escape"),
    ("04", "By Transfer"),
    ("05", "Other (specify)"),
    ("08", "Dead Autopsy"),
    ("09", "Dead Non Autopsy"),
];

static ID_COUNTER: AtomicU32 = AtomicU32::new(0);

/// - GET `EndPoint::IpdSummaryAudit`
/// - GET `EndPoint::IpdSummaryNoteId` (SummaryNoteCpn)
/// - GET `EndPoint::IpdShowPatientMainAn` (ShowPatientMainCpn)
/// - GET `EndPoint::OpdErShowPatientMainVn` (ShowPatientMainCpn)
/// - POST `EndPoint::IpdSummaryAudit` (guarded, remove 'แก้ไข','บันทึก' btn)
/// - DELETE `EndPoint::IpdSummaryAudit` (guarded, remove 'ลบ' btn)
/// - POST `EndPoint::IpdSummaryNoteId `(SummaryNoteCpn, guarded, remove note-edit div)
/// - PATCH `EndPoint::IpdSummaryNoteId` (SummaryNoteCpn, guarded, remove 'บันทึก' btn)
/// - DELETE `EndPoint::IpdSummaryNoteId` (SummaryNoteCpn, guarded, remove 'ลบ' btn)
#[derive(Clone, Default)]
pub struct IpdSummaryAuditPage {
    list_loaded: Mutable<bool>,
    ipd_summary_audit_list: MutableVec<Rc<SummaryAudit>>,
    patient: Mutable<Rc<ShowPatientMainCpn>>,

    // summary related data
    summary_data: Mutable<Option<Rc<SummaryData>>>,
    // from doctor
    doctor_pdx: MutableVec<Rc<DxData>>,
    doctor_sdxs: MutableVec<Rc<DxData>>,
    doctor_odxs: MutableVec<Rc<DxData>>,
    doctor_ops: MutableVec<Rc<DxData>>,
    // from coder
    coder_pdxs: MutableVec<Rc<DxData>>,
    coder_sdxs: MutableVec<Rc<DxData>>,
    coder_odxs: MutableVec<Rc<DxData>>,
    coder_ops: MutableVec<Rc<DxData>>,
    coder_dagger_asterisk_state: Rc<DaggerAsteriskState>,
    coder_dagger_asterisk_pairs: Mutable<Vec<(Option<Arc<I10vx>>, Arc<I10vx>)>>,
    // in his
    his_pdx: MutableVec<Rc<DxData>>,
    his_sdxs: MutableVec<Rc<DxData>>,
    his_odxs: MutableVec<Rc<DxData>>,
    his_ops: MutableVec<Rc<DxData>>,
    his_dagger_asterisk_state: Rc<DaggerAsteriskState>,
    his_dagger_asterisk_pairs: Mutable<Vec<(Option<Arc<I10vx>>, Arc<I10vx>)>>,

    all_doctors: MutableVec<Rc<DoctorData>>,

    ipd_summary_audit_selected: Mutable<Option<Rc<SummaryAudit>>>,
    ipd_summary_audit_mutable: Mutable<Option<Rc<IpdSummaryAuditMutable>>>,

    an: Mutable<String>,

    summary_id: Mutable<u32>,
    status: Mutable<AuditStatus>,
    status_loaded: Mutable<bool>,
    status_changed: Mutable<bool>,

    lab_alerts: MutableVec<Rc<LabAlertData>>,
    lab_history_modal: Mutable<Option<Rc<LabHistory>>>,
    problem_lists: MutableVec<String>,
}

impl IpdSummaryAuditPage {
    pub fn new(an: String) -> Rc<Self> {
        Rc::new(Self {
            an: Mutable::new(an),
            ..Default::default()
        })
    }

    fn load_list(page: Rc<Self>, app: Rc<App>) {
        if let Some(an) = str_some(page.an.get_cloned()) {
            page.ipd_summary_audit_selected.set(None);
            page.ipd_summary_audit_mutable.set(None);
            page.ipd_summary_audit_list.lock_mut().clear();
            page.doctor_pdx.lock_mut().clear();
            page.doctor_sdxs.lock_mut().clear();
            page.doctor_odxs.lock_mut().clear();
            page.doctor_ops.lock_mut().clear();
            page.coder_pdxs.lock_mut().clear();
            page.coder_sdxs.lock_mut().clear();
            page.coder_odxs.lock_mut().clear();
            page.coder_ops.lock_mut().clear();
            page.his_pdx.lock_mut().clear();
            page.his_sdxs.lock_mut().clear();
            page.his_odxs.lock_mut().clear();
            page.his_ops.lock_mut().clear();
            app.async_load(
                true,
                clone!(app => async move {
                    let mut has_summary = false;
                    let icd_worker = app.drg_worker().await;
                    // GET `EndPoint::HisIptDiagAn`
                    match HisIptDiag::call_api_get(&an, app.state()).await {
                        Ok(responses) => {
                            let mut his_pdx = Vec::new();
                            let mut his_sdxs = Vec::new();
                            let mut his_odxs = Vec::new();
                            for dx in responses {
                                match dx.diagtype {
                                    Some(s) => {
                                        match s.as_str() {
                                            "1" => {
                                                his_pdx.push(DxData::new(1, dx.icd10.clone(), str_some(icd_worker.get_icd10_desc(dx.icd10.clone().unwrap_or_default()).await)));
                                                if let Some(icd) = dx.icd10.as_ref() {
                                                    page.his_dagger_asterisk_state.insert_code(icd);
                                                }
                                            }
                                            "2" | "3" => {
                                                his_sdxs.push(DxData::new(2, dx.icd10.clone(), str_some(icd_worker.get_icd10_desc(dx.icd10.clone().unwrap_or_default()).await)));
                                                if let Some(icd) = dx.icd10.as_ref() {
                                                    page.his_dagger_asterisk_state.insert_code(icd);
                                                }
                                            }
                                            "4" => {
                                                his_odxs.push(DxData::new(4, dx.icd10.clone(), str_some(icd_worker.get_icd10_desc(dx.icd10.clone().unwrap_or_default()).await)));
                                                if let Some(icd) = dx.icd10.as_ref() {
                                                    page.his_dagger_asterisk_state.insert_code(icd);
                                                }
                                            }
                                            _ => {}
                                        }
                                    }
                                    None => {}
                                }
                            }
                            page.his_pdx.lock_mut().extend(his_pdx.into_iter());
                            page.his_sdxs.lock_mut().extend(his_sdxs.into_iter());
                            page.his_odxs.lock_mut().extend(his_odxs.into_iter());
                            page.his_dagger_asterisk_state.start_parsing();
                        }
                        Err(e) => {
                            app.alert_app_error(&e).await;
                        }
                    }
                    // GET `EndPoint::HisIptOprtAn`
                    match HisIptOprt::call_api_get(&an, app.state()).await {
                        Ok(responses) => {
                            let mut his_ops = Vec::with_capacity(responses.len());
                            for op in responses {
                                his_ops.push(DxData::new(9, op.icd9.clone(), str_some(icd_worker.get_proc_desc(op.icd9.clone().unwrap_or_default()).await)));
                            }
                            page.his_ops.lock_mut().extend(his_ops);
                        }
                        Err(e) => {
                            app.alert_app_error(&e).await;
                        }
                    }
                    // GET `EndPoint::IpdSummary`
                    let summary_params = SummaryParams { an: Some(an.clone()), ..Default::default() };
                    match Summary::call_api_get(&summary_params, app.state()).await {
                        Ok(summary) => {
                            if let Some(sum) = summary.summary.as_ref() {
                                page.summary_id.set(sum.summary_id);
                                // PDx dx-text + code
                                page.doctor_pdx.lock_mut().extend(vec![DxData::new(1, sum.principal_diagnosis_icd10.clone(), sum.principal_diagnosis.clone())]);
                                if let Some(code) = sum.principal_diagnosis_code.as_ref() {
                                    let icd = code.trim().replace('.', "");
                                    page.coder_pdxs.lock_mut().push_cloned(DxData::new(1, str_some(icd.to_owned()), str_some(icd_worker.get_icd10_desc(icd.to_owned()).await)));
                                    page.coder_dagger_asterisk_state.insert_code(&icd);
                                }
                                // SDx + ODx codes
                                let mut coder_sdxs = Vec::new();
                                let mut coder_odxs = Vec::new();
                                if let Some(codes) = sum.pre_admission_comorbidity_codes.as_ref() {
                                    for code in codes.split(',') {
                                        let icd = code.trim().replace('.', "");
                                        coder_sdxs.push(DxData::new(2, str_some(icd.to_owned()), str_some(icd_worker.get_icd10_desc(icd.to_owned()).await)));
                                        page.coder_dagger_asterisk_state.insert_code(&icd);
                                    }
                                }
                                if let Some(codes) = sum.post_admission_comorbidity_codes.as_ref() {
                                    for code in codes.split(',') {
                                        let icd = code.trim().replace('.', "");
                                        coder_sdxs.push(DxData::new(3, str_some(icd.to_owned()), str_some(icd_worker.get_icd10_desc(icd.to_owned()).await)));
                                        page.coder_dagger_asterisk_state.insert_code(&icd);
                                    }
                                }
                                if let Some(codes) = sum.other_diagnosis_codes.as_ref() {
                                    for code in codes.split(',') {
                                        let icd = code.trim().replace('.', "");
                                        coder_odxs.push(DxData::new(4, str_some(icd.to_owned()), str_some(icd_worker.get_icd10_desc(icd.to_owned()).await)));
                                        page.coder_dagger_asterisk_state.insert_code(&icd);
                                    }
                                }
                                page.coder_sdxs.lock_mut().extend(coder_sdxs);
                                page.coder_odxs.lock_mut().extend(coder_odxs);
                                page.coder_dagger_asterisk_state.start_parsing();
                                // Op codes
                                let other_procs = sum.other_procedure_codes.as_ref().map(|concat| concat.split(',').map(|s| s.trim().to_owned()).collect::<Vec<String>>()).unwrap_or_default();
                                let mut sum_procs = Vec::with_capacity(other_procs.len() + 1);
                                if let Some(main_proc) = &sum.main_procedure_code {
                                    sum_procs.push(DxData::new(9, str_some(main_proc.to_owned()), str_some(icd_worker.get_proc_desc(main_proc.to_owned()).await)));
                                }
                                for other_proc in other_procs {
                                    sum_procs.push(DxData::new(9, str_some(other_proc.to_owned()), str_some(icd_worker.get_proc_desc(other_proc.to_owned()).await)));
                                }
                                page.coder_ops.lock_mut().extend(sum_procs);
                                // Op dx-text
                                let mut op_data = summary.or_data.iter().map(|op| DxData::new(9, op.icd9.clone(), op.name.clone())).collect::<Vec<Rc<DxData>>>();
                                if sum.tracheostomy.as_ref().map(|s| s == "Y").unwrap_or_default() {
                                    op_data.push(DxData::new(9, Some(String::from("311")), Some(String::from("Temporary tracheostomy"))));
                                }
                                if sum.packed_redcells.as_ref().map(|s| s == "Y").unwrap_or_default() {
                                    op_data.push(DxData::new(9, Some(String::from("9904")), Some(String::from("Packed cell transfusion"))));
                                }
                                if sum.fresh_frozen_plasma.as_ref().map(|s| s == "Y").unwrap_or_default() {
                                    op_data.push(DxData::new(9, Some(String::from("9907")), Some(String::from("Serum transfusion NEC"))));
                                }
                                if sum.platelets.as_ref().map(|s| s == "Y").unwrap_or_default() {
                                    op_data.push(DxData::new(9, Some(String::from("9905")), Some(String::from("Platelet transfusion"))));
                                }
                                if sum.cryoprecipitate.as_ref().map(|s| s == "Y").unwrap_or_default() {
                                    op_data.push(DxData::new(9, Some(String::from("9906")), Some(String::from("Coag factor transfusion"))));
                                }
                                if sum.whole_blood.as_ref().map(|s| s == "Y").unwrap_or_default() {
                                    op_data.push(DxData::new(9, Some(String::from("9903")), Some(String::from("Whole blood transfus NEC"))));
                                }
                                if sum.chemotherapy.as_ref().map(|s| s == "Y").unwrap_or_default() {
                                    op_data.push(DxData::new(9, Some(String::from("9925")), Some(String::from("Inject ca chemother NEC"))));
                                }
                                if sum.hemodialysis.as_ref().map(|s| s == "Y").unwrap_or_default() {
                                    op_data.push(DxData::new(9, Some(String::from("3995")), Some(String::from("Hemodialysis"))));
                                }
                                match sum.mechanical_ventilation.clone().unwrap_or_default().as_str() {
                                    "1" => {
                                        op_data.push(DxData::new(9, Some(String::from("9672")), Some(String::from("Cont mech vent 96+ hrs"))));
                                    }
                                    "2" => {
                                        op_data.push(DxData::new(9, Some(String::from("9671")), Some(String::from("Cont mech vent < 96 hrs"))));
                                    }
                                    _ => {}
                                }
                                page.doctor_ops.lock_mut().extend(op_data.into_iter());
                            }
                            // SDx + ODx dx-text
                            page.doctor_sdxs.lock_mut().extend(summary.dx_data.iter().filter_map(|i| ([2,3].contains(&i.ty)).then(|| Rc::new(i.clone()))));
                            page.doctor_odxs.lock_mut().extend(summary.dx_data.iter().filter_map(|i| (i.ty == 4).then(|| Rc::new(i.clone()))));
                            // Doctors
                            page.all_doctors.lock_mut().extend(summary.doctor_data.into_iter().map(Rc::new));
                            // labs alert and problem list
                            page.lab_alerts.lock_mut().extend(summary.lab_alert_data.into_iter().map(Rc::new));
                            page.problem_lists.lock_mut().replace_cloned(summary.problem_list_data);
                            // Fin
                            page.summary_data.set(summary.summary.map(Rc::new));
                            has_summary = true;
                        }
                        Err(e) => {
                            app.alert_app_error(&e).await;
                        }
                    }
                    if has_summary {
                        // GET `EndPoint::SummaryAudit`
                        let audit_params = SummaryAuditParams { an: Some(an), ..Default::default() };
                        match SummaryAudit::call_api_get(&audit_params, app.state()).await {
                            Ok(responses) => {
                                page.ipd_summary_audit_list.lock_mut().extend(responses.into_iter().map(Rc::new));
                            }
                            Err(e) => {
                                app.alert_app_error(&e).await;
                            }
                        }
                    }
                }),
            )
        }
    }

    fn load_status(page: Rc<Self>, app: Rc<App>) {
        if let Some(summary_id) = zero_none(page.summary_id.get()) {
            app.async_load(
                true,
                clone!(app => async move {
                    // GET `EndPoint::IpdSummaryStatusId`
                    match SummaryStatus::call_api_get(summary_id, app.state()).await {
                        Ok(opt) => {
                            page.status.set(opt.map(|summary_status| AuditStatus::from_summary_status(summary_status)).unwrap_or_default());
                        }
                        Err(e) => {
                            app.alert_app_error(&e).await;
                        }
                    }
                }),
            )
        }
    }

    fn update_status(page: Rc<Self>, app: Rc<App>) {
        if let Some(summary_id) = zero_none(page.summary_id.get()) {
            let status = page.status.get_cloned();
            app.async_load(
                true,
                clone!(app => async move {
                    let saver = SummaryStatus { status: status.as_data() };
                    // PATCH `EndPoint::IpdSummaryStatusId`
                    match saver.call_api_put(summary_id, app.state()).await {
                        Ok(response) => {
                            app.alert_execute_response(&response, clone!(app => async move {
                                // app.alert("บันทึกข้อมูลสำเร็จ");
                                page.status_changed.set(false);
                                page.status_loaded.set(false);
                                if matches!(status, AuditStatus::Review) {
                                    Self::send_review_sse(page.clone(), app.clone());
                                }
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

    fn send_review_sse(page: Rc<Self>, app: Rc<App>) {
        let an = page.an.get_cloned();

        let doctors = page.all_doctors.lock_ref().iter().map(|d| d.doctor.clone()).collect::<HashSet<String>>();
        for doctor in doctors {
            let message = SsePostMessage {
                message: ["AN: ", &an, " รอแพทย์ทบทวนการสรุปเวชระเบียน"].concat(),
                person: Some(doctor),
                route: Some(Route::Summary {
                    view_by: String::from("doctor"),
                    an: an.clone(),
                }),
                ..Default::default()
            };
            app.send_sse(message);
        }
    }

    fn init_inner(audit: Rc<SummaryAudit>, page: Rc<Self>) {
        page.ipd_summary_audit_selected.set(Some(audit));
        let audit_mutable = IpdSummaryAuditMutable::new(page.clone());
        audit_mutable.set_selected();
        page.ipd_summary_audit_mutable.set(Some(audit_mutable));
    }

    fn new_items(&self) -> Vec<SummaryAuditItem> {
        if let Some(summary) = self.summary_data.get_cloned() {
            let summary_id = summary.summary_id;

            // prepare Dx
            let his_pdx = self.his_pdx.lock_ref().first().and_then(|dx| dx.icd.clone());
            let doctor_pdx = self.doctor_pdx.lock_ref().first().and_then(|dx| dx.detail.clone());
            let pdx_code = self.coder_pdxs.lock_ref().first().and_then(|dx| dx.icd.clone());

            let mut his_sdxs = self.his_sdxs.lock_ref().iter().filter_map(|dx| dx.icd.clone()).collect::<Vec<String>>();
            let mut his_odxs = self.his_odxs.lock_ref().iter().filter_map(|dx| dx.icd.clone()).collect::<Vec<String>>();

            let doctor_sdxs = self.doctor_sdxs.lock_ref().iter().filter_map(|dx| dx.detail.clone()).collect::<Vec<String>>();
            let doctor_odxs = self.doctor_odxs.lock_ref().iter().filter_map(|dx| dx.detail.clone()).collect::<Vec<String>>();

            let coder_sdxs = self.coder_sdxs.lock_ref().iter().filter_map(|dx| dx.icd.clone()).collect::<Vec<String>>();
            let coder_odxs = self.coder_odxs.lock_ref().iter().filter_map(|dx| dx.icd.clone()).collect::<Vec<String>>();

            let sdxs_max_len = [his_sdxs.len(), doctor_sdxs.len(), coder_sdxs.len()].into_iter().max().unwrap_or_default();
            let odxs_max_len = [his_odxs.len(), doctor_odxs.len(), coder_odxs.len()].into_iter().max().unwrap_or_default();

            // prepare Procs
            let mut his_ops = self.his_ops.lock_ref().iter().filter_map(|dx| dx.icd.clone()).collect::<Vec<String>>();
            let mut doctor_ops = self.doctor_ops.lock_ref().to_vec();
            let coder_ops = self.coder_ops.lock_ref().iter().filter_map(|dx| dx.icd.clone()).collect::<Vec<String>>();

            let ops_max_len = [coder_ops.len(), doctor_ops.len(), his_ops.len()].into_iter().max().unwrap_or_default();

            // Put together
            let mut results = Vec::with_capacity(1 + sdxs_max_len + odxs_max_len + ops_max_len);
            // Add PDx
            results.push(SummaryAuditItem::new("PDx", summary_id, &doctor_pdx, &pdx_code, &his_pdx));

            // Add SDx
            for i in 0..sdxs_max_len {
                let dx = coder_sdxs.get(i).cloned();
                let matched_his = dx.as_ref().and_then(|c| {
                    if let Some(pos) = his_sdxs.iter().position(|i| i == c) {
                        Some(his_sdxs.swap_remove(pos))
                    } else {
                        None
                    }
                });
                results.push(SummaryAuditItem::new("SDx", summary_id, &doctor_sdxs.get(i).cloned(), &dx, &matched_his));
            }
            for dx in his_sdxs {
                results.push(SummaryAuditItem::new("SDx", summary_id, &None, &None, &Some(dx)));
            }

            // Add ODx
            for i in 0..odxs_max_len {
                let dx = coder_odxs.get(i).cloned();
                let matched_his = dx.as_ref().and_then(|c| {
                    if let Some(pos) = his_odxs.iter().position(|i| i == c) {
                        Some(his_odxs.swap_remove(pos))
                    } else {
                        None
                    }
                });
                results.push(SummaryAuditItem::new("ODx", summary_id, &doctor_odxs.get(i).cloned(), &dx, &matched_his));
            }
            for dx in his_odxs {
                results.push(SummaryAuditItem::new("ODx", summary_id, &None, &None, &Some(dx)));
            }

            // Add Procs
            for sum_proc in coder_ops {
                let matched_or_data = if let Some(pos) = doctor_ops.iter().position(|i| i.icd.as_ref().map(|op| *op == sum_proc).unwrap_or_default()) {
                    Some(doctor_ops.swap_remove(pos))
                } else {
                    None
                };
                let matched_his = if let Some(pos) = his_ops.iter().position(|s| *s == sum_proc) {
                    Some(his_ops.swap_remove(pos))
                } else {
                    None
                };
                results.push(SummaryAuditItem::new(
                    "Op",
                    summary_id,
                    &matched_or_data.and_then(|tuple| tuple.detail.clone()),
                    &Some(sum_proc),
                    &matched_his,
                ));
            }
            for op_data in doctor_ops {
                let matched_his = if let Some(pos) = his_ops.iter().position(|s| op_data.icd.as_ref().map(|op| op == s).unwrap_or_default()) {
                    Some(his_ops.swap_remove(pos))
                } else {
                    None
                };
                results.push(SummaryAuditItem::new("Op", summary_id, &op_data.detail, &op_data.icd, &matched_his));
            }
            for his_ops in his_ops {
                results.push(SummaryAuditItem::new("Op", summary_id, &None, &Some(his_ops), &None));
            }

            results
        } else {
            Vec::new()
        }
    }

    pub fn render(page: Rc<Self>, app: Rc<App>) -> Dom {
        app.set_title("KPHIS - IPD Summary Audit");

        let show_patient_main = ShowPatientMainCpn::new_with_an(page.an.get_cloned());
        let hn = show_patient_main.hn.clone();
        let patient_main = ShowPatientMainCpn::render(false, show_patient_main.clone(), app.clone());
        page.patient.set(show_patient_main);

        html!("div", {
            .child(patient_main)
            .child_signal(window_size().map(|ws| ws.width < SCREEN_WIDTH_EXTRA).dedupe().map(clone!(app, page => move |is_not_wide| {
                Some(if is_not_wide {
                    Self::render_body(page.clone(), app.clone())
                } else {
                    // aside_resizer
                    let report_selected = SystemReport::new(&app.report_select.lock_ref());
                    AsideResizerCpn::render(
                        Self::render_body(page.clone(), app.clone()),
                        Some((true, page.patient.lock_ref().patient.clone())),
                        AsideResizerCpn::new(
                            Mutable::new(report_selected), Mutable::new(false),
                            Mutable::new(None), Mutable::new(false),
                            page.an.clone(), hn.clone(), SystemReport::ipd_set(),
                            "ipd-summary-audit-page-main", None, None, app.clone(),
                        ),
                        app.clone(),
                    )
                })
            })))
            .child(html!("div", {
                .class("modal")
                .attr("id", "labHistoryModal")
                .attr("role", "dialog")
                .attr("tabindex", "-1")
                .child_signal(page.lab_history_modal.signal_cloned().map(move |opt| {
                    opt.as_ref().map(clone!(app => move |modal| {
                        LabHistory::render(modal.clone(), app, None)
                    })).or(Some(blank_modal()))
                }))
            }))
        })
    }

    pub fn render_body(page: Rc<Self>, app: Rc<App>) -> Dom {
        html!("div", {
            .future(map_ref!(
                let busy = app.loader_is_loading(),
                let loaded = page.list_loaded.signal() =>
                !busy && !loaded
            ).for_each(clone!(app, page => move |ready| {
                if ready {
                    Self::load_list(page.clone(), app.clone());
                    page.list_loaded.set_neq(true);
                }
                async {}
            })))
            .future(map_ref!(
                let busy = app.loader_is_loading(),
                let is_parsing_dagger_asterisk = page.coder_dagger_asterisk_state.is_parsing_signal() =>
                !busy && *is_parsing_dagger_asterisk
            ).for_each(clone!(app, page => move |ready| {
                if ready {
                    DaggerAsteriskState::parse_codes(page.coder_dagger_asterisk_state.clone(), app.clone());
                }
                async {}
            })))
            .future(page.coder_dagger_asterisk_state.is_parsed_signal().for_each(clone!(page => move |is_parsed| {
                if is_parsed {
                    page.coder_dagger_asterisk_pairs.set(page.coder_dagger_asterisk_state.get_all_pairs());
                }
                async {}
            })))
            .future(map_ref!(
                let busy = app.loader_is_loading(),
                let is_parsing_dagger_asterisk = page.his_dagger_asterisk_state.is_parsing_signal() =>
                !busy && *is_parsing_dagger_asterisk
            ).for_each(clone!(app, page => move |ready| {
                if ready {
                    DaggerAsteriskState::parse_codes(page.his_dagger_asterisk_state.clone(), app.clone());
                }
                async {}
            })))
            .future(page.his_dagger_asterisk_state.is_parsed_signal().for_each(clone!(page => move |is_parsed| {
                if is_parsed {
                    page.his_dagger_asterisk_pairs.set(page.his_dagger_asterisk_state.get_all_pairs());
                }
                async {}
            })))
            .future(map_ref!(
                let busy = app.loader_is_loading(),
                let loaded = page.status_loaded.signal() =>
                !busy && !loaded
            ).for_each(clone!(app, page => move |ready| {
                if ready {
                    Self::load_status(page.clone(), app.clone());
                    page.status_loaded.set_neq(true);
                }
                async {}
            })))
            .class(class::CONF_P3)
            .attr("id", "ipd-summary-audit-page-main")
            .style("min-width","920px")
            .children([
                html!("div", {
                    .class(class::ROW)
                    .children([
                        html!("div", {
                            .class("col-auto")
                            .child(html!("button", {
                                .attr("type", "button")
                                .class(class::BTN_LT_BLUE)
                                .child(html!("i", {.class(class::FA_L_ARROW)}))
                                .text(" กลับ")
                                .event(clone!(app => move |_: events::Click| {
                                    if app.go_back_else() {
                                        for route in [
                                            Route::IpdPostAdmitList { view_by: String::from("doctor") },
                                            Route::IpdPostAdmitList { view_by: String::from("nurse") },
                                            Route::IpdPostAdmitList { view_by: String::from("pharmacist") },
                                            Route::IpdPostAdmitList { view_by: String::from("other") },
                                            Route::Info,
                                        ] {
                                            if route.has_permission(app.state()) {
                                                route.hard_redirect();
                                                break;
                                            }
                                        }
                                    }
                                }))
                            }))
                        }),
                        html!("div", {
                            .class(class::COLA_BOLD_P2)
                            .text("IPD SUMMARY / CODING AUDIT")
                        }),
                    ])
                }),
                html!("hr"),
                html!("div", {
                    .class(class::FLEX_WRAP_T)
                    .children_signal_vec(page.ipd_summary_audit_list.signal_vec_cloned().map(clone!(page, app => move |audit| {
                        let summary_audit_id = audit.summary_audit_id;
                        html!("button" => HtmlButtonElement, {
                            .attr("type", "button")
                            .class(class::BTN_LT)
                            .class_signal("btn-primary", page.ipd_summary_audit_selected.signal_cloned().map(move |opt| opt.as_ref().map(|selected_audit| selected_audit.summary_audit_id == summary_audit_id).unwrap_or_default()))
                            .class_signal("btn-secondary", not(page.ipd_summary_audit_selected.signal_cloned().map(move |opt| opt.as_ref().map(|selected_audit| selected_audit.summary_audit_id == summary_audit_id).unwrap_or_default())))
                            .text(if audit.audit_type.as_str() == "E" {"External"} else {"Internal"})
                            // .text(&[" (",&audit.score().to_string(),"/",&audit.full().to_string(),") โดย "].concat())
                            .text(" audit โดย ")
                            .text(&audit.create_username.clone().unwrap_or(String::from("ไม่ระบุผู้ประเมิน")))
                            .text(" เมื่อ ")
                            .text(&datetime_th_opt_relative(&audit.create_datetime))
                            .apply(mixins::click_with_loader_checked(clone!(page, app, audit => move || {
                                Self::init_inner(audit.clone(), page.clone());
                                page.ipd_summary_audit_selected.set(Some(audit.clone()));
                            }), app.state()))
                        })
                    })))
                    .child_signal(page.summary_data.signal_ref(|opt| opt.is_some()).map(clone!(page, app => move |has_summary| {
                        if has_summary {
                            Some(html!("button" => HtmlButtonElement, {
                                .attr("type", "button")
                                .class(class::BTN_LT)
                                .class_signal("btn-primary", page.ipd_summary_audit_selected.signal_cloned().map(move |opt| opt.map(|selected| selected.summary_audit_id == 0).unwrap_or_default()))
                                .class_signal("btn-secondary", not(page.ipd_summary_audit_selected.signal_cloned().map(move |opt| opt.map(|selected| selected.summary_audit_id == 0).unwrap_or_default())))
                                .child(html!("i", {.class(class::FA_PLUS)}))
                                .text(" เพิ่ม")
                                .apply(mixins::click_with_loader_checked(clone!(app, page => move || {
                                    let audit_opt = SummaryAudit::new_from_parts(&page.patient.lock_ref().patient.get_cloned(), &page.summary_data.lock_ref(), page.new_items());

                                    if let Some(audit) = audit_opt {
                                        Self::init_inner(audit, page.clone());
                                    } else {
                                        app.alert("ไม่พบ Summary", "ไม่พบข้อมูลการสรุปเวชระเบียน");
                                    }
                                }), app.state()))
                            }))
                        } else {
                            Some(html!("div", {
                                .text("ไม่พบการสรุปเวชระเบียน")
                            }))
                        }
                    })))
                }),
                html!("div", {
                    .class(class::ROW_T)
                    .child(html!("div", {
                        .style("column-width","480px")
                        .style("column-gap","8px")
                        .children([
                            render_problem_list(page.problem_lists.clone()),
                            render_lab_alert(page.lab_alerts.clone(), page.patient.lock_ref().hn.clone(), page.lab_history_modal.clone(), app.clone()),
                            Self::render_dx_op(page.clone(), app.clone()),
                            Self::render_codes(page.clone(), app.clone()),
                            Self::render_his(page.clone(), app.clone()),
                            Self::render_auditor_note(page.clone(), app.clone()),
                            Self::render_summary_status(page.clone(), app.clone()),
                        ])
                    }))
                }),
            ])
            .child(html!("hr"))
            .child_signal(page.ipd_summary_audit_mutable.signal_cloned().map(move |opt| {
                opt.map(|audit_mutable| {
                    IpdSummaryAuditMutable::render(audit_mutable, page.list_loaded.clone(), app.clone())
                })
            }))
        })
    }

    fn render_his(page: Rc<Self>, app: Rc<App>) -> Dom {
        html!("div", {
            .class(class::CARD)
            .style("break-inside","avoid")
            .children([
                html!("div", {.class(class::CARD_HEAD_BDARKS_CYANS).class("fw-bold").text("Current ICD in Computer (HOSxP)")}),
                html!("div", {
                    .class("card-body")
                    .children([
                        html!("div", {
                            .child(ty_badge("PDx"))
                            .children_signal_vec(page.his_pdx.signal_vec_cloned().map(clone!(app, page => move |dx| {
                                html!("span", {
                                    .attr("id", &["pdx-his-", &ID_COUNTER.fetch_add(1, Ordering::SeqCst).to_string()].concat())
                                    .attr("draggable","true")
                                    .apply(mixins::drag_start_only(app.drag_start_state.clone()))
                                    .class(class::BOX_INLINE_ROUND)
                                    .class_signal("bg-danger-subtle", page.ipd_summary_audit_mutable.signal_cloned().map(clone!(dx => move |opt: Option<Rc<IpdSummaryAuditMutable>>| {
                                        option(opt.map(clone!(dx => move |audit| {
                                            audit.summary_audit_pdx.signal_vec_cloned().filter_signal_cloned(clone!(dx => move |item| {
                                                item.com_icd.signal_cloned().map(clone!(dx => move |s| dx.icd.as_ref().map(|icd| *icd == s).unwrap_or_default()))
                                            })).is_empty()
                                        })))
                                    })).flatten().map(|op: Option<bool>| op.unwrap_or(true)))
                                    .text(&dx.icd.clone().unwrap_or_default())
                                    .attr("title", &dx.detail.clone().unwrap_or_default())
                                })
                            })))
                        }),
                        html!("div", {
                            .child(ty_badge("SDx"))
                            .children_signal_vec(page.his_sdxs.signal_vec_cloned().map(clone!(app, page => move |dx| {
                                html!("span", {
                                    .attr("id", &["sdx-his-", &ID_COUNTER.fetch_add(1, Ordering::SeqCst).to_string()].concat())
                                    .attr("draggable","true")
                                    .apply(mixins::drag_start_only(app.drag_start_state.clone()))
                                    .class(class::BOX_INLINE_ROUND)
                                    .class_signal("bg-danger-subtle", page.ipd_summary_audit_mutable.signal_cloned().map(clone!(dx => move |opt: Option<Rc<IpdSummaryAuditMutable>>| {
                                        option(opt.map(clone!(dx => move |audit| {
                                            audit.summary_audit_sdxs.signal_vec_cloned().filter_signal_cloned(clone!(dx => move |item| {
                                                item.com_icd.signal_cloned().map(clone!(dx => move |s| dx.icd.as_ref().map(|icd| *icd == s).unwrap_or_default()))
                                            })).is_empty()
                                        })))
                                    })).flatten().map(|op: Option<bool>| op.unwrap_or(true)))
                                    .text(&dx.icd.clone().unwrap_or_default())
                                    .attr("title", &dx.detail.clone().unwrap_or_default())
                                })
                            })))
                        }),
                        html!("div", {
                            .child(ty_badge("ODx"))
                            .children_signal_vec(page.his_odxs.signal_vec_cloned().map(clone!(app, page => move |dx| {
                                html!("span", {
                                    .attr("id", &["odx-his-", &ID_COUNTER.fetch_add(1, Ordering::SeqCst).to_string()].concat())
                                    .attr("draggable","true")
                                    .apply(mixins::drag_start_only(app.drag_start_state.clone()))
                                    .class(class::BOX_INLINE_ROUND)
                                    .class_signal("bg-danger-subtle", page.ipd_summary_audit_mutable.signal_cloned().map(clone!(dx => move |opt: Option<Rc<IpdSummaryAuditMutable>>| {
                                        option(opt.map(clone!(dx => move |audit| {
                                            audit.summary_audit_odxs.signal_vec_cloned().filter_signal_cloned(clone!(dx => move |item| {
                                                item.com_icd.signal_cloned().map(clone!(dx => move |s| dx.icd.as_ref().map(|icd| *icd == s).unwrap_or_default()))
                                            })).is_empty()
                                        })))
                                    })).flatten().map(|op: Option<bool>| op.unwrap_or(true)))
                                    .text(&dx.icd.clone().unwrap_or_default())
                                    .attr("title", &dx.detail.clone().unwrap_or_default())
                                })
                            })))
                        }),
                        html!("div", {
                            .child(ty_badge("Op"))
                            .children_signal_vec(page.his_ops.signal_vec_cloned().map(clone!(app, page => move |dx| {
                                html!("span", {
                                    .attr("id", &["op-his-", &ID_COUNTER.fetch_add(1, Ordering::SeqCst).to_string()].concat())
                                    .attr("draggable","true")
                                    .apply(mixins::drag_start_only(app.drag_start_state.clone()))
                                    .class(class::BOX_INLINE_ROUND)
                                    .class_signal("bg-danger-subtle", page.ipd_summary_audit_mutable.signal_cloned().map(clone!(dx => move |opt: Option<Rc<IpdSummaryAuditMutable>>| {
                                        option(opt.map(clone!(dx => move |audit| {
                                            audit.summary_audit_ops.signal_vec_cloned().filter_signal_cloned(clone!(dx => move |item| {
                                                item.com_icd.signal_cloned().map(clone!(dx => move |s| dx.icd.as_ref().map(|icd| *icd == s).unwrap_or_default()))
                                            })).is_empty()
                                        })))
                                    })).flatten().map(|op: Option<bool>| op.unwrap_or(true)))
                                    .text(&dx.icd.clone().unwrap_or_default())
                                    .attr("title", &dx.detail.clone().unwrap_or_default())
                                })
                            })))
                        }),
                    ])
                    .child_signal(page.his_dagger_asterisk_pairs.signal_cloned().map(|pairs| {
                        (!pairs.is_empty()).then(|| {
                            html!("div", {
                                .child(html!("span", {
                                    .class(class::BADGE_RED75_L)
                                    .text("Dagger-Asterisk")
                                }))
                                .children(pairs.into_iter().map(|(opt, aster)| {
                                    dagger_aster_badges(opt, aster)
                                }))
                            })
                        })
                    }))
                }),
            ])
        })
    }

    fn render_dx_op(page: Rc<Self>, app: Rc<App>) -> Dom {
        html!("div", {
            .class(class::CARD)
            .style("break-inside","avoid")
            .children([
                html!("div", {.class(class::CARD_HEAD_BDARKS_GOLDS).class("fw-bold").text("Current Dx/Op in Summary (KPHIS)")}),
                html!("div", {
                    .class("card-body")
                    .children([
                        html!("div", {
                            .child(ty_badge("PDx"))
                            .children_signal_vec(page.doctor_pdx.signal_vec_cloned().map(clone!(app, page => move |dx| {
                                html!("span", {
                                    .attr("id", &["pdx-dx-", &ID_COUNTER.fetch_add(1, Ordering::SeqCst).to_string()].concat())
                                    .attr("draggable","true")
                                    .apply(mixins::drag_start_only(app.drag_start_state.clone()))
                                    .class(class::BOX_INLINE_ROUND)
                                    .class_signal("bg-danger-subtle", page.ipd_summary_audit_mutable.signal_cloned().map(clone!(dx => move |opt: Option<Rc<IpdSummaryAuditMutable>>| {
                                        option(opt.map(clone!(dx => move |audit| {
                                            audit.summary_audit_pdx.signal_vec_cloned().filter_signal_cloned(clone!(dx => move |item| {
                                                item.sum_dx.signal_cloned().map(clone!(dx => move |s| dx.detail.as_ref().map(|detail| *detail == s).unwrap_or_default()))
                                            })).is_empty()
                                        })))
                                    })).flatten().map(|op: Option<bool>| op.unwrap_or(true)))
                                    .text(&dx.detail.clone().unwrap_or_default())
                                })
                            })))
                        }),
                        html!("div", {
                            .child(ty_badge("SDx"))
                            .children_signal_vec(page.doctor_sdxs.signal_vec_cloned().map(clone!(app, page => move |dx| {
                                html!("span", {
                                    .attr("id", &["sdx-dx-", &ID_COUNTER.fetch_add(1, Ordering::SeqCst).to_string()].concat())
                                    .attr("draggable","true")
                                    .apply(mixins::drag_start_only(app.drag_start_state.clone()))
                                    .class(class::BOX_INLINE_ROUND)
                                    .class_signal("bg-danger-subtle", page.ipd_summary_audit_mutable.signal_cloned().map(clone!(dx => move |opt: Option<Rc<IpdSummaryAuditMutable>>| {
                                        option(opt.map(clone!(dx => move |audit| {
                                            audit.summary_audit_sdxs.signal_vec_cloned().filter_signal_cloned(clone!(dx => move |item| {
                                                item.sum_dx.signal_cloned().map(clone!(dx => move |s| dx.detail.as_ref().map(|detail| *detail == s).unwrap_or_default()))
                                            })).is_empty()
                                        })))
                                    })).flatten().map(|op: Option<bool>| op.unwrap_or(true)))
                                    .text(&dx.detail.clone().unwrap_or_default())
                                })
                            })))
                        }),
                        html!("div", {
                            .child(ty_badge("ODx"))
                            .children_signal_vec(page.doctor_odxs.signal_vec_cloned().map(clone!(app, page => move |dx| {
                                html!("span", {
                                    .attr("id", &["odx-dx-", &ID_COUNTER.fetch_add(1, Ordering::SeqCst).to_string()].concat())
                                    .attr("draggable","true")
                                    .apply(mixins::drag_start_only(app.drag_start_state.clone()))
                                    .class(class::BOX_INLINE_ROUND)
                                    .class_signal("bg-danger-subtle", page.ipd_summary_audit_mutable.signal_cloned().map(clone!(dx => move |opt: Option<Rc<IpdSummaryAuditMutable>>| {
                                        option(opt.map(clone!(dx => move |audit| {
                                            audit.summary_audit_odxs.signal_vec_cloned().filter_signal_cloned(clone!(dx => move |item| {
                                                item.sum_dx.signal_cloned().map(clone!(dx => move |s| dx.detail.as_ref().map(|detail| *detail == s).unwrap_or_default()))
                                            })).is_empty()
                                        })))
                                    })).flatten().map(|op: Option<bool>| op.unwrap_or(true)))
                                    .text(&dx.detail.clone().unwrap_or_default())
                                })
                            })))
                        }),
                        html!("div", {
                            .child(ty_badge("Op"))
                            .children_signal_vec(page.doctor_ops.signal_vec_cloned().map(clone!(app, page => move |dx| {
                                html!("span", {
                                    .attr("id", &["op-dx-", &ID_COUNTER.fetch_add(1, Ordering::SeqCst).to_string()].concat())
                                    .attr("draggable","true")
                                    .apply(mixins::drag_start_only(app.drag_start_state.clone()))
                                    .class(class::BOX_INLINE_ROUND)
                                    .class_signal("bg-danger-subtle", page.ipd_summary_audit_mutable.signal_cloned().map(clone!(dx => move |opt: Option<Rc<IpdSummaryAuditMutable>>| {
                                        option(opt.map(clone!(dx => move |audit| {
                                            audit.summary_audit_ops.signal_vec_cloned().filter_signal_cloned(clone!(dx => move |item| {
                                                item.sum_dx.signal_cloned().map(clone!(dx => move |s| dx.detail.as_ref().map(|detail| *detail == s).unwrap_or_default()))
                                            })).is_empty()
                                        })))
                                    })).flatten().map(|op: Option<bool>| op.unwrap_or(true)))
                                    .text(&dx.detail.clone().unwrap_or_default())
                                })
                            })))
                        }),
                    ])
                }),
            ])
        })
    }

    fn render_codes(page: Rc<Self>, app: Rc<App>) -> Dom {
        html!("div", {
            .class(class::CARD)
            .style("break-inside","avoid")
            .children([
                html!("div", {.class(class::CARD_HEAD_BDARKS_GREENS).class("fw-bold").text("Current ICD in Summary (KPHIS)")}),
                html!("div", {
                    .class("card-body")
                    .children([
                        html!("div", {
                            .child(ty_badge("PDx"))
                            .children_signal_vec(page.coder_pdxs.signal_vec_cloned().map(clone!(app, page => move |dx| {
                                html!("span", {
                                    .attr("id", &["pdx-sum-", &ID_COUNTER.fetch_add(1, Ordering::SeqCst).to_string()].concat())
                                    .attr("draggable","true")
                                    .apply(mixins::drag_start_only(app.drag_start_state.clone()))
                                    .class(class::BOX_INLINE_ROUND)
                                    .class_signal("bg-danger-subtle", page.ipd_summary_audit_mutable.signal_cloned().map(clone!(dx => move |opt: Option<Rc<IpdSummaryAuditMutable>>| {
                                        option(opt.map(clone!(dx => move |audit| {
                                            audit.summary_audit_pdx.signal_vec_cloned().filter_signal_cloned(clone!(dx => move |item| {
                                                item.sum_icd.signal_cloned().map(clone!(dx => move |s| dx.icd.as_ref().map(|icd| *icd == s).unwrap_or_default()))
                                            })).is_empty()
                                        })))
                                    })).flatten().map(|op: Option<bool>| op.unwrap_or(true)))
                                    .text(&dx.icd.clone().unwrap_or_default())
                                    .attr("title", &dx.detail.clone().unwrap_or_default())
                                })
                            })))
                        }),
                        html!("div", {
                            .child(ty_badge("SDx"))
                            .children_signal_vec(page.coder_sdxs.signal_vec_cloned().map(clone!(app, page => move |dx| {
                                html!("span", {
                                    .attr("id", &["sdx-sum-", &ID_COUNTER.fetch_add(1, Ordering::SeqCst).to_string()].concat())
                                    .attr("draggable","true")
                                    .apply(mixins::drag_start_only(app.drag_start_state.clone()))
                                    .class(class::BOX_INLINE_ROUND)
                                    .class_signal("bg-danger-subtle", page.ipd_summary_audit_mutable.signal_cloned().map(clone!(dx => move |opt: Option<Rc<IpdSummaryAuditMutable>>| {
                                        option(opt.map(clone!(dx => move |audit| {
                                            audit.summary_audit_sdxs.signal_vec_cloned().filter_signal_cloned(clone!(dx => move |item| {
                                                item.sum_icd.signal_cloned().map(clone!(dx => move |s| dx.icd.as_ref().map(|icd| *icd == s).unwrap_or_default()))
                                            })).is_empty()
                                        })))
                                    })).flatten().map(|op: Option<bool>| op.unwrap_or(true)))
                                    .text(&dx.icd.clone().unwrap_or_default())
                                    .attr("title", &dx.detail.clone().unwrap_or_default())
                                })
                            })))
                        }),
                        html!("div", {
                            .child(ty_badge("ODx"))
                            .children_signal_vec(page.coder_odxs.signal_vec_cloned().map(clone!(app, page => move |dx| {
                                html!("span", {
                                    .attr("id", &["odx-sum-", &ID_COUNTER.fetch_add(1, Ordering::SeqCst).to_string()].concat())
                                    .attr("draggable","true")
                                    .apply(mixins::drag_start_only(app.drag_start_state.clone()))
                                    .class(class::BOX_INLINE_ROUND)
                                    .class_signal("bg-danger-subtle", page.ipd_summary_audit_mutable.signal_cloned().map(clone!(dx => move |opt: Option<Rc<IpdSummaryAuditMutable>>| {
                                        option(opt.map(clone!(dx => move |audit| {
                                            audit.summary_audit_odxs.signal_vec_cloned().filter_signal_cloned(clone!(dx => move |item| {
                                                item.sum_icd.signal_cloned().map(clone!(dx => move |s| dx.icd.as_ref().map(|icd| *icd == s).unwrap_or_default()))
                                            })).is_empty()
                                        })))
                                    })).flatten().map(|op: Option<bool>| op.unwrap_or(true)))
                                    .text(&dx.icd.clone().unwrap_or_default())
                                    .attr("title", &dx.detail.clone().unwrap_or_default())
                                })
                            })))
                        }),
                        html!("div", {
                            .child(ty_badge("Op"))
                            .children_signal_vec(page.coder_ops.signal_vec_cloned().map(clone!(app, page => move |dx| {
                                html!("span", {
                                    .attr("id", &["op-sum-", &ID_COUNTER.fetch_add(1, Ordering::SeqCst).to_string()].concat())
                                    .attr("draggable","true")
                                    .apply(mixins::drag_start_only(app.drag_start_state.clone()))
                                    .class(class::BOX_INLINE_ROUND)
                                    .class_signal("bg-danger-subtle", page.ipd_summary_audit_mutable.signal_cloned().map(clone!(dx => move |opt: Option<Rc<IpdSummaryAuditMutable>>| {
                                        option(opt.map(clone!(dx => move |audit| {
                                            audit.summary_audit_ops.signal_vec_cloned().filter_signal_cloned(clone!(dx => move |item| {
                                                item.sum_icd.signal_cloned().map(clone!(dx => move |s| dx.icd.as_ref().map(|icd| *icd == s).unwrap_or_default()))
                                            })).is_empty()
                                        })))
                                    })).flatten().map(|op: Option<bool>| op.unwrap_or(true)))
                                    .text(&dx.icd.clone().unwrap_or_default())
                                    .attr("title", &dx.detail.clone().unwrap_or_default())
                                })
                            })))
                        }),
                    ])
                    .child_signal(page.coder_dagger_asterisk_pairs.signal_cloned().map(|pairs| {
                        (!pairs.is_empty()).then(|| {
                            html!("div", {
                                .child(html!("span", {
                                    .class(class::BADGE_RED75_L)
                                    .text("Dagger-Asterisk")
                                }))
                                .children(pairs.into_iter().map(|(opt, aster)| {
                                    dagger_aster_badges(opt, aster)
                                }))
                            })
                        })
                    }))
                }),
            ])
        })
    }

    fn render_auditor_note(page: Rc<Self>, app: Rc<App>) -> Dom {
        html!("div", {
            .class("mb-2")
            .style("break-inside","avoid")
            .child_signal(page.summary_id.signal_cloned().map(clone!(app, page => move |may_zero| {
                zero_none(may_zero).map(|summary_id| {
                    SummaryNoteCpn::render(false, SummaryNoteCpn::new(summary_id), app.clone())
                })
            })))
        })
    }

    fn render_summary_status(page: Rc<Self>, app: Rc<App>) -> Dom {
        html!("div", {
            .class(class::CARD)
            .style("break-inside","avoid")
            .children([
                html!("div", {.class(class::CARD_HEAD_BDARKS_LIGHTS).class("fw-bold").text("SUMMARY STATUS")}),
                html!("div", {
                    .class("card-body")
                    .child(html!("div", {
                        .class(class::INPUT_GROUP_SM_T)
                        .children([
                            doms::status_btn(AuditStatus::Review, page.status.clone(), page.status_changed.clone()),
                            doms::status_btn(AuditStatus::Code, page.status.clone(), page.status_changed.clone()),
                            doms::status_btn(AuditStatus::Audit, page.status.clone(), page.status_changed.clone()),
                            doms::status_btn(AuditStatus::Claim, page.status.clone(), page.status_changed.clone()),
                            doms::status_btn(AuditStatus::Appeal, page.status.clone(), page.status_changed.clone()),
                            doms::status_btn(AuditStatus::Done, page.status.clone(), page.status_changed.clone()),
                        ])
                    }))
                    .apply_if(app.endpoint_is_allow(&Method::PATCH, &EndPoint::IpdSummaryNoteId, false), |dom| dom
                        .child(html!("div", {
                            .class("text-end")
                            .child(html!("button" => HtmlButtonElement, {
                                .attr("type","button")
                                .class(class::BTN_SM_BLUE)
                                .child(html!("i", {.class(class::FA_SAVE)}))
                                .text(" บันทึก")
                                .apply(mixins::click_with_loader_checked_or_true_disable_signal(clone!(app, page => move || {
                                    Self::update_status(page.clone(), app.clone());
                                }), not(page.status_changed.signal()), app.state()))
                            }))
                        }))
                    )
                }),
            ])
        })
    }
}

#[derive(Clone, Default)]
struct IpdSummaryAuditMutable {
    new_audit: Mutable<Option<Rc<SummaryAudit>>>,
    ipd_summary_audit_selected: Mutable<Option<Rc<SummaryAudit>>>,

    // checker: Mutable<bool>,
    changed: Mutable<bool>,
    check_com_drg: Mutable<bool>,
    check_rev_drg: Mutable<bool>,

    // 0 is new
    summary_audit_id: Mutable<u32>,
    summary_id: Mutable<u32>,
    /// UC, OFC, LGO, SSS
    payer: Mutable<String>,
    /// - I : Internal
    /// - E : Extermal
    audit_type: Mutable<String>,
    /// - `N` : No authentication
    /// - `T` : Text Signature
    /// - `C` : Cursive Signature
    /// - `D` : Digital Signature
    doctor_auth: Mutable<String>,
    com_hn: Mutable<String>,
    com_an: Mutable<String>,
    com_adm_datetime: Mutable<String>,
    // not NULL
    com_dch_datetime: Mutable<String>,
    com_leaveday: Mutable<String>,
    /// 1 = Male, 2 = Female
    com_sex: Mutable<String>,
    com_birthday: Mutable<String>,
    /// in Grams
    com_bw: Mutable<String>,
    com_dchstts: Mutable<String>,
    com_dchtype: Mutable<String>,
    com_pid: Mutable<String>,
    com_drg: Mutable<String>,
    com_rw: Mutable<String>,
    com_adjrw: Mutable<String>,
    rev_hn: Mutable<String>,
    rev_an: Mutable<String>,
    rev_adm_datetime: Mutable<String>,
    rev_dch_datetime: Mutable<String>,
    rev_leaveday: Mutable<String>,
    /// 1 = Male, 2 = Female
    rev_sex: Mutable<String>,
    rev_birthday: Mutable<String>,
    /// in Grams
    rev_bw: Mutable<String>,
    rev_dchstts: Mutable<String>,
    rev_dchtype: Mutable<String>,
    rev_pid: Mutable<String>,
    rev_drg: Mutable<String>,
    rev_rw: Mutable<String>,
    rev_adjrw: Mutable<String>,
    sa: Mutable<String>,
    ca: Mutable<String>,

    create_username: Mutable<String>,
    create_datetime: Mutable<String>,
    update_datetime: Mutable<String>,

    summary_audit_pdx: MutableVec<Rc<SummaryAuditItemMutable>>,
    summary_audit_sdxs: MutableVec<Rc<SummaryAuditItemMutable>>,
    summary_audit_odxs: MutableVec<Rc<SummaryAuditItemMutable>>,
    summary_audit_ops: MutableVec<Rc<SummaryAuditItemMutable>>,

    com_drg_panel: Mutable<String>,
    rev_drg_panel: Mutable<String>,
}

impl IpdSummaryAuditMutable {
    fn new(parent_page: Rc<IpdSummaryAuditPage>) -> Rc<Self> {
        Rc::new(Self {
            ipd_summary_audit_selected: parent_page.ipd_summary_audit_selected.clone(),
            ..Default::default()
        })
    }

    fn adjrw_diff_signal(&self) -> impl Signal<Item = Option<f64>> + use<> {
        map_ref! {
            let com_adjrw = self.com_adjrw.signal_cloned(),
            let rev_adjrw = self.rev_adjrw.signal_cloned() =>
            if let (Ok(com), Ok(rev)) = (com_adjrw.parse::<f64>(), rev_adjrw.parse::<f64>()) {
                Some(rev - com)
            } else {
                None
            }
        }
    }

    fn cannot_save_signal(&self, app: Rc<App>) -> impl Signal<Item = bool> + use<> {
        map_ref! {
            let is_owner = self.is_creator_signal(app.clone()),
            let changed = self.changed.signal() =>
            !is_owner || !changed
        }
    }

    fn is_creator_signal(&self, app: Rc<App>) -> impl Signal<Item = bool> + use<> {
        self.create_username.signal_cloned().map(clone!(app => move |username| {
            str_some(username).and_then(|s| app.user_name().map(|uname| uname == s)).unwrap_or(true)
        }))
    }

    fn set_selected(&self) {
        if let Some(selected) = self.ipd_summary_audit_selected.get_cloned() {
            self.summary_audit_id.set(selected.summary_audit_id);
            self.summary_id.set(selected.summary_id);
            self.payer.set(selected.payer.clone().unwrap_or_default());
            self.audit_type.set(selected.audit_type.clone());
            self.doctor_auth.set(selected.doctor_auth.clone().unwrap_or_default());
            self.com_hn.set(selected.com_hn.clone().unwrap_or_default());
            self.com_an.set(selected.com_an.clone().unwrap_or_default());
            self.com_adm_datetime.set(selected.com_adm_datetime.map(|dt| dt.js_string()).unwrap_or_default());
            self.com_dch_datetime.set(selected.com_dch_datetime.map(|dt| dt.js_string()).unwrap_or_default());
            self.com_leaveday.set(selected.com_leaveday.map(|i| i.to_string()).unwrap_or_default());
            self.com_sex.set(selected.com_sex.clone().unwrap_or_default());
            self.com_birthday.set(selected.com_birthday.map(|d| d.to_string()).unwrap_or_default());
            self.com_bw.set(selected.com_bw.map(|i| i.to_string()).unwrap_or_default());
            self.com_dchstts.set(selected.com_dchstts.clone().unwrap_or_default());
            self.com_dchtype.set(selected.com_dchtype.clone().unwrap_or_default());
            self.com_pid.set(selected.com_pid.clone().unwrap_or_default());
            self.com_drg.set(selected.com_drg.clone().unwrap_or_default());
            self.com_rw.set(selected.com_rw.map(|f| f.to_string()).unwrap_or_default());
            self.com_adjrw.set(selected.com_adjrw.map(|f| f.to_string()).unwrap_or_default());
            self.rev_hn.set(selected.rev_hn.clone().unwrap_or_default());
            self.rev_an.set(selected.rev_an.clone().unwrap_or_default());
            self.rev_adm_datetime.set(selected.rev_adm_datetime.map(|dt| dt.js_string()).unwrap_or_default());
            self.rev_dch_datetime.set(selected.rev_dch_datetime.map(|dt| dt.js_string()).unwrap_or_default());
            self.rev_leaveday.set(selected.rev_leaveday.map(|i| i.to_string()).unwrap_or_default());
            self.rev_sex.set(selected.rev_sex.clone().unwrap_or_default());
            self.rev_birthday.set(selected.rev_birthday.map(|d| d.to_string()).unwrap_or_default());
            self.rev_bw.set(selected.rev_bw.map(|i| i.to_string()).unwrap_or_default());
            self.rev_dchstts.set(selected.rev_dchstts.clone().unwrap_or_default());
            self.rev_dchtype.set(selected.rev_dchtype.clone().unwrap_or_default());
            self.rev_pid.set(selected.rev_pid.clone().unwrap_or_default());
            self.rev_drg.set(selected.rev_drg.clone().unwrap_or_default());
            self.rev_rw.set(selected.rev_rw.map(|f| f.to_string()).unwrap_or_default());
            self.rev_adjrw.set(selected.rev_adjrw.map(|f| f.to_string()).unwrap_or_default());
            self.sa.set(selected.sa.clone());
            self.ca.set(selected.ca.clone());
            self.create_username.set(selected.create_username.clone().unwrap_or_default());
            self.create_datetime.set(selected.create_datetime.map(|dt| dt.js_string()).unwrap_or_default());
            self.update_datetime.set(selected.update_datetime.map(|dt| dt.js_string()).unwrap_or_default());
            {
                let mut pdx_lock = self.summary_audit_pdx.lock_mut();
                pdx_lock.clear();
                pdx_lock.extend(
                    selected
                        .summary_audit_items
                        .iter()
                        .filter_map(|item| item.ty.as_ref().map(|ty| ty == "PDx").unwrap_or_default().then(|| SummaryAuditItemMutable::new_from_audit_item(item))),
                );
            }
            {
                let mut sdxs_lock = self.summary_audit_sdxs.lock_mut();
                sdxs_lock.clear();
                sdxs_lock.extend(
                    selected
                        .summary_audit_items
                        .iter()
                        .filter_map(|item| item.ty.as_ref().map(|ty| ty == "SDx").unwrap_or_default().then(|| SummaryAuditItemMutable::new_from_audit_item(item))),
                );
            }
            {
                let mut odxs_lock = self.summary_audit_odxs.lock_mut();
                odxs_lock.clear();
                odxs_lock.extend(
                    selected
                        .summary_audit_items
                        .iter()
                        .filter_map(|item| item.ty.as_ref().map(|ty| ty == "ODx").unwrap_or_default().then(|| SummaryAuditItemMutable::new_from_audit_item(item))),
                );
            }
            {
                let mut ops_lock = self.summary_audit_ops.lock_mut();
                ops_lock.clear();
                ops_lock.extend(
                    selected
                        .summary_audit_items
                        .iter()
                        .filter_map(|item| item.ty.as_ref().map(|ty| ty == "Op").unwrap_or_default().then(|| SummaryAuditItemMutable::new_from_audit_item(item))),
                );
            }

            self.check_com_drg.set_neq(selected.com_drg.is_some());
            self.check_rev_drg.set_neq(selected.rev_drg.is_some());

            self.new_audit.set(Some(selected));
            self.changed.set(false);
        }
    }

    fn to_audit(&self) -> SummaryAudit {
        let summary_audit_items = self
            .summary_audit_pdx
            .lock_ref()
            .iter()
            .map(|item| item.to_audit_item())
            .chain(self.summary_audit_sdxs.lock_ref().iter().map(|item| item.to_audit_item()))
            .chain(self.summary_audit_odxs.lock_ref().iter().map(|item| item.to_audit_item()))
            .chain(self.summary_audit_ops.lock_ref().iter().map(|item| item.to_audit_item()))
            .collect::<Vec<SummaryAuditItem>>();
        SummaryAudit {
            summary_audit_id: self.summary_audit_id.get(),
            summary_id: self.summary_id.get(),
            payer: str_some(self.payer.get_cloned()),
            audit_type: self.audit_type.get_cloned(),
            doctor_auth: str_some(self.doctor_auth.get_cloned()),
            com_hn: str_some(self.com_hn.get_cloned()),
            com_an: str_some(self.com_an.get_cloned()),
            com_adm_datetime: datetime_8601(&self.com_adm_datetime.get_cloned()),
            com_dch_datetime: datetime_8601(&self.com_dch_datetime.get_cloned()),
            com_leaveday: self.com_leaveday.get_cloned().parse::<i32>().ok(),
            com_sex: str_some(self.com_sex.get_cloned()),
            com_birthday: date_8601(&self.com_birthday.get_cloned()),
            com_bw: self.com_bw.get_cloned().parse::<i32>().ok(),
            com_dchstts: str_some(self.com_dchstts.get_cloned()),
            com_dchtype: str_some(self.com_dchtype.get_cloned()),
            com_pid: str_some(self.com_pid.get_cloned()),
            com_drg: str_some(self.com_drg.get_cloned()),
            com_rw: self.com_rw.get_cloned().parse::<f64>().ok(),
            com_adjrw: self.com_adjrw.get_cloned().parse::<f64>().ok(),
            rev_hn: str_some(self.rev_hn.get_cloned()),
            rev_an: str_some(self.rev_an.get_cloned()),
            rev_adm_datetime: datetime_8601(&self.rev_adm_datetime.get_cloned()),
            rev_dch_datetime: datetime_8601(&self.rev_dch_datetime.get_cloned()),
            rev_leaveday: self.rev_leaveday.get_cloned().parse::<i32>().ok(),
            rev_sex: str_some(self.rev_sex.get_cloned()),
            rev_birthday: date_8601(&self.rev_birthday.get_cloned()),
            rev_bw: self.rev_bw.get_cloned().parse::<i32>().ok(),
            rev_dchstts: str_some(self.rev_dchstts.get_cloned()),
            rev_dchtype: str_some(self.rev_dchtype.get_cloned()),
            rev_pid: str_some(self.rev_pid.get_cloned()),
            rev_drg: str_some(self.rev_drg.get_cloned()),
            rev_rw: self.rev_rw.get_cloned().parse::<f64>().ok(),
            rev_adjrw: self.rev_adjrw.get_cloned().parse::<f64>().ok(),
            sa: self.sa.get_cloned(),
            ca: self.ca.get_cloned(),
            create_username: str_some(self.create_username.get_cloned()),
            create_datetime: datetime_8601(&self.create_datetime.lock_ref()),
            update_datetime: datetime_8601(&self.update_datetime.lock_ref()),
            summary_audit_items,
        }
    }

    fn save(audit: Rc<Self>, list_loaded: Mutable<bool>, app: Rc<App>) {
        let saver = audit.to_audit();

        app.async_load(
            true,
            clone!(app => async move {
                // POST `EndPoint::SummaryAudit`
                // PUT `EndPoint::SummaryAudit`
                match saver.call_api_save(app.state()).await {
                    Ok((_, responses)) => {
                        app.alert_execute_responses(&responses, async move {
                            list_loaded.set(false);
                        }).await;
                    }
                    Err(e) => {
                        app.alert_app_error(&e).await;
                    }
                }
            }),
        );
    }

    fn delete(audit: Rc<Self>, list_loaded: Mutable<bool>, app: Rc<App>) {
        if let Some(summary_audit_id) = zero_none(audit.summary_audit_id.get()) {
            app.async_load(
                true,
                clone!(app => async move {
                    if app.confirm("ยืนยันลบรายการ").await {
                        let params = SummaryAuditParams { summary_audit_id: Some(summary_audit_id), ..Default::default()};
                        // DELETE `EndPoint::SummaryAudit`
                        match SummaryAudit::call_api_delete(&params, app.state()).await {
                            Ok(response) => {
                                app.alert_execute_response(&response, async move {
                                    list_loaded.set(false);
                                }).await;
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

    fn check_drg(is_rev: bool, audit: Rc<Self>, app: Rc<App>) {
        app.async_load(
            true,
            clone!(app, audit => async move {

                let input_build = if is_rev {
                    let pdx = audit.summary_audit_pdx.lock_ref().first().map(|i| i.rev_icd.lock_ref().trim().replace('.',"")).unwrap_or_default();
                    let sdxs = audit.summary_audit_sdxs.lock_ref().iter().filter_map(|i| str_some(i.rev_icd.lock_ref().trim().replace('.',"")))
                        .chain(audit.summary_audit_odxs.lock_ref().iter().filter_map(|i| str_some(i.rev_icd.lock_ref().trim().replace('.',""))))
                        .collect::<Vec<String>>();
                    let procs = audit.summary_audit_ops.lock_ref().iter().filter_map(|i| str_some(i.rev_icd.lock_ref().trim().replace('.',""))).collect::<Vec<String>>();

                    let gender = str_some(audit.rev_sex.get_cloned());
                    let dob = str_some(audit.rev_birthday.get_cloned()).and_then(|s| date_8601(&s)).map(|d| PrimitiveDateTime::new(d, Time::MIDNIGHT));
                    let adm_wt = audit.rev_bw.get_cloned().parse::<u16>().ok();
                    let adm_date = datetime_8601(&audit.rev_adm_datetime.lock_ref());
                    let dch_date = datetime_8601(&audit.rev_dch_datetime.lock_ref());
                    let dch_type = audit.rev_dchtype.get_cloned();
                    let leave_day = audit.rev_leaveday.get_cloned().parse::<u32>().unwrap_or_default();

                    GrouperInput::new(&pdx, &sdxs, &procs, &gender, dob, adm_wt, adm_date, dch_date, &dch_type, leave_day)
                } else {
                    let pdx = audit.summary_audit_pdx.lock_ref().first().map(|i| i.com_icd.lock_ref().trim().replace('.',"")).unwrap_or_default();
                    let sdxs = audit.summary_audit_sdxs.lock_ref().iter().filter_map(|i| str_some(i.com_icd.lock_ref().trim().replace('.',"")))
                        .chain(audit.summary_audit_odxs.lock_ref().iter().filter_map(|i| str_some(i.com_icd.lock_ref().trim().replace('.',""))))
                        .collect::<Vec<String>>();
                    let procs = audit.summary_audit_ops.lock_ref().iter().filter_map(|i| str_some(i.com_icd.lock_ref().trim().replace('.',""))).collect::<Vec<String>>();

                    let gender = str_some(audit.com_sex.get_cloned());
                    let dob = str_some(audit.com_birthday.get_cloned()).and_then(|s| date_8601(&s)).map(|d| PrimitiveDateTime::new(d, Time::MIDNIGHT));
                    let adm_wt = audit.com_bw.get_cloned().parse::<u16>().ok();
                    let adm_date = datetime_8601(&audit.com_adm_datetime.lock_ref());
                    let dch_date = datetime_8601(&audit.com_dch_datetime.lock_ref());
                    let dch_type = audit.com_dchtype.get_cloned();
                    let leave_day = audit.com_leaveday.get_cloned().parse::<u32>().unwrap_or_default();

                    GrouperInput::new(&pdx, &sdxs, &procs, &gender, dob, adm_wt, adm_date, dch_date, &dch_type, leave_day)
                };

                match input_build {
                    Ok(input) => {
                        if let Ok(input_json) = serde_json::to_string(&input) {
                            let output_json = app.drg_worker().await.run(input_json).await;
                            if let Ok(output) = serde_json::from_str::<GrouperOutput>(&output_json) {
                                if output.errors.is_empty() {
                                    let results = output.drg.iter().map(|drg| {
                                        let adj_rw = f32_rescale(drg.adj_rw(), 5);
                                        let drg_data = &drg.drg;
                                        let input_data = &drg.source;
                                        let los_min = if drg_data.wtlos > 3.0 {f32_rescale(drg_data.wtlos / 3.0, 2)} else {1.0};
                                        let sdxs = input_data.sdxs.clone().into_iter().collect::<Vec<String>>().join(", ");
                                        let procs = input_data.procs.clone().into_iter().collect::<Vec<String>>().join(", ");
                                        ([
                                            "    - AdjRW: ", &adj_rw.to_string(),
                                            "\n    - DRG: ", &drg_data.drg, " ", &drg_data.detail,
                                            "\n    - RW: ", &drg_data.rw.to_string(),
                                            "\n    - WtLOS: ", &drg_data.wtlos.to_string(),
                                            "\n    - OT: ", &drg_data.ot.to_string(),
                                            "\n    - PDx: ", &input_data.pdx,
                                            "\n    - SDx: ", &if sdxs.is_empty() {String::from("ไม่มี")} else {sdxs},
                                            "\n    - Proc: ", &if procs.is_empty() {String::from("ไม่มี")} else {procs},
                                            "\n    - LOS: ", &input_data.los.to_string(), " วัน",
                                            "\n    - วันนอนขั้นต่ำ: ", &los_min.to_string(), " วัน (AdjRw ", &f32_rescale(drg_data.adj_rw(los_f32_to_u32(los_min)), 5).to_string(), ")",
                                            "\n    - วันนอนขั้นสูง 1: ", &drg_data.ot.to_string(), " วัน (AdjRw ", &f32_rescale(drg_data.adj_rw(los_f32_to_u32(drg_data.ot)), 5).to_string(), ")",
                                            "\n    - วันนอนขั้นสูง 2: ", &(drg_data.ot * 2.0).to_string(), " วัน (AdjRw ", &f32_rescale(drg_data.adj_rw(los_f32_to_u32(drg_data.ot * 2.0)), 5).to_string(),")",
                                            "\n    - วันนอนขั้นสูง 3: ", &(drg_data.ot * 3.0).to_string(), " วัน (AdjRw ", &f32_rescale(drg_data.adj_rw(los_f32_to_u32(drg_data.ot * 3.0)), 5).to_string(), ")",
                                        ].concat(), drg_data.drg.clone(), adj_rw.to_string(), adj_rw.to_string())
                                    }).collect::<Vec<(String, String, String, String)>>();
                                    if let Some((_, drg, rw, adjrw)) = results.first() {
                                        if is_rev {
                                            audit.rev_drg.set(drg.to_owned());
                                            audit.rev_rw.set(rw.to_owned());
                                            audit.rev_adjrw.set(adjrw.to_owned());
                                        } else {
                                            audit.com_drg.set(drg.to_owned());
                                            audit.com_rw.set(rw.to_owned());
                                            audit.com_adjrw.set(adjrw.to_owned());
                                        }
                                    }
                                    let drgs_report = results.into_iter().map(|triplet| triplet.0).collect::<Vec<String>>().join("\n");
                                    if is_rev {
                                        audit.rev_drg_panel.set(drgs_report);
                                    } else {
                                        audit.com_drg_panel.set(drgs_report);
                                    }
                                } else {
                                    app.alert_with_close("ผลการตรวจสอบ", &output.errors.iter().map(|e| e.string()).collect::<Vec<String>>().join("\n"), true).await;
                                }
                            }
                        }
                    }
                    Err(errors) => {
                        app.alert_with_close("ผลการตรวจสอบ", &errors.iter().map(|e| e.string()).collect::<Vec<String>>().join("\n"), true).await;
                    }
                }
            }),
        )
    }

    fn render(audit: Rc<Self>, list_loaded: Mutable<bool>, app: Rc<App>) -> Dom {
        html!("div", {
            .future(map_ref!(
                let busy = app.loader_is_loading(),
                let check = audit.check_com_drg.signal() =>
                !busy && *check
            ).for_each(clone!(app, audit => move |ready| {
                if ready {
                    Self::check_drg(false, audit.clone(), app.clone());
                    audit.check_com_drg.set_neq(false);
                }
                async {}
            })))
            .future(map_ref!(
                let busy = app.loader_is_loading(),
                let check = audit.check_rev_drg.signal() =>
                !busy && *check
            ).for_each(clone!(app, audit => move |ready| {
                if ready {
                    Self::check_drg(true, audit.clone(), app.clone());
                    audit.check_rev_drg.set_neq(false);
                }
                async {}
            })))
            .style("width","calc(100% - 5px)")
            .children([
                // HEADER
                html!("div", {
                    .style("justify-items","center")
                    .children([
                        html!("div", {
                            .class(class::ROW_AUTO_SM_G2_JCT)
                            .children([
                                html!("div", {
                                    .class("col-auto")
                                    .child(html!("div", {
                                        .class(class::INPUT_GROUP_SM_T)
                                        .children([
                                            html!("span", {
                                                .class("input-group-text")
                                                .text("Audit type")
                                            }),
                                                html!("button", {
                                                .class(class::BTN_SM_BLUEO)
                                                .class_signal("active", audit.audit_type.signal_cloned().map(move |t| t == "I"))
                                                .attr("type", "button")
                                                .attr("data-bs-toggle", "button")
                                                .text("Internal")
                                                .event(clone!(audit => move |_: events::Click| {
                                                    audit.audit_type.set(String::from("I"));
                                                    audit.changed.set(true);
                                                }))
                                            }),
                                            html!("button", {
                                                .class(class::BTN_SM_BLUEO)
                                                .class_signal("active", audit.audit_type.signal_cloned().map(move |t| t == "E"))
                                                .attr("type", "button")
                                                .attr("data-bs-toggle", "button")
                                                .text("External")
                                                .event(clone!(audit => move |_: events::Click| {
                                                    audit.audit_type.set(String::from("E"));
                                                    audit.changed.set(true);
                                                }))
                                            }),
                                        ])
                                    }))
                                }),
                                html!("div", {
                                    .class("col-auto")
                                    .child(html!("div", {
                                        .class(class::INPUT_GROUP_SM_T)
                                        .child(html!("span", {
                                            .class("input-group-text")
                                            .text("Payer type")
                                        }))
                                        .children(["UC","OFC","LGO","SSS"].into_iter().map(clone!(audit => move |payer| {
                                            html!("button", {
                                                .class(class::BTN_SM_BLUEO)
                                                .class_signal("active", audit.payer.signal_ref(move |t| t == payer))
                                                .attr("type", "button")
                                                .attr("data-bs-toggle", "button")
                                                .text(payer)
                                                .event(clone!(audit => move |_: events::Click| {
                                                    audit.payer.set(payer.to_string());
                                                    audit.changed.set(true);
                                                }))
                                            })
                                        })))
                                    }))
                                }),
                                html!("div", {
                                    .class("col-auto")
                                    .child(html!("div", {
                                        .class(class::INPUT_GROUP_SM_T)
                                        .children([
                                            html!("span", {
                                                .class("input-group-text")
                                                .text("Physician's authentication")
                                            }),
                                                html!("button", {
                                                .class(class::BTN_SM_BLUEO)
                                                .class_signal("active", audit.doctor_auth.signal_cloned().map(move |t| t == "N"))
                                                .attr("type", "button")
                                                .attr("data-bs-toggle", "button")
                                                .text("No")
                                                .event(clone!(audit => move |_: events::Click| {
                                                    audit.doctor_auth.set(String::from("N"));
                                                    audit.changed.set(true);
                                                }))
                                            }),
                                            html!("button", {
                                                .class(class::BTN_SM_BLUEO)
                                                .class_signal("active", audit.doctor_auth.signal_cloned().map(move |t| t == "T"))
                                                .attr("type", "button")
                                                .attr("data-bs-toggle", "button")
                                                .text("Text")
                                                .event(clone!(audit => move |_: events::Click| {
                                                    audit.doctor_auth.set(String::from("T"));
                                                    audit.changed.set(true);
                                                }))
                                            }),
                                            html!("button", {
                                                .class(class::BTN_SM_BLUEO)
                                                .class_signal("active", audit.doctor_auth.signal_cloned().map(move |t| t == "C"))
                                                .attr("type", "button")
                                                .attr("data-bs-toggle", "button")
                                                .text("Cursive")
                                                .event(clone!(audit => move |_: events::Click| {
                                                    audit.doctor_auth.set(String::from("C"));
                                                    audit.changed.set(true);
                                                }))
                                            }),
                                            html!("button", {
                                                .class(class::BTN_SM_BLUEO)
                                                .class_signal("active", audit.doctor_auth.signal_cloned().map(move |t| t == "D"))
                                                .attr("type", "button")
                                                .attr("data-bs-toggle", "button")
                                                .text("Digital")
                                                .event(clone!(audit => move |_: events::Click| {
                                                    audit.doctor_auth.set(String::from("D"));
                                                    audit.changed.set(true);
                                                }))
                                            }),
                                        ])
                                    }))
                                }),
                                html!("div", {
                                    .class("col-auto")
                                    .child(html!("div", {
                                        .class(class::INPUT_GROUP_SM_T)
                                        .style("max-width","480px")
                                        .children([
                                            html!("span", {.class("input-group-text").text("SA")}),
                                            select_sa("", audit.sa.clone(), audit.changed.clone()),
                                            html!("span", {.class("input-group-text").text("CA")}),
                                            select_ca("", audit.ca.clone(), audit.changed.clone()),
                                        ])
                                    }))
                                }),
                            ])
                        }),
                    ])
                }),
                // FORM
                html!("div", {
                    .class(class::ROW_T)
                    .child(html!("div", {
                        .style("column-width","600px")
                        .style("column-gap","8px")
                        .children([
                            Self::render_search(app.clone()),
                            Self::render_info(audit.clone(), app.clone()),
                            Self::render_items(audit.clone(), app.clone()),
                            Self::render_com_drg(audit.clone()),
                            Self::render_rev_drg(audit.clone()),
                        ])
                    }))
                }),
                // FOOTER
                html!("div", {
                    .class(class::FLEX_JCR)
                    .child(html!("button", {
                        .attr("type", "button")
                        .class(class::BTN_L_BLUE)
                        .child(html!("i", {.class(class::FA_L_ARROW)}))
                        .text(" กลับ")
                        .event(clone!(app => move |_: events::Click| {
                            if app.go_back_else() {
                                for route in [
                                    Route::IpdPostAdmitList { view_by: String::from("doctor") },
                                    Route::IpdPostAdmitList { view_by: String::from("nurse") },
                                    Route::IpdPostAdmitList { view_by: String::from("pharmacist") },
                                    Route::IpdPostAdmitList { view_by: String::from("other") },
                                    Route::Info,
                                ] {
                                    if route.has_permission(app.state()) {
                                        route.hard_redirect();
                                        break;
                                    }
                                }
                            }
                        }))
                    }))
                    .child_signal(audit.summary_audit_id.signal_cloned().map(clone!(app, audit, list_loaded => move |summary_audit_id| {
                        let is_pre_admit = app.is_pre_admit(&audit.com_an.lock_ref());
                        app.endpoint_is_allow(&Method::POST, &EndPoint::IpdSummaryAudit, is_pre_admit).then(|| {
                            html!("button" => HtmlButtonElement, {
                                .attr("type", "button")
                                .class(class::BTN_L)
                                .class_signal("btn-primary", audit.changed.signal())
                                .class_signal("btn-secondary", not(audit.changed.signal()))
                                .text(if summary_audit_id > 0 { "แก้ไข" } else { "บันทึก" })
                                .apply(mixins::click_with_loader_checked_or_true_disable_signal(clone!(app, audit, list_loaded => move || {
                                    Self::save(audit.clone(), list_loaded.clone(), app.clone());
                                }), audit.cannot_save_signal(app.clone()), app.state()))
                            })
                        })
                    })))
                    .child_signal(audit.changed.signal().map(clone!(audit => move |changed| {
                        changed.then(|| {
                            html!("button", {
                                .attr("type", "button")
                                .class(class::BTN_GRAY)
                                .text("ยกเลิก")
                                .event(clone!(audit => move |_:events::Click| {
                                    audit.set_selected();
                                }))
                            })
                        })
                    })))
                    .children(PdfButtons::buttons(
                        PdfButtons::new(
                            TypstReport::from_system_with_coercion(SystemReport::IpdSummaryAudit, &app.state().report_coercions()),
                            audit.com_an.clone(),
                            Mutable::new(true),
                            audit.changed.clone(),
                            clone!(audit => move || {serde_json::json!({
                                "id": audit.com_an.lock_ref().as_str(),
                                "audit": [audit.to_audit()],
                            }).to_string()})
                        ), "PDF", Some("PDF (All)"), app.clone()
                    ))
                    .apply_if(app.endpoint_is_allow(&Method::DELETE, &EndPoint::IpdSummaryAudit, app.is_pre_admit(&audit.com_an.lock_ref())), |dom| dom
                        .child_signal(audit.summary_audit_id.signal_cloned().map(clone!(audit, list_loaded => move |summary_audit_id| {
                            (summary_audit_id > 0).then(|| {
                                html!("button" => HtmlButtonElement, {
                                    .attr("type", "button")
                                    .class(class::BTN_R_RED)
                                    .child(html!("i", {.class(class::FA_TRASH)}))
                                    .text(" ลบ")
                                    .apply(mixins::click_with_loader_checked_or_true_disable_signal(clone!(app, audit, list_loaded => move || {
                                        Self::delete(audit.clone(), list_loaded.clone(), app.clone());
                                    }), audit.cannot_save_signal(app.clone()), app.state()))
                                })
                            })
                        })))
                    )
                }),
            ])
        })
    }

    fn render_search(app: Rc<App>) -> Dom {
        html!("div", {
            .class(class::BOX_ROUND_T)
            .style("break-inside","avoid")
            .children([
                html!("a", {
                    .class(class::BTN_SM_FR_T_BLUEO)
                    .attr("href", "https://icd.who.int/browse10/2016/en")
                    .attr("rel","noopener noreferrer")
                    .attr("target","_blank")
                    .child(html!("i", {.class(class::FA_EXT_LINK)}))
                    .text(" ICD-10-WHO")
                }),
                html!("div", {.class(class::BOLD_M2).text("ค้นหา ICD10")}),
                DxSearchboxCpn::render(
                    DxSearchboxCpn::new(false, false, Mutable::new(None), DaggerAsteriskState::new()),
                    app.clone(),
                    None,
                    Mutable::new(false),
                    true,
                ),
                html!("div", {.class(class::BOLD_M2).text("ค้นหา ICD9CM")}),
                ProcSearchboxCpn::render(
                    ProcSearchboxCpn::new(Mutable::new(None)),
                    app.clone(),
                    Mutable::new(false),
                ),
            ])
        })
    }

    fn render_info(audit: Rc<Self>, app: Rc<App>) -> Dom {
        html!("div", {
            .class(class::BOX_ROUND_T)
            .style("break-inside","avoid")
            .children([
                html!("button", {
                    .attr("type","button")
                    .class(class::BTN_SM_FR_T_BLUEO)
                    .child(html!("i", {.class(class::FA_CHECK_CIRCLE_GREEN)}))
                    .text(" ALL")
                    .event(clone!(audit => move |_:events::Click| {
                        audit.rev_hn.set(audit.com_hn.get_cloned());
                        audit.rev_an.set(audit.com_an.get_cloned());
                        audit.rev_adm_datetime.set(audit.com_adm_datetime.get_cloned());
                        audit.rev_dch_datetime.set(audit.com_dch_datetime.get_cloned());
                        audit.rev_leaveday.set(audit.com_leaveday.get_cloned());
                        audit.rev_sex.set(audit.com_sex.get_cloned());
                        audit.rev_birthday.set(audit.com_birthday.get_cloned());
                        audit.rev_bw.set(audit.com_bw.get_cloned());
                        audit.rev_dchstts.set(audit.com_dchstts.get_cloned());
                        audit.rev_dchtype.set(audit.com_dchtype.get_cloned());
                        audit.rev_pid.set(audit.com_pid.get_cloned());
                        audit.changed.set_neq(true);
                    }))
                }),
                html!("div", {.class(class::BOLD_M2).text("ประเมินการบันทึกข้อมูล")}),
                html!("div", {
                    .child(doms::table_responsive(class::TABLE, clone!(app, audit => move |table| { table
                        .children([
                            html!("thead", {
                                .class("text-center")
                                .child(html!("tr", {
                                    .children([
                                        html!("th", {.attr("scope", "col").text("รายการ").style("min-width","120px")}),
                                        html!("th", {.attr("scope", "col").text("In Computer")}),
                                        html!("th", {.attr("scope", "col").text("From Review")}),
                                        html!("th", {.attr("scope", "col").text("Correct")}),
                                    ])
                                }))
                            }),
                            html!("tbody", {
                                .children([
                                    html!("tr", {
                                        .children([
                                            html!("td", {.text("HN").style("padding","6px 10px")}),
                                            td_input_text(audit.com_hn.clone(), audit.changed.clone(),"9"),
                                            td_input_text(audit.rev_hn.clone(), audit.changed.clone(),"9"),
                                            td_correct_toggle(audit.com_hn.clone(), audit.rev_hn.clone(), audit.changed.clone()),
                                        ])
                                    }),
                                    html!("tr", {
                                        .children([
                                            html!("td", {.text("AN").style("padding","6px 10px")}),
                                            td_input_text(audit.com_an.clone(), audit.changed.clone(),"13"),
                                            td_input_text(audit.rev_an.clone(), audit.changed.clone(),"13"),
                                            td_correct_toggle(audit.com_an.clone(), audit.rev_an.clone(), audit.changed.clone()),
                                        ])
                                    }),
                                    html!("tr", {
                                        .children([
                                            html!("td", {.text("ADM DATE").style("padding","6px 10px")}),
                                            td_input_datetime(audit.com_adm_datetime.clone(), audit.changed.clone()),
                                            td_input_datetime(audit.rev_adm_datetime.clone(), audit.changed.clone()),
                                            td_correct_toggle(audit.com_adm_datetime.clone(), audit.rev_adm_datetime.clone(), audit.changed.clone()),
                                        ])
                                    }),
                                    html!("tr", {
                                        .children([
                                            html!("td", {.text("DCH DATE").style("padding","6px 10px")}),
                                            td_input_datetime(audit.com_dch_datetime.clone(), audit.changed.clone()),
                                            td_input_datetime(audit.rev_dch_datetime.clone(), audit.changed.clone()),
                                            td_correct_toggle(audit.com_dch_datetime.clone(), audit.rev_dch_datetime.clone(), audit.changed.clone()),
                                        ])
                                    }),
                                    html!("tr", {
                                        .children([
                                            html!("td", {.text("LEAVE DAYS").style("padding","6px 10px")}),
                                            td_input_number(audit.com_leaveday.clone(), audit.changed.clone(), "1"),
                                            td_input_number(audit.rev_leaveday.clone(), audit.changed.clone(), "1"),
                                            td_correct_toggle(audit.com_leaveday.clone(), audit.rev_leaveday.clone(), audit.changed.clone()),
                                        ])
                                    }),
                                    html!("tr", {
                                        .children([
                                            html!("td", {.text("SEX").style("padding","6px 10px")}),
                                            td_input_select(&SEX_ITEMS, audit.com_sex.clone(), audit.changed.clone()),
                                            td_input_select(&SEX_ITEMS, audit.rev_sex.clone(), audit.changed.clone()),
                                            td_correct_toggle(audit.com_sex.clone(), audit.rev_sex.clone(), audit.changed.clone()),
                                        ])
                                    }),
                                    html!("tr", {
                                        .children([
                                            html!("td", {.text("DOB").style("padding","6px 10px")}),
                                            td_input_date(audit.com_birthday.clone(), audit.changed.clone()),
                                            td_input_date(audit.rev_birthday.clone(), audit.changed.clone()),
                                            td_correct_toggle(audit.com_birthday.clone(), audit.rev_birthday.clone(), audit.changed.clone()),
                                        ])
                                    }),
                                    html!("tr", {
                                        .children([
                                            html!("td", {.text("ADM WT(KG)").style("padding","6px 10px")}),
                                            td_input_kg_to_gram(audit.com_bw.clone(), audit.changed.clone()),
                                            td_input_kg_to_gram(audit.rev_bw.clone(), audit.changed.clone()),
                                            td_correct_toggle(audit.com_bw.clone(), audit.rev_bw.clone(), audit.changed.clone()),
                                        ])
                                    }),
                                    html!("tr", {
                                        .children([
                                            html!("td", {.text("DCH STATUS").style("padding","6px 10px")}),
                                            td_input_select(&DCH_STTS_ITEMS, audit.com_dchstts.clone(), audit.changed.clone()),
                                            td_input_select(&DCH_STTS_ITEMS, audit.rev_dchstts.clone(), audit.changed.clone()),
                                            td_correct_toggle(audit.com_dchstts.clone(), audit.rev_dchstts.clone(), audit.changed.clone()),
                                        ])
                                    }),
                                    html!("tr", {
                                        .children([
                                            html!("td", {.text("DCH TYPE").style("padding","6px 10px")}),
                                            td_input_select(&DCH_TYPE_ITEMS, audit.com_dchtype.clone(), audit.changed.clone()),
                                            td_input_select(&DCH_TYPE_ITEMS, audit.rev_dchtype.clone(), audit.changed.clone()),
                                            td_correct_toggle(audit.com_dchtype.clone(), audit.rev_dchtype.clone(), audit.changed.clone()),
                                        ])
                                    }),
                                    html!("tr", {
                                        .children([
                                            html!("td", {.text("PID").style("padding","6px 10px")}),
                                            td_input_text(audit.com_pid.clone(), audit.changed.clone(),"13"),
                                            td_input_text(audit.rev_pid.clone(), audit.changed.clone(),"13"),
                                            td_correct_toggle(audit.com_pid.clone(), audit.rev_pid.clone(), audit.changed.clone()),
                                        ])
                                    }),
                                ])
                            }),
                        ])
                    })))
                }),
            ])
        })
    }

    fn render_items(audit: Rc<Self>, app: Rc<App>) -> Dom {
        html!("div", {
            .class(class::BOX_ROUND_T)
            .style("break-inside","avoid")
            .child(html!("div", {
                .class("text-center")
                .child_signal(audit.summary_audit_pdx.signal_vec_cloned().is_empty().map(clone!(audit => move |no_pdx| {
                    no_pdx.then(|| {
                        html!("button", {
                            .attr("type","button")
                            .class(class::BTN_SM_L_BLUE)
                            .child(html!("i", {.class(class::FA_PLUS_L)}))
                            .text("เพิ่ม PDx")
                            .event(clone!(audit => move |_:events::Click| {
                                audit.summary_audit_pdx.lock_mut().push_cloned(SummaryAuditItemMutable::new("PDx", audit.summary_audit_id.get(), audit.summary_id.get()));
                            }))
                        })
                    })
                })))
                .children([
                    html!("button", {
                        .attr("type","button")
                        .class(class::BTN_SM_L_BLUE)
                        .child(html!("i", {.class(class::FA_PLUS_L)}))
                        .text("เพิ่ม SDx")
                        .event(clone!(audit => move |_:events::Click| {
                            audit.summary_audit_sdxs.lock_mut().push_cloned(SummaryAuditItemMutable::new("SDx", audit.summary_audit_id.get(), audit.summary_id.get()));
                        }))
                    }),
                    html!("button", {
                        .attr("type","button")
                        .class(class::BTN_SM_L_BLUE)
                        .child(html!("i", {.class(class::FA_PLUS_L)}))
                        .text("เพิ่ม ODx")
                        .event(clone!(audit => move |_:events::Click| {
                            audit.summary_audit_odxs.lock_mut().push_cloned(SummaryAuditItemMutable::new("ODx", audit.summary_audit_id.get(), audit.summary_id.get()));
                        }))
                    }),
                    html!("button", {
                        .attr("type","button")
                        .class(class::BTN_SM_L_BLUE)
                        .child(html!("i", {.class(class::FA_PLUS_L)}))
                        .text("เพิ่ม Op")
                        .event(clone!(audit => move |_:events::Click| {
                            audit.summary_audit_ops.lock_mut().push_cloned(SummaryAuditItemMutable::new("Op", audit.summary_audit_id.get(), audit.summary_id.get()));
                        }))
                    }),
                ])
            }))
            .children_signal_vec(audit.summary_audit_pdx.signal_vec_cloned().map(clone!(app, audit => move |audit_item| {
                SummaryAuditItemMutable::render(None,  audit_item, audit.summary_audit_pdx.clone(), audit.changed.clone(), app.clone())
            })))
            .children_signal_vec(audit.summary_audit_sdxs.signal_vec_cloned().enumerate().map(clone!(app, audit => move |(i, audit_item)| {
                SummaryAuditItemMutable::render(Some(i), audit_item, audit.summary_audit_sdxs.clone(), audit.changed.clone(), app.clone())
            })))
            .children_signal_vec(audit.summary_audit_odxs.signal_vec_cloned().enumerate().map(clone!(app, audit => move |(i, audit_item)| {
                SummaryAuditItemMutable::render(Some(i), audit_item, audit.summary_audit_odxs.clone(), audit.changed.clone(), app.clone())
            })))
            .children_signal_vec(audit.summary_audit_ops.signal_vec_cloned().enumerate().map(clone!(app, audit => move |(i, audit_item)| {
                SummaryAuditItemMutable::render(Some(i), audit_item, audit.summary_audit_ops.clone(), audit.changed.clone(), app.clone())
            })))
        })
    }

    fn render_com_drg(audit: Rc<Self>) -> Dom {
        html!("div", {
            .class(class::BOX_ROUND_T)
            .style("break-inside","avoid")
            .children([
                html!("button", {
                    .attr("type","button")
                    .class(class::BTN_SM_FR_RT_CYAN)
                    .text("คำนวน DRG")
                    .event(clone!(audit => move |_:events::Click| {
                        audit.check_com_drg.set_neq(true);
                        audit.changed.set_neq(true);
                    }))
                }),
                html!("div", {.class(class::BOLD_M2).text("DRG in Computer")}),
                html!("div", {
                    .children([
                        html!("div", {
                            .class(class::INPUT_GROUP_SM_T)
                            .children([
                                html!("span", {.class("input-group-text").text("DRG")}),
                                html!("input" => HtmlInputElement, {
                                    .attr("type","text")
                                    .class(class::FORM_CTRL_SM)
                                    .apply(mixins::string_value(audit.com_drg.clone(), audit.changed.clone()))
                                }),
                                html!("span", {.class("input-group-text").text("RW")}),
                                html!("input" => HtmlInputElement, {
                                    .attr("type","text")
                                    .class(class::FORM_CTRL_SM)
                                    .apply(mixins::string_value(audit.com_rw.clone(), audit.changed.clone()))
                                }),
                                html!("span", {.class("input-group-text").text("AdjRW")}),
                                html!("input" => HtmlInputElement, {
                                    .attr("type","text")
                                    .class(class::FORM_CTRL_SM)
                                    .apply(mixins::string_value(audit.com_adjrw.clone(), audit.changed.clone()))
                                }),
                            ])
                        }),
                        html!("textarea", {
                            .attr("rows", "13")
                            .class(class::FORM_CTRL_SM)
                            .class(&*class::MONO_PRE_WRAP)
                            .text_signal(audit.com_drg_panel.signal_cloned())
                        })
                    ])
                })
            ])
        })
    }

    fn render_rev_drg(audit: Rc<Self>) -> Dom {
        html!("div", {
            .class(class::BOX_ROUND_T)
            .style("break-inside","avoid")
            .children([
                html!("button", {
                    .attr("type","button")
                    .class(class::BTN_SM_FR_RT_CYAN)
                    .text("คำนวน DRG")
                    .event(clone!(audit => move |_:events::Click| {
                        audit.check_rev_drg.set_neq(true);
                        audit.changed.set_neq(true);
                    }))
                }),
                html!("div", {
                    .children([
                        html!("span", {.class(class::BOLD_M2).text("DRG from R ICD")}),
                        html!("span", {
                            .class("fw-bold")
                            .class_signal("text-danger", audit.adjrw_diff_signal().map(|opt| opt.map(|diff| diff < 0.0).unwrap_or_default()))
                            .class_signal("text-success", audit.adjrw_diff_signal().map(|opt| opt.map(|diff| diff > 0.0).unwrap_or_default()))
                            .text_signal(audit.adjrw_diff_signal().map(|opt| opt.map(|diff| {
                                ["(", if diff > 0.0 {"+"} else {""}, &f64_rescale(diff, 4).to_string(), " AdjRW)"].concat()
                            }).unwrap_or_default()))
                        }),
                    ])
                }),
                html!("div", {
                    .children([
                        html!("div", {
                            .class(class::INPUT_GROUP_SM_T)
                            .children([
                                html!("span", {.class("input-group-text").text("DRG")}),
                                html!("input" => HtmlInputElement, {
                                    .attr("type","text")
                                    .class(class::FORM_CTRL_SM)
                                    .apply(mixins::string_value(audit.rev_drg.clone(), audit.changed.clone()))
                                }),
                                html!("span", {.class("input-group-text").text("RW")}),
                                html!("input" => HtmlInputElement, {
                                    .attr("type","text")
                                    .class(class::FORM_CTRL_SM)
                                    .apply(mixins::string_value(audit.rev_rw.clone(), audit.changed.clone()))
                                }),
                                html!("span", {.class("input-group-text").text("AdjRW")}),
                                html!("input" => HtmlInputElement, {
                                    .attr("type","text")
                                    .class(class::FORM_CTRL_SM)
                                    .apply(mixins::string_value(audit.rev_adjrw.clone(), audit.changed.clone()))
                                }),
                            ])
                        }),
                        html!("textarea", {
                            .attr("rows", "13")
                            .class(class::FORM_CTRL_SM)
                            .class(&*class::MONO_PRE_WRAP)
                            .text_signal(audit.rev_drg_panel.signal_cloned())
                        })
                    ])
                })
            ])
        })
    }
}

#[derive(Clone, Default)]
struct SummaryAuditItemMutable {
    auto_id: u32,
    summary_audit_item_id: Mutable<u32>,
    summary_audit_id: Mutable<u32>,
    summary_id: Mutable<u32>,
    ty: String,
    sum_dx: Mutable<String>,
    sum_icd: Mutable<String>,
    com_icd: Mutable<String>,
    rev_dx: Mutable<String>,
    rev_icd: Mutable<String>,
    sa: Mutable<String>,
    ca: Mutable<String>,
    remark: Mutable<String>,

    // Some(true) = dx, Some(false) = op
    find_dx_op: Mutable<Option<bool>>,
}

impl SummaryAuditItemMutable {
    fn new(ty: &str, summary_audit_id: u32, summary_id: u32) -> Rc<Self> {
        Rc::new(Self {
            auto_id: ID_COUNTER.fetch_add(1, Ordering::SeqCst),
            summary_audit_id: Mutable::new(summary_audit_id),
            summary_id: Mutable::new(summary_id),
            ty: ty.to_owned(),
            ..Default::default()
        })
    }

    fn new_from_audit_item(audit_item: &SummaryAuditItem) -> Rc<Self> {
        Rc::new(Self {
            auto_id: ID_COUNTER.fetch_add(1, Ordering::SeqCst),
            summary_audit_item_id: Mutable::new(audit_item.summary_audit_item_id),
            summary_audit_id: Mutable::new(audit_item.summary_audit_id),
            summary_id: Mutable::new(audit_item.summary_id),
            ty: audit_item.ty.clone().unwrap_or(String::from("ODx")),
            sum_dx: Mutable::new(audit_item.sum_dx.clone().unwrap_or_default()),
            sum_icd: Mutable::new(audit_item.sum_icd.clone().unwrap_or_default()),
            com_icd: Mutable::new(audit_item.com_icd.clone().unwrap_or_default()),
            rev_dx: Mutable::new(audit_item.rev_dx.clone().unwrap_or_default()),
            rev_icd: Mutable::new(audit_item.rev_icd.clone().unwrap_or_default()),
            sa: Mutable::new(audit_item.sa.clone()),
            ca: Mutable::new(audit_item.ca.clone()),
            remark: Mutable::new(audit_item.remark.clone().unwrap_or_default()),
            ..Default::default()
        })
    }

    fn to_audit_item(&self) -> SummaryAuditItem {
        SummaryAuditItem {
            summary_audit_item_id: self.summary_audit_item_id.get(),
            summary_audit_id: self.summary_audit_id.get(),
            summary_id: self.summary_id.get(),
            ty: Some(self.ty.clone()),
            sum_dx: str_some(self.sum_dx.get_cloned()),
            sum_icd: str_some(self.sum_icd.get_cloned()),
            com_icd: str_some(self.com_icd.get_cloned()),
            rev_dx: str_some(self.rev_dx.get_cloned()),
            rev_icd: str_some(self.rev_icd.get_cloned()),
            sa: self.sa.get_cloned(),
            ca: self.ca.get_cloned(),
            remark: str_some(self.remark.get_cloned()),
        }
    }

    /// icd10tm(dx) or proc(not dx)
    fn find_icd(is_dx: bool, item: Rc<Self>, app: Rc<App>, changed: Mutable<bool>) {
        app.async_load(
            false,
            clone!(app, item => async move {
                let thread = app.drg_worker().await;
                let search_text = item.rev_icd.get_cloned();
                if !search_text.is_empty() {
                    let detail = if is_dx {
                        thread.get_icd10_desc(search_text).await
                    } else {
                        thread.get_proc_desc(search_text).await
                    };
                    let neq = item.rev_dx.lock_ref().as_str() != &detail;
                    if neq {
                        item.rev_dx.set(detail);
                        changed.set_neq(true);
                    }
                }
            }),
        )
    }

    fn render(i_opt: Option<ReadOnlyMutable<Option<usize>>>, item: Rc<Self>, audit: MutableVec<Rc<SummaryAuditItemMutable>>, changed: Mutable<bool>, app: Rc<App>) -> Dom {
        html!("div", {
            .future(map_ref! {
                let busy = app.loader_is_loading(),
                let opt = item.find_dx_op.signal() =>
                (!busy && opt.is_some(), opt.unwrap_or_default())
            }.for_each(clone!(app, item, changed => move |(ready, is_dx)| {
                if ready {
                    Self::find_icd(is_dx, item.clone(), app.clone(), changed.clone());
                    item.find_dx_op.set(None);
                }
                async {}
            })))
            .class(class::BORDER_ROUND)
            .apply(clone!(i_opt => move |dom| {
                if let Some(i) = i_opt {
                    dom.child(html!("button", {
                        .attr("type","button")
                        .class(class::BTN_SM_FR_RED)
                        .child(html!("i", {.class(class::FA_X)}))
                        .event(clone!(audit => move |_:events::Click| {
                            audit.lock_mut().remove(i.get().unwrap_or_default());
                        }))
                    }))
                } else {
                    dom
                }
            }))
            .children([
                html!("div", {
                    .class(class::BOLD_T2)
                    .child(html!("span", {
                        .class(match item.ty.as_str() {
                            "PDx" => class::BADGE_BLUE_L,
                            "SDx" => class::BADGE_CYAN_L,
                            "ODx" => class::BADGE_GRAY_L,
                            "Op" => class::BADGE_GOLD_L,
                            _ => class::BADGE_RED_L,
                        })
                        .text(&item.ty)
                        .apply(clone!(i_opt => move |dom| {
                            if let Some(i) = i_opt {
                                dom.text_signal(i.signal().map(|id|(id.unwrap_or_default() + 1).to_string()))
                            } else {
                                dom
                            }
                        }))
                    }))
                }),
                html!("div", {
                    .class(class::INPUT_GROUP_SM_T)
                    .children([
                        html!("span", {
                            .class("input-group-text")
                            .text(&item.ty)
                            .text(" in Summary")
                        }),
                        html!("input" => HtmlInputElement, {
                            .attr("type","text")
                            .attr("id", &["sum-dx-", &item.auto_id.to_string()].concat())
                            .attr("draggable","true")
                            .class(class::FORM_CTRL_SM)
                            .apply(mixins::string_value(item.sum_dx.clone(), changed.clone()))
                            .apply(drag_and_drop("sum-dx", item.sum_dx.clone(), None, changed.clone(), app.clone()))
                        }),
                        html!("button", {
                            .attr("type","button")
                            .class(class::BTN_SM_GRAY)
                            .child(html!("i", {.class(class::FA_X)}))
                            .event(clone!(item, changed => move |_:events::Click| {
                                let is_empty = item.sum_dx.lock_ref().is_empty();
                                if !is_empty {
                                    item.sum_dx.set(String::new());
                                    changed.set_neq(true);
                                }
                            }))
                        }),
                    ])
                }),
                html!("div", {
                    .class(class::INPUT_GROUP_SM_T)
                    .children([
                        html!("span", {.class("input-group-text").text("S ICD")}),
                        html!("input" => HtmlInputElement, {
                            .attr("type","text")
                            .attr("id", &["sum-icd-", &item.auto_id.to_string()].concat())
                            .attr("draggable","true")
                            .class(class::FORM_CTRL_SM)
                            .style("max-width","70px")
                            .apply(mixins::string_value(item.sum_icd.clone(), changed.clone()))
                            .apply(drag_and_drop("sum", item.sum_icd.clone(), None, changed.clone(), app.clone()))
                        }),
                        html!("span", {.class("input-group-text").text("C ICD")}),
                        html!("input" => HtmlInputElement, {
                            .attr("type","text")
                            .attr("id", &["com-icd-", &item.auto_id.to_string()].concat())
                            .attr("draggable","true")
                            .class(class::FORM_CTRL_SM)
                            .style("max-width","70px")
                            .apply(mixins::string_value(item.com_icd.clone(), changed.clone()))
                            .apply(drag_and_drop("com", item.com_icd.clone(), None, changed.clone(), app.clone()))
                        }),
                        html!("span", {.class("input-group-text").text("R ICD")}),
                        html!("input" => HtmlInputElement, {
                            .attr("type","text")
                            .attr("id", &["rev-icd-", &item.auto_id.to_string()].concat())
                            .attr("draggable","true")
                            .class(class::FORM_CTRL_SM_GOLD)
                            .style("max-width","70px")
                            .apply(drag_and_drop("rev", item.rev_icd.clone(), Some((item.ty.as_str() != "Op", item.find_dx_op.clone())), changed.clone(), app.clone()))
                            .prop_signal("value", item.rev_icd.signal_cloned())
                            .with_node!(element => {
                                .event(clone!(app, item, changed, element => move |_:events::Change| {
                                    let value = element.value().to_ascii_uppercase();
                                    let value_is_valid = !value.is_empty();
                                    let rev_icd = item.rev_icd.get_cloned();
                                    let neq = rev_icd != value;
                                    if neq {
                                        item.rev_icd.set(value);
                                        if value_is_valid {
                                            item.find_dx_op.set(Some(item.ty.as_str() != "Op"));
                                        }
                                        changed.set_neq(true);
                                    }
                                }))
                                .event_with_options(&EventOptions::preventable(), clone!(app, item, changed => move |event: events::KeyDown| {
                                    if event.key() == "Enter" {
                                        event.prevent_default();
                                        let value = element.value().to_ascii_uppercase();
                                        let value_is_valid = !value.is_empty();
                                        let rev_icd = item.rev_icd.get_cloned();
                                        let neq = rev_icd != value;
                                        if neq {
                                            item.rev_icd.set(value);
                                            if value_is_valid {
                                                item.find_dx_op.set(Some(item.ty.as_str() != "Op"));
                                            }
                                            changed.set_neq(true);
                                        }
                                    }
                                }))
                            })
                        }),
                        html!("span", {.class("input-group-text").text("Remark")}),
                        html!("input" => HtmlInputElement, {
                            .attr("type","text")
                            .class(class::FORM_CTRL_SM_GOLD)
                            .apply(mixins::string_value(item.remark.clone(), changed.clone()))
                        }),
                    ])
                }),
                html!("div", {
                    .class(class::INPUT_GROUP_SM_T)
                    .children([
                        html!("span", {
                            .class("input-group-text")
                            .text(&item.ty)
                            .text(" from Review")
                        }),
                        html!("input" => HtmlInputElement, {
                            .attr("type","text")
                            .attr("id", &["rev-dx-", &item.auto_id.to_string()].concat())
                            .attr("draggable","true")
                            .class(class::FORM_CTRL_SM_GOLD)
                            .apply(mixins::string_value(item.rev_dx.clone(), changed.clone()))
                            .apply(drag_and_drop("rev-dx", item.rev_dx.clone(), None, changed.clone(), app.clone()))
                        }),
                        html!("button", {
                            .attr("type","button")
                            .class(class::BTN_SM_GRAY)
                            .child(html!("i", {.class(class::FA_X)}))
                            .event(clone!(item, changed => move |_:events::Click| {
                                let is_empty = item.rev_dx.lock_ref().is_empty();
                                if !is_empty {
                                    item.rev_dx.set(String::new());
                                    changed.set_neq(true);
                                }
                            }))
                        }),
                    ])
                }),
                html!("div", {
                    .class(class::INPUT_GROUP_SM_T)
                    .children([
                        html!("span", {.class("input-group-text").text("SA")}),
                        select_sa(&item.ty, item.sa.clone(), changed.clone()),
                        html!("span", {.class("input-group-text").text("CA")}),
                        select_ca(&item.ty, item.ca.clone(), changed.clone()),
                    ])
                }),
            ])
        })
    }
}

fn td_input_text(mutable: Mutable<String>, changed: Mutable<bool>, max_length: &str) -> Dom {
    html!("td", {
        .class("p-0")
        .child(html!("input" => HtmlInputElement, {
            .attr("type", "text")
            .class(class::FORM_CTRL_B0)
            .attr("maxlength", max_length)
            .apply(mixins::string_value(mutable, changed))
        }))
    })
}

fn td_input_number(mutable: Mutable<String>, changed: Mutable<bool>, step: &str) -> Dom {
    html!("td", {
        .class("p-0")
        .child(html!("input" => HtmlInputElement, {
            .attr("type", "number")
            .class(class::FORM_CTRL_B0)
            .attr("step", step)
            .apply(mixins::string_value(mutable, changed))
        }))
    })
}

fn td_input_kg_to_gram(mutable: Mutable<String>, changed: Mutable<bool>) -> Dom {
    html!("td", {
        .class("p-0")
        .child(html!("input" => HtmlInputElement, {
            .attr("type", "number")
            .class(class::FORM_CTRL_B0)
            .attr("step", "0.001")
            .prop_signal("value", mutable.signal_ref(|v| v.parse::<f64>().map(|f| f64_rescale(f / 1000.0, 3).to_string()).unwrap_or_default()))
            .with_node!(element => {
                .event(move |_:events::Change| {
                    if let Ok(v_f64) = element.value().parse::<f64>() {
                        let v = f64_rescale(v_f64 * 1000.0, 0).to_string();
                        let is_neq = mutable.lock_ref().as_str() != v;
                        if is_neq {
                            mutable.set(v);
                            changed.set_neq(true);
                        }
                    } else {
                        mutable.set(String::new());
                    }
                })
            })
        }))
    })
}

fn td_input_select(items: &[(&str, &str)], mutable: Mutable<String>, changed: Mutable<bool>) -> Dom {
    html!("td", {
        .class("p-0")
        .child(html!("select" => HtmlSelectElement, {
            .class(class::FORM_SELECT_R0)
            .style("border-color","transparent")
            .child(html!("option", {.attr("value", "").text("เลือก")}))
            .children(items.iter().map(|(k,v)| {
                html!("option", {.attr("value", k).text(v)})
            }))
            .apply(mixins::string_value_select(mutable, changed))
        }))
    })
}

fn td_input_date(mutable: Mutable<String>, changed: Mutable<bool>) -> Dom {
    html!("td", {
        .class("p-0")
        .child(doms::date_picker(
            mutable,
            changed, always(false), None,
            |d| d.style("min-width","135px"),
            |d| d.class(class::B0R0),
            |d| d.class(class::B0R0),
            |s| s, always(None),
        ))
    })
}

fn td_input_datetime(mutable: Mutable<String>, changed: Mutable<bool>) -> Dom {
    html!("td", {
        .class("p-0")
        .child(doms::datetime_picker(
            mutable,
            changed, always(false),
            |d| d.style("min-width","190px"),
            |d| d.class(class::B0R0),
            |d| d.class(class::B0R0),
            |s| s, always(None),
        ))
    })
}

fn td_correct_toggle(com_mutable: Mutable<String>, rev_mutable: Mutable<String>, changed: Mutable<bool>) -> Dom {
    html!("td", {
        .class("p-0")
        .child(html!("div", {
            .class("text-center")
            .child(html!("button", {
                .attr("type","button")
                .style("border-style","none")
                .style("background-color","transparent")
                .style("font-size","24px")
                .style("padding","0")
                .child_signal(map_ref! {
                    let com = com_mutable.signal_cloned(),
                    let rev = rev_mutable.signal_cloned() =>
                    com == rev
                }.map(|is_same| {
                    if is_same {
                        Some(html!("i", {.class(class::FA_CHECK_CIRCLE_GREEN)}))
                    } else {
                        Some(html!("i", {.class(class::FA_X_CIRCLE_RED)}))
                    }
                }))
                .event(move |_:events::Click| {
                    let is_eq = com_mutable.lock_ref().as_str() == rev_mutable.lock_ref().as_str();
                    if is_eq {
                        rev_mutable.set(String::new());
                    } else {
                        rev_mutable.set(com_mutable.get_cloned());
                    }
                    changed.set_neq(true);
                })
            }))
        }))
    })
}

fn select_sa(ty: &str, mutable: Mutable<String>, changed: Mutable<bool>) -> Dom {
    let items = match ty {
        "PDx" => vec![
            ("0", "SA0 : ความเห็นเกี่ยวกับการสรุป สอดคล้องกัน"),
            ("1a", "SA1a : ไม่สรุป PDx"),
            ("1b", "SA1b : สรุป PDx ไม่ถูกต้อง"),
            ("1c", "SA1c : สรุป PDx ไม่เฉพาะเจาะจง"),
            ("1d", "SA1d : สรุป PDx โดยไม่มีหลักฐานในเวชระเบียน"),
        ],
        "SDx" | "ODx" => vec![
            ("0", "SA0 : ความเห็นเกี่ยวกับการสรุป สอดคล้องกัน"),
            ("2a", "SA2a : ไม่สรุป CC"),
            ("2b", "SA2b : สรุป CC ไม่ถูกต้อง"),
            ("2c", "SA2c : สรุป CC ไม่เฉพาะเจาะจง"),
            ("2d", "SA2d : สรุป CC โดยไม่มีหลักฐานในเวชระเบียน"),
        ],
        "Op" => vec![
            ("0", "SA0 : ความเห็นเกี่ยวกับการสรุป สอดคล้องกัน"),
            ("3a", "SA3a : ไม่สรุป Op"),
            ("3b", "SA3b : สรุป Op ไม่ถูกต้อง"),
            ("3c", "SA3c : สรุป Op ไม่เฉพาะเจาะจง"),
            ("3d", "SA3d : สรุป Op โดยไม่มีหลักฐานในเวชระเบียน"),
        ],
        _ => vec![
            ("0", "SA0 : ความเห็นเกี่ยวกับการสรุป สอดคล้องกัน"),
            ("5", "SA5 : ไม่มีการสรุปเวชระเบียน"),
            ("6", "SA6 : ปัญหาอื่น ใช้คำย่อ คำกำกวม อ่านไม่ออก"),
        ],
    };
    html!("select" => HtmlSelectElement, {
        .class(class::FORM_SELECT_SM)
        .class_signal("bg-danger-subtle", mutable.signal_cloned().map(|s| !s.is_empty() && s.as_str() != "0"))
        .class_signal("bg-warning-subtle", mutable.signal_cloned().map(|s| s.is_empty()))
        .class_signal("bg-success-subtle", mutable.signal_cloned().map(|s| s.as_str() == "0"))
        .child(html!("option", {.attr("value", "").text("เลือก")}))
        .children(items.iter().map(|(k,v)| {
            html!("option", {.attr("value", k).text(v)})
        }))
        .apply(mixins::string_value_select(mutable, changed))
    })
}

fn select_ca(ty: &str, mutable: Mutable<String>, changed: Mutable<bool>) -> Dom {
    let items = match ty {
        "PDx" => vec![
            ("0", "CA0 : ความเห็นเกี่ยวกับการให้รหัส สอดคล้องกัน"),
            ("1a", "CA1a : ไม่ให้รหัส PDx"),
            ("1b", "CA1b : ให้รหัส PDx ไม่ถูกต้องตามมาตรฐานการให้รหัส"),
            ("1c", "CA1c : ให้รหัส PDx ไม่เฉพาะเจาะจงตามมาตรฐานการให้รหัส"),
        ],
        "SDx" | "ODx" => vec![
            ("0", "CA0 : ความเห็นเกี่ยวกับการให้รหัส สอดคล้องกัน"),
            ("2a", "CA2a : ไม่ให้รหัส CC"),
            ("2b", "CA2b : ให้รหัส CC ไม่ถูกต้องตามมาตรฐานการให้รหัส"),
            ("2c", "CA2c : ให้รหัส CC ไม่เฉพาะเจาะจงตามมาตรฐานการให้รหัส"),
            ("2d", "CA2d : เพิ่มรหัส CC ไม่ตรงตามมาตรฐานการให้รหัส"),
        ],
        "Op" => vec![
            ("0", "CA0 : ความเห็นเกี่ยวกับการให้รหัส สอดคล้องกัน"),
            ("3a", "CA3a : ไม่ให้รหัส Op"),
            ("3b", "CA3b : ให้รหัส Op ไม่ถูกต้องตามมาตรฐานการให้รหัส"),
            ("3c", "CA3c : ให้รหัส Op ไม่เฉพาะเจาะจงตามมาตรฐานการให้รหัส"),
            ("3d", "CA3d : เพิ่มรหัส Op ไม่ตรงตามมาตรฐานการให้รหัส"),
        ],
        _ => vec![
            ("0", "CA0 : ความเห็นเกี่ยวกับการให้รหัส สอดคล้องกัน"),
            ("6", "CA6 : ปัญหาอื่น ซึ่งอาจทำให้การวินิจฉัยหรือหัตถการของ Doctor และ Coder ต่างกัน"),
        ],
    };
    html!("select" => HtmlSelectElement, {
        .class(class::FORM_SELECT_SM)
        .class_signal("bg-danger-subtle", mutable.signal_cloned().map(|s| !s.is_empty() && s.as_str() != "0"))
        .class_signal("bg-warning-subtle", mutable.signal_cloned().map(|s| s.is_empty()))
        .class_signal("bg-success-subtle", mutable.signal_cloned().map(|s| s.as_str() == "0"))
        .child(html!("option", {.attr("value", "").text("เลือก")}))
        .children(items.iter().map(|(k,v)| {
            html!("option", {.attr("value", k).text(v)})
        }))
        .apply(mixins::string_value_select(mutable, changed))
    })
}

fn ty_badge(ty: &str) -> Dom {
    html!("span", {
        .class(match ty {
            "PDx" => class::BADGE_BLUE_L,
            "SDx" => class::BADGE_CYAN_L,
            "ODx" => class::BADGE_GRAY_L,
            _ => class::BADGE_GOLD_L,
        })
        .text(&ty)
    })
}

fn dagger_aster_badges(opt: Option<Arc<I10vx>>, aster: Arc<I10vx>) -> Dom {
    let dagger = opt
        .map(|dag| {
            html!("span", {
                .class(class::BADGE_GOLD)
                .style("cursor","pointer")
                .text(&icd10_dot(&dag.code))
                .child(html!("i", {.class(class::FA_CROSS_RED)}))
                .attr("title", &dag.desc)
            })
        })
        .unwrap_or(html!("span", {
            .class(class::BADGE_CYAN)
            .style("cursor","pointer")
            .text("???")
            .child(html!("i", {.class(class::FA_CROSS_RED)}))
        }));
    html!("span", {
        .class("me-1")
        .children([
            dagger,
            html!("span", {
                .class(class::BADGE_GOLD)
                .style("cursor","pointer")
                .text(&icd10_dot(&aster.code))
                .child(html!("i", {.class(class::FA_ASTERISK_RED)}))
                .attr("title", &aster.desc)
            })
        ])
    })
}

/// - source and this has the same `ty`, it will `swap` the value between source `mutable` with this `mutable` (if source is NOT a INPUT element, it will `copy` textContent to this `mutable`)
/// - source and this has different `ty`, source value/textContent will `copy` to this mutable
fn drag_and_drop(
    ty: &'static str,
    mutable: Mutable<String>,
    find_dx_op_mutable: Option<(bool, Mutable<Option<bool>>)>,
    changed: Mutable<bool>,
    app: Rc<App>,
) -> impl FnOnce(DomBuilder<HtmlInputElement>) -> DomBuilder<HtmlInputElement> {
    #[inline]
    move |dom| {
        with_node!(dom, element => {
            .event(clone!(app, mutable => move |event: events::DragStart| {
                app.drag_start_state.set(Some(DragStartState::Input((ty, mutable.clone()))));
                if let Some(data_transfer) = event.data_transfer() {
                    data_transfer.set_data("text", &element.value()).unwrap();
                }
            }))
            .event_with_options(&EventOptions::preventable(), move |event: events::DragOver| {
                event.prevent_default();
            })
            .event_with_options(&EventOptions::preventable(), move |event: events::Drop| {
                event.prevent_default();
                // if let Some(data_transfer) = event.data_transfer() {
                    // if let Ok(source_id) = data_transfer.get_data("text") {
                        // if let Some(source_elm) = app.get_id(&source_id) {
                            if let Some(drag_start_state) = app.drag_start_state.get_cloned() {
                                let old = mutable.get_cloned();
                                let source = drag_start_state.get_value();
                                if old != source {
                                    let source_is_valid = !source.is_empty();
                                    mutable.set(source);
                                    if drag_start_state.get_type() == ty { //} && source_elm.tag_name().as_str() == "INPUT" {
                                        drag_start_state.set_input(&old);
                                    }
                                    if source_is_valid && let Some((is_dx, find_dx_op)) = find_dx_op_mutable.as_ref() {
                                        find_dx_op.set_neq(Some(*is_dx));
                                    }
                                    changed.set_neq(true);
                                }
                            }
                        // }
                    // }
                // }
            })
        })
    }
}
