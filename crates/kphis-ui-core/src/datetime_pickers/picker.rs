use dominator::{Dom, clone, events, html, with_node};
use futures_signals::{
    map_ref,
    signal::{Mutable, Signal, SignalExt, not},
};
use std::rc::Rc;
use time::{Date, Duration, Month, PrimitiveDateTime, Time, Weekday};
use time_datepicker_core::{
    config::{
        PickerConfig,
        date_constraints::{DateConstraints, HasDateConstraints},
    },
    dialog_view_type::DialogViewType,
    utils::{should_display_next_button, should_display_previous_button},
    viewed_date::{ViewedDate, year_group_end, year_group_range, year_group_start},
};

use kphis_util::datetime::{JsTime, date_8601, datetime_8601, js_now, month_thai, month_thai_full, time_8601, weekday_thai};

use crate::class;

const DATEPICKER_ROOT: &str = "datepicker-root";
const DATEPICKER_BACKDROP: &str = "datepicker-backdrop";

const DATE_CONTAINER: &str = "datepicker-date-container";
const TIME_CONTAINER: &str = "datepicker-time-container";
const HOUR_CONTAINER: &str = "datepicker-hour-container";
const MINUTE_CONTAINER: &str = "datepicker-minute-container";

const HEADER: &str = "datepicker-header";
const BODY: &str = "datepicker-body";
const FOOTER: &str = "datepicker-footer";

const TITLE: &str = "datepicker-title";
const BUTTON: &str = "datepicker-button";
const PREVIOUS: &str = "datepicker-previous";
const NEXT: &str = "datepicker-next";
const CLOSE: &str = "datepicker-close";

const EMPTY: &str = "datepicker-empty";
const TODAY: &str = "datepicker-today";
const FINISH: &str = "datepicker-finish";

const HOUR: &str = "datepicker-hour";
const MINUTE: &str = "datepicker-minute";

const SELECTABLE: &str = "datepicker-selectable";
const SELECTED: &str = "datepicker-selected";
const UNAVAILABLE: &str = "datepicker-unavailable";
const GRID_HEADER: &str = "datepicker-grid-header";
const OTHER_MONTH: &str = "datepicker-other-month";

pub struct DatePicker<F: Fn(String) -> String + 'static> {
    /// DateTime or Date or Time
    with_date: bool,
    with_time: bool,

    /// external state, DateTime or Date
    date_mutable: Mutable<String>,
    changed_mutable: Mutable<bool>,

    /// update value function
    update_fn: F,

    /// external self
    container: Mutable<Option<Rc<Self>>>,

    /// value of the date that is selected, start with Mutable's value or Config initial's value
    selected_date: Mutable<Option<PrimitiveDateTime>>,
    selected_hour: Mutable<Option<u8>>,
    selected_minute: Mutable<Option<u8>>,

    /// viewed date, start with NOW
    viewed_date: Mutable<PrimitiveDateTime>,

    /// dialog type
    dialog_view_type: Mutable<DialogViewType>,

    /// configuration of the picker, should be passed in during init and not modified later
    config: PickerConfig<DateConstraints>,
}

