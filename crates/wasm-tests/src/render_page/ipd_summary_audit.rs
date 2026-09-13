use wasm_bindgen_test::wasm_bindgen_test;

use crate::{new_app, replace_body};

#[wasm_bindgen_test]
async fn test_ipd_summary_audit_page() {
    let app = new_app();
    let page = kphis_ui_page::ipd_summary_audit::IpdSummaryAuditPage::new(String::from("660001234"));
    let dom = kphis_ui_page::ipd_summary_audit::IpdSummaryAuditPage::render(page, app);
    replace_body(dom).await;
}
