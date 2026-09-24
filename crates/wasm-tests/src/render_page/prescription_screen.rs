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
    let app = new_app();
    let dom = kphis_ui_page::prescription_screen::PrescriptionScreenPage::render_visit_message(&Rc::new(kphis_model::prescription::PrescriptionVn::demo()), app);
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

#[wasm_bindgen_test]
async fn test_prescription_screen_page_alert_message() {
    let app = new_app();
    let mut visit = kphis_model::prescription::PrescriptionVn::demo();
    let mut lab = kphis_model::prescription::Lab::demo();
    lab.lab_order_result = Some(String::from("15.5"));
    visit.latest_egfr = Some(lab.clone());
    visit.latest_crcl = Some(lab.clone());
    let mut med1 = kphis_model::prescription::Medicine::demo();
    med1.icode = Some(String::from("1000110"));
    med1.name_drugitems = Some(String::from("IBUPROFEN 400 mg tab"));
    let mut med2 = kphis_model::prescription::Medicine::demo();
    med2.icode = Some(String::from("1000152"));
    med2.name_drugitems = Some(String::from("DICLOFENAC 25 mg tab"));
    let mut med3 = kphis_model::prescription::Medicine::demo();
    med3.icode = Some(String::from("1000184"));
    med3.name_drugitems = Some(String::from("METFORMIN 500 mg tab"));
    visit.medicines = vec![med1, med2, med3];

    let result = visit.drug_alert_messages(app.state());
    // 1. Duplicate, 2. eGFR, 3.CrCl (test with eGFR demo)
    let expected = "มีการสั่งใช้ยากลุ่ม NSAIDs ซ้ำซ้อน : IBUPROFEN 400 mg tab, DICLOFENAC 25 mg tab, ควรหลีกเลี่ยงการใช้ยา Metformin (eGFR<30) eGFR: 15.5 mL/min (METFORMIN 500 mg tab), ควรหลีกเลี่ยงการใช้ยา Metformin (eGFR<30) eGFR: 15.5 mL/min (METFORMIN 500 mg tab)";
    assert_eq!(result.join(", "), String::from(expected));
}
