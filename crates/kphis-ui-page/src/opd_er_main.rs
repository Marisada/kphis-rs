use dominator::{Dom, EventOptions, clone, events, html, window_size, with_node};
use futures_signals::{
    map_ref,
    signal::{Mutable, Signal, SignalExt, always},
};
use std::rc::Rc;
use web_sys::{HtmlButtonElement, HtmlSelectElement, HtmlTextAreaElement};

use kphis_model::{
    LEFT_PANEL_MIN_WIDTH, SCREEN_WIDTH_EXTRA,
    antibiogram::Antibiograms,
    endpoint::EndPoint,
    fetch::{Method, call_api_get_exists_key_id},
    opd_er::order_master::{OpdErOrderMaster, OpdErOrderMasterSave},
    report::SystemReport,
    route::Route,
    tab::Tab,
};
use kphis_ui_app::App;
use kphis_ui_component::{
    document::DocumentCpn,
    emr::EmrCpn,
    gadget::{aside_resizer::AsideResizerCpn, searchbox::opd_visit::OpdVisitSearchboxCpn},
    index_plan::IndexPlanCpn,
    io::IoCpn,
    lab::LabCpn,
    med_reconcile::med_reconcile_main::MedReconcileCpn,
    nurse_note::nurse_note_main::NurseNoteCpn,
    opd_er_emergency::OpdErEmergencyCpn,
    order::OrderCpn,
    refer_out::ReferOutCpn,
    show_patient_main::ShowPatientMainCpn,
    vital_sign::vital_sign_main::VitalSignCpn,
    xray::XrayCpn,
};
use kphis_ui_core::{class, doms, mixins};
use kphis_util::{
    datetime::{JsTime, date_8601, js_now, time_8601},
    util::{str_some, zero_none},
};

/// - GET `EndPoint::OpdErOrderMasterId`
/// - GET `EndPoint::OpdErShowPatientMainVn` (ShowPatientMainCpn)
/// - GET `EndPoint::OpdErShowPatientMainId` (ShowPatientMainCpn)
/// - POST `EndPoint::OpdErOrderMaster` (guarded, remove 'บันทึก' btn)
/// - GET `EndPoint::ExistsKeyId` (guarded, no alert badges)
/// - GET `EndPoint::EmrDateHn` (EmrCpn, guarded, remove 'EMR' tab)
/// - GET `EndPoint::EmrVisitVn` (EmrCpn, guarded, remove 'EMR' tab)
/// - GET `EndPoint::OpdErFocusNoteId` (NurseNoteCpn, guarded, remove 'Nursing Progress Note' btn)
/// - GET `EndPoint::OpdErIoDateId` (IoCpn, guarded, remove 'I/O' tab)
/// - GET `EndPoint::OpdErIo` (IoCpn, guarded, remove 'I/O' tab)
/// - GET `EndPoint::OpdErOrderMasterCheckVn` (OrderCpn, guarded, remove 'Order' tab)
/// - GET `EndPoint::OpdErOrderOrder` (OrderCpn, guarded, remove 'Order' tab)
/// - GET `EndPoint::OpdErOrderProgressNote` (OrderCpn, guarded, remove 'Order' tab)
/// - GET `EndPoint::OpdErHisMedVn` (opd-er only) (OrderCpn, guarded, remove 'Order' tab)
/// - GET `EndPoint::OpdErMedReconcile` (OrderCpn/MedReconcileCpn, guarded, remove 'Order' and 'Med Reconciliation' tab)
/// - GET `EndPoint::OpdErMedicalHistory` (OpdErEmergencyCpn, guarded, remove 'ประวัติผู้ป่วย' tab)
/// - GET `EndPoint::OpdErMedicalHistoryTrauma` (OpdErEmergencyCpn, guarded, remove 'ประวัติผู้ป่วย' tab)
/// - GET `EndPoint::OpdErMedicalHistoryAllergy` (OpdErEmergencyCpn, guarded, remove 'ประวัติผู้ป่วย' tab)
/// - GET `EndPoint::OpdErMedicalHistoryScreen` (OpdErEmergencyCpn, guarded, remove 'ประวัติผู้ป่วย' tab)
/// - GET `EndPoint::OpdErMedicalHistoryScan` (OpdErEmergencyCpn, guarded, remove 'ประวัติผู้ป่วย' tab)
/// - GET `EndPoint::OpdErMedicalHistoryConsult` (OpdErEmergencyCpn, guarded, remove 'ประวัติผู้ป่วย' tab)
/// - GET `EndPoint::OpdErMedicalHistoryFt` (OpdErEmergencyCpn, guarded, remove 'ประวัติผู้ป่วย' tab)
/// - GET `EndPoint::OpdErOrderItem` (IndexPlanCpn, guarded, remove 'Nurse Planning' tab)
/// - GET `EndPoint::OpdErVitalSign` (VitalSignCpn, guarded, remove 'Vital Sign' tab)
/// - GET `EndPoint::LabHead` (LabCpn, guarded, remove 'Lab' tab)
/// - GET `EndPoint::XrayReportHn` (XrayCpn, guarded, rempve 'X-Ray' btn)
/// - GET `EndPoint::XrayPacsXn` (XrayCpn, guarded, rempve 'X-Ray' btn)
/// - GET `EndPoint::SearchBoxOpdVisitModeText` (OpdVisitSearchboxCpn, guarded, remove search-patient btn)
#[derive(Clone, Default)]
pub struct OpdErMainPage {
    loaded_master: Mutable<bool>,
    active_tab: Mutable<Tab>,

    pub view_by: Mutable<String>,
    focused_id: Mutable<u32>,
    pub opd_er_order_master_id: Mutable<u32>,

    pub pass_24_hour_from_dch: Mutable<String>,
    order_doctor_name: Mutable<String>,
    pub vn: Mutable<String>,
    pub hn: Mutable<String>,
    pub an: Mutable<String>,
    opd_visit_detail: Mutable<String>,
    bedno: Mutable<String>,
    er_patient_status_id: Mutable<String>,
    er_dch_type_id: Mutable<String>,
    discharge_date: Mutable<String>,
    discharge_time: Mutable<String>,
    note: Mutable<String>,
    // pass_24_hour_from_dch_display: Mutable<String>,
    changed: Mutable<bool>,
    display_patient_searchbox: Mutable<bool>,
    tab_order_loaded: Mutable<Option<Mutable<bool>>>,
    patient: Mutable<Rc<ShowPatientMainCpn>>,

    loaded_med_reconciliation_exists_spinner: Mutable<bool>,
    show_med_reconciliation_exists_spinner: Mutable<bool>,

    loaded_med_reconciliation_has_data: Mutable<bool>,
    show_med_reconciliation_has_data: Mutable<bool>,

