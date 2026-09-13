mod index;
mod info;

mod drug_use_duration;
mod image;
mod index_plan;
mod ipd_admission_note_dr;
mod ipd_admission_note_nurse;
mod ipd_consult_list;
mod ipd_main;
mod ipd_mra;
mod ipd_order_pharmacy;
mod ipd_post_admit_list;
mod ipd_pre_admit_list;
mod ipd_pre_order_list;
mod ipd_pre_order_main;
mod ipd_search_patient_dr;
mod ipd_search_patient_nurse;
mod ipd_search_patient_other;
mod ipd_search_patient_pharmacist;
mod ipd_summary_audit;

mod not_found;

mod opd_er_main;
mod opd_er_order_list;
mod opd_er_order_pharmacy;

mod permission_list;
mod prescription_screen;

mod report_designer;

mod report_viewer;
mod setting_template_dc_plan;
mod setting_template_nurse_note;
mod summary;

mod user_list;

mod vital_sign;
mod unauthorized;

use dominator::Dom;
use futures_signals::{signal::Mutable, signal_vec::MutableVec};
use std::rc::Rc;
use wasm_bindgen::JsValue;
use wasm_bindgen_futures::JsFuture;

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
    dominator::replace_dom(
        &dominator::body(),
        &dominator::body().first_child().unwrap(),
        dom,
    );
    // move to next tick
    JsFuture::from(js_sys::Promise::resolve(&JsValue::null())).await.unwrap();
}
