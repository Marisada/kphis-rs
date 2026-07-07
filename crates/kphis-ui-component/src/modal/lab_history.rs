// ipd-nurse-lab-history.php

use chart_js_rs::{
    ChartExt, ChartInteraction, ChartOptions, ChartPlugins, ChartScale, Dataset, DatasetDataExt, DatasetIterExt, DisplayFormats, FnWithArgs, Grid, LegendLabel, PluginLegend, PluginZoom,
    ScaleAdapters, ScaleAdaptersDate, ScaleTicks, ScaleTime, TooltipCallbacks, TooltipPlugin, XYDataset, ZoomPan, ZoomPinchOptions, ZoomWheelOptions, ZoomZoom, scatter::Scatter,
};
use dominator::{Dom, clone, events, html};
use futures_signals::{
    map_ref,
    signal::{Mutable, SignalExt},
    signal_vec::{MutableVec, SignalVecExt},
};
use std::{collections::HashMap, rc::Rc, time::Duration};
use time::{Date, PrimitiveDateTime, Time};

use kphis_model::lab::{LabItem, LabItemParams};
use kphis_ui_app::App;
use kphis_ui_core::{class, doms};
use kphis_util::{
    datetime::{date_8601, date_th_opt, datetime_ts, js_now, time_hm_opt},
    util::{str_some, zero_none},
};

/// - GET `EndPoint::LabItem`
#[derive(Default)]
pub struct LabHistory {
    loaded: Mutable<bool>,

    hn: Mutable<String>,
    // vn: Mutable<String>,
    lab_items_code: Mutable<i32>,
    lab_items_name_ref: Mutable<String>,
    lab_items_unit: Mutable<String>,
    lab_order_number: Mutable<i32>,

    lab_items: MutableVec<Rc<LabItem>>,

    // chart
    chart_render: Mutable<bool>,
    min_date: Mutable<Option<Date>>,
    start_date: Mutable<String>,
    end_date: Mutable<String>,
    day_range: Mutable<u64>,
}

impl LabHistory {
    pub fn new(hn: Mutable<String>, lab_items_code: i32, lab_items_name_ref: &Option<String>, lab_items_unit: &Option<String>, lab_order_number: &Option<i32>) -> Rc<Self> {
        Rc::new(Self {
            hn,
            // vn: Mutable::new(vn.to_owned()),
            lab_items_code: Mutable::new(lab_items_code),
            lab_items_name_ref: Mutable::new(lab_items_name_ref.clone().unwrap_or_default()),
            lab_items_unit: Mutable::new(lab_items_unit.clone().unwrap_or_default()),
            lab_order_number: Mutable::new(lab_order_number.unwrap_or_default()),
            ..Default::default()
        })
    }

    fn set_day_range(&self, days: u64) {
        let now = js_now().date();
        if days == 0 {
            self.start_date.set_neq((self.min_date.get().unwrap_or(now) - Duration::new(24 * 3600, 0)).to_string());
        } else {
            self.start_date.set_neq((now - Duration::new(days * 24 * 3600, 0)).to_string());
        }
        self.end_date.set_neq(now.to_string());
        self.day_range.set(days);
        self.chart_render.set(false);
        self.chart_render.set(true);
    }

    fn load(modal: Rc<Self>, app: Rc<App>) {
        let params = LabItemParams {
            hn: str_some(modal.hn.get_cloned()),
            lab_items_code: zero_none(modal.lab_items_code.get()),
            ..Default::default()
        };
        app.async_load(
            true,
            clone!(app => async move {
                // GET `EndPoint::LabItem`
                match LabItem::call_api_get(&params, app.state()).await {
                    Ok(responses) => {
                        let min_date_opt = responses.iter().filter_map(|lab| lab.report_date).min();
                        modal.min_date.set(min_date_opt);
                        if let Some(min_date) = min_date_opt {
                            let day_range = (js_now().date() - min_date).whole_days().unsigned_abs();
                            if let Some(res) = [1u64, 3u64, 7u64, 15u64, 30u64, 92u64, 183u64, 365u64, 1095u64, 1826u64].iter().find(|i| day_range < **i) {
                                modal.set_day_range(*res);
                            } else {
                                modal.set_day_range(0);
                            }
                        } else {
                            modal.set_day_range(0);
                        }
                        modal.lab_items.lock_mut().extend(responses.into_iter().map(Rc::new));
                        modal.chart_render.set(true);
                    }
                    Err(e) => {
                        app.alert_app_error(&e).await;
                    }
                }
            }),
        )
    }

