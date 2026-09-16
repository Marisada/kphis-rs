use futures_signals::signal::Mutable;
use time::macros::date;
use wasm_bindgen_test::wasm_bindgen_test;

use crate::{new_app, replace_body};

#[wasm_bindgen_test]
async fn test_ipd_mra_page() {
    let app = new_app();
    let page = kphis_ui_page::ipd_mra::IpdMraPage::new(String::from("660001234"));
    let dom = kphis_ui_page::ipd_mra::IpdMraPage::render(page, app);
    replace_body(dom).await;
}

#[wasm_bindgen_test]
async fn test_ipd_mra_page_body() {
    let app = new_app();
    let page = kphis_ui_page::ipd_mra::IpdMraPage::new(String::from("660001234"));
    let dom = kphis_ui_page::ipd_mra::IpdMraPage::render_body(page, app);
    replace_body(dom).await;
}

#[wasm_bindgen_test]
async fn test_ipd_mra_page_item() {
    let app = new_app();
    let page = kphis_ui_page::ipd_mra::IpdMraPage::new(String::from("660001234"));
    let inner = kphis_ui_page::ipd_mra::IpdMraMutable::new(&Some(String::from("660001234")), Some(date!(2023 - 12 - 31)), Some(date!(2024 - 01 - 11)), &Some(String::from("auditor")), page);
    let dom = kphis_ui_page::ipd_mra::IpdMraMutable::render(inner, Mutable::new(false), app);
    replace_body(dom).await;
}
