use dominator::{Dom, clone, events, html};
use futures_signals::{
    map_ref,
    signal::{Mutable, Signal, SignalExt, and, not, or},
    signal_vec::{MutableVec, SignalVecExt},
};
use std::rc::Rc;
use web_sys::{HtmlButtonElement, HtmlInputElement, HtmlTextAreaElement};

use kphis_model::{
    app::VisitTypeId,
    endpoint::EndPoint,
    fetch::Method,
    med_reconcile::{MedReconciliation, MedReconciliationParams},
    order::{MedOrderItem, Order, OrderButtons, OrderSave, OrderTypeName},
    patient_info::PatientInfo,
    pre_order::order::PreOrderSave,
};
use kphis_ui_app::App;
use kphis_ui_core::{class, doms, mixins};
use kphis_util::{
    datetime::js_now,
    util::{str_some, zero_none},
};

use crate::{
    gadget::searchbox::{
        ivfluid::IvfluidSearchboxCpn,
        lab::LabSearchboxCpn,
        med::{MedSearchboxCpn, search_drugusage},
        xray::XraySearchboxCpn,
    },
    order::{InsertTextAreaButton, MedSearchable, OrderItemMutable},
};

/// - POST `EndPoint::IpdPreOrderOrder`
/// - POST `EndPoint::IpdOrderOrder`
/// - POST `EndPoint::OpdErOrderOrder`
/// - GET `EndPoint::IpdOrderOnedayPreviousAn` (guarded,remove 'Yesterday Order' btn)
/// - GET `EndPoint::IpdOrderToHomeMedAn` (guarded,remove 'Cont. Order' btn)
/// - GET `EndPoint::IpdMedReconcile` (guarded, remove 'Used MR'/'Held MR'/'Offed MR' btns)
/// - GET `EndPoint::OpdErMedReconcile` (guarded, remove 'Used MR'/'Held MR'/'Offed MR' btns)
/// - GET `EndPoint::SearchBoxIvfluidText` (IvfluidSearchboxCpn, guarded, remove iv-search btn)
/// - GET `EndPoint::SearchBoxLabText` (LabSearchboxCpn, guarded, remove lab-search btn)
/// - GET `EndPoint::SearchBoxMedHnText` (MedSearchboxCpn, guarded, remove med/homemed-search btn)
/// - GET `EndPoint::SearchBoxMedDuplicate` (MedSearchboxCpn, guarded, remove med/homemed-search btn)
/// - GET `EndPoint::SearchBoxMedInteraction` (MedSearchboxCpn, guarded, remove med/homemed-search btn)
/// - GET `EndPoint::SearchBoxXrayText` (XraySearchboxCpn, guarded, remove xray-search btn)
#[derive(Default)]
pub struct OneDayForm {
    view_by: Mutable<String>,
    patient: Mutable<Option<Rc<PatientInfo>>>,
    pre_order_master_id: Mutable<Option<u32>>,

    buttons_loaded: Mutable<bool>,
    to_scroll: Mutable<bool>,
    note_buttons: MutableVec<Rc<InsertTextAreaButton>>,
    lab_buttons: MutableVec<Rc<InsertTextAreaButton>>,
    xray_buttons: MutableVec<Rc<InsertTextAreaButton>>,
    ivfluid_buttons: MutableVec<Rc<InsertTextAreaButton>>,
    serial_buttons: MutableVec<Rc<InsertTextAreaButton>>,
    record_buttons: MutableVec<Rc<InsertTextAreaButton>>,
    retain_buttons: MutableVec<Rc<InsertTextAreaButton>>,
    other_buttons: MutableVec<Rc<InsertTextAreaButton>>,
    pharm_buttons: MutableVec<Rc<InsertTextAreaButton>>,
    discharge_buttons: MutableVec<Rc<InsertTextAreaButton>>,

    order_id: Mutable<Option<u32>>,
    order_doctor: Mutable<String>,

    changed: Mutable<bool>,
    focused: Mutable<Option<u32>>,

    notes: MutableVec<Rc<OrderItemMutable>>,
    offs: MutableVec<Rc<OrderItemMutable>>,
    offs_by_parent: MutableVec<Rc<OrderItemMutable>>,
    labs: MutableVec<Rc<OrderItemMutable>>,
    xrays: MutableVec<Rc<OrderItemMutable>>,
    ivfluids: MutableVec<Rc<OrderItemMutable>>,
    serials: MutableVec<Rc<OrderItemMutable>>,
    records: MutableVec<Rc<OrderItemMutable>>,
    meds: MutableVec<Rc<OrderItemMutable>>,
    retains: MutableVec<Rc<OrderItemMutable>>,
    others: MutableVec<Rc<OrderItemMutable>>,
    pharms: MutableVec<Rc<OrderItemMutable>>,
    discharges: MutableVec<Rc<OrderItemMutable>>,
    home_medications: MutableVec<Rc<OrderItemMutable>>,

    display_lab_searchbox: Mutable<bool>,
    display_xray_searchbox: Mutable<bool>,
    display_ivfluid_searchbox: Mutable<bool>,
    display_med_searchbox: Mutable<bool>,
    display_homemed_searchbox: Mutable<bool>,
}

impl MedSearchable for OneDayForm {
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
        self.display_homemed_searchbox.clone()
    }
    fn display_ivfluid_searchbox(&self) -> Mutable<bool> {
        self.display_ivfluid_searchbox.clone()
    }
    fn ivfluids(&self) -> MutableVec<Rc<OrderItemMutable>> {
        self.ivfluids.clone()
    }
    fn meds(&self) -> MutableVec<Rc<OrderItemMutable>> {
        self.meds.clone()
    }
    fn homemeds(&self) -> MutableVec<Rc<OrderItemMutable>> {
        self.home_medications.clone()
    }
    fn offs(&self) -> MutableVec<Rc<OrderItemMutable>> {
        self.offs.clone()
    }
}

impl OneDayForm {
    pub fn new(
        order_opt: Option<Rc<Order>>,
        patient: Mutable<Option<Rc<PatientInfo>>>,
        pre_order_master_id: Option<u32>,
        order_doctor: String,
        view_by: Mutable<String>,
        offs_by_parent: MutableVec<Rc<OrderItemMutable>>,
    ) -> Rc<Self> {
        let order_form = Rc::new(Self {
            view_by,
            patient,
            pre_order_master_id: Mutable::new(pre_order_master_id),
            order_doctor: Mutable::new(order_doctor),
            offs_by_parent,
            ..Default::default()
        });
        if let Some(order) = order_opt {
            order_form.order_id.set(Some(order.order_id));
            for order_item_type in order.order_item_types.clone() {
                match order_item_type.order_item_type {
                    OrderTypeName::Note => order_form.notes.lock_mut().extend(order_item_type.order_items.into_iter().map(|i| Rc::new(i.into()))),
                    OrderTypeName::Off => order_form.offs.lock_mut().extend(order_item_type.order_items.into_iter().map(|i| Rc::new(i.into()))),
                    OrderTypeName::Lab => order_form.labs.lock_mut().extend(order_item_type.order_items.into_iter().map(|i| Rc::new(i.into()))),
                    OrderTypeName::Xray => order_form.xrays.lock_mut().extend(order_item_type.order_items.into_iter().map(|i| Rc::new(i.into()))),
                    OrderTypeName::Ivfluid => order_form.ivfluids.lock_mut().extend(order_item_type.order_items.into_iter().map(|i| Rc::new(i.into()))),
                    OrderTypeName::Serial => order_form.serials.lock_mut().extend(order_item_type.order_items.into_iter().map(|i| Rc::new(i.into()))),
                    OrderTypeName::Record => order_form.records.lock_mut().extend(order_item_type.order_items.into_iter().map(|i| Rc::new(i.into()))),
                    OrderTypeName::Injection | OrderTypeName::Med => order_form.meds.lock_mut().extend(order_item_type.order_items.into_iter().map(|i| Rc::new(i.into()))),
                    //  => order_form.meds.lock_mut().extend(order_item_type.order_items.into_iter().map(|i| Rc::new(i.into()))),
                    OrderTypeName::Retain => order_form.retains.lock_mut().extend(order_item_type.order_items.into_iter().map(|i| Rc::new(i.into()))),
                    OrderTypeName::Other => order_form.others.lock_mut().extend(order_item_type.order_items.into_iter().map(|i| Rc::new(i.into()))),
                    OrderTypeName::Pharm => order_form.pharms.lock_mut().extend(order_item_type.order_items.into_iter().map(|i| Rc::new(i.into()))),
                    OrderTypeName::Discharge => order_form.discharges.lock_mut().extend(order_item_type.order_items.into_iter().map(|i| Rc::new(i.into()))),
                    OrderTypeName::HomeMedication => order_form.home_medications.lock_mut().extend(order_item_type.order_items.into_iter().map(|i| Rc::new(i.into()))),
                    OrderTypeName::Food | OrderTypeName::Activity => order_form.others.lock_mut().extend(order_item_type.order_items.into_iter().map(|i| Rc::new(i.into()))),
                }
            }
            order_form.changed.set(true);
        };
        // if let Some((off_id, detail)) = off_detail {
        //     let order_item = OrderItemMutable::new("off", pre_order_master_id);
        //     order_item.order_item_detail.set(detail);
        //     order_item.off_order_item_id.set(Some(off_id));
        //     order_form.offs.lock_mut().push_cloned(order_item);
        //     order_form.changed.set(true);
        // }

        order_form
    }

