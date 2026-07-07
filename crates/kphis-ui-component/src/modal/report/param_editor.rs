use dominator::{Dom, clone, events, html, with_node};
use futures_signals::{
    map_ref,
    signal::{Mutable, ReadOnlyMutable, SignalExt},
    signal_vec::{MutableVec, SignalVecExt},
};
use std::{
    rc::Rc,
    sync::atomic::{AtomicU32, Ordering},
};
use strum::IntoEnumIterator;
use web_sys::{HtmlInputElement, HtmlSelectElement, HtmlTextAreaElement};

use kphis_model::report::{BasicType, KeyLabel, ReportParam, SystemListType, VarType};
use kphis_ui_app::App;
use kphis_ui_core::{class, mixins};

static ID_COUNTER: AtomicU32 = AtomicU32::new(0);

#[derive(Default)]
pub struct ReportParamEditor {
    changed: Mutable<bool>,
    param_items: MutableVec<Rc<ReportParamMutable>>,

    parent_params: Mutable<String>,
    parent_changed: Mutable<bool>,
}

impl ReportParamEditor {
    pub fn new(parent_params: Mutable<String>, parent_changed: Mutable<bool>, app: Rc<App>) -> Rc<Self> {
        let param_items = MutableVec::new_with_values(
            ReportParam::from_cap_pipe(&parent_params.lock_ref())
                .iter()
                .map(|rp| ReportParamMutable::from_report_param(rp, app.clone()))
                .collect(),
        );
        Rc::new(Self {
            param_items,
            parent_params,
            parent_changed,
            ..Default::default()
        })
    }

