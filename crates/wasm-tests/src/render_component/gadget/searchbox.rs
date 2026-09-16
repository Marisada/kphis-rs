use futures_signals::{signal::Mutable, signal_vec::MutableVec};
use std::{collections::BTreeMap, rc::Rc, sync::Arc};
use wasm_bindgen_test::wasm_bindgen_test;

use crate::{new_app, replace_body};

#[wasm_bindgen_test]
async fn test_dx_searchbox_cpn() {
    let app = new_app();
    let cpn = kphis_ui_component::gadget::searchbox::dx::DxSearchboxCpn::new(true, false, Mutable::new(Some(Rc::new(kphis_model::search::searchbox::Icd10::demo()))), kphis_ui_app::DaggerAsteriskState::new());
    let dom = kphis_ui_component::gadget::searchbox::dx::DxSearchboxCpn::render(cpn, app, None, Mutable::new(false), false);
    replace_body(dom).await;
}
#[wasm_bindgen_test]
async fn test_dx_searchbox_cpn_full() {
    let app = new_app();
    let cpn = kphis_ui_component::gadget::searchbox::dx::DxSearchboxCpn::new(true, true, Mutable::new(Some(Rc::new(kphis_model::search::searchbox::Icd10::demo()))), kphis_ui_app::DaggerAsteriskState::new());
    let dom = kphis_ui_component::gadget::searchbox::dx::DxSearchboxCpn::render(cpn, app, None, Mutable::new(false), false);
    replace_body(dom).await;
}
#[wasm_bindgen_test]
async fn test_dx_searchbox_cpn_enabled() {
    let app = new_app();
    let cpn = kphis_ui_component::gadget::searchbox::dx::DxSearchboxCpn::new(true, false, Mutable::new(Some(Rc::new(kphis_model::search::searchbox::Icd10::demo()))), kphis_ui_app::DaggerAsteriskState::new());
    let dom = kphis_ui_component::gadget::searchbox::dx::DxSearchboxCpn::render(cpn, app, None, Mutable::new(false), true);
    replace_body(dom).await;
}
#[wasm_bindgen_test]
async fn test_dx_searchbox_cpn_full_enabled() {
    let app = new_app();
    let cpn = kphis_ui_component::gadget::searchbox::dx::DxSearchboxCpn::new(true, true, Mutable::new(Some(Rc::new(kphis_model::search::searchbox::Icd10::demo()))), kphis_ui_app::DaggerAsteriskState::new());
    let dom = kphis_ui_component::gadget::searchbox::dx::DxSearchboxCpn::render(cpn, app, None, Mutable::new(false), true);
    replace_body(dom).await;
}
#[wasm_bindgen_test]
async fn test_dx_searchbox_cpn_search_head() {
    let cpn = kphis_ui_component::gadget::searchbox::dx::DxSearchboxCpn::new(false, true, Mutable::new(Some(Rc::new(kphis_model::search::searchbox::Icd10::demo()))), kphis_ui_app::DaggerAsteriskState::new());
    let dom = kphis_ui_component::gadget::searchbox::dx::DxSearchboxCpn::render_search_header(cpn, false);
    replace_body(dom).await;
}
#[wasm_bindgen_test]
async fn test_dx_searchbox_cpn_search_result() {
    let cpn = kphis_ui_component::gadget::searchbox::dx::DxSearchboxCpn::new(false, true, Mutable::new(Some(Rc::new(kphis_model::search::searchbox::Icd10::demo()))), kphis_ui_app::DaggerAsteriskState::new());
    let dom = kphis_ui_component::gadget::searchbox::dx::DxSearchboxCpn::render_search_result(
        (
            Arc::new(kphis_drg_worker::drg::model::I10vx {
                code: String::from("A09"),
                is_valid: true,
                desc: String::from("desc"),
                is_tm: true,
            }),
            1.0,
            1,
        ),
        cpn,
        None,
    );
    replace_body(dom).await;
}
#[wasm_bindgen_test]
async fn test_dx_searchbox_cpn_index_header() {
    let cpn = kphis_ui_component::gadget::searchbox::dx::DxSearchboxCpn::new(false, true, Mutable::new(Some(Rc::new(kphis_model::search::searchbox::Icd10::demo()))), kphis_ui_app::DaggerAsteriskState::new());
    let dom = kphis_ui_component::gadget::searchbox::dx::DxSearchboxCpn::render_index_header(cpn, false);
    replace_body(dom).await;
}
#[wasm_bindgen_test]
async fn test_dx_searchbox_cpn_index_result() {
    let cpn = kphis_ui_component::gadget::searchbox::dx::DxSearchboxCpn::new(false, true, Mutable::new(Some(Rc::new(kphis_model::search::searchbox::Icd10::demo()))), kphis_ui_app::DaggerAsteriskState::new());
    let dom = kphis_ui_component::gadget::searchbox::dx::DxSearchboxCpn::render_index_result(
        (
            (
                String::from("txt"),
                Arc::new(kphis_drg_worker::i10::index::I10Pointer {
                    note: Some(kphis_drg_worker::i10::index::Note::Code(String::from("A09"))),
                    bracket_notes: vec![kphis_drg_worker::i10::index::Note::Code(String::from("A09"))],
                    code: Some(kphis_drg_worker::i10::index::Code::Single(String::from("B52"))),
                }),
            ),
            1.0,
            1,
        ),
        cpn,
        None,
    );
    replace_body(dom).await;
}
#[wasm_bindgen_test]
async fn test_dx_searchbox_cpn_code_badge() {
    let cpn = kphis_ui_component::gadget::searchbox::dx::DxSearchboxCpn::new(false, true, Mutable::new(Some(Rc::new(kphis_model::search::searchbox::Icd10::demo()))), kphis_ui_app::DaggerAsteriskState::new());
    let dom = kphis_ui_component::gadget::searchbox::dx::DxSearchboxCpn::render_code_badge(String::from("A09"), None, cpn, true);
    replace_body(dom).await;
}
#[wasm_bindgen_test]
async fn test_dx_searchbox_cpn_code_book() {
    let cpn = kphis_ui_component::gadget::searchbox::dx::DxSearchboxCpn::new(false, true, Mutable::new(Some(Rc::new(kphis_model::search::searchbox::Icd10::demo()))), kphis_ui_app::DaggerAsteriskState::new());
    let dom = kphis_ui_component::gadget::searchbox::dx::DxSearchboxCpn::render_code_book(String::from("A09"), cpn);
    replace_body(dom).await;
}
#[wasm_bindgen_test]
async fn test_dx_searchbox_cpn_code_pair() {
    let cpn = kphis_ui_component::gadget::searchbox::dx::DxSearchboxCpn::new(false, true, Mutable::new(Some(Rc::new(kphis_model::search::searchbox::Icd10::demo()))), kphis_ui_app::DaggerAsteriskState::new());
    let dom = kphis_ui_component::gadget::searchbox::dx::DxSearchboxCpn::render_code_pair_detail(true, String::from("A09"), cpn);
    replace_body(dom).await;
}
#[wasm_bindgen_test]
async fn test_dx_searchbox_cpn_pair_wo() {
    let cpn = kphis_ui_component::gadget::searchbox::dx::DxSearchboxCpn::new(false, true, Mutable::new(Some(Rc::new(kphis_model::search::searchbox::Icd10::demo()))), kphis_ui_app::DaggerAsteriskState::new());
    let dom = kphis_ui_component::gadget::searchbox::dx::DxSearchboxCpn::render_pair_detail_without_code(true, true, String::from("A09"), cpn);
    replace_body(dom).await;
}
#[wasm_bindgen_test]
async fn test_dx_searchbox_cpn_i10() {
    let cpn = kphis_ui_component::gadget::searchbox::dx::DxSearchboxCpn::new(false, true, Mutable::new(Some(Rc::new(kphis_model::search::searchbox::Icd10::demo()))), kphis_ui_app::DaggerAsteriskState::new());
    let usage_kind = kphis_drg_worker::i10::claml::UsageKind::Aster;
    let rubric = kphis_drg_worker::i10::claml::Rubric {
        kind: kphis_drg_worker::i10::claml::RubricKind::Preferred,
        text: String::from("text"),
        usage: Some(usage_kind.clone()),
        reference: vec![kphis_drg_worker::i10::claml::Reference {
            label: String::from("A09"),
            code: Some(String::from("A09")),
            usage: Some(kphis_drg_worker::i10::claml::UsageKind::Dagger),
            position: 1,
        }],
    };
    let modifier = kphis_drg_worker::i10::claml::ModifierDetail {
        code: String::from("A09"),
        subclasses: vec![(String::from(".1"), vec![rubric.clone()])],
        rubrics: BTreeMap::new(),
    };
    let detail = kphis_drg_worker::i10::claml::I10Detail {
        code: String::from("A09"),
        usage: Some(usage_kind),
        subclasses: vec![(String::from(".1"), vec![rubric.clone()])],
        superclass: Some(String::from("A")),
        modified_by: Some(modifier.clone()),
        sub_modifier: Some(modifier),
        r_prefered: vec![rubric],
        r_definitions: Vec::new(),
        r_texts: Vec::new(),
        r_inclusions: Vec::new(),
        r_exclusions: Vec::new(),
        r_coding_hints: Vec::new(),
        r_notes: Vec::new(),
        r_foot_notes: Vec::new(),
    };
    let dom = kphis_ui_component::gadget::searchbox::dx::DxSearchboxCpn::render_i10_detail(Arc::new(detail), cpn);
    replace_body(dom).await;
}
#[wasm_bindgen_test]
async fn test_dx_searchbox_cpn_note() {
    let cpn = kphis_ui_component::gadget::searchbox::dx::DxSearchboxCpn::new(false, true, Mutable::new(Some(Rc::new(kphis_model::search::searchbox::Icd10::demo()))), kphis_ui_app::DaggerAsteriskState::new());
    let dom = kphis_ui_component::gadget::searchbox::dx::DxSearchboxCpn::render_note(String::from("note"), cpn);
    replace_body(dom).await;
}

