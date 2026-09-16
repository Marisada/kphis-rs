use futures_signals::{signal::Mutable, signal_vec::MutableVec};
use wasm_bindgen_test::wasm_bindgen_test;

use crate::{new_app, replace_body};

#[wasm_bindgen_test]
async fn test_ipd_summary_audit_page() {
    let app = new_app();
    let page = kphis_ui_page::ipd_summary_audit::IpdSummaryAuditPage::new(String::from("660001234"));
    let dom = kphis_ui_page::ipd_summary_audit::IpdSummaryAuditPage::render(page, app);
    replace_body(dom).await;
}

#[wasm_bindgen_test]
async fn test_ipd_summary_audit_page_body() {
    let app = new_app();
    let page = kphis_ui_page::ipd_summary_audit::IpdSummaryAuditPage::new(String::from("660001234"));
    let dom = kphis_ui_page::ipd_summary_audit::IpdSummaryAuditPage::render_body(page, app);
    replace_body(dom).await;
}

#[wasm_bindgen_test]
async fn test_ipd_summary_audit_page_mut() {
    let app = new_app();
    let page = kphis_ui_page::ipd_summary_audit::IpdSummaryAuditPage::new(String::from("660001234"));
    let audit = kphis_ui_page::ipd_summary_audit::IpdSummaryAuditMutable::new(page);
    let dom = kphis_ui_page::ipd_summary_audit::IpdSummaryAuditMutable::render(audit, Mutable::new(false), app);
    replace_body(dom).await;
}

#[wasm_bindgen_test]
async fn test_ipd_summary_audit_page_item() {
    let app = new_app();
    let audit_item = kphis_ui_page::ipd_summary_audit::SummaryAuditItemMutable::new("ODx", 1, 1);
    let dom = kphis_ui_page::ipd_summary_audit::SummaryAuditItemMutable::render(Some(Mutable::new(Some(1)).read_only()), audit_item.clone(), MutableVec::new_with_values(vec![audit_item]), Mutable::new(false), app);
    replace_body(dom).await;
}
