use futures_signals::signal_vec::MutableVec;
use std::rc::Rc;
use wasm_bindgen_test::wasm_bindgen_test;

use crate::{new_app, replace_body};

#[wasm_bindgen_test]
async fn test_ipd_order_pharmacy_page() {
    let app = new_app();
    let page = kphis_ui_page::ipd_order_pharmacy::IpdOrderPharmacyPage::new();
    let dom = kphis_ui_page::ipd_order_pharmacy::IpdOrderPharmacyPage::render(page, app);
    replace_body(dom).await;
}

#[wasm_bindgen_test]
async fn test_ipd_order_pharmacy_page_pharmacy_order() {
    let app = new_app();
    let dom = kphis_ui_page::ipd_order_pharmacy::render_pharmacy_order(1, Rc::new(kphis_model::ipd::pharmacy_monitor::IpdOrderPharmacy::demo()), app);
    replace_body(dom).await;
}

#[wasm_bindgen_test]
async fn test_ipd_order_pharmacy_page_order_table() {
    let app = new_app();
    let dom = kphis_ui_page::ipd_order_pharmacy::render_order_table(MutableVec::new_with_values(vec![Rc::new(kphis_model::ipd::pharmacy_monitor::IpdOrderPharmacy::demo())]), "label", app);
    replace_body(dom).await;
}

#[wasm_bindgen_test]
async fn test_ipd_order_pharmacy_page_ipt_table() {
    let dom = kphis_ui_page::ipd_order_pharmacy::render_ipt_table(MutableVec::new_with_values(vec![Rc::new(kphis_model::ipd::pharmacy_monitor::PharmacyIpt::demo())]), "label");
    replace_body(dom).await;
}

#[wasm_bindgen_test]
async fn test_ipd_order_pharmacy_page_pharmacy_ipt() {
    let dom = kphis_ui_page::ipd_order_pharmacy::render_pharmacy_ipt(1, Rc::new(kphis_model::ipd::pharmacy_monitor::PharmacyIpt::demo()));
    replace_body(dom).await;
}
