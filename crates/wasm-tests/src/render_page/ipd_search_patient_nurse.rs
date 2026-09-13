use wasm_bindgen_test::wasm_bindgen_test;

use crate::{new_app, replace_body};

#[wasm_bindgen_test]
async fn test_ipd_search_patient_nurse_page() {
    let app = new_app();
    let page = kphis_ui_page::ipd_search_patient_nurse::IpdSearchPatientNursePage::new();
    let dom = kphis_ui_page::ipd_search_patient_nurse::IpdSearchPatientNursePage::render(page, app);
    replace_body(dom).await;
}
