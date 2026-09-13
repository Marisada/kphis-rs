mod datetime_picker;
mod dom;
mod popup;

use dominator::Dom;
use futures_signals::signal::Mutable;
use std::rc::Rc;
use wasm_bindgen::JsValue;
use wasm_bindgen_futures::JsFuture;
use wasm_bindgen_test::wasm_bindgen_test;

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
async fn test_draggable() {
    let cpn = kphis_ui_core::draggable::Dragable::new(Some(9));
    let dom = kphis_ui_core::draggable::Dragable::render(Rc::new(kphis_ui_core::draggable::Group::new()), cpn, Mutable::new(None), Dom::empty(), true);
    replace_body(dom).await;
}
#[wasm_bindgen_test]
async fn test_draggable_fixed() {
    let cpn = kphis_ui_core::draggable::Dragable::new(Some(9));
    let dom = kphis_ui_core::draggable::Dragable::render(Rc::new(kphis_ui_core::draggable::Group::new()), cpn, Mutable::new(None), Dom::empty(), false);
    replace_body(dom).await;
}

#[wasm_bindgen_test]
async fn test_pincode() {
    let cpn = kphis_ui_core::pin_code::PinCode::new(Mutable::new(String::from("123456")), Mutable::new(false));
    let dom = kphis_ui_core::pin_code::PinCode::render(cpn);
    replace_body(dom).await;
}

#[wasm_bindgen_test]
async fn test_resizable() {
    let cpn = kphis_ui_core::resizable::Resizable::new(55.0, false);
    let dom = kphis_ui_core::resizable::Resizable::render(cpn, Rc::new(kphis_ui_core::resizable::ResizeState::default()));
    replace_body(dom).await;
}
#[wasm_bindgen_test]
async fn test_resizable_vertical() {
    let cpn = kphis_ui_core::resizable::Resizable::new(55.0, true);
    let dom = kphis_ui_core::resizable::Resizable::render(cpn, Rc::new(kphis_ui_core::resizable::ResizeState::default()));
    replace_body(dom).await;
}
