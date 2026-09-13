mod report;
mod scoring;

use futures_signals::signal::Mutable;
use std::rc::Rc;
use time::macros::date;
use wasm_bindgen_test::wasm_bindgen_test;

use crate::{new_app, replace_body};

#[wasm_bindgen_test]
async fn test_aside_resizer_cpn() {
    let app = new_app();
    let modal = kphis_ui_component::modal::consult_form::ConsultForm::new(Mutable::new(Some(Rc::new(kphis_model::patient_info::PatientInfo::demo()))), Some(1), kphis_ui_component::modal::consult_form::ConsultFormMode::View);
    let dom = kphis_ui_component::modal::consult_form::ConsultForm::render_modal(modal.clone(), Mutable::new(Some(modal)), Mutable::new(false), app);
    replace_body(dom).await;
}

#[wasm_bindgen_test]
async fn test_drug_details_cpn() {
    let app = new_app();
    let modal = kphis_ui_component::modal::drug_details::DrugDetailModal::new(false);
    let dom = kphis_ui_component::modal::drug_details::DrugDetailModal::render_modal(modal.clone(), Mutable::new(Some(modal)), None, app);
    replace_body(dom).await;
}
#[wasm_bindgen_test]
async fn test_drug_details_cpn_form() {
    let app = new_app();
    let modal = kphis_ui_component::modal::drug_details::DrugDetailModal::new(true);
    let dom = kphis_ui_component::modal::drug_details::DrugDetailModal::render_modal(modal.clone(), Mutable::new(Some(modal)), None, app);
    replace_body(dom).await;
}
#[wasm_bindgen_test]
async fn test_drug_details_cpn_search() {
    let app = new_app();
    let modal = kphis_ui_component::modal::drug_details::DrugDetailModal::new(true);
    let dom = kphis_ui_component::modal::drug_details::DrugDetailModal::render_med_searchbox(modal.clone(), app);
    replace_body(dom).await;
}

#[wasm_bindgen_test]
async fn test_drug_duplication_cpn() {
    let app = new_app();
    let modal = kphis_ui_component::modal::drug_duplication::DrugDuplication::new("PARACETAMOL 500 mg. เม็ด", &[kphis_model::search::searchbox::DrugDuplicateCheck::demo()]);
    let dom = kphis_ui_component::modal::drug_duplication::DrugDuplication::render_modal(&modal.clone(), Mutable::new(Some(modal)), Mutable::new(false), app);
    replace_body(dom).await;
}

#[wasm_bindgen_test]
async fn test_drug_interaction_cpn() {
    let app = new_app();
    let modal = kphis_ui_component::modal::drug_interaction::DrugInteraction::new(&[kphis_model::search::searchbox::DrugInteractionCheck::demo()]);
    let dom = kphis_ui_component::modal::drug_interaction::DrugInteraction::render_modal(&modal.clone(), Mutable::new(Some(modal)), Mutable::new(false), app);
    replace_body(dom).await;
}

#[wasm_bindgen_test]
async fn test_drug_notify_cpn() {
    let app = new_app();
    let modal = kphis_ui_component::modal::drug_notify::DrugNotify::new("PARACETAMOL 500 mg. เม็ด", "Alert!!");
    let dom = kphis_ui_component::modal::drug_notify::DrugNotify::render_modal(&modal.clone(), Mutable::new(Some(modal)), app);
    replace_body(dom).await;
}

#[wasm_bindgen_test]
async fn test_index_note_form_cpn() {
    let app = new_app();
    let modal = Rc::new(kphis_ui_component::modal::index_note_form::IndexNoteForm::default());
    let dom = kphis_ui_component::modal::index_note_form::IndexNoteForm::render_modal(modal.clone(), Mutable::new(Some(modal)), None, app);
    replace_body(dom).await;
}

