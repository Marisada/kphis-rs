use dominator::{Dom, clone, events, html, with_node};
use futures_signals::{
    map_ref,
    signal::{Mutable, Signal, SignalExt, not},
    signal_vec::{MutableVec, SignalVecExt},
};
use std::rc::Rc;
use web_sys::{HtmlButtonElement, HtmlInputElement, HtmlSelectElement, HtmlTextAreaElement};

use kphis_model::{
    drug_use_duration::{DrugUseDuration, DrugUseDurationParams},
    endpoint::EndPoint,
    fetch::Method,
    search::searchbox::MedSearchbox,
};
use kphis_ui_app::App;
use kphis_ui_core::{class, doms, mixins};
use kphis_util::util::str_some;

/// not is_form
/// - GET `EndPoint::DrugUseDuration`
/// - GET `EndPoint::SearchBoxMedHnText`
/// is_form
/// - GET `EndPoint::SearchBoxMedHnText`
/// - POST `EndPoint::DrugUseDuration`
#[derive(Clone, Default)]
pub struct DrugDetailModal {
    // editable form or information only
    is_form: bool,
    // loaded: Mutable<bool>,
    changed: Mutable<bool>,
    parent_need_reload: Mutable<bool>,

    icode: Mutable<String>,
    med_name: Mutable<String>,
    usage: Mutable<String>,
    duration1: Mutable<String>,
    exceed_duration1_color: Mutable<String>,
    duration2: Mutable<String>,
    exceed_duration2_color: Mutable<String>,
    duration3: Mutable<String>,
    exceed_duration3_color: Mutable<String>,
    status: Mutable<String>,

    monitor: Mutable<String>,
    monitor_count: Mutable<String>,
    monitor_duration: Mutable<String>,
    monitor_status: Mutable<String>,

    info: Mutable<String>,
    info_status: Mutable<String>,

    display_med_searchbox: Mutable<bool>,
    search_text: Mutable<String>,
    results: MutableVec<Rc<MedSearchbox>>,
    status_text: Mutable<Option<String>>,
}

impl DrugDetailModal {
    pub fn new(is_form: bool) -> Rc<Self> {
        Rc::new(Self { is_form, ..Default::default() })
    }

    pub fn new_with_med(med: &Rc<DrugUseDuration>) -> Rc<Self> {
        Rc::new(Self {
            is_form: true,
            icode: Mutable::new(med.icode.clone()),
            med_name: Mutable::new(med.med_name.clone().unwrap_or_default()),

            usage: Mutable::new(med.usage.clone().unwrap_or_default()),
            duration1: Mutable::new(med.duration1.map(|n| n.to_string()).unwrap_or_default()),
            exceed_duration1_color: Mutable::new(med.exceed_duration1_color.clone().unwrap_or_default()),
            duration2: Mutable::new(med.duration2.map(|n| n.to_string()).unwrap_or_default()),
            exceed_duration2_color: Mutable::new(med.exceed_duration2_color.clone().unwrap_or_default()),
            duration3: Mutable::new(med.duration3.map(|n| n.to_string()).unwrap_or_default()),
            exceed_duration3_color: Mutable::new(med.exceed_duration3_color.clone().unwrap_or_default()),
            status: Mutable::new(med.status.clone().unwrap_or_default()),

            monitor: Mutable::new(med.monitor.clone().unwrap_or_default()),
            monitor_count: Mutable::new(med.monitor_count.map(|n| n.to_string()).unwrap_or_default()),
            monitor_duration: Mutable::new(med.monitor_duration.map(|n| n.to_string()).unwrap_or_default()),
            monitor_status: Mutable::new(med.monitor_status.clone().unwrap_or_default()),

            info: Mutable::new(med.info.clone().unwrap_or_default()),
            info_status: Mutable::new(med.info_status.clone().unwrap_or_default()),

            // loaded: Mutable::new(true),
            ..Default::default()
        })
    }

