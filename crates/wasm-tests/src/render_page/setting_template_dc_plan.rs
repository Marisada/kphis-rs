use wasm_bindgen_test::wasm_bindgen_test;

use crate::{new_app, replace_body};

#[wasm_bindgen_test]
async fn test_setting_template_dc_plan_page() {
    let app = new_app();
    let page = kphis_ui_page::setting_template_dc_plan::SettingTemplateDcPlanPage::new();
    let dom = kphis_ui_page::setting_template_dc_plan::SettingTemplateDcPlanPage::render(page, app);
    replace_body(dom).await;
}
