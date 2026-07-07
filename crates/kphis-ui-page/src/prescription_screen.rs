use dominator::{Dom, EventOptions, clone, events, html, window_size, with_node};
use futures_signals::{
    map_ref,
    signal::{Mutable, SignalExt, not},
};
use std::rc::Rc;
use web_sys::{HtmlButtonElement, HtmlInputElement, HtmlTextAreaElement};

use kphis_model::{
    endpoint::EndPoint,
    fetch::Method,
    prescription::{LastMedicine, PostalPatch, PrescriptionInfo, PrescriptionScreen, PrescriptionScreenParams, PrescriptionScreenPatch, PrescriptionVn, TelemedPatch, VisitDate},
    route::Route,
};
use kphis_ui_app::App;
use kphis_ui_component::{
    lab::LabCmp,
    modal::{blank_modal, lab_history::LabHistory},
};
use kphis_ui_core::{class, doms, mixins};
use kphis_util::{
    datetime::{date_and_time_th_opt_relative, date_th_relative, datetime_from_opt, datetime_str_th_relative, datetime_th_relative},
    util::{f64_rescale, opt_zero_none, sanity_dot_space, str_some},
};

/// - GET `EndPoint::PrescrptionScreen`
/// - POST `EndPoint::PrescrptionScreen` (guarded, remove 'รับรายการ' btn)
/// - PATCH `EndPoint::PrescrptionScreen` (guarded, remove 'ตรวจสอบ' and 'จ่ายยา' btn)
/// - GET `EndPoint::LabItem` (LabHistory, guarded, cannot click lab result)
#[derive(Clone, Default)]
pub struct PrescriptionScreenPage {
    loaded: Mutable<bool>,
    reload_visit: Mutable<bool>,
    changed: Mutable<bool>,

    search_text: Mutable<String>,
    current_date: Mutable<Option<VisitDate>>,

    info: Mutable<Option<Rc<PrescriptionInfo>>>,
    visit: Mutable<Option<Rc<PrescriptionVn>>>,

    telemed_changed: Mutable<bool>,
    telemed_add: Mutable<String>,
    telemed_dose_up: Mutable<String>,
    telemed_dose_down: Mutable<String>,
    telemed_off: Mutable<String>,
    telemed_other: Mutable<String>,

    pharmacy_care_changed: Mutable<bool>,
    pharmacy_care: Mutable<String>,

    lab_history_modal: Mutable<Option<Rc<LabHistory>>>,
}

impl PrescriptionScreenPage {
    pub fn new(search_text: String) -> Rc<Self> {
        Rc::new(Self {
            search_text: Mutable::new(search_text),
            ..Default::default()
        })
    }

    fn set_telemed_opt(&self, visit_opt: &Option<PrescriptionVn>) {
        self.telemed_add.set_neq(visit_opt.as_ref().and_then(|visit| visit.telemed_add.clone()).unwrap_or_default());
        self.telemed_dose_up.set_neq(visit_opt.as_ref().and_then(|visit| visit.telemed_dose_up.clone()).unwrap_or_default());
        self.telemed_dose_down.set_neq(visit_opt.as_ref().and_then(|visit| visit.telemed_dose_down.clone()).unwrap_or_default());
        self.telemed_off.set_neq(visit_opt.as_ref().and_then(|visit| visit.telemed_off.clone()).unwrap_or_default());
        self.telemed_other.set_neq(visit_opt.as_ref().and_then(|visit| visit.telemed_other.clone()).unwrap_or_default());
        self.telemed_changed.set_neq(false);
    }

    fn set_telemed(&self, visit: &PrescriptionVn) {
        self.telemed_add.set_neq(visit.telemed_add.clone().unwrap_or_default());
        self.telemed_dose_up.set_neq(visit.telemed_dose_up.clone().unwrap_or_default());
        self.telemed_dose_down.set_neq(visit.telemed_dose_down.clone().unwrap_or_default());
        self.telemed_off.set_neq(visit.telemed_off.clone().unwrap_or_default());
        self.telemed_other.set_neq(visit.telemed_other.clone().unwrap_or_default());
        self.telemed_changed.set_neq(false);
    }

    // pharmacy-prescription-screen-data-select.php
    fn submit_search(page: Rc<Self>, app: Rc<App>) {
        let search = str_some(page.search_text.get_cloned());
        if search.is_some() {
            let params = PrescriptionScreenParams { search, ..Default::default() };

            app.async_load(
                true,
                clone!(app, page => async move {
                    // GET `EndPoint::PrescrptionScreen`
                    match PrescriptionScreen::call_api_get(&params, app.state()).await {
                        Ok(screen) => {
                            if screen.info.is_some() {
                                page.current_date.set(screen.info.as_ref().and_then(|info| info.dates.first().cloned()));
                                page.info.set(screen.info.map(Rc::new));
                            }
                            page.set_telemed_opt(&screen.visit);
                            page.pharmacy_care.set(screen.visit.as_ref().and_then(|visit| visit.pharmacy_care.clone()).unwrap_or_default());
                            page.pharmacy_care_changed.set_neq(false);
                            if screen.visit.is_some() {
                                page.visit.set(screen.visit.map(Rc::new));
                            }
                        }
                        Err(e) => {
                            app.alert_app_error(&e).await;
                        }
                    }
                }),
            );
        }
    }

    fn get_visit(page: Rc<Self>, app: Rc<App>) {
        let vn_opt = page.current_date.lock_ref().as_ref().and_then(|date| date.vn.to_owned().and_then(str_some));
        if vn_opt.is_some() {
            let params = PrescriptionScreenParams { vn: vn_opt, ..Default::default() };

            app.async_load(
                true,
                clone!(app, page => async move {
                    // GET `EndPoint::PrescrptionScreen`
                    match PrescriptionScreen::call_api_get(&params, app.state()).await {
                        Ok(screen) => {
                            page.set_telemed_opt(&screen.visit);
                            page.pharmacy_care.set(screen.visit.as_ref().and_then(|visit| visit.pharmacy_care.clone()).unwrap_or_default());
                            page.pharmacy_care_changed.set_neq(false);
                            if screen.visit.is_some() {
                                page.visit.set(screen.visit.map(Rc::new));
                            }
                        }
                        Err(e) => {
                            app.alert_app_error(&e).await;
                        }
                    }
                }),
            );
        }
    }

