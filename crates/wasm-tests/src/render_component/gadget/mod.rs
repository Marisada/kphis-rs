mod searchbox;

use dominator::{Dom, html};
use futures_signals::signal::Mutable;
use std::rc::Rc;
use strum::IntoEnumIterator;
use wasm_bindgen_test::wasm_bindgen_test;

use crate::{new_app, replace_body};

#[wasm_bindgen_test]
async fn test_aside_resizer_cpn() {
    let app = new_app();
    let cpn = kphis_ui_component::gadget::aside_resizer::AsideResizerCpn::new(Mutable::new(Some(kphis_model::report::SystemReport::IpdOrder)), Mutable::new(false), Mutable::new(Some(kphis_model::image::file_path::DocumentType::EKG)), Mutable::new(false), Mutable::new(String::from("660001234")), Mutable::new(String::from("0001234")), kphis_model::report::SystemReport::iter().collect(), "1", Some(Mutable::new(true)), Some(Mutable::new(true)), app.clone());
    let dom = kphis_ui_component::gadget::aside_resizer::AsideResizerCpn::render(Dom::empty(), None, cpn, app);
    replace_body(dom).await;
}

#[wasm_bindgen_test]
async fn test_image_cpn() {
    let app = new_app();
    let cpn = kphis_ui_component::gadget::image::ImageCpn::new_with_key(kphis_model::image::file_path::ImageUsage::Unknown, 1, false, Mutable::new(Some(Rc::new(kphis_model::patient_info::PatientInfo::demo()))), Some(String::from("20221231235959")), "1");
    let dom = kphis_ui_component::gadget::image::ImageCpn::render("130px", cpn, app);
    replace_body(dom).await;
}
#[wasm_bindgen_test]
async fn test_image_cpn_editable() {
    let app = new_app();
    let cpn = kphis_ui_component::gadget::image::ImageCpn::new_with_key(kphis_model::image::file_path::ImageUsage::Unknown, 1, true, Mutable::new(Some(Rc::new(kphis_model::patient_info::PatientInfo::demo()))), Some(String::from("20221231235959")), "1");
    let dom = kphis_ui_component::gadget::image::ImageCpn::render("130px", cpn, app);
    replace_body(dom).await;
}
#[wasm_bindgen_test]
async fn test_image_cpn_returning() {
    let app = new_app();
    let cpn = kphis_ui_component::gadget::image::ImageCpn::new_returning(Mutable::new(kphis_ui_component::gadget::image::ImagePaths::default()), Mutable::new(Some(Rc::new(kphis_model::patient_info::PatientInfo::demo()))), Some(String::from("20221231235959")), "1");
    let dom = kphis_ui_component::gadget::image::ImageCpn::render("130px", cpn, app);
    replace_body(dom).await;
}
#[wasm_bindgen_test]
async fn test_image_cpn_storage() {
    let app = new_app();
    let cpn = kphis_ui_component::gadget::image::ImageCpn::new_using_local_storage();
    let dom = kphis_ui_component::gadget::image::ImageCpn::render("130px", cpn, app);
    replace_body(dom).await;
}
#[wasm_bindgen_test]
async fn test_image_cpn_modal() {
    let app = new_app();
    let cpn = kphis_ui_component::gadget::image::ImageCpn::new_using_local_storage();
    let dom = kphis_ui_component::gadget::image::ImageCpn::render_capture_modal(cpn, app);
    replace_body(dom).await;
}

#[wasm_bindgen_test]
async fn test_pdf_button_cpn() {
    let app = new_app();
    let cpn = kphis_ui_component::gadget::pdf_button::PdfButtons::new(kphis_model::report::TypstReport::System(kphis_model::report::SystemReport::IpdTPR), Mutable::new(1), Mutable::new(1), Mutable::new(false), || {String::new()});
    let doms = kphis_ui_component::gadget::pdf_button::PdfButtons::buttons(cpn, "1", None, app);
    replace_body(html!("div", {.children(doms)})).await;
}

#[wasm_bindgen_test]
async fn test_xray_viewer_cpn() {
    let cpn = kphis_ui_component::gadget::xray_viewer::XrayViewer::new(Mutable::new(Some(kphis_model::pacs::PacsImageData::demo())));
    let dom = kphis_ui_component::gadget::xray_viewer::XrayViewer::render("1", cpn);
    replace_body(dom).await;
}
