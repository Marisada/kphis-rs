use dominator::{Dom, EventOptions, clone, events, html, with_node};
use futures_signals::{
    map_ref,
    signal::{Mutable, Signal, SignalExt, always, not},
    signal_vec::SignalVec,
};
use std::{collections::HashSet, rc::Rc, time::Duration};
use time::{Date, PrimitiveDateTime, Time};
use web_sys::{HtmlButtonElement, HtmlInputElement, HtmlSelectElement, HtmlTextAreaElement};

use kphis_model::{
    app::VisitTypeId,
    endpoint::EndPoint,
    fetch::Method,
    index_action::IndexAction,
    index_monitor::IndexMonitor,
    index_plan::{IndexPlan, IndexPlanSave},
    order::{OrderItem, OrderItemPatch, OrderItemPatchAction, OrderParams},
    patient_info::PatientInfo,
    user::permission::Permission,
    vital_sign::{VitalSignParams, VitalSignSave},
};
use kphis_ui_app::App;
use kphis_ui_core::{binding::NiceSelect, class, doms, mixins};
use kphis_util::{
    datetime::{JsTime, date_8601, date_th, date_th_opt, datetime_8601, datetime_from_opt, datetime_th_opt, datetime_th_opt_relative, datetime_th_relative, js_now, time_8601, time_hm_opt},
    error::CONTACT_ADMIN,
    util::{find_decimal_in_text, opt_zero_none, str_some, zero_none},
};

use crate::show_patient_main::{load_patient_info, render_patient_info};

/// - GET `EndPoint::IpdOrderItem`
/// - GET `EndPoint::OpdErOrderItem`
/// - PATCH `EndPoint::IpdOrderItem` (guarded, remove nurse-assign, injection-goggle btn)
/// - PATCH `EndPoint::OpdErOrderItem` (guarded, remove nurse-assign, injection-goggle btn)
/// - POST `EndPoint::IpdIndexPlan` (guarded, remove plan-save btn)
/// - POST `EndPoint::OpdErIndexPlan` (guarded, remove plan-save btn)
/// - DELETE `EndPoint::IpdIndexPlanId` (guarded, remove plan-delete btn)
/// - DELETE `EndPoint::OpdErIndexPlanId` (guarded, remove plan-delete btn)
/// - POST `EndPoint::IpdIndexAction` (guarded, remove action-save btn)
/// - POST `EndPoint::OpdErIndexAction` (guarded, remove action-save btn)
/// - DELETE `EndPoint::IpdIndexActionId` (guarded, remove action-delete btn)
/// - DELETE `EndPoint::OpdErIndexActionId` (guarded, remove action-delete btn)
/// - POST/PUT `EndPoint::IpdVitalSign` (guarded, remove DTX/HCT btn)
/// - POST/PUT `EndPoint::OpdErVitalSign` (guarded, remove DTX/HCT btn)
/// - GET `EndPoint::IpdShowPatientMainAn` (load_patient_info, guarded, no patient-info)
/// - GET `EndPoint::OpdErShowPatientMainVn` (load_patient_info, guarded, no patient-info)
/// - GET `EndPoint::OpdErShowPatientMainId` (load_patient_info, guarded, no patient-info)
#[derive(Clone, Default)]
pub struct IndexPlanActionForm {
    view_by: Mutable<String>,

    loaded: Mutable<bool>,
    parent_need_reload: Mutable<bool>,
    form_type: Mutable<FormType>,

    patient: Mutable<Option<Rc<PatientInfo>>>,
    visit_type: Mutable<Option<VisitTypeId>>,

    // for `without order`
    order_type: OrderType,
    order_item_id: Mutable<u32>,
    order_item: Mutable<Option<Rc<OrderItem>>>,

    active_tab: Mutable<PlanTab>,
    plan_id: Mutable<Option<u32>>,
    plan_date: Mutable<String>,
    plan_time: Mutable<String>,
    plan_detail: Mutable<String>,
    plan_times_in_date: Mutable<Option<Vec<Time>>>,

    allow_plan_before_order: Mutable<String>,
    plan_add_datetimes: Mutable<HashSet<PrimitiveDateTime>>,
    plan_time_start_hour: Mutable<u8>,
    selected_plan: Mutable<Option<Rc<IndexPlan>>>,

    action_id: Mutable<Option<u32>>,
    vs_id: Mutable<Option<u32>>,

    check_datetime: Mutable<String>,
    check_person: Mutable<String>,
    redraw_check_person: Mutable<bool>,

    action_date: Mutable<String>,
    action_time: Mutable<String>,
    action_report_back: Mutable<String>,
    action_result: Mutable<String>,
    action_remark: Mutable<String>,
    action_blood_had: Mutable<String>,
    action_person_1: Mutable<String>,
    action_person_2: Mutable<String>,
    redraw_person_1: Mutable<bool>,
    redraw_person_2: Mutable<bool>,

    monitor_id: Mutable<Option<u32>>,
    monitor_datetime: Mutable<String>,
    monitor_abnormal: Mutable<Option<String>>,
    monitor_result: Mutable<String>,
    monitor_remark: Mutable<String>,
    // for calculate duration
    action_datetime: Mutable<Option<PrimitiveDateTime>>,

    changed: Mutable<bool>,
}

impl IndexPlanActionForm {
    /// order_item_id = 0 is plan without order <br>
    /// plan_id = None is start with no plan selected <br>
    /// action_id = None is start with no action selected <br>
    pub fn new(
        order_item_id: u32,
        plan_id: Option<u32>,
        action_id: Option<u32>,
        patient: Mutable<Option<Rc<PatientInfo>>>,
        order_type: OrderType,
        form_type: FormType,
        view_by: Mutable<String>,
    ) -> Rc<Self> {
        Rc::new(Self {
            order_item_id: Mutable::new(order_item_id),
            plan_id: Mutable::new(plan_id),
            action_id: Mutable::new(action_id),
            patient,
            order_type,
            form_type: Mutable::new(form_type),
            view_by,
            ..Default::default()
        })
    }

    /// order_item_id = 0 is plan without order <br>
    /// plan_id = None is start with no plan selected <br>
    /// action_id = None is start with no action selected <br>
    /// this will call load_patient_info() from visit_type later
    pub fn new_with_visit_type(
        order_item_id: u32,
        plan_id: Option<u32>,
        action_id: Option<u32>,
        visit_type: VisitTypeId,
        order_type: OrderType,
        form_type: FormType,
        view_by: Mutable<String>,
    ) -> Rc<Self> {
        Rc::new(Self {
            order_item_id: Mutable::new(order_item_id),
            plan_id: Mutable::new(plan_id),
            action_id: Mutable::new(action_id),
            visit_type: Mutable::new(Some(visit_type)),
            order_type,
            form_type: Mutable::new(form_type),
            view_by,
            ..Default::default()
        })
    }

    pub fn is_continuous(&self) -> bool {
        matches!(self.order_type, OrderType::Continuous)
    }

    fn is_ipd(&self) -> bool {
        self.patient.lock_ref().as_ref().map(|pt| pt.visit_type.is_ipd()).unwrap_or_default()
    }

    fn is_ipd_and_is_pre_admit(&self) -> (bool, bool) {
        self.patient.lock_ref().as_ref().map(|pt| pt.visit_type.is_ipd_and_is_pre_admit()).unwrap_or_default()
    }

    fn is_med(&self) -> bool {
        self.order_item
            .lock_ref()
            .as_ref()
            .and_then(|order_item| order_item.order_item_type.as_ref().map(|oit| ["med", "ivfluids", "injection"].contains(&oit.as_str())))
            .unwrap_or_default()
    }

    fn is_med_signal(&self) -> impl Signal<Item = bool> + use<> {
        self.order_item.signal_cloned().map(|opt| {
            opt.as_ref()
                .and_then(|order_item| order_item.order_item_type.as_ref().map(|oit| ["med", "ivfluids", "injection"].contains(&oit.as_str())))
                .unwrap_or_default()
        })
    }

    fn is_stat_signal(&self) -> impl Signal<Item = bool> + use<> {
        self.order_item.signal_cloned().map(|opt| {
            opt.as_ref()
                .and_then(|order_item| {
                    // allow `without_order` to use `stat` tab
                    if order_item.order_item_id == 0 {
                        Some(true)
                    } else {
                        order_item.stat.as_ref().map(|stat| stat.as_str() == "Y")
                    }
                })
                .unwrap_or_default()
        })
    }

    fn is_new_plan_signal(&self) -> impl Signal<Item = bool> + use<> {
        self.plan_id.signal().map(|opt| opt == Some(0))
    }

    fn is_new_action_signal(&self) -> impl Signal<Item = bool> + use<> {
        self.action_id.signal().map(|opt| opt == Some(0))
    }

    fn is_new_monitor_signal(&self) -> impl Signal<Item = bool> + use<> {
        self.monitor_id.signal().map(|opt| opt == Some(0))
    }

    fn is_add_hours_input_empty_signal(&self) -> impl Signal<Item = bool> + use<> {
        self.plan_add_datetimes.signal_cloned().map(|dts| dts.is_empty())
    }

    fn allow_plan_before_order_signal(&self) -> impl Signal<Item = bool> + use<> {
        self.allow_plan_before_order.signal_cloned().map(|allow| allow.as_str() == "Y")
    }

    /// no order_item -> false <br>
    /// no plans -> false <br>
    /// no actions -> true
    fn has_plan_without_action(&self, plan_id: u32) -> bool {
        self.order_item
            .lock_ref()
            .as_ref()
            .map(|order_item| order_item.index_plans.iter().any(|plan| plan.plan_id == plan_id && plan.actions.is_empty()))
            .unwrap_or_default()
    }

    fn has_dtx_signal(&self) -> impl Signal<Item = bool> + use<> {
        self.order_item.signal_cloned().map(|opt| opt.as_ref().map(|order_item| order_item.has_dtx()).unwrap_or_default())
    }

    fn has_hct_signal(&self) -> impl Signal<Item = bool> + use<> {
        self.order_item.signal_cloned().map(|opt| opt.as_ref().map(|order_item| order_item.has_hct()).unwrap_or_default())
    }

    fn load_patient_info(modal: Rc<Self>, app: Rc<App>) {
        if let Some(visit_type) = modal.visit_type.get_cloned() {
            app.async_load(
                true,
                clone!(app, modal => async move {
                    // GET `EndPoint::IpdShowPatientMainAn`
                    // GET `EndPoint::OpdErShowPatientMainVn`
                    // GET `EndPoint::OpdErShowPatientMainId`
                    let patient = load_patient_info(visit_type, app).await;
                    modal.patient.set(Some(Rc::new(patient)));
                }),
            )
        }
    }

    fn load(modal: Rc<Self>, app: Rc<App>) {
        let visit_type = modal.patient.lock_ref().as_ref().map(|pt| pt.visit_type());
        app.async_load(
            true,
            clone!(app => async move {
                let order_item_id = zero_none(modal.order_item_id.get());
                let without_order = order_item_id.is_none().then_some(String::from("Y"));
                match visit_type {
                    Some(VisitTypeId::Ipd(an)) | Some(VisitTypeId::PreAdmit(an)) => {
                        let params = OrderParams {
                            order_item_id,
                            an: Some(an.clone()),
                            // current_date: Some(now),
                            view_by: str_some(modal.view_by.get_cloned()),
                            without_order,
                            ..Default::default()
                        };
                        // GET `EndPoint::IpdOrderItem`
                        match OrderItem::call_api_get_ipd(&params, app.state()).await {
                            Ok(order_items) => {
                                let first_item = order_items.first().cloned().map(Rc::new);
                                modal.set_order_item(first_item);
                            }
                            Err(e) => {
                                app.alert_app_error(&e).await;
                            }
                        }
                        modal.changed.set(false);
                    }
                    Some(VisitTypeId::OpdEr(_vn, opd_er_order_master_id)) => {
                        let params = OrderParams {
                            order_item_id,
                            opd_er_order_master_id: Some(opd_er_order_master_id),
                            view_by: str_some(modal.view_by.get_cloned()),
                            without_order,
                            ..Default::default()
                        };
                        // GET `EndPoint::OpdErOrderItem`
                        match OrderItem::call_api_get_opd_er(&params, app.state()).await {
                            Ok(order_items) => {
                                let first_item = order_items.first().cloned().map(Rc::new);
                                modal.set_order_item(first_item);
                            }
                            Err(e) => {
                                app.alert_app_error(&e).await;
                            }
                        }
                        modal.changed.set(false);
                    }
                    Some(VisitTypeId::Visit(_))
                    | None => {}
                }
            }),
        )
    }

    fn submit_plan(modal: Rc<Self>, app: Rc<App>) {
        let visit_type_opt = modal.patient.lock_ref().as_ref().map(|pt| pt.visit_type());
        if let Some(visit_type) = visit_type_opt {
            let save = IndexPlanSave {
                visit_type,
                plan_id: modal.plan_id.get().and_then(zero_none),
                order_item_id: zero_none(modal.order_item_id.get()),
                plan_detail: str_some(modal.plan_detail.get_cloned()),
                plan_date: date_8601(&modal.plan_date.lock_ref()),
                plan_time: time_8601(&modal.plan_time.lock_ref()),
                plan_sch_type: Some(modal.active_tab.lock_ref().as_str().to_owned()),
            };
            app.async_load(
                true,
                clone!(app => async move {
                    // POST `EndPoint::IpdIndexPlan`
                    // POST `EndPoint::OpdErIndexPlan`
                    match save.call_api_post(app.state()).await {
                        Ok((id, results)) => if id > 0 && results[0].rows_affected > 0 {
                            modal.set_no_plan();
                            modal.parent_need_reload.set_neq(true);
                            modal.loaded.set(false);
                        }
                        Err(e) => {
                            app.alert_app_error(&e).await;
                        }
                    }
                }),
            )
        }
    }

    fn submit_plan_new_now(modal: Rc<Self>, app: Rc<App>) {
        let visit_type_opt = modal.patient.lock_ref().as_ref().map(|pt| pt.visit_type());
        if let Some(visit_type) = visit_type_opt {
            let now = js_now();
            let save = IndexPlanSave {
                visit_type,
                plan_id: Some(0),
                order_item_id: zero_none(modal.order_item_id.get()),
                plan_detail: None,
                plan_date: Some(now.date()),
                plan_time: Some(now.time()),
                plan_sch_type: Some(modal.active_tab.lock_ref().as_str().to_owned()),
            };
            app.async_load(
                true,
                clone!(app => async move {
                    // POST `EndPoint::IpdIndexPlan`
                    // POST `EndPoint::OpdErIndexPlan`
                    match save.call_api_post(app.state()).await {
                        Ok((id, results)) => if id > 0 && results[0].rows_affected > 0 {
                            modal.set_no_plan();
                            modal.parent_need_reload.set_neq(true);
                            modal.loaded.set(false);
                        }
                        Err(e) => {
                            app.alert_app_error(&e).await;
                        }
                    }
                }),
            )
        }
    }

    fn submit_plan_new_and_action_now(modal: Rc<Self>, app: Rc<App>) {
        let visit_type_opt = modal.patient.lock_ref().as_ref().map(|pt| pt.visit_type());
        if let Some(visit_type) = visit_type_opt {
            let now = js_now();
            let plan_save = IndexPlanSave {
                visit_type: visit_type.clone(),
                plan_id: Some(0),
                order_item_id: zero_none(modal.order_item_id.get()),
                plan_detail: None,
                plan_date: Some(now.date()),
                plan_time: Some(now.time()),
                plan_sch_type: Some(modal.active_tab.lock_ref().as_str().to_owned()),
            };
            app.async_load(
                true,
                clone!(app, visit_type => async move {
                    // POST `EndPoint::IpdIndexPlan`
                    // POST `EndPoint::OpdErIndexPlan`
                    match plan_save.call_api_post(app.state()).await {
                        Ok((plan_id, results)) => if plan_id > 0 && results[0].rows_affected > 0 {
                            let (check_datetime, check_person) = if modal.is_med() {
                                (Some(now + Duration::from_secs(9)), app.doctor_code())
                            } else {
                                (None, None)
                            };
                            let next = now + Duration::from_secs(18);
                            let action_save = IndexAction {
                                visit_type,
                                plan_id: Some(plan_id),
                                action_date: Some(next.date()),
                                action_time: Some(next.time()),
                                check_datetime,
                                check_person,
                                action_person_1: app.doctor_code(),
                                ..Default::default()
                            };
                            // POST `EndPoint::IpdIndexAction`
                            // POST `EndPoint::OpdErIndexAction`
                            match action_save.call_api_post(app.state()).await {
                                Ok((id, results)) => if id > 0 && results[0].rows_affected > 0 {
                                    modal.set_no_action();
                                }
                                Err(e) => {
                                    app.alert_app_error(&e).await;
                                }
                            }
                            modal.set_no_plan();
                            modal.parent_need_reload.set_neq(true);
                            modal.loaded.set(false);
                        }
                        Err(e) => {
                            app.alert_app_error(&e).await;
                        }
                    }
                }),
            )
        }
    }

    /// only for multiple oneday `time` sch_type, add new plans only
    fn submit_plan_datetimes(modal: Rc<Self>, app: Rc<App>) {
        let visit_type_opt = modal.patient.lock_ref().as_ref().map(|pt| pt.visit_type());
        if let Some(visit_type) = visit_type_opt {
            let saves = modal
                .plan_add_datetimes
                .lock_ref()
                .iter()
                .map(|plan_datetime| IndexPlanSave {
                    visit_type: visit_type.clone(),
                    plan_id: Some(0),
                    order_item_id: zero_none(modal.order_item_id.get()),
                    plan_detail: None,
                    plan_date: Some(plan_datetime.date()),
                    plan_time: Some(plan_datetime.time()),
                    plan_sch_type: Some(String::from("time")),
                })
                .collect::<Vec<IndexPlanSave>>();
            Self::submit_plans(saves, modal, app);
        }
    }

    fn submit_plans(saves: Vec<IndexPlanSave>, modal: Rc<Self>, app: Rc<App>) {
        app.async_load(
            true,
            clone!(app => async move {
                let mut success = true;
                for save in saves {
                    // POST `EndPoint::IpdIndexPlan`
                    // POST `EndPoint::OpdErIndexPlan`
                    if let Err(e) = save.call_api_post(app.state()).await {
                        app.alert_app_error(&e).await;
                        success = false;
                        break;
                    }
                }
                if success {
                    modal.plan_add_datetimes.lock_mut().clear();
                    modal.set_no_plan();
                    modal.parent_need_reload.set_neq(true);
                    modal.loaded.set(false);
                }
            }),
        )
    }

