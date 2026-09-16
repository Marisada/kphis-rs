mod document;
mod gadget;
mod med_reconcile;
mod modal;
mod nurse_note;
mod order_form;
mod vital_sign;

use dominator::Dom;
use futures_signals::{signal::Mutable, signal_vec::MutableVec};
use std::rc::Rc;
use wasm_bindgen::JsValue;
use wasm_bindgen_futures::JsFuture;
use wasm_bindgen_test::wasm_bindgen_test;

use kphis_model::app::AppState;
use kphis_ui_app::App;

wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);

pub fn new_app() -> Rc<App> {
    let app_state = AppState::new_from_local_storage("/");
    let asset = Rc::new(kphis_model::app::AppAsset::demo());
    app_state.app_asset.set(Some(asset));
    let mut status = kphis_model::app::AppStatus::demo();
    status.is_production = false;
    status.is_read_only_mode = false;
    app_state.app_status.set(Some(Rc::new(status)));
    let now = kphis_util::datetime::get_timestamp_wasm();
    let client = kphis_model::user::his::UserClientMutable {
        user: kphis_model::user::his::UserMutable::from(kphis_model::user::his::User::demo()),
        roles: MutableVec::new(),
        permissions: Mutable::new(Vec::new()),
        token: Mutable::new(String::from("TOKEN")),
        sub: Mutable::new(String::from("1")),
        iat: Mutable::new(now),
        exp: Mutable::new(u64::MAX),
        earlier_second: Mutable::new(0),
    };
    app_state.user.set(Some(Rc::new(client)));
    App::new(app_state)
}

pub async fn replace_body(dom: Dom) {
    dominator::replace_dom(&dominator::body(), &dominator::body().first_child().unwrap(), dom);
    // move to next tick
    JsFuture::from(js_sys::Promise::resolve(&JsValue::null())).await.unwrap();
}

#[wasm_bindgen_test]
async fn test_admission_note_cpn() {
    let app = new_app();
    let cpn = kphis_ui_component::admission_note::AdmissionNoteCpn::new(Mutable::new(Some(Rc::new(kphis_model::patient_info::PatientInfo::demo()))));
    let dom = kphis_ui_component::admission_note::AdmissionNoteCpn::render(cpn, Mutable::new(false), false, app);
    replace_body(dom).await;
}
#[wasm_bindgen_test]
async fn test_admission_note_cpn_aside() {
    let app = new_app();
    let cpn = kphis_ui_component::admission_note::AdmissionNoteCpn::new(Mutable::new(Some(Rc::new(kphis_model::patient_info::PatientInfo::demo()))));
    let dom = kphis_ui_component::admission_note::AdmissionNoteCpn::render(cpn, Mutable::new(false), true, app);
    replace_body(dom).await;
}

#[wasm_bindgen_test]
async fn test_doctor_in_charge_cpn() {
    let app = new_app();
    let cpn = kphis_ui_component::doctor_in_charge::DoctorInChargeCpn::new(Mutable::new(String::from("660001234")), Mutable::new(String::from("0001234")));
    let dom = kphis_ui_component::doctor_in_charge::DoctorInChargeCpn::render(cpn, app);
    replace_body(dom).await;
}

#[wasm_bindgen_test]
async fn test_document_cpn() {
    let app = new_app();
    let cpn = kphis_ui_component::document::DocumentCpn::new(Mutable::new(Some(String::from("20221231235959"))), Mutable::new(Some(Rc::new(kphis_model::patient_info::PatientInfo::demo()))));
    let dom = kphis_ui_component::document::DocumentCpn::render(cpn, app);
    replace_body(dom).await;
}

#[wasm_bindgen_test]
async fn test_emr_cpn() {
    let app = new_app();
    let cpn = kphis_ui_component::emr::EmrCpn::new(Mutable::new(String::from("0001234")));
    let dom = kphis_ui_component::emr::EmrCpn::render("1", cpn, app);
    replace_body(dom).await;
}
#[wasm_bindgen_test]
async fn test_emr_cpn_visit() {
    let app = new_app();
    let cpn = kphis_ui_component::emr::EmrCpn::new(Mutable::new(String::from("0001234")));
    let dom = kphis_ui_component::emr::EmrCpn::render_visit("1", &Rc::new(kphis_model::emr::EmrVisit::demo()), cpn, app);
    replace_body(dom).await;
}

