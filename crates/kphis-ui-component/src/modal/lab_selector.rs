use dominator::{Dom, clone, events, html};
use futures_signals::{
    map_ref,
    signal::{Mutable, Signal, SignalExt, always},
    signal_vec::SignalVecExt,
};
use std::rc::Rc;

use kphis_model::{
    lab::{LabHead, LabHeadParams},
    patient_info::PatientInfo,
};
use kphis_ui_app::App;
use kphis_ui_core::{class, doms};
use kphis_util::{
    datetime::{date_8601, date_th_opt, js_now, time_hm_opt},
    util::{set_day_last, set_days_next},
};

use crate::lab::full_text;

/// GET `EndPoint::LabHead`
#[derive(Default)]
pub struct LabSelector {
    with_datetime: bool,
    patient: Mutable<Option<Rc<PatientInfo>>>,

    loaded: Mutable<bool>,
    changed: Mutable<bool>,

    start_lab_date: Mutable<String>,
    end_lab_date: Mutable<String>,
    lab_result: Mutable<Vec<Rc<LabHead>>>,
    checked: Mutable<bool>,

    parent_result: Mutable<String>,
    parent_changed: Mutable<bool>,
}

impl LabSelector {
    pub fn new(with_datetime: bool, patient: Mutable<Option<Rc<PatientInfo>>>, parent_result: Mutable<String>, parent_changed: Mutable<bool>) -> Rc<Self> {
        let now = js_now().date();
        Rc::new(Self {
            with_datetime,
            patient,
            start_lab_date: Mutable::new(now.previous_day().unwrap_or(now).to_string()),
            end_lab_date: Mutable::new(now.to_string()),
            parent_result,
            parent_changed,
            ..Default::default()
        })
    }

    fn is_ipd(&self) -> impl Signal<Item = bool> + use<> {
        self.patient.signal_cloned().map(|opt| opt.as_ref().map(|pt| pt.is_ipd()).unwrap_or_default())
    }

    fn set_day_last(&self, days: u64, until_now: bool) {
        if let Some(patient) = self.patient.lock_ref().as_ref() {
            let last_date = if until_now { Some(js_now().date()) } else { patient.lastdate() };
            set_day_last(patient.regdate(), last_date, self.start_lab_date.clone(), self.end_lab_date.clone(), self.changed.clone(), days);
        }
    }

    fn set_days_next(&self, forward: bool) {
        set_days_next(self.start_lab_date.clone(), self.end_lab_date.clone(), self.changed.clone(), forward);
    }

    fn load(modal: Rc<Self>, app: Rc<App>) {
        let hn = modal.patient.lock_ref().as_ref().and_then(|pt| pt.hn());
        if hn.is_some() {
            app.async_load(
                true,
                clone!(app, modal => async move {
                    let params = LabHeadParams {
                        hn,
                        start_date: date_8601(&modal.start_lab_date.lock_ref()),
                        end_date: date_8601(&modal.end_lab_date.lock_ref()),
                        ..Default::default()
                    };
                    // GET `EndPoint::LabHead`
                    match LabHead::call_api_get(&params, app.state()).await {
                        Ok(items) => {
                            modal.checked.set_neq(!items.is_empty());
                            modal.lab_result.set(items.into_iter().filter_map(|head| {
                                if head.confirm_report == Some(String::from("Y")) {
                                    Some(Rc::new(head))
                                } else {
                                    None
                                }
                            }).collect());
                        }
                        Err(e) => {
                            app.alert_app_error(&e).await;
                        }
                    }
                }),
            )
        }
    }

