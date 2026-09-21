use dominator::{Dom, clone, events, html, with_node};
use futures_signals::{
    map_ref,
    signal::{Mutable, SignalExt},
    signal_vec::{MutableVec, SignalVecExt},
};
use std::rc::Rc;
use web_sys::HtmlInputElement;

use kphis_model::search::searchbox::HospSearchBox;
use kphis_ui_app::App;
use kphis_ui_core::class;

/// GET `EndPoint::SearchBoxHospText`
#[derive(Default)]
pub struct HospSearchboxCpn {
    text: Mutable<String>,
    results: MutableVec<Rc<HospSearchBox>>,
    status_text: Mutable<Option<String>>,

    pub selected_result: Mutable<Option<Rc<HospSearchBox>>>,
}

impl HospSearchboxCpn {
    pub fn new(value: Mutable<Option<Rc<HospSearchBox>>>) -> Rc<Self> {
        let text = value.lock_ref().as_ref().map(|v| v.text()).unwrap_or_default();
        Rc::new(Self {
            text: Mutable::new(text),
            selected_result: value,
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
                    // GET `EndPoint::SearchBoxHospText`
                    match HospSearchBox::call_api_get(&search_text, app.state()).await {
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

    pub fn render(page: Rc<Self>, app: Rc<App>, changed: Mutable<bool>, can_save: bool, not_empty: bool) -> Dom {
        html!("div", {
            .child(html!("div", {
                .style("position","relative")
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
                                .attr("placeholder", "กรอกอย่างน้อย 3 ตัวอักษรเพื่อค้นหา")
                                .attr("autocomplete", "off")
                                .apply_if(!can_save, |d| d.attr("disabled",""))
                                .focused(page.text.lock_ref().is_empty())
                                .apply_if(not_empty, |d| d.class_signal("border-danger", page.selected_result.signal_ref(|opt| opt.is_none())))
                                .prop_signal("value", page.selected_result.signal_ref(|opt| opt.as_ref().map(|res| res.text()).unwrap_or_default()))
                                .with_node!(element => {
                                    .apply(|dom| {
                                        let load_mut = Mutable::new(false);
                                        dom.event(clone!(page, load_mut => move |_: events::Input| {
                                            let search_text = element.value();
                                            if search_text.chars().count() > 2 {
                                                page.text.set_neq(search_text);
                                                load_mut.set(true);
                                            } else if search_text.is_empty() {
                                                page.results.lock_mut().clear();
                                                page.status_text.set_neq(None);
                                            } else {
                                                page.status_text.set_neq(Some(String::from("กรอกอย่างน้อย 3 ตัวอักษรเพื่อค้นหา")));
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
                                        //         if search_text.chars().count() > 2 {
                                        //             page.text.set_neq(search_text);
                                        //             load_mut.set(true);
                                        //         } else if search_text.is_empty() {
                                        //             page.results.lock_mut().clear();
                                        //             page.status_text.set_neq(None);
                                        //         } else {
                                        //             page.status_text.set_neq(Some(String::from("กรอกอย่างน้อย 3 ตัวอักษรเพื่อค้นหา")));
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
                        ])
                        .apply_if(can_save, |dom| { dom
                            .child_signal(page.text.signal_cloned().map(clone!(page, changed => move |text| {
                                (!text.is_empty()).then(|| {
                                    html!("button", {
                                        .class(class::BTN_SM_GRAY)
                                        .attr("type", "button")
                                        .child(html!("i", {.class(class::FA_X)}))
                                        .event(clone!(page, changed => move |_: events::Click| {
                                            page.text.set_neq(String::new());
                                            page.results.lock_mut().clear();
                                            page.selected_result.set(None);
                                            changed.set_neq(true);
                                        }))
                                    })
                                })
                            })))
                        })
                    }),
                    html!("div", {
                        .style("position","absolute")
                        .style("max-height","250px")
                        .style("width", "100%")
                        .style("border-width","thin")
                        .style("border-right","1px solid lightgrey")
                        .style("overflow-y","auto")
                        .style("z-index","3")
                        .child(html!("table", {
                            //.attr("id", "common-searchbox-hosp-table")
                            .class(class::TABLE_STRIP)
                            .style("margin-bottom","0")
                            .children([
                                html!("thead", {
                                    .visible(false)
                                    .child(html!("tr", {
                                        .child(html!("th", {
                                            .attr("scope", "col")
                                            .text("Name")
                                        }))
                                    }))
                                }),
                                html!("tbody", {
                                    //.attr("id", "hosp-searchbox-tbody")
                                    .child_signal(page.status_text.signal_cloned().map(|opt| {
                                        opt.as_ref().map(|text| {
                                            html!("tr", {
                                                .child(html!("td", {
                                                    .attr("colspan", "2")
                                                    .text(text)
                                                }))
                                            })
                                        })
                                    }))
                                    .children_signal_vec(page.results.signal_vec_cloned().map(move |result| {
                                        html!("tr", {
                                            .style("cursor","pointer")
                                            .child(html!("td", {
                                                .child(html!("span", {.text(&result.text())}))
                                                .apply_if(result.addrname.is_some(), |dom| {
                                                    dom.child(html!("div", {
                                                        .child(html!("small", {.text(&result.addrname.clone().unwrap_or_default())}))
                                                    }))
                                                })
                                            }))
                                            .event(clone!(page, result, changed => move |_: events::Click| {
                                                page.text.set_neq(result.text());
                                                page.selected_result.set(Some(result.clone()));
                                                page.results.lock_mut().clear();
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