#[wasm_bindgen_test]
async fn test_index_plan_cpn_doctor() {
    let app = new_app();
    let cpn = kphis_ui_component::index_plan::IndexPlanCpn::new(Mutable::new(Some(Rc::new(kphis_model::patient_info::PatientInfo::demo()))), Mutable::new(String::from("doctor")));
    let dom = kphis_ui_component::index_plan::IndexPlanCpn::render(cpn, app);
    replace_body(dom).await;
}
#[wasm_bindgen_test]
async fn test_index_plan_cpn_nurse() {
    let app = new_app();
    let cpn = kphis_ui_component::index_plan::IndexPlanCpn::new(Mutable::new(Some(Rc::new(kphis_model::patient_info::PatientInfo::demo()))), Mutable::new(String::from("nurse")));
    let dom = kphis_ui_component::index_plan::IndexPlanCpn::render(cpn, app);
    replace_body(dom).await;
}
#[wasm_bindgen_test]
async fn test_index_plan_cpn_pharmacist() {
    let app = new_app();
    let cpn = kphis_ui_component::index_plan::IndexPlanCpn::new(Mutable::new(Some(Rc::new(kphis_model::patient_info::PatientInfo::demo()))), Mutable::new(String::from("pharmacist")));
    let dom = kphis_ui_component::index_plan::IndexPlanCpn::render(cpn, app);
    replace_body(dom).await;
}
#[wasm_bindgen_test]
async fn test_index_plan_cpn_other() {
    let app = new_app();
    let cpn = kphis_ui_component::index_plan::IndexPlanCpn::new(Mutable::new(Some(Rc::new(kphis_model::patient_info::PatientInfo::demo()))), Mutable::new(String::from("other")));
    let dom = kphis_ui_component::index_plan::IndexPlanCpn::render(cpn, app);
    replace_body(dom).await;
}

#[wasm_bindgen_test]
async fn test_index_plan_cpn_plan_doctor() {
    let app = new_app();
    let dom = kphis_ui_component::index_plan::render_index_plan(
        Rc::new(kphis_model::order::OrderItem::demo()),
        Mutable::new(None),
        Some(Mutable::new(Some(Rc::new(kphis_model::patient_info::PatientInfo::demo())))),
        Mutable::new(String::from("doctor")),
        Mutable::new(false),
        None,
        app,
    );
    replace_body(dom).await;
}
#[wasm_bindgen_test]
async fn test_index_plan_cpn_plan_nurse() {
    let app = new_app();
    let dom = kphis_ui_component::index_plan::render_index_plan(
        Rc::new(kphis_model::order::OrderItem::demo()),
        Mutable::new(None),
        Some(Mutable::new(Some(Rc::new(kphis_model::patient_info::PatientInfo::demo())))),
        Mutable::new(String::from("nurse")),
        Mutable::new(false),
        None,
        app,
    );
    replace_body(dom).await;
}
#[wasm_bindgen_test]
async fn test_index_plan_cpn_plan_pharmacist() {
    let app = new_app();
    let dom = kphis_ui_component::index_plan::render_index_plan(
        Rc::new(kphis_model::order::OrderItem::demo()),
        Mutable::new(None),
        Some(Mutable::new(Some(Rc::new(kphis_model::patient_info::PatientInfo::demo())))),
        Mutable::new(String::from("pharmacist")),
        Mutable::new(false),
        None,
        app,
    );
    replace_body(dom).await;
}
#[wasm_bindgen_test]
async fn test_index_plan_cpn_plan_other() {
    let app = new_app();
    let dom = kphis_ui_component::index_plan::render_index_plan(
        Rc::new(kphis_model::order::OrderItem::demo()),
        Mutable::new(None),
        Some(Mutable::new(Some(Rc::new(kphis_model::patient_info::PatientInfo::demo())))),
        Mutable::new(String::from("other")),
        Mutable::new(false),
        None,
        app,
    );
    replace_body(dom).await;
}

#[wasm_bindgen_test]
async fn test_io_cpn() {
    let app = new_app();
    let cpn = kphis_ui_component::io::IoCpn::new(Mutable::new(Some(Rc::new(kphis_model::patient_info::PatientInfo::demo()))));
    let dom = kphis_ui_component::io::IoCpn::render(cpn, app);
    replace_body(dom).await;
}

