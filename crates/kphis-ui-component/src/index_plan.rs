// ipd-nurse-index-plan.php
// opd-er-nurse-index-plan.php

use dominator::{Dom, clone, events, html, with_node};
use futures_signals::{
    map_ref,
    signal::{Mutable, Signal, SignalExt, not},
    signal_vec::{MutableVec, SignalVecExt},
};
// use rand::RngExt;
use std::rc::Rc;
use time::{Date, Duration, PrimitiveDateTime, Time};
use web_sys::{HtmlButtonElement, HtmlElement, HtmlInputElement, HtmlSelectElement};

use kphis_model::{
    app::VisitTypeId,
    endpoint::EndPoint,
    fetch::Method,
    index_action::IndexAction,
    index_plan::{IndexActionStatus, IndexPlanDate},
    order::{OrderItem, OrderParams},
    patient_info::PatientInfo,
    report::{SystemReport, TypstReport},
};
use kphis_ui_app::App;
use kphis_ui_core::{class, doms, mixins};
use kphis_util::{
    datetime::{date_th, date_th_opt, datetime_from_opt, datetime_th_opt_relative, datetime_th_relative, js_now, time_hm, time_hm_opt},
    util::{str_some, zero_none},
};

use crate::{
    gadget::pdf_button::PdfButtons,
    modal::{
        blank_modal,
        index_plan_action_form::{FormType, IndexPlanActionForm, OrderType},
    },
};

/// - GET `EndPoint::IpdIndexPlanDateAn`
/// - GET `EndPoint::IpdOrderItem` (self/IndexPlanActionForm)
/// - GET `EndPoint::OpdErOrderItem` (self, IndexPlanActionForm)
/// - POST `EndPoint::IpdIndexAction` (guarded, remove action btns)
/// - POST `EndPoint::OpdErIndexAction` (guarded, remove action btns)
#[derive(Default)]
pub struct IndexPlanCpn {
    view_by: Mutable<String>,

    loaded_order_date: Mutable<bool>,
    loaded_index_plan: Mutable<bool>,

    pub changed: Mutable<bool>,
    checker: Mutable<bool>,
    status: Mutable<Option<String>>,
    nurse_assign: Mutable<String>,
    order_item_type: Mutable<String>,
    is_redraw: Mutable<bool>,
    is_rescroll: Mutable<bool>,

    patient: Mutable<Option<Rc<PatientInfo>>>,

    current_date: Mutable<Option<IndexPlanDate>>,
    plan_dates: MutableVec<Rc<IndexPlanDate>>,

    without_order: Mutable<String>, // "Y"

    order_items_show: MutableVec<Rc<OrderItem>>,
    order_items_all: MutableVec<Rc<OrderItem>>,

    index_plan_action_modal: Mutable<Option<Rc<IndexPlanActionForm>>>,
}

impl IndexPlanCpn {
    pub fn new(patient: Mutable<Option<Rc<PatientInfo>>>, view_by: Mutable<String>) -> Rc<Self> {
        Rc::new(Self {
            patient,
            view_by,
            status: Mutable::new(Some(String::from("wait"))),
            ..Default::default()
        })
    }

    fn is_ipd(&self) -> bool {
        self.patient.lock_ref().as_ref().map(|pt| pt.visit_type.is_ipd()).unwrap_or_default()
    }

    fn current_is_plan_date(&self, plan_date: Rc<IndexPlanDate>) -> impl Signal<Item = bool> + use<> {
        self.current_date.signal_cloned().map(move |opt| opt.as_ref().map(|cd| *cd == *plan_date).unwrap_or_default())
    }

    fn is_today_not_pass_dchdate(&self) -> bool {
        self.patient.lock_ref().as_ref().and_then(|pt| pt.lastdate()).map(|dchdate| js_now().date() <= dchdate).unwrap_or(true)
    }

    fn load_plan_date(page: Rc<Self>, app: Rc<App>) {
        if let Some(patient) = page.patient.get_cloned() {
            match patient.visit_type() {
                VisitTypeId::Ipd(an) | VisitTypeId::PreAdmit(an) => {
                    app.async_load(
                        true,
                        clone!(app => async move {
                            // GET `EndPoint::IpdIndexPlanDateAn`
                            match IndexPlanDate::call_api_get(&an, app.state()).await {
                                Ok(response) => {
                                    let mut dates = Vec::new();
                                    let now = js_now().date();
                                    let today = IndexPlanDate { plan_date: now, is_today: true };
                                    if !response.iter().any(|od| od.plan_date == now) {
                                        if let Some(dchdate) = patient.lastdate() {
                                            if now <= dchdate {
                                                dates.push(today);
                                            }
                                        } else {
                                            dates.push(today);
                                        }
                                    }
                                    dates.extend(response);
                                    page.current_date.set_neq(dates.first().cloned());

                                    let mut lock = page.plan_dates.lock_mut();
                                    if !lock.is_empty() {
                                        lock.clear();
                                    }
                                    dates.sort_by(|a, b| b.plan_date.cmp(&a.plan_date));
                                    lock.extend(dates.iter().map(|d| Rc::new(d.clone())));
                                }
                                Err(e) => {
                                    app.alert_app_error(&e).await;
                                }
                            }
                        }),
                    );
                }
                VisitTypeId::OpdEr(_, _) | VisitTypeId::Visit(_) => {}
            }
        }
    }

