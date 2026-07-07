// ipd-show-patient-main.php
// ipd-show-patient-main-sticky.php
// opd-er-show-patient-main.php
// opd-er-show-patient-main-sticky.php

use dominator::{Dom, clone, events, html, is_window_loaded};
use futures_signals::{
    map_ref,
    signal::{Mutable, SignalExt, not},
};
use rust_decimal::Decimal;
use std::rc::Rc;
use wasm_bindgen::JsCast;
use web_sys::HtmlDivElement;

use kphis_model::{app::VisitTypeId, endpoint::EndPoint, fetch::Method, patient_info::PatientInfo, route::Route};
use kphis_ui_app::App;
use kphis_ui_core::class;
use kphis_util::{
    datetime::{date_th_opt, datetime_th, time_hm_opt},
    util::{opt_empty_none, raw_concat_to_comma_equal, str_some, zero_none},
};

/// - GET `EndPoint::IpdShowPatientMainAn`
/// - GET `EndPoint::OpdErShowPatientMainVn`
/// - GET `EndPoint::OpdErShowPatientMainId`
/// - PATCH `EndPoint::IpdAdmissionNoteDrPharmCheckAn` (guarded, disable pharm-check)
#[derive(Default)]
pub struct ShowPatientMainCpn {
    sticky: Mutable<bool>,
    top: Mutable<i32>,

    success: Mutable<bool>,
    pub loaded: Mutable<bool>,

    pub patient: Mutable<Option<Rc<PatientInfo>>>,

    pub is_ipd: bool,
    pub an: Mutable<String>,
    pub hn: Mutable<String>,
    pub vn: Mutable<Option<String>>,
    pub opd_er_order_master_id: Mutable<u32>,

    allergy_drug_history: Mutable<Option<String>>,
    allergy_drug_pharmacy_check_person: Mutable<Option<String>>,
}

impl ShowPatientMainCpn {
    pub fn new_with_an(an: String) -> Rc<Self> {
        Rc::new(Self {
            is_ipd: true,
            an: Mutable::new(an),
            ..Default::default()
        })
    }

    pub fn new_with_id(opd_er_order_master_id: u32) -> Rc<Self> {
        Rc::new(Self {
            is_ipd: false,
            opd_er_order_master_id: Mutable::new(opd_er_order_master_id),
            ..Default::default()
        })
    }

    pub fn new_with_vn(vn: String) -> Rc<Self> {
        Rc::new(Self {
            is_ipd: false,
            vn: Mutable::new(str_some(vn)),
            ..Default::default()
        })
    }

    fn load(page: Rc<Self>, app: Rc<App>) {
        app.async_load(
            true,
            clone!(app, page => async move {

                let visit_type = if page.is_ipd {
                    let an = page.an.get_cloned();
                    if an.len() > app.hosxp_an_len() {
                        VisitTypeId::PreAdmit(an)
                    } else {
                        VisitTypeId::Ipd(an)
                    }
                } else if let Some(opd_er_order_master_id) = zero_none(page.opd_er_order_master_id.get()) {
                    VisitTypeId::OpdEr(page.vn.get_cloned().unwrap_or_default(), opd_er_order_master_id)
                } else {
                    VisitTypeId::Visit(page.vn.get_cloned().unwrap_or_default())
                };

                // GET `EndPoint::IpdShowPatientMainAn`
                // GET `EndPoint::OpdErShowPatientMainVn`
                // GET `EndPoint::OpdErShowPatientMainId`
                let patient = load_patient_info(visit_type, app.clone()).await;

                if page.is_ipd {
                    page.allergy_drug_history.set_neq(patient.allergy_drug_history.clone());
                    page.allergy_drug_pharmacy_check_person.set_neq(patient.allergy_drug_pharmacy_check_person.clone());
                } else {
                    page.an.set(patient.an.clone().unwrap_or_default());
                }
                page.hn.set(patient.hn.clone().unwrap_or_default());
                page.vn.set(patient.vn.clone());

                page.patient.set(Some(Rc::new(patient)));
                page.success.set(true);
                page.loaded.set(true);
            }),
        )
    }