    loaded_lab_unread_exists_spinner: Mutable<bool>,
    show_lab_unread_exists_spinner: Mutable<bool>,

    loaded_lab_unreport_has_data: Mutable<bool>,
    show_lab_unreport_has_data: Mutable<bool>,

    loaded_xray_unread_exists_spinner: Mutable<bool>,
    show_xray_unread_exists_spinner: Mutable<bool>,

    loaded_antibiograms: Mutable<bool>,
    antibiograms: Mutable<Vec<Rc<Antibiograms>>>,
}

impl OpdErMainPage {
    pub fn new(view_by: String, opd_er_order_master_id: u32, tab: Tab, focused_id: u32) -> Rc<Self> {
        Rc::new(Self {
            active_tab: Mutable::new(tab),
            view_by: Mutable::new(view_by),
            focused_id: Mutable::new(focused_id),
            opd_er_order_master_id: Mutable::new(opd_er_order_master_id),
            ..Default::default()
        })
    }

    fn is_discharged(&self) -> impl Signal<Item = bool> + use<> {
        self.er_patient_status_id.signal_cloned().map(|id| id == "7")
    }

    fn is_view_by_nurse(&self) -> impl Signal<Item = bool> + use<> {
        self.view_by.signal_cloned().map(|view_by| view_by == "nurse")
    }

    fn is_view_by_doctor_or_nurse(&self) -> impl Signal<Item = bool> + use<> {
        self.view_by.signal_cloned().map(|view_by| ["doctor", "nurse"].contains(&view_by.as_str()))
    }

    fn is_readonly(&self) -> impl Signal<Item = bool> + use<> {
        self.pass_24_hour_from_dch.signal_cloned().map(|readonly| readonly == "Y")
    }

    fn is_readonly_or_not_view_by_doctor_or_nurse(&self) -> impl Signal<Item = bool> + use<> {
        map_ref! {
            let is_dn = self.is_view_by_doctor_or_nurse(),
            let read_only = self.is_readonly() =>
            !is_dn || *read_only
        }
    }

    fn load_master(page: Rc<Self>, app: Rc<App>) {
        app.async_load(
            true,
            clone!(app => async move {
                let opd_er_order_master_id = page.opd_er_order_master_id.get();
                // GET `EndPoint::OpdErOrderMasterId`
                match OpdErOrderMaster::call_api_get(opd_er_order_master_id, app.state()).await {
                    Ok(response) => {
                        if let Some(master) = response {
                            let hn = master.hn.clone().unwrap_or_default();
                            let vn = master.vn.clone().unwrap_or_default();
                            let opd_visit_detail = if master.vn.is_some() {
                                [
                                    "VN: ", &vn,
                                    " HN: ", &hn,
                                    &master.ptname.as_ref().map(|ptname| [" (", ptname, ")"].concat()).unwrap_or_default()
                                ].concat()
                            } else {
                                String::new()
                            };

                            // reload patient
                            page.order_doctor_name.set([if master.order_doctor_is_intern.unwrap_or_default() {"(Intern) "} else {""}, &master.order_doctor_name.unwrap_or_default()].concat());
                            page.pass_24_hour_from_dch.set(master.pass_24_hour_from_dch.unwrap_or_default());
                            page.bedno.set(master.bedno.unwrap_or_default().to_string());
                            page.er_patient_status_id.set(master.er_patient_status_id.map(|id| id.to_string()).unwrap_or_default());
                            page.er_dch_type_id.set(master.er_dch_type_id.map(|id| id.to_string()).unwrap_or_default());
                            page.discharge_date.set(master.discharge_date.map(|d| d.to_string()).unwrap_or_default());
                            page.discharge_time.set(master.discharge_time.map(|t| t.to_string()).unwrap_or_default());
                            page.note.set(master.note.unwrap_or_default());
                            page.vn.set(vn);
                            page.hn.set(hn);
                            // page.an.set(master.an.clone().unwrap_or_default());
                            page.opd_visit_detail.set(opd_visit_detail);
                            // page.pass_24_hour_from_dch_display

                            page.changed.set_neq(false);
                            Self::reload_patient(page);
                        }
                    }
                    Err(e) => {
                        app.alert_app_error(&e).await;
                    }
                }
            }),
        )
    }

    fn load_antibiograms(page: Rc<Self>, app: Rc<App>) {
        app.async_load(
            true,
            clone!(app, page => async move {
                match Antibiograms::get(app.state()).await {
                    Ok(antibiograms) => {
                        page.antibiograms.set(antibiograms.into_iter().map(Rc::new).collect());
                    }
                    Err(e) => {
                        app.alert_app_error(&e).await;
                    }
                }
            }),
        )
    }

    pub fn save_master(page: Rc<Self>, app: Rc<App>) {
        let save = OpdErOrderMasterSave {
            opd_er_order_master_id: zero_none(page.opd_er_order_master_id.get()),
            vn: str_some(page.vn.get_cloned()),
            // an: str_some(page.an.get_cloned()),
            note: str_some(page.note.get_cloned()),
            bedno: page.bedno.lock_ref().parse::<u32>().ok(),
            er_patient_status_id: page.er_patient_status_id.lock_ref().parse::<u32>().ok(),
            er_dch_type_id: page.er_dch_type_id.lock_ref().parse::<u32>().ok(),
            discharge_date: date_8601(&page.discharge_date.lock_ref()),
            discharge_time: time_8601(&page.discharge_time.lock_ref()),
        };

        app.async_load(
            true,
            clone!(app, page => async move {
                // POST `EndPoint::OpdErOrderMaster`
                match save.call_api_post(app.state()).await {
                    Ok((id, response)) => {
                        app.alert_execute_response(&response, async move {
                            // set id for opd_show_patiet_main to re-render
                            page.opd_er_order_master_id.set(id);
                            page.changed.set_neq(false);
                            // reload order
                            if let Some(order_loaded) = page.tab_order_loaded.lock_ref().as_ref() {
                                order_loaded.set(false);
                            }

                            Self::reload_patient(page);
                        }).await;
                    }
                    Err(e) => {
                        app.alert_app_error(&e).await;
                    }
                }
            }),
        );
    }

    fn reload_patient(page: Rc<Self>) {
        let patient = page.patient.lock_ref();
        if page.vn.get_cloned() != patient.vn.get_cloned().unwrap_or_default() {
            patient.loaded.set(false);
        }
    }

    fn loaded_med_reconciliation_exists(page: Rc<Self>, app: Rc<App>) {
        if let Some(opd_er_order_master_id) = zero_none(page.opd_er_order_master_id.get()) {
            app.async_load(
                true,
                clone!(app, page => async move {
                    // GET `EndPoint::ExistsKeyId`
                    match call_api_get_exists_key_id("opd-er-med-reconcile/", &opd_er_order_master_id.to_string(), app.state()).await {
                        Ok(exs) => {
                            page.show_med_reconciliation_has_data.set_neq(exs);
                        }
                        Err(e) => {
                            app.alert_app_error(&e).await;
                        }
                    }
                }),
            )
        }
    }

