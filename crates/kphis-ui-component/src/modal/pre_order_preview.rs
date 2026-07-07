// ipd-dr-pre-order-preview.php

use dominator::{Dom, clone, events, html};
use futures_signals::{
    map_ref,
    signal::{Mutable, Signal, SignalExt},
    signal_vec::{MutableVec, SignalVecExt},
};
use std::rc::Rc;
use web_sys::HtmlButtonElement;

use kphis_model::{
    endpoint::EndPoint,
    fetch::Method,
    pre_order::{
        master::{PreOrderMaster, PreOrderMasterParams},
        order::{PreOrder, PreOrderIntoCommand, PreOrderParams},
        progress_note::{PreProgressNote, PreProgressNoteParams},
    },
};
use kphis_ui_app::App;
use kphis_ui_core::{class, doms, mixins};
use kphis_util::{
    datetime::date_str_th,
    util::{str_some, zero_none},
};

use crate::ipd_pre_order::{render_order, render_progress_note};

#[derive(Clone, Default)]
pub enum ToOrderType {
    #[default]
    Order,
    PreOrder,
    OpdErOrder,
}

/// - GET `EndPoint::IpdPreOrderMaster`
/// - GET `EndPoint::IpdPreOrderOrder`
/// - GET `EndPoint::IpdPreOrderProgressNote`
/// - POST `EndPoint::IpdPreOrderInto` (guarded, remove 'ใช้งาน' btn)
#[derive(Clone, Default)]
pub struct PreOrderPreview {
    loaded_master: Mutable<bool>,
    loaded_order_all: Mutable<bool>,
    pre_order_master_id: Mutable<u32>,
    caller_id: Mutable<String>, // an OR pre_order_master_id

    order_doctor_name: Mutable<String>,
    pre_order_type: Mutable<String>,
    hn: Mutable<String>,
    ptname: Mutable<String>,
    template_name: Mutable<String>,
    order_for_date: Mutable<String>,
    used: Mutable<String>,

    oneday: MutableVec<Rc<PreOrder>>,
    continuous: MutableVec<Rc<PreOrder>>,
    progress_note: MutableVec<Rc<PreProgressNote>>,

    to_order_type: ToOrderType,
}

impl PreOrderPreview {
    /// caller_id as `an` or `pre_order_master_id`
    pub fn new(pre_order_master_id: u32, caller_id: Mutable<String>, to_order_type: ToOrderType) -> Rc<Self> {
        Rc::new(Self {
            pre_order_master_id: Mutable::new(pre_order_master_id),
            caller_id,
            to_order_type,
            ..Default::default()
        })
    }

    pub fn is_template(&self) -> impl Signal<Item = bool> + use<> {
        self.pre_order_type.signal_cloned().map(|ot| ot == "template")
    }

    pub fn is_used(&self) -> impl Signal<Item = bool> + use<> {
        self.used.signal_cloned().map(|ot| ot == "Y")
    }

    fn load_master(page: Rc<Self>, app: Rc<App>) {
        app.async_load(
            true,
            clone!(app => async move {
                let pre_order_master_id = page.pre_order_master_id.get();
                if pre_order_master_id > 0 {
                    let params = PreOrderMasterParams {
                        pre_order_master_id: Some(pre_order_master_id),
                        ..Default::default()
                    };
                    // GET `EndPoint::IpdPreOrderMaster`
                    match PreOrderMaster::call_api_get(&params, app.state()).await {
                        Ok(response) => {
                            if let Some(pre_order) = response.first().cloned() {
                                let hn = pre_order.hn.unwrap_or_default();

                                page.order_doctor_name.set([if pre_order.order_doctor_is_intern.unwrap_or_default() {"(Intern) "} else {""}, &pre_order.order_doctor_name.unwrap_or_default()].concat());
                                page.pre_order_type.set(pre_order.pre_order_type);
                                page.hn.set(hn.clone());
                                page.ptname.set([&hn, " (", &pre_order.fullname.clone().unwrap_or_default(), ")"].concat());
                                page.template_name.set(pre_order.template_name.unwrap_or_default());
                                page.order_for_date.set(pre_order.order_for_date.map(|date| date.to_string()).unwrap_or_default());
                                page.used.set(pre_order.used.unwrap_or_default());
                            }
                        }
                        Err(e) => {
                            app.alert_app_error(&e).await;
                        }
                    }
                }
            }),
        )
    }

