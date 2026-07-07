use dominator::{Dom, clone, events, html};
use futures_signals::{
    map_ref,
    signal::{Mutable, SignalExt},
    signal_vec::{MutableVec, SignalVecExt},
};
use std::rc::Rc;
use web_sys::{HtmlInputElement, HtmlSelectElement};

use kphis_model::drug_use_duration::{DrugUseDuration, DrugUseDurationParams};
use kphis_ui_app::App;
use kphis_ui_component::modal::{blank_modal, drug_details::DrugDetailModal};
use kphis_ui_core::{class, doms, mixins};
use kphis_util::util::str_some;

#[derive(Clone, Default)]
pub struct DrugUseDurationPage {
    loaded: Mutable<bool>,
    changed: Mutable<bool>,

    drugs: MutableVec<Rc<DrugUseDuration>>,

    icode: Mutable<String>,
    med_name: Mutable<String>,
    // "Y", "N", ""
    due_status: Mutable<String>,
    // "Y", "N", ""
    monitor_status: Mutable<String>,
    // "Y", "N", ""
    info_status: Mutable<String>,

    drug_details_modal: Mutable<Option<Rc<DrugDetailModal>>>,
}

impl DrugUseDurationPage {
    pub fn new() -> Rc<Self> {
        Rc::new(Self { ..Default::default() })
    }

    // send GET method
    fn submit(page: Rc<Self>, app: Rc<App>) {
        let params = DrugUseDurationParams {
            icode: str_some(page.icode.get_cloned()),
            med_name: str_some(page.med_name.get_cloned()),
            due_status: str_some(page.due_status.get_cloned()),
            monitor_status: str_some(page.monitor_status.get_cloned()),
            info_status: str_some(page.info_status.get_cloned()),
        };
        app.async_load(
            true,
            clone!(app, page => async move {
                // GET `EndPoint::DrugUseDuration`
                match DrugUseDuration::call_api_get(&params, app.state()).await {
                    Ok(response) => {
                        let mut lock = page.drugs.lock_mut();
                        lock.clear();
                        lock.extend(response.into_iter().map(Rc::new));

                    }
                    Err(e) => {
                        app.alert_app_error(&e).await;
                    }
                }
            }),
        );
    }

