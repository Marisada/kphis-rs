use futures_signals::signal::Mutable;
use wasm_bindgen_test::wasm_bindgen_test;

use crate::{new_app, replace_body};

#[wasm_bindgen_test]
async fn test_addict_assist_cpn() {
    let app = new_app();
    let modal = kphis_ui_component::modal::scoring::addict_assist::AddictAssistV2::new(Mutable::new(String::new()), Mutable::new(String::new()), Mutable::new(false));
    let dom = kphis_ui_component::modal::scoring::addict_assist::AddictAssistV2::render_modal(modal.clone(), Mutable::new(Some(modal)), app);
    replace_body(dom).await;
}

#[wasm_bindgen_test]
async fn test_aggression_oas_cpn() {
    let app = new_app();
    let modal = kphis_ui_component::modal::scoring::aggression_oas::AggressionOAS::new(Mutable::new(String::new()), Mutable::new(false));
    let dom = kphis_ui_component::modal::scoring::aggression_oas::AggressionOAS::render_modal(modal.clone(), Mutable::new(Some(modal)), app);
    replace_body(dom).await;
}

#[wasm_bindgen_test]
async fn test_alcohol_audit_cpn() {
    let app = new_app();
    let modal = kphis_ui_component::modal::scoring::alcohol_audit::AlcoholAudit::new(Mutable::new(String::new()), Mutable::new(false));
    let dom = kphis_ui_component::modal::scoring::alcohol_audit::AlcoholAudit::render_modal(modal.clone(), Mutable::new(Some(modal)), app);
    replace_body(dom).await;
}

#[wasm_bindgen_test]
async fn test_alcohol_aws_cpn() {
    let app = new_app();
    let modal = kphis_ui_component::modal::scoring::alcohol_aws::AlcoholAws::new(Mutable::new(String::new()), Mutable::new(false));
    let dom = kphis_ui_component::modal::scoring::alcohol_aws::AlcoholAws::render_modal(modal.clone(), Mutable::new(Some(modal)), app);
    replace_body(dom).await;
}

#[wasm_bindgen_test]
async fn test_alcohol_ciwa_ar_cpn() {
    let app = new_app();
    let modal = kphis_ui_component::modal::scoring::alcohol_ciwa_ar::AlcoholCiwaAr::new(Mutable::new(String::new()), Mutable::new(false));
    let dom = kphis_ui_component::modal::scoring::alcohol_ciwa_ar::AlcoholCiwaAr::render_modal(modal.clone(), Mutable::new(Some(modal)), app);
    replace_body(dom).await;
}

#[wasm_bindgen_test]
async fn test_amphetamine_awq_cpn() {
    let app = new_app();
    let modal = kphis_ui_component::modal::scoring::amphetamine_awq::AmphetamineAwqV2::new(Mutable::new(String::new()), Mutable::new(false));
    let dom = kphis_ui_component::modal::scoring::amphetamine_awq::AmphetamineAwqV2::render_modal(modal.clone(), Mutable::new(Some(modal)), app);
    replace_body(dom).await;
}

#[wasm_bindgen_test]
async fn test_barthel_index_cpn() {
    let app = new_app();
    let modal = kphis_ui_component::modal::scoring::barthel_index::BarthelIndex::new(Mutable::new(String::new()), Mutable::new(false));
    let dom = kphis_ui_component::modal::scoring::barthel_index::BarthelIndex::render_modal(modal.clone(), Mutable::new(Some(modal)), app);
    replace_body(dom).await;
}

#[wasm_bindgen_test]
async fn test_braden_cpn() {
    let app = new_app();
    let modal = kphis_ui_component::modal::scoring::braden::Braden::new(Mutable::new(String::new()), Mutable::new(false));
    let dom = kphis_ui_component::modal::scoring::braden::Braden::render_modal(modal.clone(), Mutable::new(Some(modal)), app);
    replace_body(dom).await;
}

#[wasm_bindgen_test]
async fn test_depress_2q_cpn() {
    let app = new_app();
    let modal = kphis_ui_component::modal::scoring::depress_2q::Depress2Q::new(Mutable::new(String::new()), Mutable::new(false));
    let dom = kphis_ui_component::modal::scoring::depress_2q::Depress2Q::render_modal(modal.clone(), Mutable::new(Some(modal)), app);
    replace_body(dom).await;
}

#[wasm_bindgen_test]
async fn test_depress_9q_cpn() {
    let app = new_app();
    let modal = kphis_ui_component::modal::scoring::depress_9q::Depress9Q::new(Mutable::new(String::new()), Mutable::new(false));
    let dom = kphis_ui_component::modal::scoring::depress_9q::Depress9Q::render_modal(modal.clone(), Mutable::new(Some(modal)), app);
    replace_body(dom).await;
}

