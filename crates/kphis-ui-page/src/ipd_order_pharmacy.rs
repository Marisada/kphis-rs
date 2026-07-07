use dominator::{Dom, EventOptions, clone, events, html, is_window_loaded, text, with_node};
use futures_signals::{
    map_ref,
    signal::{Mutable, SignalExt, always},
    signal_vec::{MutableVec, SignalVecExt},
};
use std::{ops::Deref, rc::Rc};
use time::PrimitiveDateTime;
use wasm_bindgen::JsCast;
use web_sys::{HtmlAudioElement, HtmlButtonElement, HtmlInputElement, HtmlOptionElement, HtmlSelectElement};

use kphis_model::{
    ipd::pharmacy_monitor::{IpdOrderPharmacy, IpdOrderPharmacyMonitor, IpdOrderPharmacyParams, PharmacyIpt},
    route::Route,
    tab::Tab,
};
use kphis_ui_app::App;
use kphis_ui_core::{binding::NiceSelect, class, doms, mixins};
use kphis_util::{
    datetime::{date_8601, datetime_th, datetime_th_opt, js_now},
    util::{opt_zero_none, str_some},
};

use crate::NEW_ORDER;

/// - GET `EndPoint::IpdOrderPharmacy`
#[derive(Clone, Default)]
pub struct IpdOrderPharmacyPage {
    doctor_in_charge: Mutable<String>,
    order_date_from: Mutable<String>,
    order_date_to: Mutable<String>,
    patient: Mutable<String>,
    search_order_med_result: MutableVec<Rc<IpdOrderPharmacy>>,
    search_order_no_med_result: MutableVec<Rc<IpdOrderPharmacy>>,
    search_admit_result: MutableVec<Rc<PharmacyIpt>>,
    last_order_time: Mutable<Option<PrimitiveDateTime>>,

    play_sound: Mutable<bool>,

    timer_handle: Mutable<Option<i32>>,
    timer_second: Mutable<f32>,
    changed: Mutable<bool>,
}

impl IpdOrderPharmacyPage {
    pub fn new() -> Rc<Self> {
        let now = js_now().date();
        Rc::new(Self {
            order_date_from: Mutable::new(now.previous_day().map(|d| d.to_string()).unwrap_or_default()),
            order_date_to: Mutable::new(now.to_string()),
            ..Default::default()
        })
    }

    fn test_audio(&self, app: Rc<App>) {
        let toggle_sound = if app.order_monitor_new_order_sound_on.lock_ref().deref() == "on" {
            match app.get_id("audio").and_then(|elm| elm.dyn_into::<HtmlAudioElement>().ok()) {
                Some(elm) => {
                    if elm.play().is_ok() {
                        "on"
                    } else {
                        "off"
                    }
                }
                None => "off",
            }
        } else {
            "off"
        };
        app.order_monitor_new_order_sound_on.set_neq(String::from(toggle_sound));
    }