    fn set_med(&self, med: &DrugUseDuration) {
        self.icode.set(med.icode.clone());
        self.med_name.set(med.med_name.clone().unwrap_or_default());

        self.usage.set(med.usage.clone().unwrap_or_default());
        // self.duration1.set(med.duration1.map(|n| n.to_string()).unwrap_or_default());
        // self.exceed_duration1_color.set(med.exceed_duration1_color.clone().unwrap_or_default());
        // self.duration2.set(med.duration2.map(|n| n.to_string()).unwrap_or_default());
        // self.exceed_duration2_color.set(med.exceed_duration2_color.clone().unwrap_or_default());
        // self.duration3.set(med.duration3.map(|n| n.to_string()).unwrap_or_default());
        // self.exceed_duration3_color.set(med.exceed_duration3_color.clone().unwrap_or_default());
        // self.status.set(med.status.clone().unwrap_or_default());

        self.monitor.set(med.monitor.clone().unwrap_or_default());
        // self.monitor_count.set(med.monitor_count.map(|n| n.to_string()).unwrap_or_default());
        // self.monitor_duration.set(med.monitor_duration.map(|n| n.to_string()).unwrap_or_default());
        // self.monitor_status.set(med.monitor_status.clone().unwrap_or_default());

        self.info.set(med.info.clone().unwrap_or_default());
        // self.info_status.set(med.info_status.clone().unwrap_or_default());
    }

    fn set_blank(&self) {
        self.icode.set(String::new());
        self.med_name.set(String::new());

        self.usage.set(String::new());
        // self.duration1.set(String::new());
        // self.exceed_duration1_color.set(String::new());
        // self.duration2.set(String::new());
        // self.exceed_duration2_color.set(String::new());
        // self.duration3.set(String::new());
        // self.exceed_duration3_color.set(String::new());
        // self.status.set(String::new());

        self.monitor.set(String::new());
        // self.monitor_count.set(String::new());
        // self.monitor_duration.set(String::new());
        // self.monitor_status.set(String::new());

        self.info.set(String::new());
        // self.info_status.set(String::new());
    }

    fn not_ready_signal(&self) -> impl Signal<Item = bool> + use<> {
        map_ref! {
            let has_usage = self.usage.signal_ref(|s| !s.is_empty()),
            let has_monitor = self.monitor.signal_ref(|s| !s.is_empty()),
            let has_info = self.info.signal_ref(|s| !s.is_empty()),
            let is_use = self.status.signal_ref(|s| s == "Y"),
            let is_monitor_use = self.monitor_status.signal_ref(|s| s == "Y"),
            let is_info_use = self.info_status.signal_ref(|s| s == "Y"),
            let changed = self.changed.signal() =>
            !changed || (*is_use && !has_usage) || (*is_monitor_use && !has_monitor) || (*is_info_use && !has_info)
        }
    }

    fn load_meds(modal: Rc<Self>, app: Rc<App>) {
        modal.status_text.set_neq(Some(String::from("กำลังค้นหา...")));
        app.async_load(
            true,
            clone!(app, modal => async move {
                let search_text = modal.search_text.get_cloned();
                if !search_text.is_empty() {
                    // GET `EndPoint::SearchBoxMedHnText`
                    match MedSearchbox::call_api_get("-", &search_text, app.state()).await {
                        Ok(results) => {
                            let mut lock = modal.results.lock_mut();
                            if !lock.is_empty() {
                                lock.clear();
                            }
                            if results.is_empty() {
                                modal.status_text.set(Some(String::from("ไม่พบรายการที่ค้นหา")));
                            } else {
                                lock.extend(results.into_iter().map(Rc::new));
                                modal.status_text.set(None);
                            }
                        }
                        Err(e) => {
                            modal.status_text.set(Some(String::from("การเชื่อมต่อขัดข้อง")));
                            app.alert_app_error(&e).await;
                        }
                    }
                }
            }),
        )
    }