// not use view_by in rendering
#[wasm_bindgen_test]
async fn test_index_plan_action_form_cpn_oneday_plan() {
    let app = new_app();
    let modal = kphis_ui_component::modal::index_plan_action_form::IndexPlanActionForm::new(1, Some(1), Some(1), Mutable::new(Some(Rc::new(kphis_model::patient_info::PatientInfo::demo()))), kphis_ui_component::modal::index_plan_action_form::OrderType::OneDay, kphis_ui_component::modal::index_plan_action_form::FormType::Plan, Mutable::new(String::from("nurse")));
    let dom = kphis_ui_component::modal::index_plan_action_form::IndexPlanActionForm::render_modal(modal, Mutable::new(None), None, app);
    replace_body(dom).await;
}
#[wasm_bindgen_test]
async fn test_index_plan_action_form_cpn_oneday_action() {
    let app = new_app();
    let modal = kphis_ui_component::modal::index_plan_action_form::IndexPlanActionForm::new(1, Some(1), Some(1), Mutable::new(Some(Rc::new(kphis_model::patient_info::PatientInfo::demo()))), kphis_ui_component::modal::index_plan_action_form::OrderType::OneDay, kphis_ui_component::modal::index_plan_action_form::FormType::Action, Mutable::new(String::from("nurse")));
    let dom = kphis_ui_component::modal::index_plan_action_form::IndexPlanActionForm::render_modal(modal, Mutable::new(None), None, app);
    replace_body(dom).await;
}
#[wasm_bindgen_test]
async fn test_index_plan_action_form_cpn_oneday_monitor() {
    let app = new_app();
    let modal = kphis_ui_component::modal::index_plan_action_form::IndexPlanActionForm::new(1, Some(1), Some(1), Mutable::new(Some(Rc::new(kphis_model::patient_info::PatientInfo::demo()))), kphis_ui_component::modal::index_plan_action_form::OrderType::OneDay, kphis_ui_component::modal::index_plan_action_form::FormType::Monitor, Mutable::new(String::from("nurse")));
    let dom = kphis_ui_component::modal::index_plan_action_form::IndexPlanActionForm::render_modal(modal, Mutable::new(None), None, app);
    replace_body(dom).await;
}
#[wasm_bindgen_test]
async fn test_index_plan_action_form_cpn_cont_plan() {
    let app = new_app();
    let modal = kphis_ui_component::modal::index_plan_action_form::IndexPlanActionForm::new(1, Some(1), Some(1), Mutable::new(Some(Rc::new(kphis_model::patient_info::PatientInfo::demo()))), kphis_ui_component::modal::index_plan_action_form::OrderType::Continuous, kphis_ui_component::modal::index_plan_action_form::FormType::Plan, Mutable::new(String::from("nurse")));
    let dom = kphis_ui_component::modal::index_plan_action_form::IndexPlanActionForm::render_modal(modal, Mutable::new(None), None, app);
    replace_body(dom).await;
}
#[wasm_bindgen_test]
async fn test_index_plan_action_form_cpn_cont_action() {
    let app = new_app();
    let modal = kphis_ui_component::modal::index_plan_action_form::IndexPlanActionForm::new(1, Some(1), Some(1), Mutable::new(Some(Rc::new(kphis_model::patient_info::PatientInfo::demo()))), kphis_ui_component::modal::index_plan_action_form::OrderType::Continuous, kphis_ui_component::modal::index_plan_action_form::FormType::Action, Mutable::new(String::from("nurse")));
    let dom = kphis_ui_component::modal::index_plan_action_form::IndexPlanActionForm::render_modal(modal, Mutable::new(None), None, app);
    replace_body(dom).await;
}
#[wasm_bindgen_test]
async fn test_index_plan_action_form_cpn_cont_monitor() {
    let app = new_app();
    let modal = kphis_ui_component::modal::index_plan_action_form::IndexPlanActionForm::new(1, Some(1), Some(1), Mutable::new(Some(Rc::new(kphis_model::patient_info::PatientInfo::demo()))), kphis_ui_component::modal::index_plan_action_form::OrderType::Continuous, kphis_ui_component::modal::index_plan_action_form::FormType::Monitor, Mutable::new(String::from("nurse")));
    let dom = kphis_ui_component::modal::index_plan_action_form::IndexPlanActionForm::render_modal(modal, Mutable::new(None), None, app);
    replace_body(dom).await;
}
#[wasm_bindgen_test]
async fn test_index_plan_action_form_cpn_plan_content() {
    let app = new_app();
    let modal = kphis_ui_component::modal::index_plan_action_form::IndexPlanActionForm::new(1, Some(1), Some(1), Mutable::new(Some(Rc::new(kphis_model::patient_info::PatientInfo::demo()))), kphis_ui_component::modal::index_plan_action_form::OrderType::Continuous, kphis_ui_component::modal::index_plan_action_form::FormType::Monitor, Mutable::new(String::from("nurse")));
    let dom = kphis_ui_component::modal::index_plan_action_form::IndexPlanActionForm::render_plan_content(modal, app);
    replace_body(dom).await;
}
#[wasm_bindgen_test]
async fn test_index_plan_action_form_cpn_monitor_content() {
    let app = new_app();
    let modal = kphis_ui_component::modal::index_plan_action_form::IndexPlanActionForm::new(1, Some(1), Some(1), Mutable::new(Some(Rc::new(kphis_model::patient_info::PatientInfo::demo()))), kphis_ui_component::modal::index_plan_action_form::OrderType::Continuous, kphis_ui_component::modal::index_plan_action_form::FormType::Monitor, Mutable::new(String::from("nurse")));
    let dom = kphis_ui_component::modal::index_plan_action_form::IndexPlanActionForm::render_monitor_content(modal, app);
    replace_body(dom).await;
}
#[wasm_bindgen_test]
async fn test_index_plan_action_form_cpn_action_content() {
    let app = new_app();
    let modal = kphis_ui_component::modal::index_plan_action_form::IndexPlanActionForm::new(1, Some(1), Some(1), Mutable::new(Some(Rc::new(kphis_model::patient_info::PatientInfo::demo()))), kphis_ui_component::modal::index_plan_action_form::OrderType::Continuous, kphis_ui_component::modal::index_plan_action_form::FormType::Monitor, Mutable::new(String::from("nurse")));
    let dom = kphis_ui_component::modal::index_plan_action_form::IndexPlanActionForm::render_action_content(modal, app);
    replace_body(dom).await;
}
#[wasm_bindgen_test]
async fn test_index_plan_action_form_cpn_plan_sch_stat() {
    let app = new_app();
    let modal = kphis_ui_component::modal::index_plan_action_form::IndexPlanActionForm::new(1, Some(1), Some(1), Mutable::new(Some(Rc::new(kphis_model::patient_info::PatientInfo::demo()))), kphis_ui_component::modal::index_plan_action_form::OrderType::Continuous, kphis_ui_component::modal::index_plan_action_form::FormType::Monitor, Mutable::new(String::from("nurse")));
    let dom = kphis_ui_component::modal::index_plan_action_form::IndexPlanActionForm::render_plan_sch_stat(vec![Rc::new(kphis_model::index_plan::IndexPlan::demo())], modal, app);
    replace_body(dom).await;
}
#[wasm_bindgen_test]
async fn test_index_plan_action_form_cpn_plan_sch_date() {
    let app = new_app();
    let modal = kphis_ui_component::modal::index_plan_action_form::IndexPlanActionForm::new(1, Some(1), Some(1), Mutable::new(Some(Rc::new(kphis_model::patient_info::PatientInfo::demo()))), kphis_ui_component::modal::index_plan_action_form::OrderType::Continuous, kphis_ui_component::modal::index_plan_action_form::FormType::Monitor, Mutable::new(String::from("nurse")));
    let dom = kphis_ui_component::modal::index_plan_action_form::IndexPlanActionForm::render_plan_sch_date(vec![Rc::new(kphis_model::index_plan::IndexPlan::demo())], modal, app);
    replace_body(dom).await;
}
#[wasm_bindgen_test]
async fn test_index_plan_action_form_cpn_plan_sch_time() {
    let app = new_app();
    let modal = kphis_ui_component::modal::index_plan_action_form::IndexPlanActionForm::new(1, Some(1), Some(1), Mutable::new(Some(Rc::new(kphis_model::patient_info::PatientInfo::demo()))), kphis_ui_component::modal::index_plan_action_form::OrderType::Continuous, kphis_ui_component::modal::index_plan_action_form::FormType::Monitor, Mutable::new(String::from("nurse")));
    let dom = kphis_ui_component::modal::index_plan_action_form::IndexPlanActionForm::render_plan_sch_time(vec![Rc::new(kphis_model::index_plan::IndexPlan::demo())], modal, app);
    replace_body(dom).await;
}
#[wasm_bindgen_test]
async fn test_index_plan_action_form_cpn_plan_sch_time_multi() {
    let app = new_app();
    let modal = kphis_ui_component::modal::index_plan_action_form::IndexPlanActionForm::new(1, Some(1), Some(1), Mutable::new(Some(Rc::new(kphis_model::patient_info::PatientInfo::demo()))), kphis_ui_component::modal::index_plan_action_form::OrderType::Continuous, kphis_ui_component::modal::index_plan_action_form::FormType::Monitor, Mutable::new(String::from("nurse")));
    let dom = kphis_ui_component::modal::index_plan_action_form::IndexPlanActionForm::render_plan_sch_time_multiple(vec![Rc::new(kphis_model::index_plan::IndexPlan::demo())], modal, app);
    replace_body(dom).await;
}
#[wasm_bindgen_test]
async fn test_index_plan_action_form_cpn_plan_sch_time_single() {
    let app = new_app();
    let modal = kphis_ui_component::modal::index_plan_action_form::IndexPlanActionForm::new(1, Some(1), Some(1), Mutable::new(Some(Rc::new(kphis_model::patient_info::PatientInfo::demo()))), kphis_ui_component::modal::index_plan_action_form::OrderType::Continuous, kphis_ui_component::modal::index_plan_action_form::FormType::Monitor, Mutable::new(String::from("nurse")));
    let dom = kphis_ui_component::modal::index_plan_action_form::IndexPlanActionForm::render_plan_sch_time_single(vec![Rc::new(kphis_model::index_plan::IndexPlan::demo())], modal, app);
    replace_body(dom).await;
}
#[wasm_bindgen_test]
async fn test_index_plan_action_form_cpn_action_sch_stat() {
    let modal = kphis_ui_component::modal::index_plan_action_form::IndexPlanActionForm::new(1, Some(1), Some(1), Mutable::new(Some(Rc::new(kphis_model::patient_info::PatientInfo::demo()))), kphis_ui_component::modal::index_plan_action_form::OrderType::Continuous, kphis_ui_component::modal::index_plan_action_form::FormType::Monitor, Mutable::new(String::from("nurse")));
    let dom = kphis_ui_component::modal::index_plan_action_form::IndexPlanActionForm::render_action_sch_stat(vec![Rc::new(kphis_model::index_plan::IndexPlan::demo())], modal);
    replace_body(dom).await;
}
#[wasm_bindgen_test]
async fn test_index_plan_action_form_cpn_action_sch_date() {
    let modal = kphis_ui_component::modal::index_plan_action_form::IndexPlanActionForm::new(1, Some(1), Some(1), Mutable::new(Some(Rc::new(kphis_model::patient_info::PatientInfo::demo()))), kphis_ui_component::modal::index_plan_action_form::OrderType::Continuous, kphis_ui_component::modal::index_plan_action_form::FormType::Monitor, Mutable::new(String::from("nurse")));
    let dom = kphis_ui_component::modal::index_plan_action_form::IndexPlanActionForm::render_action_sch_date(vec![Rc::new(kphis_model::index_plan::IndexPlan::demo())], modal);
    replace_body(dom).await;
}
#[wasm_bindgen_test]
async fn test_index_plan_action_form_cpn_action_sch_time() {
    let modal = kphis_ui_component::modal::index_plan_action_form::IndexPlanActionForm::new(1, Some(1), Some(1), Mutable::new(Some(Rc::new(kphis_model::patient_info::PatientInfo::demo()))), kphis_ui_component::modal::index_plan_action_form::OrderType::Continuous, kphis_ui_component::modal::index_plan_action_form::FormType::Monitor, Mutable::new(String::from("nurse")));
    let dom = kphis_ui_component::modal::index_plan_action_form::IndexPlanActionForm::render_action_sch_time(vec![Rc::new(kphis_model::index_plan::IndexPlan::demo())], modal);
    replace_body(dom).await;
}
#[wasm_bindgen_test]
async fn test_index_plan_action_form_cpn_plan_input() {
    let app = new_app();
    let modal = kphis_ui_component::modal::index_plan_action_form::IndexPlanActionForm::new(1, Some(1), Some(1), Mutable::new(Some(Rc::new(kphis_model::patient_info::PatientInfo::demo()))), kphis_ui_component::modal::index_plan_action_form::OrderType::Continuous, kphis_ui_component::modal::index_plan_action_form::FormType::Monitor, Mutable::new(String::from("nurse")));
    let dom = kphis_ui_component::modal::index_plan_action_form::IndexPlanActionForm::render_plan_inputs(modal, app);
    replace_body(dom).await;
}
#[wasm_bindgen_test]
async fn test_index_plan_action_form_cpn_action_input() {
    let app = new_app();
    let modal = kphis_ui_component::modal::index_plan_action_form::IndexPlanActionForm::new(1, Some(1), Some(1), Mutable::new(Some(Rc::new(kphis_model::patient_info::PatientInfo::demo()))), kphis_ui_component::modal::index_plan_action_form::OrderType::Continuous, kphis_ui_component::modal::index_plan_action_form::FormType::Monitor, Mutable::new(String::from("nurse")));
    let dom = kphis_ui_component::modal::index_plan_action_form::IndexPlanActionForm::render_action_inputs(modal, app);
    replace_body(dom).await;
}
#[wasm_bindgen_test]
async fn test_index_plan_action_form_cpn_monitor_input() {
    let app = new_app();
    let modal = kphis_ui_component::modal::index_plan_action_form::IndexPlanActionForm::new(1, Some(1), Some(1), Mutable::new(Some(Rc::new(kphis_model::patient_info::PatientInfo::demo()))), kphis_ui_component::modal::index_plan_action_form::OrderType::Continuous, kphis_ui_component::modal::index_plan_action_form::FormType::Monitor, Mutable::new(String::from("nurse")));
    let dom = kphis_ui_component::modal::index_plan_action_form::IndexPlanActionForm::render_monitor_inputs(modal, app);
    replace_body(dom).await;
}
#[wasm_bindgen_test]
async fn test_index_plan_action_form_cpn_plan() {
    let modal = kphis_ui_component::modal::index_plan_action_form::IndexPlanActionForm::new(1, Some(1), Some(1), Mutable::new(Some(Rc::new(kphis_model::patient_info::PatientInfo::demo()))), kphis_ui_component::modal::index_plan_action_form::OrderType::Continuous, kphis_ui_component::modal::index_plan_action_form::FormType::Monitor, Mutable::new(String::from("nurse")));
    let dom = kphis_ui_component::modal::index_plan_action_form::IndexPlanActionForm::plan_dom(Rc::new(kphis_model::index_plan::IndexPlan::demo()), modal, Some(String::from("lebel")));
    replace_body(dom).await;
}
#[wasm_bindgen_test]
async fn test_index_plan_action_form_cpn_plan_new() {
    let modal = kphis_ui_component::modal::index_plan_action_form::IndexPlanActionForm::new(1, Some(1), Some(1), Mutable::new(Some(Rc::new(kphis_model::patient_info::PatientInfo::demo()))), kphis_ui_component::modal::index_plan_action_form::OrderType::Continuous, kphis_ui_component::modal::index_plan_action_form::FormType::Monitor, Mutable::new(String::from("nurse")));
    let dom = kphis_ui_component::modal::index_plan_action_form::IndexPlanActionForm::plan_dom_new(modal, Some(date!(2023-12-31)));
    replace_body(dom).await;
}
#[wasm_bindgen_test]
async fn test_index_plan_action_form_cpn_plan_new_now() {
    let app = new_app();
    let modal = kphis_ui_component::modal::index_plan_action_form::IndexPlanActionForm::new(1, Some(1), Some(1), Mutable::new(Some(Rc::new(kphis_model::patient_info::PatientInfo::demo()))), kphis_ui_component::modal::index_plan_action_form::OrderType::Continuous, kphis_ui_component::modal::index_plan_action_form::FormType::Monitor, Mutable::new(String::from("nurse")));
    let dom = kphis_ui_component::modal::index_plan_action_form::IndexPlanActionForm::plan_dom_new_now(modal, app);
    replace_body(dom).await;
}
#[wasm_bindgen_test]
async fn test_index_plan_action_form_cpn_plan_new_action_now() {
    let app = new_app();
    let modal = kphis_ui_component::modal::index_plan_action_form::IndexPlanActionForm::new(1, Some(1), Some(1), Mutable::new(Some(Rc::new(kphis_model::patient_info::PatientInfo::demo()))), kphis_ui_component::modal::index_plan_action_form::OrderType::Continuous, kphis_ui_component::modal::index_plan_action_form::FormType::Monitor, Mutable::new(String::from("nurse")));
    let dom = kphis_ui_component::modal::index_plan_action_form::IndexPlanActionForm::plan_dom_new_and_action_now(modal, app);
    replace_body(dom).await;
}
#[wasm_bindgen_test]
async fn test_index_plan_action_form_cpn_order_item() {
    let app = new_app();
    let dom = kphis_ui_component::modal::index_plan_action_form::render_order_item(Rc::new(kphis_model::order::OrderItem::demo()), app);
    replace_body(dom).await;
}

