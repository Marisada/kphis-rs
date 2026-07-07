// ipd-nurse-focus-list.php
// opd-er-nurse-focus-list.php

use dominator::{Dom, clone, events, html, with_node};
use futures_signals::{
    map_ref,
    signal::{Mutable, Signal, SignalExt, always, not},
    signal_vec::{MutableVec, SignalVecExt},
};
use std::rc::Rc;
use wasm_bindgen::JsCast;
use web_sys::{HtmlButtonElement, HtmlInputElement, HtmlSelectElement, HtmlTextAreaElement};

use kphis_model::{
    app::VisitTypeId,
    endpoint::EndPoint,
    fetch::Method,
    focus_list::{FocusList, FocusListParams, FocusListSave, FocusListSaveParams},
    ipd::tmp::{TmpFocus, TmpGoal, TmpGroup, TmpParams, TmpSubGroup},
    patient_info::PatientInfo,
    report::{SystemReport, TypstReport},
    user::permission::Permission,
};
use kphis_ui_app::App;
use kphis_ui_core::{binding::NiceSelect, class, doms, mixins};
use kphis_util::{
    datetime::{JsTime, date_8601, js_now, time_8601},
    util::{set_day_last, set_days_next, str_some, zero_none},
};

use crate::gadget::pdf_button::PdfButtons;

/// - GET `EndPoint::IpdFocusListAn`
/// - GET `EndPoint::OpdErFocusListId`
/// - GET `EndPoint::IpdTmpGroup` (guarded, remove form, 'Add focus list' btn)
/// - GET `EndPoint::IpdTmpSubgroup` (guarded, remove form, 'Add focus list' btn)
/// - GET `EndPoint::IpdTmpFocus` (guarded, remove form, 'Add focus list' btn)
/// - GET `EndPoint::IpdTmpGoal` (guarded, remove form, 'Add focus list' btn)
/// - POST `EndPoint::IpdFocusListAn` (guarded, remove 'บันทึกข้อมูล' btn)
/// - POST `EndPoint::OpdErFocusListId` (guarded, remove 'บันทึกข้อมูล' btn)
/// - DELETE `EndPoint::IpdFocusListAn` (guarded, remove 'ลบ' btn)
/// - DELETE `EndPoint::OpdErFocusListId` (guarded, remove 'ลบ' btn)
#[derive(Clone, Default)]
pub struct FocusListCpn {
    patient: Mutable<Option<Rc<PatientInfo>>>,
    version: Mutable<i32>,

    changed: Mutable<bool>,
    checked: Mutable<bool>,

    start_date: Mutable<String>,
    end_date: Mutable<String>,
    search_status: Mutable<String>, // "0","1","2"
    focus_list: MutableVec<Rc<FocusList>>,

    form_show: Mutable<bool>,
    form_changed: Mutable<bool>,
    used: Mutable<bool>,
    fclist_id: Mutable<u32>,
    fclist_stdate: Mutable<String>,
    fclist_sttime: Mutable<String>,
    fclist_enddate: Mutable<String>,
    fclist_endtime: Mutable<String>,
    fclist_status: Mutable<String>,

    loaded_groups: Mutable<bool>,
    group_select_redraw: Mutable<bool>,
    groups: Mutable<Vec<TmpGroup>>,
    smp_id: Mutable<String>,

    loaded_subgroups: Mutable<bool>,
    subgroup_select_redraw: Mutable<bool>,
    subgroups: Mutable<Vec<TmpSubGroup>>,
    subgroup: Mutable<String>,

    loaded_focus: Mutable<bool>,
    focuses: Mutable<Vec<TmpFocus>>,
    focus_select_redraw: Mutable<bool>,
    focus_id: Mutable<String>,
    focus_text: Mutable<String>,

    loaded_goal: Mutable<bool>,
    goals: Mutable<Vec<TmpGoal>>,
    goal_ids: MutableVec<u32>,
    goal_text: Mutable<String>,
}

impl FocusListCpn {
    pub fn new(patient: Mutable<Option<Rc<PatientInfo>>>) -> Rc<Self> {
        let now = js_now();
        let end_date = patient.lock_ref().as_ref().and_then(|pt| pt.lastdate()).unwrap_or(now.date());
        let start_date = end_date.previous_day().unwrap_or(now.date());

        Rc::new(Self {
            patient,
            start_date: Mutable::new(start_date.to_string()),
            end_date: Mutable::new(end_date.to_string()),
            changed: Mutable::new(true),
            fclist_stdate: Mutable::new(now.date().to_string()),
            fclist_sttime: Mutable::new(now.time().js_string()),
            fclist_status: Mutable::new(String::from("1")),
            loaded_subgroups: Mutable::new(true),
            loaded_focus: Mutable::new(true),
            loaded_goal: Mutable::new(true),
            ..Default::default()
        })
    }

    fn is_ipd(&self) -> impl Signal<Item = bool> + use<> {
        self.patient.signal_cloned().map(|opt| opt.as_ref().map(|pt| pt.is_ipd()).unwrap_or_default())
    }

    fn is_ipd_and_is_pre_admit(&self) -> impl Signal<Item = (bool, bool)> + use<> {
        self.patient
            .signal_cloned()
            .map(|opt| opt.as_ref().map(|pt| pt.visit_type.is_ipd_and_is_pre_admit()).unwrap_or_default())
    }