    pub fn render(page: Rc<Self>, app: Rc<App>) -> Dom {
        app.set_title("KPHIS - IPD Pharmacy Order");

        let (ward_select_option, doctor_select_option) = app
            .app_asset
            .lock_ref()
            .as_ref()
            .map(|asset| (asset.ward_select_option.clone(), asset.doctor_select_option.clone()))
            .unwrap_or_default();

        html!("section", {
            .future(is_window_loaded().for_each(clone!(app, page => move |value| {
                if value {
                    if let Some(elm) = app.get_id("ward") {
                        NiceSelect::new_default_with_value(&elm, &app.ward_multiple_select.lock_ref());
                    }
                    if let Some(elm) = app.get_id("doctor_in_charge") {
                        NiceSelect::new_default(&elm);
                    }
                    page.test_audio(app.clone());
                    page.changed.set(true);
                }
                async {}
            })))
            .future(map_ref!(
                let busy = app.loader_is_loading(),
                let changed = page.changed.signal() =>
                !busy && *changed
            ).for_each(clone!(app, page => move |ready| {
                if ready {
                    submit(page.clone(), app.clone());
                    page.changed.set(false);
                }
                async {}
            })))
            .future(app.monitor_refresh_interval.signal_cloned().for_each(clone!(app, page => move |interval_str| {
                if let Some(handle_id) = page.timer_handle.get() {
                    app.clear_interval(handle_id);
                }
                page.timer_second.set(0.0);
                if let Some(interval) = opt_zero_none(interval_str.parse::<i32>().ok().map(|i| i * 1000 / 60)) {
                    let handle = app.set_interval(clone!(page => move || {
                        let timer = page.timer_second.get();
                        if timer == 59.0 {
                            page.changed.set_neq(true);
                            page.timer_second.set(0.0);
                        } else {
                            page.timer_second.set(timer + 1.0);
                        }
                    }), interval);
                    page.timer_handle.set_neq(Some(handle));
                }
                async {}
            })))
            .future(page.play_sound.signal().for_each(clone!(app, page => move |play| {
                if play && app.order_monitor_new_order_sound_on.lock_ref().deref() == "on" {
                    if let Some(elm) = app.get_id("audio1").and_then(|elm| elm.dyn_into::<HtmlAudioElement>().ok()) {
                        let _ = elm.pause();
                        elm.set_current_time(0.0);
                        let _ = elm.play();
                    }
                    page.play_sound.set(false);
                }
                async {}
            })))
            .class("container-fluid")
            .children([
                doms::alert_row(clone!(app, page => move |alert| { alert
                    .children([
                        doms::form_inline(clone!(app, page => move |form| { form
                            .children([
                                // .style("width","550px")
                                doms::form_inline_group_sm(clone!(app, page => move |group| { group
                                    .children([
                                        doms::label_group_for("ward","แผนก"),
                                        html!("div", {
                                            .class(class::FLEX_GROW1)
                                            .child(html!("select" => HtmlSelectElement, {
                                                .class(class::FORM_CTRL_SM)
                                                .attr("id", "ward")
                                                .attr("multiple", "multiple")
                                                .children(ward_select_option.iter().map(|option| {
                                                    doms::select_option(option, "")
                                                }))
                                                .with_node!(element => {
                                                    .event(clone!(app, page => move |_: events::Change| {
                                                        let options = element.selected_options();
                                                        let mut values = Vec::new();
                                                        for j in 0..options.length() {
                                                            if let Some(item) = options.item(j) {
                                                                if let Ok(option) = item.dyn_into::<HtmlOptionElement>() {
                                                                    values.push(option.value());
                                                                }
                                                            }
                                                        }
                                                        app.ward_multiple_select.set_neq(values.join(","));
                                                        app.to_local_storage();
                                                        page.changed.set_neq(true);
                                                    }))
                                                })
                                                //.attr("onchange", "onchange_select_ward()")
                                            }))
                                        }),
                                    ])
                                })),
                                doms::form_inline_radio(clone!(app, page => move |check| { check
                                    .children([
                                        html!("input" => HtmlInputElement, {
                                            .class("form-check-input")
                                            .attr("type", "radio")
                                            .attr("id", "inverse_ward_selection1")
                                            .attr("value", "N")
                                            .with_node!(element => {
                                                .future(app.inverse_ward_select.signal_cloned().for_each(clone!(element => move |v| {
                                                    if v == "N" || v.is_empty()  {
                                                        element.set_checked(true);
                                                    } else {
                                                        element.set_checked(false);
                                                    }
                                                    async {}
                                                })))
                                                .event(clone!(app, page, element => move |_: events::Change| {
                                                    app.inverse_ward_select.set_neq(element.value());
                                                    app.to_local_storage();
                                                    page.changed.set_neq(true);
                                                }))
                                            })
                                            //.attr("onchange", "onchange_select_ward()")
                                        }),
                                        doms::label_check_for("inverse_ward_selection1","แสดงเฉพาะแผนกที่เลือก"),
                                    ])
                                })),
                                doms::form_inline_radio(clone!(app, page => move |check| { check
                                    .children([
                                        html!("input" => HtmlInputElement, {
                                            .class("form-check-input")
                                            .attr("type", "radio")
                                            .attr("id", "inverse_ward_selection2")
                                            .attr("value", "Y")
                                            .with_node!(element => {
                                                .future(app.inverse_ward_select.signal_cloned().for_each(clone!(element => move |v| {
                                                    if v == "Y" {
                                                        element.set_checked(true);
                                                    } else {
                                                        element.set_checked(false);
                                                    }
                                                    async {}
                                                })))
                                                .event(clone!(app, page, element => move |_: events::Change| {
                                                    app.inverse_ward_select.set_neq(element.value());
                                                    app.to_local_storage();
                                                    page.changed.set_neq(true);
                                                }))
                                            })
                                            //.attr("onchange", "onchange_select_ward()")
                                        }),
                                        doms::label_check_for("inverse_ward_selection2","แสดงทั้งหมด ยกเว้นแผนกที่เลือก"),
                                    ])
                                })),
                                // .style("width","250px")
                                doms::form_inline_group_sm(clone!(page => move |group| { group
                                    .children([
                                        doms::label_group_for("doctor_in_charge","แพทย์เจ้าของไข้"),
                                        html!("div", {
                                            .class(class::FLEX_GROW1)
                                            .child(html!("select" => HtmlSelectElement, {
                                                .class(class::FORM_CTRL_SM)
                                                .attr("id", "doctor_in_charge")
                                                .child(html!("option", {
                                                    .attr("value", "")
                                                    .text("เลือก")
                                                }))
                                                .children(doctor_select_option.iter().map(|option| {
                                                    doms::select_option(option, "")
                                                }))
                                                .apply(mixins::string_value_select(page.doctor_in_charge.clone(), page.changed.clone()))
                                            }))
                                        }),
                                    ])
                                })),
                                html!("div", {
                                    .class("col-12")
                                    .child(doms::is_discharged_radio(app.ipd_pharmacy_order_monitor_is_discharged.clone(), page.changed.clone(), app.state()))
                                }),
                                doms::form_inline_group_sm(clone!(page => move |group| { group
                                    .children([
                                        doms::label_group_for("order_date_from","วันที่ Order ตั้งแต่"),
                                        doms::date_picker(
                                            page.order_date_from.clone(),
                                            page.changed.clone(), always(false), None,
                                            |d| d.class(class::FLEX_GROW1).style("min-width","120px"),
                                            |d| d.class(class::FORM_CTRL_ONLY_SM_R0),
                                            |d| d.class(class::FORM_CTRL_ONLY_SM_R0).attr("id", "order_date_from"),
                                            |s| s, always(None),
                                        ),
                                        doms::label_group_for("order_date_to","ถึงวันที่"),
                                        doms::date_picker(
                                            page.order_date_to.clone(),
                                            page.changed.clone(), always(false), None,
                                            |d| d.class(class::FLEX_GROW1).style("min-width","120px"),
                                            |d| d.class(class::FORM_CTRL_ONLY_SM_R0_L),
                                            |d| d.class(class::FORM_CTRL_ONLY_SM_R0_L).attr("id", "order_date_to"),
                                            |s| s, always(None),
                                        ),
                                    ])
                                })),
                                doms::form_inline_group_sm(clone!(page => move |group| { group
                                    .children([
                                        doms::label_group_for("patient","HN, AN, CID, ชื่อ-สกุล"),
                                        html!("input" => HtmlInputElement, {
                                            .attr("type", "text")
                                            .class(class::FORM_CTRL_SM)
                                            .attr("id", "patient")
                                            .attr("autocomplete","off")
                                            .prop_signal("value", page.patient.signal_cloned())
                                            .with_node!(element => {
                                                .event_with_options(&EventOptions::preventable(), clone!(page => move |event: events::KeyDown| {
                                                    if event.key() == "Enter" {
                                                        event.prevent_default();
                                                        page.patient.set_neq(element.value());
                                                        page.changed.set_neq(true);
                                                    }
                                                }))
                                            })
                                        }),
                                        html!("button", {
                                            .attr("type", "button")
                                            .class(class::BTN_SM_GRAY)
                                            .child(html!("i", {.class(class::FA_SEARCH)}))
                                            .text(" ค้นหา")
                                            .event(clone!(page => move |_: events::Click| {
                                                page.changed.set_neq(true);
                                            }))
                                        }),
                                    ])
                                })),
                                doms::form_inline_end(clone!(app, page => move |end| { end
                                    .children([
                                        doms::form_inline_group_sm(clone!(app, page => move |group| { group
                                            .children([
                                                doms::label_group_for("refresh_interval","รอบการ Update "),
                                                html!("select" => HtmlSelectElement, {
                                                    .class(class::FORM_SELECT_SM)
                                                    .attr("id", "refresh_interval")
                                                    .children([
                                                        html!("option", {.attr("value", "0").text("ไม่ต้องทำ")}),
                                                        html!("option", {.attr("value", "5").text("5 วินาที")}),
                                                        html!("option", {.attr("value", "15").text("15 วินาที")}),
                                                        html!("option", {.attr("value", "30").text("30 วินาที")}),
                                                        html!("option", {.attr("value", "60").attr("selected", "").text("1 นาที")}),
                                                        html!("option", {.attr("value", "180").text("3 นาที")}),
                                                        html!("option", {.attr("value", "300").text("5 นาที")}),
                                                        html!("option", {.attr("value", "600").text("10 นาที")}),
                                                        html!("option", {.attr("value", "900").text("15 นาที")}),
                                                        html!("option", {.attr("value", "1200").text("20 นาที")}),
                                                        html!("option", {.attr("value", "1800").text("30 นาที")}),
                                                        html!("option", {.attr("value", "2700").text("45 นาที")}),
                                                        html!("option", {.attr("value", "3600").text("1 ชั่วโมง")}),
                                                    ])
                                                    .prop_signal("value", app.monitor_refresh_interval.signal_cloned())
                                                    .with_node!(element => {
                                                        .event(clone!(app, page, element => move |_: events::Change| {
                                                            app.monitor_refresh_interval.set_neq(element.value());
                                                            app.to_local_storage();
                                                            page.changed.set_neq(true);
                                                        }))
                                                    })
                                                }),
                                                html!("label", {
                                                    .class("input-group-text")
                                                    .child(doms::timer_svg(page.timer_second.clone()))
                                                }),
                                                html!("button" => HtmlButtonElement, {
                                                    .attr("type", "button")
                                                    .class(class::BTN_SM_L_GRAY)
                                                    //.attr("id", "toggleSoundButton")
                                                    .child_signal(app.order_monitor_new_order_sound_on.signal_cloned().map(|sound| {
                                                        if sound == "on" {
                                                            Some(html!("i", {.class(class::FA_VOL_UP)}))
                                                        } else {
                                                            Some(html!("i", {.class(class::FA_VOL_MUTE)}))
                                                        }
                                                    }))
                                                    .event(clone!(app, page => move |_: events::Click| {
                                                        let is_on = app.order_monitor_new_order_sound_on.lock_ref().as_str() == "on";
                                                        if is_on {
                                                            app.order_monitor_new_order_sound_on.set(String::from("off"));
                                                        } else {
                                                            app.order_monitor_new_order_sound_on.set(String::from("on"));
                                                        }
                                                        app.to_local_storage();
                                                        page.test_audio(app.clone());
                                                    }))
                                                    // .attr("onclick", "onclickToggleSoundButton()")
                                                }),
                                                html!("button", {
                                                    .attr("type", "button")
                                                    .class(class::BTN_SM_GRAY)
                                                    .child(html!("i", {.class(class::FA_PLAY)}))
                                                    .text(" Test")
                                                    .event(clone!(page => move |_: events::Click| {
                                                        page.play_sound.set(true);
                                                    }))
                                                    // .attr("onclick", "playNewOrderInSound()")
                                                }),
                                            ])
                                        })),
                                        html!("audio", {
                                            .attr("src", "statics/audio/audio_test.mp3")
                                            .attr("autostart", "false")
                                            .attr("id", "audio")
                                        }),
                                        html!("audio", {
                                            .attr("src", NEW_ORDER)
                                            .attr("autostart", "false")
                                            .attr("id", "audio1")
                                        }),
                                    ])
                                })),
                            ])
                        })),
                        html!("div", {
                            .class("col-sm")
                            .child(doms::badge_info_center("หากค้นหาด้วย HN, AN, CID, ชื่อ-สกุล โปรแกรมจะแสดงเฉพาะ 100 รายการล่าสุด"))
                            .child_signal(page.search_order_med_result.signal_vec_cloned().len().map(|i| {
                                Some(doms::badge_count_with_limit(i, 100))
                            }))
                        }),
                    ])
                })),
                html!("div", {
                    .children([
                        render_order_table(
                            page.search_order_med_result.clone(),
                            "ผู้ป่วยที่มี Order ยาในช่วงเวลาที่เลือก",
                            app.clone(),
                        ),
                        render_order_table(
                            page.search_order_no_med_result.clone(),
                            "ผู้ป่วยที่ไม่มี Order ยาในช่วงเวลาที่เลือก",
                            app.clone(),
                        ),
                    ])
                    .child_signal(page.patient.signal_cloned().map(clone!(page => move |patient| {
                        is_u64_parsable(&patient).then(|| {
                            render_ipt_table(
                                page.search_admit_result.clone(),
                                "ประวัติการ Admit",
                            )
                        })
                    })))
                }),
            ])
        })
    }
}