#[wasm_bindgen_test]
async fn test_hosp_searchbox_cpn() {
    let app = new_app();
    let cpn = kphis_ui_component::gadget::searchbox::hosp::HospSearchboxCpn::new(Mutable::new(Some(Rc::new(kphis_model::search::searchbox::HospSearchBox::demo()))));
    let dom = kphis_ui_component::gadget::searchbox::hosp::HospSearchboxCpn::render(cpn, app, Mutable::new(false), false, false);
    replace_body(dom).await;
}
#[wasm_bindgen_test]
async fn test_hosp_searchbox_cpn_savable() {
    let app = new_app();
    let cpn = kphis_ui_component::gadget::searchbox::hosp::HospSearchboxCpn::new(Mutable::new(Some(Rc::new(kphis_model::search::searchbox::HospSearchBox::demo()))));
    let dom = kphis_ui_component::gadget::searchbox::hosp::HospSearchboxCpn::render(cpn, app, Mutable::new(false), true, false);
    replace_body(dom).await;
}
#[wasm_bindgen_test]
async fn test_hosp_searchbox_cpn_em() {
    let app = new_app();
    let cpn = kphis_ui_component::gadget::searchbox::hosp::HospSearchboxCpn::new(Mutable::new(Some(Rc::new(kphis_model::search::searchbox::HospSearchBox::demo()))));
    let dom = kphis_ui_component::gadget::searchbox::hosp::HospSearchboxCpn::render(cpn, app, Mutable::new(false), false, true);
    replace_body(dom).await;
}
#[wasm_bindgen_test]
async fn test_hosp_searchbox_cpn_savable_em() {
    let app = new_app();
    let cpn = kphis_ui_component::gadget::searchbox::hosp::HospSearchboxCpn::new(Mutable::new(Some(Rc::new(kphis_model::search::searchbox::HospSearchBox::demo()))));
    let dom = kphis_ui_component::gadget::searchbox::hosp::HospSearchboxCpn::render(cpn, app, Mutable::new(false), true, true);
    replace_body(dom).await;
}

