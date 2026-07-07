use dominator::{Dom, clone, events, html};
use futures_signals::signal::Mutable;

use kphis_ui_core::class;

// ipd-dr-order-item-drug-notify-modal.php
#[derive(Clone, Default)]
pub struct DrugNotify {
    med_name: String,
    show_notify_text: String,
}

impl DrugNotify {
    pub fn new(med_name: &str, show_notify_text: &str) -> Self {
        Self {
            med_name: String::from(med_name),
            show_notify_text: String::from(show_notify_text),
        }
    }

    pub fn render(&self, display: Mutable<Option<Self>>) -> Dom {
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
                                .text("Drug Notify")
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
                        //.attr("id", "drugNotifyModalBody")
                        .child(html!("span", {
                            .children([
                                html!("div", {
                                    .class("fw-bold")
                                    .text(&self.med_name)
                                }),
                                html!("div", {
                                    .style("white-space","pre-wrap")
                                    .text(&self.show_notify_text)
                                }),
                            ])
                        }))
                    }),
                    html!("div", {
                        .class("modal-footer")
                        .child(html!("button", {
                            .attr("type", "button")
                            .class(class::BTN_GRAY)
                            //.attr("id", "drugNotifyModalDismissButton")
                            .attr("data-bs-dismiss", "modal")
                            .text("ปิด")
                            .event(clone!(display => move |_: events::Click| {
                                display.set(None);
                            }))
                        }))
                    }),
                ])
            }))
            .event(move |_: events::Click| {
                display.set(None);
            })
        })
    }
}
