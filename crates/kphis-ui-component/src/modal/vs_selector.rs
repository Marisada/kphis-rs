use dominator::{Dom, clone, events, html};
use futures_signals::{
    map_ref,
    signal::{Mutable, Signal, SignalExt, always},
    signal_vec::SignalVecExt,
};
use std::rc::Rc;
use web_sys::HtmlButtonElement;

use kphis_model::{
    app::VisitTypeId,
    endpoint::EndPoint,
    fetch::Method,
    opd_er::medical_history::{OpdErMedicalHistory, OpdErMedicalHistoryParams},
    patient_info::PatientInfo,
    vital_sign::{VitalSign, VitalSignParams},
};
use kphis_ui_app::App;
use kphis_ui_core::{class, doms, mixins};
use kphis_util::{
    datetime::{date_8601, date_th_opt, datetime_th, js_now, time_hm_opt},
    util::{set_day_last, set_days_next, zero_none},
};

use crate::vital_sign::row::full_text;

/// - GET `EndPoint::IpdVitalSign`
/// - GET `EndPoint::OpdErVitalSign`
/// - GET `EndPoint::OpdErMedicalHistory` (guarded, hide 'HOSxP' btn)
#[derive(Default)]
pub struct VsSelector {
    with_datetime: bool,
    patient: Mutable<Option<Rc<PatientInfo>>>,

    loaded: Mutable<bool>,
    changed: Mutable<bool>,

    start_vs_date: Mutable<String>,
    end_vs_date: Mutable<String>,
    vs_result: Mutable<Vec<Rc<VitalSign>>>,
    checked: Mutable<bool>,

    parent_result: Mutable<String>,
    parent_changed: Mutable<bool>,
}

impl VsSelector {
    pub fn new(with_datetime: bool, patient: Mutable<Option<Rc<PatientInfo>>>, parent_result: Mutable<String>, parent_changed: Mutable<bool>) -> Rc<Self> {
        let now = js_now().date();
        Rc::new(Self {
            with_datetime,
            patient,
            start_vs_date: Mutable::new(now.previous_day().unwrap_or(now).to_string()),
            end_vs_date: Mutable::new(now.to_string()),
            parent_result,
            parent_changed,
            ..Default::default()
        })
    }

    fn is_ipd(&self) -> impl Signal<Item = bool> + use<> {
        self.patient.signal_cloned().map(|opt| opt.as_ref().map(|pt| pt.is_ipd()).unwrap_or_default())
    }

    fn set_day_last(&self, days: u64, from_now: bool) {
        if let Some(patient) = self.patient.lock_ref().as_ref() {
            let last_date = if from_now { Some(js_now().date()) } else { patient.lastdate() };
            set_day_last(patient.regdate(), last_date, self.start_vs_date.clone(), self.end_vs_date.clone(), self.changed.clone(), days);
        }
    }

    fn set_days_next(&self, forward: bool) {
        set_days_next(self.start_vs_date.clone(), self.end_vs_date.clone(), self.changed.clone(), forward);
    }

    fn load(modal: Rc<Self>, app: Rc<App>) {
        let visit_type_opt = modal.patient.lock_ref().as_ref().map(|pt| pt.visit_type());
        if let Some(visit_type) = visit_type_opt {
            app.async_load(
                true,
                clone!(app, modal => async move {
                    let (params_opt, is_ipd) = match visit_type {
                        VisitTypeId::Ipd(an) | VisitTypeId::PreAdmit(an) => {
                            (Some(VitalSignParams {
                                an: Some(an),
                                start_date: date_8601(&modal.start_vs_date.lock_ref()),
                                end_date: date_8601(&modal.end_vs_date.lock_ref()),
                                ..Default::default()
                            }), true)

                        }
                        VisitTypeId::OpdEr(_, opd_er_order_master_id) => {
                            (Some(VitalSignParams {
                                opd_er_order_master_id: Some(opd_er_order_master_id),
                                start_date: date_8601(&modal.start_vs_date.lock_ref()),
                                end_date: date_8601(&modal.end_vs_date.lock_ref()),
                                ..Default::default()
                            }), false)
                        }
                        VisitTypeId::Visit(_) => {
                            (None, false)
                        }
                    };

                    if let Some(params) = params_opt {
                        // GET `EndPoint::IpdVitalSign`
                        // GET `EndPoint::OpdErVitalSign`
                        match VitalSign::call_api_get(is_ipd, &params, app.state()).await {
                            Ok(items) => {
                                modal.checked.set_neq(!items.is_empty());
                                modal.vs_result.set(items.into_iter().map(Rc::new).collect());
                            }
                            Err(e) => {
                                app.alert_app_error(&e).await;
                            }
                        }
                    }
                }),
            )
        }
    }

