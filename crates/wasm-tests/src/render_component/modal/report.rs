use futures_signals::signal::Mutable;
use wasm_bindgen_test::wasm_bindgen_test;

use crate::{new_app, replace_body};

#[wasm_bindgen_test]
async fn test_param_editor_cpn() {
    let app = new_app();
    let modal = kphis_ui_component::modal::report::param_editor::ReportParamEditor::new(Mutable::new(String::from("text")), Mutable::new(false), app.clone());
    let dom = kphis_ui_component::modal::report::param_editor::ReportParamEditor::render(modal.clone(), Mutable::new(Some(modal)), app);
    replace_body(dom).await;
}
#[wasm_bindgen_test]
async fn test_param_editor_cpn_param() {
    let app = new_app();
    let modal = kphis_ui_component::modal::report::param_editor::ReportParamEditor::new(Mutable::new(String::from("text")), Mutable::new(false), app.clone());
    let dom = kphis_ui_component::modal::report::param_editor::ReportParamEditor::render_param(
        Mutable::new(Some(1)).read_only(),
        kphis_ui_component::modal::report::param_editor::ReportParamMutable::new(kphis_model::report::VarType::List, true),
        modal.clone(),
    );
    replace_body(dom).await;
}
#[wasm_bindgen_test]
async fn test_param_editor_cpn_param_item() {
    let app = new_app();
    let modal = kphis_ui_component::modal::report::param_editor::ReportParamEditor::new(Mutable::new(String::from("text")), Mutable::new(false), app.clone());
    let dom = kphis_ui_component::modal::report::param_editor::ReportParamEditor::render_param_item(
        Mutable::new(Some(1)).read_only(),
        kphis_ui_component::modal::report::param_editor::KeyLabelMutable::new(),
        kphis_ui_component::modal::report::param_editor::ReportParamMutable::new(kphis_model::report::VarType::List, true),
        modal.clone(),
    );
    replace_body(dom).await;
}

#[wasm_bindgen_test]
async fn test_param_input_cpn() {
    let app = new_app();
    let modal = kphis_ui_component::modal::report::param_input::ReportParamInput::new(&kphis_model::report::ReportParam {
        id: String::from("1"),
        title: String::from("report"),
        ty: kphis_model::report::ParamType::Basic(kphis_model::report::BasicType::Bool),
    });
    let dom = kphis_ui_component::modal::report::param_input::ReportParamInput::render(modal, Mutable::new(false), app);
    replace_body(dom).await;
}

#[wasm_bindgen_test]
async fn test_preview_cpn() {
    let app = new_app();
    let modal = kphis_ui_component::modal::report::preview::ReportPreview::new(
        kphis_model::report::TypstReport::System(kphis_model::report::SystemReport::IpdOrder),
        String::from("1,2"),
        Some(String::from(r#"{"id":1}"#)),
        false,
        Some(String::from("title")),
    );
    let dom = kphis_ui_component::modal::report::preview::ReportPreview::render_modal(modal.clone(), Mutable::new(Some(modal)), app);
    replace_body(dom).await;
}
#[wasm_bindgen_test]
async fn test_preview_cpn_can_sign() {
    let app = new_app();
    let modal = kphis_ui_component::modal::report::preview::ReportPreview::new(
        kphis_model::report::TypstReport::System(kphis_model::report::SystemReport::IpdOrder),
        String::from("1,2"),
        Some(String::from(r#"{"id":1}"#)),
        true,
        Some(String::from("title")),
    );
    let dom = kphis_ui_component::modal::report::preview::ReportPreview::render_modal(modal.clone(), Mutable::new(Some(modal)), app);
    replace_body(dom).await;
}
#[wasm_bindgen_test]
async fn test_preview_cpn_static() {
    let app = new_app();
    let modal = kphis_ui_component::modal::report::preview::ReportPreview::new_static("template", Some(String::from(r#"{"id":1}"#)), "title");
    let dom = kphis_ui_component::modal::report::preview::ReportPreview::render_modal(modal.clone(), Mutable::new(Some(modal)), app);
    replace_body(dom).await;
}
