use futures_signals::signal::Mutable;
use std::rc::Rc;
use time::macros::date;
use wasm_bindgen_test::wasm_bindgen_test;

use crate::{new_app, replace_body};

// // can't test rendering chartjs
// #[wasm_bindgen_test]
// async fn test_chart_cpn_gen() {
//     let dom = kphis_ui_component::vital_sign::chart::render(&[Rc::new(kphis_model::vital_sign::VitalSign::demo())], "", "", kphis_model::vital_sign::VsMode::General, false);
//     replace_body(dom).await;
// }
// #[wasm_bindgen_test]
// async fn test_chart_cpn_labour() {
//     let dom = kphis_ui_component::vital_sign::chart::render(&[Rc::new(kphis_model::vital_sign::VitalSign::demo())], "", "", kphis_model::vital_sign::VsMode::Labour, false);
//     replace_body(dom).await;
// }
// #[wasm_bindgen_test]
// async fn test_chart_cpn_neuro() {
//     let dom = kphis_ui_component::vital_sign::chart::render(&[Rc::new(kphis_model::vital_sign::VitalSign::demo())], "", "", kphis_model::vital_sign::VsMode::Neuro, false);
//     replace_body(dom).await;
// }
// #[wasm_bindgen_test]
// async fn test_chart_cpn_psychia() {
//     let dom = kphis_ui_component::vital_sign::chart::render(&[Rc::new(kphis_model::vital_sign::VitalSign::demo())], "", "", kphis_model::vital_sign::VsMode::Psychia, false);
//     replace_body(dom).await;
// }

#[wasm_bindgen_test]
async fn test_items_general_cpn() {
    let app = new_app();
    let dom = kphis_ui_component::vital_sign::items_general::render_vs_result(Rc::new(kphis_model::vital_sign::VitalSign::demo()), Mutable::new(1), Mutable::new(false), Some(date!(2000-12-31)), app);
    replace_body(dom).await;
}

#[wasm_bindgen_test]
async fn test_items_labour_cpn() {
    let app = new_app();
    let dom = kphis_ui_component::vital_sign::items_labour::render_lr_result(Rc::new(kphis_model::vital_sign::VitalSign::demo()), Mutable::new(1), Mutable::new(false), Some(date!(2000-12-31)), app);
    replace_body(dom).await;
}

#[wasm_bindgen_test]
async fn test_items_neuro_cpn() {
    let app = new_app();
    let dom = kphis_ui_component::vital_sign::items_neuro::render_neuro_result(Rc::new(kphis_model::vital_sign::VitalSign::demo()), Mutable::new(1), Mutable::new(false), Some(date!(2000-12-31)), app);
    replace_body(dom).await;
}

#[wasm_bindgen_test]
async fn test_items_psychia_cpn() {
    let app = new_app();
    let dom = kphis_ui_component::vital_sign::items_psychia::render_psychia_result(Rc::new(kphis_model::vital_sign::VitalSign::demo()), Mutable::new(1), Mutable::new(false), Some(date!(2000-12-31)), app);
    replace_body(dom).await;
}