#[wasm_bindgen_test]
async fn test_depress_cdi_cpn() {
    let app = new_app();
    let modal = kphis_ui_component::modal::scoring::depress_cdi::DepressCdi::new(Mutable::new(String::new()), Mutable::new(false));
    let dom = kphis_ui_component::modal::scoring::depress_cdi::DepressCdi::render_modal(modal.clone(), Mutable::new(Some(modal)), app);
    replace_body(dom).await;
}

#[wasm_bindgen_test]
async fn test_depress_cesd_cpn() {
    let app = new_app();
    let modal = kphis_ui_component::modal::scoring::depress_cesd::DepressCesD::new(Mutable::new(String::new()), Mutable::new(false));
    let dom = kphis_ui_component::modal::scoring::depress_cesd::DepressCesD::render_modal(modal.clone(), Mutable::new(Some(modal)), app);
    replace_body(dom).await;
}

#[wasm_bindgen_test]
async fn test_depress_phqa_cpn() {
    let app = new_app();
    let modal = kphis_ui_component::modal::scoring::depress_phqa::DepressPhqA::new(Mutable::new(String::new()), Mutable::new(false));
    let dom = kphis_ui_component::modal::scoring::depress_phqa::DepressPhqA::render_modal(modal.clone(), Mutable::new(Some(modal)), app);
    replace_body(dom).await;
}

#[wasm_bindgen_test]
async fn test_motor_activity_maas_cpn() {
    let app = new_app();
    let modal = kphis_ui_component::modal::scoring::motor_activity_maas::MotorActivityMaas::new(Mutable::new(String::new()), Mutable::new(false));
    let dom = kphis_ui_component::modal::scoring::motor_activity_maas::MotorActivityMaas::render_modal(modal.clone(), Mutable::new(Some(modal)), app);
    replace_body(dom).await;
}

#[wasm_bindgen_test]
async fn test_nicotin_ftnd_cpn() {
    let app = new_app();
    let modal = kphis_ui_component::modal::scoring::nicotin_ftnd::NicotinFtnd::new(Mutable::new(String::new()), Mutable::new(false));
    let dom = kphis_ui_component::modal::scoring::nicotin_ftnd::NicotinFtnd::render_modal(modal.clone(), Mutable::new(Some(modal)), app);
    replace_body(dom).await;
}

#[wasm_bindgen_test]
async fn test_ptsd_cries13_cpn() {
    let app = new_app();
    let modal = kphis_ui_component::modal::scoring::ptsd_cries13::PtsdCries13::new(Mutable::new(String::new()), Mutable::new(false));
    let dom = kphis_ui_component::modal::scoring::ptsd_cries13::PtsdCries13::render_modal(modal.clone(), Mutable::new(Some(modal)), app);
    replace_body(dom).await;
}

#[wasm_bindgen_test]
async fn test_ptsd_pisces10_cpn() {
    let app = new_app();
    let modal = kphis_ui_component::modal::scoring::ptsd_pisces10::PtsdPisces10::new(Mutable::new(String::new()), Mutable::new(false));
    let dom = kphis_ui_component::modal::scoring::ptsd_pisces10::PtsdPisces10::render_modal(modal.clone(), Mutable::new(Some(modal)), app);
    replace_body(dom).await;
}

#[wasm_bindgen_test]
async fn test_ptsd_screen_cpn() {
    let app = new_app();
    let modal = kphis_ui_component::modal::scoring::ptsd_screen::PtsdScreen::new(Mutable::new(String::new()), Mutable::new(false));
    let dom = kphis_ui_component::modal::scoring::ptsd_screen::PtsdScreen::render_modal(modal.clone(), Mutable::new(Some(modal)), app);
    replace_body(dom).await;
}

#[wasm_bindgen_test]
async fn test_stress_st5_cpn() {
    let app = new_app();
    let modal = kphis_ui_component::modal::scoring::stress_st5::StressST5::new(Mutable::new(String::new()), Mutable::new(false));
    let dom = kphis_ui_component::modal::scoring::stress_st5::StressST5::render_modal(modal.clone(), Mutable::new(Some(modal)), app);
    replace_body(dom).await;
}

#[wasm_bindgen_test]
async fn test_suicide_8q_cpn() {
    let app = new_app();
    let modal = kphis_ui_component::modal::scoring::suicide_8q::Suicide8Q::new(Mutable::new(String::new()), Mutable::new(false));
    let dom = kphis_ui_component::modal::scoring::suicide_8q::Suicide8Q::render_modal(modal.clone(), Mutable::new(Some(modal)), app);
    replace_body(dom).await;
}
