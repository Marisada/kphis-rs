// ipd-dr-med-reconcile-remed.php
// ipd-dr-med-reconcile-remed-visit-data.php
// ipd-dr-med-reconcile-remed-med-data.php an/vn
use dominator::{Dom, clone, events, html};
use futures_signals::{
    map_ref,
    signal::{Mutable, Signal, SignalExt, not},
    signal_vec::{MutableVec, SignalVecExt},
};
use std::rc::Rc;
use web_sys::HtmlButtonElement;

use kphis_model::{
    app::VisitTypeId,
    med_reconcile::{MedReconciliation, MedReconciliationItemSave, MedReconciliationParams, ReMedMedication, ReMedVisit},
    patient_info::PatientInfo,
};
use kphis_ui_app::App;
use kphis_ui_core::{class, doms, mixins};
use kphis_util::{
    datetime::{date_th_opt, time_hm_opt},
    util::{sanity_dot_space, str_some, zero_none},
};

/// - GET `EndPoint::IpdMedReconcileRemedVisitHn`
/// - GET `EndPoint::IpdMedReconcileRemedMed`
/// - POST `EndPoint::IpdMedReconcile`
/// - POST `EndPoint::OpdErMedReconcile`
#[derive(Default)]
pub struct MedReconcileRemed {
    // from parent
    this_visible: Mutable<bool>,
    parent_loaded: Mutable<bool>,
    loaded_med_reconciliation_has_data: Mutable<bool>,

    loaded_visit: Mutable<bool>,
    loaded_opd: Mutable<bool>,
    loaded_ipd: Mutable<bool>,

    patient: Mutable<Option<Rc<PatientInfo>>>,

    search_vn: Mutable<String>,
    search_an: Mutable<String>,

    visit_selected: Mutable<usize>,
    opd_med_selected: MutableVec<String>,
    ipd_med_selected: MutableVec<String>,

    visits: MutableVec<Rc<ReMedVisit>>,
    opd_meds: MutableVec<Rc<ReMedMedication>>,
    ipd_meds: MutableVec<Rc<ReMedMedication>>,
    selected_meds: MutableVec<Rc<ReMedMedication>>,
}

impl MedReconcileRemed {
    pub fn new(patient: Mutable<Option<Rc<PatientInfo>>>, this_visible: Mutable<bool>, parent_loaded: Mutable<bool>, loaded_med_reconciliation_has_data: Mutable<bool>) -> Rc<Self> {
        Rc::new(Self {
            patient,
            this_visible,
            parent_loaded,
            loaded_med_reconciliation_has_data,
            ..Default::default()
        })
    }

    fn has_selected(&self) -> impl Signal<Item = bool> + use<> {
        self.selected_meds.signal_vec_cloned().to_signal_cloned().map(|selecteds| !selecteds.is_empty())
    }

    fn load_visit(modal: Rc<Self>, app: Rc<App>) {
        let hn_opt = modal.patient.lock_ref().as_ref().and_then(|pt| pt.hn());
        if let Some(hn) = hn_opt {
            app.async_load(
                true,
                clone!(app => async move {
                    // GET `EndPoint::IpdMedReconcileRemedVisitHn`
                    match ReMedVisit::call_api_get(&hn, app.state()).await {
                        Ok(responses) => {
                            modal.visits.lock_mut().extend(responses.into_iter().map(Rc::new));
                        }
                        Err(e) => {
                            app.alert_app_error(&e).await;
                        }
                    }
                }),
            )
        }
    }

    fn load_opd_med(modal: Rc<Self>, app: Rc<App>) {
        if let Some(vn) = str_some(modal.search_vn.get_cloned()) {
            modal.opd_meds.lock_mut().clear();
            let params = MedReconciliationParams { vn: Some(vn), ..Default::default() };
            app.async_load(
                true,
                clone!(app => async move {
                    // GET `EndPoint::IpdMedReconcileRemedMed`
                    match ReMedMedication::call_api_get(&params, app.state()).await {
                        Ok(responses) => {
                            modal.opd_meds.lock_mut().extend(responses.into_iter().map(Rc::new));
                        }
                        Err(e) => {
                            app.alert_app_error(&e).await;
                        }
                    }
                }),
            )
        }
    }

