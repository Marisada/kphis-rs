// opd-er-order-medical-history-data.php

use dominator::{Dom, clone, events, html};
use futures_signals::{
    map_ref,
    signal::{Mutable, SignalExt},
};
use rust_decimal::Decimal;
use std::rc::Rc;

use kphis_model::{
    app::VisitTypeId,
    opd_er::medical_history::{OpdErMedicalHistory, OpdErMedicalHistoryParams},
    patient_info::PatientInfo,
    score::{ScoreDispatch, Scores},
};
use kphis_ui_app::App;
use kphis_ui_core::{class, doms};
use kphis_util::{
    datetime::{JsTime, datetime_th},
    util::zero_none,
};

/// - GET `EndPoint::OpdErMedicalHistory`
#[derive(Default)]
pub struct OpdErMedicalHistoryCpn {
    patient: Mutable<Option<Rc<PatientInfo>>>,

    loaded: Mutable<bool>,
    medical_history: Mutable<Rc<OpdErMedicalHistory>>,
}

impl OpdErMedicalHistoryCpn {
    pub fn new(patient: Mutable<Option<Rc<PatientInfo>>>, source: Option<Mutable<Rc<OpdErMedicalHistory>>>) -> Rc<Self> {
        Rc::new(Self {
            patient,
            loaded: Mutable::new(source.is_some()),
            medical_history: source.unwrap_or_default(),
        })
    }

