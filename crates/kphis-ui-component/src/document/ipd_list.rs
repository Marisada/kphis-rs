// ipd-document-main.php
// opd-document-main.php

use dominator::{Dom, clone, events, html};
use futures_signals::{
    map_ref,
    signal::{Mutable, SignalExt},
};
use std::rc::Rc;
use web_sys::HtmlButtonElement;

use kphis_model::{
    app::AppState,
    endpoint::EndPoint,
    fetch::Method,
    image::file_path::DocumentType,
    ipd::document::IpdDocumentExists,
    report::{SystemReport, TypstReport},
};
use kphis_ui_app::App;
use kphis_ui_core::{class, mixins};
use kphis_util::util::str_some;

use super::{PdfInner, PdfSource, check_used, check_used_group, opd_er_list::OpdErDocumentListCpn};
use crate::modal::{blank_modal, report::preview::ReportPreview};

/// - GET `EndPoint::IpdDocumentListVnAn`
/// - GET `EndPoint::ReportTemplateTypeId` (guarded, remove `รวม xxx` btns)
/// - GET `EndPoint::OpdErDocumentListVnId` (self/OpdErDocumentListCpn, guarded, remove view opd-er list)
/// - GET `EndPoint::ReportRawTemplateTypeId` (ReportPreview, guarded, cannot view report)
#[derive(Clone, Default)]
pub struct IpdDocumentListCpn {
    is_full: bool,

    vn: Mutable<Option<String>>,
    an: Option<String>,
    opd_er_order_master_id: Mutable<Option<u32>>,

    loaded: Mutable<bool>,
    result: Mutable<IpdDocumentExists>,

    render_pdf: Mutable<bool>,
    pdf_path: Mutable<Option<PdfInner>>,

    load_full_general_pdf: Mutable<bool>,
    load_full_labour_pdf: Mutable<bool>,
    load_full_psychia_pdf: Mutable<bool>,

    report_modal: Mutable<Option<Rc<ReportPreview>>>,
}

impl IpdDocumentListCpn {
    pub fn new(vn: Mutable<Option<String>>, an: &str, is_full: bool) -> Rc<Self> {
        Rc::new(Self {
            is_full,
            vn,
            an: str_some(an.to_owned()),
            ..Default::default()
        })
    }