    fn load_ipd_med(modal: Rc<Self>, app: Rc<App>) {
        if let Some(an) = str_some(modal.search_an.get_cloned()) {
            modal.ipd_meds.lock_mut().clear();
            let params = MedReconciliationParams { an: Some(an), ..Default::default() };
            app.async_load(
                true,
                clone!(app => async move {
                    // GET `EndPoint::IpdMedReconcileRemedMed`
                    match ReMedMedication::call_api_get(&params, app.state()).await {
                        Ok(responses) => {
                            modal.ipd_meds.lock_mut().extend(responses.into_iter().map(Rc::new));
                        }
                        Err(e) => {
                            app.alert_app_error(&e).await;
                        }
                    }
                }),
            )
        }
    }

    // ipd-dr-med-reconcile-save.php
    fn save(modal: Rc<Self>, app: Rc<App>) {
        let visit_type_opt = modal.patient.lock_ref().as_ref().map(|pt| pt.visit_type());
        if let Some(visit_type) = visit_type_opt {
            let hospital_name = app.app_status.lock_ref().as_ref().map(|api| api.hospital_name.clone());
            let medrec_icode = app.state().hosxp_medrec_icode();
            let items = modal
                .selected_meds
                .lock_ref()
                .iter()
                .map(|med| {
                    let (med_name, custom_med_name, usage) = if med.icode == medrec_icode {
                        (
                            None,
                            med.name1.clone(),
                            Some([&med.name2.clone().unwrap_or_default(), " ", &med.name3.clone().unwrap_or_default()].concat()),
                        )
                    } else {
                        (med.item_name.clone(), None, med.usage.clone())
                    };
                    MedReconciliationItemSave {
                        icode: med.icode.clone(),
                        med_name,
                        custom_med_name,
                        receive_from: hospital_name.clone(),
                        receive_date: med.rxdate,
                        old_drugusage: usage.map(|usage| sanity_dot_space(&usage)),
                        receive_qty: med.qty,
                    }
                })
                .collect::<Vec<MedReconciliationItemSave>>();

            app.async_load(
                true,
                clone!(app, modal => async move {
                    let result_opt = match visit_type {
                        VisitTypeId::Ipd(an)
                        | VisitTypeId::PreAdmit(an) => {
                            let params = MedReconciliationParams {
                                an: Some(an.to_owned()),
                                ..Default::default()
                            };
                            // POST `EndPoint::IpdMedReconcile`
                            Some(MedReconciliation::call_api_post(true, &items, &params, app.state()).await)
                        }
                        VisitTypeId::OpdEr(_, opd_er_order_master_id) => {
                            let params = MedReconciliationParams {
                                opd_er_order_master_id: zero_none(opd_er_order_master_id),
                                ..Default::default()
                            };
                            // POST `EndPoint::OpdErMedReconcile`
                            Some(MedReconciliation::call_api_post(false, &items, &params, app.state()).await)
                        }
                        VisitTypeId::Visit(_) => None,
                    };

                    if let Some(result) = result_opt {
                        match result {
                            Ok((_id, responses)) => {
                                app.alert_execute_responses(&responses, async move {
                                    // app.alert("บันทึกข้อมูลเรียบร้อย","");
                                    modal.loaded_med_reconciliation_has_data.set_neq(false);
                                    modal.parent_loaded.set(false);
                                    modal.this_visible.set(false);
                                }).await;
                            }
                            Err(e) => {
                                app.alert_app_error(&e).await;
                            }
                        }
                    }
                }),
            );
        }
    }

