// ipd-dr-order.php's modal

use dominator::{Dom, clone, events, html, with_node};
use futures_signals::{
    map_ref,
    signal::{Mutable, Signal, SignalExt, always},
    signal_vec::{MutableVec, SignalVecExt},
};
use std::rc::Rc;
use web_sys::{HtmlInputElement, HtmlSelectElement};

use kphis_model::{
    pre_order::master::{PreOrderMaster, PreOrderMasterParams},
    timer::Timeout,
};
use kphis_ui_app::App;
use kphis_ui_core::{binding::NiceSelect, class, doms, mixins};
use kphis_util::{
    datetime::{date_8601, date_th_opt, datetime_th_opt},
    util::{pre_order_type_display, str_some, zero_none},
};

use crate::modal::pre_order_preview::{PreOrderPreview, ToOrderType};

/// - GET `EndPoint::IpdPreOrderMaster` (self/PreOrderPreview)
/// - GET `EndPoint::IpdPreOrderOrder` (PreOrderPreview)
/// - GET `EndPoint::IpdPreOrderProgressNote` (PreOrderPreview)
#[derive(Clone, Default)]
pub struct PreOrderSelect {
    preorder_type: Mutable<PreOrderType>,
    preorders: MutableVec<Rc<PreOrderMaster>>,

    loaded_list: Mutable<bool>,
    changed: Mutable<bool>,
    redraw: Mutable<bool>,

    caller_id: Mutable<String>, // an(order) OR pre_order_master_id(pre_order)
    hn: Mutable<String>,
    order_doctor: Mutable<String>,
    start_order_date: Mutable<String>,
    end_order_date: Mutable<String>,
    include_shared_template: Mutable<String>,
    template_name: Mutable<String>,
    used: Mutable<String>,

    pre_order_master_id: Mutable<Option<u32>>,
    to_order_type: ToOrderType,
}

impl PreOrderSelect {
    pub fn new(preorder_type: PreOrderType, caller_id: &str, to_order_type: ToOrderType) -> Rc<Self> {
        Rc::new(Self {
            caller_id: Mutable::new(String::from(caller_id)),
            preorder_type: Mutable::new(preorder_type),
            used: Mutable::new(String::from("N")),
            to_order_type,
            ..Default::default()
        })
    }

    pub fn is_template(&self) -> impl Signal<Item = bool> + use<> {
        self.preorder_type.signal_cloned().map(|ty| matches!(ty, PreOrderType::Template))
    }

    pub fn is_pre_order(&self) -> impl Signal<Item = bool> + use<> {
        self.preorder_type.signal_cloned().map(|ty| matches!(ty, PreOrderType::PreOrder(_)))
    }

