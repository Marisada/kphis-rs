use dominator::{Dom, clone, events, html};
use futures_signals::{
    map_ref,
    signal::{Mutable, Signal, SignalExt, always, not},
    signal_vec::{MutableVec, SignalVecExt},
};
use std::rc::Rc;
use web_sys::{HtmlButtonElement, HtmlTextAreaElement};

use kphis_model::{
    app::VisitTypeId,
    endpoint::EndPoint,
    fetch::Method,
    image::file_path::ImageUsage,
    order::{OrderButtons, OrderDate},
    patient_info::PatientInfo,
    pre_order::progress_note::PreProgressNoteSave,
    progress_note::{ProgressNote, ProgressNoteItem, ProgressNoteParams, ProgressNoteSave, ProgressNoteTypeName},
};
use kphis_ui_app::App;
use kphis_ui_core::{class, doms, mixins};
use kphis_util::{
    datetime::{JsTime, date_8601, js_now, time_8601},
    util::str_some,
};

use crate::{
    gadget::image::{ImageCpn, ImagePaths},
    modal::{blank_modal, lab_selector::LabSelector, vs_selector::VsSelector},
    order::{InsertTextAreaButton, OrderItemMutable},
};

/// - POST `EndPoint::IpdPreOrderProgressNote`
/// - POST `EndPoint::IpdOrderProgressNote`
/// - POST `EndPoint::OpdErOrderProgressNote`
/// - GET `EndPoint::LabHead` (LabSelector, guarded, remove lab btn)
/// - GET `EndPoint::IpdVitalSign` (VsSelector, guarded, remove v/s btn)
/// - GET `EndPoint::OpdErVitalSign` (VsSelector, guarded, remove v/s btn)
/// - GET `EndPoint::IpdOrderProgressPrevious` (guarded, remove 'Last Day' btn)
/// - POST `EndPoint::ImageUsage` (guarded, remove image gadget)
#[derive(Default)]
pub struct ProgressNoteForm {
    view_by: Mutable<String>,
    patient: Mutable<Option<Rc<PatientInfo>>>,
    pre_order_master_id: Mutable<Option<u32>>,

    // image component callback state
    image_callback: Mutable<ImagePaths>,

    is_auditor: Mutable<bool>,
    buttons_loaded: Mutable<bool>,
    to_scroll: Mutable<bool>,
    note_buttons: MutableVec<Rc<InsertTextAreaButton>>,
    subjective_buttons: MutableVec<Rc<InsertTextAreaButton>>,
    objective_buttons: MutableVec<Rc<InsertTextAreaButton>>,
    assessment_buttons: MutableVec<Rc<InsertTextAreaButton>>,
    plan_buttons: MutableVec<Rc<InsertTextAreaButton>>,

    pub progress_note_id: Mutable<Option<u32>>, // oneDayForm_order_id -> exclude_order_id
    current_date: Mutable<Option<Rc<OrderDate>>>,
    progress_note_doctor: Mutable<String>,
    // custom date-time
    progress_note_date: Mutable<String>,
    progress_note_time: Mutable<String>,

    pub changed: Mutable<bool>,
    pub focused: Mutable<Option<u32>>,

    problem_list: MutableVec<Rc<OrderItemMutable>>,
    notes: MutableVec<Rc<OrderItemMutable>>,
    subjectives: MutableVec<Rc<OrderItemMutable>>,
    objectives: MutableVec<Rc<OrderItemMutable>>,
    assessments: MutableVec<Rc<OrderItemMutable>>,
    plans: MutableVec<Rc<OrderItemMutable>>,

    vs_selector_modal: Mutable<Option<Rc<VsSelector>>>,
    lab_selector_modal: Mutable<Option<Rc<LabSelector>>>,
}

