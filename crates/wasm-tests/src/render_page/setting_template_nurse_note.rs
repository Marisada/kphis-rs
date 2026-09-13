use wasm_bindgen_test::wasm_bindgen_test;

use crate::{new_app, replace_body};

#[wasm_bindgen_test]
async fn test_setting_template_nurse_note_page() {
    let app = new_app();
    let page = kphis_ui_page::setting_template_nurse_note::SettingTemplateNurseNotePage::new();
    let dom = kphis_ui_page::setting_template_nurse_note::SettingTemplateNurseNotePage::render(page, app);
    replace_body(dom).await;
}