    pub fn render(modal: Rc<Self>, app: Rc<App>) -> Dom {
        html!("div", {
            .future(map_ref!(
                let busy = app.loader_is_loading(),
                let loaded = modal.loaded_visit.signal() =>
                !busy && !loaded
            ).for_each(clone!(app, modal => move |ready| {
                if ready {
                    Self::load_visit(modal.clone(), app.clone());
                    modal.loaded_visit.set_neq(true);
                }
                async {}
            })))
            .future(map_ref!(
                let busy = app.loader_is_loading(),
                let loaded = modal.loaded_opd.signal() =>
                !busy && !loaded
            ).for_each(clone!(app, modal => move |ready| {
                if ready {
                    clone!(app, modal => Self::load_opd_med(modal, app));
                    modal.loaded_opd.set_neq(true);
                }
                async {}
            })))
            .future(map_ref!(
                let busy = app.loader_is_loading(),
                let loaded = modal.loaded_ipd.signal() =>
                !busy && !loaded
            ).for_each(clone!(app, modal => move |ready| {
                if ready {
                    Self::load_ipd_med(modal.clone(), app.clone());
                    modal.loaded_ipd.set_neq(true);
                }
                async {}
            })))
            .class(class::MODAL_DIALOG_FULL)
            //.class(["mw-100","w-100","px-3"])
            .attr("role", "document")
            .child(html!("div", {
                .class("modal-content")
                .children([
                    html!("div", {
                        .class("modal-header")
                        .children([
                            html!("h5", {
                                .class("modal-title")
                                .text("Med Reconciliation: Remed")
                            }),
                            doms::close_modal_x_btn(),
                        ])
                    }),
                    html!("div", {
                        .class("modal-body")
                        //.attr("id", "medReconciliationRemedModalBody")
                        .child(html!("div", {
                            .class("row")
                            .children([
                                html!("div", {
                                    .class("col-md-2")
                                    .style("height","calc(100vh - 170px)")
                                    .style("overflow-y","auto")
                                    .child(html!("table", {
                                        .class(class::TABLE_SM)
                                        .children([
                                            html!("caption", {
                                                .style("caption-side","top")
                                                .text("Visit")
                                            }),
                                            html!("thead", {
                                                .children([
                                                    html!("tr", {.child(html!("th", {.visible(false).attr("scope", "col").text("#")}))}),
                                                    html!("tr", {.child(html!("th", {.attr("scope", "col").text("วันที่ | เวลา")}))}),
                                                ])
                                            }),
                                            html!("tbody", {
                                                .children_signal_vec(modal.visits.signal_vec_cloned().enumerate().map(clone!(modal => move |(i,visit)| {
                                                    let id = i.get().unwrap_or_default();
                                                    let callback = clone!(modal, visit => move || {
                                                        modal.opd_meds.lock_mut().clear();
                                                        modal.ipd_meds.lock_mut().clear();
                                                        if visit.ipd_home_med_item_exists {
                                                            if let Some(an) = visit.an.clone() {
                                                                modal.search_an.set_neq(an);
                                                                modal.loaded_ipd.set(false);
                                                            }
                                                        }
                                                        if visit.opd_item_exists {
                                                            if let Some(vn) = visit.vn.clone() {
                                                                modal.search_vn.set_neq(vn);
                                                                modal.loaded_opd.set(false);
                                                            }
                                                        }
                                                        modal.visit_selected.set(id);
                                                    });
                                                    if id == 0 {
                                                        callback();
                                                    }
                                                    html!("tr", {
                                                        .style("cursor","pointer")
                                                        .children([
                                                            html!("td", {.visible(false).text(&(id + 1).to_string())}),
                                                            html!("td", {
                                                                .class_signal("bg-info", modal.visit_selected.signal_cloned().map(move |x| x == id))
                                                                //.apply_if(visit.an.is_some(), |dom| dom.class(class::BOLD_RED_L))
                                                                .text(&[date_th_opt(&visit.vstdate), time_hm_opt(&visit.vsttime)].join(" "))
                                                                .apply_if(visit.opd_item_exists, |dom| dom.child(html!("span", {
                                                                    .class(class::BADGE_WRAP_R_GRAY)
                                                                    .style("cursor","default")
                                                                    .text("OPD HM")
                                                                })))
                                                                .apply_if(visit.ipd_home_med_item_exists, |dom| dom.child(html!("span", {
                                                                    .class(class::BADGE_WRAP_R_RED)
                                                                    .style("cursor","default")
                                                                    .text("IPD HM")
                                                                })))
                                                            })
                                                        ])
                                                        .event(move |_:events::Click| {
                                                            callback();
                                                        })
                                                    })
                                                })))
                                            }),
                                        ])
                                    }))
                                }),
                                html!("div", {
                                    .class("col-md-5")
                                    .style("height","calc(100vh - 170px)")
                                    .style("overflow-y","auto")
                                    .children([
                                        html!("div", {
                                            //.attr("id", "opd_med_table_div")
                                            .visible_signal(not(modal.opd_meds.signal_vec_cloned().is_empty()))
                                            .child(html!("table", {
                                                .class(class::TABLE_SM)
                                                .children([
                                                    html!("caption", {
                                                        .style("caption-side","top")
                                                        .text("OPD Med")
                                                    }),
                                                    html!("thead", {
                                                        .child(html!("tr", {
                                                            .children([
                                                                html!("th", {.attr("scope", "col").text("#")}),
                                                                html!("th", {.attr("scope", "col").text("ชื่อยา")}),
                                                                html!("th", {.attr("scope", "col").text("วิธีใช้")}),
                                                                html!("th", {.attr("scope", "col").text("จำนวน").class("text-end")}),
                                                                html!("th", {.attr("scope", "col").text("วันที่ได้รับยา")}),
                                                            ])
                                                        }))
                                                    }),
                                                    html!("tbody", {
                                                        .children_signal_vec(modal.opd_meds.signal_vec_cloned().enumerate().map(clone!(app, modal => move |(i,med)| {
                                                            let medrec_icode = app.state().hosxp_medrec_icode();
                                                            let (item_name, usage) = if med.icode == medrec_icode {
                                                                (med.name1.clone(), Some([&med.name2.clone().unwrap_or_default(), " ", &med.name3.clone().unwrap_or_default()].concat()))
                                                            } else {
                                                                (med.item_name.clone(), med.usage.clone())
                                                            };
                                                            html!("tr", {
                                                                .style("cursor","pointer")
                                                                .class_signal("bg-info", modal.opd_med_selected.signal_vec_cloned().to_signal_cloned().map(clone!(med => move |ids| ids.contains(&med.hos_guid))))
                                                                .children([
                                                                    html!("td", {.text(&(i.get().unwrap_or_default() + 1).to_string()).class("text-center")}),
                                                                    html!("td", {.text(&item_name.unwrap_or_default())}),
                                                                    html!("td", {.text(&usage.as_ref().map(|s| sanity_dot_space(s)).unwrap_or_default())}),
                                                                    html!("td", {.text(&med.qty.map(|n| n.to_string()).unwrap_or_default()).class("text-end")}),
                                                                    html!("td", {.text(&date_th_opt(&med.rxdate)).class("text-center")}),
                                                                ])
                                                                .event(clone!(modal, med => move |_:events::Click| {
                                                                    let has_guid = modal.opd_med_selected.lock_ref().contains(&med.hos_guid);
                                                                    if has_guid {
                                                                        modal.opd_med_selected.lock_mut().retain(|uid| uid != &med.hos_guid);
                                                                        modal.selected_meds.lock_mut().retain(|m| m.hos_guid != med.hos_guid);
                                                                    } else {
                                                                        modal.opd_med_selected.lock_mut().push_cloned(med.hos_guid.clone());
                                                                        modal.selected_meds.lock_mut().push_cloned(med.clone());
                                                                    }
                                                                }))
                                                            })
                                                        })))
                                                    }),
                                                ])
                                            }))
                                        }),
                                        html!("div", {
                                            //.attr("id", "ipd_med_table_div")
                                            .visible_signal(not(modal.ipd_meds.signal_vec_cloned().is_empty()))
                                            .child(html!("table", {
                                                .class(class::TABLE_SM)
                                                .children([
                                                    html!("caption", {
                                                        .style("caption-side","top")
                                                        .text("IPD Home Med")
                                                    }),
                                                    html!("thead", {
                                                        .child(html!("tr", {
                                                            .children([
                                                                html!("th", {.attr("scope", "col").text("#")}),
                                                                html!("th", {.attr("scope", "col").text("ชื่อยา")}),
                                                                html!("th", {.attr("scope", "col").text("วิธีใช้")}),
                                                                html!("th", {.attr("scope", "col").text("จำนวน").class("text-end")}),
                                                                html!("th", {.attr("scope", "col").text("วันที่ได้รับยา")}),
                                                            ])
                                                        }))
                                                    }),
                                                    html!("tbody", {
                                                        .children_signal_vec(modal.ipd_meds.signal_vec_cloned().enumerate().map(clone!(app, modal => move |(i,med)| {
                                                            let medrec_icode = app.state().hosxp_medrec_icode();
                                                            let (item_name, usage) = if med.icode == medrec_icode {
                                                                (med.name1.clone(), Some([&med.name2.clone().unwrap_or_default(), " ", &med.name3.clone().unwrap_or_default()].concat()))
                                                            } else {
                                                                (med.item_name.clone(), med.usage.clone())
                                                            };
                                                            html!("tr", {
                                                                .style("cursor","pointer")
                                                                .class_signal("bg-info", modal.ipd_med_selected.signal_vec_cloned().to_signal_cloned().map(clone!(med => move |ids| ids.contains(&med.hos_guid))))
                                                                .children([
                                                                    html!("td", {.text(&(i.get().unwrap_or_default() + 1).to_string()).class("text-center")}),
                                                                    html!("td", {.text(&item_name.unwrap_or_default())}),
                                                                    html!("td", {.text(&usage.as_ref().map(|s| sanity_dot_space(s)).unwrap_or_default())}),
                                                                    html!("td", {.text(&med.qty.map(|n| n.to_string()).unwrap_or_default()).class("text-end")}),
                                                                    html!("td", {.text(&date_th_opt(&med.rxdate)).class("text-center")}),
                                                                ])
                                                                .event(clone!(modal, med => move |_:events::Click| {
                                                                    let has_guid = modal.ipd_med_selected.lock_ref().contains(&med.hos_guid);
                                                                    if has_guid {
                                                                        modal.ipd_med_selected.lock_mut().retain(|uid| uid != &med.hos_guid);
                                                                        modal.selected_meds.lock_mut().retain(|m| m.hos_guid != med.hos_guid);
                                                                    } else {
                                                                        modal.ipd_med_selected.lock_mut().push_cloned(med.hos_guid.clone());
                                                                        modal.selected_meds.lock_mut().push_cloned(med.clone());
                                                                    }
                                                                }))
                                                            })
                                                        })))
                                                    }),
                                                ])
                                            }))
                                        }),
                                    ])
                                }),
                                html!("div", {
                                    .class("col-md-5")
                                    .style("height","calc(100vh - 170px)")
                                    .style("overflow-y","auto")
                                    .child(html!("div", {
                                        //.attr("id", "med_reconcile_selected_med_table_div")
                                        .child(html!("table", {
                                            .class(class::TABLE_SM)
                                            //.attr("id", "med_reconcile_selected_med_table")
                                            .children([
                                                html!("caption", {
                                                    .style("caption-side","top")
                                                    .text("Selected")
                                                }),
                                                html!("thead", {
                                                    .child(html!("tr", {
                                                        .children([
                                                            html!("th", {.attr("scope", "col").text("#")}),
                                                            html!("th", {.attr("scope", "col").text("ชื่อยา")}),
                                                            html!("th", {.attr("scope", "col").text("วิธีใช้")}),
                                                            html!("th", {.attr("scope", "col").text("จำนวน").class("text-end")}),
                                                            html!("th", {.attr("scope", "col").text("วันที่ได้รับยา")}),
                                                        ])
                                                    }))
                                                }),
                                                html!("tbody", {
                                                    //.attr("id", "med_reconcile_selected_med_table_tbody")
                                                    .children_signal_vec(modal.selected_meds.signal_vec_cloned().enumerate().map(clone!(app, modal => move |(i, med)| {
                                                        let medrec_icode = app.state().hosxp_medrec_icode();
                                                        let (item_name, usage) = if med.icode == medrec_icode {
                                                            (med.name1.clone(), Some([&med.name2.clone().unwrap_or_default(), " ", &med.name3.clone().unwrap_or_default()].concat()))
                                                        } else {
                                                            (med.item_name.clone(), med.usage.clone())
                                                        };
                                                        html!("tr", {
                                                            .style("cursor","pointer")
                                                            .children([
                                                                html!("td", {.text_signal(i.signal_cloned().map(|opt| opt.map(|n| (n + 1).to_string()).unwrap_or_default())).class("text-center")}),
                                                                html!("td", {.text(&item_name.unwrap_or_default())}),
                                                                html!("td", {.text(&usage.as_ref().map(|s| sanity_dot_space(s)).unwrap_or_default())}),
                                                                html!("td", {.text(&med.qty.map(|i| i.to_string()).unwrap_or_default()).class("text-end")}),
                                                                html!("td", {.text(&date_th_opt(&med.rxdate)).class("text-center")}),
                                                            ])
                                                            .event(clone!(modal => move |_:events::Click| {
                                                                modal.selected_meds.lock_mut().retain(|m| m.hos_guid != med.hos_guid);
                                                                modal.opd_med_selected.lock_mut().retain(|uid| uid != &med.hos_guid);
                                                                modal.ipd_med_selected.lock_mut().retain(|uid| uid != &med.hos_guid);
                                                            }))
                                                        })
                                                    })))
                                                }),
                                            ])
                                        }))
                                    }))
                                }),
                            ])
                        }))
                    }),
                    html!("div", {
                        .class("modal-footer")
                        .children([
                            html!("div", {
                                //.attr("id", "med_reconciliation_remed_selected_row_display_div")
                                .text_signal(modal.selected_meds.signal_vec_cloned().to_signal_cloned().map(|selecteds| {
                                    let len = selecteds.len();
                                    if len > 0 {
                                        ["เลือกแล้ว ", &len.to_string(), " รายการ"].concat()
                                    } else {
                                        String::new()
                                    }
                                }))
                            }),
                            html!("button" => HtmlButtonElement, {
                                .attr("type", "button")
                                .attr("data-bs-dismiss", "modal")
                                .class("btn")
                                .class_signal("btn-primary", modal.has_selected())
                                .class_signal("btn-secondary", not(modal.has_selected()))
                                .child(html!("i", {.class(class::FA_PLUS)}))
                                .text(" Add")
                                .apply(mixins::click_with_loader_checked_or_true_disable_signal(clone!(app, modal => move || {
                                    Self::save(modal.clone(), app.clone());
                                }), not(modal.has_selected()), app.state()))
                            }),
                            html!("button", {
                                .attr("type", "button")
                                .class(class::BTN_GRAY)
                                .attr("data-bs-dismiss", "modal")
                                .child(html!("i", {.class(class::FA_X)}))
                                .text(" Cancel")
                            }),
                        ])
                    }),
                ])
            }))
        })
    }
}
