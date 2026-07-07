use dominator::{Dom, DomBuilder, clone, events, html, text, with_node};
use futures_signals::{
    map_ref,
    signal::{Mutable, Signal, SignalExt, always, not},
    signal_vec::{MutableVec, SignalVecExt},
};
use std::rc::Rc;
use web_sys::HtmlInputElement;

use kphis_model::{
    app::VisitTypeId,
    endpoint::EndPoint,
    fetch::Method,
    ipd::his::HisOperationAdmit,
    patient_info::PatientInfo,
    report::{SystemReport, TypstReport},
    score::Scores,
    vital_sign::{VitalSign, VitalSignParams, VsMode},
};
use kphis_ui_app::App;
use kphis_ui_core::{class, doms};
use kphis_util::{
    datetime::{date_8601, datetime_th_opt, js_now},
    util::{set_day_last, set_days_next},
};

use super::{
    items_general::{render_vs_result, search_btns_general, table_heads_general},
    items_labour::{render_lr_result, search_btns_labour, table_heads_labour},
    items_neuro::{render_neuro_result, search_btns_neuro, table_heads_neuro},
    items_psychia::{render_psychia_result, search_btns_psychia, table_heads_psychia},
};
use crate::gadget::pdf_button::PdfButtons;

/// - GET `EndPoint::IpdVitalSign`
/// - GET `EndPoint::OpdErVitalSign`
/// - GET `EndPoint::HisOperationAdmitAn` (guarded, remove operation-admit)
#[derive(Clone, Default)]
pub struct VitalSignDataCpn {
    patient: Mutable<Option<Rc<PatientInfo>>>,

    start_vs_date: Mutable<String>,
    end_vs_date: Mutable<String>,
    day_range: Mutable<u64>,
    vs_result: Mutable<Vec<Rc<VitalSign>>>,
    op_result: MutableVec<Rc<HisOperationAdmit>>,

    changed: Mutable<bool>,
    checked: Mutable<bool>,
    vs_mode: Mutable<VsMode>,
    zoomable: Mutable<bool>,

    chart_render: Mutable<bool>,
    table_render: Mutable<bool>,
    data_redraw: Mutable<bool>,

    search_text: Mutable<String>,
}

impl VitalSignDataCpn {
    pub fn new(patient: Mutable<Option<Rc<PatientInfo>>>, vs_result: Mutable<Vec<Rc<VitalSign>>>, start_vs_date: Mutable<String>, end_vs_date: Mutable<String>, vs_mode: Mutable<VsMode>) -> Rc<Self> {
        let not_has_vs_date = start_vs_date.lock_ref().is_empty() || end_vs_date.lock_ref().is_empty();
        let is_ipd = patient.lock_ref().as_ref().map(|pt| pt.is_ipd()).unwrap_or_default();
        let data = Self {
            patient,
            start_vs_date,
            end_vs_date,
            vs_result,
            vs_mode,
            changed: Mutable::new(true),
            ..Default::default()
        };
        if not_has_vs_date {
            if is_ipd {
                data.set_day_last(7, false);
            } else {
                data.set_day_last(2, false);
            }
        }
        Rc::new(data)
    }

    fn is_ipd(&self) -> impl Signal<Item = bool> + use<> {
        self.patient.signal_cloned().map(|opt| opt.as_ref().map(|pt| pt.is_ipd()).unwrap_or_default())
    }

    fn is_ipd_and_is_pre_admit(&self) -> impl Signal<Item = (bool, bool)> + use<> {
        self.patient.signal_ref(|opt| opt.as_ref().map(|pt| pt.visit_type.is_ipd_and_is_pre_admit()).unwrap_or_default())
    }

    pub fn set_day_last(&self, days: u64, from_now: bool) {
        if let Some(patient) = self.patient.lock_ref().as_ref() {
            let last_date = if from_now { Some(js_now().date()) } else { patient.lastdate() };
            self.day_range.set(days);
            set_day_last(patient.regdate(), last_date, self.start_vs_date.clone(), self.end_vs_date.clone(), self.changed.clone(), days);
        }
    }

    fn set_days_next(&self, forward: bool) {
        set_days_next(self.start_vs_date.clone(), self.end_vs_date.clone(), self.changed.clone(), forward);
    }

