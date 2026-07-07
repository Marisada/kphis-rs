use dominator::{Dom, clone, events, html};
use futures_signals::{
    map_ref,
    signal::{Mutable, Signal, SignalExt, always, not},
};
use std::rc::Rc;
use web_sys::{HtmlButtonElement, HtmlInputElement};

use kphis_model::{pre_order::master::PreOrderMasterSave, route::Route};
use kphis_ui_app::App;
use kphis_ui_core::{class, doms, mixins};
use kphis_util::{
    datetime::{date_8601, js_now},
    util::str_some,
};

use crate::gadget::searchbox::patient::PatientSearchboxCpn;

/// - POST `EndPoint::IpdPreOrderMaster`
/// - GET `EndPoint::SearchBoxPatientText` (PatientSearchboxCpn)
#[derive(Clone, Default)]
pub struct PreOrderNew {
    view_by: Mutable<String>,
    // order_doctor: Mutable<String>,
    hn: Mutable<String>,
    pt_name: Mutable<String>,
    order_for_date: Mutable<String>,
    template_name: Mutable<String>,
    pre_order_type: Mutable<String>,

    display_patient_searchbox: Mutable<bool>,
    changed: Mutable<bool>,
}

impl PreOrderNew {
    pub fn new(view_by: Mutable<String>) -> Rc<Self> {
        Rc::new(Self { view_by, ..Default::default() })
    }

    fn is_valid(&self) -> impl Signal<Item = bool> + use<> {
        map_ref! {
            let pre_order_type = self.pre_order_type.signal_cloned(),
            let hn = self.hn.signal_cloned(),
            let order_for_date = self.order_for_date.signal_cloned(),
            let template_name = self.template_name.signal_cloned() =>
            match pre_order_type.as_str() {
                "appointment" | "opd" => !hn.is_empty() && !order_for_date.is_empty(),
                "template" => !template_name.is_empty(),
                _ => false,
            }
        }
    }

    pub fn is_template(&self) -> impl Signal<Item = bool> + use<> {
        self.pre_order_type.signal_cloned().map(|ot| ot == "template")
    }

