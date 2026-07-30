use dominator::{Dom, body, clone, html};
use futures_signals::{
    map_ref,
    signal::{Mutable, SignalExt},
    signal_vec::{MutableVec, SignalVecExt},
};
use serde::{Deserialize, Serialize};
use std::rc::Rc;
use time::Date;

use kphis_model::{app::AppState, fetch::fetch_json_api};
use kphis_ui_app::App;
use kphis_ui_core::class;
use kphis_util::{
    datetime::date_th,
    error::{AppError, Source},
};

#[derive(Clone, Default)]
pub struct InfoPage {
    loaded: Mutable<bool>,
    announcements: MutableVec<Rc<Announcement>>,
}

impl InfoPage {
    pub fn new() -> Rc<Self> {
        Rc::new(Self::default())
    }

    fn load(page: Rc<Self>, app: Rc<App>) {
        app.async_load(
            true,
            clone!(app => async move {
                match Announcement::get(app.state()).await {
                    Ok(responses) => {
                        page.announcements.lock_mut().extend(responses.into_iter().map(Rc::new));
                    }
                    Err(e) => {
                        // silent error (for off-lined)
                        log::error!("{}", e.string());
                    }
                }
            }),
        );
    }

    pub fn render(page: Rc<Self>, app: Rc<App>) -> Dom {
        app.set_title("KPHIS - Info");
        body().set_class_name("");

        html!("section", {
            .future(map_ref!(
                let busy = app.loader_is_loading(),
                let loaded = page.loaded.signal() =>
                !busy && !loaded
            ).for_each(clone!(app, page => move |ready| {
                if ready {
                    Self::load(page.clone(), app.clone());
                    page.loaded.set(true);
                }
                async {}
            })))
            .class(["container","pt-3"])
            .child(html!("article", {
                .class("jumbotron")
                .child_signal(app.state().user.signal_cloned().map(|opt| {
                    opt.map(|user| {
                        html!("h1", {
                            .class("display-6")
                            .text("ยินดีต้อนรับ ")
                            .text_signal(user.user.name.signal_cloned())
                        })
                    })
                }))
                .children([
                    html!("p", {
                        .class("lead")
                        .text("กรุณาเลือกการทำงานของคุณจากเมนูด้านบน")
                    }),
                    html!("p", {
                        .class("lead")
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
            .children_signal_vec(page.announcements.signal_vec_cloned().map(|annot| {
                annot.render()
            }))
        })
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
struct Announcement {
    title: String,
    date: Date,
    items: Vec<String>,
}

impl Announcement {
    async fn get(app: Rc<AppState>) -> Result<Vec<Self>, AppError> {
        match fetch_json_api("announcement.json", "GET", None, app).await {
            Ok((response, true)) => {
                let response: Vec<Self> = serde_wasm_bindgen::from_value(response).map_err(|e| Source::SerdeWasm.to_teapot_error(e, "Fetch Announcement"))?;
                Ok(response)
            }
            Ok((app_error, false)) => {
                let error: AppError = serde_wasm_bindgen::from_value(app_error).map_err(|e| Source::SerdeWasm.to_teapot_error(e, "Fetch Announcement"))?;
                Err(error)
            }
            Err(e) => Err(AppError::new_from_js(e, "Fetch Json")),
        }
    }

    fn render(&self) -> Dom {
        html!("div", {
            .class(class::CARD)
            .children([
                html!("div", {
                    .class("card-header")
                    .attr("role","button")
                    .attr("data-bs-toggle","collapse")
                    .attr("data-bs-target",&["#info-",&self.date.to_string()].concat())
                    .child(html!("i",{.class(class::FA_BULLHORN)}))
                    .text("\u{00a0}\u{00a0}")
                    .text(&self.title)
                    .child(html!("span", {
                        .class("float-end")
                        .text(&date_th(&self.date))
                    }))
                }),
                html!("div",{
                    .class(["announcement","collapse"])
                    .attr("id",&["info-",&self.date.to_string()].concat())
                    .attr("data-bs-announce-date", &self.date.to_string())
                    .child(html!("div",{
                        .class("card-body")
                        .child(html!("div",{
                            .class("card-text")
                            .child(html!("ul",{
                                .children(self.items.iter().map(|item| {
                                    html!("li",{.text(&item)})
                                }))
                            }))
                        }))
                    }))
                })
            ])
        })
    }
}