// ipd-pharmacy-order-monitor-table.php
fn submit(page: Rc<IpdOrderPharmacyPage>, app: Rc<App>) {
    let request = IpdOrderPharmacyParams {
        wards: str_some(app.ward_multiple_select.get_cloned()),
        inverse_ward_select: str_some(app.inverse_ward_select.get_cloned()),
        doctor_in_charge: str_some(page.doctor_in_charge.get_cloned()),
        order_date_from: date_8601(&page.order_date_from.lock_ref()),
        order_date_to: date_8601(&page.order_date_to.lock_ref()),
        is_discharged: str_some(app.ipd_pharmacy_order_monitor_is_discharged.get_cloned()),
        patient: str_some(page.patient.get_cloned()),
    };

    app.async_load(
        true,
        clone!(app, page => async move {
            // GET `EndPoint::IpdOrderPharmacy`
            match IpdOrderPharmacyMonitor::call_api_get(&request, app.state()).await {
                Ok(monitor) => {
                    let mut med_lock = page.search_order_med_result.lock_mut();
                    let mut no_med_lock = page.search_order_no_med_result.lock_mut();
                    let mut admits_lock = page.search_admit_result.lock_mut();
                    med_lock.clear();
                    no_med_lock.clear();
                    admits_lock.clear();

                    // let mut orders = monitor.orders.clone();
                    // orders.sort_by(|a, b| {
                    //     let a_max_opt = a.max_order_date_time.as_ref().and_then(|dt| datetime_8601(dt));
                    //     let b_max_opt = b.max_order_date_time.as_ref().and_then(|dt| datetime_8601(dt));
                    //     if let (Some(a_max), Some(b_max)) = (a_max_opt, b_max_opt) {
                    //         b_max.cmp(&a_max)
                    //     } else if a_max_opt.is_some() {
                    //         std::cmp::Ordering::Less
                    //     } else if b_max_opt.is_some() {
                    //         std::cmp::Ordering::Greater
                    //     } else if b.count_item_not_accept_stat != a.count_item_not_accept_stat {
                    //         b.count_item_not_accept_stat.cmp(&a.count_item_not_accept_stat)
                    //     } else if b.count_not_accept != a.count_not_accept {
                    //         b.count_not_accept.cmp(&a.count_not_accept)
                    //     } else {
                    //         b.an.cmp(&a.an)
                    //     }
                    // });

                    let last_order_time = page.last_order_time.get();
                    let new_last_order_time = monitor.orders.first().and_then(|i| i.max_order_date_time);
                    if let Some(new) = new_last_order_time {
                        page.last_order_time.set_neq(Some(new));
                        if let Some(last) = last_order_time {
                            if new > last {
                                page.play_sound.set(true);
                            }
                        }
                    }

                    let dt_from_opt = date_8601(&page.order_date_from.lock_ref());
                    let dt_to_opt = date_8601(&page.order_date_to.lock_ref());
                    let (med, mut no_med): (Vec<IpdOrderPharmacy>, Vec<IpdOrderPharmacy>) = monitor.orders.clone().into_iter().partition(|i| {
                        if let (Some(min_dt), Some(max_dt)) = (i.min_order_date_time, i.max_order_date_time) {
                            if let (Some(dt_from), Some(dt_to)) = (dt_from_opt, dt_to_opt) {
                                max_dt.date() >= dt_from && min_dt.date() <= dt_to
                            } else if let Some(dt_from) = dt_from_opt {
                                max_dt.date() >= dt_from
                            } else if let Some(dt_to) = dt_to_opt {
                                min_dt.date() <= dt_to
                            } else {
                                false
                            }
                        } else {
                            false
                        }
                    });

                    med_lock.extend(med.into_iter().map(Rc::new));
                    no_med.sort_by(|a, b| {
                        if let (Some(bed_a), Some(bed_b)) = (a.bedno.as_ref(), b.bedno.as_ref()) {
                            bed_a.cmp(&bed_b)
                        } else {
                            a.an.cmp(&b.an)
                        }
                    });
                    no_med_lock.extend(no_med.into_iter().map(Rc::new));
                    admits_lock.extend(monitor.admits.into_iter().map(Rc::new));
                }
                Err(e) => {
                    app.alert_app_error(&e).await;
                }
            }
        }),
    );
}