impl<F: Fn(String) -> String + 'static> DatePicker<F> {
    pub fn new_date(
        date_mutable: Mutable<String>,
        changed_mutable: Mutable<bool>,
        paired_mutable: Option<Mutable<String>>,
        container: Mutable<Option<Rc<Self>>>,
        update_fn: F,
        config: PickerConfig<DateConstraints>,
    ) -> Rc<Self> {
        let default_datetime = config.guess_allowed_year_month();
        let view_type_adjusted = match config.selection_type() {
            DialogViewType::Days => default_datetime,
            DialogViewType::Months => PrimitiveDateTime::new(Date::from_calendar_date(default_datetime.year(), default_datetime.month(), 1).unwrap(), default_datetime.time()),
            DialogViewType::Years => PrimitiveDateTime::new(Date::from_calendar_date(default_datetime.year(), Month::January, 1).unwrap(), default_datetime.time()),
        };
        let viewed_date = paired_mutable
            .and_then(|time_paired| time_8601(&time_paired.lock_ref()))
            .map(|t| PrimitiveDateTime::new(default_datetime.date(), t))
            .unwrap_or(view_type_adjusted);
        Rc::new(Self {
            with_date: true,
            with_time: false,
            date_mutable,
            changed_mutable,
            update_fn,
            container,
            selected_date: Mutable::new(None),
            selected_hour: Mutable::new(None),
            selected_minute: Mutable::new(None),
            viewed_date: Mutable::new(viewed_date),
            dialog_view_type: Mutable::new(*config.initial_view_type()),
            config,
        })
    }

    pub fn new_time(
        time_mutable: Mutable<String>,
        changed_mutable: Mutable<bool>,
        paired_mutable: Option<Mutable<String>>,
        container: Mutable<Option<Rc<Self>>>,
        update_fn: F,
        config: PickerConfig<DateConstraints>,
    ) -> Rc<Self> {
        let default_datetime = config.guess_allowed_year_month() - Duration::days(1);
        let viewed_date = paired_mutable
            .and_then(|date_paired| date_8601(&date_paired.lock_ref()))
            .map(|d| PrimitiveDateTime::new(d, default_datetime.time()))
            .unwrap_or(default_datetime);
        Rc::new(Self {
            with_date: false,
            with_time: true,
            date_mutable: time_mutable,
            changed_mutable,
            update_fn,
            container,
            selected_date: Mutable::new(None),
            selected_hour: Mutable::new(None),
            selected_minute: Mutable::new(None),
            viewed_date: Mutable::new(viewed_date),
            dialog_view_type: Mutable::new(*config.initial_view_type()),
            config,
        })
    }

    pub fn new_datetime(date_mutable: Mutable<String>, changed_mutable: Mutable<bool>, container: Mutable<Option<Rc<Self>>>, update_fn: F, config: PickerConfig<DateConstraints>) -> Rc<Self> {
        let default_datetime = config.guess_allowed_year_month();
        let view_type_adjusted = match config.selection_type() {
            DialogViewType::Days => default_datetime,
            DialogViewType::Months => PrimitiveDateTime::new(Date::from_calendar_date(default_datetime.year(), default_datetime.month(), 1).unwrap(), default_datetime.time()),
            DialogViewType::Years => PrimitiveDateTime::new(Date::from_calendar_date(default_datetime.year(), Month::February, 1).unwrap(), default_datetime.time()),
        };
        Rc::new(Self {
            with_date: true,
            with_time: true,
            date_mutable,
            changed_mutable,
            update_fn,
            container,
            selected_date: Mutable::new(None),
            selected_hour: Mutable::new(None),
            selected_minute: Mutable::new(None),
            viewed_date: Mutable::new(view_type_adjusted),
            dialog_view_type: Mutable::new(*config.initial_view_type()),
            config,
        })
    }

    fn should_display_previous_button(picker: Rc<Self>) -> impl Signal<Item = bool> + use<F> {
        map_ref! {
            let viewed_date = picker.viewed_date.signal(),
            let dialog_view_type = picker.dialog_view_type.signal_cloned() =>
            (viewed_date.clone(), *dialog_view_type)
        }
        .map(clone!(picker => move |(viewed_date, dialog_view_type)| {
            should_display_previous_button(&dialog_view_type, &viewed_date, &picker.config)
        }))
    }

    fn should_display_next_button(picker: Rc<Self>) -> impl Signal<Item = bool> + use<F> {
        map_ref! {
            let viewed_date = picker.viewed_date.signal(),
            let dialog_view_type = picker.dialog_view_type.signal_cloned() =>
            (viewed_date.clone(), *dialog_view_type)
        }
        .map(clone!(picker => move |(viewed_date, dialog_view_type)| {
            should_display_next_button(&dialog_view_type, &viewed_date, &picker.config)
        }))
    }

    fn create_dialog_title_text(&self) -> impl Signal<Item = String> + use<F> {
        map_ref! {
            let viewed_date = self.viewed_date.signal(),
            let dialog_view_type = self.dialog_view_type.signal_cloned() =>
            create_dialog_title_text(dialog_view_type, &viewed_date.date())
        }
    }

    fn set_date(&self, display_date: PrimitiveDateTime, view_type: DialogViewType) {
        if let (Some(hour), Some(minute)) = (self.selected_hour.get(), self.selected_minute.get()) {
            // datetime mode, time selected
            let new_time = Time::from_hms(hour, minute, display_date.second()).unwrap();
            let new_date = match view_type {
                DialogViewType::Days => PrimitiveDateTime::new(display_date.date(), new_time),
                DialogViewType::Months => PrimitiveDateTime::new(Date::from_calendar_date(display_date.year(), display_date.month(), 1).unwrap(), new_time),
                DialogViewType::Years => PrimitiveDateTime::new(Date::from_calendar_date(display_date.year(), Month::January, 1).unwrap(), new_time),
            };
            self.viewed_date.set(new_date);
            self.selected_date.set(Some(new_date));
            // self.apply_update_fn_and_set_mutable(new_date.js_string());
            // self.container.set(None);
        } else {
            let new_date = match view_type {
                DialogViewType::Days => display_date,
                DialogViewType::Months => PrimitiveDateTime::new(Date::from_calendar_date(display_date.year(), display_date.month(), 1).unwrap(), display_date.time()),
                DialogViewType::Years => PrimitiveDateTime::new(Date::from_calendar_date(display_date.year(), Month::January, 1).unwrap(), display_date.time()),
            };
            if self.with_time {
                // datetime mode, time not selected
                self.viewed_date.set(new_date);
                self.selected_date.set(Some(new_date));
            } else {
                // date mode
                self.apply_update_fn_and_set_mutable(new_date.date().to_string());
                self.container.set(None);
            }
        }
    }

    pub fn render(picker: Rc<Self>) -> Dom {
        html!("div", {
            .children([
                html!("div", {
                    .future(picker.date_mutable.signal_cloned().for_each(clone!(picker => move |date_mutable| {
                        if picker.with_date && picker.with_time {
                            // datetime mode
                            let datetime_opt = datetime_8601(&date_mutable).or(*picker.config.initial_date());
                            picker.selected_date.set(datetime_opt);
                            picker.selected_hour.set(datetime_opt.map(|dt| dt.hour()));
                            picker.selected_minute.set(datetime_opt.map(|dt| dt.minute()));
                            if let Some(datetime) = datetime_opt {
                                picker.viewed_date.set(datetime);
                            }
                        } else if picker.with_date {
                            // date mode
                            let date_opt = date_8601(&date_mutable).or(picker.config.initial_date().map(|dt| dt.date())).map(|d| PrimitiveDateTime::new(d, Time::MIDNIGHT));
                            picker.selected_date.set(date_opt);
                            if let Some(date) = date_opt {
                                picker.viewed_date.set(date);
                            }
                        } else if picker.with_time {
                            // time mode
                            let time_opt = time_8601(&date_mutable).or(picker.config.initial_date().map(|dt| dt.time()));
                            picker.selected_hour.set(time_opt.map(|t| t.hour()));
                            picker.selected_minute.set(time_opt.map(|t| t.minute()));
                        }
                        async {}
                    })))
                    .class(DATEPICKER_ROOT)
                    .apply_if(picker.with_date, |dom| { dom
                        .child(html!("div", {
                            .class(DATE_CONTAINER)
                            .apply_if(picker.with_time, |dom| dom.style("margin-right", "8px"))
                            .child(Self::render_header(picker.clone()))
                            .child_signal(picker.dialog_view_type.signal_cloned().map(clone!(picker => move |dialog_view_type| {
                                Some(match dialog_view_type {
                                    DialogViewType::Days => Self::render_dialog_days(picker.clone()),
                                    DialogViewType::Months => Self::render_dialog_months(picker.clone()),
                                    DialogViewType::Years => Self::render_dialog_years(picker.clone()),
                                })
                            })))
                            .child(Self::render_date_footer(picker.clone()))
                        }))
                    })
                    .apply_if(picker.with_time, |dom| { dom
                        .child(html!("div", {
                            .class(TIME_CONTAINER)
                            .children([
                                html!("div", {
                                    .class(BODY)
                                    .children([
                                        Self::render_dialog_hours(picker.clone()),
                                        Self::render_dialog_minutes(picker.clone()),
                                    ])
                                }),
                                Self::render_time_footer(picker.clone()),
                            ])
                        }))
                    })
                }),
                html!("div", {
                    .class(DATEPICKER_BACKDROP)
                    .event(clone!(picker => move |_:events::Click| {
                        picker.exit();
                    }))
                }),
            ])
        })
    }

    fn render_header(picker: Rc<Self>) -> Dom {
        html!("div", {
            .class(HEADER)
            .children([
                html!("button", {
                    .attr("type", "button")
                    .class([BUTTON, PREVIOUS])
                    .style_signal("visibility", Self::should_display_previous_button(picker.clone()).map(|display| {
                        if display {
                            "visible"
                        } else {
                            "hidden"
                        }
                    }))
                    .child(html!("i", {.class(class::FA_L_ARROW)}))
                    .event(clone!(picker => move |_:events::Click| {
                        let current = picker.viewed_date.get();
                        let viewed_date = match *picker.dialog_view_type.lock_ref() {
                            DialogViewType::Days => current.previous_month(),
                            DialogViewType::Months => current.previous_year(),
                            DialogViewType::Years => current.previous_year_group(),
                        };
                        picker.viewed_date.set(viewed_date);
                    }))
                }),
                html!("span", {
                    .class(TITLE)
                    .attr("role", "heading")
                    .text_signal(picker.create_dialog_title_text())
                    .event(clone!(picker => move |_:events::Click| {
                        if let Some(new_dialog_type) = picker.dialog_view_type.get_cloned().larger_type() {
                            picker.dialog_view_type.set(new_dialog_type);
                        }
                    }))
                }),
                html!("button", {
                    .attr("type", "button")
                    .class([BUTTON, NEXT])
                    .style_signal("visibility", Self::should_display_next_button(picker.clone()).map(|display| {
                        if display {
                            "visible"
                        } else {
                            "hidden"
                        }
                    }))
                    .child(html!("i", {.class(class::FA_R_ARROW)}))
                    .event(clone!(picker => move |_:events::Click| {
                        let current = picker.viewed_date.get();
                        let viewed_date = match *picker.dialog_view_type.lock_ref() {
                            DialogViewType::Days => current.next_month(),
                            DialogViewType::Months => current.next_year(),
                            DialogViewType::Years => current.next_year_group(),
                        };
                        picker.viewed_date.set(viewed_date);
                    }))
                }),
            ])
            .apply_if(!picker.with_time, |dom| { dom
                .child(Self::render_exit(picker.clone()))
            })
        })
    }

    fn render_date_footer(picker: Rc<Self>) -> Dom {
        html!("div", {
            .class(FOOTER)
            .child(html!("button", {
                .attr("type", "button")
                .class([BUTTON, EMPTY])
                .style_signal("visibility", picker.selected_date.signal_cloned().map(|opt| {
                    if opt.is_some() {
                        "visible"
                    } else {
                        "hidden"
                    }
                }))
                .text("ล้างข้อมูล")
                .event(clone!(picker => move |_:events::Click| {
                    let empty_date = picker.date_mutable.lock_ref().is_empty();
                    if !empty_date {
                        picker.date_mutable.set(String::new());
                        picker.changed_mutable.set_neq(true);
                    }
                    picker.container.set(None);
                }))
            }))
            .apply(|dom| {
                if picker.with_time {
                    dom.child_signal(map_ref! {
                        let selected_hour = picker.selected_hour.signal(),
                        let selected_minute = picker.selected_minute.signal() =>
                        (*selected_hour, *selected_minute)
                    }.map(clone!(picker => move |(selected_hour_opt, selected_minute_opt)| {
                        if let (Some(selected_hour), Some(selected_minute)) = (selected_hour_opt, selected_minute_opt) {
                            // datetime mode with completed time, check to save
                            // selected_hour and selected_minute comes from defined valid value, cannot panic
                            let new_datetime = PrimitiveDateTime::new(js_now().date(), Time::from_hms(selected_hour, selected_minute, 0).unwrap());
                            if picker.config.is_datetime_forbidden(&new_datetime) {
                                None
                            } else {
                                Some(html!("button", {
                                    .attr("type", "button")
                                    .class([BUTTON, TODAY])
                                    .text("วันนี้")
                                    .event(clone!(picker => move |_:events::Click| {
                                        picker.apply_update_fn_and_set_mutable(new_datetime.js_string());
                                        picker.container.set(None);
                                    }))
                                }))
                            }
                        } else {
                            // datetime mode without time, wait for time
                            let new_datetime = js_now();
                            if picker.config.is_datetime_forbidden(&new_datetime) {
                                None
                            } else {
                                Some(html!("button", {
                                    .attr("type", "button")
                                    .class([BUTTON, TODAY])
                                    .text("วันนี้")
                                    .event(clone!(picker => move |_:events::Click| {
                                        picker.selected_date.set(Some(new_datetime));
                                        picker.viewed_date.set(new_datetime);
                                    }))
                                }))
                            }
                        }
                    })))
                } else {
                    // date mode
                    let new_datetime = js_now();
                    if picker.config.is_day_forbidden(&new_datetime) {
                        dom
                    } else {
                        dom.child(html!("button", {
                            .attr("type", "button")
                            .class([BUTTON, TODAY])
                            .text("วันนี้")
                            .event(clone!(picker => move |_:events::Click| {
                                picker.apply_update_fn_and_set_mutable(new_datetime.date().to_string());
                                picker.container.set(None);
                            }))
                        }))
                    }
                }
            })
            .child(html!("button", {
                .attr("type", "button")
                .class([BUTTON, FINISH])
                .apply(clone!(picker => move |dom| {
                    if picker.with_time {
                        dom.style_signal("visibility", map_ref! {
                            let selected_date_opt = picker.selected_date.signal(),
                            let selected_hour_opt = picker.selected_hour.signal(),
                            let selected_minute_opt = picker.selected_minute.signal() =>
                            if let (Some(selected_date), Some(selected_hour), Some(selected_minute)) = (selected_date_opt, selected_hour_opt, selected_minute_opt) {
                                !picker.config.is_datetime_forbidden(&PrimitiveDateTime::new(selected_date.date(), Time::from_hms(*selected_hour, *selected_minute, 0).unwrap()))
                            } else {
                                false
                            }
                        }.map(|ready| {
                            if ready {
                                "visible"
                            } else {
                                "hidden"
                            }
                        }))
                    } else {
                        dom.style_signal("visibility", picker.selected_date.signal().map(clone!(picker => move |selected_date| {
                            if selected_date.map(|dt| !picker.config.is_day_forbidden(&dt)).unwrap_or_default()  {
                                "visible"
                            } else {
                                "hidden"
                            }
                        })))
                    }
                }))
                .text("บันทึก")
                .event(clone!(picker => move |_:events::Click| {
                    if picker.with_time {
                        if let (Some(selected_date), Some(selected_hour), Some(selected_minute)) = (picker.selected_date.get(), picker.selected_hour.get(), picker.selected_minute.get()) {
                            let new_datetime = PrimitiveDateTime::new(selected_date.date(), Time::from_hms(selected_hour, selected_minute, 0).unwrap());
                            picker.apply_update_fn_and_set_mutable(new_datetime.js_string());
                            picker.container.set(None);
                        }
                    } else if let Some(new_datetime) = picker.selected_date.get() {
                        picker.apply_update_fn_and_set_mutable(new_datetime.date().to_string());
                        picker.container.set(None);
                    }
                }))
            }))
        })
    }

    fn render_time_footer(picker: Rc<Self>) -> Dom {
        html!("div", {
            .class(FOOTER)
            .apply(|dom| {
                let now = js_now();
                if picker.with_date {
                    // datetime mode
                    dom.child_signal(picker.selected_date.signal_cloned().map(clone!(picker => move |selected_date_opt| {
                        if let Some(selected_date) = selected_date_opt {
                            let new_datetime = PrimitiveDateTime::new(selected_date.date(), now.time());
                            if picker.config.is_datetime_forbidden(&new_datetime) {
                                None
                            } else {
                                Some(html!("button", {
                                    .attr("type", "button")
                                    .class([BUTTON, TODAY])
                                    .text("เวลานี้")
                                    .event(clone!(picker => move |_:events::Click| {
                                        picker.apply_update_fn_and_set_mutable(new_datetime.js_string());
                                        picker.container.set(None);
                                    }))
                                }))
                            }
                        } else {
                            Some(html!("button", {
                                .attr("type", "button")
                                .class([BUTTON, TODAY])
                                .text("เวลานี้")
                                .event(clone!(picker => move |_:events::Click| {
                                    picker.selected_hour.set_neq(Some(now.hour()));
                                    picker.selected_minute.set_neq(Some(now.minute()));
                                }))
                            }))
                        }
                    })))
                } else {
                    // time mode
                    dom.child_signal(map_ref!{
                        let selected_hour = picker.selected_hour.signal(),
                        let selected_minute = picker.selected_minute.signal() =>
                        (*selected_hour, *selected_minute)
                    }.map(clone!(picker => move |(selected_hour_opt, selected_minute_opt)| {
                        if let (Some(selected_hour), Some(selected_minute)) = (selected_hour_opt, selected_minute_opt) && selected_hour == now.hour() && selected_minute == now.minute() {
                            Some(html!("button", {
                                .attr("type", "button")
                                .class([BUTTON, EMPTY])
                                .text("ล้างข้อมูล")
                                .event(clone!(picker => move |_:events::Click| {
                                    picker.set_neq_mutable(String::new());
                                    picker.container.set(None);
                                }))
                            }))
                        } else {
                            Some(html!("button", {
                                .attr("type", "button")
                                .class([BUTTON, TODAY])
                                .text("เวลานี้")
                                .event(clone!(picker => move |_:events::Click| {
                                    picker.apply_update_fn_and_set_mutable(now.time().js_string());
                                    picker.container.set(None);
                                }))
                            }))
                        }
                    })))
                }
            })
        })
    }

    fn render_dialog_years(picker: Rc<Self>) -> Dom {
        html!("div", {
            .class(BODY)
            .style("grid-template-columns", "1fr ".repeat(4))
            .children_signal_vec(picker.viewed_date.signal_cloned().map(clone!(picker => move |d| {
                year_group_range(d.year()).map(|y| {
                    Self::render_year_cell(PrimitiveDateTime::new(Date::from_calendar_date(y, d.month(), d.day()).unwrap(), d.time()), picker.clone())
                }).collect::<Vec<Dom>>()
            })).to_signal_vec())
        })
    }

    fn render_year_cell(display_year: PrimitiveDateTime, picker: Rc<Self>) -> Dom {
        let is_year_forbidden = picker.config.is_year_forbidden(&display_year);
        html!("span", {
            .text(&(display_year.year() + 543).to_string())
            .attr("role", "gridcell")
            .class_signal(SELECTED, picker.selected_date.signal_cloned().map(move |opt| {
                opt.map_or(false, |optval| optval.year() == display_year.year())
            }))
            .prop_signal("aria-selected", picker.selected_date.signal_cloned().map(move |opt| {
                if opt.map_or(false, |optval| optval.year() == display_year.year()) {"true"} else {"false"}
            }))
            .class(if is_year_forbidden {
                UNAVAILABLE
            } else {
                SELECTABLE
            })
            .event(clone!(picker => move |_:events::Click| {
                if picker.config.selection_type() == &DialogViewType::Years {
                    picker.set_date(display_year, DialogViewType::Years);
                } else {
                    picker.viewed_date.set(display_year);
                    picker.dialog_view_type.set(DialogViewType::Months);
                }
            }))
        })
    }

    fn render_dialog_months(picker: Rc<Self>) -> Dom {
        html!("div", {
            .class(BODY)
            .style("grid-template-columns", "1fr ".repeat(3))
            .children_signal_vec(picker.viewed_date.signal_cloned().map(clone!(picker => move |d| {
                (1..=12u8).map(|m| {
                    // date from pre-defined month, cannot panic
                    let new_month = PrimitiveDateTime::new(Date::from_calendar_date(d.year(), Month::try_from(m).unwrap(), 1).unwrap(), d.time());
                    Self::render_month_cell(new_month, picker.clone())
                }).collect::<Vec<Dom>>()
            })).to_signal_vec())
        })
    }

    fn render_month_cell(display_month: PrimitiveDateTime, picker: Rc<Self>) -> Dom {
        let is_month_forbidden = picker.config.is_month_forbidden(&display_month);
        html!("span", {
            .text(&month_thai(&display_month.month()))
            .attr("role", "gridcell")
            .class_signal(SELECTED, picker.selected_date.signal_cloned().map(move |opt| {
                opt.map_or(false, |optval| optval.month() == display_month.month())
            }))
            .prop_signal("aria-selected", picker.selected_date.signal_cloned().map(move |opt| {
                if opt.map_or(false, |optval| optval.month() == display_month.month()) {"true"} else {"false"}
            }))
            .class(if is_month_forbidden {
                UNAVAILABLE
            } else {
                SELECTABLE
            })
            .event(clone!(picker => move |_:events::Click| {
                if picker.config.selection_type() == &DialogViewType::Months {
                    picker.set_date(display_month, DialogViewType::Months);
                } else {
                    picker.viewed_date.set(display_month);
                    picker.dialog_view_type.set(DialogViewType::Days);
                }
            }))
        })
    }

    fn render_dialog_days(picker: Rc<Self>) -> Dom {
        html!("div", {
            .class(BODY)
            .style("grid-template-columns", "1fr ".repeat(7))
            .children([
                render_weekday_name(Weekday::Sunday),
                render_weekday_name(Weekday::Monday),
                render_weekday_name(Weekday::Tuesday),
                render_weekday_name(Weekday::Wednesday),
                render_weekday_name(Weekday::Thursday),
                render_weekday_name(Weekday::Friday),
                render_weekday_name(Weekday::Saturday),
            ])
            .children_signal_vec(picker.viewed_date.signal_cloned().map(clone!(picker => move |d| {
                let first_day_of_month = d.first_day_of_month();
                let offset = first_day_of_month.weekday().number_days_from_sunday();
                let first_day_of_calendar = first_day_of_month - Duration::days(offset as i64);
                first_day_of_calendar.dates_fill_calendar(offset).iter().map(|d| {
                    Self::render_day_cell(*d, picker.clone())
                }).collect::<Vec<Dom>>()
            })).to_signal_vec())
        })
    }

    fn render_day_cell(display_day: PrimitiveDateTime, picker: Rc<Self>) -> Dom {
        let is_day_forbidden = picker.config.is_day_forbidden(&display_day);
        html!("span", {
            .text(&display_day.day().to_string())
            .attr("role", "gridcell")
            .class_signal(OTHER_MONTH, picker.viewed_date.signal_cloned().map(move |viewed_date| viewed_date.month() != display_day.month()))
            .class_signal(SELECTED, picker.selected_date.signal_cloned().map(move |opt| {
                opt.map_or(false, |optval| optval.date() == display_day.date())
            }))
            .prop_signal("aria-selected", picker.selected_date.signal_cloned().map(move |opt| {
                if opt.map_or(false, |optval| optval.date() == display_day.date()) {"true"} else {"false"}
            }))
            .class(if is_day_forbidden {
                UNAVAILABLE
            } else {
                SELECTABLE
            })
            .event(clone!(picker => move |_:events::Click| {
                picker.set_date(display_day, DialogViewType::Days);
            }))
        })
    }

    fn render_dialog_hours(picker: Rc<Self>) -> Dom {
        html!("div", {
            .class(HOUR_CONTAINER)
            .apply_if(picker.with_date, |dom| { dom
                .style("border-left", "1px solid var(--bs-border-color)")
            })
            .child(html!("div", {
                .class(HOUR)
                .style("grid-template-columns", "1fr")
                .children((0..=23u8).map(|h| {
                    Self::render_hour_cell(h, picker.clone())
                }))
            }))
            .with_node!(element => {
                .future(map_ref! {
                    let viewed_date = picker.viewed_date.signal(),
                    let selected_hour = picker.selected_hour.signal() =>
                    selected_hour.unwrap_or(viewed_date.hour())
                }.for_each(clone!(element => move |hour| {
                    // gap 3px, padding-top 3px, padding-bottom 3px
                    element.set_scroll_top(hour as i32 * 30);
                    async {}
                })))
            })
        })
    }

    fn is_hour_forbidden_signal(picker: Rc<Self>, display_hour: u8) -> impl Signal<Item = bool> + use<F> {
        picker.viewed_date.signal_cloned().map(move |dt| {
            let min = PrimitiveDateTime::new(dt.date(), Time::from_hms(display_hour, 0, 0).unwrap());
            let max = PrimitiveDateTime::new(dt.date(), Time::from_hms(display_hour, 59, 59).unwrap());
            picker.config.is_datetime_forbidden(&min) && picker.config.is_datetime_forbidden(&max)
        })
    }

    fn render_hour_cell(display_hour: u8, picker: Rc<Self>) -> Dom {
        html!("span", {
            .text(&display_hour.to_string())
            .class_signal(SELECTED, picker.selected_hour.signal_cloned().map(move |opt| opt.map_or(false, |selected_hour| selected_hour == display_hour)))
            .attr("role", "gridcell")
            .prop_signal("aria-selected", picker.selected_hour.signal_cloned().map(move |opt| {
                if opt.map_or(false, |selected_hour| selected_hour == display_hour) {"true"} else {"false"}
            }))
            .class_signal(UNAVAILABLE, Self::is_hour_forbidden_signal(picker.clone(), display_hour))
            .class_signal(SELECTABLE, not(Self::is_hour_forbidden_signal(picker.clone(), display_hour)))
            .event(clone!(picker => move |_:events::Click| {
                picker.selected_hour.set(Some(display_hour));
                if picker.with_date {
                    let viewed_date = picker.viewed_date.get();
                    // display_hour was pre-defined, cannot panic
                    let new_date = PrimitiveDateTime::new(viewed_date.date(), Time::from_hms(display_hour, viewed_date.minute(), viewed_date.second()).unwrap());
                    picker.viewed_date.set(new_date);
                    picker.selected_date.set(Some(new_date));
                } else if let Some(selected_minute) = picker.selected_minute.get() {
                    let new_time = Time::from_hms(display_hour, selected_minute, 0).unwrap();
                    picker.apply_update_fn_and_set_mutable(new_time.js_string());
                    picker.container.set(None);
                }
            }))
        })
    }

    fn render_dialog_minutes(picker: Rc<Self>) -> Dom {
        html!("div", {
            .class(MINUTE_CONTAINER)
            .style("border-left", "1px solid var(--bs-border-color)")
            .child(html!("div", {
                .class(MINUTE)
                .style("grid-template-columns", "1fr")
                .children((0..=59u8).map(|m| {
                    Self::render_minute_cell(m, picker.clone())
                }))
            }))
            .with_node!(element => {
                .future(map_ref! {
                    let viewed_date = picker.viewed_date.signal(),
                    let selected_minute = picker.selected_minute.signal() =>
                    selected_minute.unwrap_or(viewed_date.minute())
                }.for_each(clone!(element => move |minute| {
                    // gap 3px, padding-top 3px, padding-bottom 3px
                    element.set_scroll_top(minute as i32 * 30);
                    async {}
                })))
            })
        })
    }

    fn is_minute_forbidden_signal(picker: Rc<Self>, display_minute: u8) -> impl Signal<Item = bool> + use<F> {
        picker.viewed_date.signal_cloned().map(move |dt| {
            let min = PrimitiveDateTime::new(dt.date(), Time::from_hms(dt.hour(), display_minute, 0).unwrap());
            let max = PrimitiveDateTime::new(dt.date(), Time::from_hms(dt.hour(), display_minute, 59).unwrap());
            picker.config.is_datetime_forbidden(&min) && picker.config.is_datetime_forbidden(&max)
        })
    }

    fn render_minute_cell(display_minute: u8, picker: Rc<Self>) -> Dom {
        html!("span", {
            .text(&display_minute.to_string())
            .class_signal(SELECTED, picker.selected_minute.signal_cloned().map(move |opt| opt.map_or(false, |selected_minute| selected_minute == display_minute)))
            .attr("role", "gridcell")
            .prop_signal("aria-selected", picker.selected_hour.signal_cloned().map(move |opt| {
                if opt.map_or(false, |selected_hour| selected_hour == display_minute) {"true"} else {"false"}
            }))
            .class_signal(UNAVAILABLE, Self::is_minute_forbidden_signal(picker.clone(), display_minute))
            .class_signal(SELECTABLE, not(Self::is_minute_forbidden_signal(picker.clone(), display_minute)))
            .event(clone!(picker => move |_:events::Click| {
                picker.selected_minute.set(Some(display_minute));
                if picker.with_date {
                    let viewed_date = picker.viewed_date.get();
                    // display_hour was pre-defined, cannot panic
                    let new_date = PrimitiveDateTime::new(viewed_date.date(), Time::from_hms(viewed_date.hour(), display_minute, viewed_date.second()).unwrap());
                    picker.viewed_date.set(new_date);
                    picker.selected_date.set(Some(new_date));
                    if picker.selected_hour.get().is_some() {
                        picker.apply_update_fn_and_set_mutable(new_date.js_string());
                        picker.container.set(None);
                    }
                } else if let Some(selected_hour) = picker.selected_hour.get() {
                    let new_time = Time::from_hms(selected_hour, display_minute, 0).unwrap();
                    picker.apply_update_fn_and_set_mutable(new_time.js_string());
                    picker.container.set(None);
                }
            }))
        })
    }

    fn render_exit(picker: Rc<Self>) -> Dom {
        html!("button", {
            .attr("type", "button")
            .class([BUTTON, CLOSE])
            .apply(|dom| {
                if picker.with_time { dom
                    .text("บันทึก")
                } else { dom
                    .child(html!("i", {.class(class::FA_X)}))
                }
            })
            .event(clone!(picker => move |_:events::Click| {
                picker.exit();
            }))
        })
    }

    fn exit(&self) {
        if self.with_date {
            if let Some(selected_date) = self.selected_date.get() {
                let iso = if self.with_time {
                    // always use new select_hour and select_minute
                    if let (Some(selected_hour), Some(selected_minute)) = (self.selected_hour.get(), self.selected_minute.get()) {
                        // selected_hour and selected_minute comes from defined valid value, cannot panic
                        let selected_time = Time::from_hms(selected_hour, selected_minute, 0).unwrap();
                        PrimitiveDateTime::new(selected_date.date(), selected_time).js_string()
                    } else {
                        String::new()
                    }
                } else {
                    selected_date.date().to_string()
                };
                if iso.is_empty() {
                    self.set_neq_mutable(iso);
                } else {
                    self.apply_update_fn_and_set_mutable(iso);
                }
            }
        } else if self.with_time {
            if let (Some(selected_hour), Some(selected_minute)) = (self.selected_hour.get(), self.selected_minute.get()) {
                // selected_hour and selected_minute comes from defined valid value, cannot panic
                let selected_time = Time::from_hms(selected_hour, selected_minute, 0).unwrap();
                self.apply_update_fn_and_set_mutable(selected_time.js_string());
            } else {
                self.set_neq_mutable(String::new());
            }
        }
        self.container.set(None);
    }

    /// apply and set when Mutable was changed
    fn apply_update_fn_and_set_mutable(&self, value: String) {
        let v = if value.is_empty() { value } else { (self.update_fn)(value) };
        self.set_neq_mutable(v);
    }

    /// set when Mutable was changed
    fn set_neq_mutable(&self, value: String) {
        let neq = value.as_str() != self.date_mutable.lock_ref().as_str();
        if neq {
            self.date_mutable.set(value);
            self.changed_mutable.set_neq(true);
        }
    }
}

