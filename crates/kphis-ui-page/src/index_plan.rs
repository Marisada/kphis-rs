// ipd-nurse-index-plan-monitor.php

use dominator::{Dom, EventOptions, clone, events, html, is_window_loaded, window_size, with_node};
use futures_signals::{
    map_ref,
    signal::{Mutable, SignalExt},
    signal_vec::{MutableVec, SignalVecExt},
};
use std::rc::Rc;
use web_sys::{HtmlInputElement, HtmlSelectElement};

use kphis_model::{
    app::VisitTypeId,
    avatar::{AvatarEnum, AvatarOpdEr, AvatarParams, AvatarWard},
    order::{OrderItem, OrderParams},
    patient_info::PatientInfo,
};
use kphis_util::util::{str_some, zero_none};

use kphis_ui_app::App;
use kphis_ui_component::{
    index_plan::{redraw_index_plan, render_index_plan},
    modal::{blank_modal, index_plan_action_form::IndexPlanActionForm},
    show_patient_main::ShowPatientMainCpn,
};
use kphis_ui_core::{binding::NiceSelect, class, doms, mixins};

/// - GET `EndPoint::AvatarIpd`
/// - GET `EndPoint::AvatarOpdEr`
/// - GET `EndPoint::IpdOrderItem` (self/IndexPlanActionForm)
/// - GET `EndPoint::OpdErOrderItem` (self/IndexPlanActionForm)
/// - GET `EndPoint::IpdShowPatientMainAn` (ShowPatientMainCpn)
/// - GET `EndPoint::OpdErShowPatientMainVn` (ShowPatientMainCpn)
/// - GET `EndPoint::OpdErShowPatientMainId` (ShowPatientMainCpn)
#[derive(Clone, Default)]
pub struct IndexPlanPage {
    is_ipd: bool,
    search: Mutable<String>,

    search_result: MutableVec<Rc<AvatarEnum>>,
    search_changed: Mutable<bool>,
    search_selected: Mutable<Option<VisitTypeId>>,

    patient: Mutable<Option<Mutable<Option<Rc<PatientInfo>>>>>,
    patient_loaded: Mutable<Mutable<bool>>,

    index_changed: Mutable<bool>,
    checker: Mutable<bool>,
    status: Mutable<Option<String>>,
    order_item_type: Mutable<String>,
    nurse_assign: Mutable<String>,
    is_redraw: Mutable<bool>,

    without_order: Mutable<String>, // "Y"

    order_items_show: MutableVec<Rc<OrderItem>>,
    order_items_all: MutableVec<Rc<OrderItem>>,

    index_plan_action_modal: Mutable<Option<Rc<IndexPlanActionForm>>>,

    is_rescroll: Mutable<bool>,
    scroll_position: Mutable<(i32, i32)>,
}

impl IndexPlanPage {
    pub fn new_ipd() -> Rc<Self> {
        Rc::new(Self {
            is_ipd: true,
            status: Mutable::new(Some(String::from("wait"))),
            ..Default::default()
        })
    }

    pub fn new_opd_er() -> Rc<Self> {
        Rc::new(Self {
            is_ipd: false,
            status: Mutable::new(Some(String::from("wait"))),
            ..Default::default()
        })
    }

    fn submit_search(page: Rc<Self>, app: Rc<App>) {
        app.async_load(
            true,
            clone!(app, page => async move {
                if page.is_ipd {
                    let params = AvatarParams {
                        ward: str_some(app.ward_select.get_cloned()),
                        search: str_some(page.search.get_cloned()),
                    };
                    if params.is_empty() {
                        page.search_result.lock_mut().clear();
                    } else {
                        // GET `EndPoint::AvatarIpd`
                        match AvatarWard::call_api_get(&params, app.state()).await {
                            Ok(items) => {
                                let mut lock = page.search_result.lock_mut();
                                lock.clear();
                                lock.extend(items.iter().map(|im| Rc::new(AvatarEnum::from(im))));
                                if items.len() == 1 {
                                    page.search_selected.set(items.first().map(|i| i.visit_type(app.hosxp_an_len())));
                                } else {
                                    page.search_selected.set(None);
                                }
                            }
                            Err(e) => {
                                app.alert_app_error(&e).await;
                            }
                        }
                    }
                } else {
                    // GET `EndPoint::AvatarOpdEr`
                    match AvatarOpdEr::call_api_get(app.state()).await {
                        Ok(items) => {
                            let mut lock = page.search_result.lock_mut();
                            lock.clear();
                            lock.extend(items.iter().map(|im| Rc::new(AvatarEnum::from(im))));
                            if items.len() == 1 {
                                page.search_selected.set(items.first().map(|i| i.visit_type()));
                            } else {
                                page.search_selected.set(None);
                            }
                        }
                        Err(e) => {
                            app.alert_app_error(&e).await;
                        }
                    }
                }
            }),
        );
    }

