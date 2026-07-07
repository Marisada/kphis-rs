// ipd-nurse-note.php
// opd-er-nurse-note.php
// ipd-nurse-focus-note-table-all.php
// opd-er-nurse-focus-note-table-all.php

use dominator::{Dom, clone, events, html, with_node};
use futures_signals::{
    map_ref,
    signal::{Mutable, Signal, SignalExt, always, not},
    signal_vec::{MutableVec, SignalVecExt},
};
use std::rc::Rc;
use web_sys::HtmlSelectElement;

use kphis_model::{
    app::VisitTypeId,
    focus_list::{FocusList, FocusListParams},
    focus_note::{FocusNote, FocusNoteParams},
    patient_info::PatientInfo,
    report::{SystemReport, TypstReport},
    user::permission::Permission,
};
use kphis_ui_app::App;
use kphis_ui_core::{class, doms, mixins};
use kphis_util::{
    datetime::{date_8601, js_now},
    util::{set_day_last, set_days_next, str_some},
};

use crate::gadget::pdf_button::PdfButtons;

/// - GET `EndPoint::IpdFocusListAn`
/// - GET `EndPoint::OpdErFocusListId`
/// - GET `EndPoint::IpdFocusNoteAn`
/// - GET `EndPoint::OpdErFocusNoteId`
#[derive(Clone, Default)]
pub struct NoteListCpn {
    loaded_focus_list: Mutable<bool>,
    focus_list: MutableVec<Rc<FocusList>>,

    patient: Mutable<Option<Rc<PatientInfo>>>,

    changed: Mutable<bool>,
    checked: Mutable<bool>,

    start_date: Mutable<String>,
    end_date: Mutable<String>,
    fclist_id: Mutable<String>,
    // "1" = not solved, "2" = solved, "" = all
    status: Mutable<String>,

    notes: MutableVec<Rc<(VisitTypeId, Rc<FocusNote>)>>,
}

impl NoteListCpn {
    pub fn new(focus_list: MutableVec<Rc<FocusList>>, loaded_focus_list: Mutable<bool>, patient: Mutable<Option<Rc<PatientInfo>>>, reload_list: Mutable<bool>) -> Rc<Self> {
        let now = js_now();
        let end_date = patient.lock_ref().as_ref().and_then(|pt| pt.lastdate()).unwrap_or(now.date());
        let start_date = end_date.previous_day().unwrap_or(now.date());

        Rc::new(Self {
            loaded_focus_list,
            focus_list,
            patient,
            start_date: Mutable::new(start_date.to_string()),
            end_date: Mutable::new(end_date.to_string()),
            changed: reload_list,
            ..Default::default()
        })
    }

    fn is_ipd(&self) -> impl Signal<Item = bool> + use<> {
        self.patient.signal_cloned().map(|opt| opt.as_ref().map(|pt| pt.visit_type().is_ipd()).unwrap_or_default())
    }

    fn can_edit(&self, app: Rc<App>) -> bool {
        match self.patient.lock_ref().as_ref() {
            Some(patient) => {
                let is_pre_admit = matches!(patient.visit_type, VisitTypeId::PreAdmit(_));
                if patient.is_ipd() {
                    app.has_permission(Permission::IpdNurseNoteEdit) || (is_pre_admit && app.has_permission(Permission::OpdErNurseNoteEdit))
                } else {
                    app.has_permission(Permission::OpdErNurseNoteEdit)
                }
            }
            None => false,
        }
    }

    fn set_day_last(&self, days: u64, from_now: bool) {
        if let Some(patient) = self.patient.lock_ref().as_ref() {
            let last_date = if from_now { Some(js_now().date()) } else { patient.lastdate() };
            set_day_last(patient.regdate(), last_date, self.start_date.clone(), self.end_date.clone(), self.changed.clone(), days);
        }
    }

    fn set_days_next(&self, forward: bool) {
        set_days_next(self.start_date.clone(), self.end_date.clone(), self.changed.clone(), forward);
    }