#[wasm_bindgen_test]
async fn test_io_selector_cpn() {
    let app = new_app();
    let modal = kphis_ui_component::modal::io_selector::IoSelector::new(false, Mutable::new(Some(Rc::new(kphis_model::patient_info::PatientInfo::demo()))), Mutable::new(String::new()), Mutable::new(false));
    let dom = kphis_ui_component::modal::io_selector::IoSelector::render_modal(modal.clone(), Mutable::new(Some(modal)), app);
    replace_body(dom).await;
}
#[wasm_bindgen_test]
async fn test_io_selector_cpn_date() {
    let app = new_app();
    let modal = kphis_ui_component::modal::io_selector::IoSelector::new(true, Mutable::new(Some(Rc::new(kphis_model::patient_info::PatientInfo::demo()))), Mutable::new(String::new()), Mutable::new(false));
    let dom = kphis_ui_component::modal::io_selector::IoSelector::render_modal(modal.clone(), Mutable::new(Some(modal)), app);
    replace_body(dom).await;
}

#[wasm_bindgen_test]
async fn test_ipd_passcode_cpn() {
    let app = new_app();
    let modal = kphis_ui_component::modal::ipd_passcode::IpdPasscodeForm::new();
    let dom = kphis_ui_component::modal::ipd_passcode::IpdPasscodeForm::render_modal(modal, Mutable::new(true), app);
    replace_body(dom).await;
}
#[wasm_bindgen_test]
async fn test_ipd_passcode_cpn_using() {
    let app = new_app();
    let modal = kphis_ui_component::modal::ipd_passcode::IpdPasscodeForm::new();
    let dom = kphis_ui_component::modal::ipd_passcode::IpdPasscodeForm::render_using_passcode(Rc::new(kphis_model::ipd::passcode::ConfigIpdWardPasscode::demo()), 1, app, modal);
    replace_body(dom).await;
}

