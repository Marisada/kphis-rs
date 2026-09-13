use wasm_bindgen_test::wasm_bindgen_test;

use crate::{new_app, replace_body};

#[wasm_bindgen_test]
async fn test_ipd_consult_list_page_doctor() {
    let app = new_app();
    let page = kphis_ui_page::ipd_consult_list::IpdConsultListPage::new(String::from("doctor"));
    let dom = kphis_ui_page::ipd_consult_list::IpdConsultListPage::render(page, app);
    replace_body(dom).await;
}
#[wasm_bindgen_test]
async fn test_ipd_consult_list_page_nurse() {
    let app = new_app();
    let page = kphis_ui_page::ipd_consult_list::IpdConsultListPage::new(String::from("nurse"));
    let dom = kphis_ui_page::ipd_consult_list::IpdConsultListPage::render(page, app);
    replace_body(dom).await;
}
#[wasm_bindgen_test]
async fn test_ipd_consult_list_page_pharmacist() {
    let app = new_app();
    let page = kphis_ui_page::ipd_consult_list::IpdConsultListPage::new(String::from("pharmacist"));
    let dom = kphis_ui_page::ipd_consult_list::IpdConsultListPage::render(page, app);
    replace_body(dom).await;
}
#[wasm_bindgen_test]
async fn test_ipd_consult_list_page_other() {
    let app = new_app();
    let page = kphis_ui_page::ipd_consult_list::IpdConsultListPage::new(String::from("other"));
    let dom = kphis_ui_page::ipd_consult_list::IpdConsultListPage::render(page, app);
    replace_body(dom).await;
}
