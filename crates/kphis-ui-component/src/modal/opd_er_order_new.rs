use dominator::{Dom, clone, events, html};
use futures_signals::{
    map_ref,
    signal::{Mutable, Signal, SignalExt, not},
};
use std::rc::Rc;
use web_sys::{HtmlButtonElement, HtmlSelectElement, HtmlTextAreaElement};

use kphis_model::{
    route::Route,
    {opd_er::order_master::OpdErOrderMasterSave, tab::Tab},
};
use kphis_ui_app::App;
use kphis_ui_core::{class, doms, mixins};
use kphis_util::util::{str_some, zero_none};

use crate::gadget::searchbox::opd_visit::OpdVisitSearchboxCpn;

/// - POST `EndPoint::OpdErOrderMaster`
/// - GET `EndPoint::SearchBoxOpdVisitModeText` (OpdVisitSearchboxCpn)
#[derive(Clone, Default)]
pub struct OpdErOrderNew {
    new_opd_visit_detail: Mutable<String>,
    new_vn: Mutable<String>,
    new_note: Mutable<String>,
    new_order_bedno: Mutable<String>,

    display_patient_searchbox: Mutable<bool>,
    changed: Mutable<bool>,
}

impl OpdErOrderNew {
    pub fn new() -> Rc<Self> {
        Rc::new(Self::default())
    }

    fn is_valid(&self) -> impl Signal<Item = bool> + use<> {
        map_ref! {
            let bedno = self.new_order_bedno.signal_cloned(),
            let new_vn = self.new_vn.signal_cloned(),
            let new_note = self.new_note.signal_cloned() =>
            !(bedno.is_empty() || (new_vn.is_empty() && new_note.is_empty()))
        }
    }

    pub fn render(modal: Rc<Self>, view_by: Mutable<String>, display: Mutable<Option<Rc<Self>>>, changed: Mutable<bool>, app: Rc<App>) -> Dom {
        let all_er_bed_select_option = match app.app_asset.lock_ref().as_ref() {
            Some(assets_arc) => {
                let asset = assets_arc.as_ref().to_owned();
                asset.er_bed_select_options
            }
            None => Vec::new(),
        };

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
                        .child(html!("div", {
                            //.attr("id", "opd-er-order-master-query-form")
                            .child(html!("div", {
                                .class(class::INPUT_GROUP_SM)
                                .class(class::M_LT)
                                .attr("id", "new_vn_input_group")
                                .children([
                                    doms::label_group_for("new_opd_visit_detail","Visit"),
                                    html!("input", {
                                        .attr("type", "text")
                                        .class(class::FORM_CTRL_SM)
                                        .attr("id", "new_opd_visit_detail")
                                        .attr("size", "50")
                                        .attr("readonly", "readonly")
                                        .style("cursor","pointer")
                                        .prop_signal("value", modal.new_opd_visit_detail.signal_cloned())
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
                                        // .attr("onclick", "onclick_search_new_vn_button(event, this)")
                                    }),
                                    html!("button", {
                                        .class(class::BTN_SM_RED)
                                        .attr("type", "button")
                                        .child(html!("i", {.class(class::FA_X)}))
                                        .event(clone!(modal => move |_: events::Click| {
                                            modal.new_vn.set_neq(String::new());
                                            modal.new_opd_visit_detail.set_neq(String::new());
                                            modal.changed.set_neq(true);
                                        }))
                                        // .attr("onclick", "onclick_clear_new_vn_button(event, this)")
                                    }),
                                ])
                            }))
                            .child_signal(modal.display_patient_searchbox.signal_cloned().map(clone!(app, modal => move |show| {
                                if show {
                                    app.get_id("new_vn_input_group").map(|elm| {
                                        OpdVisitSearchboxCpn::render(
                                            OpdVisitSearchboxCpn::new(),
                                            modal.display_patient_searchbox.clone(),
                                            modal.new_vn.clone(),
                                            modal.new_opd_visit_detail.clone(),
                                            elm.get_bounding_client_rect(),
                                            modal.changed.clone(),
                                            app.clone(),
                                        )
                                    })
                                } else {
                                    None
                                }
                            })))
                            .children([
                                html!("div", {
                                    .class(class::M_LT)
                                    .children([
                                        html!("label", {
                                            .attr("for", "new_note")
                                            .class("me-2")
                                            .text("Note")
                                        }),
                                        html!("textarea" => HtmlTextAreaElement, {
                                            .class("form-control")
                                            .attr("id", "new_note")
                                            .attr("rows", "3")
                                            .attr("cols", "50")
                                            .apply(mixins::string_value(modal.new_note.clone(), modal.changed.clone()))
                                        }),
                                    ])
                                }),
                                html!("div", {
                                    .style("width","200px")
                                    .child(html!("div", {
                                        .class(class::INPUT_GROUP_SM)
                                        .class(class::M_LT)
                                        .children([
                                            doms::label_group_for("new_order_bedno","เตียง"),
                                            html!("select" => HtmlSelectElement, {
                                                .class(class::FORM_SELECT_SM)
                                                .attr("id", "new_order_bedno")
                                                .child(html!("option", {
                                                    .attr("value", "")
                                                    .style("color","#777")
                                                    .style("background-color","white")
                                                    .text("เลือก")
                                                }))
                                                .children(all_er_bed_select_option.iter().map(|option| {
                                                    doms::select_option_color(option, &modal.new_order_bedno.lock_ref())
                                                }))
                                                .apply(mixins::string_value_select(modal.new_order_bedno.clone(), modal.changed.clone()))
                                                // .attr("onchange", "onchangeNewBedNoSelect(event, this)")
                                            }),
                                        ])
                                    }))
                                }),
                            ])
                        }))
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
                                    Self::submit(modal.clone(), view_by.clone(), display.clone(), changed.clone(), app.clone());
                                }), not(modal.is_valid()), app.state()))
                                // .attr("onclick", "onclickSaveOpdErOrderMasterButton(event);")
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
    fn submit(modal: Rc<Self>, view_by: Mutable<String>, display: Mutable<Option<Rc<Self>>>, changed: Mutable<bool>, app: Rc<App>) {
        let save = OpdErOrderMasterSave {
            vn: str_some(modal.new_vn.get_cloned()),
            note: str_some(modal.new_note.get_cloned()),
            bedno: zero_none(modal.new_order_bedno.lock_ref().parse::<u32>().unwrap_or_default()),
            ..Default::default()
        };

        app.async_load(
            true,
            clone!(app => async move {
                // POST `EndPoint::OpdErOrderMaster`
                match save.call_api_post(app.state()).await {
                    Ok((opd_er_order_master_id, response)) => {
                        app.alert_execute_response(&response, clone!(app => async move {
                            changed.set_neq(true);
                            display.set(None);
                            let route = Route::OpdErMain {
                                view_by: view_by.get_cloned(),
                                opd_er_order_master_id,
                                tab: Tab::Order.str().to_owned(),
                                id: 0,
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
