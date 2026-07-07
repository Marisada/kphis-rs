// ipd-dr-main.php

use dominator::{Dom, EventOptions, clone, events, html, window_size};
use futures_signals::{
    map_ref,
    signal::{Mutable, Signal, SignalExt},
};
use std::rc::Rc;

use kphis_model::{
    LEFT_PANEL_MIN_WIDTH, SCREEN_WIDTH_EXTRA,
    endpoint::EndPoint,
    fetch::{Method, call_api_get_exists_key_id},
    route::Route,
    {antibiogram::Antibiograms, report::SystemReport, tab::Tab},
};
use kphis_ui_app::App;
use kphis_ui_component::{
    doctor_in_charge::DoctorInChargeCpn, document::DocumentCpn, emr::EmrCpn, gadget::aside_resizer::AsideResizerCpn, index_plan::IndexPlanCpn, io::IoCpn, ipd_consult::IpdConsultCpn, lab::LabCpn,
    med_reconcile::med_reconcile_main::MedReconcileCpn, nurse_note::nurse_note_main::NurseNoteCpn, order::OrderCpn, refer_out::ReferOutCpn, show_patient_main::ShowPatientMainCpn,
    vital_sign::vital_sign_main::VitalSignCpn, xray::XrayCpn,
};
use kphis_ui_core::{class, doms};
use kphis_util::util::str_some;

/// - GET `EndPoint::IpdShowPatientMainAn` (ShowPatientMainCpn)
/// - GET `EndPoint::OpdErShowPatientMainVn` (ShowPatientMainCpn)
/// - GET `EndPoint::ExistsKeyId` (guarded, no alert badges)
/// - GET `EndPoint::EmrDateHn` (EmrCpn, guarded, remove 'EMR' tab)
/// - GET `EndPoint::EmrVisitVn` (EmrCpn, guarded, remove 'EMR' tab)
/// - GET `EndPoint::IpdConsultAn` (IpdConsultCpn, guarded, remove 'Consult' tab)
/// - GET `EndPoint::IpdConsultId` (IpdConsultCpn, guarded, remove 'Consult' tab)
/// - GET `EndPoint::IpdDoctorInCharge` (DoctorInChargeCpn, guarded, remove 'แพทย์เจ้าของไข้' tab)
/// - GET `EndPoint::IpdFocusNoteAn` (NurseNoteCpn, guarded, remove 'Nursing Progress Note' tab)
/// - GET `EndPoint::IpdIoDateAn` (IoCpn, guarded, remove 'I/O' tab)
/// - GET `EndPoint::IpdIo` (IoCpn, guarded, remove 'I/O' tab)
/// - GET `EndPoint::IpdIndexPlanDateAn` (IndexPlanCpn, guarded, remove 'Nurse Planning' tab)
/// - GET `EndPoint::IpdOrderItem` (IndexPlanCpn, guarded, remove 'Nurse Planning' tab)
/// - GET `EndPoint::IpdMedReconcile` (OrderCpn/MedReconcileCpn, guarded, remove MedRec/Order tab)
/// - GET `EndPoint::IpdOrderOrderDateAn` (OrderCpn, guarded, remove Order tab)
/// - GET `EndPoint::IpdOrderOrder` (OrderCpn, guarded, remove Order tab)
/// - GET `EndPoint::IpdOrderPrevious` (OrderCpn, guarded, remove Order tab)
/// - GET `EndPoint::IpdOrderProgressNote` (OrderCpn, guarded, remove Order tab)
/// - GET `EndPoint::IpdVitalSign` (VitalSignCpn, guarded, remove 'Vital Sign' tab)
/// - GET `EndPoint::LabHead` (LabCpn, guarded, remove 'Lab' tab)
/// - GET `EndPoint::XrayPacsXn` (XrayCpn, guarded, remove 'X-Ray' tab)
/// - GET `EndPoint::XrayReportHn` (XrayCpn, guarded, remove 'X-Ray tab)
#[derive(Clone, Default)]
pub struct IpdMainPage {
    active_tab: Mutable<Tab>,