    pub fn load_focus_list(page: Rc<Self>, app: Rc<App>) {
        page.focus_list.lock_mut().clear();
        let visit_type_opt = page.patient.lock_ref().as_ref().map(|pt| pt.visit_type());
        if let Some(visit_type) = visit_type_opt {
            app.async_load(
                true,
                clone!(app, page => async move {
                    let fcl_params = FocusListParams {
                        status: str_some(page.status.get_cloned()),
                        ..Default::default()
                    };
                    let result_opt = match visit_type {
                        VisitTypeId::Ipd(an) | VisitTypeId::PreAdmit(an) => {
                            // GET `EndPoint::IpdFocusListAn`
                            Some(FocusList::call_api_get_ipd(&an, &fcl_params, app.state()).await)
                        }
                        VisitTypeId::OpdEr(_, opd_er_order_master_id) => {
                            // GET `EndPoint::OpdErFocusListId`
                            Some(FocusList::call_api_get_opd_er(opd_er_order_master_id, &fcl_params, app.state()).await)
                        }
                        VisitTypeId::Visit(_) => None,
                    };
                    if let Some(result) = result_opt {
                        match result {
                            Ok(focus_lists) => {
                                page.focus_list.lock_mut().extend(focus_lists.into_iter().map(Rc::new));
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

    pub fn load_note_list(page: Rc<Self>, app: Rc<App>) {
        let no_status = page.status.lock_ref().is_empty();
        if no_status || (!no_status && !page.fclist_id.lock_ref().is_empty()) {
            let visit_type_opt = page.patient.lock_ref().as_ref().map(|pt| pt.visit_type());
            if let Some(visit_type) = visit_type_opt {
                page.notes.lock_mut().clear();

                app.async_load(
                    true,
                    clone!(app, page => async move {
                        let params = FocusNoteParams {
                            start_date: date_8601(&page.start_date.lock_ref()),
                            end_date: date_8601(&page.end_date.lock_ref()),
                            fclist_id: page.fclist_id.lock_ref().parse::<u32>().ok(),
                            ..Default::default()
                        };
                        let result_opt = match &visit_type {
                            VisitTypeId::Ipd(an) | VisitTypeId::PreAdmit(an) => {
                                // GET `EndPoint::IpdFocusNoteAn`
                                Some(FocusNote::call_api_get_ipd(an, &params, app.state()).await)
                            }
                            VisitTypeId::OpdEr(_, opd_er_order_master_id) => {
                                // GET `EndPoint::OpdErFocusNoteId`
                                Some(FocusNote::call_api_get_opd_er(*opd_er_order_master_id, &params, app.state()).await)
                            }
                            VisitTypeId::Visit(_) => None,
                        };
                        if let Some(result) = result_opt {
                            match result {
                                Ok(focus_notes) => {
                                    page.checked.set_neq(!focus_notes.is_empty());
                                    page.notes.lock_mut().extend(focus_notes.into_iter().map(|note| Rc::new((visit_type.clone(), Rc::new(note)))));
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
    }

    pub fn render(page: Rc<Self>, app: Rc<App>, parent_focus_note: Option<Mutable<Option<Rc<FocusNote>>>>) -> Dom {
        let can_edit = page.can_edit(app.clone());

        html!("div", {
            .future(map_ref!{
                let busy = app.loader_is_loading(),
                let loaded = page.loaded_focus_list.signal() =>
                !busy && !loaded
            }.for_each(clone!(app, page => move |ready| {
                if ready {
                    Self::load_focus_list(page.clone(), app.clone());
                    page.loaded_focus_list.set(true);
                }
                async {}
            })))
            .future(map_ref!(
                let busy = app.loader_is_loading(),
                let changed = page.changed.signal() =>
                !busy && *changed
            ).for_each(clone!(app, page => move |ready| {
                if ready {
                    Self::load_note_list(page.clone(), app.clone());
                    page.changed.set_neq(false);
                }
                async {}
            })))
            .class("col")
            .child(html!("div", {
                .children([
                    html!("div", {
                        .class(class::FLEX_WRAP_T)
                        .children([
                            html!("div", {
                                .class(class::COLA_PY_L)
                                .child(html!("div", {
                                    .class(class::INPUT_GROUP)
                                    .style("max-width", "min-content")
                                    .children([
                                        doms::label_group_for("search_startdate","วันที่"),
                                        doms::date_picker(
                                            page.start_date.clone(),
                                            page.changed.clone(), always(false), None,
                                            |d| d.class(class::FLEX_GROW1).style("min-width","135px"),
                                            |d| d.class("rounded-0"),
                                            |d| d.class("rounded-0").attr("id", "search_startdate"),
                                            |s| s, always(None),
                                        ),
                                        doms::label_group_for("search_enddate","ถึง"),
                                        doms::date_picker(
                                            page.end_date.clone(),
                                            page.changed.clone(), always(false), None,
                                            |d| d.class(class::FLEX_GROW1).style("min-width","135px"),
                                            |d| d.class("rounded-start-0"),
                                            |d| d.class("rounded-start-0").attr("id", "search_enddate"),
                                            |s| s, always(None),
                                        ),
                                    ])
                                }))
                            }),
                            html!("div", {
                                .class("py-1")
                                .children([
                                    html!("button", {
                                        .attr("type", "button")
                                        .class(class::BTN_L_GRAY)
                                        .text("วันนี้")
                                        .event(clone!(page => move |_: events::Click| {
                                            page.set_day_last(1, true);
                                        }))
                                    }),
                                    html!("button", {
                                        .visible_signal(not(page.is_ipd()))
                                        .attr("type", "button")
                                        .class(class::BTN_L_GRAY)
                                        .text("2 วัน")
                                        .event(clone!(page => move |_: events::Click| {
                                            page.set_day_last(2, true);
                                        }))
                                    }),
                                    html!("button", {
                                        .visible_signal(page.is_ipd())
                                        .attr("type", "button")
                                        .class(class::BTN_L_GRAY)
                                        .text("3 วัน")
                                        .event(clone!(page => move |_: events::Click| {
                                            page.set_day_last(3, true);
                                        }))
                                    }),
                                    html!("button", {
                                        .visible_signal(page.is_ipd())
                                        .attr("type", "button")
                                        .class(class::BTN_L_GRAY)
                                        .text("7 วัน")
                                        .event(clone!(page => move |_: events::Click| {
                                            page.set_day_last(7, true);
                                        }))
                                    }),
                                    html!("button", {
                                        .attr("type", "button")
                                        .class(class::BTN_L_GRAY)
                                        .text("ทั้งหมด")
                                        .event(clone!(page => move |_: events::Click| {
                                            page.set_day_last(0, true);
                                        }))
                                    }),
                                    html!("button", {
                                        .attr("type", "button")
                                        .class(class::BTN_L_GRAY)
                                        .child(html!("i", {.class(class::FA_BACKWARD)}))
                                        .event(clone!(page => move |_: events::Click| {
                                            page.set_days_next(false);
                                        }))
                                    }),
                                    html!("button", {
                                        .attr("type", "button")
                                        .class(class::BTN_L_GRAY)
                                        .child(html!("i", {.class(class::FA_FORWARD)}))
                                        .event(clone!(page => move |_: events::Click| {
                                            page.set_days_next(true);
                                        }))
                                    }),
                                ])
                            }),
                            html!("div", {
                                .class("py-1")
                                .child(html!("div", {
                                    .class(class::INPUT_GROUP)
                                    .children([
                                        doms::label_group_for("search_fclist","Focus"),
                                        html!("select" => HtmlSelectElement, {
                                            .class("form-select")
                                            .attr("id", "search_fclist")
                                            .child(html!("option", {
                                                .attr("value","")
                                                .text_signal(page.status.signal_cloned().map(|status| {
                                                    if status.is_empty() {"ทั้งหมด"} else {"เลือก"}
                                                }))
                                            }))
                                            .children_signal_vec(page.focus_list.signal_vec_cloned().map(|focus_list| {
                                                html!("option", {
                                                    .attr("value", &focus_list.fclist_id.to_string())
                                                    .text(&[focus_list.focus_name.clone().unwrap_or_default(), focus_list.focus_text.as_ref().map(|txt| [", ", txt].concat()).unwrap_or_default()].concat())
                                                })
                                            }))
                                            .child_signal(page.status.signal_cloned().map(|status| {
                                                status.is_empty().then(|| html!("option", {.attr("value","0").text("ไม่มี Focus")}))
                                            }))
                                            .apply(mixins::string_value_select(page.fclist_id.clone(), page.changed.clone()))
                                        }),
                                        doms::label_group_for("search_status","สถานะ"),
                                        html!("select" => HtmlSelectElement, {
                                            .class("form-select")
                                            .attr("id", "search_status")
                                            .children([
                                                html!("option", {.attr("value","").text("ทั้งหมด")}),
                                                html!("option", {.attr("value","1").text("ปัญหายังคงอยู่")}),
                                                html!("option", {.attr("value","2").text("ปัญหาหมดไป")}),
                                            ])
                                            .prop_signal("value", page.status.signal_cloned())
                                            .with_node!(element => {
                                                .event(clone!(page => move |_: events::Change| {
                                                    let value = element.value();
                                                    if value.is_empty() {
                                                        page.changed.set_neq(true);
                                                    }
                                                    page.focus_list.lock_mut().clear();
                                                    page.fclist_id.set(String::new());
                                                    page.notes.lock_mut().clear();

                                                    page.status.set(value);
                                                    page.loaded_focus_list.set(false);
                                                }))
                                            })
                                        }),
                                    ])
                                }))
                            })
                        ])
                        .child_signal(page.patient.signal_cloned().map(clone!(app, page => move |opt| {
                            opt.map(|patient| {
                                match patient.visit_type() {
                                    VisitTypeId::Ipd(an) | VisitTypeId::PreAdmit(an) => {
                                        html!("div",{
                                            .class(class::COLA_PY_RX)
                                            .children(PdfButtons::buttons(
                                                PdfButtons::new(
                                                    TypstReport::from_system_with_coercion(SystemReport::IpdFocusNote, &app.state().report_coercions()),
                                                    Mutable::new(an.clone()),
                                                    page.checked.clone(),
                                                    page.changed.clone(),
                                                    clone!(page => move || {serde_json::json!({
                                                        "id": an,
                                                        "patient": patient,
                                                        "note": page.notes.lock_ref().iter().map(|tuple| tuple.1.clone()).collect::<Vec<Rc<FocusNote>>>(),
                                                    }).to_string()})
                                                ), "PDF", Some("PDF (All)"), app.clone()
                                            ))
                                        })
                                    }
                                    VisitTypeId::OpdEr(vn, _opd_er_order_master_id) => {
                                        html!("div",{
                                            .class(class::COLA_PY_RX)
                                            .children(PdfButtons::buttons(
                                                PdfButtons::new(
                                                    TypstReport::from_system_with_coercion(SystemReport::OpdErFocusNote, &app.state().report_coercions()),
                                                    Mutable::new(vn.clone()),
                                                    page.checked.clone(),
                                                    page.changed.clone(),
                                                    clone!(page => move || {serde_json::json!({
                                                        "id": vn,
                                                        "patient": patient,
                                                        "note": page.notes.lock_ref().iter().map(|tuple| tuple.1.clone()).collect::<Vec<Rc<FocusNote>>>(),
                                                    }).to_string()})
                                                ), "PDF", Some("PDF (All)"), app.clone(),
                                            ))
                                        })
                                    }
                                    VisitTypeId::Visit(_) => Dom::empty(),
                                }
                            })
                        })))
                    }),
                    doms::table_responsive(class::TABLE_STRIP, clone!(page => move |table| { table
                        .children([
                            html!("thead", {
                                .child(html!("tr", {
                                    .class("text-center")
                                    .children([
                                        html!("th", {.attr("scope", "col").attr("min-width","35px").text("#")}),
                                        html!("th", {.attr("scope", "col").style("min-width","105px").text("วันที่/เวลา")}),
                                        html!("th", {.attr("scope", "col").text("ประเภทผู้ป่วย")}),
                                        html!("th", {.attr("scope", "col").text("Focus")}),
                                        html!("th", {.attr("scope", "col").text("A,I,E")}),
                                        html!("th", {.attr("scope", "col").style("min-width","95px").text("ผู้บันทึก")}),
                                    ])
                                }))
                            }),
                            html!("tbody", {
                                .children_signal_vec(page.notes.signal_vec_cloned().enumerate().map(move |(i,row)| {
                                    super::focus_note_row::render(i.get().unwrap_or_default(), row, parent_focus_note.clone(), page.focus_list.clone(), can_edit, app.clone())
                                }))
                            }),
                        ])
                    })),
                ])
            }))
        })
    }
}