#[wasm_bindgen_test]
async fn test_lab_history_cpn() {
    let app = new_app();
    let modal = kphis_ui_component::modal::lab_history::LabHistory::new(Mutable::new(String::from("660001234")), 1, &Some(String::from("HCT")), &Some(String::from("%")), &Some(1));
    let dom = kphis_ui_component::modal::lab_history::LabHistory::render_modal(modal.clone(), Mutable::new(Some(modal)), None, app);
    replace_body(dom).await;
}

#[wasm_bindgen_test]
async fn test_lab_selector_cpn() {
    let app = new_app();
    let modal = kphis_ui_component::modal::lab_selector::LabSelector::new(false, Mutable::new(Some(Rc::new(kphis_model::patient_info::PatientInfo::demo()))), Mutable::new(String::new()), Mutable::new(false));
    let dom = kphis_ui_component::modal::lab_selector::LabSelector::render_modal(modal.clone(), Mutable::new(Some(modal)), app);
    replace_body(dom).await;
}
#[wasm_bindgen_test]
async fn test_lab_selector_cpn_datetime() {
    let app = new_app();
    let modal = kphis_ui_component::modal::lab_selector::LabSelector::new(true, Mutable::new(Some(Rc::new(kphis_model::patient_info::PatientInfo::demo()))), Mutable::new(String::new()), Mutable::new(false));
    let dom = kphis_ui_component::modal::lab_selector::LabSelector::render_modal(modal.clone(), Mutable::new(Some(modal)), app);
    replace_body(dom).await;
}

