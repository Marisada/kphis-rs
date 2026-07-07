use dominator::{Dom, clone, events, html, with_node};
use futures_signals::{
    signal::{Mutable, SignalExt},
    signal_vec::{MutableVec, SignalVecExt},
};
use std::{rc::Rc, sync::Arc};
use web_sys::HtmlInputElement;

use kphis_drg_worker::drg::model::I9vx;
use kphis_model::timer::Timeout;
use kphis_ui_app::App;
use kphis_ui_core::class;
use kphis_util::util::{icd_dash, icd9_dot, is_icd9_without_dot};

use super::{red_keywords_in_icd_dot, red_keywords_in_sentense};

#[derive(Default)]
pub struct ProcSearchboxCpn {
    is_loading: Mutable<bool>,
    timer_handle: Mutable<Option<i32>>,

    pub text: Mutable<String>,
    pub results: MutableVec<(Arc<I9vx>, f32, u8)>,
    pub status_text: Mutable<Option<String>>,
    pub selected_result: Mutable<Option<Arc<I9vx>>>,
}

impl ProcSearchboxCpn {
    pub fn new(value: Mutable<Option<Arc<I9vx>>>) -> Rc<Self> {
        let text = value.lock_ref().as_ref().map(|v| [&v.code, " : ", &v.desc].concat()).unwrap_or_default();
        Rc::new(Self {
            text: Mutable::new(text),
            selected_result: value,
            ..Default::default()
        })
    }

    fn load_data(page: Rc<Self>, app: Rc<App>) {
        app.async_load(
            false,
            clone!(app, page => async move {
                let thread = app.drg_worker().await;
                let search_text = page.text.get_cloned();
                if !search_text.is_empty() {
                    let bytes = if let Some(code) = find_icd9(&search_text) {
                        thread.search_proc_code_prefix(code.to_ascii_uppercase()).await
                    } else {
                        thread.search_proc_desc(search_text).await
                    };
                    let results = bitcode::decode::<Vec<(Arc<I9vx>, f32, u8)>>(&bytes).unwrap_or_default();
                    if results.is_empty() {
                        page.results.lock_mut().clear();
                        page.status_text.set(Some(String::from("ไม่พบรายการที่ค้นหา")));
                    } else {
                        page.results.lock_mut().replace_cloned(results);
                        page.status_text.set(None);
                    }
                }
            }),
        )
    }

    pub fn render(page: Rc<Self>, app: Rc<App>, changed: Mutable<bool>) -> Dom {
        html!("div", {
            .future(page.selected_result.signal_cloned().for_each(clone!(page => move |opt| {
                if let Some(selected) = opt {
                    page.text.set_neq([&icd9_dot(&selected.code), " : ", &selected.desc].concat());
                }
                async {}
            })))
            .style("position","relative")
            .style("width", "100%")
            // .apply_if(parent.is_none(), |dom| dom.style("padding-left","40px"))
            .child(html!("div", {
                .class(class::INPUT_GROUP)
                .style("user-select","none")
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
                        .prop_signal("value", page.text.signal_cloned())
                        .with_node!(element => {
                            .event(clone!(app, page, element => move |_: events::Input| {
                                let search_text = element.value().replace('.', "");
                                let ready = if search_text.chars().count() > 2 {
                                    page.status_text.set_neq(None);
                                    true
                                } else if search_text.is_empty() {
                                    page.results.lock_mut().clear();
                                    page.status_text.set_neq(None);
                                    false
                                } else {
                                    page.status_text.set_neq(Some(String::from("กรอกอย่างน้อย 3 ตัวอักษรเพื่อค้นหา")));
                                    false
                                };
                                if ready {
                                    let is_icd_9 = find_icd9(&search_text).is_some();
                                    page.text.set_neq(search_text);
                                    if is_icd_9 {
                                        if let Some(handle) = page.timer_handle.get() {
                                            Timeout::manual_drop(handle);
                                        }
                                        page.is_loading.set(false);
                                        Self::load_data(page.clone(), app.clone());
                                    } else {
                                        page.is_loading.set_neq(true);
                                    }
                                } else {
                                    page.results.lock_mut().clear();
                                }
                            }))
                        })
                        .future(page.is_loading.signal().for_each(clone!(app, page => move |loading| {
                            if loading {
                                let wait = Timeout::new(500, clone!(app, page => move || {
                                    Self::load_data(page, app);
                                }));
                                // prevent multiple keyup
                                if let Some(handle) = page.timer_handle.get() {
                                    Timeout::manual_drop(handle);
                                }
                                page.timer_handle.set(Some(wait.handle()));
                                wait.forget();
                                page.is_loading.set(false);
                            }
                            async {}
                        })))
                    })
                ])
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
            }))
            .child(html!("div", {
                .style("position","absolute")
                .style("max-height","250px")
                .style("width", "100%")
                .style("border-width","thin")
                .style("border-right","1px solid lightgrey")
                .style("overflow-y","auto")
                .style("z-index","3")
                .child(html!("table", {
                    //.attr("id", "common-searchbox-dx-table")
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
                            //.attr("id", "dx-searchbox-tbody")
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
                            .children_signal_vec(page.results.signal_vec_cloned().map(clone!(page, changed => move |tuple| {
                                let txt = page.text.lock_ref();
                                let result = &tuple.0;
                                let col = tuple.2;
                                // col: 1=icd9, 2=desc
                                html!("tr", {
                                    .child(html!("td", {
                                        .style("cursor","pointer")
                                        .child(html!("div", {
                                            .children([
                                                html!("span", {
                                                    .class("fw-bold")
                                                    .apply(|dom| {
                                                        let icd9 = icd_dash(&icd9_dot(&result.code), result.is_valid);
                                                        if col == 1 {
                                                            // dom.children(red_chars_in_words(&txt, &icd9))
                                                            dom.children(red_keywords_in_icd_dot(true, &txt, &icd9))
                                                        } else {
                                                            dom.text(&icd9)
                                                        }
                                                    })
                                                }),
                                                html!("span", {.text(" : ")}),
                                                html!("span", {
                                                    .apply(|dom| {
                                                        if col == 2 {
                                                            // dom.children(red_chars_in_words(&txt, &result.desc))
                                                            dom.children(red_keywords_in_sentense(&txt, &result.desc))
                                                        } else {
                                                            dom.text(&result.desc)
                                                        }
                                                    })
                                                }),
                                            ])
                                            // .apply_if(result.is_tm, |dom|
                                            //     dom.child(html!("span", {.class(class::BADGE_GOLD_R).text("TM")}))
                                            // )
                                            .apply_if(!result.is_valid, |dom|
                                                dom.child(html!("i", {.class(class::ALIGN_MIDDLE_R).class(class::FA_R_ARROW_CIRCLE)}))
                                            )
                                        }))
                                        .event(clone!(page, result, changed => move |_: events::Click| {
                                            if result.is_valid {
                                                page.selected_result.set(Some(result.clone()));
                                                page.results.lock_mut().clear();
                                                changed.set_neq(true);
                                            } else {
                                                page.text.set_neq(result.code.to_owned());
                                                page.is_loading.set_neq(true);
                                            }
                                        }))
                                    }))
                                })
                            })))
                        }),
                    ])
                }))
            }))
        })
    }
}

fn find_icd9(search_text: &str) -> Option<String> {
    let no_dot = search_text.replace('.', "");
    let split = no_dot.split(" : ").collect::<Vec<&str>>();
    if split.len() > 1 {
        is_icd9_without_dot(split[0]).then(|| split[0].to_owned())
    } else {
        is_icd9_without_dot(search_text).then(|| search_text.to_owned())
    }
}