    pub fn render(vs_id: Mutable<u32>, form_rendered: Mutable<bool>, page: Rc<Self>, app: Rc<App>) -> Dom {
        html!("div", {
            .future(map_ref!{
                let busy = app.loader_is_loading(),
                let changed = page.changed.signal() =>
                !busy && *changed
            }.for_each(clone!(app, page => move |changed| {
                if changed {
                    Self::submit_id(page.clone(), app.clone());
                    page.changed.set(false);
                }
                async {}
            })))
            //.attr("id", "show-chart-table")
            .style("min-width","610px")
            .child(Self::render_chart(page.clone(), app.clone()))
            .child_signal(page.is_ipd_and_is_pre_admit().map(clone!(app, page => move |(is_ipd, is_pre_admit)| {
                (is_ipd && app.endpoint_is_allow(&Method::GET, &EndPoint::HisOperationAdmitAn, is_pre_admit)).then(|| {
                    Self::render_op(page.clone())
                })
            })))
            .child_signal(page.table_render.signal_cloned().map(clone!(app, page => move |table_render| {
                table_render.then(|| {
                    Self::render_table(vs_id.clone(), form_rendered.clone(), page.clone(), app.clone())
                })
            })))
        })
    }

    fn render_chart(page: Rc<Self>, app: Rc<App>) -> Dom {
        html!("div", {
            .children([
                html!("div", {
                    .class(class::FLEX_WRAP_T)
                    .children([
                        html!("div", {
                            .class(class::COLA_PY_L)
                            .child(html!("div", {
                                .class(class::INPUT_GROUP)
                                .children([
                                    doms::label_group_for("display_vs_date_from","แสดงข้อมูลวันที่"),
                                    doms::date_picker(
                                        page.start_vs_date.clone(),
                                        page.changed.clone(), always(false), None,
                                        |d| d.class(class::FLEX_GROW1).style("min-width","135px"),
                                        |d| d.class("rounded-0"),
                                        |d| d.class("rounded-0").attr("id", "display_vs_date_from"),
                                        |s| s, always(None),
                                    ),
                                    doms::label_group_for("display_vs_date_to","ถึง"),
                                    doms::date_picker(
                                        page.end_vs_date.clone(),
                                        page.changed.clone(), always(false), None,
                                        |d| d.class(class::FLEX_GROW1).style("min-width","135px"),
                                        |d| d.class("rounded-start-0"),
                                        |d| d.class("rounded-start-0").attr("id", "display_vs_date_to"),
                                        |s| s, always(None),
                                    ),
                                ])
                            }))
                        }),
                        html!("div", {
                            .class(class::FLEX_PY1)
                            .child(Self::render_range_button(page.clone(), 1, "วันนี้"))
                            .child_signal(page.is_ipd().map(clone!(page => move |is_ipd| (!is_ipd).then(|| Self::render_range_button(page.clone(), 2, "2 วัน")))))
                            .child_signal(page.is_ipd().map(clone!(page => move |is_ipd| is_ipd.then(|| Self::render_range_button(page.clone(), 3, "3 วัน")))))
                            .child_signal(page.is_ipd().map(clone!(page => move |is_ipd| is_ipd.then(|| Self::render_range_button(page.clone(), 7, "7 วัน")))))
                            .child_signal(page.is_ipd().map(clone!(page => move |is_ipd| is_ipd.then(|| Self::render_range_button(page.clone(), 15, "15 วัน")))))
                            .child_signal(page.is_ipd().map(clone!(page => move |is_ipd| is_ipd.then(|| Self::render_range_button(page.clone(), 30, "30 วัน")))))
                            .children([
                                Self::render_range_button(page.clone(), 0, "ทั้งหมด"),
                                html!("button", {
                                    .attr("type", "button")
                                    .class(class::BTN_L_GRAY)
                                    .child(html!("i", {.class(class::FA_BACKWARD)}))
                                    .event(clone!(page => move |_: events::Click| {
                                        page.set_days_next(false);
                                    }))
                                    // .attr("onclick", "onclickShiftPeriod(event,'back');")
                                }),
                                html!("button", {
                                    .attr("type", "button")
                                    .class(class::BTN_L_GRAY)
                                    .child(html!("i", {.class(class::FA_FORWARD)}))
                                    .event(clone!(page => move |_: events::Click| {
                                        page.set_days_next(true);
                                    }))
                                    // .attr("onclick", "onclickShiftPeriod(event,'forward');")
                                }),
                            ])
                        }),
                        html!("div", {
                            .class(class::FLEX_PY1)
                            .child(Self::vs_mode_radio(page.clone()))
                        }),
                        html!("div", {
                            .class(class::FLEX_PY1)
                            .child(html!("div", {
                                .class(class::FORM_CHK_SW)
                                .class("mx-3")
                                .style("padding-top","8px")
                                .children([
                                    html!("input" => HtmlInputElement, {
                                        .attr("type", "checkbox")
                                        .attr("id", "zoomable_sw")
                                        .class("form-check-input")
                                        .attr("role","switch")
                                        .with_node!(element => {
                                            .future(page.zoomable.signal().for_each(clone!(element => move |v| {
                                                element.set_checked(v);
                                                async {}
                                            })))
                                            .event(clone!(page => move |_: events::Change| {
                                                page.zoomable.set_neq(element.checked());
                                                page.chart_render.set(true);
                                            }))
                                        })
                                    }),
                                    doms::label_check_for("zoomable_sw","ย่อ/ขยายได้"),
                                ])
                            }))
                        }),
                    ])
                    .child_signal(map_ref!{
                        let patient_opt = page.patient.signal_cloned(),
                        let vs_mode = page.vs_mode.signal_cloned() =>
                        (patient_opt.clone(), vs_mode.clone())
                    }.map(clone!(app, page => move |(opt, vs_mode)| {
                        opt.map(|patient| {
                            match patient.visit_type() {
                                VisitTypeId::Ipd(an) | VisitTypeId::PreAdmit(an) => {
                                    let report = match vs_mode {
                                        VsMode::General => SystemReport::IpdVitalSignGeneral,
                                        VsMode::Neuro => SystemReport::IpdVitalSignNeuro,
                                        VsMode::Labour => SystemReport::IpdVitalSignLabour,
                                        VsMode::Psychia => SystemReport::IpdVitalSignPsychia,
                                    };
                                    html!("div",{
                                        .class(class::PY_RX)
                                        .children(PdfButtons::buttons(
                                            PdfButtons::new(
                                                TypstReport::from_system_with_coercion(report, &app.state().report_coercions()),
                                                Mutable::new(an.clone()),
                                                page.checked.clone(),
                                                page.changed.clone(),
                                                clone!(page, patient, an => move || {serde_json::json!({
                                                    "id": an,
                                                    "patient": patient,
                                                    "vs": page.vs_result.lock_ref().to_vec(),
                                                }).to_string()})
                                            ), "V/S", Some("All V/S"), app.clone()
                                        ))
                                        .children(PdfButtons::buttons(
                                            PdfButtons::new(
                                                TypstReport::from_system_with_coercion(SystemReport::IpdTPR, &app.state().report_coercions()),
                                                Mutable::new(an.clone()),
                                                page.checked.clone(),
                                                page.changed.clone(),
                                                clone!(patient, an => move || {serde_json::json!({
                                                    "id": an,
                                                    "patient": patient,
                                                }).to_string()})
                                            ), "TPR", None, app.clone()
                                        ))
                                        .apply_if(matches!(vs_mode, VsMode::Labour), |dom| dom
                                            .children(PdfButtons::buttons(
                                                PdfButtons::new(
                                                    TypstReport::from_system_with_coercion(SystemReport::IpdPartograph, &app.state().report_coercions()),
                                                    Mutable::new(an.clone()),
                                                    page.checked.clone(),
                                                    page.changed.clone(),
                                                    clone!(patient, an => move || {serde_json::json!({
                                                        "id": an,
                                                        "patient": patient,
                                                    }).to_string()})
                                                ), "LR", None, app.clone()
                                            ))
                                            .children(PdfButtons::buttons(
                                                PdfButtons::new(
                                                    TypstReport::from_system_with_coercion(SystemReport::IpdPartographWho, &app.state().report_coercions()),
                                                    Mutable::new(an.clone()),
                                                    page.checked.clone(),
                                                    page.changed.clone(),
                                                    clone!(patient, an => move || {serde_json::json!({
                                                        "id": an,
                                                        "patient": patient,
                                                    }).to_string()})
                                                ), "LR-WHO", None, app.clone()
                                            ))
                                        )
                                    })
                                }
                                VisitTypeId::OpdEr(vn, _opd_er_order_master_id) => {
                                    let report = match vs_mode {
                                        VsMode::General => SystemReport::OpdErVitalSignGeneral,
                                        VsMode::Neuro => SystemReport::OpdErVitalSignNeuro,
                                        VsMode::Labour => SystemReport::OpdErVitalSignLabour,
                                        VsMode::Psychia => SystemReport::OpdErVitalSignPsychia,
                                    };
                                    html!("div",{
                                        .class(class::PY_RX)
                                        .children(PdfButtons::buttons(
                                            PdfButtons::new(
                                                TypstReport::from_system_with_coercion(report, &app.state().report_coercions()),
                                                Mutable::new(vn.clone()),
                                                page.checked.clone(),
                                                page.changed.clone(),
                                                clone!(page => move || {serde_json::json!({
                                                    "id": vn,
                                                    "patient": patient,
                                                    "vs": page.vs_result.lock_ref().to_vec(),
                                                }).to_string()})
                                            ), "V/S", Some("All V/S"), app.clone()
                                        ))
                                    })
                                }
                                VisitTypeId::Visit(_) => Dom::empty(),
                            }
                        })
                    })))
                }),
                html!("div", {
                    //.attr("id", "canvasDiv")
                    .style("height","40vh")
                    .child_signal(page.chart_render.signal_cloned().map(clone!(page => move |render| {
                        render.then(|| super::chart::render(
                            &page.vs_result.lock_ref(),
                            &page.start_vs_date.lock_ref(),
                            &page.end_vs_date.lock_ref(),
                            page.vs_mode.get_cloned(),
                            page.zoomable.get(),
                        ))
                    })))
                }),
            ])
        })
    }

