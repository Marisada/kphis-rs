use dominator::{Dom, clone, events, html};
use futures_signals::{
    map_ref,
    signal::{Mutable, SignalExt, and, not, or},
    signal_vec::{MutableVec, SignalVecExt},
};
use std::rc::Rc;
use web_sys::{HtmlButtonElement, HtmlInputElement, HtmlTextAreaElement};

use kphis_model::{
    app::VisitTypeId,
    endpoint::EndPoint,
    fetch::Method,
    order::{Order, OrderButtons, OrderSave, OrderTypeName},
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
    gadget::searchbox::med::{MedSearchboxCpn, search_drugusage},
    order::{InsertTextAreaButton, MedSearchable, OrderItemMutable},
};

/// - POST `EndPoint::IpdPreOrderOrder`
/// - POST `EndPoint::IpdOrderOrder`
/// - POST `EndPoint::OpdErOrderOrder`
/// - GET `EndPoint::SearchBoxMedHnText` (MedSearchboxCpn, guarded, remove med-search btn)
/// - GET `EndPoint::SearchBoxMedDuplicate` (MedSearchboxCpn, guarded, remove med-search btn)
/// - GET `EndPoint::SearchBoxMedInteraction` (MedSearchboxCpn, guarded, remove med-search btn)
#[derive(Default)]
pub struct ContinuousForm {
    view_by: Mutable<String>,
    patient: Mutable<Option<Rc<PatientInfo>>>,
    pre_order_master_id: Mutable<Option<u32>>,

    buttons_loaded: Mutable<bool>,
    to_scroll: Mutable<bool>,
    note_buttons: MutableVec<Rc<InsertTextAreaButton>>,
    food_buttons: MutableVec<Rc<InsertTextAreaButton>>,
    activity_buttons: MutableVec<Rc<InsertTextAreaButton>>,
    serial_buttons: MutableVec<Rc<InsertTextAreaButton>>,
    record_buttons: MutableVec<Rc<InsertTextAreaButton>>,
    other_buttons: MutableVec<Rc<InsertTextAreaButton>>,

    order_id: Mutable<Option<u32>>,
    order_doctor: Mutable<String>,

    changed: Mutable<bool>,
    focused: Mutable<Option<u32>>,

    notes: MutableVec<Rc<OrderItemMutable>>,
    offs: MutableVec<Rc<OrderItemMutable>>,
    offs_by_parent: MutableVec<Rc<OrderItemMutable>>,
    foods: MutableVec<Rc<OrderItemMutable>>,
    activities: MutableVec<Rc<OrderItemMutable>>,
    serials: MutableVec<Rc<OrderItemMutable>>,
    records: MutableVec<Rc<OrderItemMutable>>,
    meds: MutableVec<Rc<OrderItemMutable>>,
    others: MutableVec<Rc<OrderItemMutable>>,

    display_med_searchbox: Mutable<bool>,
    display_homemed_searchbox: Mutable<bool>,
    display_ivfluid_searchbox: Mutable<bool>,
}

impl MedSearchable for ContinuousForm {
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
        MutableVec::new()
    }
    fn meds(&self) -> MutableVec<Rc<OrderItemMutable>> {
        self.meds.clone()
    }
    fn homemeds(&self) -> MutableVec<Rc<OrderItemMutable>> {
        MutableVec::new()
    }
    fn offs(&self) -> MutableVec<Rc<OrderItemMutable>> {
        self.offs.clone()
    }
}

