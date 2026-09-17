use futures_signals::signal::Mutable;
use wasm_bindgen_test::wasm_bindgen_test;

use crate::{new_app, replace_body};

#[wasm_bindgen_test]
async fn test_ipd_add_cpn() {
    let app = new_app();
    let cpn = kphis_ui_component::document::ipd_add::IpdDocumentAddCpn::new("660001234");
    let dom = kphis_ui_component::document::ipd_add::IpdDocumentAddCpn::render(cpn, app);
    replace_body(dom).await;
}

#[wasm_bindgen_test]
async fn test_ipd_list_cpn() {
    let app = new_app();
    let cpn = kphis_ui_component::document::ipd_list::IpdDocumentListCpn::new(Mutable::new(Some(String::from("20221231235959"))), "660001234", false);
    let dom = kphis_ui_component::document::ipd_list::IpdDocumentListCpn::render_inner(cpn, app);
    replace_body(dom).await;
}
#[wasm_bindgen_test]
async fn test_ipd_list_cpn_full() {
    let app = new_app();
    let cpn = kphis_ui_component::document::ipd_list::IpdDocumentListCpn::new(Mutable::new(Some(String::from("20221231235959"))), "660001234", true);
    let dom = kphis_ui_component::document::ipd_list::IpdDocumentListCpn::render_inner(cpn, app);
    replace_body(dom).await;
}

#[wasm_bindgen_test]
async fn test_ipd_scan_cpn() {
    let app = new_app();
    let cpn = kphis_ui_component::document::ipd_scan::IpdDocumentScanCpn::new("660001234", false);
    let dom = kphis_ui_component::document::ipd_scan::IpdDocumentScanCpn::render(cpn, app);
    replace_body(dom).await;
}
#[wasm_bindgen_test]
async fn test_ipd_scan_cpn_editable() {
    let app = new_app();
    let cpn = kphis_ui_component::document::ipd_scan::IpdDocumentScanCpn::new("660001234", true);
    let dom = kphis_ui_component::document::ipd_scan::IpdDocumentScanCpn::render(cpn, app);
    replace_body(dom).await;
}

#[wasm_bindgen_test]
async fn test_opd_er_list_cpn() {
    let app = new_app();
    let cpn = kphis_ui_component::document::opd_er_list::OpdErDocumentListCpn::new(Mutable::new(Some(String::from("20221231235959"))), 1, false);
    let dom = kphis_ui_component::document::opd_er_list::OpdErDocumentListCpn::render(cpn, app);
    replace_body(dom).await;
}
#[wasm_bindgen_test]
async fn test_opd_er_list_cpn_full() {
    let app = new_app();
    let cpn = kphis_ui_component::document::opd_er_list::OpdErDocumentListCpn::new(Mutable::new(Some(String::from("20221231235959"))), 1, true);
    let dom = kphis_ui_component::document::opd_er_list::OpdErDocumentListCpn::render(cpn, app);
    replace_body(dom).await;
}

#[wasm_bindgen_test]
async fn test_opd_er_scan_cpn() {
    let app = new_app();
    let cpn = kphis_ui_component::document::opd_er_scan::OpdErDocumentScanCpn::new(1, "20221231235959", false);
    let dom = kphis_ui_component::document::opd_er_scan::OpdErDocumentScanCpn::render(cpn, app);
    replace_body(dom).await;
}
#[wasm_bindgen_test]
async fn test_opd_er_scan_cpn_editable() {
    let app = new_app();
    let cpn = kphis_ui_component::document::opd_er_scan::OpdErDocumentScanCpn::new(1, "20221231235959", true);
    let dom = kphis_ui_component::document::opd_er_scan::OpdErDocumentScanCpn::render(cpn, app);
    replace_body(dom).await;
}