    fn render_op(page: Rc<Self>) -> Dom {
        html!("div", {
            .child(html!("div", {
                .class(class::ALERT_GRAY)
                .attr("role", "alert")
                .style("max-height", "calc(32vh - 120px)")
                .style("overflow-y", "auto")
                .children([
                    html!("lable", {
                        .class("fw-bold")
                        .text("ประวัติการผ่าตัด")
                    }),
                    html!("ul", {
                        .children_signal_vec(page.op_result.signal_vec_cloned().map(|op| {
                            let duration = op.end_datetime.map(|dt| js_now() - dt);
                            let year = duration.map(|du| {
                                let y = du.whole_weeks() / 52;
                                if y > 0 {[" ", &y.to_string(), " ปี"].concat()} else {String::new()}
                            }).unwrap_or_default();
                            let month = duration.map(|du| {
                                let m = du.whole_weeks() / 4;
                                if m > 0 {[" ", &m.to_string(), " เดือน"].concat()} else {String::new()}
                            }).unwrap_or_default();
                            let day = duration.map(|du| {
                                let d = du.whole_days();
                                if d > 0 {[" ", &d.to_string(), " วัน"].concat()} else {String::new()}
                            }).unwrap_or_default();
                            html!("li", {
                                .text(&[
                                    &op.name.clone().unwrap_or_default(), ", ", &op.doctor_name.clone().unwrap_or_default(),
                                    " (", &datetime_th_opt(&op.begin_datetime)," - ", &datetime_th_opt(&op.end_datetime),")[ ", &year, &month, &day, " ]"
                                ].concat())
                            })
                        }))
                        .child_signal(page.op_result.signal_vec_cloned().is_empty().map(|empty| {
                            empty.then(|| text("ไม่พบประวัติการผ่าตัด"))
                        }))
                    }),
                ])
            }))
        })
    }

