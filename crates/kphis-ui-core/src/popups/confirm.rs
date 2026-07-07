use dominator::{Dom, clone, events, html, with_node};
use futures_signals::signal::{Mutable, Signal};
use wasm_bindgen::JsCast;

use std::rc::Rc;
use web_sys::HtmlDivElement;

use super::{MODAL, MODAL_CONTENT, PopupOkCancel};
use crate::class;

#[derive(Clone)]
pub struct ConfirmPopup {
    caption: Mutable<String>,
    message: Mutable<String>,
    pub result: Mutable<PopupOkCancel>,
    finished: Mutable<bool>,
}

impl ConfirmPopup {
    pub fn new(caption: &str, message: &str) -> Rc<Self> {
        Rc::new(Self {
            caption: Mutable::new(caption.to_owned()),
            message: Mutable::new(message.to_owned()),
            result: Mutable::new(PopupOkCancel::Cancel),
            finished: Mutable::new(false),
        })
    }

    pub fn finished(&self) -> impl Signal<Item = bool> + use<> {
        self.finished.signal_cloned()
    }

    pub fn render(page: Rc<Self>) -> Dom {
        html!("div" => HtmlDivElement, {
            .class(&*MODAL)
            .with_node!(element => {
                .event(clone!(page => move |e: events::Click| {
                    if let Some(target) = e.target() {
                        if let Ok(target_div) = target.dyn_into::<HtmlDivElement>() {
                            if element == target_div {
                                page.finished.set(true);
                            }
                        }
                    }
                }))
            })
            .child(html!("div", {
                .class(&*MODAL_CONTENT)
                .child(html!("div",{
                    .child(html!("div",{
                        .class(class::ROW_M)
                        .children([
                            html!("div",{
                                .class(class::COL_MD12_T)
                                .children([
                                    html!("h4", {.text(&page.caption.lock_ref())}),
                                    html!("p", {.text(&page.message.lock_ref())}),
                                ])
                            }),
                            html!("div",{
                                .class(class::TXT_R_PY)
                                .children([
                                    html!("button",{
                                        .attr("type", "button")
                                        .class(class::BTN_L_BLUE)
                                        .focused(true)
                                        .text("OK")
                                        .event(clone!(page => move |_: events::Click| {
                                            page.result.set(PopupOkCancel::Ok);
                                            page.finished.set(true);
                                        }))
                                    }),
                                    html!("button", {
                                        .attr("type", "button")
                                        .class(class::BTN_GRAY)
                                        .text("Cancel")
                                        .event(move |_: events::Click| {
                                            page.finished.set(true);
                                        })
                                    }),
                                ])
                            }),
                        ])
                    }))
                }))
            }))
        })
    }
}