    fn get_vs_hosxp(modal: Rc<Self>, app: Rc<App>) {
        let vn_opt = modal.patient.lock_ref().as_ref().and_then(|pt| pt.vn());
        if let Some(vn) = vn_opt {
            if !vn.is_empty() {
                app.async_load(
                    true,
                    clone!(app, modal => async move {

                        let params = OpdErMedicalHistoryParams {
                            vn: Some(vn),
                            only_opdscreen: Some(true),
                            ..Default::default()
                        };
                        // GET `EndPoint::OpdErMedicalHistory`
                        match OpdErMedicalHistory::call_api_get(&params, app.state()).await {
                            Ok(response) => {
                                if let Some(opdscreen) = response.opdscreen {
                                    let vs_date = date_th_opt(&opdscreen.vstdate);
                                    let vs_time = time_hm_opt(&opdscreen.vsttime);

                                    let mut buf = Vec::new();
                                    if let Some(bt) = opdscreen.temperature.and_then(zero_none).map(|f| f.to_string()) {
                                        buf.push(["T: ", &bt, " °C"].concat());
                                    }
                                    if let Some(pr) = opdscreen.pulse.and_then(zero_none).map(|f| f.to_string()) {
                                        buf.push(["P: ", &pr, " /min"].concat());
                                    }
                                    if let Some(rr) = opdscreen.rr.and_then(zero_none).map(|f| f.to_string()) {
                                        buf.push(["R: ", &rr, " /min"].concat());
                                    }
                                    if let (Some(bps), Some(bpd)) = (opdscreen.bps.and_then(zero_none), opdscreen.bpd.and_then(zero_none)) {
                                        buf.push(["BP: ", &bps.to_string(), "/", &bpd.to_string(), " mmHg"].concat());
                                        let map = ((bpd as u32 * 2) + bps as u32) / 3;
                                        buf.push(["MAP: ", &map.to_string(), " mmHg"].concat());
                                    }
                                    if let Some(ps) = opdscreen.pain_score.and_then(zero_none).map(|i| i.to_string()) {
                                        buf.push(["PS: ", &ps, "/10"].concat());
                                    }
                                    if let (Some(e), Some(v), Some(m)) = (
                                        opdscreen.gcs_e.and_then(zero_none).map(|f| f.to_string()),
                                        opdscreen.gcs_v.and_then(zero_none).map(|f| f.to_string()),
                                        opdscreen.gcs_m.and_then(zero_none).map(|f| f.to_string()),
                                    ) {
                                        buf.push(["GCS: E", &e, "V", &v, "M", &m].concat());
                                    }
                                    if let Some(bw) = opdscreen.bw.and_then(zero_none).map(|f| f.to_string()) {
                                        buf.push(["BW: ", &bw, " Kg"].concat());
                                    }
                                    if let Some(ht) = opdscreen.height.and_then(zero_none).map(|i| i.to_string()) {
                                        buf.push(["HT: ", &ht, " cm"].concat());
                                    }
                                    let vs_text = buf.join(", ");

                                    let old_text = modal.parent_result.get_cloned();
                                    let spacer = match (old_text.is_empty(), modal.with_datetime) {
                                        (true, true) => ["- [", &vs_date, " ", &vs_time, "] "].concat(),
                                        (true, false) => String::new(),
                                        (false, true) => ["\r\n- [", &vs_date, " ", &vs_time, "] "].concat(),
                                        (false, false) => String::from(" "),
                                    };
                                    modal.parent_result.set([old_text.as_str(), spacer.as_str(), &vs_text].concat());
                                    modal.parent_changed.set(true);
                                }
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

    pub fn render(modal: Rc<Self>, app: Rc<App>) -> Dom {
        html!("div", {
            .future(map_ref!(
                let busy = app.loader_is_loading(),
                let loaded = modal.loaded.signal() =>
                !busy && !loaded
            ).for_each(clone!(app, modal => move |ready| {
                if ready {
                    Self::load(modal.clone(), app.clone());
                    modal.loaded.set_neq(true);
                }
                async {}
            })))
            .future(map_ref!{
                let busy = app.loader_is_loading(),
                let changed = modal.changed.signal() =>
                !busy && *changed
            }.for_each(clone!(app, modal => move |changed| {
                if changed {
                    Self::load(modal.clone(), app.clone());
                    modal.changed.set(false);
                }
                async {}
            })))
            .class(class::MODAL_DIALOG_LG)
            .attr("role", "document")
            .child(html!("div", {
                .class("modal-content")
                .children([
                    html!("div", {
                        .class("modal-header")
                        .children([
                            html!("h5", {.class("modal-title").text("เลือกสัญญาณชีพ")}),
                            doms::close_modal_x_btn(),
                        ])
                    }),
                    html!("div", {
                        .class("modal-body")
                        //.style("height","400px")
                        .style("width", "100%")
                        .apply_if(app.endpoint_is_allow(&Method::GET, &EndPoint::OpdErMedicalHistory, false), |dom| dom
                            .child(html!("button" => HtmlButtonElement, {
                                .attr("type", "button")
                                .class(class::BTN_FR_GRAY)
                                .attr("data-bs-dismiss", "modal")
                                .child(html!("i", {.class(class::FA_DOWNLOAD)}))
                                .text(" HOSxP")
                                .apply(mixins::click_with_loader_checked(clone!(app, modal => move || {
                                    Self::get_vs_hosxp(modal.clone(), app.clone())
                                }), app.state()))
                            }))
                        )
                        .children([
                            html!("div", {
                                .class(class::FLEX_WRAP_T)
                                .children([
                                    html!("div", {
                                        .class(class::COLA_PY_L)
                                        .child(html!("div", {
                                            .class(class::INPUT_GROUP)
                                            .children([
                                                doms::label_group_for("display_vs_date_from","วันที่"),
                                                doms::date_picker(
                                                    modal.start_vs_date.clone(),
                                                    modal.changed.clone(), always(false), None,
                                                    |d| d.class(class::FLEX_GROW1).style("min-width","135px"),
                                                    |d| d.class("rounded-0"),
                                                    |d| d.class("rounded-0").attr("id", "display_vs_date_from"),
                                                    |s| s, always(None),
                                                ),
                                                doms::label_group_for("display_vs_date_to","ถึง"),
                                                doms::date_picker(
                                                    modal.end_vs_date.clone(),
                                                    modal.changed.clone(), always(false), None,
                                                    |d| d.class(class::FLEX_GROW1).style("min-width","135px"),
                                                    |d| d.class("rounded-start-0"),
                                                    |d| d.class("rounded-start-0").attr("id", "display_vs_date_to"),
                                                    |s| s, always(None),
                                                ),
                                            ])
                                        }))
                                    }),
                                    html!("div", {
                                        .class("py-1")
                                        .child(html!("button", {
                                            .attr("type", "button")
                                            .class(class::BTN_L_GRAY)
                                            .text("วันนี้")
                                            .event(clone!(modal => move |_: events::Click| {
                                                modal.set_day_last(1, true);
                                            }))
                                        }))
                                        .child_signal(modal.is_ipd().map(clone!(modal => move |is_ipd| (!is_ipd).then(|| {
                                            html!("button", {
                                                .attr("type", "button")
                                                .class(class::BTN_L_GRAY)
                                                .text("2 วัน")
                                                .event(clone!(modal => move |_: events::Click| {
                                                    modal.set_day_last(2, true);
                                                }))
                                            })
                                        }))))
                                        .child_signal(modal.is_ipd().map(clone!(modal => move |is_ipd| is_ipd.then(|| {
                                            html!("button", {
                                                .attr("type", "button")
                                                .class(class::BTN_L_GRAY)
                                                .text("3 วัน")
                                                .event(clone!(modal => move |_: events::Click| {
                                                    modal.set_day_last(3, true);
                                                }))
                                            })
                                        }))))
                                        .child_signal(modal.is_ipd().map(clone!(modal => move |is_ipd| is_ipd.then(|| {
                                            html!("button", {
                                                .attr("type", "button")
                                                .class(class::BTN_L_GRAY)
                                                .text("7 วัน")
                                                .event(clone!(modal => move |_: events::Click| {
                                                    modal.set_day_last(7, true);
                                                }))
                                            })
                                        }))))
                                        .children([
                                            html!("button", {
                                                .attr("type", "button")
                                                .class(class::BTN_L_GRAY)
                                                .text("ทั้งหมด")
                                                .event(clone!(modal => move |_: events::Click| {
                                                    modal.set_day_last(0, true);
                                                }))
                                            }),
                                            html!("button", {
                                                .attr("type", "button")
                                                .class(class::BTN_L_GRAY)
                                                .child(html!("i", {.class(class::FA_BACKWARD)}))
                                                .event(clone!(modal => move |_: events::Click| {
                                                    modal.set_days_next(false);
                                                }))
                                            }),
                                            html!("button", {
                                                .attr("type", "button")
                                                .class(class::BTN_L_GRAY)
                                                .child(html!("i", {.class(class::FA_FORWARD)}))
                                                .event(clone!(modal => move |_: events::Click| {
                                                    modal.set_days_next(true);
                                                }))
                                            }),
                                        ])
                                    }),
                                ])
                            }),
                            html!("div", {
                                .style("overflow-y","auto")
                                .style("max-height","50vh")
                                .child(doms::table_responsive(class::TABLE_STRIP, clone!(modal => move |table| { table
                                    .children([
                                        html!("thead", {
                                            .child(html!("tr", {
                                                .children([
                                                    html!("th", {.attr("scope", "col").text("วัน-เวลา")}),
                                                    html!("th", {.attr("scope", "col").text("รายละเอียด")}),
                                                ])
                                            }))
                                        }),
                                        html!("tbody", {
                                            .children_signal_vec(modal.vs_result.signal_cloned().to_signal_vec().map(move |row| {
                                                let birthday = modal.patient.lock_ref().as_ref().and_then(|pt| pt.birthday());
                                                let (vital_sign_text, _) = full_text(row.clone(), birthday, app.clone());
                                                html!("tr", {
                                                    .style("cursor","pointer")
                                                    .attr("data-bs-dismiss", "modal")
                                                    .children([
                                                        html!("td", {
                                                            .class("text-nowrap")
                                                            .text(&datetime_th(&row.vs_datetime))
                                                            .attr("title", &[
                                                                "บันทึกโดย: ", &row.create_opduser_name.clone().unwrap_or_default(),
                                                                " (", &datetime_th(&row.create_datetime), ")\nแก้ไขล่าสุด: ", &row.update_opduser_name.clone().unwrap_or_default(),
                                                                " (", &datetime_th(&row.update_datetime), ")"
                                                            ].concat())
                                                        }),
                                                        html!("td", {.text(&vital_sign_text)}),
                                                    ])
                                                    .event(clone!(modal => move |_:events::Click| {
                                                        let old_text = modal.parent_result.get_cloned();
                                                        let spacer = match (old_text.is_empty(), modal.with_datetime) {
                                                            (true, true) => ["- [", &datetime_th(&row.vs_datetime), "] "].concat(),
                                                            (true, false) => String::new(),
                                                            (false, true) => ["\r\n- [", &datetime_th(&row.vs_datetime), "] "].concat(),
                                                            (false, false) => String::from(" "),
                                                        };
                                                        modal.parent_result.set([
                                                            old_text.as_str(), spacer.as_str(), &vital_sign_text
                                                        ].concat());
                                                        modal.parent_changed.set(true);
                                                    }))
                                                })
                                            }))
                                        }),
                                    ])
                                })))
                            }),
                        ])
                    }),
                    html!("div", {
                        .class("modal-footer")
                        .child(html!("button", {
                            .attr("type", "button")
                            .class(class::BTN_GRAY)
                            .attr("data-bs-dismiss", "modal")
                            .text("ปิด")
                        }))
                    }),
                ])
            }))
        })
    }
}