impl ProgressNoteForm {
    pub fn new(
        is_auditor: bool,
        progress_note_opt: Option<Rc<ProgressNote>>,
        current_date: Mutable<Option<Rc<OrderDate>>>,
        view_by: Mutable<String>,
        patient: Mutable<Option<Rc<PatientInfo>>>,
        pre_order_master_id: Option<u32>,
        progress_note_doctor: String,
    ) -> Rc<Self> {
        let progress_note_form = Rc::new(Self {
            view_by,
            patient,
            pre_order_master_id: Mutable::new(pre_order_master_id),
            is_auditor: Mutable::new(is_auditor),
            current_date,
            progress_note_doctor: Mutable::new(progress_note_doctor),
            ..Default::default()
        });
        if is_auditor {
            progress_note_form.changed.set_neq(true);
        }
        if let Some(progress_note) = progress_note_opt {
            progress_note_form.progress_note_id.set(Some(progress_note.progress_note_id));
            progress_note_form.progress_note_date.set(progress_note.progress_note_date.to_string());
            progress_note_form.progress_note_time.set(progress_note.progress_note_time.js_string());
            for progress_note_item_type in progress_note.progress_note_item_types.clone() {
                match progress_note_item_type.progress_note_item_type {
                    ProgressNoteTypeName::ProblemList => progress_note_form
                        .problem_list
                        .lock_mut()
                        .extend(progress_note_item_type.progress_note_items.into_iter().map(|i| Rc::new(i.into()))),
                    ProgressNoteTypeName::Note => progress_note_form
                        .notes
                        .lock_mut()
                        .extend(progress_note_item_type.progress_note_items.into_iter().map(|i| Rc::new(i.into()))),
                    ProgressNoteTypeName::Subjective => progress_note_form
                        .subjectives
                        .lock_mut()
                        .extend(progress_note_item_type.progress_note_items.into_iter().map(|i| Rc::new(i.into()))),
                    ProgressNoteTypeName::Objective => progress_note_form
                        .objectives
                        .lock_mut()
                        .extend(progress_note_item_type.progress_note_items.into_iter().map(|i| Rc::new(i.into()))),
                    ProgressNoteTypeName::Assessment => progress_note_form
                        .assessments
                        .lock_mut()
                        .extend(progress_note_item_type.progress_note_items.into_iter().map(|i| Rc::new(i.into()))),
                    ProgressNoteTypeName::Plan => progress_note_form
                        .plans
                        .lock_mut()
                        .extend(progress_note_item_type.progress_note_items.into_iter().map(|i| Rc::new(i.into()))),
                }
            }
            progress_note_form.changed.set(true);
        };

        progress_note_form
    }

    fn allow_previous_signal(&self, app: Rc<App>) -> impl Signal<Item = bool> + use<> {
        self.patient.signal_cloned().map(move |opt| {
            opt.map(|pt| {
                let (is_ipd, is_pre_admit) = pt.visit_type.is_ipd_and_is_pre_admit();
                is_ipd && app.endpoint_is_allow(&Method::GET, &EndPoint::IpdOrderProgressPrevious, is_pre_admit)
            })
            .unwrap_or_default()
        })
    }

    fn allow_vs_selector_signal(&self, app: Rc<App>) -> impl Signal<Item = bool> + use<> {
        self.patient.signal_cloned().map(move |opt| {
            opt.map(|pt| {
                let (is_ipd, is_pre_admit) = pt.visit_type.is_ipd_and_is_pre_admit();
                if is_ipd {
                    app.endpoint_is_allow(&Method::GET, &EndPoint::IpdVitalSign, is_pre_admit)
                } else {
                    app.endpoint_is_allow(&Method::GET, &EndPoint::OpdErVitalSign, false)
                }
            })
            .unwrap_or_default()
        })
    }

    fn load_buttons(page: Rc<Self>, app: Rc<App>) {
        app.async_load(
            true,
            clone!(app, page => async move {
                match OrderButtons::get("progress-note", app.state()).await {
                    Ok(buttons) => for button in buttons {
                        match button.word_type.as_str() {
                            "note" => page.note_buttons.lock_mut().extend(button.buttons.into_iter().map(|button| Rc::new(InsertTextAreaButton::from_button("note", button)))),
                            "subjective" => page.subjective_buttons.lock_mut().extend(button.buttons.into_iter().map(|button| Rc::new(InsertTextAreaButton::from_button("subjective", button)))),
                            "objective" => page.objective_buttons.lock_mut().extend(button.buttons.into_iter().map(|button| Rc::new(InsertTextAreaButton::from_button("objective", button)))),
                            "assessment" => page.assessment_buttons.lock_mut().extend(button.buttons.into_iter().map(|button| Rc::new(InsertTextAreaButton::from_button("assessment", button)))),
                            "plan" => page.plan_buttons.lock_mut().extend(button.buttons.into_iter().map(|button| Rc::new(InsertTextAreaButton::from_button("plan", button)))),
                            _ => {}
                        }
                        page.to_scroll.set(true);
                    }
                    Err(e) => {
                        app.alert_app_error(&e).await;
                    }
                }
            }),
        )
    }

    fn load_progress_previous(page: Rc<Self>, app: Rc<App>) {
        app.async_load(
            true,
            clone!(app, page => async move {
                match page.patient.lock_ref().as_ref().map(|pt| pt.visit_type()) {
                    Some(VisitTypeId::Ipd(an))
                    | Some(VisitTypeId::PreAdmit(an)) => {
                        let params = ProgressNoteParams {
                            an: str_some(an),
                            progress_note_owner_type: str_some(page.view_by.get_cloned()),
                            ..Default::default()
                        };
                        // GET `EndPoint::IpdOrderProgressPrevious`
                        match ProgressNoteItem::call_api_get_previous(&params, app.state()).await {
                            Ok(items) => {
                                if !items.is_empty() {
                                    for item in items {
                                        if let Some(progress_note_item_type) = &item.progress_note_item_type {
                                            match progress_note_item_type.as_str() {
                                                "problem-list" => page.problem_list.lock_mut().push_cloned(Rc::new(OrderItemMutable::from(item))),
                                                "note" => page.notes.lock_mut().push_cloned(Rc::new(OrderItemMutable::from(item))),
                                                "subjective" => page.subjectives.lock_mut().push_cloned(Rc::new(OrderItemMutable::from(item))),
                                                "objective" => page.objectives.lock_mut().push_cloned(Rc::new(OrderItemMutable::from(item))),
                                                "assessment" => page.assessments.lock_mut().push_cloned(Rc::new(OrderItemMutable::from(item))),
                                                "plan" => page.plans.lock_mut().push_cloned(Rc::new(OrderItemMutable::from(item))),
                                                _  => page.notes.lock_mut().push_cloned(Rc::new(OrderItemMutable::from(item))),
                                            }
                                        }
                                    }
                                }
                                page.changed.set_neq(true);
                            }
                            Err(e) => {
                                app.alert_app_error(&e).await;
                            }
                        }
                    }
                    Some(VisitTypeId::OpdEr(_, _))
                    | Some(VisitTypeId::Visit(_))
                    | None => {}
                }
            }),
        )
    }