    pub fn render(modal: Rc<Self>, display: Mutable<Option<Rc<Self>>>, changed: Mutable<bool>, app: Rc<App>) -> Dom {
        html!("div", {
            .class("modal-dialog")
            .attr("role", "document")
            .child(html!("div", {
                .class("modal-content")
                .children([
                    html!("div", {
                        .class("modal-header")
                        .children([
                            html!("h5", {
                                .class("modal-title")
                                .text("เพิ่มใบ Order ใหม่")
                            }),
                            html!("button", {
                                .attr("type", "button")
                                .class("btn-close")
                                .attr("data-bs-dismiss", "modal")
                                .attr("aria-label", "Close")
                                .event(clone!(display => move |_: events::Click| {
                                    display.set(None);
                                }))
                            }),
                        ])
                    }),
                    html!("div", {
                        .class("modal-body")
                        .children([
                            html!("p", {.text("เลือกประเภทใบ Order")}),
                            html!("div", {
                                //.attr("id", "pre-order-master-query-form")
                                .children([
                                    html!("div", {
                                        .class(class::FORM_CHK_R)
                                        .children([
                                            html!("input" => HtmlInputElement, {
                                                .class("form-check-input")
                                                .attr("type", "radio")
                                                .attr("id", "pre_order_type1")
                                                .attr("value", "appointment")
                                                .apply(mixins::radio_match(modal.pre_order_type.clone(), modal.changed.clone(), "appointment"))
                                                // .attr("onclick", "onclickPreOrderType(event)")
                                            }),
                                            doms::label_check_for("pre_order_type1","Admit ล่วงหน้า"),
                                        ])
                                    }),
                                    html!("div", {
                                        .class(class::FORM_CHK_R)
                                        .children([
                                            html!("input" => HtmlInputElement, {
                                                .class("form-check-input")
                                                .attr("type", "radio")
                                                .attr("id", "pre_order_type2")
                                                .attr("value", "opd")
                                                .apply(mixins::radio_match(modal.pre_order_type.clone(), modal.changed.clone(), "opd"))
                                                // .attr("onclick", "onclickPreOrderType(event)")
                                            }),
                                            doms::label_check_for("pre_order_type2","Admit ในวัน"),
                                        ])
                                    }),
                                    html!("div", {
                                        .class(class::FORM_CHK_R)
                                        .children([
                                            html!("input" => HtmlInputElement, {
                                                .class("form-check-input")
                                                .attr("type", "radio")
                                                .attr("id", "pre_order_type3")
                                                .attr("value", "template")
                                                .apply(mixins::radio_match(modal.pre_order_type.clone(), modal.changed.clone(), "template"))
                                                // .attr("onclick", "onclickPreOrderType(event)")
                                            }),
                                            doms::label_check_for("pre_order_type3","Template"),
                                        ])
                                    }),
                                    html!("hr"),
                                ])
                                .child_signal(modal.is_template().map(clone!(modal => move |is_template| {
                                    (!is_template).then(|| {
                                        html!("div", {
                                            .class(class::INPUT_GROUP_SM)
                                            .class(class::M_LT)
                                            .attr("id", "new_hn_input_group")
                                            .children([
                                                doms::label_group_for("new_ptname","HN"),
                                                html!("input", {
                                                    .attr("type", "text")
                                                    .class(class::FORM_CTRL_SM)
                                                    .attr("id", "new_ptname")
                                                    .attr("size", "40")
                                                    .attr("readonly", "readonly")
                                                    .style("cursor", "pointer")
                                                    .prop_signal("value", modal.pt_name.signal_cloned())
                                                    .event(clone!(modal => move |_: events::Focus| {
                                                        modal.display_patient_searchbox.set_neq(true);
                                                    }))
                                                }),
                                                html!("button", {
                                                    .class(class::BTN_SM_GRAY)
                                                    .attr("type", "button")
                                                    .child(html!("i", {.class(class::FA_SEARCH)}))
                                                    .event(clone!(modal => move |_: events::Click| {
                                                        modal.display_patient_searchbox.set_neq(true);
                                                    }))
                                                    // .attr("onclick", "onclick_search_new_hn_button(event, this)")
                                                }),
                                                html!("button", {
                                                    .class(class::BTN_SM_RED)
                                                    .attr("type", "button")
                                                    .child(html!("i", {.class(class::FA_X)}))
                                                    .event(clone!(modal => move |_: events::Click| {
                                                        modal.hn.set_neq(String::new());
                                                        modal.pt_name.set_neq(String::new());
                                                    }))
                                                    // .attr("onclick", "onclick_clear_new_hn_button(event, this)")
                                                }),
                                            ])
                                        })
                                    })
                                })))
                                .child_signal(modal.display_patient_searchbox.signal_cloned().map(clone!(app, modal => move |show| {
                                    if show {
                                        app.get_id("new_hn_input_group").map(|elm| {
                                            PatientSearchboxCpn::render(
                                                PatientSearchboxCpn::new(),
                                                modal.display_patient_searchbox.clone(),
                                                modal.hn.clone(),
                                                modal.pt_name.clone(),
                                                elm.get_bounding_client_rect(),
                                                modal.changed.clone(),
                                                app.clone(),
                                            )
                                        })
                                    } else {
                                        None
                                    }
                                })))
                                .child_signal(modal.is_template().map(clone!(modal => move |is_template| {
                                    (!is_template).then(|| {
                                        html!("div", {
                                            .class(class::INPUT_GROUP_SM)
                                            .class(class::M_LT)
                                            //.attr("id", "order_for_date_form_group")
                                            .children([
                                                doms::label_group_for("order_for_date","วันที่นัด/Admit"),
                                                doms::date_picker(
                                                    modal.order_for_date.clone(),
                                                    modal.changed.clone(),
                                                    modal.pre_order_type.signal_cloned().map(|ty| ty != "appointment"), None,
                                                    |d| d.class(class::FLEX_GROW1).style("min-width","120px"),
                                                    |d| d.class(class::FORM_CTRL_ONLY_SM_R0_L),
                                                    |d| d.class(class::FORM_CTRL_ONLY_SM_R0_L).attr("id", "order_for_date"),
                                                    |s| s, always(None),
                                                ),
                                            ])
                                            .future(modal.pre_order_type.signal_cloned().for_each(clone!(modal => move |pre_order_type| {
                                                let order_for_date = if pre_order_type.as_str() == "opd" {
                                                    js_now().date().to_string()
                                                } else {
                                                    String::new()
                                                };
                                                modal.order_for_date.set_neq(order_for_date);
                                                async{}
                                            })))
                                        })
                                    })
                                })))
                                .child_signal(modal.is_template().map(clone!(modal => move |is_template| {
                                    is_template.then(|| {
                                        html!("div", {
                                            .class(class::INPUT_GROUP_SM)
                                            .class(class::M_LT)
                                            //.attr("id", "new_template_name_form_group")
                                            .children([
                                                doms::label_group_for("new_template_name","ชื่อ Template"),
                                                html!("input" => HtmlInputElement, {
                                                    .attr("type", "text")
                                                    .class(class::FORM_CTRL_SM)
                                                    .attr("id", "new_template_name")
                                                    .apply(mixins::string_value(modal.template_name.clone(), modal.changed.clone()))
                                                }),
                                            ])
                                        })
                                    })
                                })))
                            }),
                        ])
                    }),
                    html!("div", {
                        .class("modal-footer")
                        .children([
                            html!("button" => HtmlButtonElement, {
                                .attr("type", "button")
                                .class("btn")
                                .class_signal("btn-primary", modal.changed.signal())
                                .class_signal("btn-secondary", not(modal.changed.signal()))
                                .attr("data-bs-dismiss", "modal")
                                .text("บันทึก")
                                .apply(mixins::click_with_loader_checked_or_true_disable_signal(clone!(app, modal, display => move || {
                                    Self::submit(modal.clone(), display.clone(), changed.clone(), app.clone());
                                }), not(modal.is_valid()), app.state()))
                                // .attr("onclick", "onclickSavePreOrderMasterButton(event);")
                            }),
                            html!("button", {
                                .attr("type", "button")
                                .class(class::BTN_GRAY)
                                .attr("data-bs-dismiss", "modal")
                                .text("ยกเลิก")
                                .event(move |_: events::Click| {
                                    display.set(None);
                                })
                            }),
                        ])
                    }),
                ])
            }))
        })
    }

    // ipd-dr-pre-order-master-save.php
    fn submit(modal: Rc<Self>, display: Mutable<Option<Rc<Self>>>, changed: Mutable<bool>, app: Rc<App>) {
        if let Some(order_doctor) = app.doctor_code() {
            let save = PreOrderMasterSave {
                pre_order_master_id: None,
                pre_order_type: modal.pre_order_type.get_cloned(),
                order_date: None,
                order_time: None,
                order_for_date: date_8601(&modal.order_for_date.lock_ref()),
                order_for_time: None,
                order_doctor,
                hn: str_some(modal.hn.get_cloned()),
                template_name: str_some(modal.template_name.get_cloned()),
                shared_template: None,
                used: None,
            };

            app.async_load(
                true,
                clone!(app => async move {
                    // POST `EndPoint::IpdPreOrderMaster`
                    match save.call_api_post(app.state()).await {
                        Ok((pre_order_master_id, response)) => {
                            app.alert_execute_response(&response, clone!(app => async move {
                                changed.set_neq(true);
                                display.set(None);
                                let route = Route::IpdPreOrder {
                                    view_by: modal.view_by.get_cloned(),
                                    pre_order_master_id
                                };
                                if route.has_permission(app.state()) {
                                    route.hard_redirect();
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
}
