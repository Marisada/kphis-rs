// from kphis-config-ipd-ward-passcode.php

use dominator::{Dom, clone, events, html, with_node};
use futures_signals::{
    map_ref,
    signal::{Mutable, Signal, SignalExt, not, or},
    signal_vec::{MutableVec, SignalVecExt},
};
use std::rc::Rc;
use web_sys::HtmlButtonElement;

use kphis_model::{
    ipd::passcode::{ConfigIpdWardPasscode, PasscodeGenRequest, PasscodeGenRequestMode},
    user::his::hash,
};
use kphis_ui_app::App;
use kphis_ui_core::{class, doms, mixins};
use kphis_util::error::CONTACT_ADMIN;

/// - GET `EndPoint::IpdPasscode`
/// - POST `EndPoint::IpdPasscode`
#[derive(Clone, Default)]
pub struct IpdPasscodeForm {
    // changed: Mutable<bool>,
    loaded: Mutable<bool>,

    ward: Mutable<String>,
    password: Mutable<String>,
    using_passcode: MutableVec<Rc<ConfigIpdWardPasscode>>,
    not_using_passcode: MutableVec<Rc<ConfigIpdWardPasscode>>,
}

impl IpdPasscodeForm {
    pub fn new() -> Rc<Self> {
        Rc::new(Self::default())
    }

    fn not_using_is_empty(&self) -> impl Signal<Item = bool> + use<> {
        self.not_using_passcode.signal_vec_cloned().is_empty()
    }

    // getCurrentWardPasscodeData.php
    async fn get_current_ward_passcode_data(page: Rc<Self>, app: Rc<App>) {
        {
            page.using_passcode.lock_mut().clear();
            page.not_using_passcode.lock_mut().clear();
        }
        // GET `EndPoint::IpdPasscode`
        match ConfigIpdWardPasscode::call_api_get(app.state()).await {
            Ok(all_ward) => {
                // TODO diff new vs old vec
                let new_using = extract_using_passcode(true, &all_ward);
                let new_not_using = extract_using_passcode(false, &all_ward);
                page.using_passcode.lock_mut().extend(new_using);
                page.not_using_passcode.lock_mut().extend(new_not_using);
            }
            Err(e) => {
                app.alert_app_error(&e).await;
            }
        }
    }

    // genIpdWardPasscode.php
    fn gen_ipd_ward_passcode(page: Rc<Self>, app: Rc<App>) {
        if app.can_change_ward_passcode() {
            app.async_load(
                true,
                clone!(app, page => async move {
                    let password = page.password.lock_ref();
                    match hash(&password) {
                        Ok(pwd) => {
                            let request = PasscodeGenRequest {
                                ward: page.ward.get_cloned(),
                                password: pwd,
                                mode: PasscodeGenRequestMode::Gen,
                            };
                            // POST `EndPoint::IpdPasscode`
                            match request.call_api_post(app.state()).await {
                                Ok(response) => {
                                    match response.passcode {
                                        Some(passcode) => {
                                            Self::get_current_ward_passcode_data(page.clone(), app.clone()).await;
                                            // change alert to inline dom text
                                            app.alert("Passcode ใหม่", &passcode);
                                        }
                                        None => {
                                            app.alert_error_with_closed("สร้าง Passcode ไม่สำเร็จ", "กรุณาติดต่อผู้ดูแลระบบ").await;
                                        }
                                    }
                                }
                                Err(e) => {
                                    app.alert_app_error(&e).await;
                                }
                            }
                            page.password.set_neq(String::new());
                        }
                        Err(e) => {
                            app.alert_error_with_clipboard(CONTACT_ADMIN, &["Error: ", &e.to_string()].concat()).await;
                        }
                    }
                }),
            );
        } else {
            app.alert_error("ท่านไม่มีสิทธิ์สร้าง Passcode ใหม่", "กรุณาติดต่อผู้ดูแลระบบ");
        }
    }
    // removeIpdWardPasscode.php
    fn remove_ipd_ward_passcode(ward: String, page: Rc<Self>, app: Rc<App>) {
        app.async_load(
            true,
            clone!(app, page => async move {
                if app.confirm("ยืนยันยกเลิกการใช้ Passcode").await {
                    if app.can_change_ward_passcode() {
                        let password = page.password.lock_ref();
                        match hash(&password) {
                            Ok(pwd) => {
                                let request = PasscodeGenRequest {
                                    ward,
                                    password: pwd,
                                    mode: PasscodeGenRequestMode::Remove,
                                };
                                // POST `EndPoint::IpdPasscode`
                                match request.call_api_post(app.state()).await {
                                    Ok(response) => match response.passcode {
                                        Some(_remove) => {
                                            Self::get_current_ward_passcode_data(page.clone(), app.clone()).await;
                                            // change alert to inline dom text
                                            app.alert("ยกเลิกการใช้ Passcode เรียบร้อยแล้ว", "");
                                        }
                                        None => {
                                            app.alert_error_with_closed("ยกเลิก Passcode ไม่สำเร็จ", "กรุณาติดต่อผู้ดูแลระบบ").await;
                                        }
                                    }
                                    Err(e) => {
                                        app.alert_app_error(&e).await;
                                    }
                                }
                                page.password.set_neq(String::new());
                            }
                            Err(e) => {
                                app.alert_error_with_clipboard(CONTACT_ADMIN, &["Error: ", &e.to_string()].concat()).await;
                            }
                        }
                    } else {
                        app.alert_error_with_closed("ท่านไม่มีสิทธิ์ยกเลิกการใช้ Passcode", "กรุณาติดต่อผู้ดูแลระบบ").await;
                    }
                }
            }),
        );
    }

