// ipd-nurse-focus-note.php

use dominator::{Dom, EventOptions, clone, events, html, with_node};
use futures_signals::{
    map_ref,
    signal::{Mutable, Signal, SignalExt, always, not},
    signal_vec::{MutableVec, SignalVecExt},
};
use std::rc::Rc;
use wasm_bindgen::JsCast;
use web_sys::{HtmlButtonElement, HtmlInputElement, HtmlSelectElement, HtmlTextAreaElement};

use kphis_model::{
    app::VisitTypeId,
    endpoint::EndPoint,
    fetch::Method,
    focus_list::FocusList,
    focus_note::{FocusNote, FocusNoteParams, FocusNoteSave, FocusNoteSaveParams},
    image::file_path::ImageUsage,
    ipd::tmp::{TmpDlc, TmpIntvt, TmpParams},
    patient_info::PatientInfo,
};
use kphis_ui_app::App;
use kphis_ui_core::{class, doms, mixins};
use kphis_util::{
    datetime::{JsTime, date_8601, js_now, time_8601},
    util::{str_some, zero_none},
};

use crate::{
    gadget::image::{ImageCpn, ImagePaths},
    modal::{blank_modal, lab_selector::LabSelector, vs_selector::VsSelector},
};

/// - GET `EndPoint::IpdTmpIntvt`
/// - POST `EndPoint::IpdFocusNoteAn` (guarded, remove `บันทึก` btn)
/// - POST `EndPoint::OpdErFocusNoteId` (guarded, remove `บันทึก` btn)
/// - DELETE `EndPoint::IpdFocusNoteAn` (guarded, remove `ลบ` btn)
/// - DELETE `EndPoint::OpdErFocusNoteId` (guarded, remove `ลบ` btn)
/// - POST `EndPoint::ImageUsage` (guarded, remove ImageCpn)
/// - GET `EndPoint::LabHead` (LabSelector, guarded, remove lab btn)
/// - GET `EndPoint::IpdVitalSign` (VsSelector, guarded, remove v/s btn)
/// - GET `EndPoint::OpdErVitalSign` (VsSelector, guarded, remove v/s btn)
#[derive(Default)]
pub struct NurseNoteFormCpn {
    focus_list: MutableVec<Rc<FocusList>>,
    dlcs: Mutable<Vec<TmpDlc>>,
    // request FocusNoteCpn to load dlcs
    load_dlcs: Mutable<bool>,

    patient: Mutable<Option<Rc<PatientInfo>>>,

    // image component callback state
    assess_image_callback: Mutable<ImagePaths>,
    eva_image_callback: Mutable<ImagePaths>,

    version: Mutable<i32>,
    parent_focus_note: Mutable<Option<Rc<FocusNote>>>,
    focus_note: Mutable<Option<Rc<FocusNote>>>,

    fcnote_id: Mutable<u32>,
    general_symptoms: Mutable<String>,
    fclist_id: Mutable<String>,
    assessment: Mutable<String>,
    // intvt_id: Mutable<String>,
    intvt_text: Mutable<String>,
    evalution: Mutable<String>,
    // dlc_id: Mutable<String>,
    dlc_text: Mutable<String>,
    other: Mutable<String>,
    fcnote_date: Mutable<String>,
    fcnote_time: Mutable<String>,
    fcnote_patient_type: Mutable<String>,
    doctorcode: Mutable<String>,

    // forms state
    changed: Mutable<bool>,
    parent_reload: Mutable<bool>,

    loaded_intvt: Mutable<bool>,
    intvts: MutableVec<Rc<TmpIntvt>>,
    intvt_ids: MutableVec<u32>,
    dlc_ids: MutableVec<u32>,

    vs_selector_modal: Mutable<Option<Rc<VsSelector>>>,
    lab_selector_modal: Mutable<Option<Rc<LabSelector>>>,
}

impl NurseNoteFormCpn {
    pub fn new(
        focus_list: MutableVec<Rc<FocusList>>,
        dlcs: Mutable<Vec<TmpDlc>>,
        load_dlcs: Mutable<bool>,
        patient: Mutable<Option<Rc<PatientInfo>>>,
        parent_focus_note: Mutable<Option<Rc<FocusNote>>>,
        parent_reload: Mutable<bool>,
    ) -> Rc<Self> {
        let now = js_now();
        Rc::new(Self {
            focus_list,
            load_dlcs,
            dlcs,
            patient,
            parent_focus_note,
            fcnote_date: Mutable::new(now.date().to_string()),
            loaded_intvt: Mutable::new(true),
            parent_reload,
            ..Default::default()
        })
    }

    fn new_form(page: Rc<Self>) {
        let now = js_now();

        page.parent_focus_note.set(None);
        page.focus_note.set(None);

        page.fcnote_id.set_neq(0);
        page.general_symptoms.set_neq(String::new());
        page.fclist_id.set_neq(String::new());
        page.assessment.set_neq(String::new());
        // page.intvt_id.set_neq(String::new());
        page.intvt_text.set_neq(String::new());
        page.evalution.set_neq(String::new());
        // page.dlc_id.set_neq(String::new());
        page.dlc_text.set_neq(String::new());
        page.other.set_neq(String::new());
        page.fcnote_date.set_neq(now.date().to_string());
        // page.fcnote_time.set_neq(now.time().js_string());
        page.fcnote_patient_type.set_neq(String::new());
        page.doctorcode.set_neq(String::new());
        page.version.set_neq(0);

        page.intvts.lock_mut().clear();
        page.intvt_ids.lock_mut().clear();
        page.dlc_ids.lock_mut().clear();
        page.changed.set(false);
    }