#[wasm_bindgen_test]
async fn test_ivfluid_searchbox_cpn() {
    let app = new_app();
    let cpn = kphis_ui_component::gadget::searchbox::ivfluid::IvfluidSearchboxCpn::new();
    let order_form = kphis_ui_component::order_form::oneday::OneDayForm::new(
        Some(Rc::new(kphis_model::order::Order::demo())),
        Mutable::new(Some(Rc::new(kphis_model::patient_info::PatientInfo::demo()))),
        Some(1),
        String::from("user"),
        Mutable::new(String::from("doctor")),
        MutableVec::new_with_values(vec![Rc::new(kphis_ui_component::order::OrderItemMutable::default())]),
    );
    let dom = kphis_ui_component::gadget::searchbox::ivfluid::IvfluidSearchboxCpn::render(Some(1), cpn, order_form, app);
    replace_body(dom).await;
}

#[wasm_bindgen_test]
async fn test_lab_searchbox_cpn() {
    let app = new_app();
    let cpn = kphis_ui_component::gadget::searchbox::lab::LabSearchboxCpn::new();
    let dom = kphis_ui_component::gadget::searchbox::lab::LabSearchboxCpn::render(
        Some(1),
        cpn,
        Mutable::new(true),
        MutableVec::new_with_values(vec![Rc::new(kphis_ui_component::order::OrderItemMutable::default())]),
        Mutable::new(Some(1)),
        Mutable::new(false),
        app,
    );
    replace_body(dom).await;
}