    fn render_table(vs_id: Mutable<u32>, form_rendered: Mutable<bool>, page: Rc<Self>, app: Rc<App>) -> Dom {
        html!("div", {
            .children([
                html!("div", {
                    .style("z-index", "3")
                    .style("width","fit-content")
                    .children([
                        Self::search_btn("BT", "[BT]", page.clone()),
                        Self::search_btn("PR", "[PR]", page.clone()),
                        Self::search_btn("RR", "[RR]", page.clone()),
                        Self::search_btn("SBP", "[SBP]", page.clone()),
                        Self::search_btn("DBP", "[DBP]", page.clone()),
                        Self::search_btn("MAP", "[MAP]", page.clone()),
                    ])
                    .apply(|dom| {
                        match page.vs_mode.get_cloned() {
                            VsMode::General => {
                                dom.children(search_btns_general(page.clone()))
                            }
                            VsMode::Neuro => {
                                dom.children(search_btns_neuro(page.clone()))
                            }
                            VsMode::Labour => {
                                dom.children(search_btns_labour(page.clone()))
                            }
                            VsMode::Psychia => {
                                dom.children(search_btns_psychia(page.clone()))
                            }
                        }
                    })
                    .child(Self::search_btn("No Filter", "", page.clone()))
                }),
                doms::table_responsive(class::TABLE_STRIP, clone!(page => move |table| { table
                    .children([
                        html!("thead", {
                            .child(html!("tr", {
                                .children([
                                    html!("th", {.class("text-center").attr("scope", "col").text("วัน-เวลา")}),
                                    html!("th", {.class("text-center").attr("scope", "col").text("BT")}),
                                    html!("th", {.class("text-center").attr("scope", "col").text("PR")}),
                                    html!("th", {.class("text-center").attr("scope", "col").text("RR")}),
                                    html!("th", {.class("text-center").attr("scope", "col").text("SBP")}),
                                    html!("th", {.class("text-center").attr("scope", "col").text("DBP")}),
                                    html!("th", {.class("text-center").attr("scope", "col").text("MAP")}),
                                ])
                                .apply(|dom| {
                                    match page.vs_mode.get_cloned() {
                                        VsMode::General => {
                                            dom.children(table_heads_general())
                                        }
                                        VsMode::Neuro => {
                                            dom.children(table_heads_neuro())
                                        }
                                        VsMode::Labour => {
                                            dom.children(table_heads_labour())
                                        }
                                        VsMode::Psychia => {
                                            dom.children(table_heads_psychia())
                                        }
                                    }
                                })
                                .child(html!("th", {.class("text-center").attr("scope", "col").style("min-width","104px").text("รายละเอียด")}))
                            }))
                        }),
                        html!("tbody", {
                            .children_signal_vec(map_ref! {
                                let pt = page.patient.signal_cloned(),
                                let search_text = page.search_text.signal_cloned(),
                                let vs_result = page.vs_result.signal_cloned() =>
                                (search_text.clone(), vs_result.clone(), pt.as_ref().and_then(|pt| pt.birthday()))
                            }.map(clone!(app, page => move |(search_text, rows, birth_day)| {
                                rows.into_iter().filter_map(|vs| {
                                    (match search_text.as_str() {
                                        "[BT]" => vs.bt.is_some(),
                                        "[PR]" => vs.pr.is_some(),
                                        "[RR]" => vs.rr.is_some(),
                                        "[SBP]" => vs.sbp.is_some(),
                                        "[DBP]" => vs.dbp.is_some(),
                                        "[MAP]" => vs.map.is_some(),
                                        "[POS]" => vs.lr_pos.is_some(),
                                        "[FHS]" => vs.lr_fsh.is_some(),
                                        "[CX]" => vs.lr_cer.is_some(),
                                        "[I]" => vs.lr_int.is_some(),
                                        "[D]" => vs.lr_dur.is_some(),
                                        "[ST]" => vs.lr_sta_name.is_some(),
                                        "[EFF]" => vs.lr_eff.is_some(),
                                        "[MEM]" => vs.lr_mem_name.is_some(),
                                        "[AF]" => vs.lr_af.is_some(),
                                        "[O2RA]" => vs.sat_room_air.is_some(),
                                        "[SAT]" => vs.sat.is_some(),
                                        "[O2]" => vs.o2_name.is_some(),
                                        "[EWS]" => Scores::from_vs(&vs, birth_day, app.state()).is_some(),
                                        "[HAD]" => vs.had_name.is_some(),
                                        "[PS]" => vs.pain.is_some(),
                                        "[DTX]" => vs.dtx.is_some(),
                                        "[HCT]" => vs.hct.is_some(),
                                        "[U]" => vs.urine.is_some(),
                                        "[F]" => vs.feces.is_some(),
                                        "[GCS]" => vs.eye.is_some(),
                                        "[RP]" => vs.right_pupil.is_some(),
                                        "[LP]" => vs.left_pupil.is_some(),
                                        "[RA]" => vs.rt_arm_name.is_some(),
                                        "[LA]" => vs.lt_arm_name.is_some(),
                                        "[RL]" => vs.rt_leg_name.is_some(),
                                        "[LL]" => vs.lt_leg_name.is_some(),
                                        "[ADL]" => vs.barthel_index.as_ref().and_then(|concat| concat.split(',').nth(0)).is_some(),
                                        "[AWQv2]" => vs.amphetamine_awq.as_ref().and_then(|concat| concat.split(',').nth(0)).is_some(),
                                        "[AWQ-H]" => vs.amphetamine_awq.as_ref().and_then(|concat| concat.split(',').nth(1)).is_some(),
                                        "[AWQ-A]" => vs.amphetamine_awq.as_ref().and_then(|concat| concat.split(',').nth(2)).is_some(),
                                        "[AWQ-R]" => vs.amphetamine_awq.as_ref().and_then(|concat| concat.split(',').nth(3)).is_some(),
                                        "[MOT]" => vs.motivation_scale.is_some(),
                                        "[CRV]" => vs.craving_scale.is_some(),
                                        "[SOC]" => vs.stage_of_change_id.is_some(),
                                        "[OAS]" => vs.aggression_oas.as_ref().and_then(|concat| concat.split(',').nth(0)).is_some(),
                                        "[CIWA]" => vs.alcohol_ciwa.as_ref().and_then(|concat| concat.split(',').nth(0)).is_some(),
                                        "[AWS]" => vs.alcohol_aws.as_ref().and_then(|concat| concat.split(',').nth(0)).is_some(),
                                        _ => true,
                                    }).then(|| {
                                        match page.vs_mode.get_cloned() {
                                            VsMode::General => {
                                                render_vs_result(vs, vs_id.clone(), form_rendered.clone(), birth_day, app.clone())
                                            }
                                            VsMode::Labour => {
                                                render_lr_result(vs, vs_id.clone(), form_rendered.clone(), birth_day, app.clone())
                                            }
                                            VsMode::Neuro => {
                                                render_neuro_result(vs, vs_id.clone(), form_rendered.clone(), birth_day, app.clone())
                                            }
                                            VsMode::Psychia => {
                                                render_psychia_result(vs, vs_id.clone(), form_rendered.clone(), birth_day, app.clone())
                                            }
                                        }
                                    })
                                }).collect::<Vec<Dom>>()
                            }))
                            .to_signal_vec())
                        }),
                    ])
                    .child(html!("div",{.style("height","300px")}))
                })),
            ])
        })
    }

