use dominator::Dom;
use wasm_bindgen::JsValue;
use wasm_bindgen_futures::JsFuture;
use wasm_bindgen_test::wasm_bindgen_test;

use kphis_model::app::AppState;

wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);

pub async fn replace_body(dom: Dom) {
    dominator::replace_dom(
        &dominator::body(),
        &dominator::body().first_child().unwrap(),
        dom,
    );
    // move to next tick
    JsFuture::from(js_sys::Promise::resolve(&JsValue::null())).await.unwrap();
}

#[wasm_bindgen_test]
async fn test_confirm() {
    let cpn = kphis_ui_core::popups::confirm::ConfirmPopup::new("caption", "message");
    let dom = kphis_ui_core::popups::confirm::ConfirmPopup::render(cpn);
    replace_body(dom).await;
}

#[wasm_bindgen_test]
async fn test_dom_with_cloes() {
    let cpn = kphis_ui_core::popups::dom_with_close::DomWithClosePopup::new("title", false);
    let dom = kphis_ui_core::popups::dom_with_close::DomWithClosePopup::render(Dom::empty(), cpn);
    replace_body(dom).await;
}
#[wasm_bindgen_test]
async fn test_dom_with_cloes_error() {
    let cpn = kphis_ui_core::popups::dom_with_close::DomWithClosePopup::new("title", true);
    let dom = kphis_ui_core::popups::dom_with_close::DomWithClosePopup::render(Dom::empty(), cpn);
    replace_body(dom).await;
}

#[wasm_bindgen_test]
async fn test_prompt_password() {
    let app_state = AppState::new_from_local_storage("/");
    let cpn = kphis_ui_core::popups::prompt_password::PromptPasswordPopup::new(false);
    let dom = kphis_ui_core::popups::prompt_password::PromptPasswordPopup::render(cpn, app_state);
    replace_body(dom).await;
}
#[wasm_bindgen_test]
async fn test_prompt_password_done() {
    let app_state = AppState::new_from_local_storage("/");
    let cpn = kphis_ui_core::popups::prompt_password::PromptPasswordPopup::new(true);
    let dom = kphis_ui_core::popups::prompt_password::PromptPasswordPopup::render(cpn, app_state);
    replace_body(dom).await;
}

#[wasm_bindgen_test]
async fn test_with_close() {
    let cpn = kphis_ui_core::popups::with_close::WithClosePopup::new("title", "message", false);
    let dom = kphis_ui_core::popups::with_close::WithClosePopup::render(cpn);
    replace_body(dom).await;
}
#[wasm_bindgen_test]
async fn test_with_close_error() {
    let cpn = kphis_ui_core::popups::with_close::WithClosePopup::new("title", "message", true);
    let dom = kphis_ui_core::popups::with_close::WithClosePopup::render(cpn);
    replace_body(dom).await;
}
