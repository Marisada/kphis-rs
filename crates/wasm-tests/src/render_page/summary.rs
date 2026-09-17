use wasm_bindgen_test::wasm_bindgen_test;

use crate::{new_app, replace_body};

#[wasm_bindgen_test]
async fn test_summary_page_doctor() {
    let app = new_app();
    let page = kphis_ui_page::summary::SummaryPage::new(String::from("doctor"), String::from("660001234"));
    let dom = kphis_ui_page::summary::SummaryPage::render(page, app);
    replace_body(dom).await;
}
#[wasm_bindgen_test]
async fn test_summary_page_nurse() {
    let app = new_app();
    let page = kphis_ui_page::summary::SummaryPage::new(String::from("nurse"), String::from("660001234"));
    let dom = kphis_ui_page::summary::SummaryPage::render(page, app);
    replace_body(dom).await;
}
#[wasm_bindgen_test]
async fn test_summary_page_pharmacist() {
    let app = new_app();
    let page = kphis_ui_page::summary::SummaryPage::new(String::from("pharmacist"), String::from("660001234"));
    let dom = kphis_ui_page::summary::SummaryPage::render(page, app);
    replace_body(dom).await;
}
#[wasm_bindgen_test]
async fn test_summary_page_other() {
    let app = new_app();
    let page = kphis_ui_page::summary::SummaryPage::new(String::from("other"), String::from("660001234"));
    let dom = kphis_ui_page::summary::SummaryPage::render(page, app);
    replace_body(dom).await;
}

#[wasm_bindgen_test]
async fn test_summary_page_form() {
    let app = new_app();
    let page = kphis_ui_page::summary::SummaryPage::new(String::from("doctor"), String::from("660001234"));
    let dom = kphis_ui_page::summary::SummaryPage::render_form(page, app);
    replace_body(dom).await;
}

#[wasm_bindgen_test]
async fn test_summary_page_coder() {
    let app = new_app();
    let page = kphis_ui_page::summary::SummaryPage::new(String::from("doctor"), String::from("660001234"));
    let dom = kphis_ui_page::summary::SummaryPage::render_coder(false, page, app);
    replace_body(dom).await;
}

#[wasm_bindgen_test]
async fn test_summary_page_coder_pre() {
    let app = new_app();
    let page = kphis_ui_page::summary::SummaryPage::new(String::from("doctor"), String::from("660001234"));
    let dom = kphis_ui_page::summary::SummaryPage::render_coder(true, page, app);
    replace_body(dom).await;
}

#[wasm_bindgen_test]
async fn test_summary_page_review() {
    let app = new_app();
    let page = kphis_ui_page::summary::SummaryPage::new(String::from("doctor"), String::from("660001234"));
    let dom = kphis_ui_page::summary::SummaryPage::render_review_status(page, app);
    replace_body(dom).await;
}