    // send GET method
    fn load_indexes(page: Rc<Self>, app: Rc<App>) {
        page.order_items_all.lock_mut().clear();
        page.order_items_show.lock_mut().clear();
        app.async_load(
            true,
            clone!(app, page => async move {
                if let Some(visit_type) = page.patient.lock_ref().as_ref().and_then(|m| m.lock_ref().as_ref().map(|pt| pt.visit_type())) {
                    match &visit_type {
                        VisitTypeId::Ipd(an)
                        | VisitTypeId::PreAdmit(an) => {
                            let params = OrderParams {
                                an: str_some(an.to_owned()),
                                view_by: Some(String::from("doctor")),
                                without_order: str_some(page.without_order.get_cloned()),
                                ..Default::default()
                            };
                            // GET `EndPoint::IpdOrderItem`
                            match OrderItem::call_api_get_ipd(&params, app.state()).await {
                                Ok(order_items) => {
                                    page.checker.set_neq(!order_items.is_empty());
                                    let items_iter = order_items.into_iter().map(Rc::new);
                                    page.order_items_all.lock_mut().extend(items_iter.clone());
                                    page.order_items_show.lock_mut().extend(items_iter);
                                    redraw_index_plan(
                                        None,
                                        None,
                                        page.order_items_show.clone(),
                                        page.order_items_all.clone(),
                                        page.nurse_assign.clone(),
                                        page.order_item_type.clone(),
                                        page.status.clone(),
                                        app,
                                    );
                                    page.is_rescroll.set(true);
                                }
                                Err(e) => {
                                    app.alert_app_error(&e).await;
                                }
                            }
                        }
                        VisitTypeId::OpdEr(_, opd_er_order_master_id) => {
                            let params = OrderParams {
                                opd_er_order_master_id: zero_none(*opd_er_order_master_id),
                                without_order: str_some(page.without_order.get_cloned()),
                                ..Default::default()
                            };
                            // GET `EndPoint::OpdErOrderItem`
                            match OrderItem::call_api_get_opd_er(&params, app.state()).await {
                                Ok(order_items) => {
                                    page.checker.set_neq(!order_items.is_empty());
                                    let items_iter = order_items.into_iter().map(Rc::new);
                                    page.order_items_all.lock_mut().extend(items_iter.clone());
                                    page.order_items_show.lock_mut().extend(items_iter);
                                    redraw_index_plan(
                                        None,
                                        None,
                                        page.order_items_show.clone(),
                                        page.order_items_all.clone(),
                                        page.nurse_assign.clone(),
                                        page.order_item_type.clone(),
                                        page.status.clone(),
                                        app,
                                    );
                                    page.is_rescroll.set(true);
                                }
                                Err(e) => {
                                    app.alert_app_error(&e).await;
                                }
                            }
                        }
                        VisitTypeId::Visit(_) => {
                            page.checker.set_neq(false);
                            page.order_items_all.lock_mut().clear();
                            page.order_items_show.lock_mut().clear();
                            redraw_index_plan(
                                None,
                                None,
                                page.order_items_show.clone(),
                                page.order_items_all.clone(),
                                page.nurse_assign.clone(),
                                page.order_item_type.clone(),
                                page.status.clone(),
                                app,
                            );
                            page.is_rescroll.set(true);
                        }
                    }
                } else {
                    page.checker.set_neq(false);
                    page.order_items_all.lock_mut().clear();
                    page.order_items_show.lock_mut().clear();
                    redraw_index_plan(
                        None,
                        None,
                        page.order_items_show.clone(),
                        page.order_items_all.clone(),
                        page.nurse_assign.clone(),
                        page.order_item_type.clone(),
                        page.status.clone(),
                        app,
                    );
                    page.is_rescroll.set(true);
                }
            }),
        );
    }

