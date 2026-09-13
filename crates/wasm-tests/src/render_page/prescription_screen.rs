use wasm_bindgen_test::wasm_bindgen_test;

use crate::{new_app, replace_body};

#[wasm_bindgen_test]
async fn test_prescription_screen_page() {
    let app = new_app();
    let page = kphis_ui_page::prescription_screen::PrescriptionScreenPage::new(String::from("0001234"));
    let dom = kphis_ui_page::prescription_screen::PrescriptionScreenPage::render(page, app);
    replace_body(dom).await;
}