    fn is_ipd_and_is_pre_admit(&self) -> impl Signal<Item = (bool, bool)> + use<> {
        self.patient
            .signal_cloned()
            .map(|opt| opt.as_ref().map(|pt| pt.visit_type.is_ipd_and_is_pre_admit()).unwrap_or_default())
    }

    fn is_pre_admit(&self) -> impl Signal<Item = bool> + use<> {
        self.patient.signal_cloned().map(|opt| opt.as_ref().map(|pt| pt.visit_type.is_pre_admit()).unwrap_or_default())
    }

    fn load_focus_note_row(row: Rc<FocusNote>, page: Rc<Self>, app: Rc<App>) {
        page.fcnote_id.set_neq(row.fcnote_id);
        page.general_symptoms.set_neq(row.general_symptoms.clone().unwrap_or_default());
        page.fclist_id.set_neq(row.fclist_id.map(|i| i.to_string()).unwrap_or_default());
        page.assessment.set_neq(row.assessment.clone().unwrap_or_default());
        // page.intvt_id.set_neq(row.intvt_id.clone().unwrap_or_default());
        page.intvt_text.set_neq(row.intvt_text.clone().unwrap_or_default());
        page.evalution.set_neq(row.evalution.clone().unwrap_or_default());
        // page.dlc_id.set_neq(row.dlc_id.clone().unwrap_or_default());
        page.dlc_text.set_neq(row.dlc_text.clone().unwrap_or_default());
        page.other.set_neq(row.other.clone().unwrap_or_default());
        page.fcnote_date.set_neq(row.fcnote_date.map(|d| d.to_string()).unwrap_or_default());
        page.fcnote_time.set_neq(row.fcnote_time.map(|t| t.js_string()).unwrap_or_default());
        page.fcnote_patient_type.set_neq(row.fcnote_patient_type.clone().unwrap_or_default());
        page.doctorcode.set_neq(row.doctorcode.clone().unwrap_or_default());
        page.version.set_neq(row.version);

        app.async_load(
            true,
            clone!(app => async move {
                // reload + render intvt
                let id = page.fclist_id.lock_ref().parse::<u32>().unwrap_or_default();
                if let Some(focus_list) = page.focus_list.lock_ref().iter().find(|fcl| fcl.fclist_id == id) {
                    let params = TmpParams {
                        smp_id: Some(focus_list.smp_id),
                        subgroup: focus_list.subgroup.to_owned(),
                        ..Default::default()
                    };
                    // GET `EndPoint::IpdTmpIntvt`
                    match TmpIntvt::call_api_get(&params, app.state()).await {
                        Ok(responses) => {
                            let mut lock = page.intvts.lock_mut();
                            lock.clear();
                            lock.extend(responses.into_iter().map(Rc::new));
                        }
                        Err(e) => {
                            app.alert_app_error(&e).await;
                        }
                    }
                }

                let intvt_ids = row.intvts.clone().unwrap_or_default()
                    .split('|').flat_map(|g| g.split('^').take(1))
                    .map(|s| s.parse::<u32>()).collect::<Result<Vec<u32>, std::num::ParseIntError>>()
                    .unwrap_or_default();
                {
                    let mut lock = page.intvt_ids.lock_mut();
                    lock.replace_cloned(intvt_ids);
                }
                let dlc_ids = row.dlcs.clone().unwrap_or_default()
                    .split('|').flat_map(|g| g.split('^').take(1))
                    .map(|s| s.parse::<u32>()).collect::<Result<Vec<u32>, std::num::ParseIntError>>()
                    .unwrap_or_default();
                {
                    let mut lock = page.dlc_ids.lock_mut();
                    lock.replace_cloned(dlc_ids);
                }
                page.changed.set(false);
            }),
        )
    }

    fn load_intvt(page: Rc<Self>, app: Rc<App>) {
        let id = page.fclist_id.lock_ref().parse::<u32>().unwrap_or_default();
        if let Some(focus_list) = page.focus_list.lock_ref().iter().find(|fcl| fcl.fclist_id == id).cloned() {
            app.async_load(
                true,
                clone!(app, page => async move {
                    let params = TmpParams {
                        smp_id: Some(focus_list.smp_id),
                        subgroup: focus_list.subgroup.to_owned(),
                        ..Default::default()
                    };
                    // GET `EndPoint::IpdTmpIntvt`
                    match TmpIntvt::call_api_get(&params, app.state()).await {
                        Ok(responses) => {
                            let mut lock = page.intvts.lock_mut();
                            lock.clear();
                            lock.extend(responses.into_iter().map(Rc::new));
                        }
                        Err(e) => {
                            app.alert_app_error(&e).await;
                        }
                    }
                }),
            );
        }
    }

    fn is_form_ready(&self) -> impl Signal<Item = bool> + use<> {
        map_ref! {
            let changed = self.changed.signal(),
            let fcnote_date = self.fcnote_date.signal_cloned().map(|s| !s.is_empty()) =>
            *changed && *fcnote_date
        }
    }

    fn finalized(page: Rc<Self>) -> Option<FocusNoteSave> {
        Some(FocusNoteSave {
            fcnote_id: zero_none(page.fcnote_id.get()),
            general_symptoms: str_some(page.general_symptoms.get_cloned()),
            fclist_id: page.fclist_id.lock_ref().parse::<u32>().ok(),
            assessment: str_some(page.assessment.get_cloned()),
            intvt_ids: page.intvt_ids.lock_ref().to_vec(),
            intvt_text: str_some(page.intvt_text.get_cloned()),
            evalution: str_some(page.evalution.get_cloned()),
            dlc_ids: page.dlc_ids.lock_ref().to_vec(),
            dlc_text: str_some(page.dlc_text.get_cloned()),
            other: str_some(page.other.get_cloned()),
            fcnote_date: date_8601(&page.fcnote_date.lock_ref()),
            fcnote_time: time_8601(&page.fcnote_time.lock_ref()),
            fcnote_patient_type: str_some(page.fcnote_patient_type.get_cloned()),
            version: zero_none(page.version.get()).unwrap_or(0),
        })
    }

