use dominator::{Dom, EventOptions, clone, events, html};
use futures_signals::{
    signal::{Mutable, Signal, SignalExt},
    signal_vec::MutableVec,
};
use std::rc::Rc;

use kphis_model::{endpoint::EndPoint, fetch::Method, patient_info::PatientInfo};
use kphis_ui_app::App;
use kphis_ui_core::class;

use crate::nurse_note::{dc_plan::DcPlanCpn, focus_list::FocusListCpn, focus_note::FocusNoteCpn, note_list::NoteListCpn};

#[derive(Clone, Default, PartialEq)]
enum Tab {
    #[default]
    Note,
    Focus,
    DcPlan,
}

/// - GET `EndPoint::IpdFocusListAn` (FocusListCpn / FocusNoteCpn)
/// - GET `EndPoint::OpdErFocusListId` (FocusListCpn / FocusNoteCpn)
/// - GET `EndPoint::IpdFocusNoteAn` (FocusNoteCpn)
/// - GET `EndPoint::OpdErFocusNoteId` (FocusNoteCpn)
/// - GET `EndPoint::IpdDcPlanAn` (DcPlanCpn, guarded, remove 'Discharge Plan' tab)
/// - GET `EndPoint::OpdErDcPlanId` (DcPlanCpn, guarded, remove 'Discharge Plan' tab)
/// - GET `EndPoint::IpdDcPlanTmpDx` (DcPlanCpn, guarded, remove 'Discharge Plan' tab)
/// - GET `EndPoint::IpdDcPlanTmpMed` (DcPlanCpn, guarded, remove 'Discharge Plan' tab)
/// - GET `EndPoint::IpdDcPlanTmpEnv` (DcPlanCpn, guarded, remove 'Discharge Plan' tab)
/// - GET `EndPoint::IpdDcPlanTmpTx` (DcPlanCpn, guarded, remove 'Discharge Plan' tab)
/// - GET `EndPoint::IpdDcPlanTmpDiet` (DcPlanCpn, guarded, remove 'Discharge Plan' tab)
#[derive(Clone, Default)]
pub struct NurseNoteCpn {
    active_tab: Mutable<Tab>,
    view_by: Mutable<String>,

    patient: Mutable<Option<Rc<PatientInfo>>>,
}

impl NurseNoteCpn {
    pub fn new(patient: Mutable<Option<Rc<PatientInfo>>>, view_by: Mutable<String>) -> Rc<Self> {
        Rc::new(Self {
            patient,
            view_by,
            ..Default::default()
        })
    }

    fn is_ipd_and_is_pre_admit(&self) -> impl Signal<Item = (bool, bool)> + use<> {
        self.patient.signal_ref(|opt| opt.as_ref().map(|pt| pt.visit_type.is_ipd_and_is_pre_admit()).unwrap_or_default())
    }

    pub fn render(page: Rc<Self>, app: Rc<App>) -> Dom {
        let content = NoteListCpn::new(MutableVec::new(), Mutable::new(false), page.patient.clone(), Mutable::new(true));

        if page.view_by.lock_ref().as_str() != "nurse" {
            html!("div", {
                .class("row")
                .child(NoteListCpn::render(content.clone(), app.clone(), None))
            })
        } else {
            html!("div", {
                // .class("container-fluid")
                .children([
                    html!("nav", {
                        .child(html!("div", {
                            .class(class::NAV_TABS_T)
                            .attr("role","tablist")
                            .children([
                                html!("a", {
                                    .class(class::NAV_ITEM_LINK_P2)
                                    .attr("id", "nav-focus-list-tab")
                                    .attr("data-bs-toggle","pill")
                                    .attr("href","#")
                                    .text("Focus List")
                                    .event_with_options(&EventOptions::preventable(), clone!(page => move |event: events::Click| {
                                        event.prevent_default();
                                        page.active_tab.set_neq(Tab::Focus);
                                    }))
                                }),
                                html!("a", {
                                    .class(class::NAV_ITEM_LINK_ACTIVE_P2)
                                    .attr("id", "nav-nurse-progress-note-tab")
                                    .attr("data-bs-toggle","pill")
                                    .attr("href","#")
                                    .text("Nursing Progress Note")
                                    .event_with_options(&EventOptions::preventable(), clone!(page => move |event: events::Click| {
                                        event.prevent_default();
                                        page.active_tab.set_neq(Tab::Note);
                                    }))
                                }),
                            ])
                            .child_signal(page.is_ipd_and_is_pre_admit().map(clone!(app, page => move |(is_ipd, is_pre_admit)| {
                                (if is_ipd {
                                    app.endpoint_is_allow(&Method::GET, &EndPoint::IpdDcPlanAn, is_pre_admit)
                                } else {
                                    app.endpoint_is_allow(&Method::GET, &EndPoint::OpdErDcPlanId, false)
                                }
                                    && app.endpoint_is_allow(&Method::GET, &EndPoint::IpdDcPlanTmpDx, false)
                                    && app.endpoint_is_allow(&Method::GET, &EndPoint::IpdDcPlanTmpMed, false)
                                    && app.endpoint_is_allow(&Method::GET, &EndPoint::IpdDcPlanTmpEnv, false)
                                    && app.endpoint_is_allow(&Method::GET, &EndPoint::IpdDcPlanTmpTx, false)
                                    && app.endpoint_is_allow(&Method::GET, &EndPoint::IpdDcPlanTmpDiet, false)
                                ).then(|| {
                                    html!("a", {
                                        .class(class::NAV_ITEM_LINK_P2)
                                        .attr("id", "nav-discharge-plan-tab")
                                        .attr("data-bs-toggle","pill")
                                        .attr("href","#")
                                        .text("Discharge Plan")
                                        .event_with_options(&EventOptions::preventable(), clone!(page => move |event: events::Click| {
                                            event.prevent_default();
                                            page.active_tab.set_neq(Tab::DcPlan);
                                        }))
                                    })
                                })
                            })))
                        }))
                    }),
                    html!("div", {
                        .class(class::ROW_TC)
                        .child_signal(page.active_tab.signal_cloned().map(clone!(app, page => move |tab| {
                            Some(match tab {
                                Tab::Focus => {
                                    FocusListCpn::render(FocusListCpn::new(page.patient.clone()), app.clone())
                                }
                                Tab::Note => {
                                    FocusNoteCpn::render(FocusNoteCpn::new(page.patient.clone()), app.clone())
                                }
                                Tab::DcPlan => {
                                    DcPlanCpn::render(DcPlanCpn::new(page.patient.clone()), app.clone())
                                }
                            })
                        })))
                    }),
                ])
            })
        }
    }
}