    pub fn render(page: Rc<Self>, app: Rc<App>) -> Dom {
        app.set_title(if page.is_ipd {
            "KPHIS - Search IPD Nurse Planning Action"
        } else {
            "KPHIS - Search OPD-ER Nurse Planning Action"
        });

        let ward_select_option = if page.is_ipd {
            app.app_asset.lock_ref().as_ref().map(|asset| asset.ward_select_option.clone()).unwrap_or_default()
        } else {
            Vec::new()
        };

        html!("section", {
            .future(is_window_loaded().for_each(clone!(app, page => move |value| {
                if value {
                    if page.is_ipd {
                        if let Some(elm) = app.get_id("wards") {
                            NiceSelect::new_default(&elm);
                        }
                    }
                    page.search_changed.set(true);
                }
                async {}
            })))
            .future(map_ref!(
                let busy = app.loader_is_loading(),
                let changed = page.search_changed.signal() =>
                !busy && *changed
            ).for_each(clone!(app, page => move |ready| {
                if ready {
                    Self::submit_search(page.clone(), app.clone());
                    page.search_changed.set(false);
                }
                async {}
            })))
            .future(page.patient_loaded.signal_cloned().map(|patient_loaded| patient_loaded.signal()).flatten().for_each(clone!(page => move |loaded| {
                if loaded {
                    page.index_changed.set(true);
                }
                async {}
            })))
            .future(map_ref!(
                let busy = app.loader_is_loading(),
                let changed = page.index_changed.signal() =>
                !busy && *changed
            ).for_each(clone!(app, page => move |ready| {
                if ready {
                    Self::load_indexes(page.clone(), app.clone());
                    page.index_changed.set(false);
                }
                async {}
            })))
            .future(page.is_redraw.signal().for_each(clone!(app, page => move |changed| {
                if changed {
                    redraw_index_plan(
                        None,
                        None,
                        page.order_items_show.clone(),
                        page.order_items_all.clone(),
                        page.nurse_assign.clone(),
                        page.order_item_type.clone(),
                        page.status.clone(),
                        app.clone(),
                    );
                    page.is_redraw.set_neq(false);
                }
                async {}
            })))
            .class(class::CONF_B)
            .child(html!("div", {
                .class("row")
                .children([
                    // left panel
                    html!("div", {
                        .style("width","350px")
                        .apply_if(page.is_ipd, |dom| { dom
                            .children([
                                html!("div", {
                                    .class(class::INPUT_GROUP_SM)
                                    .children([
                                        doms::span_group_text("Ward"),
                                        html!("div", {
                                            .class(class::FLEX_W100)
                                            .child(html!("select" => HtmlSelectElement, {
                                                .class(class::FORM_CTRL_SM)
                                                .attr("id", "wards")
                                                .children(ward_select_option.iter().map(|option| {
                                                    doms::select_option(option, &app.ward_select.lock_ref())
                                                }))
                                                .prop_signal("value", app.ward_select.signal_cloned())
                                                .with_node!(element => {
                                                    .event(clone!(app, page, element => move |_: events::Change| {
                                                        app.ward_select.set_neq(element.value());
                                                        app.to_local_storage();
                                                        page.search_changed.set_neq(true);
                                                    }))
                                                })
                                            }))
                                        }),
                                        html!("button", {
                                            .attr("type", "button")
                                            .class(class::BTN_SM_RED)
                                            .child(html!("i", {.class(class::FA_X)}))
                                            .event(clone!(app, page => move |_:events::Click| {
                                                let empty_ward = app.ward_select.lock_ref().is_empty();
                                                if !empty_ward {
                                                    app.ward_select.set(String::new());
                                                    if let Some(elm) = app.get_id("wards") {
                                                        NiceSelect::new_default(&elm);
                                                    }
                                                    page.search_changed.set_neq(true);
                                                }
                                            }))
                                        }),
                                    ])
                                }),
                                html!("div", {
                                    .class(class::INPUT_GROUP_SM)
                                    .class("mt-2")
                                    .children([
                                        html!("input" => HtmlInputElement, {
                                            .attr("type", "text")
                                            .class(class::FORM_CTRL_SM)
                                            .focused(true)
                                            .attr("placeholder", "HN/AN/ชื่อ-สกุล")
                                            .prop_signal("value", page.search.signal_cloned())
                                            .with_node!(element => {
                                                .event_with_options(&EventOptions::preventable(), clone!(page, element => move |event: events::KeyDown| {
                                                    if event.key() == "Enter" {
                                                        event.prevent_default();
                                                        page.search.set_neq(element.value());
                                                        page.search_changed.set_neq(true);
                                                    }
                                                }))
                                                .event(clone!(page => move |_: events::Change| {
                                                    page.search.set_neq(element.value());
                                                }))
                                            })
                                        }),
                                        html!("button", {
                                            .attr("type", "button")
                                            .class(class::BTN_SM_RED)
                                            .child(html!("i", {.class(class::FA_X)}))
                                            .event(clone!(page => move |_:events::Click| {
                                                let empty_ward = page.search.lock_ref().is_empty();
                                                if !empty_ward {
                                                    page.search.set(String::new());
                                                    page.search_changed.set_neq(true);
                                                }
                                            }))
                                        }),
                                        html!("button", {
                                            .attr("type", "button")
                                            .class(class::BTN_SM_BLUE)
                                            .text("ค้นหา")
                                            .event(clone!(page => move |_: events::Click| {
                                                page.search_changed.set_neq(true);
                                            }))
                                        }),
                                    ])
                                }),
                            ])
                        })
                        .child(html!("div", {
                            //.attr("id", "show-patient")
                            .style("height","calc(100vh - 165px)")
                            .style("width", "100%")
                            .style("box-sizing","border-box")
                            .style("overflow-y","auto")
                            .apply_if(page.is_ipd, |dom| { dom.class("mt-2")})
                            .children([
                                html!("table", {
                                    .class(class::TABLE_STRIP)
                                    .children([
                                        html!("thead", {
                                            .child(html!("tr", {
                                                .child(html!("th", {
                                                    .attr("scope", "col")
                                                    .class("th-sm")
                                                    .text("รายชื่อผู้ป่วย")
                                                }))
                                            }))
                                        }),
                                        html!("tbody", {
                                            .children_signal_vec(page.search_result.signal_vec_cloned().map(clone!(app, page => move |row| {
                                                doms::render_avatar(row, page.search_selected.clone(), app.state())
                                            })))
                                        }),
                                    ])
                                })
                            ])
                        }))
                    }),
                    // middle + right panel
                    html!("div", {
                        .class(class::COL_PS0)
                        .style("height", "calc(100vh - 90px)")
                        .style("width", "calc(100vw - 365px)")
                        .style("box-sizing", "border-box")
                        .child(html!("div", {
                            .attr("id", "show-patient-header")
                            .child_signal(page.search_selected.signal_cloned().map(clone!(app, page => move |opt| {
                                opt.as_ref().and_then(clone!(page, app => move |visit_type| {
                                    let show_patient_main_opt = match visit_type {
                                        VisitTypeId::Ipd(an) | VisitTypeId::PreAdmit(an) => (!an.is_empty()).then(|| {
                                            ShowPatientMainCpn::new_with_an(an.to_owned())
                                        }),
                                        VisitTypeId::OpdEr(_vn, opd_er_order_master_id) => (*opd_er_order_master_id > 0).then(|| {
                                            ShowPatientMainCpn::new_with_id(*opd_er_order_master_id)
                                        }),
                                        VisitTypeId::Visit(vn) => (!vn.is_empty()).then(|| {
                                            ShowPatientMainCpn::new_with_vn(vn.to_owned())
                                        }),
                                    };
                                    show_patient_main_opt.map(|show_patient_main| {
                                        let dom = ShowPatientMainCpn::render(true, show_patient_main.clone(), app);
                                        page.patient.set(Some(show_patient_main.patient.clone()));
                                        page.patient_loaded.set(show_patient_main.loaded.clone());
                                        dom
                                    })
                                }))
                            })))
                        }))
                        .child(html!("div", {
                            .class_signal("mt-2", page.search_selected.signal_cloned().map(|opt| opt.is_some()))
                            .children([
                                doms::alert_row(clone!(app, page => move |alert| { alert
                                    .class("mx-0")
                                    .attr("id", "index-query-tools")
                                    .children([
                                        doms::form_inline(clone!(app, page => move |form| { form
                                            .children([
                                                html!("div", {
                                                    .class("col-12")
                                                    .child(doms::nurse_assign_dropdown(page.nurse_assign.clone(), page.is_redraw.clone(), app.state()))
                                                }),
                                                html!("div", {
                                                    .class("col-12")
                                                    .child(doms::order_item_types_radio(page.order_item_type.clone(), page.is_redraw.clone()))
                                                }),
                                                html!("div", {
                                                    .class("col-12")
                                                    .child(html!("div", {
                                                        .class(class::FORM_CHK_SW)
                                                        .children([
                                                            html!("input" => HtmlInputElement, {
                                                                .attr("type", "checkbox")
                                                                .class("form-check-input")
                                                                .attr("role","switch")
                                                                .attr("id", "without-order-toggle")
                                                                .attr("value", "Y")
                                                                .apply(mixins::checkbox_toggle(page.without_order.clone(), page.index_changed.clone(), "Y", ""))
                                                            }),
                                                            doms::label_check_for("without-order-toggle"," ไม่ผูกกับ Order"),
                                                        ])
                                                    }))
                                                }),
                                                html!("div", {
                                                    .class("col-12")
                                                    .child(doms::index_plan_status_radio(page.status.clone(), page.is_redraw.clone()))
                                                }),
                                            ])
                                        })),
                                        html!("div", {
                                            .class("col-sm")
                                            .child_signal(page.order_items_show.signal_vec_cloned().len().map(|i| {
                                                Some(doms::badge_count_with_limit(i, 200))
                                            }))
                                        }),
                                    ])
                                })),
                                html!("div", {
                                    .style("overflow","auto")
                                    .style_signal("height", map_ref!{
                                        let ws = window_size(),
                                        // we include index_changed here to recalculate height every index_changed
                                        let index_changed = page.index_changed.signal() =>
                                        (*index_changed, *ws)
                                    }.map(clone!(app => move |(_index_changed, ws)| {
                                        let patient_height = app.get_id("show-patient-header").map(|el| el.client_height()).unwrap_or_default() as f64;
                                        let tools_height = app.get_id("index-query-tools").map(|el| el.client_height()).unwrap_or_default() as f64;
                                        let cal_height = ws.height - patient_height - tools_height - 110.0;
                                        [&cal_height.to_string(), "px"].concat()
                                    })))
                                    .with_node!(element => {
                                        .future(page.is_rescroll.signal().for_each(clone!(page, element => move |rescroll| {
                                            if rescroll {
                                                let (x, y) = page.scroll_position.get();
                                                element.set_scroll_left(x);
                                                element.set_scroll_top(y);
                                                page.is_rescroll.set(false);
                                            }
                                            async {}
                                        })))
                                        .child(html!("div", {
                                            .style("min-width","750px")
                                            .children_signal_vec(page.order_items_show.signal_vec_cloned().map(clone!(app, page, element => move |row| {
                                                render_index_plan(
                                                    row,
                                                    page.index_plan_action_modal.clone(),
                                                    page.patient.get_cloned(),
                                                    Mutable::new(String::from("nurse")),
                                                    page.index_changed.clone(),
                                                    Some((element.clone(), page.scroll_position.clone())),
                                                    app.clone(),
                                                )
                                            })))
                                        }))
                                    })
                                }),
                                html!("div", {
                                    .class("modal")
                                    .attr("id", "indexPlanActionModal")
                                    .attr("role", "dialog")
                                    .attr("tabindex", "-1")
                                    .child_signal(page.index_plan_action_modal.signal_cloned().map(clone!(app, page => move |opt| {
                                        opt.as_ref().map(clone!(app, page => move |modal| {
                                            IndexPlanActionForm::render(
                                                modal.clone(),
                                                page.index_plan_action_modal.clone(),
                                                Some(page.index_changed.clone()),
                                                app,
                                            )
                                        })).or(Some(blank_modal()))
                                    })))
                                }),
                            ])
                        }))
                    }),
                ])
            }))
        })
    }
}