    pub fn render(page: Rc<Self>, app: Rc<App>) -> Dom {
        app.set_title("KPHIS - Drug Use Duration List");

        html!("section", {
            .future(map_ref!(
                let busy = app.loader_is_loading(),
                let loaded = page.loaded.signal() =>
                !busy && !loaded
            ).for_each(clone!(app, page => move |ready| {
                if ready {
                    Self::submit(page.clone(), app.clone());
                    page.loaded.set(true);
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
            .class("container-fluid")
            .attr("id", "content")
            .children([
                doms::alert_row(clone!(page => move |alert| { alert
                    .children([
                        doms::form_inline(clone!(page => move |form| { form
                            .children([
                                doms::form_inline_group_sm(clone!(page => move |group| { group
                                    .children([
                                        doms::label_group_for("status","สถานะ Info"),
                                        html!("select" => HtmlSelectElement, {
                                            .class("form-control")
                                            .attr("id", "status")
                                            .children([
                                                html!("option", {.attr("value", "").text("ทั้งหมด")}),
                                                html!("option", {.attr("value", "Y").text("ใช้งาน")}),
                                                html!("option", {.attr("value", "N").text("ไม่ใช้งาน")}),
                                            ])
                                            .apply(mixins::string_value_select(page.info_status.clone(), page.changed.clone()))
                                        }),
                                    ])
                                })),
                                doms::form_inline_group_sm(clone!(page => move |group| { group
                                    .children([
                                        doms::label_group_for("status","สถานะ DUE"),
                                        html!("select" => HtmlSelectElement, {
                                            .class("form-control")
                                            .attr("id", "status")
                                            .children([
                                                html!("option", {.attr("value", "").text("ทั้งหมด")}),
                                                html!("option", {.attr("value", "Y").text("ใช้งาน")}),
                                                html!("option", {.attr("value", "N").text("ไม่ใช้งาน")}),
                                            ])
                                            .apply(mixins::string_value_select(page.due_status.clone(), page.changed.clone()))
                                        }),
                                    ])
                                })),
                                doms::form_inline_group_sm(clone!(page => move |group| { group
                                    .children([
                                        doms::label_group_for("status","สถานะ Monitor"),
                                        html!("select" => HtmlSelectElement, {
                                            .class("form-control")
                                            .attr("id", "status")
                                            .children([
                                                html!("option", {.attr("value", "").text("ทั้งหมด")}),
                                                html!("option", {.attr("value", "Y").text("ใช้งาน")}),
                                                html!("option", {.attr("value", "N").text("ไม่ใช้งาน")}),
                                            ])
                                            .apply(mixins::string_value_select(page.monitor_status.clone(), page.changed.clone()))
                                        }),
                                    ])
                                })),
                                doms::form_inline_group_sm(clone!(page => move |group| { group
                                    .children([
                                        doms::label_group_for("icode","icode"),
                                        html!("input" => HtmlInputElement, {
                                            .attr("type", "text")
                                            .class("form-control")
                                            .attr("id", "icode")
                                            .apply(mixins::string_value_end(page.icode.clone(), page.changed.clone()))
                                        }),
                                        doms::label_group_for("med_name","ชื่อยา ปริมาณ บรรจุ"),
                                        html!("input" => HtmlInputElement, {
                                            .attr("type", "text")
                                            .class("form-control")
                                            .attr("id", "med_name")
                                            .apply(mixins::string_value_end(page.med_name.clone(), page.changed.clone()))
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
                                doms::form_inline_end(clone!(page => move |group| { group
                                    .child(html!("button", {
                                        .attr("type", "button")
                                        .class(class::BTN_SM_BLUE)
                                        .attr("data-bs-toggle", "modal")
                                        .attr("data-bs-target", "#drugUseDurationFormModal")
                                        .child(html!("i", {.class(class::FA_PLUS)}))
                                        .text(" เพิ่ม")
                                        .event(clone!(page => move |_: events::Click| {
                                            page.drug_details_modal.set(Some(DrugDetailModal::new(true)));
                                        }))
                                    }))
                                }))
                            ])
                        })),
                    ])
                })),
                doms::table_responsive(class::TABLE_STRIP, clone!(app, page => move |table| { table
                    .children([
                        html!("thead", {
                            .child(html!("tr", {
                                .class("text-center")
                                .children([
                                    html!("th", {.attr("scope","col").text("#")}),
                                    html!("th", {.attr("scope","col").text("icode")}),
                                    html!("th", {.attr("scope","col").text("ชื่อยา ปริมาณ บรรจุ")}),
                                    html!("th", {.attr("scope","col").text("Info").style("border-left-width","5px")}),
                                    html!("th", {.attr("scope","col").text("สถานะ")}),
                                    html!("th", {.attr("scope","col").text("DUE").style("border-left-width","5px")}),
                                    html!("th", {.attr("scope","col").text("ต่ำ")}),
                                    html!("th", {.attr("scope","col").text("กลาง")}),
                                    html!("th", {.attr("scope","col").text("สูง")}),
                                    html!("th", {.attr("scope","col").text("สถานะ")}),
                                    html!("th", {.attr("scope","col").text("Monitor").style("border-left-width","5px")}),
                                    html!("th", {.attr("scope","col").text("ครั้ง")}),
                                    html!("th", {.attr("scope","col").text("นาที")}),
                                    html!("th", {.attr("scope","col").text("สถานะ")}),
                                ])
                            }))
                        }),
                        html!("tbody", {
                            .children_signal_vec(page.drugs.signal_vec_cloned().enumerate().map(clone!(app, page => move |(i,row)| {
                                Self::render_result(i.get().unwrap_or_default(), row, page.clone())
                            })))
                        }),
                    ])
                })),
                html!("div", {
                    .class("modal")
                    .attr("id", "drugUseDurationFormModal")
                    .attr("role", "dialog")
                    .attr("tabindex", "-1")
                    .child_signal(page.drug_details_modal.signal_cloned().map(clone!(app => move |opt| {
                        opt.map(|modal| DrugDetailModal::render(modal, page.drug_details_modal.clone(), Some(page.changed.clone()), app.clone())).or(Some(blank_modal()))
                    })))
                }),
            ])
        })
    }

    fn render_result(i: usize, row: Rc<DrugUseDuration>, page: Rc<Self>) -> Dom {
        html!("tr", {
            .style("cursor","pointer")
            .attr("data-bs-toggle", "modal")
            .attr("data-bs-target", "#drugUseDurationFormModal")
            .children([
                html!("td", {.class("text-center").text(&(i + 1).to_string())}),
                html!("td", {.class("text-center").text(&row.icode.clone())}),
                html!("td", {.class("text-nowrap").text(&row.med_name.clone().unwrap_or_default())}),
                html!("td", {
                    .class(class::SMALL_TRUNC)
                    .style("max-width","200px")
                    // .style("white-space","pre-wrap")
                    .style("border-left-width","5px")
                    .children(doms::square_bracket_to_span(&row.info.clone().unwrap_or_default()))
                }),
                html!("td", {
                    .class(class::TXT_C_P1)
                    .apply(|dom| {
                        match row.info_status.clone().unwrap_or_default().as_str() {
                            "Y" => dom.child(html!("i", {.class(class::FA_CHECK_CIRCLE_GREEN).style("font-size","30px")})),
                            "N" => dom.child(html!("i", {.class(class::FA_X_CIRCLE_RED).style("font-size","30px")})),
                            _ => dom,
                        }
                    })
                }),
                html!("td", {
                    .class(class::SMALL_TRUNC)
                    .style("max-width","200px")
                    // .style("white-space","pre-wrap")
                    .style("border-left-width","5px")
                    .children(doms::square_bracket_to_span(&row.usage.clone().unwrap_or_default()))
                }),
                html!("td", {
                    .class(class::BOLD_C)
                    .style("color",&row.exceed_duration1_color.clone().unwrap_or(String::from("inherit")))
                    .text(&row.duration1.map(|d| d.to_string()).unwrap_or_default())
                }),
                html!("td", {
                    .class(class::BOLD_C)
                    .style("color",&row.exceed_duration2_color.clone().unwrap_or(String::from("inherit")))
                    .text(&row.duration2.map(|d| d.to_string()).unwrap_or_default())
                }),
                html!("td", {
                    .class(class::BOLD_C)
                    .style("color",&row.exceed_duration3_color.clone().unwrap_or(String::from("inherit")))
                    .text(&row.duration3.map(|d| d.to_string()).unwrap_or_default())
                }),
                html!("td", {
                    .class(class::TXT_C_P1)
                    .apply(|dom| {
                        match row.status.clone().unwrap_or_default().as_str() {
                            "Y" => dom.child(html!("i", {.class(class::FA_CHECK_CIRCLE_GREEN).style("font-size","30px")})),
                            "N" => dom.child(html!("i", {.class(class::FA_X_CIRCLE_RED).style("font-size","30px")})),
                            _ => dom,
                        }
                    })
                }),
                html!("td", {
                    .class(class::SMALL_TRUNC)
                    .style("max-width","200px")
                    // .style("white-space","pre-wrap")
                    .style("border-left-width","5px")
                    .children(doms::square_bracket_to_span(&row.monitor.clone().unwrap_or_default()))
                }),
                html!("td", {.class("text-center").text(&row.monitor_count.map(|u| u.to_string()).unwrap_or_default())}),
                html!("td", {.class("text-center").text(&row.monitor_duration.map(|u| u.to_string()).unwrap_or_default())}),
                html!("td", {
                    .class(class::TXT_C_P1)
                    .apply(|dom| {
                        match row.monitor_status.clone().unwrap_or_default().as_str() {
                            "Y" => dom.child(html!("i", {.class(class::FA_CHECK_CIRCLE_GREEN).style("font-size","30px")})),
                            "N" => dom.child(html!("i", {.class(class::FA_X_CIRCLE_RED).style("font-size","30px")})),
                            _ => dom,
                        }
                    })
                }),
            ])
            .event(move |_:events::Click| {
                page.drug_details_modal.set(Some(DrugDetailModal::new_with_med(&row)));
            })
        })
    }
}
