// ipd-nurse-index-note-form.php

use dominator::{Dom, clone, html};
use futures_signals::signal::{Mutable, SignalExt};
use std::rc::Rc;
use web_sys::{HtmlButtonElement, HtmlTextAreaElement};

use kphis_model::{
    endpoint::EndPoint,
    fetch::Method,
    {ipd::index_note::IndexNote, user::permission::Permission},
};
use kphis_ui_app::App;
use kphis_ui_core::{class, doms, mixins};
use kphis_util::util::str_some;

/// - POST `EndPoint::IpdIndexNote` (guarded, remove Save btn)
/// - DELETE `EndPoint::IpdIndexNoteId` (guarded, remove Delete btn)
#[derive(Clone, Default)]
pub struct IndexNoteForm {
    nurse_index_note_id: Mutable<u32>,
    an: Mutable<String>,
    nurse_index_note: Mutable<String>,

    changed: Mutable<bool>,
}

impl IndexNoteForm {
    // pub fn new(an: &str) -> Rc<Self> {
    //     Rc::new(Self {
    //         an: Mutable::new(an.to_owned()),
    //         ..Default::default()
    //     })
    // }

    /// load struct to modal, ** this function doesn't fetch anything
    pub fn load(index_note: &IndexNote) -> Rc<Self> {
        Rc::new(Self {
            nurse_index_note_id: Mutable::new(index_note.nurse_index_note_id),
            an: Mutable::new(index_note.an.clone().unwrap_or_default()),
            nurse_index_note: Mutable::new(index_note.clone().nurse_index_note.unwrap_or_default()),
            ..Default::default()
        })
    }

    // ipd-nurse-index-note-save.php
    /// nurse_index_note_id == 0 then create new note<br>
    /// or else => edit exists note
    pub fn edit_index_plan(parent_reload: Option<Mutable<bool>>, modal: Rc<Self>, app: Rc<App>) {
        app.async_load(
            true,
            clone!(app, modal => async move {
                let index_note = IndexNote {
                    nurse_index_note_id: modal.nurse_index_note_id.get(),
                    an: str_some(modal.an.get_cloned()),
                    nurse_index_note: str_some(modal.nurse_index_note.get_cloned()),
                };
                // POST `EndPoint::IpdIndexNote`
                match index_note.call_api_post(app.state()).await {
                    Ok((id, response)) => {
                        if response.rows_affected > 0 {
                            modal.nurse_index_note_id.set_neq(id);
                            if let Some(loaded) = parent_reload {
                                loaded.set(true);
                            }
                        }
                    }
                    Err(e) => {
                        app.alert_app_error(&e).await;
                    }
                }
            }),
        )
    }

    // ipd-nurse-index-note-delete.php
    pub fn delete_index_note(parent_reload: Option<Mutable<bool>>, modal: Rc<Self>, app: Rc<App>) {
        app.async_load(
            true,
            clone!(app, modal => async move {
                if app.confirm("ยืนยันลบรายการ").await {
                    let nurse_index_note_id = modal.nurse_index_note_id.get();
                    if nurse_index_note_id > 0 {
                        // DELETE `EndPoint::IpdIndexNoteId`
                        match IndexNote::call_api_delete(nurse_index_note_id, app.state()).await {
                            Ok(response) => {
                                if response.rows_affected > 0 {
                                    if let Some(loaded) = parent_reload {
                                        loaded.set(true);
                                    }
                                }
                            }
                            Err(e) => {
                                app.alert_app_error(&e).await;
                            }
                        }
                    }
                }
            }),
        )
    }

    pub fn render(
        modal: Rc<Self>,
        // display: Mutable<Option<Rc<Self>>>,
        parent_reload: Option<Mutable<bool>>,
        app: Rc<App>,
    ) -> Dom {
        let is_pre_admit = app.is_pre_admit(&modal.an.lock_ref());

        html!("div", {
            .class(class::MODAL_DIALOG_LG)
            .attr("role", "document")
            .child(html!("div", {
                .class("modal-content")
                .children([
                    html!("div", {
                        .class("modal-header")
                        .children([
                            html!("h5", {
                                .class("modal-title")
                                .text("Note")
                            }),
                            doms::close_modal_x_btn(),
                        ])
                    }),
                    html!("div", {
                        .class("modal-body")
                        //.attr("id", "nurseIndexNoteFormModalBody")
                        .child(html!("div", {
                            //.attr("id", "index-note-form")
                            .children([
                                html!("div", {
                                    .class("mb-3")
                                    .children([
                                        html!("label", {
                                            .attr("for", "index-note-nurse-index-note")
                                            .text("Note")
                                        }),
                                        html!("textarea" => HtmlTextAreaElement, {
                                            .class("form-control")
                                            .attr("id", "index-note-nurse-index-note")
                                            .apply(mixins::textarea_value_auto_expand(modal.nurse_index_note.clone(), modal.changed.clone()))
                                        }),
                                    ])
                                }),
                            ])
                        }))
                    }),
                    html!("div", {
                        .class("modal-footer")
                        .child_signal(modal.nurse_index_note_id.signal_cloned().map(clone!(app, modal, parent_reload => move |id| {
                            (id > 0 && app.endpoint_is_allow(&Method::DELETE, &EndPoint::IpdIndexNoteId, is_pre_admit)).then(|| {
                                html!("button" => HtmlButtonElement, {
                                    .attr("type", "button")
                                    .class(class::BTN_LX_RED)
                                    //.attr("id", "index-note-delete-button")
                                    .child(html!("i", {.class(class::FA_TRASH)}))
                                    .text(" Delete")
                                    .apply(mixins::click_with_loader_checked(clone!(app, modal, parent_reload => move || {
                                        Self::delete_index_note(parent_reload.clone(), modal.clone(), app.clone());
                                    }), app.state()))
                                })
                            })
                        })))
                        .child_signal(modal.nurse_index_note_id.signal_cloned().map(clone!(app, modal, parent_reload => move |id| {
                            (app.endpoint_is_allow(&Method::POST, &EndPoint::IpdIndexNote, is_pre_admit)
                            && if id == 0 {
                                app.has_permission(Permission::IpdNurseIndexNoteAdd) || app.has_permission(Permission::OpdErNurseIndexNoteAdd)
                            } else {
                                app.has_permission(Permission::IpdNurseIndexNoteEdit) || app.has_permission(Permission::OpdErNurseIndexNoteEdit)
                            }).then(|| {
                                html!("button" => HtmlButtonElement, {
                                    .attr("type", "button")
                                    .class(class::BTN_BLUE)
                                    .child(html!("i", {.class(class::FA_SAVE)}))
                                    .attr("data-bs-dismiss", "modal")
                                    .text(" Save")
                                    .apply(mixins::click_with_loader_checked_or_true_disable_signal(clone!(app, modal, parent_reload => move || {
                                        Self::edit_index_plan(parent_reload.clone(), modal.clone(), app.clone());
                                    }), modal.an.signal_cloned().map(|an| an.is_empty()), app.state()))
                                })
                            })
                        })))
                        .child(html!("button", {
                            .attr("type", "button")
                            .class(class::BTN_GRAY)
                            .attr("data-bs-dismiss", "modal")
                            .child(html!("i", {.class(class::FA_X)}))
                            .text(" Cancel")
                        }))
                    }),
                ])
            }))
        })
    }
}
