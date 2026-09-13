use wasm_bindgen_test::wasm_bindgen_test;

use crate::{new_app, replace_body};

#[wasm_bindgen_test]
async fn test_opd_er_main_page_doctor() {
    let app = new_app();
    let page = kphis_ui_page::opd_er_main::OpdErMainPage::new(String::from("doctor"), 0, kphis_model::tab::Tab::Order, 1);
    let dom = kphis_ui_page::opd_er_main::OpdErMainPage::render(page, app);
    replace_body(dom).await;
}
#[wasm_bindgen_test]
async fn test_opd_er_main_page_nurse() {
    let app = new_app();
    let page = kphis_ui_page::opd_er_main::OpdErMainPage::new(String::from("nurse"), 0, kphis_model::tab::Tab::Order, 1);
    let dom = kphis_ui_page::opd_er_main::OpdErMainPage::render(page, app);
    replace_body(dom).await;
}
#[wasm_bindgen_test]
async fn test_opd_er_main_page_pharmacist() {
    let app = new_app();
    let page = kphis_ui_page::opd_er_main::OpdErMainPage::new(String::from("pharmacist"), 0, kphis_model::tab::Tab::Order, 1);
    let dom = kphis_ui_page::opd_er_main::OpdErMainPage::render(page, app);
    replace_body(dom).await;
}
#[wasm_bindgen_test]
async fn test_opd_er_main_page_other() {
    let app = new_app();
    let page = kphis_ui_page::opd_er_main::OpdErMainPage::new(String::from("other"), 0, kphis_model::tab::Tab::Order, 1);
    let dom = kphis_ui_page::opd_er_main::OpdErMainPage::render(page, app);
    replace_body(dom).await;
}
