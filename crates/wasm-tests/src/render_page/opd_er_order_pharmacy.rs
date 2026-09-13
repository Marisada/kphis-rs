use wasm_bindgen_test::wasm_bindgen_test;

use crate::{new_app, replace_body};

#[wasm_bindgen_test]
async fn test_opd_er_order_pharmacy_page() {
    let app = new_app();
    let page = kphis_ui_page::opd_er_order_pharmacy::OpdErOrderPharmacyPage::new();
    let dom = kphis_ui_page::opd_er_order_pharmacy::OpdErOrderPharmacyPage::render(page, app);
    replace_body(dom).await;
}
