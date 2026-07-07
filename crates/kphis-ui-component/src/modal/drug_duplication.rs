use dominator::{Dom, clone, events, html};
use futures_signals::signal::Mutable;

use kphis_model::search::searchbox::DrugDuplicateCheck;
use kphis_ui_core::class;
use kphis_util::datetime::{date_th_opt, time_hm_opt};

// ipd-dr-order-item-drug-duplication-modal.php
#[derive(Clone, Default)]
pub struct DrugDuplication {
    med_name: String,
    duplications: Vec<DrugDuplicateCheck>,
}

impl DrugDuplication {
    pub fn new(med_name: &str, duplication: &[DrugDuplicateCheck]) -> Self {
        Self {
            med_name: String::from(med_name),
            duplications: duplication.to_vec(),
        }
    }

    pub fn render(&self, display: Mutable<Option<Self>>, allowed: Mutable<bool>) -> Dom {
        html!("div", {
            .class(class::MODAL_DIALOG_LG)
            .attr("role", "document")
            .child(html!("div", {
                .class("modal-content")
                .children([
                    html!("div", {
                        .class("modal-header")
                        .children([
                            html!("h5", {
                                .class("modal-title")
                                .text("ต้องการสั่งยาชื่อสามัญ (Generic Name) เดียวกันหรือไม่")
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
                        //.attr("id", "drugDuplicationModalBody")
                        .children([
                            html!("div", {
                                .class(class::BOLD_T3L)
                                .text("รายการที่สั่ง : ")
                                .child(html!("span", {
                                    //.attr("id", "drugDuplicationNameSpan")
                                    .class("ms-1")
                                    .text(&self.med_name)
                                }))
                            }),
                            html!("table", {
                                .class(class::TABLE_STRIP)
                                .children([
                                    html!("thead", {
                                        .child(html!("tr", {
                                            .children([
                                                html!("th", {
                                                    .attr("scope", "col")
                                                    .class("text-nowrap")
                                                    .text("รายการยาที่บันทึกไว้แล้วก่อนหน้านี้")
                                                }),
                                                html!("th", {
                                                    .attr("scope", "col")
                                                    .class("text-nowrap")
                                                    .text("วันที่สั่ง")
                                                }),
                                            ])
                                        }))
                                    }),
                                    html!("tbody", {
                                        //.attr("id", "drugDuplicationModalTableBody")
                                        .children(self.duplications.iter().map(|dup| {
                                            html!("tr", {
                                                .children([
                                                    html!("td", {
                                                        .children([
                                                            html!("div", {.text(&dup.med_name.clone().unwrap_or_default())}),
                                                            html!("div", {
                                                                .style("white-space","pre-wrap")
                                                                .text(&dup.order_item_detail.clone().unwrap_or_default())
                                                            }),
                                                        ])
                                                    }),
                                                    html!("td", {
                                                        .child(html!("div", {
                                                            .text(&[date_th_opt(&dup.order_date), time_hm_opt(&dup.order_time)].join(" "))
                                                        }))
                                                    }),
                                                ])
                                            })
                                        }))
                                    }),
                                ])
                            }),
                        ])
                    }),
                    html!("div", {
                        .class("modal-footer")
                        .children([
                            html!("button", {
                                .attr("type", "button")
                                .class(class::BTN_BLUE)
                                //.attr("id", "drugDuplicationModalOkButton")
                                .attr("data-bs-dismiss", "modal")
                                .text("ตกลง")
                                .event(clone!(display => move |_: events::Click| {
                                    display.set(None);
                                    allowed.set(true);
                                }))
                            }),
                            html!("button", {
                                .attr("type", "button")
                                .class(class::BTN_GRAY)
                                //.attr("id", "drugDuplicationModalCancelButton")
                                .attr("data-bs-dismiss", "modal")
                                .text("ยกเลิก")
                                .event(move |_: events::Click| {
                                    display.set(None);
                                })
                            }),
                        ])
                    }),
                ])
            }))
        })
    }
}
