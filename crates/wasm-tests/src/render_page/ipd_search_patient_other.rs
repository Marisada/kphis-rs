use std::rc::Rc;
use wasm_bindgen_test::wasm_bindgen_test;

use crate::{new_app, replace_body};

#[wasm_bindgen_test]
async fn test_ipd_search_patient_other_page() {
    let app = new_app();
    let page = kphis_ui_page::ipd_search_patient_other::IpdSearchPatientOtherPage::new();
    let dom = kphis_ui_page::ipd_search_patient_other::IpdSearchPatientOtherPage::render(page, app);
    replace_body(dom).await;
}

#[wasm_bindgen_test]
async fn test_ipd_search_patient_other_page_card() {
    let app = new_app();
    let dom = kphis_ui_page::ipd_search_patient_other::render_card(Rc::new(kphis_model::search::ipd_search_patient_other::IpdSearchPatientOtherResponse::demo()), app);
    replace_body(dom).await;
}

#[wasm_bindgen_test]
async fn test_ipd_search_patient_other_page_table() {
    let app = new_app();
    let dom = kphis_ui_page::ipd_search_patient_other::render_table(1, Rc::new(kphis_model::search::ipd_search_patient_other::IpdSearchPatientOtherResponse::demo()), app);
    replace_body(dom).await;
}