    fn patch_pharmacy_check(page: Rc<Self>, app: Rc<App>) {
        app.async_load(
            true,
            clone!(app, page => async move {
                if app.confirm("ยืนยันตรวจสอบการแพ้ยา").await {
                    let an = page.an.lock_ref();
                    if !an.is_empty() {
                        if let Some(doctorcode) = app.doctor_code() {
                            if page.is_ipd {
                                // PATCH `EndPoint::IpdAdmissionNoteDrPharmCheckAn`
                                match PatientInfo::call_api_patch(&an, app.state()).await {
                                    Ok(response) => if response.rows_affected > 0 {
                                        page.allergy_drug_pharmacy_check_person.set_neq(Some(doctorcode))
                                    }
                                    Err(e) => {
                                        app.alert_app_error(&e).await;
                                    }
                                }
                            }
                        }
                    }
                }
            }),
        );
    }

    pub fn render(is_compact: bool, page: Rc<Self>, app: Rc<App>) -> Dom {
        app.onscroll(clone!(app, page => move || {
            let top = page.top.get() as f64;
            if app.window_scroll_y() > top {
                page.sticky.set_neq(true);
            } else {
                page.sticky.set_neq(false);
            }
        }));

        html!("div" => HtmlDivElement, {
            .future(map_ref!(
                let busy = app.loader_is_loading(),
                let loaded = page.loaded.signal_cloned() =>
                !busy && !loaded
            ).for_each(clone!(app, page => move |ready| {
                if ready {
                    // load patient data
                    Self::load(page.clone(), app.clone());
                }
                async {}
            })))
            .child_signal(page.success.signal_cloned().map(clone!(page => move |success| {
                if success {
                    let update_allergy = html!("span", {
                        .child_signal(map_ref! {
                            let adh = page.allergy_drug_history.signal_cloned(),
                            let person = page.allergy_drug_pharmacy_check_person.signal_cloned() =>
                            (adh.clone(), person.is_some())
                        }.map(clone!(app, page => move |(adh, has_person)| adh.map(|allergy_drug_history| html!("span", {
                            .class("allergyDrugHistoryFromAdmissionNote")
                            .child(html!("label", {
                                .child(html!("span", {
                                    .class(class::BOLD_RED_L)
                                    .class("me-2")
                                    .text(&[
                                        "แจ้งแพ้ยา (แรกรับ) : ",
                                        &raw_concat_to_comma_equal(&allergy_drug_history),
                                        if has_person {" (ประเมินแล้ว)"} else {" (รอเภสัชประเมิน)"}
                                    ].concat())
                                    .apply(|dom| {
                                        let act_mut = Mutable::new(false);
                                        let is_pre_admit = app.is_pre_admit(&page.an.lock_ref());
                                        if !has_person && app.endpoint_is_allow(&Method::PATCH, &EndPoint::IpdAdmissionNoteDrPharmCheckAn, is_pre_admit) {
                                            dom.style("cursor","pointer")
                                            .event(clone!(act_mut => move |_: events::Click| {
                                                act_mut.set(true);
                                            }))
                                            .future(map_ref!{
                                                let busy = app.loader_is_loading(),
                                                let act = act_mut.signal() =>
                                                !busy && *act
                                            }.for_each(clone!(app, page, act_mut => move |ready| {
                                                if ready {
                                                    act_mut.set(false);
                                                    Self::patch_pharmacy_check(page.clone(), app.clone());
                                                }
                                                async {}
                                            })))
                                        } else {
                                            dom
                                        }
                                    })
                                }))
                            }))
                        })))))
                    });

                    page.patient.get_cloned().map(clone!(app, page => move |patient| {
                        render_patient_info(is_compact, patient, Some(update_allergy), Some(page), false, app.clone())
                    }))

                } else {
                    Some(html!("div", {
                        .class(class::ALERT_GRAY)
                        .attr("role", "alert")
                        .text("ไม่มีข้อมูลผู้ป่วย")
                    }))
                }
            })))
        })
    }
}

