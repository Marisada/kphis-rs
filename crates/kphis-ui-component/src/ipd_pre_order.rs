// ipd-dr-pre-order.php

use dominator::{Dom, clone, events, html};
use futures_signals::{
    map_ref,
    signal::{Mutable, SignalExt, not},
    signal_vec::{MutableVec, SignalVecExt},
};
use std::{ops::Deref, rc::Rc};
use web_sys::HtmlButtonElement;

use kphis_model::{
    endpoint::EndPoint,
    fetch::Method,
    order::{Order, OrderTypeName},
    pre_order::{
        order::{PreOrder, PreOrderParams},
        progress_note::{PreProgressNote, PreProgressNoteParams},
    },
    progress_note::ProgressNote,
    user::permission::Permission,
};
use kphis_ui_app::App;
use kphis_ui_core::{class, mixins};
use kphis_util::{
    datetime::{date_th, datetime_th_opt, time_hm},
    util::zero_none,
};

use crate::order_form::{continuous::ContinuousForm, oneday::OneDayForm, progress_note::ProgressNoteForm};

/// - GET `EndPoint::IpdPreOrderOrder`
/// - GET `EndPoint::IpdPreOrderProgressNote`
/// - POST `EndPoint::IpdPreOrderOrder` (OneDayForm/ContinuousForm, guarded, remove '+Add' btn)
/// - POST `EndPoint::IpdPreOrderProgressNote` (ProgressNoteForm, guarded, remove '+Add' btn)
/// - DELETE `EndPoint::IpdPreOrderOrderId` (guarded, remove 'Delete' btn)
/// - DELETE `EndPoint::IpdPreOrderProgressNoteId` (guarded, remove 'Delete' btn)
#[derive(Default)]
pub struct IpdPreOrderCpn {
    pub loaded_order_all: Mutable<bool>,
    pre_order_master_id: Mutable<u32>,

    oneday: MutableVec<Rc<PreOrder>>,
    continuous: MutableVec<Rc<PreOrder>>,
    progress_note: MutableVec<Rc<PreProgressNote>>,

    reload_order_oneday: Mutable<bool>,
    reload_order_continuous: Mutable<bool>,
    reload_progress_note: Mutable<bool>,

    edit_order: Mutable<Option<Rc<Order>>>,
    edit_progress_note: Mutable<Option<Rc<ProgressNote>>>,

    show_oneday_input: Mutable<bool>,
    show_continuous_input: Mutable<bool>,
    show_progress_note_input: Mutable<bool>,
}