    /// Fixed modal, NOT bootstrap
    pub fn render(modal: Rc<Self>, self_opt: Mutable<Option<Rc<Self>>>, app: Rc<App>) -> Dom {
        html!("div", {
            .style("position","fixed")
            .style("right","25px")
            .style("top","115px")
            .style("width","730px")
            .style("background-color","var(--bs-body-bg)")
            .class(class::BOX_ROUND)
            .future(modal.changed.signal().for_each(clone!(modal => move |changed| {
                if changed {
                    modal.parent_params.set(modal.param_items.lock_ref().iter().filter_map(|item| item.to_str()).collect::<Vec<String>>().join("|"));
                    modal.parent_changed.set_neq(true);
                    modal.changed.set(false);
                }
                async {}
            })))
            .child(html!("div", {
                .children([
                    html!("div", {
                        .children([
                            html!("span", {
                                .class(class::BOLD_FS5_R)
                                .text("Query Parameters")
                            }),
                            html!("button", {
                                .attr("type", "button")
                                .class(class::BTN_FR_R)
                                .child(html!("i", {.class(class::FA_X)}))
                                .event(clone!(self_opt => move |_:events::Click| {
                                    self_opt.set(None);
                                }))
                            }),
                        ])
                    }),
                    html!("hr", {.class("my-2")}),
                    html!("div", {
                        .class("mb-2")
                        .child(html!("textarea" => HtmlTextAreaElement, {
                            .class("form-control")
                            .style("font-family","Consolas, monospace, serif")
                            .style("color","var(--bs-body-bg)")
                            .style("background-color","var(--bs-body-color)")
                            .prop_signal("value", modal.parent_params.signal_cloned())
                            .with_node!(element => {
                                .style_signal("height", modal.parent_params.signal_ref(clone!(element => move |_| {
                                    element.style().set_property("height", "auto").unwrap();
                                    let min_height = 40 + ((element.rows() - 1) * 24) as i32;
                                    let scroll_height = element.scroll_height();
                                    let height = if scroll_height < min_height {min_height} else {scroll_height + 2};
                                    [&height.to_string(), "px"].concat()
                                })))
                                .event(clone!(app, modal => move |_: events::Input| {
                                    let value = element.value();
                                    let neq = modal.parent_params.lock_ref().as_str() != value;
                                    if neq {
                                        modal.param_items.lock_mut().replace_cloned(ReportParam::from_cap_pipe(&value)
                                            .iter()
                                            .map(|rp| ReportParamMutable::from_report_param(rp, app.clone()))
                                            .collect()
                                        );
                                        modal.parent_params.set(value.to_owned());
                                        modal.parent_changed.set_neq(true);
                                    }
                                }))
                            })
                        }))
                    }),
                    html!("div", {
                        .class(class::FLEX_JCR_T)
                        .children([
                            html!("button", {
                                .attr("type","button")
                                .class(class::BTN_SM_GRAY)
                                .child(html!("i", {.class(class::FA_PLUS_L)}))
                                .text("Single")
                                .event(clone!(modal => move |_:events::Click| {
                                    modal.param_items.lock_mut().push_cloned(ReportParamMutable::new(VarType::Basic, false));
                                    modal.changed.set(true);
                                }))
                            }),
                            html!("button", {
                                .attr("type","button")
                                .class(class::BTN_SM_R_GRAY)
                                .child(html!("i", {.class(class::FA_PLUS_L)}))
                                .text("List")
                                .event(clone!(modal => move |_:events::Click| {
                                    modal.param_items.lock_mut().push_cloned(ReportParamMutable::new(VarType::List, false));
                                    modal.changed.set(true);
                                }))
                            }),
                            html!("button", {
                                .attr("type","button")
                                .class(class::BTN_SM_R_GRAY)
                                .child(html!("i", {.class(class::FA_PLUS_L)}))
                                .text("System List")
                                .event(clone!(modal => move |_:events::Click| {
                                    modal.param_items.lock_mut().push_cloned(ReportParamMutable::new(VarType::System, false));
                                    modal.changed.set(true);
                                }))
                            }),
                            html!("button", {
                                .attr("type","button")
                                .class(class::BTN_SM_R_CYAN)
                                .child(html!("i", {.class(class::FA_PLUS_L)}))
                                .text("Array")
                                .event(clone!(modal => move |_:events::Click| {
                                    modal.param_items.lock_mut().push_cloned(ReportParamMutable::new(VarType::Basic, true));
                                    modal.changed.set(true);
                                }))
                            }),
                            html!("button", {
                                .attr("type","button")
                                .class(class::BTN_SM_R_CYAN)
                                .child(html!("i", {.class(class::FA_PLUS_L)}))
                                .text("Array of List")
                                .event(clone!(modal => move |_:events::Click| {
                                    modal.param_items.lock_mut().push_cloned(ReportParamMutable::new(VarType::List, true));
                                    modal.changed.set(true);
                                }))
                            }),
                            html!("button", {
                                .attr("type","button")
                                .class(class::BTN_SM_R_CYAN)
                                .child(html!("i", {.class(class::FA_PLUS_L)}))
                                .text("Array of System List")
                                .event(clone!(modal => move |_:events::Click| {
                                    modal.param_items.lock_mut().push_cloned(ReportParamMutable::new(VarType::System, true));
                                    modal.changed.set(true);
                                }))
                            }),
                        ])
                    }),
                    html!("div", {
                        .class("mb-2")
                        .style("height","500px")
                        .style("overflow-y","auto")
                        .child(html!("div", {
                            .children_signal_vec(modal.param_items.signal_vec_cloned().enumerate().map(clone!(modal => move |(i, param)| {
                                Self::render_param(i, param.clone(), modal.clone())
                            })))
                        }))
                    }),
                    html!("div", {
                        .child(html!("button", {
                            .attr("type", "button")
                            .class(class::BTN_FR_GRAY)
                            .text("ปิด")
                            .event(clone!(self_opt => move |_:events::Click| {
                                self_opt.set(None);
                            }))
                        }))
                    }),
                ])
            }))
        })
    }