    fn load_order_all(page: Rc<Self>, app: Rc<App>) {
        page.oneday.lock_mut().clear();
        page.continuous.lock_mut().clear();
        page.progress_note.lock_mut().clear();
        app.async_load(
            true,
            clone!(app, page => async move {
                let pre_order_master_id = zero_none(page.pre_order_master_id.get());
                // one day
                let oneday_params = PreOrderParams {
                    pre_order_master_id,
                    order_id: None,
                    order_type: Some(String::from("oneday")),
                    order_confirm: None,
                    order_owner_type: None,
                    view_by: Some(String::from("doctor")),
                };
                // GET `EndPoint::IpdPreOrderOrder`
                match PreOrder::call_api_get(&oneday_params, app.state()).await {
                    Ok(orders) => page.oneday.lock_mut().extend(orders.into_iter().map(Rc::new)),
                    Err(e) => {
                        app.alert_app_error(&e).await;
                    }
                }
                // continuous
                let continuous_params = PreOrderParams {
                    pre_order_master_id,
                    order_id: None,
                    order_type: Some(String::from("continuous")),
                    order_confirm: None,
                    order_owner_type: None,
                    view_by: Some(String::from("doctor")),
                };
                // GET `EndPoint::IpdPreOrderOrder`
                match PreOrder::call_api_get(&continuous_params, app.state()).await {
                    Ok(orders) => page.continuous.lock_mut().extend(orders.into_iter().map(Rc::new)),
                    Err(e) => {
                        app.alert_app_error(&e).await;
                    }
                }
                // progressn note
                let progress_note_params = PreProgressNoteParams {
                    pre_order_master_id,
                    progress_note_id: None,
                    progress_note_date: None,
                };
                // GET `EndPoint::IpdPreOrderProgressNote`
                match PreProgressNote::call_api_get(&progress_note_params, app.state()).await {
                    Ok(progress_notes) => page.progress_note.lock_mut().extend(progress_notes.into_iter().map(Rc::new)),
                    Err(e) => {
                        app.alert_app_error(&e).await;
                    }
                }
            }),
        )
    }

    fn pre_order_to(command: &PreOrderIntoCommand, parent_date_loaded: Option<Mutable<bool>>, parent_count_loaded: Option<Mutable<bool>>, page: Rc<Self>, app: Rc<App>) {
        app.async_load(
            true,
            clone!(app, page, command => async move {
                if page.pre_order_master_id.get() > 0 {
                    // POST `EndPoint::IpdPreOrderInto`
                    match command.call_api_post(app.state()).await {
                        Ok(responses) => {
                            app.alert_execute_responses(&responses, async move {
                                if let Some(date_loaded) = parent_date_loaded {
                                    date_loaded.set_neq(false);
                                }
                                if let Some(count_loaded) = parent_count_loaded {
                                    count_loaded.set_neq(false);
                                }
                            }).await;
                        }
                        Err(e) => {
                            app.alert_app_error(&e).await;
                        }
                    }
                }
            }),
        )
    }