    fn load_item(modal: Rc<Self>, app: Rc<App>) {
        let params = DrugUseDurationParams {
            icode: str_some(modal.icode.get_cloned()),
            ..Default::default()
        };
        app.async_load(
            true,
            clone!(app, modal => async move {
                // GET `EndPoint::DrugUseDuration`
                match DrugUseDuration::call_api_get(&params, app.state()).await {
                    Ok(response) => {
                        if let Some(med) = response.first() {
                            modal.set_med(&med);
                        } else {
                            modal.set_blank();
                        }
                    }
                    Err(e) => {
                        app.alert_app_error(&e).await;
                    }
                }
            }),
        );
    }

    fn clear_med_searchbox(&self) {
        self.search_text.set_neq(String::new());
        self.results.lock_mut().clear();
        self.status_text.set_neq(Some(String::from("กรอกข้อความเพื่อค้นหา")));
        self.display_med_searchbox.set(false);
    }

    fn save(modal: Rc<Self>, app: Rc<App>) {
        let save = DrugUseDuration {
            icode: modal.icode.get_cloned(),
            med_name: None,

            usage: str_some(modal.usage.get_cloned()),
            duration1: modal.duration1.lock_ref().parse::<i16>().ok(),
            exceed_duration1_color: str_some(modal.exceed_duration1_color.get_cloned()),
            duration2: modal.duration2.lock_ref().parse::<i16>().ok(),
            exceed_duration2_color: str_some(modal.exceed_duration2_color.get_cloned()),
            duration3: modal.duration3.lock_ref().parse::<i16>().ok(),
            exceed_duration3_color: str_some(modal.exceed_duration3_color.get_cloned()),
            status: str_some(modal.status.get_cloned()),

            monitor: str_some(modal.monitor.get_cloned()),
            monitor_count: modal.monitor_count.lock_ref().parse::<u8>().ok(),
            monitor_duration: modal.monitor_duration.lock_ref().parse::<u32>().ok(),
            monitor_status: str_some(modal.monitor_status.get_cloned()),

            info: str_some(modal.info.get_cloned()),
            info_status: str_some(modal.info_status.get_cloned()),
        };

        app.async_load(
            true,
            clone!(app => async move {
                // POST `EndPoint::DrugUseDuration`
                match DrugUseDuration::call_api_post(&save, app.state()).await {
                    Ok(response) => {
                        app.alert_execute_response(&response, clone!(modal => async move {
                            modal.changed.set_neq(false);
                            modal.parent_need_reload.set_neq(true);
                        })).await;
                    }
                    Err(e) => {
                        app.alert_app_error(&e).await;
                    }
                }
            }),
        )
    }