#[wasm_bindgen_test]
async fn test_vital_sign_data_cpn_gen() {
    let app = new_app();
    let cpn = kphis_ui_component::vital_sign::vital_sign_data::VitalSignDataCpn::new(Mutable::new(Some(Rc::new(kphis_model::patient_info::PatientInfo::demo()))), Mutable::new(vec!(Rc::new(kphis_model::vital_sign::VitalSign::demo()))), Mutable::new(String::from("2023-12-31")), Mutable::new(String::from("2023-12-31")), Mutable::new(kphis_model::vital_sign::VsMode::General));
    let dom = kphis_ui_component::vital_sign::vital_sign_data::VitalSignDataCpn::render(Mutable::new(1), Mutable::new(false), cpn, app);
    replace_body(dom).await;
}
#[wasm_bindgen_test]
async fn test_vital_sign_data_cpn_labour() {
    let app = new_app();
    let cpn = kphis_ui_component::vital_sign::vital_sign_data::VitalSignDataCpn::new(Mutable::new(Some(Rc::new(kphis_model::patient_info::PatientInfo::demo()))), Mutable::new(vec!(Rc::new(kphis_model::vital_sign::VitalSign::demo()))), Mutable::new(String::from("2023-12-31")), Mutable::new(String::from("2023-12-31")), Mutable::new(kphis_model::vital_sign::VsMode::Labour));
    let dom = kphis_ui_component::vital_sign::vital_sign_data::VitalSignDataCpn::render(Mutable::new(1), Mutable::new(false), cpn, app);
    replace_body(dom).await;
}
#[wasm_bindgen_test]
async fn test_vital_sign_data_cpn_neuro() {
    let app = new_app();
    let cpn = kphis_ui_component::vital_sign::vital_sign_data::VitalSignDataCpn::new(Mutable::new(Some(Rc::new(kphis_model::patient_info::PatientInfo::demo()))), Mutable::new(vec!(Rc::new(kphis_model::vital_sign::VitalSign::demo()))), Mutable::new(String::from("2023-12-31")), Mutable::new(String::from("2023-12-31")), Mutable::new(kphis_model::vital_sign::VsMode::Neuro));
    let dom = kphis_ui_component::vital_sign::vital_sign_data::VitalSignDataCpn::render(Mutable::new(1), Mutable::new(false), cpn, app);
    replace_body(dom).await;
}
#[wasm_bindgen_test]
async fn test_vital_sign_data_cpn_psychia() {
    let app = new_app();
    let cpn = kphis_ui_component::vital_sign::vital_sign_data::VitalSignDataCpn::new(Mutable::new(Some(Rc::new(kphis_model::patient_info::PatientInfo::demo()))), Mutable::new(vec!(Rc::new(kphis_model::vital_sign::VitalSign::demo()))), Mutable::new(String::from("2023-12-31")), Mutable::new(String::from("2023-12-31")), Mutable::new(kphis_model::vital_sign::VsMode::Psychia));
    let dom = kphis_ui_component::vital_sign::vital_sign_data::VitalSignDataCpn::render(Mutable::new(1), Mutable::new(false), cpn, app);
    replace_body(dom).await;
}
#[wasm_bindgen_test]
async fn test_vital_sign_data_cpn_op() {
    let cpn = kphis_ui_component::vital_sign::vital_sign_data::VitalSignDataCpn::new(Mutable::new(Some(Rc::new(kphis_model::patient_info::PatientInfo::demo()))), Mutable::new(vec!(Rc::new(kphis_model::vital_sign::VitalSign::demo()))), Mutable::new(String::from("2023-12-31")), Mutable::new(String::from("2023-12-31")), Mutable::new(kphis_model::vital_sign::VsMode::General));
    let dom = kphis_ui_component::vital_sign::vital_sign_data::VitalSignDataCpn::render_op(cpn);
    replace_body(dom).await;
}
#[wasm_bindgen_test]
async fn test_vital_sign_data_cpn_table() {
    let app = new_app();
    let cpn = kphis_ui_component::vital_sign::vital_sign_data::VitalSignDataCpn::new(Mutable::new(Some(Rc::new(kphis_model::patient_info::PatientInfo::demo()))), Mutable::new(vec!(Rc::new(kphis_model::vital_sign::VitalSign::demo()))), Mutable::new(String::from("2023-12-31")), Mutable::new(String::from("2023-12-31")), Mutable::new(kphis_model::vital_sign::VsMode::General));
    let dom = kphis_ui_component::vital_sign::vital_sign_data::VitalSignDataCpn::render_table(Mutable::new(1), Mutable::new(false), cpn, app);
    replace_body(dom).await;
}
#[wasm_bindgen_test]
async fn test_vital_sign_data_cpn_search_btn() {
    let cpn = kphis_ui_component::vital_sign::vital_sign_data::VitalSignDataCpn::new(Mutable::new(Some(Rc::new(kphis_model::patient_info::PatientInfo::demo()))), Mutable::new(vec!(Rc::new(kphis_model::vital_sign::VitalSign::demo()))), Mutable::new(String::from("2023-12-31")), Mutable::new(String::from("2023-12-31")), Mutable::new(kphis_model::vital_sign::VsMode::General));
    let dom = kphis_ui_component::vital_sign::vital_sign_data::VitalSignDataCpn::search_btn("label", "txt", cpn);
    replace_body(dom).await;
}
#[wasm_bindgen_test]
async fn test_vital_sign_data_cpn_range_btn() {
    let cpn = kphis_ui_component::vital_sign::vital_sign_data::VitalSignDataCpn::new(Mutable::new(Some(Rc::new(kphis_model::patient_info::PatientInfo::demo()))), Mutable::new(vec!(Rc::new(kphis_model::vital_sign::VitalSign::demo()))), Mutable::new(String::from("2023-12-31")), Mutable::new(String::from("2023-12-31")), Mutable::new(kphis_model::vital_sign::VsMode::General));
    let dom = kphis_ui_component::vital_sign::vital_sign_data::VitalSignDataCpn::render_range_button(cpn, 1, "label");
    replace_body(dom).await;
}
#[wasm_bindgen_test]
async fn test_vital_sign_data_cpn_vs_radio() {
    let cpn = kphis_ui_component::vital_sign::vital_sign_data::VitalSignDataCpn::new(Mutable::new(Some(Rc::new(kphis_model::patient_info::PatientInfo::demo()))), Mutable::new(vec!(Rc::new(kphis_model::vital_sign::VitalSign::demo()))), Mutable::new(String::from("2023-12-31")), Mutable::new(String::from("2023-12-31")), Mutable::new(kphis_model::vital_sign::VsMode::General));
    let dom = kphis_ui_component::vital_sign::vital_sign_data::VitalSignDataCpn::vs_mode_radio(cpn);
    replace_body(dom).await;
}