    view_by: Mutable<String>,
    an: Mutable<String>,
    sub: Mutable<String>,
    focused_id: Mutable<u32>,
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

impl IpdMainPage {
    pub fn new(view_by: String, an: String, tab: Tab, sub: String, focused_id: u32) -> Rc<Self> {
        Rc::new(Self {
            active_tab: Mutable::new(tab),
            view_by: Mutable::new(view_by),
            an: Mutable::new(an),
            sub: Mutable::new(sub),
            focused_id: Mutable::new(focused_id),
            ..Default::default()
        })
    }

    fn hn(&self) -> impl Signal<Item = String> + use<> {
        self.patient
            .signal_cloned()
            .map(|pt| pt.patient.signal_cloned())
            .flatten()
            .map(|opt| opt.as_ref().and_then(|pt| pt.hn()).unwrap_or_default())
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

    fn loaded_med_reconciliation_exists(page: Rc<Self>, app: Rc<App>) {
        if let Some(an) = str_some(page.an.get_cloned()) {
            app.async_load(
                true,
                clone!(app, page => async move {
                    // GET `EndPoint::ExistsKeyId`
                    match call_api_get_exists_key_id("ipd-med-reconcile/", &an, app.state()).await {
                        Ok(exists) => {
                            page.show_med_reconciliation_has_data.set_neq(exists);
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
        if let Some(an) = str_some(page.an.get_cloned()) {
            app.async_load(
                true,
                clone!(app, page => async move {
                    // GET `EndPoint::ExistsKeyId`
                    match call_api_get_exists_key_id("ipd-med-reconcile-dr-unconfirm/", &an, app.state()).await {
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
        if let Some(an) = str_some(page.an.get_cloned()) {
            app.async_load(
                true,
                clone!(app, page => async move {
                    // GET `EndPoint::ExistsKeyId`
                    match call_api_get_exists_key_id("lab-unreport/", &an, app.state()).await {
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
        if let Some(an) = str_some(page.an.get_cloned()) {
            app.async_load(
                true,
                clone!(app, page => async move {
                    // GET `EndPoint::ExistsKeyId`
                    match call_api_get_exists_key_id("lab-unread/", &an, app.state()).await {
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
        if let Some(an) = str_some(page.an.get_cloned()) {
            app.async_load(
                true,
                clone!(app, page => async move {
                    // GET `EndPoint::ExistsKeyId`
                    match call_api_get_exists_key_id("ipd-xray-unread/", &an, app.state()).await {
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
        app.set_title("KPHIS - IPD Main Page");

        let show_patient_main = ShowPatientMainCpn::new_with_an(page.an.get_cloned());
        let hn = show_patient_main.hn.clone();
        let patient_main = ShowPatientMainCpn::render(false, show_patient_main.clone(), app.clone());
        page.patient.set(show_patient_main);

        html!("div", {
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
                        Some((true, page.patient.lock_ref().patient.clone())),
                        AsideResizerCpn::new(
                            Mutable::new(report_selected), Mutable::new(false),
                            Mutable::new(None), Mutable::new(false),
                            page.an.clone(), hn.clone(), SystemReport::ipd_set(),
                            "ipd-main-container",
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
        let is_pre_admit = app.is_pre_admit(&page.an.lock_ref());
        let allow_medrec = app.endpoint_is_allow(&Method::GET, &EndPoint::IpdMedReconcile, is_pre_admit);
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
                    ["doctor","nurse"].contains(&view_by.as_str()) && !busy && !loaded
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
            .attr("id", "ipd-main-container")
            .class(class::CONF_B)
            .style("min-width",LEFT_PANEL_MIN_WIDTH)
            .children([
                html!("ul", {
                    .class(class::NAV_PILLS_T)
                    //.attr("id", "pills-tab")
                    .attr("role","tablist")
                    .child(html!("li", {
                        .class(class::NAV_ITEM_PY)
                        .child(html!("a", {
                            .class(class::BTN_L_BLUE)
                            .attr("href","#")
                            .child(html!("i", {.class(class::FA_L_ARROW)}))
                            .text(" กลับ")
                            .event_with_options(&EventOptions::preventable(), clone!(app, page => move |event: events::Click| {
                                event.prevent_default();
                                if app.go_back_else() {
                                    let route = match page.view_by.lock_ref().as_str() {
                                        "doctor" => Route::IpdSearchPatientDr,
                                        "nurse" => Route::IpdSearchPatientNurse,
                                        "pharmacist" => Route::IpdSearchPatientPharmacist,
                                        "other" => Route::IpdSearchPatientOther,
                                        _ => Route::Info,
                                    };
                                    route.hard_redirect();
                                }
                            }))
                        }))
                    }))
                    .apply_if(allow_medrec, |dom| { dom
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
                                        // ./ipd-dr-med-reconcile-check-doctor-unconfirm.php", {an: IPD_MED_RECONCILIATION_AN}) > 0
                                        Some(html!("span", {.class(class::SPIN_SM_GLOW_RED)}))
                                    } else if has_data {
                                        Some(html!("i", {
                                            .class(class::FA_CIRCLE_GREEN)
                                            // ./ipd-dr-med-reconcile-check.php", {an: IPD_MED_RECONCILIATION_AN}) > 0
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
                        app.endpoint_is_allow(&Method::GET, &EndPoint::IpdOrderOrderDateAn, is_pre_admit)
                        && app.endpoint_is_allow(&Method::GET, &EndPoint::IpdOrderOrder, is_pre_admit)
                        && app.endpoint_is_allow(&Method::GET, &EndPoint::IpdOrderPrevious, is_pre_admit)
                        && app.endpoint_is_allow(&Method::GET, &EndPoint::IpdOrderProgressNote, is_pre_admit)
                        && app.endpoint_is_allow(&Method::GET, &EndPoint::IpdMedReconcile, is_pre_admit),
                    |dom| dom
                        .child(html!("li", {
                            .class(class::NAV_ITEM_PY)
                            .child(html!("a", {
                                .class("nav-link")
                                .class_signal("active", page.active_tab.signal_cloned().map(|tab| matches!(tab, Tab::Order)))
                                .attr("data-bs-toggle","pill")
                                .attr("href","#")
                                .text_signal(page.view_by.signal_cloned().map(|view_by| {
                                    if view_by.as_str() == "nurse" {
                                        "Order & Index"
                                    } else {
                                        "Order"
                                    }
                                }))
                                .event_with_options(&EventOptions::preventable(), clone!(page => move |event: events::Click| {
                                    event.prevent_default();
                                    page.active_tab.set_neq(Tab::Order);
                                }))
                            }))
                        }))
                    )
                    .apply_if(app.endpoint_is_allow(&Method::GET, &EndPoint::IpdVitalSign, is_pre_admit), |dom| { dom
                        .child(html!("li", {
                            .class(class::NAV_ITEM_PY)
                            .child(html!("a", {
                                .class("nav-link")
                                .class_signal("active", page.active_tab.signal_cloned().map(|tab| matches!(tab, Tab::VitalSign)))
                                .attr("data-bs-toggle","pill")
                                .attr("href","#")
                                .text("Vital Sign")
                                .event_with_options(&EventOptions::preventable(), clone!(page => move |event: events::Click| {
                                    event.prevent_default();
                                    page.active_tab.set_neq(Tab::VitalSign);
                                }))
                            }))
                        }))
                    })
                    .apply_if(
                        app.endpoint_is_allow(&Method::GET, &EndPoint::IpdIoDateAn, is_pre_admit)
                        && app.endpoint_is_allow(&Method::GET, &EndPoint::IpdIo, is_pre_admit),
                    |dom| { dom
                        .child_signal(page.view_by.signal_cloned().map(clone!(page => move |view_by| {
                            ["doctor","nurse","pharmacist"].contains(&view_by.as_str()).then(||
                                html!("li", {
                                    .class(class::NAV_ITEM_PY)
                                    .child(html!("a", {
                                        .class("nav-link")
                                        .class_signal("active", page.active_tab.signal_cloned().map(|tab| matches!(tab, Tab::Io)))
                                        .attr("data-bs-toggle","pill")
                                        .attr("href","#")
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
                    .apply_if(app.endpoint_is_allow(&Method::GET, &EndPoint::IpdFocusNoteAn, is_pre_admit), |dom| { dom
                        .child_signal(page.view_by.signal_cloned().map(clone!(page => move |view_by| {
                            (["doctor","nurse"].contains(&view_by.as_str())).then(||
                                html!("li", {
                                    .class(class::NAV_ITEM_PY)
                                    .child(html!("a", {
                                        .class("nav-link")
                                        .class_signal("active", page.active_tab.signal_cloned().map(|tab| matches!(tab, Tab::NurseNote)))
                                        .attr("data-bs-toggle","pill")
                                        .attr("href","#")
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
                    .apply_if(
                        app.endpoint_is_allow(&Method::GET, &EndPoint::IpdIndexPlanDateAn, is_pre_admit)
                        && app.endpoint_is_allow(&Method::GET, &EndPoint::IpdOrderItem, is_pre_admit),
                    |dom| { dom
                        .child_signal(page.view_by.signal_cloned().map(clone!(page => move |view_by| {
                            (view_by.as_str() == "nurse").then(||
                                html!("li", {
                                    .class(class::NAV_ITEM_PY)
                                    .child(html!("a", {
                                        .class("nav-link")
                                        .class_signal("active", page.active_tab.signal_cloned().map(|tab| matches!(tab, Tab::NursePlan)))
                                        .attr("data-bs-toggle","pill")
                                        .attr("href","#")
                                        .text("Nurse Planning")
                                        .event_with_options(&EventOptions::preventable(), clone!(page => move |event: events::Click| {
                                            event.prevent_default();
                                            page.active_tab.set_neq(Tab::NursePlan);
                                        }))
                                    }))
                                })
                            )
                        })))
                    })
                    .apply_if(allow_lab, |dom| { dom
                        .child(html!("li", {
                            .class(class::NAV_ITEM_PY)
                            .child(html!("a", {
                                .class("nav-link")
                                .class_signal("active", page.active_tab.signal_cloned().map(|tab| matches!(tab, Tab::Lab)))
                                .attr("data-bs-toggle","pill")
                                .attr("href","#")
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
                        if app.has_pacs_host() && allow_xray {
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
                            dom.child_signal(page.hn().map(clone!(app => move |hn| {
                                app.pacs_hn_url(&hn).map(|hn_url| {
                                    doms::nav_item_external_url(&hn_url, "X-Ray ")
                                })
                            })))
                        }
                    })
                    .child_signal(page.hn().map(clone!(app => move |hn| {
                        app.ekg_hn_url(&hn).map(|hn_url| {
                            doms::nav_item_external_url(&hn_url, "EKG ")
                        })
                    })))
                    .child_signal(page.hn().map(clone!(app => move |hn| {
                        app.scan_hn_url(&hn).map(|hn_url| {
                            doms::nav_item_external_url(&hn_url, "Scan ")
                        })
                    })))
                    .apply(|dom| {
                        if let Some(hn) = page.patient.lock_ref().patient.lock_ref().as_ref().and_then(|pt| pt.hn()) {
                            let route = Route::PrescriptionScreen {hn};
                            if route.has_permission(app.state()) {
                                dom.child(html!("li", {
                                    .class(class::NAV_ITEM_PY)
                                    .child(html!("a", {
                                        .class("nav-link")
                                        .attr("data-bs-toggle","pill")
                                        .attr("href","#")
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
                        } else {
                            dom
                        }
                    })
                    .apply_if(
                        app.endpoint_is_allow(&Method::GET, &EndPoint::EmrDateHn, false)
                        && app.endpoint_is_allow(&Method::GET, &EndPoint::EmrVisitVn, false),
                    |dom| dom
                        .children([
                            html!("li", {
                                .class(class::NAV_ITEM_PY)
                                .child(html!("a", {
                                    .class("nav-link")
                                    .class_signal("active", page.active_tab.signal_cloned().map(|tab| matches!(tab, Tab::Emr)))
                                    .attr("data-bs-toggle","pill")
                                    .attr("href","#")
                                    .text("EMR")
                                    .event_with_options(&EventOptions::preventable(), clone!(page => move |event: events::Click| {
                                        event.prevent_default();
                                        page.active_tab.set_neq(Tab::Emr);
                                    }))
                                }))
                            }),
                            html!("li", {
                                .class(class::NAV_ITEM_PY)
                                .child(html!("a", {
                                    .class("nav-link")
                                    .class_signal("active", page.active_tab.signal_cloned().map(|tab| matches!(tab, Tab::Document)))
                                    .attr("data-bs-toggle","pill")
                                    .attr("href","#")
                                    .text("เอกสาร")
                                    .event_with_options(&EventOptions::preventable(), clone!(page => move |event: events::Click| {
                                        event.prevent_default();
                                        page.active_tab.set_neq(Tab::Document);
                                    }))
                                }))
                            }),
                        ])
                    )
                    .apply_if(app.endpoint_is_allow(&Method::GET, &EndPoint::IpdDoctorInCharge, is_pre_admit), |dom| { dom
                        .child_signal(page.view_by.signal_cloned().map(clone!(page => move |view_by| {
                            (view_by.as_str() == "nurse").then(||
                                html!("li", {
                                    .class(class::NAV_ITEM_PY)
                                    .child(html!("a", {
                                        .class("nav-link")
                                        .class_signal("active", page.active_tab.signal_cloned().map(|tab| matches!(tab, Tab::Doctor)))
                                        .attr("data-bs-toggle","pill")
                                        .attr("href","#")
                                        .text("แพทย์เจ้าของไข้")
                                        .event_with_options(&EventOptions::preventable(), clone!(page => move |event: events::Click| {
                                            event.prevent_default();
                                            page.active_tab.set_neq(Tab::Doctor);
                                        }))
                                    }))
                                })
                            )
                        })))
                    })
                    .apply_if(
                        app.endpoint_is_allow(&Method::GET, &EndPoint::IpdConsultAn, is_pre_admit)
                        && app.endpoint_is_allow(&Method::GET, &EndPoint::IpdConsultId, is_pre_admit),
                    |dom| { dom
                        .child_signal(page.view_by.signal_cloned().map(clone!(page => move |view_by| {
                            ["doctor","nurse","pharmacist"].contains(&view_by.as_str()).then(||
                                html!("li", {
                                    .class(class::NAV_ITEM_PY)
                                    .child(html!("a", {
                                        .class("nav-link")
                                        .class_signal("active", page.active_tab.signal_cloned().map(|tab| matches!(tab, Tab::Consult)))
                                        .attr("data-bs-toggle","pill")
                                        .attr("href","#")
                                        .text("Consult")
                                        .event_with_options(&EventOptions::preventable(), clone!(page => move |event: events::Click| {
                                            event.prevent_default();
                                            page.active_tab.set_neq(Tab::Consult);
                                        }))
                                    }))
                                })
                            )
                        })))
                    })
                    .apply_if(app.endpoint_is_allow(&Method::GET, &EndPoint::HisReferOutVnan, is_pre_admit), |dom| { dom
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
                    // .child_signal(page.view_by.signal_cloned().map(clone!(page => move |view_by| {
                    //     (view_by.as_str() == "doctor").then(||
                    //         html!("li", {
                    //             .class(class::NAV_ITEM_PY)
                    //             .child(html!("a", {
                    //                 .class("nav-link")
                    //                 .class_signal("active", page.active_tab.signal_cloned().map(|tab| matches!(tab, Tab::Operation)))
                    //                 .attr("data-bs-toggle","pill")
                    //                 .attr("href","#")
                    //                 .text("ประวัติผ่าตัด")
                    //                 .event_with_options(&EventOptions::preventable(), clone!(page => move |event: events::Click| {
                    //                     event.prevent_default();
                    //                     page.active_tab.set_neq(Tab::Operation);
                    //                 }))
                    //             }))
                    //         })
                    //     )
                    // })))
                    .child_signal(page.view_by.signal_cloned().map(clone!(app => move |view_by| {
                        (view_by.as_str() == "nurse" && app.food_url().is_some()).then(||
                            doms::nav_item_external_url(&app.food_url().unwrap_or_default(), "สั่งอาหาร ")
                        )
                    })))
                    .child_signal(page.view_by.signal_cloned().map(clone!(app, page => move |view_by| {
                        let cart_an_url = app.cart_vnan_url(&page.an.lock_ref());
                        (view_by.as_str() == "nurse" && cart_an_url.is_some()).then(||
                            doms::nav_item_external_url(&cart_an_url.unwrap_or_default(), "ขอเปล ")
                        )
                    })))
                    .child_signal(page.antibiograms.signal_cloned().map(|antibiograms| {
                        (!antibiograms.is_empty()).then(|| doms::antibiogram_dropdown(&antibiograms))
                    }))
                }),
                html!("div", {
                    .class("tab-content")
                    //.attr("id", "pills-tabContent")
                    .child(html!("hr"))
                    .child_signal(page.active_tab.signal_cloned().map(clone!(app, page => move |tab| {
                        let patient_cpn = page.patient.lock_ref();
                        let patient = patient_cpn.patient.clone();
                        Some(match tab {
                            Tab::MedHx => Dom::empty(),
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
                                let order = OrderCpn::new(
                                    true,
                                    patient.clone(),
                                    page.view_by.clone(),
                                    Mutable::new(String::new()),
                                    page.sub.clone(),
                                    page.focused_id.clone(),
                                    app.clone(),
                                );
                                OrderCpn::render("ipd-main", order, app.clone())
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
                            Tab::NurseNote => {
                                let nurse_note = NurseNoteCpn::new(
                                    patient.clone(),
                                    page.view_by.clone(),
                                );
                                NurseNoteCpn::render(nurse_note, app.clone())
                            }
                            Tab::NursePlan => {
                                let index_plan = IndexPlanCpn::new(patient.clone(), page.view_by.clone());
                                IndexPlanCpn::render(index_plan, app.clone())
                            }
                            Tab::Lab => {
                                let lab = LabCpn::new(
                                    patient.clone(),
                                    patient_cpn.hn.clone(),
                                    page.an.clone(),
                                    Some(page.loaded_lab_unread_exists_spinner.clone()),
                                );
                                LabCpn::render("ipd-main", lab, app.clone())
                            }
                            Tab::XRay => {
                                let xray = XrayCpn::new_ipd(
                                    patient_cpn.hn.clone(),
                                    page.an.clone(),
                                    Some(page.loaded_xray_unread_exists_spinner.clone()),
                                );
                                XrayCpn::render("ipd-main", xray, app.clone())
                            }
                            Tab::Emr => {
                                let emr = EmrCpn::new(patient_cpn.hn.clone());
                                EmrCpn::render("ipd-main", emr, app.clone())
                            }
                            Tab::Document => {
                                let document = DocumentCpn::new(
                                    patient_cpn.vn.clone(),
                                    patient.clone(),
                                );
                                DocumentCpn::render(document, app.clone())
                            }
                            Tab::Doctor => {
                                let doctor_in_charge = DoctorInChargeCpn::new(page.an.clone(), patient_cpn.hn.clone());
                                DoctorInChargeCpn::render(doctor_in_charge, app.clone())
                            }
                            Tab::Consult => {
                                let consult = IpdConsultCpn::new(patient.clone(), page.sub.clone(), page.focused_id.clone());
                                IpdConsultCpn::render(consult, app.clone())
                            }
                            Tab::ReferOut => {
                                let referout = ReferOutCpn::new(patient.clone());
                                ReferOutCpn::render(referout, app.clone())
                            }
                        })
                    })))
                }),
            ])
        })
    }
}
