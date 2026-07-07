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
    endpoint::EndPoint,
    fetch::Method,
    med_reconcile::{MedReconciliation, MedReconciliationHeader, MedReconciliationItem, MedReconciliationItemSave, MedReconciliationParams},
    patient_info::PatientInfo,
};
use kphis_ui_app::App;
use kphis_ui_core::{class, mixins};
use kphis_util::{
    datetime::{date_th_opt, datetime_th_opt},
    util::{str_some, zero_none},
};

/// - GET `EndPoint::MedReconcileHn` (guarded, invisible)
/// - GET `EndPoint::IpdMedReconcile` (guarded, invisible)
/// - GET `EndPoint::OpdErMedReconcile` (guarded, invisible)
/// - POST `EndPoint::IpdMedReconcile` (guarded, remove `นำมาพิจารณาใหม่` button)
/// - POST `EndPoint::OpdErMedReconcile` (guarded, remove `นำมาพิจารณาใหม่` button)
#[derive(Default)]
pub struct MedReconcileHistoryCpn {
    head_loaded: Mutable<bool>,
    recon_loaded: Mutable<bool>,

    parent_loaded: Mutable<bool>,
    patient: Mutable<Option<Rc<PatientInfo>>>,
    visit_type: Mutable<Option<VisitTypeId>>,

    heads: MutableVec<Rc<MedReconciliationHeader>>,
    recons: MutableVec<Rc<MedReconciliation>>,
}

impl MedReconcileHistoryCpn {
    pub fn new(parent_loaded: Mutable<bool>, patient: Mutable<Option<Rc<PatientInfo>>>) -> Rc<Self> {
        Rc::new(Self {
            parent_loaded,
            patient,
            ..Default::default()
        })
    }

    fn is_allow_load(&self, app: Rc<App>) -> impl Signal<Item = bool> + use<> {
        self.patient.signal_ref(move |opt| {
            opt.as_ref()
                .map(|pt| {
                    let (is_ipd, is_pre_admit) = pt.visit_type.is_ipd_and_is_pre_admit();
                    app.endpoint_is_allow(&Method::GET, &EndPoint::MedReconcileHn, false)
                        && if is_ipd {
                            app.endpoint_is_allow(&Method::GET, &EndPoint::IpdMedReconcile, is_pre_admit)
                        } else {
                            app.endpoint_is_allow(&Method::GET, &EndPoint::OpdErMedReconcile, false)
                        }
                })
                .unwrap_or_default()
        })
    }

    fn load_heads(page: Rc<Self>, app: Rc<App>) {
        if let Some(patient) = page.patient.get_cloned() {
            if let Some(hn) = patient.hn() {
                page.heads.lock_mut().clear();
                app.async_load(
                    true,
                    clone!(app => async move {
                        // GET `EndPoint::MedReconcileHn`
                        match MedReconciliationHeader::call_api_get(&hn, app.state()).await {
                            Ok(mut responses) => {
                                responses.sort_by(|a, b| {
                                    if let (Some(dt_a), Some(dt_b)) = (a.visit_datetime, b.visit_datetime) {
                                        dt_b.cmp(&dt_a)
                                    } else {
                                        b.visit_type.cmp(&a.visit_type)
                                    }
                                });
                                // remove current visit
                                page.heads.lock_mut().extend(responses.into_iter().filter(|res| res.visit_type.vnan() != patient.visit_type.vnan()).map(Rc::new));
                            }
                            Err(e) => {
                                app.alert_app_error(&e).await;
                            }
                        }
                    }),
                )
            }
        }
    }