#[wasm_bindgen_test]
async fn test_ipd_consult_cpn() {
    let app = new_app();
    let cpn = kphis_ui_component::ipd_consult::IpdConsultCpn::new(Mutable::new(Some(Rc::new(kphis_model::patient_info::PatientInfo::demo()))), Mutable::new(String::new()), Mutable::new(1));
    let dom = kphis_ui_component::ipd_consult::IpdConsultCpn::render(cpn, app);
    replace_body(dom).await;
}
#[wasm_bindgen_test]
async fn test_ipd_consult_cpn_consult() {
    let app = new_app();
    let cpn = kphis_ui_component::ipd_consult::IpdConsultCpn::new(Mutable::new(Some(Rc::new(kphis_model::patient_info::PatientInfo::demo()))), Mutable::new(String::new()), Mutable::new(1));
    let dom = kphis_ui_component::ipd_consult::IpdConsultCpn::render_consult(Rc::new(kphis_model::ipd::consult::ConsultWithName::demo()), cpn, app);
    replace_body(dom).await;
}

#[wasm_bindgen_test]
async fn test_ipd_pre_order_cpn() {
    let app = new_app();
    let cpn = kphis_ui_component::ipd_pre_order::IpdPreOrderCpn::new(Mutable::new(1));
    let dom = kphis_ui_component::ipd_pre_order::IpdPreOrderCpn::render(Mutable::new(String::from("Y")), cpn, app);
    replace_body(dom).await;
}
#[wasm_bindgen_test]
async fn test_ipd_pre_order_cpn_order_oneday() {
    let app = new_app();
    let cpn = kphis_ui_component::ipd_pre_order::IpdPreOrderCpn::new(Mutable::new(1));
    let dom = kphis_ui_component::ipd_pre_order::render_order(Rc::new(kphis_model::pre_order::order::PreOrder::demo()), true, Mutable::new(String::from("Y")), Some(cpn), app);
    replace_body(dom).await;
}
#[wasm_bindgen_test]
async fn test_ipd_pre_order_cpn_order_cont() {
    let app = new_app();
    let cpn = kphis_ui_component::ipd_pre_order::IpdPreOrderCpn::new(Mutable::new(1));
    let dom = kphis_ui_component::ipd_pre_order::render_order(Rc::new(kphis_model::pre_order::order::PreOrder::demo()), false, Mutable::new(String::from("Y")), Some(cpn), app);
    replace_body(dom).await;
}
#[wasm_bindgen_test]
async fn test_ipd_pre_order_cpn_progress_oneday() {
    let app = new_app();
    let cpn = kphis_ui_component::ipd_pre_order::IpdPreOrderCpn::new(Mutable::new(1));
    let dom = kphis_ui_component::ipd_pre_order::render_progress_note(Rc::new(kphis_model::pre_order::progress_note::PreProgressNote::demo()), Some(cpn), Mutable::new(String::from("Y")), app);
    replace_body(dom).await;
}
#[wasm_bindgen_test]
async fn test_ipd_pre_order_cpn_progress_cont() {
    let app = new_app();
    let cpn = kphis_ui_component::ipd_pre_order::IpdPreOrderCpn::new(Mutable::new(1));
    let dom = kphis_ui_component::ipd_pre_order::render_progress_note(Rc::new(kphis_model::pre_order::progress_note::PreProgressNote::demo()), Some(cpn), Mutable::new(String::from("Y")), app);
    replace_body(dom).await;
}

#[wasm_bindgen_test]
async fn test_lab_cpn() {
    let app = new_app();
    let cpn = kphis_ui_component::lab::LabCpn::new(
        Mutable::new(Some(Rc::new(kphis_model::patient_info::PatientInfo::demo()))),
        Mutable::new(String::from("0001234")),
        Mutable::new(String::from("660001234")),
        None,
    );
    let dom = kphis_ui_component::lab::LabCpn::render("1", cpn, app);
    replace_body(dom).await;
}
#[wasm_bindgen_test]
async fn test_lab_cpn_detail() {
    let app = new_app();
    let cpn = kphis_ui_component::lab::LabCpn::new(
        Mutable::new(Some(Rc::new(kphis_model::patient_info::PatientInfo::demo()))),
        Mutable::new(String::from("0001234")),
        Mutable::new(String::from("660001234")),
        None,
    );
    let dom = kphis_ui_component::lab::LabCpn::render_detail("1", Rc::new(kphis_model::lab::LabHead::demo()), cpn, app);
    replace_body(dom).await;
}

