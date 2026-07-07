use dominator::{Dom, DomBuilder, EventOptions, clone, events, html, is_window_loaded, link, with_node};
use futures_signals::{
    map_ref,
    signal::{Mutable, SignalExt},
    signal_vec::{MutableVec, SignalVecExt},
};
use std::{ops::Deref, rc::Rc};
use wasm_bindgen::JsCast;
use web_sys::{HtmlAudioElement, HtmlButtonElement, HtmlInputElement, HtmlSelectElement, HtmlTableCellElement};

use kphis_model::{
    endpoint::EndPoint,
    fetch::Method,
    route::Route,
    score::Scores,
    search::ipd_search_patient_nurse::{IpdSearchPatientNurseRequest, IpdSearchPatientNurseResponse},
    tab::Tab,
};
use kphis_ui_app::App;
use kphis_ui_component::modal::ipd_passcode::IpdPasscodeForm;
use kphis_ui_core::{binding::NiceSelect, class, doms, mixins};
use kphis_util::{
    datetime::{datetime_8601, datetime_th_opt_relative, datetime_th_relative},
    util::{opt_zero_none, str_some},
};

#[derive(Default, Clone, PartialEq)]
enum SortBy {
    #[default]
    BedNo,
    An,
    Hn,
    Name,
    Age,
    RegDateTime,
    MaxVsDateTime,
    MaxFcNoteType,
    MaxOrderDateTime,
    MaxFcNoteDateTime,
}

use crate::NEW_ORDER;

/// - GET `EndPoint::SearchNurse`
/// - GET `EndPoint::IpdPasscode` (IpdPasscodeForm)
/// - POST `EndPoint::IpdPasscode` (IpdPasscodeForm)
#[derive(Clone, Default)]
pub struct IpdSearchPatientNursePage {
    doctor_in_charge: Mutable<String>,
    patient: Mutable<String>,
    passcode: Mutable<String>,
    search_result: MutableVec<Rc<IpdSearchPatientNurseResponse>>,

    timer_handle: Mutable<Option<i32>>,
    timer_second: Mutable<f32>,

    // last_order_time: Mutable<Option<PrimitiveDateTime>>,
    play_sound: Mutable<bool>,

    sorted_by: Mutable<SortBy>,
    is_desc: Mutable<bool>,

    changed: Mutable<bool>,
}

impl IpdSearchPatientNursePage {
    pub fn new() -> Rc<Self> {
        Rc::new(Self::default())
    }

    fn sortable_mixin(sort_by: SortBy, page: Rc<Self>) -> impl FnOnce(DomBuilder<HtmlTableCellElement>) -> DomBuilder<HtmlTableCellElement> {
        #[inline]
        move |dom| {
            with_node!(dom, element => {
                .style("cursor","pointer")
                .child_signal(map_ref! {
                    let is_this = page.sorted_by.signal_ref(clone!(sort_by => move |sb| *sb == sort_by)),
                    let is_desc = page.is_desc.signal() =>
                    is_this.then(|| {
                        html!("i", {
                            .class("ms-1")
                            .class(if *is_desc {
                                class::FA_UP91
                            } else {
                                class::FA_DOWN19
                            })
                        })
                    })
                })
                .event(clone!(sort_by => move |_:events::Click| {
                    if page.sorted_by.get_cloned() != sort_by {
                        page.sorted_by.set(sort_by.clone());
                        page.is_desc.set_neq(false);
                    } else {
                        page.is_desc.set(!page.is_desc.get());
                    }
                    page.sort();
                }))
            })
        }
    }