    fn allow_from_med_rec_signal(&self, app: Rc<App>) -> impl Signal<Item = bool> + use<> {
        self.patient.signal_cloned().map(move |opt| {
            opt.map(|pt| {
                let (is_ipd, is_pre_admit) = pt.visit_type.is_ipd_and_is_pre_admit();
                if is_ipd {
                    app.endpoint_is_allow(&Method::GET, &EndPoint::IpdMedReconcile, is_pre_admit)
                } else {
                    app.endpoint_is_allow(&Method::GET, &EndPoint::OpdErMedReconcile, false)
                }
            })
            .unwrap_or_default()
        })
    }

    fn load_buttons(page: Rc<Self>, app: Rc<App>) {
        app.async_load(
            true,
            clone!(app, page => async move {
                match OrderButtons::get("one-day", app.state()).await {
                    Ok(buttons) => for button in buttons {
                        match button.word_type.as_str() {
                            "note" => page.note_buttons.lock_mut().extend(button.buttons.into_iter().map(|button| Rc::new(InsertTextAreaButton::from_button("note", button)))),
                            "lab" => page.lab_buttons.lock_mut().extend(button.buttons.into_iter().map(|button| Rc::new(InsertTextAreaButton::from_button("lab", button)))),
                            "xray" => page.xray_buttons.lock_mut().extend(button.buttons.into_iter().map(|button| Rc::new(InsertTextAreaButton::from_button("xray", button)))),
                            "ivfluid" => page.ivfluid_buttons.lock_mut().extend(button.buttons.into_iter().map(|button| Rc::new(InsertTextAreaButton::from_button("ivfluid", button)))),
                            "serial" => page.serial_buttons.lock_mut().extend(button.buttons.into_iter().map(|button| Rc::new(InsertTextAreaButton::from_button("serial", button)))),
                            "record" => page.record_buttons.lock_mut().extend(button.buttons.into_iter().map(|button| Rc::new(InsertTextAreaButton::from_button("record", button)))),
                            "retain" => page.retain_buttons.lock_mut().extend(button.buttons.into_iter().map(|button| Rc::new(InsertTextAreaButton::from_button("retain", button)))),
                            "other" => page.other_buttons.lock_mut().extend(button.buttons.into_iter().map(|button| Rc::new(InsertTextAreaButton::from_button("other", button)))),
                            "pharm" => page.pharm_buttons.lock_mut().extend(button.buttons.into_iter().map(|button| Rc::new(InsertTextAreaButton::from_button("pharm", button)))),
                            "discharge" => page.discharge_buttons.lock_mut().extend(button.buttons.into_iter().map(|button| Rc::new(InsertTextAreaButton::from_button("discharge", button)))),
                            _ => {}
                        }
                        page.to_scroll.set(true);
                    }
                    Err(e) => {
                        app.alert_app_error(&e).await;
                    }
                }
            }),
        )
    }

    fn load_one_day_previous(page: Rc<Self>, app: Rc<App>) {
        match page.patient.lock_ref().as_ref().map(|pt| pt.visit_type()) {
            Some(VisitTypeId::Ipd(an)) | Some(VisitTypeId::PreAdmit(an)) => {
                app.async_load(
                    true,
                    clone!(app, page => async move {
                        // GET `EndPoint::IpdOrderOnedayPreviousAn`
                        match MedOrderItem::call_api_get_ipd_oneday_previous(&an, app.state()).await {
                            Ok(prevs) => {
                                if !prevs.is_empty() {
                                    for prev in prevs {
                                        if let Some(order_item_type) = &prev.order_item_type {
                                            match order_item_type.as_str() {
                                                "note" => page.notes.lock_mut().push_cloned(Rc::new(OrderItemMutable::from(prev))),
                                                "lab"  => page.labs.lock_mut().push_cloned(Rc::new(OrderItemMutable::from(prev))),
                                                "xray" => page.xrays.lock_mut().push_cloned(Rc::new(OrderItemMutable::from(prev))),
                                                "ivfluid" => page.ivfluids.lock_mut().push_cloned(Rc::new(OrderItemMutable::from(prev))),
                                                "serial" => page.serials.lock_mut().push_cloned(Rc::new(OrderItemMutable::from(prev))),
                                                "record" => page.records.lock_mut().push_cloned(Rc::new(OrderItemMutable::from(prev))),
                                                "injection" | "med" => page.meds.lock_mut().push_cloned(Rc::new(OrderItemMutable::from(prev))),
                                                //  => page.meds.lock_mut().push_cloned(Rc::new(OrderItemMutable::from(prev))),
                                                "retain" => page.retains.lock_mut().push_cloned(Rc::new(OrderItemMutable::from(prev))),
                                                "other" => page.others.lock_mut().push_cloned(Rc::new(OrderItemMutable::from(prev))),
                                                "pharm" => page.pharms.lock_mut().push_cloned(Rc::new(OrderItemMutable::from(prev))),
                                                "discharge" => page.discharges.lock_mut().push_cloned(Rc::new(OrderItemMutable::from(prev))),
                                                "home-medication" => page.home_medications.lock_mut().push_cloned(Rc::new(OrderItemMutable::from(prev))),
                                                _ => {}
                                            }
                                        }
                                    }
                                    page.changed.set_neq(true);
                                }
                            }
                            Err(e) => {
                                app.alert_app_error(&e).await;
                            }
                        }
                    }),
                );
            }
            Some(VisitTypeId::OpdEr(_, _)) | Some(VisitTypeId::Visit(_)) | None => {}
        }
    }

    fn load_home_med_from_cont(page: Rc<Self>, app: Rc<App>) {
        match page.patient.lock_ref().as_ref().map(|pt| pt.visit_type()) {
            Some(VisitTypeId::Ipd(an)) | Some(VisitTypeId::PreAdmit(an)) => {
                app.async_load(
                    true,
                    clone!(app, page => async move {
                        // GET `EndPoint::IpdOrderToHomeMedAn`
                        match MedOrderItem::call_api_get_ipd_cont_to_home_med(&an, app.state()).await {
                            Ok(meds) => {
                                if !meds.is_empty() {
                                    page.home_medications.lock_mut().extend(meds.into_iter().map(|mut med| {
                                        med.order_item_type = Some(String::from("home-medication"));
                                        Rc::new(OrderItemMutable::from(med))
                                    }));
                                    page.changed.set_neq(true);
                                }
                            }
                            Err(e) => {
                                app.alert_app_error(&e).await;
                            }
                        }
                    }),
                );
            }
            Some(VisitTypeId::OpdEr(_, _)) | Some(VisitTypeId::Visit(_)) | None => {}
        }
    }