#[wasm_bindgen_test]
async fn test_login_cpn() {
    let app = new_app();
    let cpn = kphis_ui_component::login::LoginCpn::new(Mutable::new(false), Mutable::new(None));
    let dom = kphis_ui_component::login::LoginCpn::render(cpn, app);
    replace_body(dom).await;
}

#[wasm_bindgen_test]
async fn test_menu_cpn() {
    let app = new_app();
    let cpn = kphis_ui_component::menu::MenuCpn::new();
    let dom = kphis_ui_component::menu::MenuCpn::render(cpn, app);
    replace_body(dom).await;
}

#[wasm_bindgen_test]
async fn test_opd_er_emergency_cpn_doctor() {
    let app = new_app();
    let cpn = kphis_ui_component::opd_er_emergency::OpdErEmergencyCpn::new(Mutable::new(Some(Rc::new(kphis_model::patient_info::PatientInfo::demo()))), Mutable::new(String::from("doctor")));
    let dom = kphis_ui_component::opd_er_emergency::OpdErEmergencyCpn::render(cpn, app);
    replace_body(dom).await;
}
#[wasm_bindgen_test]
async fn test_opd_er_emergency_cpn_nurse() {
    let app = new_app();
    let cpn = kphis_ui_component::opd_er_emergency::OpdErEmergencyCpn::new(Mutable::new(Some(Rc::new(kphis_model::patient_info::PatientInfo::demo()))), Mutable::new(String::from("nurse")));
    let dom = kphis_ui_component::opd_er_emergency::OpdErEmergencyCpn::render(cpn, app);
    replace_body(dom).await;
}
#[wasm_bindgen_test]
async fn test_opd_er_emergency_cpn_pharmacist() {
    let app = new_app();
    let cpn = kphis_ui_component::opd_er_emergency::OpdErEmergencyCpn::new(Mutable::new(Some(Rc::new(kphis_model::patient_info::PatientInfo::demo()))), Mutable::new(String::from("pharmacist")));
    let dom = kphis_ui_component::opd_er_emergency::OpdErEmergencyCpn::render(cpn, app);
    replace_body(dom).await;
}
#[wasm_bindgen_test]
async fn test_opd_er_emergency_cpn_other() {
    let app = new_app();
    let cpn = kphis_ui_component::opd_er_emergency::OpdErEmergencyCpn::new(Mutable::new(Some(Rc::new(kphis_model::patient_info::PatientInfo::demo()))), Mutable::new(String::from("other")));
    let dom = kphis_ui_component::opd_er_emergency::OpdErEmergencyCpn::render(cpn, app);
    replace_body(dom).await;
}
#[wasm_bindgen_test]
async fn test_opd_er_emergency_cpn_pe() {
    let dom = kphis_ui_component::opd_er_emergency::OpdErEmergencyCpn::render_pe_hosxp(&kphis_model::opd_er::medical_history::OpdScreenHistory::demo());
    replace_body(dom).await;
}
#[wasm_bindgen_test]
async fn test_opd_er_emergency_cpn_triage() {
    let dom = kphis_ui_component::opd_er_emergency::OpdErEmergencyCpn::render_triage(&kphis_model::opd_er::medical_history::OpdScreenHistory::demo());
    replace_body(dom).await;
}
#[wasm_bindgen_test]
async fn test_opd_er_emergency_cpn_drug_allergy() {
    let item = kphis_ui_component::opd_er_emergency::DrugAllergy::new();
    let cpn = kphis_ui_component::opd_er_emergency::OpdErEmergencyCpn::new(Mutable::new(Some(Rc::new(kphis_model::patient_info::PatientInfo::demo()))), Mutable::new(String::from("doctor")));
    let dom = kphis_ui_component::opd_er_emergency::DrugAllergy::render(item, cpn);
    replace_body(dom).await;
}
#[wasm_bindgen_test]
async fn test_opd_er_emergency_cpn_consult() {
    let app = new_app();
    let item = kphis_ui_component::opd_er_emergency::ConsultItem::new();
    let cpn = kphis_ui_component::opd_er_emergency::OpdErEmergencyCpn::new(Mutable::new(Some(Rc::new(kphis_model::patient_info::PatientInfo::demo()))), Mutable::new(String::from("doctor")));
    let dom = kphis_ui_component::opd_er_emergency::ConsultItem::render(item, cpn, app);
    replace_body(dom).await;
}