    fn loaded_med_reconciliation_doctor_unconfirm_exists(page: Rc<Self>, app: Rc<App>) {
        if let Some(opd_er_order_master_id) = zero_none(page.opd_er_order_master_id.get()) {
            app.async_load(
                true,
                clone!(app, page => async move {
                    // GET `EndPoint::ExistsKeyId`
                    match call_api_get_exists_key_id("opd-er-med-reconcile-dr-unconfirm/", &opd_er_order_master_id.to_string(), app.state()).await {
                        Ok(exists) => {
                            page.show_med_reconciliation_exists_spinner.set_neq(exists);
                        }
                        Err(e) => {
                            app.alert_app_error(&e).await;
                        }
                    }
                }),
            )
        }
    }

    fn loaded_lab_unreport_exists(page: Rc<Self>, app: Rc<App>) {
        if let Some(vn) = str_some(page.vn.get_cloned()) {
            app.async_load(
                true,
                clone!(app, page => async move {
                    // GET `EndPoint::ExistsKeyId`
                    match call_api_get_exists_key_id("lab-unreport/", &vn, app.state()).await {
                        Ok(exists) => {
                            page.show_lab_unreport_has_data.set_neq(exists);
                        }
                        Err(e) => {
                            app.alert_app_error(&e).await;
                        }
                    }
                }),
            )
        }
    }

    fn loaded_lab_unread_exists(page: Rc<Self>, app: Rc<App>) {
        if let Some(vn) = str_some(page.vn.get_cloned()) {
            app.async_load(
                true,
                clone!(app, page => async move {
                    // GET `EndPoint::ExistsKeyId`
                    match call_api_get_exists_key_id("lab-unread/", &vn, app.state()).await {
                        Ok(exs) => {
                            page.show_lab_unread_exists_spinner.set_neq(exs);
                        }
                        Err(e) => {
                            app.alert_app_error(&e).await;
                        }
                    }
                }),
            )
        }
    }