    fn submit(page: Rc<Self>, app: Rc<App>) {
        let fcnote_date = page.fcnote_date.lock_ref();
        let fcnote_time = page.fcnote_time.lock_ref();

        if fcnote_date.is_empty() {
            if let Some(elm) = app.get_id("fcnote_date").and_then(|elm| elm.dyn_into::<HtmlInputElement>().ok()) {
                if let Err(e) = elm.focus() {
                    app.show_jsvalue_message(&e);
                }
            }
        } else if fcnote_time.is_empty() {
            if let Some(elm) = app.get_id("fcnote_time").and_then(|elm| elm.dyn_into::<HtmlInputElement>().ok()) {
                if let Err(e) = elm.focus() {
                    app.show_jsvalue_message(&e);
                }
            }
        } else if let Some(saver) = Self::finalized(page.clone()) {
            if let Some(patient) = page.patient.get_cloned() {
                app.async_load(
                    true,
                    clone!(app, page => async move {
                        let visit_type = patient.visit_type();
                        let saver_result_opt = match visit_type.clone() {
                            VisitTypeId::Ipd(an) | VisitTypeId::PreAdmit(an) => {
                                let params = FocusNoteSaveParams {
                                    hn: patient.hn(),
                                    ward: patient.ward(),
                                };
                                // POST `EndPoint::IpdFocusNoteAn`
                                Some(saver.call_api_post_ipd(&an, &params, app.state()).await)
                            }
                            VisitTypeId::OpdEr(_, opd_er_order_master_id) => {
                                // POST `EndPoint::OpdErFocusNoteId`
                                Some(saver.call_api_post_opd_er(opd_er_order_master_id, app.state()).await)
                            }
                            VisitTypeId::Visit(_) => None,
                        };
                        if let Some(saver_result) = saver_result_opt {
                            match saver_result {
                                Ok((id, responses)) => {
                                    app.alert_execute_responses(&responses, clone!(app => async move {
                                        let (is_ipd, is_pre_admit) = visit_type.is_ipd_and_is_pre_admit();
                                        // app.alert("บันทึกข้อมูลสำเร็จ");
                                        // update images
                                        if page.fcnote_id.get() == 0 && app.endpoint_is_allow(&Method::POST, &EndPoint::ImageUsage, is_pre_admit) {
                                            let (assess_im_usage, eva_im_usage) = if is_ipd {
                                                (ImageUsage::IpdFocusNoteAssessment, ImageUsage::IpdFocusNoteEvaluation)
                                            } else {
                                                (ImageUsage::OpdErFocusNoteAssessment, ImageUsage::OpdErFocusNoteEvaluation)
                                            };
                                            // POST `EndPoint::ImageUsage`
                                            page.assess_image_callback.lock_ref().post_images(assess_im_usage, id, app.clone()).await;
                                            page.eva_image_callback.lock_ref().post_images(eva_im_usage, id, app).await;
                                        }
                                        // clearing
                                        // page.fcnote_id.set_neq(id);
                                        page.changed.set_neq(false);
                                        page.parent_reload.set_neq(true);
                                        Self::new_form(page);
                                    })).await;
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

    fn delete(page: Rc<Self>, app: Rc<App>) {
        let visit_type_opt = page.patient.lock_ref().as_ref().map(|pt| pt.visit_type());
        if let (Some(fcnote_id), Some(version), Some(visit_type)) = (zero_none(page.fcnote_id.get()), zero_none(page.version.get()), visit_type_opt) {
            app.async_load(
                true,
                clone!(app, page => async move {
                    if app.confirm("ยืนยันรายการ").await {
                        let params = FocusNoteParams {
                            fcnote_id: Some(fcnote_id),
                            version: Some(version),
                            ..Default::default()
                        };
                        let result_opt = match visit_type {
                            VisitTypeId::Ipd(an) | VisitTypeId::PreAdmit(an) => {
                                // DELETE `EndPoint::IpdFocusNoteAn`
                                Some(FocusNote::call_api_delete_ipd(&an, &params, app.state()).await)
                            }
                            VisitTypeId::OpdEr(_, opd_er_order_master_id) => {
                                // DELETE `EndPoint::OpdErFocusNoteId`
                                Some(FocusNote::call_api_delete_opd_er(opd_er_order_master_id, &params, app.state()).await)
                            }
                            VisitTypeId::Visit(_) => None,
                        };
                        if let Some(result) = result_opt {
                            match result {
                                Ok(responses) => {
                                    app.alert_execute_responses(&responses, async move {
                                        page.parent_reload.set_neq(true);
                                    }).await;
                                }
                                Err(e) => {
                                    app.alert_app_error(&e).await;
                                }
                            }
                        }
                    }
                }),
            );
        }
    }

    pub fn render(page: Rc<Self>, app: Rc<App>) -> Dom {
        let fcnote_patient_types_select_options = match app.app_asset.lock_ref().as_ref() {
            Some(assets_arc) => {
                let asset = assets_arc.as_ref().to_owned();
                asset.fcnote_patient_type_select_options
            }
            None => Vec::new(),
        };
        let load_dlcs_once = Mutable::new(false);

        html!("div", {
            .future(load_dlcs_once.signal().for_each(clone!(app, page, load_dlcs_once => move |load_once| {
                if !load_once {
                    page.load_dlcs.set(true);
                    load_dlcs_once.set(true);
                }
                async {}
            })))
            .future(map_ref!{
                let busy = app.loader_is_loading(),
                let loaded = page.loaded_intvt.signal() =>
                !busy && !loaded
            }.for_each(clone!(app, page => move |ready| {
                if ready {
                    Self::load_intvt(page.clone(), app.clone());
                    page.loaded_intvt.set(true);
                }
                async {}
            })))
            .future(map_ref!{
                let busy = app.loader_is_loading(),
                let parent_opt = page.parent_focus_note.signal_cloned(),
                let note_opt = page.focus_note.signal_cloned() =>
                if !busy && match (parent_opt, note_opt) {
                    (Some(parent), Some(note)) => {
                        // clone a new one MUST have fclist_id
                        ((parent.fcnote_id == 0 && parent.fclist_id.is_some()) ||
                        // edit
                        parent.fcnote_id > 0) &&
                        parent != note
                    }
                    (Some(_), None) => true,
                    (None, _) => false,
                } { parent_opt.clone() } else { None }
            }.for_each(clone!(app, page => move |note_opt| {
                if let Some(focus_note) = note_opt {
                    page.focus_note.set(Some(focus_note.clone()));
                    Self::load_focus_note_row(focus_note, page.clone(), app.clone());
                }
                async {}
            })))
            .children([
                html!("div", {
                    .class("card")
                    .style("border-color","#B1ABB4")
                    .children([
                        html!("div", {
                            .class(class::CARD_HEAD)
                            .style("position","relative")
                            .children([
                                html!("i", {.class(class::FA_FILE)}),
                                html!("b", {.text(" ฟอร์มกรอกข้อมูล บันทึกการพยาบาล")}),
                            ])
                            .child_signal(page.fcnote_id.signal_cloned().map(clone!(page => move |id| {
                                zero_none(id).is_some().then(|| {
                                    html!("a", {
                                        .attr("href","#")
                                        .class("float-end")
                                        .child(html!("b", {.text("+Add")}))
                                        .event_with_options(&EventOptions::preventable(), clone!(page => move |event: events::Click| {
                                            event.prevent_default();
                                            Self::new_form(page.clone());
                                        }))
                                    })
                                })
                            })))
                        }),
                        html!("div", {
                            .class("card-body")
                            .children([
                                doms::form_inline(clone!(page => move |form| {
                                    form.children([
                                        doms::form_inline_group_sm(clone!(page => move |group| { group
                                            .children([
                                                html!("label", {
                                                    .attr("for", "fcnote_date")
                                                    .class("input-group-text")
                                                    .child(html!("b", {.text("วันที่")}))
                                                }),
                                                doms::date_picker(
                                                    page.fcnote_date.clone(),
                                                    page.changed.clone(), always(false), None,
                                                    |d| d.class(class::FLEX_GROW1).style("min-width","120px"),
                                                    |d| d.class(class::FORM_CTRL_ONLY_SM_R0),
                                                    |d| d.class(class::FORM_CTRL_ONLY_SM_R0).attr("id", "fcnote_date"),
                                                    |s| s, always(None),
                                                ),
                                                html!("label", {
                                                    .attr("for", "fcnote_time")
                                                    .class("input-group-text")
                                                    .child(html!("b", {.text("เวลา")}))
                                                }),
                                                doms::time_picker(
                                                    page.fcnote_time.clone(),
                                                    page.changed.clone(), always(false), None,
                                                    |d| d.class(class::FLEX_GROW1).style("min-width","95px"),
                                                    |d| d.class(class::FORM_CTRL_ONLY_SM_R0_L),
                                                    |d| d.class(class::FORM_CTRL_ONLY_SM_R0_L).attr("id", "fcnote_time"),
                                                    |s| s, always(None),
                                                ),
                                            ])
                                        })),
                                        doms::form_inline_group_sm(clone!(page => move |group| { group
                                            .children([
                                                html!("label", {
                                                    .attr("for", "fcnote_patient_type")
                                                    .class("input-group-text")
                                                    .child(html!("b", {.text("ประเภทผู้ป่วย")}))
                                                }),
                                                html!("select" => HtmlSelectElement, {
                                                    .class(class::FORM_SELECT_SM)
                                                    .attr("id", "fcnote_patient_type")
                                                    .child(html!("option", {.attr("value", "").text("--")}))
                                                    .children(fcnote_patient_types_select_options.iter().map(|option| {
                                                        doms::select_option_color(option, &page.fcnote_patient_type.lock_ref())
                                                    }))
                                                    .apply(mixins::string_value_select(page.fcnote_patient_type.clone(), page.changed.clone()))
                                                }),
                                            ])
                                        })),
                                    ])
                                })),
                                html!("hr"),
                                html!("div", {
                                    .class(class::ROW)
                                    .children([
                                        html!("label", {
                                            .class(class::COL_SM3_R_P0)
                                            .attr("for", "general_symptoms")
                                            .text("อาการผู้ป่วย")
                                        }),
                                        html!("div", {
                                            .class("col-sm-9")
                                            .child(html!("textarea" => HtmlTextAreaElement, {
                                                .class("form-control")
                                                .attr("id", "general_symptoms")
                                                .attr("rows", "3")
                                                .apply(mixins::textarea_value_auto_expand(page.general_symptoms.clone(), page.changed.clone()))
                                            }))
                                        }),
                                    ])
                                }),
                                html!("div", {
                                    .class(class::ROW)
                                    .children([
                                        html!("label", {
                                            .class(class::COL_SM3_R_P0)
                                            .attr("for", "fclist_select")
                                            .text("Focus")
                                        }),
                                        html!("div", {
                                            .class("col-sm-9")
                                            .child(html!("select" => HtmlSelectElement, {
                                                .class("form-select")
                                                .attr("id", "fclist_select")
                                                .child(html!("option", {.attr("value","").text("เลือก")}))
                                                .children_signal_vec(page.focus_list.signal_vec_cloned().map(|focus_list| {
                                                    html!("option", {
                                                        .attr("value", &focus_list.fclist_id.to_string())
                                                        .attr("data-smp", &focus_list.focus_id.to_string())
                                                        .text(&[focus_list.focus_name.clone().unwrap_or_default(), focus_list.focus_text.as_ref().map(|txt| [", ", txt].concat()).unwrap_or_default()].concat())
                                                    })
                                                }))
                                                .prop_signal("value", page.fclist_id.signal_cloned())
                                                .with_node!(element => {
                                                    .event(clone!(page => move |_: events::Change| {
                                                        let value = element.value();
                                                        // let fclist_id = value.parse::<u32>().unwrap_or_default();
                                                        page.fclist_id.set_neq(value);
                                                        // if let Some(row) = page.focus_list.lock_ref().iter().find(|r| r.fclist_id == fclist_id) {
                                                        //     page.focus_id.set_neq(row.focus_id);
                                                        // }
                                                        page.loaded_intvt.set(false);
                                                    }))
                                                })
                                                // .attr("onchange", "onchange_focus();")
                                            }))
                                        }),
                                    ])
                                }),
                                html!("div", {
                                    .class(class::ROW)
                                    .children([
                                        html!("div", {
                                            .class(class::COL_SM3_R_P0)
                                            .style("position","relative")
                                            .child(html!("label", {
                                                .attr("for", "assessment")
                                                .text("Assessment")
                                            }))
                                            .child_signal(page.is_ipd_and_is_pre_admit().map(clone!(app, page => move |(is_ipd, is_pre_admit)| {
                                                (if is_ipd {
                                                    app.endpoint_is_allow(&Method::GET, &EndPoint::IpdVitalSign, is_pre_admit)
                                                } else {
                                                    app.endpoint_is_allow(&Method::GET, &EndPoint::OpdErVitalSign, false)
                                                }).then(|| {
                                                    html!("button", {
                                                        .attr("type", "button")
                                                        .class(class::BTN_GRAY)
                                                        .attr("data-bs-toggle", "modal")
                                                        .attr("data-bs-target", "#vsSelectorModal")
                                                        .style("position","absolute")
                                                        .style("top","30px")
                                                        .style("right","50px")
                                                        .child(html!("i", {.class(class::FA_HEARTBEAT)}))
                                                        .event(clone!(page => move |_: events::Click| {
                                                            page.vs_selector_modal.set(Some(VsSelector::new(
                                                                false,
                                                                page.patient.clone(),
                                                                page.assessment.clone(),
                                                                page.changed.clone(),
                                                            )));
                                                        }))
                                                    })
                                                })
                                            })))
                                            .apply_if(app.endpoint_is_allow(&Method::GET, &EndPoint::LabHead, false), |dom| dom
                                                .child(html!("button", {
                                                    .attr("type", "button")
                                                    .class(class::BTN_GRAY)
                                                    .attr("data-bs-toggle", "modal")
                                                    .attr("data-bs-target", "#labSelectorModal")
                                                    .style("position","absolute")
                                                    .style("top","30px")
                                                    .style("right","0px")
                                                    .child(html!("i", {.class(class::FA_FLASK)}))
                                                    .event(clone!(page => move |_: events::Click| {
                                                        page.lab_selector_modal.set(Some(LabSelector::new(
                                                            false,
                                                            page.patient.clone(),
                                                            page.assessment.clone(),
                                                            page.changed.clone(),
                                                        )));
                                                    }))
                                                }))
                                            )
                                        }),
                                        html!("div", {
                                            .class("col-sm-9")
                                            .child(html!("textarea" => HtmlTextAreaElement, {
                                                .class("form-control")
                                                .attr("id", "assessment")
                                                .attr("rows", "3")
                                                .apply(mixins::textarea_value_auto_expand(page.assessment.clone(), page.changed.clone()))
                                            }))
                                            .child_signal(map_ref!{
                                                let fcnote_id = page.fcnote_id.signal(),
                                                let is_pre_admit = page.is_pre_admit() =>
                                                (*fcnote_id, *is_pre_admit)
                                            }.map(clone!(page, app => move |(fcnote_id, is_pre_admit)| {
                                                let vnan = page.patient.lock_ref().as_ref().map(|pt| pt.visit_type.vnan().to_owned());
                                                if fcnote_id == 0 {
                                                    app.endpoint_is_allow(&Method::POST, &EndPoint::ImageUsage, is_pre_admit).then(|| {
                                                        html!("div", {
                                                            .class("mt-1")
                                                            // POST `EndPoint::ImageUsage`
                                                            .child(ImageCpn::render("170px", ImageCpn::new_returning(
                                                                page.assess_image_callback.clone(),
                                                                page.patient.clone(),
                                                                vnan,
                                                                "FOCUS-NOTE-ASSESSMENT",
                                                            ), app.clone()))
                                                        })
                                                    })
                                                } else {
                                                    let im_usage = if page.patient.lock_ref().as_ref().map(|pt| pt.visit_type.is_ipd()).unwrap_or_default() {
                                                        ImageUsage::IpdFocusNoteAssessment
                                                    } else {
                                                        ImageUsage::OpdErFocusNoteAssessment
                                                    };
                                                    let is_editable = app.doctor_code().map(|code| code.as_str() == page.doctorcode.lock_ref().as_str()).unwrap_or_default();
                                                    Some(html!("div", {
                                                        .class("mt-1")
                                                        .child(ImageCpn::render("170px", ImageCpn::new_with_key(
                                                            im_usage,
                                                            fcnote_id,
                                                            is_editable,
                                                            page.patient.clone(),
                                                            vnan,
                                                            "FOCUS-NOTE-ASSESSMENT",
                                                        ), app.clone()))
                                                    }))
                                                }
                                            })))
                                        }),
                                    ])
                                }),
                                html!("div", {
                                    .class(class::ROW)
                                    .children([
                                        html!("label", {
                                            .class(class::COL_SM3_R_P0)
                                            .text("Intervention")
                                        }),
                                        html!("div", {
                                            .class("col-sm-9")
                                            .children_signal_vec(page.intvts.signal_vec_cloned().map(clone!(page => move |intvt| {
                                                let id = ["intvt_", &intvt.intvt_id.to_string()].concat();
                                                if intvt.intvt_status == Some(String::from("N")) {
                                                    page.intvt_ids.lock_mut().retain(|x| *x != intvt.intvt_id);
                                                }
                                                html!("div", {
                                                    .class(class::FORM_CHK_COL_SM12)
                                                    .children([
                                                        html!("input" => HtmlInputElement, {
                                                            .attr("type", "checkbox")
                                                            .attr("id", &id)
                                                            .class("form-check-input")
                                                            .attr("value", &intvt.intvt_id.to_string())
                                                            .apply_if(intvt.intvt_status == Some(String::from("N")), |dom| dom.attr("disabled",""))
                                                            .with_node!(element => {
                                                                .future(page.intvt_ids.signal_vec_cloned().to_signal_cloned().for_each(clone!(element, intvt => move |ids| {
                                                                    if ids.contains(&intvt.intvt_id) {
                                                                        element.set_checked(true);
                                                                    }
                                                                    async {}
                                                                })))
                                                                .event(clone!(page, intvt => move |_: events::Change| {
                                                                    if element.checked() {
                                                                        page.intvt_ids.lock_mut().push(intvt.intvt_id);
                                                                    } else {
                                                                        page.intvt_ids.lock_mut().retain(|x| *x != intvt.intvt_id);
                                                                    }
                                                                    page.changed.set_neq(true);
                                                                }))
                                                            })
                                                        }),
                                                        html!("label", {
                                                            .class("form-check-label")
                                                            .attr("for", &id)
                                                            // .style("user-select","none")
                                                            .apply_if(intvt.intvt_status == Some(String::from("N")), |dom| dom.text("(\u{274c} ยกเลิกการใช้งาน) "))
                                                            .children(doms::square_bracket_to_span(&intvt.intvt_name.clone().unwrap_or_default()))
                                                            // .text(&intvt.intvt_name.clone().unwrap_or_default())
                                                        })
                                                    ])
                                                })
                                            })))
                                            .child(html!("div", {
                                                .class(class::FORM_CHK_COL_SM12)
                                                .children([
                                                    html!("input" => HtmlInputElement, {
                                                        .attr("type", "checkbox")
                                                        .attr("id", "intvt_9999")
                                                        .class("form-check-input")
                                                        .attr("value", "9999")
                                                        .with_node!(element => {
                                                            .future(page.intvt_ids.signal_vec_cloned().to_signal_cloned().for_each(clone!(element => move |ids| {
                                                                element.set_checked(ids.contains(&9999));
                                                                async {}
                                                            })))
                                                            .event(clone!(page => move |_: events::Change| {
                                                                if element.checked() {
                                                                    page.intvt_ids.lock_mut().push(9999);
                                                                } else {
                                                                    page.intvt_ids.lock_mut().retain(|x| *x != 9999);
                                                                    page.intvt_text.set_neq(String::new());
                                                                }
                                                                page.changed.set_neq(true);
                                                            }))
                                                        })
                                                    }),
                                                    doms::label_check_for("intvt_9999","อื่นๆ"),
                                                ])
                                            }))
                                        }),
                                    ])
                                }),
                            ])
                            .child_signal(page.intvt_ids.signal_vec_cloned().to_signal_cloned().map(clone!(page => move |ids| {
                                ids.contains(&9999).then(|| {
                                    html!("div", {
                                        .class(class::ROW)
                                        .children([
                                            html!("label", {.attr("for", "intvt_text").class(class::COL_SM3_R_P0).text("ระบุ..")}),
                                            html!("div", {
                                                .class("col-md-9")
                                                .child(html!("textarea" => HtmlTextAreaElement, {
                                                    .class(class::FORM_CTRL_SM)
                                                    .attr("id", "intvt_text")
                                                    .attr("rows","2")
                                                    .apply(mixins::textarea_value_auto_expand(page.intvt_text.clone(), page.changed.clone()))
                                                }))
                                            })
                                        ])
                                    })
                                })
                            })))
                            .children([
                                html!("div", {
                                    .class(class::ROW)
                                    .children([
                                        html!("label", {
                                            .class(class::COL_SM3_R_P0)
                                            .text("Dailycare")
                                        }),
                                        html!("div", {
                                            .class("col-sm-9")
                                            .children([
                                                html!("div", {
                                                    .class("mb-1")
                                                    .child(html!("textarea" => HtmlTextAreaElement, {
                                                        .class("form-control")
                                                        .attr("rows","2")
                                                        .apply(mixins::textarea_value_auto_expand(page.dlc_text.clone(), page.changed.clone()))
                                                    }))
                                                }),
                                                html!("div", {
                                                    .class("accordion")
                                                    .attr("id", "dailycare-container")
                                                    .child(html!("div", {
                                                        .class("accordion-item")
                                                        .children([
                                                            html!("div", {
                                                                .class("accordion-header")
                                                                .child(html!("button", {
                                                                    .attr("type", "button")
                                                                    .class(class::ACCORDION_BTN_COLLAPSED_CYANS_P2)
                                                                    .attr("data-bs-toggle","collapse")
                                                                    .attr("data-bs-target","#dailycare-all")
                                                                    .attr("aria-expanded","false")
                                                                    .attr("aria-controls","dailycare-all")
                                                                    .text("แสดง Dailycares (เลือก ")
                                                                    .text_signal(page.dlc_ids.signal_vec_cloned().len().map(|i| i.to_string()))
                                                                    .text(" รายการ)")
                                                                }))
                                                            }),
                                                            html!("div", {
                                                                .attr("id", "dailycare-all")
                                                                .class(class::ACCORDION_COLLAPSE)
                                                                .attr("data-bs-parent","#dailycare-container")
                                                                .child(html!("div", {
                                                                   .class("accordion-body")
                                                                   .children_signal_vec(page.dlcs.signal_cloned().to_signal_vec().map(clone!(page => move |dlc| {
                                                                        let id = ["dlc_", &dlc.dlc_id.to_string()].concat();
                                                                        html!("div", {
                                                                            .class(class::FORM_CHK_COL_SM12)
                                                                            .children([
                                                                                html!("input" => HtmlInputElement, {
                                                                                    .attr("type", "checkbox")
                                                                                    .attr("id", &id)
                                                                                    .class("form-check-input")
                                                                                    .attr("value", &dlc.dlc_id.to_string())
                                                                                    .with_node!(element => {
                                                                                        .future(page.dlc_ids.signal_vec_cloned().to_signal_cloned().for_each(clone!(element, dlc => move |ids| {
                                                                                            element.set_checked(ids.contains(&dlc.dlc_id));
                                                                                            async {}
                                                                                        })))
                                                                                        .event(clone!(page => move |_: events::Change| {
                                                                                            if element.checked() {
                                                                                                page.dlc_ids.lock_mut().push(dlc.dlc_id);
                                                                                            } else {
                                                                                                page.dlc_ids.lock_mut().retain(|x| *x != dlc.dlc_id);
                                                                                            }
                                                                                            page.changed.set_neq(true);
                                                                                        }))
                                                                                    })
                                                                                }),
                                                                                doms::label_check_for_selectable(&id, &dlc.dlc_name),
                                                                            ])
                                                                        })
                                                                    })))
                                                                }))
                                                            }),
                                                        ])
                                                    }))
                                                }),
                                            ])
                                        }),
                                    ])
                                }),
                                // html!("br"),
                                html!("div", {
                                    .class(class::ROW)
                                    .children([
                                        html!("div", {
                                            .class(class::COL_SM3_R_P0)
                                            .style("position","relative")
                                            .children([
                                                html!("label", {
                                                    .attr("for", "evalution")
                                                    .text("Evaluation")
                                                }),
                                                html!("button", {
                                                    .attr("type", "button")
                                                    .class(class::BTN_GRAY)
                                                    .attr("data-bs-toggle", "modal")
                                                    .attr("data-bs-target", "#vsSelectorModal")
                                                    .style("position","absolute")
                                                    .style("top","30px")
                                                    .style("right","50px")
                                                    .child(html!("i", {.class(class::FA_HEARTBEAT)}))
                                                    .event(clone!(page => move |_: events::Click| {
                                                        page.vs_selector_modal.set(Some(VsSelector::new(
                                                            false,
                                                            page.patient.clone(),
                                                            page.evalution.clone(),
                                                            page.changed.clone(),
                                                        )));
                                                    }))
                                                }),
                                                html!("button", {
                                                    .attr("type", "button")
                                                    .class(class::BTN_GRAY)
                                                    .attr("data-bs-toggle", "modal")
                                                    .attr("data-bs-target", "#labSelectorModal")
                                                    .style("position","absolute")
                                                    .style("top","30px")
                                                    .style("right","0px")
                                                    .child(html!("i", {.class(class::FA_FLASK)}))
                                                    .event(clone!(page => move |_: events::Click| {
                                                        page.lab_selector_modal.set(Some(LabSelector::new(
                                                            false,
                                                            page.patient.clone(),
                                                            page.evalution.clone(),
                                                            page.changed.clone(),
                                                        )));
                                                    }))
                                                }),
                                            ])
                                        }),
                                        html!("div", {
                                            .class("col-sm-9")
                                            .child(html!("textarea" => HtmlTextAreaElement, {
                                                .class("form-control")
                                                .attr("id", "evalution")
                                                .attr("rows", "3")
                                                .apply(mixins::textarea_value_auto_expand(page.evalution.clone(), page.changed.clone()))
                                            }))
                                            .child_signal(map_ref!{
                                                let fcnote_id = page.fcnote_id.signal(),
                                                let is_pre_admit = page.is_pre_admit() =>
                                                (*fcnote_id, *is_pre_admit)
                                            }.map(clone!(page, app => move |(fcnote_id, is_pre_admit)| {
                                                let vnan = page.patient.lock_ref().as_ref().map(|pt| pt.visit_type.vnan().to_owned());
                                                if fcnote_id == 0 {
                                                    app.endpoint_is_allow(&Method::POST, &EndPoint::ImageUsage, is_pre_admit).then(|| {
                                                        html!("div", {
                                                            .class("mt-1")
                                                            // POST `EndPoint::ImageUsage`
                                                            .child(ImageCpn::render("170px", ImageCpn::new_returning(
                                                                page.eva_image_callback.clone(),
                                                                page.patient.clone(),
                                                                vnan,
                                                                "FOCUS-NOTE-EVALUATION",
                                                            ), app.clone()))
                                                        })
                                                    })
                                                } else {
                                                    let im_usage = if page.patient.lock_ref().as_ref().map(|pt| pt.visit_type.is_ipd()).unwrap_or_default() {
                                                        ImageUsage::IpdFocusNoteEvaluation
                                                    } else {
                                                        ImageUsage::OpdErFocusNoteEvaluation
                                                    };
                                                    let is_editable = app.doctor_code().map(|code| code.as_str() == page.doctorcode.lock_ref().as_str()).unwrap_or_default();
                                                    Some(html!("div", {
                                                        .class("mt-1")
                                                        .child(ImageCpn::render("170px", ImageCpn::new_with_key(
                                                            im_usage,
                                                            fcnote_id,
                                                            is_editable,
                                                            page.patient.clone(),
                                                            vnan,
                                                            "FOCUS-NOTE-EVALUATION",
                                                        ), app.clone()))
                                                    }))
                                                }
                                            })))
                                        }),
                                    ])
                                }),
                                // html!("br"),
                                html!("div", {
                                    .class(class::ROW)
                                    .children([
                                        html!("label", {
                                            .class(class::COL_SM3_R_P0)
                                            .attr("for", "other")
                                            .text("อื่น ๆ")
                                        }),
                                        html!("div", {
                                            .class("col-sm-9")
                                            .child(html!("textarea" => HtmlTextAreaElement, {
                                                .class("form-control")
                                                .attr("id", "other")
                                                .attr("rows", "3")
                                                .apply(mixins::textarea_value_auto_expand(page.other.clone(), page.changed.clone()))
                                            }))
                                        }),
                                    ])
                                }),
                                html!("hr"),
                                html!("div", {
                                    .class("row")
                                    .child(html!("div", {
                                        .class("col-sm-12")
                                        .child(html!("button", {
                                            .attr("type", "button")
                                            .class(class::BTN_FR_GRAY)
                                            .child(html!("i", {.class(class::FA_UNDO)}))
                                            .text(" ยกเลิกการแก้ไข")
                                            .event(clone!(app, page => move |_: events::Click| {
                                                if let Some(focus_note) = page.parent_focus_note.get_cloned() {
                                                    Self::load_focus_note_row(focus_note, page.clone(), app.clone());
                                                } else {
                                                    Self::new_form(page.clone());
                                                }
                                            }))
                                            //.attr("onclick", "cancel_focusnote()")
                                        }))
                                        .child_signal(page.is_ipd_and_is_pre_admit().map(clone!(app, page => move |(is_ipd, is_pre_admit)| {
                                            (if is_ipd {
                                                app.endpoint_is_allow(&Method::POST, &EndPoint::IpdFocusNoteAn, is_pre_admit)
                                            } else {
                                                app.endpoint_is_allow(&Method::POST, &EndPoint::OpdErFocusNoteId, false)
                                            }).then(|| {
                                                html!("button" => HtmlButtonElement, {
                                                    .attr("type", "button")
                                                    .class(class::BTN_FR_L)
                                                    .class_signal("btn-primary", page.is_form_ready())
                                                    .class_signal("btn-secondary", not(page.is_form_ready()))
                                                    .child(html!("i", {.class(class::FA_SAVE)}))
                                                    .text(" บันทึก")
                                                    .apply(mixins::click_with_loader_checked_or_true_disable_signal(clone!(app, page => move || {
                                                        Self::submit(page.clone(), app.clone());
                                                    }), not(page.is_form_ready()), app.state()))
                                                    //.attr("onclick", "save_focusnote_form();")
                                                })
                                            })
                                        })))
                                        .child_signal(map_ref!{
                                            let (is_ipd, is_pre_admit) = page.is_ipd_and_is_pre_admit(),
                                            let fcnote_id = page.fcnote_id.signal_cloned() =>
                                            (*fcnote_id, *is_ipd, *is_pre_admit)
                                        }.map(clone!(app, page => move |(id, is_ipd, is_pre_admit)| {
                                            (id > 0
                                            && if is_ipd {
                                                app.endpoint_is_allow(&Method::DELETE, &EndPoint::IpdFocusNoteAn, is_pre_admit)
                                            } else {
                                                app.endpoint_is_allow(&Method::DELETE, &EndPoint::OpdErFocusNoteId, false)
                                            }).then(|| {
                                                html!("button" => HtmlButtonElement, {
                                                    .attr("type", "button")
                                                    .class(class::BTN_RED)
                                                    //.attr("id", "btn_delete_focusnote")
                                                    .child(html!("i", {.class(class::FA_TRASH)}))
                                                    .text(" ลบ")
                                                    .apply(mixins::click_with_loader_checked(clone!(app, page => move || {
                                                        Self::delete(page.clone(), app.clone());
                                                    }), app.state()))
                                                    // .attr("onclick", "delete_focuslist_form()")
                                                })
                                            })
                                        })))
                                    }))
                                }),
                            ])
                        }),
                    ])
                }),
                html!("br"),
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
}
