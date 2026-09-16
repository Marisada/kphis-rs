use wasm_bindgen_test::wasm_bindgen_test;

use crate::{new_app, replace_body};

#[wasm_bindgen_test]
async fn test_report_viewer_page() {
    let app = new_app();
    let page = kphis_ui_page::report_viewer::ReportViewerPage::new(app.clone());
    let dom = kphis_ui_page::report_viewer::ReportViewerPage::render(page, app);
    replace_body(dom).await;
}

#[wasm_bindgen_test]
async fn test_report_viewer_page_list_ipd() {
    let app = new_app();
    let page = kphis_ui_page::report_viewer::ReportViewerPage::new(app.clone());
    let dom = kphis_ui_page::report_viewer::ReportViewerPage::render_patient_list(true, page, app);
    replace_body(dom).await;
}
#[wasm_bindgen_test]
async fn test_report_viewer_page_list_opd_er() {
    let app = new_app();
    let page = kphis_ui_page::report_viewer::ReportViewerPage::new(app.clone());
    let dom = kphis_ui_page::report_viewer::ReportViewerPage::render_patient_list(false, page, app);
    replace_body(dom).await;
}

#[wasm_bindgen_test]
async fn test_report_viewer_page_params() {
    let app = new_app();
    let page = kphis_ui_page::report_viewer::ReportViewerPage::new(app.clone());
    let dom = kphis_ui_page::report_viewer::ReportViewerPage::render_custom_params(page, app);
    replace_body(dom).await;
}