    pub fn render(modal: Rc<Self>, app: Rc<App>) -> Dom {
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
            .future(map_ref!{
                let busy = app.loader_is_loading(),
                let changed = modal.changed.signal() =>
                !busy && *changed
            }.for_each(clone!(app, modal => move |changed| {
                if changed {
                    Self::load(modal.clone(), app.clone());
                    modal.changed.set(false);
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
                            html!("h5", {.class("modal-title").text("เลือกผลการตรวจทางห้องปฏิบัติการ")}),
                            doms::close_modal_x_btn(),
                        ])
                    }),
                    html!("div", {
                        .class("modal-body")
                        //.style("height","400px")
                        .style("width", "100%")
                        .children([
                            html!("div", {
                                .class(class::FLEX_WRAP_T)
                                .children([
                                    html!("div", {
                                        .class(class::COLA_PY_L)
                                        .child(html!("div", {
                                            .class(class::INPUT_GROUP)
                                            .children([
                                                doms::label_group_for("display_vs_date_from","วันที่"),
                                                doms::date_picker(
                                                    modal.start_lab_date.clone(),
                                                    modal.changed.clone(), always(false), None,
                                                    |d| d.class(class::FLEX_GROW1).style("min-width","135px"),
                                                    |d| d.class("rounded-0"),
                                                    |d| d.class("rounded-0").attr("id", "display_vs_date_from"),
                                                    |s| s, always(None),
                                                ),
                                                doms::label_group_for("display_vs_date_to","ถึง"),
                                                doms::date_picker(
                                                    modal.end_lab_date.clone(),
                                                    modal.changed.clone(), always(false), None,
                                                    |d| d.class(class::FLEX_GROW1).style("min-width","135px"),
                                                    |d| d.class("rounded-start-0"),
                                                    |d| d.class("rounded-start-0").attr("id", "display_vs_date_to"),
                                                    |s| s, always(None),
                                                ),
                                            ])
                                        }))
                                    }),
                                    html!("div", {
                                        .class("py-1")
                                        .child(html!("button", {
                                            .attr("type", "button")
                                            .class(class::BTN_L_GRAY)
                                            .text("วันนี้")
                                            .event(clone!(modal => move |_: events::Click| {
                                                modal.set_day_last(1, true);
                                            }))
                                        }))
                                        .child_signal(modal.is_ipd().map(clone!(modal => move |is_ipd| (!is_ipd).then(|| {
                                            html!("button", {
                                                .attr("type", "button")
                                                .class(class::BTN_L_GRAY)
                                                .text("2 วัน")
                                                .event(clone!(modal => move |_: events::Click| {
                                                    modal.set_day_last(2, true);
                                                }))
                                            })
                                        }))))
                                        .child_signal(modal.is_ipd().map(clone!(modal => move |is_ipd| is_ipd.then(|| {
                                            html!("button", {
                                                .attr("type", "button")
                                                .class(class::BTN_L_GRAY)
                                                .text("3 วัน")
                                                .event(clone!(modal => move |_: events::Click| {
                                                    modal.set_day_last(3, true);
                                                }))
                                            })
                                        }))))
                                        .child_signal(modal.is_ipd().map(clone!(modal => move |is_ipd| is_ipd.then(|| {
                                            html!("button", {
                                                .attr("type", "button")
                                                .class(class::BTN_L_GRAY)
                                                .text("7 วัน")
                                                .event(clone!(modal => move |_: events::Click| {
                                                    modal.set_day_last(7, true);
                                                }))
                                            })
                                        }))))
                                        .children([
                                            html!("button", {
                                                .attr("type", "button")
                                                .class(class::BTN_L_GRAY)
                                                .text("ทั้งหมด")
                                                .event(clone!(modal => move |_: events::Click| {
                                                    modal.set_day_last(0, true);
                                                }))
                                            }),
                                            html!("button", {
                                                .attr("type", "button")
                                                .class(class::BTN_L_GRAY)
                                                .child(html!("i", {.class(class::FA_BACKWARD)}))
                                                .event(clone!(modal => move |_: events::Click| {
                                                    modal.set_days_next(false);
                                                }))
                                            }),
                                            html!("button", {
                                                .attr("type", "button")
                                                .class(class::BTN_L_GRAY)
                                                .child(html!("i", {.class(class::FA_FORWARD)}))
                                                .event(clone!(modal => move |_: events::Click| {
                                                    modal.set_days_next(true);
                                                }))
                                            }),
                                        ])
                                    }),
                                ])
                            }),
                            html!("div", {
                                .style("overflow-y","auto")
                                .style("max-height","50vh")
                                .child(doms::table_responsive(class::TABLE_STRIP, clone!(modal => move |table| { table
                                    .children([
                                        html!("thead", {
                                            .child(html!("tr", {
                                                .children([
                                                    html!("th", {.attr("scope", "col").text("วัน-เวลา")}),
                                                    html!("th", {.attr("scope", "col").text("ผลตรวจ")}),
                                                    html!("th", {.attr("scope", "col").text("รายละเอียด")}),
                                                ])
                                            }))
                                        }),
                                        html!("tbody", {
                                            .children_signal_vec(modal.lab_result.signal_cloned().to_signal_vec().map(move |row| {
                                                let lab_text = full_text(&row);
                                                html!("tr", {
                                                    .style("cursor","pointer")
                                                    .attr("data-bs-dismiss", "modal")
                                                    .children([
                                                        html!("td", {
                                                            .class("text-nowrap")
                                                            .text(&date_th_opt(&row.report_date))
                                                            .text(" ")
                                                            .text(&time_hm_opt(&row.report_time))
                                                            .attr("title", &["รายงานโดย: ", &row.reporter_name.clone().unwrap_or_default()].concat())
                                                        }),
                                                        html!("td", {.text(&row.lab_name_cc.clone().unwrap_or_default())}),
                                                        html!("td", {.text(&lab_text)}),
                                                    ])
                                                    .event(clone!(modal => move |_:events::Click| {
                                                        let old_text = modal.parent_result.get_cloned();
                                                        let spacer = match (old_text.is_empty(), modal.with_datetime) {
                                                            (true, true) => ["- [", &date_th_opt(&row.report_date.or(row.order_date)), " ", &time_hm_opt(&row.report_time.or(row.order_time)), "] "].concat(),
                                                            (true, false) => String::new(),
                                                            (false, true) => ["\r\n- [", &date_th_opt(&row.report_date.or(row.order_date)), " ", &time_hm_opt(&row.report_time.or(row.order_time)), "] "].concat(),
                                                            (false, false) => String::from(" "),
                                                        };
                                                        modal.parent_result.set([
                                                            old_text.as_str(), spacer.as_str(), &lab_text
                                                        ].concat());
                                                        modal.parent_changed.set(true);
                                                    }))
                                                })
                                            }))
                                        }),
                                    ])
                                })))
                            }),
                        ])
                    }),
                    html!("div", {
                        .class("modal-footer")
                        .child(html!("button", {
                            .attr("type", "button")
                            .class(class::BTN_GRAY)
                            .attr("data-bs-dismiss", "modal")
                            .text("ปิด")
                        }))
                    }),
                ])
            }))
        })
    }
}