#[wasm_bindgen_test]
async fn test_opd_er_medical_history_cpn() {
    let app = new_app();
    let cpn = kphis_ui_component::opd_er_medical_history::OpdErMedicalHistoryCpn::new(Mutable::new(Some(Rc::new(kphis_model::patient_info::PatientInfo::demo()))), None);
    let dom = kphis_ui_component::opd_er_medical_history::OpdErMedicalHistoryCpn::render(cpn, Some(Mutable::new(true)), app);
    replace_body(dom).await;
}

#[wasm_bindgen_test]
async fn test_order_cpn_ipd_doctor() {
    let app = new_app();
    let cpn = kphis_ui_component::order::OrderCpn::new(
        true,
        Mutable::new(Some(Rc::new(kphis_model::patient_info::PatientInfo::demo()))),
        Mutable::new(String::from("doctor")),
        Mutable::new(String::from("N")),
        Mutable::new(String::new()),
        Mutable::new(1),
        app.clone(),
    );
    let dom = kphis_ui_component::order::OrderCpn::render("1", cpn, app);
    replace_body(dom).await;
}
#[wasm_bindgen_test]
async fn test_order_cpn_ipd_nurse() {
    let app = new_app();
    let cpn = kphis_ui_component::order::OrderCpn::new(
        true,
        Mutable::new(Some(Rc::new(kphis_model::patient_info::PatientInfo::demo()))),
        Mutable::new(String::from("nurse")),
        Mutable::new(String::from("N")),
        Mutable::new(String::new()),
        Mutable::new(1),
        app.clone(),
    );
    let dom = kphis_ui_component::order::OrderCpn::render("1", cpn, app);
    replace_body(dom).await;
}
#[wasm_bindgen_test]
async fn test_order_cpn_ipd_pharmacist() {
    let app = new_app();
    let cpn = kphis_ui_component::order::OrderCpn::new(
        true,
        Mutable::new(Some(Rc::new(kphis_model::patient_info::PatientInfo::demo()))),
        Mutable::new(String::from("pharmacist")),
        Mutable::new(String::from("N")),
        Mutable::new(String::new()),
        Mutable::new(1),
        app.clone(),
    );
    let dom = kphis_ui_component::order::OrderCpn::render("1", cpn, app);
    replace_body(dom).await;
}
#[wasm_bindgen_test]
async fn test_order_cpn_ipd_other() {
    let app = new_app();
    let cpn = kphis_ui_component::order::OrderCpn::new(
        true,
        Mutable::new(Some(Rc::new(kphis_model::patient_info::PatientInfo::demo()))),
        Mutable::new(String::from("other")),
        Mutable::new(String::from("N")),
        Mutable::new(String::new()),
        Mutable::new(1),
        app.clone(),
    );
    let dom = kphis_ui_component::order::OrderCpn::render("1", cpn, app);
    replace_body(dom).await;
}
#[wasm_bindgen_test]
async fn test_order_cpn_opd_er_doctor() {
    let app = new_app();
    let cpn = kphis_ui_component::order::OrderCpn::new(
        false,
        Mutable::new(Some(Rc::new(kphis_model::patient_info::PatientInfo::demo()))),
        Mutable::new(String::from("doctor")),
        Mutable::new(String::from("N")),
        Mutable::new(String::new()),
        Mutable::new(1),
        app.clone(),
    );
    let dom = kphis_ui_component::order::OrderCpn::render("1", cpn, app);
    replace_body(dom).await;
}
#[wasm_bindgen_test]
async fn test_order_cpn_opd_er_nurse() {
    let app = new_app();
    let cpn = kphis_ui_component::order::OrderCpn::new(
        false,
        Mutable::new(Some(Rc::new(kphis_model::patient_info::PatientInfo::demo()))),
        Mutable::new(String::from("nurse")),
        Mutable::new(String::from("N")),
        Mutable::new(String::new()),
        Mutable::new(1),
        app.clone(),
    );
    let dom = kphis_ui_component::order::OrderCpn::render("1", cpn, app);
    replace_body(dom).await;
}
#[wasm_bindgen_test]
async fn test_order_cpn_opd_er_pharmacist() {
    let app = new_app();
    let cpn = kphis_ui_component::order::OrderCpn::new(
        false,
        Mutable::new(Some(Rc::new(kphis_model::patient_info::PatientInfo::demo()))),
        Mutable::new(String::from("pharmacist")),
        Mutable::new(String::from("N")),
        Mutable::new(String::new()),
        Mutable::new(1),
        app.clone(),
    );
    let dom = kphis_ui_component::order::OrderCpn::render("1", cpn, app);
    replace_body(dom).await;
}
#[wasm_bindgen_test]
async fn test_order_cpn_opd_er_other() {
    let app = new_app();
    let cpn = kphis_ui_component::order::OrderCpn::new(
        false,
        Mutable::new(Some(Rc::new(kphis_model::patient_info::PatientInfo::demo()))),
        Mutable::new(String::from("other")),
        Mutable::new(String::from("N")),
        Mutable::new(String::new()),
        Mutable::new(1),
        app.clone(),
    );
    let dom = kphis_ui_component::order::OrderCpn::render("1", cpn, app);
    replace_body(dom).await;
}
#[wasm_bindgen_test]
async fn test_order_cpn_order_oneday() {
    let app = new_app();
    let cpn = kphis_ui_component::order::OrderCpn::new(
        true,
        Mutable::new(Some(Rc::new(kphis_model::patient_info::PatientInfo::demo()))),
        Mutable::new(String::from("nurse")),
        Mutable::new(String::from("N")),
        Mutable::new(String::new()),
        Mutable::new(1),
        app.clone(),
    );
    let dom = kphis_ui_component::order::OrderCpn::render_order("1", Rc::new(kphis_model::order::Order::demo()), true, cpn, app);
    replace_body(dom).await;
}
#[wasm_bindgen_test]
async fn test_order_cpn_order_cont() {
    let app = new_app();
    let cpn = kphis_ui_component::order::OrderCpn::new(
        true,
        Mutable::new(Some(Rc::new(kphis_model::patient_info::PatientInfo::demo()))),
        Mutable::new(String::from("nurse")),
        Mutable::new(String::from("N")),
        Mutable::new(String::new()),
        Mutable::new(1),
        app.clone(),
    );
    let dom = kphis_ui_component::order::OrderCpn::render_order("1", Rc::new(kphis_model::order::Order::demo()), false, cpn, app);
    replace_body(dom).await;
}
#[wasm_bindgen_test]
async fn test_order_cpn_order_prev_oneday() {
    let app = new_app();
    let cpn = kphis_ui_component::order::OrderCpn::new(
        true,
        Mutable::new(Some(Rc::new(kphis_model::patient_info::PatientInfo::demo()))),
        Mutable::new(String::from("nurse")),
        Mutable::new(String::from("N")),
        Mutable::new(String::new()),
        Mutable::new(1),
        app.clone(),
    );
    let dom = kphis_ui_component::order::OrderCpn::render_previous_order(&kphis_model::order::OrderItem::demo(), kphis_ui_component::modal::index_plan_action_form::OrderType::OneDay, cpn, app);
    replace_body(dom).await;
}
#[wasm_bindgen_test]
async fn test_order_cpn_order_prev_cont() {
    let app = new_app();
    let cpn = kphis_ui_component::order::OrderCpn::new(
        true,
        Mutable::new(Some(Rc::new(kphis_model::patient_info::PatientInfo::demo()))),
        Mutable::new(String::from("nurse")),
        Mutable::new(String::from("N")),
        Mutable::new(String::new()),
        Mutable::new(1),
        app.clone(),
    );
    let dom = kphis_ui_component::order::OrderCpn::render_previous_order(&kphis_model::order::OrderItem::demo(), kphis_ui_component::modal::index_plan_action_form::OrderType::Continuous, cpn, app);
    replace_body(dom).await;
}
#[wasm_bindgen_test]
async fn test_order_cpn_order_medrec() {
    let app = new_app();
    let dom = kphis_ui_component::order::OrderCpn::render_med_rec(&Rc::new(kphis_model::med_reconcile::MedReconciliationItem::demo()), app);
    replace_body(dom).await;
}
#[wasm_bindgen_test]
async fn test_order_cpn_progress() {
    let app = new_app();
    let cpn = kphis_ui_component::order::OrderCpn::new(
        true,
        Mutable::new(Some(Rc::new(kphis_model::patient_info::PatientInfo::demo()))),
        Mutable::new(String::from("nurse")),
        Mutable::new(String::from("N")),
        Mutable::new(String::new()),
        Mutable::new(1),
        app.clone(),
    );
    let dom = kphis_ui_component::order::OrderCpn::render_progress_note("1", Rc::new(kphis_model::progress_note::ProgressNote::demo()), cpn, app);
    replace_body(dom).await;
}
#[wasm_bindgen_test]
async fn test_order_cpn_opdmed() {
    let dom = kphis_ui_component::order::OrderCpn::render_opd_med(1, Rc::new(kphis_model::opd_er::hosxp_med::OpdMed::demo()));
    replace_body(dom).await;
}
#[wasm_bindgen_test]
async fn test_order_cpn_textarea() {
    let app = new_app();
    let cpn = kphis_ui_component::order::InsertTextAreaButton::from_button(
        "med",
        kphis_model::order::Button {
            is_new: true,
            separator: String::from(", "),
            word: String::from("HCT"),
            minus_from_end: 0,
            id: Some(String::from("1234")),
        },
    );
    let dom = kphis_ui_component::order::InsertTextAreaButton::render(
        Rc::new(cpn),
        MutableVec::new_with_values(vec![Rc::new(kphis_ui_component::order::OrderItemMutable::default())]),
        Mutable::new(Some(1)),
        Mutable::new(false),
        app,
    );
    replace_body(dom).await;
}
#[wasm_bindgen_test]
async fn test_order_cpn_textarea_adder() {
    let app = new_app();
    let cpn = kphis_ui_component::order::InsertTextAreaButton::from_button(
        "med",
        kphis_model::order::Button {
            is_new: true,
            separator: String::from(", "),
            word: String::from("HCT"),
            minus_from_end: 0,
            id: Some(String::from("1234")),
        },
    );
    let dom = kphis_ui_component::order::InsertTextAreaButton::render_maybe_adder(
        Rc::new(cpn),
        MutableVec::new_with_values(vec![Rc::new(kphis_ui_component::order::OrderItemMutable::default())]),
        MutableVec::new_with_values(vec![Rc::new(kphis_ui_component::order::OrderItemMutable::default())]),
        Mutable::new(Some(1)),
        Mutable::new(false),
        Some(1),
        app,
    );
    replace_body(dom).await;
}

