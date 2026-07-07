// common-searchbox-patient.php

use dominator::{Dom, clone, events, html, with_node};
use futures_signals::{
    map_ref,
    signal::{Mutable, SignalExt},
    signal_vec::{MutableVec, SignalVecExt},
};
use std::rc::Rc;
use web_sys::{DomRect, HtmlInputElement};

use kphis_model::search::searchbox::PatientSearchbox;
use kphis_ui_app::App;
use kphis_ui_core::{class, doms};

/// GET `EndPoint::SearchBoxPatientText`
#[derive(Default)]
pub struct PatientSearchboxCpn {
    text: Mutable<String>,
    results: MutableVec<Rc<PatientSearchbox>>,
    status_text: Mutable<Option<String>>,
}

impl PatientSearchboxCpn {
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
                    // GET `EndPoint::SearchBoxPatientText`
                    match PatientSearchbox::call_api_get(&search_text, app.state()).await {
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
                }
            }),
        )
    }

    pub fn render(page: Rc<Self>, display_mutable: Mutable<bool>, hn: Mutable<String>, ptname: Mutable<String>, anchor_rect: DomRect, changed: Mutable<bool>, app: Rc<App>) -> Dom {
        doms::under_box(anchor_rect, 900.0, super::BOX_HEIGHT, app.window_scroll_y(), move |bx| {
            bx.child(html!("div", {
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
                                .attr("placeholder", "กรอกข้อความเพื่อค้นหา (HN, ชื่อ นามสกุล, เลขประจำตัวประชาชน)")
                                .attr("autocomplete", "off")
                                .focused(true)
                                .with_node!(element => {
                                    .apply(|dom| {
                                        let load_mut = Mutable::new(false);
                                        dom.event(clone!(page, load_mut => move |_: events::Input| {
                                            let search_text = element.value();
                                            if search_text.is_empty() {
                                                page.results.lock_mut().clear();
                                                page.status_text.set_neq(Some(String::from("กรอกข้อความเพื่อค้นหา")));
                                            } else {
                                                page.text.set_neq(search_text);
                                                load_mut.set(true);
                                            }
                                        }))
                                        .future(map_ref!{
                                            let busy = app.loader_is_loading(),
                                            let load = load_mut.signal() =>
                                            !busy && *load
                                        }.for_each(clone!(app, page, load_mut => move |ready| {
                                            if ready {
                                                load_mut.set(false);
                                                Self::load_data(page.clone(), app.clone());
                                            }
                                            async {}
                                        })))
                                        // let timer_handle = Mutable::new(None);
                                        // dom.event(clone!(page, element, timer_handle, load_mut => move |_: events::KeyUp| {
                                        //     let wait = Timeout::new(1000, clone!(page, element, load_mut => move || {
                                        //         let search_text = element.value();
                                        //         if search_text.is_empty() {
                                        //             page.results.lock_mut().clear();
                                        //             page.status_text.set_neq(Some(String::from("กรอกข้อความเพื่อค้นหา")));
                                        //         } else {
                                        //             page.text.set_neq(search_text);
                                        //             load_mut.set(true);
                                        //         }
                                        //     }));
                                        //     // prevent multiple keyup
                                        //     if let Some(handle) = timer_handle.get() {
                                        //         Timeout::manual_drop(handle);
                                        //     }
                                        //     timer_handle.set(Some(wait.handle()));
                                        //     wait.forget();
                                        // }))
                                        // .event_with_options(&EventOptions::preventable(), clone!(timer_handle => move |event: events::KeyDown| {
                                        //     if let Some(handle) = timer_handle.get() {
                                        //         Timeout::manual_drop(handle);
                                        //     }
                                        //     if event.key() == "Enter" {
                                        //         event.prevent_default();
                                        //     }
                                        // }))
                                    })
                                })
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
                        //.attr("id", "common-searchbox-patient-table-div")
                        .class("bg-info-subtle")
                        .style("border-width","thin")
                        .style("overflow-y","auto")
                        .child(html!("table", {
                            //.attr("id", "common-searchbox-patient-table")
                            .class(class::TABLE_STRIP)
                            .children([
                                html!("thead", {
                                    .child(html!("tr", {
                                        .children([
                                            html!("th", {
                                                .attr("scope", "col")
                                                .text("HN")
                                            }),
                                            html!("th", {
                                                .attr("scope", "col")
                                                .text("ชื่อ-สกุล")
                                            }),
                                            html!("th", {
                                                .attr("scope", "col")
                                                .text("บิดา")
                                            }),
                                            html!("th", {
                                                .attr("scope", "col")
                                                .text("มารดา")
                                            }),
                                            html!("th", {
                                                .attr("scope", "col")
                                                .text("CID/Passport")
                                            }),
                                        ])
                                    }))
                                }),
                                html!("tbody", {
                                    //.attr("id", "patient-searchbox-tbody")
                                    .child_signal(page.status_text.signal_cloned().map(|opt| {
                                        opt.as_ref().map(|text| {
                                            html!("tr", {
                                                .child(html!("td", {
                                                    .attr("colspan", "6")
                                                    .text(text)
                                                }))
                                            })
                                        })
                                    }))
                                    .children_signal_vec(page.results.signal_vec_cloned().map(move |result| {
                                        html!("tr", {
                                            .style("cursor","pointer")
                                            .children([
                                                html!("td", {.text(&result.hn.clone().unwrap_or_default())}),
                                                html!("td", {.text(&result.ptname.clone().unwrap_or_default())}),
                                                html!("td", {.text(&result.fathername.clone().unwrap_or_default())}),
                                                html!("td", {.text(&result.mathername.clone().unwrap_or_default())}),
                                                html!("td", {.text(&[result.cid.clone().unwrap_or_default(), result.passport_no.clone().unwrap_or_default()].join(" "))}),
                                            ])
                                            .event(clone!(hn, ptname, display_mutable, result, changed => move |_: events::Click| {
                                                let hn_n = result.hn.clone().unwrap_or_default();
                                                hn.set_neq(hn_n.clone());
                                                ptname.set_neq([&hn_n, " (", &result.ptname.clone().unwrap_or_default(), ")"].concat());
                                                display_mutable.set(false);
                                                changed.set_neq(true);
                                            }))
                                        })
                                    }))
                                })
                            ])
                        }))
                    }),
                ])
            }))
        })
    }
}