/// - GET `EndPoint::IpdShowPatientMainAn`
/// - GET `EndPoint::OpdErShowPatientMainVn`
/// - GET `EndPoint::OpdErShowPatientMainId`
pub async fn load_patient_info(visit_type: VisitTypeId, app: Rc<App>) -> PatientInfo {
    let result = match &visit_type {
        // GET `EndPoint::IpdShowPatientMainAn`
        VisitTypeId::Ipd(an) => PatientInfo::call_api_get_an(&an, app.state()).await,
        // GET `EndPoint::OpdErShowPatientMainVn`
        VisitTypeId::PreAdmit(vn) | VisitTypeId::Visit(vn) => PatientInfo::call_api_get_vn(&vn, app.state()).await,
        // GET `EndPoint::OpdErShowPatientMainId`
        VisitTypeId::OpdEr(_, opd_er_order_master_id) => PatientInfo::call_api_get_id(*opd_er_order_master_id, app.state()).await,
    };

    match result {
        Ok(Some(info)) => info,
        Ok(None) => {
            Route::NotFound { path: visit_type.vnan().to_owned() }.hard_redirect();
            PatientInfo::default()
        }
        Err(e) => {
            app.alert_app_error(&e).await;
            Route::NotFound { path: visit_type.vnan().to_owned() }.hard_redirect();
            PatientInfo::default()
        }
    }
}