#[wasm_bindgen_test]
async fn test_lab_wbc_cpn() {
    let app = new_app();
    let modal = kphis_ui_component::modal::lab_wbc::LabWbc::new(String::from("hn"), String::from("00001234"), Mutable::new(String::new()), Mutable::new(String::new()), Mutable::new(false));
    let dom = kphis_ui_component::modal::lab_wbc::LabWbc::render_modal(modal.clone(), Mutable::new(Some(modal)), app);
    replace_body(dom).await;
}

#[wasm_bindgen_test]
async fn test_med_reconcile_remed_cpn() {
    let app = new_app();
    let modal = kphis_ui_component::modal::med_reconcile_remed::MedReconcileRemed::new(Mutable::new(Some(Rc::new(kphis_model::patient_info::PatientInfo::demo()))), Mutable::new(true), Mutable::new(false), Mutable::new(false));
    let dom = kphis_ui_component::modal::med_reconcile_remed::MedReconcileRemed::render_modal(modal, app);
    replace_body(dom).await;
}

#[wasm_bindgen_test]
async fn test_medplan_form_cpn_oneday() {
    let app = new_app();
    let order_cpn = kphis_ui_component::order::OrderCpn::default();
    let modal = kphis_ui_component::modal::medplan_form::MedPlanForm::new(Rc::new(kphis_model::order::Order::demo()), Rc::new(order_cpn), Mutable::new(String::new()), false);
    let dom = kphis_ui_component::modal::medplan_form::MedPlanForm::render_modal(modal.clone(), Mutable::new(Some(modal)), app);
    replace_body(dom).await;
}
#[wasm_bindgen_test]
async fn test_medplan_form_cpn_cont() {
    let app = new_app();
    let order_cpn = kphis_ui_component::order::OrderCpn::default();
    let modal = kphis_ui_component::modal::medplan_form::MedPlanForm::new(Rc::new(kphis_model::order::Order::demo()), Rc::new(order_cpn), Mutable::new(String::new()), true);
    let dom = kphis_ui_component::modal::medplan_form::MedPlanForm::render_modal(modal.clone(), Mutable::new(Some(modal)), app);
    replace_body(dom).await;
}
#[wasm_bindgen_test]
async fn test_medplan_form_cpn_search() {
    let app = new_app();
    let order_cpn = kphis_ui_component::order::OrderCpn::default();
    let modal = kphis_ui_component::modal::medplan_form::MedPlanForm::new(Rc::new(kphis_model::order::Order::demo()), Rc::new(order_cpn), Mutable::new(String::new()), true);
    let dom = kphis_ui_component::modal::medplan_form::MedPlanForm::render_ivfluid_searchbox(modal.clone(), app);
    replace_body(dom).await;
}

