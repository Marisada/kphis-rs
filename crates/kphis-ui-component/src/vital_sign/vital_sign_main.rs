use dominator::{Dom, clone, html};
use futures_signals::signal::{Mutable, SignalExt};
use std::rc::Rc;

use kphis_model::{patient_info::PatientInfo, user::permission::Permission, vital_sign::VitalSign};
use kphis_ui_app::App;
use kphis_util::util::zero_none;

use crate::vital_sign::{vital_sign_data::VitalSignDataCpn, vital_sign_form::VitalSignFormCpn};

/// - GET `EndPoint::IpdVitalSign` (VitalSignDataCpn)
/// - GET `EndPoint::OpdErVitalSign` (VitalSignDataCpn)
#[derive(Clone, Default)]
pub struct VitalSignCpn {
    view_by: Mutable<String>,
    patient: Mutable<Option<Rc<PatientInfo>>>,
    patient_loaded: Mutable<bool>,

    vs_changed: Mutable<bool>,
    vs_result: Mutable<Vec<Rc<VitalSign>>>,
    vs_data: Mutable<Option<Rc<VitalSignDataCpn>>>,
    start_vs_date: Mutable<String>,
    end_vs_date: Mutable<String>,

    pub vs_id: Mutable<u32>,
    pub form_rendered: Mutable<bool>,
    form: Mutable<Option<Rc<VitalSignFormCpn>>>,
}

impl VitalSignCpn {
    pub fn new(patient: Mutable<Option<Rc<PatientInfo>>>, patient_loaded: Mutable<bool>, view_by: Mutable<String>) -> Rc<Self> {
        Rc::new(Self {
            patient,
            patient_loaded,
            view_by,
            ..Default::default()
        })
    }

    pub fn render(page: Rc<Self>, app: Rc<App>) -> Dom {
        let can_use_form = page.view_by.lock_ref().as_str() == "nurse"
            && (app.has_permission(Permission::VitalSignAdd)
                || app.has_permission(Permission::VitalSignEdit)
                || app.has_permission(Permission::OpdErVitalSignAdd)
                || app.has_permission(Permission::OpdErVitalSignEdit));

        html!("div", {
            .future(page.patient_loaded.signal().for_each(clone!(page => move |loaded| {
                if loaded {
                    page.vs_changed.set(true);
                    page.form_rendered.set(false);
                }
                async {}
            })))
            .future(page.vs_changed.signal().for_each(clone!(app, page => move |changed| {
                if changed {
                    let empty_visit = page.patient.lock_ref().as_ref().map(|pt| pt.visit_type.is_empty()).unwrap_or_default();
                    let vs_data_opt = (!empty_visit).then(|| {
                        VitalSignDataCpn::new(
                            page.patient.clone(),
                            page.vs_result.clone(),
                            page.start_vs_date.clone(),
                            page.end_vs_date.clone(),
                            app.vs_mode.clone(),
                        )
                    });
                    page.vs_data.set(vs_data_opt);
                    page.vs_changed.set(false);
                }
                async {}
            })))
            .future(page.form_rendered.signal().for_each(clone!(app, page => move |done| {
                if !done {
                    let empty_visit = page.patient.lock_ref().as_ref().map(|pt| pt.visit_type.is_empty()).unwrap_or_default();
                    let form = (!empty_visit).then(|| {
                        let vs_opt = zero_none(page.vs_id.get()).and_then(|id| {
                            page.vs_result.lock_ref().iter().find(|vs| vs.vs_id == id).cloned()
                        });
                        VitalSignFormCpn::new(
                            page.patient.clone(),
                            vs_opt,
                            app.clone(),
                        )
                    });
                    page.form.set(form);
                    page.form_rendered.set(true);
                }
                async {}
            })))
            .class("row")
            .child(html!("div", {
                .apply_if(can_use_form, |dom| dom.style("width","calc(100% - 420px)"))
                .style("overflow-x","auto")
                .child_signal(page.vs_data.signal_cloned().map(clone!(app, page => move |vs_data_opt| {
                    vs_data_opt.as_ref().map(clone!(app, page => move |vs_data| VitalSignDataCpn::render(page.vs_id.clone(), page.form_rendered.clone(), vs_data.clone(), app)))
                })))
            }))
            .apply_if(can_use_form, |dom| dom
                .child_signal(page.form.signal_cloned().map(clone!(app, page => move |form_opt| {
                    form_opt.as_ref().map(clone!(app, page => move |form| VitalSignFormCpn::render(page.vs_id.clone(), page.vs_changed.clone(), page.form_rendered.clone(), form.clone(), app)))
                })))
            )
        })
    }
}
