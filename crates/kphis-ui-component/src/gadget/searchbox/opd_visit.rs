// common-searchbox-opd-visit.php

use dominator::{Dom, EventOptions, clone, events, html, with_node};
use futures_signals::{
    map_ref,
    signal::{Mutable, SignalExt},
    signal_vec::{MutableVec, SignalVecExt},
};
use std::rc::Rc;
use web_sys::{DomRect, HtmlInputElement};

use kphis_model::{
    search::searchbox::{OpdVisitSearchType, OpdVisitSearchbox},
    timer::Timeout,
};
use kphis_ui_app::App;
use kphis_ui_core::{class, doms};
use kphis_util::datetime::date_and_time_th_opt_relative;

/// GET `EndPoint::SearchBoxOpdVisitModeText`
#[derive(Default)]
pub struct OpdVisitSearchboxCpn {
    loading: Mutable<bool>,

    load_mut: Mutable<bool>,
    timer_handle: Mutable<Option<i32>>,

    text: Mutable<String>,
    search_type: Mutable<OpdVisitSearchType>,
    text_hn: Mutable<String>,
    text_qn: Mutable<String>,
    text_vn: Mutable<String>,
    text_ptname: Mutable<String>,
    text_cid: Mutable<String>,
    results: MutableVec<Rc<OpdVisitSearchbox>>,
    status_text: Mutable<Option<String>>,
}

impl OpdVisitSearchboxCpn {
    pub fn new() -> Rc<Self> {
        Rc::new(Self {
            status_text: Mutable::new(Some(String::from("กรอกข้อความเพื่อค้นหา (HN, ชื่อ นามสกุล, เลขประจำตัวประชาชน)"))),
            ..Default::default()
        })
    }

    fn load_data(page: Rc<Self>, app: Rc<App>) {
        page.status_text.set_neq(Some(String::from("กำลังค้นหา...")));
        app.async_load(
            true,
            clone!(app, page => async move {
                let search_text = page.text.get_cloned();
                if !search_text.is_empty() {
                    page.loading.set_neq(true);
                    // GET `EndPoint::SearchBoxOpdVisitModeText`
                    match OpdVisitSearchbox::call_api_get(&search_text, page.search_type.get_cloned().string(), app.state()).await {
                        Ok(results) => {
                            let mut lock = page.results.lock_mut();
                            if !lock.is_empty() {
                                lock.clear();
                            }
                            if results.is_empty() {
                                page.status_text.set(Some(String::from("ไม่พบรายการที่ค้นหา")));
                            } else {
                                lock.extend(results.into_iter().map(Rc::new));
                                page.status_text.set(None);
                            }
                        }
                        Err(e) => {
                            page.results.lock_mut().clear();
                            page.status_text.set(Some(String::from("การเชื่อมต่อขัดข้อง")));
                            app.alert_app_error(&e).await;
                        }
                    }
                    page.loading.set_neq(false);
                }
            }),
        )
    }

