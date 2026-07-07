use dominator::{Dom, clone, events, html};
use futures_signals::signal::Mutable;

use kphis_model::search::searchbox::DrugInteractionCheck;
use kphis_ui_core::class;

// ipd-dr-order-item-drug-interaction-modal.php
#[derive(Clone, Default)]
pub struct DrugInteraction {
    // med_name: String,
    interactions: Vec<DrugInteractionCheck>,
}

impl DrugInteraction {
    pub fn new(
        // med_name: &str,
        interactions: &[DrugInteractionCheck],
    ) -> Self {
        Self {
            // med_name: String::from(med_name),
            interactions: interactions.to_vec(),
        }
    }

    pub fn render(&self, display: Mutable<Option<Self>>, allowed: Mutable<bool>) -> Dom {
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
                                .attr("data-bs-dismiss", "modal")
                                .attr("aria-label", "Close")
                                .event(clone!(display => move |_: events::Click| {
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
                                //.attr("id", "drugInteractionModalOkButton")
                                .attr("data-bs-dismiss", "modal")
                                .text("ตกลง")
                                .event(clone!(display => move |_: events::Click| {
                                    display.set(None);
                                    allowed.set_neq(true);
                                }))
                            }))
                        })
                        .child(html!("button", {
                            .attr("type", "button")
                            .class(class::BTN_GRAY)
                            //.attr("id", "drugInteractionModalDismissButton")
                            .attr("data-bs-dismiss", "modal")
                            .text("ปิด")
                            .event(move |_: events::Click| {
                                display.set(None);
                            })
                        }))
                    }),
                ])
            }))
        })
    }
}
