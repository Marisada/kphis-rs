use dominator::{Dom, clone, events, html};
use futures_signals::{
    map_ref,
    signal::{Mutable, Signal, SignalExt, always},
    signal_vec::{MutableVec, SignalVecExt},
};
use std::rc::Rc;
use web_sys::{HtmlInputElement, HtmlSelectElement};

use kphis_model::report::{BasicType, ParamType, ReportParam};
use kphis_ui_app::App;
use kphis_ui_core::{class, doms, mixins};

#[derive(Clone)]
pub struct ReportParamInput {
    pub title: String,
    pub ty: ParamType,
    pub id: Mutable<String>,
    pub ids: MutableVec<Mutable<String>>,
}

impl ReportParamInput {
    pub fn new(param: &ReportParam) -> Rc<Self> {
        let ids = if param.ty.is_array() {
            MutableVec::new_with_values(vec![Mutable::new(String::new())])
        } else {
            MutableVec::new()
        };
        Rc::new(Self {
            title: param.title.to_owned(),
            ty: param.ty.to_owned(),
            id: Mutable::new(String::new()),
            ids,
        })
    }

    // value in array type separated by ','
    pub fn new_with_value(param: &ReportParam, value: &str) -> Rc<Self> {
        let values = value.split(',').map(|s| Mutable::new(s.trim().to_owned())).collect::<Vec<Mutable<String>>>();
        let (ids, id) = if param.ty.is_array() {
            (MutableVec::new_with_values(values), Mutable::new(String::new()))
        } else if let Some(first) = values.first() {
            (MutableVec::new(), first.clone())
        } else {
            (MutableVec::new(), Mutable::new(String::new()))
        };
        Rc::new(Self {
            title: param.title.to_owned(),
            ty: param.ty.to_owned(),
            id,
            ids,
        })
    }

    pub fn is_empty_signal(&self) -> impl Signal<Item = bool> + use<> {
        map_ref! {
            let id_is_empty = self.id.signal_ref(|s| s.is_empty()),
            let ids_is_empty = self.ids.signal_vec_cloned().is_empty() =>
            *id_is_empty && *ids_is_empty
        }
    }

    pub fn to_request_id(&self) -> String {
        if self.ids.lock_ref().is_empty() {
            self.id.get_cloned()
        } else {
            self.ids.lock_ref().iter().map(|m| m.get_cloned()).collect::<Vec<String>>().join(",")
        }
    }

