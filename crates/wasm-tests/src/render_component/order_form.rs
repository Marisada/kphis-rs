use futures_signals::{signal::Mutable, signal_vec::MutableVec};
use std::rc::Rc;
use wasm_bindgen_test::wasm_bindgen_test;

use crate::{new_app, replace_body};

#[wasm_bindgen_test]
async fn test_continuous_cpn_doctor() {
    let app = new_app();
    let cpn = kphis_ui_component::order_form::continuous::ContinuousForm::new(
        Some(Rc::new(kphis_model::order::Order::demo())),
        Mutable::new(Some(Rc::new(kphis_model::patient_info::PatientInfo::demo()))),
        Some(1),
        String::from("user"),
        Mutable::new(String::from("doctor")),
        MutableVec::new_with_values(vec![Rc::new(kphis_ui_component::order::OrderItemMutable::default())]),
    );
    let dom = kphis_ui_component::order_form::continuous::ContinuousForm::render(cpn, Mutable::new(true), Mutable::new(None), Mutable::new(false), app);
    replace_body(dom).await;
}
#[wasm_bindgen_test]
async fn test_continuous_cpn_nurse() {
    let app = new_app();
    let cpn = kphis_ui_component::order_form::continuous::ContinuousForm::new(
        Some(Rc::new(kphis_model::order::Order::demo())),
        Mutable::new(Some(Rc::new(kphis_model::patient_info::PatientInfo::demo()))),
        Some(1),
        String::from("user"),
        Mutable::new(String::from("nurse")),
        MutableVec::new_with_values(vec![Rc::new(kphis_ui_component::order::OrderItemMutable::default())]),
    );
    let dom = kphis_ui_component::order_form::continuous::ContinuousForm::render(cpn, Mutable::new(true), Mutable::new(None), Mutable::new(false), app);
    replace_body(dom).await;
}
#[wasm_bindgen_test]
async fn test_continuous_cpn_pharmacist() {
    let app = new_app();
    let cpn = kphis_ui_component::order_form::continuous::ContinuousForm::new(
        Some(Rc::new(kphis_model::order::Order::demo())),
        Mutable::new(Some(Rc::new(kphis_model::patient_info::PatientInfo::demo()))),
        Some(1),
        String::from("user"),
        Mutable::new(String::from("pharmacist")),
        MutableVec::new_with_values(vec![Rc::new(kphis_ui_component::order::OrderItemMutable::default())]),
    );
    let dom = kphis_ui_component::order_form::continuous::ContinuousForm::render(cpn, Mutable::new(true), Mutable::new(None), Mutable::new(false), app);
    replace_body(dom).await;
}
#[wasm_bindgen_test]
async fn test_continuous_cpn_other() {
    let app = new_app();
    let cpn = kphis_ui_component::order_form::continuous::ContinuousForm::new(
        Some(Rc::new(kphis_model::order::Order::demo())),
        Mutable::new(Some(Rc::new(kphis_model::patient_info::PatientInfo::demo()))),
        Some(1),
        String::from("user"),
        Mutable::new(String::from("other")),
        MutableVec::new_with_values(vec![Rc::new(kphis_ui_component::order::OrderItemMutable::default())]),
    );
    let dom = kphis_ui_component::order_form::continuous::ContinuousForm::render(cpn, Mutable::new(true), Mutable::new(None), Mutable::new(false), app);
    replace_body(dom).await;
}

