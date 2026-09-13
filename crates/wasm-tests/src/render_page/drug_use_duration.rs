use std::rc::Rc;
use wasm_bindgen_test::wasm_bindgen_test;

use crate::{new_app, replace_body};

#[wasm_bindgen_test]
async fn test_drug_use_duration_page() {
    let app = new_app();
    let page = kphis_ui_page::drug_use_duration::DrugUseDurationPage::new();
    let dom = kphis_ui_page::drug_use_duration::DrugUseDurationPage::render(page, app);
    replace_body(dom).await;
}
#[wasm_bindgen_test]
async fn test_drug_use_duration_page_result() {
    let app = new_app();
    let page = kphis_ui_page::drug_use_duration::DrugUseDurationPage::new();
    let dom = kphis_ui_page::drug_use_duration::DrugUseDurationPage::render_result(1, Rc::new(kphis_model::drug_use_duration::DrugUseDuration::demo()), page, app);
    replace_body(dom).await;
}
