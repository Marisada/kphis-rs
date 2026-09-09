use dominator::{Dom, clone, events, html};
use futures_signals::{
    map_ref,
    signal::{Mutable, Signal, SignalExt, always},
};
use rust_decimal::{Decimal, prelude::Zero};
use std::{collections::HashMap, rc::Rc};
use time::Date;

use kphis_model::{
    app::VisitTypeId,
    ipd::io::{IoParams, IoShift},
    patient_info::PatientInfo,
    shift::NurseShift,
};
use kphis_ui_app::App;
use kphis_ui_core::{class, doms};
use kphis_util::{
    datetime::{date_8601, date_th, js_now},
    util::{decimal_rescale, set_day_last, set_days_next, str_some, thousands_without_dot, zero_none},
};

use crate::modal::modal_show_option_mixins;

#[derive(Clone, Default, PartialEq)]
enum IoDisplayMode {
    #[default]
    FullDay,
    ShiftDay,
    ShiftEvening,
    ShiftNight,
}

impl IoDisplayMode {
    fn label(&self) -> &'static str {
        match self {
            IoDisplayMode::FullDay => "เต็มวัน",
            IoDisplayMode::ShiftNight => "เวรดึก",
            IoDisplayMode::ShiftDay => "เวรเช้า",
            IoDisplayMode::ShiftEvening => "เวรบ่าย",
        }
    }

    fn shift(&self) -> &'static str {
        match self {
            IoDisplayMode::FullDay => "",
            IoDisplayMode::ShiftNight => " เวรดึก",
            IoDisplayMode::ShiftDay => " เวรเช้า",
            IoDisplayMode::ShiftEvening => " เวรบ่าย",
        }
    }

    fn is_shift(&self) -> bool {
        !matches!(self, IoDisplayMode::FullDay)
    }
}

/// - GET `EndPoint::IpdIo`
/// - GET `EndPoint::OpdErIo`
#[derive(Default)]
pub struct IoSelector {
    with_date: bool,
    patient: Mutable<Option<Rc<PatientInfo>>>,

    loaded: Mutable<bool>,
    changed: Mutable<bool>,

    display_mode: Mutable<IoDisplayMode>,

    start_io_date: Mutable<String>,
    end_io_date: Mutable<String>,
    io_result: Mutable<Vec<Rc<IoShift>>>,
    checked: Mutable<bool>,

    parent_result: Mutable<String>,
    parent_changed: Mutable<bool>,
}

impl IoSelector {
    pub fn new(with_date: bool, patient: Mutable<Option<Rc<PatientInfo>>>, parent_result: Mutable<String>, parent_changed: Mutable<bool>) -> Rc<Self> {
        let now = js_now().date();
        Rc::new(Self {
            with_date,
            patient,
            start_io_date: Mutable::new(now.previous_day().unwrap_or(now).to_string()),
            end_io_date: Mutable::new(now.to_string()),
            parent_result,
            parent_changed,
            ..Default::default()
        })
    }

    fn is_ipd(&self) -> impl Signal<Item = bool> + use<> {
        self.patient.signal_cloned().map(|opt| opt.as_ref().map(|pt| pt.is_ipd()).unwrap_or_default())
    }

    fn set_day_last(&self, days: u64, from_now: bool) {
        if let Some(patient) = self.patient.lock_ref().as_ref() {
            let last_date = if from_now { Some(js_now().date()) } else { patient.lastdate() };
            set_day_last(patient.regdate(), last_date, self.start_io_date.clone(), self.end_io_date.clone(), self.changed.clone(), days);
        }
    }

    fn set_days_next(&self, forward: bool) {
        set_days_next(self.start_io_date.clone(), self.end_io_date.clone(), self.changed.clone(), forward);
    }