#[wasm_bindgen_test]
async fn test_refer_out_cpn() {
    let app = new_app();
    let cpn = kphis_ui_component::refer_out::ReferOutCpn::new(Mutable::new(Some(Rc::new(kphis_model::patient_info::PatientInfo::demo()))));
    let dom = kphis_ui_component::refer_out::ReferOutCpn::render(cpn, app);
    replace_body(dom).await;
}
#[wasm_bindgen_test]
async fn test_refer_out_cpn_out() {
    let app = new_app();
    let cpn = kphis_ui_component::refer_out::ReferOutCpn::new(Mutable::new(Some(Rc::new(kphis_model::patient_info::PatientInfo::demo()))));
    let dom = kphis_ui_component::refer_out::ReferOutCpn::render_refer_out(cpn, app);
    replace_body(dom).await;
}
#[wasm_bindgen_test]
async fn test_refer_out_cpn_note() {
    let app = new_app();
    let cpn = kphis_ui_component::refer_out::ReferOutCpn::new(Mutable::new(Some(Rc::new(kphis_model::patient_info::PatientInfo::demo()))));
    let dom = kphis_ui_component::refer_out::ReferOutCpn::render_refer_note(cpn, app);
    replace_body(dom).await;
}

#[wasm_bindgen_test]
async fn test_show_patient_main_cpn_an() {
    let app = new_app();
    let cpn = kphis_ui_component::show_patient_main::ShowPatientMainCpn::new_with_an(String::from("660001234"));
    let dom = kphis_ui_component::show_patient_main::ShowPatientMainCpn::render(false, cpn, app);
    replace_body(dom).await;
}
#[wasm_bindgen_test]
async fn test_show_patient_main_cpn_id() {
    let app = new_app();
    let cpn = kphis_ui_component::show_patient_main::ShowPatientMainCpn::new_with_id(1);
    let dom = kphis_ui_component::show_patient_main::ShowPatientMainCpn::render(false, cpn, app);
    replace_body(dom).await;
}
#[wasm_bindgen_test]
async fn test_show_patient_main_cpn_vn_compact() {
    let app = new_app();
    let cpn = kphis_ui_component::show_patient_main::ShowPatientMainCpn::new_with_vn("20221231235959");
    let dom = kphis_ui_component::show_patient_main::ShowPatientMainCpn::render(true, cpn, app);
    replace_body(dom).await;
}
#[wasm_bindgen_test]
async fn test_show_patient_main_cpn_info() {
    let app = new_app();
    let cpn = kphis_ui_component::show_patient_main::ShowPatientMainCpn::new_with_an(String::from("660001234"));
    let dom = kphis_ui_component::show_patient_main::render_patient_info(
        false,
        Rc::new(kphis_model::patient_info::PatientInfo::demo()),
        Some(kphis_ui_component::show_patient_main::ShowPatientMainCpn::render_allergy(cpn.clone(), app.clone())),
        Some(cpn),
        false,
        app,
    );
    replace_body(dom).await;
}
#[wasm_bindgen_test]
async fn test_show_patient_main_cpn_info_mini() {
    let app = new_app();
    let cpn = kphis_ui_component::show_patient_main::ShowPatientMainCpn::new_with_an(String::from("660001234"));
    let dom = kphis_ui_component::show_patient_main::render_patient_info(
        false,
        Rc::new(kphis_model::patient_info::PatientInfo::demo()),
        Some(kphis_ui_component::show_patient_main::ShowPatientMainCpn::render_allergy(cpn.clone(), app.clone())),
        Some(cpn),
        true,
        app,
    );
    replace_body(dom).await;
}