#[wasm_bindgen_test]
async fn test_vital_sign_data_cpn() {
    let app = new_app();
    let cpn = kphis_ui_component::vital_sign::vital_sign_form::VitalSignFormCpn::new(Mutable::new(Some(Rc::new(kphis_model::patient_info::PatientInfo::demo()))), Some(Rc::new(kphis_model::vital_sign::VitalSign::demo())), app.clone());
    let dom = kphis_ui_component::vital_sign::vital_sign_form::VitalSignFormCpn::render(Mutable::new(1), Mutable::new(false), Mutable::new(false), cpn, app);
    replace_body(dom).await;
}
#[wasm_bindgen_test]
async fn test_vital_sign_data_cpn_tab_vs() {
    let app = new_app();
    let cpn = kphis_ui_component::vital_sign::vital_sign_form::VitalSignFormCpn::new(Mutable::new(Some(Rc::new(kphis_model::patient_info::PatientInfo::demo()))), Some(Rc::new(kphis_model::vital_sign::VitalSign::demo())), app.clone());
    let dom = kphis_ui_component::vital_sign::vital_sign_form::VitalSignFormCpn::render_tab_vs(cpn, app);
    replace_body(dom).await;
}
#[wasm_bindgen_test]
async fn test_vital_sign_data_cpn_tab_neuro() {
    let app = new_app();
    let cpn = kphis_ui_component::vital_sign::vital_sign_form::VitalSignFormCpn::new(Mutable::new(Some(Rc::new(kphis_model::patient_info::PatientInfo::demo()))), Some(Rc::new(kphis_model::vital_sign::VitalSign::demo())), app.clone());
    let dom = kphis_ui_component::vital_sign::vital_sign_form::VitalSignFormCpn::render_tab_neuro(cpn, app);
    replace_body(dom).await;
}
#[wasm_bindgen_test]
async fn test_vital_sign_data_cpn_tab_score() {
    let app = new_app();
    let cpn = kphis_ui_component::vital_sign::vital_sign_form::VitalSignFormCpn::new(Mutable::new(Some(Rc::new(kphis_model::patient_info::PatientInfo::demo()))), Some(Rc::new(kphis_model::vital_sign::VitalSign::demo())), app.clone());
    let dom = kphis_ui_component::vital_sign::vital_sign_form::VitalSignFormCpn::render_tab_score(cpn, app);
    replace_body(dom).await;
}
#[wasm_bindgen_test]
async fn test_vital_sign_data_cpn_tab_had() {
    let app = new_app();
    let cpn = kphis_ui_component::vital_sign::vital_sign_form::VitalSignFormCpn::new(Mutable::new(Some(Rc::new(kphis_model::patient_info::PatientInfo::demo()))), Some(Rc::new(kphis_model::vital_sign::VitalSign::demo())), app.clone());
    let dom = kphis_ui_component::vital_sign::vital_sign_form::VitalSignFormCpn::render_tab_had(cpn);
    replace_body(dom).await;
}
#[wasm_bindgen_test]
async fn test_vital_sign_data_cpn_tab_o2() {
    let app = new_app();
    let cpn = kphis_ui_component::vital_sign::vital_sign_form::VitalSignFormCpn::new(Mutable::new(Some(Rc::new(kphis_model::patient_info::PatientInfo::demo()))), Some(Rc::new(kphis_model::vital_sign::VitalSign::demo())), app.clone());
    let dom = kphis_ui_component::vital_sign::vital_sign_form::VitalSignFormCpn::render_tab_o2(cpn, app);
    replace_body(dom).await;
}
#[wasm_bindgen_test]
async fn test_vital_sign_data_cpn_tab_lr() {
    let app = new_app();
    let cpn = kphis_ui_component::vital_sign::vital_sign_form::VitalSignFormCpn::new(Mutable::new(Some(Rc::new(kphis_model::patient_info::PatientInfo::demo()))), Some(Rc::new(kphis_model::vital_sign::VitalSign::demo())), app.clone());
    let dom = kphis_ui_component::vital_sign::vital_sign_form::VitalSignFormCpn::render_tab_lr(cpn, app);
    replace_body(dom).await;
}
#[wasm_bindgen_test]
async fn test_vital_sign_data_cpn_tab_other() {
    let app = new_app();
    let cpn = kphis_ui_component::vital_sign::vital_sign_form::VitalSignFormCpn::new(Mutable::new(Some(Rc::new(kphis_model::patient_info::PatientInfo::demo()))), Some(Rc::new(kphis_model::vital_sign::VitalSign::demo())), app.clone());
    let dom = kphis_ui_component::vital_sign::vital_sign_form::VitalSignFormCpn::render_tab_other(cpn, app);
    replace_body(dom).await;
}