    pub fn render(modal: Rc<Self>, app: Rc<App>, parent_lab_order_number: Option<Mutable<i32>>) -> Dom {
        html!("div", {
            .future(map_ref!(
                let busy = app.loader_is_loading(),
                let loaded = modal.loaded.signal() =>
                !busy && !loaded
            ).for_each(clone!(app, modal => move |ready| {
                if ready {
                    Self::load(modal.clone(), app.clone());
                    modal.loaded.set_neq(true);
                }
                async {}
            })))
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
                                .child(html!("span", {
                                    .class("lab_history_lab_items_name")
                                    .text_signal(modal.lab_items_name_ref.signal_cloned())
                                    .text_signal(modal.lab_items_unit.signal_cloned().map(|unit| {
                                        if unit.is_empty() {String::new()} else {[" (", &unit, ")"].concat()}
                                    }))
                                }))
                            }),
                            doms::close_modal_x_btn(),
                        ])
                    }),
                    html!("div", {
                        .class("modal-body")
                        //.attr("id", "lab_history_modal_body")
                        .children([
                            html!("div", {
                                .class("pb-2")
                                .children([
                                    html!("div", {
                                        .class("pb-2")
                                        .child_signal(modal.chart_render.signal_cloned().map(clone!(modal => move |render| {
                                            render.then(|| render_chart(
                                                &modal.lab_items.lock_ref(),
                                                &modal.lab_items_name_ref.lock_ref(),
                                                &modal.lab_items_unit.lock_ref(),
                                                &modal.start_date.lock_ref(),
                                                &modal.end_date.lock_ref(),
                                            ))
                                        })))
                                    }),
                                    html!("div", {
                                        .class("text-center")
                                        .children([
                                            Self::render_range_button(modal.clone(), 0, "ทั้งหมด"),
                                            Self::render_range_button(modal.clone(), 1826, "5 ปี"),
                                            Self::render_range_button(modal.clone(), 1095, "3 ปี"),
                                            Self::render_range_button(modal.clone(), 365, "1 ปี"),
                                            Self::render_range_button(modal.clone(), 183, "6 เดือน"),
                                            Self::render_range_button(modal.clone(), 92, "3 เดือน"),
                                            Self::render_range_button(modal.clone(), 30, "30 วัน"),
                                            Self::render_range_button(modal.clone(), 15, "15 วัน"),
                                            Self::render_range_button(modal.clone(), 7, "7 วัน"),
                                            Self::render_range_button(modal.clone(), 3, "3 วัน"),
                                            Self::render_range_button(modal.clone(), 1, "24 ชั่วโมง"),
                                        ])
                                    })
                                ])
                            }),
                            html!("small", {
                                .class("mb-2")
                                .text("*ข้อมูลแสดงตามวันที่รายงานผลเป็นหลัก หากไม่มีข้อมูลจะแสดงวันที่รับและวันที่สั่งตามลำดับ")
                            }),
                            html!("div", {
                                .style("overflow-y","auto")
                                .style("max-height","25vh")
                                .child(html!("table", {
                                    .class(class::TABLE_STRIP)
                                    .children([
                                        html!("thead", {
                                            .child(html!("tr", {
                                                .children([
                                                    html!("th", {.attr("scope", "col").style("width","30%").text("วันที่รายงานผล")}),
                                                    html!("th", {.attr("scope", "col").style("width","30%").text("ผล")}),
                                                ])
                                            }))
                                        }),
                                        html!("tbody", {
                                            .class("lab_history_lab_items_table_tbody")
                                            .children_signal_vec(modal.lab_items.signal_vec_cloned().map(clone!(modal => move |item| {
                                                let (lab_datetime, lab_datetime_type) = if item.report_date.is_some() {
                                                    ([date_th_opt(&item.report_date), time_hm_opt(&item.report_time)].join(" "), "")
                                                } else if item.receive_date.is_some() {
                                                    ([date_th_opt(&item.receive_date), time_hm_opt(&item.receive_time)].join(" "), "เวลาที่รับ")
                                                } else if item.order_date.is_some() {
                                                    ([date_th_opt(&item.order_date), time_hm_opt(&item.order_time)].join(" "), "เวลาที่สั่ง")
                                                } else {
                                                    (String::new(), "")
                                                };
                                                html!("tr", {
                                                    .class_signal("bg-info-subtle", modal.lab_order_number.signal_cloned().map(clone!(item => move |num| num == item.lab_order_number.unwrap_or_default())))
                                                    .children([
                                                        html!("td", {
                                                            .children([
                                                                html!("span", {.text(&lab_datetime)}),
                                                                html!("badge", {
                                                                    .class(class::BADGE_FR_GRAY)
                                                                    .style("cursor","help")
                                                                    .attr("role","button")
                                                                    .attr("title", &[
                                                                        "เวลาที่รายงาน: ", &date_th_opt(&item.report_date), " ", &time_hm_opt(&item.report_time),
                                                                        "\nเวลาที่รับ: ", &date_th_opt(&item.receive_date), " ", &time_hm_opt(&item.receive_time),
                                                                        "\nเวลาที่สั่ง: ", &date_th_opt(&item.order_date), " ", &time_hm_opt(&item.order_time)
                                                                    ].concat())
                                                                    .children([
                                                                        html!("span", {
                                                                            .apply_if(!lab_datetime_type.is_empty(), |dom| dom.class("me-1"))
                                                                            .text(lab_datetime_type)
                                                                        }),
                                                                        html!("i", {.class(class::FA_INFO)}),
                                                                    ])
                                                                })
                                                            ])
                                                        }),
                                                        html!("td", {
                                                            .child(html!("span", {.text(&item.lab_order_result.clone().unwrap_or_default())}))
                                                            .apply_if(item.lab_order_remark.as_ref().map(|s| !s.is_empty()).unwrap_or_default(), |dom| {
                                                                dom.child(html!("div", {
                                                                    .children([
                                                                        html!("span", {.class("fw-bold").text("หมายเหตุ: ")}),
                                                                        html!("span", {.text(&item.lab_order_remark.clone().unwrap_or_default())}),
                                                                    ])
                                                                }))
                                                            })
                                                            .apply_if(parent_lab_order_number.is_some(), clone!(parent_lab_order_number => move |dom| {
                                                                dom.child(html!("button", {
                                                                    .attr("type", "button")
                                                                    .class(class::BTN_SM_FR_GRAY)
                                                                    .attr("data-bs-dismiss", "modal")
                                                                    .text("ดูผลอื่นๆ")
                                                                    .event(clone!(parent_lab_order_number => move |_:events::Click| {
                                                                        if let (Some(number), Some(parent)) = (item.lab_order_number, parent_lab_order_number.as_ref()) {
                                                                            parent.set_neq(number);
                                                                        }
                                                                    }))
                                                                }))
                                                            }))
                                                        }),
                                                    ])
                                                })
                                            })))
                                        }),
                                    ])
                                }))
                            }),
                        ])
                    }),
                    html!("div", {
                        .class("modal-footer")
                        .child(html!("button", {
                            .attr("type", "button")
                            .class(class::BTN_GRAY)
                            .attr("data-bs-dismiss", "modal")
                            .text("Close")
                        }))
                    }),
                ])
            }))
        })
    }

    fn render_range_button(modal: Rc<Self>, days: u64, label: &str) -> Dom {
        html!("button", {
            .attr("type", "button")
            .class(class::BTN_SM_L)
            .class_signal("btn-primary", modal.day_range.signal_cloned().map(move |r| r == days))
            .class_signal("btn-secondary", modal.day_range.signal_cloned().map(move |r| r != days))
            .text(label)
            .event(clone!(modal => move |_:events::Click| {
                modal.set_day_range(days);
            }))
        })
    }
}