fn render_pharmacy_order(i: usize, row: Rc<IpdOrderPharmacy>, app: Rc<App>) -> Dom {
    let age_y = row.age_y.unwrap_or_default();
    let age_m = row.age_m.unwrap_or_default();
    let age_d = row.age_d.unwrap_or_default();
    let age_str = if age_y > 0 {
        [&age_y.to_string(), " ปี"].concat()
    } else if age_m > 0 {
        [&age_m.to_string(), " เดือน ", &age_d.to_string(), " วัน"].concat()
    } else {
        [&age_d.to_string(), " วัน"].concat()
    };
    let (order_priority, order_priority_text) = if row.count_not_accept > 0 {
        if row.count_item_not_accept_stat > 0 {
            (1, "ยังไม่ได้รับ - มี Stat Order")
        } else if row.count_item_not_accept_homemed > 0 {
            (2, "ยังไม่ได้รับ - มี Home-Med Order")
        } else {
            (3, "ยังไม่ได้รับ")
        }
    } else if row.count_accept > 0 {
        if row.count_item_accept_stat > 0 {
            (4, "รับแล้ว - มี Stat Order")
        } else if row.count_item_accept_homemed > 0 {
            (5, "รับแล้ว - มี Home-Med Order")
        } else {
            (6, "รับแล้ว")
        }
    } else if row.count_check > 0 {
        if row.count_item_accept_stat > 0 {
            (7, "ตรวจสอบแล้ว - มี Stat Order")
        } else if row.count_item_accept_homemed > 0 {
            (8, "ตรวจสอบแล้ว - มี Home-Med Order")
        } else {
            (9, "ตรวจสอบแล้ว")
        }
    } else if row.count_done > 0 {
        if row.count_item_accept_stat > 0 {
            (10, "จ่ายแล้ว - มี Stat Order")
        } else if row.count_item_accept_homemed > 0 {
            (11, "จ่ายแล้ว - มี Home-Med Order")
        } else {
            (12, "จ่ายแล้ว")
        }
    } else {
        (13, "")
    };

    html!("tr", {
        .apply(|dom| match order_priority {
            ..=1 => dom.class("table-danger"),
            2..=3 => dom.class("table-warning"),
            4..=6 => dom.class("table-info"),
            4..=9 => dom.class("table-success"),
            _ => dom,
        })
        .style("cursor","pointer")
        .children([
            html!("td", {.class("text-center").text(&(i + 1).to_string())}),
            html!("td", {.class("text-truncate").text(&row.ward_name.clone().unwrap_or_default())}),
            html!("td", {.class("text-truncate").text(&row.bedno.clone().unwrap_or_default())}),
            html!("td", {.class("text-truncate").text(&row.an)}),
            html!("td", {.class("text-truncate").text(&row.hn.clone().unwrap_or_default())}),
            html!("td", {
                .class(class::TRUNC_BOLD)
                .attr("title",&row.pname.clone().unwrap_or_default())
                .style("max-width","200px")
                .text(&row.pname.clone().unwrap_or_default())
            }),
            html!("td", {.class("text-truncate").text(&age_str)}),
            html!("td", {
                .text(&[
                    row.min_order_date_time.as_ref().map(|dt| datetime_th(dt)).unwrap_or_default(),
                    row.max_order_date_time.as_ref().map(|dt| datetime_th(dt)).unwrap_or_default()
                ].join(" - "))
            }),
            html!("td", {
                .class("text-center")
                .apply(|dom| {
                    if let Some(dchtype_name) = row.dchtype_name.as_ref() {
                        if dchtype_name.as_str() == "By Transfer" {
                            dom.child(html!("i", {.class(class::FA_AMBULANCE)}))
                        } else if dchtype_name.contains("Dead") {
                            dom.child(html!("i", {.class(class::FA_SKULL)}))
                        } else {
                            dom.child(html!("i", {.class(class::FA_HOUSE)}))
                        }
                    } else {
                        dom
                    }
                })
            }),
            html!("td", {
                .class("text-center")
                .apply_if(row.count_pharm_notify > 0, |dom| {
                    dom.child(html!("i", {.class(class::FA_ALERT_GOLD)}))
                })
            }),
            html!("td", {
                .apply(|dom| match order_priority {
                    ..=3 => dom.child(html!("i", {.class(class::FA_ENV)})),
                    4..=6 => dom.child(html!("i", {.class(class::FA_EDIT)})),
                    7..=9 => dom.child(html!("i", {.class(class::FA_CHECK_CIRCLE)})),
                    _ => dom,
                })
                .text(" ")
                .text(&order_priority_text)
                .text(" ")
                .child(html!("div", {
                    .class("float-end")
                    .children([
                        html!("i", {.class(class::FA_ENV)}),text(" "),
                        text(&row.count_not_accept.to_string()),text(" "),
                        html!("i", {.class(class::FA_EDIT)}),text(" "),
                        text(&row.count_accept.to_string()),text(" "),
                        html!("i", {.class(class::FA_CHECK_CIRCLE)}),text(" "),
                        text(&row.count_check.to_string()),text(" "),
                        html!("i", {.class(class::FA_PILLS)}),text(" "),
                        text(&row.count_done.to_string()),text(" "),
                        html!("i", {.class(class::FA_ALERT_GOLD)}),text(" "),
                        text(&row.count_pharm_notify.to_string()),
                    ])
                }))
            })
        ])
        .apply(|dom| {
            let route = Route::IpdMain {
                view_by: String::from("pharmacist"),
                an: row.an.clone(),
                tab: Tab::Order.str().to_owned(),
                sub: String::new(),
                id: 0,
            };
            if route.has_permission(app.state()) {
                dom.event(move |_: events::Click| {
                    route.hard_redirect();
                })
            } else {
                dom
            }
        })
    })
}

