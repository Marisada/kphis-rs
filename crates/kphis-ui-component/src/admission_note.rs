// ipd-nurse-index.php::viewDrAdmissionNoteAndNurseIndexNote()

use dominator::{Dom, clone, events, html, link};
use futures_signals::{
    map_ref,
    signal::{Mutable, SignalExt},
};
use std::rc::Rc;

use kphis_model::{
    app::VisitTypeId,
    endpoint::EndPoint,
    fetch::Method,
    ipd::{
        admission_note_dr::IpdAdmissionNoteDrRaw,
        admission_note_nurse::IpdNurseAdmissionNote,
        index_note::{IndexNote, IndexNoteParams},
    },
    patient_info::PatientInfo,
    route::Route,
    score::{ScoreDispatch, Scores},
};
use kphis_ui_app::App;
use kphis_ui_core::class;
use kphis_util::{
    datetime::{date_and_time_th_opt_relative, date_th_opt, datetime_th_opt_relative, datetime_th_relative},
    util::{explode, str_some},
};

use crate::modal::{blank_modal, index_note_form::IndexNoteForm};

/// - GET `EndPoint::IpdAdmissionNoteDrAn`
/// - GET `EndPoint::IpdAdmissionNoteNurseAn`
/// - GET `EndPoint::IpdIndexNote` (guarded, remove index-note div)
/// - POST `EndPoint::IpdIndexNote` (guarded, remove 'แก้ไข' btn)
#[derive(Default)]
pub struct AdmissionNoteCpn {
    loaded: Mutable<bool>,
    reload_index_note: Mutable<bool>,

    //an: Mutable<String>,
    patient: Mutable<Option<Rc<PatientInfo>>>,

    // index_note
    nurse_index_note_id: Mutable<u32>, // zero for None
    nurse_index_note: Mutable<String>,

    admission_note_dr: Mutable<Option<Rc<IpdAdmissionNoteDrRaw>>>,
    admission_note_nurse: Mutable<Option<Rc<IpdNurseAdmissionNote>>>,
    index_note_modal: Mutable<Option<Rc<IndexNoteForm>>>,
}

impl AdmissionNoteCpn {
    pub fn new(patient: Mutable<Option<Rc<PatientInfo>>>) -> Rc<Self> {
        Rc::new(Self { patient, ..Default::default() })
    }

    // we use IpdAdmissionNoteDrRaw + load index_note
    // instead of ipd-nurse-index-print-data.php
    fn load(page: Rc<Self>, app: Rc<App>) {
        if let Some((an, is_pre_admit)) = page.patient.lock_ref().as_ref().and_then(|pt| pt.visit_type.an_and_is_pre_admit_owned()) {
            app.async_load(
                true,
                clone!(app, page, an => async move {
                    if app.endpoint_is_allow(&Method::GET, &EndPoint::IpdIndexNote, is_pre_admit) {
                        Self::fetch_index_note(page.clone(), app.clone()).await;
                    }
                    // GET `EndPoint::IpdAdmissionNoteDrAn`
                    match IpdAdmissionNoteDrRaw::call_api_get(&an, app.state()).await {
                        Ok(response) => {
                            page.admission_note_dr.set(Some(Rc::new(response)));
                        }
                        Err(e) => {
                            app.alert_app_error(&e).await;
                        }
                    }
                    // GET `EndPoint::IpdAdmissionNoteNurseAn`
                    match IpdNurseAdmissionNote::call_api_get(&an, app.state()).await {
                        Ok(response) => {
                            page.admission_note_nurse.set(response.map(Rc::new));
                        }
                        Err(e) => {
                            app.alert_app_error(&e).await;
                        }
                    }
                }),
            )
        }
    }

    fn load_index_note(page: Rc<Self>, app: Rc<App>) {
        app.async_load(
            true,
            clone!(app, page => async move {
                Self::fetch_index_note(page, app).await
            }),
        )
    }
    async fn fetch_index_note(page: Rc<Self>, app: Rc<App>) {
        if let Some(an) = match page.patient.lock_ref().as_ref().map(|pt| pt.visit_type()) {
            Some(VisitTypeId::Ipd(an)) | Some(VisitTypeId::PreAdmit(an)) => Some(an),
            Some(VisitTypeId::OpdEr(_, _)) | Some(VisitTypeId::Visit(_)) | None => None,
        } {
            let params = IndexNoteParams {
                an: str_some(an),
                nurse_index_note_id: None,
            };
            // GET `EndPoint::IpdIndexNote`
            match IndexNote::call_api_get(&params, app.state()).await {
                Ok(response) => {
                    if let Some(note) = response.first() {
                        page.nurse_index_note_id.set_neq(note.nurse_index_note_id);
                        page.nurse_index_note.set_neq(note.nurse_index_note.clone().unwrap_or_default());
                    }
                }
                Err(e) => {
                    app.alert_app_error(&e).await;
                }
            }
        }
    }

