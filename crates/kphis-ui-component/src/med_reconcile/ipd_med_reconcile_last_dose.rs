// ipd-dr-med-reconcile-dr-admission-note-last-dose.php

use dominator::{Dom, clone, html};
use futures_signals::{
    map_ref,
    signal::{Mutable, Signal, SignalExt},
};
use kphis_util::datetime::datetime_str_th_relative;
use std::rc::Rc;

use kphis_model::{
    app::VisitTypeId,
    endpoint::EndPoint,
    fetch::Method,
    {med_reconcile::AdmissionNoteLastDose, patient_info::PatientInfo},
};
use kphis_ui_app::App;
use kphis_ui_core::class;

/// GET `EndPoint::IpdMedReconcileLastDoseAn` (guarded, invisible)
#[derive(Default)]
pub struct IpdMedReconcileLastDoseCpn {
    loaded: Mutable<bool>,

    patient: Mutable<Option<Rc<PatientInfo>>>,

    last_dose_taken_time: Mutable<String>,
    last_dose_taken_remark: Mutable<String>,
}

impl IpdMedReconcileLastDoseCpn {
    pub fn new(patient: Mutable<Option<Rc<PatientInfo>>>) -> Rc<Self> {
        Rc::new(Self { patient, ..Default::default() })
    }

    fn is_allow_load(&self, app: Rc<App>) -> impl Signal<Item = bool> + use<> {
        self.patient.signal_ref(move |opt| {
            opt.as_ref()
                .map(|pt| {
                    let is_pre_admit = pt.visit_type.is_pre_admit();
                    app.endpoint_is_allow(&Method::GET, &EndPoint::IpdMedReconcileLastDoseAn, is_pre_admit)
                })
                .unwrap_or_default()
        })
    }

    fn load(page: Rc<Self>, app: Rc<App>) {
        let visit_type = page.patient.lock_ref().as_ref().map(|pt| pt.visit_type());
        match visit_type {
            Some(VisitTypeId::Ipd(an)) | Some(VisitTypeId::PreAdmit(an)) => app.async_load(
                true,
                clone!(app => async move {
                    // GET `EndPoint::IpdMedReconcileLastDoseAn`
                    match AdmissionNoteLastDose::call_api_get(&an, app.state()).await {
                        Ok(Some(response)) => {
                            page.last_dose_taken_time.set_neq(response.last_dose_taken_time.map(|t| t.to_string()).unwrap_or_default());
                            page.last_dose_taken_remark.set_neq(response.last_dose_taken_remark.clone().unwrap_or_default());
                        }
                        Ok(None) => {
                            page.last_dose_taken_time.set_neq(String::new());
                            page.last_dose_taken_remark.set_neq(String::new());
                        }
                        Err(e) => {
                            app.alert_app_error(&e).await;
                        }
                    }
                }),
            ),
            Some(VisitTypeId::OpdEr(_, _)) | Some(VisitTypeId::Visit(_)) | None => {}
        }
    }

    pub fn render(page: Rc<Self>, app: Rc<App>) -> Dom {
        html!("div", {
            .visible_signal(map_ref!{
                let is_empty = page.last_dose_taken_time.signal_cloned().map(|t| t.is_empty()),
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
            .class(class::ROW_T)
            .child(html!("div", {
                .class("col")
                .child(html!("div", {
                    .class("card")
                    .children([
                        html!("div", {
                            .class(class::CARD_HEAD)
                            .class("bg-success-subtle")
                            .text("เวลาที่กินยามื้อสุดท้าย (จากแบบบันทึกการรับใหม่ผู้ป่วยใน)")
                        }),
                        html!("div", {
                            .class("card-body")
                            .children([
                                html!("div", {
                                    .children([
                                        html!("span", {
                                            .class("fw-bold")
                                            .text("เวลาที่กินยามื้อสุดท้าย : ")
                                        }),
                                        html!("span", {
                                            .style("white-space","pre-wrap")
                                            .text_signal(page.last_dose_taken_time.signal_ref(|s| datetime_str_th_relative(s)))
                                        }),
                                    ])
                                }),
                                html!("div", {
                                    .children([
                                        html!("span", {
                                            .class("fw-bold")
                                            .text("หมายเหตุ : ")
                                        }),
                                        html!("span", {
                                            .style("white-space","pre-wrap")
                                            .text_signal(page.last_dose_taken_remark.signal_cloned())
                                        }),
                                    ])
                                }),
                            ])
                        }),
                    ])
                }))
            }))
        })
    }
}
