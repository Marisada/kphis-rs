use std::rc::Rc;
use wasm_bindgen_test::wasm_bindgen_test;

use crate::{new_app, replace_body};

#[wasm_bindgen_test]
async fn test_ipd_pre_admit_list_page_doctor() {
    let app = new_app();
    let page = kphis_ui_page::ipd_pre_admit_list::IpdPreAdmitListPage::new("doctor");
    let dom = kphis_ui_page::ipd_pre_admit_list::IpdPreAdmitListPage::render(page, app);
    replace_body(dom).await;
}
#[wasm_bindgen_test]
async fn test_ipd_pre_admit_list_page_nurse() {
    let app = new_app();
    let page = kphis_ui_page::ipd_pre_admit_list::IpdPreAdmitListPage::new("nurse");
    let dom = kphis_ui_page::ipd_pre_admit_list::IpdPreAdmitListPage::render(page, app);
    replace_body(dom).await;
}
#[wasm_bindgen_test]
async fn test_ipd_pre_admit_list_page_pharmacist() {
    let app = new_app();
    let page = kphis_ui_page::ipd_pre_admit_list::IpdPreAdmitListPage::new("pharmacist");
    let dom = kphis_ui_page::ipd_pre_admit_list::IpdPreAdmitListPage::render(page, app);
    replace_body(dom).await;
}
#[wasm_bindgen_test]
async fn test_ipd_pre_admit_list_page_other() {
    let app = new_app();
    let page = kphis_ui_page::ipd_pre_admit_list::IpdPreAdmitListPage::new("other");
    let dom = kphis_ui_page::ipd_pre_admit_list::IpdPreAdmitListPage::render(page, app);
    replace_body(dom).await;
}

#[wasm_bindgen_test]
async fn test_ipd_pre_admit_list_page_card() {
    let app = new_app();
    let page = kphis_ui_page::ipd_pre_admit_list::IpdPreAdmitListPage::new("doctor");
    let dom = kphis_ui_page::ipd_pre_admit_list::IpdPreAdmitListPage::render_card(Rc::new(kphis_model::pre_admit::PreAdmitList::demo()), page, app);
    replace_body(dom).await;
}

#[wasm_bindgen_test]
async fn test_ipd_pre_admit_list_page_table() {
    let app = new_app();
    let page = kphis_ui_page::ipd_pre_admit_list::IpdPreAdmitListPage::new("doctor");
    let dom = kphis_ui_page::ipd_pre_admit_list::IpdPreAdmitListPage::render_table(1, Rc::new(kphis_model::pre_admit::PreAdmitList::demo()), page, app);
    replace_body(dom).await;
}