#[wasm_bindgen_test]
async fn test_summary_note_cpn() {
    let app = new_app();
    let cpn = kphis_ui_component::summary_note::SummaryNoteCpn::new(1);
    let dom = kphis_ui_component::summary_note::SummaryNoteCpn::render(false, cpn, app);
    replace_body(dom).await;
}
#[wasm_bindgen_test]
async fn test_summary_note_cpn_pre() {
    let app = new_app();
    let cpn = kphis_ui_component::summary_note::SummaryNoteCpn::new(1);
    let dom = kphis_ui_component::summary_note::SummaryNoteCpn::render(true, cpn, app);
    replace_body(dom).await;
}
#[wasm_bindgen_test]
async fn test_summary_note_cpn_lab() {
    let app = new_app();
    let dom = kphis_ui_component::summary_note::render_lab_alert(
        MutableVec::new_with_values(vec![Rc::new(kphis_model::ipd::summary::LabAlertData::demo())]),
        Mutable::new(String::from("0001234")),
        Mutable::new(None),
        app,
    );
    replace_body(dom).await;
}
#[wasm_bindgen_test]
async fn test_summary_note_cpn_problem() {
    let dom = kphis_ui_component::summary_note::render_problem_list(MutableVec::new_with_values(vec![String::from("problem")]));
    replace_body(dom).await;
}

#[wasm_bindgen_test]
async fn test_xray_cpn_ipd() {
    let app = new_app();
    let cpn = kphis_ui_component::xray::XrayCpn::new_ipd(Mutable::new(String::from("0001234")), Mutable::new(String::from("660001234")), None);
    let dom = kphis_ui_component::xray::XrayCpn::render("1", cpn, app);
    replace_body(dom).await;
}
#[wasm_bindgen_test]
async fn test_xray_cpn_opd_er() {
    let app = new_app();
    let cpn = kphis_ui_component::xray::XrayCpn::new_opd_er(Mutable::new(String::from("0001234")), Mutable::new(String::from("20221231235959")), None);
    let dom = kphis_ui_component::xray::XrayCpn::render("1", cpn, app);
    replace_body(dom).await;
}
#[wasm_bindgen_test]
async fn test_xray_cpn_ipd_report() {
    let app = new_app();
    let cpn = kphis_ui_component::xray::XrayCpn::new_ipd(Mutable::new(String::from("0001234")), Mutable::new(String::from("660001234")), None);
    let dom = kphis_ui_component::xray::XrayCpn::render_report("1", Rc::new(kphis_model::xray::XrayReport::demo()), cpn, app);
    replace_body(dom).await;
}