    fn sort(&self) {
        let mut items = self.search_result.lock_ref().to_vec();
        if self.is_desc.get() {
            match self.sorted_by.get_cloned() {
                SortBy::BedNo => items.sort_by(|a, b| b.bedno.cmp(&a.bedno)),
                SortBy::An => items.sort_by(|a, b| b.an.cmp(&a.an)),
                SortBy::Hn => items.sort_by(|a, b| b.hn.cmp(&a.hn)),
                SortBy::Name => items.sort_by(|a, b| b.fullname.cmp(&a.fullname)),
                SortBy::Age => items.sort_by(|a, b| b.age_y.cmp(&a.age_y).then(b.age_m.cmp(&a.age_m)).then(b.age_d.cmp(&a.age_d))),
                SortBy::RegDateTime => items.sort_by(|a, b| b.regdatetime.cmp(&a.regdatetime)),
                SortBy::MaxVsDateTime => items.sort_by(|a, b| {
                    b.ews_concat
                        .as_ref()
                        .and_then(|concat| concat.split('|').next())
                        .and_then(|s| datetime_8601(s))
                        .cmp(&a.ews_concat.as_ref().and_then(|concat| concat.split('|').next()).and_then(|s| datetime_8601(s)))
                }),
                SortBy::MaxFcNoteType => items.sort_by(|a, b| b.max_fcnote_patient_type.cmp(&a.max_fcnote_patient_type)),
                SortBy::MaxOrderDateTime => items.sort_by(|a, b| b.max_order_datetime.cmp(&a.max_order_datetime)),
                SortBy::MaxFcNoteDateTime => items.sort_by(|a, b| b.max_fcnote_datetime.cmp(&a.max_fcnote_datetime)),
            }
        } else {
            match self.sorted_by.get_cloned() {
                SortBy::BedNo => items.sort_by(|a, b| a.bedno.cmp(&b.bedno)),
                SortBy::An => items.sort_by(|a, b| a.an.cmp(&b.an)),
                SortBy::Hn => items.sort_by(|a, b| a.hn.cmp(&b.hn)),
                SortBy::Name => items.sort_by(|a, b| a.fullname.cmp(&b.fullname)),
                SortBy::Age => items.sort_by(|a, b| a.age_y.cmp(&b.age_y).then(a.age_m.cmp(&b.age_m)).then(a.age_d.cmp(&b.age_d))),
                SortBy::RegDateTime => items.sort_by(|a, b| a.regdatetime.cmp(&b.regdatetime)),
                SortBy::MaxVsDateTime => items.sort_by(|a, b| {
                    a.ews_concat
                        .as_ref()
                        .and_then(|concat| concat.split('|').next())
                        .and_then(|s| datetime_8601(s))
                        .cmp(&b.ews_concat.as_ref().and_then(|concat| concat.split('|').next()).and_then(|s| datetime_8601(s)))
                }),
                SortBy::MaxFcNoteType => items.sort_by(|a, b| a.max_fcnote_patient_type.cmp(&b.max_fcnote_patient_type)),
                SortBy::MaxOrderDateTime => items.sort_by(|a, b| a.max_order_datetime.cmp(&b.max_order_datetime)),
                SortBy::MaxFcNoteDateTime => items.sort_by(|a, b| a.max_fcnote_datetime.cmp(&b.max_fcnote_datetime)),
            }
        }
        self.search_result.lock_mut().replace_cloned(items);
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

    // ipd-nurse-search-patient-table.php
    fn submit(page: Rc<Self>, app: Rc<App>) {
        let request = IpdSearchPatientNurseRequest {
            ward: str_some(app.ward_select.get_cloned()),
            doctor_in_charge: str_some(page.doctor_in_charge.get_cloned()),
            patient: str_some(page.patient.get_cloned()),
            passcode: str_some(page.passcode.get_cloned()),
        };

        app.async_load(
            true,
            clone!(app, page => async move {
                // GET `EndPoint::SearchNurse`
                match IpdSearchPatientNurseResponse::call_api_get(&request, app.state()).await {
                    Ok(items) => {
                        page.play_sound.set_neq(items.iter().any(|item| item.nurse_not_accept_exists));
                        let mut lock = page.search_result.lock_mut();
                        lock.clear();
                        lock.extend(items.into_iter().map(Rc::new));
                        page.sorted_by.set(SortBy::BedNo);
                        page.is_desc.set_neq(false);
                    }
                    Err(e) => {
                        app.alert_app_error(&e).await;
                    }
                }
            }),
        );
    }

    pub fn render(page: Rc<Self>, app: Rc<App>) -> Dom {
        app.set_title("KPHIS - Search Patient");

        let (ward_select_option, doctor_select_option) = app
            .app_asset
            .lock_ref()
            .as_ref()
            .map(|asset| (asset.ward_select_option.clone(), asset.doctor_select_option.clone()))
            .unwrap_or_default();

        let allow_passcode =
            app.can_change_ward_passcode() && app.endpoint_is_allow(&Method::GET, &EndPoint::IpdPasscode, false) && app.endpoint_is_allow(&Method::POST, &EndPoint::IpdPasscode, false);

        html!("section", {
            .future(is_window_loaded().for_each(clone!(app, page => move |value| {
                if value {
                    if let Some(elm) = app.get_id("ward") {
                        NiceSelect::new_default(&elm);
                    }
                    if let Some(elm) = app.get_id("doctor_in_charge") {
                        NiceSelect::new_default(&elm);
                    }
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
                    Self::submit(page.clone(), app.clone());
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
                                // .style("width","250px")
                                doms::form_inline_group_sm(clone!(app, page => move |group| { group
                                    .children([
                                        doms::label_group_for("ward","แผนก"),
                                        html!("div", {
                                            .class(class::FLEX_GROW1)
                                            .child(html!("select" => HtmlSelectElement, {
                                                .class(class::FORM_CTRL_SM)
                                                .attr("id", "ward")
                                                .child(html!("option", {
                                                    .attr("value","")
                                                    .text("ทั้งหมด")
                                                }))
                                                .children(ward_select_option.iter().map(|option| {
                                                    doms::select_option(option, &app.ward_select.lock_ref())
                                                }))
                                                // .apply(mixins::string_value_select(app.ward_select.clone(), page.changed.clone()))
                                                .prop_signal("value", app.ward_select.signal_cloned())
                                                .with_node!(element => {
                                                    .event(clone!(app, page, element => move |_: events::Change| {
                                                        app.ward_select.set_neq(element.value());
                                                        app.to_local_storage();
                                                        page.changed.set_neq(true);
                                                    }))
                                                })
                                            }))
                                        }),
                                    ])
                                })),
                                doms::form_inline_group_sm(clone!(app, page => move |group| { group
                                    .children([
                                        doms::label_group_for("passcode","Passcode"),
                                        html!("input" => HtmlInputElement, {
                                            .attr("type", "text")
                                            .class(class::FORM_CTRL_SM)
                                            .attr("id", "passcode")
                                            .attr("autocomplete","off")
                                            .attr("length","4")
                                            .attr("size","4")
                                            // .apply(mixins::string_value(page.passcode.clone(), page.changed.clone()))
                                            .prop_signal("value", page.passcode.signal_cloned())
                                            .with_node!(element => {
                                                .event_with_options(&EventOptions::preventable(), clone!(page, element => move |event: events::KeyDown| {
                                                    if event.key() == "Enter" {
                                                        event.prevent_default();
                                                        page.passcode.set_neq(element.value());
                                                        page.changed.set_neq(true);
                                                    }
                                                }))
                                                .event(clone!(page => move |_: events::Change| {
                                                    page.passcode.set_neq(element.value());
                                                    page.changed.set_neq(true);
                                                }))
                                            })
                                        })
                                    ])
                                    // NURSE require it?
                                    .apply_if(allow_passcode, |dom| {
                                        dom.child(html!("button", {
                                            .attr("type", "button")
                                            .class(class::BTN_SM_BLUE)
                                            .attr("data-bs-toggle","modal")
                                            .attr("data-bs-target","#passcodeModal")
                                            .child(html!("i", {
                                                .class(class::FA_COG)
                                            }))
                                        }))
                                    })
                                })),
                                // .style("width","310px")
                                doms::form_inline_group_sm(clone!(app, page => move |group| { group
                                    .children([
                                        doms::label_group_for("doctor_in_charge","แพทย์เจ้าของไข้"),
                                        html!("div", {
                                            .class(class::FLEX_GROW1)
                                            .child(html!("select" => HtmlSelectElement, {
                                                .class(class::FORM_CTRL_SM)
                                                .attr("id", "doctor_in_charge")
                                                .child(html!("option", {
                                                    .attr("value","")
                                                    .text("ทั้งหมด")
                                                }))
                                                .children(doctor_select_option.iter().map(|option| {
                                                    doms::select_option(option, "")
                                                }))
                                                .apply(mixins::string_value_select(page.doctor_in_charge.clone(), page.changed.clone()))
                                            }))
                                        }),
                                        html!("button", {
                                            .attr("type", "button")
                                            .class(class::BTN_SM_RED)
                                            .child(html!("i", {.class(class::FA_X)}))
                                            .event(clone!(app, page => move |_: events::Click| {
                                                let no_doctor = page.doctor_in_charge.lock_ref().is_empty();
                                                if !no_doctor {
                                                    page.doctor_in_charge.set_neq(String::new());
                                                    if let Some(elm) = app.get_id("doctor_in_charge") {
                                                        NiceSelect::new_default_with_value(&elm,"");
                                                    }
                                                    page.changed.set_neq(true);
                                                }
                                            }))
                                            // onclick="setDoctorInchargeAsBlank()
                                        }),
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
                                            .apply(mixins::string_value_end(page.patient.clone(), page.changed.clone()))
                                        }),
                                        html!("button", {
                                            .attr("type", "button")
                                            .class(class::BTN_SM_GRAY)
                                            .child(html!("i", {.class(class::FA_SEARCH)}))
                                            .text(" ค้นหา")
                                            .event(clone!(page => move |_: events::Click| {
                                                page.changed.set_neq(true);
                                            }))
                                        })
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
                                                }),
                                                html!("button", {
                                                    .attr("type", "button")
                                                    .class(class::BTN_SM_GRAY)
                                                    .child(html!("i", {.class(class::FA_PLAY)}))
                                                    .text(" Test")
                                                    .event(clone!(page => move |_: events::Click| {
                                                        page.play_sound.set(true);
                                                    }))
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
                                }))
                            ])
                        })),
                        html!("div", {
                            .class("col-sm")
                            .child(doms::badge_info_center("หากค้นหาด้วย HN, AN, CID, ชื่อ-สกุล โปรแกรมจะแสดงเฉพาะ 200 รายการล่าสุด"))
                            .child_signal(page.search_result.signal_vec_cloned().len().map(|i| {
                                Some(doms::badge_count_with_limit(i, 200))
                            }))
                        }),
                    ])
                }))
            ])
            // /kphis-config-ipd-ward-passcode.php
            .apply_if(allow_passcode, |dom| { dom
                .child(html!("div", {
                    .class("modal")
                    .attr("id", "passcodeModal")
                    .attr("role","dialog")
                    .attr("tabindex", "-1")
                    .child(IpdPasscodeForm::render(IpdPasscodeForm::new(), app.clone()))
                }))
            })
            .child_signal(app.is_wide_screen_card_or_table().map(clone!(app, page => move |is_wide_card| {
                Some(match is_wide_card {
                    // NOT wide screen
                    None => {
                        html!("div", {
                            .class(class::ROW_COL_RESP4_G2)
                            .children_signal_vec(page.search_result.signal_vec_cloned().map(clone!(app => move |row| {
                                render_card(row, app.clone())
                            })))
                        })
                    }
                    // wide screen card
                    Some(true) => {
                        html!("div", {
                            .class(class::ROW_COL5_G2)
                            .children_signal_vec(page.search_result.signal_vec_cloned().map(clone!(app => move |row| {
                                render_card(row, app.clone())
                            })))
                        })
                    }
                    // wide screen table
                    Some(false) => {
                        doms::table_responsive(class::TABLE_STRIP, clone!(app, page => move |table| { table
                            .children([
                                html!("thead", {
                                    .child(html!("tr", {
                                        .class("text-center")
                                        .children([
                                            html!("th", {.class("th-sm").attr("scope","col").text("#")}),
                                            html!("th" => HtmlTableCellElement, {
                                                .class("th-sm").attr("scope","col").text("เตียง")
                                                .apply(Self::sortable_mixin(SortBy::BedNo, page.clone()))
                                            }),
                                            html!("th" => HtmlTableCellElement, {
                                                .class("th-sm").attr("scope","col").text("AN")
                                                .apply(Self::sortable_mixin(SortBy::An, page.clone()))
                                            }),
                                            html!("th" => HtmlTableCellElement, {
                                                .class("th-sm").attr("scope","col").text("HN")
                                                .apply(Self::sortable_mixin(SortBy::Hn, page.clone()))
                                            }),
                                            html!("th" => HtmlTableCellElement, {
                                                .class("th-sm").attr("scope","col").text("ชื่อ - สกุล")
                                                .apply(Self::sortable_mixin(SortBy::Name, page.clone()))
                                            }),
                                            html!("th" => HtmlTableCellElement, {
                                                .class("th-sm").attr("scope","col").text("อายุ")
                                                .apply(Self::sortable_mixin(SortBy::Age, page.clone()))
                                            }),
                                            html!("th", {.class("th-sm").attr("scope","col").text("แพทย์เจ้าของไข้")}),
                                            html!("th" => HtmlTableCellElement, {
                                                .class("th-sm").attr("scope","col").text("เวลาที่ Admit")
                                                .apply(Self::sortable_mixin(SortBy::RegDateTime, page.clone()))
                                            }),
                                            html!("th" => HtmlTableCellElement, {
                                                .class("th-sm").attr("scope","col").text("เวลาล่าสุด").child(html!("br")).text("Vital Sign")
                                                .apply(Self::sortable_mixin(SortBy::MaxVsDateTime, page.clone()))
                                            }),
                                            html!("th", {.class("th-sm").attr("scope","col").style("min-width","100px")
                                                .text("EWS/qSOFA/SIRS")
                                                // .text(&app.scores_table_header())
                                            }),
                                            html!("th" => HtmlTableCellElement, {
                                                .class("th-sm").attr("scope","col").text("Severity")
                                                .apply(Self::sortable_mixin(SortBy::MaxFcNoteType, page.clone()))
                                            }),
                                            html!("th" => HtmlTableCellElement, {
                                                .class("th-sm").attr("scope","col").text("เวลาล่าสุด").child(html!("br")).text("Order")
                                                .apply(Self::sortable_mixin(SortBy::MaxOrderDateTime, page.clone()))
                                            }),
                                            html!("th" => HtmlTableCellElement, {
                                                .class("th-sm").attr("scope","col").text("เวลาล่าสุด").child(html!("br")).text("Nurse Note")
                                                .apply(Self::sortable_mixin(SortBy::MaxFcNoteDateTime, page.clone()))
                                            }),
                                            html!("th", {.class("th-sm").attr("scope","col")
                                                .attr("title","ผลการติดตามอาการ ยังไม่ปกติ หรือยังติดตามไม่ครบตามเกณฑ์")
                                                .child(html!("i", {.class(class::FA_ALERT_RED)}))
                                            }),
                                            html!("th", {.class("th-sm").attr("scope","col")
                                                .attr("title","มีผลแลปที่ยังไม่ได้อ่าน")
                                                .child(html!("i", {.class(class::FA_FLASK)}))
                                            }),
                                            html!("th", {.class("th-sm").attr("scope","col")
                                                .attr("title","มีผล X-Ray ที่ยังไม่ได้อ่าน")
                                                .child(html!("i", {.class(class::FA_XRAY)}))
                                            }),
                                            html!("th", {.class("th-sm").attr("scope","col")
                                                .attr("title","Med Reconciliation").text("MR")
                                            }),
                                            html!("th", {.class("th-sm").attr("scope","col")
                                                .attr("title","แบบบันทึกการรับใหม่ผู้ป่วยใน")
                                                .text("Hx")
                                                //.child(html!("i", {.class(class::FA_NOTE_MED)}))
                                            }),
                                            html!("th", {.class("th-sm").attr("scope","col")
                                                .attr("title","การประเมินสภาพผู้ป่วยแรกรับและแบบแผนสุขภาพ")
                                                .child(html!("i", {.class(class::FA_NOTE_MED)}))
                                            }),
                                        ])
                                    }))
                                }),
                                html!("tbody", {
                                    .children_signal_vec(page.search_result.signal_vec_cloned().enumerate().map(move |(i,row)| {
                                        render_table(i.get().unwrap_or_default(), row, app.clone())
                                    }))
                                }),
                            ])
                        }))
                    }
                })
            })))
        })
    }
}