#[wasm_bindgen_test]
async fn test_med_searchbox_cpn() {
    let app = new_app();
    let cpn = kphis_ui_component::gadget::searchbox::med::MedSearchboxCpn::new(Some(String::from("0001234")), false);
    let order_form = kphis_ui_component::order_form::oneday::OneDayForm::new(
        Some(Rc::new(kphis_model::order::Order::demo())),
        Mutable::new(Some(Rc::new(kphis_model::patient_info::PatientInfo::demo()))),
        Some(1),
        String::from("user"),
        Mutable::new(String::from("doctor")),
        MutableVec::new_with_values(vec![Rc::new(kphis_ui_component::order::OrderItemMutable::default())]),
    );
    let dom = kphis_ui_component::gadget::searchbox::med::MedSearchboxCpn::render(Some(1), cpn, order_form, app);
    replace_body(dom).await;
}
#[wasm_bindgen_test]
async fn test_med_searchbox_cpn_homemed() {
    let app = new_app();
    let cpn = kphis_ui_component::gadget::searchbox::med::MedSearchboxCpn::new(Some(String::from("0001234")), true);
    let order_form = kphis_ui_component::order_form::oneday::OneDayForm::new(
        Some(Rc::new(kphis_model::order::Order::demo())),
        Mutable::new(Some(Rc::new(kphis_model::patient_info::PatientInfo::demo()))),
        Some(1),
        String::from("user"),
        Mutable::new(String::from("doctor")),
        MutableVec::new_with_values(vec![Rc::new(kphis_ui_component::order::OrderItemMutable::default())]),
    );
    let dom = kphis_ui_component::gadget::searchbox::med::MedSearchboxCpn::render(Some(1), cpn, order_form, app);
    replace_body(dom).await;
}

#[wasm_bindgen_test]
async fn test_opd_visit_searchbox_cpn() {
    let app = new_app();
    let cpn = kphis_ui_component::gadget::searchbox::opd_visit::OpdVisitSearchboxCpn::new();
    let dom = kphis_ui_component::gadget::searchbox::opd_visit::OpdVisitSearchboxCpn::render(
        cpn,
        Mutable::new(true),
        Mutable::new(String::from("20221231235959")),
        Mutable::new(String::from("detail")),
        web_sys::DomRect::new().unwrap(),
        Mutable::new(false),
        app,
    );
    replace_body(dom).await;
}

#[wasm_bindgen_test]
async fn test_patient_searchbox_cpn() {
    let app = new_app();
    let cpn = kphis_ui_component::gadget::searchbox::patient::PatientSearchboxCpn::new();
    let dom = kphis_ui_component::gadget::searchbox::patient::PatientSearchboxCpn::render(
        cpn,
        Mutable::new(true),
        Mutable::new(String::from("0001234")),
        Mutable::new(String::from("user")),
        web_sys::DomRect::new().unwrap(),
        Mutable::new(false),
        app,
    );
    replace_body(dom).await;
}

#[wasm_bindgen_test]
async fn test_proc_searchbox_cpn() {
    let app = new_app();
    let cpn = kphis_ui_component::gadget::searchbox::proc::ProcSearchboxCpn::new(Mutable::new(Some(Arc::new(kphis_drg_worker::drg::model::I9vx {
        code: String::from("1234"),
        is_valid: true,
        desc: String::from("proc"),
    }))));
    let dom = kphis_ui_component::gadget::searchbox::proc::ProcSearchboxCpn::render(cpn, app, Mutable::new(false));
    replace_body(dom).await;
}

#[wasm_bindgen_test]
async fn test_xray_searchbox_cpn() {
    let app = new_app();
    let cpn = kphis_ui_component::gadget::searchbox::xray::XraySearchboxCpn::new();
    let dom = kphis_ui_component::gadget::searchbox::xray::XraySearchboxCpn::render(
        Some(1),
        cpn,
        Mutable::new(false),
        MutableVec::new_with_values(vec![Rc::new(kphis_ui_component::order::OrderItemMutable::default())]),
        Mutable::new(Some(1)),
        Mutable::new(false),
        app,
    );
    replace_body(dom).await;
}