fn render_order_table(source: MutableVec<Rc<IpdOrderPharmacy>>, label: &str, app: Rc<App>) -> Dom {
    html!("div", {
        .children([
            html!("hr"),
            html!("h5", {.text(label)}),
            doms::table_responsive(class::TABLE_STRIP, |table| { table
                .children([
                    html!("thead", {
                        .child(html!("tr", {
                            .class("text-center")
                            .children([
                                html!("th", {.class("th-sm").attr("scope", "col").text("#")}),
                                html!("th", {.class("th-sm").attr("scope", "col").text("แผนก")}),
                                html!("th", {.class("th-sm").attr("scope", "col").text("เตียง")}),
                                html!("th", {.class("th-sm").attr("scope", "col").text("AN")}),
                                html!("th", {.class("th-sm").attr("scope", "col").text("HN")}),
                                html!("th", {.class("th-sm").attr("scope", "col").text("ชื่อ - สกุล")}),
                                html!("th", {.class("th-sm").attr("scope", "col").text("อายุ")}),
                                html!("th", {.class("th-sm").attr("scope", "col").text("เวลา Order")}),
                                html!("th", {.class("th-sm").attr("scope", "col")
                                    .child(html!("i", {.class(class::FA_HOUSE)}))
                                }),
                                html!("th", {.class("th-sm").attr("scope", "col")
                                    .child(html!("i", {.class(class::FA_ALERT)}))
                                }),
                                html!("th", {.class("th-sm").attr("scope", "col").text("Priority")}),
                            ])
                        }))
                    }),
                    html!("tbody", {
                        .children_signal_vec(source.signal_vec_cloned().enumerate().map(clone!(app => move |(i, row)| {
                            render_pharmacy_order(i.get().unwrap_or_default(), row, app.clone())
                        })))
                    }),
                ])
            })
        ])
    })
}