fn render_chart(data: &[Rc<LabItem>], lab_items_name_ref: &str, lab_items_unit: &str, start_date: &str, end_date: &str) -> Dom {
    let day_ts = 24 * 3600 * 1000;
    let now = datetime_ts(&js_now());
    let start = date_8601(start_date).map(|d| datetime_ts(&PrimitiveDateTime::new(d, Time::MIDNIGHT))).unwrap_or(now);
    let end = date_8601(end_date).map(|d| datetime_ts(&PrimitiveDateTime::new(d, Time::MIDNIGHT))).unwrap_or(now);
    let ts_diff = end - start;

    let min = start;
    let max = end + day_ts;

    let time_unit = match ts_diff / day_ts {
        ..=3 => "hour",
        4..=30 => "day",
        31.. => "month",
    };

    let scales: HashMap<String, ChartScale> = HashMap::from([
        (
            String::from("x"),
            ChartScale::new()
                .scale_type("time")
                .adapters(ScaleAdapters::new().date(ScaleAdaptersDate::new().output_calendar("buddhist")))
                .time(
                    ScaleTime::new()
                        .unit(time_unit)
                        .tooltip_format("d MMM yyyy HH:mm")
                        .display_formats(DisplayFormats::new().minute("HH:mm").hour("(d MMM) HH:mm").day("d MMM")),
                )
                .position("top")
                .min(min)
                .max(max)
                .grid(Grid::new().color("green"))
                .ticks(ScaleTicks::new().color("green")),
        ),
        (String::from("y1"), ChartScale::new().grid(Grid::new().color("dodgerblue")).ticks(ScaleTicks::new().color("dodgerblue"))),
    ]);

    let dataset = data
        .iter()
        .filter_map(|lab| {
            lab.lab_order_result
                .as_ref()
                .and_then(|result| result.chars().filter(|c| c.is_ascii_digit() || c == &'.').collect::<String>().parse::<f32>().ok())
                .and_then(|y| {
                    if let Some(report_date) = lab.report_date {
                        Some((datetime_ts(&PrimitiveDateTime::new(report_date, lab.report_time.unwrap_or(Time::MIDNIGHT))), y))
                    } else {
                        lab.receive_date
                            .map(|receive_date| (datetime_ts(&PrimitiveDateTime::new(receive_date, lab.receive_time.unwrap_or(Time::MIDNIGHT))), y))
                    }
                })
        })
        .into_data_iter()
        .unsorted_to_dataset_data();

    let data = Dataset::new().datasets([XYDataset::new()
        .data(dataset)
        .y_axis_id("y1")
        .label([lab_items_name_ref, &(!lab_items_unit.is_empty()).then(|| [" (", lab_items_unit, ")"].concat()).unwrap_or_default()].concat())
        .background_color("dodgerblue")
        .border_color("dodgerblue")
        .point_radius(5)
        .point_hover_radius(5)
        .show_line(true)]);

    let options = ChartOptions::new()
        .locale("th")
        .span_gaps(true)
        .aspect_ratio(2)
        .maintain_aspect_ratio(true)
        .responsive(true)
        .interaction(ChartInteraction::new().mode("index").intersect(false))
        .plugins(
            ChartPlugins::new()
                .legend(PluginLegend::new().labels(LegendLabel::new().use_point_style(true)).position("bottom"))
                .tooltip(
                    TooltipPlugin::new()
                        .use_point_style(true)
                        .callbacks(TooltipCallbacks::new().title(FnWithArgs::new().args(["tooltipItems"]).rust_closure(|_ctx| "รายการ: (วันที่รายงานผล, ผล)".into()))),
                )
                .zoom(
                    PluginZoom::new().pan(ZoomPan::new().enabled(true).mode("xy")).zoom(
                        ZoomZoom::new()
                            .mode("xy")
                            .wheel(ZoomWheelOptions::new().enabled(true).speed(0.1))
                            .pinch(ZoomPinchOptions::new().enabled(true)),
                    ),
                ),
        )
        .scales(scales);

    let chart = Scatter::new("canvas").data(data).options(options);

    html!("canvas", {
        // id needed by chart-js
        .attr("id", "canvas")
        .class(class::ROUND_WHITE)
        .after_inserted(move |_| {
            chart.into_chart().render()
        })
    })
}
