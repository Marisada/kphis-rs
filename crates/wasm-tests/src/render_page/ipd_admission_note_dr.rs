use dominator::html;
use std::rc::Rc;
use wasm_bindgen_test::wasm_bindgen_test;

use crate::{new_app, replace_body};

#[wasm_bindgen_test]
async fn test_ipd_admission_note_dr_page() {
    let app = new_app();
    let page = kphis_ui_page::ipd_admission_note_dr::IpdAdmissionNoteDrPage::new(String::from("660001234"));
    let dom = kphis_ui_page::ipd_admission_note_dr::IpdAdmissionNoteDrPage::render(page, app);
    replace_body(dom).await;
}

#[wasm_bindgen_test]
async fn test_ipd_admission_note_dr_page_form() {
    let app = new_app();
    let page = kphis_ui_page::ipd_admission_note_dr::IpdAdmissionNoteDrPage::new(String::from("660001234"));
    let dom = kphis_ui_page::ipd_admission_note_dr::IpdAdmissionNoteDrPage::render_form(page, app);
    replace_body(dom).await;
}

#[wasm_bindgen_test]
async fn test_ipd_admission_note_dr_page_ph_mother() {
    let page = kphis_ui_page::ipd_admission_note_dr::IpdAdmissionNoteDrPage::new(String::from("660001234"));
    let doms = kphis_ui_page::ipd_admission_note_dr::IpdAdmissionNoteDrPage::render_ph_mother(page);
    replace_body(html!("div", {.children(doms)})).await;
}

#[wasm_bindgen_test]
async fn test_ipd_admission_note_dr_page_doctor() {
    let dom = kphis_ui_page::ipd_admission_note_dr::render_doctor(Rc::new(kphis_model::ipd::admission_note_dr::AdmissionNoteDoctor::demo()));
    replace_body(dom).await;
}

#[wasm_bindgen_test]
async fn test_ipd_admission_note_dr_page_disease_detail() {
    let app = new_app();
    let page = kphis_ui_page::ipd_admission_note_dr::IpdAdmissionNoteDrPage::new(String::from("660001234"));
    let dom = kphis_ui_page::ipd_admission_note_dr::DiseaseDetail::render(kphis_ui_page::ipd_admission_note_dr::DiseaseDetail::new(), page, app);
    replace_body(dom).await;
}

#[wasm_bindgen_test]
async fn test_ipd_admission_note_dr_page_drug_allergy() {
    let page = kphis_ui_page::ipd_admission_note_dr::IpdAdmissionNoteDrPage::new(String::from("660001234"));
    let dom = kphis_ui_page::ipd_admission_note_dr::DrugAllergy::render(kphis_ui_page::ipd_admission_note_dr::DrugAllergy::new(), page);
    replace_body(dom).await;
}

#[wasm_bindgen_test]
async fn test_ipd_admission_note_dr_page_food_allergy() {
    let page = kphis_ui_page::ipd_admission_note_dr::IpdAdmissionNoteDrPage::new(String::from("660001234"));
    let dom = kphis_ui_page::ipd_admission_note_dr::FoodAllergy::render(kphis_ui_page::ipd_admission_note_dr::FoodAllergy::new(), page);
    replace_body(dom).await;
}

#[wasm_bindgen_test]
async fn test_ipd_admission_note_dr_page_etc_allergy() {
    let page = kphis_ui_page::ipd_admission_note_dr::IpdAdmissionNoteDrPage::new(String::from("660001234"));
    let dom = kphis_ui_page::ipd_admission_note_dr::EtcAllergy::render(kphis_ui_page::ipd_admission_note_dr::EtcAllergy::new(), page);
    replace_body(dom).await;
}

#[wasm_bindgen_test]
async fn test_ipd_admission_note_dr_page_family_medical() {
    let page = kphis_ui_page::ipd_admission_note_dr::IpdAdmissionNoteDrPage::new(String::from("660001234"));
    let dom = kphis_ui_page::ipd_admission_note_dr::FamilyMedical::render(kphis_ui_page::ipd_admission_note_dr::FamilyMedical::new(), page);
    replace_body(dom).await;
}

#[wasm_bindgen_test]
async fn test_ipd_admission_note_dr_page_addcit_assist() {
    let app = new_app();
    let page = kphis_ui_page::ipd_admission_note_dr::IpdAdmissionNoteDrPage::new(String::from("660001234"));
    let dom = kphis_ui_page::ipd_admission_note_dr::AddictAssist::render(kphis_ui_page::ipd_admission_note_dr::AddictAssist::new(), page, app);
    replace_body(dom).await;
}