    fn load(modal: Rc<Self>, app: Rc<App>) {
        let visit_type_opt = modal.patient.lock_ref().as_ref().map(|pt| pt.visit_type());
        if let Some(visit_type) = visit_type_opt {
            app.async_load(
                true,
                clone!(app, modal => async move {
                    match visit_type {
                        VisitTypeId::Ipd(an) | VisitTypeId::PreAdmit(an) => {
                            let params = IoParams {
                                an: str_some(&an),
                                start_date: date_8601(&modal.start_io_date.lock_ref()),
                                end_date: date_8601(&modal.end_io_date.lock_ref()),
                                ..Default::default()
                            };
                            // GET `EndPoint::IpdIo`
                            match IoShift::call_api_get_ipd(&params, app.state()).await {
                                Ok(items) => {
                                    modal.checked.set_neq(!items.is_empty());
                                    modal.io_result.set(items.into_iter().map(Rc::new).collect());
                                }
                                Err(e) => {
                                    app.alert_app_error(&e).await;
                                }
                            }
                        }
                        VisitTypeId::OpdEr(_, opd_er_order_master_id) => {
                            let params = IoParams {
                                opd_er_order_master_id: zero_none(opd_er_order_master_id),
                                start_date: date_8601(&modal.start_io_date.lock_ref()),
                                end_date: date_8601(&modal.end_io_date.lock_ref()),
                                ..Default::default()
                            };
                            // GET `EndPoint::OpdErIo`
                            match IoShift::call_api_get_opd_er(&params, app.state()).await {
                                Ok(items) => {
                                    modal.checked.set_neq(!items.is_empty());
                                    modal.io_result.set(items.into_iter().map(Rc::new).collect());
                                }
                                Err(e) => {
                                    app.alert_app_error(&e).await;
                                }
                            }
                        }
                        VisitTypeId::Visit(_) => {}
                    }
                }),
            )
        }
    }

    pub fn render_modal(modal: Rc<Self>, display: Mutable<Option<Rc<Self>>>, app: Rc<App>) -> Dom {
        html!("div", {
            .child(Self::render_dialog(modal, display.clone(), app.clone()))
            .apply(modal_show_option_mixins(display, app))
        })
    }

    fn render_dialog(modal: Rc<Self>, display: Mutable<Option<Rc<Self>>>, app: Rc<App>) -> Dom {
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
                            html!("h5", {.class("modal-title").text("เลือกสมดุลน้ำ (I/O)")}),
                            html!("button", {
                                .attr("type", "button")
                                .class("btn-close")
                                .attr("aria-label", "Close")
                                .event(clone!(app, display => move |_: events::Click| {
                                    app.clear_modal_backdrop();
                                    display.set(None);
                                }))
                            }),
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
                                                doms::label_group_for("display_io_date_from","วันที่"),
                                                doms::date_picker(
                                                    modal.start_io_date.clone(),
                                                    modal.changed.clone(), always(false), None,
                                                    |d| d.class(class::FLEX_GROW1).style("min-width","135px"),
                                                    |d| d.class("rounded-0"),
                                                    |d| d.class("rounded-0").attr("id", "display_vs_date_from"),
                                                    |s| s, always(None),
                                                ),
                                                doms::label_group_for("display_io_date_to","ถึง"),
                                                doms::date_picker(
                                                    modal.end_io_date.clone(),
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
                                    html!("div", {
                                        .class(class::COLA_PY_L)
                                        .child(io_mode_radio(modal.display_mode.clone()))
                                    }),
                                ])
                            }),
                            html!("div", {
                                .style("overflow-y","auto")
                                .style("max-height","50vh")
                                .child(doms::table_responsive(class::TABLE_STRIP, clone!(app, modal, display => move |table| { table
                                    .children([
                                        html!("thead", {
                                            .child(html!("tr", {
                                                .children([
                                                    html!("th", {.attr("scope", "col").text("วันที่")}),
                                                    html!("th", {.attr("scope", "col").text("รายละเอียด")}),
                                                ])
                                            }))
                                        }),
                                        html!("tbody", {
                                            .children_signal_vec(map_ref!(
                                                let display_mode = modal.display_mode.signal_cloned(),
                                                let io_result = modal.io_result.signal_cloned() =>
                                                IoSummary::generate(display_mode, io_result)
                                            ).map(clone!(app, modal, display => move |rows| {
                                                rows.into_iter().map(|row| {
                                                    html!("tr", {
                                                        .style("cursor","pointer")
                                                        .children([
                                                            html!("td", {
                                                                .class("text-nowrap")
                                                                .text(&date_th(&row.date))
                                                            }),
                                                            html!("td", {.text(&row.string())}),
                                                        ])
                                                        .event(clone!(app, modal, display => move |_:events::Click| {
                                                            let old_text = modal.parent_result.get_cloned();
                                                            let shift = modal.display_mode.lock_ref().shift();
                                                            let spacer = match (old_text.is_empty(), modal.with_date) {
                                                                (true, true) => ["- [", &date_th(&row.date), shift, "] "].concat(),
                                                                (true, false) => String::new(),
                                                                (false, true) => ["\r\n- [", &date_th(&row.date), shift, "] "].concat(),
                                                                (false, false) => String::from(" "),
                                                            };
                                                            modal.parent_result.set([
                                                                old_text, spacer, row.string()
                                                            ].concat());
                                                            modal.parent_changed.set(true);
                                                            app.clear_modal_backdrop();
                                                            display.set(None);
                                                        }))
                                                    })
                                                }).collect()
                                            })).to_signal_vec())
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
                            .child(html!("i", {.class(class::FA_X)}))
                            .text(" ปิด")
                            .event(move |_: events::Click| {
                                app.clear_modal_backdrop();
                                display.set(None);
                            })
                        }))
                    }),
                ])
            }))
        })
    }
}

