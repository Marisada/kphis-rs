use dominator::{Dom, clone, events, html, text, with_node};
use futures_signals::{
    map_ref,
    signal::{Mutable, SignalExt, not},
    signal_vec::{MutableVec, SignalVecExt},
};
use std::rc::Rc;
use web_sys::{HtmlButtonElement, HtmlElement, HtmlTextAreaElement};

use kphis_model::{
    endpoint::EndPoint,
    fetch::Method,
    ipd::summary::{LabAlertData, SummaryNote, SummaryNoteSave},
};
use kphis_ui_app::App;

use kphis_ui_core::{class, mixins};
use kphis_util::{
    datetime::{date_th_opt, datetime_th, time_hm_opt},
    util::{opt_zero_none, str_some, zero_none},
};

use crate::modal::lab_history::LabHistory;

/// - GET `EndPoint::IpdSummaryNoteId`
/// - POST `EndPoint::IpdSummaryNoteId `(guarded, remove note-edit div)
/// - PATCH `EndPoint::IpdSummaryNoteId` (guarded, remove 'บันทึก' btn)
/// - DELETE `EndPoint::IpdSummaryNoteId` (guarded, remove 'ลบ' btn)
#[derive(Default)]
pub struct SummaryNoteCpn {
    loaded: Mutable<bool>,
    summary_id: Mutable<u32>,

    note_rescroll: Mutable<bool>,
    notes: MutableVec<Rc<SummaryNote>>,
    note_changed: Mutable<bool>,
    note_text: Mutable<String>,
    note_edit_text: Mutable<String>,
    note_edit_changed: Mutable<bool>,
}

impl SummaryNoteCpn {
    pub fn new(summary_id: u32) -> Rc<Self> {
        Rc::new(Self {
            summary_id: Mutable::new(summary_id),
            ..Default::default()
        })
    }

    fn load_note(page: Rc<Self>, app: Rc<App>) {
        if let Some(summary_id) = zero_none(page.summary_id.get()) {
            app.async_load(
                true,
                clone!(app => async move {
                    // GET `EndPoint::IpdSummaryNoteId`
                    match SummaryNote::call_api_get(summary_id, app.state()).await {
                        Ok(responses) => {
                            let mut lock = page.notes.lock_mut();
                            lock.clear();
                            lock.extend(responses.into_iter().map(Rc::new));
                        }
                        Err(e) => {
                            app.alert_app_error(&e).await;
                        }
                    }
                }),
            )
        }
    }

    fn save_note(page: Rc<Self>, app: Rc<App>) {
        if let (Some(summary_id), Some(note)) = (zero_none(page.summary_id.get()), str_some(page.note_text.get_cloned())) {
            app.async_load(
                true,
                clone!(app => async move {
                    let saver = SummaryNoteSave { note, ..Default::default() };
                    // POST `EndPoint::IpdSummaryNoteId`
                    match saver.call_api_save("POST", summary_id, app.state()).await {
                        Ok(response) => {
                            app.alert_execute_response(&response, async move {
                                // app.alert("บันทึกข้อมูลสำเร็จ");
                                page.note_text.set(String::new());
                                page.note_changed.set(false);
                                page.loaded.set(false);
                            }).await;
                        }
                        Err(e) => {
                            app.alert_app_error(&e).await;
                        }
                    }
                }),
            )
        }
    }

    fn update_note(summary_note_id: u32, page: Rc<Self>, app: Rc<App>) {
        if let (Some(summary_id), Some(note)) = (zero_none(page.summary_id.get()), str_some(page.note_edit_text.get_cloned())) {
            app.async_load(
                true,
                clone!(app => async move {
                    let saver = SummaryNoteSave { note_id: Some(summary_note_id), note };
                    // PATCH `EndPoint::IpdSummaryNoteId`
                    match saver.call_api_save("PATCH", summary_id, app.state()).await {
                        Ok(response) => {
                            app.alert_execute_response(&response, async move {
                                // app.alert("บันทึกข้อมูลสำเร็จ");
                                page.note_edit_text.set(String::new());
                                page.note_edit_changed.set(false);
                                page.loaded.set(false);
                            }).await;
                        }
                        Err(e) => {
                            app.alert_app_error(&e).await;
                        }
                    }
                }),
            )
        }
    }