    fn new_form(&self) {
        let now = js_now();
        self.fclist_id.set_neq(0);
        self.smp_id.set_neq(String::new());
        self.subgroup.set_neq(String::new());
        self.subgroups.set(Vec::new());
        self.focus_id.set_neq(String::new());
        self.focuses.set(Vec::new());
        self.focus_text.set_neq(String::new());
        self.goals.set(Vec::new());
        self.goal_ids.lock_mut().clear();
        self.goal_text.set_neq(String::new());
        self.fclist_stdate.set_neq(now.date().to_string());
        self.fclist_sttime.set_neq(now.time().js_string());
        self.fclist_enddate.set_neq(String::new());
        self.fclist_endtime.set_neq(String::new());
        self.fclist_status.set_neq(String::from("1"));
        self.version.set_neq(0);
        self.used.set_neq(false);
        self.form_changed.set_neq(false);

        self.group_select_redraw.set_neq(true);
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
        let visit_type_opt = page.patient.lock_ref().as_ref().map(|pt| pt.visit_type());
        if let Some(visit_type) = visit_type_opt {
            page.focus_list.lock_mut().clear();

            app.async_load(
                true,
                clone!(app, page => async move {
                    let params = FocusListParams {
                        start_date: date_8601(&page.start_date.lock_ref()),
                        end_date: date_8601(&page.end_date.lock_ref()),
                        status: str_some(page.search_status.get_cloned()),
                        ..Default::default()
                    };
                    let result_opt = match visit_type {
                        VisitTypeId::Ipd(an) | VisitTypeId::PreAdmit(an) => {
                            // GET `EndPoint::IpdFocusListAn`
                            Some(FocusList::call_api_get_ipd(&an, &params, app.state()).await)
                        }
                        VisitTypeId::OpdEr(_, opd_er_order_master_id) => {
                            // GET `EndPoint::OpdErFocusListId`
                            Some(FocusList::call_api_get_opd_er(opd_er_order_master_id, &params, app.state()).await)
                        }
                        VisitTypeId::Visit(_) => None,
                    };
                    if let Some(result) = result_opt {
                        match result {
                            Ok(focus_lists) => {
                                page.checked.set_neq(!focus_lists.is_empty());
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

    fn load_group(page: Rc<Self>, app: Rc<App>) {
        app.async_load(
            true,
            clone!(app => async move {
                // GET `EndPoint::IpdTmpGroup`
                match TmpGroup::call_api_get(&TmpParams::default(), app.state()).await {
                    Ok(responses) => {
                        page.groups.set(responses);
                        page.group_select_redraw.set(true);
                    }
                    Err(e) => {
                        app.alert_app_error(&e).await;
                    }
                }
            }),
        )
    }

    fn load_subgroup(page: Rc<Self>, app: Rc<App>) {
        let opt = page.smp_id.lock_ref().parse::<u32>();
        if let Ok(smp_id) = opt {
            app.async_load(
                true,
                clone!(app => async move {
                    let params = TmpParams {
                        smp_id: Some(smp_id),
                        ..Default::default()
                    };
                    // GET `EndPoint::IpdTmpSubgroup`
                    match TmpSubGroup::call_api_get(&params, app.state()).await {
                        Ok(responses) => {
                            if responses.is_empty() {
                                page.loaded_focus.set(false);
                                page.loaded_goal.set(false);
                            } else {
                                page.subgroups.set(responses);
                                page.subgroup_select_redraw.set(true);
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

    fn load_focus(page: Rc<Self>, app: Rc<App>) {
        app.async_load(
            true,
            clone!(app => async move {
                let params = TmpParams {
                    smp_id: page.smp_id.lock_ref().parse::<u32>().ok(),
                    subgroup: page.subgroup.lock_ref().parse::<u32>().ok(),
                    ..Default::default()
                };
                // GET `EndPoint::IpdTmpFocus`
                match TmpFocus::call_api_get(&params, app.state()).await {
                    Ok(responses) => {
                        page.focuses.set(responses);
                        page.focus_select_redraw.set(true);
                    }
                    Err(e) => {
                        app.alert_app_error(&e).await;
                    }
                }
            }),
        )
    }

    fn load_goal(page: Rc<Self>, app: Rc<App>) {
        let subgroup = page.subgroup.lock_ref().parse::<u32>().ok();
        if subgroup.is_some() {
            app.async_load(
                true,
                clone!(app => async move {
                    let params = TmpParams {
                        smp_id: page.smp_id.lock_ref().parse::<u32>().ok(),
                        subgroup,
                        ..Default::default()
                    };
                    // GET `EndPoint::IpdTmpGoal`
                    match TmpGoal::call_api_get(&params, app.state()).await {
                        Ok(responses) => {
                            page.goals.set(responses);
                            // page.goal_select_redraw.set(true);
                        }
                        Err(e) => {
                            app.alert_app_error(&e).await;
                        }
                    }
                }),
            )
        }
    }

    fn load_focus_list_row(row: Rc<FocusList>, page: Rc<Self>, app: Rc<App>) {
        page.fclist_id.set_neq(row.fclist_id);
        page.smp_id.set_neq(row.smp_id.to_string());
        page.group_select_redraw.set(true);

        page.focus_text.set_neq(row.focus_text.clone().unwrap_or_default());
        page.goal_text.set_neq(row.goal_text.clone().unwrap_or_default());
        page.fclist_stdate.set_neq(row.fclist_stdate.map(|d| d.to_string()).unwrap_or_default());
        page.fclist_sttime.set_neq(row.fclist_sttime.js_string());
        page.fclist_enddate.set_neq(row.fclist_enddate.map(|d| d.to_string()).unwrap_or_default());
        page.fclist_endtime.set_neq(row.fclist_endtime.map(|t| t.js_string()).unwrap_or_default());
        page.fclist_status.set_neq(row.fclist_status.clone());
        page.version.set_neq(row.version);
        page.used.set(row.used);
        page.form_changed.set_neq(false);

        app.async_load(
            true,
            clone!(app => async move {

                // load subgroup
                let params = TmpParams {
                    smp_id: Some(row.smp_id),
                    ..Default::default()
                };
                // GET `EndPoint::IpdTmpSubgroup`
                match TmpSubGroup::call_api_get(&params, app.state()).await {
                    Ok(responses) => {
                        page.subgroups.set(responses);
                        page.subgroup.set_neq(row.subgroup.map(|i| i.to_string()).unwrap_or_default());
                        page.subgroup_select_redraw.set(true);
                    }
                    Err(e) => {
                        app.alert_app_error(&e).await;
                    }
                }

                // reload + render focus
                let params = TmpParams {
                    smp_id: Some(row.smp_id),
                    subgroup: row.subgroup,
                    ..Default::default()
                };
                // GET `EndPoint::IpdTmpFocus`
                match TmpFocus::call_api_get(&params, app.state()).await {
                    Ok(responses) => {
                        page.focuses.set(responses);
                        page.focus_id.set(row.focus_id.to_string());
                        page.focus_select_redraw.set(true);
                    }
                    Err(e) => {
                        app.alert_app_error(&e).await;
                    }
                }

                // reload + render goal
                let goal_ids = row.goals.clone().unwrap_or_default()
                    .split('|').flat_map(|g| g.split('^').take(1))
                    .map(|s| s.parse::<u32>()).collect::<Result<Vec<u32>, std::num::ParseIntError>>()
                    .unwrap_or_default();
                {
                    let mut lock = page.goal_ids.lock_mut();
                    lock.replace_cloned(goal_ids);
                    // if row.goal_text.as_ref().map(|txt| !txt.is_empty()).unwrap_or_default() {
                    //     lock.push(999);
                    // }
                }
                // GET `EndPoint::IpdTmpGoal`
                match TmpGoal::call_api_get(&params, app.state()).await {
                    Ok(responses) => {
                        page.goals.set(responses);
                    }
                    Err(e) => {
                        app.alert_app_error(&e).await;
                    }
                }
                page.form_show.set_neq(true);
            }),
        )
    }

    fn is_form_ready(&self) -> impl Signal<Item = bool> + use<> {
        map_ref! {
            let changed = self.form_changed.signal(),
            let smp_id = self.smp_id.signal_cloned().map(|s| !s.is_empty()),
            let subgroup = self.subgroup.signal_cloned().map(|s| !s.is_empty()),
            let focus_id = self.focus_id.signal_cloned().map(|s| !s.is_empty()),
            let goal_ids = self.goal_ids.signal_vec_cloned().to_signal_cloned().map(|gs| !gs.is_empty()),
            let fclist_stdate = self.fclist_stdate.signal_cloned().map(|s| !s.is_empty()),
            let fclist_sttime = self.fclist_sttime.signal_cloned().map(|s| !s.is_empty()),
            let not_ended = self.fclist_status.signal_cloned().map(|s| s != "2"),
            let fclist_enddate = self.fclist_enddate.signal_cloned().map(|s| !s.is_empty()),
            let fclist_endtime = self.fclist_endtime.signal_cloned().map(|s| !s.is_empty()) =>
            *changed && *smp_id && *subgroup && *focus_id && *goal_ids && *fclist_stdate && *fclist_sttime && (*not_ended || (*fclist_enddate && *fclist_endtime))
        }
    }

    fn finalized(page: Rc<Self>) -> Option<FocusListSave> {
        let smp_id = page.smp_id.lock_ref().parse::<u32>().ok();
        let focus_id = page.focus_id.lock_ref().parse::<u32>().ok();
        let fclist_sttime = time_8601(&page.fclist_sttime.lock_ref());
        if let (Some(smp_id), Some(focus_id), Some(fclist_sttime)) = (smp_id, focus_id, fclist_sttime) {
            Some(FocusListSave {
                fclist_id: zero_none(page.fclist_id.get()),
                smp_id,
                focus_id,
                focus_text: str_some(page.focus_text.get_cloned()),
                goal_ids: page.goal_ids.lock_ref().to_vec(),
                goal_text: str_some(page.goal_text.get_cloned()),
                fclist_stdate: date_8601(&page.fclist_stdate.lock_ref()),
                fclist_sttime,
                fclist_enddate: date_8601(&page.fclist_enddate.lock_ref()),
                fclist_endtime: time_8601(&page.fclist_endtime.lock_ref()),
                fclist_status: page.fclist_status.get_cloned(),
                version: zero_none(page.version.get()).unwrap_or(0),
            })
        } else {
            None
        }
    }

    fn submit_form(page: Rc<Self>, app: Rc<App>) {
        let is_ended = page.fclist_status.lock_ref().as_str() == "2";
        let smp_id = page.smp_id.lock_ref();
        let focus_id = page.focus_id.lock_ref();
        let goal_text = page.goal_text.lock_ref();
        let fclist_stdate = page.fclist_stdate.lock_ref();
        let fclist_sttime = page.fclist_sttime.lock_ref();
        let fclist_enddate = page.fclist_enddate.lock_ref();
        let fclist_endtime = page.fclist_endtime.lock_ref();

        if smp_id.is_empty() {
            if let Some(elm) = app.get_id("search_temp_smp").and_then(|elm| elm.dyn_into::<HtmlSelectElement>().ok()) {
                if let Err(e) = elm.focus() {
                    app.show_jsvalue_message(&e);
                }
            }
        } else if focus_id.is_empty() {
            if let Some(elm) = app.get_id("tmp_focus").and_then(|elm| elm.dyn_into::<HtmlSelectElement>().ok()) {
                if let Err(e) = elm.focus() {
                    app.show_jsvalue_message(&e);
                }
            }
        } else if page.goal_ids.lock_ref().is_empty() && goal_text.is_empty() {
            if let Some(elm) = app.get_id("goal_text").and_then(|elm| elm.dyn_into::<HtmlSelectElement>().ok()) {
                if let Err(e) = elm.focus() {
                    app.show_jsvalue_message(&e);
                }
            }
        } else if fclist_stdate.is_empty() {
            if let Some(elm) = app.get_id("fclist_stdate").and_then(|elm| elm.dyn_into::<HtmlSelectElement>().ok()) {
                if let Err(e) = elm.focus() {
                    app.show_jsvalue_message(&e);
                }
            }
        } else if fclist_sttime.is_empty() {
            if let Some(elm) = app.get_id("fclist_sttime").and_then(|elm| elm.dyn_into::<HtmlSelectElement>().ok()) {
                if let Err(e) = elm.focus() {
                    app.show_jsvalue_message(&e);
                }
            }
        } else if is_ended && fclist_enddate.is_empty() {
            if let Some(elm) = app.get_id("fclist_enddate").and_then(|elm| elm.dyn_into::<HtmlSelectElement>().ok()) {
                if let Err(e) = elm.focus() {
                    app.show_jsvalue_message(&e);
                }
            }
        } else if is_ended && fclist_endtime.is_empty() {
            if let Some(elm) = app.get_id("fclist_endtime").and_then(|elm| elm.dyn_into::<HtmlSelectElement>().ok()) {
                if let Err(e) = elm.focus() {
                    app.show_jsvalue_message(&e);
                }
            }
        } else if let Some(saver) = Self::finalized(page.clone()) {
            let opt = page.patient.lock_ref().as_ref().map(|pt| (pt.visit_type(), pt.hn()));
            if let Some((visit_type, hn)) = opt {
                app.async_load(
                    true,
                    clone!(app, page => async move {
                        let saver_result_opt = match visit_type {
                            VisitTypeId::Ipd(an) | VisitTypeId::PreAdmit(an) => {
                                let params = FocusListSaveParams {
                                    hn,
                                };
                                // POST `EndPoint::IpdFocusListAn`
                                Some(saver.call_api_post_ipd(&an, &params, app.state()).await)
                            }
                            VisitTypeId::OpdEr(_, opd_er_order_master_id) => {
                                // POST `EndPoint::OpdErFocusListId`
                                Some(saver.call_api_post_opd_er(opd_er_order_master_id, app.state()).await)
                            }
                            VisitTypeId::Visit(_) => None,
                        };
                        if let Some(saver_result) = saver_result_opt {
                            match saver_result {
                                Ok((id, responses)) => {
                                    app.alert_execute_responses(&responses, async move {
                                        // app.alert("บันทึกข้อมูลสำเร็จ");
                                        page.fclist_id.set_neq(id);
                                        page.form_changed.set_neq(false);
                                        page.changed.set_neq(true);
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
        }
    }

    fn delete_focus_list(page: Rc<Self>, app: Rc<App>) {
        let visit_type_opt = page.patient.lock_ref().as_ref().map(|pt| pt.visit_type());
        if let (Some(fclist_id), Some(version), Some(visit_type)) = (zero_none(page.fclist_id.get()), zero_none(page.version.get()), visit_type_opt) {
            app.async_load(
                true,
                clone!(app, page => async move {
                    if app.confirm("ยืนยันรายการ").await {
                        let params = FocusListParams {
                            fclist_id: Some(fclist_id),
                            version: Some(version),
                            ..Default::default()
                        };
                        let result_opt = match visit_type {
                            VisitTypeId::Ipd(an) | VisitTypeId::PreAdmit(an) => {
                                // DELETE `EndPoint::IpdFocusListAn`
                                Some(FocusList::call_api_delete_ipd(&an, &params, app.state()).await)
                            }
                            VisitTypeId::OpdEr(_, opd_er_order_master_id) => {
                                // DELETE `EndPoint::OpdErFocusListId`
                                Some(FocusList::call_api_delete_opd_er(opd_er_order_master_id, &params, app.state()).await)
                            }
                            VisitTypeId::Visit(_) => None,
                        };
                        if let Some(result) = result_opt {
                            match result {
                                Ok(responses) => {
                                    app.alert_execute_responses(&responses, async move {
                                        page.new_form();
                                        page.form_show.set(false);
                                        page.changed.set_neq(true);
                                    }).await;
                                }
                                Err(e) => {
                                    app.alert_app_error(&e).await;
                                }
                            }
                        }
                    }
                }),
            );
        }
    }

    pub fn render(page: Rc<Self>, app: Rc<App>) -> Dom {
        let is_allow_form = app.endpoint_is_allow(&Method::GET, &EndPoint::IpdTmpGroup, false)
            && app.endpoint_is_allow(&Method::GET, &EndPoint::IpdTmpSubgroup, false)
            && app.endpoint_is_allow(&Method::GET, &EndPoint::IpdTmpFocus, false)
            && app.endpoint_is_allow(&Method::GET, &EndPoint::IpdTmpGoal, false);

        html!("div", {
            .future(map_ref!(
                let busy = app.loader_is_loading(),
                let changed = page.changed.signal() =>
                !busy && *changed
            ).for_each(clone!(app, page => move |ready| {
                if ready {
                    Self::load_focus_list(page.clone(), app.clone());
                    page.changed.set_neq(false);
                }
                async {}
            })))
            .attr("role", "tabpanel")
            .attr("aria-labelledby", "nav-focus-list-tab")
            .class("col")
            .child(html!("div", {
                .apply_if(is_allow_form, |dom| dom.child(Self::render_form(page.clone(), app.clone())))
                .children([
                    html!("div", {
                        .class(class::FLEX_WRAP_T)
                        .children([
                            html!("div", {
                                .class(class::COLA_PY_L)
                                .child(html!("div", {
                                    .class(class::INPUT_GROUP)
                                    .children([
                                        doms::label_group_for("search_startdate","วันที่เริ่มต้นปัญหา"),
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
                                .class(class::COLA_PY_L)
                                .child(html!("div", {
                                    .class(class::INPUT_GROUP)
                                    .children([
                                        doms::label_group_for("search_status","สถานะ"),
                                        html!("select" => HtmlSelectElement, {
                                            .class("form-select")
                                            .attr("id", "search_status")
                                            .children([
                                                html!("option", {.attr("value","").text("ทั้งหมด")}),
                                                html!("option", {.attr("value","1").text("ปัญหายังคงอยู่")}),
                                                html!("option", {.attr("value","2").text("ปัญหาหมดไป")}),
                                            ])
                                            .apply(mixins::string_value_select(page.search_status.clone(), page.changed.clone()))
                                        }),
                                    ])
                                }))
                            }),
                            html!("div", {
                                .class(class::PY_L)
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
                                        .attr("type", "button")
                                        .visible_signal(page.is_ipd())
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
                                .class(class::PY_RX)
                                .apply_if(is_allow_form, |dom| dom
                                    .child_signal(page.is_ipd_and_is_pre_admit().map(clone!(app, page => move |(is_ipd, is_pre_admit)| {
                                        (if is_ipd {
                                            app.has_permission(Permission::IpdNurseNoteAdd) || (is_pre_admit && app.has_permission(Permission::OpdErNurseNoteAdd))
                                        } else {
                                            app.has_permission(Permission::OpdErNurseNoteAdd)
                                        }).then(|| {
                                            html!("button", {
                                                .attr("type", "button")
                                                .class(class::BTN_L_BLUE)
                                                .child(html!("i", {.class(class::FA_PLUS_SQ)}))
                                                .text(" Add focus list")
                                                .event(clone!(page => move |_: events::Click| {
                                                    page.new_form();
                                                    let show = page.form_show.get();
                                                    if !show {
                                                        page.form_show.set(true);
                                                    }
                                                }))
                                            })
                                        })
                                    })))
                                )
                                .child_signal(page.patient.signal_cloned().map(clone!(app, page => move |opt| {
                                    opt.map(|patient| {
                                        match patient.visit_type() {
                                            VisitTypeId::Ipd(an) | VisitTypeId::PreAdmit(an) => {
                                                html!("div",{
                                                    .class("float-end")
                                                    .children(PdfButtons::buttons(
                                                        PdfButtons::new(
                                                            TypstReport::from_system_with_coercion(SystemReport::IpdFocusList, &app.state().report_coercions()),
                                                            Mutable::new(an.clone()),
                                                            page.checked.clone(),
                                                            page.changed.clone(),
                                                            clone!(page => move || {serde_json::json!({
                                                                "id": an,
                                                                "patient": patient,
                                                                "focus": page.focus_list.lock_ref().to_vec(),
                                                            }).to_string()})
                                                        ), "PDF", Some("PDF (All)"), app.clone()
                                                    ))
                                                })
                                            }
                                            VisitTypeId::OpdEr(vn, _opd_er_order_master_id) => {
                                                html!("div",{
                                                    .class("float-end")
                                                    .children(PdfButtons::buttons(
                                                        PdfButtons::new(
                                                            TypstReport::from_system_with_coercion(SystemReport::OpdErFocusList, &app.state().report_coercions()),
                                                            Mutable::new(vn.clone()),
                                                            page.checked.clone(),
                                                            page.changed.clone(),
                                                            clone!(page => move || {serde_json::json!({
                                                                "id": vn,
                                                                "patient": patient,
                                                                "focus": page.focus_list.lock_ref().to_vec(),
                                                            }).to_string()})
                                                        ), "PDF", Some("PDF (All)"), app.clone()
                                                    ))
                                                })
                                            }
                                            VisitTypeId::Visit(_) => Dom::empty(),
                                        }
                                    })
                                })))
                            }),
                        ])
                    }),
                    doms::table_responsive(class::TABLE_STRIP, clone!(page => move |table| { table
                        .attr("width", "100%")
                        .children([
                            html!("thead", {
                                .child(html!("tr", {
                                    .class("text-center")
                                    .children([
                                        html!("th", {.class("text-center").attr("width","4%").text("#")}),
                                        html!("th", {.class("text-center").attr("width","24%").text("Focus")}),
                                        html!("th", {.class("text-center").attr("width","27%").text("เป้าหมาย / ผลลัพธ์ที่ต้องการ")}),
                                        html!("th", {.class("text-center").attr("width","13%").text("วันที่เริ่มปัญหา")}),
                                        html!("th", {.class("text-center").attr("width","13%").text("วันที่สิ้นสุดปัญหา")}),
                                        html!("th", {.class("text-center").attr("width","15%").text("สถานะ")}),
                                        html!("th", {.class("text-center").attr("width","4%").text("แก้ไข")}),
                                    ])
                                }))
                            }),
                            html!("tbody", {
                                .children_signal_vec(page.focus_list.signal_vec_cloned().enumerate().map(clone!(page => move |(i, row)| {
                                    let edit_dom = html!("td", {
                                        .class("text-center")
                                        // SessionManager::checkPermission('IPD_NURSE_NOTE','EDIT')
                                        .child_signal(page.is_ipd_and_is_pre_admit().map(clone!(app, page, row => move |(is_ipd, is_pre_admit)| {
                                            (app.endpoint_is_allow(&Method::GET, &EndPoint::IpdTmpFocus, is_pre_admit)
                                            && app.endpoint_is_allow(&Method::GET, &EndPoint::IpdTmpGoal, is_pre_admit)
                                            && if is_ipd {
                                                app.has_permission(Permission::IpdNurseNoteEdit) || (is_pre_admit && app.has_permission(Permission::OpdErNurseNoteEdit))
                                            } else {
                                                app.has_permission(Permission::OpdErNurseNoteEdit)
                                            }).then(|| {
                                                html!("button" => HtmlButtonElement, {
                                                    .attr("type", "button")
                                                    .attr("title","คลิก เพื่อทำการแก้ไขข้อมูล")
                                                    .class(class::BTN_GRAY)
                                                    .child(html!("i", {.class(class::FA_EDIT)}))
                                                    .apply(mixins::click_with_loader_checked(clone!(app, page, row => move || {
                                                        Self::load_focus_list_row(row.clone(), page.clone(), app.clone());
                                                    }), app.state()))
                                                })
                                            })
                                        })))
                                    });
                                    super::focus_list_row::render(i.get().unwrap_or_default(), row, edit_dom)
                                })))
                            }),
                        ])
                    })),
                ])
            }))
        })
    }

    fn render_form(page: Rc<Self>, app: Rc<App>) -> Dom {
        html!("div", {
            .visible_signal(page.form_show.signal())
            // .future(page.form_show.signal().for_each(clone!(app => move |show| {
            //     if show {
            //         if let Some(elm) = app.get_id("close_btn").and_then(|elm| elm.dyn_into::<HtmlButtonElement>().ok()) {
            //             if let Err(e) = elm.focus() {
            //                 app.show_jsvalue_error_message(&e);
            //             }
            //         }
            //     }
            //     async {}
            // })))
            .future(map_ref!{
                let busy = app.loader_is_loading(),
                let loaded = page.loaded_groups.signal() =>
                !busy && !loaded
            }.for_each(clone!(app, page => move |loaded| {
                if loaded {
                    Self::load_group(page.clone(), app.clone());
                    page.loaded_groups.set(true);
                }
                async {}
            })))
            .future(map_ref!{
                let busy = app.loader_is_loading(),
                let loaded = page.loaded_subgroups.signal() =>
                !busy && !loaded
            }.for_each(clone!(app, page => move |loaded| {
                if loaded {
                    clone!(app, page => Self::load_subgroup(page, app));
                    page.loaded_subgroups.set(true);
                }
                async {}
            })))
            .future(map_ref!{
                let busy = app.loader_is_loading(),
                let loaded = page.loaded_focus.signal() =>
                !busy && !loaded
            }.for_each(clone!(app, page => move |loaded| {
                if loaded {
                    Self::load_focus(page.clone(), app.clone());
                    page.loaded_focus.set(true);
                }
                async {}
            })))
            .future(map_ref!{
                let busy = app.loader_is_loading(),
                let loaded = page.loaded_goal.signal() =>
                !busy && !loaded
            }.for_each(clone!(app, page => move |loaded| {
                if loaded {
                    Self::load_goal(page.clone(), app.clone());
                    page.loaded_goal.set(true);
                }
                async {}
            })))
            .future(page.group_select_redraw.signal().for_each(clone!(app, page => move |redraw| {
                if redraw {
                    if let Some(elm) = app.get_id("search_temp_smp") {
                        NiceSelect::new_default_with_value(&elm, &page.smp_id.lock_ref());
                        page.group_select_redraw.set(false);
                    }
                }
                async {}
            })))
            .future(page.subgroup_select_redraw.signal().for_each(clone!(app, page => move |redraw| {
                if redraw {
                    if let Some(elm) = app.get_id("search_temp_subgroup") {
                        NiceSelect::new_default_with_value(&elm, &page.subgroup.lock_ref());
                        page.subgroup_select_redraw.set(false);
                    }
                }
                async {}
            })))
            .future(page.focus_select_redraw.signal().for_each(clone!(app, page => move |redraw| {
                if redraw {
                    if let Some(elm) = app.get_id("tmp_focus") {
                        NiceSelect::new_default_with_value(&elm, &page.focus_id.lock_ref());
                        page.focus_select_redraw.set(false);
                    }
                }
                async {}
            })))
            .class(class::CARD)
            .child(html!("div", {
                //.attr("id", "event_focuslist")
                .children([
                    html!("div", {
                        .class(class::CARD_HEAD)
                        .text("Focus List")
                        .child(html!("div", {
                            .class(class::FLOAT_RR)
                            .child(html!("button", {
                                .attr("type", "button")
                                .attr("id", "close_btn")
                                .class("btn-close")
                                .event(clone!(page => move |_: events::Click| {
                                    page.form_show.set(false);
                                }))
                            }))
                        }))
                    }),
                    html!("div", {
                        .class("card-body")
                        .children([
                            html!("div", {
                                .class(class::ROW)
                                .children([
                                    html!("div", {
                                        .class("col-sm-6")
                                        .children([
                                            html!("div", {
                                                .class(class::ROW)
                                                .children([
                                                    html!("label", {
                                                        .class(class::COL_SM3_RB)
                                                        .attr("for","search_temp_smp")
                                                        .child(html!("b", {.text("กลุ่มอาการ")}))
                                                    }),
                                                    html!("div", {
                                                        .class("col-md-9")
                                                        //.attr("id", "show_template_smp")
                                                        .child(html!("select" => HtmlSelectElement, {
                                                            .class("form-control")
                                                            .attr("id", "search_temp_smp")
                                                            .child(html!("option", {.attr("value","").text("เลือก")}))
                                                            .children_signal_vec(page.groups.signal_cloned().to_signal_vec().map(|group| {
                                                                html!("option", {
                                                                    .attr("value", &group.smp_id.to_string())
                                                                    .text(&group.smp_name.unwrap_or_default())
                                                                })
                                                            }))
                                                            .prop_signal("value", page.smp_id.signal_cloned())
                                                            .with_node!(element => {
                                                                .future(page.used.signal().for_each(clone!(page, element => move |v| {
                                                                    element.set_disabled(v);
                                                                    page.group_select_redraw.set(true);
                                                                    async {}
                                                                })))
                                                                .event(clone!(page => move |_: events::Change| {
                                                                    page.smp_id.set_neq(element.value());
                                                                    page.subgroup.set_neq(String::new());
                                                                    page.subgroups.lock_mut().clear();
                                                                    page.focus_id.set_neq(String::new());
                                                                    page.focuses.lock_mut().clear();
                                                                    page.focus_text.set_neq(String::new());
                                                                    page.goals.lock_mut().clear();
                                                                    page.goal_ids.lock_mut().clear();
                                                                    page.goal_text.set_neq(String::new());
                                                                    page.form_changed.set_neq(true);
                                                                    // page.loaded_focus.set(false);
                                                                    page.loaded_subgroups.set(false);
                                                                }))
                                                            })
                                                        }))
                                                    }),
                                                ])
                                            }),
                                            html!("div", {
                                                .class(class::ROW)
                                                .children([
                                                    html!("label", {
                                                        .class(class::COL_SM3_RB)
                                                        .attr("for","search_temp_subgroup")
                                                        .child(html!("b", {.text("กลุ่มย่อย")}))
                                                    }),
                                                    html!("div", {
                                                        .class("col-md-9")
                                                        //.attr("id", "dropdown_data_subgroup")
                                                        .visible_signal(page.smp_id.signal_cloned().map(|smp_id| !smp_id.is_empty()))
                                                        .child(html!("select" => HtmlSelectElement, {
                                                            .class("form-control")
                                                            .attr("id", "search_temp_subgroup")
                                                            .child(html!("option", {.attr("value","").text("เลือก")}))
                                                            .children_signal_vec(page.subgroups.signal_cloned().to_signal_vec().map(|subgroup| {
                                                                html!("option", {
                                                                    .attr("value", &subgroup.subgroup.to_string())
                                                                    .text(&subgroup.subgroup_name.unwrap_or_default())
                                                                })
                                                            }))
                                                            .child(html!("option", {.attr("value","0").text("**ไม่ระบุ(แสดงเสมอ)**")}))
                                                            .prop_signal("value", page.subgroup.signal_cloned())
                                                            .with_node!(element => {
                                                                .future(page.used.signal().for_each(clone!(page, element => move |v| {
                                                                    element.set_disabled(v);
                                                                    page.subgroup_select_redraw.set(true);
                                                                    async {}
                                                                })))
                                                                .event(clone!(page => move |_: events::Change| {
                                                                    page.subgroup.set_neq(element.value());
                                                                    page.focus_id.set_neq(String::new());
                                                                    page.focuses.lock_mut().clear();
                                                                    page.focus_text.set_neq(String::new());
                                                                    page.goals.lock_mut().clear();
                                                                    page.goal_ids.lock_mut().clear();
                                                                    page.goal_text.set_neq(String::new());
                                                                    page.loaded_focus.set(false);
                                                                    page.loaded_goal.set(false);
                                                                }))
                                                            })
                                                        }))
                                                    }),
                                                ])
                                            }),
                                            html!("div", {
                                                .class(class::ROW)
                                                .children([
                                                    html!("label", {
                                                        .class(class::COL_SM3_RB)
                                                        .attr("for","tmp_focus")
                                                        .child(html!("b", {.text("Focus")}))
                                                    }),
                                                    html!("div", {
                                                        .class("col-md-9")
                                                        //.attr("id", "show_template_focus")
                                                        // .visible_signal(page.has_group_and_subgroup())
                                                        .visible_signal(page.smp_id.signal_cloned().map(|id| !id.is_empty()))
                                                        .child(html!("select" => HtmlSelectElement, {
                                                            .class("form-control")
                                                            .attr("id", "tmp_focus")
                                                            .child(html!("option", {.attr("value","").text("เลือก")}))
                                                            .children_signal_vec(page.focuses.signal_cloned().to_signal_vec().map(|focus| {
                                                                html!("option", {
                                                                    .attr("value", &focus.focus_id.to_string())
                                                                    .text(&focus.focus_name.unwrap_or_default())
                                                                })
                                                            }))
                                                            .child(html!("option", {.attr("value","999").text("อื่นๆ")}))
                                                            .prop_signal("value", page.focus_id.signal_cloned())
                                                            .with_node!(element => {
                                                                .future(page.used.signal().for_each(clone!(page, element => move |v| {
                                                                    element.set_disabled(v);
                                                                    page.focus_select_redraw.set(true);
                                                                    async {}
                                                                })))
                                                                .event(clone!(page => move |_: events::Change| {
                                                                    let value = element.value();
                                                                    let focus_id = value.parse::<u32>().unwrap_or_default();
                                                                    page.focus_id.set_neq(value);
                                                                    // focus and goal are relate to subgroup except focus = `อื่นๆ`
                                                                    let mut refresh_subgroup_and_goals = false;
                                                                    // `อื่นๆ` will set subgroup to 0 and refresh goals
                                                                    if focus_id == 999 {
                                                                        page.subgroup.set_neq(String::from("0"));
                                                                        refresh_subgroup_and_goals = true;
                                                                    // when swich back from `อื่นๆ`, reclaim subgroup and refresh goals
                                                                    } else if let Some(focus) = page.focuses.lock_ref().iter().find(|focus| focus.focus_id == focus_id) {
                                                                        if page.subgroup.lock_ref().parse::<u32>().unwrap_or_default() != focus.subgroup {
                                                                            page.subgroup.set(focus.subgroup.to_string());
                                                                            refresh_subgroup_and_goals = true;
                                                                        }
                                                                    }
                                                                    if refresh_subgroup_and_goals {
                                                                        page.subgroup_select_redraw.set(true);
                                                                        page.goals.lock_mut().clear();
                                                                        page.goal_ids.lock_mut().clear();
                                                                        page.goal_text.set_neq(String::new());
                                                                        page.loaded_goal.set(false);
                                                                    }
                                                                    page.focus_text.set_neq(String::new());
                                                                    page.form_changed.set_neq(true);
                                                                }))
                                                            })
                                                        }))
                                                    }),
                                                ])
                                            }),
                                        ])
                                        .child_signal(page.focus_id.signal_cloned().map(clone!(page => move |focus_id| {
                                            (focus_id == "999").then(|| {
                                                html!("div", {
                                                    .class(class::ROW)
                                                    .children([
                                                        html!("label", {
                                                            .attr("for", "focus_text")
                                                            .class(class::COL_SM3_RB)
                                                            .text("ระบุ..")
                                                        }),
                                                        html!("div", {
                                                            .class("col-md-9")
                                                            .child(html!("input" => HtmlInputElement, {
                                                                .attr("type", "text")
                                                                .class("form-control")
                                                                .attr("id", "focus_text")
                                                                .apply(mixins::string_value(page.focus_text.clone(), page.form_changed.clone()))
                                                                .apply(mixins::other_true_disable(page.used.clone()))
                                                            }))
                                                        })
                                                    ])
                                                })
                                            })
                                        })))
                                        .child(html!("div", {
                                            .class(class::ROW)
                                            .children([
                                                html!("div", {
                                                    .class(class::COL_SM3_RB)
                                                    .child(html!("b", {.text("เป้าหมาย")}))
                                                }),
                                                html!("div", {
                                                    .class("col-md-9")
                                                    //.attr("id", "show_template_goal")
                                                    .visible_signal(page.smp_id.signal_cloned().map(|id| !id.is_empty()))
                                                    // .visible_signal(page.has_group_and_subgroup())
                                                    .children_signal_vec(page.goals.signal_cloned().to_signal_vec().map(clone!(page => move |goal| {
                                                        let id = ["goal_", &goal.goal_id.to_string()].concat();
                                                        if goal.goal_status == Some(String::from("N")) {
                                                            page.goal_ids.lock_mut().retain(|x| *x != goal.goal_id);
                                                        }
                                                        html!("div", {
                                                            .class(class::FORM_CHK_COL_SM12)
                                                            .children([
                                                                html!("input" => HtmlInputElement, {
                                                                    .attr("type", "checkbox")
                                                                    .attr("id", &id)
                                                                    .class("form-check-input")
                                                                    .attr("value", &goal.goal_id.to_string())
                                                                    .with_node!(element => {
                                                                        .future(page.used.signal().for_each(clone!(page, element, goal => move |v| {
                                                                            if goal.goal_status == Some(String::from("N")) {
                                                                                element.set_disabled(true);
                                                                            } else {
                                                                                element.set_disabled(v);
                                                                            }
                                                                            if page.goal_ids.lock_ref().contains(&goal.goal_id) {
                                                                                element.set_checked(true);
                                                                            }
                                                                            async {}
                                                                        })))
                                                                        .event(clone!(page => move |_: events::Change| {
                                                                            if element.checked() {
                                                                                page.goal_ids.lock_mut().push(goal.goal_id);
                                                                            } else {
                                                                                page.goal_ids.lock_mut().retain(|x| *x != goal.goal_id);
                                                                            }
                                                                            page.form_changed.set_neq(true);
                                                                        }))
                                                                    })
                                                                }),
                                                                html!("label", {
                                                                    .class("form-check-label")
                                                                    .attr("for", &id)
                                                                    .style("user-select","none")
                                                                    .apply_if(goal.goal_status == Some(String::from("N")), |dom| dom.text("(\u{274c} ยกเลิกการใช้งาน) "))
                                                                    .text(&goal.goal_name.unwrap_or_default())
                                                                })
                                                            ])
                                                        })
                                                    })))
                                                    .child(html!("div", {
                                                        .class(class::FORM_CHK_COL_SM12)
                                                        .children([
                                                            html!("input" => HtmlInputElement, {
                                                                .attr("type", "checkbox")
                                                                .attr("id", "goal_999")
                                                                .class("form-check-input")
                                                                .attr("value", "999")
                                                                .with_node!(element => {
                                                                    .future(page.used.signal().for_each(clone!(element => move |v| {
                                                                        element.set_disabled(v);
                                                                        async {}
                                                                    })))
                                                                    .future(page.goal_ids.signal_vec_cloned().to_signal_cloned().for_each(clone!(element => move |ids| {
                                                                        element.set_checked(ids.contains(&999));
                                                                        async {}
                                                                    })))
                                                                    .event(clone!(page => move |_: events::Change| {
                                                                        if element.checked() {
                                                                            page.goal_ids.lock_mut().push(999);
                                                                        } else {
                                                                            page.goal_ids.lock_mut().retain(|x| *x != 999);
                                                                            page.goal_text.set_neq(String::new());
                                                                        }
                                                                        page.form_changed.set_neq(true);
                                                                    }))
                                                                })
                                                            }),
                                                            doms::label_check_for("goal_999","อื่นๆ"),
                                                        ])
                                                    }))
                                                }),
                                            ])
                                        }))
                                        .child_signal(page.goal_ids.signal_vec_cloned().to_signal_cloned().map(clone!(page => move |ids| {
                                            ids.contains(&999).then(|| {
                                                html!("div", {
                                                    .class(class::ROW)
                                                    .children([
                                                        html!("label", {
                                                            .attr("for", "goal_text")
                                                            .class(class::COL_SM3_RB)
                                                            .text("ระบุ..")
                                                        }),
                                                        html!("div", {
                                                            .class("col-md-9")
                                                            .child(html!("textarea" => HtmlTextAreaElement, {
                                                                .class("form-control")
                                                                .attr("id", "goal_text")
                                                                .attr("rows","2")
                                                                .apply(mixins::string_value(page.goal_text.clone(), page.form_changed.clone()))
                                                                .apply(mixins::other_true_disable(page.used.clone()))
                                                            }))
                                                        })
                                                    ])
                                                })
                                            })
                                        })))
                                    }),
                                    html!("div", {
                                        .class("col-sm-6")
                                        .children([
                                            html!("div", {
                                                .class(class::ROW)
                                                .children([
                                                    html!("div", {
                                                        .class(class::COL_SM5_RB)
                                                        .child(html!("b", {.text("วันที่เริ่มต้นปัญหา")}))
                                                    }),
                                                    html!("div", {
                                                        .class("col-sm-4")
                                                        .child(doms::date_picker(
                                                            page.fclist_stdate.clone(),
                                                            page.form_changed.clone(), page.used.signal(), None,
                                                            |d| d.style("min-width","135px"),
                                                            |d| d.class("form-control"),
                                                            |d| d.class("form-control").attr("id", "fclist_stdate"),
                                                            |s| s, always(None),
                                                        ))
                                                    }),
                                                    html!("div", {
                                                        .class("col-sm-3")
                                                        .child(doms::time_picker(
                                                            page.fclist_sttime.clone(),
                                                            page.form_changed.clone(), page.used.signal(), None,
                                                            |d| d.style("min-width","110px"),
                                                            |d| d.class("form-control"),
                                                            |d| d.class("form-control").attr("id", "fclist_sttime"),
                                                            |s| s, always(None),
                                                        ))
                                                    }),
                                                ])
                                            }),
                                        ])
                                        .child_signal(page.fclist_id.signal_cloned().map(clone!(page => move |id| {
                                            zero_none(id).is_some().then(|| {
                                                html!("div", {
                                                    //.attr("id", "event_hide_show_form")
                                                    .children([
                                                        html!("div", {
                                                            .class(class::ROW)
                                                            .children([
                                                                html!("div", {
                                                                    .class(class::COL_SM5_RB)
                                                                    .child(html!("b", {.text("วันที่สิ้นสุดปัญหา")}))
                                                                }),
                                                                html!("div", {
                                                                    .class("col-sm-4")
                                                                    .child(doms::date_picker(
                                                                        page.fclist_enddate.clone(),
                                                                        page.form_changed.clone(), always(false), None,
                                                                        |d| d.style("min-width","135px"),
                                                                        |d| d.class("form-control"),
                                                                        |d| d.class("form-control").attr("id", "fclist_enddate"),
                                                                        |s| s, always(None),
                                                                    ))
                                                                }),
                                                                html!("div", {
                                                                    .class("col-sm-3")
                                                                    .child(doms::time_picker(
                                                                        page.fclist_endtime.clone(),
                                                                        page.form_changed.clone(), always(false), None,
                                                                        |d| d.style("min-width","110px"),
                                                                        |d| d.class("form-control"),
                                                                        |d| d.class("form-control").attr("id", "fclist_endtime"),
                                                                        |s| s, always(None),
                                                                    ))
                                                                }),
                                                            ])
                                                        }),
                                                        html!("p"),
                                                        html!("div", {
                                                            .class(class::ROW)
                                                            .children([
                                                                html!("label", {
                                                                    .class(class::COL_SM5_R)
                                                                    .attr("for","fclist_status1")
                                                                    .child(html!("b", {.text("สถานะ")}))
                                                                }),
                                                                html!("div", {
                                                                    .class(class::FORM_CHK_COL_SM4)
                                                                    .children([
                                                                        html!("input" => HtmlInputElement, {
                                                                            .attr("type", "radio")
                                                                            .class("form-check-input")
                                                                            .attr("value", "1")
                                                                            .attr("id", "fclist_status1")
                                                                            .apply(mixins::radio_match(page.fclist_status.clone(), page.form_changed.clone(), "1"))
                                                                        }),
                                                                        doms::label_check_for("fclist_status1","ปัญหายังคงอยู่"),
                                                                    ])
                                                                }),
                                                            ])
                                                        }),
                                                        html!("div", {
                                                            .class(class::ROW)
                                                            .children([
                                                                html!("div", {
                                                                    .class(class::COL_SM5_R)
                                                                }),
                                                                html!("div", {
                                                                    .class(class::FORM_CHK_COL_SM3)
                                                                    .children([
                                                                        html!("input" => HtmlInputElement, {
                                                                            .attr("type", "radio")
                                                                            .class("form-check-input")
                                                                            .attr("value", "2")
                                                                            .attr("id", "fclist_status2")
                                                                            .apply(mixins::radio_match(page.fclist_status.clone(), page.form_changed.clone(), "2"))
                                                                        }),
                                                                        doms::label_check_for("fclist_status2","ปัญหาหมดไป"),
                                                                    ])
                                                                }),
                                                            ])
                                                        }),
                                                    ])
                                                })
                                            })
                                        })))
                                    }),
                                ])
                            }),
                            html!("div", {
                                .class("row")
                                .child(html!("div", {
                                    .class(class::COL_SM12_R)
                                    .child_signal(page.is_ipd_and_is_pre_admit().map(clone!(app, page => move |(is_ipd, is_pre_admit)| {
                                        (if is_ipd {
                                            app.endpoint_is_allow(&Method::POST, &EndPoint::IpdFocusListAn, is_pre_admit)
                                        } else {
                                            app.endpoint_is_allow(&Method::POST, &EndPoint::OpdErFocusListId, false)
                                        }).then(|| {
                                            html!("button" => HtmlButtonElement, {
                                                .attr("type", "button")
                                                .class(class::BTN_L)
                                                .class_signal("btn-primary", page.is_form_ready())
                                                .class_signal("btn-secondary", not(page.is_form_ready()))
                                                .child(html!("i", {.class(class::FA_SAVE)}))
                                                .text(" บันทึกข้อมูล")
                                                .apply(mixins::click_with_loader_checked_or_true_disable_signal(clone!(app, page => move || {
                                                    Self::submit_form(page.clone(), app.clone());
                                                    page.form_show.set_neq(false);
                                                }), not(page.is_form_ready()), app.state()))
                                            })
                                        })
                                    })))
                                    .child_signal(page.is_ipd_and_is_pre_admit().map(clone!(app, page => move |(is_ipd, is_pre_admit)| {
                                        (app.endpoint_is_allow(&Method::GET, &EndPoint::IpdTmpFocus, is_pre_admit)
                                        && app.endpoint_is_allow(&Method::GET, &EndPoint::IpdTmpGoal, is_pre_admit)
                                        && if is_ipd {
                                            app.has_permission(Permission::IpdNurseNoteEdit) || (is_pre_admit && app.has_permission(Permission::OpdErNurseNoteEdit))
                                        } else {
                                            app.has_permission(Permission::OpdErNurseNoteEdit)
                                        }).then(|| {
                                            html!("button" => HtmlButtonElement, {
                                                .attr("type", "button")
                                                .class(class::BTN_L_GRAY)
                                                .child(html!("i", {.class(class::FA_UNDO)}))
                                                .text(" ยกเลิกการแก้ไข")
                                                .apply(mixins::click_with_loader_checked_or_true_disable_signal(clone!(app, page => move || {
                                                    match zero_none(page.fclist_id.get()) {
                                                        Some(fclist_id) => if let Some(row) = page.focus_list.lock_ref().iter().find(|r| r.fclist_id == fclist_id) {
                                                            Self::load_focus_list_row(row.clone(), page.clone(), app.clone());
                                                        }
                                                        None => page.new_form(),
                                                    }
                                                }), not(page.form_changed.signal()), app.state()))
                                            })
                                        })
                                    })))
                                    .child_signal(map_ref!{
                                        let (is_ipd, is_pre_admit) = page.is_ipd_and_is_pre_admit(),
                                        let fclist_id = page.fclist_id.signal_cloned(),
                                        let used = page.used.signal() =>
                                        (*fclist_id, *used, *is_ipd, *is_pre_admit)
                                    }.map(clone!(app, page => move |(id, used, is_ipd, is_pre_admit)| (
                                        id > 0
                                        && !used
                                        && (if is_ipd {
                                            app.endpoint_is_allow(&Method::DELETE, &EndPoint::IpdFocusListAn, is_pre_admit)
                                        } else {
                                            app.endpoint_is_allow(&Method::DELETE, &EndPoint::OpdErFocusListId, false)
                                        })
                                    ).then(|| {
                                        html!("button" => HtmlButtonElement, {
                                            .attr("type", "button")
                                            .class(class::BTN_RED)
                                            //.attr("id", "btn_delete_focuslist")
                                            .child(html!("i", {.class(class::FA_TRASH)}))
                                            .text(" ลบ")
                                            .apply(mixins::click_with_loader_checked(clone!(app, page => move || {
                                                Self::delete_focus_list(page.clone(), app.clone());
                                            }), app.state()))
                                        })
                                    }))))
                                }))
                            }),
                        ])
                    })
                ])
            }))
        })
    }
}