    fn new_objective(&self) -> Mutable<String> {
        let mut lock = self.objectives.lock_mut();
        match lock.last() {
            Some(item) => {
                let has_detail = item.order_item_detail.lock_ref().is_empty();
                if has_detail {
                    item.order_item_detail.clone()
                } else {
                    let item = OrderItemMutable::new("objective", self.pre_order_master_id.get());
                    let objective = item.order_item_detail.clone();
                    self.focused.set_neq(Some(item.id));
                    lock.push_cloned(item);
                    objective
                }
            }
            None => {
                let item = OrderItemMutable::new("objective", self.pre_order_master_id.get());
                let objective = item.order_item_detail.clone();
                self.focused.set_neq(Some(item.id));
                lock.push_cloned(item);
                objective
            }
        }
    }

    pub fn render(
        page: Rc<Self>,
        parent_show_progress_note_input: Mutable<bool>,
        parent_show_progress_note_auditor_input: Mutable<bool>,
        parent_edit_progress_note: Mutable<Option<Rc<ProgressNote>>>,
        parent_reload_progress_note: Mutable<bool>,
        app: Rc<App>,
    ) -> Dom {
        html!("div", {
            .future(map_ref!(
                let busy = app.loader_is_loading(),
                let buttons_loaded = page.buttons_loaded.signal() =>
                !busy && !buttons_loaded
            ).for_each(clone!(app, page => move |ready| {
                if ready {
                    Self::load_buttons(page.clone(), app.clone());
                    page.buttons_loaded.set(true);
                }
                async {}
            })))
            .future(page.to_scroll.signal().for_each(clone!(app, page => move |scroll| {
                if scroll {
                    app.scroll_into_view("addProgressNoteFormContainer");
                    page.to_scroll.set(false);
                }
                async {}
            })))
            .attr("id", "addProgressNoteFormContainer")
            .child(html!("div", {
                .style("position","relative")
                .child(html!("div", {
                    .class("mb-2")
                    .child_signal(map_ref! {
                        let is_auditor = page.is_auditor.signal(),
                        let is_today = page.current_date.signal_ref(|opt| opt.as_ref().map(|cd| cd.is_today).unwrap_or_default()),
                        let is_allow = page.allow_previous_signal(app.clone()),
                        let pre = page.pre_order_master_id.signal() =>
                            *is_today && !is_auditor && *is_allow && pre.is_none()
                        }.map(clone!(app, page => move |can_last_day| {
                        (can_last_day).then(|| {
                            html!("button" => HtmlButtonElement, {
                                .attr("type", "button")
                                //.attr("id", "add_previous_button")
                                .class(class::BTN_SM_T_GRAY)
                                .children([
                                    html!("i", {.class(class::FA_PLUS_L)}),
                                    html!("i", {.class(class::FA_BOLT_L)}),
                                    html!("i", {.class(class::FA_CLOCK_L_ROTATE)}),
                                ])
                                .text(" Last Day")
                                .apply(mixins::click_with_loader_checked(clone!(app, page => move || {
                                    Self::load_progress_previous(page.clone(), app.clone());
                                }), app.state()))
                            })
                        })
                    })))
                    .child_signal(page.pre_order_master_id.signal_cloned().map(clone!(page => move |opt| {
                        opt.is_none().then(|| {
                            html!("div", {
                                //.attr("id", "problem-list")
                                .class("mb-1")
                                .children([
                                    html!("span", {
                                        .class("fw-bold")
                                        //.attr("id", "progressNoteForm-problem-list-label")
                                        .text("Problem List")
                                    }),
                                    html!("button", {
                                        .attr("type", "button")
                                        .class(class::BTN_SM_RT_BLUE)
                                        .child(html!("i", {.class(class::FA_PLUS)}))
                                        .event(clone!(page => move |_: events::Click| {
                                            let item = OrderItemMutable::new("problem-list", page.pre_order_master_id.get());
                                            page.focused.set_neq(Some(item.id));
                                            page.problem_list.lock_mut().push_cloned(item);
                                        }))
                                        // onclick_add_button_progressNoteForm(event, 'problem-list')
                                    }),
                                    html!("div", {
                                        //.attr("id", "progressNoteForm-problem-list-input-div")
                                        .children_signal_vec(page.problem_list.signal_vec_cloned().map(clone!(page => move |problem| {
                                            html!("div", {
                                                .class(class::BOX_ROUND_P1_T)
                                                .class("bg-info-subtle")
                                                .children([
                                                    html!("label", {.class("fw-bold").text("Problem")}),
                                                    html!("div", {
                                                        .class(class::INPUT_GROUP_T)
                                                        .children([
                                                            html!("textarea" => HtmlTextAreaElement, {
                                                                .class(class::FORM_CTRL_SM)
                                                                .apply(mixins::textarea_value_auto_expand(problem.order_item_detail.clone(), page.changed.clone()))
                                                                .event(clone!(page, problem => move |_: events::Focus| {
                                                                    page.focused.set_neq(Some(problem.id));
                                                                }))
                                                            }),
                                                            html!("button", {
                                                                .class(class::BTN_SM_RED)
                                                                .attr("type", "button")
                                                                .child(html!("i", {.class(class::FA_MINUS)}))
                                                                .event(clone!(page, problem => move |_: events::Click| {
                                                                    page.problem_list.lock_mut().retain(|x| *x != problem);
                                                                    page.changed.set_neq(true);
                                                                }))
                                                            }),
                                                        ])
                                                    }),
                                                    html!("label", {.text("Problem Note")}),
                                                    html!("div", {
                                                        .child(html!("textarea" => HtmlTextAreaElement, {
                                                            .class(class::FORM_CTRL_SM)
                                                            .apply(mixins::textarea_value_auto_expand(problem.order_item_detail_2.clone(), page.changed.clone()))
                                                            .event(clone!(page => move |_: events::Focus| {
                                                                page.focused.set_neq(Some(problem.id));
                                                            }))
                                                        }))
                                                    })
                                                ])
                                            })
                                        })))
                                    }),
                                ])
                            })
                        })
                    })))
                    .children([
                        html!("div", {
                            //.attr("id", "note")
                            .class("mb-1")
                            .children([
                                html!("span", {
                                    .class("fw-bold")
                                    //.attr("id", "progressNoteForm-note-label")
                                    .text_signal(page.is_auditor.signal_cloned().map(|is_auditor| if is_auditor {"Auditor Note"} else {"Note"}))
                                }),
                                html!("button", {
                                    .attr("type", "button")
                                    .class(class::BTN_SM_RT_BLUE)
                                    .child(html!("i", {.class(class::FA_PLUS)}))
                                    .event(clone!(page => move |_: events::Click| {
                                        let item = OrderItemMutable::new("note", page.pre_order_master_id.get());
                                        page.focused.set_neq(Some(item.id));
                                        page.notes.lock_mut().push_cloned(item);
                                    }))
                                }),
                                html!("div", {
                                    //.attr("id", "progressNoteForm-note-input-div")
                                    .children_signal_vec(page.notes.signal_vec_cloned().map(clone!(page => move |note| {
                                        html!("div", {
                                            .child(html!("div", {
                                                .class(class::INPUT_GROUP_T)
                                                .children([
                                                    html!("textarea" => HtmlTextAreaElement, {
                                                        .class(class::FORM_CTRL_SM)
                                                        .attr("id", &["textarea-", &note.id.to_string()].concat())
                                                        .apply(mixins::textarea_value_auto_expand(note.order_item_detail.clone(), page.changed.clone()))
                                                        .event(clone!(page, note => move |_: events::Focus| {
                                                            page.focused.set_neq(Some(note.id));
                                                        }))
                                                    }),
                                                    html!("button", {
                                                        .class(class::BTN_SM_RED)
                                                        .attr("type", "button")
                                                        .child(html!("i", {.class(class::FA_MINUS)}))
                                                        .event(clone!(page => move |_: events::Click| {
                                                            page.notes.lock_mut().retain(|x| *x != note);
                                                            page.changed.set_neq(true);
                                                        }))
                                                    }),
                                                ])
                                            }))
                                        })
                                    })))
                                }),
                            ])
                            .child_signal(page.is_auditor.signal_cloned().map(clone!(app, page => move |is_auditor| {
                                (!is_auditor).then(|| {
                                    html!("div", {
                                        //.attr("id", "note_option")
                                        .children_signal_vec(page.note_buttons.signal_vec_cloned()
                                            .map(clone!(app, page => move |btn| InsertTextAreaButton::render(btn, page.notes.clone(), page.focused.clone(), page.changed.clone(), app.clone()))
                                        ))
                                    })
                                })
                            })))
                        }),
                    ])
                }))
                .child_signal(page.is_auditor.signal_cloned().map(clone!(app, page => move |is_auditor| {
                    (!is_auditor).then(|| {
                        html!("div", {
                            .children([
                                // S: Subjective
                                html!("div", {
                                    //.attr("id", "subjective")
                                    .class("mb-1")
                                    .children([
                                        html!("span", {
                                            .class("fw-bold")
                                            //.attr("id", "progressNoteForm-subjective-label")
                                            .text("Subjective")
                                        }),
                                        html!("button", {
                                            .attr("type", "button")
                                            .class(class::BTN_SM_RT_BLUE)
                                            .child(html!("i", {.class(class::FA_PLUS)}))
                                            .event(clone!(page => move |_: events::Click| {
                                                let item = OrderItemMutable::new("subjective", page.pre_order_master_id.get());
                                                page.focused.set_neq(Some(item.id));
                                                page.subjectives.lock_mut().push_cloned(item);
                                            }))
                                        }),
                                        html!("div", {
                                            //.attr("id", "progressNoteForm-subjective-input-div")
                                            .children_signal_vec(page.subjectives.signal_vec_cloned().map(clone!(page => move |subjective| {
                                                html!("div", {
                                                    .child(html!("div", {
                                                        .class(class::INPUT_GROUP_T)
                                                        .children([
                                                            html!("textarea" => HtmlTextAreaElement, {
                                                                .class(class::FORM_CTRL_SM)
                                                                .attr("id", &["textarea-", &subjective.id.to_string()].concat())
                                                                .apply(mixins::textarea_value_auto_expand(subjective.order_item_detail.clone(), page.changed.clone()))
                                                                .event(clone!(page, subjective => move |_: events::Focus| {
                                                                    page.focused.set_neq(Some(subjective.id));
                                                                }))
                                                            }),
                                                            html!("button", {
                                                                .class(class::BTN_SM_RED)
                                                                .attr("type", "button")
                                                                .child(html!("i", {.class(class::FA_MINUS)}))
                                                                .event(clone!(page => move |_: events::Click| {
                                                                    page.subjectives.lock_mut().retain(|x| *x != subjective);
                                                                    page.changed.set_neq(true);
                                                                }))
                                                            }),
                                                        ])
                                                    }))
                                                })
                                            })))
                                        }),
                                        html!("div", {
                                            //.attr("id", "subjective_option")
                                            .children_signal_vec(page.subjective_buttons.signal_vec_cloned()
                                                .map(clone!(app, page => move |btn| InsertTextAreaButton::render(btn, page.subjectives.clone(), page.focused.clone(), page.changed.clone(), app.clone()))
                                            ))
                                        }),
                                    ])
                                }),
                                // O: Objective
                                html!("div", {
                                    //.attr("id", "objective")
                                    .class("mb-1")
                                    .children([
                                        html!("span", {
                                            .class("fw-bold")
                                            //.attr("id", "progressNoteForm-objective-label")
                                            .text("Objective")
                                        }),
                                        html!("button", {
                                            .attr("type", "button")
                                            .class(class::BTN_SM_RT_BLUE)
                                            .child(html!("i", {.class(class::FA_PLUS)}))
                                            .event(clone!(page => move |_: events::Click| {
                                                let item = OrderItemMutable::new("objective", page.pre_order_master_id.get());
                                                page.focused.set_neq(Some(item.id));
                                                page.objectives.lock_mut().push_cloned(item);
                                            }))
                                        }),
                                    ])
                                    .child_signal(map_ref!{
                                        let is_allow = page.allow_vs_selector_signal(app.clone()),
                                        let not_pre_order = page.pre_order_master_id.signal_ref(|opt| opt.is_none()) =>
                                        *is_allow && *not_pre_order
                                    }.map(clone!(page => move |ready| {
                                        ready.then(|| {
                                            html!("button", {
                                                .attr("type", "button")
                                                .class(class::BTN_SM_RT_BLUE)
                                                .attr("data-bs-toggle", "modal")
                                                .attr("data-bs-target", "#vsSelectorModal")
                                                .child(html!("i", {.class(class::FA_HEARTBEAT)}))
                                                .event(clone!(page => move |_: events::Click| {
                                                    page.vs_selector_modal.set(Some(VsSelector::new(
                                                        false,
                                                        page.patient.clone(),
                                                        page.new_objective(),
                                                        page.changed.clone(),
                                                    )));
                                                }))
                                            })
                                        })
                                    })))
                                    .apply_if(app.endpoint_is_allow(&Method::GET, &EndPoint::LabHead, false), |dom| dom
                                        .child_signal(page.pre_order_master_id.signal_cloned().map(clone!(page => move |opt| {
                                            opt.is_none().then(|| {
                                                html!("button", {
                                                    .attr("type", "button")
                                                    .class(class::BTN_SM_RT_BLUE)
                                                    .attr("data-bs-toggle", "modal")
                                                    .attr("data-bs-target", "#labSelectorModal")
                                                    .child(html!("i", {.class(class::FA_FLASK)}))
                                                    .event(clone!(page => move |_: events::Click| {
                                                        page.lab_selector_modal.set(Some(LabSelector::new(
                                                            false,
                                                            page.patient.clone(),
                                                            page.new_objective(),
                                                            page.changed.clone(),
                                                        )));
                                                    }))
                                                })
                                            })
                                        })))
                                    )
                                    .children([
                                        html!("div", {
                                            //.attr("id", "progressNoteForm-objective-input-div")
                                            .children_signal_vec(page.objectives.signal_vec_cloned().map(clone!(page => move |objective| {
                                                html!("div", {
                                                    .class("order-input-group")
                                                    .child(html!("div", {
                                                        .class(class::INPUT_GROUP_T)
                                                        .children([
                                                            html!("textarea" => HtmlTextAreaElement, {
                                                                .class(class::FORM_CTRL_SM)
                                                                .attr("id", &["textarea-", &objective.id.to_string()].concat())
                                                                .apply(mixins::textarea_value_auto_expand(objective.order_item_detail.clone(), page.changed.clone()))
                                                                .event(clone!(page, objective => move |_: events::Focus| {
                                                                    page.focused.set_neq(Some(objective.id));
                                                                }))
                                                            }),
                                                            html!("button", {
                                                                .class(class::BTN_SM_RED)
                                                                .attr("type", "button")
                                                                .child(html!("i", {.class(class::FA_MINUS)}))
                                                                .event(clone!(page => move |_: events::Click| {
                                                                    page.objectives.lock_mut().retain(|x| *x != objective);
                                                                    page.changed.set_neq(true);
                                                                }))
                                                            }),
                                                        ])
                                                    }))
                                                })
                                            })))
                                        }),
                                        html!("div", {
                                            //.attr("id", "objective_option")
                                            .children_signal_vec(page.objective_buttons.signal_vec_cloned()
                                                .map(clone!(app, page => move |btn| InsertTextAreaButton::render(btn, page.objectives.clone(), page.focused.clone(), page.changed.clone(), app.clone()))
                                            ))
                                        }),
                                    ])
                                }),
                            ])
                            .child_signal(map_ref!{
                                let id_opt = page.pre_order_master_id.signal_cloned(),
                                let vnan_is_pre_opt = page.patient.signal_ref(|opt| opt.as_ref().map(|pt| pt.visit_type.vnan_and_is_pre_admit_owned())) =>
                                (id_opt.is_none(), vnan_is_pre_opt.clone())
                            }.map(clone!(app, page => move |(id_is_none, vnan_is_pre_opt)| {
                                let (vnan, is_pre_admit) = if let Some((s, b)) = vnan_is_pre_opt {
                                    (Some(s), b)
                                } else {
                                    (None, false)
                                };
                                (id_is_none && app.endpoint_is_allow(&Method::POST, &EndPoint::ImageUsage, is_pre_admit)).then(|| {
                                    // Image uploader
                                    html!("div", {
                                        .class("mb-2")
                                        // POST `EndPoint::ImageUsage`
                                        .child(ImageCpn::render("170px", ImageCpn::new_returning(
                                            page.image_callback.clone(),
                                            page.patient.clone(),
                                            vnan,
                                            "PROGRESS-NOTE-OBJECTIVE",
                                        ), app.clone()))
                                    })
                                })
                            })))
                            .children([
                                // A: Assessment
                                html!("div", {
                                    //.attr("id", "assessment")
                                    .class("mb-1")
                                    .children([
                                        html!("span", {
                                            .class("fw-bold")
                                            //.attr("id", "progressNoteForm-assessment-label")
                                            .text("Assessment")
                                        }),
                                        html!("button", {
                                            .attr("type", "button")
                                            .class(class::BTN_SM_RT_BLUE)
                                            .child(html!("i", {.class(class::FA_PLUS)}))
                                            .event(clone!(page => move |_: events::Click| {
                                                let item = OrderItemMutable::new("assessment", page.pre_order_master_id.get());
                                                page.focused.set_neq(Some(item.id));
                                                page.assessments.lock_mut().push_cloned(item);
                                            }))
                                        }),
                                        html!("div", {
                                            //.attr("id", "progressNoteForm-assessment-input-div")
                                            .children_signal_vec(page.assessments.signal_vec_cloned().map(clone!(page => move |assessment| {
                                                html!("div", {
                                                    .class("order-input-group")
                                                    .child(html!("div", {
                                                        .class(class::INPUT_GROUP_T)
                                                        .children([
                                                            html!("textarea" => HtmlTextAreaElement, {
                                                                .class(class::FORM_CTRL_SM)
                                                                .attr("id", &["textarea-", &assessment.id.to_string()].concat())
                                                                .apply(mixins::textarea_value_auto_expand(assessment.order_item_detail.clone(), page.changed.clone()))
                                                                .event(clone!(page, assessment => move |_: events::Focus| {
                                                                    page.focused.set_neq(Some(assessment.id));
                                                                }))
                                                            }),
                                                            html!("button", {
                                                                .class(class::BTN_SM_RED)
                                                                .attr("type", "button")
                                                                .child(html!("i", {.class(class::FA_MINUS)}))
                                                                .event(clone!(page => move |_: events::Click| {
                                                                    page.assessments.lock_mut().retain(|x| *x != assessment);
                                                                    page.changed.set_neq(true);
                                                                }))
                                                            }),
                                                        ])
                                                    }))
                                                })
                                            })))
                                        }),
                                        html!("div", {
                                            //.attr("id", "assessment_option")
                                            .children_signal_vec(page.assessment_buttons.signal_vec_cloned()
                                                .map(clone!(app, page => move |btn| InsertTextAreaButton::render(btn, page.assessments.clone(), page.focused.clone(), page.changed.clone(), app.clone()))
                                            ))
                                        }),
                                    ])
                                }),
                                // P: Plan
                                html!("div", {
                                    //.attr("id", "plan")
                                    .class("mb-1")
                                    .children([
                                        html!("span", {
                                            .class("fw-bold")
                                            //.attr("id", "progressNoteForm-plan-label")
                                            .text("Plan")
                                        }),
                                        html!("button", {
                                            .attr("type", "button")
                                            .class(class::BTN_SM_RT_BLUE)
                                            .child(html!("i", {.class(class::FA_PLUS)}))
                                            .event(clone!(page => move |_: events::Click| {
                                                let item = OrderItemMutable::new("plan", page.pre_order_master_id.get());
                                                page.focused.set_neq(Some(item.id));
                                                page.plans.lock_mut().push_cloned(item);
                                            }))
                                        }),
                                        html!("div", {
                                            //.attr("id", "progressNoteForm-plan-input-div")
                                            .children_signal_vec(page.plans.signal_vec_cloned().map(clone!(page => move |plan| {
                                                html!("div", {
                                                    .class("order-input-group")
                                                    .child(html!("div", {
                                                        .class(class::INPUT_GROUP_T)
                                                        .children([
                                                            html!("textarea" => HtmlTextAreaElement, {
                                                                .class(class::FORM_CTRL_SM)
                                                                .attr("id", &["textarea-", &plan.id.to_string()].concat())
                                                                .apply(mixins::textarea_value_auto_expand(plan.order_item_detail.clone(), page.changed.clone()))
                                                                .event(clone!(page, plan => move |_: events::Focus| {
                                                                    page.focused.set_neq(Some(plan.id));
                                                                }))
                                                            }),
                                                            html!("button", {
                                                                .class(class::BTN_SM_RED)
                                                                .attr("type", "button")
                                                                .child(html!("i", {.class(class::FA_MINUS)}))
                                                                .event(clone!(page => move |_: events::Click| {
                                                                    page.plans.lock_mut().retain(|x| *x != plan);
                                                                    page.changed.set_neq(true);
                                                                }))
                                                            }),
                                                        ])
                                                    }))
                                                })
                                            })))
                                        }),
                                        html!("div", {
                                            //.attr("id", "plan_option")
                                            .children_signal_vec(page.plan_buttons.signal_vec_cloned()
                                                .map(clone!(app, page => move |btn| InsertTextAreaButton::render(btn, page.plans.clone(), page.focused.clone(), page.changed.clone(), app.clone()))
                                            ))
                                        }),
                                    ])
                                }),
                            ])
                        })
                    })
                })))
                // custom progress_note_time
                .child(html!("div", {
                    .class(class::FLEX_FIX_B2)
                    .child(html!("i", {
                        .class(class::FA_CLOCK)
                        .class("p-2")
                        .style("width","30px")
                        .event(clone!(page => move |_:events::Click| {
                            let empty_time = page.progress_note_time.lock_ref().is_empty();
                            if empty_time {
                                page.progress_note_time.set(js_now().time().js_string());
                            } else {
                                page.progress_note_time.set(String::new());
                            }
                        }))
                    }))
                    .child_signal(page.progress_note_time.signal_cloned().map(clone!(page => move |order_time| {
                        (!order_time.is_empty()).then(|| {
                            doms::time_picker(
                                page.progress_note_time.clone(),
                                Mutable::new(true), always(false), None,
                                |d| d.style("max-width", "120px"),
                                |d| d.class("form-control-sm"),
                                |d| d.class("form-control-sm"),
                                |s| s, always(None),
                            )
                            // html!("input" => HtmlInputElement, {
                            //     .attr("type", "time")
                            //     .class(class::FORM_CTRL_SM)
                            //     .style("max-width", "120px")
                            //     .apply(mixins::string_value_end(page.progress_note_time.clone(), Mutable::new(true)))
                            // })
                        })
                    })))
                }))
                .children([
                    html!("button", {
                        .attr("type", "button")
                        .class(class::BTN_FR_B_GRAY)
                        .text("Cancel")
                        .event(clone!(parent_show_progress_note_input, parent_show_progress_note_auditor_input, parent_edit_progress_note => move |_: events::Click| {
                            parent_show_progress_note_input.set_neq(false);
                            parent_show_progress_note_auditor_input.set_neq(false);
                            parent_edit_progress_note.set(None);
                        }))
                    }),
                    html!("button" => HtmlButtonElement, {
                        .attr("type", "button")
                        .class(class::BTN_FR_LB)
                        .class_signal("btn-primary", page.changed.signal())
                        .class_signal("btn-secondary", not(page.changed.signal()))
                        .text("Save")
                        .apply(mixins::click_with_loader_checked_or_true_disable_signal(clone!(app, page => move || {
                            Self::submit(
                                page.clone(),
                                parent_show_progress_note_input.clone(),
                                parent_show_progress_note_auditor_input.clone(),
                                parent_edit_progress_note.clone(),
                                parent_reload_progress_note.clone(),
                                app.clone(),
                            );
                        }), not(page.changed.signal()), app.state()))
                    }),
                ])
            }))
            .child(html!("div", {
                .class("modal")
                .attr("id", "vsSelectorModal")
                .attr("role", "dialog")
                .attr("tabindex", "-1")
                .child_signal(page.vs_selector_modal.signal_cloned().map(clone!(app => move |opt| {
                    opt.as_ref().map(clone!(app => move |modal| {
                        VsSelector::render(modal.clone(), app)
                    })).or(Some(blank_modal()))
                })))
            }))
            .child(html!("div", {
                .class("modal")
                .attr("id", "labSelectorModal")
                .attr("role", "dialog")
                .attr("tabindex", "-1")
                .child_signal(page.lab_selector_modal.signal_cloned().map(clone!(app => move |opt| {
                    opt.as_ref().map(clone!(app => move |modal| {
                        LabSelector::render(modal.clone(), app)
                    })).or(Some(blank_modal()))
                })))
            }))
        })
    }

