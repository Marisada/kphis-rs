use futures_signals::signal::Mutable;
use std::rc::Rc;
use time::macros::date;
use wasm_bindgen_test::wasm_bindgen_test;

use crate::{new_app, replace_body};

#[wasm_bindgen_test]
async fn test_ipd_med_reconcile_hosxp_cpn() {
    let app = new_app();
    let cpn = kphis_ui_component::med_reconcile::ipd_med_reconcile_hosxp::IpdMedReconcileHosXpCpn::new(Mutable::new(Some(Rc::new(kphis_model::patient_info::PatientInfo::demo()))));
    let dom = kphis_ui_component::med_reconcile::ipd_med_reconcile_hosxp::IpdMedReconcileHosXpCpn::render(cpn, app);
    replace_body(dom).await;
}
#[wasm_bindgen_test]
async fn test_ipd_med_reconcile_hosxp_cpn_recon() {
    let dom = kphis_ui_component::med_reconcile::ipd_med_reconcile_hosxp::IpdMedReconcileHosXpCpn::render_recon(Rc::new(kphis_model::med_reconcile::MedReconciliationDetail::demo()), 1);
    replace_body(dom).await;
}

#[wasm_bindgen_test]
async fn test_ipd_med_reconcile_last_dose_cpn() {
    let app = new_app();
    let cpn = kphis_ui_component::med_reconcile::ipd_med_reconcile_last_dose::IpdMedReconcileLastDoseCpn::new(Mutable::new(Some(Rc::new(kphis_model::patient_info::PatientInfo::demo()))));
    let dom = kphis_ui_component::med_reconcile::ipd_med_reconcile_last_dose::IpdMedReconcileLastDoseCpn::render(cpn, app);
    replace_body(dom).await;
}

#[wasm_bindgen_test]
async fn test_med_reconcile_form_cpn_doctor() {
    let app = new_app();
    let cpn = kphis_ui_component::med_reconcile::med_reconcile_form::MedReconForm::new(Mutable::new(String::from("doctor")), Mutable::new(Some(date!(2023-12-31))), Mutable::new(kphis_model::tab::Tab::Order), Mutable::new(false), Mutable::new(false), Mutable::new(None), kphis_model::med_reconcile::MedReconciliation::demo());
    let dom = kphis_ui_component::med_reconcile::med_reconcile_form::MedReconForm::render(Rc::new(cpn), app);
    replace_body(dom).await;
}
#[wasm_bindgen_test]
async fn test_med_reconcile_form_cpn_nurse() {
    let app = new_app();
    let cpn = kphis_ui_component::med_reconcile::med_reconcile_form::MedReconForm::new(Mutable::new(String::from("nurse")), Mutable::new(Some(date!(2023-12-31))), Mutable::new(kphis_model::tab::Tab::Order), Mutable::new(false), Mutable::new(false), Mutable::new(None), kphis_model::med_reconcile::MedReconciliation::demo());
    let dom = kphis_ui_component::med_reconcile::med_reconcile_form::MedReconForm::render(Rc::new(cpn), app);
    replace_body(dom).await;
}
#[wasm_bindgen_test]
async fn test_med_reconcile_form_cpn_pharmacist() {
    let app = new_app();
    let cpn = kphis_ui_component::med_reconcile::med_reconcile_form::MedReconForm::new(Mutable::new(String::from("pharmacist")), Mutable::new(Some(date!(2023-12-31))), Mutable::new(kphis_model::tab::Tab::Order), Mutable::new(false), Mutable::new(false), Mutable::new(None), kphis_model::med_reconcile::MedReconciliation::demo());
    let dom = kphis_ui_component::med_reconcile::med_reconcile_form::MedReconForm::render(Rc::new(cpn), app);
    replace_body(dom).await;
}
#[wasm_bindgen_test]
async fn test_med_reconcile_form_cpn_other() {
    let app = new_app();
    let cpn = kphis_ui_component::med_reconcile::med_reconcile_form::MedReconForm::new(Mutable::new(String::from("other")), Mutable::new(Some(date!(2023-12-31))), Mutable::new(kphis_model::tab::Tab::Order), Mutable::new(false), Mutable::new(false), Mutable::new(None), kphis_model::med_reconcile::MedReconciliation::demo());
    let dom = kphis_ui_component::med_reconcile::med_reconcile_form::MedReconForm::render(Rc::new(cpn), app);
    replace_body(dom).await;
}
#[wasm_bindgen_test]
async fn test_med_reconcile_form_cpn_modal() {
    let app = new_app();
    let cpn = kphis_ui_component::med_reconcile::med_reconcile_form::MedReconForm::new(Mutable::new(String::from("other")), Mutable::new(Some(date!(2023-12-31))), Mutable::new(kphis_model::tab::Tab::Order), Mutable::new(false), Mutable::new(false), Mutable::new(None), kphis_model::med_reconcile::MedReconciliation::demo());
    let dom = kphis_ui_component::med_reconcile::med_reconcile_form::MedReconForm::render_note_modal(Rc::new(cpn), app);
    replace_body(dom).await;
}