    // opd-document-main-data.php
    fn load(page: Rc<Self>, app: Rc<App>) {
        if let (Some(vn), Some(an)) = (page.vn.get_cloned().and_then(str_some), page.an.clone()) {
            app.async_load(
                true,
                clone!(app => async move {
                    // GET `EndPoint::IpdDocumentListVnAn`
                    match IpdDocumentExists::call_api_get(&vn, &an, app.state()).await {
                        Ok(response) => {
                            page.opd_er_order_master_id.set_neq(response.opd_er_order_master_id);
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

    /// GET `EndPoint::ReportTemplateTypeId`
    fn load_full_pdf(full_type: &'static str, page: Rc<Self>, app: Rc<App>) {
        if let (Some(an), Some(vn)) = (page.an.as_ref(), page.vn.lock_ref().as_ref()) {
            let ids = [vn, "|", an].concat();
            let file_name = [an, "-", full_type].concat();
            app.async_load(
                true,
                clone!(app => async move {
                    // GET `EndPoint::ReportTemplateTypeId`
                    match AppState::call_api_get_pdf_report(full_type, "system", &ids, app.state()).await {
                        Ok(blob) => {
                            app.open_response_blob(blob, &file_name);
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
        let allow_opd_er_list = app.endpoint_is_allow(&Method::GET, &EndPoint::OpdErDocumentListVnId, false);

        html!("div", {
            .apply_if(page.is_full && allow_opd_er_list, |dom| {
                dom.child_signal(page.opd_er_order_master_id.signal_cloned().map(clone!(app, page => move |opt| {
                    opt.map(|opd_er_order_master_id| {
                        // GET `EndPoint::OpdErDocumentListVnId`
                        let opd_er_list = OpdErDocumentListCpn::new(page.vn.clone(), opd_er_order_master_id, true);
                        OpdErDocumentListCpn::render(opd_er_list, app.clone())
                    })
                })))
            })
            .child(Self::render_inner(page, app))
        })
    }

    pub fn render_inner(page: Rc<Self>, app: Rc<App>) -> Dom {
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
            .future(map_ref!(
                let busy = app.loader_is_loading(),
                let load = page.load_full_general_pdf.signal() =>
                !busy && *load
            ).for_each(clone!(app, page => move |ready| {
                if ready {
                    Self::load_full_pdf("full-general", page.clone(), app.clone());
                    page.load_full_general_pdf.set_neq(false);
                }
                async {}
            })))
            .future(map_ref!(
                let busy = app.loader_is_loading(),
                let load = page.load_full_labour_pdf.signal() =>
                !busy && *load
            ).for_each(clone!(app, page => move |ready| {
                if ready {
                    Self::load_full_pdf("full-labour", page.clone(), app.clone());
                    page.load_full_labour_pdf.set_neq(false);
                }
                async {}
            })))
            .future(map_ref!(
                let busy = app.loader_is_loading(),
                let load = page.load_full_psychia_pdf.signal() =>
                !busy && *load
            ).for_each(clone!(app, page => move |ready| {
                if ready {
                    Self::load_full_pdf("full-psychia", page.clone(), app.clone());
                    page.load_full_psychia_pdf.set_neq(false);
                }
                async {}
            })))
            .class(class::CARD)
            .children([
                html!("div", {
                    .class(class::CARD_HEAD_CYANS)
                    .style("line-height","34px") // for vertical-middle span
                    .child(html!("span", {
                        .class("fw-bold").text(&["AN : ", &page.an.clone().unwrap_or_default()].concat())
                    }))
                    .apply_if(page.is_full && app.endpoint_is_allow(&Method::GET, &EndPoint::ReportRawTemplateTypeId, false), |dom| {
                        dom.child(html!("span", {
                            .class("float-end")
                            .child(html!("button", {
                                .attr("type", "button")
                                .class(class::BTN_GRAY)
                                .attr("data-bs-toggle", "modal")
                                .attr("data-bs-target", "#ipd-doc-report-modal")
                                .child(html!("i", {.class(class::FA_PRINT)}))
                                .text(" พิมพ์เอกสารใบปะหน้า")
                                .event(clone!(app, page => move |_: events::Click| {
                                    page.pdf_path.set(page.an.clone().map(|an| PdfInner::new(TypstReport::from_system_with_coercion(SystemReport::IpdDocument, &app.state().report_coercions()), an, None)));
                                    page.render_pdf.set(true);
                                }))
                            }))
                            .apply_if(app.endpoint_is_allow(&Method::GET, &EndPoint::ReportTemplateTypeId, false), |dom| dom
                                .children([
                                    html!("button" => HtmlButtonElement, {
                                        .attr("type", "button")
                                        .class(class::BTN_R_GRAY)
                                        .child(html!("i", {.class(class::FA_PRINT)}))
                                        .text(" รวม ทั่วไป")
                                        .apply(mixins::click_with_loader_checked(clone!(page => move || {
                                            page.load_full_general_pdf.set(true);
                                        }), app.state()))
                                    }),
                                    html!("button" => HtmlButtonElement, {
                                        .attr("type", "button")
                                        .class(class::BTN_R_GRAY)
                                        .child(html!("i", {.class(class::FA_PRINT)}))
                                        .text(" รวม ห้องคลอด")
                                        .apply(mixins::click_with_loader_checked(clone!(page => move || {
                                            page.load_full_labour_pdf.set(true);
                                        }), app.state()))
                                    }),
                                    html!("button" => HtmlButtonElement, {
                                        .attr("type", "button")
                                        .class(class::BTN_R_GRAY)
                                        .child(html!("i", {.class(class::FA_PRINT)}))
                                        .text(" รวม จิตเวช")
                                        .apply(mixins::click_with_loader_checked(clone!(page => move || {
                                            page.load_full_psychia_pdf.set(true);
                                        }), app.state()))
                                    }),
                                ])
                            )
                        }))
                    })
                }),
                html!("div", {
                    .class("card-body")
                    .children([
                        html!("div", {
                            .child(html!("ul", {
                                .class(class::LIST_GROUP_FLUSH_OVFA)
                                .child_signal(page.result.signal_cloned().map(|res| res.has_data_summary2).map(clone!(app, page => move |is_used| {
                                    (page.is_full || is_used).then(|| check_used(0, "#ipd-doc-report-modal", "Discharge Summary", vec![
                                        PdfSource::Kphis(is_used, page.an.clone().map(|an| PdfInner::new(TypstReport::from_system_with_coercion(SystemReport::IpdSummary, &app.state().report_coercions()), an, None)), "")
                                    ], page.pdf_path.clone(), page.render_pdf.clone(), app.clone()))
                                })))
                                .child_signal(page.result.signal_cloned().map(|res| res.has_scan_consent).map(clone!(app, page, scan_template => move |is_used| {
                                    (page.is_full || is_used).then(|| check_used(0, "#ipd-doc-report-modal", "ใบยินยอม", vec![
                                        PdfSource::Scan(is_used, page.an.as_ref().map(|an| PdfInner::new(scan_template.clone(), [an,"|1|1"].concat(), Some(DocumentType::InformedConsent.label().to_owned()))), "")
                                    ], page.pdf_path.clone(), page.render_pdf.clone(), app.clone()))
                                })))
                                .child_signal(page.result.signal_cloned().map(|res| res.has_scan_refer_in).map(clone!(app, page, scan_template => move |is_used| {
                                    (page.is_full || is_used).then(|| check_used(0, "#ipd-doc-report-modal", "ใบ Refer-In", vec![
                                        PdfSource::Scan(is_used, page.an.as_ref().map(|an| PdfInner::new(scan_template.clone(), [an,"|3|1"].concat(), Some(DocumentType::ReferIn.label().to_owned()))), "")
                                    ], page.pdf_path.clone(), page.render_pdf.clone(), app.clone()))
                                })))
                                .child_signal(page.result.signal_cloned().map(|res| (res.has_data_refer_out, res.has_scan_refer_out)).map(clone!(app, page, scan_template => move |(is_used, has_scan)| {
                                    (page.is_full || is_used || has_scan).then(|| check_used(0, "#ipd-doc-report-modal", "ใบ Refer-Out", vec![
                                        PdfSource::HosXp(is_used, page.an.clone().map(|an| PdfInner::new(TypstReport::from_system_with_coercion(SystemReport::ReferOut, &app.state().report_coercions()), an, None)), ""),
                                        PdfSource::Scan(has_scan, page.an.as_ref().map(|an| PdfInner::new(scan_template.clone(), [an,"|4|1"].concat(), Some(DocumentType::ReferOut.label().to_owned()))), "")
                                    ], page.pdf_path.clone(), page.render_pdf.clone(), app.clone()))
                                })))
                                .child_signal(page.result.signal_cloned().map(|res| res.has_data_dr_admission_note).map(clone!(app, page => move |is_used| {
                                    (page.is_full || is_used).then(|| check_used(0, "#ipd-doc-report-modal", "แบบประเมินแรกรับใหม่ผู้ป่วยใน", vec![
                                        PdfSource::Kphis(is_used, page.an.clone().map(|an| PdfInner::new(TypstReport::from_system_with_coercion(SystemReport::IpdAdmissionNoteDr, &app.state().report_coercions()), an, None)), "")
                                    ], page.pdf_path.clone(), page.render_pdf.clone(), app.clone()))
                                })))
                                .child_signal(page.result.signal_cloned().map(|res| res.has_data_nurse_admission_note).map(clone!(app, page => move |is_used| {
                                    (page.is_full || is_used).then(|| check_used(0, "#ipd-doc-report-modal", "ใบการประเมินสภาพผู้ป่วยแรกรับและแผนสุขภาพ", vec![
                                        PdfSource::Kphis(is_used, page.an.clone().map(|an| PdfInner::new(TypstReport::from_system_with_coercion(SystemReport::IpdAdmissionNoteNurse, &app.state().report_coercions()), an, None)), "")
                                    ], page.pdf_path.clone(), page.render_pdf.clone(), app.clone()))
                                })))
                                .child_signal(page.result.signal_cloned().map(clone!(app, page => move |res| {
                                    (page.is_full || res.has_data_med_reconciliation || res.has_data_med_reconciliation_hosxp).then(|| check_used(0, "#ipd-doc-report-modal", "Med Reconciliation", vec![
                                        PdfSource::HosXp(res.has_data_med_reconciliation_hosxp, page.an.clone().map(|an| PdfInner::new(TypstReport::from_system_with_coercion(SystemReport::IpdMedReconciliationHosXp, &app.state().report_coercions()), an, None)), ""),
                                        PdfSource::Kphis(res.has_data_med_reconciliation, page.an.clone().map(|an| PdfInner::new(TypstReport::from_system_with_coercion(SystemReport::IpdMedReconciliation, &app.state().report_coercions()), an, None)), ""),
                                    ], page.pdf_path.clone(), page.render_pdf.clone(), app.clone()))
                                })))
                                .child_signal(page.result.signal_cloned().map(|res| res.has_data_order || res.has_data_progress_note).map(clone!(app, page => move |is_used| {
                                    (page.is_full || is_used).then(|| check_used(0, "#ipd-doc-report-modal", "Order and Progress Note", vec![
                                        PdfSource::Kphis(is_used, page.an.clone().map(|an| PdfInner::new(TypstReport::from_system_with_coercion(SystemReport::IpdOrder, &app.state().report_coercions()), an, None)), "")
                                    ], page.pdf_path.clone(), page.render_pdf.clone(), app.clone()))
                                })))
                                .child_signal(page.result.signal_cloned().map(|res| res.has_data_dr_consult).map(clone!(app, page => move |is_used| {
                                    (page.is_full || is_used).then(|| check_used(0, "#ipd-doc-report-modal", "Consulation Report", vec![
                                        PdfSource::Kphis(is_used, page.an.clone().map(|an| PdfInner::new(TypstReport::from_system_with_coercion(SystemReport::IpdConsult, &app.state().report_coercions()), an, None)), "")
                                    ], page.pdf_path.clone(), page.render_pdf.clone(), app.clone()))
                                })))
                                .child_signal(page.result.signal_cloned().map(|res| (res.has_data_operation, res.has_scan_oper)).map(clone!(app, page, scan_template => move |(is_used, has_scan)| {
                                    (page.is_full || is_used || has_scan).then(|| check_used(0, "#ipd-doc-report-modal", "Operative Report", vec![
                                        PdfSource::HosXp(is_used, None, ""),
                                        PdfSource::Scan(has_scan, page.an.as_ref().map(|an| PdfInner::new(scan_template.clone(), [an,"|12|1"].concat(), Some(DocumentType::Operation.label().to_owned()))), ""),
                                    ], page.pdf_path.clone(), page.render_pdf.clone(), app.clone()))
                                })))
                                .child_signal(page.result.signal_cloned().map(|res| res.has_scan_anes).map(clone!(app, page, scan_template => move |is_used| {
                                    (page.is_full || is_used).then(|| check_used(0, "#ipd-doc-report-modal", "Anesthetic Report", vec![
                                        PdfSource::Scan(is_used, page.an.as_ref().map(|an| PdfInner::new(scan_template.clone(), [an,"|13|1"].concat(), Some(DocumentType::Anesthesia.label().to_owned()))), "")
                                    ], page.pdf_path.clone(), page.render_pdf.clone(), app.clone()))
                                })))
                                .child_signal(page.result.signal_cloned().map(|res| (res.has_scan_labour, res.has_data_vital_sign)).map(clone!(app, page, scan_template => move |(is_scan, is_vs)| {
                                    (page.is_full || (is_scan || is_vs)).then(|| check_used(0, "#ipd-doc-report-modal", "Labour Record", vec![
                                        PdfSource::Scan(is_scan, page.an.as_ref().map(|an| PdfInner::new(scan_template.clone(), [an,"|14|1"].concat(), Some(DocumentType::Labour.label().to_owned()))), ""),
                                        PdfSource::Kphis(is_vs, page.an.clone().map(|an| PdfInner::new(TypstReport::from_system_with_coercion(SystemReport::IpdPartographWho, &app.state().report_coercions()), an, None)), "Partograph WHO"),
                                        PdfSource::Kphis(is_vs, page.an.clone().map(|an| PdfInner::new(TypstReport::from_system_with_coercion(SystemReport::IpdPartograph, &app.state().report_coercions()), an, None)), "Partograph"),
                                    ], page.pdf_path.clone(), page.render_pdf.clone(), app.clone()))
                                })))
                                .child_signal(page.result.signal_cloned().map(|res| {
                                    res.has_scan_physio
                                }).map(clone!(page => move |is_used| {
                                    (page.is_full || is_used).then(|| check_used_group(0, "Paramedical Section", is_used))
                                })))
                                .child_signal(page.result.signal_cloned().map(|res| res.has_scan_physio).map(clone!(app, page, scan_template => move |is_used| {
                                    (page.is_full || is_used).then(|| check_used(1, "#ipd-doc-report-modal", "Physiotherapy Sheet (กายภาพบำบัด)", vec![
                                        PdfSource::Scan(is_used, page.an.as_ref().map(|an| PdfInner::new(scan_template.clone(), [an,"|15|1"].concat(), Some(DocumentType::Physiotherapy.label().to_owned()))), "")
                                    ], page.pdf_path.clone(), page.render_pdf.clone(), app.clone()))
                                })))
                                .child_signal(page.result.signal_cloned().map(|res| {
                                    res.has_scan_culture || res.has_data_lab || res.has_scan_ekg ||
                                    res.has_data_xray || res.has_data_ct || res.has_data_mri ||
                                    res.has_scan_xray  || res.has_scan_ct || res.has_scan_mri || res.has_scan_special
                                }).map(clone!(page => move |is_used| {
                                    (page.is_full || is_used).then(|| check_used_group(0, "Pathology, Laboratory, X-rays Report", is_used))
                                })))
                                .child_signal(page.result.signal_cloned().map(|res| res.has_scan_culture).map(clone!(app, page, scan_template => move |is_used| {
                                    (page.is_full || is_used).then(|| check_used(1, "#ipd-doc-report-modal", "ผลการเพาะเชื้อ/ชิ้นเนื้อ", vec![
                                        PdfSource::Scan(is_used, page.an.as_ref().map(|an| PdfInner::new(scan_template.clone(), [an,"|5|1"].concat(), Some(DocumentType::CulturePatho.label().to_owned()))), "")
                                    ], page.pdf_path.clone(), page.render_pdf.clone(), app.clone()))
                                })))
                                .child_signal(page.result.signal_cloned().map(|res| res.has_data_lab).map(clone!(app, page => move |is_used| {
                                    (page.is_full || is_used).then(|| check_used(1, "#ipd-doc-report-modal", "Laboratory Report", vec![
                                        PdfSource::HosXp(is_used, page.an.clone().map(|an| PdfInner::new(TypstReport::from_system_with_coercion(SystemReport::Lab, &app.state().report_coercions()), an, None)), ""),
                                        PdfSource::Kphis(is_used, page.an.clone().map(|an| PdfInner::new(TypstReport::from_system_with_coercion(SystemReport::LabSummary, &app.state().report_coercions()), an, None)), "Lab summary"),
                                    ], page.pdf_path.clone(), page.render_pdf.clone(), app.clone()))
                                })))
                                .child_signal(page.result.signal_cloned().map(|res| res.has_scan_ekg).map(clone!(app, page, scan_template => move |is_used| {
                                    (page.is_full || is_used).then(|| check_used(1, "#ipd-doc-report-modal", "Electrocardiogram Report", vec![
                                        PdfSource::Scan(is_used, page.an.as_ref().map(|an| PdfInner::new(scan_template.clone(), [an,"|8|1"].concat(), Some(DocumentType::EKG.label().to_owned()))), "")
                                    ], page.pdf_path.clone(), page.render_pdf.clone(), app.clone()))
                                })))
                                .child_signal(page.result.signal_cloned().map(|res| (res.has_data_xray, res.has_scan_xray)).map(clone!(app, page, scan_template => move |(is_used, has_scan)| {
                                    (page.is_full || is_used || has_scan).then(|| check_used(1, "#ipd-doc-report-modal", "X-rays Report", vec![
                                        PdfSource::HosXp(is_used, None, ""),
                                        PdfSource::Scan(has_scan, page.an.as_ref().map(|an| PdfInner::new(scan_template.clone(), [an,"|9|1"].concat(), Some(DocumentType::Xray.label().to_owned()))), "")
                                    ], page.pdf_path.clone(), page.render_pdf.clone(), app.clone()))
                                })))
                                .child_signal(page.result.signal_cloned().map(|res| (res.has_data_ct, res.has_scan_ct)).map(clone!(app, page, scan_template => move |(is_used, has_scan)| {
                                    (page.is_full || is_used || has_scan).then(|| check_used(1, "#ipd-doc-report-modal", "CT scan", vec![
                                        PdfSource::HosXp(is_used, None, ""),
                                        PdfSource::Scan(has_scan, page.an.as_ref().map(|an| PdfInner::new(scan_template.clone(), [an,"|10|1"].concat(), Some(DocumentType::CT.label().to_owned()))), "")
                                    ], page.pdf_path.clone(), page.render_pdf.clone(), app.clone()))
                                })))
                                .child_signal(page.result.signal_cloned().map(|res| (res.has_data_mri, res.has_scan_mri)).map(clone!(app, page, scan_template => move |(is_used, has_scan)| {
                                    (page.is_full || is_used || has_scan).then(|| check_used(1, "#ipd-doc-report-modal", "MRI", vec![
                                        PdfSource::HosXp(is_used, None, ""),
                                        PdfSource::Scan(has_scan, page.an.as_ref().map(|an| PdfInner::new(scan_template.clone(), [an,"|11|1"].concat(), Some(DocumentType::MRI.label().to_owned()))), ""),
                                    ], page.pdf_path.clone(), page.render_pdf.clone(), app.clone()))
                                })))
                                .child_signal(page.result.signal_cloned().map(|res| res.has_scan_special).map(clone!(app, page, scan_template => move |is_used| {
                                    (page.is_full || is_used).then(|| check_used(1, "#ipd-doc-report-modal", "ผลตรวจพิเศษ", vec![
                                        PdfSource::Scan(is_used, page.an.as_ref().map(|an| PdfInner::new(scan_template.clone(), [an,"|7|1"].concat(), Some(DocumentType::SpecialLab.label().to_owned()))), "")
                                    ], page.pdf_path.clone(), page.render_pdf.clone(), app.clone()))
                                })))
                                .child_signal(page.result.signal_cloned().map(|res| {
                                    res.has_data_focus_list || res.has_data_focus_note || res.has_data_index_plan || res.has_data_vital_sign
                                }).map(clone!(page => move |is_used| {
                                    (page.is_full || is_used).then(|| check_used_group(0, "Nursing Section", is_used))
                                })))
                                .child_signal(page.result.signal_cloned().map(|res| res.has_data_focus_list).map(clone!(app, page => move |is_used| {
                                    (page.is_full || is_used).then(|| check_used(1, "#ipd-doc-report-modal", "Focus List", vec![
                                        PdfSource::Kphis(is_used, page.an.clone().map(|an| PdfInner::new(TypstReport::from_system_with_coercion(SystemReport::IpdFocusList, &app.state().report_coercions()), an, None)), "")
                                    ], page.pdf_path.clone(), page.render_pdf.clone(), app.clone()))
                                })))
                                .child_signal(page.result.signal_cloned().map(|res| res.has_data_focus_note).map(clone!(app, page => move |is_used| {
                                    (page.is_full || is_used).then(|| check_used(1, "#ipd-doc-report-modal", "Nurses' Notes", vec![
                                        PdfSource::Kphis(is_used, page.an.clone().map(|an| PdfInner::new(TypstReport::from_system_with_coercion(SystemReport::IpdFocusNote, &app.state().report_coercions()), an, None)), ""),
                                        PdfSource::Kphis(is_used, page.an.clone().map(|an| PdfInner::new(TypstReport::from_system_with_coercion(SystemReport::IpdDischargePlan, &app.state().report_coercions()), an, None)), "Discharge plan"),
                                    ], page.pdf_path.clone(), page.render_pdf.clone(), app.clone()))
                                })))
                                .child_signal(page.result.signal_cloned().map(|res| res.has_data_index_plan).map(clone!(app, page => move |is_used| {
                                    (page.is_full || is_used).then(|| check_used(1, "#ipd-doc-report-modal", "Index (Nurse Planning)", vec![
                                        PdfSource::Kphis(is_used, page.an.clone().map(|an| PdfInner::new(TypstReport::from_system_with_coercion(SystemReport::IpdIndexPlan, &app.state().report_coercions()), an, None)), "")
                                    ], page.pdf_path.clone(), page.render_pdf.clone(), app.clone()))
                                })))
                                .child_signal(page.result.signal_cloned().map(|res| res.has_data_vital_sign).map(clone!(app, page => move |is_used| {
                                    (page.is_full || is_used).then(|| check_used(1, "#ipd-doc-report-modal", "Vital Sign Record", vec![
                                        PdfSource::Kphis(is_used, page.an.clone().map(|an| PdfInner::new(TypstReport::from_system_with_coercion(SystemReport::IpdVitalSignGeneral, &app.state().report_coercions()), an, None)), "V/S"),
                                        PdfSource::Kphis(is_used, page.an.clone().map(|an| PdfInner::new(TypstReport::from_system_with_coercion(SystemReport::IpdVitalSignNeuro, &app.state().report_coercions()), an, None)), "Neuro"),
                                        PdfSource::Kphis(is_used, page.an.clone().map(|an| PdfInner::new(TypstReport::from_system_with_coercion(SystemReport::IpdVitalSignLabour, &app.state().report_coercions()), an, None)), "LR"),
                                        PdfSource::Kphis(is_used, page.an.clone().map(|an| PdfInner::new(TypstReport::from_system_with_coercion(SystemReport::IpdVitalSignPsychia, &app.state().report_coercions()), an, None)), "Psychia"),
                                    ], page.pdf_path.clone(), page.render_pdf.clone(), app.clone()))
                                })))
                                .child_signal(page.result.signal_cloned().map(|res| res.has_data_vital_sign).map(clone!(app, page => move |is_used| {
                                    (page.is_full || is_used).then(|| check_used(0, "#ipd-doc-report-modal", "Graphic (T.P.R.) Chart", vec![
                                        PdfSource::Kphis(is_used, page.an.clone().map(|an| PdfInner::new(TypstReport::from_system_with_coercion(SystemReport::IpdTPR, &app.state().report_coercions()), an, None)), ""),
                                    ], page.pdf_path.clone(), page.render_pdf.clone(), app.clone()))
                                })))
                                .child_signal(page.result.signal_cloned().map(|res| res.has_data_io).map(clone!(app, page => move |is_used| {
                                    (page.is_full || is_used).then(|| check_used(0, "#ipd-doc-report-modal", "Fluid Balance Summary", vec![
                                        PdfSource::Kphis(is_used, page.an.clone().map(|an| PdfInner::new(TypstReport::from_system_with_coercion(SystemReport::IpdIo, &app.state().report_coercions()), an, None)), "")
                                    ], page.pdf_path.clone(), page.render_pdf.clone(), app.clone()))
                                })))
                                .child_signal(page.result.signal_cloned().map(|res| res.has_data_index_plan).map(clone!(app, page => move |is_used| {
                                    (page.is_full || is_used).then(|| check_used(0, "#ipd-doc-report-modal", "Medication Administration Record (eMAR)", vec![
                                        PdfSource::Kphis(is_used, page.an.clone().map(|an| PdfInner::new(TypstReport::from_system_with_coercion(SystemReport::IpdMAR, &app.state().report_coercions()), an, None)), "")
                                    ], page.pdf_path.clone(), page.render_pdf.clone(), app.clone()))
                                })))
                                .child_signal(page.result.signal_cloned().map(|res| res.has_scan_blood).map(clone!(app, page, scan_template => move |is_used| {
                                    (page.is_full || is_used).then(|| check_used(0, "#ipd-doc-report-modal", "Blood transfusion Report (ใบของห้องเลือด)", vec![
                                        PdfSource::Scan(is_used, page.an.as_ref().map(|an| PdfInner::new(scan_template.clone(), [an,"|6|1"].concat(), Some(DocumentType::Blood.label().to_owned()))), "")
                                    ], page.pdf_path.clone(), page.render_pdf.clone(), app.clone()))
                                })))
                                .child_signal(page.result.signal_cloned().map(|res| res.has_scan_opd_card).map(clone!(app, page, scan_template => move |is_used| {
                                    (page.is_full || is_used).then(|| check_used(0, "#ipd-doc-report-modal", "OPD card", vec![
                                        PdfSource::Scan(is_used, page.an.as_ref().map(|an| PdfInner::new(scan_template.clone(), [an,"|20|1"].concat(), Some(DocumentType::OPDcard.label().to_owned()))), "")
                                    ], page.pdf_path.clone(), page.render_pdf.clone(), app.clone()))
                                })))
                                .child_signal(page.result.signal_cloned().map(|res| res.has_scan_insure).map(clone!(app, page, scan_template => move |is_used| {
                                    (page.is_full || is_used).then(|| check_used(0, "#ipd-doc-report-modal", "ใบตรวจสอบสิทธิ์", vec![
                                        PdfSource::Scan(is_used, page.an.as_ref().map(|an| PdfInner::new(scan_template.clone(), [an,"|2|1"].concat(), Some(DocumentType::InsureCheck.label().to_owned()))), "")
                                    ], page.pdf_path.clone(), page.render_pdf.clone(), app.clone()))
                                })))
                                .child_signal(page.result.signal_cloned().map(|res| res.has_scan_alt_med).map(clone!(app, page, scan_template => move |is_used| {
                                    (page.is_full || is_used).then(|| check_used(0, "#ipd-doc-report-modal", "บันทึกการแพทย์ทางเลือก", vec![
                                        PdfSource::Scan(is_used, page.an.as_ref().map(|an| PdfInner::new(scan_template.clone(), [an,"|16|1"].concat(), Some(DocumentType::AlternativeRx.label().to_owned()))), "")
                                    ], page.pdf_path.clone(), page.render_pdf.clone(), app.clone()))
                                })))
                                .child_signal(page.result.signal_cloned().map(|res| res.has_scan_nutrition).map(clone!(app, page, scan_template => move |is_used| {
                                    (page.is_full || is_used).then(|| check_used(0, "#ipd-doc-report-modal", "บันทึกโภชนาการ", vec![
                                        PdfSource::Scan(is_used, page.an.as_ref().map(|an| PdfInner::new(scan_template.clone(), [an,"|17|1"].concat(), Some(DocumentType::Nutrition.label().to_owned()))), "")
                                    ], page.pdf_path.clone(), page.render_pdf.clone(), app.clone()))
                                })))
                                .child_signal(page.result.signal_cloned().map(|res| res.has_scan_other_sp_clinic).map(clone!(app, page, scan_template => move |is_used| {
                                    (page.is_full || is_used).then(|| check_used(0, "#ipd-doc-report-modal", "Other Special Clinical Report", vec![
                                        PdfSource::Scan(is_used, page.an.as_ref().map(|an| PdfInner::new(scan_template.clone(), [an,"|19|1"].concat(), Some(DocumentType::OtherSpClinic.label().to_owned()))), "")
                                    ], page.pdf_path.clone(), page.render_pdf.clone(), app.clone()))
                                })))
                                .child_signal(page.result.signal_cloned().map(|res| res.has_scan_others).map(clone!(app, page, scan_template => move |is_used| {
                                    (page.is_full || is_used).then(|| check_used(0, "#ipd-doc-report-modal", "เอกสารอื่นๆ", vec![
                                        PdfSource::Scan(is_used, page.an.as_ref().map(|an| PdfInner::new(scan_template.clone(), [an,"|18|1"].concat(), Some(DocumentType::Others.label().to_owned()))), "")
                                    ], page.pdf_path.clone(), page.render_pdf.clone(), app.clone()))
                                })))
                                .child_signal(page.result.signal_cloned().map(|res| res.has_scan_finance).map(clone!(app, page, scan_template => move |is_used| {
                                    (page.is_full || is_used).then(|| check_used(0, "#ipd-doc-report-modal", "เอกสารใบค่าใช้จ่าย", vec![
                                        PdfSource::Scan(is_used, page.an.as_ref().map(|an| PdfInner::new(scan_template.clone(), [an,"|21|1"].concat(), Some(DocumentType::Finance.label().to_owned()))), "")
                                    ], page.pdf_path.clone(), page.render_pdf.clone(), app.clone()))
                                })))
                                .child_signal(page.result.signal_cloned().map(|res| {
                                    res.has_data_order || res.has_data_progress_note || res.has_data_dr_consult || res.has_data_index_plan || res.has_data_vital_sign || res.has_data_io || res.has_data_lab
                                }).map(clone!(app, page => move |is_used| {
                                    (page.is_full || is_used).then(|| check_used(0, "#ipd-doc-report-modal", "Event Logs", vec![
                                        PdfSource::Kphis(is_used, page.an.clone().map(|an| PdfInner::new(TypstReport::from_system_with_coercion(SystemReport::IpdEventLog, &app.state().report_coercions()), an, None)), "")
                                    ], page.pdf_path.clone(), page.render_pdf.clone(), app.clone()))
                                })))
                            }))
                        }),
                    ])
                }),
                html!("div", {
                    .class("modal")
                    .attr("id", "ipd-doc-report-modal")
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