    fn submit_check(modal: Rc<Self>, app: Rc<App>) {
        let visit_type_opt = modal.patient.lock_ref().as_ref().map(|pt| pt.visit_type());
        if let Some(visit_type) = visit_type_opt {
            let save = IndexAction {
                visit_type,
                plan_id: modal.plan_id.get().and_then(zero_none),
                action_id: modal.action_id.get().and_then(zero_none),
                check_datetime: datetime_8601(&modal.check_datetime.lock_ref()),
                check_person: str_some(modal.check_person.get_cloned()),
                ..Default::default()
            };
            app.async_load(
                true,
                clone!(app => async move {
                    // POST `EndPoint::IpdIndexAction`
                    // POST `EndPoint::OpdErIndexAction`
                    match save.call_api_post(app.state()).await {
                        Ok((id, results)) => if id > 0 && results[0].rows_affected > 0 {
                            modal.set_no_action();
                            modal.parent_need_reload.set_neq(true);
                            modal.loaded.set(false);
                        }
                        Err(e) => {
                            app.alert_app_error(&e).await;
                        }
                    }
                }),
            )
        }
    }

    fn submit_action(modal: Rc<Self>, app: Rc<App>) {
        let visit_type_opt = modal.patient.lock_ref().as_ref().map(|pt| pt.visit_type());
        if let Some(visit_type) = visit_type_opt {
            let save = IndexAction {
                visit_type,
                plan_id: modal.plan_id.get().and_then(zero_none),
                action_id: modal.action_id.get().and_then(zero_none),
                check_datetime: datetime_8601(&modal.check_datetime.lock_ref()),
                check_person: str_some(modal.check_person.get_cloned()),
                action_result: str_some(modal.action_result.get_cloned()),
                action_remark: str_some(modal.action_remark.get_cloned()),
                action_date: date_8601(&modal.action_date.lock_ref()),
                action_time: time_8601(&modal.action_time.lock_ref()),
                action_report_back: str_some(modal.action_report_back.get_cloned()),
                action_blood_had: str_some(modal.action_blood_had.get_cloned()),
                action_person_1: str_some(modal.action_person_1.get_cloned()),
                action_person_2: str_some(modal.action_person_2.get_cloned()),
                ..Default::default()
            };
            app.async_load(
                true,
                clone!(app => async move {
                    // POST `EndPoint::IpdIndexAction`
                    // POST `EndPoint::OpdErIndexAction`
                    match save.call_api_post(app.state()).await {
                        Ok((id, results)) => if id > 0 && results[0].rows_affected > 0 {
                            let is_dtx_or_hct = modal.order_item.get_cloned().map(|oi| oi.has_dtx() || oi.has_hct()).unwrap_or_default();
                            let has_vs_id = modal.vs_id.get().is_some();
                            if !is_dtx_or_hct || has_vs_id {
                                modal.set_no_action();
                                modal.loaded.set(false);
                            } else {
                                // prepare for add_vs()
                                modal.action_id.set(Some(id));
                                modal.changed.set(false);
                            }
                            modal.parent_need_reload.set_neq(true);
                        }
                        Err(e) => {
                            app.alert_app_error(&e).await;
                        }
                    }
                }),
            )
        }
    }

    fn submit_monitor(modal: Rc<Self>, app: Rc<App>) {
        let visit_type_opt = modal.patient.lock_ref().as_ref().map(|pt| pt.visit_type());
        if let (Some(visit_type), Some(action_id), Some(monitor_doctor)) = (visit_type_opt, modal.action_id.get().and_then(zero_none), app.doctor_code()) {
            let save = IndexMonitor {
                visit_type,
                action_id,
                monitor_id: modal.monitor_id.get().and_then(zero_none),
                monitor_datetime: datetime_8601(&modal.monitor_datetime.lock_ref()),
                monitor_doctor,
                monitor_abnormal: modal.monitor_abnormal.get_cloned(),
                monitor_result: str_some(modal.monitor_result.get_cloned()),
                monitor_remark: str_some(modal.monitor_remark.get_cloned()),
                monitor_doctor_name: None,
            };
            app.async_load(
                true,
                clone!(app => async move {
                    // POST `EndPoint::IpdIndexMonitor`
                    // POST `EndPoint::OpdErIndexMonitor`
                    match save.call_api_post(app.state()).await {
                        Ok((id, results)) => if id > 0 && results[0].rows_affected > 0 {
                            modal.set_no_monitor();
                            modal.parent_need_reload.set_neq(true);
                            modal.loaded.set(false);
                            modal.action_id.set(modal.action_id.get());
                        }
                        Err(e) => {
                            app.alert_app_error(&e).await;
                        }
                    }
                }),
            )
        }
    }

