// ipd-lab-wbc.php
use dominator::{Dom, clone, events, html};
use futures_signals::{
    map_ref,
    signal::{Mutable, SignalExt},
    signal_vec::{MutableVec, SignalVecExt},
};
use std::rc::Rc;

use kphis_model::lab::LabWbcBand;
use kphis_ui_app::App;
use kphis_ui_core::{class, doms};
use kphis_util::datetime::date_th_opt;

/// GET `EndPoint::LabWbcKeyValue`
#[derive(Default)]
pub struct LabWbc {
    loaded: Mutable<bool>,
    // HosXp store `an` and `vn` in the same column `vn`
    key: Mutable<String>,   // hn, vn(an)
    value: Mutable<String>, // value of hn, vn(an)
    labs: MutableVec<Rc<LabWbcBand>>,

    parent_wbc: Mutable<String>,
    parent_band: Mutable<String>,
    parent_changed: Mutable<bool>,
}

impl LabWbc {
    pub fn new(key: String, value: String, parent_wbc: Mutable<String>, parent_band: Mutable<String>, parent_changed: Mutable<bool>) -> Rc<Self> {
        Rc::new(Self {
            key: Mutable::new(key),
            value: Mutable::new(value),
            parent_wbc,
            parent_band,
            parent_changed,
            ..Default::default()
        })
    }

    fn load(modal: Rc<Self>, app: Rc<App>) {
        app.async_load(
            true,
            clone!(app => async move {
                // GET `EndPoint::LabWbcKeyValue`
                match LabWbcBand::call_api_get(
                    &modal.key.lock_ref(),
                    &modal.value.lock_ref(),
                    app.state(),
                ).await {
                    Ok(responses) => {
                        modal.labs.lock_mut().extend(responses.into_iter().map(Rc::new));
                    }
                    Err(e) => {
                        app.alert_app_error(&e).await;
                    }
                }
            }),
        )
    }

    pub fn render(modal: Rc<Self>, app: Rc<App>) -> Dom {
        html!("div", {
            .future(map_ref!(
                let busy = app.loader_is_loading(),
                let loaded = modal.loaded.signal() =>
                !busy && !loaded
            ).for_each(clone!(app, modal => move |ready| {
                if ready {
                    Self::load(modal.clone(), app.clone());
                    modal.loaded.set_neq(true);
                }
                async {}
            })))
            .class(class::MODAL_DIALOG_LG)
            .attr("role", "document")
            .child(html!("div", {
                .class("modal-content")
                .children([
                    html!("div", {
                        .class("modal-header")
                        .children([
                            html!("h5", {.class("modal-title").text("เลือกรายการ")}),
                            doms::close_modal_x_btn(),
                        ])
                    }),
                    html!("div", {
                        .class("modal-body")
                        //.attr("id", "selectLabWBCModalBody")
                        .style("height","400px")
                        .style("width", "100%")
                        .child(doms::table_responsive(class::TABLE_STRIP, clone!(modal => move |table| { table
                            .children([
                                html!("thead", {
                                    .child(html!("tr", {
                                        .children([
                                            // html!("th", {.attr("scope", "col").visible(false).text("#")}),
                                            html!("th", {.attr("scope", "col").text("Lab Order No.")}),
                                            html!("th", {.attr("scope", "col").text("เวลาที่สั่ง")}),
                                            html!("th", {.attr("scope", "col").text("เวลาที่รับ")}),
                                            html!("th", {.attr("scope", "col").text("เวลาที่รายงาน")}),
                                            html!("th", {.attr("scope", "col").text("wbc(cell/mm").child(html!("sup", {.style("padding-right","0").text("3")})).text(")")}),
                                            html!("th", {.attr("scope", "col").text("band(%)")}),
                                        ])
                                    }))
                                }),
                                html!("tbody", {
                                    .children_signal_vec(modal.labs.signal_vec_cloned().map(clone!(modal => move |lab| {
                                        html!("tr", {
                                            .style("cursor","pointer")
                                            .attr("data-bs-dismiss", "modal")
                                            .children([
                                                html!("td", {.class("text-center").text(&lab.lab_order_number.to_string())}),
                                                html!("td", {.class("text-center").text(&date_th_opt(&lab.order_date))}),
                                                html!("td", {.class("text-center").text(&date_th_opt(&lab.report_date))}),
                                                html!("td", {.class("text-center").text(&date_th_opt(&lab.receive_date))}),
                                                html!("td", {.class("text-center").text(&lab.wbc.clone().unwrap_or_default())}),
                                                html!("td", {.class("text-center").text(&lab.band.clone().unwrap_or_default())}),
                                            ])
                                            .event(clone!(modal => move |_:events::Click| {
                                                modal.parent_wbc.set_neq(lab.wbc.clone().map(|wbc| (wbc.replace(',',"").parse::<f32>().unwrap_or_default() / 1000.0).to_string()).unwrap_or_default());
                                                modal.parent_band.set_neq(lab.band.clone().unwrap_or_default());
                                                modal.parent_changed.set_neq(true);
                                            }))
                                        })
                                    })))
                                }),
                            ])
                        })))
                    }),
                    html!("div", {
                        .class("modal-footer")
                        .child(html!("button", {
                            .attr("type", "button")
                            .class(class::BTN_GRAY)
                            .attr("data-bs-dismiss", "modal")
                            .text("ปิด")
                        }))
                    }),
                ])
            }))
        })
    }
}
