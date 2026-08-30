use dominator::{Dom, clone, events, html};
use futures_signals::signal::Mutable;
use std::rc::Rc;

use kphis_model::search::searchbox::DrugInteractionCheck;
use kphis_ui_app::App;
use kphis_ui_core::class;

use crate::modal::modal_show_option_mixins;

// ipd-dr-order-item-drug-interaction-modal.php
#[derive(Clone, Default)]
pub struct DrugInteraction {
    // med_name: String,
    interactions: Vec<DrugInteractionCheck>,
}

impl DrugInteraction {
    pub fn new(interactions: &[DrugInteractionCheck]) -> Rc<Self> {
        Rc::new(Self {
            // med_name: String::from(med_name),
            interactions: interactions.to_vec(),
        })
    }

    pub fn render_modal(&self, display: Mutable<Option<Rc<Self>>>, allowed: Mutable<bool>, app: Rc<App>) -> Dom {
        html!("div", {
            .child(self.render_dialog(display.clone(), allowed.clone(), app.clone()))
            .apply(modal_show_option_mixins(display, app))
        })
    }

    fn render_dialog(&self, display: Mutable<Option<Rc<Self>>>, allowed: Mutable<bool>, app: Rc<App>) -> Dom {
        html!("div", {
            .class(class::MODAL_DIALOG_LG)
            // .class(["mw-100","w-75"])
            .attr("role", "document")
            .child(html!("div", {
                .class("modal-content")
                .children([
                    html!("div", {
                        .class("modal-header")
                        .children([
                            html!("h5", {
                                .class("modal-title")
                                .text("Drug Interaction")
                            }),
                            html!("button", {
                                .attr("type", "button")
                                .class("btn-close")
                                .attr("aria-label", "Close")
                                .event(clone!(app, display => move |_: events::Click| {
                                    app.clear_modal_backdrop();
                                    display.set(None);
                                }))
                            }),
                        ])
                    }),
                    html!("div", {
                        .class("modal-body")
                        //.attr("id", "drugInteractionModalBody")
                        .children([
                            html!("table", {
                                .class(class::TABLE_STRIP)
                                .children([
                                    html!("thead", {
                                        .child(html!("tr", {
                                            .children([
                                                html!("th", {
                                                    .attr("scope", "col")
                                                    .class("text-nowrap")
                                                    .text("ชื่อยา 1")
                                                }),
                                                html!("th", {
                                                    .attr("scope", "col")
                                                    .class("text-nowrap")
                                                    .text("ชื่อยา 2")
                                                }),
                                                html!("th", {
                                                    .attr("scope", "col")
                                                    .class(class::NOWRAP_C)
                                                    .text("ระดับความรุนแรง")
                                                }),
                                                html!("th", {
                                                    .attr("scope", "col")
                                                    .class("text-nowrap")
                                                    .text("หมายเหตุ")
                                                }),
                                                html!("th", {
                                                    .attr("scope", "col")
                                                    .class("text-nowrap")
                                                    .text("ห้ามสั่งใช้")
                                                }),
                                            ])
                                        }))
                                    }),
                                    html!("tbody", {
                                        //.attr("id", "drugInteractionModalTableBody")
                                        .children(self.interactions.iter().map(|interacion| {
                                            html!("tr", {
                                                .children([
                                                    html!("td", {.text(&interacion.drugname1.clone().unwrap_or_default())}),
                                                    html!("td", {.text(&interacion.drugname2.clone().unwrap_or_default())}),
                                                    html!("td", {
                                                        .class("text-center")
                                                        .text(&interacion.severity.unwrap_or_default().to_string())
                                                    }),
                                                    html!("td", {
                                                        .style("white-space","pre-wrap")
                                                        .text(&interacion.note.clone().unwrap_or_default())
                                                    }),
                                                    html!("td", {
                                                        .class(class::BOLD_RED_L)
                                                        .text(if interacion.not_allow == Some(String::from("Y")) {"ห้ามสั่งใช้"} else {""})
                                                    }),
                                                ])
                                            })
                                        }))
                                    })
                                ])
                            }),
                            html!("div", {
                                .class("text-center")
                                .child(html!("span", {
                                    .class(class::BADGE_WRAP_R_GRAY)
                                    .style("cursor","default")
                                    .text("หากมีรายการที่ห้ามสั่งใช้อย่างน้อย 1 รายการจะไม่สามารถเพิ่มรายการได้")
                                }))
                            }),
                        ])
                    }),
                    html!("div", {
                        .class("modal-footer")
                        .apply_if(!self.interactions.iter().any(|interacion| interacion.not_allow == Some(String::from("Y"))), |dom| {
                            dom.child(html!("button", {
                                .attr("type", "button")
                                .class(class::BTN_BLUE)
                                .text("ตกลง")
                                .event(clone!(app, display => move |_: events::Click| {
                                    app.clear_modal_backdrop();
                                    display.set(None);
                                    allowed.set_neq(true);
                                }))
                            }))
                        })
                        .child(html!("button", {
                            .attr("type", "button")
                            .class(class::BTN_GRAY)
                            .text("ปิด")
                            .event(move |_: events::Click| {
                                app.clear_modal_backdrop();
                                display.set(None);
                            })
                        }))
                    }),
                ])
            }))
        })
    }
}