fn render_weekday_name(day: Weekday) -> Dom {
    html!("span", {
        .text(weekday_thai(&day))
        .class(GRID_HEADER)
        .attr("role", "columnheader")
    })
}

/// Creates the text that should be the title of the datepicker dialog.
pub fn create_dialog_title_text(dialog_view_type: &DialogViewType, viewed_date: &Date) -> String {
    match dialog_view_type {
        DialogViewType::Days => format!("{} {}", month_thai_full(&viewed_date.month()), viewed_date.year() + 543),
        DialogViewType::Months => (viewed_date.year() + 543).to_string(),
        DialogViewType::Years => format!("{} - {}", year_group_start(viewed_date.year() + 543), year_group_end(viewed_date.year() + 543)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;
    use time_datepicker_core::{
        utils::from_ymd,
        viewed_date::{DayNumber, MonthNumber, YearNumber},
    };

    #[fixture(year = 1990, month = 1, day = 1)]
    fn create_date(year: YearNumber, month: MonthNumber, day: DayNumber) -> Date {
        from_ymd(year, month, day)
    }

    #[rstest(
        expected,
        dialog_view_type,
        viewed_date,
        case::days_default("มกราคม 2533", DialogViewType::Days, create_date(1990, 1, 1)),
        case::months("2533", DialogViewType::Months, create_date(1990, 1, 1)),
        case::years("2520 - 2539", DialogViewType::Years, create_date(1990, 1, 1))
    )]
    fn test_create_dialog_title_text(expected: &str, dialog_view_type: DialogViewType, viewed_date: Date) {
        assert_eq!(expected, create_dialog_title_text(&dialog_view_type, &viewed_date));
    }
}
