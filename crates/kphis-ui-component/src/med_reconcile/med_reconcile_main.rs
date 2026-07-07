// ipd-dr-med-reconcile.php

use dominator::{Dom, clone, events, html};
use futures_signals::{
    map_ref,
    signal::{Mutable, Signal, SignalExt, always, not},
    signal_vec::{MutableVec, SignalVecExt},
};
use std::rc::Rc;
use web_sys::{HtmlButtonElement, HtmlInputElement};

use kphis_model::{
    app::VisitTypeId,
    endpoint::EndPoint,
    fetch::Method,
    med_reconcile::{MedReconciliation, MedReconciliationItemSave, MedReconciliationParams},
    patient_info::PatientInfo,
    report::{SystemReport, TypstReport},
    tab::Tab,
    user::permission::Permission,
};
use kphis_ui_app::App;
use kphis_ui_core::{class, doms, mixins};
use kphis_util::{
    datetime::date_8601,
    util::{sanity_dot_space, str_some, zero_none},
};

use crate::{
    gadget::{
        pdf_button::PdfButtons,
        searchbox::med::{MedSearchboxCpn, search_drugusage},
    },
    modal::{blank_modal, med_reconcile_remed::MedReconcileRemed},
    order::{MedSearchable, OrderItemMutable},
};

use super::{
    ipd_med_reconcile_hosxp::IpdMedReconcileHosXpCpn, ipd_med_reconcile_last_dose::IpdMedReconcileLastDoseCpn, med_reconcile_form::MedReconForm, med_reconcile_history::MedReconcileHistoryCpn,
};

/// - GET `EndPoint::IpdMedReconcile`
/// - GET `EndPoint::OpdErMedReconcile`
/// - POST `EndPoint::IpdMedReconcile` (guarded, remove 'เพิ่มรายการใหม่', 'ไม่มียาเดิม' btns)
/// - POST `EndPoint::OpdErMedReconcile` (guarded, remove 'เพิ่มรายการใหม่', 'ไม่มียาเดิม' btns)
/// - GET `EndPoint::IpdMedReconcileRemedVisitHn` (`MedReconcileRemed`, guarded, remove remed btn)
/// - GET `EndPoint::IpdMedReconcileRemedMed` (`MedReconcileRemed`, guarded, remove remed btn)
/// - POST `EndPoint::IpdMedReconcile` (`MedReconcileRemed`, guarded, remove remed btn)
/// - POST `EndPoint::OpdErMedReconcile` (`MedReconcileRemed`, guarded, remove remed btn)
/// - GET `EndPoint::SearchBoxMedHnText` (`MedSearchboxCpn`, guarded, remove search btn)
/// - GET `EndPoint::SearchBoxMedDuplicate` (`MedSearchboxCpn`, guarded, remove search btn)
/// - GET `EndPoint::SearchBoxMedInteraction` (`MedSearchboxCpn`, guarded, remove search btn)
#[derive(Default)]
pub struct MedReconcileCpn {
    loaded: Mutable<bool>,

    patient: Mutable<Option<Rc<PatientInfo>>>,

    // from parent
    view_by: Mutable<String>,
    active_tab: Mutable<Tab>,
    loaded_med_reconciliation_count_spinner: Mutable<bool>,
    loaded_med_reconciliation_has_data: Mutable<bool>,

    changed: Mutable<bool>, // impl MedSearchable trait
    checker: Mutable<bool>,

    // for impl MedSearchable trait
    order_id: Mutable<Option<u32>>,
    focused: Mutable<Option<u32>>,
    display_med_searchbox: Mutable<bool>,
    meds: MutableVec<Rc<OrderItemMutable>>,
    search_done: Mutable<bool>,

    icode: Mutable<String>,
    generic_name: Mutable<String>,
    med_name: Mutable<String>,
    medrec_name: Mutable<String>,
    old_drugusage: Mutable<String>,
    receive_qty: Mutable<String>,
    receive_date: Mutable<String>,
    receive_from: Mutable<String>,

    recons_raw: MutableVec<Rc<MedReconciliation>>,
    recons: MutableVec<Rc<MedReconForm>>,
    recons_remove_pending: Mutable<Option<u32>>,
    show_remed_modal: Mutable<bool>,
}