fn render_ipt_table(source: MutableVec<Rc<PharmacyIpt>>, label: &str) -> Dom {
    html!("div", {
        .children([
            html!("hr"),
            html!("h5", {.text(label)}),
            doms::table_responsive(class::TABLE_STRIP, |table| { table
                .children([
                    html!("thead", {
                        .child(html!("tr", {
                            .class("text-center")
                            .children([
                                html!("th", {.class("th-sm").attr("scope", "col").text("#")}),
                                html!("th", {.class("th-sm").attr("scope", "col").text("แผนก")}),
                                html!("th", {.class("th-sm").attr("scope", "col").text("เตียง")}),
                                html!("th", {.class("th-sm").attr("scope", "col").text("AN")}),
                                html!("th", {.class("th-sm").attr("scope", "col").text("HN")}),
                                html!("th", {.class("th-sm").attr("scope", "col").text("ชื่อ - สกุล")}),
                                html!("th", {.class("th-sm").attr("scope", "col").text("อายุ")}),
                                html!("th", {.class("th-sm").attr("scope", "col").text("แพทย์เจ้าของไข้")}),
                                html!("th", {.class("th-sm").attr("scope", "col").children([
                                    text("เวลาล่าสุด"),html!("br"),text("Progress Note"),
                                ])}),
                                html!("th", {.class("th-sm").attr("scope", "col").children([
                                    text("เวลาล่าสุด"),html!("br"),text("Order"),
                                ])}),
                            ])
                        }))
                    }),
                    html!("tbody", {
                        .children_signal_vec(source.signal_vec_cloned().enumerate().map(|(i, row)| {
                            render_pharmacy_ipt(i.get().unwrap_or_default(), row)
                        }))
                    }),
                ])
            }),
        ])
    })
}