fn render_card(row: Rc<IpdSearchPatientNurseResponse>, app: Rc<App>) -> Dom {
    let fcnote_patient_types_select_options = app.app_asset.lock_ref().as_ref().map(|asset| asset.fcnote_patient_type_select_options.clone()).unwrap_or_default();

    let age_y = row.age_y.unwrap_or_default();
    let age_m = row.age_m.unwrap_or_default();
    let age_d = row.age_d.unwrap_or_default();

    let kphis_incharge_doctor_name_dom = html!("div", {
        .style("min-width","40%")
        .children([
            html!("span", {
                .class(class::BADGE_CYAN)
                .style("cursor","default")
                .text("แพทย์เจ้าของไข้")
            }),
            html!("div", {
                .class("small")
                .children(row.kphis_incharge_doctor_name.clone().map(|docs| {
                    docs.split(',').map(|doc| {
                        html!("div", {
                            .class(class::TRUNC_SM)
                            .text(doc)
                        })
                    }).collect::<Vec<Dom>>()
                }).unwrap_or_default())
            }),
        ])
    });

    let consult_reply_name_dom = html!("div", {
        .class("ps-2")
        .style("max-width","60%")
        .children([
            html!("span", {
                .class(class::BADGE_GOLD_L)
                .style("cursor","default")
                .text("แพทย์ผู้ตอบ Consult")
            }),
            html!("div", {
                .class("small")
                .children(row.consult_reply_name.clone().map(|reps| {
                    reps.split('|').map(|rep| {
                        html!("div", {
                            .class(class::TRUNC_SM)
                            .text(&rep.split(',').collect::<Vec<&str>>().join(" / "))
                        })
                    }).collect::<Vec<Dom>>()
                }).unwrap_or_default())
            }),
        ])
    });

    let monitor_dom = if row.need_monitor {
        html!("span", {
            .class("text-center")
            .child(html!("i", {.class(class::FA_ALERT_RED)}))
            .style("cursor","help")
            .attr("title","ผลการติดตามอาการ ยังไม่ปกติ หรือยังติดตามไม่ครบตามเกณฑ์")
        })
    } else {
        Dom::empty()
    };

    let med_rec_dom = if row.mr_unconfirmed_exists {
        html!("span", {
            .class(class::SMALL_BOLD_C)
            .class("text-danger")
            .style("cursor","help")
            .text("MR")
            .attr("title","มีรายการ MR ที่แพทย์ยังไม่ได้พิจารณา")
        })
    } else if row.mr_confirmed_exists {
        html!("span", {
            .class(class::SMALL_BOLD_C)
            .class("text-success")
            .style("cursor","help")
            .text("MR")
            .attr("title","แพทย์พิจารณาแล้ว")
        })
    } else {
        Dom::empty()
    };

    let (vs_datetime_opt, ews_dom, qsofa_dom, sirs_dom) = doms::badge_scores_and_vs_datetime(&Scores::from_concat(&row.ews_concat, row.birthday, app.state()));

    let lab_dom = if row.lab_unreaded_exists {
        html!("span", {
            .class(class::SMALL_BOLD_C)
            .class("text-danger")
            .style("cursor","help")
            .text("Lab")
            .attr("title","มีรายการแลปที่แพทย์ยังไม่ได้อ่าน")
        })
    } else if row.lab_unreported_exists {
        html!("span", {
            .class(class::SMALL_BOLD_C)
            .class("text-warning")
            .style("cursor","help")
            .text("Lab")
            .attr("title","มีรายการแลปที่รอผลตรวจ")
        })
    } else {
        Dom::empty()
    };

    let xray_dom = if row.xray_unreaded_exists {
        html!("span", {
            .class(class::SMALL_BOLD_C)
            .class("text-danger")
            .style("cursor","help")
            .text("X-Ray")
            .attr("title","มีรายการ X-Ray ที่แพทย์ยังไม่ได้อ่าน")
        })
    } else {
        Dom::empty()
    };

    let main_route = Route::IpdMain {
        view_by: String::from("nurse"),
        an: row.an.clone(),
        tab: Tab::Order.str().to_owned(),
        sub: String::new(),
        id: 0,
    };
    let allow_main_route = main_route.has_permission(app.state());

    html!("div", {
        .class("col")
        .child(html!("div", {
            .class(class::CARD_SHADOW)
            .children([
                html!("div", {
                    .class(class::CARD_HEAD_P2)
                    .children([
                        html!("span", {
                            .class("fw-bold")
                            .text(&[row.ward_name.clone().unwrap_or_default(), row.bedno.clone().unwrap_or_default()].join(" "))
                        }),
                        html!("span", {
                            .class("float-end")
                            .children([
                                html!("span", {.class(class::SMALL_R2).text(&["HN: ", &row.hn.clone().unwrap_or_default()].concat())}),
                                html!("span", {.class(class::SMALL_R2).text(&["AN: ", &row.an].concat())}),
                                doms::color_box_span(&row.max_fcnote_patient_type.as_ref().and_then(|key| {
                                    fcnote_patient_types_select_options.iter().find(|op| op.key.as_str() == key).map(|op| op.color.clone())
                                }).unwrap_or_default(), &row.max_fcnote_patient_type.clone().unwrap_or_default())
                            ])
                        }),
                    ])
                }),
                html!("div", {
                    .class(class::CARD_BODY_P2)
                    .apply(|dom| {
                        if row.stat_order_nurse_not_accept_exists {
                            dom.class("bg-danger-subtle")
                        } else if row.discharge_order_exists {
                            dom.class("bg-success-subtle")
                        } else if row.nurse_not_accept_exists {
                            dom.class("bg-warning-subtle")
                        } else {
                            dom
                        }
                    })
                    .children([
                        html!("div", {
                            .class("d-flex")
                            .children([
                                html!("div", {
                                    .apply(|dom| {
                                        let image_dom = doms::patient_image(&row.hn, "90px");
                                        if allow_main_route {
                                            dom.child(link!(main_route.string(), {
                                                .child(image_dom)
                                            }))
                                        } else {
                                            dom.child(image_dom)
                                        }
                                    })
                                }),
                                html!("div", {
                                    .class("ps-2")
                                    .style("width","calc(100% - 128px)")
                                    .apply(|dom| {
                                        if allow_main_route {
                                            dom.child(link!(main_route.string(), {
                                                .class(class::TRUNC_BOLD)
                                                .text(&row.fullname.clone().unwrap_or_default())
                                            }))
                                        } else {
                                            dom.child(html!("span", {
                                                .class(class::TRUNC_BOLD)
                                                .text(&row.fullname.clone().unwrap_or_default())
                                            }))
                                        }
                                    })
                                    .children([
                                        html!("div", {
                                            .class(class::SMALL_TRUNC)
                                            .child(html!("span", {
                                                .class("me-1")
                                                .text(&row.sex_name.clone().unwrap_or_default())
                                            }))
                                            .apply_if(age_y > 0, |dom| {
                                                dom.text(&[&age_y.to_string(), " ปี"].concat())
                                            })
                                            .apply_if(age_y == 0 && age_m > 0, |dom| {
                                                dom.text(&[&age_m.to_string(), " เดือน ", &age_d.to_string(), " วัน"].concat())
                                            })
                                            .apply_if(age_y == 0 && age_m == 0, |dom| {
                                                dom.text(&[&age_d.to_string(), " วัน"].concat())
                                            })
                                            .child(html!("span", {
                                                .text(" ")
                                                .text(&row.rtname.clone().unwrap_or(String::from("ไม่ระบุ")))
                                            }))
                                        }),
                                        html!("div", {
                                            .class(class::SMALL_TRUNC)
                                            .text("Admit ")
                                            .text(&row.regdatetime.as_ref().map(|dt| datetime_th_relative(dt)).unwrap_or(String::from("ไม่ระบุ")))
                                        }),
                                        html!("div", {
                                            .class(class::SMALL_TRUNC)
                                            .text("Order ")
                                            .apply_if(row.max_order_datetime.is_none(), |dom| dom.class("text-danger"))
                                            .text(&row.max_order_datetime.as_ref().map(|dt| datetime_th_relative(dt)).unwrap_or(String::from("ไม่มี")))
                                            .apply_if(row.stat_order_nurse_not_accept_exists, |dom| {
                                                dom.child(html!("span", {
                                                    .class(class::BADGE_RED_R)
                                                    .style("cursor","default")
                                                    .text("STAT")
                                                }))
                                            })
                                            .apply_if(row.nurse_not_accept_exists, |dom| {
                                                dom.child(html!("span", {
                                                    .class(class::BADGE_GOLD_R)
                                                    .style("cursor","default")
                                                    .text("ยังไม่รับ")
                                                }))
                                            })
                                            .apply_if(row.discharge_order_exists, |dom| {
                                                dom.child(html!("span", {
                                                    .class(class::BADGE_CYAN_R)
                                                    .style("cursor","default")
                                                    .text("D/C")
                                                }))
                                            })
                                        }),
                                        html!("div", {
                                            .class(class::SMALL_TRUNC)
                                            .children([
                                                html!("span", {
                                                    .text("VS ")
                                                    .apply_if(vs_datetime_opt.is_none(), |dom| dom.class("text-danger"))
                                                    .text(&vs_datetime_opt.map(|vs_dt| datetime_th_relative(&vs_dt)).unwrap_or(String::from("ไม่มี")))
                                                }),
                                                html!("span", {
                                                    .text(" Note ")
                                                    .apply_if(row.max_fcnote_datetime.is_none(), |dom| dom.class("text-danger"))
                                                    .text(&row.max_fcnote_datetime.as_ref().map(|dt| datetime_th_relative(dt)).unwrap_or(String::from("ไม่มี")))
                                                }),
                                            ])
                                        }),
                                        html!("div", {
                                            .apply(|dom| {
                                                let route = Route::IpdAdmissionNoteDr {an: row.an.clone()};
                                                let is_allow = route.has_permission(app.state());
                                                let child = html!("span", {
                                                    .class(class::BTN_SM_L)
                                                    .apply_if(!is_allow, |d| d.class("disabled"))
                                                    .apply(|d| if row.dr_admission_note_exists {
                                                        d.class("btn-outline-primary")
                                                    } else {
                                                        d.class("btn-outline-danger")
                                                    })
                                                    .text("Hx/PE")
                                                });

                                                if is_allow {
                                                    dom.child(link!(route.string(), {
                                                        .child(child)
                                                    }))
                                                } else {
                                                    dom.child(child)
                                                }
                                            })
                                            .apply(|dom| {
                                                let route = Route::IpdAdmissionNoteNurse {an: row.an.clone()};
                                                let is_allow = route.has_permission(app.state());
                                                let child = html!("span", {
                                                    .class(class::BTN_SM_L)
                                                    .apply_if(!is_allow, |d| d.class("disabled"))
                                                    .apply(|d| if row.nurse_admission_note_exists {
                                                        d.class("btn-outline-primary")
                                                    } else {
                                                        d.class("btn-outline-danger")
                                                    })
                                                    .text("ประเมินแรกรับ")
                                                });

                                                if is_allow {
                                                    dom.child(link!(route.string(), {
                                                        .child(child)
                                                    }))
                                                } else {
                                                    dom.child(child)
                                                }
                                            })
                                        }),
                                    ])
                                }),
                                html!("div", {
                                    .class(class::FLEX_COL)
                                    .child(ews_dom).child(qsofa_dom).child(sirs_dom).child(monitor_dom).child(med_rec_dom).child(lab_dom).child(xray_dom)
                                }),
                            ])
                        }),
                        html!("div", {
                            .class("d-flex")
                            .apply_if(row.kphis_incharge_doctor_name.is_some(), |dom| {
                                dom.child(kphis_incharge_doctor_name_dom)
                            })
                            .apply_if(row.consult_reply_name.is_some(), |dom| {
                                dom.child(consult_reply_name_dom)
                            })
                        }),
                    ])
                }),
            ])
        }))
    })
}

