use wasm_bindgen_test::wasm_bindgen_test;

use crate::{new_app, replace_body};

#[wasm_bindgen_test]
async fn test_ipd_main_page_doctor() {
    let app = new_app();
    let page = kphis_ui_page::ipd_main::IpdMainPage::new(String::from("doctor"), String::from("660001234"), kphis_model::tab::Tab::Order, String::from("order"), 1);
    let dom = kphis_ui_page::ipd_main::IpdMainPage::render(page, app);
    replace_body(dom).await;
}
#[wasm_bindgen_test]
async fn test_ipd_main_page_nurse() {
    let app = new_app();
    let page = kphis_ui_page::ipd_main::IpdMainPage::new(String::from("nurse"), String::from("660001234"), kphis_model::tab::Tab::Order, String::from("order"), 1);
    let dom = kphis_ui_page::ipd_main::IpdMainPage::render(page, app);
    replace_body(dom).await;
}
#[wasm_bindgen_test]
async fn test_ipd_main_page_pharmacist() {
    let app = new_app();
    let page = kphis_ui_page::ipd_main::IpdMainPage::new(String::from("pharmacist"), String::from("660001234"), kphis_model::tab::Tab::Order, String::from("order"), 1);
    let dom = kphis_ui_page::ipd_main::IpdMainPage::render(page, app);
    replace_body(dom).await;
}
#[wasm_bindgen_test]
async fn test_ipd_main_page_other() {
    let app = new_app();
    let page = kphis_ui_page::ipd_main::IpdMainPage::new(String::from("other"), String::from("660001234"), kphis_model::tab::Tab::Order, String::from("order"), 1);
    let dom = kphis_ui_page::ipd_main::IpdMainPage::render(page, app);
    replace_body(dom).await;
}
