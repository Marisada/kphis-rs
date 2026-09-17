use dominator::Dom;
use futures_signals::{signal::Mutable, signal_vec::MutableVec};
use std::rc::Rc;
use wasm_bindgen_test::wasm_bindgen_test;

use crate::{new_app, replace_body};

#[wasm_bindgen_test]
async fn test_dc_plan_cpn() {
    let app = new_app();
    let cpn = kphis_ui_component::nurse_note::dc_plan::DcPlanCpn::new(Mutable::new(Some(Rc::new(kphis_model::patient_info::PatientInfo::demo()))));
    let dom = kphis_ui_component::nurse_note::dc_plan::DcPlanCpn::render(cpn, app);
    replace_body(dom).await;
}

#[wasm_bindgen_test]
async fn test_focus_list_cpn() {
    let app = new_app();
    let cpn = kphis_ui_component::nurse_note::focus_list::FocusListCpn::new(Mutable::new(Some(Rc::new(kphis_model::patient_info::PatientInfo::demo()))));
    let dom = kphis_ui_component::nurse_note::focus_list::FocusListCpn::render(cpn, app);
    replace_body(dom).await;
}
#[wasm_bindgen_test]
async fn test_focus_list_cpn_form() {
    let app = new_app();
    let cpn = kphis_ui_component::nurse_note::focus_list::FocusListCpn::new(Mutable::new(Some(Rc::new(kphis_model::patient_info::PatientInfo::demo()))));
    let dom = kphis_ui_component::nurse_note::focus_list::FocusListCpn::render_form(cpn, app);
    replace_body(dom).await;
}

#[wasm_bindgen_test]
async fn test_focus_list_row_cpn() {
    let dom = kphis_ui_component::nurse_note::focus_list_row::render(1, Rc::new(kphis_model::focus_list::FocusList::demo()), Dom::empty());
    replace_body(dom).await;
}

#[wasm_bindgen_test]
async fn test_focus_note_cpn() {
    let app = new_app();
    let cpn = kphis_ui_component::nurse_note::focus_note::FocusNoteCpn::new(Mutable::new(Some(Rc::new(kphis_model::patient_info::PatientInfo::demo()))));
    let dom = kphis_ui_component::nurse_note::focus_note::FocusNoteCpn::render(cpn, app);
    replace_body(dom).await;
}

#[wasm_bindgen_test]
async fn test_focus_note_row_cpn() {
    let app = new_app();
    let dom = kphis_ui_component::nurse_note::focus_note_row::render(
        1,
        Rc::new((kphis_model::app::VisitTypeId::Ipd(String::from("660001234")), Rc::new(kphis_model::focus_note::FocusNote::demo()))),
        None,
        MutableVec::new_with_values(vec![Rc::new(kphis_model::focus_list::FocusList::demo())]),
        false,
        app,
    );
    replace_body(dom).await;
}
#[wasm_bindgen_test]
async fn test_focus_note_row_cpn_editable() {
    let app = new_app();
    let dom = kphis_ui_component::nurse_note::focus_note_row::render(
        1,
        Rc::new((kphis_model::app::VisitTypeId::Ipd(String::from("660001234")), Rc::new(kphis_model::focus_note::FocusNote::demo()))),
        None,
        MutableVec::new_with_values(vec![Rc::new(kphis_model::focus_list::FocusList::demo())]),
        true,
        app,
    );
    replace_body(dom).await;
}

#[wasm_bindgen_test]
async fn test_note_form_cpn() {
    let app = new_app();
    let cpn = kphis_ui_component::nurse_note::note_form::NurseNoteFormCpn::new(
        MutableVec::new_with_values(vec![Rc::new(kphis_model::focus_list::FocusList::demo())]),
        Mutable::new(vec![kphis_model::ipd::tmp::TmpDlc::demo()]),
        Mutable::new(false),
        Mutable::new(Some(Rc::new(kphis_model::patient_info::PatientInfo::demo()))),
        Mutable::new(None),
        Mutable::new(false),
    );
    let dom = kphis_ui_component::nurse_note::note_form::NurseNoteFormCpn::render(cpn, app);
    replace_body(dom).await;
}

#[wasm_bindgen_test]
async fn test_note_list_cpn() {
    let app = new_app();
    let cpn = kphis_ui_component::nurse_note::note_list::NoteListCpn::new(
        MutableVec::new_with_values(vec![Rc::new(kphis_model::focus_list::FocusList::demo())]),
        Mutable::new(false),
        Mutable::new(Some(Rc::new(kphis_model::patient_info::PatientInfo::demo()))),
        Mutable::new(false),
    );
    let dom = kphis_ui_component::nurse_note::note_list::NoteListCpn::render(cpn, app, None);
    replace_body(dom).await;
}

#[wasm_bindgen_test]
async fn test_nurse_note_main_cpn_doctor() {
    let app = new_app();
    let cpn = kphis_ui_component::nurse_note::nurse_note_main::NurseNoteCpn::new(Mutable::new(Some(Rc::new(kphis_model::patient_info::PatientInfo::demo()))), Mutable::new(String::from("doctor")));
    let dom = kphis_ui_component::nurse_note::nurse_note_main::NurseNoteCpn::render(cpn, app);
    replace_body(dom).await;
}
#[wasm_bindgen_test]
async fn test_nurse_note_main_cpn_nurse() {
    let app = new_app();
    let cpn = kphis_ui_component::nurse_note::nurse_note_main::NurseNoteCpn::new(Mutable::new(Some(Rc::new(kphis_model::patient_info::PatientInfo::demo()))), Mutable::new(String::from("nurse")));
    let dom = kphis_ui_component::nurse_note::nurse_note_main::NurseNoteCpn::render(cpn, app);
    replace_body(dom).await;
}
#[wasm_bindgen_test]
async fn test_nurse_note_main_cpn_pharmacist() {
    let app = new_app();
    let cpn = kphis_ui_component::nurse_note::nurse_note_main::NurseNoteCpn::new(Mutable::new(Some(Rc::new(kphis_model::patient_info::PatientInfo::demo()))), Mutable::new(String::from("pharmacist")));
    let dom = kphis_ui_component::nurse_note::nurse_note_main::NurseNoteCpn::render(cpn, app);
    replace_body(dom).await;
}
#[wasm_bindgen_test]
async fn test_nurse_note_main_cpn_other() {
    let app = new_app();
    let cpn = kphis_ui_component::nurse_note::nurse_note_main::NurseNoteCpn::new(Mutable::new(Some(Rc::new(kphis_model::patient_info::PatientInfo::demo()))), Mutable::new(String::from("other")));
    let dom = kphis_ui_component::nurse_note::nurse_note_main::NurseNoteCpn::render(cpn, app);
    replace_body(dom).await;
}