impl MedSearchable for MedReconcileCpn {
    fn order_id(&self) -> Option<u32> {
        self.order_id.get()
    }
    fn an(&self) -> Option<String> {
        match self.patient.lock_ref().as_ref().map(|pt| pt.visit_type()) {
            Some(VisitTypeId::Ipd(an)) | Some(VisitTypeId::PreAdmit(an)) => str_some(an),
            Some(VisitTypeId::OpdEr(_, _)) | Some(VisitTypeId::Visit(_)) | None => None,
        }
    }
    fn focused(&self) -> Mutable<Option<u32>> {
        self.focused.clone()
    }
    fn changed(&self) -> Mutable<bool> {
        self.changed.clone()
    }
    fn display_med_searchbox(&self) -> Mutable<bool> {
        self.display_med_searchbox.clone()
    }
    fn display_homemed_searchbox(&self) -> Mutable<bool> {
        Mutable::new(false)
    }
    fn display_ivfluid_searchbox(&self) -> Mutable<bool> {
        Mutable::new(false)
    }
    fn ivfluids(&self) -> MutableVec<Rc<OrderItemMutable>> {
        MutableVec::new()
    }
    fn meds(&self) -> MutableVec<Rc<OrderItemMutable>> {
        self.meds.clone()
    }
    fn homemeds(&self) -> MutableVec<Rc<OrderItemMutable>> {
        self.meds.clone()
    }
    fn offs(&self) -> MutableVec<Rc<OrderItemMutable>> {
        MutableVec::new()
    }
}

impl MedReconcileCpn {
    pub fn new(
        view_by: Mutable<String>,
        patient: Mutable<Option<Rc<PatientInfo>>>,
        active_tab: Mutable<Tab>,
        loaded_med_reconciliation_count_spinner: Mutable<bool>,
        loaded_med_reconciliation_has_data: Mutable<bool>,
    ) -> Rc<Self> {
        Rc::new(Self {
            view_by,
            patient,
            active_tab,
            loaded_med_reconciliation_count_spinner,
            loaded_med_reconciliation_has_data,
            ..Default::default()
        })
    }

    fn is_ipd(&self) -> impl Signal<Item = bool> + use<> {
        self.patient.signal_ref(|opt| opt.as_ref().map(|pt| pt.visit_type.is_ipd()).unwrap_or_default())
    }

    fn is_ipd_and_is_pre_admit_signal(&self) -> impl Signal<Item = (bool, bool)> + use<> {
        self.patient.signal_ref(|opt| opt.as_ref().map(|pt| pt.visit_type.is_ipd_and_is_pre_admit()).unwrap_or_default())
    }

    fn is_allow_post_signal(&self, app: Rc<App>) -> impl Signal<Item = bool> + use<> {
        self.is_ipd_and_is_pre_admit_signal().map(move |(is_ipd, is_pre_admit)| {
            if is_ipd {
                app.endpoint_is_allow(&Method::POST, &EndPoint::IpdMedReconcile, is_pre_admit)
            } else {
                app.endpoint_is_allow(&Method::POST, &EndPoint::OpdErMedReconcile, false)
            }
        })
    }

    fn is_allow_remed_signal(&self, app: Rc<App>) -> impl Signal<Item = bool> + use<> {
        self.is_ipd_and_is_pre_admit_signal().map(move |(is_ipd, is_pre_admit)| {
            app.endpoint_is_allow(&Method::GET, &EndPoint::IpdMedReconcileRemedVisitHn, false)
                && app.endpoint_is_allow(&Method::GET, &EndPoint::IpdMedReconcileRemedMed, false)
                && if is_ipd {
                    app.endpoint_is_allow(&Method::POST, &EndPoint::IpdMedReconcile, is_pre_admit)
                } else {
                    app.endpoint_is_allow(&Method::POST, &EndPoint::OpdErMedReconcile, false)
                }
        })
    }

    fn ready(&self) -> impl Signal<Item = bool> + use<> {
        map_ref! {
            let med_name = self.med_name.signal_cloned(),
            let medrec_name = self.medrec_name.signal_cloned(),
            let old_drugusage = self.old_drugusage.signal_cloned() =>
            // let receive_qty = self.receive_qty.signal_cloned(),
            // let receive_date = self.receive_date.signal_cloned(),
            // let receive_from = self.receive_from.signal_cloned() =>
            (!med_name.is_empty() || !medrec_name.is_empty()) && !old_drugusage.is_empty()
            // && !receive_qty.is_empty() && !receive_date.is_empty() && !receive_from.is_empty()
        }
    }