    pub fn load_index_plan(page: Rc<Self>, app: Rc<App>) {
        page.order_items_all.lock_mut().clear();
        page.order_items_show.lock_mut().clear();
        let patient = page.patient.get_cloned();
        match patient.as_ref().map(|pt| pt.visit_type()) {
            Some(VisitTypeId::Ipd(an)) | Some(VisitTypeId::PreAdmit(an)) => {
                app.async_load(
                    true,
                    clone!(app, page => async move {
                        let params = OrderParams {
                            an: str_some(an),
                            view_by: Some(String::from("doctor")),
                            plan_date: page.current_date.lock_ref().as_ref().map(|d| d.plan_date),
                            without_order: str_some(page.without_order.get_cloned()),
                            ..Default::default()
                        };
                        // GET `EndPoint::IpdOrderItem`
                        match OrderItem::call_api_get_ipd(&params, app.state()).await {
                            Ok(order_items) => {
                                let dch_date = patient.as_ref().and_then(|pt| pt.dchdate);
                                let dch_time = patient.as_ref().and_then(|pt| pt.dchtime);
                                page.checker.set_neq(!order_items.is_empty());
                                let items_iter = order_items.into_iter().map(Rc::new);
                                page.order_items_all.lock_mut().extend(items_iter.clone());
                                page.order_items_show.lock_mut().extend(items_iter);
                                redraw_index_plan(
                                    dch_date,
                                    dch_time,
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
                    }),
                );
            }
            Some(VisitTypeId::OpdEr(_vn, opd_er_order_master_id)) => app.async_load(
                true,
                clone!(app, page => async move {
                    let params = OrderParams {
                        opd_er_order_master_id: zero_none(opd_er_order_master_id),
                        view_by: Some(String::from("doctor")),
                        without_order: str_some(page.without_order.get_cloned()),
                        ..Default::default()
                    };
                    // GET `EndPoint::OpdErOrderItem`
                    match OrderItem::call_api_get_opd_er(&params, app.state()).await {
                        Ok(order_items) => {
                            let last_datetime = patient.as_ref().and_then(|pt| pt.latest_vs_datetime);
                            page.checker.set_neq(!order_items.is_empty());
                            let items_iter = order_items.into_iter().map(Rc::new);
                            page.order_items_all.lock_mut().extend(items_iter.clone());
                            page.order_items_show.lock_mut().extend(items_iter);
                            redraw_index_plan(
                                last_datetime.map(|dt| dt.date()),
                                last_datetime.map(|dt| dt.time()),
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
                }),
            ),
            Some(VisitTypeId::Visit(_)) | None => {}
        }
    }

    pub fn render(page: Rc<Self>, app: Rc<App>) -> Dom {
        html!("div", {
            .apply_if(page.is_ipd(), |dom| { dom
                .future(map_ref!(
                    let busy = app.loader_is_loading(),
                    let loaded = page.loaded_order_date.signal() =>
                    !busy && !loaded
                ).for_each(clone!(app, page => move |ready| {
                    if ready {
                        Self::load_plan_date(page.clone(), app.clone());
                        page.loaded_order_date.set_neq(true);
                    }
                    async {}
                })))
            })
            .future(map_ref!(
                let busy = app.loader_is_loading(),
                let loaded = page.loaded_index_plan.signal() =>
                !busy && !loaded
            ).for_each(clone!(app, page => move |ready| {
                if ready {
                    Self::load_index_plan(page.clone(), app.clone());
                    page.loaded_index_plan.set_neq(true);
                }
                async {}
            })))
            .future(map_ref!(
                let busy = app.loader_is_loading(),
                let changed = page.changed.signal() =>
                !busy && *changed
            ).for_each(clone!(app, page => move |ready| {
                if ready {
                    Self::load_index_plan(page.clone(), app.clone());
                    page.changed.set_neq(false);
                }
                async {}
            })))
            .future(page.is_redraw.signal().for_each(clone!(app, page => move |redraw| {
                if redraw {
                    let patient = page.patient.lock_ref();
                    let (last_date, last_time) = if page.is_ipd() {
                        (patient.as_ref().and_then(|pt| pt.dchdate), patient.as_ref().and_then(|pt| pt.dchtime))
                    } else {
                        let last_datetime = patient.as_ref().and_then(|pt| pt.latest_vs_datetime);
                        (last_datetime.map(|dt| dt.date()), last_datetime.map(|dt| dt.time()))
                    };
                    redraw_index_plan(
                        last_date,
                        last_time,
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
            .future(page.is_rescroll.signal().for_each(clone!(app, page => move |rescroll| {
                if rescroll {
                    app.scroll_position_restore();
                    page.is_rescroll.set(false);
                }
                async {}
            })))
            .children([
                html!("div", {
                    .children([
                        html!("div", {
                            .class(class::FLEX_WRAP_T)
                            .apply_if(page.is_ipd(), |dom| { dom
                                .child(html!("div", {
                                    .class(class::COLA_PY_L)
                                    .visible_signal(not(page.plan_dates.signal_vec_cloned().is_empty()))
                                    .child(html!("div", {
                                        .class(class::INPUT_GROUP)
                                        .children([
                                            html!("div", {
                                                .class("input-group-text")
                                                .text("วันที่เริ่มต้น Plan")
                                            }),
                                            html!("select" => HtmlSelectElement, {
                                                .class("form-select")
                                                .child(html!("option", {.attr("value", "").text("ทุกวัน")}))
                                                .children_signal_vec(page.plan_dates.signal_vec_cloned().map(|date| {
                                                    html!("option", {
                                                        .attr("value", &date.string())
                                                        .text(&[date_th(&date.plan_date), if date.is_today {String::from(" (วันนี้)")} else {String::new()}].concat())
                                                        .apply_if(date.is_today, |dom| dom.attr("selected",""))
                                                    })
                                                }))
                                                .prop_signal("value", page.current_date.signal_cloned().map(|opt| opt.as_ref().map(|d| d.string()).unwrap_or_default()))
                                                .with_node!(element => {
                                                    .event(clone!(page => move |_: events::Change| {
                                                        let value = element.value();
                                                        page.current_date.set_neq(IndexPlanDate::from_string(&value));
                                                        if value.is_empty() {
                                                            page.status.set_neq(None);
                                                        }
                                                        page.loaded_index_plan.set_neq(false);
                                                    }))
                                                })
                                                // .attr("onchange", "onchange_select_index_plan_date(event)")
                                            }),
                                        ])
                                    }))
                                }))
                                .child(html!("div", {
                                    .class(class::COLA_P)
                                        .child(html!("button", {
                                            .attr("type", "button")
                                            .class("btn")
                                            .class_signal("btn-primary", page.current_date.signal_cloned().map(|opt| opt.is_none()))
                                            .class_signal("btn-secondary", page.current_date.signal_cloned().map(|opt| opt.is_some()))
                                            .text("ทั้งหมด")
                                            .event(clone!(page => move |_: events::Click| {
                                                page.current_date.set_neq(None);
                                                page.loaded_index_plan.set_neq(false);
                                            }))
                                        }))
                                }))
                                .children_signal_vec(page.plan_dates.signal_vec_cloned().map(clone!(page => move |date| {
                                    html!("div", {
                                        .class(class::COLA_P)
                                        .child(html!("button", {
                                            .attr("type", "button")
                                            .class("btn")
                                            .class_signal("btn-primary", page.current_is_plan_date(date.clone()))
                                            .class_signal("btn-secondary", not(page.current_is_plan_date(date.clone())))
                                            .text(&[date_th(&date.plan_date), if date.is_today {String::from(" (วันนี้)")} else {String::new()}].concat())
                                            .event(clone!(page => move |_: events::Click| {
                                                page.current_date.set_neq(Some(date.as_ref().to_owned()));
                                                page.status.set_neq(None);
                                                page.loaded_index_plan.set_neq(false);
                                            }))
                                        }))
                                    })
                                })))
                            })
                            .child(html!("div", {
                                .class(class::PY_RX)
                                .apply_if(page.is_today_not_pass_dchdate(), |dom| dom.child(html!("button", {
                                    .attr("type", "button")
                                    .class(class::BTN_L_BLUE)
                                    //.attr("id", "openIndexPlanActionButton")
                                    .attr("data-bs-toggle", "modal")
                                    .attr("data-bs-target", "#indexPlanActionModal")
                                    .child(html!("i", {.class(class::FA_CLIPBOARD)}))
                                    .text(" เพิ่ม Plan ที่ไม่ผูก Order")
                                    .event(clone!(page => move |_: events::Click| {
                                        page.index_plan_action_modal.set(Some(IndexPlanActionForm::new(
                                            0,
                                            None,
                                            None,
                                            page.patient.clone(),
                                            OrderType::Continuous,
                                            FormType::Plan,
                                            page.view_by.clone(),
                                        )));
                                    }))
                                    // .attr("onclick", "onclickAddIndexPlanOrderItem(event,null,null);")
                                })))
                                .apply(|dom| {
                                    if page.is_ipd() { dom
                                        .children_signal_vec(clone!(page, app => page.patient.signal_cloned().map(move |opt| {
                                            if let Some(patient) = opt {
                                                match patient.visit_type() {
                                                    VisitTypeId::Ipd(an)
                                                    | VisitTypeId::PreAdmit(an) => {
                                                        PdfButtons::buttons(
                                                            PdfButtons::new(
                                                                TypstReport::from_system_with_coercion(SystemReport::IpdIndexPlan, &app.state().report_coercions()),
                                                                Mutable::new(an.clone()),
                                                                page.checker.clone(),
                                                                page.changed.clone(),
                                                                clone!(page => move || {serde_json::json!({
                                                                    "id": an,
                                                                    "patient": patient,
                                                                    "plan":  page.order_items_show.lock_ref().to_vec(),
                                                                }).to_string()})
                                                            ), "PDF", Some("PDF (All)"), app.clone()
                                                        )
                                                    }
                                                    VisitTypeId::OpdEr(_, _)
                                                    | VisitTypeId::Visit(_) => Vec::new(),
                                                }
                                            } else {
                                                Vec::new()
                                            }
                                        }).to_signal_vec()))
                                    } else { dom
                                        .children_signal_vec(clone!(page, app => page.patient.signal_cloned().map(move |opt| {
                                            if let Some(patient) = opt {
                                                if let VisitTypeId::OpdEr(vn, _) = patient.visit_type() {
                                                    PdfButtons::buttons(
                                                        PdfButtons::new(
                                                            TypstReport::from_system_with_coercion(SystemReport::OpdErIndexPlan, &app.state().report_coercions()),
                                                            Mutable::new(vn.clone()),
                                                            page.checker.clone(),
                                                            page.changed.clone(),
                                                            clone!(page => move || {serde_json::json!({
                                                                "id": vn,
                                                                "patient": patient,
                                                                "plan":  page.order_items_show.lock_ref().to_vec(),
                                                            }).to_string()})
                                                        ), "PDF", None, app.clone()
                                                    )
                                                } else {
                                                    Vec::new()
                                                }
                                            } else {
                                                Vec::new()
                                            }
                                        }).to_signal_vec()))
                                    }
                                })
                            }))
                        }),
                        html!("div", {
                            .class("row")
                            .children([
                                html!("div", {
                                    .class(class::COLA_PX_T_PE0)
                                    .child(doms::nurse_assign_dropdown(page.nurse_assign.clone(), page.is_redraw.clone(), app.state()))
                                }),
                                html!("div", {
                                    .class(class::COLA_PX_T_PE0)
                                    .child(doms::order_item_types_radio(page.order_item_type.clone(), page.is_redraw.clone()))
                                }),
                                html!("div", {
                                    .class(class::COLA_PX_T_PE0)
                                    .class("pt-1")
                                    .child(html!("div", {
                                        .class(class::FORM_CHK_SW)
                                        .children([
                                            html!("input" => HtmlInputElement, {
                                                .attr("type", "checkbox")
                                                .class("form-check-input")
                                                .attr("role","switch")
                                                .attr("id", "without-order-toggle")
                                                .attr("value", "Y")
                                                .apply(mixins::checkbox_toggle(page.without_order.clone(), page.changed.clone(), "Y", ""))
                                            }),
                                            doms::label_check_for("without-order-toggle"," ไม่ผูกกับ Order")
                                        ])
                                    }))
                                }),
                                html!("div", {
                                    .class(class::COLA_T)
                                    .child_signal(page.current_date.signal_cloned().map(clone!(page => move |current_date| {
                                        current_date.is_none().then(|| {
                                            doms::index_plan_status_radio(page.status.clone(), page.is_redraw.clone())
                                        })
                                    })))
                                }),
                            ])
                        }),
                    ])
                }),
                html!("div", {
                    .style("min-width","750px")
                    .children_signal_vec(page.order_items_show.signal_vec_cloned().map(clone!(app, page => move |row| {
                        render_index_plan(
                            row,
                            page.index_plan_action_modal.clone(),
                            Some(page.patient.clone()),
                            page.view_by.clone(),
                            page.changed.clone(),
                            None,
                            app.clone(),
                        )
                    })))
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
                                Some(page.changed.clone()),
                                app,
                            )
                        })).or(Some(blank_modal()))
                    })))
                }),
            ])
        })
    }
}

pub fn redraw_index_plan(
    ref_date: Option<Date>,
    ref_time: Option<Time>,
    order_items_show: MutableVec<Rc<OrderItem>>,
    order_items_all: MutableVec<Rc<OrderItem>>,
    nurse_assign: Mutable<String>,
    order_item_type: Mutable<String>,
    status: Mutable<Option<String>>,
    app: Rc<App>,
) {
    let nurse_assign = str_some(nurse_assign.get_cloned());
    let order_item_type = str_some(order_item_type.get_cloned());
    let items = match status.lock_ref().as_ref() {
        Some(status) => match status.as_str() {
            "wait" => order_items_all
                .lock_ref()
                .iter()
                .filter(|item| {
                    !item.index_plans.is_empty()
                        && item.nurse_assign.clone().zip(nurse_assign.clone()).map(|(a, b)| a == b).unwrap_or(true)
                        && item
                            .order_item_type
                            .clone()
                            .zip(order_item_type.clone())
                            .map(|(a, b)| if b == "other" { !["med", "ivfluid", "injection"].contains(&a.as_str()) } else { a == b })
                            .unwrap_or(true)
                        && item.index_plans.iter().any(|plan| {
                            let (wait, _missed, _done) = plan.check_wait_missed_done(ref_date, ref_time, app.state());
                            wait
                        })
                })
                .cloned()
                .collect::<Vec<Rc<OrderItem>>>(),
            "missed" => order_items_all
                .lock_ref()
                .iter()
                .filter(|item| {
                    !item.index_plans.is_empty()
                        && item.nurse_assign.clone().zip(nurse_assign.clone()).map(|(a, b)| a == b).unwrap_or(true)
                        && item
                            .order_item_type
                            .clone()
                            .zip(order_item_type.clone())
                            .map(|(a, b)| if b == "other" { !["med", "ivfluid", "injection"].contains(&a.as_str()) } else { a == b })
                            .unwrap_or(true)
                        && item.index_plans.iter().any(|plan| {
                            let (_wait, missed, _done) = plan.check_wait_missed_done(ref_date, ref_time, app.state());
                            missed
                        })
                })
                .cloned()
                .collect::<Vec<Rc<OrderItem>>>(),
            "done" => order_items_all
                .lock_ref()
                .iter()
                .filter(|item| {
                    !item.index_plans.is_empty()
                        && item.nurse_assign.clone().zip(nurse_assign.clone()).map(|(a, b)| a == b).unwrap_or(true)
                        && item
                            .order_item_type
                            .clone()
                            .zip(order_item_type.clone())
                            .map(|(a, b)| if b == "other" { !["med", "ivfluid", "injection"].contains(&a.as_str()) } else { a == b })
                            .unwrap_or(true)
                        && item.index_plans.iter().all(|plan| {
                            let (_wait, _missed, done) = plan.check_wait_missed_done(ref_date, ref_time, app.state());
                            done
                        })
                })
                .cloned()
                .collect::<Vec<Rc<OrderItem>>>(),
            _ => order_items_all
                .lock_ref()
                .iter()
                .filter(|item| {
                    !item.index_plans.is_empty()
                        && item.nurse_assign.clone().zip(nurse_assign.clone()).map(|(a, b)| a == b).unwrap_or(true)
                        && item
                            .order_item_type
                            .clone()
                            .zip(order_item_type.clone())
                            .map(|(a, b)| if b == "other" { !["med", "ivfluid", "injection"].contains(&a.as_str()) } else { a == b })
                            .unwrap_or(true)
                })
                .cloned()
                .collect::<Vec<Rc<OrderItem>>>(),
        },
        None => order_items_all
            .lock_ref()
            .iter()
            .filter(|item| {
                !item.index_plans.is_empty()
                    && item.nurse_assign.clone().zip(nurse_assign.clone()).map(|(a, b)| a == b).unwrap_or(true)
                    && item
                        .order_item_type
                        .clone()
                        .zip(order_item_type.clone())
                        .map(|(a, b)| if b == "other" { !["med", "ivfluid", "injection"].contains(&a.as_str()) } else { a == b })
                        .unwrap_or(true)
            })
            .cloned()
            .collect::<Vec<Rc<OrderItem>>>(),
    };
    order_items_show.lock_mut().replace_cloned(items);
}

/// if patient_opt is None, system will get patient via row.visit_type
pub fn render_index_plan(
    row: Rc<OrderItem>,
    index_plan_action_modal: Mutable<Option<Rc<IndexPlanActionForm>>>,
    patient_opt: Option<Mutable<Option<Rc<PatientInfo>>>>,
    view_by: Mutable<String>,
    parent_changed: Mutable<bool>,
    scroll_container: Option<(HtmlElement, Mutable<(i32, i32)>)>,
    app: Rc<App>,
) -> Dom {
    let (med_name, order_item_detail) = row
        .order_item_type
        .as_ref()
        .map(|item_type| {
            if item_type == "off" {
                (["Off: ", &row.off_med_name.clone().unwrap_or_default()].concat(), row.off_order_item_detail.clone().unwrap_or_default())
            } else {
                (row.med_name.clone().unwrap_or_default(), row.order_item_detail.clone().unwrap_or_default())
            }
        })
        .unwrap_or_default();

    let is_med = row.order_item_type.as_ref().map(|oit| ["med", "ivfluid", "injection"].contains(&oit.as_str())).unwrap_or_default();
    let (is_ipd, is_pre_admit) = row.visit_type.is_ipd_and_is_pre_admit();

    let mut plans = row.index_plans.clone();
    plans.sort_by(|a, b| {
        if let (Some(sch_a), Some(sch_b)) = (&a.plan_sch_type, &b.plan_sch_type) {
            if sch_a == sch_b {
                if let (Some(date_a), Some(date_b)) = (&a.plan_date, &b.plan_date) {
                    if date_a == date_b {
                        if let (Some(time_a), Some(time_b)) = (&a.plan_time, &b.plan_time) {
                            time_a.cmp(&time_b)
                        } else {
                            a.plan_id.cmp(&b.plan_id)
                        }
                    } else {
                        date_a.cmp(&date_b)
                    }
                } else {
                    a.plan_id.cmp(&b.plan_id)
                }
            } else {
                // day, stat, time
                sch_a.cmp(&sch_b)
            }
        } else {
            a.plan_id.cmp(&b.plan_id)
        }
    });

    html!("div", {
        .class(class::FLEX_E2)
        .class(class::BOX_ROUND_T)
        .children([
            // Order
            html!("div", {
                .style("min-width", "300px")
                .apply(|dom| {
                    if row.order_id.is_some() { dom
                        .child(html!("span", {.text(&[date_th_opt(&row.order_date), time_hm_opt(&row.order_time)].join(" "))}))
                        .apply_if(row.order_type.as_ref().map(|order_type| order_type.as_str() == "continuous").unwrap_or_default(), |d| {
                            d.child(html!("span",{.class(class::BADGE_WRAP_R_BLUE).style("cursor","default").text("Continuous")}))
                        })
                        .apply_if(row.order_owner_type.as_ref().map(|owner_type| owner_type.as_str() == "nurse").unwrap_or_default(), |d| {
                            d.child(html!("span",{.class(class::BADGE_WRAP_R_GOLD).style("cursor","default").text("Nurse")}))
                        })
                        .children([
                            html!("br"),
                            html!("span", {
                                .apply_if(row.off_by_datetime.is_some(), |d| d.style("text-decoration","line-through"))
                                .apply_if(!med_name.is_empty(), |d| {
                                    d.child(html!("span", {
                                        .style("white-space","pre-wrap")
                                        .apply_if(is_med, |dd| {
                                            dd.class(class::BOLD_BLUE_EM_L)
                                        })
                                        .text(&med_name)
                                    }))
                                    // Drug allergy badge
                                    .apply_if(row.allergy_agent_symptom.is_some(), |d| d.child(html!("div", {
                                        .class(class::BADGE_WRAP_R_RED)
                                        .style("cursor","help")
                                        .attr("title", &row.allergy_agent_symptom.clone().unwrap_or(String::from("ไม่ระบุอาการ")))
                                        .text("แพ้ยา/เฝ้าระวัง")
                                    })))
                                    // HAD/LASA badge
                                    .children(app.drug_alert_badge(row.displaycolor))
                                    // Med Reconcile badge
                                    .apply_if(row.med_reconciliation_item_id.is_some(), |d| d.child(html!("div", {
                                        .style("cursor","help")
                                        .apply(|dd| {
                                            match &row.used {
                                                Some(used) => {
                                                    match used.as_str() {
                                                        "N" => dd.class(class::BADGE_WRAP_R_GRAY),
                                                        "H" => dd.class(class::BADGE_WRAP_R_CYAN),
                                                        "Y" => {
                                                            if row.is_med_rec_change_usage() {
                                                                dd.class(class::BADGE_WRAP_R_GOLD)
                                                            } else {
                                                                dd.class(class::BADGE_WRAP_R_GREEN)
                                                            }
                                                        }
                                                        _ => dd,
                                                    }
                                                }
                                                None => dd,
                                            }

                                        })
                                        .attr("title", &row.med_rec_info())
                                        .text("MR")
                                    })))
                                    .child(html!("br"))
                                })
                                .child(html!("span", {
                                    .text(&order_item_detail)
                                }))
                            }),
                        ])
                    } else {
                        dom.text("*รายการนี้ไม่ได้ผูกกับ Order*")
                    }
                })
                .apply_if(row.stat == Some(String::from("Y")), |dom| dom.child(html!("span", {
                    .class(class::BADGE_WRAP_R_RED)
                    .style("cursor","default")
                    .text("STAT")
                })))
                .apply_if(row.off_by_datetime.is_some(), |d| d.child(html!("span", {
                    .class(class::BADGE_WRAP_R_GOLD)
                    .style("cursor","default")
                    .text(&["OFF ", &datetime_th_opt_relative(&row.off_by_datetime)].concat())
                })))
            }),
            // Plans and actions
            html!("div", {
                .class(class::FLEX_COL)
                .style("width", "100%")
                .children(plans.into_iter().map(clone!(row, index_plan_action_modal, patient_opt, view_by => move |plan| {
                    let mut actions = plan.actions.clone();
                    actions.sort();
                    actions.reverse();
                    let checked_not_act = actions.iter().filter(|action| {
                        action.check_datetime.is_some() && action.action_date.is_none() && action.action_time.is_none()
                    }).cloned().collect::<Vec<IndexAction>>();
                    let unchecked_not_act_med = if is_med {
                        actions.iter().filter(|action| {
                            action.check_datetime.is_none() && action.action_date.is_none() && action.action_time.is_none()
                        }).cloned().collect::<Vec<IndexAction>>()
                    } else {
                        Vec::new()
                    };

                    html!("div", {
                        .class(class::BORDER_U_FLEX)
                        .children([
                            // Timing + detailed title
                            html!("span", {
                                .attr("data-bs-toggle", "modal")
                                .attr("data-bs-target", "#indexPlanActionModal")
                                .class(class::TXT_C_MIDDLE)
                                .style("cursor","pointer")
                                .style("width", "70px")
                                .child(html!("span", {
                                    .class("fw-bold")
                                    .text(&plan.schedule_title())
                                }))
                                .apply(|dom| {
                                    if plan.plan_detail.is_some() {
                                        dom.children([
                                            html!("br"),
                                            html!("i", {.class(class::FA_INFO)}),
                                        ])
                                    } else {
                                        dom
                                    }
                                })
                                .attr("title", &[&plan.plan_detail.clone().unwrap_or_default(), " ", &plan.schedule()].concat())
                                .event(clone!(app, row, index_plan_action_modal, patient_opt, view_by, scroll_container => move |_: events::Click| {
                                    match &patient_opt {
                                        Some(patient) => {
                                            index_plan_action_modal.set(Some(IndexPlanActionForm::new(
                                                row.order_item_id,
                                                zero_none(plan.plan_id),
                                                None,
                                                patient.clone(),
                                                // 'without order' will use modal as continuous order
                                                OrderType::new_from_str(&row.order_type.clone().unwrap_or(String::from("continuous"))),
                                                FormType::Action,
                                                view_by.clone(),
                                            )));
                                        }
                                        None => {
                                            index_plan_action_modal.set(Some(IndexPlanActionForm::new_with_visit_type(
                                                row.order_item_id,
                                                zero_none(plan.plan_id),
                                                None,
                                                row.visit_type.clone(),
                                                // 'without order' will use modal as continuous order
                                                OrderType::new_from_str(&row.order_type.clone().unwrap_or(String::from("continuous"))),
                                                FormType::Action,
                                                view_by.clone(),
                                            )));
                                        }
                                    }
                                    if let Some((elm, scroll)) = scroll_container.as_ref() {
                                        scroll.set((elm.scroll_left(), elm.scroll_top()));
                                    } else {
                                        app.scroll_position_set();
                                    }
                                }))
                            }),
                            // Old actions
                            html!("div", {
                                .class(class::BORDER_SIDE)
                                .style("width", "calc(100% - 225px)")
                                .style("max-height","100px")
                                .style("overflow-y","auto")
                                .children(actions.into_iter().map(clone!(app, row, index_plan_action_modal, patient_opt, view_by, scroll_container => move |action| {
                                    // golden badge when > 30 minutes time precision
                                    let (bg_color, sym) = if let (
                                        Some(sch_type), Some(order_type), Some(pd), Some(pt), Some(ad), Some(at)
                                    ) = (
                                        plan.plan_sch_type.as_ref(), row.order_type.as_ref(), plan.plan_date, plan.plan_time, action.action_date, action.action_time
                                    ) {
                                        let is_cont = order_type.as_str() == "continuous";
                                        let plan_dt = PrimitiveDateTime::new(pd, pt);
                                        let action_dt = PrimitiveDateTime::new(ad, at);
                                        let is_missed = match sch_type.as_str() {
                                            "stat" => {
                                                (action_dt - plan_dt).abs() > Duration::new(1800,0)
                                            }
                                            "date" => {
                                                if is_cont {
                                                    action_dt < plan_dt
                                                } else {
                                                    let end_dt = plan_dt + Duration::DAY;
                                                    action_dt < plan_dt || end_dt < action_dt
                                                }
                                            }
                                            _ => { // "time"
                                                if is_cont {
                                                    ad < pd || (at - pt).abs() > Duration::new(1800,0)
                                                } else {
                                                    (action_dt - plan_dt).abs() > Duration::new(1800,0)
                                                }
                                            }
                                        };
                                        if is_missed {
                                            ("text-bg-warning", doms::had_monitor_status(&action, &row, false))
                                        } else {
                                            ("bg-primary", doms::had_monitor_status(&action, &row, false))
                                        }
                                    } else if action.check_datetime.is_some() {
                                        ("text-bg-secondary", html!("span", {.text("\u{2717} รอดำเนินการ")}))
                                    } else if is_med {
                                        ("text-bg-danger", html!("span", {.text("\u{2717} รอเตรียมการ")}))
                                    } else {
                                        ("text-bg-danger", html!("span", {.text("\u{2717} รอดำเนินการ")}))
                                    };

                                    let action_dt = if let Some(adt) = datetime_from_opt(action.action_date, action.action_time) {
                                        datetime_th_relative(&adt)
                                    } else {
                                        [&date_th_opt(&action.action_date), " ", &time_hm_opt(&action.action_time)].concat()
                                    };

                                    html!("span", {
                                        .class(class::BADGE_LB)
                                        .class(bg_color)
                                        .style("cursor","pointer")
                                        .attr("data-bs-toggle", "modal")
                                        .attr("data-bs-target", "#indexPlanActionModal")
                                        .child(sym)
                                        .text(" ")
                                        .text(&action_dt)
                                        .attr("title", &["ผลลัพธ์: ", &action.action_result.clone().unwrap_or(String::from("ไม่ระบุ")), "\nหมายเหตุ: ", &action.action_remark.clone().unwrap_or(String::from("ไม่ระบุ"))].concat())
                                        .event(clone!(app, row, index_plan_action_modal, patient_opt, view_by, scroll_container => move |_: events::Click| {
                                            if let Some(patient) = &patient_opt {
                                                index_plan_action_modal.set(Some(IndexPlanActionForm::new(
                                                    row.order_item_id,
                                                    zero_none(plan.plan_id),
                                                    action.action_id,
                                                    patient.clone(),
                                                    // 'without order' will use modal as continuous order
                                                    OrderType::new_from_str(&row.order_type.clone().unwrap_or(String::from("continuous"))),
                                                    FormType::Action,
                                                    view_by.clone(),
                                                )));
                                            } else {
                                                index_plan_action_modal.set(Some(IndexPlanActionForm::new_with_visit_type(
                                                    row.order_item_id,
                                                    zero_none(plan.plan_id),
                                                    action.action_id,
                                                    row.visit_type.clone(),
                                                    // 'without order' will use modal as continuous order
                                                    OrderType::new_from_str(&row.order_type.clone().unwrap_or(String::from("continuous"))),
                                                    FormType::Action,
                                                    view_by.clone(),
                                                )));
                                            }
                                            if let Some((elm, scroll)) = scroll_container.as_ref() {
                                                scroll.set((elm.scroll_left(), elm.scroll_top()));
                                            } else {
                                                app.scroll_position_set();
                                            }
                                        }))
                                    })
                                })))
                            }),
                            // Shortcut
                            html!("div", {
                                .class("text-center")
                                .style("width","155px")
                                .apply(|dom| {
                                    let mut actions = Vec::new();
                                    let is_allow_post = if is_ipd {
                                        app.endpoint_is_allow(&Method::POST, &EndPoint::IpdIndexAction, is_pre_admit)
                                    } else {
                                        app.endpoint_is_allow(&Method::POST, &EndPoint::OpdErIndexAction, is_pre_admit)
                                    };
                                    if is_allow_post {
                                        if is_med {
                                            if unchecked_not_act_med.is_empty() {
                                                // New Check Now
                                                actions.push(html!("button" => HtmlButtonElement, {
                                                    .attr("type", "button")
                                                    .class(class::BTN_SM_RT_BLUEO)
                                                    .text("Check Now")
                                                    .apply(mixins::click_with_loader_checked(clone!(app, patient_opt, parent_changed, scroll_container => move || {
                                                        if let Some(visit_type) = patient_opt.as_ref().and_then(|ptc| ptc.lock_ref().as_ref().map(|pt| pt.visit_type())) {
                                                            let action_dt = js_now();
                                                            let save = IndexAction {
                                                                visit_type,
                                                                plan_id: zero_none(plan.plan_id),
                                                                check_datetime: Some(action_dt),
                                                                check_person: app.doctor_code(),
                                                                ..Default::default()
                                                            };
                                                            app.async_load(
                                                                true,
                                                                clone!(app, parent_changed, scroll_container => async move {
                                                                    // POST `EndPoint::IpdIndexAction`
                                                                    // POST `EndPoint::OpdErIndexAction`
                                                                    match save.call_api_post(app.state()).await {
                                                                        Ok((id, results)) => if id > 0 && results[0].rows_affected > 0 {
                                                                            if let Some((elm, scroll)) = scroll_container.as_ref() {
                                                                                scroll.set((elm.scroll_left(), elm.scroll_top()));
                                                                            } else {
                                                                                app.scroll_position_set();
                                                                            }
                                                                            parent_changed.set(true);
                                                                        }
                                                                        Err(e) => {
                                                                            app.alert_app_error(&e).await;
                                                                        }
                                                                    }
                                                                }),
                                                            )
                                                        }
                                                    }), app.state()))
                                                }))
                                            } else {
                                                // Check on Unchecks
                                                for uncheck in unchecked_not_act_med {
                                                    actions.push(html!("button" => HtmlButtonElement, {
                                                        .attr("type", "button")
                                                        .class(class::BTN_SM_RT_BLUEO)
                                                        .text("Check Now")
                                                        .apply(mixins::click_with_loader_checked(clone!(app, parent_changed, scroll_container => move || {
                                                            let mut save = uncheck.to_owned();
                                                            save.check_datetime = Some(js_now());
                                                            save.check_person = app.doctor_code();
                                                            app.async_load(
                                                                true,
                                                                clone!(app, parent_changed, scroll_container => async move {
                                                                    // POST `EndPoint::IpdIndexAction`
                                                                    // POST `EndPoint::OpdErIndexAction`
                                                                    match save.call_api_post(app.state()).await {
                                                                        Ok((id, results)) => if id > 0 && results[0].rows_affected > 0 {
                                                                            if let Some((elm, scroll)) = scroll_container.as_ref() {
                                                                                scroll.set((elm.scroll_left(), elm.scroll_top()));
                                                                            } else {
                                                                                app.scroll_position_set();
                                                                            }
                                                                            parent_changed.set(true);
                                                                        }
                                                                        Err(e) => {
                                                                            app.alert_app_error(&e).await;
                                                                        }
                                                                    }
                                                                }),
                                                            )
                                                        }), app.state()))
                                                    }))
                                                }
                                            }
                                            if !checked_not_act.is_empty() {
                                                // Action Now on Checked
                                                for checked in checked_not_act {
                                                    // Dom of Action Now on Checked
                                                    let action_now = html!("button" => HtmlButtonElement, {
                                                        .attr("type", "button")
                                                        .class(class::BTN_SM_RT_REDO)
                                                        .text("Action Now")
                                                        .child(html!("br"))
                                                        .text("(เตรียม ")
                                                        .text(&datetime_th_opt_relative(&checked.check_datetime))
                                                        .text(")")
                                                        .apply(mixins::click_with_loader_checked(clone!(app, parent_changed, scroll_container, checked => move || {
                                                            let mut save = checked.to_owned();
                                                            let action_dt = js_now();
                                                            save.action_date = Some(action_dt.date());
                                                            save.action_time = Some(action_dt.time());
                                                            save.action_person_1 = app.doctor_code();
                                                            app.async_load(
                                                                true,
                                                                clone!(app, parent_changed, scroll_container => async move {
                                                                    // POST `EndPoint::IpdIndexAction`
                                                                    // POST `EndPoint::OpdErIndexAction`
                                                                    match save.call_api_post(app.state()).await {
                                                                        Ok((id, results)) => if id > 0 && results[0].rows_affected > 0 {
                                                                            if let Some((elm, scroll)) = scroll_container.as_ref() {
                                                                                scroll.set((elm.scroll_left(), elm.scroll_top()));
                                                                            } else {
                                                                                app.scroll_position_set();
                                                                            }
                                                                            parent_changed.set(true);
                                                                        }
                                                                        Err(e) => {
                                                                            app.alert_app_error(&e).await;
                                                                        }
                                                                    }
                                                                }),
                                                            )
                                                        }), app.state()))
                                                    });

                                                    if let (Some(plan_time), Some(check_datetime)) = (plan.plan_time, checked.check_datetime) {
                                                        let action_at_plan = PrimitiveDateTime::new(js_now().date(), plan_time);
                                                        // Check MUST before Action (included random 10 minutes)
                                                        if (action_at_plan - Duration::new(10 * 60, 0)) > check_datetime {
                                                            // show 2 buttons (At plan + Now)
                                                            // Action at Planed time on Checked
                                                            actions.push(html!("button" => HtmlButtonElement, {
                                                                .attr("type", "button")
                                                                .class(class::BTN_SM_RT_BLUEO)
                                                                .text("วันนี้ ")
                                                                .text(&time_hm(&plan_time))
                                                                .child(html!("br"))
                                                                .text("(เตรียม ")
                                                                .text(&datetime_th_relative(&check_datetime))
                                                                .text(")")
                                                                .apply(mixins::click_with_loader_checked(clone!(app, parent_changed, scroll_container => move || {
                                                                    let mut save = checked.to_owned();
                                                                    let plan_dt = PrimitiveDateTime::new(js_now().date(), plan_time);

                                                                    let random_number = app.get_random(&mut [0]) as i64; // 0-255
                                                                    let action_dt = plan_dt - Duration::new(random_number * 2, 0);
                                                                    save.action_date = Some(action_dt.date());
                                                                    save.action_time = Some(action_dt.time());
                                                                    save.action_person_1 = app.doctor_code();
                                                                    app.async_load(
                                                                        true,
                                                                        clone!(app, parent_changed, scroll_container => async move {
                                                                            // POST `EndPoint::IpdIndexAction`
                                                                            // POST `EndPoint::OpdErIndexAction`
                                                                            match save.call_api_post(app.state()).await {
                                                                                Ok((id, results)) => if id > 0 && results[0].rows_affected > 0 {
                                                                                    if let Some((elm, scroll)) = scroll_container.as_ref() {
                                                                                        scroll.set((elm.scroll_left(), elm.scroll_top()));
                                                                                    } else {
                                                                                        app.scroll_position_set();
                                                                                    }
                                                                                    parent_changed.set(true);
                                                                                }
                                                                                Err(e) => {
                                                                                    app.alert_app_error(&e).await;
                                                                                }
                                                                            }
                                                                        }),
                                                                    )
                                                                }), app.state()))
                                                            }));
                                                            // Action Now on Checked
                                                            actions.push(action_now);
                                                        } else {
                                                            // Action Now on Checked
                                                            actions.push(action_now);
                                                        }
                                                    } else {
                                                        // Action Now on Checked
                                                        actions.push(action_now);
                                                    }
                                                }
                                            }
                                        } else if let Some(plan_time) = plan.plan_time {
                                            // Action at Planed time (Non-Med: can do without check)
                                            actions.push(html!("button" => HtmlButtonElement, {
                                                .attr("type", "button")
                                                .class(class::BTN_SM_RT_BLUEO)
                                                .text("วันนี้ ")
                                                .text(&time_hm(&plan_time))
                                                .apply(mixins::click_with_loader_checked(clone!(app, patient_opt, parent_changed, scroll_container => move || {
                                                    if let Some(visit_type) = patient_opt.as_ref().and_then(|ptc| ptc.lock_ref().as_ref().map(|pt| pt.visit_type())) {
                                                        let plan_dt = PrimitiveDateTime::new(js_now().date(), plan_time);

                                                        let random_number = app.get_random(&mut [0]) as i64; // 0-255
                                                        let action_dt = plan_dt - Duration::new(random_number * 2, 0);

                                                        let save = IndexAction {
                                                            visit_type,
                                                            plan_id: zero_none(plan.plan_id),
                                                            action_date: Some(action_dt.date()),
                                                            action_time: Some(action_dt.time()),
                                                            action_person_1: app.doctor_code(),
                                                            ..Default::default()
                                                        };
                                                        app.async_load(
                                                            true,
                                                            clone!(app, parent_changed, scroll_container => async move {
                                                                // POST `EndPoint::IpdIndexAction`
                                                                // POST `EndPoint::OpdErIndexAction`
                                                                match save.call_api_post(app.state()).await {
                                                                    Ok((id, results)) => if id > 0 && results[0].rows_affected > 0 {
                                                                        if let Some((elm, scroll)) = scroll_container.as_ref() {
                                                                            scroll.set((elm.scroll_left(), elm.scroll_top()));
                                                                        } else {
                                                                            app.scroll_position_set();
                                                                        }
                                                                        parent_changed.set(true);
                                                                    }
                                                                    Err(e) => {
                                                                        app.alert_app_error(&e).await;
                                                                    }
                                                                }
                                                            }),
                                                        )
                                                    }
                                                }), app.state()))
                                            }));
                                            // Action Now (Non-Med: can do without check)
                                            actions.push(html!("button" => HtmlButtonElement, {
                                                .attr("type", "button")
                                                .class(class::BTN_SM_RT_REDO)
                                                .text("Action Now")
                                                .apply(mixins::click_with_loader_checked(clone!(app, patient_opt, parent_changed, scroll_container => move || {
                                                    if let Some(visit_type) = patient_opt.as_ref().and_then(|ptc| ptc.lock_ref().as_ref().map(|pt| pt.visit_type())) {
                                                        let action_dt = js_now();
                                                        let save = IndexAction {
                                                            visit_type,
                                                            plan_id: zero_none(plan.plan_id),
                                                            action_date: Some(action_dt.date()),
                                                            action_time: Some(action_dt.time()),
                                                            action_person_1: app.doctor_code(),
                                                            ..Default::default()
                                                        };
                                                        app.async_load(
                                                            true,
                                                            clone!(app, parent_changed, scroll_container => async move {
                                                                // POST `EndPoint::IpdIndexAction`
                                                                // POST `EndPoint::OpdErIndexAction`
                                                                match save.call_api_post(app.state()).await {
                                                                    Ok((id, results)) => if id > 0 && results[0].rows_affected > 0 {
                                                                        if let Some((elm, scroll)) = scroll_container.as_ref() {
                                                                            scroll.set((elm.scroll_left(), elm.scroll_top()));
                                                                        } else {
                                                                            app.scroll_position_set();
                                                                        }
                                                                        parent_changed.set(true);
                                                                    }
                                                                    Err(e) => {
                                                                        app.alert_app_error(&e).await;
                                                                    }
                                                                }
                                                            }),
                                                        )
                                                    }
                                                }), app.state()))
                                            }));
                                        }
                                    }

                                    if actions.is_empty() {
                                        dom
                                    } else {
                                        dom.children(actions)
                                    }
                                })
                            }),
                        ])
                    })
                })))
            })
        ])
    })
}