impl ContinuousForm {
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
            order_form.order_id.set(zero_none(order.order_id));
            for order_item_type in order.order_item_types.clone() {
                match order_item_type.order_item_type {
                    OrderTypeName::Note => order_form.notes.lock_mut().extend(order_item_type.order_items.into_iter().map(|i| Rc::new(i.into()))),
                    OrderTypeName::Off => order_form.offs.lock_mut().extend(order_item_type.order_items.into_iter().map(|i| Rc::new(i.into()))),
                    OrderTypeName::Food => order_form.foods.lock_mut().extend(order_item_type.order_items.into_iter().map(|i| Rc::new(i.into()))),
                    OrderTypeName::Activity => order_form.activities.lock_mut().extend(order_item_type.order_items.into_iter().map(|i| Rc::new(i.into()))),
                    OrderTypeName::Serial => order_form.serials.lock_mut().extend(order_item_type.order_items.into_iter().map(|i| Rc::new(i.into()))),
                    OrderTypeName::Record => order_form.records.lock_mut().extend(order_item_type.order_items.into_iter().map(|i| Rc::new(i.into()))),
                    OrderTypeName::Injection | OrderTypeName::Med => order_form.meds.lock_mut().extend(order_item_type.order_items.into_iter().map(|i| Rc::new(i.into()))),
                    // OrderTypeName::Med => order_form.meds.lock_mut().extend(order_item_type.order_items.into_iter().map(|i| Rc::new(i.into()))),
                    OrderTypeName::Other => order_form.others.lock_mut().extend(order_item_type.order_items.into_iter().map(|i| Rc::new(i.into()))),
                    OrderTypeName::Lab | OrderTypeName::Xray | OrderTypeName::Ivfluid | OrderTypeName::Retain | OrderTypeName::Pharm | OrderTypeName::Discharge | OrderTypeName::HomeMedication => {
                        order_form.others.lock_mut().extend(order_item_type.order_items.into_iter().map(|i| Rc::new(i.into())))
                    }
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

    fn load_buttons(page: Rc<Self>, app: Rc<App>) {
        app.async_load(
            true,
            clone!(app, page => async move {
                match OrderButtons::get("continuous", app.state()).await {
                    Ok(buttons) => for button in buttons {
                        match button.word_type.as_str() {
                            "note" => page.note_buttons.lock_mut().extend(button.buttons.into_iter().map(|button| Rc::new(InsertTextAreaButton::from_button("note", button)))),
                            "food" => page.food_buttons.lock_mut().extend(button.buttons.into_iter().map(|button| Rc::new(InsertTextAreaButton::from_button("food", button)))),
                            "activity" => page.activity_buttons.lock_mut().extend(button.buttons.into_iter().map(|button| Rc::new(InsertTextAreaButton::from_button("activity", button)))),
                            "serial" => page.serial_buttons.lock_mut().extend(button.buttons.into_iter().map(|button| Rc::new(InsertTextAreaButton::from_button("serial", button)))),
                            "record" => page.record_buttons.lock_mut().extend(button.buttons.into_iter().map(|button| Rc::new(InsertTextAreaButton::from_button("record", button)))),
                            "other" => page.other_buttons.lock_mut().extend(button.buttons.into_iter().map(|button| Rc::new(InsertTextAreaButton::from_button("other", button)))),
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

    pub fn render(page: Rc<Self>, parent_show_continuous_input: Mutable<bool>, parent_edit_order: Mutable<Option<Rc<Order>>>, parent_reload_order_continuous: Mutable<bool>, app: Rc<App>) -> Dom {
        let injections = app.app_status.lock_ref().as_ref().map(|status| status.hosxp_injection_dosageforms.clone()).unwrap_or_default();
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
                    app.scroll_into_view("addContinuousFormContainer");
                    page.to_scroll.set(false);
                }
                async {}
            })))
            .attr("id", "addContinuousFormContainer")
            .child(html!("div", {
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
                                    //.attr("id", "continuousForm-off-input-div")
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
                                //.attr("id", "food")
                                .class("mb-1")
                                .children([
                                    html!("span", {
                                        .class("fw-bold")
                                        //.attr("id", "continuousForm-food-label")
                                        .text("Food")
                                    }),
                                    html!("button", {
                                        .attr("type", "button")
                                        .class(class::BTN_SM_RT_GRAY)
                                        .child(html!("i", {.class(class::FA_PLUS)}))
                                        .event(clone!(page => move |_: events::Click| {
                                            let item = OrderItemMutable::new("food", page.pre_order_master_id.get());
                                            page.focused.set_neq(Some(item.id));
                                            page.foods.lock_mut().push_cloned(item);
                                        }))
                                    }),
                                ])
                                .children([
                                    html!("div", {
                                        //.attr("id", "continuousForm-food-input-div")
                                        .children_signal_vec(page.foods.signal_vec_cloned().map(clone!(page => move |food| {
                                            html!("div", {
                                                .child(html!("div", {
                                                    .class(class::INPUT_GROUP_TR)
                                                    .children([
                                                        html!("textarea" => HtmlTextAreaElement, {
                                                            .class(class::FORM_CTRL_SM)
                                                            .attr("id", &["textarea-", &food.id.to_string()].concat())
                                                            .apply(mixins::textarea_value_auto_expand(food.order_item_detail.clone(), page.changed.clone()))
                                                            .event(clone!(page, food => move |_: events::Focus| {
                                                                page.focused.set_neq(Some(food.id));
                                                            }))
                                                        }),
                                                        html!("button", {
                                                            .class(class::BTN_SM_RED)
                                                            .attr("type", "button")
                                                            .child(html!("i", {.class(class::FA_MINUS)}))
                                                            .event(clone!(page => move |_: events::Click| {
                                                                page.foods.lock_mut().retain(|x| *x != food);
                                                                page.changed.set_neq(true);
                                                            }))
                                                        }),
                                                    ])
                                                }))
                                            })
                                        })))
                                    }),
                                    html!("div", {
                                        //.attr("id", "food_option")
                                        .children_signal_vec(page.food_buttons.signal_vec_cloned()
                                            .map(clone!(app, page => move |btn| InsertTextAreaButton::render(btn, page.foods.clone(), page.focused.clone(), page.changed.clone(), app.clone()))
                                        ))
                                    }),
                                ])
                            }),
                            html!("div", {
                                //.attr("id", "activity")
                                .class("mb-1")
                                .children([
                                    html!("span", {
                                        .class("fw-bold")
                                        //.attr("id", "continuousForm-activitiy-label")
                                        .text("Activity")
                                    }),
                                    html!("button", {
                                        .attr("type", "button")
                                        .class(class::BTN_SM_RT_GRAY)
                                        .child(html!("i", {.class(class::FA_PLUS)}))
                                        .event(clone!(page => move |_: events::Click| {
                                            let item = OrderItemMutable::new("activitiy", page.pre_order_master_id.get());
                                            page.focused.set_neq(Some(item.id));
                                            page.activities.lock_mut().push_cloned(item);
                                        }))
                                    }),
                                ])
                                .children([
                                    html!("div", {
                                        //.attr("id", "continuousForm-activitiy-input-div")
                                        .children_signal_vec(page.activities.signal_vec_cloned().map(clone!(page => move |activity| {
                                            html!("div", {
                                                .child(html!("div", {
                                                    .class(class::INPUT_GROUP_TR)
                                                    .children([
                                                        html!("textarea" => HtmlTextAreaElement, {
                                                            .class(class::FORM_CTRL_SM)
                                                            .attr("id", &["textarea-", &activity.id.to_string()].concat())
                                                            .apply(mixins::textarea_value_auto_expand(activity.order_item_detail.clone(), page.changed.clone()))
                                                            .event(clone!(page, activity => move |_: events::Focus| {
                                                                page.focused.set_neq(Some(activity.id));
                                                            }))
                                                        }),
                                                        html!("button", {
                                                            .class(class::BTN_SM_RED)
                                                            .attr("type", "button")
                                                            .child(html!("i", {.class(class::FA_MINUS)}))
                                                            .event(clone!(page => move |_: events::Click| {
                                                                page.activities.lock_mut().retain(|x| *x != activity);
                                                                page.changed.set_neq(true);
                                                            }))
                                                        }),
                                                    ])
                                                }))
                                            })
                                        })))
                                    }),
                                    html!("div", {
                                        //.attr("id", "activitiy_option")
                                        .children_signal_vec(page.activity_buttons.signal_vec_cloned()
                                            .map(clone!(app, page => move |btn| InsertTextAreaButton::render(btn, page.activities.clone(), page.focused.clone(), page.changed.clone(), app.clone()))
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
                                        //.attr("id", "continuousFormSerialLabel")
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
                                            //.attr("id", "continuousForm-serial-input-div")
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
                                        //.attr("id", "continuousForm-record-label")
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
                                        //.attr("id", "continuousForm-record-input-div")
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
                                    //.attr("id", "continuousFormMedLabel")
                                    .text("Medication")
                                }))
                                .apply_if(
                                    app.endpoint_is_allow(&Method::GET, &EndPoint::SearchBoxMedHnText , false)
                                    && app.endpoint_is_allow(&Method::GET, &EndPoint::SearchBoxMedDuplicate , false)
                                    && app.endpoint_is_allow(&Method::GET, &EndPoint::SearchBoxMedInteraction , false),
                                |dom| dom
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
                                            .child(MedSearchboxCpn::render(None, searchbox.clone(), page.clone(), app.clone()))
                                            .children(MedSearchboxCpn::render_modals(searchbox))
                                        })
                                    })
                                })))
                                .child(html!("div", {
                                    //.attr("id", "continuousForm-med-input-div")
                                    .children_signal_vec(page.meds.signal_vec_cloned().map(clone!(app, page, injections => move |med| {
                                        let is_injection = med.dosageform.lock_ref().as_ref().map(|dosageform| injections.contains(&dosageform)).unwrap_or_default();
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
                                //.attr("id", "other")
                                .class("mb-1")
                                .children([
                                    html!("span", {
                                        .class("fw-bold")
                                        //.attr("id", "continuousFormOtherLabel")
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
                                        //.attr("id", "continuousForm-other-input-div")
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
                        ])
                    }),
                    html!("button", {
                        .attr("type", "button")
                        .class(class::BTN_FR_GRAY)
                        .text("Cancel")
                        .event(clone!(page, parent_show_continuous_input, parent_edit_order => move |_: events::Click| {
                            parent_show_continuous_input.set_neq(false);
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
                            Self::submit(page.clone(), parent_show_continuous_input.clone(), parent_edit_order.clone(), parent_reload_order_continuous.clone(), app.clone());
                        }), and(not(page.changed.signal()), page.offs_by_parent.signal_vec_cloned().is_empty()), app.state()))
                    }),
                ])
            }))
        })
    }

    fn submit(page: Rc<Self>, parent_show_continuous_input: Mutable<bool>, parent_edit_order: Mutable<Option<Rc<Order>>>, parent_reload_order_continuous: Mutable<bool>, app: Rc<App>) {
        let injections = app.app_status.lock_ref().as_ref().map(|status| status.hosxp_injection_dosageforms.clone()).unwrap_or_default();
        let mut order_items = Vec::new();
        order_items.extend(page.notes.lock_ref().iter().filter_map(OrderItemMutable::save));
        order_items.extend(page.offs.lock_ref().iter().filter_map(OrderItemMutable::save));
        order_items.extend(page.offs_by_parent.lock_ref().iter().filter_map(OrderItemMutable::save));
        order_items.extend(page.foods.lock_ref().iter().filter_map(OrderItemMutable::save));
        order_items.extend(page.activities.lock_ref().iter().filter_map(OrderItemMutable::save));
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
        order_items.extend(page.others.lock_ref().iter().filter_map(OrderItemMutable::save));

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
                            order_type: String::from("continuous"),
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
                            order_type: String::from("continuous"),
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
                                    parent_show_continuous_input.set_neq(false);
                                    parent_edit_order.set(None);
                                    parent_reload_order_continuous.set(true);
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