    pub fn render(modal: Rc<Self>, display: Mutable<Option<Rc<Self>>>, parent_date_loaded: Option<Mutable<bool>>, parent_count_loaded: Option<Mutable<bool>>, app: Rc<App>) -> Dom {
        html!("div", {
            .class(class::MODAL_DIALOG_LG)
            .attr("role", "document")
            .child(html!("div", {
                .class("modal-content")
                .children([
                    html!("div", {
                        .class("modal-header")
                        .children([
                            html!("h5", {
                                .class("modal-title")
                                .text("เลือกรายการ")
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
                        //.attr("id", "selectPreOrderModalBody")
                        .child_signal(modal.pre_order_master_id.signal_cloned().map(clone!(app, modal, parent_date_loaded, parent_count_loaded => move |opt| {
                            match opt {
                                Some(pre_order_master_id) => Some(PreOrderPreview::render(
                                    PreOrderPreview::new(pre_order_master_id, modal.caller_id.clone(), modal.to_order_type.clone()),
                                    app.clone(),
                                    modal.redraw.clone(),
                                    modal.pre_order_master_id.clone(),
                                    parent_date_loaded.clone(),
                                    parent_count_loaded.clone(),
                                )),
                                // need cloned here because preorder_type will be &self after render(), cannot pass locked as &self
                                None => Some(modal.preorder_type.get_cloned().render(modal.clone(), app.clone())),
                            }
                        })))
                    }),
                    html!("div", {
                        .class("modal-footer")
                        .child(html!("button", {
                            .attr("type", "button")
                            .class(class::BTN_GRAY)
                            .attr("data-bs-dismiss", "modal")
                            .text("ปิด")
                            .event(clone!(display => move |_: events::Click| {
                                display.set(None);
                            }))
                        }))
                    }),
                ])
            }))
        })
    }
}

// ipd-dr-pre-order-list-searchbox.php
/// GET `EndPoint::IpdPreOrderMaster`
#[derive(Clone, Default)]
pub enum PreOrderType {
    #[default]
    Template,
    PreOrder(String),
}

impl PreOrderType {
    fn string(&self) -> String {
        match self {
            PreOrderType::Template => String::from("template"),
            PreOrderType::PreOrder(_hn) => String::from("pre_order"),
        }
    }

    fn load_list(modal: Rc<PreOrderSelect>, app: Rc<App>) {
        app.async_load(
            true,
            clone!(app => async move {

                let params = PreOrderMasterParams {
                    pre_order_master_id: None,
                    hn: str_some(modal.hn.get_cloned()),
                    start_order_date: date_8601(&modal.start_order_date.lock_ref()),
                    end_order_date: date_8601(&modal.end_order_date.lock_ref()),
                    order_doctor: str_some(modal.order_doctor.get_cloned()),
                    include_shared_template: str_some(modal.include_shared_template.get_cloned()),
                    pre_order_type: Some(modal.preorder_type.lock_ref().string()),
                    template_name: str_some(modal.template_name.get_cloned()),
                    used: str_some(modal.used.get_cloned()),
                };
                // GET `EndPoint::IpdPreOrderMaster`
                match PreOrderMaster::call_api_get(&params, app.state()).await {
                    Ok(response) => {
                        let mut lock = modal.preorders.lock_mut();
                        lock.clear();
                        if !response.is_empty() {
                            lock.extend(response.into_iter().map(Rc::new));
                        }
                    }
                    Err(e) => {
                        app.alert_app_error(&e).await;
                    }
                }
            }),
        )
    }

    fn render(&self, modal: Rc<PreOrderSelect>, app: Rc<App>) -> Dom {
        let (hn, order_doctor) = match modal.preorder_type.get_cloned() {
            PreOrderType::PreOrder(hn) => (Some(hn), None),
            PreOrderType::Template => (None, app.user.lock_ref().as_ref().map(|client| client.user.doctorcode.get_cloned())),
        };
        modal.hn.set_neq(hn.unwrap_or_default());
        modal.order_doctor.set_neq(order_doctor.unwrap_or_default());

        let all_doctor_select_option = match app.app_asset.lock_ref().as_ref() {
            Some(assets_arc) => {
                let asset = assets_arc.as_ref().to_owned();
                asset.all_doctor_select_option
            }
            None => Vec::new(),
        };

        html!("div", {
            .future(map_ref!(
                let busy = app.loader_is_loading(),
                let loaded = modal.loaded_list.signal() =>
                !busy && !loaded
            ).for_each(clone!(app, modal => move |ready| {
                if ready {
                    Self::load_list(modal.clone(), app.clone());
                    modal.loaded_list.set_neq(true);
                    modal.redraw.set_neq(true);
                }
                async {}
            })))
            .children([
                doms::alert_row(clone!(app, modal => move |alert| { alert
                    .children([
                        doms::form_inline(clone!(app, modal => move |form| { form
                            .children([
                                doms::form_inline_group_sm(clone!(modal => move |group| { group
                                    .children([
                                        doms::label_group_for("order_doctor","ผู้บันทึก"),
                                        html!("div", {
                                            .class(class::FLEX_GROW1)
                                            .child(html!("select" => HtmlSelectElement, {
                                                .class(class::FORM_CTRL_SM)
                                                .attr("id", "order_doctor")
                                                .child(html!("option", {
                                                    .attr("value", "")
                                                    .text("ทั้งหมด")
                                                }))
                                                .children(all_doctor_select_option.iter().map(|option| {
                                                    doms::select_option(option, &modal.order_doctor.lock_ref())
                                                }))
                                                .prop_signal("value", modal.order_doctor.signal_cloned())
                                                .with_node!(element => {
                                                    .event(clone!(element, modal => move |_: events::Change| {
                                                        modal.order_doctor.set_neq(element.value());
                                                        modal.changed.set_neq(true);
                                                        modal.loaded_list.set(false);
                                                    }))
                                                })
                                            }))
                                        }),
                                    ])
                                })),
                                doms::form_inline_group_sm(clone!(app, modal => move |group| { group
                                    .children([
                                        doms::label_group_for("hn","HN"),
                                        html!("input", {
                                            .attr("type", "text")
                                            .class(class::FORM_CTRL_SM)
                                            .attr("id", "hn")
                                            .attr("readonly", "readonly")
                                            .prop_signal("size", app.hosxp_hn_len_signal().map(|n| n.to_owned()))
                                            .prop_signal("value", modal.hn.signal_cloned())
                                        }),
                                    ])
                                })),
                                doms::form_inline_group_sm(clone!(modal => move |group| { group
                                    .children([
                                        doms::label_group_for("start_order_date","วันที่บันทึก"),
                                        doms::date_picker(
                                            modal.start_order_date.clone(),
                                            modal.changed.clone(), always(false), None,
                                            |d| d.class(class::FLEX_GROW1).style("min-width","120px"),
                                            |d| d.class(class::FORM_CTRL_ONLY_SM_R0),
                                            |d| d.class(class::FORM_CTRL_ONLY_SM_R0).attr("id", "start_order_date"),
                                            clone!(modal => move |s| {
                                                modal.loaded_list.set(false);
                                                s
                                            }), always(None),
                                        ),
                                        doms::label_group_for("end_order_date","ถึง"),
                                        doms::date_picker(
                                            modal.end_order_date.clone(),
                                            modal.changed.clone(), always(false), None,
                                            |d| d.class(class::FLEX_GROW1).style("min-width","120px"),
                                            |d| d.class(class::FORM_CTRL_ONLY_SM_R0_L),
                                            |d| d.class(class::FORM_CTRL_ONLY_SM_R0_L).attr("id", "end_order_date"),
                                            clone!(modal => move |s| {
                                                modal.loaded_list.set(false);
                                                s
                                            }), always(None),
                                        ),
                                    ])
                                })),
                            ])
                            .apply_if(matches!(modal.preorder_type.get_cloned(), PreOrderType::Template), |is_template| { is_template
                                .children([
                                    doms::form_inline_group_sm(clone!(modal => move |group| { group
                                        .children([
                                            doms::label_group_for("template_name","ชื่อ Template"),
                                            html!("input" => HtmlInputElement, {
                                                .attr("type", "text")
                                                .class(class::FORM_CTRL_SM)
                                                .attr("id", "template_name")
                                                .apply(mixins::string_value(modal.template_name.clone(), modal.changed.clone()))
                                                .event(clone!(modal => move |_: events::Change| {
                                                    modal.loaded_list.set(false);
                                                }))
                                            }),
                                        ])
                                    })),
                                    doms::form_inline_switch(clone!(modal => move |group| { group
                                        .children([
                                            html!("input" => HtmlInputElement, {
                                                .attr("type", "checkbox")
                                                .class("form-check-input")
                                                .attr("role","switch")
                                                .attr("id", "include_shared_template")
                                                .apply(mixins::checkbox_toggle(modal.include_shared_template.clone(), modal.changed.clone(), "Y", "N"))
                                                .event(clone!(modal => move |_: events::Change| {
                                                    modal.loaded_list.set(false);
                                                }))
                                            }),
                                            html!("label", {
                                                .attr("for", "include_shared_template")
                                                .class("form-check-lebel")
                                                .text("แสดง Shared Template")
                                            }),
                                        ])
                                    })),
                                ])
                            })
                            .apply_if(matches!(modal.preorder_type.get_cloned(), PreOrderType::PreOrder(_)), |is_pre_order| { is_pre_order
                                .child(doms::form_inline_group_sm(clone!(modal => move |group| { group
                                    .children([
                                        doms::label_group_for("used","ใช้งาน"),
                                        html!("select" => HtmlSelectElement, {
                                            .class(class::FORM_CTRL_SM)
                                            .attr("id", "used")
                                            .children([
                                                html!("option", {
                                                    .attr("value", "")
                                                    .text("ทั้งหมด")
                                                }),
                                                html!("option", {
                                                    .attr("value", "Y")
                                                    .text("ใช้แล้ว")
                                                }),
                                                html!("option", {
                                                    .attr("value", "N")
                                                    .text("ยังไม่ได้ใช้")
                                                }),
                                            ])
                                            .prop_signal("value", modal.used.signal_cloned())
                                            .with_node!(element => {
                                                .event(clone!(element, modal => move |_: events::Change| {
                                                    modal.used.set_neq(element.value());
                                                    modal.changed.set_neq(true);
                                                    modal.loaded_list.set(false);
                                                }))
                                            })
                                        }),
                                    ])
                                })))
                            })
                        })),
                    ])
                })),
                doms::table_responsive(class::TABLE_STRIP, clone!(modal => move |table| { table
                    .children([
                        html!("thead", {
                            .child(html!("tr", {
                                .children([
                                    // html!("th", {
                                    //     .attr("scope", "col")
                                    //     .visible(false)
                                    //     .text("#")
                                    // }),
                                    html!("th", {
                                        .attr("scope", "col")
                                        .text("เวลาที่บันทึก")
                                    }),
                                    html!("th", {
                                        .attr("scope", "col")
                                        .visible_signal(modal.is_pre_order())
                                        .text("ประเภทใบ Order")
                                    }),
                                    html!("th", {
                                        .attr("scope", "col")
                                        .text("ผู้บันทึก")
                                    }),
                                    html!("th", {
                                        .attr("scope", "col")
                                        .visible_signal(modal.is_pre_order())
                                        .text("HN")
                                    }),
                                    html!("th", {
                                        .attr("scope", "col")
                                        .visible_signal(modal.is_pre_order())
                                        .text("ชื่อ-นามสกุล")
                                    }),
                                    html!("th", {
                                        .attr("scope", "col")
                                        .visible_signal(modal.is_pre_order())
                                        .text("วันที่นัด/Admit")
                                    }),
                                    html!("th", {
                                        .attr("scope", "col")
                                        .visible_signal(modal.is_pre_order())
                                        .text("ใช้งาน")
                                    }),
                                    html!("th", {
                                        .attr("scope", "col")
                                        .visible_signal(modal.is_template())
                                        .text("ชื่อ Template")
                                    }),
                                ])
                            }))
                        }),
                        html!("tbody", {
                            //.attr("id", "pre_order_master_table_body")
                            .future(modal.redraw.signal_cloned().for_each(clone!(app, modal => move |redraw| {
                                if redraw {
                                    if let Some(elm) = app.get_id("order_doctor") {
                                        Timeout::new(0, clone!(modal => move || {
                                            NiceSelect::new_default(&elm);
                                            modal.redraw.set(false);
                                        })).forget();
                                    }
                                }
                                async {}
                            })))
                            .children_signal_vec(modal.preorders.signal_vec_cloned().map(clone!(modal => move |preorder| {
                                html!("tr", {
                                    .style("cursor","pointer")
                                    .children([
                                        // html!("td", {.visible(false)//.text(i+1)}),
                                        html!("td", {
                                            .text(&datetime_th_opt(&preorder.order_date_time))
                                        }),
                                        html!("td", {
                                            .visible_signal(modal.is_pre_order())
                                            .text(pre_order_type_display(&preorder.pre_order_type))
                                        }),
                                        html!("td", {
                                            .apply_if(preorder.order_doctor_is_intern.unwrap_or_default(), |dom| dom.text("(Intern) "))
                                            .text(&preorder.order_doctor_name.clone().unwrap_or_default())
                                        }),
                                        html!("td", {
                                            .visible_signal(modal.is_pre_order())
                                            .text(&preorder.hn.clone().unwrap_or_default())
                                        }),
                                        html!("td", {
                                            .visible_signal(modal.is_pre_order())
                                            .text(&preorder.fullname.clone().unwrap_or_default())
                                        }),
                                        html!("td", {
                                            .visible_signal(modal.is_pre_order())
                                            .text(&date_th_opt(&preorder.order_for_date))
                                        }),
                                        html!("td", {
                                            .visible_signal(modal.is_pre_order())
                                            .text(match preorder.used.clone().unwrap_or_default().as_ref() {
                                                "Y" => "ใช้แล้ว",
                                                _ => "ยังไม่ได้ใช้",
                                            })
                                        }),
                                        html!("td", {
                                            .visible_signal(modal.is_template())
                                            .apply_if(preorder.shared_template.clone().unwrap_or_default() == *"Y", |dom| {
                                                dom.child(html!("i", {.class(class::FA_STAR_L)}))
                                            })
                                            .child(html!("span", {.text(&preorder.template_name.clone().unwrap_or_default())}))
                                        }),
                                    ])
                                    .event(clone!(modal => move |_: events::Click| {
                                        modal.pre_order_master_id.set_neq(zero_none(preorder.pre_order_master_id));
                                    }))
                                    // click => window.location.href = './ipd-dr-pre-order-preview.php?pre_order_master_id='+v.pre_order_master_id;
                                })
                            })))
                        }),
                    ])
                })),
            ])
        })
    }
}