struct IoSummary {
    date: Date,
    is_shift: bool,
    parenteral: Decimal,
    oral: i32,
    output: i32,
}

impl IoSummary {
    fn new(date: Date, is_shift: bool) -> Self {
        Self {
            date,
            is_shift,
            parenteral: Decimal::zero(),
            oral: 0,
            output: 0,
        }
    }

    fn add(&mut self, io: &Rc<IoShift>) {
        if let Some(parenteral_absorb) = io.io_parenteral_absorb {
            self.parenteral += parenteral_absorb;
        }
        if let Some(oral_absorb) = io.io_oral_absorb {
            self.oral += oral_absorb;
        }
        if let Some(output_amount) = io.io_output_amount {
            self.output += output_amount;
        }
    }

    fn generate(io_mode: &IoDisplayMode, io_list: &[Rc<IoShift>]) -> Vec<Self> {
        let is_shift = io_mode.is_shift();
        let mut map = io_list.iter().filter_map(|io| io.shift_date).map(|d| (d, Self::new(d, is_shift))).collect::<HashMap<Date, Self>>();
        for io in io_list {
            if let Some(shift_date) = io.shift_date {
                match io_mode {
                    IoDisplayMode::FullDay => {
                        map.get_mut(&shift_date).map(|res| res.add(io));
                    }
                    IoDisplayMode::ShiftNight => {
                        if io.shift == Some(NurseShift::Night) {
                            map.get_mut(&shift_date).map(|res| res.add(io));
                        }
                    }
                    IoDisplayMode::ShiftDay => {
                        if io.shift == Some(NurseShift::Day) {
                            map.get_mut(&shift_date).map(|res| res.add(io));
                        }
                    }
                    IoDisplayMode::ShiftEvening => {
                        if io.shift == Some(NurseShift::Evening) {
                            map.get_mut(&shift_date).map(|res| res.add(io));
                        }
                    }
                }
            }
        }
        let mut results = map.into_values().collect::<Vec<Self>>();
        results.sort_by(|a, b| b.date.cmp(&a.date));
        results
    }

    fn string(&self) -> String {
        let header = if self.is_shift { "I/O ในเวร: " } else { "I/O: " };
        [
            header,
            &thousands_without_dot(&decimal_rescale(Decimal::new(self.oral as i64, 0).saturating_add(self.parenteral), 0).to_string()),
            "/",
            &thousands_without_dot(&self.output.to_string()),
            " cc.",
        ]
        .concat()
    }
}

fn io_mode_radio(io_mode_mutable: Mutable<IoDisplayMode>) -> Dom {
    html!("div", {
        .class(class::INPUT_GROUP)
        .children([
            doms::span_group_text("ปริมาณรวม"),
            io_mode_btn(IoDisplayMode::FullDay, io_mode_mutable.clone()),
            io_mode_btn(IoDisplayMode::ShiftNight, io_mode_mutable.clone()),
            io_mode_btn(IoDisplayMode::ShiftDay, io_mode_mutable.clone()),
            io_mode_btn(IoDisplayMode::ShiftEvening, io_mode_mutable),
        ])
    })
}

fn io_mode_btn(io_mode: IoDisplayMode, io_mode_mutable: Mutable<IoDisplayMode>) -> Dom {
    html!("button", {
        .attr("type", "button")
        .class(class::BTN_BLUEO)
        .class_signal("active", io_mode_mutable.signal_ref(clone!(io_mode => move |t| io_mode.eq(t))))
        .text(io_mode.label())
        .event(move |_: events::Click| {
            io_mode_mutable.set(io_mode.clone());
        })
    })
}