    pub fn render(param: Rc<Self>, changed: Mutable<bool>, app: Rc<App>) -> Dom {
        match param.ty.clone() {
            ParamType::Basic(basic_type) => {
                html!("div", {
                    .class(class::INPUT_GROUP_SM_T)
                    .children([
                        doms::span_group_text(&param.title),
                        render_basic_type(&basic_type, param.id.clone(), changed),
                    ])
                })
            }
            ParamType::List(_, items) => {
                html!("div", {
                    .class(class::INPUT_GROUP_SM_T)
                    .children([
                        doms::span_group_text(&param.title),
                        html!("div", {
                            .class(class::FLEX_GROW1)
                            .child(html!("select" => HtmlSelectElement, {
                                .class(class::FORM_SELECT_SM)
                                .child(html!("option", {.attr("value", "").text("เลือก")}))
                                .children(items.iter().map(|option| {
                                    html!("option", {
                                        .attr("value", &option.key)
                                        .text(&option.label)
                                    })
                                }))
                                .apply(mixins::string_value_select(param.id.clone(), changed))
                            }))
                        }),
                    ])
                })
            }
            ParamType::ListSystem(system_list_type) => {
                html!("div", {
                    .class(class::INPUT_GROUP_SM_T)
                    .children([
                        doms::span_group_text(&param.title),
                        html!("div", {
                            .class(class::FLEX_GROW1)
                            .child(html!("select" => HtmlSelectElement, {
                                .class(class::FORM_SELECT_SM)
                                .child(html!("option", {.attr("value", "").text("เลือก")}))
                                .apply(|dom| {
                                    if let Some(assets) = app.app_asset.lock_ref().as_ref() {
                                        dom.children(system_list_type.get_items(assets).iter().map(|option| {
                                            html!("option", {
                                                .attr("value", &option.key)
                                                .text(&option.label)
                                            })
                                        }))
                                    } else {
                                        dom
                                    }
                                })
                                .apply(mixins::string_value_select(param.id.clone(), changed))
                            }))
                        }),
                    ])
                })
            }
            ParamType::Array(basic_type) => {
                html!("div", {
                    .class(class::BORDER_ROUND)
                    .children([
                        html!("div", {
                            .class("mb-2")
                            .children([
                                html!("span", {.text(&param.title)}),
                                html!("buton", {
                                    .attr("type","button")
                                    .class(class::BTN_SM_FR_GRAY)
                                    .child(html!("i", {.class(class::FA_PLUS)}))
                                    .event(clone!(param => move |_: events::Click| {
                                        param.ids.lock_mut().push_cloned(Mutable::new(String::new()));
                                    }))
                                }),
                            ])
                        }),
                        html!("div", {
                            .children_signal_vec(param.ids.signal_vec_cloned().enumerate().map(clone!(param, changed => move |(i, id_mutable)| {
                                html!("div", {
                                    .class(class::INPUT_GROUP_SM_T)
                                    .children([
                                        html!("span", {
                                            .class("input-group-text")
                                            .text_signal(i.signal().map(|opt| opt.map(|u| (u + 1).to_string()).unwrap_or_default()))
                                        }),
                                        render_basic_type(&basic_type, id_mutable, changed.clone()),
                                        html!("botton", {
                                            .attr("type","button")
                                            .class(class::BTN_SM_RED)
                                            .child(html!("i", {.class(class::FA_X)}))
                                            .event(clone!(param => move |_: events::Click| {
                                                if let Some(pos) = i.get() {
                                                    param.ids.lock_mut().remove(pos);
                                                }
                                            }))
                                        }),
                                    ])
                                })
                            })))
                        }),
                    ])
                })
            }
            ParamType::ArrayList(_, items) => {
                html!("div", {
                    .class(class::BORDER_ROUND)
                    .children([
                        html!("div", {
                            .class("mb-2")
                            .children([
                                html!("span", {.text(&param.title)}),
                                html!("buton", {
                                    .attr("type","button")
                                    .class(class::BTN_SM_FR_GRAY)
                                    .child(html!("i", {.class(class::FA_PLUS)}))
                                    .event(clone!(param => move |_: events::Click| {
                                        param.ids.lock_mut().push_cloned(Mutable::new(String::new()));
                                    }))
                                }),
                            ])
                        }),
                        html!("div", {
                            .children_signal_vec(param.ids.signal_vec_cloned().enumerate().map(clone!(param, changed => move |(i, id_mutable)| {
                                html!("div", {
                                    .class(class::INPUT_GROUP_SM_T)
                                    .children([
                                        html!("span", {
                                            .class("input-group-text")
                                            .text_signal(i.signal().map(|opt| opt.map(|u| (u + 1).to_string()).unwrap_or_default()))
                                        }),
                                        html!("div", {
                                            .class(class::FLEX_GROW1)
                                            .child(html!("select" => HtmlSelectElement, {
                                                .class(class::FORM_SELECT_SM)
                                                .child(html!("option", {.attr("value", "").text("เลือก")}))
                                                .children(items.iter().map(|option| {
                                                    html!("option", {
                                                        .attr("value", &option.key)
                                                        .text(&option.label)
                                                    })
                                                }))
                                                .apply(mixins::string_value_select(id_mutable, changed.clone()))
                                            }))
                                        }),
                                        html!("botton", {
                                            .attr("type","button")
                                            .class(class::BTN_SM_RED)
                                            .child(html!("i", {.class(class::FA_X)}))
                                            .event(clone!(param => move |_: events::Click| {
                                                if let Some(pos) = i.get() {
                                                    param.ids.lock_mut().remove(pos);
                                                }
                                            }))
                                        }),
                                    ])
                                })
                            })))
                        }),
                    ])
                })
            }
            ParamType::ArrayListSystem(system_list_type) => {
                html!("div", {
                    .class(class::BORDER_ROUND)
                    .children([
                        html!("div", {
                            .class("mb-2")
                            .children([
                                html!("span", {.text(&param.title)}),
                                html!("buton", {
                                    .attr("type","button")
                                    .class(class::BTN_SM_FR_GRAY)
                                    .child(html!("i", {.class(class::FA_PLUS)}))
                                    .event(clone!(param => move |_: events::Click| {
                                        param.ids.lock_mut().push_cloned(Mutable::new(String::new()));
                                    }))
                                }),
                            ])
                        }),
                        html!("div", {
                            .children_signal_vec(param.ids.signal_vec_cloned().enumerate().map(clone!(param, changed => move |(i, id_mutable)| {
                                html!("div", {
                                    .class(class::INPUT_GROUP_SM_T)
                                    .children([
                                        html!("span", {
                                            .class("input-group-text")
                                            .text_signal(i.signal().map(|opt| opt.map(|u| (u + 1).to_string()).unwrap_or_default()))
                                        }),
                                        html!("div", {
                                            .class(class::FLEX_GROW1)
                                            .child(html!("select" => HtmlSelectElement, {
                                                .class(class::FORM_SELECT_SM)
                                                .child(html!("option", {.attr("value", "").text("เลือก")}))
                                                .apply(|dom| {
                                                    if let Some(assets) = app.app_asset.lock_ref().as_ref() {
                                                        dom.children(system_list_type.get_items(assets).iter().map(|option| {
                                                            html!("option", {
                                                                .attr("value", &option.key)
                                                                .text(&option.label)
                                                            })
                                                        }))
                                                    } else {
                                                        dom
                                                    }
                                                })
                                                .apply(mixins::string_value_select(id_mutable.clone(), changed.clone()))
                                            }))
                                        }),
                                        html!("botton", {
                                            .attr("type","button")
                                            .class(class::BTN_SM_RED)
                                            .child(html!("i", {.class(class::FA_X)}))
                                            .event(clone!(param => move |_: events::Click| {
                                                if let Some(pos) = i.get() {
                                                    param.ids.lock_mut().remove(pos);
                                                }
                                            }))
                                        }),
                                    ])
                                })
                            })))
                        }),
                    ])
                })
            }
        }
    }
}