#[wasm_bindgen_test]
async fn test_med_reconcile_history_cpn() {
    let app = new_app();
    let cpn = kphis_ui_component::med_reconcile::med_reconcile_history::MedReconcileHistoryCpn::new(Mutable::new(false), Mutable::new(Some(Rc::new(kphis_model::patient_info::PatientInfo::demo()))));
    let dom = kphis_ui_component::med_reconcile::med_reconcile_history::MedReconcileHistoryCpn::render(cpn, app);
    replace_body(dom).await;
}
#[wasm_bindgen_test]
async fn test_med_reconcile_history_cpn_recon() {
    let app = new_app();
    let cpn = kphis_ui_component::med_reconcile::med_reconcile_history::MedReconcileHistoryCpn::new(Mutable::new(false), Mutable::new(Some(Rc::new(kphis_model::patient_info::PatientInfo::demo()))));
    let dom = kphis_ui_component::med_reconcile::med_reconcile_history::MedReconcileHistoryCpn::render_recon(Rc::new(kphis_model::med_reconcile::MedReconciliation::demo()), cpn, app);
    replace_body(dom).await;
}

#[wasm_bindgen_test]
async fn test_med_reconcile_main_cpn_doctor() {
    let app = new_app();
    let cpn = kphis_ui_component::med_reconcile::med_reconcile_main::MedReconcileCpn::new(Mutable::new(String::from("doctor")), Mutable::new(Some(Rc::new(kphis_model::patient_info::PatientInfo::demo()))), Mutable::new(kphis_model::tab::Tab::Order), Mutable::new(false), Mutable::new(false));
    let dom = kphis_ui_component::med_reconcile::med_reconcile_main::MedReconcileCpn::render(cpn, app);
    replace_body(dom).await;
}
#[wasm_bindgen_test]
async fn test_med_reconcile_main_cpn_nurse() {
    let app = new_app();
    let cpn = kphis_ui_component::med_reconcile::med_reconcile_main::MedReconcileCpn::new(Mutable::new(String::from("nurse")), Mutable::new(Some(Rc::new(kphis_model::patient_info::PatientInfo::demo()))), Mutable::new(kphis_model::tab::Tab::Order), Mutable::new(false), Mutable::new(false));
    let dom = kphis_ui_component::med_reconcile::med_reconcile_main::MedReconcileCpn::render(cpn, app);
    replace_body(dom).await;
}
#[wasm_bindgen_test]
async fn test_med_reconcile_main_cpn_pharmacist() {
    let app = new_app();
    let cpn = kphis_ui_component::med_reconcile::med_reconcile_main::MedReconcileCpn::new(Mutable::new(String::from("pharmacist")), Mutable::new(Some(Rc::new(kphis_model::patient_info::PatientInfo::demo()))), Mutable::new(kphis_model::tab::Tab::Order), Mutable::new(false), Mutable::new(false));
    let dom = kphis_ui_component::med_reconcile::med_reconcile_main::MedReconcileCpn::render(cpn, app);
    replace_body(dom).await;
}
#[wasm_bindgen_test]
async fn test_med_reconcile_main_cpn_other() {
    let app = new_app();
    let cpn = kphis_ui_component::med_reconcile::med_reconcile_main::MedReconcileCpn::new(Mutable::new(String::from("other")), Mutable::new(Some(Rc::new(kphis_model::patient_info::PatientInfo::demo()))), Mutable::new(kphis_model::tab::Tab::Order), Mutable::new(false), Mutable::new(false));
    let dom = kphis_ui_component::med_reconcile::med_reconcile_main::MedReconcileCpn::render(cpn, app);
    replace_body(dom).await;
}
