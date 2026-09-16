use futures_signals::signal_vec::MutableVec;
use std::rc::Rc;
use wasm_bindgen_test::wasm_bindgen_test;

use crate::{new_app, replace_body};

#[wasm_bindgen_test]
async fn test_opd_er_order_pharmacy_page() {
    let app = new_app();
    let page = kphis_ui_page::opd_er_order_pharmacy::OpdErOrderPharmacyPage::new();
    let dom = kphis_ui_page::opd_er_order_pharmacy::OpdErOrderPharmacyPage::render(page, app);
    replace_body(dom).await;
}

#[wasm_bindgen_test]
async fn test_opd_er_order_pharmacy_page_table() {
    let app = new_app();
    let dom = kphis_ui_page::opd_er_order_pharmacy::render_order_table(MutableVec::new_with_values(vec![Rc::new(kphis_model::opd_er::pharmacy_monitor::OpdErOrderPharmacy::demo())]), "label", app);
    replace_body(dom).await;
}
