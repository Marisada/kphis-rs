use wasm_bindgen_test::wasm_bindgen_test;

use crate::{new_app, replace_body};

#[wasm_bindgen_test]
async fn test_index_plan_page_ipd() {
    let app = new_app();
    let page = kphis_ui_page::index_plan::IndexPlanPage::new_ipd();
    let dom = kphis_ui_page::index_plan::IndexPlanPage::render(page, app);
    replace_body(dom).await;
}
#[wasm_bindgen_test]
async fn test_index_plan_page_opd_er() {
    let app = new_app();
    let page = kphis_ui_page::index_plan::IndexPlanPage::new_opd_er();
    let dom = kphis_ui_page::index_plan::IndexPlanPage::render(page, app);
    replace_body(dom).await;
}