    fn loaded_xray_unread_exists(page: Rc<Self>, app: Rc<App>) {
        if let Some(vn) = str_some(page.vn.get_cloned()) {
            app.async_load(
                true,
                clone!(app, page => async move {
                    // GET `EndPoint::ExistsKeyId`
                    match call_api_get_exists_key_id("opd-er-xray-unread/", &vn, app.state()).await {
                        Ok(exs) => {
                            page.show_xray_unread_exists_spinner.set_neq(exs);
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
        app.set_title("KPHIS - Opd-Er Main Page");

        let (patient_main, hn) = match zero_none(page.opd_er_order_master_id.get()) {
            Some(id) => {
                let show_patient_main = ShowPatientMainCpn::new_with_id(id);
                let hn = show_patient_main.hn.clone();
                let patient_main = ShowPatientMainCpn::render(false, show_patient_main.clone(), app.clone());
                page.patient.set(show_patient_main);
                (patient_main, hn)
            }
            None => (
                html!("div", {
                    .class(class::ALERT_GRAY)
                    .attr("role", "alert")
                    .text("ยังไม่มีข้อมูลแพ้ยาเนื่องจากยังไม่ได้เลือก Visit")
                }),
                Mutable::new(String::new()),
            ),
        };

        html!("div", {
            .future(map_ref!(
                let busy = app.loader_is_loading(),
                let loaded = page.loaded_master.signal() =>
                !busy && !loaded
            ).for_each(clone!(app, page => move |ready| {
                if ready {
                    Self::load_master(page.clone(), app.clone());
                    page.loaded_master.set_neq(true);
                }
                async {}
            })))
            // patient panel
            .child(patient_main)
            .child_signal(window_size().map(|ws| ws.width < SCREEN_WIDTH_EXTRA).dedupe().map(move |is_not_wide| {
                Some(if is_not_wide {
                    Self::render_tabs(page.clone(), app.clone())
                } else {
                    // aside_resizer
                    let report_selected = SystemReport::new(&app.report_select.lock_ref());
                    AsideResizerCpn::render(
                        Self::render_tabs(page.clone(), app.clone()),
                        Some((false, page.patient.lock_ref().patient.clone())),
                        AsideResizerCpn::new(
                            Mutable::new(report_selected), Mutable::new(false),
                            Mutable::new(None), Mutable::new(false),
                            page.vn.clone(), hn.clone(), SystemReport::opd_er_set(),
                            "opd-er-main",
                            Some(page.loaded_lab_unread_exists_spinner.clone()),
                            Some(page.loaded_xray_unread_exists_spinner.clone()),
                            app.clone(),
                        ),
                        app.clone(),
                    )
                })
            }))
        })
    }

    pub fn render_tabs(page: Rc<Self>, app: Rc<App>) -> Dom {
        let (er_bed_select_options, er_patient_status_select_options, er_dch_type_select_options) = app
            .app_asset
            .lock_ref()
            .as_ref()
            .map(|asset| {
                (
                    asset.er_bed_select_options.clone(),
                    asset.er_patient_status_select_options.clone(),
                    asset.er_dch_type_select_options.clone(),
                )
            })
            .unwrap_or_default();

        let allow_medrec = app.endpoint_is_allow(&Method::GET, &EndPoint::OpdErMedReconcile, false);
        let allow_lab = app.endpoint_is_allow(&Method::GET, &EndPoint::LabHead, false);
        let allow_xray = app.endpoint_is_allow(&Method::GET, &EndPoint::XrayReportHn, false) && app.endpoint_is_allow(&Method::GET, &EndPoint::XrayPacsXn, false);

        html!("div", {
            .future(map_ref!{
                let busy = app.loader_is_loading(),
                let view_by = page.view_by.signal_cloned(),
                let loaded = page.loaded_antibiograms.signal() =>
                view_by == "doctor" && !busy && !loaded
            }.for_each(clone!(app, page => move |ready| {
                if ready {
                    Self::load_antibiograms(page.clone(), app.clone());
                    page.loaded_antibiograms.set(true);
                }
                async {}
            })))
            .apply_if(allow_medrec, |dom| dom
                .future(map_ref!{
                    let busy = app.loader_is_loading(),
                    let view_by = page.view_by.signal_cloned(),
                    let loaded = page.loaded_med_reconciliation_has_data.signal() =>
                    ["doctor","nurse","pharmacist"].contains(&view_by.as_str()) && !busy && !loaded
                }.for_each(clone!(app, page => move |ready| {
                    if ready {
                        Self::loaded_med_reconciliation_exists(page.clone(), app.clone());
                        page.loaded_med_reconciliation_has_data.set(true);
                    }
                    async {}
                })))
                .future(map_ref!{
                    let busy = app.loader_is_loading(),
                    let view_by = page.view_by.signal_cloned(),
                    let loaded = page.loaded_med_reconciliation_exists_spinner.signal() =>
                    view_by == "doctor" && !busy && !loaded
                }.for_each(clone!(app, page => move |ready| {
                    if ready {
                        Self::loaded_med_reconciliation_doctor_unconfirm_exists(page.clone(), app.clone());
                        page.loaded_med_reconciliation_exists_spinner.set(true);
                    }
                    async {}
                })))
            )
            .apply_if(allow_lab, |dom| dom
                .future(map_ref!{
                    let busy = app.loader_is_loading(),
                    let view_by = page.view_by.signal_cloned(),
                    let loaded = page.loaded_lab_unreport_has_data.signal() =>
                    ["doctor","nurse"].contains(&view_by.as_str()) && !busy && !loaded
                }.for_each(clone!(app, page => move |ready| {
                    if ready {
                        Self::loaded_lab_unreport_exists(page.clone(), app.clone());
                        page.loaded_lab_unreport_has_data.set(true);
                    }
                    async {}
                })))
                .future(map_ref!{
                    let busy = app.loader_is_loading(),
                    let view_by = page.view_by.signal_cloned(),
                    let loaded = page.loaded_lab_unread_exists_spinner.signal() =>
                    view_by == "doctor" && !busy && !loaded
                }.for_each(clone!(app, page => move |ready| {
                    if ready {
                        Self::loaded_lab_unread_exists(page.clone(), app.clone());
                        page.loaded_lab_unread_exists_spinner.set(true);
                    }
                    async {}
                })))
            )
            .apply_if(allow_xray, |dom| dom
                .future(map_ref!{
                    let busy = app.loader_is_loading(),
                    let view_by = page.view_by.signal_cloned(),
                    let loaded = page.loaded_xray_unread_exists_spinner.signal() =>
                    ["doctor","nurse"].contains(&view_by.as_str()) && !busy && !loaded
                }.for_each(clone!(app, page => move |ready| {
                    if ready {
                        Self::loaded_xray_unread_exists(page.clone(), app.clone());
                        page.loaded_xray_unread_exists_spinner.set(true);
                    }
                    async {}
                })))
            )
            .class(class::CONF_B)
            .attr("id", "opd-er-main")
            .style("min-width",LEFT_PANEL_MIN_WIDTH)
            .children([
                doms::form_inline(clone!(app, page => move |form| { form
                    .children([
                        doms::form_inline_group_sm(clone!(page => move |group| { group
                            .children([
                                doms::label_group_for("order_doctor_name","ผู้บันทึก"),
                                html!("input", {
                                    .attr("type", "text")
                                    .class(class::FORM_CTRL_SM)
                                    .attr("id", "order_doctor_name")
                                    .attr("readonly", "readonly")
                                    .style("cursor","pointer")
                                    .prop_signal("value", page.order_doctor_name.signal_cloned())
                                }),
                            ])
                        })),
                        doms::form_inline_group_sm(clone!(app, page => move |group| { group
                            .attr("id", "vn-input-group")
                            .children([
                                doms::label_group_for("opd_visit_detail","Visit"),
                                html!("input", {
                                    .attr("type", "text")
                                    .class(class::FORM_CTRL_SM)
                                    .attr("id", "opd_visit_detail")
                                    .attr("size", "60")
                                    .attr("readonly", "readonly")
                                    .style("cursor","pointer")
                                    .prop_signal("value",page.opd_visit_detail.signal_cloned())
                                }),
                            ])
                            // OPD_ER_ORDER_EDIT
                            .apply_if(app.endpoint_is_allow(&Method::GET, &EndPoint::SearchBoxOpdVisitModeText, false), |dom| dom
                                .child_signal(page.is_readonly_or_not_view_by_doctor_or_nurse().map(clone!(page => move |is_readonly| {
                                    (!is_readonly).then(|| {
                                        html!("button", {
                                            .class(class::BTN_SM_GRAY)
                                            .attr("type", "button")
                                            .child(html!("i", {.class(class::FA_SEARCH)}))
                                            .event(clone!(page => move |_: events::Click| {
                                                page.display_patient_searchbox.set_neq(true);
                                            }))
                                            // .attr("onclick", "onclick_search_vn_button(event, this)")
                                        })
                                    })
                                })))
                            )
                            .child_signal(page.is_readonly_or_not_view_by_doctor_or_nurse().map(clone!(page => move |is_readonly| {
                                (!is_readonly).then(|| {
                                    html!("button", {
                                        .class(class::BTN_SM_RED)
                                        .attr("type", "button")
                                        .child(html!("i", {.class(class::FA_X)}))
                                        .event(clone!(page => move |_: events::Click| {
                                            page.vn.set_neq(String::new());
                                            page.opd_visit_detail.set_neq(String::new());
                                            page.changed.set_neq(true);
                                        }))
                                        // .attr("onclick", "onclick_clear_vn_button(event, this)")
                                    })
                                })
                            })))
                        })),
                        doms::form_inline_group_sm(clone!(page => move |group| { group
                            .children([
                                doms::label_group_for("bedno","เตียง"),
                                html!("select" => HtmlSelectElement, {
                                    .class(class::FORM_SELECT_SM)
                                    // .style("width","150px")
                                    .attr("id", "bedno")
                                    .prop_signal("style", page.bedno.signal_cloned().map(clone!(er_bed_select_options => move |bedno| {
                                        er_bed_select_options.iter()
                                            .find(|option| option.key.to_owned().as_str() == bedno)
                                            .map(|option| ["background-color:",&option.color,";color:black;font-weight:bold;"].concat())
                                            .unwrap_or_default()
                                    })))
                                    .with_node!(element => {
                                        .future(page.is_readonly_or_not_view_by_doctor_or_nurse().for_each(move |is_readonly| {
                                            element.set_disabled(is_readonly);
                                            async {}
                                        }))
                                    })
                                    .children(er_bed_select_options.iter().map(|option| {
                                        doms::select_option_color(option, &page.bedno.lock_ref())
                                    }))
                                    .apply(mixins::string_value_select(page.bedno.clone(), page.changed.clone()))
                                    //.attr("onchange", "onchangeBedNoSelect(event, this)")
                                }),
                            ])
                        })),
                        doms::form_inline_group_sm(clone!(page => move |group| { group
                            .children([
                                doms::label_group_for("er_patient_status_id","สถานะ"),
                                html!("select" => HtmlSelectElement, {
                                    .class(class::FORM_SELECT_SM)
                                    .attr("id", "er_patient_status_id")
                                    .with_node!(element => {
                                        .future(page.is_readonly_or_not_view_by_doctor_or_nurse().for_each(move |is_readonly| {
                                            element.set_disabled(is_readonly);
                                            async {}
                                        }))
                                    })
                                    .children(er_patient_status_select_options.iter().map(|option| {
                                        doms::select_option(option, &page.er_patient_status_id.lock_ref())
                                    }))
                                    .apply(mixins::string_value_select(page.er_patient_status_id.clone(), page.changed.clone()))
                                    // .attr("onchange", "onchangePatientStatusSelect(event)")
                                }),
                            ])
                        })),
                    ])
                    .child_signal(map_ref!{
                        let is_readonly = page.is_readonly_or_not_view_by_doctor_or_nurse(),
                        let display_search = page.display_patient_searchbox.signal() =>
                        !is_readonly && *display_search
                    }.map(clone!(app, page => move |show| {
                        if show {
                            app.get_id("vn-input-group").map(clone!(app, page => move |elm| {
                                OpdVisitSearchboxCpn::render(
                                    OpdVisitSearchboxCpn::new(),
                                    page.display_patient_searchbox.clone(),
                                    page.vn.clone(),
                                    page.opd_visit_detail.clone(),
                                    elm.get_bounding_client_rect(),
                                    page.changed.clone(),
                                    app,
                                )
                            }))
                        } else {
                            None
                        }
                    })))
                    .child_signal(page.is_discharged().map(clone!(page, er_dch_type_select_options => move |discharged| discharged.then(|| {
                        doms::form_inline_group_sm(clone!(page, er_dch_type_select_options => move |group| { group
                            .children([
                                doms::label_group_for("er_dch_type_id","ประเภทการ Discharge"),
                                html!("select" => HtmlSelectElement, {
                                    .class(class::FORM_SELECT_SM)
                                    .attr("id", "er_dch_type_id")
                                    .with_node!(element => {
                                        .future(page.is_readonly_or_not_view_by_doctor_or_nurse().for_each(move |is_readonly| {
                                            element.set_disabled(is_readonly);
                                            async {}
                                        }))
                                    })
                                    .child(html!("option", {.attr("value", "").text("เลือก")}))
                                    .children(er_dch_type_select_options.iter().map(|option| {
                                        doms::select_option(option, &page.er_dch_type_id.lock_ref())
                                    }))
                                    .apply(mixins::string_value_select(page.er_dch_type_id.clone(), page.changed.clone()))
                                    // .attr("onchange", "onchangePatientStatusSelect(event)")
                                }),
                            ])
                        }))
                    }))))
                    .child_signal(page.is_discharged().map(clone!(page => move |discharged| discharged.then(|| {
                        doms::form_inline_group_sm(clone!(page => move |group| { group
                            .children([
                                doms::label_group_for("discharge_date","วันที่จำหน่าย"),
                                doms::date_picker(
                                    page.discharge_date.clone(),
                                    page.changed.clone(), page.is_readonly_or_not_view_by_doctor_or_nurse(), None,
                                    |d| d.class(class::FLEX_GROW1).style("min-width","120px"),
                                    |d| d.class(class::FORM_CTRL_ONLY_SM_R0),
                                    |d| d.class(class::FORM_CTRL_ONLY_SM_R0).attr("id", "discharge_date"),
                                    |s| s, always(None),
                                ),
                                doms::time_picker(
                                    page.discharge_time.clone(),
                                    page.changed.clone(), page.is_readonly_or_not_view_by_doctor_or_nurse(), None,
                                    |d| d.class(class::FLEX_GROW1).style("min-width","95px"),
                                    |d| d.class(class::FORM_CTRL_ONLY_SM_R0),
                                    |d| d.class(class::FORM_CTRL_ONLY_SM_R0),
                                    |s| s, always(None),
                                ),
                                html!("button" => HtmlButtonElement, {
                                    .attr("type", "button")
                                    .class(class::BTN_GRAY)
                                    .attr("title", "ใช้วันที่-เวลา ปัจจุบัน")
                                    .with_node!(element => {
                                        .future(page.is_readonly_or_not_view_by_doctor_or_nurse().for_each(move |is_readonly| {
                                            element.set_disabled(is_readonly);
                                            async {}
                                        }))
                                    })
                                    .child(html!("i", {.class(class::FA_CLOCK)}))
                                    .event(clone!(page => move |_: events::Click| {
                                        let now = js_now();
                                        page.discharge_date.set(now.date().to_string());
                                        page.discharge_time.set(now.time().js_string());
                                    }))
                                    // .attr("onclick", "
                                    //      event.preventDefault();
                                    //      event.stopPropagation();
                                    //      setCurrentDateAndTime(document.querySelector('#discharge_date'),document.querySelector('#discharge_time'));
                                    //      onchangePatientStatusSelect(event);")
                                }),
                            ])
                        }))
                    }))))
                    .child(html!("div", {
                        .class("w-100")
                        .children([
                            html!("label", {
                                .attr("for", "note")
                                .text("Note")
                            }),
                            html!("textarea" => HtmlTextAreaElement, {
                                .class("form-control")
                                .attr("id", "note")
                                .attr("rows", "3")
                                .with_node!(element => {
                                    .future(page.is_readonly_or_not_view_by_doctor_or_nurse().for_each(move |is_readonly| {
                                        element.set_disabled(is_readonly);
                                        async {}
                                    }))
                                })
                                .apply(mixins::string_value(page.note.clone(), page.changed.clone()))
                                //.attr("oninput", "onchangeNote(event)")
                            }),
                        ])
                    }))
                    .apply_if(app.endpoint_is_allow(&Method::POST, &EndPoint::OpdErOrderMaster, false), |dom| dom
                        .child(doms::form_inline_end(clone!(app, page => move |end| { end
                            .child_signal(page.is_readonly_or_not_view_by_doctor_or_nurse().map(clone!(app, page => move |is_readonly| {
                                (!is_readonly).then(|| {
                                    html!("button" => HtmlButtonElement, {
                                        .attr("type", "button")
                                        .class(class::BTN_SM_L_BLUE)
                                        .child(html!("i", {.class(class::FA_SAVE)}))
                                        .text(" บันทึก")
                                        .visible_signal(page.changed.signal())
                                        .apply(mixins::click_with_loader_checked_or_true_disable_signal(clone!(app, page => move || {
                                            Self::save_master(page.clone(), app.clone());
                                        }), page.vn.signal_cloned().map(|vn| vn.is_empty()), app.state()))
                                        // .attr("onclick", "onclickSaveOpdErOrderMasterButton(event);")
                                    })
                                })
                            })))
                            .child_signal(page.is_readonly_or_not_view_by_doctor_or_nurse().map(clone!(page => move |is_readonly| {
                                (!is_readonly).then(|| {
                                    html!("button", {
                                        .attr("type", "button")
                                        .visible_signal(page.changed.signal())
                                        .class(class::BTN_SM_GRAY)
                                        .child(html!("i", {.class(class::FA_X)}))
                                        .text(" ยกเลิก")
                                        .event(clone!(page => move |_: events::Click| {
                                            page.loaded_master.set_neq(false);
                                        }))
                                        // .attr("onclick", "onclickResetOpdErOrderMasterButton(event);")
                                    })
                                })
                            })))
                        })))
                    )
                })),
                html!("hr"),
                html!("ul", {
                    .class(class::NAV_PILLS_T)
                    //.attr("id", "opd-er-main-menu-pills-tab")
                    .attr("role", "tablist")
                    .child(html!("li", {
                        .class(class::NAV_ITEM_PY)
                        .child(html!("a", {
                            .class(class::BTN_L_BLUE)
                            .attr("href", "#")
                            .child(html!("i", {.class(class::FA_L_ARROW)}))
                            .text(" กลับ")
                            .event_with_options(&EventOptions::preventable(), clone!(app, page => move |event: events::Click| {
                                event.prevent_default();
                                if app.go_back_else() {
                                    let route = Route::OpdErOrderList {view_by: page.view_by.get_cloned()};
                                    if route.has_permission(app.state()) {
                                        route.hard_redirect();
                                    } else {
                                        Route::Info.hard_redirect();
                                    }
                                }
                            }))
                            // .attr("onclick", "onclickBackButton(event);")
                        }))
                    }))
                    .apply_if(
                        app.endpoint_is_allow(&Method::GET, &EndPoint::OpdErMedicalHistory, false)
                        && app.endpoint_is_allow(&Method::GET, &EndPoint::OpdErMedicalHistoryTrauma, false)
                        && app.endpoint_is_allow(&Method::GET, &EndPoint::OpdErMedicalHistoryAllergy, false)
                        && app.endpoint_is_allow(&Method::GET, &EndPoint::OpdErMedicalHistoryScreen, false)
                        && app.endpoint_is_allow(&Method::GET, &EndPoint::OpdErMedicalHistoryScan, false)
                        && app.endpoint_is_allow(&Method::GET, &EndPoint::OpdErMedicalHistoryConsult, false)
                        && app.endpoint_is_allow(&Method::GET, &EndPoint::OpdErMedicalHistoryFt, false),
                    |dom| { dom
                        .child(html!("li", {
                            .class(class::NAV_ITEM_PY)
                            .child(html!("a", {
                                .class("nav-link")
                                .class_signal("active", page.active_tab.signal_cloned().map(|tab| matches!(tab, Tab::MedHx)))
                                //.attr("id", "pills-medical-history-tab")
                                .attr("data-bs-toggle", "pill")
                                .attr("href", "#")
                                .text("ประวัติผู้ป่วย")
                                .event_with_options(&EventOptions::preventable(), clone!(page => move |event: events::Click| {
                                    event.prevent_default();
                                    page.active_tab.set_neq(Tab::MedHx);
                                }))
                            }))
                        }))
                    })
                    .apply_if(app.endpoint_is_allow(&Method::GET, &EndPoint::OpdErMedReconcile, false), |dom| { dom
                        .child(html!("li", {
                            .class(class::NAV_ITEM_PY)
                            .child(html!("a", {
                                .class("nav-link")
                                .class_signal("active", page.active_tab.signal_cloned().map(|tab| matches!(tab, Tab::MedReconcile)))
                                .attr("data-bs-toggle","pill")
                                .attr("href","#")
                                .text("Med Reconciliation ")
                                .child_signal(map_ref! {
                                    let waiting = page.show_med_reconciliation_exists_spinner.signal(),
                                    let has_data = page.show_med_reconciliation_has_data.signal() =>
                                    (*waiting, *has_data)
                                }.map(|(waiting, has_data)| {
                                    if waiting {
                                        Some(html!("span", {.class(class::SPIN_SM_GLOW_RED)}))
                                    } else if has_data {
                                        Some(html!("i", {
                                            .class(class::FA_CIRCLE_GREEN)
                                        }))
                                    } else {
                                        None
                                    }
                                }))
                                .event_with_options(&EventOptions::preventable(), clone!(page => move |event: events::Click| {
                                    event.prevent_default();
                                    page.active_tab.set_neq(Tab::MedReconcile);
                                }))
                            }))
                        }))
                    })
                    .apply_if(
                        app.endpoint_is_allow(&Method::GET, &EndPoint::OpdErOrderMasterCheckVn, false)
                        && app.endpoint_is_allow(&Method::GET, &EndPoint::OpdErOrderOrder, false)
                        && app.endpoint_is_allow(&Method::GET, &EndPoint::OpdErOrderProgressNote, false)
                        && app.endpoint_is_allow(&Method::GET, &EndPoint::OpdErHisMedVn, false)
                        && app.endpoint_is_allow(&Method::GET, &EndPoint::OpdErMedReconcile, false),
                    |dom| dom
                        .child(html!("li", {
                            .class(class::NAV_ITEM_PY)
                            .child(html!("a", {
                                .class("nav-link")
                                .class_signal("active", page.active_tab.signal_cloned().map(|tab| matches!(tab, Tab::Order)))
                                //.attr("id", "pills-order-tab")
                                .attr("data-bs-toggle", "pill")
                                .attr("href", "#")
                                .text("Order")
                                .event_with_options(&EventOptions::preventable(), clone!(page => move |event: events::Click| {
                                    event.prevent_default();
                                    page.active_tab.set_neq(Tab::Order);
                                }))
                            }))
                        }))
                    )
                    .apply_if(app.endpoint_is_allow(&Method::GET, &EndPoint::OpdErVitalSign, false), |dom| { dom
                        .child(html!("li", {
                            .class(class::NAV_ITEM_PY)
                            .child(html!("a", {
                                .class("nav-link")
                                .class_signal("active", page.active_tab.signal_cloned().map(|tab| matches!(tab, Tab::VitalSign)))
                                //.attr("id", "pills-vs-tab")
                                .attr("data-bs-toggle", "pill")
                                .attr("href", "#")
                                .text("Vital Sign")
                                .event_with_options(&EventOptions::preventable(), clone!(page => move |event: events::Click| {
                                    event.prevent_default();
                                    page.active_tab.set_neq(Tab::VitalSign);
                                }))
                            }))
                        }))
                    })
                    .apply_if(
                        app.endpoint_is_allow(&Method::GET, &EndPoint::OpdErIoDateId, false)
                        && app.endpoint_is_allow(&Method::GET, &EndPoint::OpdErIo, false),
                    |dom| { dom
                        .child_signal(page.view_by.signal_cloned().map(clone!(page => move |view_by| {
                            ["doctor","nurse","pharmacist"].contains(&view_by.as_str()).then(||
                                html!("li", {
                                    .class(class::NAV_ITEM_PY)
                                    .child(html!("a", {
                                        .class("nav-link")
                                        .class_signal("active", page.active_tab.signal_cloned().map(|tab| matches!(tab, Tab::Io)))
                                        //.attr("id", "pills-io-tab")
                                        .attr("data-bs-toggle", "pill")
                                        .attr("href", "#")
                                        .text("I/O")
                                        .event_with_options(&EventOptions::preventable(), clone!(page => move |event: events::Click| {
                                            event.prevent_default();
                                            page.active_tab.set_neq(Tab::Io);
                                        }))
                                    }))
                                })
                            )
                        })))
                    })
                    .apply_if(app.endpoint_is_allow(&Method::GET, &EndPoint::OpdErFocusNoteId, false), |dom| { dom
                        .child_signal(page.view_by.signal_cloned().map(clone!(page => move |view_by| {
                            (["doctor","nurse"].contains(&view_by.as_str())).then(||
                                html!("li", {
                                    .class(class::NAV_ITEM_PY)
                                    .child(html!("a", {
                                        .class("nav-link")
                                        .class_signal("active", page.active_tab.signal_cloned().map(|tab| matches!(tab, Tab::NurseNote)))
                                        //.attr("id", "pills-focus-tab")
                                        .attr("data-bs-toggle", "pill")
                                        .attr("href", "#")
                                        .text("Nursing Progress Note")
                                        .event_with_options(&EventOptions::preventable(), clone!(page => move |event: events::Click| {
                                            event.prevent_default();
                                            page.active_tab.set_neq(Tab::NurseNote);
                                        }))
                                    }))
                                })
                            )
                        })))
                    })
                    .apply_if(app.endpoint_is_allow(&Method::GET, &EndPoint::OpdErOrderItem, false), |dom| { dom
                        .child_signal(page.is_view_by_nurse().map(clone!(page => move |is_nurse| {
                            is_nurse.then(|| {
                                html!("li", {
                                    .class(class::NAV_ITEM_PY)
                                    .child(html!("a", {
                                        .class("nav-link")
                                        .class_signal("active", page.active_tab.signal_cloned().map(|tab| matches!(tab, Tab::NursePlan)))
                                        //.attr("id", "pills-planning-tab")
                                        .attr("data-bs-toggle", "pill")
                                        .attr("href", "#")
                                        .text("Nurse Planning")
                                        .event_with_options(&EventOptions::preventable(), clone!(page => move |event: events::Click| {
                                            event.prevent_default();
                                            page.active_tab.set_neq(Tab::NursePlan);
                                        }))
                                    }))
                                })
                            })
                        })))
                    })
                    .apply_if(app.endpoint_is_allow(&Method::GET, &EndPoint::LabHead, false), |dom| { dom
                            .child(html!("li", {
                            .class(class::NAV_ITEM_PY)
                            .child(html!("a", {
                                .class("nav-link")
                                .class_signal("active", page.active_tab.signal_cloned().map(|tab| matches!(tab, Tab::Lab)))
                                //.attr("id", "pills-lab-tab")
                                .attr("data-bs-toggle", "pill")
                                .attr("href", "#")
                                .text("Lab ")
                                .child_signal(map_ref! {
                                    let unread = page.show_lab_unread_exists_spinner.signal(),
                                    let unreport = page.show_lab_unreport_has_data.signal() =>
                                    (*unread, *unreport)
                                }.map(|(unread, unreport)| {
                                    if unread {
                                        Some(html!("span", {.class(class::SPIN_SM_GLOW_RED)}))
                                    } else if unreport {
                                        Some(html!("i", {
                                            .class(class::FA_HOURGLASS_GOLD)
                                        }))
                                    } else {
                                        None
                                    }
                                }))
                                .event_with_options(&EventOptions::preventable(), clone!(page => move |event: events::Click| {
                                    event.prevent_default();
                                    page.active_tab.set_neq(Tab::Lab);
                                }))
                            }))
                        }))
                    })
                    .apply(|dom| {
                        if app.has_pacs_host()
                            && app.endpoint_is_allow(&Method::GET, &EndPoint::XrayReportHn, false)
                            && app.endpoint_is_allow(&Method::GET, &EndPoint::XrayPacsXn, false)
                        {
                            dom.child(html!("li", {
                                .class(class::NAV_ITEM_PY)
                                .child(html!("a", {
                                    .class("nav-link")
                                    .class_signal("active", page.active_tab.signal_cloned().map(|tab| matches!(tab, Tab::XRay)))
                                    .attr("data-bs-toggle","pill")
                                    .attr("href","#")
                                    .text("X-Ray ")
                                    .child_signal(page.show_xray_unread_exists_spinner.signal().map(|unread| {
                                        if unread {
                                            Some(html!("span", {.class(class::SPIN_SM_GLOW_RED)}))
                                        } else {
                                            None
                                        }
                                    }))
                                    .event_with_options(&EventOptions::preventable(), clone!(page => move |event: events::Click| {
                                        event.prevent_default();
                                        page.active_tab.set_neq(Tab::XRay);
                                    }))
                                }))
                            }))
                        } else {
                            dom.child_signal(page.patient.lock_ref().hn.signal_cloned().map(clone!(app => move |hn| {
                                app.pacs_hn_url(&hn).map(|hn_url| {
                                    doms::nav_item_external_url(&hn_url, "X-Ray ")
                                })
                            })))
                        }
                    })
                    .child_signal(page.patient.lock_ref().hn.signal_cloned().map(clone!(app => move |hn| {
                        app.ekg_hn_url(&hn).map(|hn_url| {
                            doms::nav_item_external_url(&hn_url, "EKG ")
                        })
                    })))
                    .child_signal(page.patient.lock_ref().hn.signal_cloned().map(clone!(app => move |hn| {
                        app.scan_hn_url(&hn).map(|hn_url| {
                            doms::nav_item_external_url(&hn_url, "Scan ")
                        })
                    })))
                    .apply(|dom| {
                        let route = Route::PrescriptionScreen{ hn: page.hn.get_cloned()};
                        if route.has_permission(app.state()) {
                            dom.child(html!("li", {
                                .class(class::NAV_ITEM_PY)
                                .child(html!("a", {
                                    .class("nav-link")
                                    .attr("href", "#")
                                    //.attr("id", "pills-pharmacy-prescription-screen")
                                    .attr("data-bs-toggle", "pill")
                                    .text("ประวัติการสั่งยา ")
                                    .child(html!("i", {.class(class::FA_DISPLAY)}))
                                    .event_with_options(&EventOptions::preventable(), move |event: events::Click| {
                                        event.prevent_default();
                                        route.hard_redirect();
                                    })
                                }))
                            }))
                        } else {
                            dom
                        }
                    })
                    .apply_if(
                        app.endpoint_is_allow(&Method::GET, &EndPoint::EmrDateHn, false)
                        && app.endpoint_is_allow(&Method::GET, &EndPoint::EmrVisitVn, false),
                    |dom| dom
                        .child(html!("li", {
                            .class(class::NAV_ITEM_PY)
                            .child(html!("a", {
                                .class("nav-link")
                                .class_signal("active", page.active_tab.signal_cloned().map(|tab| matches!(tab, Tab::Emr)))
                                // .attr("id", "pills-EMR-tab")
                                .attr("data-bs-toggle", "pill")
                                .attr("href", "#")
                                .text("EMR")
                                .event_with_options(&EventOptions::preventable(), clone!(page => move |event: events::Click| {
                                    event.prevent_default();
                                    page.active_tab.set_neq(Tab::Emr);
                                }))
                            }))
                        }))
                    )
                    .apply_if(app.endpoint_is_allow(&Method::GET, &EndPoint::HisReferOutVnan, false), |dom| { dom
                        .child_signal(page.view_by.signal_cloned().map(clone!(page => move |view_by| {
                            ["doctor","nurse"].contains(&view_by.as_str()).then(||
                                html!("li", {
                                    .class(class::NAV_ITEM_PY)
                                    .child(html!("a", {
                                        .class("nav-link")
                                        .class_signal("active", page.active_tab.signal_cloned().map(|tab| matches!(tab, Tab::ReferOut)))
                                        .attr("data-bs-toggle","pill")
                                        .attr("href","#")
                                        .text("Refer Out")
                                        .event_with_options(&EventOptions::preventable(), clone!(page => move |event: events::Click| {
                                            event.prevent_default();
                                            page.active_tab.set_neq(Tab::ReferOut);
                                        }))
                                    }))
                                })
                            )
                        })))
                    })
                    .child(html!("li", {
                        .class(class::NAV_ITEM_PY)
                        .child(html!("a", {
                            .class("nav-link")
                            .class_signal("active", page.active_tab.signal_cloned().map(|tab| matches!(tab, Tab::Document)))
                            //.attr("id", "pills-opd-er-document-tab")
                            .attr("data-bs-toggle", "pill")
                            .attr("href", "#")
                            .text("เอกสาร")
                            .event_with_options(&EventOptions::preventable(), clone!(page => move |event: events::Click| {
                                event.prevent_default();
                                page.active_tab.set_neq(Tab::Document);
                            }))
                            // .attr("onclick", "onclickOPD_ER_Document();")
                        }))
                    }))
                    .child_signal(page.patient.lock_ref().vn.signal_cloned().map(clone!(app, page => move |vn_opt| {
                        if let Some(vn) = vn_opt {
                            let cart_vn_url = app.cart_vnan_url(&vn);
                            (page.view_by.lock_ref().as_str() == "nurse" && cart_vn_url.is_some()).then(||
                                doms::nav_item_external_url(&cart_vn_url.unwrap_or_default(), "ขอเปล ")
                            )
                        } else {
                            None
                        }
                    })))
                    .child_signal(page.antibiograms.signal_cloned().map(|antibiograms| {
                        (!antibiograms.is_empty()).then(|| doms::antibiogram_dropdown(&antibiograms))
                    }))
                }),
                html!("hr"),
                html!("div", {
                    .class("tab-content")
                    //.attr("id", "pills-tabContent")
                    .child_signal(page.active_tab.signal_cloned().map(clone!(app, page => move |tab| {
                        let patient_cpn = page.patient.lock_ref();
                        let patient = patient_cpn.patient.clone();
                        Some(match tab {
                            Tab::MedHx => {
                                let hx_cpn = OpdErEmergencyCpn::new(
                                    patient.clone(),
                                    page.view_by.clone(),
                                );
                                OpdErEmergencyCpn::render(hx_cpn, app.clone())
                            },
                            Tab::MedReconcile => {
                                let recon = MedReconcileCpn::new(
                                    page.view_by.clone(),
                                    patient.clone(),
                                    page.active_tab.clone(),
                                    page.loaded_med_reconciliation_exists_spinner.clone(),
                                    page.loaded_med_reconciliation_has_data.clone(),
                                );
                                MedReconcileCpn::render(recon, app.clone())
                            }
                            Tab::Order => {
                                let opd_er_order = OrderCpn::new(
                                    false,
                                    patient.clone(),
                                    page.view_by.clone(),
                                    page.pass_24_hour_from_dch.clone(),
                                    Mutable::new(String::new()),
                                    page.focused_id.clone(),
                                    app.clone(),
                                );
                                page.tab_order_loaded.set(Some(opd_er_order.loaded_all.clone()));
                                OrderCpn::render("opd-er-main", opd_er_order, app.clone())
                            }
                            Tab::NursePlan => {
                                let index_plan = IndexPlanCpn::new(patient.clone(), page.view_by.clone());
                                IndexPlanCpn::render(index_plan, app.clone())
                            }
                            Tab::Lab => {
                                let lab = LabCpn::new(
                                    patient.clone(),
                                    patient_cpn.hn.clone(),
                                    page.vn.clone(),
                                    Some(page.loaded_lab_unread_exists_spinner.clone()),
                                );
                                LabCpn::render("opd-er-main", lab, app.clone())
                            }
                            Tab::XRay => {
                                let xray = XrayCpn::new_opd_er(
                                    patient_cpn.hn.clone(),
                                    page.vn.clone(),
                                    Some(page.loaded_xray_unread_exists_spinner.clone()),
                                );
                                XrayCpn::render("opd-er-main", xray, app.clone())
                            }
                            Tab::NurseNote => {
                                let nurse_note = NurseNoteCpn::new(patient.clone(), page.view_by.clone());
                                NurseNoteCpn::render(nurse_note, app.clone())
                            }
                            Tab::VitalSign => {
                                let vs = VitalSignCpn::new(
                                    patient.clone(),
                                    patient_cpn.loaded.clone(),
                                    page.view_by.clone(),
                                );
                                VitalSignCpn::render(vs, app.clone())
                            }
                            Tab::Io => {
                                let io = IoCpn::new(patient.clone());
                                IoCpn::render(io, app.clone())
                            }
                            Tab::Emr => {
                                let emr = EmrCpn::new(patient_cpn.hn.clone());
                                EmrCpn::render("opd-er-main", emr, app.clone())
                            }
                            Tab::Document => {
                                let document = DocumentCpn::new(
                                    patient_cpn.vn.clone(),
                                    patient.clone(),
                                );
                                DocumentCpn::render(document, app.clone())
                            }
                            Tab::ReferOut => {
                                let referout = ReferOutCpn::new(patient.clone());
                                ReferOutCpn::render(referout, app.clone())
                            }
                            Tab::Doctor
                            | Tab::Consult => Dom::empty(),
                        })
                    })))
                }),
            ])
        })
    }
}
