use std::rc::Rc;
use wasm_bindgen_test::wasm_bindgen_test;

use crate::{new_app, replace_body};

#[wasm_bindgen_test]
async fn test_prescription_screen_page() {
    let app = new_app();
    let page = kphis_ui_page::prescription_screen::PrescriptionScreenPage::new(String::from("0001234"));
    let dom = kphis_ui_page::prescription_screen::PrescriptionScreenPage::render(page, app);
    replace_body(dom).await;
}

#[wasm_bindgen_test]
async fn test_prescription_screen_page_info_patient() {
    let dom = kphis_ui_page::prescription_screen::PrescriptionScreenPage::render_info_patient(&Rc::new(kphis_model::prescription::PrescriptionInfo::demo()));
    replace_body(dom).await;
}

#[wasm_bindgen_test]
async fn test_prescription_screen_page_info_allergy() {
    let dom = kphis_ui_page::prescription_screen::PrescriptionScreenPage::render_info_allergy(&Rc::new(kphis_model::prescription::PrescriptionInfo::demo()));
    replace_body(dom).await;
}

#[wasm_bindgen_test]
async fn test_prescription_screen_page_info_note() {
    let dom = kphis_ui_page::prescription_screen::PrescriptionScreenPage::render_info_note(&Rc::new(kphis_model::prescription::PrescriptionInfo::demo()));
    replace_body(dom).await;
}

#[wasm_bindgen_test]
async fn test_prescription_screen_page_visit_hx() {
    let page = kphis_ui_page::prescription_screen::PrescriptionScreenPage::new(String::from("0001234"));
    let dom = kphis_ui_page::prescription_screen::PrescriptionScreenPage::render_visit_hx(&Rc::new(kphis_model::prescription::PrescriptionVn::demo()), page);
    replace_body(dom).await;
}

#[wasm_bindgen_test]
async fn test_prescription_screen_page_visit_drugs() {
    let app = new_app();
    let page = kphis_ui_page::prescription_screen::PrescriptionScreenPage::new(String::from("0001234"));
    page.set_visit(Some(kphis_model::prescription::PrescriptionVn::demo()));
    let dom = kphis_ui_page::prescription_screen::PrescriptionScreenPage::render_visit_drugs(page, app);
    replace_body(dom).await;
}

#[wasm_bindgen_test]
async fn test_prescription_screen_page_drug_interaction() {
    let dom = kphis_ui_page::prescription_screen::PrescriptionScreenPage::render_drug_interaction(&Rc::new(kphis_model::prescription::PrescriptionVn::demo()));
    replace_body(dom).await;
}

#[wasm_bindgen_test]
async fn test_prescription_screen_page_labs() {
    let app = new_app();
    let page = kphis_ui_page::prescription_screen::PrescriptionScreenPage::new(String::from("0001234"));
    let dom = kphis_ui_page::prescription_screen::PrescriptionScreenPage::render_labs(&Rc::new(kphis_model::prescription::PrescriptionVn::demo()), page, app);
    replace_body(dom).await;
}

#[wasm_bindgen_test]
async fn test_prescription_screen_page_visit_message() {
    let dom = kphis_ui_page::prescription_screen::PrescriptionScreenPage::render_visit_message(&Rc::new(kphis_model::prescription::PrescriptionVn::demo()));
    replace_body(dom).await;
}

#[wasm_bindgen_test]
async fn test_prescription_screen_page_visit_action() {
    let app = new_app();
    let page = kphis_ui_page::prescription_screen::PrescriptionScreenPage::new(String::from("0001234"));
    let dom = kphis_ui_page::prescription_screen::PrescriptionScreenPage::render_visit_action(&Rc::new(kphis_model::prescription::PrescriptionVn::demo()), page, app);
    replace_body(dom).await;
}

#[wasm_bindgen_test]
async fn test_prescription_screen_page_visit_postal() {
    let app = new_app();
    let page = kphis_ui_page::prescription_screen::PrescriptionScreenPage::new(String::from("0001234"));
    let dom = kphis_ui_page::prescription_screen::PrescriptionScreenPage::render_visit_postal(&Rc::new(kphis_model::prescription::PrescriptionVn::demo()), page, app);
    replace_body(dom).await;
}

#[wasm_bindgen_test]
async fn test_prescription_screen_page_visit_telemed() {
    let app = new_app();
    let page = kphis_ui_page::prescription_screen::PrescriptionScreenPage::new(String::from("0001234"));
    let dom = kphis_ui_page::prescription_screen::PrescriptionScreenPage::render_visit_telemed(&Rc::new(kphis_model::prescription::PrescriptionVn::demo()), page, app);
    replace_body(dom).await;
}

#[wasm_bindgen_test]
async fn test_prescription_screen_page_pharmacy_care() {
    let app = new_app();
    let page = kphis_ui_page::prescription_screen::PrescriptionScreenPage::new(String::from("0001234"));
    let dom = kphis_ui_page::prescription_screen::PrescriptionScreenPage::render_pharmacy_care(&Rc::new(kphis_model::prescription::PrescriptionVn::demo()), page, app);
    replace_body(dom).await;
}

#[wasm_bindgen_test]
async fn test_prescription_screen_page_modal() {
    let app = new_app();
    let page = kphis_ui_page::prescription_screen::PrescriptionScreenPage::new(String::from("0001234"));
    page.set_visit(Some(kphis_model::prescription::PrescriptionVn::demo()));
    let dom = kphis_ui_page::prescription_screen::PrescriptionScreenPage::render_modal(page, app);
    replace_body(dom).await;
}
