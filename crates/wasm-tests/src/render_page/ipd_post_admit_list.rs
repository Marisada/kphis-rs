use wasm_bindgen_test::wasm_bindgen_test;

use crate::{new_app, replace_body};

#[wasm_bindgen_test]
async fn test_ipd_post_admit_list_page_doctor() {
    let app = new_app();
    let page = kphis_ui_page::ipd_post_admit_list::IpdPostAdmitListPage::new("doctor");
    let dom = kphis_ui_page::ipd_post_admit_list::IpdPostAdmitListPage::render(page, app);
    replace_body(dom).await;
}
#[wasm_bindgen_test]
async fn test_ipd_post_admit_list_page_nurse() {
    let app = new_app();
    let page = kphis_ui_page::ipd_post_admit_list::IpdPostAdmitListPage::new("nurse");
    let dom = kphis_ui_page::ipd_post_admit_list::IpdPostAdmitListPage::render(page, app);
    replace_body(dom).await;
}
#[wasm_bindgen_test]
async fn test_ipd_post_admit_list_page_pharmacist() {
    let app = new_app();
    let page = kphis_ui_page::ipd_post_admit_list::IpdPostAdmitListPage::new("pharmacist");
    let dom = kphis_ui_page::ipd_post_admit_list::IpdPostAdmitListPage::render(page, app);
    replace_body(dom).await;
}
#[wasm_bindgen_test]
async fn test_ipd_post_admit_list_page_other() {
    let app = new_app();
    let page = kphis_ui_page::ipd_post_admit_list::IpdPostAdmitListPage::new("other");
    let dom = kphis_ui_page::ipd_post_admit_list::IpdPostAdmitListPage::render(page, app);
    replace_body(dom).await;
}