    fn render_param(i: ReadOnlyMutable<Option<usize>>, param_item: Rc<ReportParamMutable>, modal: Rc<Self>) -> Dom {
        html!("div", {
            .class("mb-2")
            .child(html!("div", {
                .class(class::INPUT_GROUP_SM)
                .children([
                    html!("span", {
                        .class("input-group-text")
                        .apply_if(param_item.is_array, |dom| dom.class("bg-info"))
                        .text_signal(i.signal().map(|opt| opt.map(|n| (n + 1).to_string()).unwrap_or_default()))
                        .text(" ID")}),
                    html!("input" => HtmlInputElement, {
                        .attr("type","text")
                        .class(class::FORM_CTRL_SM)
                        .apply(mixins::string_value(param_item.id.clone(), modal.changed.clone()))
                    }),
                    html!("span", {.class("input-group-text").apply_if(param_item.is_array, |dom| dom.class("bg-info")).text("Title")}),
                    html!("input" => HtmlInputElement, {
                        .attr("type","text")
                        .class(class::FORM_CTRL_SM)
                        .apply(mixins::string_value(param_item.title.clone(), modal.changed.clone()))
                    }),
                    html!("span", {.class("input-group-text").apply_if(param_item.is_array, |dom| dom.class("bg-info")).text("Type")}),
                    render_basic_type_list(matches!(param_item.var_type, VarType::System), param_item.ty.clone(), modal.changed.clone()),
                ])
                .apply_if(matches!(param_item.var_type, VarType::List), |dom| dom
                    .child(html!("button", {
                        .attr("type","button")
                        .class(class::BTN_SM_GOLD)
                        .child(html!("i", {.class(class::FA_PLUS)}))
                        .event(clone!(modal, param_item => move |_:events::Click| {
                            param_item.items.lock_mut().push_cloned(KeyLabelMutable::new());
                            modal.changed.set(true);
                        }))
                    }))
                )
                .child_signal(i.signal().map(clone!(modal => move |opt| {
                    opt.and_then(|pos| {
                        (pos > 0).then(|| {
                            html!("button", {
                                .attr("type","button")
                                .class(class::BTN_GRAY)
                                .child(html!("i", {.class(class::FA_UP)}))
                                .event(clone!(modal => move |_:events::Click| {
                                    modal.param_items.lock_mut().swap(pos, pos - 1);
                                    modal.changed.set(true);
                                }))
                            })
                        })
                    })
                })))
                .child_signal(map_ref!{
                    let opt = i.signal(),
                    let len = modal.param_items.signal_vec_cloned().len() =>
                    (*opt, *len)
                }.map(clone!(modal => move |(opt, len)| {
                    opt.and_then(|pos| {
                        (pos < len.saturating_sub(1)).then(|| {
                            html!("button", {
                                .attr("type","button")
                                .class(class::BTN_GRAY)
                                .child(html!("i", {.class(class::FA_DOWN)}))
                                .event(clone!(modal => move |_:events::Click| {
                                    modal.param_items.lock_mut().swap(pos, pos + 1);
                                    modal.changed.set(true);
                                }))
                            })
                        })
                    })
                })))
                .child(html!("button", {
                    .attr("type","button")
                    .class(class::BTN_SM_RED)
                    .child(html!("i", {.class(class::FA_X)}))
                    .event(clone!(modal, param_item => move |_:events::Click| {
                        modal.param_items.lock_mut().retain(|i| i.uid != param_item.uid);
                        modal.changed.set(true);
                    }))
                }))
            }))
            .apply(|dom| {
                match &param_item.var_type {
                    VarType::Basic => dom,
                    VarType::List => dom
                        .child(html!("div", {
                            .class(class::BORDER_U_RB)
                            .children_signal_vec(param_item.items.signal_vec_cloned().enumerate().map(clone!(modal, param_item => move |(i, item)| {
                                Self::render_param_item(i, item.clone(), param_item.clone(), modal.clone())
                            })))
                        })),
                    VarType::System => dom
                        .child(html!("div", {
                            .class(class::BORDER_U_RB)
                            .child(html!("div", {
                                .class(class::INPUT_GROUP_SM)
                                .children([
                                    html!("span", {.class("input-group-text").apply_if(param_item.is_array, |dom| dom.class("bg-info")).text("System List")}),
                                    html!("div", {
                                        .class(class::FLEX_W100)
                                        .child(html!("select" => HtmlSelectElement, {
                                            .class(class::FORM_SELECT_SM)
                                            .children(SystemListType::iter().map(|slt| {
                                                html!("option", {
                                                    .attr("value", slt.to_str())
                                                    .text(slt.to_label())
                                                    .text(" : ")
                                                    .text(slt.source_hint())
                                                })
                                            }))
                                            .prop_signal("value", param_item.system_list_type.signal_ref(|opt| opt.as_ref().map(|slt| slt.to_str()).unwrap_or_default()))
                                            .with_node!(element => {
                                                .event(clone!(modal, param_item => move |_:events::Change| {
                                                    let v = element.value();
                                                    let value = v.as_str();
                                                    let old =  param_item.system_list_type.lock_ref().as_ref().map(|slt| slt.to_str()).unwrap_or_default();
                                                    if old != value {
                                                        let new = SystemListType::from(value);
                                                        param_item.ty.set(new.key_type());
                                                        param_item.system_list_type.set(Some(new));
                                                        modal.changed.set_neq(true);
                                                    }
                                                }))
                                            })
                                        }))
                                    }),
                                ])
                            }))
                        })),
                }
            })
        })
    }