#[wasm_bindgen_test]
async fn test_oneday_cpn_doctor() {
    let app = new_app();
    let cpn = kphis_ui_component::order_form::oneday::OneDayForm::new(
        Some(Rc::new(kphis_model::order::Order::demo())),
        Mutable::new(Some(Rc::new(kphis_model::patient_info::PatientInfo::demo()))),
        Some(1),
        String::from("user"),
        Mutable::new(String::from("doctor")),
        MutableVec::new_with_values(vec![Rc::new(kphis_ui_component::order::OrderItemMutable::default())]),
    );
    let dom = kphis_ui_component::order_form::oneday::OneDayForm::render(cpn, Mutable::new(true), Mutable::new(None), Mutable::new(false), app);
    replace_body(dom).await;
}
#[wasm_bindgen_test]
async fn test_oneday_cpn_nurse() {
    let app = new_app();
    let cpn = kphis_ui_component::order_form::oneday::OneDayForm::new(
        Some(Rc::new(kphis_model::order::Order::demo())),
        Mutable::new(Some(Rc::new(kphis_model::patient_info::PatientInfo::demo()))),
        Some(1),
        String::from("user"),
        Mutable::new(String::from("nurse")),
        MutableVec::new_with_values(vec![Rc::new(kphis_ui_component::order::OrderItemMutable::default())]),
    );
    let dom = kphis_ui_component::order_form::oneday::OneDayForm::render(cpn, Mutable::new(true), Mutable::new(None), Mutable::new(false), app);
    replace_body(dom).await;
}
#[wasm_bindgen_test]
async fn test_oneday_cpn_pharmacist() {
    let app = new_app();
    let cpn = kphis_ui_component::order_form::oneday::OneDayForm::new(
        Some(Rc::new(kphis_model::order::Order::demo())),
        Mutable::new(Some(Rc::new(kphis_model::patient_info::PatientInfo::demo()))),
        Some(1),
        String::from("user"),
        Mutable::new(String::from("pharmacist")),
        MutableVec::new_with_values(vec![Rc::new(kphis_ui_component::order::OrderItemMutable::default())]),
    );
    let dom = kphis_ui_component::order_form::oneday::OneDayForm::render(cpn, Mutable::new(true), Mutable::new(None), Mutable::new(false), app);
    replace_body(dom).await;
}
#[wasm_bindgen_test]
async fn test_oneday_cpn_other() {
    let app = new_app();
    let cpn = kphis_ui_component::order_form::oneday::OneDayForm::new(
        Some(Rc::new(kphis_model::order::Order::demo())),
        Mutable::new(Some(Rc::new(kphis_model::patient_info::PatientInfo::demo()))),
        Some(1),
        String::from("user"),
        Mutable::new(String::from("other")),
        MutableVec::new_with_values(vec![Rc::new(kphis_ui_component::order::OrderItemMutable::default())]),
    );
    let dom = kphis_ui_component::order_form::oneday::OneDayForm::render(cpn, Mutable::new(true), Mutable::new(None), Mutable::new(false), app);
    replace_body(dom).await;
}

