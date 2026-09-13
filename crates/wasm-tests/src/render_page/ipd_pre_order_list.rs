use wasm_bindgen_test::wasm_bindgen_test;

use crate::{new_app, replace_body};

#[wasm_bindgen_test]
async fn test_ipd_pre_order_list_page_doctor() {
    let app = new_app();
    let page = kphis_ui_page::ipd_pre_order_list::IpdPreOrderListPage::new("doctor", app.clone());
    let dom = kphis_ui_page::ipd_pre_order_list::IpdPreOrderListPage::render(page, app);
    replace_body(dom).await;
}
#[wasm_bindgen_test]
async fn test_ipd_pre_order_list_page_nurse() {
    let app = new_app();
    let page = kphis_ui_page::ipd_pre_order_list::IpdPreOrderListPage::new("nurse", app.clone());
    let dom = kphis_ui_page::ipd_pre_order_list::IpdPreOrderListPage::render(page, app);
    replace_body(dom).await;
}
#[wasm_bindgen_test]
async fn test_ipd_pre_order_list_page_pharmacist() {
    let app = new_app();
    let page = kphis_ui_page::ipd_pre_order_list::IpdPreOrderListPage::new("pharmacist", app.clone());
    let dom = kphis_ui_page::ipd_pre_order_list::IpdPreOrderListPage::render(page, app);
    replace_body(dom).await;
}
#[wasm_bindgen_test]
async fn test_ipd_pre_order_list_page_other() {
    let app = new_app();
    let page = kphis_ui_page::ipd_pre_order_list::IpdPreOrderListPage::new("other", app.clone());
    let dom = kphis_ui_page::ipd_pre_order_list::IpdPreOrderListPage::render(page, app);
    replace_body(dom).await;
}