// not use view_by in rendering
#[wasm_bindgen_test]
async fn test_opd_er_order_new_cpn() {
    let app = new_app();
    let modal = kphis_ui_component::modal::opd_er_order_new::OpdErOrderNew::new();
    let dom = kphis_ui_component::modal::opd_er_order_new::OpdErOrderNew::render_modal(modal.clone(), Mutable::new(String::from("doctor")), Mutable::new(Some(modal)), Mutable::new(false), app);
    replace_body(dom).await;
}

// not use view_by in rendering
#[wasm_bindgen_test]
async fn test_pre_admit_new_cpn() {
    let app = new_app();
    let modal = kphis_ui_component::modal::pre_admit_new::PreAdmitNew::new();
    let dom = kphis_ui_component::modal::pre_admit_new::PreAdmitNew::render_modal(modal.clone(), Mutable::new(String::from("doctor")), Mutable::new(Some(modal)), Mutable::new(false), app);
    replace_body(dom).await;
}

// not use view_by in rendering
#[wasm_bindgen_test]
async fn test_pre_order_new_cpn() {
    let app = new_app();
    let modal = kphis_ui_component::modal::pre_order_new::PreOrderNew::new(Mutable::new(String::from("doctor")));
    let dom = kphis_ui_component::modal::pre_order_new::PreOrderNew::render_modal(modal.clone(), Mutable::new(Some(modal)), Mutable::new(false), app);
    replace_body(dom).await;
}