    fn render_param_item(i: ReadOnlyMutable<Option<usize>>, item: Rc<KeyLabelMutable>, param_item: Rc<ReportParamMutable>, modal: Rc<Self>) -> Dom {
        html!("div", {
            .class(class::INPUT_GROUP_SM_T)
            .children([
                // ew use key as parameter value, so we label with "Value" here
                html!("span", {.class("input-group-text").text("Value")}),
                html!("input" => HtmlInputElement, {
                    .attr("type","text")
                    .class(class::FORM_CTRL_SM)
                    .class_signal("bg-warning", map_ref!{
                        let key = item.key.signal_cloned(),
                        let ty = param_item.ty.signal_cloned() =>
                        !ty.is_value_parsable(&key)
                    })
                    .style("max-width","25%")
                    .prop_signal("value", item.key.signal_cloned())
                    .apply(mixins::string_value(item.key.clone(), modal.changed.clone()))
                }),
                html!("span", {.class("input-group-text").text("Label")}),
                html!("input" => HtmlInputElement, {
                    .attr("type","text")
                    .class(class::FORM_CTRL_SM)
                    .apply(mixins::string_value(item.label.clone(), modal.changed.clone()))
                }),
            ])
            .child_signal(i.signal().map(clone!(modal, param_item => move |opt| {
                opt.and_then(|pos| {
                    (pos > 0).then(|| {
                        html!("button", {
                            .attr("type","button")
                            .class(class::BTN_GRAY)
                            .child(html!("i", {.class(class::FA_UP)}))
                            .event(clone!(modal, param_item => move |_:events::Click| {
                                param_item.items.lock_mut().swap(pos, pos - 1);
                                modal.changed.set(true);
                            }))
                        })
                    })
                })
            })))
            .child_signal(map_ref!{
                let opt = i.signal(),
                let len = param_item.items.signal_vec_cloned().len() =>
                (*opt, *len)
            }.map(clone!(modal, param_item => move |(opt, len)| {
                opt.and_then(|pos| {
                    (pos < len.saturating_sub(1)).then(|| {
                        html!("button", {
                            .attr("type","button")
                            .class(class::BTN_GRAY)
                            .child(html!("i", {.class(class::FA_DOWN)}))
                            .event(clone!(modal, param_item => move |_:events::Click| {
                                param_item.items.lock_mut().swap(pos, pos + 1);
                                modal.changed.set(true);
                            }))
                        })
                    })
                })
            })))
            .child(html!("button", {
                .attr("type","button")
                .class(class::BTN_SM_RED)
                .child(html!("i", {.class(class::FA_X)}))
                .event(clone!(modal, param_item => move |_:events::Click| {
                    param_item.items.lock_mut().retain(|i| i.uid != item.uid);
                    modal.changed.set(true);
                }))
            }))
        })
    }
}

fn render_basic_type_list(is_disabled: bool, ty_mutable: Mutable<BasicType>, changed: Mutable<bool>) -> Dom {
    html!("select" => HtmlSelectElement, {
        .class(class::FORM_SELECT_SM)
        .apply_if(is_disabled, |dom| dom.attr("disabled",""))
        .children(BasicType::iter().map(|bt| {
            html!("option", {
                .attr("value", bt.to_str())
                .text(bt.to_label())
            })
        }))
        .prop_signal("value", ty_mutable.signal_ref(|ty| ty.to_str()))
        .with_node!(element => {
            .event(clone!(ty_mutable, changed => move |_: events::Change| {
                let value = element.value();
                let neq = ty_mutable.lock_ref().to_str() != value.as_str();
                if neq {
                    ty_mutable.set(BasicType::from(value.as_str()));
                    changed.set_neq(true);
                }
            }))
        })
    })
}

#[derive(Clone)]
struct ReportParamMutable {
    uid: u32,
    var_type: VarType,
    is_array: bool,
    id: Mutable<String>,
    title: Mutable<String>,
    ty: Mutable<BasicType>,
    system_list_type: Mutable<Option<SystemListType>>,
    items: MutableVec<Rc<KeyLabelMutable>>,
}

