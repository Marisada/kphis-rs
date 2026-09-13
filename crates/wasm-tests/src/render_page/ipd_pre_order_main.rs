use wasm_bindgen_test::wasm_bindgen_test;

use crate::{new_app, replace_body};

#[wasm_bindgen_test]
async fn test_ipd_pre_order_main_page_doctor() {
    let app = new_app();
    let page = kphis_ui_page::ipd_pre_order_main::IpdPreOrderPage::new("doctor", 1);
    let dom = kphis_ui_page::ipd_pre_order_main::IpdPreOrderPage::render(page, app);
    replace_body(dom).await;
}
#[wasm_bindgen_test]
async fn test_ipd_pre_order_main_page_nurse() {
    let app = new_app();
    let page = kphis_ui_page::ipd_pre_order_main::IpdPreOrderPage::new("nurse", 1);
    let dom = kphis_ui_page::ipd_pre_order_main::IpdPreOrderPage::render(page, app);
    replace_body(dom).await;
}
#[wasm_bindgen_test]
async fn test_ipd_pre_order_main_page_pharmacist() {
    let app = new_app();
    let page = kphis_ui_page::ipd_pre_order_main::IpdPreOrderPage::new("pharmacist", 1);
    let dom = kphis_ui_page::ipd_pre_order_main::IpdPreOrderPage::render(page, app);
    replace_body(dom).await;
}
#[wasm_bindgen_test]
async fn test_ipd_pre_order_main_page_other() {
    let app = new_app();
    let page = kphis_ui_page::ipd_pre_order_main::IpdPreOrderPage::new("other", 1);
    let dom = kphis_ui_page::ipd_pre_order_main::IpdPreOrderPage::render(page, app);
    replace_body(dom).await;
}