    fn add_vs(saver: VitalSignSave, modal: Rc<Self>, app: Rc<App>) {
        if let Some(patient) = modal.patient.get_cloned() {
            app.async_load(
                true,
                clone!(app => async move {
                    let method = match modal.vs_id.get() {
                        Some(_) => "PUT",
                        None => "POST",
                    };
                    let (params_opt, is_ipd) = match &patient.visit_type {
                        VisitTypeId::Ipd(an) | VisitTypeId::PreAdmit(an) => {
                            (Some(VitalSignParams {
                                hn: patient.hn(),
                                an: Some(an.to_owned()),
                                ..Default::default()
                            }), true)
                        }
                        VisitTypeId::OpdEr(_, opd_er_order_master_id) => {
                            (Some(VitalSignParams {
                                opd_er_order_master_id: Some(*opd_er_order_master_id),
                                ..Default::default()
                            }), false)
                        }
                        VisitTypeId::Visit(_) => {
                            (None, false)
                        }
                    };
                    if let Some(params) = params_opt {
                        // POST `EndPoint::IpdVitalSign`
                        // PUT `EndPoint::IpdVitalSign`
                        // POST `EndPoint::OpdErVitalSign`
                        // PUT `EndPoint::OpdErVitalSign`
                        match saver.call_api_save(is_ipd, method, &params, app.state()).await {
                            Ok(response) => {
                                app.alert_execute_response(&response, async move {
                                    if zero_none(response.last_insert_id as u32).is_some() {
                                        modal.set_no_action();
                                        modal.loaded.set(false);
                                    }
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

    fn delete_index_plan(modal: Rc<Self>, app: Rc<App>) {
        app.async_load(
            true,
            clone!(app, modal => async move {
                if app.confirm("ยืนยันลบรายการ").await {
                    if let Some(plan_id) = modal.plan_id.get().and_then(zero_none) {
                        // DELETE `EndPoint::IpdIndexPlanId`
                        // DELETE `EndPoint::OpdErIndexPlanId`
                        match IndexPlan::call_api_delete(modal.is_ipd(), plan_id, app.state()).await {
                            Ok(response) => {
                                if response.rows_affected > 0 {
                                    modal.set_no_plan();
                                    modal.loaded.set(false);
                                    modal.parent_need_reload.set_neq(true);
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

    fn delete_index_action(modal: Rc<Self>, app: Rc<App>) {
        app.async_load(
            true,
            clone!(app, modal => async move {
                if app.confirm("ยืนยันลบรายการ").await {
                    if let Some(action_id) = modal.action_id.get().and_then(zero_none) {
                        // DELETE `EndPoint::IpdIndexActionId`
                        // DELETE `EndPoint::OpdErIndexActionId`
                        match IndexAction::call_api_delete(modal.is_ipd(), action_id, app.state()).await {
                            Ok(response) => {
                                if response.rows_affected > 0 {
                                    modal.set_no_action();
                                    modal.loaded.set(false);
                                    modal.parent_need_reload.set_neq(true);
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

    fn delete_index_monitor(modal: Rc<Self>, app: Rc<App>) {
        app.async_load(
            true,
            clone!(app, modal => async move {
                if app.confirm("ยืนยันลบรายการ").await {
                    if let Some(monitor_id) = modal.monitor_id.get().and_then(zero_none) {
                        // DELETE `EndPoint::IpdIndexMonitorId`
                        // DELETE `EndPoint::OpdErIndexMonitorId`
                        match IndexMonitor::call_api_delete(modal.is_ipd(), monitor_id, app.state()).await {
                            Ok(response) => {
                                if response.rows_affected > 0 {
                                    modal.set_no_monitor();
                                    modal.loaded.set(false);
                                    modal.parent_need_reload.set_neq(true);
                                    modal.action_id.set(modal.action_id.get());
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

    pub fn render(modal: Rc<Self>, display: Mutable<Option<Rc<Self>>>, parent_reload: Option<Mutable<bool>>, app: Rc<App>) -> Dom {
        html!("div", {
            .apply_if(
                app.endpoint_is_allow(&Method::GET, &EndPoint::IpdShowPatientMainAn, false)
                || app.endpoint_is_allow(&Method::GET, &EndPoint::OpdErShowPatientMainVn, false)
                || app.endpoint_is_allow(&Method::GET, &EndPoint::OpdErShowPatientMainId, false),
            |dom| dom
                .future(map_ref!(
                    let busy = app.loader_is_loading(),
                    let has_visit_type = modal.visit_type.signal_cloned().map(|opt| opt.is_some()),
                    let has_patient = modal.patient.signal_cloned().map(|opt| opt.is_some()) =>
                    !busy && *has_visit_type && !has_patient
                ).for_each(clone!(app, modal => move |ready| {
                    if ready {
                        Self::load_patient_info(modal.clone(), app.clone());
                    }
                    async {}
                })))
            )
            .future(map_ref!(
                let busy = app.loader_is_loading(),
                let has_patient = modal.patient.signal_cloned().map(|pt| pt.is_some()),
                let loaded = modal.loaded.signal() =>
                !busy && *has_patient && !loaded
            ).for_each(clone!(app, modal => move |ready| {
                if ready {
                    Self::load(modal.clone(), app.clone());
                    modal.loaded.set_neq(true);
                }
                async {}
            })))
            .class(class::MODAL_DIALOG_LG_FULL)
            .attr("role", "document")
            .child(html!("div", {
                .class("modal-content")
                .children([
                    // header
                    html!("div", {
                        .class("modal-header")
                        .class_signal(class::ITEMS_LEFT_B0, modal.patient.signal_cloned().map(|opt| opt.is_some()))
                        .child_signal(modal.patient.signal_cloned().map(clone!(app => move |opt: Option<Rc<PatientInfo>>| {
                            opt.map(clone!(app => move |patient| {
                                render_patient_info(false, patient, None, None, true, app.clone())
                            })).or(Some(html!("h5", {.text("Index: Plan and Action (ไม่ระบุผู้ป่วย)")})))
                        })))
                        .child(html!("button", {
                            .attr("type", "button")
                            .class("btn-close")
                            .attr("data-bs-dismiss", "modal")
                            .attr("aria-label", "Close")
                            .event(clone!(modal, display, parent_reload => move |_: events::Click| {
                                if let Some(reload) = &parent_reload {
                                    reload.set_neq(modal.parent_need_reload.get());
                                }
                                display.set(None);
                            }))
                        }))
                    }),
                    html!("div", {
                        .class("modal-body")
                        .child(html!("div", {
                            // OrderItem
                            .child_signal(modal.order_item.signal_cloned().map(clone!(app => move |opt| {
                                opt.and_then(clone!(app => move |order_item| {
                                    // `without_order` not show this
                                    (order_item.order_item_id != 0).then(|| render_order_item(order_item, app))
                                })).or(Some(html!("div", {
                                    .class(class::BOX_ROUND_T)
                                    .text("รายการที่ไม่ผูกกับ Order")
                                })))
                            })))
                            // All Actions list
                            .child_signal(modal.order_item.signal_cloned().map(clone!(modal => move |order_item_opt| {
                                Self::render_actions_list(true, None, order_item_opt, modal.clone())
                            })))
                            // Plan / Action / Monitor toggle
                            .children([
                                html!("div", {
                                    .class(class::BTN_GROUP_T)
                                    .attr("role","group")
                                    .children([
                                        html!("input" => HtmlInputElement, {
                                            .attr("type", "radio")
                                            .class("btn-check")
                                            .attr("id", "main-plan-btn")
                                            .attr("autocomplete","off")
                                            .with_node!(element => {
                                                .future(modal.form_type.signal_cloned().for_each(move |v| {
                                                    if v == FormType::Plan {
                                                        element.set_checked(true);
                                                    } else {
                                                        element.set_checked(false);
                                                    }
                                                    async {}
                                                }))
                                                .event(clone!(modal => move |_: events::Click| {
                                                    modal.form_type.set_neq(FormType::Plan);
                                                    modal.set_no_plan();
                                                    modal.set_no_action();
                                                }))
                                            })
                                        }),
                                        html!("label", {
                                            .class(class::BTN_BLUEO)
                                            .attr("for", "main-plan-btn")
                                            .text("PLAN")
                                            .child_signal(modal.order_item.signal_cloned().map(move |opt| {
                                                opt.and_then(|order_item| {
                                                    let plan_len = order_item.index_plans.len();
                                                    (plan_len > 0).then(|| {
                                                        html!("span", {
                                                            .class(class::BADGE_GOLD_R)
                                                            .text(&plan_len.to_string())
                                                        })
                                                    })
                                                })
                                            }))
                                        }),
                                        html!("input" => HtmlInputElement, {
                                            .attr("type", "radio")
                                            .class("btn-check")
                                            .attr("id", "main-action-btn")
                                            .attr("autocomplete","off")
                                            .with_node!(element => {
                                                .future(modal.form_type.signal_cloned().for_each(move |v| {
                                                    if v == FormType::Action {
                                                        element.set_checked(true);
                                                    } else {
                                                        element.set_checked(false);
                                                    }
                                                    async {}
                                                }))
                                                .event(clone!(modal => move |_: events::Click| {
                                                    modal.form_type.set_neq(FormType::Action);
                                                    modal.set_no_plan();
                                                    modal.set_no_action();
                                                }))
                                            })
                                        }),
                                        html!("label", {
                                            .class(class::BTN_BLUEO)
                                            .attr("for", "main-action-btn")
                                            .text("ACTION")
                                            .child_signal(modal.order_item.signal_cloned().map(move |opt| {
                                                opt.and_then(|order_item| {
                                                    let action_len = order_item.index_plans.iter().map(|plan| plan.actions.len()).sum::<usize>();
                                                    (action_len > 0).then(|| {
                                                        html!("span", {
                                                            .class(class::BADGE_GOLD_R)
                                                            .text(&action_len.to_string())
                                                        })
                                                    })
                                                })
                                            }))
                                        }),
                                        html!("input" => HtmlInputElement, {
                                            .attr("type", "radio")
                                            .class("btn-check")
                                            .attr("id", "main-monitor-btn")
                                            .attr("autocomplete","off")
                                            .with_node!(element => {
                                                .future(modal.form_type.signal_cloned().for_each(move |v| {
                                                    if v == FormType::Monitor {
                                                        element.set_checked(true);
                                                    } else {
                                                        element.set_checked(false);
                                                    }
                                                    async {}
                                                }))
                                                .event(clone!(modal => move |_: events::Click| {
                                                    modal.form_type.set_neq(FormType::Monitor);
                                                    modal.set_no_plan();
                                                    modal.set_no_action();
                                                    modal.set_new_monitor(js_now());
                                                }))
                                            })
                                        }),
                                        html!("label", {
                                            .class(class::BTN_BLUEO)
                                            .attr("for", "main-monitor-btn")
                                            .text("MONITOR")
                                            .child_signal(modal.order_item.signal_cloned().map(move |opt| {
                                                opt.and_then(|order_item| {
                                                    let monitor_len = order_item.index_plans.iter().flat_map(|plan| plan.actions.iter().map(|action| action.monitors.len())).sum::<usize>();
                                                    (monitor_len > 0).then(|| {
                                                        html!("span", {
                                                            .class(class::BADGE_GOLD_R)
                                                            .text(&monitor_len.to_string())
                                                        })
                                                    })
                                                })
                                            }))
                                        }),
                                    ])
                                }),
                            ])
                            // Plan / Action content
                            .child_signal(modal.form_type.signal_cloned().map(clone!(modal, app => move |form_type| {
                                Some(match form_type {
                                    FormType::Plan => Self::render_plan_content(modal.clone(), app.clone()),
                                    FormType::Action => Self::render_action_content(modal.clone(), app.clone()),
                                    FormType::Monitor => Self::render_monitor_content(modal.clone(), app.clone()),
                                })
                            })))
                            // Footer
                            .child(html!("button", {
                                .attr("type", "button")
                                .class(class::BTN_FR_GRAY)
                                .attr("data-bs-dismiss", "modal")
                                .child(html!("i", {.class(class::FA_X)}))
                                .text(" ปิด")
                                .event(clone!(modal, parent_reload => move |_: events::Click| {
                                    if let Some(reload) = &parent_reload {
                                        reload.set_neq(modal.parent_need_reload.get());
                                    }
                                    display.set(None);
                                }))
                            }))
                        }))
                    }),
                ])
            }))
        })
    }

    fn render_plan_content(modal: Rc<Self>, app: Rc<App>) -> Dom {
        html!("div", {
            .class(class::BOX_ROUND_T)
            .children([
                // Tabs
                Self::render_tabs(modal.clone()),
                // Contents
                html!("div", {
                    .class("tab-content")
                    .child_signal(modal.active_tab.signal_cloned().map(clone!(modal => move |tab| {
                        Some(match tab {
                            PlanTab::Time => {
                                html!("div", {
                                    .child_signal(modal.order_item_plans_with_sch_type("time").map(clone!(modal, app => move |plans| {
                                        Some(Self::render_plan_sch_time(plans, modal.clone(), app.clone()))
                                    })))
                                })
                            }
                            PlanTab::Date => {
                                html!("div", {
                                    .child_signal(modal.order_item_plans_with_sch_type("date").map(clone!(modal, app => move |plans| {
                                        Some(Self::render_plan_sch_date(plans, modal.clone(), app.clone()))
                                    })))
                                })
                            }
                            PlanTab::Stat => {
                                html!("div", {
                                    .child_signal(modal.order_item_plans_with_sch_type("stat").map(clone!(modal, app => move |plans| {
                                        Some(Self::render_plan_sch_stat(plans, modal.clone(), app.clone()))
                                    })))
                                })
                            }
                        })
                    })))
                }),
            ])
        })
    }

    fn render_monitor_content(modal: Rc<Self>, app: Rc<App>) -> Dom {
        let actions = modal.plan_actions(None, modal.order_item.get_cloned(), true).unwrap_or_default();
        html!("div", {
            .class(class::BOX_ROUND_T)
            .child(html!("div", {
                .class(class::INPUT_GROUP_T)
                .child(html!("span", {
                    .class("input-group-text")
                    .text("เลือก Action")
                }))
                .apply(|dom| {
                    if actions.is_empty() {
                        dom.child(html!("span", {
                            .class("input-group-text")
                            .text("ยังไม่มี Action")
                        }))
                    } else {
                        let mut apply_selected = false;
                        dom.child(html!("select" => HtmlSelectElement, {
                            .class("form-select")
                            .style("max-width","200px")
                            .children(actions.iter().filter_map(|action| {
                                action.action_id.map(|action_id| {
                                    let action_dt_opt = datetime_from_opt(action.action_date, action.action_time);
                                    let label = if let Some(action_datetime) = action_dt_opt {
                                        datetime_th_relative(&action_datetime)
                                    } else {
                                        String::from("ไม่ระบุวัน-เวลา")
                                    };
                                    if !apply_selected {
                                        apply_selected = true;
                                        modal.action_id.set_neq(Some(action_id));
                                        modal.action_datetime.set(action_dt_opt);
                                        modal.action_result.set_neq(action.action_result.clone().unwrap_or_default());
                                    }
                                    html!("option", {.attr("value", &action_id.to_string()).text(&label)})
                                })
                            }))
                            .prop_signal("value", modal.action_id.signal().map(|opt| opt.map(|u| u.to_string()).unwrap_or_default()))
                            .with_node!(element => {
                                .event(clone!(modal => move |_:events::Change| {
                                    let action_id = element.value().parse::<u32>().ok();
                                    modal.action_id.set_neq(action_id);
                                    if let Some(selected) = actions.iter().find(|action| action.action_id == action_id) {
                                        modal.action_result.set_neq(selected.action_result.clone().unwrap_or_default());
                                        modal.action_datetime.set(datetime_from_opt(selected.action_date, selected.action_time));
                                    }
                                }))
                            })
                        }))
                    }
                })
                .child(html!("input", {
                    .class("form-control")
                    .attr("disabled","")
                    .prop_signal("value", modal.action_result.signal_cloned())
                }))
            }))
            // monitors
            .child_signal(map_ref!{
                let action_id_opt = modal.action_id.signal(),
                let order_item = modal.order_item.signal_cloned() =>
                (*action_id_opt, order_item.clone())
            }.map(clone!(modal => move |(action_id_opt, order_item)| {
                action_id_opt.map(|action_id| {
                    let monitors = modal.action_monitors(action_id, order_item);
                    let latest_monitor_datetime = monitors.first().and_then(|monitor| monitor.monitor_datetime).or(modal.action_datetime.get());
                    html!("div", {
                        // .class("overflow-auto")
                        // .style("max-height","81px")
                        .children([
                            html!("button", {
                                .attr("type","button")
                                .class(class::BTN_SM_LT_CYAN)
                                // .child(html!("i", {.class(class::FA_PLUS_L)}))
                                .text("ขณะ Action")
                                .event(clone!(modal => move |_: events::Click| {
                                    modal.set_new_monitor(latest_monitor_datetime.unwrap_or(js_now()));
                                }))
                            }),
                            html!("button", {
                                .attr("type","button")
                                .class(class::BTN_SM_LT_CYAN)
                                .child(html!("i", {.class(class::FA_PLUS_L)}))
                                .text("5 นาที")
                                .attr("title","5 นาที นับจากการ Monitor ครั้งล่าสุด")
                                .event(clone!(modal => move |_: events::Click| {
                                    modal.set_new_monitor(latest_monitor_datetime.map(|dt| dt + Duration::from_mins(5)).unwrap_or(js_now()));
                                }))
                            }),
                            html!("button", {
                                .attr("type","button")
                                .class(class::BTN_SM_LT_CYAN)
                                .child(html!("i", {.class(class::FA_PLUS_L)}))
                                .text("15 นาที")
                                .attr("title","15 นาที นับจากการ Monitor ครั้งล่าสุด")
                                .event(clone!(modal => move |_: events::Click| {
                                    modal.set_new_monitor(latest_monitor_datetime.map(|dt| dt + Duration::from_mins(15)).unwrap_or(js_now()));
                                }))
                            }),
                            html!("button", {
                                .attr("type","button")
                                .class(class::BTN_SM_LT_CYAN)
                                .child(html!("i", {.class(class::FA_PLUS_L)}))
                                .text("30 นาที")
                                .attr("title","30 นาที นับจากการ Monitor ครั้งล่าสุด")
                                .event(clone!(modal => move |_: events::Click| {
                                    modal.set_new_monitor(latest_monitor_datetime.map(|dt| dt + Duration::from_mins(30)).unwrap_or(js_now()));
                                }))
                            }),
                            html!("button", {
                                .attr("type","button")
                                .class(class::BTN_SM_LT_CYAN)
                                .child(html!("i", {.class(class::FA_PLUS_L)}))
                                .text("60 นาที")
                                .attr("title","60 นาที นับจากการ Monitor ครั้งล่าสุด")
                                .event(clone!(modal => move |_: events::Click| {
                                    modal.set_new_monitor(latest_monitor_datetime.map(|dt| dt + Duration::from_mins(60)).unwrap_or(js_now()));
                                }))
                            }),
                            html!("button", {
                                .attr("type","button")
                                .class(class::BTN_SM_LT)
                                .class_signal("btn-primary", modal.monitor_id.signal().map(|opt| opt == Some(0)))
                                .class_signal("btn-secondary", modal.monitor_id.signal().map(|opt| opt != Some(0)))
                                .child(html!("i", {.class(class::FA_PLUS_L)}))
                                .text("เพิ่ม")
                                .attr("title","เพิ่ม Monitor โดยใช้เวลาปัจจุบัน")
                                .event(clone!(modal => move |_: events::Click| {
                                    modal.set_new_monitor(js_now());
                                }))
                            }),
                        ])
                        .children(monitors.into_iter().map(|monitor| {
                            let this_id = monitor.monitor_id;
                            let diff_minutes = if let (Some(action_dt), Some(monitor_dt)) = (modal.action_datetime.get(), monitor.monitor_datetime) {
                                (monitor_dt - action_dt).whole_minutes()
                            } else {
                                0
                            };
                            html!("button", {
                                .attr("type","button")
                                .class(class::BTN_SM_LT)
                                .class_signal("btn-primary", modal.monitor_id.signal().map(move |monitor_id| monitor_id == this_id))
                                .class_signal("btn-secondary", modal.monitor_id.signal().map(move |monitor_id| monitor_id != this_id))
                                .apply(|d| {
                                    match &monitor.monitor_abnormal {
                                        Some(monitor_abnormal) => if monitor_abnormal.as_str() == "Y" {
                                            d.child(html!("i", {.class(class::FA_ALERT_RED).class("me-1")}))
                                        } else {
                                            d.child(html!("i", {.class(class::FA_CHECK_CIRCLE_GREEN).class("me-1")}))
                                        },
                                        None => {
                                            d.child(html!("i", {.class(class::FA_HOURGLASS_GOLD).class("me-1")}))
                                        },
                                    }
                                })
                                .text(&datetime_th_opt_relative(&monitor.monitor_datetime))
                                .text(&[" (", &diff_minutes.to_string(), " นาที)"].concat())
                                .attr("title", &["รายละเอียด : ", &monitor.monitor_result.clone().unwrap_or_default(),"\nบันทึกโดย : ", &monitor.monitor_doctor_name.clone().unwrap_or_default()].concat())
                                .event(clone!(modal => move |_: events::Click| {
                                    modal.set_monitor(&monitor);
                                }))
                            })
                        }))
                    })
                })
            })))
            // Input Form
            .child_signal(map_ref! {
                let has_action_id = modal.action_id.signal().map(|opt| opt.and_then(zero_none).is_some()),
                let has_monitor_id = modal.monitor_id.signal().map(|opt| opt.is_some()) =>
                *has_action_id && *has_monitor_id
            }.map(clone!(app, modal => move |ready| {
                ready.then(|| {
                    Self::render_monitor_inputs(modal.clone(), app.clone())
                })
            })))
        })
    }

    fn render_action_content(modal: Rc<Self>, app: Rc<App>) -> Dom {
        html!("div", {
            .class(class::BOX_ROUND_T)
            // Plan Detail
            .child_signal(modal.plan_detail.signal_cloned().map(|plan_detail| {
                (!plan_detail.is_empty()).then(|| {
                    html!("div", {
                        .class(class::INPUT_GROUP_T)
                        .children([
                            doms::span_group_text("รายละเอียดแผน"),
                            html!("input", {.class("form-control").attr("disabled","").attr("value", &plan_detail)})
                        ])
                    })
                })
            }))
            .children([
                // Tabs
                Self::render_tabs(modal.clone()),
                // Content
                html!("div", {
                    .class("tab-content")
                    .child(html!("nav", {
                        .child_signal(modal.active_tab.signal_cloned().map(clone!(modal => move |tab| {
                            Some(match tab {
                                PlanTab::Time => {
                                    html!("div", {
                                        .child_signal(modal.order_item_plans_with_sch_type("time").map(clone!(modal => move |plans| {
                                            Some(Self::render_action_sch_time(plans, modal.clone()))
                                        })))
                                    })
                                }
                                PlanTab::Date => {
                                    html!("div", {
                                        .child_signal(modal.order_item_plans_with_sch_type("date").map(clone!(modal => move |plans| {
                                            Some(Self::render_action_sch_date(plans, modal.clone()))
                                        })))
                                    })
                                }
                                PlanTab::Stat => {
                                    html!("div", {
                                        .child_signal(modal.order_item_plans_with_sch_type("stat").map(clone!(modal => move |plans| {
                                            Some(Self::render_action_sch_stat(plans, modal.clone()))
                                        })))
                                    })
                                }
                            })
                        })))
                    }))
                }),
            ])
            .child(html!("div", {
                // Actions list
                .child_signal(map_ref!{
                    let plan_id_opt = modal.plan_id.signal(),
                    let order_item_opt = modal.order_item.signal_cloned() =>
                    (plan_id_opt.to_owned(), order_item_opt.clone())
                }.map(clone!(modal => move |(plan_id_opt, order_item_opt)|{
                    Self::render_actions_list(false, plan_id_opt, order_item_opt, modal.clone())
                })))
                // Input Form
                .child_signal(modal.action_id.signal_cloned().map(clone!(modal, app => move |opt| {
                    opt.is_some().then(|| {
                        Self::render_action_inputs(modal.clone(), app.clone())
                    })
                })))
            }))
        })
    }

    fn render_tabs(modal: Rc<Self>) -> Dom {
        html!("div", {
            .class(class::NAV_TABS_T)
            .children([
                // Time tab
                html!("a", {
                    .class(class::NAV_ITEM_LINK)
                    .class_signal("active", modal.active_tab.signal_cloned().map(|tab| matches!(tab, PlanTab::Time)))
                    .attr("data-bs-toggle", "tab")
                    .attr("href", "#")
                    .attr("role", "tab")
                    .text("ตั้งเวลา")
                    .child_signal(modal.order_item_plans_with_sch_type("time").map(|plans| {
                        (!plans.is_empty()).then(|| {
                            html!("span", {
                                .class(class::BADGE_BLUE_R)
                                .style("cursor","default")
                                .text(&plans.len().to_string())
                            })
                        })
                    }))
                    .event_with_options(&EventOptions::preventable(), clone!(modal => move |event: events::Click| {
                        event.prevent_default();
                        modal.active_tab.set_neq(PlanTab::Time);
                        modal.plan_id.set_neq(None);
                        modal.action_id.set_neq(None);
                    }))
                }),
                // Date tab
                html!("a", {
                    .class(class::NAV_ITEM_LINK)
                    .class_signal("active", modal.active_tab.signal_cloned().map(|tab| matches!(tab, PlanTab::Date)))
                    .attr("data-bs-toggle", "tab")
                    .attr("href", "#")
                    .attr("role", "tab")
                    .text("ไม่ระบุเวลา (PRN)")
                    .child_signal(modal.order_item_plans_with_sch_type("date").map(|plans| {
                        (!plans.is_empty()).then(|| {
                            html!("span", {
                                .class(class::BADGE_BLUE_R)
                                .style("cursor","default")
                                .text(&plans.len().to_string())
                            })
                        })
                    }))
                    .event_with_options(&EventOptions::preventable(), clone!(modal => move |event: events::Click| {
                        event.prevent_default();
                        modal.active_tab.set_neq(PlanTab::Date);
                        modal.plan_id.set_neq(None);
                        modal.action_id.set_neq(None);
                    }))
                }),
            ])
            // Stat tab
            .child_signal(modal.is_stat_signal().map(move |is_stat| {
                is_stat.then(|| {
                    html!("a", {
                        .class(class::NAV_ITEM_LINK)
                        .class_signal("active", modal.active_tab.signal_cloned().map(|tab| matches!(tab, PlanTab::Stat)))
                        .attr("data-bs-toggle", "tab")
                        .attr("href", "#")
                        .attr("role", "tab")
                        .text("ทันที (STAT)")
                        .child_signal(modal.order_item_plans_with_sch_type("stat").map(|plans| {
                            (!plans.is_empty()).then(|| {
                                html!("span", {
                                    .class(class::BADGE_BLUE_R)
                                    .style("cursor","default")
                                    .text(&plans.len().to_string())
                                })
                            })
                        }))
                        .event_with_options(&EventOptions::preventable(), clone!(modal => move |event: events::Click| {
                            event.prevent_default();
                            modal.active_tab.set_neq(PlanTab::Stat);
                            modal.plan_id.set_neq(None);
                            modal.action_id.set_neq(None);
                        }))
                    })
                })
            }))
        })
    }

    fn render_plan_sch_stat(stat_plans: Vec<Rc<IndexPlan>>, modal: Rc<Self>, app: Rc<App>) -> Dom {
        let plan_times = stat_plans.iter().filter_map(|plan| plan.plan_time).collect::<Vec<Time>>();
        let (is_ipd, is_pre_admit) = modal.is_ipd_and_is_pre_admit();
        html!("nav", {
            .child(html!("div", {
                .class(class::NAV_PILLS_T)
                .attr("role", "tablist")
                .children(stat_plans.iter().map(clone!(modal => move |plan| {
                    Self::plan_dom(plan.clone(), modal.clone(), None)
                })))
                // `stat` can has only 1 plan
                .apply_if(
                    plan_times.is_empty()
                    && if is_ipd {
                        app.has_permission(Permission::IpdNurseIndexAdd)
                        || (is_pre_admit && app.has_permission(Permission::OpdErNurseIndexAdd))
                    } else {
                        app.has_permission(Permission::OpdErNurseIndexAdd)
                    },
                |dom| dom
                    .children([
                        Self::plan_dom_new(modal.clone(), None),
                        Self::plan_dom_new_now(modal.clone(), app.clone()),
                        Self::plan_dom_new_and_action_now(modal.clone(), app.clone()),
                    ])
                )
            }))
            // Input Form
            .child_signal(modal.plan_id.signal_cloned().map(clone!(modal => move |opt| {
                opt.is_some().then(|| {
                    Self::render_plan_inputs(modal.clone(), app.clone())
                })
            })))
        })
    }

    fn render_plan_sch_date(date_plans: Vec<Rc<IndexPlan>>, modal: Rc<Self>, app: Rc<App>) -> Dom {
        let plan_times = date_plans.iter().filter_map(|plan| plan.plan_time).collect::<Vec<Time>>();
        let (is_ipd, is_pre_admit) = modal.is_ipd_and_is_pre_admit();
        html!("nav", {
            .child(html!("div", {
                .class(class::NAV_PILLS_T)
                .attr("role", "tablist")
                .children(date_plans.iter().map(|plan| {
                    let label = if plan.is_continuous_or_none() {
                        ["ตั้งแต่ ", &date_th_opt(&plan.plan_date), " ", &time_hm_opt(&plan.plan_time)].concat()
                    } else {
                        [&date_th_opt(&plan.plan_date), " ", &time_hm_opt(&plan.plan_time), " - ", &date_th_opt(&plan.plan_date.and_then(|d| d.next_day())), " ", &time_hm_opt(&plan.plan_time)].concat()
                    };
                    Self::plan_dom(plan.clone(), modal.clone(), Some(label))
                }))
                // `date` (prn) can has only 1 plan
                .apply_if(
                    plan_times.is_empty()
                    && if is_ipd {
                        app.has_permission(Permission::IpdNurseIndexAdd)
                        || (is_pre_admit && app.has_permission(Permission::OpdErNurseIndexAdd))
                    } else {
                        app.has_permission(Permission::OpdErNurseIndexAdd)
                    },
                |dom| dom
                    .children([
                        Self::plan_dom_new(modal.clone(), None),
                        Self::plan_dom_new_now(modal.clone(), app.clone()),
                        Self::plan_dom_new_and_action_now(modal.clone(), app.clone()),
                    ])
                )
            }))
            // Input Form
            .child_signal(modal.plan_id.signal_cloned().map(clone!(modal => move |opt| {
                opt.is_some().then(|| {
                    Self::render_plan_inputs(modal.clone(), app.clone())
                })
            })))
        })
    }

    fn render_plan_sch_time(time_plans: Vec<Rc<IndexPlan>>, modal: Rc<Self>, app: Rc<App>) -> Dom {
        html!("div", {
            .class("accordion")
            .attr("id", "plan-sch-time-container")
            .children([
                html!("div", {
                    .class("accordion-item")
                    .children([
                        html!("div", {
                            .class("accordion-header")
                            .child(html!("button", {
                                .attr("type", "button")
                                .class(class::ACCORDION_BTN_CYANS_P2)
                                .attr("data-bs-toggle","collapse")
                                .attr("data-bs-target","#plan-sch-time-hours")
                                .attr("aria-expanded","true")
                                .attr("aria-controls","plan-sch-time-hours")
                                .text("กำหนดเวลา")
                            }))
                        }),
                        html!("div", {
                            .class(class::ACCORDION_COLLAPSE_SHOW)
                            .attr("id", "plan-sch-time-hours")
                            .attr("data-bs-parent","#plan-sch-time-container")
                            .child(html!("div", {
                                .class("accordion-body")
                                .child(Self::render_plan_sch_time_multiple(time_plans.clone(), modal.clone(), app.clone()))
                            }))
                        }),
                    ])
                }),
                html!("div", {
                    .class("accordion-item")
                    .children([
                        html!("div", {
                            .class("accordion-header")
                            .child(html!("button", {
                                .attr("type", "button")
                                .class(class::ACCORDION_BTN_COLLAPSED_CYANS_P2)
                                .attr("data-bs-toggle","collapse")
                                .attr("data-bs-target","#plan-sch-time-single")
                                .attr("aria-expanded","false")
                                .attr("aria-controls","plan-sch-time-single")
                                .text("รายละเอียด")
                            }))
                        }),
                        html!("div", {
                            .class(class::ACCORDION_COLLAPSE)
                            .attr("id", "plan-sch-time-single")
                            .attr("data-bs-parent","#plan-sch-time-container")
                            .child(html!("div", {
                                .class("accordion-body")
                                .child(Self::render_plan_sch_time_single(time_plans, modal.clone(), app.clone()))
                                // Input Form
                                .child_signal(modal.plan_id.signal_cloned().map(clone!(modal => move |opt| {
                                    opt.is_some().then(|| {
                                        Self::render_plan_inputs(modal.clone(), app.clone())
                                    })
                                })))
                            }))
                        }),
                    ])
                }),
            ])
        })
    }
    fn render_plan_sch_time_multiple(time_plans: Vec<Rc<IndexPlan>>, modal: Rc<Self>, app: Rc<App>) -> Dom {
        let order_hr = modal.order_item.lock_ref().as_ref().and_then(|oi| oi.order_time.map(|ot| ot.hour())).unwrap_or(js_now().time().hour());
        // set start hour with min plan datetime / order time /now
        let start_hr = time_plans
            .iter()
            .filter_map(|plan| datetime_from_opt(plan.plan_date, plan.plan_time))
            .min()
            .map(|dt| dt.hour())
            .unwrap_or(order_hr);
        modal.plan_time_start_hour.set_neq(start_hr);
        let (is_ipd, is_pre_admit) = modal.is_ipd_and_is_pre_admit();

        html!("div", {
            .children([
                // multiple hours
                html!("div", {
                    .class(class::FLEX_WRAP_JC_T)
                    // .apply_if(is_continuous, |dom| dom
                        .child(html!("button", {
                            .attr("type", "button")
                            .class(class::BTN_LB2_BLUE)
                            .child(html!("i", {.class(class::FA_L_ARROW)}))
                            .event(clone!(modal => move |_: events::Click| {
                                let current_start_hour = modal.plan_time_start_hour.get();
                                let new_start_hour = if current_start_hour == 0 {23} else {current_start_hour - 1};
                                modal.plan_time_start_hour.set(new_start_hour);
                            }))
                        }))
                    // )
                    // .apply(|dom| {
                    //     if is_continuous {
                            // dom
                            .children_signal_vec(Self::plan_24hr_buttons(order_hr, time_plans, modal.clone()))
                    //     } else {
                    //         dom.children_signal_vec(Self::plan_until_next_nth_buttons(12, order_hr, time_plans, modal.clone()))
                    //     }
                    // })
                    // .apply_if(is_continuous, |dom| dom
                        .child(html!("button", {
                            .attr("type", "button")
                            .class(class::BTN_LB2_BLUE)
                            .child(html!("i", {.class(class::FA_R_ARROW)}))
                            .event(clone!(modal => move |_: events::Click| {
                                let current_start_hour = modal.plan_time_start_hour.get();
                                let new_start_hour = if current_start_hour == 23 {0} else {current_start_hour + 1};
                                modal.plan_time_start_hour.set(new_start_hour);
                            }))
                        }))
                    // )
                }),
                // control buttons
                html!("div", {
                    .class("row")
                    .child(html!("div", {
                        .class(class::COL12_Y)
                        .child_signal(modal.is_add_hours_input_empty_signal().map(clone!(modal => move |is_empty| {
                            (!is_empty).then(|| {
                                html!("button", {
                                    .attr("type", "button")
                                    .class(class::BTN_FR_GRAY)
                                    .child(html!("i", {.class(class::FA_X)}))
                                    .text(" ยกเลิก")
                                    .event(clone!(modal => move |_: events::Click| {
                                        modal.plan_add_datetimes.lock_mut().clear();
                                    }))
                                })
                            })
                        })))
                        .apply_if(if is_ipd {
                            (app.has_permission(Permission::IpdNurseIndexAdd) || (is_pre_admit && app.has_permission(Permission::OpdErNurseIndexAdd)))
                            && app.endpoint_is_allow(&Method::POST, &EndPoint::IpdIndexPlan, is_pre_admit)
                        } else {
                            app.has_permission(Permission::OpdErNurseIndexAdd)
                            && app.endpoint_is_allow(&Method::POST, &EndPoint::OpdErIndexPlan, false)
                        }, clone!(app, modal => move |dom| {
                            dom.child(html!("button" => HtmlButtonElement, {
                                .attr("type", "button")
                                .class(class::BTN_FR_L)
                                .class_signal("btn-primary", not(modal.is_add_hours_input_empty_signal()))
                                .class_signal("btn-secondary", modal.is_add_hours_input_empty_signal())
                                .child(html!("i", {.class(class::FA_SAVE)}))
                                .text(" บันทึก")
                                .apply(mixins::click_with_loader_checked_or_true_disable_signal(clone!(app, modal => move || {
                                    Self::submit_plan_datetimes(modal.clone(), app.clone());
                                }), modal.is_add_hours_input_empty_signal(), app.state()))
                            }))
                            .children_signal_vec(modal.is_add_hours_input_empty_signal().map(clone!(app, modal => move |is_empty| {
                                if is_empty {
                                    vec![
                                        Self::plan_dom_new_now(modal.clone(), app.clone()),
                                        Self::plan_dom_new_and_action_now(modal.clone(), app.clone()),
                                    ]
                                } else {
                                    Vec::new()
                                }
                            })).to_signal_vec())
                        }))
                    }))
                }),
            ])
        })
    }

    fn plan_24hr_buttons(order_hr: u8, time_plans: Vec<Rc<IndexPlan>>, modal: Rc<Self>) -> impl SignalVec<Item = Dom> {
        modal
            .plan_time_start_hour
            .signal()
            .map(clone!(modal => move |start_hr| {
                let times = if start_hr == 0 {
                    (0u8..24u8).collect::<Vec<u8>>()
                } else {
                    (start_hr..24).chain(0..start_hr).collect::<Vec<u8>>()
                };
                let order_date_opt = modal.order_item.lock_ref().as_ref().and_then(|oi| oi.order_date);
                first_24hr_plan_buttons_iter(start_hr, order_hr, times, order_date_opt, &time_plans, modal.plan_add_datetimes.clone()).collect::<Vec<Dom>>()
            }))
            .to_signal_vec()
    }

    // fn plan_until_next_nth_buttons(next_hour: u8, order_hr: u8, time_plans: Vec<Rc<IndexPlan>>, modal: Rc<Self>) -> impl SignalVec<Item = Dom> + use<> {
    //     modal.plan_time_start_hour.signal().map(clone!(modal => move |start_hr| {
    //         let order_date_opt = modal.order_item.get_cloned().and_then(|oi| oi.order_date);
    //         let first_24hr = if start_hr == 0 {
    //             (0u8..24u8).collect::<Vec<u8>>()
    //         } else {
    //             (start_hr..24).chain(0..start_hr).collect::<Vec<u8>>()
    //         };
    //         let next_nth = (start_hr < next_hour && order_date_opt.is_some()).then(|| {
    //             (start_hr..=next_hour).collect::<Vec<u8>>()
    //         });
    //         let mut buttons = Vec::new();
    //         let order_date_opt = modal.order_item.get_cloned().and_then(|oi| oi.order_date);
    //         // first_24hr
    //         buttons.extend(first_24hr_plan_buttons_iter(start_hr, order_hr, first_24hr, order_date_opt, &time_plans, modal.plan_add_datetimes.clone()));
    //         // next nth hours
    //         if let Some(next_days) = next_nth {
    //             // we already checked `order_date_opt.is_some()` above so unwrap here is safe
    //             let btn_date = order_date_opt.and_then(|d| d.next_day()).unwrap();
    //             buttons.extend(next_days.into_iter().map(|h| {
    //                 let hour_count = time_plans.iter().filter(|plan| plan.plan_date == Some(btn_date) && plan.plan_time.map(|t| t.hour() == h).unwrap_or_default()).count();
    //                 html!("button", {
    //                     .attr("type", "button")
    //                     .class(class::BTN_LB2)
    //                     .class("position-relative")
    //                     .apply(|dom| {
    //                         if hour_count > 0 {
    //                             dom.class("btn-secondary")
    //                                 .style("pointer-events","none")
    //                                 .child(doms::badge_count_blue(hour_count).unwrap_or(Dom::empty()))
    //                         } else {
    //                             dom.class("btn-outline-primary")
    //                         }
    //                     })
    //                     .class_signal("btn-info", modal.plan_add_datetimes.signal_cloned().map(move |dts| dts.iter().any(|dt| {
    //                         btn_date == dt.date() && dt.hour() == h
    //                     })))
    //                     .style("min-width","45px")
    //                     .attr("title", &date_th(&btn_date))
    //                     .text(&h.to_string())
    //                     .event(clone!(modal => move |_: events::Click| {
    //                         let hr = if h == 24 {0} else {h};
    //                         let input_time = Time::from_hms(hr, 0, 0).unwrap();
    //                         let plan_datetime = PrimitiveDateTime::new(btn_date, input_time);
    //                         {
    //                             let mut lock = modal.plan_add_datetimes.lock_mut();
    //                             if lock.contains(&plan_datetime) {
    //                                 lock.remove(&plan_datetime);
    //                             } else {
    //                                 lock.insert(plan_datetime);
    //                             }
    //                         }
    //                     }))
    //                 })
    //             }));
    //         }
    //         buttons
    //     })).to_signal_vec()
    // }

    fn render_plan_sch_time_single(time_plans: Vec<Rc<IndexPlan>>, modal: Rc<Self>, app: Rc<App>) -> Dom {
        let (is_ipd, is_pre_admit) = modal.is_ipd_and_is_pre_admit();

        html!("nav", {
            .apply(|dom| {
                let can_add = if is_ipd {
                    app.has_permission(Permission::IpdNurseIndexAdd) || (is_pre_admit && app.has_permission(Permission::OpdErNurseIndexAdd))
                } else {
                    app.has_permission(Permission::OpdErNurseIndexAdd)
                };

                // Continuous
                if modal.is_continuous() {
                    let mut days = time_plans.iter().filter_map(|plan| plan.plan_date).collect::<HashSet<Date>>().into_iter().collect::<Vec<Date>>();
                    days.sort();
                    if days.is_empty() && can_add {
                        dom.child(html!("div", {
                            .class(class::NAV_PILLS_T)
                            .attr("role", "tablist")
                            .child(Self::plan_dom_new(modal.clone(), None))
                        }))
                    } else {
                        dom.child(html!("div", {
                            .class(class::NAV_PILLS_COL_T)
                            .attr("role", "tablist")
                            .children(days.iter().map(|plan_date| {
                                let plan_date_opt = Some(*plan_date);
                                let plans_in_date = time_plans.iter().filter(|plan| plan.plan_date == plan_date_opt).cloned().collect::<Vec<Rc<IndexPlan>>>();
                                html!("div", {
                                    .class(class::FLEX_WRAP)
                                    .child(html!("div", {
                                        .class(class::TXT_R_P)
                                        .style("width","135px")
                                        .text(&["เริ่ม ", &date_th(plan_date)].concat())
                                    }))
                                    .children(plans_in_date.iter().map(clone!(modal => move |plan| {
                                        let label = time_hm_opt(&plan.plan_time);
                                        Self::plan_dom(plan.clone(), modal.clone(), Some(label))
                                    })))
                                    .apply_if(can_add, |d| d.child(Self::plan_dom_new(modal.clone(), plan_date_opt)))
                                })
                            }))
                        }))
                    }
                // OneDay
                } else {
                    dom.child(html!("div", {
                        .class(class::NAV_PILLS_T)
                        .attr("role", "tablist")
                        .children(time_plans.into_iter().map(clone!(modal => move |plan| {
                            Self::plan_dom(plan.clone(), modal.clone(), None)
                        })))
                        .apply_if(can_add, |d| d.child(Self::plan_dom_new(modal.clone(), None)))
                    }))
                }
            })
        })
    }

    fn render_action_sch_stat(plans: Vec<Rc<IndexPlan>>, modal: Rc<Self>) -> Dom {
        html!("nav", {
            .child(html!("div", {
                .class(class::NAV_PILLS_T)
                .attr("role", "tablist")
                .children(plans.iter().map(clone!(modal => move |plan| {
                    Self::plan_dom(plan.clone(), modal.clone(), None)
                })))
            }))
        })
    }

    fn render_action_sch_date(plans: Vec<Rc<IndexPlan>>, modal: Rc<Self>) -> Dom {
        html!("nav", {
            .child(html!("div", {
                .class(class::NAV_PILLS_T)
                .attr("role", "tablist")
                .children(plans.iter().map(|plan| {
                    let label = if plan.is_continuous_or_none() {
                        ["ตั้งแต่ ", &date_th_opt(&plan.plan_date), " ", &time_hm_opt(&plan.plan_time)].concat()
                    } else {
                        [&date_th_opt(&plan.plan_date), " ", &time_hm_opt(&plan.plan_time), " - ", &date_th_opt(&plan.plan_date.and_then(|d| d.next_day())), " ", &time_hm_opt(&plan.plan_time)].concat()
                    };
                    Self::plan_dom(plan.clone(), modal.clone(), Some(label))
                }))
            }))
        })
    }

    fn render_action_sch_time(plans: Vec<Rc<IndexPlan>>, modal: Rc<Self>) -> Dom {
        html!("nav", {
            .apply(|dom| {
                if modal.is_continuous() {
                    let mut days = plans.iter().filter_map(|plan| plan.plan_date).collect::<HashSet<Date>>().into_iter().collect::<Vec<Date>>();
                    days.sort();
                    if days.is_empty() {
                        dom.child(html!("div", {
                            .class(class::NAV_PILLS_T)
                            .attr("role", "tablist")
                        }))
                    } else {
                        dom.child(html!("div", {
                            .class(class::NAV_PILLS_COL_T)
                            .attr("role", "tablist")
                            .children(days.iter().map(|plan_date| {
                                let plan_date_opt = Some(*plan_date);
                                let plans_in_date = plans.iter().filter(|plan| plan.plan_date == plan_date_opt).cloned().collect::<Vec<Rc<IndexPlan>>>();
                                html!("div", {
                                    .class(class::FLEX_WRAP)
                                    .child(html!("div", {
                                        .class(class::TXT_R_P)
                                        .style("width","135px")
                                        .text(&["เริ่ม ", &date_th(plan_date)].concat())
                                    }))
                                    .children(plans_in_date.iter().map(clone!(modal => move |plan| {
                                        let label = time_hm_opt(&plan.plan_time);
                                        Self::plan_dom(plan.clone(), modal.clone(), Some(label))
                                    })))
                                })
                            }))
                        }))
                    }
                } else {
                    dom.child(html!("div", {
                        .class(class::NAV_PILLS_T)
                        .attr("role", "tablist")
                        .children(plans.into_iter().map(clone!(modal => move |plan| {
                            Self::plan_dom(plan.clone(), modal.clone(), None)
                        })))
                    }))
                }
            })
        })
    }

    fn render_actions_list(allow_all: bool, plan_id_opt: Option<u32>, order_item_opt: Option<Rc<OrderItem>>, modal: Rc<Self>) -> Option<Dom> {
        let (container_id, container_hash, gut_id, gut_hash, title) = if plan_id_opt.is_some() {
            ("modal-actions-list-container", "#modal-actions-list-container", "modal-actions-list", "#modal-actions-list", "")
        } else {
            (
                "modal-all-actions-list-container",
                "#modal-all-actions-list-container",
                "modal-all-actions-list",
                "#modal-all-actions-list",
                "ทั้งหมด",
            )
        };
        modal.plan_actions(plan_id_opt, order_item_opt.clone(), allow_all).and_then(|actions| {
            let actions_len = actions.len();
            (actions_len > 0).then(|| {
                html!("div", {
                    .class("mb-2")
                    .child(html!("div", {
                        .class("accordion")
                        .attr("id",container_id)
                        .child(html!("div", {
                            .class("accordion-item")
                            .children([
                                html!("div", {
                                    .class("accordion-header")
                                    .child(html!("button", {
                                        .attr("type", "button")
                                        .class(class::ACCORDION_BTN_COLLAPSED_GOLD_P2)
                                        .attr("data-bs-toggle","collapse")
                                        .attr("data-bs-target",gut_hash)
                                        .attr("aria-expanded","false")
                                        .attr("aria-controls",gut_id)
                                        .text("แสดง Action ")
                                        .text(title)
                                        .text(" (")
                                        .text(&actions_len.to_string())
                                        .text(" รายการ)")
                                    }))
                                }),
                                html!("div", {
                                    .attr("id",gut_id)
                                    .class(class::ACCORDION_COLLAPSE)
                                    .attr("data-bs-parent",container_hash)
                                    .child(html!("ul", {
                                        .class(class::LIST_GROUP_FLUSH_OVFA)
                                        .style("max-height","124px")
                                        .style("cursor","pointer")
                                        .children(actions.into_iter().enumerate().map(clone!(order_item_opt => move |(i, action)| {
                                            let comma = if action.action_result.is_some() {" : "} else {""};
                                            let label = if let Some(action_datetime) = datetime_from_opt(action.action_date, action.action_time) {
                                                datetime_th_relative(&action_datetime)
                                            } else {
                                                String::from("ไม่ระบุวัน-เวลา")
                                            };
                                            html!("li", {
                                                .class("list-group-item")
                                                .class_signal(class::BOLD_BG_GRAY, modal.action_id.signal_cloned().map(clone!(action => move |action_id| {
                                                    action.action_id == action_id
                                                })))
                                                .text(&actions_len.saturating_sub(i).to_string())
                                                .text(". ")
                                                .apply(|dom| {
                                                    if action.check_person.is_some() && action.action_person_1.is_none() {
                                                        dom.text("\u{2717} รอดำเนินการ")
                                                    } else if let Some(order_item) = &order_item_opt {
                                                        dom.child(doms::had_monitor_status(&action, order_item, false))
                                                    } else {
                                                        dom.text("\u{2713}")
                                                    }
                                                })
                                                .text(" ")
                                                .text(&label)
                                                .text(comma)
                                                .text(&action.action_result.clone().unwrap_or_default())
                                                .apply_if(action.vs_id.is_some(), |dom| {
                                                    dom.child(html!("i", {.class(class::FA_LINK_GREEN)}))
                                                })
                                                .with_node!(element => {
                                                    .future(modal.action_id.signal().for_each(clone!(action => move |action_id| {
                                                        if action.action_id == action_id {
                                                            element.scroll_into_view();
                                                        }
                                                        async {}
                                                    })))
                                                })
                                                .event(clone!(modal => move |_: events::Click| {
                                                    if matches!(modal.form_type.get_cloned(), FormType::Monitor) {
                                                        modal.action_id.set_neq(action.action_id);
                                                        modal.action_datetime.set(datetime_from_opt(action.action_date, action.action_time));
                                                        modal.action_result.set_neq(action.action_result.clone().unwrap_or_default());
                                                    } else {
                                                        modal.go_and_set_action(&action);
                                                    }
                                                }))
                                            })
                                        })))
                                    }))
                                }),
                            ])
                        }))
                    }))
                })
            })
        })
    }

    fn render_plan_inputs(modal: Rc<Self>, app: Rc<App>) -> Dom {
        let plan_tab = modal.active_tab.get_cloned();
        let (date_label, time_label) = match (&plan_tab, modal.is_continuous()) {
            // time-continuous
            (PlanTab::Time, true) => ("วันที่เริ่มต้น", "เวลา"),
            // prn-oneday
            (PlanTab::Date, false) => ("วันที่", "เวลาเริ่มต้น และสิ้นสุดในวันถัดไป"),
            // prn-continuous
            (PlanTab::Date, true) => ("วันที่เริ่มต้น", "เวลาเริ่มต้น"),
            // time-oneday, stat-oneday, stat-continuous
            (_, _) => ("วันที่", "เวลา"),
        };
        let (is_ipd, is_pre_admit) = modal.is_ipd_and_is_pre_admit();

        html!("div", {
            .class(class::BORDER_ROUND)
            .class_signal("bg-info-subtle", modal.is_new_plan_signal())
            .class_signal("bg-secondary-subtle", not(modal.is_new_plan_signal()))
            // Actions
            .child_signal(map_ref!{
                let plan_id_opt = modal.plan_id.signal(),
                let order_item_opt = modal.order_item.signal_cloned() =>
                (plan_id_opt.to_owned(), order_item_opt.clone())
            }.map(clone!(modal => move |(plan_id_opt, order_item_opt)|{
                modal.plan_actions(plan_id_opt, order_item_opt, false).map(|actions| {
                    html!("div", {
                        .class(class::OVFA_T)
                        .style("max-height","81px")
                        .children(actions.into_iter().map(|action| {
                            let (checked, sym) = if action.check_person.is_some() && action.action_person_1.is_none() {("รอดำเนินการ","\u{2717} ")} else {("","\u{2713} ")};
                            let label = if modal.is_continuous() && matches!(*modal.active_tab.lock_ref(), PlanTab::Time) {
                                [sym, checked, &date_th_opt(&action.action_date)].concat()
                            } else {
                                [sym, checked, &date_th_opt(&action.action_date), " ", &time_hm_opt(&action.action_time)].concat()
                            };
                            html!("span", {
                                .apply(|d| {
                                    if checked.is_empty() {
                                        d.class(class::BADGE_GOLD_RT)
                                    } else {
                                        d.class(class::BADGE_GRAY_RT)
                                    }
                                })
                                .style("cursor","pointer")
                                .text(&label)
                                .event(clone!(modal => move |_: events::Click| {
                                    modal.go_and_set_action(&action);
                                }))
                            })
                        }))
                    })
                })
            })))
            // Input Form
            .children([
                html!("div", {
                    .children([
                        html!("div", {
                            .class(class::ROW_GT)
                            // plan_date input
                            .children([
                                html!("div", {
                                    .class("col-6")
                                    .child(html!("div", {
                                        .class(class::INPUT_GROUP)
                                        .children([
                                            doms::label_group_for("index-plan-date",date_label),
                                            doms::date_picker(
                                                modal.plan_date.clone(),
                                                modal.changed.clone(), always(false), Some(modal.plan_time.clone()),
                                                |d| d.class(class::FLEX_GROW1).style("min-width","135px"),
                                                |d| d.class("rounded-start-0"),
                                                |d| d.class("rounded-start-0").attr("id","index-plan-date"),
                                                |s| s,
                                                map_ref!{
                                                    let allow_plan_before_order = modal.allow_plan_before_order_signal(),
                                                    let order_item_opt = modal.order_item.signal_cloned() => {
                                                        if !allow_plan_before_order && let Some(order_datetime) = order_item_opt.as_ref().and_then(|order_item| datetime_from_opt(order_item.order_date, order_item.order_time)) {
                                                            Some(doms::PickerConfigBuilder::default()
                                                                .date_constraints(doms::DateConstraintsBuilder::default()
                                                                    .min_datetime(order_datetime)
                                                                    .build().unwrap()
                                                                ).build().unwrap())
                                                        } else {
                                                            None
                                                        }
                                                    }
                                                },
                                            ),
                                        ])
                                    }))
                                }),
                                html!("div", {
                                    .class("col-6")
                                    .child(html!("div", {
                                        .class(class::INPUT_GROUP)
                                        .children([
                                            doms::label_group_for("index-plan-time",time_label),
                                            doms::time_picker(
                                                modal.plan_time.clone(),
                                                modal.changed.clone(), always(false), Some(modal.plan_date.clone()),
                                                |d| d.class(class::FLEX_GROW1).style("min-width","110px"),
                                                |d| d.class("rounded-start-0"),
                                                |d| d.class("rounded-start-0").attr("id","index-plan-time"),
                                                |s| s,
                                                map_ref!{
                                                    let allow_plan_before_order = modal.allow_plan_before_order_signal(),
                                                    let order_item_opt = modal.order_item.signal_cloned() => {
                                                        if !allow_plan_before_order && let Some(order_datetime) = order_item_opt.as_ref().and_then(|order_item| datetime_from_opt(order_item.order_date, order_item.order_time)) {
                                                            Some(doms::PickerConfigBuilder::default()
                                                                .date_constraints(doms::DateConstraintsBuilder::default()
                                                                    .min_datetime(order_datetime)
                                                                    .build().unwrap()
                                                                ).build().unwrap()
                                                            )
                                                        } else {
                                                            None
                                                        }
                                                    }
                                                },
                                            ),
                                        ])
                                    }))
                                }),
                            ])
                        }),
                        // plan_detail textarea
                        html!("div", {
                            .class(class::ROW_GT)
                            .child(html!("div", {
                                .class("form-floating")
                                .children([
                                    html!("textarea" => HtmlTextAreaElement, {
                                        .class("form-control")
                                        .attr("id", "index-plan-detail")
                                        .attr("placeholder","Plan Details")
                                        .apply(mixins::textarea_value_auto_expand(modal.plan_detail.clone(), modal.changed.clone()))
                                    }),
                                    html!("label", {
                                        .attr("for", "index-plan-detail")
                                        .text("รายละเอียด")
                                    }),
                                ])
                            }))
                        }),
                    ])
                }),
                html!("div", {
                    .class(class::ROW)
                    .child(html!("div", {
                        .class("col-12")
                        .child(html!("div", {
                            .class(class::FORM_CHK_SW)
                            .class("float-end")
                            .children([
                                html!("input" => HtmlInputElement, {
                                    .attr("type", "checkbox")
                                    .attr("id", "allow-plan-before-order-toggle")
                                    .attr("role","switch")
                                    .class("form-check-input")
                                    .attr("value", "Y")
                                    .apply(mixins::checkbox_toggle(modal.allow_plan_before_order.clone(), Mutable::new(false), "Y", ""))
                                }),
                                doms::label_check_for("allow-plan-before-order-toggle","เลือกเวลาแผน ก่อนเวลาสั่งได้"),
                            ])
                        }))
                    }))
                }),
                html!("div", {
                    .class("row")
                    .child(html!("div", {
                        .class(class::COL12_Y)
                        .child(html!("button", {
                            .attr("type", "button")
                            .class(class::BTN_FR_GRAY)
                            .child(html!("i", {.class(class::FA_X)}))
                            .text(" ยกเลิก")
                            .event(clone!(modal => move |_: events::Click| {
                                modal.plan_id.set_neq(None);
                            }))
                        }))
                        .child_signal(modal.plan_id.signal_cloned().map(clone!(app, modal => move |plan_id| {
                            (plan_id.and_then(zero_none).map(|plan_id| modal.has_plan_without_action(plan_id)).unwrap_or_default() && (if is_ipd {
                                app.endpoint_is_allow(&Method::DELETE, &EndPoint::IpdIndexPlanId, is_pre_admit)
                            } else {
                                app.endpoint_is_allow(&Method::DELETE, &EndPoint::OpdErIndexPlanId, false)
                            })).then(|| {
                                html!("button" => HtmlButtonElement, {
                                    .attr("type", "button")
                                    .class(class::BTN_RED)
                                    .child(html!("i", {.class(class::FA_TRASH)}))
                                    .text(" ลบ")
                                    .apply(mixins::click_with_loader_checked(clone!(app, modal => move || {
                                        Self::delete_index_plan(modal.clone(), app.clone());
                                    }), app.state()))
                                })
                            })
                        })))
                        .child_signal(modal.plan_id.signal_cloned().map(clone!(app, modal => move |plan_id| {
                            (if is_ipd {
                                app.endpoint_is_allow(&Method::POST, &EndPoint::IpdIndexPlan, is_pre_admit)
                                && if plan_id.and_then(zero_none).is_some() {
                                    app.has_permission(Permission::IpdNurseIndexEdit) || (is_pre_admit && app.has_permission(Permission::OpdErNurseIndexEdit))
                                } else {
                                    app.has_permission(Permission::IpdNurseIndexAdd) || (is_pre_admit && app.has_permission(Permission::OpdErNurseIndexAdd))
                                }
                            } else {
                                app.endpoint_is_allow(&Method::POST, &EndPoint::OpdErIndexPlan, false)
                                && if plan_id.and_then(zero_none).is_some() {
                                    app.has_permission(Permission::OpdErNurseIndexEdit)
                                } else {
                                    app.has_permission(Permission::OpdErNurseIndexAdd)
                                }
                            }).then(|| {
                                html!("button" => HtmlButtonElement, {
                                    .attr("type", "button")
                                    .class(class::BTN_FR_L)
                                    .class_signal("btn-primary", modal.is_plan_valid())
                                    .class_signal("btn-secondary", not(modal.is_plan_valid()))
                                    .child(html!("i", {.class(class::FA_SAVE)}))
                                    .text(" บันทึก")
                                    .apply(mixins::click_with_loader_checked_or_true_disable_signal(clone!(app, modal => move || {
                                        Self::submit_plan(modal.clone(), app.clone());
                                    }), not(modal.is_plan_valid()), app.state()))
                                })
                            })
                        })))
                        .child_signal(modal.action_id.signal_cloned().map(clone!(modal => move |opt| {
                            opt.is_some().then(|| {
                                html!("button", {
                                    .attr("type", "button")
                                    .class(class::BTN_FR_L_BLUE)
                                    .text("ACTION")
                                    .event(clone!(modal => move |_: events::Click| {
                                        modal.form_type.set_neq(FormType::Action);
                                        modal.set_new_action();
                                    }))
                                })
                            })
                        })))
                    }))
                }),
            ])
        })
    }

    fn render_action_inputs(modal: Rc<Self>, app: Rc<App>) -> Dom {
        let all_doctor_select_option = app.app_asset.lock_ref().as_ref().map(|assets| assets.all_doctor_select_option.clone()).unwrap_or_default();
        let (is_ipd, is_pre_admit) = modal.is_ipd_and_is_pre_admit();

        html!("div", {
            .child_signal(modal.is_med_signal().map(clone!(app, modal, all_doctor_select_option => move |is_med| {
                is_med.then(|| {
                    html!("div", {
                        .class(class::BORDER_ROUND)
                        .class_signal("bg-info-subtle", modal.is_new_action_signal())
                        .class_signal("bg-secondary-subtle", not(modal.is_new_action_signal()))
                        .child(html!("div", {
                            .class(class::FLEX_C)
                            .children([
                                html!("div", {
                                    .class("me-1")
                                    .child(html!("div", {
                                        .class(class::INPUT_GROUP)
                                        .children([
                                            doms::label_group_for("index-check-datetime","จัดยา"),
                                            doms::datetime_picker(
                                                modal.check_datetime.clone(),
                                                modal.changed.clone(), always(false),
                                                |d| d.class(class::FLEX_GROW1).style("width","190px"),
                                                |d| d.class("rounded-start-0"),
                                                |d| d.class("rounded-start-0").attr("id", "index-check-datetime"),
                                                |s| s, always(None),
                                            ),
                                        ])
                                    }))
                                }),
                                html!("button", {
                                    .attr("type", "button")
                                    .class(class::BTN_L_CYAN)
                                    .style("white-space","nowrap")
                                    .child(html!("i", {.class(class::FA_CLOCK)}))
                                    // .text(" ปัจจุบัน")
                                    .event(clone!(modal => move |_: events::Click| {
                                        modal.check_datetime.set_neq(js_now().js_string());
                                        modal.changed.set_neq(true);
                                    }))
                                }),
                                html!("div", {
                                    .class(class::FLEX_ITEM_GROW1_L)
                                    .child(html!("div", {
                                        .class(class::INPUT_GROUP)
                                        .children([
                                            html!("button", {
                                                .attr("type", "button")
                                                .class(class::BTN_BLUE)
                                                .text("ลงชื่อ")
                                                .event(clone!(app, modal => move |_: events::Click| {
                                                    let doctorcode = app.doctor_code().unwrap_or_default();
                                                    if let Some(elm) = app.get_id("index-check-person") {
                                                        NiceSelect::new_default_with_value(&elm, &doctorcode);
                                                    }
                                                    modal.check_person.set_neq(doctorcode);
                                                }))
                                            }),
                                            html!("div", {
                                                .class(class::FLEX_GROW1)
                                                .future(modal.redraw_check_person.signal().for_each(clone!(app, modal => move |redraw| {
                                                    if redraw {
                                                        if let Some(elm) = app.get_id("index-check-person") {
                                                            NiceSelect::new_default_with_value(&elm, &modal.check_person.lock_ref());
                                                        }
                                                        modal.redraw_check_person.set_neq(false);
                                                    }
                                                    async {}
                                                })))
                                                .child(html!("select" => HtmlSelectElement, {
                                                    .class("form-control")
                                                    .attr("id", "index-check-person")
                                                    .child(html!("option", {
                                                        .attr("value", "")
                                                        .text("เลือก")
                                                    }))
                                                    .children(all_doctor_select_option.iter().map(|option| {
                                                        doms::select_option(option, &modal.check_person.lock_ref())
                                                    }))
                                                    .apply(mixins::string_value_select(modal.check_person.clone(), modal.changed.clone()))
                                                }))
                                            }),
                                        ])
                                    }))
                                }),
                            ])
                            // Show check save only NEW action
                            .child_signal(modal.action_id.signal_cloned().map(clone!(app, modal => move |action_id| {
                                (action_id == Some(0) && if is_ipd {
                                    app.endpoint_is_allow(&Method::POST, &EndPoint::IpdIndexAction, is_pre_admit)
                                    && app.has_permission(Permission::IpdNurseIndexAdd) || (is_pre_admit && app.has_permission(Permission::OpdErNurseIndexAdd))
                                } else {
                                    app.endpoint_is_allow(&Method::POST, &EndPoint::OpdErIndexAction, false)
                                    && app.has_permission(Permission::OpdErNurseIndexAdd)
                                }).then(|| {
                                    html!("button" => HtmlButtonElement, {
                                        .attr("type", "button")
                                        .class("btn")
                                        .class_signal("btn-primary", modal.is_check_valid())
                                        .class_signal("btn-secondary", not(modal.is_check_valid()))
                                        .style("white-space","nowrap")
                                        .child(html!("i", {.class(class::FA_SAVE)}))
                                        // .text(" บันทึก")
                                        .apply(mixins::click_with_loader_checked_or_true_disable_signal(clone!(app, modal => move || {
                                            Self::submit_check(modal.clone(), app.clone());
                                        }), not(modal.is_check_valid()), app.state()))
                                    })
                                })
                            })))
                        }))
                    })
                })
            })))
            .child_signal(map_ref! {
                let is_med = modal.is_med_signal(),
                let check_datetime = modal.check_datetime.signal_cloned(),
                let check_person = modal.check_person.signal_cloned(),
                let action_person_1 = modal.action_person_1.signal_cloned() =>
                !is_med || !action_person_1.is_empty() || (!check_datetime.is_empty() && !check_person.is_empty())
            }.map(clone!(modal, app => move |can_act| {
                can_act.then(|| {
                    html!("div", {
                        .children([
                            html!("div", {
                                .class(class::BORDER_ROUND)
                                .class_signal("bg-info-subtle", modal.is_new_action_signal())
                                .class_signal("bg-secondary-subtle", not(modal.is_new_action_signal()))
                                .children([
                                    html!("div", {
                                        .class(class::FLEX_C_T)
                                        .children([
                                            html!("div", {
                                                .class(class::FLEX_ITEM_GROW1_L)
                                                .child(html!("div", {
                                                    .class(class::INPUT_GROUP)
                                                    .children([
                                                        doms::label_group_for("index-action-date","Action"),
                                                        doms::date_picker(
                                                            modal.action_date.clone(),
                                                            modal.changed.clone(), always(false), None,
                                                            |d| d.class(class::FLEX_GROW1).style("min-width","135px"),
                                                            |d| d.class("rounded-start-0"),
                                                            |d| d.class("rounded-start-0").attr("id", "index-action-date"),
                                                            |s| s, always(None),
                                                        ),
                                                    ])
                                                }))
                                            }),
                                            html!("div", {
                                                .class(class::FLEX_ITEM_GROW1_L)
                                                .child(html!("div", {
                                                    .class(class::INPUT_GROUP)
                                                    .children([
                                                        doms::label_group_for("index-action-time","เวลา"),
                                                        doms::time_picker(
                                                            modal.action_time.clone(),
                                                            modal.changed.clone(), always(false), None,
                                                            |d| d.class(class::FLEX_GROW1).style("min-width","110px"),
                                                            |d| d.class("rounded-start-0"),
                                                            |d| d.class("rounded-start-0").attr("id", "index-action-time"),
                                                            |s| s, always(None),
                                                        ),
                                                    ])
                                                }))
                                            }),
                                            html!("button", {
                                                .attr("type", "button")
                                                .class(class::BTN_L_BLUE)
                                                .style("white-space","nowrap")
                                                .child(html!("i", {.class(class::FA_CLOCK)}))
                                                .text(" แผน")
                                                .event(clone!(modal => move |_: events::Click| {
                                                    modal.action_date.set_neq(modal.selected_plan.lock_ref().as_ref().and_then(|plan| plan.plan_date.map(|t| t.to_string())).unwrap_or_default());
                                                    modal.action_time.set_neq(modal.selected_plan.lock_ref().as_ref().and_then(|plan| plan.plan_time.map(|t| t.js_string())).unwrap_or_default());
                                                    modal.changed.set_neq(true);
                                                }))
                                            }),
                                            html!("button", {
                                                .attr("type", "button")
                                                .class(class::BTN_CYAN)
                                                .style("white-space","nowrap")
                                                .child(html!("i", {.class(class::FA_CLOCK)}))
                                                .text(" ปัจจุบัน")
                                                .event(clone!(modal => move |_: events::Click| {
                                                    let now = js_now();
                                                    modal.action_date.set_neq(now.date().to_string());
                                                    modal.action_time.set_neq(now.time().js_string());
                                                    modal.changed.set_neq(true);
                                                }))
                                            }),
                                        ])
                                    }),
                                    html!("div", {
                                        .class(class::ROW_GT)
                                        .children([
                                            html!("div", {
                                                .class("col-sm-6")
                                                .child(html!("div", {
                                                    .class("input-group")
                                                    .child(html!("div", {
                                                        .class("form-floating")
                                                        .children([
                                                            html!("textarea" => HtmlTextAreaElement, {
                                                                .class("form-control")
                                                                .attr("id", "index-action-result")
                                                                .attr("placeholder","Action Result")
                                                                .apply(mixins::textarea_value_auto_expand(modal.action_result.clone(), modal.changed.clone()))
                                                            }),
                                                            html!("label", {
                                                                .attr("for", "index-action-result")
                                                                .text("ผลลัพธ์")
                                                            }),
                                                        ])
                                                    }))
                                                    .apply_if(if is_ipd {
                                                        app.endpoint_is_allow(&Method::PATCH, &EndPoint::IpdOrderItem, is_pre_admit)
                                                    } else {
                                                        app.endpoint_is_allow(&Method::PATCH, &EndPoint::OpdErOrderItem, false)
                                                    }, |dom| { dom
                                                        .child_signal(map_ref!{
                                                            let action_id_opt = modal.action_id.signal().map(opt_zero_none),
                                                            let dtx_opt = modal.action_result.signal_cloned().map(|s| str_some(s)),
                                                            let has_dtx = modal.has_dtx_signal(),
                                                            let has_vs_id = modal.vs_id.signal().map(|opt| opt.is_some()) =>
                                                            (*action_id_opt, dtx_opt.clone(), *has_dtx, *has_vs_id)
                                                        }.map(clone!(modal, app => move |(action_id_opt, dtx_opt, has_dtx, has_vs_id)| {
                                                            (has_dtx && dtx_opt.is_some() && action_id_opt.is_some()).then(|| {
                                                                if has_vs_id {
                                                                    html!("div", {
                                                                        .class("input-group-text")
                                                                        .style("font-size", "14px")
                                                                        .style("flex-wrap", "wrap")
                                                                        .style("width", "50px")
                                                                        .style("cursor", "help")
                                                                        .attr("title", "ได้บันทึกใน Vital Sign แล้ว แต่หากมีการแก้ไข Action หรือ Vital Sign กรุณาแก้ไขทั้ง 2 รายการให้ตรงกันด้วย")
                                                                        .children([
                                                                            html!("span", {.text("DTX")}),
                                                                            html!("i", {
                                                                                .class(class::FA_LINK_GREEN)
                                                                                .style("font-size", "24px")
                                                                            }),
                                                                        ])
                                                                    })
                                                                } else {
                                                                    html!("button" => HtmlButtonElement, {
                                                                        .attr("type", "button")
                                                                        .class(class::BTN_CYAN)
                                                                        .style("font-size", "14px")
                                                                        .text("DTX")
                                                                        .child(html!("br"))
                                                                        .text("VS")
                                                                        .apply(mixins::click_with_loader_checked(clone!(modal, app => move || {
                                                                            if let Some(action_dt) = datetime_from_opt(date_8601(&modal.action_date.lock_ref()), time_8601(&modal.action_time.lock_ref())) {
                                                                                let mut saver = VitalSignSave::new(action_dt);
                                                                                saver.action_id = action_id_opt;
                                                                                saver.dtx = dtx_opt.clone();
                                                                                Self::add_vs(saver, modal.clone(), app.clone());
                                                                            }
                                                                        }), app.state()))
                                                                    })
                                                                }
                                                            })
                                                        })))
                                                        .child_signal(map_ref!{
                                                            let action_id_opt = modal.action_id.signal().map(opt_zero_none),
                                                            let hct_opt = modal.action_result.signal_cloned().map(|s| find_decimal_in_text(&s)),
                                                            let has_hct = modal.has_hct_signal(),
                                                            let has_vs_id = modal.vs_id.signal().map(|opt| opt.is_some()) =>
                                                            (*action_id_opt, *hct_opt, *has_hct, *has_vs_id)
                                                        }.map(clone!(modal, app => move |(action_id_opt, hct_opt, has_hct, has_vs_id)| {
                                                            (has_hct && hct_opt.is_some() && action_id_opt.is_some()).then(|| {
                                                                if has_vs_id {
                                                                    html!("div", {
                                                                        .class("input-group-text")
                                                                        .style("font-size", "14px")
                                                                        .style("flex-wrap", "wrap")
                                                                        .style("width", "50px")
                                                                        .style("cursor", "help")
                                                                        .attr("title", "ได้บันทึกใน Vital Sign แล้ว แต่หากมีการแก้ไข Action หรือ Vital Sign กรุณาแก้ไขทั้ง 2 รายการให้ตรงกันด้วย")
                                                                        .children([
                                                                            html!("span", {.text("HCT")}),
                                                                            html!("i", {
                                                                                .class(class::FA_LINK_GREEN)
                                                                                .style("font-size", "24px")
                                                                            }),
                                                                        ])
                                                                    })
                                                                } else {
                                                                    html!("button" => HtmlButtonElement, {
                                                                        .attr("type", "button")
                                                                        .class(class::BTN_CYAN)
                                                                        .style("font-size", "14px")
                                                                        .text("HCT")
                                                                        .child(html!("br"))
                                                                        .text("VS")
                                                                        .apply(mixins::click_with_loader_checked(clone!(modal, app => move || {
                                                                            if let Some(action_dt) = datetime_from_opt(date_8601(&modal.action_date.lock_ref()), time_8601(&modal.action_time.lock_ref())) {
                                                                                let mut saver = VitalSignSave::new(action_dt);
                                                                                saver.action_id = action_id_opt;
                                                                                saver.hct = hct_opt;
                                                                                Self::add_vs(saver, modal.clone(), app.clone());
                                                                            }
                                                                        }), app.state()))
                                                                    })
                                                                }
                                                            })
                                                        })))
                                                    })
                                                }))
                                            }),
                                            html!("div", {
                                                .class("col-sm-6")
                                                .child(html!("div", {
                                                    .class("form-floating")
                                                    .children([
                                                        html!("textarea" => HtmlTextAreaElement, {
                                                            .class("form-control")
                                                            .attr("id", "index-action-remark")
                                                            .attr("placeholder","Action Remark")
                                                            .apply(mixins::textarea_value_auto_expand(modal.action_remark.clone(), modal.changed.clone()))
                                                        }),
                                                        html!("label", {
                                                            .attr("for", "index-action-remark")
                                                            .text("หมายเหตุ")
                                                        }),
                                                    ])
                                                }))
                                            }),
                                        ])
                                    }),
                                    html!("div", {
                                        .class(class::ROW)
                                        .child(html!("div", {
                                            .class("col-12")
                                            .child(html!("div", {
                                                .class(class::FORM_CHK_SW)
                                                .class("float-end")
                                                .children([
                                                    html!("input" => HtmlInputElement, {
                                                        .attr("type", "checkbox")
                                                        .attr("role","switch")
                                                        .class("form-check-input")
                                                        .attr("id", "index-action-blood-had")
                                                        .attr("value", "Y")
                                                        .apply(mixins::checkbox_toggle(modal.action_blood_had.clone(), modal.changed.clone(), "Y", "N"))
                                                    }),
                                                    doms::label_check_for("index-action-blood-had","Blood/HAD (ลงชื่อด้วย จนท. 2 ท่าน)"),
                                                ])
                                            }))
                                        }))
                                    }),
                                    html!("div", {
                                        .class(class::ROW_AUTO_LG_G2_CT)
                                        .children([
                                            html!("div", {
                                                .class("col-12")
                                                .style("min-width","368px")
                                                .child(html!("div", {
                                                    .class(class::INPUT_GROUP)
                                                    .children([
                                                        html!("button", {
                                                            .attr("type", "button")
                                                            .class(class::BTN_BLUE)
                                                            .text("ลงชื่อ")
                                                            .event(clone!(app, modal => move |_: events::Click| {
                                                                let doctorcode = app.doctor_code().unwrap_or_default();
                                                                if let Some(elm) = app.get_id("index-action-person-1") {
                                                                    NiceSelect::new_default_with_value(&elm, &doctorcode);
                                                                }
                                                                modal.action_person_1.set_neq(doctorcode);
                                                            }))
                                                        }),
                                                        html!("div", {
                                                            .class(class::FLEX_GROW1)
                                                            .future(modal.redraw_person_1.signal().for_each(clone!(app, modal => move |redraw| {
                                                                if redraw {
                                                                    if let Some(elm) = app.get_id("index-action-person-1") {
                                                                        NiceSelect::new_default_with_value(&elm, &modal.action_person_1.lock_ref());
                                                                    }
                                                                    modal.redraw_person_1.set_neq(false);
                                                                }
                                                                async {}
                                                            })))
                                                            .child(html!("select" => HtmlSelectElement, {
                                                                .class("form-control")
                                                                .attr("id", "index-action-person-1")
                                                                .child(html!("option", {
                                                                    .attr("value", "")
                                                                    .text("เลือก")
                                                                }))
                                                                .children(all_doctor_select_option.iter().map(|option| {
                                                                    doms::select_option(option, &modal.action_person_1.lock_ref())
                                                                }))
                                                                .apply(mixins::string_value_select(modal.action_person_1.clone(), modal.changed.clone()))
                                                            }))
                                                        }),
                                                    ])
                                                }))
                                            }),
                                            html!("div", {
                                                .class("col-12")
                                                .style("min-width","368px")
                                                .visible_signal(modal.action_blood_had.signal_cloned().map(|had| had.as_str() == "Y"))
                                                .child(html!("div", {
                                                    .class(class::INPUT_GROUP)
                                                    .children([
                                                        doms::label_group_for("index-action-person-2","ลงชื่อ 2"),
                                                        html!("div", {
                                                            .class(class::FLEX_GROW1)
                                                            .future(map_ref! {
                                                                let had = modal.action_blood_had.signal_cloned(),
                                                                let redraw = modal.redraw_person_2.signal() =>
                                                                had.as_str() == "Y" || *redraw
                                                            }.for_each(clone!(app, modal => move |redraw| {
                                                                if redraw {
                                                                    if let Some(elm) = app.get_id("index-action-person-2") {
                                                                        NiceSelect::new_default_with_value(&elm, &modal.action_person_2.lock_ref());
                                                                    }
                                                                    modal.redraw_person_2.set_neq(false);
                                                                }
                                                                async {}
                                                            })))
                                                            .child(html!("select" => HtmlSelectElement, {
                                                                .class("form-control")
                                                                .attr("id", "index-action-person-2")
                                                                .child(html!("option", {
                                                                    .attr("value", "")
                                                                    .text("เลือก")
                                                                }))
                                                                .children(all_doctor_select_option.iter().map(|option| {
                                                                    // doms::select_option(option, &modal.action_person_2.lock_ref())
                                                                    let selected_value = modal.action_person_2.lock_ref();
                                                                    html!("option", {
                                                                        .attr("value", &option.key.to_owned())
                                                                        .apply_if(!selected_value.is_empty() && selected_value.as_str() == option.key.as_str(), |dom| { dom
                                                                            .attr("selected","")
                                                                        })
                                                                        .text(&option.key)
                                                                        .text(" : ")
                                                                        .text(&option.value)
                                                                    })
                                                                }))
                                                                .apply(mixins::string_value_select(modal.action_person_2.clone(), modal.changed.clone()))
                                                            }))
                                                        })
                                                    ])
                                                }))
                                            })
                                        ])
                                    }),
                                ])
                            }),
                            html!("div", {
                                .class("row")
                                .child(html!("div", {
                                    .class(class::COL12_Y)
                                    .child(html!("button", {
                                        .attr("type", "button")
                                        .class(class::BTN_FR_GRAY)
                                        .child(html!("i", {.class(class::FA_X)}))
                                        .text(" ยกเลิก")
                                        .event(clone!(modal => move |_: events::Click| {
                                            modal.action_id.set_neq(None);
                                        }))
                                    }))
                                    .child_signal(modal.action_id.signal_cloned().map(clone!(app, modal => move |action_id| {
                                        (action_id.and_then(zero_none).is_some() && if is_ipd {
                                            app.endpoint_is_allow(&Method::DELETE, &EndPoint::IpdIndexActionId, is_pre_admit)
                                        } else {
                                            app.endpoint_is_allow(&Method::DELETE, &EndPoint::OpdErIndexActionId, false)
                                        }).then(|| {
                                            html!("button" => HtmlButtonElement, {
                                                .attr("type", "button")
                                                .class(class::BTN_RED)
                                                .child(html!("i", {.class(class::FA_TRASH)}))
                                                .text(" ลบ")
                                                .apply(mixins::click_with_loader_checked(clone!(app, modal => move || {
                                                    Self::delete_index_action(modal.clone(), app.clone());
                                                }), app.state()))
                                            })
                                        })
                                    })))
                                    .child_signal(modal.action_id.signal_cloned().map(clone!(app, modal => move |action_id| {
                                        (if is_ipd {
                                            app.endpoint_is_allow(&Method::POST, &EndPoint::IpdIndexAction, is_pre_admit)
                                            && if action_id.and_then(zero_none).is_some() {
                                                app.has_permission(Permission::IpdNurseIndexEdit) || (is_pre_admit && app.has_permission(Permission::OpdErNurseIndexEdit))
                                            } else {
                                                app.has_permission(Permission::IpdNurseIndexAdd) || (is_pre_admit && app.has_permission(Permission::OpdErNurseIndexAdd))
                                            }
                                        } else {
                                            app.endpoint_is_allow(&Method::POST, &EndPoint::OpdErIndexAction, false)
                                            && if action_id.and_then(zero_none).is_some() {
                                                app.has_permission(Permission::OpdErNurseIndexEdit)
                                            } else {
                                                app.has_permission(Permission::OpdErNurseIndexAdd)
                                            }
                                        }).then(|| {
                                            html!("button" => HtmlButtonElement, {
                                                .attr("type", "button")
                                                .class(class::BTN_FR_L)
                                                .class_signal("btn-primary", modal.is_action_valid())
                                                .class_signal("btn-secondary", not(modal.is_action_valid()))
                                                .child(html!("i", {.class(class::FA_SAVE)}))
                                                .text(" บันทึก")
                                                .apply(mixins::click_with_loader_checked_or_true_disable_signal(clone!(app, modal => move || {
                                                    Self::submit_action(modal.clone(), app.clone());
                                                }), not(modal.is_action_valid()), app.state()))
                                            })
                                        })
                                    })))
                                }))
                            }),
                        ])
                    })
                })
            })))
        })
    }

    fn render_monitor_inputs(modal: Rc<Self>, app: Rc<App>) -> Dom {
        let (is_ipd, is_pre_admit) = modal.is_ipd_and_is_pre_admit();

        html!("div", {
            .class(class::BORDER_ROUND)
            .class_signal("bg-info-subtle", modal.is_new_monitor_signal())
            .class_signal("bg-secondary-subtle", not(modal.is_new_monitor_signal()))
            .children([
                html!("div", {
                    .class(class::FLEX_C_T)
                    .children([
                        html!("div", {
                            .class(class::FLEX_ITEM_GROW1_L2)
                            .child(html!("div", {
                                .class(class::INPUT_GROUP)
                                .children([
                                    doms::label_group_for("monitor-datetime","Monitor"),
                                    doms::datetime_picker(
                                        modal.monitor_datetime.clone(),
                                        modal.changed.clone(), always(false),
                                        |d| d.class(class::FLEX_GROW1).style("min-width","190px"),
                                        |d| d.class("rounded-start-0"),
                                        |d| d.class("rounded-start-0").attr("id", "monitor-datetime"),
                                        |s| s, always(None),
                                    ),
                                ])
                            }))
                        }),
                        html!("button", {
                            .attr("type", "button")
                            .class(class::BTN_L_CYAN)
                            .style("white-space","nowrap")
                            .child(html!("i", {.class(class::FA_CLOCK)}))
                            // .text(" ปัจจุบัน")
                            .event(clone!(modal => move |_: events::Click| {
                                modal.monitor_datetime.set_neq(js_now().js_string());
                                modal.changed.set_neq(true);
                            }))
                        }),
                        html!("div", {
                            .class(class::FLEX_ITEM_GROW1_R2)
                            // monitor-abnormal toggle
                            .child(html!("div", {
                                .class(class::INPUT_GROUP)
                                .attr("role","group")
                                .attr("aria-label","Monitor status toggle button group")
                                .children([
                                    html!("span", {.class("input-group-text").text("สถานะ")}),
                                    html!("input" => HtmlInputElement, {
                                        .attr("type", "radio")
                                        .class("btn-check")
                                        .attr("id", "monitor-wait")
                                        .attr("autocomplete","off")
                                        .apply(mixins::radio_opt_match_or_none(modal.monitor_abnormal.clone(), modal.changed.clone(), ""))
                                    }),
                                    html!("label", {
                                        .class(class::BTN_BLUEO)
                                        .attr("for", "monitor-wait")
                                        .child(html!("i", {.class(class::FA_HOURGLASS_GOLD)}))
                                        .text(" รอประเมิน")
                                    }),
                                    html!("input" => HtmlInputElement, {
                                        .attr("type", "radio")
                                        .class("btn-check")
                                        .attr("id", "monitor-normal")
                                        .attr("autocomplete","off")
                                        .apply(mixins::radio_opt_match(modal.monitor_abnormal.clone(), modal.changed.clone(), "N"))
                                    }),
                                    html!("label", {
                                        .class(class::BTN_BLUEO)
                                        .attr("for", "monitor-normal")
                                        .child(html!("i", {.class(class::FA_CHECK_CIRCLE_GREEN)}))
                                        .text(" ปกติ")
                                    }),
                                    html!("input" => HtmlInputElement, {
                                        .attr("type", "radio")
                                        .class("btn-check")
                                        .attr("id", "monitor-abnormal")
                                        .attr("autocomplete","off")
                                        .apply(mixins::radio_opt_match(modal.monitor_abnormal.clone(), modal.changed.clone(), "Y"))
                                    }),
                                    html!("label", {
                                        .class(class::BTN_BLUEO)
                                        .attr("for", "monitor-abnormal")
                                        .child(html!("i", {.class(class::FA_ALERT_RED)}))
                                        .text(" ผิดปกติ")
                                    }),
                                ])
                            }))
                        }),
                    ])
                }),
                html!("div", {
                    .class(class::ROW_GT)
                    .children([
                        html!("div", {
                            .class("col-sm-6")
                            .child(html!("div", {
                                .class("input-group")
                                .child(html!("div", {
                                    .class("form-floating")
                                    .children([
                                        html!("textarea" => HtmlTextAreaElement, {
                                            .class("form-control")
                                            .attr("id", "monitor-result")
                                            .attr("placeholder","Monitor Result")
                                            .apply(mixins::textarea_value_auto_expand(modal.monitor_result.clone(), modal.changed.clone()))
                                        }),
                                        html!("label", {
                                            .attr("for", "monitor-result")
                                            .text("รายละเอียด")
                                        }),
                                    ])
                                }))
                            }))
                        }),
                        html!("div", {
                            .class("col-sm-6")
                            .child(html!("div", {
                                .class("form-floating")
                                .children([
                                    html!("textarea" => HtmlTextAreaElement, {
                                        .class("form-control")
                                        .attr("id", "monitor-remark")
                                        .attr("placeholder","Monitor Remark")
                                        .apply(mixins::textarea_value_auto_expand(modal.monitor_remark.clone(), modal.changed.clone()))
                                    }),
                                    html!("label", {
                                        .attr("for", "monitor-remark")
                                        .text("หมายเหตุ")
                                    }),
                                ])
                            }))
                        }),
                    ])
                }),
                html!("div", {
                    .class("row")
                    .child(html!("div", {
                        .class(class::COL12_Y)
                        .child(html!("button", {
                            .attr("type", "button")
                            .class(class::BTN_FR_GRAY)
                            .child(html!("i", {.class(class::FA_X)}))
                            .text(" ยกเลิก")
                            .event(clone!(modal => move |_: events::Click| {
                                modal.monitor_id.set_neq(None);
                            }))
                        }))
                        .child_signal(modal.monitor_id.signal().map(clone!(app, modal => move |monitor_id| {
                            (monitor_id.and_then(zero_none).is_some() && if is_ipd {
                                app.endpoint_is_allow(&Method::DELETE, &EndPoint::IpdIndexMonitorId, is_pre_admit)
                            } else {
                                app.endpoint_is_allow(&Method::DELETE, &EndPoint::OpdErIndexMonitorId, false)
                            }).then(|| {
                                html!("button" => HtmlButtonElement, {
                                    .attr("type", "button")
                                    .class(class::BTN_RED)
                                    .child(html!("i", {.class(class::FA_TRASH)}))
                                    .text(" ลบ")
                                    .apply(mixins::click_with_loader_checked(clone!(app, modal => move || {
                                        Self::delete_index_monitor(modal.clone(), app.clone());
                                    }), app.state()))
                                })
                            })
                        })))
                        .apply_if(app.doctor_code().is_some(), |dom| dom
                            .child_signal(modal.monitor_id.signal_cloned().map(clone!(app, modal => move |monitor_id| {
                                (if is_ipd {
                                    app.endpoint_is_allow(&Method::POST, &EndPoint::IpdIndexMonitor, is_pre_admit)
                                    && if monitor_id.and_then(zero_none).is_some() {
                                        app.has_permission(Permission::IpdNurseIndexEdit) || (is_pre_admit && app.has_permission(Permission::OpdErNurseIndexEdit))
                                    } else {
                                        app.has_permission(Permission::IpdNurseIndexAdd) || (is_pre_admit && app.has_permission(Permission::OpdErNurseIndexAdd))
                                    }
                                } else {
                                    app.endpoint_is_allow(&Method::POST, &EndPoint::OpdErIndexMonitor, false)
                                    && if monitor_id.and_then(zero_none).is_some() {
                                        app.has_permission(Permission::OpdErNurseIndexEdit)
                                    } else {
                                        app.has_permission(Permission::OpdErNurseIndexAdd)
                                    }
                                }).then(|| {
                                    html!("button" => HtmlButtonElement, {
                                        .attr("type", "button")
                                        .class(class::BTN_FR_L)
                                        .class_signal("btn-primary", modal.is_monitor_valid())
                                        .class_signal("btn-secondary", not(modal.is_monitor_valid()))
                                        .child(html!("i", {.class(class::FA_SAVE)}))
                                        .text(" บันทึก")
                                        .apply(mixins::click_with_loader_checked_or_true_disable_signal(clone!(app, modal => move || {
                                            Self::submit_monitor(modal.clone(), app.clone());
                                        }), not(modal.is_monitor_valid()), app.state()))
                                    })
                                })
                            })))
                        )
                    }))
                }),
            ])
        })
    }

    fn plan_dom(plan: Rc<IndexPlan>, modal: Rc<Self>, custom_label: Option<String>) -> Dom {
        let label = custom_label.unwrap_or([date_th_opt(&plan.plan_date), time_hm_opt(&plan.plan_time)].join(" "));
        let actions_len = modal
            .plan_actions(Some(plan.plan_id), modal.order_item.get_cloned(), false)
            .map(|actions| actions.len())
            .unwrap_or_default();
        let is_same_plan_id = modal.plan_id.get().map(|id| id == plan.plan_id).unwrap_or_default();
        html!("a", {
            .apply(|dom| {
                if is_same_plan_id {
                    dom.class(class::NAV_ITEM_LINK_ACTIVE)
                } else {
                    dom.class(class::NAV_ITEM_LINK)
                }
            })
            .class(class::BORDER_LT_BLUE)
            .attr("data-bs-toggle", "pill")
            .attr("href", "#")
            .text(&label)
            .apply_if(actions_len > 0, |dom| {
                dom.child(html!("span", {.class(class::BADGE_GOLD_R).style("cursor","default").text(&actions_len.to_string())}))
            })
            .event_with_options(&EventOptions::preventable(), clone!(modal => move |event: events::Click| {
                event.prevent_default();
                modal.selected_plan.set(Some(plan.clone()));
                match modal.form_type.get_cloned() {
                    FormType::Plan => {
                        modal.set_plan(&plan);
                        modal.set_no_more_action();
                    }
                    FormType::Action => {
                        modal.plan_id.set_neq(zero_none(plan.plan_id));
                        modal.plan_detail.set_neq(plan.plan_detail.clone().unwrap_or_default());
                        // modal.selected_plan_time.set(plan.plan_time);
                        modal.set_new_or_no_action();
                    }
                    FormType::Monitor => {}
                }
            }))
        })
    }

    fn plan_dom_new(modal: Rc<Self>, date: Option<Date>) -> Dom {
        html!("a", {
            .class(class::NAV_ITEM_LINK_R)
            .class(class::TXT_BG_BLUE_RT)
            .attr("data-bs-toggle", "pill")
            .attr("href", "#")
            .child(html!("i", {.class(class::FA_PLUS_L)}))
            .text("เพิ่ม")
            .event_with_options(&EventOptions::preventable(), clone!(modal => move |event: events::Click| {
                event.prevent_default();
                modal.set_new_plan(date);
                // modal.plan_times_in_date.set(plan_times_in_date.clone());
            }))
        })
    }

    fn plan_dom_new_now(modal: Rc<Self>, app: Rc<App>) -> Dom {
        html!("button" => HtmlButtonElement, {
            .attr("type", "button")
            .class(class::BTN_FR_LT_REDO)
            .child(html!("i", {.class(class::FA_SAVE)}))
            .text(" Plan NOW")
            .apply(mixins::click_with_loader_checked(clone!(app => move || {
                Self::submit_plan_new_now(modal.clone(), app.clone());
            }), app.state()))
        })
    }

    fn plan_dom_new_and_action_now(modal: Rc<Self>, app: Rc<App>) -> Dom {
        let (is_ipd, is_pre_admit) = modal.is_ipd_and_is_pre_admit();
        let allow = if is_ipd {
            app.endpoint_is_allow(&Method::POST, &EndPoint::IpdIndexAction, is_pre_admit) && app.has_permission(Permission::IpdNurseIndexAdd)
                || (is_pre_admit && app.has_permission(Permission::OpdErNurseIndexAdd))
        } else {
            app.endpoint_is_allow(&Method::POST, &EndPoint::OpdErIndexAction, false) && app.has_permission(Permission::OpdErNurseIndexAdd)
        };
        if allow {
            html!("button" => HtmlButtonElement, {
                .attr("type", "button")
                .class(class::BTN_FR_LT_REDO)
                .child(html!("i", {.class(class::FA_SAVE)}))
                .text(" Plan & Action NOW")
                .apply(mixins::click_with_loader_checked(clone!(app => move || {
                    Self::submit_plan_new_and_action_now(modal.clone(), app.clone());
                }), app.state()))
            })
        } else {
            Dom::empty()
        }
    }

    /// - `order_item_opt` is None: None
    /// - `plan_id_opt` is None: all actions / None
    /// - `plan_id_opt` is Some(0): None
    /// - `plan_id_opt` is Some(plan_id) : actions of selected plan_id
    fn plan_actions(&self, plan_id_opt: Option<u32>, order_item_opt: Option<Rc<OrderItem>>, allow_all: bool) -> Option<Vec<Rc<IndexAction>>> {
        order_item_opt.and_then(|order_item| {
            match plan_id_opt {
                Some(0) => None,
                Some(plan_id) => order_item.index_plans.iter().find(|plan| plan.plan_id == plan_id).map(|plan| {
                    plan.actions
                        .iter()
                        //.filter(|action| action.action_date.is_some() && action.action_time.is_some())
                        .rev()
                        .cloned()
                        .map(Rc::new)
                        .collect()
                }),
                None => {
                    if allow_all {
                        let mut actions = order_item
                            .index_plans
                            .iter()
                            .flat_map(|plan| plan.actions.clone())
                            // .flat_map(|plan| plan.actions.iter().filter(|action| action.action_date.is_some() && action.action_time.is_some()))
                            // .cloned()
                            .map(Rc::new)
                            .collect::<Vec<Rc<IndexAction>>>();
                        actions.sort();
                        actions.reverse();
                        if actions.is_empty() { None } else { Some(actions) }
                    } else {
                        None
                    }
                }
            }
        })
    }

    fn action_monitors(&self, action_id: u32, order_item_opt: Option<Rc<OrderItem>>) -> Vec<Rc<IndexMonitor>> {
        order_item_opt
            .map(|order_item| {
                order_item
                    .index_plans
                    .iter()
                    .flat_map(|plan| {
                        plan.actions
                            .iter()
                            .find(|action| action.action_id == Some(action_id))
                            .map(|action| action.monitors.clone())
                            .unwrap_or_default()
                    })
                    .rev()
                    .map(Rc::new)
                    .collect()
            })
            .unwrap_or_default()
    }

    fn order_item_plans_with_sch_type(&self, sch_type: &'static str) -> impl Signal<Item = Vec<Rc<IndexPlan>>> + use<> {
        self.order_item.signal_cloned().map(move |opt| {
            opt.as_ref()
                .map(|order_item| {
                    order_item
                        .index_plans
                        .iter()
                        .filter(|plan| plan.plan_sch_type == Some(String::from(sch_type)))
                        .cloned()
                        .map(Rc::new)
                        .collect()
                })
                .unwrap_or_default()
        })
    }

    fn set_order_item(&self, order_item: Option<Rc<OrderItem>>) {
        self.order_item.set(order_item.clone());
        if let Some(oi) = order_item {
            // for STAT first
            if oi.stat == Some(String::from("Y")) && oi.index_plans.is_empty() {
                // && datetime_from_opt(oi.order_date, oi.order_time).map(|dt| (dt - js_now()).abs().whole_hours() < 1).unwrap_or_default() {
                self.active_tab.set_neq(PlanTab::Stat);
            }
            if let Some(plan_id) = self.plan_id.get().and_then(zero_none) {
                if let Some(plan) = oi.index_plans.iter().find(|plan| plan.plan_id == plan_id) {
                    self.active_tab.set_neq(PlanTab::from_sch_type(&plan.plan_sch_type));
                    self.selected_plan.set(Some(Rc::new(plan.clone())));
                    match self.form_type.get_cloned() {
                        FormType::Plan => {
                            self.set_plan(plan);
                            self.set_no_more_action();
                        }
                        FormType::Action => {
                            self.plan_id.set(zero_none(plan.plan_id));
                            self.plan_detail.set_neq(plan.plan_detail.clone().unwrap_or_default());
                            if let Some(action_id) = self.action_id.get() {
                                if let Some(action) = plan.actions.iter().find(|action| action.action_id == Some(action_id)) {
                                    self.set_action(&Rc::new(action.clone()));
                                } else {
                                    self.set_no_more_action();
                                }
                            }
                        }
                        FormType::Monitor => {}
                    }
                }
            }
        }
    }

    fn set_no_more_action(&self) {
        self.action_id.set_neq(if self.no_more_action() { None } else { Some(0) });
    }

    fn set_no_plan(&self) {
        self.plan_id.set_neq(None);
        self.plan_date.set_neq(String::new());
        self.plan_time.set_neq(String::new());
        self.plan_detail.set_neq(String::new());

        self.action_id.set_neq(None);
        self.plan_times_in_date.set_neq(None);
        self.allow_plan_before_order.set_neq(String::new());
        self.changed.set_neq(false);
    }

    /// set new plan using order_item's `order_date` and `order_time`, or now
    fn set_new_plan(&self, date_opt: Option<Date>) {
        let now = js_now();
        let (order_or_now_date, order_or_now_time) = self
            .order_item
            .lock_ref()
            .as_ref()
            .map(|order_item| (order_item.order_date.unwrap_or(now.date()), order_item.order_time.unwrap_or(now.time())))
            .unwrap_or((now.date(), now.time()));

        self.plan_id.set_neq(Some(0));
        self.plan_date.set_neq(date_opt.unwrap_or(order_or_now_date).to_string());
        self.plan_time.set_neq(order_or_now_time.js_string());
        self.plan_detail.set_neq(String::new());

        self.action_id.set_neq(None);
        self.plan_times_in_date.set_neq(None);
        self.allow_plan_before_order.set_neq(String::new());
        self.changed.set_neq(true);
    }

    fn set_plan(&self, plan: &IndexPlan) {
        self.plan_id.set_neq(Some(plan.plan_id));
        self.plan_date.set_neq(plan.plan_date.map(|d| d.to_string()).unwrap_or_default());
        self.plan_time.set_neq(plan.plan_time.map(|t| t.js_string()).unwrap_or_default());
        self.plan_detail.set_neq(plan.plan_detail.clone().unwrap_or_default());
        self.changed.set_neq(false);
    }

    fn set_new_or_no_action(&self) {
        if self.no_more_action() {
            self.set_no_action();
        } else {
            self.set_new_action();
        }
    }

    fn no_more_action(&self) -> bool {
        let actions_len = self
            .plan_actions(self.plan_id.get(), self.order_item.get_cloned(), false)
            .map(|actions| actions.len())
            .unwrap_or_default();
        matches!(
            (self.active_tab.get_cloned(), self.is_continuous(), actions_len > 0),
            (PlanTab::Time, false, true) | (PlanTab::Stat, _, true)
        )
    }

    fn set_no_action(&self) {
        let now = js_now();
        self.action_id.set_neq(None);
        self.vs_id.set_neq(None);
        self.check_person.set_neq(String::new());

        if self.is_med() {
            self.check_datetime.set_neq(now.js_string());
        } else {
            self.check_datetime.set_neq(String::new());
        }
        self.action_date.set_neq(now.date().to_string());
        self.action_time.set_neq(now.time().js_string());

        self.action_result.set_neq(String::new());
        self.action_remark.set_neq(String::new());
        self.action_blood_had.set_neq(String::new());
        self.action_person_1.set_neq(String::new());
        self.action_person_2.set_neq(String::new());
        self.changed.set_neq(false);
    }

    fn set_new_action(&self) {
        let now = js_now();
        let is_med = self.is_med();

        match self.selected_plan.lock_ref().as_ref() {
            Some(plan) => {
                let date_now = now.date();
                let time_now = now.time();
                let is_cont_or_none = plan.is_continuous_or_none();

                match plan.plan_sch_type.clone().unwrap_or_default().as_str() {
                    "date" => {
                        if is_cont_or_none {
                            if is_med {
                                self.check_datetime.set_neq(now.js_string());
                            } else {
                                self.check_datetime.set_neq(String::new());
                            }
                            self.action_date.set_neq(date_now.to_string());
                            self.action_time.set(time_now.js_string());
                        } else {
                            let action_date = if let Some(plan_next_dt) = datetime_from_opt(plan.plan_date.and_then(|d| d.next_day()), plan.plan_time) {
                                if now > plan_next_dt {
                                    plan_next_dt.date()
                                } else if plan.plan_date.map(|plan_date| plan_date >= date_now).unwrap_or_default() {
                                    date_now
                                } else {
                                    plan.plan_date.unwrap_or(date_now)
                                }
                            } else {
                                date_now
                            };
                            if is_med {
                                self.check_datetime.set_neq(PrimitiveDateTime::new(action_date, time_now).js_string());
                            } else {
                                self.check_datetime.set_neq(String::new());
                            }
                            self.action_date.set_neq(action_date.to_string());
                            self.action_time.set(time_now.js_string());
                        }
                    }
                    "stat" => {
                        let action_date = plan.plan_date.unwrap_or(date_now);
                        let action_time = plan.plan_time.unwrap_or(time_now);
                        if is_med {
                            self.check_datetime.set_neq(PrimitiveDateTime::new(action_date, action_time).js_string());
                        } else {
                            self.check_datetime.set_neq(String::new());
                        }
                        self.action_date.set_neq(action_date.to_string());
                        self.action_time.set(action_time.js_string());
                    }
                    _ => {
                        let action_date = if is_cont_or_none { date_now } else { plan.plan_date.unwrap_or(date_now) };
                        let action_time = plan.plan_time.unwrap_or(time_now);
                        if is_med {
                            self.check_datetime.set_neq(PrimitiveDateTime::new(action_date, action_time).js_string());
                        } else {
                            self.check_datetime.set_neq(String::new());
                        }
                        self.action_date.set_neq(action_date.to_string());
                        self.action_time.set(action_time.js_string());
                    }
                }
            }
            None => {
                if is_med {
                    self.check_datetime.set_neq(now.js_string());
                } else {
                    self.check_datetime.set_neq(String::new());
                }
                self.action_date.set_neq(now.date().to_string());
                self.action_time.set_neq(now.time().js_string());
            }
        }

        self.action_id.set_neq(Some(0));
        self.vs_id.set_neq(None);
        self.check_person.set_neq(String::new());
        self.action_result.set_neq(String::new());
        self.action_remark.set_neq(String::new());
        self.action_blood_had.set_neq(String::new());
        self.action_person_1.set_neq(String::new());
        self.action_person_2.set_neq(String::new());
        self.changed.set_neq(true);
    }

    fn go_and_set_action(&self, action: &Rc<IndexAction>) {
        self.form_type.set_neq(FormType::Action);
        if let Some(plan_id) = action.plan_id {
            if let Some(plan) = self
                .order_item
                .lock_ref()
                .as_ref()
                .and_then(|order_item| order_item.index_plans.iter().find(|plan| plan.plan_id == plan_id).cloned())
            {
                self.active_tab.set(PlanTab::from_sch_type(&plan.plan_sch_type));
                self.plan_detail.set_neq(plan.plan_detail.clone().unwrap_or_default());
            }
        }
        self.plan_id.set(action.plan_id);
        self.set_action(action);
    }

    fn set_action(&self, action: &Rc<IndexAction>) {
        self.action_id.set_neq(action.action_id);
        self.vs_id.set_neq(action.vs_id);
        self.check_datetime.set_neq(action.check_datetime.map(|dt| dt.js_string()).unwrap_or_default());
        self.check_person.set_neq(action.check_person.clone().unwrap_or_default());
        self.action_date.set_neq(action.action_date.map(|d| d.to_string()).unwrap_or_default());
        self.action_time.set_neq(action.action_time.map(|t| t.js_string()).unwrap_or_default());
        self.action_result.set_neq(action.action_result.clone().unwrap_or_default());
        self.action_remark.set_neq(action.action_remark.clone().unwrap_or_default());
        self.action_blood_had.set_neq(action.action_blood_had.clone().unwrap_or_default());
        self.action_person_1.set_neq(action.action_person_1.clone().unwrap_or_default());
        self.action_person_2.set_neq(action.action_person_2.clone().unwrap_or_default());

        if action.check_person.is_some() {
            self.redraw_check_person.set(true);
        }
        if action.action_person_1.is_some() {
            self.redraw_person_1.set(true);
        }
        if action.action_blood_had == Some(String::from("Y")) && action.action_person_2.is_some() {
            self.redraw_person_2.set(true);
        }

        self.changed.set_neq(false);
    }

    fn set_no_monitor(&self) {
        self.monitor_id.set_neq(None);
        self.monitor_datetime.set_neq(String::new());
        self.monitor_abnormal.set_neq(None);
        self.monitor_result.set_neq(String::new());
        self.monitor_remark.set_neq(String::new());
        self.changed.set_neq(false);
    }

    fn set_new_monitor(&self, monitor_datetime: PrimitiveDateTime) {
        self.monitor_id.set_neq(Some(0));
        self.monitor_datetime.set_neq(monitor_datetime.js_string());
        self.monitor_abnormal.set_neq(None);
        self.monitor_result.set_neq(String::new());
        self.monitor_remark.set_neq(String::new());
        // here is ready to submit because we already set default monitor_datetime
        self.changed.set_neq(true);
    }

    fn set_monitor(&self, monitor: &Rc<IndexMonitor>) {
        self.monitor_id.set_neq(monitor.monitor_id);
        self.monitor_datetime.set_neq(monitor.monitor_datetime.map(|dt| dt.js_string()).unwrap_or_default());
        self.monitor_abnormal.set_neq(monitor.monitor_abnormal.clone());
        self.monitor_result.set_neq(monitor.monitor_result.clone().unwrap_or_default());
        self.monitor_remark.set_neq(monitor.monitor_remark.clone().unwrap_or_default());

        self.changed.set_neq(false);
    }

    fn is_plan_valid(&self) -> impl Signal<Item = bool> + use<> {
        map_ref! {
            let changed = self.changed.signal(),
            let plan_date = self.plan_date.signal_cloned(),
            let plan_time = self.plan_time.signal_cloned() =>
            *changed && !plan_date.is_empty() && !plan_time.is_empty()
        }
    }

    fn is_check_valid(&self) -> impl Signal<Item = bool> + use<> {
        let is_med = self.is_med();
        map_ref! {
            let changed = self.changed.signal(),
            let check_datetime = self.check_datetime.signal_cloned(),
            let check_person = self.check_person.signal_cloned() =>
            !(!changed || (is_med && (check_datetime.is_empty() || check_person.is_empty())))
        }
    }

    fn is_action_valid(&self) -> impl Signal<Item = bool> + use<> {
        let is_med = self.is_med();
        map_ref! {
            let changed = self.changed.signal(),
            let check_datetime = self.check_datetime.signal_cloned(),
            let check_person = self.check_person.signal_cloned(),
            let action_date = self.action_date.signal_cloned(),
            let action_time = self.action_time.signal_cloned(),
            let action_blood_had = self.action_blood_had.signal_cloned(),
            let action_person_1 = self.action_person_1.signal_cloned(),
            let action_person_2 = self.action_person_2.signal_cloned() =>
            !(!changed || (is_med && (check_datetime.is_empty() || check_person.is_empty()))
                || action_date.is_empty() || action_time.is_empty() || action_person_1.is_empty()
                || (action_blood_had == "Y" && (action_person_2.is_empty() || action_person_1 == action_person_2))
            )
        }
    }

    fn is_monitor_valid(&self) -> impl Signal<Item = bool> + use<> {
        map_ref! {
            let changed = self.changed.signal(),
            let has_action_id = self.action_id.signal().map(|opt| opt.and_then(zero_none).is_some()),
            let monitor_datetime = self.monitor_datetime.signal_cloned() =>
            *changed && *has_action_id && !monitor_datetime.is_empty()
        }
    }
}

fn render_order_item(order_item: Rc<OrderItem>, app: Rc<App>) -> Dom {
    let is_oneday = order_item.order_type.clone().unwrap_or_default().as_str() == "oneday";
    let will_blue = if is_oneday {
        vec!["med", "home-medication", "injection", "ivfluid"]
    } else {
        vec!["med", "injection", "ivfluid"]
    };
    let (owner_text, owner_bg) = match order_item.order_owner_type.clone().unwrap_or_default().as_str() {
        "doctor" => ("Doctor", "bg-primary"),
        "nurse" => ("Nurse", "text-bg-warning"),
        _ => ("", "bg-danger"),
    };

    let is_injection = Mutable::new(order_item.order_item_type.as_ref().and_then(|ty| match ty.as_str() {
        "med" => Some(false),
        "injection" => Some(true),
        _ => None,
    }));
    let is_injection_changed = Mutable::new(false);
    let nurse_assign = Mutable::new(order_item.nurse_assign.clone().unwrap_or_default());
    let nurse_assign_changed = Mutable::new(false);

    html!("div", {
        .class(class::BOX_ROUND_T)
        .children([
            html!("span", {
                .text(&[date_th_opt(&order_item.order_date), time_hm_opt(&order_item.order_time)].join(" "))
            }),
            html!("span", {
                .class(class::BADGE_R)
                .class(owner_bg)
                .style("cursor","default")
                .text(owner_text)
            }),
            html!("span", {
                .class(class::BADGE_CYAN_R)
                .style("cursor","default")
                .text(if is_oneday {"One Day Order"} else {"Continuous Order"})
            }),
            html!("div", {
                .class(class::FLEX_END)
                .style("max-width","220px")
                .apply(|dom| {
                    let (is_ipd, is_pre_admit) = order_item.visit_type.is_ipd_and_is_pre_admit();
                    let is_allow = if is_ipd {
                        app.endpoint_is_allow(&Method::PATCH, &EndPoint::IpdOrderItem, is_pre_admit)
                    } else {
                        app.endpoint_is_allow(&Method::PATCH, &EndPoint::OpdErOrderItem, false)
                    };
                    if is_allow { dom
                        .child(doms::nurse_assign_dropdown(nurse_assign.clone(), nurse_assign_changed.clone(), app.state()))
                        .child_signal(is_injection.signal().map(clone!(is_injection, is_injection_changed => move |opt| {
                            opt.map(|is_inj| {
                                html!("div", {
                                    // .class(class::FORM_CHK_SW)
                                    .class("ms-2")
                                    .children([
                                        html!("input" => HtmlInputElement, {
                                            .attr("type", "checkbox")
                                            .attr("role", "switch")
                                            .visible(false)
                                            .attr("id", "order-item-is-injection")
                                            .attr("value", "Y")
                                            .apply(mixins::checkbox_some_bool(is_injection.clone(), is_injection_changed.clone()))
                                        }),
                                        html!("label", {
                                            .class("form-check-label")
                                            .attr("for", "order-item-is-injection")
                                            .style("font-size", "28px")
                                            .style("line-height", "100%")
                                            .style("user-select", "none")
                                            .child(html!("i", {
                                                .class(if is_inj {class::FA_SYRINGE} else {class::FA_PILLS})
                                                .style("color", if is_inj {"red"} else {"inherit" })
                                            }))
                                        }),
                                    ])
                                })
                            })
                        })))
                        .future(map_ref!(
                            let busy = app.loader_is_loading(),
                            let edit = nurse_assign_changed.signal() =>
                            !busy && *edit
                        ).for_each(clone!(app, order_item, nurse_assign_changed => move |ready| {
                            if ready {
                                nurse_assign_order_item(order_item.clone(), nurse_assign.clone(), app.clone());
                                nurse_assign_changed.set_neq(false);
                            }
                            async {}
                        })))
                        .future(map_ref!(
                            let busy = app.loader_is_loading(),
                            let edit = is_injection_changed.signal() =>
                            !busy && *edit
                        ).for_each(clone!(app, order_item, is_injection_changed => move |ready| {
                            if ready {
                                is_injection_order_item(order_item.clone(), is_injection.clone(), app.clone());
                                is_injection_changed.set_neq(false);
                            }
                            async {}
                        })))
                    } else { dom
                        .child(html!("div", {
                            .class(class::INPUT_GROUP_SM)
                            .children([
                                doms::span_group_text("Assign"),
                                doms::span_group_text(&order_item.nurse_assign.clone().unwrap_or(String::from("ทั้งหมด"))),
                            ])
                        }))
                    }
                })
            }),
            html!("div", {
                .class(class::BORDER_T_T)
                .child(html!("div", {
                    .class("clearfix")
                    .apply(|dom| {
                        // OFF
                        if order_item.order_item_type == Some(String::from("off")) { dom
                            .child(html!("span", {.class(class::BADGE_GOLD_L).style("cursor","default").text("OFF")}))
                            .child(html!("span", {
                                .child(html!("span", {
                                    .text(&order_item.off_med_name.clone().unwrap_or_default())
                                    .apply_if(order_item.off_icode.is_some(), |d| d.class(class::BOLD_BLUE_EM_L).text("\n"))
                                }))
                                .text(&order_item.off_order_item_detail.clone().unwrap_or_default())
                            }))
                        // NOT OFF
                        } else { dom
                            .child(html!("span", {
                                .child(html!("span", {
                                    .apply_if(order_item.order_item_type.as_ref().map(|ty| will_blue.contains(&ty.as_str())).unwrap_or_default(), |d| d.class(class::BOLD_BLUE_EM_L))
                                    .text(&order_item.med_name.clone().unwrap_or_default())
                                }))
                                // Drug allergy badge
                                .apply_if(order_item.allergy_agent_symptom.is_some(), |d| d.child(html!("div", {
                                    .class(class::BADGE_WRAP_R_RED)
                                    .style("cursor","help")
                                    .text("แพ้ยา/เฝ้าระวัง")
                                    .attr("title", &order_item.allergy_agent_symptom.clone().unwrap_or(String::from("ไม่ระบุอาการ")))
                                })))
                                // HAD/LASA badge
                                .children(app.drug_alert_badge(order_item.displaycolor))
                                // Med Reconcile badge
                                .apply_if(order_item.med_reconciliation_item_id.is_some(), |d| d.child(html!("div", {
                                    .style("cursor","help")
                                    .apply(|dd| {
                                        match &order_item.used {
                                            Some(used) => {
                                                match used.as_str() {
                                                    "N" => dd.class(class::BADGE_WRAP_R_GRAY),
                                                    "H" => dd.class(class::BADGE_WRAP_R_CYAN),
                                                    "Y" => {
                                                        if order_item.is_med_rec_change_usage() {
                                                            dd.class(class::BADGE_WRAP_R_GOLD)
                                                        } else {
                                                            dd.class(class::BADGE_WRAP_R_GREEN)
                                                        }
                                                    }
                                                    _ => dd,
                                                }
                                            }
                                            None => dd,
                                        }

                                    })
                                    .attr("title", &order_item.med_rec_info())
                                    .text("MR")
                                })))
                                .apply_if(order_item.icode.is_some(), |d| d.child(html!("br")))
                                .text(&order_item.order_item_detail.clone().unwrap_or_default())
                                .apply_if(order_item.off_by_datetime.is_some(), |d| d.style("text-decoration","line-through"))
                            }))
                            // STAT badge
                            .apply_if(order_item.stat == Some(String::from("Y")), |dom| dom.child(html!("span", {
                                .class(class::BADGE_WRAP_R_RED)
                                .style("cursor","default")
                                .text("STAT")
                            })))
                            // OFF badge
                            .apply_if(order_item.off_by_datetime.is_some(), |dom| dom.child(html!("span", {
                                .class(class::BADGE_WRAP_R_GOLD)
                                .style("cursor","default")
                                .text(&["OFF ", &datetime_th_opt(&order_item.off_by_datetime)].concat())
                            })))
                            // Monitor box
                            .apply_if(order_item.monitor_status.as_ref().map(|monitor_status| monitor_status == "Y").unwrap_or_default(), |dom| dom
                                .child(html!("div", {
                                    .class(class::BORDER_SMALL_BG_RED)
                                    .style("white-space","pre-wrap")
                                    .children(doms::square_bracket_to_span(&order_item.monitor.clone().unwrap_or_default()))
                                }))
                            )
                        }
                    })
                }))
            })
        ])
    })
}

#[derive(Clone, Default, PartialEq)]
pub enum FormType {
    #[default]
    Plan,
    Action,
    Monitor,
}

#[derive(Clone, Default, PartialEq)]
pub enum PlanTab {
    #[default]
    Time,
    Date,
    Stat,
}

impl PlanTab {
    pub fn from_sch_type(sch_type: &Option<String>) -> Self {
        sch_type
            .as_ref()
            .map(|sch| match sch.as_str() {
                "stat" => Self::Stat,
                "date" => Self::Date,
                _ => Self::Time,
            })
            .unwrap_or_default()
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Date => "date",
            Self::Time => "time",
            Self::Stat => "stat",
        }
    }
}

#[derive(Clone, Default)]
pub enum OrderType {
    #[default]
    OneDay,
    Continuous,
}

impl OrderType {
    pub fn new_from_str(text: &str) -> Self {
        match text {
            "continuous" => Self::Continuous,
            _ => Self::OneDay,
        }
    }
}

fn first_24hr_plan_buttons_iter(
    start_hr: u8,
    order_hr: u8,
    first_24hr: Vec<u8>,
    order_date_opt: Option<Date>,
    time_plans: &[Rc<IndexPlan>],
    plan_add_datetimes: Mutable<HashSet<PrimitiveDateTime>>,
) -> impl Iterator<Item = Dom> {
    first_24hr.into_iter().map(move |h| {
        let btn_date = if h < start_hr { order_date_opt.and_then(|d| d.next_day()) } else { order_date_opt }.unwrap_or(js_now().date());
        let plan_count = time_plans
            .iter()
            .filter(|plan| plan.plan_date == Some(btn_date) && plan.plan_time.map(|t| t.hour() == h).unwrap_or_default())
            .count();

        html!("button", {
            .attr("type", "button")
            .class(class::BTN_LB2)
            .class("position-relative")
            .apply(|dom| {
                if let Some(badge) = doms::badge_count_blue(plan_count) {
                    dom.class("btn-secondary")
                        // .style("pointer-events","none")
                        .child(badge)
                } else {
                    dom.class("btn-outline-primary")
                }
            })
            .class_signal("btn-info", plan_add_datetimes.signal_cloned().map(move |dts| dts.iter().any(|dt| {
                btn_date == dt.date() && dt.hour() == h
            })))
            .style("min-width","45px")
            .attr("title", &date_th(&btn_date))
            .apply_if(h == order_hr && h >= start_hr, |dom| dom
                .style("color","red")
                .style("border-color","red")
            )
            .text(&h.to_string())
            .event(clone!(plan_add_datetimes => move |_: events::Click| {
                if plan_count == 0 {
                    let hr = if h == 24 {0} else {h};
                    let input_time = Time::from_hms(hr, 0, 0).unwrap();
                    let plan_datetime = PrimitiveDateTime::new(btn_date, input_time);
                    {
                        let mut lock = plan_add_datetimes.lock_mut();
                        if lock.contains(&plan_datetime) {
                            lock.remove(&plan_datetime);
                        } else {
                            lock.insert(plan_datetime);
                        }
                    }
                }
            }))
        })
    })
}

fn nurse_assign_order_item(order_item: Rc<OrderItem>, nurse_assign_mutable: Mutable<String>, app: Rc<App>) {
    // `without_order` cannot `nurse-assign`
    if order_item.order_item_id != 0 {
        app.async_load(
            true,
            clone!(app => async move {
                let save = OrderItemPatch {
                    order_item_id: order_item.order_item_id,
                    action: OrderItemPatchAction::NurseAssign,
                    nurse_assign: str_some(nurse_assign_mutable.get_cloned()),
                    order_item_type: None,
                    due_doctor: None,
                    due_doctor_note: None,
                    due_pharm: None,
                    due_pharm_note: None,
                };
                // PATCH `EndPoint::IpdOrderItem`
                // PATCH `EndPoint::OpdErOrderItem`
                match save.call_api_patch(order_item.visit_type.is_ipd(), app.state()).await {
                    Ok(response) => {
                        if let Some(error) = &response.error {
                            app.alert_error_with_clipboard(CONTACT_ADMIN, &["ExecuteResponse: ", error].concat()).await;
                            nurse_assign_mutable.set(order_item.nurse_assign.clone().unwrap_or_default())
                        } else if response.rows_affected == 0 {
                            app.alert("ไม่มีการเปลี่ยนแปลง", "หากพบปัญหา กรุณาติดต่อผู้ดูแลระบบ");
                            nurse_assign_mutable.set(order_item.nurse_assign.clone().unwrap_or_default())
                        }
                    }
                    Err(e) => {
                        app.alert_app_error(&e).await;
                        nurse_assign_mutable.set(order_item.nurse_assign.clone().unwrap_or_default())
                    }
                }
            }),
        );
    }
}

fn is_injection_order_item(order_item: Rc<OrderItem>, is_injection_mutable: Mutable<Option<bool>>, app: Rc<App>) {
    if order_item.order_item_id != 0 {
        let is_injection = order_item.order_item_type.as_ref().and_then(|ty| match ty.as_str() {
            "med" => Some(false),
            "injection" => Some(true),
            _ => None,
        });
        app.async_load(
            true,
            clone!(app => async move {
                let save = OrderItemPatch {
                    order_item_id: order_item.order_item_id,
                    action: OrderItemPatchAction::OrderItemType,
                    nurse_assign: None,
                    order_item_type: is_injection_mutable.get().map(|is_inj| {
                        if is_inj {String::from("injection")} else {String::from("med")}
                    }),
                    due_doctor: None,
                    due_doctor_note: None,
                    due_pharm: None,
                    due_pharm_note: None,
                };
                // PATCH `EndPoint::IpdOrderItem`
                // PATCH `EndPoint::OpdErOrderItem`
                match save.call_api_patch(order_item.visit_type.is_ipd(), app.state()).await {
                    Ok(response) => {
                        if let Some(error) = &response.error {
                            app.alert_error_with_clipboard(CONTACT_ADMIN, &["ExecuteResponse: ", error].concat()).await;
                            is_injection_mutable.set(is_injection)
                        } else if response.rows_affected == 0 {
                            app.alert("ไม่มีการเปลี่ยนแปลง", "หากพบปัญหา กรุณาติดต่อผู้ดูแลระบบ");
                            is_injection_mutable.set(is_injection)
                        }
                    }
                    Err(e) => {
                        app.alert_app_error(&e).await;
                        is_injection_mutable.set(is_injection)
                    }
                }
            }),
        );
    }
}