impl ReportParamMutable {
    fn new(var_type: VarType, is_array: bool) -> Rc<Self> {
        let items = if matches!(var_type, VarType::List) {
            MutableVec::new_with_values(vec![KeyLabelMutable::new()])
        } else {
            MutableVec::new()
        };
        Rc::new(Self {
            uid: ID_COUNTER.fetch_add(1, Ordering::SeqCst),
            var_type,
            is_array,
            id: Mutable::new(String::new()),
            title: Mutable::new(String::new()),
            ty: Mutable::new(BasicType::Str),
            system_list_type: Mutable::new(None),
            items,
        })
    }

    fn from_report_param(item: &ReportParam, app: Rc<App>) -> Rc<Self> {
        let items = if let Some(assets) = &app.state().app_asset.lock_ref().as_ref() {
            item.ty.get_items(assets).into_iter().map(KeyLabelMutable::from_key_label).collect()
        } else {
            Vec::new()
        };
        Rc::new(Self {
            uid: ID_COUNTER.fetch_add(1, Ordering::SeqCst),
            var_type: item.ty.get_var_type(),
            is_array: item.is_array(),
            id: Mutable::new(item.id.to_owned()),
            title: Mutable::new(item.title.to_owned()),
            ty: Mutable::new(item.ty.get_basic_type().to_owned()),
            items: MutableVec::new_with_values(items),
            system_list_type: Mutable::new(item.ty.get_system_list_type()),
        })
    }

    /// remove all key that cannot parse to specified type
    fn to_str(&self) -> Option<String> {
        let id = self.id.lock_ref();
        let title = self.title.lock_ref();
        let key_type = self.ty.lock_ref();
        let key_type_str = key_type.to_str();
        let ty = match &self.var_type {
            VarType::Basic => {
                if self.is_array {
                    ["[", key_type_str, "]"].concat()
                } else {
                    key_type_str.to_owned()
                }
            }
            VarType::List => {
                let kvs = &self
                    .items
                    .lock_ref()
                    .iter()
                    .filter_map(|item| {
                        let key = item.key.get_cloned();
                        let is_key_parsable = key_type.is_value_parsable(&key);
                        let label = item.label.get_cloned();
                        (!key.is_empty() && !label.is_empty() && is_key_parsable).then(move || [key, label])
                    })
                    .flatten()
                    .collect::<Vec<String>>();
                // reverse to basic type if NO Key-Value
                if kvs.is_empty() {
                    if self.is_array { ["[", key_type_str, "]"].concat() } else { key_type_str.to_owned() }
                } else if self.is_array {
                    ["[(", key_type_str, ",", &kvs.join(","), ")]"].concat()
                } else {
                    ["(", key_type_str, ",", &kvs.join(","), ")"].concat()
                }
            }
            VarType::System => {
                if let Some(system_list_type) = self.system_list_type.lock_ref().as_ref() {
                    let system_list_type_str = system_list_type.to_str();
                    if self.is_array {
                        ["[(", system_list_type_str, ")]"].concat()
                    } else {
                        ["(", system_list_type_str, ")"].concat()
                    }
                // reverse to basic type if NO SystemListType
                } else if self.is_array {
                    ["[", key_type_str, "]"].concat()
                } else {
                    key_type_str.to_owned()
                }
            }
        };
        (!id.is_empty() && !title.is_empty() && !ty.is_empty()).then(|| [&id, &title, ty.as_str()].join("^"))
    }
}

#[derive(Clone)]
struct KeyLabelMutable {
    uid: u32,
    key: Mutable<String>,
    label: Mutable<String>,
}

impl KeyLabelMutable {
    fn new() -> Rc<Self> {
        Rc::new(Self {
            uid: ID_COUNTER.fetch_add(1, Ordering::SeqCst),
            key: Mutable::new(String::new()),
            label: Mutable::new(String::new()),
        })
    }

    fn from_key_label(item: KeyLabel) -> Rc<Self> {
        Rc::new(Self {
            uid: ID_COUNTER.fetch_add(1, Ordering::SeqCst),
            key: Mutable::new(item.key),
            label: Mutable::new(item.label),
        })
    }
}