    pub fn search_btn(label: &str, search_text: &'static str, page: Rc<Self>) -> Dom {
        html!("button", {
            .attr("type", "button")
            .class(class::BTN_SM_LT)
            .class_signal("btn-outline-primary", page.search_text.signal_cloned().map(move |txt| txt.as_str() == search_text))
            .class_signal("btn-secondary", not(page.search_text.signal_cloned().map(move |txt| txt.as_str() == search_text)))
            .text(label)
            .event(clone!(page => move |_: events::Click| {
                page.search_text.set(search_text.to_string());
            }))
        })
    }

    fn submit_id(page: Rc<Self>, app: Rc<App>) {
        let visit_type_opt = page.patient.lock_ref().as_ref().map(|pt| pt.visit_type());
        if let Some(visit_type) = visit_type_opt {
            app.async_load(
                true,
                clone!(app, page => async move {
                    if let Some((an, is_pre_admit)) = visit_type.an_and_is_pre_admit()
                        && app.endpoint_is_allow(&Method::GET, &EndPoint::HisOperationAdmitAn, is_pre_admit)
                    {
                        // GET `EndPoint::HisOperationAdmitAn`
                        match HisOperationAdmit::call_api_get(an, app.state()).await {
                            Ok(items) => {
                                let mut lock = page.op_result.lock_mut();
                                lock.clear();
                                lock.extend(items.into_iter().map(Rc::new));
                            }
                            Err(e) => {
                                app.alert_app_error(&e).await;
                            }
                        }
                    }

                    let (params_opt, is_ipd) = match visit_type {
                        VisitTypeId::Ipd(an) | VisitTypeId::PreAdmit(an) => {
                            (Some(VitalSignParams {
                                an: Some(an),
                                start_date: date_8601(&page.start_vs_date.lock_ref()),
                                end_date: date_8601(&page.end_vs_date.lock_ref()),
                                ..Default::default()
                            }), true)
                        }
                        VisitTypeId::OpdEr(_, opd_er_order_master_id) => {
                            (Some(VitalSignParams {
                                opd_er_order_master_id: Some(opd_er_order_master_id),
                                start_date: date_8601(&page.start_vs_date.lock_ref()),
                                end_date: date_8601(&page.end_vs_date.lock_ref()),
                                ..Default::default()
                            }), false)
                        }
                        VisitTypeId::Visit(_) => (None, false),
                    };

                    if let Some(params) = params_opt {
                        // GET `EndPoint::IpdVitalSign`
                        // GET `EndPoint::OpdErVitalSign`
                        match VitalSign::call_api_get(is_ipd, &params, app.state()).await {
                            Ok(items) => {
                                page.checked.set_neq(!items.is_empty());
                                page.vs_result.set(items.into_iter().map(Rc::new).collect());
                                page.chart_render.set(true);
                                page.table_render.set(true);
                                page.data_redraw.set(true);
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

    fn render_range_button(page: Rc<Self>, days: u64, label: &str) -> Dom {
        html!("button", {
            .attr("type", "button")
            .class(class::BTN_L)
            .class_signal("btn-primary", page.day_range.signal_cloned().map(move |r| r == days))
            .class_signal("btn-secondary", page.day_range.signal_cloned().map(move |r| r != days))
            .text(label)
            .event(clone!(page => move |_: events::Click| {
                page.set_day_last(days, true);
            }))
        })
    }

    fn vs_mode_radio(page: Rc<Self>) -> Dom {
        html!("div", {
            .class(class::INPUT_GROUP)
            .attr("role","group")
            .attr("aria-label","Vital Sign Mode radio toggle button group")
            .children([
                html!("span", {.class("input-group-text").text("V/S")}),
                html!("input" => HtmlInputElement, {
                    .attr("type", "radio")
                    .class("btn-check")
                    .attr("id", "mode-general")
                    .attr("autocomplete","off")
                    .apply(Self::vs_mode_radio_opt_match(VsMode::General, page.clone()))
                }),
                html!("label", {
                    .class(class::BTN_BLUEO)
                    .attr("for", "mode-general")
                    .child(html!("i", {.class(class::FA_HEARTBEAT)}))
                    .text(" ทั่วไป")
                }),
                html!("input" => HtmlInputElement, {
                    .attr("type", "radio")
                    .class("btn-check")
                    .attr("id", "mode-neuro")
                    .attr("autocomplete","off")
                    .apply(Self::vs_mode_radio_opt_match(VsMode::Neuro, page.clone()))
                }),
                html!("label", {
                    .class(class::BTN_BLUEO)
                    .attr("for", "mode-neuro")
                    .child(html!("i", {.class(class::FA_BRAIN)}))
                    .text(" ประสาท")
                }),
                html!("input" => HtmlInputElement, {
                    .attr("type", "radio")
                    .class("btn-check")
                    .attr("id", "mode-labour")
                    .attr("autocomplete","off")
                    .apply(Self::vs_mode_radio_opt_match(VsMode::Labour, page.clone()))
                }),
                html!("label", {
                    .class(class::BTN_BLUEO)
                    .attr("for", "mode-labour")
                    .child(html!("i", {.class(class::FA_PREGNANT)}))
                    .text(" ห้องคลอด")
                }),
                html!("input" => HtmlInputElement, {
                    .attr("type", "radio")
                    .class("btn-check")
                    .attr("id", "mode-psychia")
                    .attr("autocomplete","off")
                    .apply(Self::vs_mode_radio_opt_match(VsMode::Psychia, page.clone()))
                }),
                html!("label", {
                    .class(class::BTN_BLUEO)
                    .attr("for", "mode-psychia")
                    .child(html!("i", {.class(class::FA_FACE_ANGRY)}))
                    .text(" จิตเวช")
                }),
            ])
        })
    }

    fn vs_mode_radio_opt_match(keyword: VsMode, page: Rc<Self>) -> impl FnOnce(DomBuilder<HtmlInputElement>) -> DomBuilder<HtmlInputElement> {
        #[inline]
        move |dom| {
            with_node!(dom, element => {
                .future(page.vs_mode.signal_cloned().for_each(clone!(element, keyword => move |v| {
                    element.set_checked(v == keyword);
                    async {}
                })))
                .event(move |_: events::Click| {
                    if page.vs_mode.get_cloned() != keyword {
                        page.vs_mode.set(keyword.clone());
                        page.chart_render.set(true);
                        page.table_render.set(true);
                        page.data_redraw.set(true);
                    }
                })
            })
        }
    }
}