    pub fn render(modal: Rc<Self>, display: Mutable<Option<Rc<Self>>>, parent_reload: Option<Mutable<bool>>, app: Rc<App>) -> Dom {
        html!("div", {
            .apply_if(!modal.is_form, |dom| dom
                .future(map_ref!(
                    let busy = app.loader_is_loading(),
                    let changed = modal.changed.signal() =>
                    !busy && *changed
                ).for_each(clone!(app, modal => move |ready| {
                    if ready {
                        Self::load_item(modal.clone(), app.clone());
                        modal.changed.set(false);
                    }
                    async {}
                })))
            )
            .class(class::MODAL_DIALOG_XL_FULL)
            .children([
                html!("div", {
                    .class("modal-content")
                    .children([
                        html!("div", {
                            .class("modal-header")
                            .children([
                                html!("h4", {
                                    .class("modal-title")
                                    .text("ข้อมูลเวชภัณฑ์")
                                }),
                                html!("button", {
                                    .attr("type", "button")
                                    .class("btn-close")
                                    .attr("data-bs-dismiss", "modal")
                                    .attr("aria-label", "Close")
                                    .event(clone!(modal, display, parent_reload => move |_: events::Click| {
                                        if let Some(reload) = &parent_reload {
                                            reload.set_neq(modal.parent_need_reload.get());
                                        }
                                        display.set(None);
                                    }))
                                }),
                            ])
                        }),
                        html!("div", {
                            .class("modal-body")
                            .child(html!("div", {
                                .child(html!("div", {
                                    .class(class::ROW_MY2)
                                    .children([
                                        html!("div", {
                                            .class("col-sm-8")
                                            .child(html!("div", {
                                                .attr("id", "med_input_group")
                                                .class(class::INPUT_GROUP)
                                                .children([
                                                    html!("div", {
                                                        .class("input-group-text")
                                                        .text("ชื่อยา")
                                                    }),
                                                    html!("div", {
                                                        .class("input-group-text")
                                                        .style("min-width","333px")
                                                        .text_signal(modal.med_name.signal_cloned())
                                                        .apply_if(app.endpoint_is_allow(&Method::GET, &EndPoint::SearchBoxMedHnText, false), |dom| dom
                                                            .style("cursor","pointer")
                                                            .event(clone!(modal => move |_: events::Click| {
                                                                modal.med_name.set(String::new());
                                                                modal.display_med_searchbox.set_neq(true);
                                                            }))
                                                        )
                                                    }),
                                                ])
                                                .apply_if(app.endpoint_is_allow(&Method::GET, &EndPoint::SearchBoxMedHnText, false), |dom| dom
                                                    .child(html!("button", {
                                                        .attr("type", "button")
                                                        .class(class::BTN_GRAY)
                                                        .children([
                                                            html!("i", {.class(class::FA_SEARCH)}),
                                                        ])
                                                    }))
                                                    .event(clone!(modal => move |_: events::Click| {
                                                        modal.med_name.set(String::new());
                                                        modal.display_med_searchbox.set_neq(true);
                                                    }))
                                                )
                                            }))
                                            .child_signal(modal.display_med_searchbox.signal_cloned().map(clone!(app, modal => move |show| {
                                                if show {
                                                    app.get_id("med_input_group").map(|elm| {
                                                        doms::under_box(
                                                            elm.get_bounding_client_rect(),
                                                            600.0, 300.0, app.window_scroll_y(),
                                                            clone!(app, modal => move |bx| { bx
                                                                .child(Self::render_med_searchbox(modal, app))
                                                            })
                                                        )
                                                    })
                                                } else {
                                                    None
                                                }
                                            })))
                                        }),
                                    ])
                                }))
                                .apply_if(modal.is_form, |dom| dom.child(html!("hr")))
                                .children([
                                    html!("div", {
                                        .class(class::ROW_MY2)
                                        .child(html!("label", {
                                            .class(class::COL_SM12_BOLD)
                                            .attr("for","usage")
                                            .text("ข้อมูลทั่วไป (Info)")
                                        }))
                                    }),
                                    html!("div", {
                                        .class(class::ROW_MY2)
                                        .child(html!("div", {
                                            .class("col-sm-12")
                                            .apply(|dom| {
                                                if modal.is_form { dom
                                                    .child(html!("textarea" => HtmlTextAreaElement, {
                                                        .class("form-control")
                                                        .attr("id", "usage")
                                                        .attr("rows", "4")
                                                        .apply(mixins::textarea_value_auto_expand(modal.info.clone(), modal.changed.clone()))
                                                    }))
                                                } else { dom
                                                    .child(html!("div", {
                                                        .class(class::BOX_ROUND)
                                                        .style("white-space","pre-wrap")
                                                        .style("min-height","50px")
                                                        .children_signal_vec(modal.info.signal_ref(|info| {
                                                            doms::square_bracket_to_span(&info).collect()
                                                        }).to_signal_vec())
                                                    }))
                                                }
                                            })
                                        }))
                                    }),
                                ])
                                .apply_if(modal.is_form, |dom| dom
                                    .children([
                                        html!("div", {
                                            .class("col-sm-9")
                                            .child(html!("div", {
                                                .class(class::INPUT_GROUP)
                                                .children([
                                                    html!("label", {
                                                        .class(class::INPUT_GROUP_TEXT_BG_CYANS)
                                                        .attr("for", "status")
                                                        .text("เปิดใช้งาน Info เมื่อเลือกใช้ยา")
                                                    }),
                                                    html!("select" => HtmlSelectElement, {
                                                        .class("form-select")
                                                        .attr("id", "status")
                                                        .children([
                                                            html!("option", {.attr("value", "").text("เลือก")}),
                                                            html!("option", {.attr("value", "Y").text("ใช้งาน")}),
                                                            html!("option", {.attr("value", "N").text("ไม่ใช้งาน")}),
                                                        ])
                                                        .apply(mixins::string_value_select(modal.info_status.clone(), modal.changed.clone()))
                                                    }),
                                                    html!("span", {
                                                        .class("input-group-text")
                                                        .text("หากเปิดใช้งานร่วมกับ DUE จะแสดงเฉพาะ DUE (ไม่แสดง Info)")
                                                    }),
                                                ])
                                            }))
                                        }),
                                        html!("hr"),
                                        html!("div", {
                                            .class(class::ROW_MY2)
                                            .child(html!("div", {
                                                .class(class::COL_SM12_BOLD)
                                                .text("ระบุสีตัวอักษร เพื่อเตือนระยะเวลาสั่งใช้ยานานเกินกำหนด")
                                            }))
                                        }),
                                        html!("div", {
                                            .class(class::ROW_MY2)
                                            .children([
                                                html!("div", {
                                                    .class("col-sm-4")
                                                    .child(html!("div", {
                                                        .class(class::INPUT_GROUP)
                                                        .children([
                                                            doms::label_group_for("duration1", "ขั้นต่ำ เกิน"),
                                                            html!("input" => HtmlInputElement, {
                                                                .attr("type","number")
                                                                .class("form-control")
                                                                .attr("id","duration1")
                                                                .attr("min","0")
                                                                .attr("max","999")
                                                                .apply(mixins::string_value(modal.duration1.clone(), modal.changed.clone()))
                                                            }),
                                                            doms::label_group_for("exceed_duration1_color", "วัน ตัวอักษรสี"),
                                                            html!("div", {
                                                                .class("form-control")
                                                                .child(html!("input" => HtmlInputElement, {
                                                                    .attr("type","color")
                                                                    .class(class::FULL)
                                                                    .attr("list","color-list")
                                                                    .attr("id","exceed_duration1_color")
                                                                    .apply(mixins::string_value(modal.exceed_duration1_color.clone(), modal.changed.clone()))
                                                                }))
                                                            }),
                                                        ])
                                                    }))
                                                }),
                                                html!("div", {
                                                    .class("col-sm-4")
                                                    .child(html!("div", {
                                                        .class(class::INPUT_GROUP)
                                                        .children([
                                                            doms::label_group_for("duration2", "ขั้นกลาง เกิน"),
                                                            html!("input" => HtmlInputElement, {
                                                                .attr("type","number")
                                                                .class("form-control")
                                                                .attr("id","duration2")
                                                                .attr("min","0")
                                                                .attr("max","999")
                                                                .apply(mixins::string_value(modal.duration2.clone(), modal.changed.clone()))
                                                            }),
                                                            doms::label_group_for("exceed_duration2_color", "วัน ตัวอักษรสี"),
                                                            html!("div", {
                                                                .class("form-control")
                                                                .child(html!("input" => HtmlInputElement, {
                                                                    .attr("type","color")
                                                                    .class(class::FULL)
                                                                    .attr("list","color-list")
                                                                    .attr("id","exceed_duration2_color")
                                                                    .apply(mixins::string_value(modal.exceed_duration2_color.clone(), modal.changed.clone()))
                                                                }))
                                                            }),
                                                        ])
                                                    }))
                                                }),
                                                html!("div", {
                                                    .class("col-sm-4")
                                                    .child(html!("div", {
                                                        .class(class::INPUT_GROUP)
                                                        .children([
                                                            doms::label_group_for("duration3", "ขั้นสูง เกิน"),
                                                            html!("input" => HtmlInputElement, {
                                                                .attr("type","number")
                                                                .class("form-control")
                                                                .attr("id","duration3")
                                                                .attr("min","0")
                                                                .attr("max","999")
                                                                .apply(mixins::string_value(modal.duration3.clone(), modal.changed.clone()))
                                                            }),
                                                            doms::label_group_for("exceed_duration3_color", "วัน ตัวอักษรสี"),
                                                            html!("div", {
                                                                .class("form-control")
                                                                .child(html!("input" => HtmlInputElement, {
                                                                    .attr("type","color")
                                                                    .class(class::FULL)
                                                                    .attr("list","color-list")
                                                                    .attr("id","exceed_duration3_color")
                                                                    .apply(mixins::string_value(modal.exceed_duration3_color.clone(), modal.changed.clone()))
                                                                }))
                                                            }),
                                                        ])
                                                    }))
                                                }),
                                                doms::color_picker(),
                                            ])
                                        }),
                                    ])
                                )
                                .children([
                                    html!("div", {
                                        .class(class::ROW_MY2)
                                        .child(html!("label", {
                                            .class(class::COL_SM12_BOLD)
                                            .attr("for","usage")
                                            .text("เงื่อนไขการใช้ยา และขนาดยาที่เหมาะสม (DUE)")
                                        }))
                                    }),
                                    html!("div", {
                                        .class(class::ROW_MY2)
                                        .child(html!("div", {
                                            .class("col-sm-12")
                                            .apply(|dom| {
                                                if modal.is_form { dom
                                                    .child(html!("textarea" => HtmlTextAreaElement, {
                                                        .class("form-control")
                                                        .attr("id", "usage")
                                                        .attr("rows", "4")
                                                        .apply(mixins::textarea_value_auto_expand(modal.usage.clone(), modal.changed.clone()))
                                                    }))
                                                } else { dom
                                                    .child(html!("div", {
                                                        .class(class::BOX_ROUND)
                                                        .style("white-space","pre-wrap")
                                                        .style("min-height","50px")
                                                        .children_signal_vec(modal.usage.signal_ref(|usage| {
                                                            doms::square_bracket_to_span(&usage).collect()
                                                        }).to_signal_vec())
                                                    }))
                                                }
                                            })
                                        }))
                                    }),
                                ])
                                .apply_if(modal.is_form, |dom| dom
                                    .children([
                                        html!("div", {
                                            .class("col-sm-9")
                                            .child(html!("div", {
                                                .class(class::INPUT_GROUP)
                                                .children([
                                                    html!("label", {
                                                        .class(class::INPUT_GROUP_TEXT_BG_GOLDS)
                                                        .attr("for", "status")
                                                        .text("เปิดใช้งาน DUE เมื่อเลือกใช้ยา")
                                                    }),
                                                    html!("select" => HtmlSelectElement, {
                                                        .class("form-select")
                                                        .attr("id", "status")
                                                        .children([
                                                            html!("option", {.attr("value", "").text("เลือก")}),
                                                            html!("option", {.attr("value", "Y").text("ใช้งาน")}),
                                                            html!("option", {.attr("value", "N").text("ไม่ใช้งาน")}),
                                                        ])
                                                        .apply(mixins::string_value_select(modal.status.clone(), modal.changed.clone()))
                                                    }),
                                                    html!("span", {
                                                        .class("input-group-text")
                                                        .text("หากเปิดใช้งานร่วมกับ Info จะแสดงเฉพาะ DUE (ไม่แสดง Info)")
                                                    }),
                                                ])
                                            }))
                                        }),
                                        html!("hr"),
                                    ])
                                )
                                .children([
                                    html!("div", {
                                        .class(class::ROW_MY2)
                                        .child(html!("label", {
                                            .class(class::COL_SM12_BOLD)
                                            .attr("for","monitor")
                                            .text("การบริหารยา การติดตามอาการ และการแก้ปัญหาเบื้องต้น (Monitor)")
                                        }))
                                    }),
                                    html!("div", {
                                        .class(class::ROW_MY2)
                                        .child(html!("div", {
                                            .class("col-sm-12")
                                            .apply(|dom| {
                                                if modal.is_form { dom
                                                    .child(html!("textarea" => HtmlTextAreaElement, {
                                                        .class("form-control")
                                                        .attr("id", "monitor")
                                                        .attr("rows", "4")
                                                        .apply(mixins::textarea_value_auto_expand(modal.monitor.clone(), modal.changed.clone()))
                                                    }))
                                                } else { dom
                                                    .child(html!("div", {
                                                        .class(class::BOX_ROUND)
                                                        .style("white-space","pre-wrap")
                                                        .style("min-height","50px")
                                                        .children_signal_vec(modal.monitor.signal_ref(|monitor| {
                                                            doms::square_bracket_to_span(&monitor).collect()
                                                        }).to_signal_vec())
                                                    }))
                                                }
                                            })
                                        }))
                                    }),
                                ])
                                .apply_if(modal.is_form, |dom| dom
                                    .children([
                                        html!("div", {
                                            .class(class::ROW_MY2)
                                            .children([
                                                html!("div", {
                                                    .class("col-sm-6")
                                                    .child(html!("div", {
                                                        .class(class::INPUT_GROUP)
                                                        .children([
                                                            doms::label_group_for("monitor_count", "จำนวนการติดตามอาการหลังได้รับยา อย่างน้อย"),
                                                            html!("input" => HtmlInputElement, {
                                                                .class("form-control")
                                                                .attr("id","monitor_count")
                                                                .attr("type","number")
                                                                .attr("min","0")
                                                                .attr("max","255")
                                                                .apply(mixins::string_value(modal.monitor_count.clone(), modal.changed.clone()))
                                                            }),
                                                            html!("span", {
                                                                .class("input-group-text")
                                                                .text("ครั้ง")
                                                            }),
                                                        ])
                                                    }))
                                                }),
                                                html!("div", {
                                                    .class("col-sm-6")
                                                    .child(html!("div", {
                                                        .class(class::INPUT_GROUP)
                                                        .children([
                                                            doms::label_group_for("monitor_duration", "ควรได้รับการติดตามเป็นเวลา อย่างน้อย"),
                                                            html!("input" => HtmlInputElement, {
                                                                .class("form-control")
                                                                .attr("id","monitor_duration")
                                                                .attr("type","number")
                                                                .attr("min","0")
                                                                .attr("max","65535")
                                                                .apply(mixins::string_value(modal.monitor_duration.clone(), modal.changed.clone()))
                                                            }),
                                                            html!("span", {
                                                                .class("input-group-text")
                                                                .text("นาที")
                                                            }),
                                                        ])
                                                    }))
                                                }),
                                            ])
                                        }),
                                        html!("div", {
                                            .class("col-sm-4")
                                            .child(html!("div", {
                                                .class(class::INPUT_GROUP)
                                                .children([
                                                    html!("label", {
                                                        .class(class::INPUT_GROUP_TEXT_BG_REDS)
                                                        .attr("for", "monitor_status")
                                                        .text("เปิดใช้งาน Monitor หลัง Action")
                                                    }),
                                                    html!("select" => HtmlSelectElement, {
                                                        .class("form-select")
                                                        .attr("id", "monitor_status")
                                                        .children([
                                                            html!("option", {.attr("value", "").text("เลือก")}),
                                                            html!("option", {.attr("value", "Y").text("ใช้งาน")}),
                                                            html!("option", {.attr("value", "N").text("ไม่ใช้งาน")}),
                                                        ])
                                                        .apply(mixins::string_value_select(modal.monitor_status.clone(), modal.changed.clone()))
                                                    }),
                                                ])
                                            }))
                                        }),
                                    ])
                                )
                            }))
                        }),
                        html!("div", {
                            .class("modal-footer")
                            .apply_if(modal.is_form, |dom| dom
                                .child(html!("button" => HtmlButtonElement, {
                                    .attr("type", "button")
                                    .class("btn")
                                    .class_signal("btn-primary", modal.changed.signal())
                                    .class_signal("btn-secondary", not(modal.changed.signal()))
                                    .text("บันทึก")
                                    .apply(mixins::click_with_loader_checked_or_true_disable_signal(clone!(app, modal => move || {
                                        Self::save(modal.clone(), app.clone());
                                    }), modal.not_ready_signal(), app.state()))
                                }))
                            )
                            .child(html!("button", {
                                .attr("type", "button")
                                .attr("data-bs-dismiss", "modal")
                                .class(class::BTN_GRAY)
                                .child(html!("i", {.class(class::FA_X)}))
                                .text(" ปิด")
                                .event(clone!(modal, parent_reload => move |_: events::Click| {
                                    if let Some(reload) = &parent_reload {
                                        reload.set_neq(modal.parent_need_reload.get());
                                    }
                                    display.set(None);
                                }))
                            }))
                        }),
                    ])
                }),
            ])
        })
    }