#[wasm_bindgen_test]
async fn test_vital_sign_main_cpn_doctor() {
    let app = new_app();
    let cpn = kphis_ui_component::vital_sign::vital_sign_main::VitalSignCpn::new(Mutable::new(Some(Rc::new(kphis_model::patient_info::PatientInfo::demo()))), Mutable::new(false), Mutable::new(String::from("doctor")));
    let dom = kphis_ui_component::vital_sign::vital_sign_main::VitalSignCpn::render(cpn, app);
    replace_body(dom).await;
}
#[wasm_bindgen_test]
async fn test_vital_sign_main_cpn_nurse() {
    let app = new_app();
    let cpn = kphis_ui_component::vital_sign::vital_sign_main::VitalSignCpn::new(Mutable::new(Some(Rc::new(kphis_model::patient_info::PatientInfo::demo()))), Mutable::new(false), Mutable::new(String::from("nurse")));
    let dom = kphis_ui_component::vital_sign::vital_sign_main::VitalSignCpn::render(cpn, app);
    replace_body(dom).await;
}
#[wasm_bindgen_test]
async fn test_vital_sign_main_cpn_pharmacist() {
    let app = new_app();
    let cpn = kphis_ui_component::vital_sign::vital_sign_main::VitalSignCpn::new(Mutable::new(Some(Rc::new(kphis_model::patient_info::PatientInfo::demo()))), Mutable::new(false), Mutable::new(String::from("pharmacist")));
    let dom = kphis_ui_component::vital_sign::vital_sign_main::VitalSignCpn::render(cpn, app);
    replace_body(dom).await;
}
#[wasm_bindgen_test]
async fn test_vital_sign_main_cpn_other() {
    let app = new_app();
    let cpn = kphis_ui_component::vital_sign::vital_sign_main::VitalSignCpn::new(Mutable::new(Some(Rc::new(kphis_model::patient_info::PatientInfo::demo()))), Mutable::new(false), Mutable::new(String::from("other")));
    let dom = kphis_ui_component::vital_sign::vital_sign_main::VitalSignCpn::render(cpn, app);
    replace_body(dom).await;
}