    pub fn render(page: Rc<Self>, display: Mutable<bool>, is_aside: bool, app: Rc<App>) -> Dom {
        let is_pre_admit = page.patient.lock_ref().as_ref().map(|pt| pt.visit_type.is_pre_admit()).unwrap_or_default();
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
                let reload = page.reload_index_note.signal() =>
                !busy && *reload
            ).for_each(clone!(app, page => move |ready| {
                if ready {
                    Self::load_index_note(page.clone(), app.clone());
                    page.reload_index_note.set_neq(false);
                }
                async {}
            })))
            .child_signal(page.admission_note_dr.signal_cloned().map(clone!(app, page => move |admission_note_opt| {
                admission_note_opt.as_ref().map(|admission_note| {
                    html!("div", {
                        //.attr("id", "dr_admission_note_div")
                        .class(class::CARD)
                        .children([
                            html!("div", {
                                .class(class::CARD_HEAD)
                                .text("ประวัติผู้ป่วย")
                                .child(html!("button", {
                                    .attr("type", "button")
                                    .class(class::BTN_SM_FR_GRAY)
                                    .child(html!("i", {.class(class::FA_X)}))
                                    .event(clone!(display => move |_: events::Click| {
                                        display.set_neq(false);
                                    }))
                                }))
                                .apply_if(!is_aside, |dom| dom
                                    .child_signal(page.patient.signal_cloned().map(clone!(app => move |pt_opt| {
                                        pt_opt.as_ref().and_then(|pt| {
                                            match pt.visit_type() {
                                                VisitTypeId::Ipd(an)
                                                | VisitTypeId::PreAdmit(an) => {
                                                    let route = Route::IpdAdmissionNoteDr {an};
                                                    route.has_permission(app.state()).then(|| {
                                                        link!(route.string(), {
                                                            .class(class::BTN_SM_FR_GRAY)
                                                            .text("บันทึกการรับใหม่ผู้ป่วยใน")
                                                        })
                                                    })
                                                }
                                                VisitTypeId::OpdEr(_, _)
                                                | VisitTypeId::Visit(_) => None,
                                            }
                                        })
                                    })))
                                )
                            }),
                            html!("div", {
                                .class("card-body")
                                .children([
                                    html!("div", {
                                        .children([
                                            html!("span", {.class(class::BOLD_L2).text("วันที่รับบริการ: ")}),
                                            html!("span", {.text(&datetime_th_opt_relative(&admission_note.opdscreen_pe.as_ref().and_then(|op| op.vstdatetime)))}),
                                        ])
                                    }),
                                    html!("div", {
                                        .children([
                                            html!("span", {.class(class::BOLD_L2).text("DX: ")}),
                                            html!("span", {.text(&admission_note.admission_note.as_ref().and_then(|note| note.impression.clone()).unwrap_or_default())}),
                                        ])
                                    }),
                                    html!("div", {
                                        .children([
                                            html!("span", {.class(class::BOLD_L2).text("CC: ")}),
                                            html!("span", {.text(&admission_note.admission_note.as_ref().and_then(|note| note.chief_complaints.clone()).unwrap_or_default())}),
                                        ])
                                    }),
                                    html!("div", {
                                        .children([
                                            html!("span", {.class(class::BOLD_L2).text("PI: ")}),
                                            html!("span", {.text(&admission_note.admission_note.as_ref().and_then(|note| note.medical_history.clone()).unwrap_or_default())}),
                                        ])
                                    }),
                                    html!("div", {
                                        .children([
                                            html!("span", {.class(class::BOLD_L2).text("PH: ")}),
                                            html!("span", {.text(&admission_note.admission_note.as_ref().and_then(|note| note.disease.clone()).unwrap_or_default())}),
                                        ])
                                        .children(admission_note.admission_note.as_ref().and_then(|note| {
                                            note.disease_detail.as_ref().map(|dd| {
                                                explode(dd, 3).iter().map(|chunk| {
                                                    html!("div", {
                                                        .style("margin-left","50px")
                                                        .children([
                                                            html!("span", {.class("text-primary").text("โรค ")}),
                                                            html!("span", {.text(&chunk[0].to_owned())}),
                                                            html!("span", {.class("text-primary").text(" เป็นมานาน ")}),
                                                            html!("span", {.text(&chunk[1].to_owned())}),
                                                            html!("span", {.class("text-primary").text(" ปี รักษาที่ ")}),
                                                            html!("span", {.text(&chunk[2].to_owned())}),
                                                        ])
                                                    })
                                                }).collect::<Vec<Dom>>()
                                            })
                                        }).unwrap_or_default())
                                    }),
                                    html!("div", {
                                        .child(html!("span", {.class(class::BOLD_L2).text("สัญญาณชีพแรกรับ HOSxP: ")}))
                                        .apply(|dom| {
                                            if let Some(pe) = admission_note.opdscreen_pe.as_ref() {
                                                dom.children([
                                                    html!("span", {.class("text-primary").text("BT: ")}),
                                                    html!("span", {.class("me-2").text(&pe.temperature.map(|f| f.to_string()).unwrap_or_default()).text(" °C")}),
                                                    html!("span", {.class("text-primary").text("PR: ")}),
                                                    html!("span", {.class("me-2").text(&pe.pulse.map(|f| f.to_string()).unwrap_or_default()).text(" /min")}),
                                                    html!("span", {.class("text-primary").text("RR: ")}),
                                                    html!("span", {.class("me-2").text(&pe.rr.map(|f| f.to_string()).unwrap_or_default()).text(" /min")}),
                                                    html!("span", {.class("text-primary").text("BP: ")}),
                                                    html!("span", {.class("me-2").text(&[pe.bps.map(|f| f.to_string()).unwrap_or_default(), pe.bpd.map(|f| f.to_string()).unwrap_or_default()].join("/")).text(" mmHg")}),
                                                    html!("span", {.class("text-primary").text("BW: ")}),
                                                    html!("span", {.class("me-2").text(&pe.bw.map(|f| f.to_string()).unwrap_or_default()).text(" Kg.")}),
                                                    html!("span", {.class("text-primary").text("Height: ")}),
                                                    html!("span", {.class("me-2").text(&pe.height.map(|i| i.to_string()).unwrap_or_default()).text(" cm.")}),
                                                ])
                                                .apply(|d| {
                                                    if let Some(dt) = pe.vstdatetime.as_ref() {
                                                        d.child(html!("span", {
                                                            .class("text-primary")
                                                            .text(&["(", &datetime_th_relative(&dt), ")"].concat())
                                                        }))
                                                    } else {
                                                        d
                                                    }
                                                })
                                            } else {
                                                dom
                                            }
                                        })
                                    }),
                                    html!("div", {
                                        .child(html!("span", {.class(class::BOLD_L2).text("สัญญาณชีพแรกรับ KPHIS: ")}))
                                        .apply(|dom| {
                                            if let Some(vs) = admission_note.vs.as_ref() {
                                                let birthday = page.patient.lock_ref().as_ref().and_then(|pt| pt.birthday());
                                                let scores = Scores::from_concat(&vs.ews_concat, birthday, app.state());

                                                dom.children([
                                                    html!("span", {.class("text-primary").text("BT: ")}),
                                                    html!("span", {.class("me-2").text(&vs.bt.map(|d| d.to_string()).unwrap_or_default()).text(" °C")}),
                                                    html!("span", {.class("text-primary").text("PR: ")}),
                                                    html!("span", {.class("me-2").text(&vs.pr.map(|u| u.to_string()).unwrap_or_default()).text(" /min")}),
                                                    html!("span", {.class("text-primary").text("RR: ")}),
                                                    html!("span", {.class("me-2").text(&vs.rr.map(|u| u.to_string()).unwrap_or_default()).text(" /min")}),
                                                    html!("span", {.class("text-primary").text("BP: ")}),
                                                    html!("span", {.class("me-2").text(&[vs.sbp.map(|u| u.to_string()).unwrap_or_default(), vs.dbp.map(|u| u.to_string()).unwrap_or_default()].join("/")).text(" mmHg")}),
                                                    html!("span", {.class("text-primary").text("E")}),
                                                    html!("span", {.text(&vs.eye.map(|i| i.to_string()).unwrap_or(String::from("-")))}),
                                                    html!("span", {.class("text-primary").text("V")}),
                                                    html!("span", {.text(&vs.verbal.clone().unwrap_or(String::from("-")))}),
                                                    html!("span", {.class("text-primary").text("M")}),
                                                    html!("span", {.class("me-2").text(&vs.movement.map(|i| i.to_string()).unwrap_or(String::from("-")))}),
                                                    html!("span", {.class("text-primary").text(scores.as_ref().map(|sc| sc.ews.label()).unwrap_or("EWS")).text(": ")}),
                                                    html!("span", {.class("me-2").text(&scores.as_ref().and_then(|sc| sc.ews.score()).unwrap_or_default().to_string())}),
                                                    html!("span", {.class("text-primary").text(scores.as_ref().map(|sc| sc.qsofa.label()).unwrap_or("qSOFA")).text(": ")}),
                                                    html!("span", {.class("me-2").text(&scores.as_ref().and_then(|sc| sc.qsofa.score()).unwrap_or_default().to_string())}),
                                                    html!("span", {.class("text-primary").text(scores.as_ref().map(|sc| sc.sirs.label()).unwrap_or("SIRS")).text(": ")}),
                                                    html!("span", {.class("me-2").text(&scores.as_ref().and_then(|sc| sc.sirs.score()).unwrap_or_default().to_string())}),
                                                    html!("span", {.class("text-primary").text("Braden Scale: ")}),
                                                    html!("span", {.class("me-2").text(&vs.braden.to_owned().unwrap_or_default())}),
                                                ])
                                                .apply(|d| {
                                                    if let Some(dt) = vs.vs_datetime {
                                                        d.child(html!("span", {
                                                            .class("text-primary")
                                                            .text(&["(", &datetime_th_relative(&dt), ")"].concat())
                                                        }))
                                                    } else {
                                                        d
                                                    }
                                                })
                                            } else {
                                                dom
                                            }
                                        })
                                    }),
                                    html!("div", {
                                        .child(html!("span", {.class(class::BOLD_L2).text("ประวัติการผ่าตัด: ")}))
                                        .child(html!("span", {.text(&admission_note.admission_note.clone()
                                            .and_then(|note| note.operation_history).unwrap_or_default())}))
                                        .children(admission_note.operation_list.iter().map(|op| {
                                            html!("p", {.text(op)})
                                        }))
                                    }),
                                    html!("div", {
                                        .child(html!("span", {.class(class::BOLD_L2).text("ประวัติการแพ้: ")}))
                                        .child(html!("span", {
                                            .class(class::BOLD_RED_L)
                                            .child(html!("span", {
                                                .text_signal(page.patient.signal_cloned().map(|pt_opt| {
                                                    pt_opt.as_ref().and_then(|pt| pt.drugallergy.clone()).unwrap_or_default()
                                                }))
                                            }))
                                            .apply_if(admission_note.admission_note.as_ref().map(|note| note.allergy_drug_history.is_some()).unwrap_or_default(), |dom| {
                                                dom.child(html!("div", {
                                                    .style("margin-left","50px")
                                                    .child(html!("span", {.text("ยา: ")}))
                                                    .children(admission_note.admission_note.as_ref().and_then(|note| {
                                                        note.allergy_drug_history.as_ref().map(|dh| {
                                                            explode(dh, 2).iter().map(|chunk| {
                                                                html!("div", {
                                                                    .style("margin-left","50px")
                                                                    .children([
                                                                        html!("span", {.text("ชื่อ: ")}),
                                                                        html!("span", {.text(&chunk[0].to_owned())}),
                                                                        html!("span", {.text(", อาการที่แพ้: ")}),
                                                                        html!("span", {.text(&chunk[1].to_owned())}),
                                                                    ])
                                                                })
                                                            }).collect::<Vec<Dom>>()
                                                        })
                                                    }).unwrap_or_default())
                                                }))
                                            })
                                            .apply_if(admission_note.admission_note.as_ref().map(|note| note.allergy_food_history.is_some()).unwrap_or_default(), |dom| {
                                                dom.child(html!("div", {
                                                    .style("margin-left","50px")
                                                    .child(html!("span", {.text("อาหาร: ")}))
                                                    .children(admission_note.admission_note.as_ref().and_then(|note| {
                                                        note.allergy_food_history.as_ref().map(|fh| {
                                                            explode(fh, 2).iter().map(|chunk| {
                                                                html!("div", {
                                                                    .style("margin-left","50px")
                                                                    .children([
                                                                        html!("span", {.text("ชื่อ: ")}),
                                                                        html!("span", {.text(&chunk[0].to_owned())}),
                                                                        html!("span", {.text(", อาการที่แพ้: ")}),
                                                                        html!("span", {.text(&chunk[1].to_owned())}),
                                                                    ])
                                                                })
                                                            }).collect::<Vec<Dom>>()
                                                        })
                                                    }).unwrap_or_default())
                                                }))
                                            })
                                            .apply_if(admission_note.admission_note.as_ref().map(|note| note.allergy_etc_history.is_some()).unwrap_or_default(), |dom| {
                                                dom.child(html!("div", {
                                                    .style("margin-left","50px")
                                                    .child(html!("span", {.text("อื่นๆ: ")}))
                                                    .children(admission_note.admission_note.as_ref().and_then(|note| {
                                                        note.allergy_etc_history.as_ref().map(|oh| {
                                                            explode(oh, 2).iter().map(|chunk| {
                                                                html!("div", {
                                                                    .style("margin-left","50px")
                                                                    .children([
                                                                        html!("span", {.text("ชื่อ: ")}),
                                                                        html!("span", {.text(&chunk[0].to_owned())}),
                                                                        html!("span", {.text(", อาการที่แพ้: ")}),
                                                                        html!("span", {.text(&chunk[1].to_owned())}),
                                                                    ])
                                                                })
                                                            }).collect::<Vec<Dom>>()
                                                        })
                                                    }).unwrap_or_default())
                                                }))
                                            })
                                        }))
                                    })
                                ])
                                .apply_if(admission_note.admission_note.as_ref().map(|note| {
                                    note.last_child.is_some() || note.last_abort.is_some() || note.curette.is_some() || note.lmp.is_some() || note.edc.is_some()
                                }).unwrap_or_default(), |dom| {
                                    let last_child = admission_note.admission_note.as_ref().and_then(|note| note.last_child.clone());
                                    let last_abort = admission_note.admission_note.as_ref().and_then(|note| note.last_abort.clone());
                                    let curette = admission_note.admission_note.as_ref().and_then(|note| note.curette.clone());
                                    let lmp = admission_note.admission_note.as_ref().and_then(|note| note.lmp.clone());
                                    let edc = admission_note.admission_note.as_ref().and_then(|note| note.edc.clone());
                                    dom.child(html!("span", {.class(class::BOLD_L2).text("ประวัติด้านสูตินรีเวชกรรม: ")}))
                                    .apply_if(last_child.is_some(), |d| {
                                        d.child(html!("div", {
                                            .style("margin-left","50px")
                                            .children([
                                                html!("span", {.class(class::BOLD_BLUE).text("Last child: ")}),
                                                html!("span", {.text(&last_child.unwrap_or_default().to_string())}),
                                                html!("span", {.class("ms-1").text("ปี")}),
                                            ])
                                        }))
                                    })
                                    .apply_if(last_abort.is_some(), |d| {
                                        d.child(html!("div", {
                                            .style("margin-left","50px")
                                            .children([
                                                html!("span", {.class(class::BOLD_BLUE).text("Last abort: ")}),
                                                html!("span", {.text(&last_abort.unwrap_or_default())}),
                                            ])
                                        }))
                                    })
                                    .apply_if(curette.is_some(), |d| {
                                        d.child(html!("div", {
                                            .style("margin-left","50px")
                                            .children([
                                                html!("span", {.class(class::BOLD_BLUE).text("ประวัติการขูดมดลูก: ")}),
                                                html!("span", {.text(if curette.unwrap_or_default() == "Y" {"เคย"} else {"ไม่เคย"})}),
                                            ])
                                        }))
                                    })
                                    .apply_if(lmp.is_some(), |d| {
                                        d.child(html!("div", {
                                            .style("margin-left","50px")
                                            .children([
                                                html!("span", {.class(class::BOLD_BLUE).text("ประจําเดือนครั้งสุดท้าย: ")}),
                                                html!("span", {.text(&date_th_opt(&lmp))}),
                                            ])
                                        }))
                                    })
                                    .apply_if(edc.is_some(), |d| {
                                        d.child(html!("div", {
                                            .style("margin-left","50px")
                                            .children([
                                                html!("span", {.class(class::BOLD_BLUE).text("กําหนดการคลอด: ")}),
                                                html!("span", {.text(&date_th_opt(&edc))}),
                                            ])
                                        }))
                                    })
                                })
                                .apply_if(admission_note.admission_note.as_ref().map(|note| {
                                    note.pb_no.is_some() || note.giant_baby.is_some() || note.distocia.is_some() || note.extraction.is_some() || note.pph.is_some() || note.pb_etc.is_some()
                                }).unwrap_or_default(), |dom| {
                                    let pb_no = admission_note.admission_note.as_ref().and_then(|note| note.pb_no.clone());
                                    let giant_baby = admission_note.admission_note.as_ref().and_then(|note| note.giant_baby.clone());
                                    let distocia = admission_note.admission_note.as_ref().and_then(|note| note.distocia.clone());
                                    let extraction = admission_note.admission_note.as_ref().and_then(|note| note.extraction.clone());
                                    let pph = admission_note.admission_note.as_ref().and_then(|note| note.pph.clone());
                                    let pb_etc = admission_note.admission_note.as_ref().and_then(|note| note.pb_etc.clone());
                                    dom.child(html!("span", {.class(class::BOLD_L2).text("ประวัติการคลอด: ")}))
                                    .apply_if(pb_no.is_some(), |d| {
                                        d.child(html!("div", {
                                            .style("margin-left","50px")
                                            .child(html!("span", {.class("me-1").text("ปฏิเสธ")}))
                                        }))
                                    })
                                    .apply_if(giant_baby.is_some(), |d| {
                                        d.child(html!("div", {
                                            .style("margin-left","50px")
                                            .child(html!("span", {.class("me-1").text("เคยคลอดบุตร นน. > 4000 กรัม")}))
                                        }))
                                    })
                                    .apply_if(distocia.is_some(), |d| {
                                        d.child(html!("div", {
                                            .style("margin-left","50px")
                                            .child(html!("span", {.class("me-1").text("มีประวัติคลอดยาก")}))
                                        }))
                                    })
                                    .apply_if(extraction.is_some(), |d| {
                                        d.child(html!("div", {
                                            .style("margin-left","50px")
                                            .children([
                                                html!("span", {.class("me-1").text("มีประวัติคลอดหัตถการ (ระบุ): ")}),
                                                html!("span", {.text(&extraction.unwrap_or_default())}),
                                            ])
                                        }))
                                    })
                                    .apply_if(pph.is_some(), |d| {
                                        d.child(html!("div", {
                                            .style("margin-left","50px")
                                            .child(html!("span", {.class("me-1").text("มีประวัติตกเลือดหลังคลอด")}))
                                        }))
                                    })
                                    .apply_if(pb_etc.is_some(), |d| {
                                        d.child(html!("div", {
                                            .style("margin-left","50px")
                                            .children([
                                                html!("span", {.class("me-1").text("อื่นๆ: ")}),
                                                html!("span", {.text(&pb_etc.unwrap_or_default())}),
                                            ])
                                        }))
                                    })
                                })
                                .apply_if(admission_note.admission_note.as_ref().map(|note| {
                                    note.hf.is_some() || note.hf_position.is_some()
                                }).unwrap_or_default(), |dom| {
                                    let hf = admission_note.admission_note.as_ref().and_then(|note| note.hf.clone());
                                    let hf_position = admission_note.admission_note.as_ref().and_then(|note| note.hf_position.clone());
                                    dom.child(html!("span", {.class(class::BOLD_L2).text("ตรวจหน้าท้อง: ")}))
                                    .apply_if(hf.is_some(), |d| {
                                        d.child(html!("div", {
                                            .style("margin-left","50px")
                                            .children([
                                                html!("span", {.class(class::BOLD_BLUE).text("High of fundus: ")}),
                                                html!("span", {.text(&hf.unwrap_or_default().to_string())}),
                                                html!("span", {.class("ms-1").text("cm.")}),
                                            ])
                                        }))
                                    })
                                    .apply_if(hf_position.is_some(), |d| {
                                        d.child(html!("div", {
                                            .style("margin-left","50px")
                                            .children([
                                                html!("span", {.class(class::BOLD_BLUE).text("Position: ")}),
                                                html!("span", {.text(&hf_position.unwrap_or_default())}),
                                            ])
                                        }))
                                    })
                                })
                                .apply_if(admission_note.admission_note.as_ref().map(|note| {
                                    note.condition_pregnant.is_some()
                                }).unwrap_or_default(), |dom| {
                                    dom.child(html!("div", {
                                        .children([
                                            html!("span", {.class(class::BOLD_L2).text("อาการระหว่างตั้งครรภ์: ")}),
                                            html!("span", {.text(&admission_note.admission_note.as_ref().and_then(|note| note.condition_pregnant.clone()).unwrap_or_default())}),
                                        ])
                                    }))
                                })
                                .apply_if(admission_note.admission_note.as_ref().map(|note| {
                                    note.hiv.is_some() || note.vdrl.is_some() || note.hbs_ag.is_some() || note.hct.is_some() ||
                                    note.hiv2.is_some() || note.vdrl2.is_some() || note.hbs_ag2.is_some() || note.hct2.is_some() ||
                                    note.gr.is_some() || note.thalassemia.is_some() || note.husband.is_some()
                                }).unwrap_or_default(), |dom| {
                                    let hiv = admission_note.admission_note.as_ref().and_then(|note| note.hiv.clone());
                                    let vdrl = admission_note.admission_note.as_ref().and_then(|note| note.vdrl.clone());
                                    let hbs_ag = admission_note.admission_note.as_ref().and_then(|note| note.hbs_ag.clone());
                                    let hct = admission_note.admission_note.as_ref().and_then(|note| note.hct.clone());
                                    let hiv2 = admission_note.admission_note.as_ref().and_then(|note| note.hiv2.clone());
                                    let vdrl2 = admission_note.admission_note.as_ref().and_then(|note| note.vdrl2.clone());
                                    let hbs_ag2 = admission_note.admission_note.as_ref().and_then(|note| note.hbs_ag2.clone());
                                    let hct2 = admission_note.admission_note.as_ref().and_then(|note| note.hct2.clone());
                                    let gr = admission_note.admission_note.as_ref().and_then(|note| note.gr.clone());
                                    let thalassemia = admission_note.admission_note.as_ref().and_then(|note| note.thalassemia.clone());
                                    let husband = admission_note.admission_note.as_ref().and_then(|note| note.husband.clone());
                                    dom.child(html!("span", {.class(class::BOLD_L2).text("ผลเลือด: ")}))
                                    .apply_if(hiv.is_some(), |d| {
                                        d.child(html!("div", {
                                            .style("margin-left","50px")
                                            .children([
                                                html!("span", {.class(class::BOLD_BLUE).text("HIV 1: ")}),
                                                html!("span", {.text(&hiv.unwrap_or_default())}),
                                            ])
                                        }))
                                    })
                                    .apply_if(vdrl.is_some(), |d| {
                                        d.child(html!("div", {
                                            .style("margin-left","50px")
                                            .children([
                                                html!("span", {.class(class::BOLD_BLUE).text("VDRL 1: ")}),
                                                html!("span", {.text(&vdrl.unwrap_or_default())}),
                                            ])
                                        }))
                                    })
                                    .apply_if(hbs_ag.is_some(), |d| {
                                        d.child(html!("div", {
                                            .style("margin-left","50px")
                                            .children([
                                                html!("span", {.class(class::BOLD_BLUE).text("HBsAg 1: ")}),
                                                html!("span", {.text(&hbs_ag.unwrap_or_default())}),
                                            ])
                                        }))
                                    })
                                    .apply_if(hct.is_some(), |d| {
                                        d.child(html!("div", {
                                            .style("margin-left","50px")
                                            .children([
                                                html!("span", {.class(class::BOLD_BLUE).text("HCT 1: ")}),
                                                html!("span", {.text(&hct.unwrap_or_default().to_string())}),
                                                html!("span", {.class("ms-1").text("%")}),
                                            ])
                                        }))
                                    })
                                    .apply_if(hiv2.is_some(), |d| {
                                        d.child(html!("div", {
                                            .style("margin-left","50px")
                                            .children([
                                                html!("span", {.class(class::BOLD_BLUE).text("HIV 2: ")}),
                                                html!("span", {.text(&hiv2.unwrap_or_default())}),
                                            ])
                                        }))
                                    })
                                    .apply_if(vdrl2.is_some(), |d| {
                                        d.child(html!("div", {
                                            .style("margin-left","50px")
                                            .children([
                                                html!("span", {.class(class::BOLD_BLUE).text("VDRL 2: ")}),
                                                html!("span", {.text(&vdrl2.unwrap_or_default())}),
                                            ])
                                        }))
                                    })
                                    .apply_if(hbs_ag2.is_some(), |d| {
                                        d.child(html!("div", {
                                            .style("margin-left","50px")
                                            .children([
                                                html!("span", {.class(class::BOLD_BLUE).text("HBsAg 2: ")}),
                                                html!("span", {.text(&hbs_ag2.unwrap_or_default())}),
                                            ])
                                        }))
                                    })
                                    .apply_if(hct2.is_some(), |d| {
                                        d.child(html!("div", {
                                            .style("margin-left","50px")
                                            .children([
                                                html!("span", {.class(class::BOLD_BLUE).text("HCT 2: ")}),
                                                html!("span", {.text(&hct2.unwrap_or_default().to_string())}),
                                                html!("span", {.class("ms-1").text("%")}),
                                            ])
                                        }))
                                    })
                                    .apply_if(gr.is_some(), |d| {
                                        d.child(html!("div", {
                                            .style("margin-left","50px")
                                            .children([
                                                html!("span", {.class(class::BOLD_BLUE).text("Blood group: ")}),
                                                html!("span", {.text(&gr.unwrap_or_default())}),
                                            ])
                                        }))
                                    })
                                    .apply_if(thalassemia.is_some(), |d| {
                                        d.child(html!("div", {
                                            .style("margin-left","50px")
                                            .children([
                                                html!("span", {.class(class::BOLD_BLUE).text("ผล thalassemia ตัวเอง: ")}),
                                                html!("span", {.text(&thalassemia.unwrap_or_default())}),
                                            ])
                                        }))
                                    })
                                    .apply_if(husband.is_some(), |d| {
                                        d.child(html!("div", {
                                            .style("margin-left","50px")
                                            .children([
                                                html!("span", {.class(class::BOLD_BLUE).text("ผล thalassemia สามี: ")}),
                                                html!("span", {.text(&husband.unwrap_or_default())}),
                                            ])
                                        }))
                                    })
                                })
                                .apply_if(!admission_note.admission_note_doctors.is_empty(), |dom| {
                                    dom.child(html!("div", {
                                        .child(html!("span", {.class(class::BOLD_L2).text("แพทย์ผู้บันทึก: ")}))
                                        .children(admission_note.admission_note_doctors.iter().map(|d| {
                                            let licenseno = d.licenseno.as_ref().map(|no| [" (", no, ")"].concat()).unwrap_or_default();
                                            html!("div", {
                                                .style("margin-left","50px")
                                                .text(&[d.admission_note_doctorname.clone().unwrap_or_default(), licenseno].concat())
                                            })
                                        }))
                                    }))
                                })
                                .apply_if(!admission_note.doctor_in_charge.is_empty(), |dom| {
                                    dom.child(html!("div", {
                                        .style("margin-left","50px")
                                        .child(html!("span", {.class(class::BOLD_L2).text("แพทย์เจ้าของไข้: ")}))
                                        .children(admission_note.doctor_in_charge.iter().map(|d| {
                                            html!("div", {
                                                .style("margin-left","50px")
                                                .text(d)
                                            })
                                        }))
                                    }))
                                })
                                .apply(|dom| {
                                    let admit_datetime = date_and_time_th_opt_relative(&admission_note.admission_note.as_ref().and_then(|n| n.receiver_medication_date), &admission_note.admission_note.as_ref().and_then(|n| n.receiver_medication_time));
                                    if !admit_datetime.trim().is_empty() {
                                        dom.child(html!("div", {
                                            .children([
                                                html!("span", {.class(class::BOLD_L2).text("แรกรับผู้ป่วยใน (จากการประเมินสภาพผู้ป่วยแรกรับและแบบแผนสุขภาพ): ")}),
                                                html!("div", {
                                                    .style("margin-left","50px")
                                                    .children([
                                                        html!("span", {.class("fw-bold").text("วันทีรับไว้รักษา: ")}),
                                                        html!("span", {.text(&admit_datetime)}),
                                                    ])
                                                }),
                                            ])
                                        }))
                                        .children_signal_vec(page.admission_note_nurse.signal_cloned().map(|opt| {
                                            if let Some(admission_note_nurse) = opt {
                                                vec![
                                                    html!("div", {
                                                        .style("margin-left","50px")
                                                        .children([
                                                            html!("span", {.class(class::BOLD_L2).text("CC: ")}),
                                                            html!("span", {.text(&admission_note_nurse.chief_complaints.clone().unwrap_or_default())}),
                                                        ])
                                                    }),
                                                    html!("div", {
                                                        .style("margin-left","50px")
                                                        .children([
                                                            html!("span", {.class(class::BOLD_L2).text("PI: ")}),
                                                            html!("span", {.text(&admission_note_nurse.medical_history.clone().unwrap_or_default())}),
                                                        ])
                                                    }),
                                                    html!("div", {
                                                        .style("margin-left","50px")
                                                        .children([
                                                            html!("span", {.class(class::BOLD_L2).text("สัญญาณชีพแรกรับผู้ป่วยใน: ")}),
                                                            html!("span", {.text(&admission_note_nurse.vs_admit.clone().unwrap_or_default())}),
                                                        ])
                                                    }),
                                                ]
                                            } else {
                                                Vec::new()
                                            }
                                        }).to_signal_vec())
                                    } else {
                                        dom
                                    }
                                })
                            })
                        ])
                    })
                })
            })))
            .apply_if(app.endpoint_is_allow(&Method::GET, &EndPoint::IpdIndexNote, is_pre_admit), |dom| { dom
                .child(html!("div", {
                    //.attr("id", "nurse_index_note_card_div")
                    .class(class::CARD)
                    .children([
                        html!("div", {
                            .class(class::CARD_HEAD)
                            .child(html!("span",{.text("Note")}))
                            .apply_if(!is_aside && app.endpoint_is_allow(&Method::POST, &EndPoint::IpdIndexNote, is_pre_admit), |d| { d
                                .child(html!("button", {
                                    .attr("type", "button")
                                    .class(class::BTN_SM_FR_GRAY)
                                    .attr("data-bs-toggle", "modal")
                                    .attr("data-bs-target", "#nurseIndexNoteFormModal")
                                    .text("แก้ไข")
                                    .event(clone!(page => move |_: events::Click| {
                                        match page.patient.lock_ref().as_ref().map(|pt| pt.visit_type()) {
                                            Some(VisitTypeId::Ipd(an))
                                            | Some(VisitTypeId::PreAdmit(an)) => {
                                                let index_note = IndexNote {
                                                    nurse_index_note_id: page.nurse_index_note_id.get(),
                                                    an: str_some(an),
                                                    nurse_index_note: str_some(page.nurse_index_note.get_cloned()),
                                                };
                                                // POST `EndPoint::IpdIndexNote`
                                                page.index_note_modal.set(Some(IndexNoteForm::load(&index_note)));
                                                // .attr("onclick", "setIndexPlanActionActionDateTime(event)")
                                            }
                                            Some(VisitTypeId::OpdEr(_, _))
                                            | Some(VisitTypeId::Visit(_))
                                            | None => {}
                                        }
                                    }))
                                }))
                            })
                        }),
                        html!("div", {
                            //.attr("id", "nurse_index_note_display")
                            .class("card-body")
                            .text_signal(page.nurse_index_note.signal_cloned())
                        })
                    ])
                }))
            })
            .child(html!("div", {
                .class("modal")
                .attr("id", "nurseIndexNoteFormModal")
                .attr("role", "dialog")
                .attr("tabindex", "-1")
                .child_signal(page.index_note_modal.signal_cloned().map(clone!(app, page => move |opt| {
                    opt.as_ref().map(clone!(app, page => move |modal| {
                        IndexNoteForm::render(
                            modal.clone(),
                            // page.index_note_modal.clone(),
                            Some(page.reload_index_note.clone()),
                            app,
                        )
                    })).or(Some(blank_modal()))
                })))
            }))
        })
    }
}
