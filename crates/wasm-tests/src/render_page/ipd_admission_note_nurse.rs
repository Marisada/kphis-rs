use wasm_bindgen_test::wasm_bindgen_test;

use crate::{new_app, replace_body};

#[wasm_bindgen_test]
async fn test_ipd_admission_note_nurse_page() {
    let app = new_app();
    let page = kphis_ui_page::ipd_admission_note_nurse::IpdAdmissionNoteNursePage::new(String::from("660001234"), app.clone());
    let dom = kphis_ui_page::ipd_admission_note_nurse::IpdAdmissionNoteNursePage::render(page, app);
    replace_body(dom).await;
}