    fn delete_note(summary_note_id: u32, page: Rc<Self>, app: Rc<App>) {
        if let Some(summary_id) = zero_none(page.summary_id.get()) {
            app.async_load(
                true,
                clone!(app => async move {
                    let saver = SummaryNoteSave { note_id: Some(summary_note_id), ..Default::default() };
                    // DELETE `EndPoint::IpdSummaryNoteId`
                    match saver.call_api_save("DELETE", summary_id, app.state()).await {
                        Ok(response) => {
                            app.alert_execute_response(&response, async move {
                                // app.alert("บันทึกข้อมูลสำเร็จ");
                                page.note_text.set(String::new());
                                page.note_changed.set(false);
                                page.loaded.set(false);
                            }).await;
                        }
                        Err(e) => {
                            app.alert_app_error(&e).await;
                        }
                    }
                }),
            )
        }
    }

    pub fn render(is_pre_admit: bool, page: Rc<Self>, app: Rc<App>) -> Dom {
        html!("div", {
            .future(map_ref!(
                let busy = app.loader_is_loading(),
                let loaded = page.loaded.signal() =>
                !busy && !loaded
            ).for_each(clone!(app, page => move |ready| {
                if ready {
                    Self::load_note(page.clone(), app.clone());
                    page.loaded.set(true);
                }
                async {}
            })))
            .class("col")
            .child(html!("div", {
                .class(class::CARD)
                .style("break-inside","avoid")
                .children([
                    html!("div", {
                        .class("card-header")
                        .child(html!("span", {
                            .class("fw-bold")
                            .text("AUDITOR / CODER / DOCTOR NOTES")
                        }))
                    }),
                    html!("div", {
                        .class("card-body")
                        .class(class::FLEX_COL)
                        .child(html!("div" => HtmlElement, {
                            .style("max-height","300px")
                            .style("overflow-y", "auto")
                            .with_node!(element => {
                                .future(page.note_rescroll.signal().for_each(clone!(page => move |rescroll| {
                                    if rescroll {
                                        element.set_scroll_top(element.scroll_height());
                                        page.note_rescroll.set(false);
                                    }
                                    async {}
                                })))
                            })
                            .child_signal(page.notes.signal_vec_cloned().len().map(clone!(app, page => move |notes_len| {
                                (notes_len > 0).then(|| {
                                    html!("ul", {
                                        .class(class::LIST_GROUP_FLUSH_OVFA)
                                        .children_signal_vec(page.notes.signal_vec_cloned().enumerate().map(clone!(app, page => move |(id, note)| {
                                            // only last item + same user can edit/delete
                                            let is_our_last_note = (id.get().unwrap_or_default() + 1) == notes_len
                                                && note.doctor.is_some()
                                                && app.doctor_code() == note.doctor;
                                            if is_our_last_note {
                                                page.note_edit_text.set_neq(note.note.clone().unwrap_or_default());
                                                page.note_edit_changed.set_neq(false);
                                            }
                                            // we set note_rescroll here to scroll in the next tick with the future above
                                            page.note_rescroll.set(true);
                                            let last_note_is_edit = Mutable::new(false);
                                            html!("li", {
                                                .class("list-group-item")
                                                .child_signal(last_note_is_edit.signal().map(clone!(page, note => move |is_edit| {
                                                    if is_edit && is_our_last_note {
                                                        Some(html!("textarea" => HtmlTextAreaElement, {
                                                            .class(class::FORM_CTRL_T)
                                                            .apply(mixins::textarea_value_auto_expand(page.note_edit_text.clone(), page.note_edit_changed.clone()))
                                                        }))
                                                    } else {
                                                        Some(html!("div", {
                                                            .style("white-space","pre-wrap")
                                                            .text(&note.note.clone().unwrap_or_default())
                                                        }))
                                                    }
                                                })))
                                                .child(html!("div", {
                                                    .class(class::SMALL_R)
                                                    .text(&note.doctor_name.clone().unwrap_or_default())
                                                    .text(" ")
                                                    .text(&datetime_th(&note.update_datetime))
                                                }))
                                                .apply_if(is_our_last_note, |dom| {
                                                    dom.child(html!("div", {
                                                        .class(class::TXT_R_B)
                                                        .apply_if(app.endpoint_is_allow(&Method::PATCH, &EndPoint::IpdSummaryNoteId, is_pre_admit), |dom| dom
                                                            .child_signal(last_note_is_edit.signal().map(clone!(app, page, note => move |is_edit| {
                                                                is_edit.then(|| {
                                                                    html!("button" => HtmlButtonElement, {
                                                                        .attr("type", "button")
                                                                        .class(class::BTN_SM_L_BLUE)
                                                                        .text("บันทึก")
                                                                        .apply(mixins::click_with_loader_checked_or_true_disable_signal(clone!(app, page, note => move || {
                                                                            Self::update_note(note.summary_note_id, page.clone(), app.clone());
                                                                        }), not(page.note_edit_changed.signal()), app.state()))
                                                                    })
                                                                })
                                                            })))
                                                        )
                                                        .children([
                                                            html!("button" => HtmlButtonElement, {
                                                                .attr("type", "button")
                                                                .class(class::BTN_SM_L)
                                                                .class_signal("btn-info", not(last_note_is_edit.signal()))
                                                                .class_signal("btn-secondary", last_note_is_edit.signal())
                                                                .text_signal(last_note_is_edit.signal().map(|is_edit| {
                                                                    if is_edit {
                                                                        "ยกเลิก"
                                                                    } else {
                                                                        "แก้ไข"
                                                                    }
                                                                }))
                                                                .event(clone!(last_note_is_edit => move |_:events::Click| {
                                                                    last_note_is_edit.set(!last_note_is_edit.get());
                                                                }))
                                                            }),
                                                        ])
                                                        .apply_if(app.endpoint_is_allow(&Method::DELETE, &EndPoint::IpdSummaryNoteId, is_pre_admit), |dom| dom
                                                            .child(html!("button" => HtmlButtonElement, {
                                                                .attr("type", "button")
                                                                .class(class::BTN_SM_RED)
                                                                .text("ลบ")
                                                                .apply(mixins::click_with_loader_checked(clone!(app, page, note => move || {
                                                                    Self::delete_note(note.summary_note_id, page.clone(), app.clone());
                                                                }), app.state()))
                                                            }))
                                                        )
                                                    }))
                                                })
                                            })
                                        })))
                                    })
                                })
                            })))
                            .apply_if(app.endpoint_is_allow(&Method::POST, &EndPoint::IpdSummaryNoteId, is_pre_admit), |dom| dom
                                .child(html!("div", {
                                    .class("mt-2")
                                    .children([
                                        html!("textarea" => HtmlTextAreaElement, {
                                            .class(class::FORM_CTRL_T)
                                            .apply(mixins::textarea_value_auto_expand(page.note_text.clone(), page.note_changed.clone()))
                                        }),
                                        html!("div", {
                                            .class("float-end")
                                            .child(html!("button" => HtmlButtonElement, {
                                                .attr("type", "button")
                                                .class("btn")
                                                .class_signal("btn-primary", page.note_changed.signal())
                                                .class_signal("btn-secondary", not(page.note_changed.signal()))
                                                .child(html!("i", {.class(class::FA_SAVE)}))
                                                .text(" บันทึก")
                                                .apply(mixins::click_with_loader_checked_or_true_disable_signal(clone!(app, page => move || {
                                                    Self::save_note(page.clone(), app.clone());
                                                }), not(page.note_changed.signal()), app.state()))
                                            }))
                                        })
                                    ])
                                }))
                            )
                        }))
                    }),
                ])
            }))
        })
    }
}