    fn clear(page: Rc<Self>) {
        page.icode.set(String::new());
        page.generic_name.set(String::new());
        page.med_name.set(String::new());
        page.medrec_name.set(String::new());
        page.old_drugusage.set(String::new());
        page.receive_qty.set(String::new());
        page.receive_date.set(String::new());
        page.receive_from.set(String::new());
    }

    fn load(page: Rc<Self>, app: Rc<App>) {
        if let Some(patient) = page.patient.get_cloned() {
            page.recons_raw.lock_mut().clear();
            page.recons.lock_mut().clear();
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
                                page.checker.set_neq(!responses.is_empty());
                                page.recons_raw.lock_mut().extend(responses.clone().into_iter().map(Rc::new));
                                page.recons.lock_mut().extend(responses.into_iter().map(|item|Rc::new(MedReconForm::new(
                                    page.view_by.clone(),
                                    Mutable::new(patient.lastdate()),
                                    page.active_tab.clone(),
                                    page.loaded_med_reconciliation_count_spinner.clone(),
                                    page.loaded_med_reconciliation_has_data.clone(),
                                    page.recons_remove_pending.clone(),
                                    item,
                                ))));
                            }
                            Err(e) => {
                                app.alert_app_error(&e).await;
                            }
                        }
                    }
                    page.loaded.set(true);
                }),
            );
        }
    }

    // ipd-dr-med-reconcile-save.php
    fn save_new(page: Rc<Self>, app: Rc<App>) {
        let visit_type = page.patient.lock_ref().as_ref().map(|pt| pt.visit_type());
        app.async_load(
            true,
            clone!(app, page => async move {
                // `med_name` and `custom_med_name` CANNOT EXISTS together
                let med_name = str_some(page.med_name.get_cloned());
                let custom_med_name = if med_name.is_some() {None} else {str_some(page.medrec_name.get_cloned())};
                let items = vec![MedReconciliationItemSave {
                    icode: str_some(page.icode.get_cloned()).or(app.state().hosxp_medrec_icode()),
                    med_name,
                    custom_med_name,
                    receive_from: str_some(page.receive_from.get_cloned()),
                    receive_date: date_8601(&page.receive_date.lock_ref()),
                    old_drugusage: str_some(sanity_dot_space(&page.old_drugusage.lock_ref())),
                    receive_qty: page.receive_qty.lock_ref().parse::<i32>().ok(),
                }];
                let result_opt = match visit_type {
                    Some(VisitTypeId::Ipd(an))
                    | Some(VisitTypeId::PreAdmit(an)) => {
                        let params = MedReconciliationParams {
                            an: str_some(an.to_owned()),
                            ..Default::default()
                        };
                        // POST `EndPoint::IpdMedReconcile`
                        Some(MedReconciliation::call_api_post(true, &items, &params, app.state()).await)
                    }
                    Some(VisitTypeId::OpdEr(_, opd_er_order_master_id)) => {
                        let params = MedReconciliationParams {
                            opd_er_order_master_id: zero_none(opd_er_order_master_id),
                            ..Default::default()
                        };
                        // POST `EndPoint::OpdErMedReconcile`
                        Some(MedReconciliation::call_api_post(false, &items, &params, app.state()).await)
                    }
                    Some(VisitTypeId::Visit(_))
                    | None => None,
                };

                if let Some(result) = result_opt {
                    match result {
                        Ok((_id, responses)) => {
                            app.alert_execute_responses(&responses, async move {
                                // app.alert("บันทึกข้อมูลเรียบร้อย", &["จำนวน", &responses.iter().map(|r| r.rows_affected).sum::<u64>().to_string(), " รายการ"].concat());
                                page.loaded_med_reconciliation_has_data.set_neq(false);
                                page.loaded.set(false);
                                Self::clear(page);
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

    fn save_none(page: Rc<Self>, app: Rc<App>) {
        let visit_type = page.patient.lock_ref().as_ref().map(|pt| pt.visit_type());
        app.async_load(
            true,
            clone!(app, page => async move {
                let items: Vec<MedReconciliationItemSave> = Vec::new();
                let result_opt = match visit_type {
                    Some(VisitTypeId::Ipd(an))
                    | Some(VisitTypeId::PreAdmit(an)) => {
                        let params = MedReconciliationParams {
                            an: str_some(an.to_owned()),
                            ..Default::default()
                        };
                        // POST `EndPoint::IpdMedReconcile`
                        Some(MedReconciliation::call_api_post(true, &items, &params, app.state()).await)
                    }
                    Some(VisitTypeId::OpdEr(_, opd_er_order_master_id)) => {
                        let params = MedReconciliationParams {
                            opd_er_order_master_id: zero_none(opd_er_order_master_id),
                            ..Default::default()
                        };
                        // POST `EndPoint::OpdErMedReconcile`
                        Some(MedReconciliation::call_api_post(false, &items, &params, app.state()).await)
                    }
                    Some(VisitTypeId::Visit(_))
                    | None => None,
                };

                if let Some(result) = result_opt {
                    match result {
                        Ok((_id, responses)) => {
                            app.alert_execute_responses(&responses, async move {
                                // app.alert("บันทึกข้อมูลเรียบร้อย", &["จำนวน", &responses.iter().map(|r| r.rows_affected).sum::<u64>().to_string(), " รายการ"].concat());
                                page.loaded_med_reconciliation_has_data.set_neq(false);
                                page.loaded.set(false);
                                Self::clear(page);
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

    pub fn render(page: Rc<Self>, app: Rc<App>) -> Dom {
        html!("div", {
            .future(map_ref!(
                let busy = app.loader_is_loading(),
                let loaded = page.loaded.signal() =>
                !busy && !loaded
            ).for_each(clone!(app, page => move |ready| {
                if ready {
                    Self::load(page.clone(), app.clone());
                }
                async {}
            })))
            .future(page.meds.signal_vec_cloned().to_signal_cloned().for_each(clone!(page => move |items| {
                if let Some(item) = items.first() {
                    page.icode.set_neq(item.icode.get_cloned().unwrap_or_default());
                    page.generic_name.set_neq(item.generic_name.get_cloned().unwrap_or_default());
                    page.med_name.set_neq(item.med_name.get_cloned().unwrap_or_default());
                    // trim for correcting "  " from ["","",""].join(" ")
                    page.old_drugusage.set_neq(item.order_item_detail.get_cloned().trim().to_owned());
                    page.search_done.set_neq(true);
                }
                async {}
            })))
            .future(page.search_done.signal().for_each(clone!(page => move |done| {
                if done {
                    page.meds.lock_mut().clear();
                    page.search_done.set(false);
                }
                async{}
            })))
            .future(page.recons_remove_pending.signal_cloned().for_each(clone!(page => move |opt| {
                if let Some(id) = opt {
                    page.recons.lock_mut().retain(|recon| recon.med_reconciliation_id.get() != id);
                    page.recons_remove_pending.set(None);
                }
                async{}
            })))
            .child_signal(page.is_ipd().map(clone!(app, page => move |is_ipd| {
                // ipd-dr-med-reconcile-from-hosxp.php
                is_ipd.then(|| IpdMedReconcileHosXpCpn::render(IpdMedReconcileHosXpCpn::new(page.patient.clone()), app.clone()))
            })))
            .child_signal(page.is_ipd().map(clone!(app, page => move |is_ipd| {
                // ipd-dr-med-reconcile-dr-admission-note-last-dose.php
                is_ipd.then(|| IpdMedReconcileLastDoseCpn::render(IpdMedReconcileLastDoseCpn::new(page.patient.clone()), app.clone()))
            })))
            .child(MedReconcileHistoryCpn::render(MedReconcileHistoryCpn::new(page.loaded.clone(), page.patient.clone()), app.clone()))
            .child(html!("div", {
                .class("row")
                .child(html!("div", {
                    .class("col")
                    .child(html!("div", {
                        .class("cards")
                        //.attr("id", "mr_container")
                        .children_signal_vec(page.recons.signal_vec_cloned().map(clone!(app => move |recon| {
                            // ipd-dr-med-reconcile-data.php
                            MedReconForm::render(recon, app.clone())
                        })))
                    }))
                }))
            }))
            .apply_if(app.has_permission(Permission::MedReconciliationAdd), clone!(app, page => move |dom| {
                dom.child(html!("div", {
                    .class("row")
                    .child(html!("div", {
                        .class("col")
                        .child(html!("div", {
                            .class("card")
                            .children([
                                html!("div", {
                                    .class(class::CARD_HEAD)
                                    .class("bg-danger-subtle")
                                    .text("บันทึกรายการใหม่")
                                }),
                                html!("div", {
                                    .class("card-body")
                                    .child_signal(page.patient.signal_cloned().map(clone!(app, page => move |opt| {
                                        opt.map(|pt| {
                                            let med_search_box = MedSearchboxCpn::new(pt.hn(), false);
                                            doms::form_inline(clone!(app, page => move |form| { form
                                                .child(doms::form_inline_group_sm(clone!(app, page => move |group| { group
                                                    .attr("id", "med_input_group")
                                                    .child(html!("label", {
                                                        .prop_signal("for", page.med_name.signal_cloned().map(|med_name| if med_name.is_empty() {"medrec_name"} else {"med_name"}))
                                                        .class("input-group-text")
                                                        .text("ชื่อยา")
                                                    }))
                                                    .child_signal(page.med_name.signal_cloned().map(clone!(page => move |med_name| {
                                                        Some(if med_name.is_empty() {
                                                            html!("input" => HtmlInputElement, {
                                                                .attr("type", "text")
                                                                .attr("maxlength", "255")
                                                                .class("form-control")
                                                                .attr("placeholder", "ยานอกบัญชี รพ.")
                                                                .attr("id", "medrec_name")
                                                                .apply(mixins::string_value(page.medrec_name.clone(), page.changed.clone()))
                                                            })
                                                        } else {
                                                            html!("input", {
                                                                .attr("type", "text")
                                                                .class("form-control")
                                                                .attr("id", "med_name")
                                                                .attr("readonly", "")
                                                                .attr("value", &med_name)
                                                            })
                                                        })
                                                    })))
                                                    .apply_if(
                                                        app.endpoint_is_allow(&Method::GET, &EndPoint::SearchBoxMedHnText , false)
                                                        && app.endpoint_is_allow(&Method::GET, &EndPoint::SearchBoxMedDuplicate , false)
                                                        && app.endpoint_is_allow(&Method::GET, &EndPoint::SearchBoxMedInteraction , false),
                                                    |dom| dom
                                                        .child(html!("button", {
                                                            .attr("type", "button")
                                                            .class(class::BTN_GRAY)
                                                            .children([
                                                                html!("i", {.class(class::FA_PLUS_L)}),
                                                                html!("i", {.class(class::FA_SEARCH)}),
                                                            ])
                                                            .text(" ยาในบัญชี รพ.")
                                                            .event(clone!(page => move |_: events::Click| {
                                                                page.medrec_name.set(String::new());
                                                                page.display_med_searchbox.set_neq(true);
                                                            }))
                                                            // .attr("onclick", "onclick_med_reconciliation_search_button(event, this)")
                                                        }))
                                                    )
                                                    .child(html!("button", {
                                                        .attr("type", "button")
                                                        .class(class::BTN_RED)
                                                        .child(html!("i", {.class(class::FA_X)}))
                                                        .event(clone!(page => move |_:events::Click| {
                                                            Self::clear(page.clone());
                                                        }))
                                                        // .attr("onclick", "onclick_clear_med_reconciliation_search_button(event, this)")
                                                    }))
                                                })))
                                                .child_signal(page.display_med_searchbox.signal_cloned().map(clone!(app, page, med_search_box => move |show| {
                                                    if show {
                                                        app.get_id("med_input_group").map(|elm| {
                                                            doms::under_box(
                                                                elm.get_bounding_client_rect(),
                                                                600.0, 300.0, app.window_scroll_y(),
                                                                clone!(app, page, med_search_box => move |bx| { bx
                                                                    .child(MedSearchboxCpn::render(None, med_search_box, page, app))
                                                                })
                                                            )
                                                        })
                                                    } else {
                                                        None
                                                    }
                                                })))
                                                .children(MedSearchboxCpn::render_modals(med_search_box))
                                                .child(doms::form_inline_group_sm(clone!(page => move |group| { group
                                                    .attr("id", "old_drugusage_input_group")
                                                    .children([
                                                        doms::label_group_for("old_drugusage","วิธีใช้"),
                                                        html!("input" => HtmlInputElement, {
                                                            .attr("type", "text")
                                                            .class("form-control")
                                                            .style("width","410px")
                                                            .attr("id", "old_drugusage")
                                                            .attr("placeholder", "เช่น 13pt หรือ ระบุวิธีใช้ยา")
                                                            .attr("required", "")
                                                            .apply(mixins::string_value(page.old_drugusage.clone(), page.changed.clone()))
                                                        }),
                                                        html!("button", {
                                                            .class(class::BTN_SM_GRAY)
                                                            .attr("type", "button")
                                                            .child(html!("i", {.class(class::FA_X)}))
                                                            .event(clone!(page => move |_: events::Click| {
                                                                page.old_drugusage.set_neq(String::new());
                                                                page.changed.set_neq(false);
                                                            }))
                                                        }),
                                                    ])
                                                })))
                                                .child_signal(page.old_drugusage.signal_cloned().map(clone!(app, page => move |old_drugusage| {
                                                    if old_drugusage.is_empty() || old_drugusage.chars().any(|c| !c.is_ascii()) {
                                                        None
                                                    } else {
                                                        app.get_id("old_drugusage_input_group").map(|elm| {
                                                            doms::under_box(
                                                                elm.get_bounding_client_rect(),
                                                                600.0, 300.0, app.window_scroll_y(),
                                                                clone!(app, page => move |bx| { bx
                                                                    .child(html!("div", {
                                                                        .class(class::CARD_CYANS)
                                                                        .style("height", "294px")
                                                                        .child(search_drugusage("294px", page.old_drugusage.clone(), app.clone()))
                                                                    }))
                                                                })
                                                            )
                                                        })
                                                    }
                                                })))
                                                .children([
                                                    doms::form_inline_group_sm(clone!(page => move |group| { group
                                                        .children([
                                                            doms::label_group_for("receive_qty","จำนวนที่ได้รับ"),
                                                            html!("input" => HtmlInputElement, {
                                                                .attr("type", "number")
                                                                .class("form-control")
                                                                .attr("id", "receive_qty")
                                                                .apply(mixins::string_value(page.receive_qty.clone(), page.changed.clone()))
                                                            }),
                                                        ])
                                                    })),
                                                    doms::form_inline_group_sm(clone!(page => move |group| { group
                                                        .children([
                                                            doms::label_group_for("receive_date","วันที่ได้รับยา"),
                                                            doms::date_picker(
                                                                page.receive_date.clone(),
                                                                page.changed.clone(), always(false), None,
                                                                |d| d.class(class::FLEX_GROW1).style("min-width","120px"),
                                                                |d| d.class(class::FORM_CTRL_ONLY_SM_R0_L),
                                                                |d| d.class(class::FORM_CTRL_ONLY_SM_R0_L).attr("id","receive_date"),
                                                                |s| s, always(None),
                                                            ),
                                                        ])
                                                    })),
                                                    doms::form_inline_group_sm(clone!(app, page => move |group| { group
                                                        .children([
                                                            doms::label_group_for("receive_from","สถานพยาบาล"),
                                                            html!("input" => HtmlInputElement, {
                                                                .attr("type", "text")
                                                                .attr("maxlength", "255")
                                                                .class("form-control")
                                                                .attr("id", "receive_from")
                                                                .attr("required", "")
                                                                .apply(mixins::string_value(page.receive_from.clone(), page.changed.clone()))
                                                            }),
                                                            html!("button", {
                                                                .attr("type", "button")
                                                                .class(class::BTN_SM_GRAY)
                                                                .text("รับยาที่นี่")
                                                                .event(clone!(app, page => move |_:events::Click| {
                                                                    let hosp_name = app.app_status.lock_ref().as_ref().map(|aps| aps.hospital_name.clone()).unwrap_or_default();
                                                                    if hosp_name != page.receive_from.get_cloned() {
                                                                        page.receive_from.set(hosp_name);
                                                                        page.changed.set_neq(true);
                                                                    }
                                                                }))
                                                            }),
                                                        ])
                                                    })),
                                                    doms::form_inline_end(clone!(app, page => move |end| { end
                                                        .child_signal(page.is_allow_post_signal(app.clone()).map(clone!(app, page => move |is_allow| {
                                                            is_allow.then(|| {
                                                                html!("button" => HtmlButtonElement, {
                                                                    .attr("type", "button")
                                                                    .class(class::BTN_L)
                                                                    .class_signal("btn-primary", page.ready())
                                                                    .class_signal("btn-secondary", not(page.ready()))
                                                                    .child(html!("i", {.class(class::FA_PLUS)}))
                                                                    .text(" เพิ่มรายการใหม่")
                                                                    .apply(mixins::click_with_loader_checked_or_true_disable_signal(clone!(app, page => move || {
                                                                        Self::save_new(page.clone(), app.clone());
                                                                    }), not(page.ready()), app.state()))
                                                                })
                                                            })
                                                        })))
                                                        .child_signal(page.is_allow_remed_signal(app.clone()).map(clone!(app, page => move |is_allow| {
                                                            is_allow.then(|| {
                                                                html!("button", {
                                                                    .attr("type", "button")
                                                                    .class(class::BTN_L_BLUE)
                                                                    .attr("data-bs-toggle", "modal")
                                                                    .attr("data-bs-target","#medReconciliationRemedModal")
                                                                    .child(html!("i", {.class(class::FA_PLUS_L)}))
                                                                    .child(html!("i", {.class(class::FA_SEARCH)}))
                                                                    .text(" Remed")
                                                                    .event(clone!(page => move |_:events::Click| {
                                                                        page.show_remed_modal.set(true);
                                                                    }))
                                                                })
                                                            })
                                                        })))
                                                        .child_signal(map_ref!{
                                                            let is_empty = page.recons_raw.signal_vec_cloned().is_empty(),
                                                            let is_allow = page.is_allow_post_signal(app.clone()) =>
                                                            *is_empty && *is_allow
                                                        }.map(clone!(app, page => move |ready| {
                                                            ready.then(|| {
                                                                html!("button" => HtmlButtonElement, {
                                                                    .attr("type", "button")
                                                                    .class(class::BTN_CYAN)
                                                                    .text("ไม่มียาเดิม")
                                                                    .apply(mixins::click_with_loader_checked(clone!(app, page => move || {
                                                                        Self::save_none(page.clone(), app.clone());
                                                                    }), app.state()))
                                                                })
                                                            })
                                                        })))
                                                    })),
                                                ])
                                            }))
                                        })
                                    })))
                                }),
                            ])
                        }))
                    }))
                }))
            }))
            .child_signal(page.patient.signal_cloned().map(clone!(page, app => move |patient| {
                match patient.as_ref().map(|pt| pt.visit_type()) {
                    Some(VisitTypeId::Ipd(an))
                    | Some(VisitTypeId::PreAdmit(an)) => {
                        Some(html!("div", {
                            .class(class::FLOAT_RB)
                            .children(PdfButtons::buttons(
                                PdfButtons::new(
                                    TypstReport::from_system_with_coercion(SystemReport::IpdMedReconciliation, &app.state().report_coercions()),
                                    Mutable::new(an.clone()),
                                    page.checker.clone(),
                                    page.changed.clone(),
                                    clone!(page => move || {serde_json::json!({
                                        "id": an,
                                        "patient": patient,
                                        "recon":  page.recons_raw.lock_ref().to_vec(),
                                    }).to_string()})
                                ), "Print PDF", None, app.clone()
                            ))
                        }))
                    }
                    Some(VisitTypeId::OpdEr(vn, _)) => {
                        Some(html!("div", {
                            .class(class::FLOAT_RB)
                            .children(PdfButtons::buttons(
                                PdfButtons::new(
                                    TypstReport::from_system_with_coercion(SystemReport::OpdErMedReconciliation, &app.state().report_coercions()),
                                    Mutable::new(vn.clone()),
                                    page.checker.clone(),
                                    page.changed.clone(),
                                    clone!(page => move || {serde_json::json!({
                                        "id": vn,
                                        "patient": patient,
                                        "recon":  page.recons_raw.lock_ref().to_vec(),
                                    }).to_string()})
                                ), "Print PDF", None, app.clone()
                            ))
                        }))
                    }
                    Some(VisitTypeId::Visit(_))
                    | None => None,
                }
            })))
            // ipd-dr-med-reconcile-remed.php
            .child(html!("div", {
                .class("modal")
                .attr("id", "medReconciliationRemedModal")
                .attr("role", "dialog")
                .attr("tabindex", "-1")
                .child_signal(page.show_remed_modal.signal_cloned().map(clone!(app, page => move |show| {
                    show.then(|| {
                        let modal = MedReconcileRemed::new(
                            page.patient.clone(),
                            page.show_remed_modal.clone(),
                            page.loaded.clone(),
                            page.loaded_med_reconciliation_has_data.clone(),
                        );
                        MedReconcileRemed::render(modal, app.clone())
                    }).or(Some(blank_modal()))
                })))
            }))
        })
    }
}
