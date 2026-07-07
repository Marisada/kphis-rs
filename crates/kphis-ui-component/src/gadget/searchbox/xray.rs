// common-searchbox-xray.php

use dominator::{Dom, clone, events, html, with_node};
use futures_signals::{
    map_ref,
    signal::{Mutable, SignalExt},
    signal_vec::{MutableVec, SignalVecExt},
};
use std::rc::Rc;
use web_sys::HtmlInputElement;

use kphis_model::search::searchbox::XraySearchbox;
use kphis_ui_app::App;
use kphis_ui_core::class;

use crate::order::OrderItemMutable;

/// GET `EndPoint::SearchBoxXrayText`
#[derive(Default)]
pub struct XraySearchboxCpn {
    text: Mutable<String>,
    results: MutableVec<Rc<XraySearchbox>>,
    status_text: Mutable<Option<String>>,
}

impl XraySearchboxCpn {
    pub fn new() -> Rc<Self> {
        Rc::new(Self {
            status_text: Mutable::new(Some(String::from("กรอกข้อความเพื่อค้นหา"))),
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
                    // GET `EndPoint::SearchBoxXrayText`
                    match XraySearchbox::call_api_get(&search_text, app.state()).await {
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

    pub fn render(
        pre_order_master_id: Option<u32>,
        page: Rc<Self>,
        display_mutable: Mutable<bool>,
        items: MutableVec<Rc<OrderItemMutable>>,
        focus: Mutable<Option<u32>>,
        changed: Mutable<bool>,
        app: Rc<App>,
    ) -> Dom {
        html!("div", {
            .child(html!("div", {
                .class(class::CARD_TW_T_CYANS)
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
                                .attr("placeholder", "กรอกข้อความเพื่อค้นหา")
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
                                .class(class::BTN_SM_R_RED)
                                .attr("type", "button")
                                .child(html!("i", {.class(class::FA_X)}))
                                .event(clone!(display_mutable => move |_: events::Click| {
                                    display_mutable.set_neq(false);
                                }))
                            }),
                        ])
                    }),
                    html!("div", {
                        .style("height","250px")
                        .style("border-width","thin")
                        .style("overflow-y","auto")
                        .child(html!("table", {
                            //.attr("id", "common-searchbox-lab-table")
                            .class(class::TABLE_STRIP)
                            .children([
                                html!("thead", {
                                    .visible(false)
                                    .child(html!("tr", {
                                        .children([
                                            html!("th", {
                                                .attr("scope", "col")
                                                .text("Group")
                                            }),
                                            html!("th", {
                                                .attr("scope", "col")
                                                .text("Name")
                                            }),
                                        ])
                                    }))
                                }),
                                html!("tbody", {
                                    //.attr("id", "xray-searchbox-tbody")
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
                                            .children([
                                                html!("td", {
                                                    .class("fw-bold")
                                                    .text(&result.group_name.clone().unwrap_or_default())
                                                }),
                                                html!("td", {
                                                    .class("fw-bold")
                                                    .text(&result.xray_items_name.clone().unwrap_or_default())
                                                }),
                                            ])
                                            .event(clone!(items, display_mutable, focus, changed, result => move |_: events::Click| {
                                                let new = [result.group_name.clone().unwrap_or_default(), result.xray_items_name.clone().unwrap_or_default()].join(" ");
                                                let mut lock = items.lock_mut();
                                                // Always init a new box
                                                if let Some(item) = lock.last() && item.order_item_detail.get_cloned().is_empty() {
                                                    item.order_item_detail.set(new);
                                                // // Use old box, separate by comma (if not empty)
                                                // if let Some(item) = lock.last() {
                                                //     let old = item.order_item_detail.get_cloned();
                                                //     if old.is_empty() {
                                                //         item.order_item_detail.set(new);
                                                //     } else {
                                                //         item.order_item_detail.set([old, new].join(", "));
                                                //     }
                                                    focus.set(Some(item.id));
                                                } else {
                                                    let order_item = OrderItemMutable::new("xray", pre_order_master_id);
                                                    focus.set(Some(order_item.id));
                                                    order_item.order_item_detail.set(new);
                                                    lock.push_cloned(order_item);
                                                }
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