    fn render_using_passcode(item: Rc<ConfigIpdWardPasscode>, i: usize, app: Rc<App>, page: Rc<Self>) -> Dom {
        html!("li", {
            .class("list-group-item")
            .children([
                html!("span", {
                    .class("align-middle")
                    .text(&[&(i + 1).to_string(), ". ", &item.ward_name].concat())
                }),
                html!("button" => HtmlButtonElement, {
                    .attr("type", "button")
                    .class(class::BTN_SM_FR_GRAY)
                    .text("ยกเลิกการใช้")
                    .apply(mixins::click_with_loader_checked_or_true_disable_signal(clone!(app, page => move || {
                        Self::remove_ipd_ward_passcode(item.ward.clone(), page.clone(), app.clone())
                    }), page.password.signal_cloned().map(|pwd| pwd.is_empty()), app.state()))
                })
            ])
        })
    }

    pub fn render(page: Rc<Self>, app: Rc<App>) -> Dom {
        html!("div", {
            .future(map_ref!(
                let busy = app.loader_is_loading(),
                let loaded = page.loaded.signal() =>
                !busy && !loaded
            ).for_each(clone!(app, page => move |ready| {
                if ready {
                    if app.can_change_ward_passcode() {
                        app.async_load(true, clone!(app, page => async move {
                            Self::get_current_ward_passcode_data(page.clone(), app.clone()).await;
                        }));
                    }
                    page.loaded.set_neq(true);
                }
                async {}
            })))
            .class(class::MODAL_DIALOG_LG)
            .attr("role","document")
            .child(html!("div", {
                .class("modal-content")
                .children([
                    html!("div", {
                        .class("modal-header")
                        .children([
                            html!("h5", {
                                .class("modal-title")
                                .text("Ward Passcode")
                            }),
                            doms::close_modal_x_btn(),
                        ])
                    }),
                    html!("div", {
                        .class("modal-body")
                        //.attr("id", "ward_passcode_modal_body")
                        .child(html!("div", {
                            .class("content-fluid")
                            .children([
                                html!("div", {
                                    .class(class::ROW)
                                    .child(html!("ul", {
                                        .class("list-group")
                                        //.attr("id", "ward_passcode_list_group")
                                        .children_signal_vec(page.using_passcode.signal_vec_cloned().enumerate().map(clone!(app, page => move |(i, item)| {
                                            Self::render_using_passcode(item, i.get().unwrap_or_default(), app.clone(), page.clone())
                                        })))
                                    }))
                                }),
                                html!("div", {
                                    .class("row")
                                    //.attr("id", "ward_passcode_add_form")
                                    .child(html!("div",{
                                        .class("col")
                                        .child(html!("form", {
                                            .class(class::FLEX_WRAP_T)
                                            //.attr("id", "ipdWardPasscodeForm")
                                            .children([
                                                html!("div", {
                                                    .class(class::INPUT_GROUP_T)
                                                    .visible_signal(not(page.not_using_is_empty()))
                                                    .children([
                                                        doms::label_group_for("passcode_ward_select","สำหรับ ward"),
                                                        html!("select" => web_sys::HtmlSelectElement, {
                                                            .class("form-select")
                                                            .style("width","250px")
                                                            .attr("id", "passcode_ward_select")
                                                            .child(html!("option", {.attr("value","").text("เลือก")}))
                                                            .children_signal_vec(page.not_using_passcode.signal_vec_cloned().map(|item| {
                                                                render_not_using_passcode(item)
                                                            }))
                                                            .prop_signal("value", page.ward.signal_cloned())
                                                            .with_node!(element => {
                                                                .event(clone!(page => move |_: events::Change| {
                                                                    page.ward.set_neq(element.value());
                                                                }))
                                                            })
                                                        })
                                                    ])
                                                }),
                                                html!("div", {
                                                    .class(class::INPUT_GROUP_T)
                                                    .children([
                                                        html!("label", {
                                                            .attr("for", "passcode_password")
                                                            .class("input-group-text")
                                                            .class_signal("text-danger", page.password.signal_cloned().map(|pwd| pwd.is_empty()))
                                                            .text("Password HOSxP")
                                                        }),
                                                        html!("input" => web_sys::HtmlInputElement, {
                                                            .attr("type", "password")
                                                            .class("form-control")
                                                            .attr("id", "passcode_password")
                                                            .attr("placeholder","Password HOSxP")
                                                            .attr("autocomplete","off")
                                                            // .apply(mixins::string_value(page.password.clone(), page.changed.clone()))
                                                            .prop_signal("value", page.password.signal_cloned())
                                                            .with_node!(element => {
                                                                .event(clone!(page => move |_: events::Input| {
                                                                    page.password.set_neq(element.value());
                                                                }))
                                                            })
                                                        })
                                                    ])
                                                }),
                                                html!("button" => HtmlButtonElement, {
                                                    .attr("type", "button")
                                                    .class(class::BTN_RX_BLUE)
                                                    //.attr("id", "addPasscodeButton")
                                                    .text("สร้าง Passcode ใหม่")
                                                    .visible_signal(not(page.not_using_is_empty()))
                                                    .apply(mixins::click_with_loader_checked_or_true_disable_signal(
                                                        clone!(app, page => move || {
                                                            Self::gen_ipd_ward_passcode(page.clone(), app.clone());
                                                            // onclick="return genIpdWardPasscode(event)"
                                                        }),
                                                        or(page.ward.signal_cloned().map(|w| w.is_empty()), page.password.signal_cloned().map(|p| p.is_empty())),
                                                        app.state(),
                                                    ))
                                                })
                                            ])
                                            // onsubmit="return false;"
                                        }))
                                    }))
                                })
                            ])
                        }))
                    }),
                    // html!("div", {
                    //     .class("modal-footer")
                    //     .child(html!("button", {
                    //         .attr("type", "button")
                    //         .class(class::BTN_GRAY)
                    //         .attr("data-bs-dismiss","modal")
                    //         .child(html!("i", {
                    //             .class(class::FA_X)
                    //         }))
                    //         .text(" Cancel")
                    //     }))
                    // })
                ])
            }))
        })
    }
}

fn extract_using_passcode(is_using: bool, datas: &[ConfigIpdWardPasscode]) -> impl Iterator<Item = Rc<ConfigIpdWardPasscode>> {
    datas.iter().filter(move |data| data.using_passcode == is_using).cloned().map(Rc::new)
}

fn render_not_using_passcode(item: Rc<ConfigIpdWardPasscode>) -> Dom {
    html!("option", {
        .attr("value",&item.ward)
        .text(&item.ward_name)
    })
}