    fn load_recons(page: Rc<Self>, app: Rc<App>) {
        // pt.hn() use str_some() internally
        let hn = page.patient.lock_ref().as_ref().and_then(|pt| pt.hn());
        if hn.is_some() {
            page.recons.lock_mut().clear();
            app.async_load(
                true,
                clone!(app => async move {
                    if let Some(visit_type) = page.visit_type.lock_ref().as_ref() {
                        let (result, is_valid) = match &visit_type {
                            VisitTypeId::Ipd(an)
                            | VisitTypeId::PreAdmit(an) => {
                                let an = str_some(an.to_owned());
                                let valid = an.is_some();
                                let params = MedReconciliationParams {
                                    hn,
                                    an,
                                    ..Default::default()
                                };
                                // GET `EndPoint::IpdMedReconcile`
                                (MedReconciliation::call_api_get(true, &params, app.state()).await, valid)
                            }
                            VisitTypeId::OpdEr(_, id) => {
                                let opd_er_order_master_id = zero_none(*id);
                                let valid = opd_er_order_master_id.is_some();
                                let params = MedReconciliationParams {
                                    hn,
                                    opd_er_order_master_id,
                                    ..Default::default()
                                };
                                // GET `EndPoint::OpdErMedReconcile`
                                (MedReconciliation::call_api_get(false, &params, app.state()).await, valid)
                            }
                            VisitTypeId::Visit(_) => (Ok(Vec::new()), false)
                        };

                        if is_valid {
                            match result {
                                Ok(responses) => {
                                    page.recons.lock_mut().extend(responses.into_iter().map(Rc::new));
                                }
                                Err(e) => {
                                    app.alert_app_error(&e).await;
                                }
                            }
                        }
                    }
                }),
            );
        }
    }

