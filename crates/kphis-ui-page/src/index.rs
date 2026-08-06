use dominator::{Dom, body, clone, events, html};
use futures_signals::{
    map_ref,
    signal::{Mutable, SignalExt},
};
use std::rc::Rc;

use kphis_model::{route::Route, timer::Timeout};
use kphis_ui_app::App;
use kphis_ui_component::login::LoginCpn;
use kphis_ui_core::class;

/// - POST `EndPoint::User`
/// - PATCH `EndPoint::User`
#[derive(Clone, Default)]
pub struct IndexPage {
    is_success: Mutable<bool>,
    reconnecting: Mutable<Option<i32>>,
}

impl IndexPage {
    pub fn new() -> Rc<Self> {
        Rc::new(Self::default())
    }

    fn reconnecting(&self, app: Rc<App>) {
        // log::debug!("Try to set a reconnecting timeout");
        let reconnecting = self.reconnecting.clone();
        // if TimeOut already exists, do nothing
        if reconnecting.get().is_none() {
            let timer = Timeout::new(
                9000,
                clone!(app, reconnecting => move || {
                    // log::debug!("Reconnecting EventSource..");
                    app.update_sw();
                    app.sse_end(0);
                    App::sse_new_anonymous(app.clone());
                    // Drop TimeOut here
                    reconnecting.set(None);
                }),
            );
            let handle = timer.handle();
            timer.forget();
            reconnecting.set(Some(handle));
            // log::debug!("Wait for reconnecting in 9 seconds..");
        }
    }

    pub fn render(page: Rc<Self>, app: Rc<App>) -> Dom {
        app.set_title("KPHIS - Login");
        body().set_attribute("data-bs-theme", "light").unwrap();
        body().set_class_name("bg-primary");

        // FIX suspend modal backdrop after `Unauthorized`
        // we use ok() instead of unwrap()
        if let Some(backdrop) = body().query_selector(".modal-backdrop.show").ok().flatten() {
            backdrop.remove();
        }

        html!("main", {
            // tab is hidden
            .future(app.visible.signal().for_each(clone!(app => move |visible| {
                if !visible {
                    // log::debug!("hit not-visible");
                    app.sse_end(2);
                }
                async {}
            })))
            // reconnecting
            .future(app.sse_ready_state.signal().dedupe().for_each(clone!(app, page => move |state| {
                // log::debug!("sse_ready_state = {}", state);
                match state {
                    1..3 => {
                        // Clear (if exists) reconnecting TimeOut (prevent reconnect again)
                        if let Some(handle) = page.reconnecting.get() {
                            Timeout::manual_drop(handle);
                            page.reconnecting.set(None);
                        }
                    }
                    3 => {
                        page.reconnecting(app.clone());
                    }
                    _ => {}
                }
                async {}
            })))
            // update SW and start SSE after browser tab gaining focus again
            .future(map_ref!{
                let busy = app.loader_is_loading(),
                let visible = app.visible.signal(),
                let no_sse = app.sse_ready_state.signal().map(|state| state == 2) =>
                !busy && *visible && *no_sse
            }.for_each(clone!(app => move |ready| {
                if ready {
                    // log::debug!("hit visible and renew");
                    app.update_sw();
                    app.sse_end(0);
                    App::start_sse_by_renew_token(app.clone());
                }
                async{}
            })))
            .future(page.is_success.signal().for_each(|is_success| {
                if is_success {
                    Route::Info.hard_redirect();
                }
                async {}
            }))
            // .class("bg-primary")
            // .style("height","100vh")
            .child(html!("div", {
                .class("container")
                .child(html!("div", {
                    .class(["row","justify-content-center","m-3"])
                    .child(html!("div", {
                        .class(["col","bg-white","py-3","rounded-3"])
                        .style("max-width","555px")
                        .children([
                            html!("h1",{
                                .class(class::TXT_C_BLUE)
                                .text("KPHIS Login")
                            }),
                            html!("hr",{.class("m-0")}),
                            html!("img",{
                                .attr("src","statics/picture/favicon/icon17.svg")
                                .style("width", "100%")
                                .class("img-fluid")
                                .attr("alt","")
                            }),
                        ])
                        .child_signal(app.sse_ready_state.signal().dedupe().map(clone!(app, page => move |ready_state| {
                            if ready_state == 0 {
                                Some(html!("h2", {
                                    .class(class::TXT_C_GREEN)
                                    .child(html!("i",{.class(class::FA_SPIN).style("font-size","32px")}))
                                    .text(" Connecting ...")
                                }))
                            } else if ready_state == 3 {
                                Some(html!("label", {
                                    .class(class::TXT_C_BX_RED)
                                    .style("cursor","pointer")
                                    .child(html!("h2", {
                                        .text("== OFFLINE ==")
                                    }))
                                    .event(clone!(app, page => move |_: events::Click| {
                                        app.update_sw();
                                        app.sse_end(0);
                                        App::sse_new_anonymous(app.clone());
                                    }))
                                }))
                            } else {
                                app.update_sw();
                                // - POST `EndPoint::User`
                                // - PATCH `EndPoint::User`
                                let login = LoginCpn::new(page.is_success.clone(), page.reconnecting.clone());
                                Some(html!("div", {
                                    .children([
                                        LoginCpn::render(login, app.clone()),
                                        html!("div", {
                                            .class("text-center")
                                            .child(html!("a", {
                                                .attr("href","/book/")
                                                .attr("target","_blank")
                                                .attr("title","Go to tutorial")
                                                .attr("rel","noopener noreferrer")
                                                .text("คู่มือการใช้งาน")
                                            }))
                                        }),
                                    ])
                                }))
                            }
                        })))
                        .children([
                            html!("button", {
                                .attr("type", "button")
                                .style("position","fixed")
                                .style("right","119px")
                                .style("bottom","0")
                                .style("border","1px solid #333")
                                .style("border-bottom-width","0px")
                                .style("border-radius","9px 9px 0 0")
                                .style("padding","0 9px")
                                .child(html!("i", {.class(class::FA_SEARCH)}))
                                .text(" Update")
                                .event(clone!(app => move |_: events::Click| {
                                    app.update_sw();
                                }))
                            }),
                            html!("button", {
                                .attr("type", "button")
                                .style("position","fixed")
                                .style("right","19px")
                                .style("bottom","0")
                                .style("border","1px solid #333")
                                .style("border-bottom-width","0px")
                                .style("border-radius","9px 9px 0 0")
                                .style("padding","0 9px")
                                .child(html!("i", {.class(class::FA_SYNC)}))
                                .text(" Reload")
                                .event(clone!(app => move |_: events::Click| {
                                    app.window.with(|w| w.location().reload().unwrap());
                                }))
                            }),
                        ])
                    }))
                }))
            }))
        })
    }
}