fn render_basic_type(basic_type: &BasicType, mutable: Mutable<String>, changed: Mutable<bool>) -> Dom {
    match basic_type {
        BasicType::Date => doms::date_picker(
            mutable,
            changed,
            always(false),
            None,
            |d| d.class(class::FLEX_GROW1).style("min-width", "120px"),
            |d| d.class(class::FORM_CTRL_ONLY_SM_R0_R),
            |d| d.class(class::FORM_CTRL_ONLY_SM_R0_R),
            |s| s,
            always(None),
        ),
        BasicType::Time => doms::time_picker(
            mutable,
            changed,
            always(false),
            None,
            |d| d.class(class::FLEX_GROW1).style("min-width", "95px"),
            |d| d.class(class::FORM_CTRL_ONLY_SM_R0_R),
            |d| d.class(class::FORM_CTRL_ONLY_SM_R0_R),
            |s| s,
            always(None),
        ),
        BasicType::DateTime => doms::datetime_picker(
            mutable,
            changed,
            always(false),
            |d| d.class(class::FLEX_GROW1).style("min-width", "175px"),
            |d| d.class(class::FORM_CTRL_ONLY_SM_R0_R),
            |d| d.class(class::FORM_CTRL_ONLY_SM_R0_R),
            |s| s,
            always(None),
        ),
        _ => {
            html!("input" => HtmlInputElement, {
                .attr("type", "text")
                .class(class::FORM_CTRL_SM)
                .apply(mixins::string_value(mutable, changed))
            })
        }
    }
}
