// common-searchbox-ivfluid.php

use dominator::{Dom, clone, events, html, with_node};
use futures_signals::{
    map_ref,
    signal::{Mutable, SignalExt},
    signal_vec::{MutableVec, SignalVecExt},
};
use std::rc::Rc;
use web_sys::HtmlInputElement;

use kphis_model::search::searchbox::IvfluidSearchbox;
use kphis_ui_app::App;
use kphis_ui_core::{class, doms};

use super::dec_to_color;
use crate::order::{MedSearchable, OrderItemMutable};

/// GET `EndPoint::SearchBoxIvfluidText`
#[derive(Default)]
pub struct IvfluidSearchboxCpn {
    text: Mutable<String>,
    results: MutableVec<Rc<IvfluidSearchbox>>,
    status_text: Mutable<Option<String>>,
}

impl IvfluidSearchboxCpn {
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
                    // GET `EndPoint::SearchBoxIvfluidText`
                    match IvfluidSearchbox::call_api_get(&search_text, app.state()).await {
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

    pub fn render<T: MedSearchable + 'static>(pre_order_master_id: Option<u32>, page: Rc<Self>, order_form: Rc<T>, app: Rc<App>) -> Dom {
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
                                    })
                                })
                            }),
                            html!("button", {
                                .class(class::BTN_SM_R_RED)
                                .attr("type", "button")
                                .child(html!("i", {.class(class::FA_X)}))
                                .event(clone!(order_form => move |_: events::Click| {
                                    order_form.display_ivfluid_searchbox().set_neq(false);
                                }))
                            }),
                        ])
                    }),
                    html!("div", {
                        .style("height","250px")
                        .style("border-width","thin")
                        .style("overflow-y","auto")
                        .child(html!("table", {
                            //.attr("id", "common-searchbox-ivfluid-table")
                            .class(class::TABLE_STRIP)
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
                                    //.attr("id", "ivfluid-searchbox-tbody")
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
                                            .child(html!("td", {
                                                .class("fw-bold")
                                                .style("cursor","pointer")
                                                .child(doms::color_prefix_span(&dec_to_color(result.displaycolor.unwrap_or_default())))
                                                .text(&result.ivfluid_name.clone().unwrap_or_default())
                                                .event(clone!(order_form, result => move |_: events::Click| {
                                                    if let Some(new) = result.ivfluid_name.clone() {
                                                        let items = order_form.ivfluids();
                                                        let mut lock = items.lock_mut();
                                                        let order_item = OrderItemMutable::new("ivfluid", pre_order_master_id);
                                                        order_form.focused().set(Some(order_item.id));
                                                        order_form.changed().set_neq(true);
                                                        order_item.icode.set(Some(result.icode.clone()));
                                                        order_item.med_name.set(Some(new));
                                                        lock.push_cloned(order_item);
                                                        order_form.display_ivfluid_searchbox().set(false);
                                                    }
                                                }))
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