    pub fn render(
        page: Rc<Self>,
        app: Rc<App>,
        select_redraw: Mutable<bool>,
        close_mutable: Mutable<Option<u32>>,
        parent_date_loaded: Option<Mutable<bool>>,
        parent_count_loaded: Option<Mutable<bool>>,
    ) -> Dom {
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
            .future(map_ref!(
                let busy = app.loader_is_loading(),
                let loaded = page.loaded_order_all.signal() =>
                !busy && !loaded
            ).for_each(clone!(app, page => move |ready| {
                if ready {
                    Self::load_order_all(page.clone(), app.clone());
                    page.loaded_order_all.set_neq(true);
                }
                async {}
            })))
            //.class("container-fluid")
            .children([
                doms::form_inline(clone!(app, page => move |form| { form
                    .children([
                        html!("div", {
                            .class("col-12")
                            .child(html!("button", {
                                .attr("type", "button")
                                .class(class::BTN_SM_BLUE)
                                .child(html!("i", {.class(class::FA_L_ARROW)}))
                                .text(" Back")
                                .event(move |_: events::Click| {
                                    close_mutable.set(None);
                                    select_redraw.set(true);
                                // .attr("onclick", "onclickBackButton(event);")
                                })
                            }))
                        }),
                        doms::form_inline_group_sm(clone!(page => move |group| { group
                            .children([
                                doms::label_group_for("order_doctor_name","ผู้บันทึก"),
                                html!("input", {
                                    .attr("type", "text")
                                    .class(class::FORM_CTRL_SM)
                                    .attr("id", "order_doctor_name")
                                    .attr("readonly", "readonly")
                                    .prop_signal("value",page.order_doctor_name.signal_cloned())
                                }),
                            ])
                        })),
                    ])
                    .child_signal(page.is_template().map(clone!(page => move |is_template| {
                        (!is_template).then(|| {
                            doms::form_inline_group_sm(clone!(page => move |group| { group
                                .children([
                                    doms::label_group_for("ptname","HN"),
                                    html!("input", {
                                        .attr("type", "text")
                                        .class(class::FORM_CTRL_SM)
                                        .attr("id", "ptname")
                                        .attr("readonly", "readonly")
                                        .attr("size", "40")
                                        .prop_signal("value",page.ptname.signal_cloned())
                                    }),
                                ])
                            }))
                        })
                    })))
                    .child_signal(page.is_template().map(clone!(page => move |is_template| {
                        is_template.then(|| {
                            doms::form_inline_group_sm(clone!(page => move |group| { group
                                .children([
                                    doms::label_group_for("template_name","ชื่อ Template"),
                                    html!("input", {
                                        .attr("type", "text")
                                        .class(class::FORM_CTRL_SM)
                                        .attr("id", "template_name")
                                        .attr("readonly", "readonly")
                                        .prop_signal("value", page.template_name.signal_cloned())
                                    }),
                                ])
                            }))
                        })
                    })))
                    .child_signal(page.is_template().map(clone!(page => move |is_template| {
                        (!is_template).then(|| {
                            doms::form_inline_group_sm(clone!(page => move |group| { group
                                .children([
                                    doms::label_group_for("order_for_date","วันที่นัด/Admit"),
                                    html!("input", {
                                        .attr("type", "text")
                                        .class(class::FORM_CTRL_SM)
                                        .attr("id", "order_for_date")
                                        .attr("readonly", "readonly")
                                        .prop_signal("value", page.order_for_date.signal_cloned().map(|d| date_str_th(&d)))
                                    }),
                                ])
                            }))
                        })
                    })))
                    .apply_if(app.endpoint_is_allow(&Method::POST, &EndPoint::IpdPreOrderInto, false), |can_add| { can_add
                        .child_signal(map_ref!{
                            let is_template = page.is_template(),
                            let is_used = page.is_used() =>
                            *is_template || !*is_used
                        }.map(clone!(app, page, parent_date_loaded, parent_count_loaded => move |not_used_template| {
                            not_used_template.then(|| {
                                doms::form_inline_end(clone!(app, page, parent_date_loaded, parent_count_loaded => move |end| { end
                                    .child(html!("button" => HtmlButtonElement, {
                                        .attr("type", "button")
                                        .attr("data-bs-dismiss", "modal")
                                        .class(class::BTN_SM_BLUE)
                                        .child(html!("i", {.class(class::FA_PLUS)}))
                                        .text(" ใช้งาน")
                                        .apply(mixins::click_with_loader_checked(clone!(app, page, parent_date_loaded, parent_count_loaded => move || {
                                            let (from, to) = match (page.pre_order_type.lock_ref().as_str(), &page.to_order_type) {
                                                ("template", ToOrderType::Order) => ("template","order"),
                                                    // post("./ipd-dr-template-to-order.php", { pre_order_master_id: selected_ipd_pre_order_master_id, an: IPD_ORDER_AN })
                                                (_, ToOrderType::Order) => ("pre-order","order"),
                                                    // post("./ipd-dr-pre-order-to-order.php", { pre_order_master_id: selected_ipd_pre_order_master_id, an: IPD_ORDER_AN })
                                                (_, ToOrderType::PreOrder) => ("template","pre-order"),
                                                    // post("./ipd-dr-template-to-pre-order.php", { template_master_id: selected_ipd_pre_order_master_id, pre_order_master_id: IPD_PRE_ORDER_MASTER_ID }
                                                ("template", ToOrderType::OpdErOrder) => ("template","opd-er-order"),
                                                    // post("./ipd-dr-template-to-opd-er-order.php", { template_master_id: selected_ipd_opd_er_order_master_id, opd_er_order_master_id: OPD_ER_ORDER_MASTER_ID })
                                                (_, ToOrderType::OpdErOrder) => ("pre-order","opd-er-order"),
                                            };
                                            let command = PreOrderIntoCommand {
                                                from: Some(String::from(from)),
                                                into: Some(String::from(to)),
                                                from_id: str_some(page.pre_order_master_id.get().to_string()),
                                                into_id: str_some(page.caller_id.get_cloned()),
                                            };
                                            Self::pre_order_to(&command, parent_date_loaded.clone(), parent_count_loaded.clone(), page.clone(), app.clone());
                                            // use usePreOrderMaster(pre_order_master_id, pre_order_type) FROM ipd-dr-order.php OR ipd-dr-pre-order.php
                                            // .attr("onclick", "onclickUsePreOrderMasterButton(event);")
                                        }), app.state()))
                                    }))
                                }))
                            })
                        })))
                    })
                })),
                html!("div", {
                    .class("row")
                    .child(html!("div", {
                        .class("col-12")
                        .child(html!("table", {
                            .class(class::TABLE_1R)
                            .children([
                                html!("thead", {
                                    .child(html!("tr", {
                                        .children([
                                            html!("th", {
                                                .attr("scope", "col")
                                                .class(class::TXT_C_GRAYS)
                                                .style("width", "30%")
                                                //.attr("id", "progress-note-column-header")
                                                .text("Progress Note")
                                            }),
                                            html!("th", {
                                                .attr("scope", "col")
                                                .class("text-center")
                                                .style("width", "35%")
                                                //.attr("id", "one-day-column-header")
                                                .text("One Day Order")
                                            }),
                                            html!("th", {
                                                .attr("scope", "col")
                                                .class("text-center")
                                                .style("width", "35%")
                                                //.attr("id", "continuous-column-header")
                                                .text("Continuous Order")
                                            }),
                                        ])
                                    }))
                                }),
                                html!("tbody", {
                                    .child(html!("tr", {
                                        .children([
                                            html!("td", {
                                                .class("bg-secondary-subtle")
                                                .children_signal_vec(page.progress_note.signal_vec_cloned().map(clone!(app, page => move |progress_note| {
                                                    render_progress_note(progress_note, None, page.used.clone(), app.clone())
                                                })))
                                            }),
                                            html!("td", {
                                                .children_signal_vec(page.oneday.signal_vec_cloned().map(clone!(app, page => move |order| {
                                                    render_order(order, true, page.used.clone(), None, app.clone())
                                                })))
                                            }),
                                            html!("td", {
                                                .children_signal_vec(page.continuous.signal_vec_cloned().map(clone!(app, page => move |order| {
                                                    render_order(order, false, page.used.clone(), None, app.clone())
                                                })))
                                            }),
                                        ])
                                    }))
                                }),
                            ])
                        }))
                    }))
                }),
            ])
        })
    }
}
