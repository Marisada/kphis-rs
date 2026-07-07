// ipd-dr-med-reconcile-from-hosxp.php

use dominator::{Dom, clone, html};
use futures_signals::{
    map_ref,
    signal::{Mutable, Signal, SignalExt},
    signal_vec::{MutableVec, SignalVecExt},
};
use std::rc::Rc;

use kphis_model::{
    app::VisitTypeId,
    endpoint::EndPoint,
    fetch::Method,
    med_reconcile::MedReconciliationDetail,
    patient_info::PatientInfo,
    report::{SystemReport, TypstReport},
};
use kphis_ui_app::App;
use kphis_ui_core::class;
use kphis_util::datetime::date_th_opt;

use crate::gadget::pdf_button::PdfButtons;

/// GET `EndPoint::IpdMedReconcileHosxpAn` (guarded, invisible)
#[derive(Default)]
pub struct IpdMedReconcileHosXpCpn {
    loaded: Mutable<bool>,

    patient: Mutable<Option<Rc<PatientInfo>>>,

    changed: Mutable<bool>,
    checker: Mutable<bool>,

    recons: MutableVec<Rc<MedReconciliationDetail>>,
}

impl IpdMedReconcileHosXpCpn {
    pub fn new(patient: Mutable<Option<Rc<PatientInfo>>>) -> Rc<Self> {
        Rc::new(Self { patient, ..Default::default() })
    }

    fn is_allow_load(&self, app: Rc<App>) -> impl Signal<Item = bool> + use<> {
        self.patient.signal_ref(move |opt| {
            opt.as_ref()
                .map(|pt| {
                    let is_pre_admit = pt.visit_type.is_pre_admit();
                    app.endpoint_is_allow(&Method::GET, &EndPoint::IpdMedReconcileHosxpAn, is_pre_admit)
                })
                .unwrap_or_default()
        })
    }

    fn load(page: Rc<Self>, app: Rc<App>) {
        let visit_type = page.patient.lock_ref().as_ref().map(|pt| pt.visit_type());
        match visit_type {
            Some(VisitTypeId::Ipd(an)) | Some(VisitTypeId::PreAdmit(an)) => {
                page.recons.lock_mut().clear();
                app.async_load(
                    true,
                    clone!(app => async move {
                        // GET `EndPoint::IpdMedReconcileHosxpAn`
                        match MedReconciliationDetail::call_api_get_ipd(&an, app.state()).await {
                            Ok(responses) => {
                                page.checker.set_neq(!responses.is_empty());
                                page.recons.lock_mut().extend(responses.into_iter().map(Rc::new));
                            }
                            Err(e) => {
                                app.alert_app_error(&e).await;
                            }
                        }
                    }),
                )
            }
            Some(VisitTypeId::OpdEr(_, _)) | Some(VisitTypeId::Visit(_)) | None => {}
        }
    }

    pub fn render(page: Rc<Self>, app: Rc<App>) -> Dom {
        html!("div", {
            .visible_signal(map_ref! {
                let is_empty = page.recons.signal_vec_cloned().to_signal_cloned().map(|v| v.is_empty()),
                let is_allow = page.is_allow_load(app.clone()) =>
                !is_empty && *is_allow
            })
            .future(map_ref!(
                let busy = app.loader_is_loading(),
                let is_allow = page.is_allow_load(app.clone()),
                let loaded = page.loaded.signal() =>
                !busy && *is_allow && !loaded
            ).for_each(clone!(app, page => move |ready| {
                if ready {
                    Self::load(page.clone(), app.clone());
                    page.loaded.set_neq(true);
                }
                async {}
            })))
            .class("row")
            .child(html!("div", {
                .class("col-md-12")
                .child(html!("div", {
                    .class(class::CARD)
                    .children([
                        html!("div", {
                            .class(class::CARD_HEAD)
                            .class("bg-info-subtle")
                            .text("ข้อมูลจาก HOSxP")
                        }),
                        html!("div", {
                            .class("card-body")
                            .child(html!("table", {
                                .class(class::TABLE_STRIP)
                                .children([
                                    html!("thead", {
                                        .child(html!("tr", {
                                            .children([
                                                html!("th", {.attr("scope", "col").class("text-center").text("ลำดับ")}),
                                                html!("th", {.attr("scope", "col").text("ชื่อยาที่ผู้ป่วยใช้ประจำก่อน Admit").child(html!("br"))}),
                                                html!("th", {.attr("scope", "col").text("วิธีใช้")}),
                                                html!("th", {.attr("scope", "col").text("จำนวน")}),
                                                html!("th", {.attr("scope", "col").text("ที่มาของยา")}),
                                                html!("th", {.attr("scope", "col").text("วันที่สุดท้ายที่ได้ยา")}),
                                            ])
                                        }))
                                    }),
                                    html!("tbody", {
                                        .children_signal_vec(page.recons.signal_vec_cloned().enumerate().map(|(i,row)| {
                                            Self::render_recon(row, i.get().unwrap_or_default())
                                        }))
                                    }),
                                ])
                            }))
                        }),
                    ])
                    .child_signal(page.patient.signal_cloned().map(clone!(page, app => move |opt| {
                        opt.and_then(clone!(page, app => move |patient| {
                            match patient.visit_type() {
                                VisitTypeId::Ipd(an)
                                | VisitTypeId::PreAdmit(an) => {
                                    Some(html!("div", {
                                        .class(class::CARD_FOOT_R)
                                        .children(PdfButtons::buttons(
                                            PdfButtons::new(
                                                TypstReport::from_system_with_coercion(SystemReport::IpdMedReconciliationHosXp, &app.state().report_coercions()),
                                                Mutable::new(an.clone()),
                                                page.checker.clone(),
                                                page.changed.clone(),
                                                move || {serde_json::json!({
                                                    "id": an,
                                                    "patient": patient,
                                                    "recon":  page.recons.lock_ref().to_vec(),
                                                }).to_string()}
                                            ), "Print PDF", None, app
                                        ))
                                    }))
                                }
                                VisitTypeId::OpdEr(_, _)
                                | VisitTypeId::Visit(_) => None,
                            }
                        }))
                    })))
                }))
            }))
        })
    }

    fn render_recon(row: Rc<MedReconciliationDetail>, i: usize) -> Dom {
        html!("tr", {
            .children([
                html!("td", {
                    .class("text-center")
                    .text(&(i + 1).to_string())
                }),
                html!("td", {
                    .text(&row.medication_name.clone().unwrap_or_default())
                }),
                html!("td", {
                    .text(&row.usage_name.clone().unwrap_or_default())
                }),
                html!("td", {
                    .text(&row.qty.map(|i| i.to_string()).unwrap_or_default())
                }),
                html!("td", {
                    .text(&row.receive_location.clone().unwrap_or_default())
                }),
                html!("td", {
                    .text(&date_th_opt(&row.last_receive_date))
                }),
            ])
        })
    }
}
