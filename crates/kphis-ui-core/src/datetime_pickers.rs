mod picker;

use dominator::{Dom, DomBuilder, EventOptions, clone, events, html, with_node};
use futures_signals::signal::{Broadcaster, Mutable, Signal, SignalExt, not};
use time::PrimitiveDateTime;
use time_datepicker_core::config::{
    PickerConfig,
    date_constraints::{DateConstraints, HasDateConstraints},
};
use web_sys::{HtmlElement, HtmlInputElement, window};

use kphis_util::datetime::{
    JsTime, date_8601, date_from_pat, date_pat, date_str_th, datetime_8601, datetime_from_pat, datetime_pat, datetime_str_th, js_now, time_8601, time_from_pat, time_pat, time_str_hm,
};

use crate::{class, doms};

use picker::DatePicker;

#[derive(Clone)]
pub enum Picker {
    DateTime,
    Date,
    Time,
}

/// paired_mutable will use to constrain Date or Time mode<br>
/// Date mode paired with Time<br>
/// Time mode paired with Date
pub fn datetime_input_with_picker<B, C, D, F, S, T>(
    picker: Picker,
    date_mutable: Mutable<String>,
    changed_mutable: Mutable<bool>,
    disable_signal: S,
    paired_mutable: Option<Mutable<String>>,
    container_mixin: B,
    label_mixin: C,
    input_mixin: D,
    update_fn: F,
    config_signal: T,
) -> Dom
where
    B: FnOnce(DomBuilder<HtmlElement>) -> DomBuilder<HtmlElement>,
    C: FnOnce(DomBuilder<HtmlElement>) -> DomBuilder<HtmlElement>,
    D: FnOnce(DomBuilder<HtmlInputElement>) -> DomBuilder<HtmlInputElement> + Clone + 'static,
    F: Fn(String) -> String + Clone + 'static,
    S: Signal<Item = bool> + 'static,
    T: Signal<Item = Option<PickerConfig<DateConstraints>>> + 'static,
{
    let date_active = Mutable::new(false);
    let picker_mutable = Mutable::new(None);
    let now = js_now();
    let disable_broadcast = Broadcaster::new(disable_signal);
    let config_broadcast = Broadcaster::new(config_signal);

    html!("div", {
        .class("position-relative")
        .style("text-align", "left")
        .apply(container_mixin)
        // overlay label element
        .child(html!("div", {
            .class("form-control")
            .style_signal("background-color", disable_broadcast.signal_cloned().map(|is_disable| if is_disable {"var(--bs-secondary-bg)"} else {"var(--bs-body-bg)"}))
            .apply(label_mixin)
            .style("pointer-events", "none")
            .style("position", "absolute")
            .style("height", "100%")
            .style_signal("z-index", date_active.signal_cloned().map(|is_active| if is_active {"-1"} else {"1"}))
            .text_signal(date_mutable.clone().signal_cloned().map(clone!(picker => move |s| {
                match picker {
                    Picker::DateTime => datetime_str_th(&s),
                    Picker::Date => date_str_th(&s),
                    Picker::Time => time_str_hm(&s),
                }
            })))
        }))
        // input element
        .child_signal(config_broadcast.signal_cloned().map(clone!(date_mutable, changed_mutable, paired_mutable, picker, disable_broadcast, update_fn => move |config| {
            Some(html!("input" => HtmlInputElement, {
                .attr("type", "text")
                .class("form-control")
                .apply(input_mixin.clone())
                .attr("placeholder", match picker {
                    Picker::DateTime => "เช่น 31/8/68 23:45",
                    Picker::Date => "เช่น 31/8/68",
                    Picker::Time => "เช่น 23:45",
                })
                .attr("maxlength", match picker {
                    Picker::DateTime => "16",
                    Picker::Date => "10",
                    Picker::Time => "5",
                })
                .prop_signal("value", date_mutable.signal_cloned().map(clone!(picker => move |s| {
                    match picker {
                        Picker::DateTime => datetime_8601(&s).map(|dt| datetime_pat(&dt)).unwrap_or_default(),
                        Picker::Date => date_8601(&s).map(|d| date_pat(&d)).unwrap_or_default(),
                        Picker::Time => time_8601(&s).map(|t| time_pat(&t)).unwrap_or_default(),
                    }
                })))
                // set overlay label's z-index to 1 when input is blur
                .event(clone!(date_active => move |_:events::Blur| {
                    date_active.set(false);
                }))
                .with_node!(element => {
                    // set overlay label's z-index to -1 when input is focus
                    .event(clone!(element, date_active => move |_:events::Focus| {
                        date_active.set(true);
                        element.select();
                    }))
                    // set disabled
                    .future(disable_broadcast.signal().for_each(clone!(element => move |v| {
                        element.set_disabled(v);
                        async {}
                    })))
                    // key Enter will blur input
                    .event_with_options(&EventOptions::preventable(), clone!(element => move |event: events::KeyUp| {
                        if event.key() == "Enter" {
                            element.blur().unwrap();
                            event.prevent_default();
                        }
                    }))
                    // on change event
                    .event(clone!(date_mutable, changed_mutable, paired_mutable, picker, update_fn => move |_:events::Change| {
                        let v = element.value();
                        let iso = match picker {
                            Picker::DateTime => datetime_from_pat(&v).map(clone!(config => move |dt| {
                                if config.is_none() {
                                    dt.js_string()
                                } else if let Some(c) = config && !c.is_datetime_forbidden(&dt) {
                                    dt.js_string()
                                } else {
                                    String::new()
                                }
                            })).unwrap_or_default(),
                            Picker::Date => date_from_pat(&v).map(clone!(paired_mutable, config => |d| {
                                let new_time = paired_mutable.and_then(|time_paired| time_8601(&time_paired.lock_ref())).unwrap_or(now.time());
                                let new_datetime = PrimitiveDateTime::new(d, new_time);
                                if config.is_none() {
                                    d.to_string()
                                } else if let Some(c) = config && !c.is_day_forbidden(&new_datetime) {
                                    d.to_string()
                                } else {
                                    String::new()
                                }
                            })).unwrap_or_default(),
                            Picker::Time => time_from_pat(&v).map(clone!(paired_mutable, config => |t| {
                                let new_date = paired_mutable.and_then(|date_paired| date_8601(&date_paired.lock_ref())).unwrap_or(now.date());
                                let new_datetime = PrimitiveDateTime::new(new_date, t);
                                if config.is_none() {
                                    t.js_string()
                                } else if let Some(c) = config && !c.is_day_forbidden(&new_datetime) {
                                    t.js_string()
                                } else {
                                    String::new()
                                }
                            })).unwrap_or_default(),
                        };
                        let value = if iso.is_empty() {
                            iso
                        } else {
                            update_fn(iso)
                        };
                        let ready = value.as_str() != date_mutable.lock_ref().as_str();
                        if ready {
                            date_mutable.set(value);
                            changed_mutable.set_neq(true);
                        }
                    }))
                })
            }))
        })))
        // picker container
        .child_signal(config_broadcast.signal_cloned().map(move |config| {
            Some(html!("div", {
                .visible_signal(not(disable_broadcast.signal()))
                .child(html!("i", {
                    .class(match picker {
                       Picker::Date => class::FA_CALENDAR,
                       Picker::Time => class::FA_CLOCK,
                       Picker::DateTime => class::FA_CALENDARS,
                    })
                    .style("position", "absolute")
                    .style("top", "calc(50% - 13px)")
                    .style("right", "5px")
                    .style("padding", "5px 0px")
                    .style("opacity","75%")
                    .style("color", "var(--bs-body-color)")
                    .style("z-index","2")
                    .attr("title", match picker {
                        Picker::DateTime => "แสดงเครื่องมือเลือกวันที่และเวลา",
                        Picker::Date => "แสดงเครื่องมือเลือกวันที่",
                        Picker::Time => "แสดงเครื่องมือเลือกเวลา",
                    })
                    .event(clone!(date_mutable, changed_mutable, paired_mutable, picker_mutable, picker, update_fn, config => move |_:events::Click| {
                        let empty_picker = picker_mutable.lock_ref().is_none();
                        if empty_picker {
                            let new_picker = match picker {
                                Picker::DateTime => DatePicker::new_datetime(
                                    date_mutable.clone(),
                                    changed_mutable.clone(),
                                    picker_mutable.clone(),
                                    update_fn.clone(),
                                    config.clone().unwrap_or_default(),
                                ),
                                Picker::Date => DatePicker::new_date(
                                    date_mutable.clone(),
                                    changed_mutable.clone(),
                                    paired_mutable.clone(),
                                    picker_mutable.clone(),
                                    update_fn.clone(),
                                    config.clone().unwrap_or_default(),
                                ),
                                Picker::Time => DatePicker::new_time(
                                    date_mutable.clone(),
                                    changed_mutable.clone(),
                                    paired_mutable.clone(),
                                    picker_mutable.clone(),
                                    update_fn.clone(),
                                    config.clone().unwrap_or_default(),
                                ),
                            };
                            picker_mutable.set(Some(new_picker));
                        } else {
                            picker_mutable.set(None);
                        }
                    }))
                }))
                // picker component
                .with_node!(element => {
                    .child_signal(picker_mutable.signal_cloned().map(clone!(picker => move |opt| {
                        let w = match picker {
                            Picker::DateTime => 342.0,
                            Picker::Date => 254.0,
                            Picker::Time => 94.0,
                        };
                        opt.map(|picker| {
                            doms::under_box(
                                element.parent_element().unwrap().get_bounding_client_rect(),
                                w, 280.0, window().unwrap().scroll_y().unwrap(),
                                |bx| { bx.child(DatePicker::render(picker)) }
                            )
                        })
                    })))
                })
            }))
        }))
    })
}
