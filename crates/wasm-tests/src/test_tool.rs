// FOR EXPERIMENT ONLY

use dominator::{Dom, html};
use std::{rc::Rc, time::Duration};
use wasm_bindgen::JsValue;
use wasm_bindgen_futures::JsFuture;
use wasm_bindgen_test::wasm_bindgen_test;

use kphis_model::app::AppState;
use kphis_ui_app::App;

wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);

fn new_app() -> Rc<App> {
    let app_state = AppState::new_from_local_storage("/");
    App::new(app_state)
}

pub async fn replace_body(dom: Dom) {
    dominator::replace_dom(&dominator::body(), &dominator::body().first_child().unwrap(), dom);
}

pub async fn wait(millis: u64) {
    let started_at = web_time::Instant::now();
    let duration = Duration::from_millis(millis);

    loop {
        JsFuture::from(js_sys::Promise::resolve(&JsValue::null())).await.unwrap();
        if started_at.elapsed() > duration {
            break;
        }
    }
}

#[wasm_bindgen_test]
async fn test_tools() {
    // any log not show
    log::error!("ErRoR");

    // // assert will show assertion and backtrace
    //assert_eq!(1,2);

    let hello_dom = html!("div", {
        // // rust panic will show panic message and backtrace
        // .style("bla","bla")
        .text("HELLO")
    });

    let app = new_app();
    let page = kphis_ui_page::ipd_search_patient_dr::IpdSearchPatientDrPage::new();
    let rendered_page = kphis_ui_page::ipd_search_patient_dr::IpdSearchPatientDrPage::render(page, app);

    replace_body(html!("div", {
        .children([
            hello_dom,
            // headless result = ok: rendered_page fetch ?? => console.error => not show anything
            // browser result = ok: show console.error = GET http://127.0.0.1:8000/api/user 404 (Not Found)
            rendered_page,
        ])
    }))
    .await;

    // loop next tick to reach 1 second (also try 60 seconds, the same result)
    // still not show error in headless mode (result = ok)
    // wait(1000).await;
}