    fn load(page: Rc<Self>, app: Rc<App>) {
        if let Some(patient) = page.patient.get_cloned() {
            if let VisitTypeId::OpdEr(_vn, opd_er_order_master_id) = patient.visit_type() {
                app.async_load(
                    true,
                    clone!(app, page => async move {
                        let visit_datetime = if let (Some(regdate), Some(regtime)) = (patient.regdate(), patient.regtime()) {
                            Some([regdate.to_string(), regtime.js_string()].join(" "))
                        } else {
                            None
                        };
                        let params = OpdErMedicalHistoryParams {
                            opd_er_order_master_id: zero_none(opd_er_order_master_id),
                            hn: patient.hn(),
                            vn: patient.vn(),
                            visit_datetime,
                            // age_y: zero_none(page.age_y.get()),
                            ..Default::default()
                        };
                        // GET `EndPoint::OpdErMedicalHistory`
                        match OpdErMedicalHistory::call_api_get(&params, app.state()).await {
                            Ok(response) => {
                                page.medical_history.set(Rc::new(response));
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

    pub fn render(page: Rc<Self>, display: Option<Mutable<bool>>, app: Rc<App>) -> Dom {
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
            .class("mb-2")
            //.attr("id", "opd-er-order-medical-history")
            .child_signal(page.medical_history.signal_cloned().map(clone!(page, app => move |hx| {
                Some(html!("div", {
                    .class("card")
                    .children([
                        html!("div", {
                            .class(class::CARD_HEAD)
                            .child(html!("span", {
                                // make "span in the high container" as vertical-align: center
                                .style("vertical-align","sub")
                                .text("ประวัติผู้ป่วย")
                            }))
                            .apply_if(display.is_some(), |dom| {
                                dom.child(html!("div", {
                                    .class(class::FLOAT_RR)
                                    .child(html!("button", {
                                        .attr("type", "button")
                                        .class("btn-close")
                                        .event(clone!(display => move |_: events::Click| {
                                            if let Some(display_mutable) = &display {
                                                display_mutable.set_neq(false);
                                            }
                                        }))
                                    }))
                                }))
                            })
                        }),
                        html!("div", {
                            .class("card-body")
                            .children([
                                html!("div", {
                                    .class(class::BOLD_T3L)
                                    .child(html!("i", {.class(class::FA_CLIPBOARD)}))
                                    .text(" Vital Sign (การบันทึก Vital Sign ครั้งแรกจากโปรแกรม KPHIS)")
                                }),
                                html!("div", {
                                    .class("ms-3")
                                    .apply(|dom| {
                                        if let Some(vs_datetime) = hx.vs_kphis.as_ref().map(|vs| vs.vs_datetime) {
                                            dom.children([
                                                html!("span", {.class("fw-bold").text("วันที่ : ")}),
                                                html!("span", {.text(&datetime_th(&vs_datetime))}),
                                            ])
                                        } else { dom }
                                    })
                                    .apply(|dom| {
                                        if let Some(mut d) = hx.vs_kphis.as_ref().and_then(|vs| vs.bw) {
                                            if d > Decimal::new(10,0) { d.rescale(1); }
                                            dom.children([
                                                html!("span", {.class("fw-bold").text(" น้ำหนัก : ")}),
                                                html!("span", {.text(&d.to_string()).text(" กิโลกรัม")}),
                                            ])
                                        } else { dom }
                                    })
                                    .apply(|dom| {
                                        if let Some(i) = hx.vs_kphis.as_ref().and_then(|vs| vs.height) {
                                            dom.children([
                                                html!("span", {.class("fw-bold").text(" ส่วนสูง : ")}),
                                                html!("span", {.text(&i.to_string()).text(" เซนติเมตร")}),
                                            ])
                                        } else { dom }
                                    })
                                    .apply(|dom| {
                                        if let Some(i) = hx.vs_kphis.as_ref().and_then(|vs| vs.pain) {
                                            dom.children([
                                                html!("span", {.class("fw-bold").text(" PS : ")}),
                                                html!("span", {.text(&i.to_string()).text(" คะแนน")}),
                                            ])
                                        } else { dom }
                                    })
                                }),
                                html!("div", {
                                    .class("ms-3")
                                    .apply(|dom| {
                                        if let Some(d) = hx.vs_kphis.as_ref().and_then(|vs| vs.bt) {
                                            dom.children([
                                                html!("span", {.class("fw-bold").text("BT : ")}),
                                                html!("span", {.text(&d.to_string()).text(" ℃")}),
                                            ])
                                        } else { dom }
                                    })
                                    .apply(|dom| {
                                        if let Some(i) = hx.vs_kphis.as_ref().and_then(|vs| vs.pr) {
                                            dom.children([
                                                html!("span", {.class("fw-bold").text(" PR : ")}),
                                                html!("span", {.text(&i.to_string()).text(" /min")}),
                                            ])
                                        } else { dom }
                                    })
                                    .apply(|dom| {
                                        if let Some(i) = hx.vs_kphis.as_ref().and_then(|vs| vs.rr) {
                                            dom.children([
                                                html!("span", {.class("fw-bold").text(" RR : ")}),
                                                html!("span", {.text(&i.to_string()).text(" /min")}),
                                            ])
                                        } else { dom }
                                    })
                                    .apply(|dom| {
                                        if let Some((Some(sbp), Some(dbp))) = hx.vs_kphis.as_ref().map(|vs| (vs.sbp, vs.dbp)) {
                                            dom.children([
                                                html!("span", {.class("fw-bold").text(" BP : ")}),
                                                html!("span", {.text(&[sbp.to_string(), dbp.to_string()].join("/")).text(" mmHg")}),
                                            ])
                                        } else { dom }
                                    })
                                    .apply(|dom| {
                                        if let Some(i) = hx.vs_kphis.as_ref().and_then(|vs| vs.sat) {
                                            dom.children([
                                                html!("span", {.class("fw-bold").children([
                                                    html!("span", {.text(" O")}), html!("sub", {.text("2")}), html!("span", {.text("sat : ")}),
                                                ])}),
                                                html!("span", {.text(&i.to_string()).text(" %")}),
                                            ])
                                        } else { dom }
                                    })
                                    .apply(|dom| {
                                        let birthday = page.patient.lock_ref().as_ref().and_then(|pt| pt.birthday());
                                        if let Some(scores) = Scores::from_concat(&hx.vs_kphis.as_ref().and_then(|vs| vs.ews_concat.clone()), birthday, app.state()) {
                                            let ews_dom = if let Some(ews) = scores.ews.score() {
                                                html!("span", {
                                                    .children([
                                                        html!("span", {.class(class::BOLD_R2).text(scores.ews.label())}),
                                                        doms::badge_score(scores.ews.label(), ews, scores.ews.color_total(), scores.ews.bg_color_total()),
                                                    ])
                                                })
                                            } else {
                                                Dom::empty()
                                            };
                                            let qsofa_dom = if let Some(qsofa) = scores.qsofa.score() {
                                                html!("span", {
                                                    .children([
                                                        html!("span", {.class(class::BOLD_R2).text(scores.qsofa.label())}),
                                                        doms::badge_score(scores.qsofa.label(), qsofa, scores.qsofa.color_total(), scores.qsofa.bg_color_total()),
                                                    ])
                                                })
                                            } else {
                                                Dom::empty()
                                            };
                                            let sirs_dom = if let Some(sirs) = scores.sirs.score() {
                                                html!("span", {
                                                    .children([
                                                        html!("span", {.class(class::BOLD_R2).text(scores.sirs.label())}),
                                                        doms::badge_score(scores.sirs.label(), sirs, scores.sirs.color_total(), scores.sirs.bg_color_total()),
                                                    ])
                                                })
                                            } else {
                                                Dom::empty()
                                            };
                                            dom.children([ews_dom, qsofa_dom, sirs_dom])
                                        } else { dom }
                                    })
                                }),
                                html!("div", {
                                    .class("ms-3")
                                    .apply_if(hx.vs_kphis.as_ref().map(|vs| vs.eye.is_some() || vs.verbal.is_some() || vs.movement.is_some()).unwrap_or_default(), |dom| {
                                        dom.children([
                                            html!("span", {.class("fw-bold").text("GCS : ")}),
                                            html!("span", {.text("E")}),
                                            html!("sub", {.text(&hx.vs_kphis.as_ref().and_then(|vs| vs.eye).map(|i| i.to_string()).unwrap_or_default())}),
                                            html!("span", {.text("V")}),
                                            html!("sub", {.text(&hx.vs_kphis.as_ref().and_then(|vs| vs.verbal.clone()).unwrap_or_default())}),
                                            html!("span", {.text("M")}),
                                            html!("sub", {.text(&hx.vs_kphis.as_ref().and_then(|vs| vs.movement).map(|i| i.to_string()).unwrap_or_default())}),
                                        ])
                                    })
                                    .apply(|dom| {
                                        if let Some(d) = hx.vs_kphis.as_ref().and_then(|vs| vs.right_pupil) {
                                            dom.children([
                                                html!("span", {.class("fw-bold").text(" Pupil Rt : ")}),
                                                html!("span", {.text(&d.to_string())}),
                                            ])
                                        } else { dom }
                                    })
                                    .apply(|dom| {
                                        if let Some(d) = hx.vs_kphis.as_ref().and_then(|vs| vs.left_pupil) {
                                            dom.children([
                                                html!("span", {.class("fw-bold").text(" Pupil Lt : ")}),
                                                html!("span", {.text(&d.to_string())}),
                                            ])
                                        } else { dom }
                                    })
                                    .apply(|dom| {
                                        if let Some(d) = hx.vs_kphis.as_ref().and_then(|vs| vs.hct) {
                                            dom.children([
                                                html!("span", {.class("fw-bold").text(" HCT : ")}),
                                                html!("span", {.text(&d.to_string()).text(" %")}),
                                            ])
                                        } else { dom }
                                    })
                                    .apply(|dom| {
                                        if let Some(s) = hx.vs_kphis.as_ref().and_then(|vs| vs.dtx.as_ref()) {
                                            dom.children([
                                                html!("span", {.class("fw-bold").text(" DTX : ")}),
                                                html!("span", {.text(s).text(" mg/dl")}),
                                            ])
                                        } else { dom }
                                    })
                                    .apply(|dom| {
                                        if let Some(d) = hx.vs_kphis.as_ref().and_then(|vs| vs.bl) {
                                            dom.children([
                                                html!("span", {.class("fw-bold").text(" Blood Lactate : ")}),
                                                html!("span", {.text(&d.to_string()).text(" mmol/L")}),
                                            ])
                                        } else { dom }
                                    })
                                }),
                                html!("hr"),
                                html!("div", {
                                    .class(class::BOLD_T3L)
                                    .child(html!("i", {.class(class::FA_CLIPBOARD)}))
                                    .text(" ประวัติผู้ป่วย (HOSxP)")
                                }),
                            ])
                            .apply(|dom| {
                                if let Some(cc) = hx.opdscreen.as_ref().and_then(|os| os.cc.as_ref()) {
                                    dom.child(html!("div", {
                                        .class("ms-3")
                                        .children([
                                            html!("span", {.class("fw-bold").text("CC : ")}),
                                            html!("span", {.style("white-space","pre-wrap").text(cc)}),
                                        ])
                                    }))
                                } else { dom }
                            })
                            .apply(|dom| {
                                if let Some(hpi) = hx.opdscreen.as_ref().and_then(|os| os.hpi.as_ref()) {
                                    dom.child(html!("div", {
                                        .class("ms-3")
                                        .children([
                                            html!("span", {.class("fw-bold").text("PI : ")}),
                                            html!("span", {.style("white-space","pre-wrap").text(hpi)}),
                                        ])
                                    }))
                                } else { dom }
                            })
                            .children([
                                html!("div", {
                                    .class("ms-3")
                                    .child(html!("span", {.class("fw-bold").text("ประวัติการใช้ยา : ")}))
                                    .apply_if(!hx.hosxp_drug_history.is_empty(), |dom| {
                                        dom.child(html!("ul", {
                                            .children(hx.hosxp_drug_history.iter().map(|drug| {
                                                html!("li", {.text(drug)})
                                            }))
                                        }))
                                    })
                                }),
                                html!("div", {
                                    .class("ms-3")
                                    .child(html!("span", {.class("fw-bold").text("ประวัติการแพ้ยา : ")}))
                                    .apply_if(!hx.hosxp_drug_history.is_empty(), |dom| {
                                        dom.child(html!("ul", {
                                            .children(hx.hosxp_drugallergy.iter().map(|allergy| {
                                                html!("li", {.text(allergy)})
                                            }))
                                        }))
                                    })
                                }),
                                html!("div", {
                                    .class("ms-3")
                                    .child(html!("span", {.class("fw-bold").text("ประวัติการผ่าตัด : ")}))
                                    .apply_if(!hx.hosxp_drug_history.is_empty(), |dom| {
                                        dom.child(html!("ul", {
                                            .children(hx.hosxp_operation_history.iter().map(|op| {
                                                html!("li", {.text(op)})
                                            }))
                                        }))
                                    })
                                }),
                                html!("div", {
                                    .class("ms-3")
                                    .child(html!("span", {.class("fw-bold").text("Diagnosis : ")}))
                                    .apply_if(!hx.hosxp_drug_history.is_empty(), |dom| {
                                        dom.child(html!("ul", {
                                            .children(hx.hosxp_diagnosis.iter().map(|dx| {
                                                html!("li", {.text(dx)})
                                            }))
                                        }))
                                    })
                                }),
                            ])
                        }),
                    ])
                }))
            })))
        })
    }
}
