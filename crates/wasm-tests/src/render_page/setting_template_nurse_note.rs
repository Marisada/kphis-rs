use wasm_bindgen_test::wasm_bindgen_test;

use crate::{new_app, replace_body};

#[wasm_bindgen_test]
async fn test_setting_template_nurse_note_page() {
    let app = new_app();
    let page = kphis_ui_page::setting_template_nurse_note::SettingTemplateNurseNotePage::new();
    let dom = kphis_ui_page::setting_template_nurse_note::SettingTemplateNurseNotePage::render(page, app);
    replace_body(dom).await;
}

#[wasm_bindgen_test]
async fn test_setting_template_nurse_note_page_modal_smp() {
    let app = new_app();
    let page = kphis_ui_page::setting_template_nurse_note::SettingTemplateNurseNotePage::new();
    let dom = kphis_ui_page::setting_template_nurse_note::SettingTemplateNurseNotePage::render_smp_modal(page, app);
    replace_body(dom).await;
}

#[wasm_bindgen_test]
async fn test_setting_template_nurse_note_page_modal_sub() {
    let app = new_app();
    let page = kphis_ui_page::setting_template_nurse_note::SettingTemplateNurseNotePage::new();
    let dom = kphis_ui_page::setting_template_nurse_note::SettingTemplateNurseNotePage::render_sub_modal(page, app);
    replace_body(dom).await;
}

#[wasm_bindgen_test]
async fn test_setting_template_nurse_note_page_modal_focus() {
    let app = new_app();
    let page = kphis_ui_page::setting_template_nurse_note::SettingTemplateNurseNotePage::new();
    let dom = kphis_ui_page::setting_template_nurse_note::SettingTemplateNurseNotePage::render_focus_modal(page, app);
    replace_body(dom).await;
}

#[wasm_bindgen_test]
async fn test_setting_template_nurse_note_page_modal_goal() {
    let app = new_app();
    let page = kphis_ui_page::setting_template_nurse_note::SettingTemplateNurseNotePage::new();
    let dom = kphis_ui_page::setting_template_nurse_note::SettingTemplateNurseNotePage::render_goal_modal(page, app);
    replace_body(dom).await;
}

#[wasm_bindgen_test]
async fn test_setting_template_nurse_note_page_modal_intvt() {
    let app = new_app();
    let page = kphis_ui_page::setting_template_nurse_note::SettingTemplateNurseNotePage::new();
    let dom = kphis_ui_page::setting_template_nurse_note::SettingTemplateNurseNotePage::render_intvt_modal(page, app);
    replace_body(dom).await;
}

#[wasm_bindgen_test]
async fn test_setting_template_nurse_note_page_modal_dlc() {
    let app = new_app();
    let page = kphis_ui_page::setting_template_nurse_note::SettingTemplateNurseNotePage::new();
    let dom = kphis_ui_page::setting_template_nurse_note::SettingTemplateNurseNotePage::render_dlc_modal(page, app);
    replace_body(dom).await;
}
