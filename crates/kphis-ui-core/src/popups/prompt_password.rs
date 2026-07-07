use dominator::{Dom, EventOptions, clone, events, html, with_node};
use futures_signals::signal::{Mutable, Signal};
use wasm_bindgen::JsCast;

use std::rc::Rc;
use web_sys::{HtmlDivElement, HtmlElement, HtmlInputElement};

use kphis_model::app::AppState;

use super::{MODAL, MODAL_CONTENT, PopupAuth};
use crate::class;

#[derive(Clone)]
pub struct PromptPasswordPopup {
    password: Mutable<String>,
    token_2fa: Mutable<String>,
    pub result: Mutable<PopupAuth>,
    finished: Mutable<bool>,
}

impl PromptPasswordPopup {
    pub fn new() -> Rc<Self> {
        Rc::new(Self {
            password: Mutable::new(String::new()),
            token_2fa: Mutable::new(String::new()),
            result: Mutable::new(PopupAuth::Cancel),
            finished: Mutable::new(false),
        })
    }

    pub fn focus(&self, app: Rc<AppState>) {
        if let Some(elm) = app.get_id("promptPassword").and_then(|elm| elm.dyn_into::<HtmlElement>().ok()) {
            if let Err(e) = elm.focus() {
                app.show_jsvalue_message(&e);
            }
        }
    }

    pub fn finished(&self) -> impl Signal<Item = bool> + use<> {
        self.finished.signal_cloned()
    }

    fn save(&self) {
        let password = self.password.get_cloned();
        let token_2fa = self.token_2fa.get_cloned();
        let result = if !password.is_empty() { PopupAuth::Ok(password, token_2fa) } else { PopupAuth::Cancel };
        self.result.set(result);
        self.finished.set(true);
    }

    pub fn render(page: Rc<Self>, app: Rc<AppState>) -> Dom {
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
                    .child(html!("form",{
                        .class(class::ROW_M)
                        .children([
                            html!("div",{
                                .class(class::COL_MD12_T)
                                .children([
                                    html!("h4", {.text("Session expired")}),
                                    html!("p", {
                                        .text("Hi\u{00a0}")
                                        .text(&app.user_name().unwrap_or_default())
                                    }),
                                    html!("p", {
                                        .text("A password is required to continue without losing your work.")
                                        .child(html!("br"))
                                        .text("Cancel will redirect to log-in page.")
                                    }),
                                ])
                            }),
                            html!("div",{
                                .class(class::COL_MD12_T)
                                .child(html!("input" => HtmlInputElement,{
                                    .attr("type", "password")
                                    .attr("id", "promptPassword")
                                    .class(class::FORM_CTRL_LG)
                                    .attr("placeholder","Password")
                                    .attr("autocomplete","current-password")
                                    .prop_signal("value", page.password.signal_cloned())
                                    .with_node!(element => {
                                        .event(clone!(page, element => move |_: events::Input| {
                                            page.password.set(element.value());
                                        }))
                                        .event_with_options(&EventOptions::preventable(), clone!(page => move |event: events::KeyDown| {
                                            if event.key() == "Enter" {
                                                event.prevent_default();
                                                page.password.set(element.value());
                                                page.save();
                                            }
                                        }))
                                    })
                                }))
                            }),
                            html!("div",{
                                .class(class::COL_MD12_T)
                                .child(html!("input" => HtmlInputElement,{
                                    .attr("type", "text")
                                    .attr("id", "prompt2fa")
                                    .class(class::FORM_CTRL_LG)
                                    .attr("placeholder","Authenticator Token")
                                    .prop_signal("value", page.token_2fa.signal_cloned())
                                    .with_node!(element => {
                                        .event(clone!(page, element => move |_: events::Input| {
                                            page.token_2fa.set(element.value());
                                        }))
                                        .event_with_options(&EventOptions::preventable(), clone!(page => move |event: events::KeyDown| {
                                            if event.key() == "Enter" {
                                                event.prevent_default();
                                                page.token_2fa.set(element.value());
                                                page.save();
                                            }
                                        }))
                                    })
                                }))
                            }),
                            html!("div",{
                                .class(class::TXT_R_PY)
                                .children([
                                    html!("button",{
                                        .attr("type", "button")
                                        .class(class::BTN_L_BLUE)
                                        .text("OK")
                                        .event(clone!(page => move |_: events::Click| {
                                            page.save();
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