    // POST `EndPoint::IpdMedReconcile`
    // POST `EndPoint::OpdErMedReconcile`
    fn save_recon(use_changed_drugusage: bool, recon: Rc<MedReconciliation>, page: Rc<Self>, app: Rc<App>) {
        let visit_type_opt = page.patient.lock_ref().as_ref().map(|pt| pt.visit_type());
        if let Some(visit_type) = visit_type_opt {
            let items = recon
                .med_reconciliation_items
                .iter()
                .map(|med| {
                    let old_drugusage = if use_changed_drugusage { med.changed_drugusage.clone() } else { med.old_drugusage.clone() };
                    MedReconciliationItemSave {
                        icode: med.icode.clone(),
                        med_name: med.med_name.clone(),
                        custom_med_name: med.custom_med_name.clone(),
                        receive_from: med.receive_from.clone(),
                        receive_date: med.receive_date,
                        old_drugusage,
                        receive_qty: med.receive_qty,
                    }
                })
                .collect::<Vec<MedReconciliationItemSave>>();

            app.async_load(
                true,
                clone!(app, page => async move {
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
                                    page.parent_loaded.set(false);
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

    pub fn render(page: Rc<Self>, app: Rc<App>) -> Dom {
        html!("div", {
            .visible_signal(map_ref!{
                let is_empty = page.heads.signal_vec_cloned().to_signal_cloned().map(|v| v.is_empty()),
                let is_allow = page.is_allow_load(app.clone()) =>
                !is_empty && *is_allow
            })
            .future(map_ref!(
                let busy = app.loader_is_loading(),
                let has_patient = page.patient.signal_cloned().map(|opt| opt.is_some()),
                let is_allow = page.is_allow_load(app.clone()),
                let loaded = page.head_loaded.signal() =>
                !busy && !loaded && *has_patient && *is_allow
            ).for_each(clone!(app, page => move |ready| {
                if ready {
                    Self::load_heads(page.clone(), app.clone());
                    page.head_loaded.set_neq(true);
                }
                async {}
            })))
            .future(map_ref!(
                let busy = app.loader_is_loading(),
                let has_patient = page.patient.signal_cloned().map(|opt| opt.is_some()),
                let is_allow = page.is_allow_load(app.clone()),
                let loaded = page.recon_loaded.signal() =>
                !busy && !loaded && *has_patient && *is_allow
            ).for_each(clone!(app, page => move |ready| {
                if ready {
                    Self::load_recons(page.clone(), app.clone());
                    page.recon_loaded.set_neq(true);
                }
                async {}
            })))
            .class(class::ROW_T)
            .child(html!("div", {
                .class("col-md-12")
                .child(html!("div", {
                    .class("card")
                    .children([
                        html!("div", {
                            .class(class::CARD_HEAD)
                            .class("bg-warning-subtle")
                            .text("ประวัติ Med Reconciliation")
                            .attr("title","แสดงเพียง 20 ครั้งล่าสุดเท่านั้น")
                            .style("cursor","pointer")
                        }),
                        html!("div", {
                            .class(class::CARD_BODY_FLEX)
                            .style("gap", "10px")
                            .style("max-height", "555px")
                            .children([
                                html!("div", {
                                    .style("min-width", "200px")
                                    .style("overflow-y","auto")
                                    .child(html!("div", {
                                        .children_signal_vec(page.heads.signal_vec_cloned().map(clone!(page => move |head| {
                                            let same_vnan_broadcast = page.visit_type.signal_cloned().map(clone!(head => move |opt| opt.map(|visit_type| visit_type.vnan() == head.visit_type.vnan()).unwrap_or_default())).broadcast();
                                            html!("button", {
                                                .attr("type", "button")
                                                .class(class::BTN_T_W100)
                                                .class_signal("btn-primary", same_vnan_broadcast.signal())
                                                .class_signal("btn-secondary", not(same_vnan_broadcast.signal()))
                                                .text(&datetime_th_opt(&head.visit_datetime))
                                                .event(clone!(page, head => move |_:events::Click| {
                                                    page.visit_type.set(Some(head.visit_type.clone()));
                                                    page.recon_loaded.set(false);
                                                }))
                                            })
                                        })))
                                    }))
                                }),
                                html!("div", {
                                    .style("overflow","auto")
                                    .style("width", "100%")
                                    .child(html!("div", {
                                        .style("min-width", "800px")
                                        .style("width", "calc(100% - 5px)")
                                        .children_signal_vec(page.recons.signal_vec_cloned().map(move |recon| {
                                            Self::render_recon(recon, page.clone(), app.clone())
                                        }))
                                    }))
                                }),
                            ])
                        }),
                    ])
                }))
            }))
        })
    }

    fn render_recon(recon: Rc<MedReconciliation>, page: Rc<Self>, app: Rc<App>) -> Dom {
        html!("div", {
            .class(class::BOX_T)
            .class(class::FLEX_COL)
            .children([
                html!("div", {
                    .class(class::BOLD_T1)
                    .text(&datetime_th_opt(&recon.med_reconciliation_datetime))
                }),
                html!("table", {
                    .class(class::TABLE_STRIP)
                    .children([
                        html!("thead", {
                            .child(html!("tr", {
                                .children([
                                    html!("th", {.attr("scope", "col").class("text-center").text("#")}),
                                    html!("th", {.attr("scope", "col").text("ชื่อยา")}),
                                    html!("th", {.attr("scope", "col").text("วิธีใช้")}),
                                    html!("th", {.attr("scope", "col").text("จำนวน")}),
                                    html!("th", {.attr("scope", "col").text("วันที่ได้รับ").style("min-width","110px")}),
                                    html!("th", {.attr("scope", "col").text("สถานพยาบาล")}),
                                    html!("th", {.attr("scope", "col").text("คำสั่งใช้").style("min-width","80px")}),
                                    html!("th", {.attr("scope", "col").text("เปลี่ยนวิธีใช้")}),
                                    html!("th", {.attr("scope", "col").text("จำนวนคงเหลือ / หมายเหตุ")}),
                                ])
                            }))
                        }),
                        html!("tbody", {
                            .children(recon.med_reconciliation_items.iter().enumerate().map(|(i,row)| {
                                Self::render_recon_detail(row, i)
                            }))
                        }),
                    ])
                }),
            ])
            .apply(|dom| {
                if let Some(note) = recon.note.as_ref() {
                    dom.child(html!("div", {
                        .class("mb-2")
                        .children([
                            html!("span", {
                                .class("fw-bold")
                                .text("Note: ")
                            }),
                            html!("span", {
                                .text(note)
                            }),
                        ])
                    }))
                } else {
                    dom
                }
            })
            .child(html!("div", {
                .class(class::ROW_T)
                .children([
                    html!("div", {
                        .class("col")
                        .child(html!("div", {
                            .children([
                                html!("span", {
                                    .class("fw-bold")
                                    .text("ผู้บันทึกรายการ: ")
                                }),
                                html!("span", {
                                    .text(&recon.pharmacist_name.clone().unwrap_or_default())
                                }),
                            ])
                        }))
                    }),
                    html!("div", {
                        .class("col")
                        .child(html!("div", {
                            .children([
                                html!("span", {
                                    .class("fw-bold")
                                    .text("แพทย์ผู้พิจารณา: ")
                                }),
                                html!("span", {
                                    .text(&recon.doctor_name.clone().unwrap_or_default())
                                }),
                            ])
                        }))
                    }),
                ])
            }))
            .child_signal(page.patient.signal_cloned().map(clone!(app, page, recon => move |opt| opt.and_then(|pt| {
                let (is_ipd, ip_pre_admit) = pt.visit_type.is_ipd_and_is_pre_admit();
                (if is_ipd {
                    app.endpoint_is_allow(&Method::POST, &EndPoint::IpdMedReconcile, ip_pre_admit)
                } else {
                    app.endpoint_is_allow(&Method::POST, &EndPoint::OpdErMedReconcile, false)
                }).then(|| {
                    let is_reused = Mutable::new(false);
                    let has_changed_drugusage = recon.med_reconciliation_items.iter().any(|item| item.changed_drugusage.is_some());
                    html!("div", {
                        .class("row")
                        .child(html!("div", {
                            .class("text-end")
                            .child(html!("button" => HtmlButtonElement, {
                                .attr("type","button")
                                .class(class::BTN_L_BLUE)
                                .text(if has_changed_drugusage {"นำมาพิจารณาใหม่ (อ้างอิงวิธีใช้เดิม)"} else {"นำมาพิจารณาใหม่"})
                                .apply(mixins::click_with_loader_checked_or_true_disable_signal(clone!(app, page, recon, is_reused => move || {
                                    Self::save_recon(false, recon.clone(), page.clone(), app.clone());
                                    is_reused.set(true);
                                }), is_reused.signal(), app.state()))
                            }))
                            .apply_if(has_changed_drugusage, |dom| dom
                                .child(html!("button" => HtmlButtonElement, {
                                    .attr("type","button")
                                    .class(class::BTN_L_CYAN)
                                    .text("นำมาพิจารณาใหม่ (อ้างอิงวิธีใช้ใหม่)")
                                    .apply(mixins::click_with_loader_checked_or_true_disable_signal(clone!(app, page, recon, is_reused => move || {
                                        Self::save_recon(true, recon.clone(), page.clone(), app.clone());
                                        is_reused.set(true);
                                    }), is_reused.signal(), app.state()))
                                }))
                            )
                        }))
                    })
                })
            }))))
        })
    }

    fn render_recon_detail(row: &MedReconciliationItem, i: usize) -> Dom {
        html!("tr", {
            .children([
                html!("td", {
                    .class("text-center")
                    .text(&(i + 1).to_string())
                }),
                html!("td", {
                    .text(&row.med_name.clone().or(row.custom_med_name.clone()).unwrap_or_default())
                }),
                html!("td", {
                    .text(&row.old_drugusage.clone().unwrap_or_default())
                }),
                html!("td", {
                    .text(&row.receive_qty.map(|i| i.to_string()).unwrap_or_default())
                }),
                html!("td", {
                    .text(&date_th_opt(&row.receive_date))
                }),
                html!("td", {
                    .text(&row.receive_from.clone().unwrap_or_default())
                }),
                html!("td", {
                    .text(row.used.as_ref().map(|used| {
                        match used.as_str() {
                            "Y" => "สั่งใช้",
                            "N" => "ไม่สั่งใช้",
                            "H" => "Hold",
                            _ => ""
                        }
                    }).unwrap_or_default())
                }),
                html!("td", {
                    .text(&row.changed_drugusage.clone().unwrap_or_default())
                }),
                html!("td", {
                    .text(&row.last_dose_taken_remark.clone().unwrap_or_default())
                }),
            ])
        })
    }
}