impl IpdPreOrderCpn {
    pub fn new(pre_order_master_id: Mutable<u32>) -> Rc<Self> {
        Rc::new(Self {
            pre_order_master_id,
            ..Default::default()
        })
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

    pub async fn load_order_oneday(page: Rc<Self>, app: Rc<App>) {
        page.oneday.lock_mut().clear();
        let pre_order_master_id = zero_none(page.pre_order_master_id.get());
        let params = PreOrderParams {
            pre_order_master_id,
            order_id: None,
            order_type: Some(String::from("oneday")),
            order_confirm: None,
            order_owner_type: None,
            view_by: Some(String::from("doctor")),
        };
        // GET `EndPoint::IpdPreOrderOrder`
        match PreOrder::call_api_get(&params, app.state()).await {
            Ok(orders) => page.oneday.lock_mut().extend(orders.into_iter().map(Rc::new)),
            Err(e) => {
                app.alert_app_error(&e).await;
            }
        }
    }

    pub async fn load_order_continuous(page: Rc<Self>, app: Rc<App>) {
        page.continuous.lock_mut().clear();
        let pre_order_master_id = zero_none(page.pre_order_master_id.get());
        let params = PreOrderParams {
            pre_order_master_id,
            order_id: None,
            order_type: Some(String::from("continuous")),
            order_confirm: None,
            order_owner_type: None,
            view_by: Some(String::from("doctor")),
        };
        // GET `EndPoint::IpdPreOrderOrder`
        match PreOrder::call_api_get(&params, app.state()).await {
            Ok(orders) => page.continuous.lock_mut().extend(orders.into_iter().map(Rc::new)),
            Err(e) => {
                app.alert_app_error(&e).await;
            }
        }
    }

    pub async fn load_progress_note(page: Rc<Self>, app: Rc<App>) {
        page.progress_note.lock_mut().clear();
        let pre_order_master_id = zero_none(page.pre_order_master_id.get());
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
    }

    fn delete_order(order: Rc<PreOrder>, is_oneday: bool, page: Rc<Self>, app: Rc<App>) {
        app.async_load(
            true,
            clone!(app, order => async move {
                if app.confirm("ยืนยันรายการ").await {
                    // DELETE `EndPoint::IpdPreOrderOrderId`
                    match PreOrder::call_api_delete(order.order_id, app.state()).await {
                        Ok(response) => {
                            app.alert_execute_response(&response, clone!(app => async move {
                                if is_oneday {
                                    Self::load_order_oneday(page, app).await;
                                } else {
                                    Self::load_order_continuous(page, app).await;
                                }
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

    fn delete_progress_note(progress_note: Rc<PreProgressNote>, page: Rc<Self>, app: Rc<App>) {
        app.async_load(
            true,
            clone!(app, progress_note => async move {
                if app.confirm("ยืนยันรายการ").await {
                    // DELETE `EndPoint::IpdPreOrderProgressNoteId`
                    match PreProgressNote::call_api_delete(progress_note.progress_note_id, app.state()).await {
                        Ok(response) => {
                            app.alert_execute_response(&response, clone!(app => async move {
                                Self::load_progress_note(page, app).await;
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

    pub fn render(used: Mutable<String>, page: Rc<Self>, app: Rc<App>) -> Dom {
        let allow_order_form = app.endpoint_is_allow(&Method::POST, &EndPoint::IpdPreOrderOrder, true) && app.has_permission(Permission::DataTypeDoctorUse);
        let allow_order_add = app.has_permission(Permission::IpdOrderAdd) || app.has_permission(Permission::OpdErOrderAdd);
        let allow_progress_form = app.endpoint_is_allow(&Method::POST, &EndPoint::IpdPreOrderProgressNote, true) && app.has_permission(Permission::DataTypeDoctorUse);
        let allow_progress_add = app.has_permission(Permission::ProgressNoteAdd);

        html!("div", {
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
            .future(map_ref!(
                let busy = app.loader_is_loading(),
                let reload = page.reload_order_oneday.signal() =>
                !busy && *reload
            ).for_each(clone!(app, page => move |ready| {
                if ready {
                    page.reload_order_oneday.set_neq(false);
                    app.async_load(
                        true,
                        clone!(app, page => async move {
                            Self::load_order_oneday(page.clone(), app.clone()).await;
                        })
                    );
                }
                async {}
            })))
            .future(map_ref!(
                let busy = app.loader_is_loading(),
                let reload = page.reload_order_continuous.signal() =>
                !busy && *reload
            ).for_each(clone!(app, page => move |ready| {
                if ready {
                    page.reload_order_continuous.set_neq(false);
                    app.async_load(
                        true,
                        clone!(app, page => async move {
                            Self::load_order_continuous(page.clone(), app.clone()).await;
                        })
                    )
                }
                async {}
            })))
            .future(map_ref!(
                let busy = app.loader_is_loading(),
                let reload = page.reload_progress_note.signal() =>
                !busy && *reload
            ).for_each(clone!(app, page => move |ready| {
                if ready {
                    page.reload_progress_note.set_neq(false);
                    app.async_load(
                        true,
                        clone!(app, page => async move {
                            Self::load_progress_note(page.clone(), app.clone()).await;
                        })
                    )
                }
                async {}
            })))
            .class(class::TAB_FADE_SHOW_ACTIVE)
            .attr("role", "tabpanel")
            .child(html!("div", {
                .class("row")
                .child(html!("div", {
                    .class("col")
                    .child(html!("table", {
                        .class(class::TABLE_1R)
                        //.attr("id", "order-table")
                        .children([
                            html!("thead", {
                                .child(html!("tr", {
                                    .children([
                                        html!("th", {
                                            .attr("scope", "col")
                                            .class(class::TXT_C_TOP)
                                            .class("bg-secondary-subtle")
                                            .style("width","30%")
                                            //.attr("id", "progress-note-column-header")
                                            .apply_if(allow_progress_form && allow_progress_add, |dom| {
                                                dom.child(html!("button", {
                                                    //.attr("id", "addProgressNoteColumnInputHeaderLink")
                                                    .visible_signal(used.signal_cloned().map(|used| used != "Y"))
                                                    .attr("type", "button")
                                                    .class(class::BTN_FR_R)
                                                    .class_signal("btn-primary", not(page.show_progress_note_input.signal()))
                                                    .class_signal("btn-secondary", page.show_progress_note_input.signal())
                                                    .text_signal(page.show_progress_note_input.signal_cloned().map(|show| {
                                                        if show {"Cancel"} else {"+Add"}
                                                    }))
                                                    .event(clone!(page => move |_: events::Click| {
                                                        page.edit_progress_note.set(None);
                                                        page.show_progress_note_input.set(!page.show_progress_note_input.get());
                                                    }))
                                                }))
                                            })
                                            .child(html!("div",{.class("mt-2").text("Progress Note")}))
                                        }),
                                        html!("th", {
                                            .attr("scope", "col")
                                            .class(class::TXT_C_TOP)
                                            .style("width","35%")
                                            //.attr("id", "one-day-column-header")
                                            .apply_if(allow_order_form && allow_order_add, |dom| { dom
                                                .child(html!("button", {
                                                    //.attr("id", "addOneDayColumnInputHeaderLink")
                                                    .visible_signal(used.signal_cloned().map(|used| used != "Y"))
                                                    .attr("type", "button")
                                                    .class(class::BTN_FR_R)
                                                    .class_signal("btn-primary", not(page.show_oneday_input.signal()))
                                                    .class_signal("btn-secondary", page.show_oneday_input.signal())
                                                    .text_signal(page.show_oneday_input.signal_cloned().map(|show| {
                                                        if show {"Cancel"} else {"+Add"}
                                                    }))
                                                    .event(clone!(page => move |_: events::Click| {
                                                        page.edit_order.set(None);
                                                        page.show_continuous_input.set_neq(false);
                                                        page.show_oneday_input.set(!page.show_oneday_input.get());
                                                    }))
                                                }))
                                            })
                                            .child(html!("div",{.class("mt-2").text("One Day Order")}))
                                        }),
                                        html!("th", {
                                            .attr("scope", "col")
                                            .class(class::TXT_C_TOP)
                                            .style("width","35%")
                                            //.attr("id", "continuous-column-header")
                                            .apply_if(allow_order_form && allow_order_add, |dom| { dom
                                                .child(html!("button", {
                                                    //.attr("id", "addContinuousColumnInputHeaderLink")
                                                    .visible_signal(used.signal_cloned().map(|used| used != "Y"))
                                                    .attr("type", "button")
                                                    .class(class::BTN_FR_R)
                                                    .class_signal("btn-primary", not(page.show_continuous_input.signal()))
                                                    .class_signal("btn-secondary", page.show_continuous_input.signal())
                                                    .text_signal(page.show_continuous_input.signal_cloned().map(|show| {
                                                        if show {"Cancel"} else {"+Add"}
                                                    }))
                                                    .event(clone!(page => move |_: events::Click| {
                                                        page.edit_order.set(None);
                                                        page.show_oneday_input.set_neq(false);
                                                        page.show_continuous_input.set(!page.show_continuous_input.get());
                                                    }))
                                                }))
                                            })
                                            .child(html!("div",{.class("mt-2").text("Continuous Order")}))
                                        }),
                                    ])
                                }))
                            }),
                            html!("tbody", {
                                .child(html!("tr", {
                                    .class("order-table-row")
                                    .children([
                                        // Progress Note
                                        html!("td", {
                                            // ipd-dr-pre-order-progress-note-data.php
                                            .class("bg-secondary-subtle")
                                            .children_signal_vec(page.progress_note.signal_vec_cloned().map(clone!(app, page, used => move |progress_note| {
                                                render_progress_note(progress_note, Some(page.clone()), used.clone(), app.clone())
                                            })))
                                            .child(html!("div", {
                                                .class("text-end")
                                                .child(html!("div", {
                                                    //.attr("id", "progress-note-column-add-link")
                                                    .visible_signal(used.signal_cloned().map(|used| used != "Y"))
                                                    .class("text-end")
                                                    .apply_if(allow_progress_form && allow_progress_add, |dom| { dom
                                                        .child(html!("button", {
                                                            .attr("type", "button")
                                                            .class(class::BTN_R)
                                                            .class_signal("btn-primary", not(page.show_progress_note_input.signal()))
                                                            .class_signal("btn-secondary", page.show_progress_note_input.signal())
                                                            .text_signal(page.show_progress_note_input.signal_cloned().map(|show| {
                                                                if show {"Cancel"} else {"+Add"}
                                                            }))
                                                            .event(clone!(page => move |_: events::Click| {
                                                                page.edit_progress_note.set(None);
                                                                page.show_progress_note_input.set(!page.show_progress_note_input.get());
                                                            }))
                                                        }))
                                                    })
                                                }))
                                            }))
                                            .child_signal(page.show_progress_note_input.signal_cloned().map(clone!(app, page => move |show| {
                                                show.then(|| {
                                                    let form = ProgressNoteForm::new(
                                                        false,
                                                        page.edit_progress_note.get_cloned(),
                                                        Mutable::new(None),
                                                        Mutable::new(String::from("doctor")),
                                                        Mutable::new(None),
                                                        zero_none(page.pre_order_master_id.get()),
                                                        app.user.lock_ref().as_ref().map(|u|u.user.doctorcode.get_cloned()).unwrap_or_default(),
                                                    );
                                                    ProgressNoteForm::render(
                                                        form,
                                                        page.show_progress_note_input.clone(),
                                                        Mutable::new(false),
                                                        page.edit_progress_note.clone(),
                                                        page.reload_progress_note.clone(),
                                                        app.clone(),
                                                    )
                                                })
                                            })))
                                        }),
                                        // One Day Order
                                        html!("td", {
                                            .children_signal_vec(page.oneday.signal_vec_cloned().map(clone!(app, page, used => move |order| {
                                                render_order(order, true, used.clone(), Some(page.clone()), app.clone())
                                            })))
                                            .child(html!("div", {
                                                .class("text-end")
                                                .child(html!("div", {
                                                    //.attr("id", "one-day-column-add-link")
                                                    .visible_signal(used.signal_cloned().map(|used| used != "Y"))
                                                    .class("text-end")
                                                    .apply_if(allow_order_form && allow_order_add, |dom| { dom
                                                        .child(html!("button", {
                                                            .attr("type", "button")
                                                            .class(class::BTN_R)
                                                            .class_signal("btn-primary", not(page.show_oneday_input.signal()))
                                                            .class_signal("btn-secondary", page.show_oneday_input.signal())
                                                            .text_signal(page.show_oneday_input.signal_cloned().map(|show| {
                                                                if show {"Cancel"} else {"+Add"}
                                                            }))
                                                            .event(clone!(page => move |_: events::Click| {
                                                                page.edit_order.set(None);
                                                                page.show_continuous_input.set_neq(false);
                                                                page.show_oneday_input.set(!page.show_oneday_input.get());
                                                            }))
                                                        }))
                                                    })
                                                }))
                                            }))
                                            .child_signal(page.show_oneday_input.signal_cloned().map(clone!(app, page => move |show| {
                                                show.then(|| {
                                                    let form = OneDayForm::new(
                                                        page.edit_order.get_cloned(),
                                                        Mutable::new(None),
                                                        zero_none(page.pre_order_master_id.get()),
                                                        app.user.lock_ref().as_ref().map(|u|u.user.doctorcode.get_cloned()).unwrap_or_default(),
                                                        Mutable::new(String::from("doctor")),
                                                        MutableVec::new(),
                                                    );
                                                    OneDayForm::render(
                                                        form,
                                                        page.show_oneday_input.clone(),
                                                        page.edit_order.clone(),
                                                        page.reload_order_oneday.clone(),
                                                        app.clone(),
                                                    )
                                                })
                                            })))
                                        }),
                                        // Continuous Order
                                        html!("td", {
                                            // ipd-dr-pre-order-continuous-data.php
                                            .children_signal_vec(page.continuous.signal_vec_cloned().map(clone!(app, page, used => move |order| {
                                                render_order(order, false, used.clone(), Some(page.clone()), app.clone())
                                            })))
                                            .child(html!("div", {
                                                .class("text-end")
                                                .child(html!("div", {
                                                    //.attr("id", "continuous-column-add-link")
                                                    .visible_signal(used.signal_cloned().map(|used| used != "Y"))
                                                    .class("text-end")
                                                    .apply_if(allow_order_form && allow_order_add, |dom| { dom
                                                        .child(html!("button", {
                                                            .attr("type", "button")
                                                            .class(class::BTN_R)
                                                            .class_signal("btn-primary", not(page.show_continuous_input.signal()))
                                                            .class_signal("btn-secondary", page.show_continuous_input.signal())
                                                            .text_signal(page.show_continuous_input.signal_cloned().map(|show| {
                                                                if show {"Cancel"} else {"+Add"}
                                                            }))
                                                            .event(clone!(page => move |_: events::Click| {
                                                                page.edit_order.set(None);
                                                                page.show_oneday_input.set_neq(false);
                                                                page.show_continuous_input.set(!page.show_continuous_input.get());
                                                            }))
                                                        }))
                                                    })
                                                }))
                                            }))
                                            .child_signal(page.show_continuous_input.signal_cloned().map(clone!(app, page => move |show| {
                                                show.then(|| {
                                                    let form = ContinuousForm::new(
                                                        page.edit_order.get_cloned(),
                                                        Mutable::new(None),
                                                        zero_none(page.pre_order_master_id.get()),
                                                        app.user.lock_ref().as_ref().map(|u|u.user.doctorcode.get_cloned()).unwrap_or_default(),
                                                        Mutable::new(String::from("doctor")),
                                                        MutableVec::new(),
                                                    );
                                                    ContinuousForm::render(
                                                        form,
                                                        page.show_continuous_input.clone(),
                                                        page.edit_order.clone(),
                                                        page.reload_order_continuous.clone(),
                                                        app.clone(),
                                                    )
                                                })
                                            })))
                                        }),
                                    ])
                                }))
                            }),
                        ])
                    }))
                }))
            }))
        })
    }
}

// function oneday_data_to_text(one_day_order)
// function continuous_data_to_text(continuous_order)
pub fn render_order(order: Rc<PreOrder>, is_oneday: bool, used: Mutable<String>, page_opt: Option<Rc<IpdPreOrderCpn>>, app: Rc<App>) -> Dom {
    let will_blue = if is_oneday {
        vec!["med", "home-medication", "injection", "ivfluid"]
    } else {
        vec!["med", "injection", "ivfluid"]
    };
    let is_same_doctor = app
        .user
        .lock_ref()
        .as_ref()
        .map(|user| user.user.doctorcode.lock_ref().deref() == &order.order_doctor)
        .unwrap_or_default();

    let allow_order_form = app.endpoint_is_allow(&Method::POST, &EndPoint::IpdPreOrderOrder, true) && app.has_permission(Permission::DataTypeDoctorUse);
    let allow_order_edit = app.has_permission(Permission::IpdOrderEdit) || app.has_permission(Permission::OpdErOrderEdit);

    html!("div", {
        .apply_if(page_opt.is_some(), clone!(page_opt, order => move |dom| {
            match page_opt {
                Some(page) => {
                    dom.visible_signal(not(page.edit_order.signal_cloned().map(clone!(order => move |opt| opt.as_ref().map(|edit| edit.order_id == order.order_id).unwrap_or_default()))))
                }
                None => {
                    dom
                }
            }
        }))
        .attr("id", &["order_id_", &order.order_id.to_string(), "_div"].concat())
        .class(class::BOX_ROUND_T)
        .children([
            html!("span", {
                .text(&[date_th(&order.order_date), time_hm(&order.order_time)].join(" "))
            }),
            html!("div", {
                .attr("id", &["order_id_", &order.order_id.to_string(), "_inner_div"].concat())
                .class(class::BORDER_T_Y)
                .children({
                    let mut children = Vec::new();
                    order.order_item_types.iter().for_each(|order_item_type| {
                        if is_oneday && matches!(order_item_type.order_item_type, OrderTypeName::HomeMedication) {
                            children.push(html!("div", {
                                .class("fw-bold")
                                .text(order_item_type.order_item_type.string())
                            }))
                        }
                        let lis = order_item_type.order_items.iter().map(|order_item| {
                            html!("li", {
                                .class("clearfix")
                                .apply_if(order_item.order_item_type == Some(String::from("off")), |dom| {
                                    dom.child(html!("span", {.class("fw-bold").text("Off\u{00a0}")
                                        .apply_if(order_item.off_icode.is_some(), |d| {
                                            let has_detail = if order_item.off_order_item_detail.is_some() {"\n"} else {""};
                                            d.text(&[&order_item.off_med_name.clone().unwrap_or_default(), has_detail].concat())
                                        })
                                    }))
                                    .text(&order_item.off_order_item_detail.clone().unwrap_or_default())
                                })
                                .apply_if(order_item.order_item_type != Some(String::from("off")), |dom| {
                                    dom.child(html!("span", {
                                        .apply_if(order_item.icode.is_some(), |d| {
                                            let has_detail = if order_item.order_item_detail.is_some() {"\n"} else {""};
                                            d.text(&[&order_item.med_name.clone().unwrap_or_default(), has_detail].concat())
                                        })
                                        .text(&order_item.order_item_detail.clone().unwrap_or_default())
                                        .apply_if(order_item.order_item_type.as_ref().map(|ty| will_blue.contains(&ty.as_str())).unwrap_or_default(), |d| d.class(class::BOLD_BLUE_EM_L))
                                        // .apply_if(order_item.off_by_datetime.is_some(), |d| d.style("text-decoration","line-through"))
                                    }))
                                    .apply_if(order_item.stat == Some(String::from("Y")), |dom| dom.child(html!("span", {
                                        .class(class::BADGE_WRAP_R_RED)
                                        .style("cursor","default")
                                        .text("STAT")
                                    })))
                                    .apply_if(order_item.allergy_agent_symptom.is_some(), |dom| dom.children([
                                        html!("br"),
                                        html!("small", {
                                            .class(class::BOLD_RED_L)
                                            .attr("role","button")
                                            .attr("title","แพ้ยา/เฝ้าระวัง")
                                            .text(&order_item.allergy_agent_symptom.clone().unwrap_or_default())
                                        })
                                    ]))
                                })
                            })
                        }).collect::<Vec<Dom>>();
                        let ul = html!("ul", {
                            .class("dash")
                            .style("white-space","pre-wrap")
                            .children(lis)
                        });
                        children.push(ul);
                    });
                    children
                })
            }),
            html!("div", {
                .class(class::SMALL_R)
                .apply_if(order.order_doctor_is_intern.unwrap_or_default(), |dom| dom.child(html!("span", {.text("(Intern) ")})))
                .children([
                    html!("span", {.text(&[&order.order_doctor_name.clone().unwrap_or_default(), ", "].concat())}),
                    html!("span", {
                        .class("text-nowrap")
                        .text(&[date_th(&order.order_date), time_hm(&order.order_time)].join(" "))
                    })
                ])
            }),
            html!("div", {
                .apply_if(order.nurse_accept_time.is_some(), |dom| dom.class(class::SMALL_R).children([
                    html!("span", {.text(&["(RN) ", &order.nurse_accept_name.clone().unwrap_or_default(), ", "].concat())}),
                    html!("span", {
                        .class("text-nowrap")
                        .text(&datetime_th_opt(&order.nurse_accept_time))
                    })
                ]))
            }),
            html!("div", {
                .apply_if(order.pharmacist_done_time.is_some(), |dom| dom.class(class::SMALL_R).children([
                    html!("span", {.text(&["(RX) ", &order.pharmacist_done_name.clone().unwrap_or_default(), ", "].concat())}),
                    html!("span", {
                        .class("text-nowrap")
                        .text(&datetime_th_opt(&order.pharmacist_done_time))
                    })
                ]))
                .apply_if(order.pharmacist_done_time.is_none() && order.pharmacist_check_time.is_some(), |dom| dom.class(class::SMALL_R).children([
                    html!("span", {.text(&["(ห้องยาตรวจสอบรายการ) ", &order.pharmacist_check_name.clone().unwrap_or_default(), ", "].concat())}),
                    html!("span", {
                        .class("text-nowrap")
                        .text(&datetime_th_opt(&order.pharmacist_check_time))
                    })
                ]))
                .apply_if(order.pharmacist_done_time.is_none() && order.pharmacist_check_time.is_none() && order.pharmacist_accept_time.is_some(), |dom| dom.class(class::SMALL_R).children([
                    html!("span", {.text(&["(ห้องยารับรายการ) ", &order.pharmacist_accept_name.clone().unwrap_or_default(), ", "].concat())}),
                    html!("span", {
                        .class("text-nowrap")
                        .text(&datetime_th_opt(&order.pharmacist_accept_time))
                    })
                ]))
            }),
            html!("div", {
                .apply_if(is_same_doctor && page_opt.is_some() && order.order_owner_type == *"doctor", |dom| {
                    match page_opt {
                        Some(page) => dom.attr("id",&["order_id_", &order.order_id.to_string(), "_action_row_div"].concat())
                            .visible_signal(used.signal_cloned().map(|used| used != "Y"))
                            .class(class::BOLD_R)
                            .apply_if(allow_order_form && allow_order_edit, |d| { d
                                .child(html!("button", {
                                    .attr("type", "button")
                                    .class(class::BTN_SM_RB_GOLD)
                                    .text("Edit")
                                    .event(clone!(app, page, order => move |_: events::Click| {
                                        page.edit_order.set(Some(Rc::new(Order::from(order.clone()))));
                                        if is_oneday {
                                            page.show_oneday_input.set(false);
                                            page.show_oneday_input.set(true);
                                        } else {
                                            page.show_continuous_input.set(false);
                                            page.show_continuous_input.set(true);
                                        }
                                        app.scroll_into_view(&["order_id_", &order.order_id.to_string(), "_div"].concat());
                                    }))
                                }))
                            })
                            .apply_if(app.endpoint_is_allow(&Method::DELETE, &EndPoint::IpdPreOrderOrderId, false) && app.has_permission(Permission::DataTypeDoctorUse), |d| {
                                d.child(html!("button" => HtmlButtonElement, {
                                    .attr("type", "button")
                                    .class(class::BTN_SM_RB_RED)
                                    .text("Delete")
                                    .apply(mixins::click_with_loader_checked(clone!(app, page, order => move || {
                                        IpdPreOrderCpn::delete_order(order.clone(), is_oneday, page.clone(), app.clone());
                                    }), app.state()))
                                }))
                            }),
                        None => dom,
                    }
                })
            }),
        ])
    })
}

// function progress_note_data_to_text(progress_note){
pub fn render_progress_note(progress_note: Rc<PreProgressNote>, page_opt: Option<Rc<IpdPreOrderCpn>>, used: Mutable<String>, app: Rc<App>) -> Dom {
    let display_datetime = [date_th(&progress_note.progress_note_date), time_hm(&progress_note.progress_note_time)].join(" ");
    let progress_note_owner_type = match progress_note.progress_note_owner_type.as_str() {
        "doctor" => " (Doctor)",
        "nurse" => " (Nurse)",
        "pharmacist" => " (Pharmacist)",
        "other" => " (Other)",
        "auditor" => " (Auditor)",
        _ => "",
    };
    let is_same_doctor = app
        .user
        .lock_ref()
        .as_ref()
        .map(|user| user.user.doctorcode.lock_ref().deref() == &progress_note.progress_note_doctor)
        .unwrap_or_default();

    let allow_progress_form = app.endpoint_is_allow(&Method::POST, &EndPoint::IpdPreOrderProgressNote, true) && app.has_permission(Permission::DataTypeDoctorUse);
    let allow_progress_edit = app.has_permission(Permission::ProgressNoteEdit);

    html!("div", {
        .apply_if(page_opt.is_some(), clone!(page_opt, progress_note => move |dom| {
            match page_opt {
                Some(page) => {
                    dom.visible_signal(not(page.edit_progress_note.signal_cloned().map(clone!(progress_note => move |opt| opt.as_ref().map(|edit| edit.progress_note_id == progress_note.progress_note_id).unwrap_or_default()))))
                }
                None => {
                    dom
                }
            }
        }))
        .attr("id", &["progress_note_id_", &progress_note.progress_note_id.to_string(), "_div"].concat())
        .class(class::BOX_ROUND_T)
        .child(html!("span", {
            .text(&[&display_datetime, progress_note_owner_type].concat())
        }))
        .child(html!("div", {
            .attr("id", &["progress_note_id_", &progress_note.progress_note_id.to_string(), "_inner_div"].concat())
            .class(class::BORDER_T_Y)
            .children(progress_note.progress_note_item_types.iter().map(|item_type| {
                html!("div", {
                    .children([
                        html!("span", {
                            .class("fw-bold")
                            .text(item_type.progress_note_item_type.string())
                        }),
                        html!("ul", {
                            .class("dash")
                            .style("white-space","pre-wrap")
                            .children(item_type.progress_note_items.iter().filter_map(|note| {
                                note.progress_note_item_detail.as_ref().map(|detail| html!("li", {.text(detail)}))
                            }))
                        })
                    ])
                })
            }))
        }))
        .child(html!("div", {
            .class(class::SMALL_R)
            .apply_if(progress_note.order_doctor_is_intern.unwrap_or_default(), |dom| dom.child(html!("span", {.text("(Intern) ")})))
            .children([
                html!("span", {
                    .text(&[&progress_note.order_doctor_name.clone().unwrap_or_default(), ", "].concat())
                }),
                html!("span", {
                    .text(&[&progress_note.entryposition.clone().unwrap_or_default(), ", "].concat())
                }),
                html!("span", {
                    .class("text-nowrap")
                    .text(&display_datetime)
                })
            ])
        }))
        .apply_if(is_same_doctor && page_opt.is_some() && progress_note.progress_note_owner_type == *"doctor", |dom| {
            match page_opt {
                Some(page) => dom.child(html!("div", {
                    .attr("id", &["progress_note_id_", &progress_note.progress_note_id.to_string(), "_action_row_div"].concat())
                    .visible_signal(used.signal_cloned().map(|used| used != "Y"))
                    .class(class::BOLD_R)
                    .apply_if(allow_progress_form && allow_progress_edit, |d| {
                        d.child(html!("button" => HtmlButtonElement, {
                            .attr("type", "button")
                            .class(class::BTN_SM_RB_GOLD)
                            .text("Edit")
                            .event(clone!(app, page, progress_note => move |_: events::Click| {
                                page.edit_progress_note.set(Some(Rc::new(ProgressNote::from(progress_note.clone()))));
                                page.show_progress_note_input.set(false);
                                page.show_progress_note_input.set(true);
                                app.scroll_into_view(&["progress_note_id_", &progress_note.progress_note_id.to_string(), "_div"].concat());
                            }))
                        }))
                    })
                    .apply_if(app.endpoint_is_allow(&Method::DELETE, &EndPoint::IpdPreOrderProgressNoteId, false) && app.has_permission(Permission::DataTypeDoctorUse), |d| {
                        d.child(html!("button" => HtmlButtonElement, {
                            .attr("type", "button")
                            .class(class::BTN_SM_RB_RED)
                            .text("Delete")
                            .apply(mixins::click_with_loader_checked(clone!(app, page, progress_note => move || {
                                IpdPreOrderCpn::delete_progress_note(progress_note.clone(), page.clone(), app.clone());
                            }), app.state()))
                        }))
                    })
                })),
                None => dom,
            }
        })
    })
}
