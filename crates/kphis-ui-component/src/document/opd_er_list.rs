// opd-document-main.php

use dominator::{Dom, clone, events, html};
use futures_signals::{
    map_ref,
    signal::{Mutable, SignalExt},
};
use std::rc::Rc;

use kphis_model::{
    endpoint::EndPoint,
    fetch::Method,
    image::file_path::DocumentType,
    opd_er::document::OpdErDocumentExists,
    report::{SystemReport, TypstReport},
};
use kphis_ui_app::App;
use kphis_ui_core::class;
use kphis_util::util::{str_some, zero_none};

use super::{PdfInner, PdfSource, check_used};
use crate::modal::{blank_modal, report::preview::ReportPreview};

/// - GET `EndPoint::OpdErDocumentListVnId`
/// - GET `EndPoint::ReportRawTemplateTypeId` (ReportPreview, guarded, cannot view report)
#[derive(Clone, Default)]
pub struct OpdErDocumentListCpn {
    is_full: bool,

    vn: Mutable<Option<String>>,
    opd_er_order_master_id: Mutable<u32>,

    loaded: Mutable<bool>,
    result: Mutable<OpdErDocumentExists>,

    render_pdf: Mutable<bool>,
    pdf_path: Mutable<Option<PdfInner>>,

    report_modal: Mutable<Option<Rc<ReportPreview>>>,
}

impl OpdErDocumentListCpn {
    pub fn new(vn: Mutable<Option<String>>, opd_er_order_master_id: u32, is_full: bool) -> Rc<Self> {
        Rc::new(Self {
            is_full,
            vn,
            opd_er_order_master_id: Mutable::new(opd_er_order_master_id),
            ..Default::default()
        })
    }

    // opd-document-main-data.php
    fn load(page: Rc<Self>, app: Rc<App>) {
        if let (Some(vn), Some(opd_er_order_master_id)) = (page.vn.get_cloned().and_then(str_some), zero_none(page.opd_er_order_master_id.get())) {
            app.async_load(
                true,
                clone!(app => async move {
                    // GET `EndPoint::OpdErDocumentListVnId`
                    match OpdErDocumentExists::call_api_get(&vn, opd_er_order_master_id, app.state()).await {
                        Ok(response) => {
                            page.result.set(response);
                        }
                        Err(e) => {
                            app.alert_app_error(&e).await;
                        }
                    }
                }),
            );
        }
    }

    fn set_report_modal(&self) {
        if let Some(inner) = self.pdf_path.get_cloned() {
            self.report_modal.set(Some(ReportPreview::new(inner.report, inner.ids, None, true, inner.title)));
        }
    }