    fn load_home_med_from_med_rec(used: String, page: Rc<Self>, app: Rc<App>) {
        if let Some(patient) = page.patient.get_cloned() {
            app.async_load(
                true,
                clone!(app, page => async move {
                    let result_opt = match &patient.visit_type {
                        VisitTypeId::Ipd(an)
                        | VisitTypeId::PreAdmit(an) => {
                            let params = MedReconciliationParams {
                                hn: patient.hn(),
                                an: str_some(an.to_owned()),
                                used: Some(used),
                                ..Default::default()
                            };
                            // GET `EndPoint::IpdMedReconcile`
                            Some(MedReconciliation::call_api_get(true, &params, app.state()).await)
                        }
                        VisitTypeId::OpdEr(_, opd_er_order_master_id) => {
                            let params = MedReconciliationParams {
                                hn: patient.hn(),
                                opd_er_order_master_id: zero_none(*opd_er_order_master_id),
                                used: Some(used),
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
                                page.home_medications.lock_mut().extend(responses.into_iter().flat_map(|res| res.med_reconciliation_items).map(|item| {
                                    Rc::new(OrderItemMutable::from(item))
                                }));
                                page.changed.set_neq(true);
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

    pub fn render(page: Rc<Self>, parent_show_oneday_input: Mutable<bool>, parent_edit_order: Mutable<Option<Rc<Order>>>, parent_reload_order_oneday: Mutable<bool>, app: Rc<App>) -> Dom {
        let injections = app.app_status.lock_ref().as_ref().map(|status| status.hosxp_injection_dosageforms.clone()).unwrap_or_default();
        let allow_med_searchbox = app.endpoint_is_allow(&Method::GET, &EndPoint::SearchBoxMedHnText, false)
            && app.endpoint_is_allow(&Method::GET, &EndPoint::SearchBoxMedDuplicate, false)
            && app.endpoint_is_allow(&Method::GET, &EndPoint::SearchBoxMedInteraction, false);

        html!("div", {
            .future(map_ref!(
                let busy = app.loader_is_loading(),
                let buttons_loaded = page.buttons_loaded.signal_cloned() =>
                !busy && !buttons_loaded
            ).for_each(clone!(app, page => move |ready| {
                if ready {
                    Self::load_buttons(page.clone(), app.clone());
                    page.buttons_loaded.set(true);
                }
                async {}
            })))
            .future(page.to_scroll.signal().for_each(clone!(app, page => move |scroll| {
                if scroll {
                    app.scroll_into_view("addOneDayFormContainer");
                    page.to_scroll.set(false);
                }
                async {}
            })))
            .attr("id", "addOneDayFormContainer")
            .child(html!("div", {
                .child_signal(page.patient.signal_cloned().map(clone!(app, page => move |opt| {
                    let is_doctor = page.view_by.lock_ref().as_str() == "doctor";
                    opt.and_then(|pt| {
                        let (is_ipd, is_pre_admit) = pt.visit_type.is_ipd_and_is_pre_admit();
                        (is_doctor && is_ipd && app.endpoint_is_allow(&Method::GET, &EndPoint::IpdOrderOnedayPreviousAn, is_pre_admit)).then(|| {
                            html!("button" => HtmlButtonElement, {
                                .attr("type", "button")
                                //.attr("id", "add_previous_oneday_order_button")
                                .class(class::BTN_SM_RT_GRAY)
                                .children([
                                    html!("i", {.class(class::FA_PLUS_L)}),
                                    html!("i", {.class(class::FA_BOLT_L)}),
                                    html!("i", {.class(class::FA_CLOCK_L_ROTATE)}),
                                ])
                                .text(" Yesterday Order")
                                .apply(mixins::click_with_loader_checked(clone!(app, page => move || {
                                    Self::load_one_day_previous(page.clone(), app.clone());
                                }), app.state()))
                                // onclick="onclick_add_previous_oneday_order_button_oneDayForm(event)
                                // call ipd-dr-order-previous-one-day-order-data.php
                            })
                        })
                    })
                })))
                .children([
                    html!("div", {
                        .class("mb-2")
                        .children([
                            html!("div", {
                                //.attr("id", "note")
                                .class("mb-1")
                                .children([
                                    html!("span", {
                                        .class("fw-bold")
                                        //.attr("id", "oneDayFormNoteLabel")
                                        .text("Note")
                                    }),
                                    html!("button", {
                                        .attr("type", "button")
                                        .class(class::BTN_SM_RT_GRAY)
                                        .child(html!("i", {.class(class::FA_PLUS)}))
                                        .event(clone!(page => move |_: events::Click| {
                                            let item = OrderItemMutable::new("note", page.pre_order_master_id.get());
                                            page.focused.set_neq(Some(item.id));
                                            page.notes.lock_mut().push_cloned(item);
                                        }))
                                    }),
                                    html!("div", {
                                        //.attr("id", "oneDayForm-note-input-div")
                                        .children_signal_vec(page.notes.signal_vec_cloned().map(clone!(page => move |note| {
                                            html!("div", {
                                                .child(html!("div", {
                                                    .class(class::INPUT_GROUP_TR)
                                                    .children([
                                                        html!("textarea" => HtmlTextAreaElement, {
                                                            .class(class::FORM_CTRL_SM)
                                                            .attr("id", &["textarea-", &note.id.to_string()].concat())
                                                            .apply(mixins::textarea_value_auto_expand(note.order_item_detail.clone(), page.changed.clone()))
                                                            .event(clone!(page, note => move |_: events::Focus| {
                                                                page.focused.set_neq(Some(note.id));
                                                            }))
                                                        }),
                                                        html!("button", {
                                                            .class(class::BTN_SM_RED)
                                                            .attr("type", "button")
                                                            .child(html!("i", {.class(class::FA_MINUS)}))
                                                            .event(clone!(page => move |_: events::Click| {
                                                                page.notes.lock_mut().retain(|x| *x != note);
                                                                page.changed.set_neq(true);
                                                            }))
                                                        }),
                                                    ])
                                                }))
                                            })
                                        })))
                                    }),
                                    html!("div", {
                                        //.attr("id", "note_option")
                                        .children_signal_vec(page.note_buttons.signal_vec_cloned()
                                            .map(clone!(app, page => move |btn| InsertTextAreaButton::render(btn, page.notes.clone(), page.focused.clone(), page.changed.clone(), app.clone()))
                                        ))
                                    }),
                                ])
                            }),
                            html!("div", {
                                //.attr("id", "off")
                                .class("mb-1")
                                .child_signal(map_ref!{
                                    let no_offs_by_parent = page.offs_by_parent.signal_vec_cloned().is_empty(),
                                    let no_offs = page.offs.signal_vec_cloned().is_empty() =>
                                    !no_offs_by_parent || !no_offs
                                }.map(|has_offs| {
                                    has_offs.then(|| {
                                        html!("span", {
                                            .class("fw-bold")
                                            //.attr("id", "oneDayFormOffLabel")
                                            .text("Off")
                                        })
                                    })
                                }))
                                .child(html!("div", {
                                    //.attr("id", "oneDayForm-off-input-div")
                                    .children_signal_vec(page.offs_by_parent.signal_vec_cloned().map(clone!(page => move |off| {
                                        html!("div", {
                                            .child(html!("div", {
                                                .class(class::INPUT_GROUP_TR)
                                                .children([
                                                    html!("textarea" => HtmlTextAreaElement, {
                                                        .class(class::FORM_CTRL_SM)
                                                        .attr("id", &["textarea-", &off.id.to_string()].concat())
                                                        .attr("readonly", "readonly")
                                                        .style("cursor","pointer")
                                                        .prop_signal("value", off.order_item_detail.signal_cloned())
                                                        .event(clone!(page, off => move |_: events::Focus| {
                                                            page.focused.set_neq(Some(off.id));
                                                        }))
                                                        //.attr("onfocus", "onfocusContinuousText(event,'off',this)")
                                                    }),
                                                    html!("button", {
                                                        .class(class::BTN_SM_RED)
                                                        .attr("type", "button")
                                                        .child(html!("i", {.class(class::FA_MINUS)}))
                                                        .event(clone!(page  => move |_: events::Click| {
                                                            page.offs_by_parent.lock_mut().retain(|x| *x != off);
                                                            page.changed.set_neq(true);
                                                        }))
                                                    }),
                                                ])
                                            }))
                                        })
                                    })))
                                    .children_signal_vec(page.offs.signal_vec_cloned().map(clone!(page => move |off| {
                                        html!("div", {
                                            .child(html!("div", {
                                                .class(class::INPUT_GROUP_TR)
                                                .children([
                                                    html!("textarea" => HtmlTextAreaElement, {
                                                        .class(class::FORM_CTRL_SM)
                                                        .attr("id", &["textarea-", &off.id.to_string()].concat())
                                                        .attr("readonly", "readonly")
                                                        .style("cursor","pointer")
                                                        .prop_signal("value", off.order_item_detail.signal_cloned())
                                                        .event(clone!(page, off => move |_: events::Focus| {
                                                            page.focused.set_neq(Some(off.id));
                                                        }))
                                                        //.attr("onfocus", "onfocusContinuousText(event,'off',this)")
                                                    }),
                                                    html!("button", {
                                                        .class(class::BTN_SM_RED)
                                                        .attr("type", "button")
                                                        .child(html!("i", {.class(class::FA_MINUS)}))
                                                        .event(clone!(page  => move |_: events::Click| {
                                                            page.offs.lock_mut().retain(|x| *x != off);
                                                            page.changed.set_neq(true);
                                                        }))
                                                    }),
                                                ])
                                            }))
                                        })
                                    })))
                                }))
                            }),
                            html!("div", {
                                //.attr("id", "lab")
                                .class("mb-1")
                                .children([
                                    html!("span", {
                                        .class("fw-bold")
                                        //.attr("id", "oneDayForm-lab-label")
                                        .text("Lab")
                                    }),
                                    html!("button", {
                                        .attr("type", "button")
                                        .class(class::BTN_SM_RT_GRAY)
                                        .child(html!("i", {.class(class::FA_PLUS)}))
                                        .event(clone!(page => move |_: events::Click| {
                                            let item = OrderItemMutable::new("lab", page.pre_order_master_id.get());
                                            page.focused.set_neq(Some(item.id));
                                            page.labs.lock_mut().push_cloned(item);
                                        }))
                                    }),
                                ])
                                .apply_if(app.endpoint_is_allow(&Method::GET, &EndPoint::SearchBoxLabText, false), |dom| dom
                                    .child(html!("button", {
                                        .attr("type", "button")
                                        .class(class::BTN_SM_RT_GRAY)
                                        .child(html!("i", {.class(class::FA_PLUS_L)}))
                                        .child(html!("i", {.class(class::FA_SEARCH)}))
                                        .event(clone!(page => move |_: events::Click| {
                                            page.display_lab_searchbox.set_neq(true);
                                        }))
                                    }))
                                )
                                .child_signal(page.display_lab_searchbox.signal_cloned().map(clone!(app, page => move |show| {
                                    show.then(|| LabSearchboxCpn::render(
                                        page.pre_order_master_id.get(),
                                        LabSearchboxCpn::new(),
                                        page.display_lab_searchbox.clone(),
                                        page.labs.clone(),
                                        page.focused.clone(),
                                        page.changed.clone(),
                                        app.clone(),
                                    ))
                                })))
                                .children([
                                    html!("div", {
                                        //.attr("id", "oneDayForm-lab-input-div")
                                        .children_signal_vec(page.labs.signal_vec_cloned().map(clone!(page => move |lab| {
                                            html!("div", {
                                                .child(html!("div", {
                                                    .class(class::INPUT_GROUP_TR)
                                                    .children([
                                                        html!("textarea" => HtmlTextAreaElement, {
                                                            .class(class::FORM_CTRL_SM)
                                                            .attr("id", &["textarea-", &lab.id.to_string()].concat())
                                                            .apply(mixins::textarea_value_auto_expand(lab.order_item_detail.clone(), page.changed.clone()))
                                                            .event(clone!(page, lab => move |_: events::Focus| {
                                                                page.focused.set_neq(Some(lab.id));
                                                            }))
                                                        }),
                                                        html!("button", {
                                                            .class(class::BTN_SM_RED)
                                                            .attr("type", "button")
                                                            .child(html!("i", {.class(class::FA_MINUS)}))
                                                            .event(clone!(page => move |_: events::Click| {
                                                                page.labs.lock_mut().retain(|x| *x != lab);
                                                                page.changed.set_neq(true);
                                                            }))
                                                        }),
                                                    ])
                                                }))
                                            })
                                        })))
                                    }),
                                    html!("div", {
                                        //.attr("id", "lab_option")
                                        .children_signal_vec(page.lab_buttons.signal_vec_cloned()
                                            .map(clone!(app, page => move |btn| InsertTextAreaButton::render(btn, page.labs.clone(), page.focused.clone(), page.changed.clone(), app.clone()))
                                        ))
                                    }),
                                ])
                            }),
                            html!("div", {
                                //.attr("id", "xray")
                                .class("mb-1")
                                .children([
                                    html!("span", {
                                        .class("fw-bold")
                                        //.attr("id", "oneDayForm-xray-label")
                                        .text("X-Ray")
                                    }),
                                    html!("button", {
                                        .attr("type", "button")
                                        .class(class::BTN_SM_RT_GRAY)
                                        .child(html!("i", {.class(class::FA_PLUS)}))
                                        .event(clone!(page => move |_: events::Click| {
                                            let item = OrderItemMutable::new("xray", page.pre_order_master_id.get());
                                            page.focused.set_neq(Some(item.id));
                                            page.xrays.lock_mut().push_cloned(item);
                                        }))
                                    }),
                                ])
                                .apply_if(app.endpoint_is_allow(&Method::GET, &EndPoint::SearchBoxXrayText, false), |dom| dom
                                    .child(html!("button", {
                                        .attr("type", "button")
                                        .class(class::BTN_SM_RT_GRAY)
                                        .children([
                                            html!("i", {.class(class::FA_PLUS_L)}),
                                            html!("i", {.class(class::FA_SEARCH)}),
                                        ])
                                        .event(clone!(page => move |_: events::Click| {
                                            page.display_xray_searchbox.set_neq(true);
                                        }))
                                    }))
                                )
                                .child_signal(page.display_xray_searchbox.signal_cloned().map(clone!(app, page => move |show| {
                                    show.then(|| XraySearchboxCpn::render(
                                        page.pre_order_master_id.get(),
                                        XraySearchboxCpn::new(),
                                        page.display_xray_searchbox.clone(),
                                        page.xrays.clone(),
                                        page.focused.clone(),
                                        page.changed.clone(),
                                        app.clone(),
                                    ))
                                })))
                                .children([
                                    html!("div", {
                                        //.attr("id", "oneDayForm-xray-input-div")
                                        .children_signal_vec(page.xrays.signal_vec_cloned().map(clone!(page => move |xray| {
                                            html!("div", {
                                                .child(html!("div", {
                                                    .class(class::INPUT_GROUP_TR)
                                                    .children([
                                                        html!("textarea" => HtmlTextAreaElement, {
                                                            .class(class::FORM_CTRL_SM)
                                                            .attr("id", &["textarea-", &xray.id.to_string()].concat())
                                                            .apply(mixins::textarea_value_auto_expand(xray.order_item_detail.clone(), page.changed.clone()))
                                                            .event(clone!(page, xray => move |_: events::Focus| {
                                                                page.focused.set_neq(Some(xray.id));
                                                            }))
                                                        }),
                                                        html!("button", {
                                                            .class(class::BTN_SM_RED)
                                                            .attr("type", "button")
                                                            .child(html!("i", {.class(class::FA_MINUS)}))
                                                            .event(clone!(page => move |_: events::Click| {
                                                                page.xrays.lock_mut().retain(|x| *x != xray);
                                                                page.changed.set_neq(true);
                                                            }))
                                                        }),
                                                    ])
                                                }))
                                            })
                                        })))
                                    }),
                                    html!("div", {
                                        //.attr("id", "xray_option")
                                        .children_signal_vec(page.xray_buttons.signal_vec_cloned()
                                            .map(clone!(app, page => move |btn| InsertTextAreaButton::render(btn, page.xrays.clone(), page.focused.clone(), page.changed.clone(), app.clone()))
                                        ))
                                    }),
                                ])
                            }),
                            html!("div", {
                                //.attr("id", "ivfluid")
                                .class("mb-1")
                                .child(html!("span", {
                                    .class("fw-bold")
                                    //.attr("id", "oneDayForm-fluid-label")
                                    .text("IV fluid")
                                }))
                                .apply_if(app.endpoint_is_allow(&Method::GET, &EndPoint::SearchBoxIvfluidText, false), |dom| dom
                                    .child(html!("button", {
                                        .attr("type", "button")
                                        .class(class::BTN_SM_RT_GRAY)
                                        .children([
                                            html!("i", {.class(class::FA_PLUS_L)}),
                                            html!("i", {.class(class::FA_SEARCH)}),
                                        ])
                                        .event(clone!(page => move |_: events::Click| {
                                            page.display_ivfluid_searchbox.set_neq(true);
                                        }))
                                    }))
                                )
                                .child_signal(page.display_ivfluid_searchbox.signal_cloned().map(clone!(app, page => move |show| {
                                    show.then(|| IvfluidSearchboxCpn::render(
                                        page.pre_order_master_id.get(),
                                        IvfluidSearchboxCpn::new(),
                                        page.clone(),
                                        app.clone(),
                                    ))
                                })))
                                .children([
                                    html!("div", {
                                        //.attr("id", "oneDayForm-ivfluid-input-div")
                                        .children_signal_vec(page.ivfluids.signal_vec_cloned().map(clone!(page => move |ivfluid| {
                                            html!("div", {
                                                .class(class::BOX_ROUND_P1_T)
                                                .class("bg-info-subtle")
                                                .children([
                                                    html!("input", {
                                                        .attr("type", "text")
                                                        .class(class::FORM_CTRL_SM_T)
                                                        .attr("readonly", "readonly")
                                                        .prop_signal("value", ivfluid.med_name.signal_cloned().map(|name| name.unwrap_or_default()))
                                                    }),
                                                    html!("div", {
                                                        .class(class::INPUT_GROUP_TR)
                                                        .children([
                                                            html!("textarea" => HtmlTextAreaElement, {
                                                                .class(class::FORM_CTRL_SM)
                                                                .attr("id", &["textarea-", &ivfluid.id.to_string()].concat())
                                                                .attr("placeholder", "วิธีใช้สารน้ำ")
                                                                .apply(mixins::textarea_value_auto_expand(ivfluid.order_item_detail.clone(), page.changed.clone()))
                                                                .event(clone!(page, ivfluid => move |_: events::Focus| {
                                                                    page.focused.set_neq(Some(ivfluid.id));
                                                                }))
                                                            }),
                                                            html!("button", {
                                                                .class(class::BTN_SM_RED)
                                                                .attr("type", "button")
                                                                .child(html!("i", {.class(class::FA_MINUS)}))
                                                                .event(clone!(page => move |_: events::Click| {
                                                                    page.ivfluids.lock_mut().retain(|x| *x != ivfluid);
                                                                    page.changed.set_neq(true);
                                                                }))
                                                            }),
                                                        ])
                                                    }),
                                                ])
                                            })
                                        })))
                                    }),
                                    html!("div", {
                                        //.attr("id", "ivfluid_option")
                                        .children_signal_vec(page.ivfluid_buttons.signal_vec_cloned()
                                            .map(clone!(app, page => move |btn| InsertTextAreaButton::render_maybe_adder(
                                                btn,
                                                page.ivfluids.clone(),
                                                page.ivfluids.clone(),
                                                page.focused.clone(),
                                                page.changed.clone(),
                                                page.pre_order_master_id.get(),
                                                app.clone(),
                                            ))
                                        ))
                                    }),
                                ])
                            }),
                            html!("div", {
                                //.attr("id", "serial")
                                .class("mb-1")
                                .children([
                                    html!("span", {
                                        .class("fw-bold")
                                        //.attr("id", "oneDayFormSerialLabel")
                                        .text("Serial")
                                    }),
                                    html!("button", {
                                        .attr("type", "button")
                                        .class(class::BTN_SM_RT_GRAY)
                                        .child(html!("i", {.class(class::FA_PLUS)}))
                                        .event(clone!(page => move |_: events::Click| {
                                            let item = OrderItemMutable::new("serial", page.pre_order_master_id.get());
                                            page.focused.set_neq(Some(item.id));
                                            page.serials.lock_mut().push_cloned(item);
                                        }))
                                    }),
                                        html!("div", {
                                            //.attr("id", "oneDayForm-serial-input-div")
                                            .children_signal_vec(page.serials.signal_vec_cloned().map(clone!(page => move |serial| {
                                            html!("div", {
                                                .child(html!("div", {
                                                    .class(class::INPUT_GROUP_TR)
                                                    .children([
                                                        html!("textarea" => HtmlTextAreaElement, {
                                                            .class(class::FORM_CTRL_SM)
                                                            .attr("id", &["textarea-", &serial.id.to_string()].concat())
                                                            .apply(mixins::textarea_value_auto_expand(serial.order_item_detail.clone(), page.changed.clone()))
                                                            .event(clone!(page, serial => move |_: events::Focus| {
                                                                page.focused.set_neq(Some(serial.id));
                                                            }))
                                                        }),
                                                        html!("div", {
                                                            .class("input-group-text")
                                                            .children([
                                                                html!("input" => HtmlInputElement, {
                                                                    .attr("type", "checkbox")
                                                                    .attr("id", &["checkbox-", &serial.id.to_string()].concat())
                                                                    .apply(mixins::checkbox_toggle(serial.stat.clone(), page.changed.clone(), "Y", "N"))
                                                                }),
                                                                html!("label", {
                                                                    .class(class::FORM_CHK_LBL_R)
                                                                    .attr("for", &["checkbox-", &serial.id.to_string()].concat())
                                                                    .style("user-select","none")
                                                                    .text("Stat")
                                                                }),
                                                            ])
                                                        }),
                                                        html!("button", {
                                                            .class(class::BTN_SM_RED)
                                                            .attr("type", "button")
                                                            .child(html!("i", {.class(class::FA_MINUS)}))
                                                            .event(clone!(page => move |_: events::Click| {
                                                                page.serials.lock_mut().retain(|x| *x != serial);
                                                                page.changed.set_neq(true);
                                                            }))
                                                        }),
                                                    ])
                                                }))
                                            })
                                        })))
                                    }),
                                    html!("div", {
                                        //.attr("id", "serial_option")
                                        .children_signal_vec(page.serial_buttons.signal_vec_cloned()
                                            .map(clone!(app, page => move |btn| InsertTextAreaButton::render(btn, page.serials.clone(), page.focused.clone(), page.changed.clone(), app.clone()))
                                        ))
                                    }),
                                ])
                            }),
                            html!("div", {
                                //.attr("id", "record")
                                .class("mb-1")
                                .children([
                                    html!("span", {
                                        .class("fw-bold")
                                        //.attr("id", "oneDayForm-record-label")
                                        .text("Record")
                                    }),
                                    html!("button", {
                                        .attr("type", "button")
                                        .class(class::BTN_SM_RT_GRAY)
                                        .child(html!("i", {.class(class::FA_PLUS)}))
                                        .event(clone!(page => move |_: events::Click| {
                                            let item = OrderItemMutable::new("record", page.pre_order_master_id.get());
                                            page.focused.set_neq(Some(item.id));
                                            page.records.lock_mut().push_cloned(item);
                                        }))
                                    }),
                                    html!("div", {
                                        //.attr("id", "oneDayForm-record-input-div")
                                        .children_signal_vec(page.records.signal_vec_cloned().map(clone!(page => move |record| {
                                            html!("div", {
                                                .child(html!("div", {
                                                    .class(class::INPUT_GROUP_TR)
                                                    .children([
                                                        html!("textarea" => HtmlTextAreaElement, {
                                                            .class(class::FORM_CTRL_SM)
                                                            .attr("id", &["textarea-", &record.id.to_string()].concat())
                                                            .apply(mixins::textarea_value_auto_expand(record.order_item_detail.clone(), page.changed.clone()))
                                                            .event(clone!(page, record => move |_: events::Focus| {
                                                                page.focused.set_neq(Some(record.id));
                                                            }))
                                                        }),
                                                        html!("button", {
                                                            .class(class::BTN_SM_RED)
                                                            .attr("type", "button")
                                                            .child(html!("i", {.class(class::FA_MINUS)}))
                                                            .event(clone!(page => move |_: events::Click| {
                                                                page.records.lock_mut().retain(|x| *x != record);
                                                                page.changed.set_neq(true);
                                                            }))
                                                        }),
                                                    ])
                                                }))
                                            })
                                        })))
                                    }),
                                    html!("div", {
                                        //.attr("id", "record_option")
                                        .children_signal_vec(page.record_buttons.signal_vec_cloned()
                                            .map(clone!(app, page => move |btn| InsertTextAreaButton::render(btn, page.records.clone(), page.focused.clone(), page.changed.clone(), app.clone()))
                                        ))
                                    }),
                                ])
                            }),
                            html!("div", {
                                //.attr("id", "med")
                                .class("mb-1")
                                .child(html!("span", {
                                    .class("fw-bold")
                                    //.attr("id", "oneDayFormMedLabel")
                                    .text("Medication")
                                }))
                                .apply_if(allow_med_searchbox, |dom| dom
                                    .child(html!("button", {
                                        .attr("type", "button")
                                        .class(class::BTN_SM_RT_GRAY)
                                        .children([
                                            html!("i", {.class(class::FA_PLUS_L)}),
                                            html!("i", {.class(class::FA_SEARCH)}),
                                        ])
                                        .event(clone!(page => move |_: events::Click| {
                                            page.display_med_searchbox.set_neq(true);
                                        }))
                                    }))
                                )
                                .child_signal(page.display_med_searchbox.signal_cloned().map(clone!(app, page => move |show| {
                                    show.then(|| {
                                        let hn = page.patient.lock_ref().as_ref().and_then(|pt| pt.hn());
                                        let searchbox = MedSearchboxCpn::new(hn, false);
                                        html!("div", {
                                            .child(MedSearchboxCpn::render(page.pre_order_master_id.get(), searchbox.clone(), page.clone(), app.clone()))
                                            .children(MedSearchboxCpn::render_modals(searchbox))
                                        })
                                    })
                                })))
                                .child(html!("div", {
                                    //.attr("id", "oneDayForm-med-input-div")
                                    .children_signal_vec(page.meds.signal_vec_cloned().map(clone!(app, page, injections => move |med| {
                                        let is_injection = med.dosageform.lock_ref().as_ref().map(|dosageform| injections.contains(dosageform)).unwrap_or_default();
                                        html!("div", {
                                            .class(class::BOX_ROUND_P1_T)
                                            .apply(|dom| {
                                                if is_injection {
                                                    dom.class("bg-danger-subtle")
                                                } else {
                                                    dom.class("bg-info-subtle")
                                                }
                                            })
                                            .children([
                                                html!("div", {
                                                    .class(class::INPUT_GROUP_SM_T)
                                                    .child_signal(med.med_reconciliation_item_id.signal().map(clone!(med => move |opt| {
                                                        opt.is_some().then(|| {
                                                            html!("span", {
                                                                .class("input-group-text")
                                                                .style("cursor","help")
                                                                .apply(|dom| {
                                                                    match med.used.lock_ref().as_ref() {
                                                                        Some(used) => {
                                                                            match used.as_str() {
                                                                                "N" => dom.class(class::BOLD_BG_GRAY),
                                                                                "H" => dom.class(class::BOLD_BG_CYAN),
                                                                                "Y" => {
                                                                                    if med.is_med_rec_change_usage() {
                                                                                        dom.class(class::BOLD_BG_GOLD)
                                                                                    } else {
                                                                                        dom.class(class::BOLD_BG_GREEN)
                                                                                    }
                                                                                }
                                                                                _ => dom,
                                                                            }
                                                                        }
                                                                        None => dom,
                                                                    }

                                                                })
                                                                .attr("title", &med.med_rec_info())
                                                                .text("MR")
                                                            })
                                                        })
                                                    })))
                                                    .children([
                                                        html!("input", {
                                                            .attr("type", "text")
                                                            .class("form-control")
                                                            .attr("readonly", "readonly")
                                                            .prop_signal("value", med.med_name.signal_cloned().map(|name| name.unwrap_or_default()))
                                                        }),
                                                        html!("button", {
                                                            .class(class::BTN_SM_RED)
                                                            .attr("type", "button")
                                                            .child(html!("i", {.class(class::FA_MINUS)}))
                                                            .event(clone!(page, med => move |_: events::Click| {
                                                                page.meds.lock_mut().retain(|x| *x != med);
                                                                page.changed.set_neq(true);
                                                            }))
                                                        }),
                                                    ])
                                                }),
                                                html!("div", {
                                                    .class(class::INPUT_GROUP)
                                                    .children([
                                                        html!("textarea" => HtmlTextAreaElement, {
                                                            .class(class::FORM_CTRL_SM)
                                                            .attr("id", &["textarea-", &med.id.to_string()].concat())
                                                            .attr("placeholder", "เช่น 13pt หรือ ระบุวิธีใช้ยา")
                                                            .apply(mixins::textarea_value_auto_expand(med.order_item_detail.clone(), page.changed.clone()))
                                                            .event(clone!(page, med => move |_: events::Focus| {
                                                                page.focused.set_neq(Some(med.id));
                                                            }))
                                                        }),
                                                        html!("button", {
                                                            .class(class::BTN_SM_GRAY)
                                                            .attr("type", "button")
                                                            .child(html!("i", {.class(class::FA_X)}))
                                                            .event(clone!(page, med => move |_: events::Click| {
                                                                med.order_item_detail.set_neq(String::new());
                                                                page.changed.set_neq(false);
                                                            }))
                                                        }),
                                                        html!("div", {
                                                            .class("input-group-text")
                                                            .children([
                                                                html!("input" => HtmlInputElement, {
                                                                    .attr("type", "checkbox")
                                                                    .attr("id", &["checkbox-", &med.id.to_string()].concat())
                                                                    .apply(mixins::checkbox_toggle(med.stat.clone(), page.changed.clone(), "Y", "N"))
                                                                }),
                                                                html!("label", {
                                                                    .class(class::FORM_CHK_LBL_R)
                                                                    .attr("for", &["checkbox-", &med.id.to_string()].concat())
                                                                    .style("user-select","none")
                                                                    .text("Stat")
                                                                }),
                                                            ])
                                                        }),
                                                    ])
                                                }),
                                                search_drugusage("280px", med.order_item_detail.clone(), app.clone()),
                                            ])
                                            .child_signal(map_ref!{
                                                let is_due = med.due_status.signal_ref(|opt| opt.as_ref().map(|due_status| due_status == "Y").unwrap_or_default()),
                                                let has_info = med.info_status.signal_ref(|opt| opt.as_ref().map(|info_status| info_status == "Y").unwrap_or_default()) =>
                                                (*is_due, *has_info)
                                            }.map(clone!(med => move |(is_due, has_info)| {
                                                if is_due {
                                                    Some(html!("div", {
                                                        .class("p-2")
                                                        .style("white-space","pre-wrap")
                                                        .children(doms::square_bracket_to_span(&med.due_usage.get_cloned().unwrap_or_default()))
                                                    }))
                                                } else if has_info {
                                                    Some(html!("div", {
                                                        .class("p-2")
                                                        .style("white-space","pre-wrap")
                                                        .children(doms::square_bracket_to_span(&med.info.get_cloned().unwrap_or_default()))
                                                    }))
                                                } else {
                                                    None
                                                }
                                            })))
                                        })
                                    })))
                                }))
                            }),
                            html!("div", {
                                //.attr("id", "retain")
                                .class("mb-1")
                                .children([
                                    html!("span", {
                                        .class("fw-bold")
                                        //.attr("id", "oneDayFormRetainLabel")
                                        .text("Retain")
                                    }),
                                    html!("button", {
                                        .attr("type", "button")
                                        .class(class::BTN_SM_RT_GRAY)
                                        .child(html!("i", {.class(class::FA_PLUS)}))
                                        .event(clone!(page => move |_: events::Click| {
                                            let item = OrderItemMutable::new("retain", page.pre_order_master_id.get());
                                            page.focused.set_neq(Some(item.id));
                                            page.retains.lock_mut().push_cloned(item);
                                        }))
                                    }),
                                    html!("div", {
                                        //.attr("id", "oneDayForm-retain-input-div")
                                        .children_signal_vec(page.retains.signal_vec_cloned().map(clone!(page => move |retain| {
                                            html!("div", {
                                                .child(html!("div", {
                                                    .class(class::INPUT_GROUP_TR)
                                                    .children([
                                                        html!("textarea" => HtmlTextAreaElement, {
                                                            .class(class::FORM_CTRL_SM)
                                                            .attr("id", &["textarea-", &retain.id.to_string()].concat())
                                                            .apply(mixins::textarea_value_auto_expand(retain.order_item_detail.clone(), page.changed.clone()))
                                                            .event(clone!(page, retain => move |_: events::Focus| {
                                                                page.focused.set_neq(Some(retain.id));
                                                            }))
                                                        }),
                                                        html!("button", {
                                                            .class(class::BTN_SM_RED)
                                                            .attr("type", "button")
                                                            .child(html!("i", {.class(class::FA_MINUS)}))
                                                            .event(clone!(page => move |_: events::Click| {
                                                                page.retains.lock_mut().retain(|x| *x != retain);
                                                                page.changed.set_neq(true);
                                                            }))
                                                        }),
                                                    ])
                                                }))
                                            })
                                        })))
                                    }),
                                    html!("div", {
                                        //.attr("id", "retain_option")
                                        .children_signal_vec(page.retain_buttons.signal_vec_cloned()
                                            .map(clone!(app, page => move |btn| InsertTextAreaButton::render(btn, page.retains.clone(), page.focused.clone(), page.changed.clone(), app.clone()))
                                        ))
                                    }),
                                ])
                            }),
                            html!("div", {
                                //.attr("id", "other")
                                .class("mb-1")
                                .children([
                                    html!("span", {
                                        .class("fw-bold")
                                        //.attr("id", "oneDayFormOtherLabel")
                                        .text("Other")
                                    }),
                                    html!("button", {
                                        .attr("type", "button")
                                        .class(class::BTN_SM_RT_GRAY)
                                        .child(html!("i", {.class(class::FA_PLUS)}))
                                        .event(clone!(page => move |_: events::Click| {
                                            let item = OrderItemMutable::new("other", page.pre_order_master_id.get());
                                            page.focused.set_neq(Some(item.id));
                                            page.others.lock_mut().push_cloned(item);
                                        }))
                                    }),
                                        html!("div", {
                                            //.attr("id", "oneDayForm-other-input-div")
                                            .children_signal_vec(page.others.signal_vec_cloned().map(clone!(page => move |other| {
                                            html!("div", {
                                                .child(html!("div", {
                                                    .class(class::INPUT_GROUP_TR)
                                                    .children([
                                                        html!("textarea" => HtmlTextAreaElement, {
                                                            .class(class::FORM_CTRL_SM)
                                                            .attr("id", &["textarea-", &other.id.to_string()].concat())
                                                            .apply(mixins::textarea_value_auto_expand(other.order_item_detail.clone(), page.changed.clone()))
                                                            .event(clone!(page, other => move |_: events::Focus| {
                                                                page.focused.set_neq(Some(other.id));
                                                            }))
                                                        }),
                                                        html!("div", {
                                                            .class("input-group-text")
                                                            .children([
                                                                html!("input" => HtmlInputElement, {
                                                                    .attr("type", "checkbox")
                                                                    .attr("id", &["checkbox-", &other.id.to_string()].concat())
                                                                    .apply(mixins::checkbox_toggle(other.stat.clone(), page.changed.clone(), "Y", "N"))
                                                                }),
                                                                html!("label", {
                                                                    .class(class::FORM_CHK_LBL_R)
                                                                    .attr("for", &["checkbox-", &other.id.to_string()].concat())
                                                                    .style("user-select","none")
                                                                    .text("Stat")
                                                                }),
                                                            ])
                                                        }),
                                                        html!("button", {
                                                            .class(class::BTN_SM_RED)
                                                            .attr("type", "button")
                                                            .child(html!("i", {.class(class::FA_MINUS)}))
                                                            .event(clone!(page => move |_: events::Click| {
                                                                page.others.lock_mut().retain(|x| *x != other);
                                                                page.changed.set_neq(true);
                                                            }))
                                                        }),
                                                    ])
                                                }))
                                            })
                                        })))
                                    }),
                                    html!("div", {
                                        //.attr("id", "other_option")
                                        .children_signal_vec(page.other_buttons.signal_vec_cloned()
                                            .map(clone!(app, page => move |btn| InsertTextAreaButton::render(btn, page.others.clone(), page.focused.clone(), page.changed.clone(), app.clone()))
                                        ))
                                    }),
                                ])
                            }),
                            html!("div", {
                                //.attr("id", "pharm")
                                .class("mb-1")
                                .children([
                                    html!("span", {
                                        .class("fw-bold")
                                        //.attr("id", "oneDayFormPharmLabel")
                                        .text("Pharmacist Notify")
                                    }),
                                    html!("button", {
                                        .attr("type", "button")
                                        .class(class::BTN_SM_RT_GRAY)
                                        .child(html!("i", {.class(class::FA_PLUS)}))
                                        .event(clone!(page => move |_: events::Click| {
                                            let item = OrderItemMutable::new("pharm", page.pre_order_master_id.get());
                                            page.focused.set_neq(Some(item.id));
                                            page.pharms.lock_mut().push_cloned(item);
                                        }))
                                    }),
                                    html!("div", {
                                        //.attr("id", "oneDayForm-pharm-input-div")
                                        .children_signal_vec(page.pharms.signal_vec_cloned().map(clone!(page => move |pharm| {
                                            html!("div", {
                                                .child(html!("div", {
                                                    .class(class::INPUT_GROUP_TR)
                                                    .children([
                                                        html!("textarea" => HtmlTextAreaElement, {
                                                            .class(class::FORM_CTRL_SM)
                                                            .attr("id", &["textarea-", &pharm.id.to_string()].concat())
                                                            .apply(mixins::textarea_value_auto_expand(pharm.order_item_detail.clone(), page.changed.clone()))
                                                            .event(clone!(page, pharm => move |_: events::Focus| {
                                                                page.focused.set_neq(Some(pharm.id));
                                                            }))
                                                        }),
                                                        html!("div", {
                                                            .class("input-group-text")
                                                            .children([
                                                                html!("input" => HtmlInputElement, {
                                                                    .attr("type", "checkbox")
                                                                    .attr("id", &["checkbox-", &pharm.id.to_string()].concat())
                                                                    .apply(mixins::checkbox_toggle(pharm.stat.clone(), page.changed.clone(), "Y", "N"))
                                                                }),
                                                                html!("label", {
                                                                    .class(class::FORM_CHK_LBL_R)
                                                                    .attr("for", &["checkbox-", &pharm.id.to_string()].concat())
                                                                    .style("user-select","none")
                                                                    .text("Stat")
                                                                }),
                                                            ])
                                                        }),
                                                        html!("button", {
                                                            .class(class::BTN_SM_RED)
                                                            .attr("type", "button")
                                                            .child(html!("i", {.class(class::FA_MINUS)}))
                                                            .event(clone!(page => move |_: events::Click| {
                                                                page.pharms.lock_mut().retain(|x| *x != pharm);
                                                                page.changed.set_neq(true);
                                                            }))
                                                        }),
                                                    ])
                                                }))
                                            })
                                        })))
                                    }),
                                    html!("div", {
                                        //.attr("id", "pharm_option")
                                        .children_signal_vec(page.pharm_buttons.signal_vec_cloned()
                                            .map(clone!(app, page => move |btn| InsertTextAreaButton::render(btn, page.pharms.clone(), page.focused.clone(), page.changed.clone(), app.clone()))
                                        ))
                                    }),
                                ])
                            }),
                            html!("div", {
                                //.attr("id", "discharge")
                                .class("mb-1")
                                .children([
                                    html!("span", {
                                        .class("fw-bold")
                                        //.attr("id", "oneDayFormDischargeLabel")
                                        .text("Discharge")
                                    }),
                                    html!("button", {
                                        .attr("type", "button")
                                        .class(class::BTN_SM_RT_GRAY)
                                        .child(html!("i", {.class(class::FA_PLUS)}))
                                        .event(clone!(page => move |_: events::Click| {
                                            let item = OrderItemMutable::new("discharge", page.pre_order_master_id.get());
                                            page.focused.set_neq(Some(item.id));
                                            page.discharges.lock_mut().push_cloned(item);
                                        }))
                                    }),
                                    html!("div", {
                                        //.attr("id", "oneDayForm-discharge-input-div")
                                        .children_signal_vec(page.discharges.signal_vec_cloned().map(clone!(page => move |discharge| {
                                            html!("div", {
                                                .child(html!("div", {
                                                    .class(class::INPUT_GROUP_TR)
                                                    .children([
                                                        html!("textarea" => HtmlTextAreaElement, {
                                                            .class(class::FORM_CTRL_SM)
                                                            .attr("id", &["textarea-", &discharge.id.to_string()].concat())
                                                            .apply(mixins::textarea_value_auto_expand(discharge.order_item_detail.clone(), page.changed.clone()))
                                                            .event(clone!(page, discharge => move |_: events::Focus| {
                                                                page.focused.set_neq(Some(discharge.id));
                                                            }))
                                                        }),
                                                        html!("div", {
                                                            .class("input-group-text")
                                                            .children([
                                                                html!("input" => HtmlInputElement, {
                                                                    .attr("type", "checkbox")
                                                                    .attr("id", &["checkbox-", &discharge.id.to_string()].concat())
                                                                    .apply(mixins::checkbox_toggle(discharge.stat.clone(), page.changed.clone(), "Y", "N"))
                                                                }),
                                                                html!("label", {
                                                                    .class(class::FORM_CHK_LBL_R)
                                                                    .attr("for", &["checkbox-", &discharge.id.to_string()].concat())
                                                                    .style("user-select","none")
                                                                    .text("Stat")
                                                                }),
                                                            ])
                                                        }),
                                                        html!("button", {
                                                            .class(class::BTN_SM_RED)
                                                            .attr("type", "button")
                                                            .child(html!("i", {.class(class::FA_MINUS)}))
                                                            .event(clone!(page => move |_: events::Click| {
                                                                page.discharges.lock_mut().retain(|x| *x != discharge);
                                                                page.changed.set_neq(true);
                                                            }))
                                                        }),
                                                    ])
                                                }))
                                            })
                                        })))
                                    }),
                                    html!("div", {
                                        //.attr("id", "discharge_option")
                                        .children_signal_vec(page.discharge_buttons.signal_vec_cloned()
                                            .map(clone!(app, page => move |btn| InsertTextAreaButton::render(btn, page.discharges.clone(), page.focused.clone(), page.changed.clone(), app.clone()))
                                        ))
                                    }),
                                ])
                            }),
                            html!("div", {
                                //.attr("id", "home-medication")
                                .class("mb-1")
                                .child(html!("span", {
                                    .class("fw-bold")
                                    //.attr("id", "oneDayFormHomeMedLabel")
                                    .text("Home Medication")
                                }))
                                .apply_if(allow_med_searchbox, |dom| dom
                                    .child(html!("button", {
                                        .attr("type", "button")
                                        .class(class::BTN_SM_RT_GRAY)
                                        .children([
                                            html!("i", {.class(class::FA_PLUS_L)}),
                                            html!("i", {.class(class::FA_SEARCH)}),
                                        ])
                                        .event(clone!(page => move |_: events::Click| {
                                            page.display_homemed_searchbox.set_neq(true);
                                        }))
                                    }))
                                )
                                .child_signal(page.patient.signal_cloned().map(clone!(app, page => move |opt| {
                                    opt.and_then(|pt| {
                                        let (is_ipd, is_pre_admit) = pt.visit_type.is_ipd_and_is_pre_admit();
                                        (is_ipd && app.endpoint_is_allow(&Method::GET, &EndPoint::IpdOrderToHomeMedAn, is_pre_admit)).then(|| {
                                            html!("button" => HtmlButtonElement, {
                                                .attr("type", "button")
                                                .class(class::BTN_SM_RT_GRAY)
                                                //.attr("id", "add_continue_order_med_to_home_med_button")
                                                .children([
                                                    html!("i", {.class(class::FA_PLUS_L)}),
                                                    html!("i", {.class(class::FA_BOLT_L)}),
                                                ])
                                                .text("Cont. Order")
                                                .apply(mixins::click_with_loader_checked(clone!(app, page => move || {
                                                    Self::load_home_med_from_cont(page.clone(), app.clone());
                                                }), app.state()))
                                            })
                                        })
                                    })
                                })))
                                .child_signal(page.allow_from_med_rec_signal(app.clone()).map(clone!(app, page => move |is_allow| {
                                    is_allow.then(|| {
                                        html!("button" => HtmlButtonElement, {
                                            .attr("type", "button")
                                            .class(class::BTN_SM_RT_GRAY)
                                            .children([
                                                html!("i", {.class(class::FA_PLUS_L)}),
                                                html!("i", {.class(class::FA_BOLT_L)}),
                                            ])
                                            .text("Used MR")
                                            .apply(mixins::click_with_loader_checked(clone!(app, page => move || {
                                                Self::load_home_med_from_med_rec(String::from("Y"), page.clone(), app.clone());
                                            }), app.state()))
                                        })
                                    })
                                })))
                                .child_signal(page.allow_from_med_rec_signal(app.clone()).map(clone!(app, page => move |is_allow| {
                                    is_allow.then(|| {
                                        html!("button" => HtmlButtonElement, {
                                            .attr("type", "button")
                                            .class(class::BTN_SM_RT_GRAY)
                                            .children([
                                                html!("i", {.class(class::FA_PLUS_L)}),
                                                html!("i", {.class(class::FA_BOLT_L)}),
                                            ])
                                            .text("Held MR")
                                            .apply(mixins::click_with_loader_checked(clone!(app, page => move || {
                                                Self::load_home_med_from_med_rec(String::from("H"), page.clone(), app.clone());
                                            }), app.state()))
                                        })
                                    })
                                })))
                                .child_signal(page.allow_from_med_rec_signal(app.clone()).map(clone!(app, page => move |is_allow| {
                                    is_allow.then(|| {
                                        html!("button" => HtmlButtonElement, {
                                            .attr("type", "button")
                                            .class(class::BTN_SM_RT_GRAY)
                                            .children([
                                                html!("i", {.class(class::FA_PLUS_L)}),
                                                html!("i", {.class(class::FA_BOLT_L)}),
                                            ])
                                            .text("Offed MR")
                                            .apply(mixins::click_with_loader_checked(clone!(app, page => move || {
                                                Self::load_home_med_from_med_rec(String::from("N"), page.clone(), app.clone());
                                            }), app.state()))
                                        })
                                    })
                                })))
                                .child_signal(page.display_homemed_searchbox.signal_cloned().map(clone!(app, page => move |show| {
                                    show.then(|| {
                                        let hn = page.patient.lock_ref().as_ref().and_then(|pt| pt.hn());
                                        let searchbox = MedSearchboxCpn::new(hn, true);
                                        html!("div", {
                                            .child(MedSearchboxCpn::render(page.pre_order_master_id.get(), searchbox.clone(), page.clone(), app.clone()))
                                            .children(MedSearchboxCpn::render_modals(searchbox))
                                        })
                                    })
                                })))
                                .children([
                                    html!("div", {
                                        //.attr("id", "oneDayForm-home-medication-input-div")
                                        .children_signal_vec(page.home_medications.signal_vec_cloned().map(clone!(app, page, injections => move |home_medication| {
                                            let is_injection = home_medication.dosageform.lock_ref().as_ref().map(|dosageform| injections.contains(dosageform)).unwrap_or_default();
                                            html!("div", {
                                                .class(class::BOX_ROUND_P1_T)
                                                .apply(|dom| {
                                                    if is_injection {
                                                        dom.class("bg-danger-subtle")
                                                    } else {
                                                        dom.class("bg-info-subtle")
                                                    }
                                                })
                                                .children([
                                                    html!("div", {
                                                        .class(class::INPUT_GROUP_SM_T)
                                                        // MR label
                                                        .child_signal(home_medication.med_reconciliation_item_id.signal().map(clone!(home_medication => move |opt| {
                                                            opt.is_some().then(|| {
                                                                html!("span", {
                                                                    .class("input-group-text")
                                                                    .style("cursor","help")
                                                                    .apply(|dom| {
                                                                        match home_medication.used.lock_ref().as_ref() {
                                                                            Some(used) => {
                                                                                match used.as_str() {
                                                                                    "N" => dom.class(class::BOLD_BG_GRAY),
                                                                                    "H" => dom.class(class::BOLD_BG_CYAN),
                                                                                    "Y" => {
                                                                                        if home_medication.is_med_rec_change_usage() {
                                                                                            dom.class(class::BOLD_BG_GOLD)
                                                                                        } else {
                                                                                            dom.class(class::BOLD_BG_GREEN)
                                                                                        }
                                                                                    }
                                                                                    _ => dom,
                                                                                }
                                                                            }
                                                                            None => dom,
                                                                        }

                                                                    })
                                                                    .attr("title", &home_medication.med_rec_info())
                                                                    .text("MR")
                                                                })
                                                            })
                                                        })))
                                                        .children([
                                                            html!("input", {
                                                                .attr("type", "text")
                                                                .class("form-control")
                                                                .attr("readonly", "readonly")
                                                                .prop_signal("value", home_medication.med_name.signal_cloned())
                                                            }),
                                                            html!("input" => HtmlInputElement, {
                                                                .attr("type", "number")
                                                                .class("form-control")
                                                                .class_signal("bg-warning", home_medication.first_qty.signal_cloned().map(|s| s.is_empty()))
                                                                .style("max-width", "72px")
                                                                .attr("placeholder", "จำนวน")
                                                                .apply(mixins::string_value(home_medication.first_qty.clone(), page.changed.clone()))
                                                            }),
                                                            html!("button", {
                                                                .class(class::BTN_SM_RED)
                                                                .attr("type", "button")
                                                                .child(html!("i", {.class(class::FA_MINUS)}))
                                                                .event(clone!(page, home_medication => move |_: events::Click| {
                                                                    page.home_medications.lock_mut().retain(|x| *x != home_medication);
                                                                    page.changed.set_neq(true);
                                                                }))
                                                            }),
                                                        ])
                                                    }),
                                                    html!("div", {
                                                        .class(class::INPUT_GROUP)
                                                        .children([
                                                            html!("textarea" => HtmlTextAreaElement, {
                                                                .class(class::FORM_CTRL_SM)
                                                                .attr("id", &["textarea-", &home_medication.id.to_string()].concat())
                                                                .attr("placeholder", "เช่น 13pt หรือ ระบุวิธีใช้ยา")
                                                                .apply(mixins::textarea_value_auto_expand(home_medication.order_item_detail.clone(), page.changed.clone()))
                                                                .event(clone!(page, home_medication => move |_: events::Focus| {
                                                                    page.focused.set_neq(Some(home_medication.id));
                                                                }))
                                                            }),
                                                            html!("button", {
                                                                .class(class::BTN_SM_GRAY)
                                                                .attr("type", "button")
                                                                .child(html!("i", {.class(class::FA_X)}))
                                                                .event(clone!(page, home_medication => move |_: events::Click| {
                                                                    home_medication.order_item_detail.set_neq(String::new());
                                                                    page.changed.set_neq(false);
                                                                }))
                                                            }),
                                                        ])
                                                    }),
                                                    search_drugusage("280px", home_medication.order_item_detail.clone(), app.clone()),
                                                ])
                                                .child_signal(map_ref!{
                                                    let is_due = home_medication.due_status.signal_ref(|opt| opt.as_ref().map(|due_status| due_status == "Y").unwrap_or_default()),
                                                    let has_info = home_medication.info_status.signal_ref(|opt| opt.as_ref().map(|info_status| info_status == "Y").unwrap_or_default()) =>
                                                    (*is_due, *has_info)
                                                }.map(clone!(home_medication => move |(is_due, has_info)| {
                                                    if is_due {
                                                        Some(html!("div", {
                                                            .class("p-2")
                                                            .style("white-space","pre-wrap")
                                                            .children(doms::square_bracket_to_span(&home_medication.due_usage.get_cloned().unwrap_or_default()))
                                                        }))
                                                    } else if has_info {
                                                        Some(html!("div", {
                                                            .class("p-2")
                                                            .style("white-space","pre-wrap")
                                                            .children(doms::square_bracket_to_span(&home_medication.info.get_cloned().unwrap_or_default()))
                                                        }))
                                                    } else {
                                                        None
                                                    }
                                                })))
                                            })
                                        })))
                                    }),
                                ])
                            }),
                        ])
                    }),
                    html!("button", {
                        .attr("type", "button")
                        .class(class::BTN_FR_GRAY)
                        .text("Cancel")
                        .event(clone!(page, parent_show_oneday_input, parent_edit_order => move |_: events::Click| {
                            parent_show_oneday_input.set_neq(false);
                            parent_edit_order.set(None);
                            page.offs_by_parent.lock_mut().clear();
                        }))
                    }),
                    html!("button" => HtmlButtonElement, {
                        .attr("type", "button")
                        .class(class::BTN_FR_L)
                        .class_signal("btn-primary", or(page.changed.signal(), not(page.offs_by_parent.signal_vec_cloned().is_empty())))
                        .class_signal("btn-secondary", and(not(page.changed.signal()), page.offs_by_parent.signal_vec_cloned().is_empty()))
                        .text("Save")
                        .apply(mixins::click_with_loader_checked_or_true_disable_signal(clone!(app, page => move || {
                            Self::submit(page.clone(), parent_show_oneday_input.clone(), parent_edit_order.clone(), parent_reload_order_oneday.clone(), app.clone());
                        }), and(not(page.changed.signal()), page.offs_by_parent.signal_vec_cloned().is_empty()), app.state()))
                    }),
                ])
            }))
        })
    }

    fn submit(page: Rc<Self>, parent_show_oneday_input: Mutable<bool>, parent_edit_order: Mutable<Option<Rc<Order>>>, parent_reload_order_oneday: Mutable<bool>, app: Rc<App>) {
        let injections = app.app_status.lock_ref().as_ref().map(|status| status.hosxp_injection_dosageforms.clone()).unwrap_or_default();
        let mut order_items = Vec::new();
        order_items.extend(page.notes.lock_ref().iter().filter_map(OrderItemMutable::save));
        order_items.extend(page.offs.lock_ref().iter().filter_map(OrderItemMutable::save));
        order_items.extend(page.offs_by_parent.lock_ref().iter().filter_map(OrderItemMutable::save));
        order_items.extend(page.labs.lock_ref().iter().filter_map(OrderItemMutable::save));
        order_items.extend(page.xrays.lock_ref().iter().filter_map(OrderItemMutable::save));
        order_items.extend(page.ivfluids.lock_ref().iter().filter_map(OrderItemMutable::save));
        order_items.extend(page.serials.lock_ref().iter().filter_map(OrderItemMutable::save));
        order_items.extend(page.records.lock_ref().iter().filter_map(OrderItemMutable::save));
        order_items.extend(page.meds.lock_ref().iter().filter_map(|med| {
            let order_item_type = if med.dosageform.lock_ref().as_ref().map(|dosageform| injections.contains(dosageform)).unwrap_or_default() {
                "injection"
            } else {
                "med"
            };
            med.order_item_type.set_neq(order_item_type.into());
            OrderItemMutable::save(med)
        }));
        order_items.extend(page.retains.lock_ref().iter().filter_map(OrderItemMutable::save));
        order_items.extend(page.others.lock_ref().iter().filter_map(OrderItemMutable::save));
        order_items.extend(page.pharms.lock_ref().iter().filter_map(OrderItemMutable::save));
        order_items.extend(page.discharges.lock_ref().iter().filter_map(OrderItemMutable::save));
        order_items.extend(page.home_medications.lock_ref().iter().filter_map(OrderItemMutable::save));

        if !order_items.is_empty() {
            app.async_load(
                true,
                clone!(app => async move {
                    let result_opt = if let Some(pre_order_master_id) = page.pre_order_master_id.get() {
                        let now = js_now();
                        let form = PreOrderSave {
                            pre_order_master_id,
                            order_id: page.order_id.get().unwrap_or_default(),
                            order_date: now.date(),
                            order_time: now.time(),
                            order_confirm: String::from("N"),
                            order_doctor: page.order_doctor.get_cloned(),
                            order_type: String::from("oneday"),
                            order_owner_type: page.view_by.get_cloned(),
                            order_items,
                        };
                        // POST `EndPoint::IpdPreOrderOrder`
                        Some(form.call_api_post(app.state()).await)
                    } else if let Some(visit_type) = page.patient.lock_ref().as_ref().map(|pt| pt.visit_type()) {
                        let form = OrderSave {
                            visit_type,
                            order_id: page.order_id.get(),
                            order_doctor: page.order_doctor.get_cloned(),
                            order_type: String::from("oneday"),
                            order_owner_type: page.view_by.get_cloned(),
                            order_items,
                        };
                        // POST `EndPoint::IpdOrderOrder`
                        // POST `EndPoint::OpdErOrderOrder`
                        Some(form.call_api_post(app.state()).await)
                    } else {
                        None
                    };

                    if let Some(result) = result_opt {
                        match result {
                            Ok((_, responses)) => {
                                app.alert_execute_responses(&responses, async move {
                                    // app.alert("บันทึกข้อมูลสำเร็จ");
                                    page.offs_by_parent.lock_mut().clear();
                                    parent_show_oneday_input.set_neq(false);
                                    parent_edit_order.set(None);
                                    parent_reload_order_oneday.set(true);
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
}
