use dominator::{Dom, clone, events, html, with_node};
use futures_signals::signal::{Mutable, Signal};
use wasm_bindgen::JsCast;

use std::rc::Rc;
use web_sys::HtmlDivElement;

use super::{MODAL, MODAL_CONTENT};
use crate::{class, popups::MODAL_CONTENT_ALERT};

#[derive(Clone)]
pub struct WithClosePopup {
    is_error: bool,
    title: Mutable<String>,
    message: Mutable<String>,
    finished: Mutable<bool>,
}

impl WithClosePopup {
    pub fn new(title: &str, message: &str, is_error: bool) -> Rc<Self> {
        Rc::new(Self {
            is_error,
            title: Mutable::new(title.to_owned()),
            message: Mutable::new(message.to_owned()),
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
                .apply(|dom| {
                    if page.is_error {
                        dom.class(&*MODAL_CONTENT_ALERT)
                    } else {
                        dom.class(&*MODAL_CONTENT)
                    }
                })
                .child(html!("div",{
                    .class(class::ROW_M)
                    .children([
                        html!("div",{
                            .class(class::COL_MD12_T)
                            .child(html!("h3", {
                                .class("fw-bold")
                                .text(&page.title.lock_ref())
                            }))
                        }),
                        html!("hr"),
                        html!("div", {
                            .class(class::COL_MD12_T)
                            .style("overflow-y","auto")
                            .style("max-height","60vh")
                            .child(html!("div",{
                                .child(html!("div", {
                                    .class(&*class::MONO_PRE_WRAP)
                                    .text(&page.message.lock_ref())
                                }))
                            }))
                        }),
                        html!("hr",{.class(class::M_Y31)}),
                        html!("div",{
                            .class(class::TXT_R_PY)
                            .children([
                                html!("button",{
                                    .attr("type", "button")
                                    .class(class::BTN_L_BLUE)
                                    .focused(true)
                                    .text("ปิด")
                                    .event(clone!(page => move |_: events::Click| {
                                        page.finished.set(true);
                                    }))
                                }),
                            ])
                        }),
                    ])
                }))
            }))
        })
    }
}