    fn insert_pharmacist_accept(visit: Rc<PrescriptionVn>, page: Rc<Self>, app: Rc<App>) {
        app.async_load(
            true,
            clone!(app => async move {
                if app.confirm("ยืนยัน บันทึกการรับรายการ ด้วยเวลาปัจจุบัน").await {
                    let params = PrescriptionScreenParams { vn: visit.vn.clone(), ..Default::default() };
                    // POST `EndPoint::PrescrptionScreen`
                    match PrescriptionScreen::call_api_post(&params, app.state()).await {
                        Ok(response) => {
                            app.alert_execute_response(&response, async move {
                                page.reload_visit.set_neq(true);
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

    fn update_pharmacist_check(visit: Rc<PrescriptionVn>, page: Rc<Self>, app: Rc<App>) {
        app.async_load(
            true,
            clone!(app => async move {
                if app.confirm("ยืนยัน บันทึกการตรวจสอบ ด้วยเวลาปัจจุบัน").await {
                    let params = PrescriptionScreenParams { vn: visit.vn.clone(), action: Some(String::from("check")), ..Default::default() };
                    let patcher = PrescriptionScreenPatch::default();
                    // PATCH `EndPoint::PrescrptionScreen`
                    match patcher.call_api_patch(&params, app.state()).await {
                        Ok(response) => {
                            app.alert_execute_response(&response, async move {
                                page.reload_visit.set_neq(true);
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

    fn update_pharmacist_done(visit: Rc<PrescriptionVn>, page: Rc<Self>, app: Rc<App>) {
        app.async_load(
            true,
            clone!(app => async move {
                if app.confirm("ยืนยัน บันทึกการจ่ายยา ด้วยเวลาปัจจุบัน").await {
                    let params = PrescriptionScreenParams { vn: visit.vn.clone(), action: Some(String::from("done")), ..Default::default() };
                    let patcher = PrescriptionScreenPatch::default();
                    // PATCH `EndPoint::PrescrptionScreen`
                    match patcher.call_api_patch(&params, app.state()).await {
                        Ok(response) => {
                            app.alert_execute_response(&response, async move {
                                page.reload_visit.set_neq(true);
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

    fn update_postal(status: bool, visit: Rc<PrescriptionVn>, page: Rc<Self>, app: Rc<App>) {
        let postal = PostalPatch {
            postal_status: Some(if status { String::from("Y") } else { String::from("N") }),
        };
        app.async_load(
            true,
            clone!(app => async move {
                let message = if status {
                    "ยืนยัน ส่งยาทางไปรษณีย์"
                } else {
                    "ยืนยัน ยกเลิกการส่งยาทางไปรษณีย์"
                };
                if app.confirm(message).await {
                    let params = PrescriptionScreenParams { vn: visit.vn.clone(), action: Some(String::from("postal")), ..Default::default() };
                    let mut patcher = PrescriptionScreenPatch::default();
                    patcher.postal = Some(postal);
                    // PATCH `EndPoint::PrescrptionScreen`
                    match patcher.call_api_patch(&params, app.state()).await {
                        Ok(response) => {
                            app.alert_execute_response(&response, async move {
                                page.reload_visit.set_neq(true);
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

    fn update_telemed(visit: Rc<PrescriptionVn>, page: Rc<Self>, app: Rc<App>) {
        let telemed = TelemedPatch {
            telemed_add: str_some(page.telemed_add.get_cloned()),
            telemed_dose_up: str_some(page.telemed_dose_up.get_cloned()),
            telemed_dose_down: str_some(page.telemed_dose_down.get_cloned()),
            telemed_off: str_some(page.telemed_off.get_cloned()),
            telemed_other: str_some(page.telemed_other.get_cloned()),
        };
        app.async_load(
            true,
            clone!(app => async move {
                if app.confirm("ยืนยัน บันทึกการให้บริการ Telemed").await {
                    let params = PrescriptionScreenParams { vn: visit.vn.clone(), action: Some(String::from("telemed")), ..Default::default() };
                    let mut patcher = PrescriptionScreenPatch::default();
                    patcher.telemed = Some(telemed);
                    // PATCH `EndPoint::PrescrptionScreen`
                    match patcher.call_api_patch(&params, app.state()).await {
                        Ok(response) => {
                            app.alert_execute_response(&response, async move {
                                page.reload_visit.set_neq(true);
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

    fn update_pharmacy_care(visit: Rc<PrescriptionVn>, page: Rc<Self>, app: Rc<App>) {
        app.async_load(
            true,
            clone!(app => async move {
                if app.confirm("ยืนยัน บันทึก Pharmacy Care").await {
                    let params = PrescriptionScreenParams { vn: visit.vn.clone(), action: Some(String::from("pharmacy-care")), ..Default::default() };
                    let mut patcher = PrescriptionScreenPatch::default();
                    patcher.pharmacy_care = str_some(page.pharmacy_care.get_cloned());
                    // PATCH `EndPoint::PrescrptionScreen`
                    match patcher.call_api_patch(&params, app.state()).await {
                        Ok(response) => {
                            app.alert_execute_response(&response, async move {
                                page.reload_visit.set_neq(true);
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

    pub fn render(page: Rc<Self>, app: Rc<App>) -> Dom {
        app.set_title("KPHIS - Prescription Screen");

        html!("div", {
            .future(map_ref!(
                let busy = app.loader_is_loading(),
                let loaded = page.loaded.signal() =>
                !busy && !loaded
            ).for_each(clone!(app, page => move |ready| {
                if ready {
                    Self::submit_search(page.clone(), app.clone());
                    page.loaded.set(true);
                }
                async {}
            })))
            .future(map_ref!(
                let busy = app.loader_is_loading(),
                let changed = page.changed.signal() =>
                !busy && *changed
            ).for_each(clone!(app, page => move |ready| {
                if ready {
                    Self::submit_search(page.clone(), app.clone());
                    page.changed.set(false);
                }
                async {}
            })))
            .future(map_ref!(
                let busy = app.loader_is_loading(),
                let reload = page.reload_visit.signal() =>
                !busy && *reload
            ).for_each(clone!(app, page => move |ready| {
                if ready {
                    Self::get_visit(page.clone(), app.clone());
                    page.reload_visit.set(false);
                }
                async {}
            })))
            .children([
                html!("div", {
                    .class(class::CONF_B)
                    .child(html!("div", {
                        .class(class::ROW_MY2)
                        .children([
                            html!("div", {
                                .class("col-auto")
                                .child(html!("button", {
                                    .attr("type", "button")
                                    .class(class::BTN_BLUE)
                                    .child(html!("i", {.class(class::FA_L_ARROW)}))
                                    .text(" กลับ")
                                    .event(clone!(app => move |_: events::Click| {
                                        if app.go_back_else() {
                                            Route::Info.hard_redirect();
                                        }
                                    }))
                                }))
                            }),
                            html!("div", {
                                .class("col-auto")
                                .child(html!("h3", {.text("Screen ใบสั่งยา")}))
                            }),
                            html!("div", {
                                .class("col-auto")
                                .child(html!("div", {
                                    .class(class::INPUT_GROUP)
                                    .class("me-2")
                                    .children([
                                        doms::span_group_text("ค้นหา QN/HN/VN/CID"),
                                        html!("input" => HtmlInputElement, {
                                            .attr("type", "text")
                                            .class("form-control")
                                            .focused(true)
                                            .attr("placeholder", "QN(0-9999)/HN/VN/CID")
                                            //.apply(|dom| mixins::string_value(dom, page.search_text.clone(), page.changed.clone()))
                                            .prop_signal("value", page.search_text.signal_cloned())
                                            .with_node!(element => {
                                                .event_with_options(&EventOptions::preventable(), clone!(page, element => move |event: events::KeyDown| {
                                                    if event.key() == "Enter" {
                                                        event.prevent_default();
                                                        page.search_text.set_neq(element.value());
                                                        page.changed.set_neq(true);
                                                    }
                                                }))
                                                .event(clone!(page => move |_: events::Change| {
                                                    page.search_text.set_neq(element.value());
                                                }))
                                            })
                                        }),
                                        html!("button", {
                                            .attr("type", "button")
                                            .class(class::BTN_L_BLUE)
                                            .text("ค้นหา")
                                            .event(clone!(page => move |_: events::Click| {
                                                page.changed.set_neq(true);
                                            }))
                                        })
                                    ])
                                }))
                            }),
                        ])
                        .child_signal(app.loader_is_loading().map(|is_loading| {
                            is_loading.then(|| {
                                html!("div", {
                                    .class("col-auto")
                                    .child(html!("i",{.class(class::FA_SPIN).style("font-size","38px")}))
                                })
                            })
                        }))
                    }))
                    .child_signal(page.info.signal_cloned().map(clone!(app, page => move |info_opt| {
                        info_opt.as_ref().map(|info| {
                            html!("div", {
                                .children([
                                    html!("div", {
                                        .attr("id","prescription-info-row")
                                        .class(class::ROW_T)
                                        .child(html!("div", {
                                            .style("column-width","480px")
                                            .style("column-gap","8px")
                                            .children([
                                                Self::render_info_patient(info),
                                                Self::render_info_allergy(info),
                                                Self::render_info_note(info),
                                            ])
                                        }))
                                    }),
                                    html!("div", {
                                        .class(class::ROW_T)
                                        // left panel
                                        .child(html!("div", {
                                            .style("width","220px")
                                            .style_signal("height", window_size().map(clone!(app => move |ws| {
                                                let info_height = if let Some(elm) = app.get_id("prescription-info-row") {
                                                    elm.client_height()
                                                } else {
                                                    128
                                                };
                                                [&((ws.height as i32).saturating_sub(info_height + 140)).to_string(), "px"].concat()
                                            })))
                                            .style("overflow-y","auto")
                                            .children(info.dates.clone().into_iter().map(|visit_date| {
                                                html!("button", {
                                                    .attr("type", "button")
                                                    .class(class::BTN_T_W100)
                                                    .class_signal("btn-primary", page.current_date.signal_cloned().map(clone!(visit_date => move |opt| opt.as_ref().map(|date| date.vn == visit_date.vn).unwrap_or_default())))
                                                    .class_signal("btn-secondary", page.current_date.signal_cloned().map(clone!(visit_date => move |opt| opt.as_ref().map(|date| date.vn != visit_date.vn).unwrap_or(true))))
                                                    .apply_if(visit_date.an.is_some(), |dom| dom.class("text-danger"))
                                                    .text(&date_and_time_th_opt_relative(&visit_date.vstdate, &visit_date.vsttime))
                                                    .event(clone!(page => move |_: events::Click| {
                                                        page.current_date.set(Some(visit_date.clone()));
                                                        page.reload_visit.set_neq(true);
                                                    }))
                                                })
                                            }))
                                        }))
                                        // right panel
                                        .child_signal(page.visit.signal_cloned().map(clone!(app, page => move |visit_opt| {
                                            visit_opt.as_ref().map(|visit| {
                                                html!("div", {
                                                    .style("width","calc(100vw - 238px)")
                                                    .style("column-width","480px")
                                                    .style("column-gap","8px")
                                                    .children([
                                                        Self::render_visit_hx(visit, page.clone()),
                                                        Self::render_visit_drugs(visit),
                                                        Self::render_drug_interaction(visit),
                                                        Self::render_labs(visit, page.clone(), app.clone()),
                                                        Self::render_visit_message(visit),
                                                        Self::render_visit_action(visit, page.clone(), app.clone()),
                                                        Self::render_visit_postal(visit, page.clone(), app.clone()),
                                                        Self::render_visit_telemed(visit, page.clone(), app.clone()),
                                                        Self::render_pharmacy_care(visit, page.clone(), app.clone()),
                                                    ])
                                                })
                                            })
                                        })))
                                    }),
                                ])
                            })
                        })
                    })))
                }),
                Self::render_modal(page.clone()),
            ])
        })
    }

    fn render_info_patient(info: &Rc<PrescriptionInfo>) -> Dom {
        let age_y = info.age_y.unwrap_or_default();
        let age_m = info.age_m.unwrap_or_default();
        let age_d = info.age_d.unwrap_or_default();
        html!("div", {
            .class(class::BOX_ROUND_T)
            .style("break-inside","avoid")
            .child(html!("div", {
                .class("d-flex")
                .children([
                    doms::patient_image(&info.hn, "90px"),
                    html!("div", {
                        .class("ms-1")
                        .children([
                            html!("div", {
                                .children([
                                    html!("span", {.class("fw-bold").text("HN : ")}),
                                    html!("span", {.text(&info.hn.clone().unwrap_or_default())}),
                                ])
                                .apply(|dom| {
                                    if let Some(cid) = info.cid.as_ref() {
                                        dom.children([
                                            html!("span", {.class("fw-bold").text(" CID : ")}),
                                            html!("span", {.text(cid)}),
                                        ])
                                    } else {
                                        dom
                                    }
                                })
                            }),
                            html!("div", {
                                .class("text-truncate")
                                .text(&info.fullname.clone().unwrap_or_default())
                            }),
                            html!("div", {
                                .child(html!("span", {
                                    .class("me-1")
                                    .text(&info.sex_name.clone().unwrap_or_default())
                                }))
                                .apply_if(age_y > 0, |dom| {
                                    dom.text(&[&age_y.to_string(), " ปี"].concat())
                                })
                                .apply_if(age_y == 0 && age_m > 0, |dom| {
                                    dom.text(&[&age_m.to_string(), " เดือน ", &age_d.to_string(), " วัน"].concat())
                                })
                                .apply_if(age_y == 0 && age_m == 0, |dom| {
                                    dom.text(&[&age_d.to_string(), " วัน"].concat())
                                })
                            }),
                            html!("div", {
                                .children([
                                    html!("span", {.class("fw-bold").text("ที่อยู่ : ")}),
                                    html!("span", {.text(&info.homeaddr.clone().unwrap_or(String::from("ไม่ระบุ")))}),
                                ])
                                .apply(|dom| {
                                    if let Some(hometel) = &info.hometel {
                                        dom.children([
                                            html!("span", {.class("fw-bold").text(" โทร : ")}),
                                            html!("span", {.text(&hometel)}),
                                        ])
                                    } else {
                                        dom
                                    }
                                })
                            }),
                        ])
                    }),
                ])
            }))
        })
    }

    fn render_info_allergy(info: &Rc<PrescriptionInfo>) -> Dom {
        html!("div", {
            .class(class::BOX_ROUND_T)
            .style("break-inside","avoid")
            .children([
                html!("div", {.class("fw-bold").text("ประวัติการแพ้ยา")}),
                html!("ul", {
                    .class(class::BORDER_T2_Y)
                    .children(info.drug_allergies.iter().map(|allergy| {
                        html!("li", {
                            .class(class::BOLD_RED_L)
                            .text(allergy)
                        })
                    }))
                    .apply_if(info.drug_allergies.is_empty(), |dom| {
                        dom.text("ไม่มีประวัติการแพ้ยา")
                    })
                }),
            ])
        })
    }

    fn render_info_note(info: &Rc<PrescriptionInfo>) -> Dom {
        html!("div", {
            .class(class::BOX_ROUND_T)
            .style("break-inside","avoid")
            .children([
                html!("div", {.class("fw-bold").text("Note")}),
                html!("ul", {
                    .class(class::BORDER_T2_Y)
                    .children(info.notes.iter().map(|note| {
                        html!("li", {
                            .text(&note.string())
                        })
                    }))
                    .apply_if(info.notes.is_empty(), |dom| {
                        dom.text("ไม่มี note")
                    })
                }),
            ])
        })
    }

    fn render_visit_hx(visit: &Rc<PrescriptionVn>, page: Rc<Self>) -> Dom {
        html!("div", {
            .class(class::BOX_ROUND_T)
            .style("break-inside","avoid")
            .children([
                html!("div", {
                    .children([
                        html!("span", {.class("fw-bold").text("แพทย์ผู้ตรวจ : ")}),
                        html!("span", {.text(&visit.doctor_name.clone().unwrap_or_default())}),
                    ])
                }),
                html!("div", {
                    .children([
                        html!("span", {.class("fw-bold").text("สิทธิการรักษา : ")}),
                        html!("span", {.text(&visit.pttype_name.clone().unwrap_or_default())}),
                    ])
                }),
                html!("div", {
                    .children([
                        html!("span", {.class("fw-bold").text("BW : ")}),
                        html!("span", {.text(&visit.bw.map(|f| f.to_string()).unwrap_or(String::from("ไม่ระบุ"))).text(" Kg.")}),
                        html!("span", {.class("fw-bold").text(" HT : ")}),
                        html!("span", {.text(&visit.height.map(|f| f.to_string()).unwrap_or(String::from("ไม่ระบุ"))).text(" Cm.")}),
                        html!("span", {.class("fw-bold").text(" BMI : ")}),
                        html!("span", {.text(&visit.bmi.map(|f| f64_rescale(f, 2).to_string()).unwrap_or(String::from("ไม่ระบุ")))}),
                    ])
                }),
                html!("div", {
                    .children([
                        html!("span", {.class("fw-bold").text("BT : ")}),
                        html!("span", {.text(&visit.temperature.map(|f| f.to_string()).unwrap_or(String::from("ไม่ระบุ"))).text("\u{00b0}C")}),
                        html!("span", {.class("fw-bold").text(" BP : ")}),
                        html!("span", {.text(&[visit.bps.map(|f| f.to_string()).unwrap_or(String::from("-")), visit.bpd.map(|f| f.to_string()).unwrap_or(String::from("-"))].join("/")).text(" mmHg.")}),
                    ])
                    .apply(|dom| {
                        if let Some(fbs) = opt_zero_none(visit.fbs) {
                            dom.children([
                                html!("span", {.class("fw-bold").text(" FBS : ")}),
                                html!("span", {.text(&fbs.to_string()).text(" mg/dl")}),
                            ])
                        } else {
                            dom
                        }
                    })
                }),
                html!("div", {
                    .children([
                        html!("span", {.class("fw-bold").text("CC : ")}),
                        html!("span", {.style("white-space","pre-wrap").text(&visit.cc.clone().unwrap_or_default())}),
                    ])
                }),
                html!("div", {
                    .children([
                        html!("span", {.class("fw-bold").text("HPI : ")}),
                        html!("span", {.style("white-space","pre-wrap").text(&visit.hpi.clone().unwrap_or_default())}),
                    ])
                }),
                html!("div", {
                    .children([
                        html!("span", {.class("fw-bold").text("PE : ")}),
                        html!("span", {.style("white-space","pre-wrap").text(&visit.pe.clone().unwrap_or_default())}),
                    ])
                }),
                html!("div", {
                    .children([
                        html!("span", {.class("fw-bold").text("Diagnosis :")}),
                        html!("ul", {
                            .children(visit.diag.as_ref().map(|concat| {
                                concat.split("\n").map(|dx| {
                                    html!("li", {.text(dx)})
                                }).collect::<Vec<Dom>>()
                            }).unwrap_or_default())
                        })
                    ])
                }),
                html!("div", {
                    .class("col")
                    .child(html!("div", {
                        .children([
                            html!("div", {.class("fw-bold").text("นัดหมาย :")}),
                            html!("ul", {
                                .children(visit.next_app.iter().map(clone!(page => move |next_app| {
                                    let appointment_date = next_app.nextdate.and_then(|nextdate| {
                                        page.info.lock_ref().as_ref().and_then(|info| {
                                            info.dates.iter().find(|date| date.vstdate.map(|d| d == nextdate).unwrap_or_default()).cloned()
                                        })
                                    });
                                    html!("li", {
                                        .text(&next_app.string())
                                        .apply_if(appointment_date.is_some(), |dom| { dom
                                            .style("cursor","pointer")
                                            .child(html!("i", {.class(class::FA_REPLY).class("ms-1")}))
                                            .event(clone!(page => move |_:events::Click| {
                                                page.current_date.set(appointment_date.clone());
                                                page.reload_visit.set_neq(true);
                                            }))
                                        })
                                    })
                                })))
                            }),
                        ])
                    }))
                }),
            ])
        })
    }

    fn render_visit_drugs(visit: &Rc<PrescriptionVn>) -> Dom {
        html!("div", {
            .class(class::BOX_ROUND_T)
            .style("break-inside","avoid")
            .children([
                html!("div", {.class(class::BOLD_T2).text("รายการยา")}),
                html!("div", {
                    // .class("overflow-auto")
                    // .style("height", "40vh")
                    // .style("overflow-y", "auto")
                    .child(html!("table", {
                        .class(class::TABLE_SM_STRIP)
                        .attr("data-bs-toggle","modal")
                        .attr("data-bs-target","#infoMedicineLastDrugModal")
                        .style("cursor","pointer")
                        .children([
                            html!("thead", {
                                .child(html!("tr", {
                                    .children([
                                        html!("th", {.attr("scope", "col").text("#")}),
                                        html!("th", {.attr("scope", "col").text("รายการ")}),
                                        html!("th", {.attr("scope", "col").text("จำนวน")}),
                                        html!("th", {.attr("scope", "col").text("วิธีใช้")}),
                                    ])
                                }))
                            }),
                            html!("tbody", {
                                //.attr("id", "medicine")
                                .children(visit.medicines.iter().enumerate().map(|(i, med)| {
                                    let last_opt = LastMedicine::new(&med.last_prescription);
                                    let is_same = last_opt.as_ref().map(|last| {
                                        last.icode == med.icode && last.strength == med.strength && last.shortlist == med.shortlist
                                    }).unwrap_or(true);

                                    html!("tr", {
                                        .apply_if(last_opt.is_none() || !is_same, |dom| dom.class("table-info"))
                                        .children([
                                            html!("td", {.text(&(i+1).to_string())}),
                                            html!("td", {
                                                // .class("text-end")
                                                .child(html!("span",{.text(&med.name_drugitems.clone().unwrap_or_default())}))
                                            }),
                                            html!("td", {.text(&med.qty.map(|i| i.to_string()).unwrap_or_default())}),
                                            html!("td", {.text(&med.shortlist.as_ref().map(|s| sanity_dot_space(s)).unwrap_or_default())}),
                                        ])
                                    })
                                }))
                            }),
                        ])
                    }))
                })
            ])
        })
    }

    fn render_drug_interaction(visit: &Rc<PrescriptionVn>) -> Dom {
        html!("div", {
            .class(class::BOX_ROUND_T)
            .style("break-inside","avoid")
            .children([
                html!("div", {.class(class::BOLD_T2).text("Drug Interaction")}),
                html!("div", {
                    // .class(class::OVFA_CYANS)
                    .child(html!("table", {
                        .class(class::TABLE_SM_STRIP)
                        .children([
                            html!("thead", {
                                .child(html!("tr", {
                                    .children([
                                        html!("th", {.attr("scope", "col").text("#")}),
                                        html!("th", {.attr("scope", "col").text("ชื่อยา 1")}),
                                        html!("th", {.attr("scope", "col").text("ชื่อยา 2")}),
                                        html!("th", {.attr("scope", "col").text("Level")}),
                                        html!("th", {.attr("scope", "col").text("หมายเหตุ")}),
                                    ])
                                }))
                            }),
                            html!("tbody", {
                                //.attr("id", "drug_interaction")
                                .class("table-warning")
                                .children(visit.drug_interactions.iter().enumerate().map(|(i,di)| {
                                    html!("tr", {.children([
                                        html!("td", {.class("text-end").text(&(i+1).to_string())}),
                                        html!("td", {.text(&di.drugname1.clone().unwrap_or_default())}),
                                        html!("td", {.text(&di.drugname2.clone().unwrap_or_default())}),
                                        html!("td", {.class("text-center").text(&di.severity.map(|i| i.to_string()).unwrap_or_default())}),
                                        html!("td", {.style("white-space","pre-wrap").text(&di.note.clone().unwrap_or_default())}),
                                    ])})
                                }))
                            }),
                        ])
                    }))
                }),
            ])
        })
    }

    fn render_labs(visit: &Rc<PrescriptionVn>, page: Rc<Self>, app: Rc<App>) -> Dom {
        html!("div", {
            .class(class::BOX_ROUND_T)
            .style("break-inside","avoid")
            .children([
                html!("ul", {
                    .class(class::NAV_PILLS_T)
                    //.attr("id", "pills-tab")
                    .attr("role", "tablist")
                    .children([
                        html!("li", {
                            .class("nav-item")
                            .child(html!("a", {
                                .class(class::NAV_LINK_ACTIVE)
                                .attr("id", "pills-home-tab")
                                .attr("data-bs-toggle", "pill")
                                .attr("href", "#pills-home")
                                .attr("role", "tab")
                                .attr("aria-controls", "pills-home")
                                .attr("aria-selected", "true")
                                .text("LAB ล่าสุด")
                            }))
                        }),
                        html!("li", {
                            .class("nav-item")
                            .child(html!("a", {
                                .class("nav-link")
                                .attr("id", "pills-profile-tab")
                                .attr("data-bs-toggle", "pill")
                                .attr("href", "#pills-profile")
                                .attr("role", "tab")
                                .attr("aria-controls", "pills-profile")
                                .attr("aria-selected", "false")
                                .text("LAB ของ Visit")
                            }))
                        }),
                    ])
                }),
                html!("div", {
                    .class("tab-content")
                    //.attr("id", "pills-tabContent")
                    .children([
                        html!("div", {
                            .class(class::TAB_FADE_SHOW_ACTIVE)
                            .attr("id", "pills-home")
                            .attr("role", "tabpanel")
                            .attr("aria-labelledby", "pills-home-tab")
                            // .style("height", "calc(100vh - 590px)")"
                            // .style("overflow-y", "auto")
                            .child(html!("table", {
                                .class(class::TABLE_SM_STRIP)
                                .children([
                                    html!("thead", {
                                        .child(html!("tr", {
                                            .children([
                                                html!("th", {.class("text-center").attr("scope", "col").text("LAB")}),
                                                html!("th", {.class("text-center").attr("scope", "col").text("ผล")}),
                                                html!("th", {.class("text-center").attr("scope", "col").text("LAB")}),
                                                html!("th", {.class("text-center").attr("scope", "col").text("ผล")}),
                                            ])
                                        }))
                                    }),
                                    html!("tbody", {
                                        .children_signal_vec(page.info.signal_cloned().map(clone!(app, page => move |opt| opt.as_ref().map(|info| {
                                            info.last_labs.chunks(2).map(|tuple| {
                                                html!("tr", {
                                                    .children(tuple.iter().flat_map(|lab| {
                                                        let cmp = if let (Some(prev), Some(normal)) = (&lab.lab_order_result, &lab.lab_items_normal_value) {
                                                            LabCmp::new(prev, normal)
                                                        } else {
                                                            LabCmp::Normal
                                                        };
                                                        [
                                                            html!("td", {
                                                                .child(html!("span", {.class("fw-bold").text(&lab.lab_name)}))
                                                                .apply(|dom| {
                                                                    if let Some(lab_items_normal_value) = lab.lab_items_normal_value.as_ref() {
                                                                        dom.child(html!("span", {
                                                                            .class("small")
                                                                            .text(&[" (", lab_items_normal_value, ")"].concat())
                                                                        }))
                                                                    } else {
                                                                        dom
                                                                    }
                                                                })
                                                            }),
                                                            html!("td", {
                                                                .class("text-end")
                                                                .child(html!("span", {
                                                                    .class("fw-bold")
                                                                    .apply(|dom| if matches!(cmp, LabCmp::Normal) {
                                                                        dom.class("text-primary")
                                                                    } else {
                                                                        dom.class("text-danger")
                                                                    })
                                                                    .text(&lab.lab_order_result.clone().unwrap_or(String::from("-")))
                                                                }))
                                                                .apply_if(matches!(cmp, LabCmp::High), |dom| { dom
                                                                    .child(html!("span", {
                                                                        .class(class::BOLD_RED)
                                                                        .text(" H")
                                                                    }))
                                                                })
                                                                .apply_if(matches!(cmp, LabCmp::Low), |dom| { dom
                                                                    .child(html!("span", {
                                                                        .class(class::BOLD_RED)
                                                                        .text(" L")
                                                                    }))
                                                                })
                                                                .child(html!("span",{.class("small").text(&lab.order_date.map(|date| [" (",&date_th_relative(&date),")"].concat()).unwrap_or_default())}))
                                                                .apply(|dom| {
                                                                    if app.endpoint_is_allow(&Method::GET, &EndPoint::LabItem, false)
                                                                        && lab.lab_order_result.is_some()
                                                                        && lab.lab_name.as_str() != "CrCl"
                                                                    {
                                                                        dom.attr("data-bs-toggle", "modal")
                                                                        .attr("data-bs-target", "#lab-history-modal-prescription-screen")
                                                                        .style("cursor","pointer")
                                                                        .event(clone!(page, lab, info => move |_:events::Click| {
                                                                            if let (Some(hn), Some(lab_items_code)) = (&info.hn, opt_zero_none(lab.lab_items_code)) {
                                                                                let lab_history_modal = LabHistory::new(
                                                                                    Mutable::new(hn.to_owned()),
                                                                                    lab_items_code,
                                                                                    &lab.lab_items_name_ref,
                                                                                    &lab.lab_items_unit,
                                                                                    &lab.lab_order_number,
                                                                                );
                                                                                page.lab_history_modal.set(Some(lab_history_modal));
                                                                            }
                                                                        }))
                                                                    } else {
                                                                        dom.style("cursor","default")
                                                                    }
                                                                })
                                                            }),
                                                        ]
                                                    }))
                                                })
                                            }).collect::<Vec<Dom>>()
                                        }).unwrap_or_default())).to_signal_vec())
                                    }),
                                ])
                            }))
                        }),
                        html!("div", {
                            .class(class::TAB_FADE)
                            .attr("id", "pills-profile")
                            .attr("role", "tabpanel")
                            .attr("aria-labelledby", "pills-profile-tab")
                            // .style("height", "calc(100vh - 590px)")
                            // .style("overflow-y", "auto")
                            .child(html!("table", {
                                .class(class::TABLE_SM_STRIP)
                                .children([
                                    html!("thead", {
                                        .child(html!("tr", {
                                            .children([
                                                html!("th", {.class("text-center").attr("scope", "col").text("LAB")}),
                                                html!("th", {.class("text-center").attr("scope", "col").text("ผล")}),
                                                html!("th", {.class("text-center").attr("scope", "col").text("LAB")}),
                                                html!("th", {.class("text-center").attr("scope", "col").text("ผล")}),
                                            ])
                                        }))
                                    }),
                                    html!("tbody", {
                                        .children(visit.labs.chunks(2).map(|tuple| {
                                            html!("tr", {
                                                .children(tuple.iter().flat_map(|lab| {
                                                    let cmp = if let (Some(prev), Some(normal)) = (&lab.lab_order_result, &lab.lab_items_normal_value) {
                                                        LabCmp::new(prev, normal)
                                                    } else {
                                                        LabCmp::Normal
                                                    };
                                                    [
                                                        html!("td", {
                                                            .child(html!("span", {.class("fw-bold").text(&lab.lab_name)}))
                                                            .apply(|dom| {
                                                                if let Some(lab_items_normal_value) = lab.lab_items_normal_value.as_ref() {
                                                                    dom.child(html!("span", {
                                                                        .class("small")
                                                                        .text(&[" (", lab_items_normal_value, ")"].concat())
                                                                    }))
                                                                } else {
                                                                    dom
                                                                }
                                                            })
                                                        }),
                                                        html!("td", {
                                                            .class("text-end")
                                                            .child(html!("span", {
                                                                .class("fw-bold")
                                                                .apply(|dom| if matches!(cmp, LabCmp::Normal) {
                                                                    dom.class("text-primary")
                                                                } else {
                                                                    dom.class("text-danger")
                                                                })
                                                                .text(&lab.lab_order_result.clone().unwrap_or(String::from("-")))
                                                            }))
                                                            .apply_if(matches!(cmp, LabCmp::High), |dom| { dom
                                                                .child(html!("span", {
                                                                    .class(class::BOLD_RED)
                                                                    .text(" H")
                                                                }))
                                                            })
                                                            .apply_if(matches!(cmp, LabCmp::Low), |dom| { dom
                                                                .child(html!("span", {
                                                                    .class(class::BOLD_RED)
                                                                    .text(" L")
                                                                }))
                                                            })
                                                            .apply(|dom| {
                                                                if lab.lab_order_result.is_some() && lab.lab_name.as_str() != "CrCl" { dom
                                                                    .attr("data-bs-toggle", "modal")
                                                                    .attr("data-bs-target", "#lab-history-modal-prescription-screen")
                                                                    .style("cursor","pointer")
                                                                    .event(clone!(page, lab, visit => move |_:events::Click| {
                                                                        if let (Some(hn), Some(lab_items_code)) = (&visit.hn, opt_zero_none(lab.lab_items_code)) {
                                                                            let lab_history_modal = LabHistory::new(
                                                                                Mutable::new(hn.to_owned()),
                                                                                lab_items_code,
                                                                                &lab.lab_items_name_ref,
                                                                                &lab.lab_items_unit,
                                                                                &lab.lab_order_number,
                                                                            );
                                                                            page.lab_history_modal.set(Some(lab_history_modal));
                                                                        }
                                                                    }))
                                                                } else {
                                                                    dom.style("cursor","default")
                                                                }
                                                            })
                                                        }),
                                                    ]
                                                }))
                                            })
                                        }))
                                    }),
                                ])
                            }))
                        }),
                        html!("div", {
                            .class("modal")
                            .attr("id", "lab-history-modal-prescription-screen")
                            .attr("role", "dialog")
                            .attr("tabindex", "-1")
                            .child_signal(page.lab_history_modal.signal_cloned().map(clone!(app => move |opt| {
                                opt.as_ref().map(clone!(app => move |modal| {
                                    LabHistory::render(modal.clone(), app, None)
                                })).or(Some(blank_modal()))
                            })))
                        }),
                    ])
                }),
            ])
        })
    }

    fn render_visit_message(visit: &Rc<PrescriptionVn>) -> Dom {
        html!("div", {
            .class(class::BOX_ROUND_T)
            .style("break-inside","avoid")
            .children([
                html!("div", {.class(class::BOLD_T2).text("ข้อความเตือนการใช้ยา")}),
                html!("ul", {
                    .class(class::BORDER_T2_Y)
                    .apply(|dom| {
                        if visit.mess_vn.is_empty() {
                            dom.text("ไม่มีข้อความเตือนการใช้ยา")
                        } else {
                            dom.class(class::TXT_WHITE_RED)
                        }
                    })
                    .children(visit.mess_vn.iter().map(|mess| {
                        html!("li", {.text(mess)})
                    }))
                }),
            ])
        })
    }

    fn render_visit_action(visit: &Rc<PrescriptionVn>, page: Rc<Self>, app: Rc<App>) -> Dom {
        let total_minutes_opt = if let (Some(visit_datetime), Some(pharmacist_done_time)) = (datetime_from_opt(visit.vstdate, visit.vsttime), visit.pharmacist_done_time) {
            Some((pharmacist_done_time - visit_datetime).whole_minutes())
        } else {
            None
        };
        let allow_patch = app.endpoint_is_allow(&Method::PATCH, &EndPoint::PrescrptionScreen, false);

        html!("div", {
            .class(class::BOX_ROUND_T)
            .style("break-inside","avoid")
            .children([
                html!("div", {
                    .class(class::BOLD_T2)
                    .text("บันทึกเวลาดำเนินการ")
                    .apply(|dom| {
                        if let Some(total_minutes) = total_minutes_opt {
                            dom.text(" (")
                            .text(&total_minutes.to_string())
                            .text(" นาที)")
                        } else {
                            dom
                        }
                    })
                }),
                html!("ul", {
                    .class(class::BORDER_T2_Y)
                    // ACCEPT
                    .apply(|dom| {
                        if visit.vn.is_some() && app.endpoint_is_allow(&Method::POST, &EndPoint::PrescrptionScreen, false) {
                            let edit_btn_opt = if visit.pharmacist_check_time.is_none() {
                                Some(html!("button" => HtmlButtonElement, {
                                    .apply(|d| {
                                        if visit.pharmacist_accept_time.is_some() {
                                            d.class(class::BTN_R_REDO)
                                        } else {
                                            d.class(class::BTN_R_BLUE)
                                        }
                                    })
                                    .text("รับรายการ")
                                    .apply(mixins::click_with_loader_checked(clone!(app, page, visit => move || {
                                        Self::insert_pharmacist_accept(visit.clone(), page.clone(), app.clone());
                                    }), app.state()))
                                }))
                            } else {
                                None
                            };
                            if let (Some(pharmacist_accept_name), Some(pharmacist_accept_time)) = (&visit.pharmacist_accept_name, visit.pharmacist_accept_time) {
                                let diff_minutes_opt = if let Some(visit_datetime) = datetime_from_opt(visit.vstdate, visit.vsttime) {
                                    Some((pharmacist_accept_time - visit_datetime).whole_minutes())
                                } else {
                                    None
                                };
                                dom.child(html!("li", {
                                    .class("mb-1")
                                    .child(html!("span", {
                                        .child(html!("b", {.text("รับรายการ : ")}))
                                        .text(&datetime_th_relative(&pharmacist_accept_time))
                                        .apply(|d| {
                                            if let Some(diff_minutes) = diff_minutes_opt {
                                                d.text(" (")
                                                .text(&diff_minutes.to_string())
                                                .text(" นาที)")
                                            } else {
                                                d
                                            }
                                        })
                                        .child(html!("br"))
                                        .text("โดย ")
                                        .text(pharmacist_accept_name)
                                    }))
                                    .apply(|d| {
                                        if let Some(edit_btn) = edit_btn_opt {
                                            d.child(edit_btn)
                                        } else {
                                            d
                                        }
                                    })
                                }))
                            } else {
                                dom.child(html!("li", {
                                    .class("mb-1")
                                    .apply(|d| {
                                        if let Some(edit_btn) = edit_btn_opt {
                                            d.child(edit_btn)
                                        } else {
                                            d
                                        }
                                    })
                                }))
                            }
                        } else {
                            dom
                        }
                    })
                    // CHECK
                    .apply(|dom| {
                        if allow_patch && visit.vn.is_some() && visit.pharmacist_accept_time.is_some() {
                            let edit_btn_opt = if visit.pharmacist_done_time.is_none() {
                                Some(html!("button" => HtmlButtonElement, {
                                    .apply(|d| {
                                        if visit.pharmacist_check_time.is_some() {
                                            d.class(class::BTN_R_REDO)
                                        } else {
                                            d.class(class::BTN_R_BLUE)
                                        }
                                    })
                                    .text("ตรวจสอบ")
                                    .apply(mixins::click_with_loader_checked(clone!(app, page, visit => move || {
                                        Self::update_pharmacist_check(visit.clone(), page.clone(), app.clone());
                                    }), app.state()))
                                }))
                            } else {
                                None
                            };
                            if let (Some(pharmacist_check_name), Some(pharmacist_check_time)) = (&visit.pharmacist_check_name, visit.pharmacist_check_time) {
                                let diff_minutes_opt = if let Some(pharmacist_accept_time) = visit.pharmacist_accept_time {
                                    Some((pharmacist_check_time - pharmacist_accept_time).whole_minutes())
                                } else {
                                    None
                                };
                                dom.child(html!("li", {
                                    .class("mb-1")
                                    .child(html!("span", {
                                        .child(html!("b", {.text("ตรวจสอบ : ")}))
                                        .text(&datetime_th_relative(&pharmacist_check_time))
                                        .apply(|d| {
                                            if let Some(diff_minutes) = diff_minutes_opt {
                                                d.text(" (")
                                                .text(&diff_minutes.to_string())
                                                .text(" นาที)")
                                            } else {
                                                d
                                            }
                                        })
                                        .child(html!("br"))
                                        .text("โดย ")
                                        .text(pharmacist_check_name)
                                    }))
                                    .apply(|d| {
                                        if let Some(edit_btn) = edit_btn_opt {
                                            d.child(edit_btn)
                                        } else {
                                            d
                                        }
                                    })
                                }))
                            } else {
                                dom.child(html!("li", {
                                    .class("mb-1")
                                    .apply(|d| {
                                        if let Some(edit_btn) = edit_btn_opt {
                                            d.child(edit_btn)
                                        } else {
                                            d
                                        }
                                    })
                                }))
                            }
                        } else {
                            dom
                        }
                    })
                    // DONE
                    .apply(|dom| {
                        if allow_patch && visit.vn.is_some() && visit.pharmacist_accept_time.is_some() && visit.pharmacist_check_time.is_some(){
                            let edit_btn = html!("button" => HtmlButtonElement, {
                                .apply(|d| {
                                        if visit.pharmacist_done_time.is_some() {
                                            d.class(class::BTN_R_REDO)
                                        } else {
                                            d.class(class::BTN_R_BLUE)
                                        }
                                    })
                                .text("จ่ายยา")
                                .apply(mixins::click_with_loader_checked(clone!(app, page, visit => move || {
                                    Self::update_pharmacist_done(visit.clone(), page.clone(), app.clone());
                                }), app.state()))
                            });
                            if let (Some(pharmacist_done_name), Some(pharmacist_done_time)) = (&visit.pharmacist_done_name, visit.pharmacist_done_time) {
                                let diff_minutes_opt = if let Some(pharmacist_check_time) = visit.pharmacist_check_time {
                                    Some((pharmacist_done_time - pharmacist_check_time).whole_minutes())
                                } else {
                                    None
                                };
                                dom.child(html!("li", {
                                    .class("mb-1")
                                    .children([
                                        html!("span", {
                                            .child(html!("b", {.text("จ่ายยา : ")}))
                                            .text(&datetime_th_relative(&pharmacist_done_time))
                                            .apply(|d| {
                                                if let Some(diff_minutes) = diff_minutes_opt {
                                                    d.text(" (")
                                                    .text(&diff_minutes.to_string())
                                                    .text(" นาที)")
                                                } else {
                                                    d
                                                }
                                            })
                                            .child(html!("br"))
                                            .text("โดย ")
                                            .text(pharmacist_done_name)
                                        }),
                                        edit_btn,
                                    ])
                                }))
                            } else {
                                dom.child(html!("li", {
                                    .class("mb-1")
                                    .child(edit_btn)
                                }))
                            }
                        } else {
                            dom
                        }
                    })
                }),
            ])
        })
    }

    fn render_visit_postal(visit: &Rc<PrescriptionVn>, page: Rc<Self>, app: Rc<App>) -> Dom {
        html!("div", {
            .class(class::BOX_ROUND_T)
            .style("break-inside","avoid")
            .children([
                html!("div", {
                    .class(class::BOLD_T2)
                    .text("บันทึกการส่งยาทางไปรษณีย์")
                }),
                html!("ul", {
                    .class(class::BORDER_T2_Y)
                    .apply(|dom| {
                        if visit.vn.is_some() && visit.pharmacist_check_time.is_some() {
                            let is_ok = visit.postal_status.as_ref().map(|s| s == "Y");
                            let edit_btn_opt = app.endpoint_is_allow(&Method::PATCH, &EndPoint::PrescrptionScreen, false).then(|| {
                                html!("button" => HtmlButtonElement, {
                                    .apply(|d| {
                                        match is_ok {
                                            Some(true) => {
                                                d.class(class::BTN_R_REDO)
                                                .text("ยกเลิก")
                                            }
                                            Some(false) => {
                                                d.class(class::BTN_R_REDO)
                                                .text("ส่งใหม่")
                                            }
                                            None => {
                                                d.class(class::BTN_R_BLUE)
                                                .text("ส่งยาทางไปรษณีย์")
                                            }
                                        }
                                    })
                                    .apply(mixins::click_with_loader_checked(clone!(app, page, visit => move || {
                                        Self::update_postal(!is_ok.unwrap_or_default(), visit.clone(), page.clone(), app.clone());
                                    }), app.state()))
                                })
                            });
                            if let (Some(postal_doctor_name), Some(postal_time)) = (&visit.postal_doctor_name, visit.postal_time) {
                                dom.children([
                                    html!("li", {
                                        .class("mb-1")
                                        .child(html!("span", {
                                            .child(html!("b", {.text(if is_ok.unwrap_or_default() {"จัดส่ง : "} else {"ยกเลิกการส่ง : "})}))
                                            .text(&datetime_th_relative(&postal_time))
                                            .text(" โดย ")
                                            .text(postal_doctor_name)
                                        }))
                                    }),
                                    html!("div", {
                                        .class("text-end")
                                        .apply(|d| {
                                            if let Some(edit_btn) = edit_btn_opt {
                                                d.child(edit_btn)
                                            } else {
                                                d
                                            }
                                        })
                                    }),
                                ])
                            } else {
                                dom.child(html!("div", {
                                    .class("text-end")
                                    .apply(|d| {
                                        if let Some(edit_btn) = edit_btn_opt {
                                            d.child(edit_btn)
                                        } else {
                                            d
                                        }
                                    })
                                }))
                            }
                        } else {
                            dom.child(html!("div", {
                                .text("รอรับรายการ")
                            }))
                        }
                    })
                }),
            ])
        })
    }

    fn render_visit_telemed(visit: &Rc<PrescriptionVn>, page: Rc<Self>, app: Rc<App>) -> Dom {
        html!("div", {
            .class(class::BOX_ROUND_T)
            .style("break-inside","avoid")
            .children([
                html!("div", {
                    .class(class::BOLD_T2)
                    .text("บันทึกการให้บริการ Telemed")
                }),
                html!("ul", {
                    .class(class::BORDER_T2_Y)
                    .apply(|dom| {
                        if visit.vn.is_some() {
                            dom.children([
                                html!("li", {
                                    .class(class::ROW)
                                    .children([
                                        html!("label", {
                                            .class(class::COL_SM3_R)
                                            .attr("for","telemed_add")
                                            .text("ยาใหม่")
                                        }),
                                        html!("div", {
                                            .class("col-sm-9")
                                            .child(html!("textarea" => HtmlTextAreaElement, {
                                                .class(class::FORM_CTRL_SM)
                                                .attr("id","telemed_add")
                                                .attr("rows","1")
                                                .apply(mixins::textarea_value_auto_expand(page.telemed_add.clone(), page.telemed_changed.clone()))
                                            }))
                                        }),
                                    ])
                                }),
                                html!("li", {
                                    .class(class::ROW)
                                    .children([
                                        html!("label", {
                                            .class(class::COL_SM3_R)
                                            .attr("for","telemed_dose_up")
                                            .text("เพิ่มขนาดยา")
                                        }),
                                        html!("div", {
                                            .class("col-sm-9")
                                            .child(html!("textarea" => HtmlTextAreaElement, {
                                                .class(class::FORM_CTRL_SM)
                                                .attr("id","telemed_dose_up")
                                                .attr("rows","1")
                                                .apply(mixins::textarea_value_auto_expand(page.telemed_dose_up.clone(), page.telemed_changed.clone()))
                                            }))
                                        }),
                                    ])
                                }),
                                html!("li", {
                                    .class(class::ROW)
                                    .children([
                                        html!("label", {
                                            .class(class::COL_SM3_R)
                                            .attr("for","telemed_dose_down")
                                            .text("ลดขนาดยา")
                                        }),
                                        html!("div", {
                                            .class("col-sm-9")
                                            .child(html!("textarea" => HtmlTextAreaElement, {
                                                .class(class::FORM_CTRL_SM)
                                                .attr("id","telemed_dose_down")
                                                .attr("rows","1")
                                                .apply(mixins::textarea_value_auto_expand(page.telemed_dose_down.clone(), page.telemed_changed.clone()))
                                            }))
                                        }),
                                    ])
                                }),
                                html!("li", {
                                    .class(class::ROW)
                                    .children([
                                        html!("label", {
                                            .class(class::COL_SM3_R)
                                            .attr("for","telemed_off")
                                            .text("หยุดยา")
                                        }),
                                        html!("div", {
                                            .class("col-sm-9")
                                            .child(html!("textarea" => HtmlTextAreaElement, {
                                                .class(class::FORM_CTRL_SM)
                                                .attr("id","telemed_off")
                                                .attr("rows","1")
                                                .apply(mixins::textarea_value_auto_expand(page.telemed_off.clone(), page.telemed_changed.clone()))
                                            }))
                                        }),
                                    ])
                                }),
                                html!("li", {
                                    .class(class::ROW)
                                    .children([
                                        html!("label", {
                                            .class(class::COL_SM3_R)
                                            .attr("for","telemed_other")
                                            .text("อื่นๆ")
                                        }),
                                        html!("div", {
                                            .class("col-sm-9")
                                            .child(html!("textarea" => HtmlTextAreaElement, {
                                                .class(class::FORM_CTRL_SM)
                                                .attr("id","telemed_other")
                                                .attr("rows","1")
                                                .apply(mixins::textarea_value_auto_expand(page.telemed_other.clone(), page.telemed_changed.clone()))
                                            }))
                                        }),
                                    ])
                                }),
                            ])
                            .apply(|dom| {
                                let edit_btn_opt = if app.endpoint_is_allow(&Method::PATCH, &EndPoint::PrescrptionScreen, false) {
                                    Some(html!("button" => HtmlButtonElement, {
                                        .class(class::BTN_R_BLUE)
                                        .text("บันทึก")
                                        .apply(mixins::click_with_loader_checked_or_true_disable_signal(clone!(app, page, visit => move || {
                                            Self::update_telemed(visit.clone(), page.clone(), app.clone());
                                        }), not(page.telemed_changed.signal()), app.state()))
                                    }))
                                } else {
                                    None
                                };
                                if let (Some(telemed_doctor_name), Some(telemed_time)) = (&visit.telemed_doctor_name, visit.telemed_time) {
                                    dom.children([
                                        html!("li", {
                                            .class("my-2")
                                            .child(html!("span", {
                                                .child(html!("b", {.text("บันทึก : ")}))
                                                .text(&datetime_th_relative(&telemed_time))
                                                .text(" โดย ")
                                                .text(telemed_doctor_name)
                                            }))
                                        }),
                                        html!("div", {
                                            .class(class::TXT_R_B2)
                                            .apply(|d| {
                                                if let Some(edit_btn) = edit_btn_opt {
                                                    d.child_signal(page.telemed_changed.signal().map(clone!(page, visit => move |changed| {
                                                        changed.then(|| {
                                                            html!("button" => HtmlButtonElement, {
                                                                .class(class::BTN_R_GRAY)
                                                                .text("ยกเลิก")
                                                                .event(clone!(page, visit => move |_:events::Click| {
                                                                    page.set_telemed(&visit);
                                                                }))
                                                            })
                                                        })
                                                    })))
                                                    .child(edit_btn)
                                                } else {
                                                    d
                                                }
                                            })
                                        }),
                                    ])
                                } else {
                                    dom.child(html!("div", {
                                        .class(class::TXT_R_B2)
                                        .apply(|d| {
                                            if let Some(edit_btn) = edit_btn_opt {
                                                d.child(edit_btn)
                                            } else {
                                                d
                                            }
                                        })
                                    }))
                                }
                            })
                        } else {
                            dom.child(html!("div", {
                                // vn never None
                                .text("ไม่มีข้อมูล")
                            }))
                        }
                    })
                }),
            ])
        })
    }

    fn render_pharmacy_care(visit: &Rc<PrescriptionVn>, page: Rc<Self>, app: Rc<App>) -> Dom {
        html!("div", {
            .class(class::BOX_ROUND_T)
            .style("break-inside","avoid")
            .children([
                html!("div", {
                    .class(class::BOLD_T2)
                    .text("Pharmacy Care")
                }),
                html!("div", {
                    .class("col")
                    .child(html!("textarea" => HtmlTextAreaElement, {
                        .class(class::FORM_CTRL_SM)
                        .attr("rows","1")
                        .apply(mixins::textarea_value_auto_expand(page.pharmacy_care.clone(), page.pharmacy_care_changed.clone()))
                    }))
                }),
            ])
            .apply(|dom| {
                let edit_btn_opt = if app.endpoint_is_allow(&Method::PATCH, &EndPoint::PrescrptionScreen, false) {
                    Some(html!("button" => HtmlButtonElement, {
                        .class(class::BTN_R_BLUE)
                        .text("บันทึก")
                        .apply(mixins::click_with_loader_checked_or_true_disable_signal(clone!(app, page, visit => move || {
                            Self::update_pharmacy_care(visit.clone(), page.clone(), app.clone());
                        }), not(page.pharmacy_care_changed.signal()), app.state()))
                    }))
                } else {
                    None
                };
                if let (Some(pharmacy_care_doctor_name), Some(pharmacy_care_time)) = (&visit.pharmacy_care_doctor_name, visit.pharmacy_care_time) {
                    dom.children([
                        html!("ul", {
                            .child(html!("li", {
                                .class("my-2")
                                .child(html!("span", {
                                    .child(html!("b", {.text("บันทึก : ")}))
                                    .text(&datetime_th_relative(&pharmacy_care_time))
                                    .text(" โดย ")
                                    .text(pharmacy_care_doctor_name)
                                }))
                            }))
                        }),
                        html!("div", {
                            .class(class::TXT_R_B2)
                            .apply(|d| {
                                if let Some(edit_btn) = edit_btn_opt {
                                    d.child_signal(page.pharmacy_care_changed.signal().map(clone!(page, visit => move |changed| {
                                        changed.then(|| {
                                            html!("button" => HtmlButtonElement, {
                                                .class(class::BTN_R_GRAY)
                                                .text("ยกเลิก")
                                                .event(clone!(page, visit => move |_:events::Click| {
                                                    page.pharmacy_care.set(visit.pharmacy_care.clone().unwrap_or_default());
                                                    page.pharmacy_care_changed.set_neq(false);
                                                }))
                                            })
                                        })
                                    })))
                                    .child(edit_btn)
                                } else {
                                    d
                                }
                            })
                        }),
                    ])
                } else {
                    dom.child(html!("div", {
                        .class(class::TXT_R_B2)
                        .apply(|d| {
                            if let Some(edit_btn) = edit_btn_opt {
                                d.child(edit_btn)
                            } else {
                                d
                            }
                        })
                    }))
                }
            })
        })
    }

    fn render_modal(page: Rc<Self>) -> Dom {
        html!("div", {
            .class("modal")
            .attr("id", "infoMedicineLastDrugModal")
            .attr("tabindex", "-1")
            .attr("aria-labelledby", "infoMedicineLastDrugModal")
            .child(html!("div", {
                .class(class::MODAL_DIALOG_FULL_C)
                .child(html!("div", {
                    .class(class::MODAL_CONTENT_X)
                    .children([
                        html!("div", {
                            .class("modal-header")
                            .children([
                                html!("h5", {
                                    .class("modal-title")
                                    //.attr("id", "exampleModalLabel")
                                    .text("ยา")
                                }),
                                doms::close_modal_x_btn(),
                            ])
                        }),
                        html!("div", {
                            .class("modal-body")
                            .child(html!("div", {
                                .class("col-12")
                                .children([
                                    html!("div", {
                                        .class("overflow-auto")
                                        .style("overflow-y","auto")
                                        .child(html!("table", {
                                            .class(class::TABLE_SM_STRIP)
                                            .children([
                                                html!("thead", {
                                                    .child(html!("tr", {
                                                        .children([
                                                            html!("th", {.class("text-nowrap").attr("scope", "col").text("#")}),
                                                            html!("th", {.class("text-nowrap").attr("scope", "col").text("ยาปัจจุบัน")}),
                                                            html!("th", {.class("text-nowrap").attr("scope", "col").text("จำนวน")}),
                                                            html!("th", {.class("text-nowrap").attr("scope", "col").text("ยาเดิม")}),
                                                            html!("th", {.class("text-nowrap").attr("scope", "col").text("จำนวนเดิม")}),
                                                            html!("th", {.class("text-nowrap").attr("scope", "col").text("วิธีใช้ปัจจุบัน")}),
                                                            html!("th", {.class("text-nowrap").attr("scope", "col").text("วิธีใช้เดิม")}),
                                                            html!("th", {.class("text-nowrap").attr("scope", "col").text("วันที่ ที่ได้รับครั้งก่อนหน้า")}),
                                                            html!("th", {.class("text-nowrap").attr("scope", "col").text("สรุป")}),
                                                        ])
                                                    }))
                                                }),
                                                html!("tbody", {
                                                    //.attr("id", "info_medicine_last_tb")
                                                    .children_signal_vec(page.visit.signal_cloned().map(|opt| opt.as_ref().map(|visit| {
                                                        visit.medicines.iter().enumerate().map(|(i, med)| {
                                                            let last_opt = LastMedicine::new(&med.last_prescription);
                                                            let is_icode_changed = last_opt.as_ref().map(|last| last.icode != med.icode).unwrap_or_default();
                                                            let is_qty_changed = last_opt.as_ref().map(|last| last.qty != med.qty).unwrap_or_default();
                                                            let is_strength_changed = last_opt.as_ref().map(|last| last.strength != med.strength).unwrap_or_default();
                                                            let is_shortlist_changed = last_opt.as_ref().map(|last| last.shortlist != med.shortlist).unwrap_or_default();

                                                            html!("tr", {
                                                                .attr("data-bs-toggle","modal")
                                                                .attr("data-bs-target","#infoMedicineLastDrugModal")
                                                                .apply_if(last_opt.is_none() || is_icode_changed || is_strength_changed || is_shortlist_changed, |dom| dom.class("table-warning"))
                                                                .children([
                                                                    html!("td", {.text(&(i+1).to_string())}),
                                                                    html!("td", {.child(html!("span",{.text(&med.name_drugitems.clone().unwrap_or_default())}))}),
                                                                    html!("td", {.text(&med.qty.map(|i| i.to_string()).unwrap_or_default())}),
                                                                    html!("td", {
                                                                        .apply_if(is_icode_changed, |dom| dom.text(&last_opt.as_ref().and_then(|last| last.name_drugitems.clone()).unwrap_or_default()))
                                                                    }),
                                                                    html!("td", {
                                                                        .apply_if(is_qty_changed, |dom| dom.text(&last_opt.as_ref().and_then(|last| last.qty.map(|i| i.to_string())).unwrap_or_default()))
                                                                    }),
                                                                    html!("td", {.text(&med.shortlist.as_ref().map(|s| sanity_dot_space(s)).unwrap_or_default())}),
                                                                    html!("td", {
                                                                        .apply_if(is_shortlist_changed, |dom| dom.text(&last_opt.as_ref().and_then(|last| last.shortlist.as_ref().map(|s| sanity_dot_space(s))).unwrap_or_default()))
                                                                    }),
                                                                    html!("td", {
                                                                        .apply_if(last_opt.is_some(), |dom| dom.text(&last_opt.as_ref().and_then(|last| last.rxdatetime.as_ref().map(|s| datetime_str_th_relative(s))).unwrap_or_default()))
                                                                    }),
                                                                    html!("td", {
                                                                        .apply_if(last_opt.is_none(), |dom| dom.child(html!("span", {.class(class::BADGE_GOLD_L).style("cursor","default").text("ยาใหม่")})))
                                                                        .children([
                                                                            html!("span", {
                                                                                .class(class::BADGE_L)
                                                                                .style("cursor","default")
                                                                                .class(if is_icode_changed {"text-bg-warning"} else {"text-bg-secondary"})
                                                                                .text(if is_icode_changed {"icode เปลี่ยน"} else {"icode เดิม"})
                                                                            }),
                                                                            html!("span", {
                                                                                .class(class::BADGE_L)
                                                                                .style("cursor","default")
                                                                                .class(if is_strength_changed {"text-bg-warning"} else {"text-bg-secondary"})
                                                                                .text(if is_strength_changed {"Strength เปลี่ยน"} else {"Strength เดิม"})
                                                                            }),
                                                                            html!("span", {
                                                                                .class(class::BADGE_L)
                                                                                .style("cursor","default")
                                                                                .class(if is_shortlist_changed {"text-bg-warning"} else {"text-bg-secondary"})
                                                                                .text(if is_shortlist_changed {"วิธีใช้เปลี่ยน"} else {"วิธีใช้เดิม"})
                                                                            }),
                                                                        ])
                                                                    })
                                                                ])
                                                            })
                                                        }).collect::<Vec<Dom>>()
                                                    }).unwrap_or_default()).to_signal_vec())
                                                }),
                                            ])
                                        }))
                                    }),
                                ])
                            }))
                        }),
                    ])
                }))
            }))
        })
    }
}