// not use to_order_type in rendering
#[wasm_bindgen_test]
async fn test_pre_order_preview_cpn() {
    let app = new_app();
    let modal = kphis_ui_component::modal::pre_order_preview::PreOrderPreview::new(1, Mutable::new(String::from("1")), kphis_ui_component::modal::pre_order_preview::ToOrderType::Order);
    let dom = kphis_ui_component::modal::pre_order_preview::PreOrderPreview::render(modal.clone(), Mutable::new(None), app, Mutable::new(None), None, None);
    replace_body(dom).await;
}

#[wasm_bindgen_test]
async fn test_pre_order_select_cpn_template() {
    let app = new_app();
    let modal = kphis_ui_component::modal::pre_order_select::PreOrderSelect::new(kphis_ui_component::modal::pre_order_select::PreOrderType::Template, "1", kphis_ui_component::modal::pre_order_preview::ToOrderType::Order);
    let dom = kphis_ui_component::modal::pre_order_select::PreOrderSelect::render_modal(modal.clone(), Mutable::new(Some(modal)), None, None, app);
    replace_body(dom).await;
}
#[wasm_bindgen_test]
async fn test_pre_order_select_cpn_pre() {
    let app = new_app();
    let modal = kphis_ui_component::modal::pre_order_select::PreOrderSelect::new(kphis_ui_component::modal::pre_order_select::PreOrderType::PreOrder(String::new()), "1", kphis_ui_component::modal::pre_order_preview::ToOrderType::Order);
    let dom = kphis_ui_component::modal::pre_order_select::PreOrderSelect::render_modal(modal.clone(), Mutable::new(Some(modal)), None, None, app);
    replace_body(dom).await;
}
#[wasm_bindgen_test]
async fn test_pre_order_select_cpn_inner_template() {
    let app = new_app();
    let modal = kphis_ui_component::modal::pre_order_select::PreOrderSelect::new(kphis_ui_component::modal::pre_order_select::PreOrderType::PreOrder(String::new()), "1", kphis_ui_component::modal::pre_order_preview::ToOrderType::Order);
    let dom = kphis_ui_component::modal::pre_order_select::PreOrderType::Template.render(modal, app);
    replace_body(dom).await;
}
#[wasm_bindgen_test]
async fn test_pre_order_select_cpn_inner_pre() {
    let app = new_app();
    let modal = kphis_ui_component::modal::pre_order_select::PreOrderSelect::new(kphis_ui_component::modal::pre_order_select::PreOrderType::PreOrder(String::new()), "1", kphis_ui_component::modal::pre_order_preview::ToOrderType::Order);
    let dom = kphis_ui_component::modal::pre_order_select::PreOrderType::PreOrder(String::from("0001234")).render(modal, app);
    replace_body(dom).await;
}

#[wasm_bindgen_test]
async fn test_vs_selector_cpn() {
    let app = new_app();
    let modal = kphis_ui_component::modal::vs_selector::VsSelector::new(false, Mutable::new(Some(Rc::new(kphis_model::patient_info::PatientInfo::demo()))), Mutable::new(String::new()), Mutable::new(false));
    let dom = kphis_ui_component::modal::vs_selector::VsSelector::render_modal(modal.clone(), Mutable::new(Some(modal)), app);
    replace_body(dom).await;
}
#[wasm_bindgen_test]
async fn test_vs_selector_cpn_datetime() {
    let app = new_app();
    let modal = kphis_ui_component::modal::vs_selector::VsSelector::new(true, Mutable::new(Some(Rc::new(kphis_model::patient_info::PatientInfo::demo()))), Mutable::new(String::new()), Mutable::new(false));
    let dom = kphis_ui_component::modal::vs_selector::VsSelector::render_modal(modal.clone(), Mutable::new(Some(modal)), app);
    replace_body(dom).await;
}