#[wasm_bindgen_test]
async fn test_progress_note_cpn_doctor() {
    let app = new_app();
    let cpn = kphis_ui_component::order_form::progress_note::ProgressNoteForm::new(
        false,
        Some(Rc::new(kphis_model::progress_note::ProgressNote::demo())),
        Mutable::new(Some(Rc::new(kphis_model::order::OrderDate::demo()))),
        Mutable::new(String::from("doctor")),
        Mutable::new(Some(Rc::new(kphis_model::patient_info::PatientInfo::demo()))),
        Some(1),
        String::from("user"),
    );
    let dom = kphis_ui_component::order_form::progress_note::ProgressNoteForm::render(cpn, Mutable::new(true), Mutable::new(true), Mutable::new(None), Mutable::new(false), app);
    replace_body(dom).await;
}
#[wasm_bindgen_test]
async fn test_progress_note_cpn_nurse() {
    let app = new_app();
    let cpn = kphis_ui_component::order_form::progress_note::ProgressNoteForm::new(
        false,
        Some(Rc::new(kphis_model::progress_note::ProgressNote::demo())),
        Mutable::new(Some(Rc::new(kphis_model::order::OrderDate::demo()))),
        Mutable::new(String::from("nurse")),
        Mutable::new(Some(Rc::new(kphis_model::patient_info::PatientInfo::demo()))),
        Some(1),
        String::from("user"),
    );
    let dom = kphis_ui_component::order_form::progress_note::ProgressNoteForm::render(cpn, Mutable::new(true), Mutable::new(true), Mutable::new(None), Mutable::new(false), app);
    replace_body(dom).await;
}
#[wasm_bindgen_test]
async fn test_progress_note_cpn_pharmacist() {
    let app = new_app();
    let cpn = kphis_ui_component::order_form::progress_note::ProgressNoteForm::new(
        false,
        Some(Rc::new(kphis_model::progress_note::ProgressNote::demo())),
        Mutable::new(Some(Rc::new(kphis_model::order::OrderDate::demo()))),
        Mutable::new(String::from("pharmacist")),
        Mutable::new(Some(Rc::new(kphis_model::patient_info::PatientInfo::demo()))),
        Some(1),
        String::from("user"),
    );
    let dom = kphis_ui_component::order_form::progress_note::ProgressNoteForm::render(cpn, Mutable::new(true), Mutable::new(true), Mutable::new(None), Mutable::new(false), app);
    replace_body(dom).await;
}
#[wasm_bindgen_test]
async fn test_progress_note_cpn_other() {
    let app = new_app();
    let cpn = kphis_ui_component::order_form::progress_note::ProgressNoteForm::new(
        false,
        Some(Rc::new(kphis_model::progress_note::ProgressNote::demo())),
        Mutable::new(Some(Rc::new(kphis_model::order::OrderDate::demo()))),
        Mutable::new(String::from("other")),
        Mutable::new(Some(Rc::new(kphis_model::patient_info::PatientInfo::demo()))),
        Some(1),
        String::from("user"),
    );
    let dom = kphis_ui_component::order_form::progress_note::ProgressNoteForm::render(cpn, Mutable::new(true), Mutable::new(true), Mutable::new(None), Mutable::new(false), app);
    replace_body(dom).await;
}
#[wasm_bindgen_test]
async fn test_progress_note_cpn_doctor_auditor() {
    let app = new_app();
    let cpn = kphis_ui_component::order_form::progress_note::ProgressNoteForm::new(
        true,
        Some(Rc::new(kphis_model::progress_note::ProgressNote::demo())),
        Mutable::new(Some(Rc::new(kphis_model::order::OrderDate::demo()))),
        Mutable::new(String::from("doctor")),
        Mutable::new(Some(Rc::new(kphis_model::patient_info::PatientInfo::demo()))),
        Some(1),
        String::from("user"),
    );
    let dom = kphis_ui_component::order_form::progress_note::ProgressNoteForm::render(cpn, Mutable::new(true), Mutable::new(true), Mutable::new(None), Mutable::new(false), app);
    replace_body(dom).await;
}
#[wasm_bindgen_test]
async fn test_progress_note_cpn_nurse_auditor() {
    let app = new_app();
    let cpn = kphis_ui_component::order_form::progress_note::ProgressNoteForm::new(
        true,
        Some(Rc::new(kphis_model::progress_note::ProgressNote::demo())),
        Mutable::new(Some(Rc::new(kphis_model::order::OrderDate::demo()))),
        Mutable::new(String::from("nurse")),
        Mutable::new(Some(Rc::new(kphis_model::patient_info::PatientInfo::demo()))),
        Some(1),
        String::from("user"),
    );
    let dom = kphis_ui_component::order_form::progress_note::ProgressNoteForm::render(cpn, Mutable::new(true), Mutable::new(true), Mutable::new(None), Mutable::new(false), app);
    replace_body(dom).await;
}
#[wasm_bindgen_test]
async fn test_progress_note_cpn_pharmacist_auditor() {
    let app = new_app();
    let cpn = kphis_ui_component::order_form::progress_note::ProgressNoteForm::new(
        true,
        Some(Rc::new(kphis_model::progress_note::ProgressNote::demo())),
        Mutable::new(Some(Rc::new(kphis_model::order::OrderDate::demo()))),
        Mutable::new(String::from("pharmacist")),
        Mutable::new(Some(Rc::new(kphis_model::patient_info::PatientInfo::demo()))),
        Some(1),
        String::from("user"),
    );
    let dom = kphis_ui_component::order_form::progress_note::ProgressNoteForm::render(cpn, Mutable::new(true), Mutable::new(true), Mutable::new(None), Mutable::new(false), app);
    replace_body(dom).await;
}
#[wasm_bindgen_test]
async fn test_progress_note_cpn_other_auditor() {
    let app = new_app();
    let cpn = kphis_ui_component::order_form::progress_note::ProgressNoteForm::new(
        true,
        Some(Rc::new(kphis_model::progress_note::ProgressNote::demo())),
        Mutable::new(Some(Rc::new(kphis_model::order::OrderDate::demo()))),
        Mutable::new(String::from("other")),
        Mutable::new(Some(Rc::new(kphis_model::patient_info::PatientInfo::demo()))),
        Some(1),
        String::from("user"),
    );
    let dom = kphis_ui_component::order_form::progress_note::ProgressNoteForm::render(cpn, Mutable::new(true), Mutable::new(true), Mutable::new(None), Mutable::new(false), app);
    replace_body(dom).await;
}
