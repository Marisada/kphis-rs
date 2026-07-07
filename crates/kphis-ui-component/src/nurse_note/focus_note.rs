// ipd-nurse-focus-note.php
// opd-er-nurse-focus-note.php

use dominator::{Dom, clone, html};
use futures_signals::{
    map_ref,
    signal::{Mutable, Signal, SignalExt},
    signal_vec::MutableVec,
};
use std::rc::Rc;

use kphis_model::{
    endpoint::EndPoint,
    fetch::Method,
    focus_list::FocusList,
    focus_note::FocusNote,
    ipd::tmp::{TmpDlc, TmpParams},
    patient_info::PatientInfo,
    user::permission::Permission,
};
use kphis_ui_app::App;

use crate::nurse_note::{note_form::NurseNoteFormCpn, note_list::NoteListCpn};

/// - GET `EndPoint::IpdFocusListAn` (NoteListCpn)
/// - GET `EndPoint::OpdErFocusListId` (NoteListCpn)
/// - GET `EndPoint::IpdFocusNoteAn` (NoteListCpn)
/// - GET `EndPoint::OpdErFocusNoteId` (NoteListCpn)
/// - GET `EndPoint::IpdTmpDlc` (NurseNoteFormCpn, guarded, remove form)
/// - GET `EndPoint::IpdTmpIntvt` (NurseNoteFormCpn, guarded, remove form)
#[derive(Clone, Default)]
pub struct FocusNoteCpn {
    loaded_focus_list: Mutable<bool>,
    loaded_dlcs: Mutable<bool>,
    // for calling from NurseNoteFormCpn (if needed)
    load_dlcs: Mutable<bool>,
    // shared data with NurseNoteFormCpn (if needed)
    focus_list: MutableVec<Rc<FocusList>>,
    // shared data with NurseNoteFormCpn (if needed)
    dlcs: Mutable<Vec<TmpDlc>>,

    // view_by: Mutable<String>,
    patient: Mutable<Option<Rc<PatientInfo>>>,

    focus_note: Mutable<Option<Rc<FocusNote>>>,

    reload_list: Mutable<bool>,
}

impl FocusNoteCpn {
    pub fn new(
        patient: Mutable<Option<Rc<PatientInfo>>>,
        // view_by: Mutable<String>,
    ) -> Rc<Self> {
        Rc::new(Self {
            patient,
            reload_list: Mutable::new(true),
            // view_by,
            ..Default::default()
        })
    }

    fn is_ipd_and_is_pre_admit(&self) -> impl Signal<Item = (bool, bool)> + use<> {
        self.patient.signal_ref(|opt| opt.as_ref().map(|pt| pt.visit_type.is_ipd_and_is_pre_admit()).unwrap_or_default())
    }

    pub fn load_dlcs(page: Rc<Self>, app: Rc<App>) {
        app.async_load(
            true,
            clone!(app, page => async move {
                let params = TmpParams::default();
                // GET `EndPoint::IpdTmpDlc`
                match TmpDlc::call_api_get(&params, app.state()).await {
                    Ok(responses) => {
                        page.dlcs.set(responses);
                    }
                    Err(e) => {
                        app.alert_app_error(&e).await;
                    }
                }
            }),
        )
    }

    pub fn render(page: Rc<Self>, app: Rc<App>) -> Dom {
        let allow_form = app.endpoint_is_allow(&Method::GET, &EndPoint::IpdTmpDlc, false) && app.endpoint_is_allow(&Method::GET, &EndPoint::IpdTmpIntvt, false);

        html!("div", {
            .future(map_ref!{
                let busy = app.loader_is_loading(),
                let loaded = page.loaded_dlcs.signal(),
                let load = page.load_dlcs.signal() =>
                (*loaded, !busy && *load)
            }.for_each(clone!(app, page => move |(loaded, ready)| {
                if loaded {
                    page.load_dlcs.set_neq(false);
                } else if ready {
                    Self::load_dlcs(page.clone(), app.clone());
                    page.loaded_dlcs.set(true);
                    page.load_dlcs.set(false);
                }
                async {}
            })))
            .attr("role", "tabpanel")
            .attr("aria-labelledby", "nav-nurse-progress-note-tab")
            .class("container-fluid")
            .child(html!("div", {
                .class("row")
                .child(html!("div", {
                    .style_signal("width", app.aside_prev_percent.signal().map(move |percent| {
                        if allow_form {
                            if percent == 100.0 {
                                "60%"
                            } else {
                                "calc(100% - 450px)"
                            }
                        } else {
                            "100%"
                        }
                    }))
                    .style("overflow-x","auto")
                    .child(NoteListCpn::render(NoteListCpn::new(
                        page.focus_list.clone(),
                        page.loaded_focus_list.clone(),
                        page.patient.clone(),
                        page.reload_list.clone(),
                    ), app.clone(), Some(page.focus_note.clone())))
                }))
                .apply_if(allow_form, |dom| dom
                    .child_signal(map_ref!{
                        let (is_ipd, is_pre_admit) = page.is_ipd_and_is_pre_admit(),
                        let is_no_fcnote = page.focus_note.signal_ref(|opt| opt.is_none()) =>
                        (*is_ipd, *is_pre_admit, *is_no_fcnote)
                    }.map(clone!(app, page => move |(is_ipd, is_pre_admit, is_no_fcnote)| {
                        (match (is_ipd, is_no_fcnote) {
                            (true, true) => app.has_permission(Permission::IpdNurseNoteAdd) || (is_pre_admit && app.has_permission(Permission::OpdErNurseNoteAdd)),
                            (true, false) => app.has_permission(Permission::IpdNurseNoteEdit) || (is_pre_admit && app.has_permission(Permission::OpdErNurseNoteEdit)),
                            (false, true) => app.has_permission(Permission::OpdErNurseNoteAdd),
                            (false, false) => app.has_permission(Permission::OpdErNurseNoteEdit),
                        }).then(|| {
                            html!("div", {
                                .style_signal("width", app.aside_prev_percent.signal().map(|percent| {
                                    if percent == 100.0 {
                                        "40%"
                                    } else {
                                        "450px"
                                    }
                                }))
                                .child(NurseNoteFormCpn::render(NurseNoteFormCpn::new(
                                    page.focus_list.clone(),
                                    page.dlcs.clone(),
                                    page.load_dlcs.clone(),
                                    page.patient.clone(),
                                    page.focus_note.clone(),
                                    page.reload_list.clone(),
                                ), app.clone()))
                            })
                        })
                    })))
                )
            }))
        })
    }
}