    pub fn render(page: Rc<Self>, display_mutable: Mutable<bool>, new_vn: Mutable<String>, new_opd_visit_detail: Mutable<String>, anchor_rect: DomRect, changed: Mutable<bool>, app: Rc<App>) -> Dom {
        doms::under_box(anchor_rect, 900.0, super::BOX_HEIGHT, app.window_scroll_y(), move |bx| {
            bx
            .child(html!("div", {
                .future(map_ref!{
                    let busy = app.loader_is_loading(),
                    let load = page.load_mut.signal() =>
                    !busy && *load
                }.for_each(clone!(app, page => move |ready| {
                    if ready {
                        page.load_mut.set(false);
                        Self::load_data(page.clone(), app.clone());
                    }
                    async {}
                })))
                .class(class::CARD_TW_T_CYANS)
                .style("height", [&super::BOX_HEIGHT.to_string(),"px"].concat())
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
                                .attr("placeholder", "HN")
                                .attr("autocomplete", "off")
                                .focused(true)
                                .prop_signal("value", page.text_hn.signal_cloned())
                                .with_node!(element => {
                                    .event(clone!(page, element => move |_: events::KeyUp| {
                                        let wait = Timeout::new(500, clone!(page, element => move || {
                                            let search_text = element.value();

                                            page.text.set_neq(search_text.clone());
                                            page.text_hn.set_neq(search_text.clone());
                                            page.text_qn.set_neq(String::new());
                                            page.text_vn.set_neq(String::new());
                                            page.text_ptname.set_neq(String::new());
                                            page.text_cid.set_neq(String::new());
                                            page.search_type.set_neq(OpdVisitSearchType::Hn);
                                            if search_text.chars().count() > 4 {
                                                page.load_mut.set(true);
                                            } else {
                                                page.results.lock_mut().clear();
                                                page.status_text.set_neq(Some(String::from("กรอกข้อความอย่างน้อย 5 ตัวอักษรเพื่อค้นหา")));
                                            }
                                        }));
                                        // prevent multiple keyup
                                        if let Some(handle) = page.timer_handle.get() {
                                            Timeout::manual_drop(handle);
                                        }
                                        page.timer_handle.set(Some(wait.handle()));
                                        wait.forget();
                                    }))
                                    .event_with_options(&EventOptions::preventable(), clone!(page => move |event: events::KeyDown| {
                                        if let Some(handle) = page.timer_handle.get() {
                                            Timeout::manual_drop(handle);
                                        }
                                        if event.key() == "Enter" {
                                            event.prevent_default();
                                        }
                                    }))
                                })
                                // .attr("onkeydown", "if(event.keyCode == 13) {event.preventDefault();return false;}")
                                // .attr("oninput", "onkeyup_opd_visit_search_text(event, this)")
                            }),
                            html!("input" => HtmlInputElement, {
                                .attr("type", "text")
                                .class(class::FORM_CTRL_SM)
                                .attr("placeholder", "QN")
                                .attr("autocomplete", "off")
                                .prop_signal("value", page.text_qn.signal_cloned())
                                .with_node!(element => {
                                    .event(clone!(page, element => move |_: events::KeyUp| {
                                        let wait = Timeout::new(500, clone!(page, element => move || {
                                            let search_text = element.value();

                                            page.text.set_neq(search_text.clone());
                                            page.text_hn.set_neq(String::new());
                                            page.text_qn.set_neq(search_text.clone());
                                            page.text_vn.set_neq(String::new());
                                            page.text_ptname.set_neq(String::new());
                                            page.text_cid.set_neq(String::new());
                                            page.search_type.set_neq(OpdVisitSearchType::Qn);
                                            if !search_text.is_empty() {
                                                page.load_mut.set(true);
                                            } else {
                                                page.results.lock_mut().clear();
                                                page.status_text.set_neq(Some(String::from("กรอกข้อความเพื่อค้นหา")));
                                            }
                                        }));
                                        // prevent multiple keyup
                                        if let Some(handle) = page.timer_handle.get() {
                                            Timeout::manual_drop(handle);
                                        }
                                        page.timer_handle.set(Some(wait.handle()));
                                        wait.forget();
                                    }))
                                    .event_with_options(&EventOptions::preventable(), clone!(page => move |event: events::KeyDown| {
                                        if let Some(handle) = page.timer_handle.get() {
                                            Timeout::manual_drop(handle);
                                        }
                                        if event.key() == "Enter" {
                                            event.prevent_default();
                                        }
                                    }))
                                })
                                // .attr("onkeydown", "if(event.keyCode == 13) {event.preventDefault();return false;}")
                                // .attr("oninput", "onkeyup_opd_visit_search_text(event, this)")
                            }),
                            html!("input" => HtmlInputElement, {
                                .attr("type", "text")
                                .class(class::FORM_CTRL_SM)
                                .attr("placeholder", "VN")
                                .attr("autocomplete", "off")
                                .prop_signal("value", page.text_vn.signal_cloned())
                                .with_node!(element => {
                                    .event(clone!(page, element => move |_: events::KeyUp| {
                                        let wait = Timeout::new(500, clone!(page, element => move || {
                                            let search_text = element.value();

                                            page.text.set_neq(search_text.clone());
                                            page.text_hn.set_neq(String::new());
                                            page.text_qn.set_neq(String::new());
                                            page.text_vn.set_neq(search_text.clone());
                                            page.text_ptname.set_neq(String::new());
                                            page.text_cid.set_neq(String::new());
                                            page.search_type.set_neq(OpdVisitSearchType::Vn);
                                            if search_text.chars().count() > 4 {
                                                page.load_mut.set(true);
                                            } else {
                                                page.results.lock_mut().clear();
                                                page.status_text.set_neq(Some(String::from("กรอกข้อความอย่างน้อย 5 ตัวอักษรเพื่อค้นหา")));
                                            }
                                        }));
                                        // prevent multiple keyup
                                        if let Some(handle) = page.timer_handle.get() {
                                            Timeout::manual_drop(handle);
                                        }
                                        page.timer_handle.set(Some(wait.handle()));
                                        wait.forget();
                                    }))
                                    .event_with_options(&EventOptions::preventable(), clone!(page => move |event: events::KeyDown| {
                                        if let Some(handle) = page.timer_handle.get() {
                                            Timeout::manual_drop(handle);
                                        }
                                        if event.key() == "Enter" {
                                            event.prevent_default();
                                        }
                                    }))
                                })
                                // .attr("onkeydown", "if(event.keyCode == 13) {event.preventDefault();return false;}")
                                // .attr("oninput", "onkeyup_opd_visit_search_text(event, this)")
                            }),
                            html!("input" => HtmlInputElement, {
                                .attr("type", "text")
                                .class(class::FORM_CTRL_SM)
                                .attr("placeholder", "ชื่อ นามสกุล")
                                .attr("autocomplete", "off")
                                .prop_signal("value", page.text_ptname.signal_cloned())
                                .with_node!(element => {
                                    .event(clone!(page, element => move |_: events::KeyUp| {
                                        let wait = Timeout::new(500, clone!(page, element => move || {
                                            let search_text = element.value();

                                            page.text.set_neq(search_text.clone());
                                            page.text_hn.set_neq(String::new());
                                            page.text_qn.set_neq(String::new());
                                            page.text_vn.set_neq(String::new());
                                            page.text_ptname.set_neq(search_text.clone());
                                            page.text_cid.set_neq(String::new());
                                            page.search_type.set_neq(OpdVisitSearchType::PtName);
                                            if search_text.chars().count() > 4 {
                                                page.load_mut.set(true);
                                            } else {
                                                page.results.lock_mut().clear();
                                                page.status_text.set_neq(Some(String::from("กรอกข้อความอย่างน้อย 5 ตัวอักษรเพื่อค้นหา")));
                                            }
                                        }));
                                        // prevent multiple keyup
                                        if let Some(handle) = page.timer_handle.get() {
                                            Timeout::manual_drop(handle);
                                        }
                                        page.timer_handle.set(Some(wait.handle()));
                                        wait.forget();
                                    }))
                                    .event_with_options(&EventOptions::preventable(), clone!(page => move |event: events::KeyDown| {
                                        if let Some(handle) = page.timer_handle.get() {
                                            Timeout::manual_drop(handle);
                                        }
                                        if event.key() == "Enter" {
                                            event.prevent_default();
                                        }
                                    }))
                                })
                                // .attr("onkeydown", "if(event.keyCode == 13) {event.preventDefault();return false;}")
                                // .attr("oninput", "onkeyup_opd_visit_search_text(event, this)")
                            }),
                            html!("input" => HtmlInputElement, {
                                .attr("type", "text")
                                .class(class::FORM_CTRL_SM)
                                .attr("placeholder", "เลขประจำตัวประชาชน")
                                .attr("autocomplete", "off")
                                .prop_signal("value", page.text_cid.signal_cloned())
                                .with_node!(element => {
                                    .event(clone!(page, element => move |_: events::KeyUp| {
                                        let wait = Timeout::new(500, clone!(page, element => move || {
                                            let search_text = element.value();

                                            page.text.set_neq(search_text.clone());
                                            page.text_hn.set_neq(String::new());
                                            page.text_qn.set_neq(String::new());
                                            page.text_vn.set_neq(String::new());
                                            page.text_ptname.set_neq(String::new());
                                            page.text_cid.set_neq(search_text.clone());
                                            page.search_type.set_neq(OpdVisitSearchType::Cid);
                                            if search_text.chars().count() > 4 {
                                                page.load_mut.set(true);
                                            } else {
                                                page.results.lock_mut().clear();
                                                page.status_text.set_neq(Some(String::from("กรอกข้อความอย่างน้อย 5 ตัวอักษรเพื่อค้นหา")));
                                            }
                                        }));
                                        // prevent multiple keyup
                                        if let Some(handle) = page.timer_handle.get() {
                                            Timeout::manual_drop(handle);
                                        }
                                        page.timer_handle.set(Some(wait.handle()));
                                        wait.forget();
                                    }))
                                    .event_with_options(&EventOptions::preventable(), clone!(page => move |event: events::KeyDown| {
                                        if let Some(handle) = page.timer_handle.get() {
                                            Timeout::manual_drop(handle);
                                        }
                                        if event.key() == "Enter" {
                                            event.prevent_default();
                                        }
                                    }))
                                })
                                // .attr("onkeydown", "if(event.keyCode == 13) {event.preventDefault();return false;}")
                                // .attr("oninput", "onkeyup_opd_visit_search_text(event, this)")
                            }),
                            html!("button", {
                                .class(class::BTN_SM_RED)
                                .attr("type", "button")
                                .child(html!("i", {.class(class::FA_X)}))
                                .event(clone!(display_mutable => move |_: events::Click| {
                                    display_mutable.set_neq(false);
                                }))
                            }),
                        ])
                    }),
                    html!("div", {
                        //.attr("id", "common-searchbox-opd-visit-table-div")
                        .style("border-width","thin")
                        .style("overflow-y","auto")
                        .child(html!("table", {
                            //.attr("id", "common-searchbox-opd-visit-table")
                            .class(class::TABLE_STRIP)
                            .children([
                                html!("thead", {
                                    .child(html!("tr", {
                                        .children([
                                            html!("th", {.attr("scope", "col").text("เวลา(VN)")}),
                                            html!("th", {.attr("scope", "col").text("HN")}),
                                            html!("th", {.attr("scope", "col").text("ชื่อ-สกุล")}),
                                            html!("th", {.attr("scope", "col").text("บิดา")}),
                                            html!("th", {.attr("scope", "col").text("มารดา")}),
                                            html!("th", {.attr("scope", "col").text("CID/Passport")}),
                                        ])
                                    }))
                                }),
                                html!("tbody", {
                                    //.attr("id", "opd-visit-searchbox-tbody")
                                    .child_signal(page.status_text.signal_cloned().map(clone!(page => move |opt| {
                                        opt.as_ref().map(|text| {
                                            html!("tr", {
                                                .child(html!("td", {
                                                    .attr("colspan", "7")
                                                    .child_signal(page.loading.signal_cloned().map(|loading| {
                                                        loading.then(|| {
                                                            html!("div", {
                                                                .class(class::SPIN_SM_BLUE)
                                                                .attr("role","status")
                                                            })
                                                        })
                                                    }))
                                                    .text(&["\u{00a0}", text].concat())
                                                }))
                                            })
                                        })
                                    })))
                                    .children_signal_vec(page.results.signal_vec_cloned().map(move |result| {
                                        html!("tr", {
                                            .style("cursor","pointer")
                                            .children([
                                                html!("td", {
                                                    .children([
                                                        html!("div", {
                                                            .class("text-nowrap")
                                                            .text(&date_and_time_th_opt_relative(&result.vstdate, &result.vsttime))
                                                        }),
                                                        html!("div", {
                                                            .class("text-nowrap")
                                                            .text(&["(VN: ", &result.vn.clone().unwrap_or_default(),")"].concat())
                                                        }),
                                                    ])
                                                }),
                                                html!("td", {.text(&result.hn.clone().unwrap_or_default())}),
                                                html!("td", {.text(&result.ptname.clone().unwrap_or_default())}),
                                                html!("td", {.text(&result.fathername.clone().unwrap_or_default())}),
                                                html!("td", {.text(&result.mathername.clone().unwrap_or_default())}),
                                                html!("td", {.text(&[result.cid.clone().unwrap_or_default(), result.passport_no.clone().unwrap_or_default()].join(" "))}),
                                            ])
                                            .event(clone!(new_vn, new_opd_visit_detail, display_mutable, result, changed => move |_: events::Click| {
                                                let vn_n = result.vn.clone().unwrap_or_default();
                                                new_vn.set_neq(vn_n.clone());
                                                new_opd_visit_detail.set_neq(["VN: ", &vn_n, " HN: ", &result.hn.clone().unwrap_or_default(), " (", &result.ptname.clone().unwrap_or_default(), ")"].concat());
                                                display_mutable.set(false);
                                                changed.set_neq(true);
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