    fn render_med_searchbox(modal: Rc<Self>, app: Rc<App>) -> Dom {
        html!("div", {
            .child(html!("div", {
                .class(class::CARD_TW_T_CYANS)
                .children([
                    html!("div", {
                        .class(class::INPUT_GROUP_T)
                        .children([
                            html!("span", {
                                .class("input-group-text")
                                .child(html!("i", {.class(class::FA_SEARCH)}))
                            }),
                            html!("input" => HtmlInputElement, {
                                .attr("type", "text")
                                .class(class::FORM_CTRL_SM)
                                .attr("placeholder", "กรอกข้อความเพื่อค้นหา")
                                .attr("autocomplete", "off")
                                .focused(true)
                                .with_node!(element => {
                                    .apply(|dom| {
                                        let load_mut = Mutable::new(false);
                                        dom.event(clone!(modal, load_mut => move |_: events::Input| {
                                            let search_text = element.value();
                                            if search_text.is_empty() {
                                                modal.results.lock_mut().clear();
                                                modal.status_text.set_neq(Some(String::from("กรอกข้อความเพื่อค้นหา")));
                                            } else {
                                                modal.search_text.set_neq(search_text);
                                                load_mut.set(true);
                                            }
                                        }))
                                        .future(map_ref!{
                                            let busy = app.loader_is_loading(),
                                            let load = load_mut.signal() =>
                                            !busy && *load
                                        }.for_each(clone!(app, modal, load_mut => move |ready| {
                                            if ready {
                                                load_mut.set(false);
                                                Self::load_meds(modal.clone(), app.clone());
                                            }
                                            async {}
                                        })))
                                    })
                                })
                            }),
                            html!("button", {
                                .class(class::BTN_SM_R_RED)
                                .attr("type", "button")
                                .child(html!("i", {.class(class::FA_X)}))
                                .event(clone!(modal => move |_: events::Click| {
                                    modal.display_med_searchbox.set_neq(false);
                                }))
                            }),
                        ])
                    }),
                    html!("div", {
                        .style("height","250px")
                        .style("border-width","thin")
                        .style("overflow-y","auto")
                        .child(html!("table", {
                            .class(class::TABLE_STRIP)
                            .children([
                                html!("thead", {
                                    .visible(false)
                                    .child(html!("tr", {
                                        .child(html!("th", {
                                            .attr("scope", "col")
                                            .text("Name")
                                        }))
                                    }))
                                }),
                                html!("tbody", {
                                    .child_signal(modal.status_text.signal_cloned().map(|opt| {
                                        opt.as_ref().map(|text| {
                                            html!("tr", {
                                                .child(html!("td", {
                                                    .attr("colspan", "2")
                                                    .text(text)
                                                }))
                                            })
                                        })
                                    }))
                                    .children_signal_vec(modal.results.signal_vec_cloned().map(move |result| {
                                        html!("tr", {
                                            .child(html!("td", {
                                                .class("fw-bold")
                                                .style("cursor","pointer")
                                                .text(&result.med_name.clone().unwrap_or_default())
                                                .event(clone!(modal, result => move |_: events::Click| {
                                                    modal.med_name.set(result.med_name.clone().unwrap_or_default());
                                                    modal.icode.set(result.icode.clone());
                                                    modal.clear_med_searchbox();
                                                    modal.changed.set_neq(true);
                                                }))
                                            }))
                                        })
                                    }))
                                }),
                            ])
                        }))
                    }),
                ])
            }))
        })
    }
}