fn render_pharmacy_ipt(i: usize, row: Rc<PharmacyIpt>) -> Dom {
    let age_y = row.age_y.unwrap_or_default();
    let age_m = row.age_m.unwrap_or_default();
    let age_d = row.age_d.unwrap_or_default();
    let age_str = if age_y > 0 {
        [&age_y.to_string(), " ปี"].concat()
    } else if age_m > 0 {
        [&age_m.to_string(), " เดือน ", &age_d.to_string(), " วัน"].concat()
    } else {
        [&age_d.to_string(), " วัน"].concat()
    };
    let kphis_incharge_doctor_name_with_html = html!("div", {
        .class("text-truncate")
        .children(row.kphis_incharge_doctor_name.clone().map(|docs| {
            docs.split(',').map(|doc| {
                html!("div", {
                    .class(class::TRUNC_SM)
                    .style("max-width","162px")
                    .text(doc)
                })
            }).collect::<Vec<Dom>>()
        }).unwrap_or_default())
    });

    html!("tr", {
        .style("cursor","pointer")
        .children([
            html!("td", {.class("text-center").text(&(i + 1).to_string())}),
            html!("td", {.child(html!("div", {.class("text-truncate").text(&row.ward_name.clone().unwrap_or_default())}))}),
            html!("td", {.child(html!("div", {.class("text-truncate").text(&row.bedno.clone().unwrap_or_default())}))}),
            html!("td", {.class("text-center").child(html!("div", {.class("text-truncate").text(&row.an)}))}),
            html!("td", {.class("text-center").child(html!("div", {.class("text-truncate").text(&row.hn.clone().unwrap_or_default())}))}),
            html!("td", {
                .attr("title",&row.pname.clone().unwrap_or_default())
                .class(class::TRUNC_BOLD)
                .text(&row.pname.clone().unwrap_or_default())
            }),
            html!("td", {.child(html!("div", {.class("text-truncate").text(&age_str)}))}),
            html!("td", {
                .attr("title", &row.kphis_incharge_doctor_name.clone().unwrap_or_default())
                .child(kphis_incharge_doctor_name_with_html)
            }),
            html!("td", {.class("text-center").child(html!("div", {.class("text-truncate").text(
                &datetime_th_opt(&row.max_fcnote_datetime)
            )}))}),
            html!("td", {.class("text-center").child(html!("div", {.class("text-truncate").text(
                &datetime_th_opt(&row.max_order_datetime)
            )}))}),
        ])
        .event(clone!(row => move |_: events::Click| {
            Route::IpdMain {
                view_by: String::from("pharmacist"),
                an: row.an.clone(),
                tab: Tab::Order.str().to_owned(),
                sub: String::new(),
                id: 0,
            }.hard_redirect();
        }))
    })
}

#[inline]
fn is_u64_parsable(text: &str) -> bool {
    !text.is_empty() && text.parse::<u64>().is_ok()
}