    fn submit(
        page: Rc<Self>,
        parent_show_progress_note_input: Mutable<bool>,
        parent_show_progress_note_auditor_input: Mutable<bool>,
        parent_edit_progress_note: Mutable<Option<Rc<ProgressNote>>>,
        parent_reload_progress_note: Mutable<bool>,
        app: Rc<App>,
    ) {
        let mut progress_note_items = Vec::new();
        progress_note_items.extend(page.problem_list.lock_ref().iter().filter_map(OrderItemMutable::save_as_progress_note));
        progress_note_items.extend(page.notes.lock_ref().iter().filter_map(OrderItemMutable::save_as_progress_note));
        progress_note_items.extend(page.subjectives.lock_ref().iter().filter_map(OrderItemMutable::save_as_progress_note));
        progress_note_items.extend(page.objectives.lock_ref().iter().filter_map(OrderItemMutable::save_as_progress_note));
        progress_note_items.extend(page.assessments.lock_ref().iter().filter_map(OrderItemMutable::save_as_progress_note));
        progress_note_items.extend(page.plans.lock_ref().iter().filter_map(OrderItemMutable::save_as_progress_note));

        if !progress_note_items.is_empty() {
            app.async_load(
                true,
                clone!(app => async move {
                    let result_opt = if let Some(pre_order_master_id) = page.pre_order_master_id.get() {
                        let now = js_now();
                        let form = PreProgressNoteSave {
                            pre_order_master_id,
                            progress_note_id: page.progress_note_id.get().unwrap_or_default(),
                            progress_note_date: now.date(),
                            progress_note_time: now.time(),
                            progress_note_owner_type: String::from("doctor"),
                            progress_note_doctor: page.progress_note_doctor.get_cloned(),
                            progress_note_items,
                        };
                        // POST `EndPoint::IpdPreOrderProgressNote`
                        Some(form.call_api_post(app.state()).await)
                    } else if let Some(pt) = page.patient.lock_ref().as_ref() {
                        // IPD: check current_date
                        // OPD-ER: current_date is null, use page.progress_note_date
                        let progress_note_for_past_date = page.current_date.lock_ref().as_ref().and_then(|cd| {
                            (!cd.is_today).then_some(cd.order_date)
                        }).or(date_8601(&page.progress_note_date.lock_ref()));
                        let form = ProgressNoteSave {
                            visit_type: pt.visit_type(),
                            progress_note_id: page.progress_note_id.get(),
                            progress_note_for_past_date,
                            progress_note_for_past_time: time_8601(&page.progress_note_time.lock_ref()),
                            progress_note_doctor: page.progress_note_doctor.get_cloned(),
                            progress_note_owner_type: page.view_by.get_cloned(),
                            progress_note_items,
                        };
                        // POST `EndPoint::IpdOrderProgressNote`
                        // POST `EndPoint::OpdErOrderProgressNote`
                        Some(form.call_api_post(app.state()).await)
                    } else {
                        None
                    };

                    if let Some(result) = result_opt {
                        match result {
                            Ok((id, responses)) => {
                                app.alert_execute_responses(&responses, clone!(app => async move {
                                    // app.alert("บันทึกข้อมูลสำเร็จ");
                                    // update images
                                    if id > 0 {
                                        // POST `EndPoint::ImageUsage`
                                        page.image_callback.lock_ref().post_images(ImageUsage::IpdProgressNote, id, app).await;
                                    }
                                    // clearing
                                    parent_show_progress_note_input.set_neq(false);
                                    parent_show_progress_note_auditor_input.set_neq(false);
                                    parent_edit_progress_note.set(None);
                                    parent_reload_progress_note.set(true);
                                })).await;
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
