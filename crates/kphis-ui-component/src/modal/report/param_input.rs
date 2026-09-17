use dominator::{Dom, clone, events, html};
use futures_signals::{
    map_ref,
    signal::{Mutable, Signal, SignalExt, always},
    signal_vec::{MutableVec, SignalVecExt},
};
use std::rc::Rc;
use web_sys::HtmlInputElement;

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
        let ids = if matches!(param.ty, ParamType::Array(_)) {
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
        let (ids, id) = if matches!(param.ty, ParamType::Array(_)) {
            (MutableVec::new_with_values(values), Mutable::new(String::new()))
        } else {
            (MutableVec::new(), Mutable::new(value.to_owned()))
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
                    .class(class::INPUT_GROUP_T)
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
                        doms::select_box(
                            "", Some("เลือก"), false,
                            param.id.clone(), changed,
                            |d| d.class("form-control"), || {},
                            items,
                        ),
                    ])
                })
            }
            ParamType::ListSystem(system_list_type) => {
                let items = app.app_asset.lock_ref().as_ref().map(|assets| system_list_type.get_items(assets)).unwrap_or_default();
                html!("div", {
                    .class(class::INPUT_GROUP_T)
                    .children([
                        doms::span_group_text(&param.title),
                        doms::select_box(
                            "", Some("เลือก"), false,
                            param.id.clone(), changed,
                            |d| d.class("form-control"), || {},
                            items,
                        ),
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
                                    .event(clone!(param, changed => move |_: events::Click| {
                                        param.ids.lock_mut().push_cloned(Mutable::new(String::new()));
                                        changed.set(true);
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
                                            .class(class::BTN_RED)
                                            .child(html!("i", {.class(class::FA_X)}))
                                            .event(clone!(param, changed => move |_: events::Click| {
                                                if let Some(pos) = i.get() {
                                                    param.ids.lock_mut().remove(pos);
                                                    changed.set(true);
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
                    .class(class::INPUT_GROUP_T)
                    .children([
                        doms::span_group_text(&param.title),
                        doms::select_box(
                            "", None, true,
                            param.id.clone(), changed.clone(),
                            |d| d.class("form-control"), || {},
                            items,
                        ),
                        html!("botton", {
                            .attr("type","button")
                            .class(class::BTN_RED)
                            .child(html!("i", {.class(class::FA_X)}))
                            .event(clone!(param => move |_: events::Click| {
                                let is_empty = param.id.lock_ref().is_empty();
                                if !is_empty {
                                    param.id.set_neq(String::new());
                                    changed.set(true);
                                }
                            }))
                        }),
                    ])
                })
            }
            ParamType::ArrayListSystem(system_list_type) => {
                let items = app.app_asset.lock_ref().as_ref().map(|assets| system_list_type.get_items(assets)).unwrap_or_default();
                html!("div", {
                    .class(class::INPUT_GROUP_T)
                    .children([
                        doms::span_group_text(&param.title),
                        doms::select_box(
                            "", None, true,
                            param.id.clone(), changed.clone(),
                            |d| d.class("form-control"), || {},
                            items,
                        ),
                        html!("botton", {
                            .attr("type","button")
                            .class(class::BTN_RED)
                            .child(html!("i", {.class(class::FA_X)}))
                            .event(clone!(param => move |_: events::Click| {
                                let is_empty = param.id.lock_ref().is_empty();
                                if !is_empty {
                                    param.id.set_neq(String::new());
                                    changed.set(true);
                                }
                            }))
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
            |d| d.class(class::FLEX_GROW1).style("min-width", "135px"),
            |d| d.class("rounded-end-0"),
            |d| d.class("rounded-end-0"),
            |s| s,
            always(None),
        ),
        BasicType::Time => doms::time_picker(
            mutable,
            changed,
            always(false),
            None,
            |d| d.class(class::FLEX_GROW1).style("min-width", "110px"),
            |d| d.class("rounded-end-0"),
            |d| d.class("rounded-end-0"),
            |s| s,
            always(None),
        ),
        BasicType::DateTime => doms::datetime_picker(
            mutable,
            changed,
            always(false),
            |d| d.class(class::FLEX_GROW1).style("min-width", "190px"),
            |d| d.class("rounded-end-0"),
            |d| d.class("rounded-end-0"),
            |s| s,
            always(None),
        ),
        _ => {
            html!("input" => HtmlInputElement, {
                .attr("type", "text")
                .class("form-control")
                .apply(mixins::string_value(mutable, changed))
            })
        }
    }
}