fn render_table(i: usize, row: Rc<IpdSearchPatientNurseResponse>, app: Rc<App>) -> Dom {
    let fcnote_patient_types_select_options = app.app_asset.lock_ref().as_ref().map(|asset| asset.fcnote_patient_type_select_options.clone()).unwrap_or_default();

    let age_y = row.age_y.unwrap_or_default();
    let age_m = row.age_m.unwrap_or_default();
    let age_d = row.age_d.unwrap_or_default();

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

    let (vs_datetime_opt, ews_dom, qsofa_dom, sirs_dom) = doms::badge_scores_and_vs_datetime(&Scores::from_concat(&row.ews_concat, row.birthday, app.state()));

    html!("tr", {
        .apply(|dom| {
            if row.stat_order_nurse_not_accept_exists {
                dom.class("table-danger")
            } else if row.discharge_order_exists {
                dom.class("table-success")
            } else if row.nurse_not_accept_exists {
                dom.class("table-warning")
            } else {
                dom
            }
        })
        .children([
            html!("td", {.class("text-center").text(&(i + 1).to_string())}),
            html!("td", {
                .child(html!("div", {.class("text-truncate").text(&row.bedno.clone().unwrap_or_default())}))
            }),
            html!("td", {
                .class("text-center")
                .child(html!("div", {.class("text-truncate").text(&row.an)}))
            }),
            html!("td", {
                .class("text-center")
                .child(html!("div", {.class("text-truncate").text(&row.hn.clone().unwrap_or_default())}))
            }),
            html!("td", {
                .attr("title", &row.fullname.clone().unwrap_or_default())
                .apply(|dom| {
                    let child = html!("div", {
                        .class(class::TRUNC_BOLD)
                        .style("max-width","162px")
                        .apply_if(row.wp_status > 0, |d| d.text("* "))
                        .text(&row.fullname.clone().unwrap_or_default())
                    });
                    let route = Route::IpdMain {
                        view_by: String::from("nurse"),
                        an: row.an.clone(),
                        tab: Tab::Order.str().to_owned(),
                        sub: String::new(),
                        id: 0,
                    };
                    if route.has_permission(app.state()) {
                        dom.child(link!(route.string(), {
                            .child(child)
                        }))
                    } else {
                        dom.child(child)
                    }
                })
            }),
            html!("td", {
                .child(html!("div", {
                    .class("text-truncate")
                    .apply_if(age_y > 0, |dom| {
                        dom.text(&[&age_y.to_string(), " ปี"].concat())
                    })
                    .apply_if(age_y == 0 && age_m > 0, |dom| {
                        dom.text(&[&age_m.to_string(), " เดือน ", &age_d.to_string(), " วัน"].concat())
                    })
                    .apply_if(age_y == 0 && age_m == 0, |dom| {
                        dom.text(&[&age_d.to_string(), " วัน"].concat())
                    })
                }))
            }),
            html!("td", {
                .attr("title", &row.kphis_incharge_doctor_name.clone().unwrap_or_default())
                .child(kphis_incharge_doctor_name_with_html)
            }),
            html!("td", {
                .class("text-center")
                .child(html!("div", {
                    .class("text-truncate")
                    .text(&datetime_th_opt_relative(&row.regdatetime))
                }))
            }),
            html!("td", {
                .class("text-center")
                .child(html!("div", {
                    .class("text-truncate")
                    .text(&datetime_th_opt_relative(&vs_datetime_opt))
                }))
            }),
            html!("td", {
                .class("text-center")
                .child(ews_dom).child(qsofa_dom).child(sirs_dom)
            }),
            html!("td", {
                .class("text-center")
                .child(html!("div", {
                    .child(doms::color_box_span(&row.max_fcnote_patient_type.as_ref().and_then(|key| {
                        fcnote_patient_types_select_options.iter().find(|op| op.key.as_str() == key).map(|op| op.color.clone())
                    }).unwrap_or_default(), &row.max_fcnote_patient_type.clone().unwrap_or_default()))
                }))
            }),
            html!("td", {
                .class("text-center")
                .child(html!("div", {
                    .class("text-truncate")
                    .text(&datetime_th_opt_relative(&row.max_order_datetime))
                }))
                .apply_if(row.stat_order_nurse_not_accept_exists, |dom| {
                    dom.child(html!("span", {
                        .class(class::BADGE_RED_R)
                        .style("cursor","default")
                        .text("STAT")
                    }))
                })
                .apply_if(row.nurse_not_accept_exists, |dom| {
                    dom.child(html!("span", {
                        .class(class::BADGE_GOLD_R)
                        .style("cursor","default")
                        .text("ยังไม่รับ")
                    }))
                })
                .apply_if(row.discharge_order_exists, |dom| {
                    dom.child(html!("span", {
                        .class(class::BADGE_CYAN_R)
                        .style("cursor","default")
                        .text("D/C")
                    }))
                })
            }),
            html!("td", {
                .class("text-center")
                .child(html!("div", {
                    .class("text-truncate")
                    .text(&datetime_th_opt_relative(&row.max_fcnote_datetime))
                }))
            }),
            html!("td", {
                .class("text-center")
                .child(html!("div", {
                    .apply_if(row.need_monitor, |dom| dom
                        .child(html!("i", {
                            .class(class::FA_ALERT_RED)
                            .attr("title","ผลการติดตามอาการ ยังไม่ปกติ หรือยังติดตามไม่ครบตามเกณฑ์")
                        }))
                    )
                }))
            }),
            html!("td", {
                .class("text-center")
                .child(html!("div", {
                    .apply(|dom| {
                        if row.lab_unreaded_exists {
                            dom.child(html!("i", {
                                .class(class::FA_CIRCLE_RED)
                                .attr("title","มีรายการ Lab ที่ยังไม่ได้อ่าน")
                            }))
                        } else if row.lab_unreported_exists {
                            dom.child(html!("i", {
                                .class(class::FA_HOURGLASS_GOLD)
                                .attr("title","มีรายการ Lab ที่ยังรอผล")
                            }))
                        } else {
                            dom
                        }
                    })
                }))
            }),
            html!("td", {
                .class("text-center")
                .child(html!("div", {
                    .apply_if(row.xray_unreaded_exists, |dom| dom
                        .child(html!("i", {
                            .class(class::FA_CIRCLE_RED)
                            .attr("title","มีรายการ X-Ray ที่ยังไม่ได้อ่าน")
                        }))
                    )
                }))
            }),
            html!("td", {
                .class("text-center")
                .child(html!("div", {
                    .apply(|dom| {
                        if row.mr_unconfirmed_exists {
                            dom.child(html!("i", {
                                .class(class::FA_CIRCLE_RED)
                                .attr("title","มีรายการ MR ที่แพทย์ยังไม่ได้พิจารณา")
                            }))
                        } else if row.mr_confirmed_exists {
                            dom.child(html!("i", {
                                .class(class::FA_CIRCLE_GREEN)
                                .attr("title","แพทย์พิจารณาแล้ว")
                            }))
                        } else {
                            dom
                        }
                    })
                }))
            }),
            html!("td", {
                .class("text-center")
                .attr("title","แบบบันทึกการรับใหม่ผู้ป่วยใน")
                .child(html!("div", {
                    .apply(|dom| {
                        let child = html!("i", {
                            .apply(|d| if row.dr_admission_note_exists {
                                d.class(class::FA_CHECK_GREEN)
                            } else {
                                d.class(class::FA_X_RED)
                            })
                        });
                        let route = Route::IpdAdmissionNoteDr {an: row.an.clone()};
                        if route.has_permission(app.state()) {
                            dom.child(link!(route.string(), {
                                .child(child)
                            }))
                        } else {
                            dom.child(child)
                        }
                    })
                }))
            }),
            html!("td", {
                .class("text-center")
                .attr("title","การประเมินสภาพผู้ป่วยแรกรับและแบบแผนสุขภาพ")
                .child(html!("div", {
                    .apply(|dom| {
                        let child = html!("i", {
                            .apply(|d| if row.nurse_admission_note_exists {
                                d.class(class::FA_CHECK_GREEN)
                            } else {
                                d.class(class::FA_X_RED)
                            })
                        });
                        let route = Route::IpdAdmissionNoteNurse {an: row.an.clone()};
                        if route.has_permission(app.state()) {
                            dom.child(link!(route.string(), {
                                .child(child)
                            }))
                        } else {
                            dom.child(child)
                        }
                    })
                }))
            }),
        ])
    })
}
