use dominator::{Dom, EventOptions, clone, events, html, is_window_loaded, text, with_node};
use futures_signals::{
    map_ref,
    signal::{Mutable, SignalExt, always},
    signal_vec::{MutableVec, SignalVecExt},
};
use std::{ops::Deref, rc::Rc};
use time::PrimitiveDateTime;
use wasm_bindgen::JsCast;
use web_sys::{HtmlAudioElement, HtmlButtonElement, HtmlInputElement, HtmlSelectElement};

use kphis_model::{
    opd_er::pharmacy_monitor::{OpdErOrderPharmacy, OpdErOrderPharmacyMonitor, OpdErOrderPharmacyParams},
    route::Route,
    tab::Tab,
};
use kphis_ui_app::App;
use kphis_ui_core::{class, doms};
use kphis_util::{
    datetime::{date_8601, datetime_th_opt_relative, js_now},
    util::{opt_zero_none, str_some},
};

use crate::NEW_ORDER;

/// GET `EndPoint::OpdErOrderPharmacy`
#[derive(Clone, Default)]
pub struct OpdErOrderPharmacyPage {
    order_date_from: Mutable<String>,
    order_date_to: Mutable<String>,
    patient: Mutable<String>,
    search_order_med_result: MutableVec<Rc<OpdErOrderPharmacy>>,
    search_order_no_med_result: MutableVec<Rc<OpdErOrderPharmacy>>,
    // search_master_result: MutableVec<Rc<OpdErOrderPharmacyMaster>>,
    last_order_time: Mutable<Option<PrimitiveDateTime>>,

    play_sound: Mutable<bool>,

    timer_handle: Mutable<Option<i32>>,
    timer_second: Mutable<f32>,
    changed: Mutable<bool>,
}

impl OpdErOrderPharmacyPage {
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
        app.set_title("KPHIS - OPD-ER Pharmacy Order");

        html!("section", {
            .future(is_window_loaded().for_each(clone!(app, page => move |value| {
                if value {
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
                                html!("div", {
                                    .class("col-12")
                                    .child(doms::is_discharged_radio(app.opd_er_pharmacy_order_monitor_is_discharged.clone(), page.changed.clone(), app.state()))
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
                                        doms::label_group_for("patient","HN, VN, CID, ชื่อ-สกุล"),
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
                                                        page.test_audio(app.clone());
                                                        app.to_local_storage();
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
                            .child(doms::badge_info_center("หากค้นหาด้วย HN, VN, ชื่อ-สกุล โปรแกรมจะแสดงเฉพาะ 100 รายการล่าสุด"))
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
                }),
            ])
        })
    }
}

// opd-er-pharmacy-order-monitor-table.php
fn submit(page: Rc<OpdErOrderPharmacyPage>, app: Rc<App>) {
    let request = OpdErOrderPharmacyParams {
        order_date_from: date_8601(&page.order_date_from.lock_ref()),
        order_date_to: date_8601(&page.order_date_to.lock_ref()),
        is_discharged: str_some(app.opd_er_pharmacy_order_monitor_is_discharged.get_cloned()),
        patient: str_some(page.patient.get_cloned()),
    };

    app.async_load(
        true,
        clone!(app, page => async move {
            // GET `EndPoint::OpdErOrderPharmacy`
            match OpdErOrderPharmacyMonitor::call_api_get(&request, app.state()).await {
                Ok(monitor) => {
                    let mut med_lock = page.search_order_med_result.lock_mut();
                    let mut no_med_lock = page.search_order_no_med_result.lock_mut();
                    // let mut masters_lock = page.search_master_result.lock_mut();
                    med_lock.clear();
                    no_med_lock.clear();
                    // masters_lock.clear();

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

                    let date_now = js_now().date();
                    let (med, no_med): (Vec<OpdErOrderPharmacy>, Vec<OpdErOrderPharmacy>) = monitor.orders.clone().into_iter().partition(|i| {
                        if let (Some(min_dt), Some(max_dt)) = (i.min_order_date_time, i.max_order_date_time) {
                            max_dt.date() <= date_now && min_dt.date() <= date_now
                        } else {
                            false
                        }
                    });

                    med_lock.extend(med.into_iter().map(Rc::new));
                    no_med_lock.extend(no_med.into_iter().map(Rc::new));
                    // masters_lock.extend(monitor.order_masters.into_iter().map(Rc::new));
                }
                Err(e) => {
                    app.alert_app_error(&e).await;
                }
            }
        }),
    );
}

fn render_order_table(source: MutableVec<Rc<OpdErOrderPharmacy>>, label: &str, app: Rc<App>) -> Dom {
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
                                html!("th", {.class("th-sm").attr("scope", "col").text("เตียง")}),
                                html!("th", {.class("th-sm").attr("scope", "col").children([
                                    text("เลขที่ใบ Order"),html!("br"),text("เวลาที่บันทึก"),
                                ])}),
                                html!("th", {.class("th-sm").attr("scope", "col").text("VN")}),
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

fn render_pharmacy_order(i: usize, row: Rc<OpdErOrderPharmacy>, app: Rc<App>) -> Dom {
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
            html!("td", {.class("text-truncate").text(&row.bedno.clone().unwrap_or_default())}),
            html!("td", {.class("text-center").text(
                &[&row.opd_er_order_master_id.to_string(), "(", &datetime_th_opt_relative(&row.order_date_time), ")"].concat()
            )}),
            html!("td", {.class("text-truncate").text(&row.vn.clone().unwrap_or_default())}),
            html!("td", {.class("text-truncate").text(&row.hn.clone().unwrap_or_default())}),
            html!("td", {.children([
                html!("div", {
                    .class(class::TRUNC_BOLD)
                    .attr("title",&row.fullname.clone().unwrap_or_default())
                    .text(&row.fullname.clone().unwrap_or_default())
                }),
                html!("div", {
                    .class("text-truncate")
                    .style("white-space","pre-wrap")
                    .attr("title",&row.note.clone().unwrap_or_default())
                    .text(&row.note.clone().unwrap_or_default())
                }),
            ])}),
            html!("td", {.class("text-truncate").text(&age_str)}),
            html!("td", {
                .text(&[
                    datetime_th_opt_relative(&row.min_order_date_time),
                    datetime_th_opt_relative(&row.max_order_date_time),
                ].join( " - "))
            }),
            html!("td", {
                .class("text-center")
                .apply(|dom| {
                    match row.er_patient_status_id.unwrap_or_default() {
                        7 => {
                            dom.child(html!("i", {.class(class::FA_HOUSE)}))
                        }
                        10 => {
                            dom.child(html!("i", {.class(class::FA_BED)}))
                        }
                        _ => dom,
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
            let route = Route::OpdErMain {
                view_by: String::from("pharmacist"),
                opd_er_order_master_id: row.opd_er_order_master_id,
                tab: Tab::Order.str().to_owned(),
                id: 0,
            };
            if route.has_permission(app.state()) {
                dom.event(move |_: events::Click| {
                    route.hard_redirect();
                })
                // onclick="onclickOrderTableRow(<?=htmlspecialchars($row['opd_er_order_master_id'])
            } else {
                dom
            }
        })
    })
}
