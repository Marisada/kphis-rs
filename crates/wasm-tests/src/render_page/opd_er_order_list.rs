use futures_signals::signal::Mutable;
use std::rc::Rc;
use wasm_bindgen_test::wasm_bindgen_test;

use crate::{new_app, replace_body};

#[wasm_bindgen_test]
async fn test_opd_er_order_list_page_doctor() {
    let app = new_app();
    let page = kphis_ui_page::opd_er_order_list::OpdErOrderListPage::new(String::from("doctor"));
    let dom = kphis_ui_page::opd_er_order_list::OpdErOrderListPage::render(page, app);
    replace_body(dom).await;
}
#[wasm_bindgen_test]
async fn test_opd_er_order_list_page_nurse() {
    let app = new_app();
    let page = kphis_ui_page::opd_er_order_list::OpdErOrderListPage::new(String::from("nurse"));
    let dom = kphis_ui_page::opd_er_order_list::OpdErOrderListPage::render(page, app);
    replace_body(dom).await;
}
#[wasm_bindgen_test]
async fn test_opd_er_order_list_page_pharmacist() {
    let app = new_app();
    let page = kphis_ui_page::opd_er_order_list::OpdErOrderListPage::new(String::from("pharmacist"));
    let dom = kphis_ui_page::opd_er_order_list::OpdErOrderListPage::render(page, app);
    replace_body(dom).await;
}
#[wasm_bindgen_test]
async fn test_opd_er_order_list_page_other() {
    let app = new_app();
    let page = kphis_ui_page::opd_er_order_list::OpdErOrderListPage::new(String::from("other"));
    let dom = kphis_ui_page::opd_er_order_list::OpdErOrderListPage::render(page, app);
    replace_body(dom).await;
}

#[wasm_bindgen_test]
async fn test_opd_er_order_list_page_card_doctor() {
    let app = new_app();
    let dom = kphis_ui_page::opd_er_order_list::render_card(Rc::new(kphis_model::opd_er::order_master::OpdErOrderMasterList::demo()), Mutable::new(String::from("doctor")), app);
    replace_body(dom).await;
}
#[wasm_bindgen_test]
async fn test_opd_er_order_list_page_card_nurse() {
    let app = new_app();
    let dom = kphis_ui_page::opd_er_order_list::render_card(Rc::new(kphis_model::opd_er::order_master::OpdErOrderMasterList::demo()), Mutable::new(String::from("nurse")), app);
    replace_body(dom).await;
}
#[wasm_bindgen_test]
async fn test_opd_er_order_list_page_card_pharmacist() {
    let app = new_app();
    let dom = kphis_ui_page::opd_er_order_list::render_card(Rc::new(kphis_model::opd_er::order_master::OpdErOrderMasterList::demo()), Mutable::new(String::from("pharmacist")), app);
    replace_body(dom).await;
}
#[wasm_bindgen_test]
async fn test_opd_er_order_list_page_card_other() {
    let app = new_app();
    let dom = kphis_ui_page::opd_er_order_list::render_card(Rc::new(kphis_model::opd_er::order_master::OpdErOrderMasterList::demo()), Mutable::new(String::from("other")), app);
    replace_body(dom).await;
}

#[wasm_bindgen_test]
async fn test_opd_er_order_list_page_table_doctor() {
    let app = new_app();
    let dom = kphis_ui_page::opd_er_order_list::render_table(1, Rc::new(kphis_model::opd_er::order_master::OpdErOrderMasterList::demo()), Mutable::new(String::from("doctor")), app);
    replace_body(dom).await;
}
#[wasm_bindgen_test]
async fn test_opd_er_order_list_page_table_nurse() {
    let app = new_app();
    let dom = kphis_ui_page::opd_er_order_list::render_table(1, Rc::new(kphis_model::opd_er::order_master::OpdErOrderMasterList::demo()), Mutable::new(String::from("nurse")), app);
    replace_body(dom).await;
}
#[wasm_bindgen_test]
async fn test_opd_er_order_list_page_table_pharmacist() {
    let app = new_app();
    let dom = kphis_ui_page::opd_er_order_list::render_table(1, Rc::new(kphis_model::opd_er::order_master::OpdErOrderMasterList::demo()), Mutable::new(String::from("pharmacist")), app);
    replace_body(dom).await;
}
#[wasm_bindgen_test]
async fn test_opd_er_order_list_page_table_other() {
    let app = new_app();
    let dom = kphis_ui_page::opd_er_order_list::render_table(1, Rc::new(kphis_model::opd_er::order_master::OpdErOrderMasterList::demo()), Mutable::new(String::from("other")), app);
    replace_body(dom).await;
}
