use dominator::{Dom, body, clone, events, html};
use futures_signals::{
    map_ref,
    signal::{Mutable, SignalExt},
};
use std::rc::Rc;

use kphis_model::{fetch::get_text_from_url, route::Route, timer::Timeout};
use kphis_ui_app::App;
use kphis_ui_component::login::LoginCpn;
use kphis_ui_core::class;

/// - POST `EndPoint::User`
/// - PATCH `EndPoint::User`
#[derive(Clone, Default)]
pub struct IndexPage {
    is_success: Mutable<bool>,
    reconnecting: Mutable<Option<i32>>,

    load_oauth_uri: Mutable<bool>,
    oauth_uri: Mutable<String>,
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

    fn load_oauth_uri(page: Rc<Self>, app: Rc<App>) {
        app.async_load(
            false,
            clone!(app, page => async move {
                match get_text_from_url("provider/oauth", app.state()).await {
                    Ok(uri_opt) => {
                        page.oauth_uri.set(uri_opt.unwrap_or_default());
                    }
                    Err(e) => {
                        app.alert_app_error(&e).await;
                    }
                }
            }),
        );
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
            .future(map_ref!{
                let busy = app.loader_is_loading(),
                let load_oauth = page.load_oauth_uri.signal() =>
                !busy && *load_oauth
            }.for_each(clone!(app, page => move |ready| {
                if ready {
                    Self::load_oauth_uri(page.clone(), app.clone());
                    page.load_oauth_uri.set(false);
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
                                Some(html!("span", {
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
                                // load OAUTH uri + stage
                                page.load_oauth_uri.set(true);
                                Some(html!("div", {
                                    .child(LoginCpn::render(login, app.clone()))
                                    .child_signal(page.oauth_uri.signal_cloned().map(clone!(app, page => move |oauth_uri| {
                                        (!oauth_uri.is_empty()).then(|| {
                                            html!("button", {
                                                .attr("type", "button")
                                                .class(class::BTN_T_CTRL_LIGHT_P0)
                                                .children([
                                                    html!("span", {
                                                        .class("me-1")
                                                        .style("vertical-align","sub")
                                                        .text("เข้าใช้งานด้วย")
                                                    }),
                                                    html!("img", {
                                                        .attr("src","statics/picture/logo-provider-id.webp")
                                                        .style("height","40px")
                                                    }),
                                                ])
                                                .event(clone!(app, page => move |_:events::Click| {
                                                    app.window.with(|w| {
                                                        w.location().set_href(&page.oauth_uri.lock_ref()).unwrap();
                                                    })
                                                }))
                                            })
                                        })
                                    })))
                                    .child(html!("div", {
                                        .class("text-center")
                                        .child(html!("a", {
                                            .attr("href","/book/")
                                            .attr("target","_blank")
                                            .attr("title","Go to tutorial")
                                            .attr("rel","noopener noreferrer")
                                            .text("คู่มือการใช้งาน")
                                        }))
                                    }))
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
