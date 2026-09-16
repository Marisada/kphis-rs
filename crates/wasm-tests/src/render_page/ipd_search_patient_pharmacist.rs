use std::rc::Rc;
use wasm_bindgen_test::wasm_bindgen_test;

use crate::{new_app, replace_body};

#[wasm_bindgen_test]
async fn test_ipd_search_patient_pharmacist_page() {
    let app = new_app();
    let page = kphis_ui_page::ipd_search_patient_pharmacist::IpdSearchPatientPharmacistPage::new();
    let dom = kphis_ui_page::ipd_search_patient_pharmacist::IpdSearchPatientPharmacistPage::render(page, app);
    replace_body(dom).await;
}

#[wasm_bindgen_test]
async fn test_ipd_search_patient_pharmacist_page_card() {
    let app = new_app();
    let dom = kphis_ui_page::ipd_search_patient_pharmacist::render_card(Rc::new(kphis_model::search::ipd_search_patient_pharmacist::IpdSearchPatientPharmacistResponse::demo()), app);
    replace_body(dom).await;
}

#[wasm_bindgen_test]
async fn test_ipd_search_patient_pharmacist_page_table() {
    let app = new_app();
    let dom = kphis_ui_page::ipd_search_patient_pharmacist::render_table(1, Rc::new(kphis_model::search::ipd_search_patient_pharmacist::IpdSearchPatientPharmacistResponse::demo()), app);
    replace_body(dom).await;
}