    pub fn render(page: Rc<Self>, app: Rc<App>) -> Dom {
        let scan_template = TypstReport::from_system_with_coercion(SystemReport::DocumentImages, &app.state().report_coercions());

        html!("div", {
            .future(map_ref!(
                let busy = app.loader_is_loading(),
                let loaded = page.loaded.signal() =>
                !busy && !loaded
            ).for_each(clone!(app, page => move |ready| {
                if ready {
                    Self::load(page.clone(), app.clone());
                    page.loaded.set_neq(true);
                }
                async {}
            })))
            .future(map_ref!(
                let busy = app.loader_is_loading(),
                let render = page.render_pdf.signal() =>
                !busy && *render
            ).for_each(clone!(page => move |ready| {
                if ready {
                    page.set_report_modal();
                    //Self::pdf_cert(page.clone(), app.clone());
                    page.render_pdf.set_neq(false);
                }
                async {}
            })))
            .class(class::CARD)
            .children([
                html!("div", {
                    .class(class::CARD_HEAD_CYANS)
                    .style("line-height","34px") // for vertical-middle span
                    .child(html!("span", {
                        .class("fw-bold").text(&["VN : ", &page.vn.get_cloned().unwrap_or_default()].concat())
                    }))
                    .apply_if(page.is_full && app.endpoint_is_allow(&Method::GET, &EndPoint::ReportRawTemplateTypeId, false), |dom| {
                        dom.child(html!("span", {
                            .class("float-end")
                            .child(html!("button", {
                                .attr("type", "button")
                                .class(class::BTN_GRAY)
                                .attr("data-bs-toggle", "modal")
                                .attr("data-bs-target", "#opd-er-doc-report-modal")
                                .child(html!("i", {.class(class::FA_PRINT)}))
                                .text(" พิมพ์เอกสารใบปะหน้า")
                                .event(clone!(app, page => move |_: events::Click| {
                                    page.pdf_path.set(page.vn.get_cloned().map(|vn| PdfInner::new(TypstReport::from_system_with_coercion(SystemReport::OpdErDocument, &app.state().report_coercions()), vn, None)));
                                    page.render_pdf.set(true);
                                }))
                            }))
                        }))
                    })
                }),
                html!("div", {
                    .class("card-body")
                    .children([
                        html!("div", {
                            .child(html!("ul", {
                                .class(class::LIST_GROUP_FLUSH_OVFA)
                                .child_signal(page.result.signal_cloned().map(|res| res.has_scan_consent).map(clone!(app, page, scan_template => move |is_used| {
                                    (page.is_full || is_used).then(|| check_used(0, "#opd-er-doc-report-modal", "ใบยินยอม", vec![
                                        PdfSource::Scan(is_used, page.vn.lock_ref().as_ref().map(|vn| PdfInner::new(scan_template.clone(), [vn,"|1|1"].concat(), Some(DocumentType::InformedConsent.label().to_owned()))), "")
                                    ], page.pdf_path.clone(), page.render_pdf.clone(), app.clone()))
                                })))
                                .child_signal(page.result.signal_cloned().map(|res| res.has_scan_refer_in).map(clone!(app, page, scan_template => move |is_used| {
                                    (page.is_full || is_used).then(|| check_used(0, "#opd-er-doc-report-modal", "ใบ Refer-In", vec![
                                        PdfSource::Scan(is_used, page.vn.lock_ref().as_ref().map(|vn| PdfInner::new(scan_template.clone(), [&vn,"|3|1"].concat(), Some(DocumentType::ReferIn.label().to_owned()))), "")
                                    ], page.pdf_path.clone(), page.render_pdf.clone(), app.clone()))
                                })))
                                .child_signal(page.result.signal_cloned().map(|res| (res.has_data_refer_out, res.has_scan_refer_out)).map(clone!(app, page, scan_template => move |(is_used, has_scan)| {
                                    (page.is_full || is_used || has_scan).then(|| check_used(1, "#opd-er-doc-report-modal", "ใบ Refer-Out", vec![
                                        PdfSource::HosXp(is_used, page.vn.get_cloned().map(|vn| PdfInner::new(TypstReport::from_system_with_coercion(SystemReport::ReferOut, &app.state().report_coercions()), vn, None)), ""),
                                        PdfSource::Scan(has_scan, page.vn.lock_ref().as_ref().map(|vn| PdfInner::new(scan_template.clone(), [vn,"|4|1"].concat(), Some(DocumentType::ReferOut.label().to_owned()))), "")
                                    ], page.pdf_path.clone(), page.render_pdf.clone(), app.clone()))
                                })))
                                .child_signal(page.result.signal_cloned().map(|res| res.has_data_er_master_id).map(clone!(app, page => move |is_used| {
                                    (page.is_full || is_used).then(|| check_used(0, "#opd-er-doc-report-modal", "ประวัติผู้ป่วย", vec![
                                        PdfSource::Kphis(is_used, page.vn.get_cloned().map(|vn| PdfInner::new(TypstReport::from_system_with_coercion(SystemReport::OpdErMedicalHistory, &app.state().report_coercions()), vn, None)), "")
                                    ], page.pdf_path.clone(), page.render_pdf.clone(), app.clone()))
                                })))
                                .child_signal(page.result.signal_cloned().map(|res| res.has_data_med_reconciliation).map(clone!(app, page => move |is_used| {
                                    (page.is_full || is_used).then(|| check_used(0, "#opd-er-doc-report-modal", "Med Reconciliation", vec![
                                        PdfSource::Kphis(is_used, page.vn.get_cloned().map(|vn| PdfInner::new(TypstReport::from_system_with_coercion(SystemReport::IpdMedReconciliation, &app.state().report_coercions()), vn, None)), "")
                                    ], page.pdf_path.clone(), page.render_pdf.clone(), app.clone()))
                                })))
                                .child_signal(page.result.signal_cloned().map(|res| res.has_scan_physio).map(clone!(app, page, scan_template => move |is_used| {
                                    (page.is_full || is_used).then(|| check_used(0, "#opd-er-doc-report-modal", "Physiotherapy Sheet (กายภาพบำบัด)", vec![
                                        PdfSource::Scan(is_used, page.vn.lock_ref().as_ref().map(|vn| PdfInner::new(scan_template.clone(), [vn,"|15|1"].concat(), Some(DocumentType::Physiotherapy.label().to_owned()))), "")
                                    ], page.pdf_path.clone(), page.render_pdf.clone(), app.clone()))
                                })))
                                .child_signal(page.result.signal_cloned().map(|res| res.has_data_order || res.has_data_progress_note).map(clone!(app, page => move |is_used| {
                                    (page.is_full || is_used).then(|| check_used(0, "#opd-er-doc-report-modal", "Order and Progress Note", vec![
                                        PdfSource::Kphis(is_used, page.vn.get_cloned().map(|vn| PdfInner::new(TypstReport::from_system_with_coercion(SystemReport::OpdErOrder, &app.state().report_coercions()), vn, None)), "")
                                    ], page.pdf_path.clone(), page.render_pdf.clone(), app.clone()))
                                })))
                                .child_signal(page.result.signal_cloned().map(|res| res.has_scan_oper).map(clone!(app, page, scan_template => move |is_used| {
                                    (page.is_full || is_used).then(|| check_used(0, "#opd-er-doc-report-modal", "Operative Report", vec![
                                        PdfSource::Scan(is_used, page.vn.lock_ref().as_ref().map(|vn| PdfInner::new(scan_template.clone(), [vn,"|12|1"].concat(), Some(DocumentType::Operation.label().to_owned()))), "")
                                    ], page.pdf_path.clone(), page.render_pdf.clone(), app.clone()))
                                })))
                                .child_signal(page.result.signal_cloned().map(|res| res.has_scan_anes).map(clone!(app, page, scan_template => move |is_used| {
                                    (page.is_full || is_used).then(|| check_used(0, "#opd-er-doc-report-modal", "Anesthetic Report", vec![
                                        PdfSource::Scan(is_used, page.vn.lock_ref().as_ref().map(|vn| PdfInner::new(scan_template.clone(), [vn,"|13|1"].concat(), Some(DocumentType::Anesthesia.label().to_owned()))), "")
                                    ], page.pdf_path.clone(), page.render_pdf.clone(), app.clone()))
                                })))
                                .child_signal(page.result.signal_cloned().map(|res| res.has_scan_labour).map(clone!(app, page, scan_template => move |is_used| {
                                    (page.is_full || is_used).then(|| check_used(0, "#opd-er-doc-report-modal", "Labour Record", vec![
                                        PdfSource::Scan(is_used, page.vn.lock_ref().as_ref().map(|vn| PdfInner::new(scan_template.clone(), [vn,"|14|1"].concat(), Some(DocumentType::Labour.label().to_owned()))), "")
                                    ], page.pdf_path.clone(), page.render_pdf.clone(), app.clone()))
                                })))
                                .child_signal(page.result.signal_cloned().map(|res| res.has_scan_culture).map(clone!(app, page, scan_template=> move |is_used| {
                                    (page.is_full || is_used).then(|| check_used(0, "#opd-er-doc-report-modal", "ผลการเพาะเชื้อ/ชิ้นเนื้อ", vec![
                                        PdfSource::Scan(is_used, page.vn.lock_ref().as_ref().map(|vn| PdfInner::new(scan_template.clone(), [vn,"|5|1"].concat(), Some(DocumentType::CulturePatho.label().to_owned()))), "")
                                    ], page.pdf_path.clone(), page.render_pdf.clone(), app.clone()))
                                })))
                                .child_signal(page.result.signal_cloned().map(|res| res.has_data_lab).map(clone!(app, page => move |is_used| {
                                    (page.is_full || is_used).then(|| check_used(0, "#opd-er-doc-report-modal", "Lab", vec![
                                        PdfSource::HosXp(is_used, page.vn.get_cloned().map(|vn| PdfInner::new(TypstReport::from_system_with_coercion(SystemReport::Lab, &app.state().report_coercions()), vn, None)), ""),
                                        PdfSource::Kphis(is_used, page.vn.get_cloned().map(|vn| PdfInner::new(TypstReport::from_system_with_coercion(SystemReport::LabSummary, &app.state().report_coercions()), vn, None)), ""),
                                    ], page.pdf_path.clone(), page.render_pdf.clone(), app.clone()))
                                })))
                                .child_signal(page.result.signal_cloned().map(|res| res.has_scan_ekg).map(clone!(app, page, scan_template => move |is_used| {
                                    (page.is_full || is_used).then(|| check_used(0, "#opd-er-doc-report-modal", "Electrocardiogram Report", vec![
                                        PdfSource::Scan(is_used, page.vn.lock_ref().as_ref().map(|vn| PdfInner::new(scan_template.clone(), [vn,"|8|1"].concat(), Some(DocumentType::EKG.label().to_owned()))), "")
                                    ], page.pdf_path.clone(), page.render_pdf.clone(), app.clone()))
                                })))
                                .child_signal(page.result.signal_cloned().map(|res| res.has_scan_xray).map(clone!(app, page, scan_template => move |is_used| {
                                    (page.is_full || is_used).then(|| check_used(0, "#opd-er-doc-report-modal", "X-rays Report", vec![
                                        PdfSource::Scan(is_used, page.vn.lock_ref().as_ref().map(|vn| PdfInner::new(scan_template.clone(), [vn,"|9|1"].concat(), Some(DocumentType::Xray.label().to_owned()))), "")
                                    ], page.pdf_path.clone(), page.render_pdf.clone(), app.clone()))
                                })))
                                .child_signal(page.result.signal_cloned().map(|res| res.has_scan_ct).map(clone!(app, page, scan_template => move |is_used| {
                                    (page.is_full || is_used).then(|| check_used(0, "#opd-er-doc-report-modal", "CT scan", vec![
                                        PdfSource::Scan(is_used, page.vn.lock_ref().as_ref().map(|vn| PdfInner::new(scan_template.clone(), [vn,"|10|1"].concat(), Some(DocumentType::CT.label().to_owned()))), "")
                                    ], page.pdf_path.clone(), page.render_pdf.clone(), app.clone()))
                                })))
                                .child_signal(page.result.signal_cloned().map(|res| res.has_scan_mri).map(clone!(app, page, scan_template => move |is_used| {
                                    (page.is_full || is_used).then(|| check_used(0, "#opd-er-doc-report-modal", "MRI", vec![
                                        PdfSource::Scan(is_used, page.vn.lock_ref().as_ref().map(|vn| PdfInner::new(scan_template.clone(), [vn,"|11|1"].concat(), Some(DocumentType::MRI.label().to_owned()))), "")
                                    ], page.pdf_path.clone(), page.render_pdf.clone(), app.clone()))
                                })))
                                .child_signal(page.result.signal_cloned().map(|res| res.has_scan_special).map(clone!(app, page, scan_template => move |is_used| {
                                    (page.is_full || is_used).then(|| check_used(0, "#opd-er-doc-report-modal", "ผลตรวจพิเศษ", vec![
                                        PdfSource::Scan(is_used, page.vn.lock_ref().as_ref().map(|vn| PdfInner::new(scan_template.clone(), [vn,"|7|1"].concat(), Some(DocumentType::SpecialLab.label().to_owned()))), "")
                                    ], page.pdf_path.clone(), page.render_pdf.clone(), app.clone()))
                                })))
                                .child_signal(page.result.signal_cloned().map(|res| res.has_data_focus_list).map(clone!(app, page => move |is_used| {
                                    (page.is_full || is_used).then(|| check_used(0, "#opd-er-doc-report-modal", "Focus List", vec![
                                        PdfSource::Kphis(is_used, page.vn.get_cloned().map(|vn| PdfInner::new(TypstReport::from_system_with_coercion(SystemReport::OpdErFocusList, &app.state().report_coercions()), vn, None)), "")
                                    ], page.pdf_path.clone(), page.render_pdf.clone(), app.clone()))
                                })))
                                .child_signal(page.result.signal_cloned().map(|res| res.has_data_focus_note).map(clone!(app, page => move |is_used| {
                                    (page.is_full || is_used).then(|| check_used(0, "#opd-er-doc-report-modal", "Nursing Progress Note", vec![
                                        PdfSource::Kphis(is_used, page.vn.get_cloned().map(|vn| PdfInner::new(TypstReport::from_system_with_coercion(SystemReport::OpdErFocusNote, &app.state().report_coercions()), vn, None)), "")
                                    ], page.pdf_path.clone(), page.render_pdf.clone(), app.clone()))
                                })))
                                .child_signal(page.result.signal_cloned().map(|res| res.has_data_index_plan).map(clone!(app, page => move |is_used| {
                                    (page.is_full || is_used).then(|| check_used(0, "#opd-er-doc-report-modal", "Index (Nurse Planning)", vec![
                                        PdfSource::Kphis(is_used, page.vn.get_cloned().map(|vn| PdfInner::new(TypstReport::from_system_with_coercion(SystemReport::OpdErIndexPlan, &app.state().report_coercions()), vn, None)), "")
                                    ], page.pdf_path.clone(), page.render_pdf.clone(), app.clone()))
                                })))
                                .child_signal(page.result.signal_cloned().map(|res| res.has_data_vital_sign).map(clone!(app, page => move |is_used| {
                                    (page.is_full || is_used).then(|| check_used(0, "#opd-er-doc-report-modal", "Vital Sign", vec![
                                        PdfSource::Kphis(is_used, page.vn.get_cloned().map(|vn| PdfInner::new(TypstReport::from_system_with_coercion(SystemReport::OpdErVitalSignGeneral, &app.state().report_coercions()), vn, None)), "V/S"),
                                        PdfSource::Kphis(is_used, page.vn.get_cloned().map(|vn| PdfInner::new(TypstReport::from_system_with_coercion(SystemReport::OpdErVitalSignNeuro, &app.state().report_coercions()), vn, None)), "Neuro"),
                                        PdfSource::Kphis(is_used, page.vn.get_cloned().map(|vn| PdfInner::new(TypstReport::from_system_with_coercion(SystemReport::OpdErVitalSignLabour, &app.state().report_coercions()), vn, None)), "LR"),
                                        PdfSource::Kphis(is_used, page.vn.get_cloned().map(|vn| PdfInner::new(TypstReport::from_system_with_coercion(SystemReport::OpdErVitalSignPsychia, &app.state().report_coercions()), vn, None)), "Psychia"),
                                    ], page.pdf_path.clone(), page.render_pdf.clone(), app.clone()))
                                })))
                                .child_signal(page.result.signal_cloned().map(|res| res.has_data_io).map(clone!(app, page => move |is_used| {
                                    (page.is_full || is_used).then(|| check_used(0, "#opd-er-doc-report-modal", "I/O", vec![
                                        PdfSource::Kphis(is_used, page.vn.get_cloned().map(|vn| PdfInner::new(TypstReport::from_system_with_coercion(SystemReport::OpdErIo, &app.state().report_coercions()), vn, None)), "")
                                    ], page.pdf_path.clone(), page.render_pdf.clone(), app.clone()))
                                })))
                                .child_signal(page.result.signal_cloned().map(|res| res.has_scan_blood).map(clone!(app, page, scan_template => move |is_used| {
                                    (page.is_full || is_used).then(|| check_used(0, "#opd-er-doc-report-modal", "Blood transfusion Report (ใบของห้องเลือด)", vec![
                                        PdfSource::Scan(is_used, page.vn.lock_ref().as_ref().map(|vn| PdfInner::new(scan_template.clone(), [vn,"|6|1"].concat(), Some(DocumentType::Blood.label().to_owned()))), "")
                                    ], page.pdf_path.clone(), page.render_pdf.clone(), app.clone()))
                                })))
                                .child_signal(page.result.signal_cloned().map(|res| res.has_scan_opd_card).map(clone!(app, page, scan_template => move |is_used| {
                                    (page.is_full || is_used).then(|| check_used(0, "#opd-er-doc-report-modal", "OPD card", vec![
                                        PdfSource::Scan(is_used, page.vn.lock_ref().as_ref().map(|vn| PdfInner::new(scan_template.clone(), [vn,"|20|1"].concat(), Some(DocumentType::OPDcard.label().to_owned()))), "")
                                    ], page.pdf_path.clone(), page.render_pdf.clone(), app.clone()))
                                })))
                                .child_signal(page.result.signal_cloned().map(|res| res.has_scan_insure).map(clone!(app, page, scan_template => move |is_used| {
                                    (page.is_full || is_used).then(|| check_used(0, "#opd-er-doc-report-modal", "ใบตรวจสอบสิทธิ์", vec![
                                        PdfSource::Scan(is_used, page.vn.lock_ref().as_ref().map(|vn| PdfInner::new(scan_template.clone(), [vn,"|2|1"].concat(), Some(DocumentType::InsureCheck.label().to_owned()))), "")
                                    ], page.pdf_path.clone(), page.render_pdf.clone(), app.clone()))
                                })))
                                .child_signal(page.result.signal_cloned().map(|res| res.has_scan_alt_med).map(clone!(app, page, scan_template => move |is_used| {
                                    (page.is_full || is_used).then(|| check_used(0, "#opd-er-doc-report-modal", "บันทึกการแพทย์ทางเลือก", vec![
                                        PdfSource::Scan(is_used, page.vn.lock_ref().as_ref().map(|vn| PdfInner::new(scan_template.clone(), [vn,"|16|1"].concat(), Some(DocumentType::AlternativeRx.label().to_owned()))), "")
                                    ], page.pdf_path.clone(), page.render_pdf.clone(), app.clone()))
                                })))
                                .child_signal(page.result.signal_cloned().map(|res| res.has_scan_nutrition).map(clone!(app, page, scan_template=> move |is_used| {
                                    (page.is_full || is_used).then(|| check_used(0, "#opd-er-doc-report-modal", "บันทึกโภชนาการ", vec![
                                        PdfSource::Scan(is_used, page.vn.lock_ref().as_ref().map(|vn| PdfInner::new(scan_template.clone(), [vn,"|17|1"].concat(), Some(DocumentType::Nutrition.label().to_owned()))), "")
                                    ], page.pdf_path.clone(), page.render_pdf.clone(), app.clone()))
                                })))
                                .child_signal(page.result.signal_cloned().map(|res| res.has_scan_other_sp_clinic).map(clone!(app, page, scan_template => move |is_used| {
                                    (page.is_full || is_used).then(|| check_used(0, "#opd-er-doc-report-modal", "Other Special Clinical Report", vec![
                                        PdfSource::Scan(is_used, page.vn.lock_ref().as_ref().map(|vn| PdfInner::new(scan_template.clone(), [vn,"|19|1"].concat(), Some(DocumentType::OtherSpClinic.label().to_owned()))), "")
                                    ], page.pdf_path.clone(), page.render_pdf.clone(), app.clone()))
                                })))
                                .child_signal(page.result.signal_cloned().map(|res| res.has_scan_others).map(clone!(app, page, scan_template => move |is_used| {
                                    (page.is_full || is_used).then(|| check_used(0, "#opd-er-doc-report-modal", "เอกสารอื่นๆ", vec![
                                        PdfSource::Scan(is_used, page.vn.lock_ref().as_ref().map(|vn| PdfInner::new(scan_template.clone(), [vn,"|18|1"].concat(), Some(DocumentType::Others.label().to_owned()))), "")
                                    ], page.pdf_path.clone(), page.render_pdf.clone(), app.clone()))
                                })))
                                .child_signal(page.result.signal_cloned().map(|res| res.has_scan_finance).map(clone!(app, page, scan_template => move |is_used| {
                                    (page.is_full || is_used).then(|| check_used(0, "#opd-er-doc-report-modal", "เอกสารใบค่าใช้จ่าย", vec![
                                        PdfSource::Scan(is_used, page.vn.lock_ref().as_ref().map(|vn| PdfInner::new(scan_template.clone(), [vn,"|21|1"].concat(), Some(DocumentType::Finance.label().to_owned()))), "")
                                    ], page.pdf_path.clone(), page.render_pdf.clone(), app.clone()))
                                })))
                                .child_signal(page.result.signal_cloned().map(|res| {
                                    res.has_data_order || res.has_data_progress_note || res.has_data_index_plan || res.has_data_vital_sign || res.has_data_io || res.has_data_lab
                                }).map(clone!(app, page => move |is_used| {
                                    (page.is_full || is_used).then(|| check_used(0, "#opd-er-doc-report-modal", "Event Logs", vec![
                                        PdfSource::Kphis(is_used, page.vn.get_cloned().map(|vn| PdfInner::new(TypstReport::from_system_with_coercion(SystemReport::OpdErEventLog, &app.state().report_coercions()), vn, None)), "")
                                    ], page.pdf_path.clone(), page.render_pdf.clone(), app.clone()))
                                })))
                            }))
                        }),
                    ])
                }),
                html!("div", {
                    .class("modal")
                    .attr("id", "opd-er-doc-report-modal")
                    .attr("role", "dialog")
                    .attr("tabindex", "-1")
                    .child_signal(page.report_modal.signal_cloned().map(clone!(app => move |opt| {
                        opt.as_ref().map(clone!(app => move |modal| {
                            // GET `EndPoint::ReportRawTemplateTypeId`
                            ReportPreview::render(modal.clone(), app)
                        })).or(Some(blank_modal()))
                    })))
                }),
            ])
        })
    }
}