/// set 'component' argument as Some for sticky feature<br>
/// set 'update_allergy' for as Some for pharmacist allergy comfirm feature <br>
/// minify will take effect when component is None
pub fn render_patient_info(is_compact: bool, info: Rc<PatientInfo>, update_allergy: Option<Dom>, component: Option<Rc<ShowPatientMainCpn>>, minify: bool, app: Rc<App>) -> Dom {
    let sticky = component.as_ref().map(|cmp| cmp.sticky.clone()).unwrap_or(Mutable::new(false));
    let mini = minify && component.is_none();

    sticky.set(is_compact);

    html!("div", {
        .apply(|dom| {
            if let Some(page) = component {
                dom.future(is_window_loaded().for_each(clone!(app, page => move |loaded| {
                    if loaded {
                        if let Some(elm) = app.get_id("patientInfoContainer").and_then(|elm| elm.dyn_into::<HtmlDivElement>().ok()) {
                            page.top.set_neq(elm.offset_top());
                        }
                    }
                    async {}
                })))
            } else {
                dom
            }
        })
        .children([
            html!("div", {
                .apply_if(!mini, |dom| { dom
                    .class(class::ALERT_GRAY)
                    .class(class::M0_PY0)
                    .attr("id", "patientInfoContainer")
                    .attr("role","alert")
                    .style("z-index","8")
                })
                .apply_if(!is_compact, |dom| { dom
                    .class_signal("fixed-top", sticky.signal())
                })
                .child(html!("div", {
                    .class(class::FLEX_C)
                    .children([
                        html!("div", {
                            .class("me-2")
                            .child(html!("img", {
                                // .class("img-thumbnail")
                                .attr("src", &info.image())
                                .attr("alt", "รูปผู้ป่วย")
                                .apply(|dom| {
                                    if mini || is_compact { dom
                                        .style("width","50px").style("min-width","50px")
                                    } else { dom
                                        .style_signal("width", sticky.signal_cloned().map(|sticky| {
                                            if sticky {"50px"} else {"120px"}
                                        }))
                                        .style_signal("min-width", sticky.signal_cloned().map(|sticky| {
                                            if sticky {"50px"} else {"120px"}
                                        }))
                                        .style("transition", "0.2s")
                                    }
                                })
                            }))
                        }),
                        html!("div", {
                            .apply(|dom| {
                                if mini { dom
                                    .class("small")
                                } else { dom
                                    .class(class::P_L0)
                                }
                            })
                            .child(html!("h5", {
                                .class(class::TXT_BLUE_EM)
                                .apply(|dom| {
                                    if mini { dom
                                        .visible(false)
                                    } else { dom
                                        .visible_signal(not(sticky.signal()))
                                    }
                                })
                                .text("ข้อมูลผู้ป่วย")
                                .apply(|dom| {
                                    if let Some(ward_name) = opt_empty_none(info.ward_name.clone()) {
                                        dom.text(" ").text(&ward_name)
                                    } else {
                                        dom
                                    }
                                })
                                .apply(|dom| {
                                    if let Some(bedno) = opt_empty_none(info.bedno.clone()) {
                                        dom.text(" เตียง ").text(&bedno)
                                    } else {
                                        dom
                                    }
                                })
                            }))
                            .apply(|dom| {
                                if let Some(ward_name) = opt_empty_none(info.ward_name.clone()) {
                                    dom.child(html!("span", {
                                        .apply(|d| {
                                            if mini || is_compact { d
                                                .visible(true)
                                            } else { d
                                                .visible_signal(sticky.signal())
                                            }
                                        })
                                        .class("d-inline-block")
                                        .children([
                                            html!("span", {.class(class::BOLD_BLUE_EM_L).text(&ward_name)})
                                        ])
                                    }))
                                } else {
                                    dom
                                }
                            })
                            .apply(|dom| {
                                if let Some(bedno) = opt_empty_none(info.bedno.clone()) {
                                    dom.child(html!("span", {
                                        .apply(|d| {
                                            if mini || is_compact { d
                                                .visible(true)
                                            } else { d
                                                .visible_signal(sticky.signal())
                                            }
                                        })
                                        .class("d-inline-block")
                                        .children([
                                            html!("span", {.class(class::BOLD_BLUE_EM_L).text(&bedno)})
                                        ])
                                    }))
                                } else {
                                    dom
                                }
                            })
                            .children([
                                html!("span", {
                                    .class("d-inline-block")
                                    .children([
                                        // html!("span", {.class(class::TXT_BLUE_EM).text("ชื่อ - สกุล :")}),
                                        html!("span", {
                                            .class(class::BOLD_BLUE_EM_L)
                                            .text(&[
                                                &info.pname.clone().unwrap_or_default(),
                                                &info.fname.clone().unwrap_or_default(), " ",
                                                &info.lname.clone().unwrap_or_default()
                                            ].concat())
                                        }),
                                    ])
                                }),
                                html!("span", {
                                    .class("d-inline-block")
                                    .children([
                                        html!("span", {.class(class::TXT_BLUE_EM).text("อายุ :")}),
                                        html!("span", {
                                            .class("me-2")
                                            .text(&[
                                                info.age_y.as_ref().map(|y| [" ", &y.to_string(), " ปี"].concat()).unwrap_or_default(),
                                                info.age_m.as_ref().map(|m| [" ", &m.to_string(), " เดือน"].concat()).unwrap_or_default(),
                                                info.age_d.as_ref().map(|d| [" ", &d.to_string(), " วัน"].concat()).unwrap_or_default()
                                            ].concat())
                                        }),
                                    ])
                                }),
                            ])
                            .apply(|dom| {
                                if let Some(hn) = &info.hn {
                                    dom.child(html!("span", {
                                        .class("d-inline-block")
                                        .children([
                                            html!("span", {.class(class::TXT_BLUE_EM).text("HN :")}),
                                            html!("span", {.class("me-2").text(&hn)})
                                        ])
                                    }))
                                } else {
                                    dom
                                }
                            })
                            .apply(|dom| {
                                if let Some(vn) = &info.vn {
                                    dom.child(html!("span", {
                                        .class("d-inline-block")
                                        .children([
                                            html!("span", {.class(class::TXT_BLUE_EM).text("VN :")}),
                                            html!("span", {.class("me-2").text(&vn)}),
                                        ])
                                    }))
                                } else {
                                    dom
                                }
                            })
                            .apply(|dom| {
                                if let Some(an) = &info.an {
                                    dom.child(html!("span", {
                                        .class("d-inline-block")
                                        .children([
                                            html!("span", {.class(class::TXT_BLUE_EM).text("AN :")}),
                                            html!("span", {.class("me-2").text(&an)}),
                                        ])
                                    }))
                                } else {
                                    dom
                                }
                            })
                            .apply(|dom| {
                                if let Some(cid) = &info.cid {
                                    dom.child(html!("span", {
                                        .class("d-inline-block")
                                        .children([
                                            html!("span", {.class(class::TXT_BLUE_EM).text("CID :")}),
                                            html!("span", {.class("me-2").text(&cid)}),
                                        ])
                                    }))
                                } else {
                                    dom
                                }
                            })
                            .apply(|dom| {
                                if let Some(spclty_name) = opt_empty_none(info.spclty_name.clone()) {
                                    dom.child(html!("span", {
                                        .class("d-inline-block")
                                        .children([
                                            html!("span", {.class(class::TXT_BLUE_EM).text("แผนก :")}),
                                            html!("span", {.class("me-2").text(&spclty_name)})
                                        ])
                                    }))
                                } else {
                                    dom
                                }
                            })
                            .child(html!("span", {
                                .class("d-inline-block")
                                .apply(|dom| {
                                    if mini { dom
                                        .visible(false)
                                    } else {
                                        dom
                                    }
                                })
                                .children([
                                    html!("span", {.class(class::TXT_BLUE_EM).text("สิทธิ :")}),
                                    html!("span", {
                                        .class("me-2")
                                        .text(&[
                                            info.pttype.as_ref().map(|ty| ["(",ty,") "].concat()).unwrap_or_default(),
                                            info.pttype_name.clone().unwrap_or_default()
                                        ].concat())
                                    })
                                ])
                            }))
                            .apply_if(!mini, |dom| { dom
                                .apply(|d| {
                                    if let Some(income) = info.income {
                                        d.child(html!("span", {
                                            .class("d-inline-block")
                                            .apply(|dom| {
                                                if mini { dom
                                                    .visible(false)
                                                } else {
                                                    dom
                                                }
                                            })
                                            .children([
                                                html!("span", {.class(class::TXT_BLUE_EM).text("ค่าใช้จ่าย :")}),
                                                html!("span", {
                                                    .class("me-2")
                                                    .text(&format!("{:.2} บาท", income))
                                                })
                                            ])
                                        }))
                                    } else {
                                        d
                                    }
                                })
                            })
                            .child(html!("br", {
                                .apply(|dom| {
                                    if mini || is_compact { dom
                                        .visible(false)
                                    } else { dom
                                        .visible_signal(not(sticky.signal()))
                                    }
                                })
                            }))
                            .apply_if(!mini, |dom| { dom
                                .apply(|d| {
                                    if info.regdate.is_some() || info.regtime.is_some() {
                                        let visit_name = if info.visit_type.is_admited() {"Admit : "} else {"Visit : "};
                                        d.child(html!("span", {
                                            .class(class::BOLD_BLUE_EM)
                                            .class(class::TXT_NOWRAP_L)
                                            .text(&[visit_name, &date_th_opt(&info.regdate), " ", &time_hm_opt(&info.regtime)].concat())
                                        }))
                                    } else {
                                        d
                                    }
                                })
                                .apply(|d| {
                                    if info.dchdate.is_some() {
                                        d.child(html!("span", {
                                            .class(class::BOLD_GOLD)
                                            .class("me-1")
                                            .text(&[
                                                "Discharge : ",
                                                &date_th_opt(&info.dchdate), " ",
                                                &time_hm_opt(&info.dchtime),
                                                &info.dchstts_name.as_ref().map(|stts| [" Status : ", stts].concat()).unwrap_or_default(),
                                                &info.dchtype_name.as_ref().map(|dty| [" Type : ", dty].concat()).unwrap_or_default()
                                            ].concat())
                                        }))
                                    } else {
                                        d
                                    }
                                })
                            })
                            .apply(|dom| {
                                if let Some(drugallergy) = &info.drugallergy {
                                    dom.child(html!("span", {
                                        .class(class::BOLD_RED_L)
                                        .class("me-1")
                                        .text(&["แพ้ยา : ", drugallergy].concat())
                                    }))
                                } else {
                                    dom
                                }
                            })
                            .apply(|dom| {
                                if let Some(er_drugallergy_history) = &info.er_drugallergy_history {
                                    dom.child(html!("span", {
                                        .class(class::BOLD_RED_L)
                                        .class("me-1")
                                        .text(&["แจ้งแพ้ยา (ER) : ", er_drugallergy_history].concat())
                                    }))
                                } else {
                                    dom
                                }
                            })
                            .apply(|dom| {
                                if let Some(update) = update_allergy {
                                    dom.child(update)
                                } else {
                                    dom
                                }
                            })
                            .apply(|dom| {
                                if info.drugallergy.is_none() && info.allergy_drug_history.is_none() && info.er_drugallergy_history.is_none() {
                                    dom.child(html!("span", {.class("me-2").text("ไม่มีข้อมูลการแพ้ยา")}))
                                } else {
                                    dom
                                }
                            })
                            .apply_if(!mini, |dom| { dom
                                .apply(|d| {
                                    if let Some(mut latest_bw) = info.latest_bw.and_then(|bw| (bw > Decimal::ZERO).then_some(bw)) {
                                        if latest_bw > Decimal::new(10, 0) {
                                            latest_bw.rescale(1);
                                        }
                                        d.child(html!("span", {
                                            .class("d-inline-block")
                                            .children([
                                                html!("span", {.class(class::TXT_BLUE_EM).text("น้ำหนักตัวล่าสุด :")}),
                                                html!("span", {
                                                    .class("me-2")
                                                    .text(&[
                                                        &latest_bw.to_string(), " kg",
                                                        &info.latest_bw_datetime.as_ref().map(|dt| [" (", &datetime_th(dt),")"].concat()).unwrap_or_default()
                                                    ].concat())
                                                })
                                            ])
                                        }))
                                    } else {
                                        d
                                    }
                                })
                            })
                            .child(html!("span", {
                                .class("d-inline-block")
                                .style("cursor", "help")
                                .apply(|dom| {
                                    if mini || is_compact { dom
                                        .visible(false)
                                    } else { dom
                                        .visible_signal(not(sticky.signal()))
                                    }
                                })
                                .children([
                                    html!("span", {.class(class::TXT_BLUE_EM).text("ติดต่อ :")}),
                                    html!("span", {.class("mx-2").child(html!("i", {.class(class::FA_PHONE)}))}),
                                    html!("span", {.text(&info.hometel.clone().unwrap_or_default())}),
                                ])
                                .attr("title", &[
                                    "บ้าน : ", &opt_empty_none(info.hometel.clone()).as_ref().map(|ht| ["โทร ", ht].concat()).unwrap_or_default(), " ", &info.homeaddr.clone().unwrap_or_default(), "\n",
                                    "ที่ทำงาน : ", &opt_empty_none(info.worktel.clone()).as_ref().map(|wt| ["โทร ", wt].concat()).unwrap_or_default(), " ", &info.workaddr.clone().unwrap_or_default(), "\n",
                                    "ผู้ติดต่อ : ", &opt_empty_none(info.informtel.clone()).as_ref().map(|it| ["โทร ", it].concat()).unwrap_or_default(), " ", &info.informname.clone().unwrap_or_default(), " ",
                                    &info.informrelation.as_ref().map(|ir| [" (", ir,")"].concat()).unwrap_or_default(), " ", &info.informaddr.clone().unwrap_or_default()
                                ].concat())
                            }))
                        })
                    ])
                }))
            }),
            html!("div", {
                .apply(|dom| {
                    if mini || is_compact { dom
                        .style("height","0")
                    } else { dom
                        .style_signal("height", sticky.signal_cloned().map(|stick| {
                            if stick {"150px"} else {"0"}
                        }))
                    }
                })
            }),
        ])
    })
}