pub fn render_lab_alert(lab_alerts: MutableVec<Rc<LabAlertData>>, hn: Mutable<String>, lab_history_modal: Mutable<Option<Rc<LabHistory>>>, app: Rc<App>) -> Dom {
    html!("div", {
        .class("col")
        .child(html!("div", {
            .class(class::CARD)
            .style("break-inside","avoid")
            .children([
                html!("div", {
                    .class("card-header")
                    .children([
                        html!("span", {
                            .attr("role", "button")
                            .class("fw-bold")
                            .text("LAB ALERT")
                        }),
                        html!("div", {
                            .attr("role", "button")
                            .class(class::BADGE_WRAP_R_GRAY)
                            .style("cursor","help")
                            .attr("tabindex", "0")
                            .attr("title", &["เงื่อนไข\n- ", &app.lab_alerts().join("\n- ")].concat())
                            .text("เงื่อนไข")
                            .child(html!("i", {.class(class::FA_INFO).class("ms-1")}))
                        }),
                        html!("div", {
                            .attr("role", "button")
                            .class(class::BADGE_WRAP_R_GRAY)
                            .style("cursor","help")
                            .attr("tabindex", "0")
                            .attr("title", "ผล Lab บางรายการ อาจไม่ปรากฏผลในรายการนี้ ตามเงื่อนไขที่กำหนดไว้\nหากเป็นค่าที่ไม่สามารถประเมินได้ เช่น ผลที่เป็นรูปภาพหรือผลที่มีข้อความรวมกับตัวเลข")
                            .text("หมายเหตุ ")
                            .child(html!("i", {.class(class::FA_ALERT_RED).class("ms-1")}))
                        }),
                    ])
                }),
                html!("div", {
                    .class("card-body")
                    .style("max-height","600px")
                    .style("overflow-y","auto")
                    .child(html!("div", {
                        .style("white-space","pre")
                        .child(html!("ul", {
                            .class("dash")
                            .style("white-space","pre-wrap")
                            .children_signal_vec(lab_alerts.signal_vec_cloned().map(clone!(app => move |lab| {
                                html!("li", {
                                    .children([
                                        html!("span", {
                                            .text(&lab.lab_items_name_ref.clone().unwrap_or_default())
                                        }),
                                        text(&[": ", &lab.lab_order_result.clone().unwrap_or_default()].concat()),
                                    ])
                                    .attr("title", &[
                                        "Lab Item Name: ", &lab.lab_items_name_ref.clone().unwrap_or_default(),
                                        "\nLab Group: ", &lab.lab_items_group_name.clone().unwrap_or_default(),
                                        "\nค่าปกติ: ", &lab.lab_items_normal_value_ref.clone().unwrap_or_default(), " ", &lab.lab_items_unit.clone().unwrap_or_default(),
                                        "\nReceive Date: ", &date_th_opt(&lab.receive_date), " ", &time_hm_opt(&lab.receive_time),
                                        "\nReport Date: ", &date_th_opt(&lab.report_date), " ", &time_hm_opt(&lab.report_time)
                                    ].concat())
                                    .apply_if(app.endpoint_is_allow(&Method::GET, &EndPoint::LabItem, false), |dom| dom
                                        .attr("role","button")
                                        .attr("data-bs-toggle", "modal")
                                        .attr("data-bs-target", "#labHistoryModal")
                                        .event(clone!(hn, lab_history_modal => move |_:events::Click| {
                                            if let Some(lab_items_code) = opt_zero_none(lab.lab_items_code) {
                                                let lab_history = LabHistory::new(
                                                    hn.clone(),
                                                    lab_items_code,
                                                    &lab.lab_items_name_ref,
                                                    &lab.lab_items_unit,
                                                    &zero_none(lab.lab_order_number),
                                                );
                                                lab_history_modal.set(Some(lab_history));
                                            }
                                        }))
                                    )
                                })
                            })))
                        }))
                    }))
                }),
            ])
        }))
    })
}

pub fn render_problem_list(problem_lists: MutableVec<String>) -> Dom {
    html!("div", {
        .class("col")
        .child(html!("div", {
            .class(class::CARD)
            .style("break-inside","avoid")
            .children([
                html!("div", {
                    .class("card-header")
                    .child(html!("span", {
                        .class("fw-bold")
                        .text("PROBLEM LIST")
                    }))
                }),
                html!("div", {
                    .class("card-body")
                    .style("max-height","600px")
                    .style("overflow-y","auto")
                    .child(html!("div", {
                        //.attr("id", "problem_list_data")
                        .child(html!("ul", {
                            .class("dash")
                            .style("white-space","pre-wrap")
                            .children_signal_vec(problem_lists.signal_vec_cloned().map(|problem| {
                                html!("li", {.text(&problem)})
                            }))
                        }))
                    }))
                }),
            ])
        }))
    })
}
