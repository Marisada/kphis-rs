use wasm_bindgen_test::wasm_bindgen_test;

use crate::{new_app, replace_body};

#[wasm_bindgen_test]
async fn test_vital_sign_page_ipd() {
    let app = new_app();
    let page = kphis_ui_page::vital_sign::VitalSignPage::new(true);
    let dom = kphis_ui_page::vital_sign::VitalSignPage::render(page, app);
    replace_body(dom).await;
}
#[wasm_bindgen_test]
async fn test_vital_sign_page_opd_er() {
    let app = new_app();
    let page = kphis_ui_page::vital_sign::VitalSignPage::new(false);
    let dom = kphis_ui_page::vital_sign::VitalSignPage::render(page, app);
    replace_body(dom).await;
}
