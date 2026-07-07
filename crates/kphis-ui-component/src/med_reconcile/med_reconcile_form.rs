use dominator::{Dom, clone, events, html, with_node};
use futures_signals::{
    map_ref,
    signal::{Mutable, Signal, SignalExt, always},
    signal_vec::{MutableVec, SignalVecExt},
};
use rust_decimal::Decimal;
use std::rc::Rc;
use time::Date;
use web_sys::{HtmlButtonElement, HtmlInputElement, HtmlTextAreaElement};

use kphis_model::{
    app::VisitTypeId,
    endpoint::EndPoint,
    fetch::Method,
    med_reconcile::{MedReconciliation, MedReconciliationItem, MedReconciliationItemPatch, MedReconciliationNote, MedReconciliationParams},
    order::{Order, OrderItem, OrderItemType, OrderTypeName},
    tab::Tab,
    user::permission::Permission,
};
use kphis_ui_app::App;
use kphis_ui_core::{class, doms, mixins};
use kphis_util::{
    datetime::{JsTime, date_8601, datetime_8601, datetime_str_th, js_now},
    error::CONTACT_ADMIN,
    util::{str_some, zero_none},
};

/// - GET `EndPoint::IpdMedReconcileNoteId` (guared, remove cancel btn)
/// - GET `EndPoint::OpdErMedReconcileNoteId` (guared, remove cancel btn)
/// - POST `EndPoint::IpdMedReconcileNoteId` (guared, remove 'Add Note' and edit btn)
/// - POST `EndPoint::OpdErMedReconcileNoteId` (guared, remove 'Add Note' and edit btn)
/// - PATCH `EndPoint::IpdMedReconcile` (guarded, remove ยืนยันรายการ, ยกเลิกการยืนยันรายการ, บันทึกผลการพิจารณา, บันทึกเฉพาะ Last Dose.. btns)
/// - PATCH `EndPoint::OpdErMedReconcile` (guarded, remove ยืนยันรายการ, ยกเลิกการยืนยันรายการ, บันทึกผลการพิจารณา, บันทึกเฉพาะ Last Dose.. btns)
/// - DELETE `EndPoint::IpdMedReconcile` (guarded, remove ลบรายการ, delete item btns)
/// - DELETE `EndPoint::OpdErMedReconcile` (guarded, remove ลบรายการ, delete item btns)
#[derive(Default)]
pub struct MedReconForm {
    visit_type: VisitTypeId,
    // from parent
    view_by: Mutable<String>,
    redraw: Mutable<bool>,
    dchdate: Mutable<Option<Date>>,
    active_tab: Mutable<Tab>,
    loaded_med_reconciliation_count_spinner: Mutable<bool>,
    loaded_med_reconciliation_has_data: Mutable<bool>,
    recons_remove_pending: Mutable<Option<u32>>,

    pharm_changed: Mutable<bool>,
    use_changed: Mutable<bool>,
    last_changed: Mutable<bool>,

    pub med_reconciliation_id: Mutable<u32>,
    // pharmacist: Mutable<String>,
    note: Mutable<String>,
    note_prev: Mutable<String>,
    // doctor: Mutable<String>,
    med_reconciliation_datetime: Mutable<String>, // datetime
    pharmacist_confirm_datetime: Mutable<String>, // datetime
    doctor_confirm_datetime: Mutable<String>,     // datetime
    pharmacist_name: Mutable<String>,
    doctor_name: Mutable<String>,
    is_pharmacist_current_user_doctor: Mutable<String>,

    med_reconciliation_items: MutableVec<Rc<MedReconItem>>,

    // note modal
    modal_note_changed: Mutable<bool>,
    // modal_note: Mutable<String>,
}

impl MedReconForm {
    pub fn new(
        view_by: Mutable<String>,
        dchdate: Mutable<Option<Date>>,
        active_tab: Mutable<Tab>,
        loaded_med_reconciliation_count_spinner: Mutable<bool>,
        loaded_med_reconciliation_has_data: Mutable<bool>,
        recons_remove_pending: Mutable<Option<u32>>,
        item: MedReconciliation,
    ) -> Self {
        Self {
            visit_type: item.visit_type.clone(),
            view_by,
            dchdate,
            active_tab,
            loaded_med_reconciliation_count_spinner,
            loaded_med_reconciliation_has_data,
            recons_remove_pending,
            redraw: Mutable::new(true),
            med_reconciliation_id: Mutable::new(item.med_reconciliation_id),
            // pharmacist: Mutable::new(item.pharmacist.unwrap_or_default()),
            note: Mutable::new(item.note.unwrap_or_default()),
            // doctor: Mutable::new(item.doctor.unwrap_or_default()),
            med_reconciliation_datetime: Mutable::new(item.med_reconciliation_datetime.map(|dt| dt.js_string()).unwrap_or_default()), // datetime
            pharmacist_confirm_datetime: Mutable::new(item.phamacist_confirm_datetime.map(|dt| dt.js_string()).unwrap_or_default()),  // datetime
            doctor_confirm_datetime: Mutable::new(item.doctor_confirm_datetime.map(|dt| dt.js_string()).unwrap_or_default()),         // datetime
            pharmacist_name: Mutable::new(item.pharmacist_name.unwrap_or_default()),
            doctor_name: Mutable::new(item.doctor_name.unwrap_or_default()),
            is_pharmacist_current_user_doctor: Mutable::new(item.is_pharmacist_current_user_doctor),

            med_reconciliation_items: MutableVec::new_with_values(item.med_reconciliation_items.into_iter().map(|mri| Rc::new(MedReconItem::from(mri))).collect()),
            ..Default::default()
        }
    }

    fn can_pharm_edit(&self) -> impl Signal<Item = bool> + use<> {
        map_ref! {
            let phamacist_confirm_datetime = self.pharmacist_confirm_datetime.signal_cloned(),
            let doctor_confirm_datetime = self.doctor_confirm_datetime.signal_cloned(),
            let is_pharm_current = self.is_pharmacist_current_user_doctor.signal_cloned() =>
            is_pharm_current == "Y" && phamacist_confirm_datetime.is_empty() && doctor_confirm_datetime.is_empty()
        }
    }

    fn allow_delete(&self, app: &Rc<App>) -> bool {
        let (is_ipd, is_pre_admit) = self.visit_type.is_ipd_and_is_pre_admit();
        if is_ipd {
            app.endpoint_is_allow(&Method::DELETE, &EndPoint::IpdMedReconcile, is_pre_admit)
        } else {
            app.endpoint_is_allow(&Method::DELETE, &EndPoint::OpdErMedReconcile, false)
        }
    }

    fn can_remove(&self, app: &Rc<App>) -> impl Signal<Item = bool> + use<> {
        map_ref! {
            let phamacist_confirm_datetime = self.pharmacist_confirm_datetime.signal_cloned(),
            let doctor_confirm_datetime = self.doctor_confirm_datetime.signal_cloned(),
            let has_remove_permision = always(self.allow_delete(app)) =>
            *has_remove_permision && phamacist_confirm_datetime.is_empty() && doctor_confirm_datetime.is_empty()
        }
    }

    fn can_consider(&self, app: &Rc<App>) -> impl Signal<Item = bool> + use<> {
        map_ref! {
            let phamacist_confirm_datetime = self.pharmacist_confirm_datetime.signal_cloned(),
            // let doctor_confirm_datetime = self.doctor_confirm_datetime.signal_cloned(),
            let has_consider_permision = always(app.has_permission(Permission::MedReconciliationConsider)),
            let view_by = self.view_by.signal_cloned() =>
            view_by == "doctor" && *has_consider_permision && !phamacist_confirm_datetime.is_empty() // && doctor_confirm_datetime.is_empty()
        }
    }

    fn can_add_cont_order(&self, app: &Rc<App>) -> impl Signal<Item = bool> + use<> {
        map_ref! {
            let doctor_confirm_datetime = self.doctor_confirm_datetime.signal_cloned(),
            let view_by = self.view_by.signal_cloned(),
            let dchdate = self.dchdate.signal_cloned(),
            let has_consider_permission = always(app.has_permission(Permission::MedReconciliationConsider)) =>
            *has_consider_permission && view_by == "doctor" &&
            dchdate.map(|d| d <= js_now().date()).unwrap_or(true) &&
            !doctor_confirm_datetime.is_empty()
        }
    }

    fn can_add_hold_items(&self, app: &Rc<App>) -> impl Signal<Item = bool> + use<> {
        map_ref! {
            let can_add = self.can_add_cont_order(&app),
            let no_h = self.med_reconciliation_items.signal_vec_cloned().filter_signal_cloned(|item| item.used.signal_cloned().map(|used| used.as_str() == "H")).is_empty() =>
            *can_add && !no_h
        }
    }

    fn can_add_used_items(&self, app: &Rc<App>) -> impl Signal<Item = bool> + use<> {
        map_ref! {
            let can_add = self.can_add_cont_order(&app),
            let no_y = self.med_reconciliation_items.signal_vec_cloned().filter_signal_cloned(|item| item.used.signal_cloned().map(|used| used.as_str() == "Y")).is_empty() =>
            *can_add && !no_y
        }
    }

    // fn is_doctor_confirmed(&self) -> impl Signal<Item = bool> + use<> {
    //     self.doctor_confirm_datetime.signal_cloned().map(|dt| !dt.is_empty())
    // }

    fn is_confirmed(&self) -> impl Signal<Item = bool> + use<> {
        map_ref! {
            let phamacist_confirm_datetime = self.pharmacist_confirm_datetime.signal_cloned(),
            let doctor_confirm_datetime = self.doctor_confirm_datetime.signal_cloned() =>
            !phamacist_confirm_datetime.is_empty() || !doctor_confirm_datetime.is_empty()
        }
    }

    fn is_confirmed_and_editable(&self) -> impl Signal<Item = bool> + use<> {
        map_ref! {
            let phamacist_confirm_datetime = self.pharmacist_confirm_datetime.signal_cloned(),
            let doctor_confirm_datetime = self.doctor_confirm_datetime.signal_cloned(),
            let view_by = self.view_by.signal_cloned() =>
            !phamacist_confirm_datetime.is_empty() &&
            ((view_by == "doctor" && doctor_confirm_datetime.is_empty()) || !doctor_confirm_datetime.is_empty())
        }
    }

    // ipd-dr-med-reconcile-doctor-confirm.php + med_reconciliation_id
    // recon SET doctor=doctor_code, doctor_confirm_datetime=NOW()
    // item SET use,changed_drugusage,last_dose_taken_time,last_dose_taken_remark
    // => page.doctor_name.set_neq(app.doctor_name());
    // => page.doctor_confirm_datetime.set(js_now().js_string());
    // => page.use_changed.set(false);
    // => page.last_changed.set(false);
    fn doctor_confirm_recon(page: Rc<Self>, app: Rc<App>) {
        app.async_load(
            true,
            clone!(app, page => async move {
                if app.confirm("ยืนยันรายการโดยแพทย์").await {
                    let med_reconciliation_id = page.med_reconciliation_id.get();
                    if med_reconciliation_id > 0 {
                        let params = MedReconciliationParams {
                            med_reconciliation_id: Some(med_reconciliation_id),
                            patch: Some(String::from("doctor")),
                            ..Default::default()
                        };
                        if Self::patch_recon(true, &params, page.clone(), app.clone()).await {
                            page.doctor_name.set_neq(app.doctor_name().clone().unwrap_or_default());
                            page.doctor_confirm_datetime.set(js_now().js_string());
                            page.use_changed.set(false);
                            page.last_changed.set(false);
                        }
                    }
                }
            }),
        );
    }

    // ipd-dr-med-reconcile-pharmacist-confirm.php + med_reconciliation_id
    // recon SET phamacist_confirm_datetime=NOW()
    // item SET last_dose_taken_time, last_dose_taken_remark
    // => page.pharmacist_confirm_datetime.set(js_now().js_string());
    // => page.last_changed.set(false);
    fn pharm_confirm_recon(page: Rc<Self>, app: Rc<App>) {
        app.async_load(
            true,
            clone!(app, page => async move {
                if app.confirm("ยืนยันรายการ").await {
                    let med_reconciliation_id = page.med_reconciliation_id.get();
                    if med_reconciliation_id > 0 {
                        let params = MedReconciliationParams {
                            med_reconciliation_id: Some(med_reconciliation_id),
                            patch: Some(String::from("pharm")),
                            ..Default::default()
                        };
                        if Self::patch_recon(true, &params, page.clone(), app).await {
                            page.pharmacist_confirm_datetime.set(js_now().js_string());
                            page.last_changed.set(false);
                            page.redraw.set(true);
                        }
                    }
                }
            }),
        );
    }

    // ipd-dr-med-reconcile-pharmacist-unconfirm.php + med_reconciliation_id
    // recon SET phamacist_confirm_datetime=null
    // item SET none
    // => page.pharmacist_confirm_datetime.set(String::new());
    fn pharm_unconfirm_recon(page: Rc<Self>, app: Rc<App>) {
        app.async_load(
            true,
            clone!(app, page => async move {
                if app.confirm("ยกเลิกการยืนยันรายการ").await {
                    let med_reconciliation_id = page.med_reconciliation_id.get();
                    if med_reconciliation_id > 0 {
                        let params = MedReconciliationParams {
                            med_reconciliation_id: Some(med_reconciliation_id),
                            patch: Some(String::from("unconfirm")),
                            ..Default::default()
                        };
                        if Self::patch_recon(false, &params, page.clone(), app).await {
                            page.pharmacist_confirm_datetime.set(String::new());
                            page.redraw.set(true);
                        }
                    }
                }
            }),
        );
    }

    fn receive_recon(page: Rc<Self>, app: Rc<App>) {
        app.async_load(
            true,
            clone!(app, page => async move {
                let med_reconciliation_id = page.med_reconciliation_id.get();
                if med_reconciliation_id > 0 {
                    let params = MedReconciliationParams {
                        med_reconciliation_id: Some(med_reconciliation_id),
                        patch: Some(String::from("receive")),
                        ..Default::default()
                    };
                    if Self::patch_recon(true, &params, page.clone(), app).await {
                        page.pharm_changed.set(false);
                    }
                }
            }),
        );
    }

    // ipd-dr-med-reconcile-last-dose-save.php + med_reconciliation_id
    // recon SET none
    // item SET last_dose_taken_time,last_dose_taken_remark
    // => page.last_changed.set(false);
    fn last_dose_recon(page: Rc<Self>, app: Rc<App>) {
        app.async_load(
            true,
            clone!(app, page => async move {
                let med_reconciliation_id = page.med_reconciliation_id.get();
                if med_reconciliation_id > 0 {
                    let params = MedReconciliationParams {
                        med_reconciliation_id: Some(med_reconciliation_id),
                        patch: Some(String::from("last")),
                        ..Default::default()
                    };
                    if Self::patch_recon(true, &params, page.clone(), app).await {
                        page.last_changed.set(false);
                    }
                }
            }),
        );
    }

    async fn patch_recon(with_items: bool, params: &MedReconciliationParams, page: Rc<Self>, app: Rc<App>) -> bool {
        let items = if with_items {
            page.med_reconciliation_items.lock_ref().iter().map(new_patch_item).collect::<Vec<MedReconciliationItemPatch>>()
        } else {
            Vec::new()
        };
        // PATCH `EndPoint::IpdMedReconcile`
        // PATCH `EndPoint::OpdErMedReconcile`
        match MedReconciliation::call_api_patch(page.visit_type.is_ipd(), &items, params, app.state()).await {
            Ok(responses) => {
                if let Some(res) = responses.iter().find(|res| res.error.is_some()) {
                    app.alert_error_with_clipboard(CONTACT_ADMIN, &["ExecuteResponse: ", &res.error.clone().unwrap_or_default()].concat())
                        .await;
                    false
                } else {
                    // app.alert("บันทึกข้อมูลเรียบร้อย", &["จำนวน", &responses.iter().map(|r| r.rows_affected).sum::<u64>().to_string(), " รายการ"].concat());
                    page.loaded_med_reconciliation_count_spinner.set_neq(false);
                    true
                }
            }
            Err(e) => {
                app.alert_app_error(&e).await;
                false
            }
        }
    }

    fn insert_home_med_order(used: &str, page: Rc<Self>, app: Rc<App>) {
        let order_items = page
            .med_reconciliation_items
            .lock_ref()
            .iter()
            .filter(|&item| item.used.lock_ref().as_str() == used)
            .map(|item| OrderItem {
                visit_type: page.visit_type.clone(),
                order_item_type: Some(String::from("home-medication")),
                order_item_detail: str_some(item.changed_drugusage.get_cloned()).or(str_some(item.old_drugusage.get_cloned())),
                icode: str_some(item.icode.get_cloned()),
                med_name: str_some(item.med_name.get_cloned()).or(str_some(item.custom_med_name.get_cloned())),
                generic_name: str_some(item.generic_name.get_cloned()),
                dosageform: str_some(item.dosageform.get_cloned()),
                allergy_agent_symptom: str_some(item.allergy_agent.get_cloned()).map(|agent| [&agent, item.allergy_agent_symptom.lock_ref().as_str()].join("=")),
                due_usage: str_some(item.due_usage.get_cloned()),
                due_status: str_some(item.due_status.get_cloned()),
                info: str_some(item.info.get_cloned()),
                info_status: str_some(item.info_status.get_cloned()),

                med_reconciliation_item_id: zero_none(item.med_reconciliation_item_id.get()),
                old_drugusage: str_some(item.old_drugusage.get_cloned()),
                receive_from: str_some(item.receive_from.get_cloned()),
                receive_date: date_8601(&item.receive_date.lock_ref()),
                receive_qty: item.receive_qty.lock_ref().parse::<i32>().ok(),
                last_dose_taken_time: datetime_8601(&item.last_dose_taken_time.lock_ref()),
                last_dose_taken_remark: str_some(item.last_dose_taken_remark.get_cloned()),
                ..Default::default()
            })
            .collect();
        let now = js_now();
        let edit_order = Order {
            visit_type: page.visit_type.clone(),
            order_id: 0,
            hn: None,
            fullname: None,
            ward_name: None,
            bedno: None,
            display_bedno: None,
            bed_type_name: None,
            bed_type_color: None,
            order_date: now.date(),
            order_time: now.time(),
            order_doctor: app.doctor_code().unwrap_or_default(),
            order_type: String::from("oneday"),
            order_owner_type: page.view_by.get_cloned(),
            order_confirm: String::from("N"),
            nurse_order_as: None,
            doctor_confirm_time: None,
            nurse_accept: None,
            nurse_accept_time: None,
            pharmacist_accept: None,
            pharmacist_accept_time: None,
            pharmacist_check: None,
            pharmacist_check_time: None,
            pharmacist_done: None,
            pharmacist_done_time: None,
            pharmacist_order_status: None,
            pre_order_id: None,
            pre_order_date: None,
            pre_order_time: None,
            order_doctor_name: None,
            order_doctor_licenseno: None,
            order_doctor_entryposition: None,
            order_doctor_is_intern: None,
            nurse_order_as_name: None,
            nurse_order_as_entryposition: None,
            nurse_order_as_licenseno: None,
            nurse_order_as_is_intern: None,
            nurse_accept_name: None,
            nurse_accept_licenseno: None,
            nurse_accept_entryposition: None,
            pharmacist_accept_name: None,
            pharmacist_accept_entryposition: None,
            pharmacist_accept_licenseno: None,
            pharmacist_check_name: None,
            pharmacist_check_entryposition: None,
            pharmacist_check_licenseno: None,
            pharmacist_done_name: None,
            pharmacist_done_entryposition: None,
            pharmacist_done_licenseno: None,
            order_item_types: vec![OrderItemType {
                order_item_type: OrderTypeName::HomeMedication,
                order_items,
            }],
        };
        app.edit_order.set(Some(Rc::new(edit_order)));
        page.active_tab.set(Tab::Order);
    }

    fn insert_cont_order(used: &str, page: Rc<Self>, app: Rc<App>) {
        let injections = app.app_status.lock_ref().as_ref().map(|status| status.hosxp_injection_dosageforms.clone()).unwrap_or_default();
        let (injection_order_items, med_order_items): (Vec<OrderItem>, Vec<OrderItem>) = page
            .med_reconciliation_items
            .lock_ref()
            .iter()
            .filter(|&item| item.used.lock_ref().as_str() == used)
            .map(|item| OrderItem {
                visit_type: page.visit_type.clone(),
                order_item_type: if injections.contains(&item.dosageform.lock_ref()) {
                    Some(String::from("injection"))
                } else {
                    Some(String::from("med"))
                },
                order_item_detail: str_some(item.changed_drugusage.get_cloned()).or(str_some(item.old_drugusage.get_cloned())),
                icode: str_some(item.icode.get_cloned()),
                med_name: str_some(item.med_name.get_cloned()).or(str_some(item.custom_med_name.get_cloned())),
                generic_name: str_some(item.generic_name.get_cloned()),
                dosageform: str_some(item.dosageform.get_cloned()),
                due_usage: str_some(item.due_usage.get_cloned()),
                due_status: str_some(item.due_status.get_cloned()),
                info: str_some(item.info.get_cloned()),
                info_status: str_some(item.info_status.get_cloned()),

                allergy_agent_symptom: str_some(item.allergy_agent.get_cloned()).map(|agent| [&agent, item.allergy_agent_symptom.lock_ref().as_str()].join("=")),

                med_reconciliation_item_id: zero_none(item.med_reconciliation_item_id.get()),
                old_drugusage: str_some(item.old_drugusage.get_cloned()),
                receive_from: str_some(item.receive_from.get_cloned()),
                receive_date: date_8601(&item.receive_date.lock_ref()),
                receive_qty: item.receive_qty.lock_ref().parse::<i32>().ok(),
                last_dose_taken_time: datetime_8601(&item.last_dose_taken_time.lock_ref()),
                last_dose_taken_remark: str_some(item.last_dose_taken_remark.get_cloned()),
                ..Default::default()
            })
            .partition(|item| item.order_item_type.as_ref().map(|order_item_type| order_item_type == "injection").unwrap_or_default());
        let now = js_now();
        let edit_order = Order {
            visit_type: page.visit_type.clone(),
            order_id: 0,
            hn: None,
            fullname: None,
            ward_name: None,
            bedno: None,
            display_bedno: None,
            bed_type_name: None,
            bed_type_color: None,
            order_date: now.date(),
            order_time: now.time(),
            order_doctor: app.doctor_code().unwrap_or_default(),
            order_type: String::from("continuous"),
            order_owner_type: page.view_by.get_cloned(),
            order_confirm: String::from("N"),
            nurse_order_as: None,
            doctor_confirm_time: None,
            nurse_accept: None,
            nurse_accept_time: None,
            pharmacist_accept: None,
            pharmacist_accept_time: None,
            pharmacist_check: None,
            pharmacist_check_time: None,
            pharmacist_done: None,
            pharmacist_done_time: None,
            pharmacist_order_status: None,
            pre_order_id: None,
            pre_order_date: None,
            pre_order_time: None,
            order_doctor_name: None,
            order_doctor_licenseno: None,
            order_doctor_entryposition: None,
            order_doctor_is_intern: None,
            nurse_order_as_name: None,
            nurse_order_as_entryposition: None,
            nurse_order_as_licenseno: None,
            nurse_order_as_is_intern: None,
            nurse_accept_name: None,
            nurse_accept_entryposition: None,
            nurse_accept_licenseno: None,
            pharmacist_accept_name: None,
            pharmacist_accept_entryposition: None,
            pharmacist_accept_licenseno: None,
            pharmacist_check_name: None,
            pharmacist_check_entryposition: None,
            pharmacist_check_licenseno: None,
            pharmacist_done_name: None,
            pharmacist_done_entryposition: None,
            pharmacist_done_licenseno: None,
            order_item_types: vec![
                OrderItemType {
                    order_item_type: OrderTypeName::Injection,
                    order_items: injection_order_items,
                },
                OrderItemType {
                    order_item_type: OrderTypeName::Med,
                    order_items: med_order_items,
                },
            ],
        };
        app.edit_order.set(Some(Rc::new(edit_order)));
        page.active_tab.set(Tab::Order);
    }

    fn delete_recon(page: Rc<Self>, app: Rc<App>) {
        if let Some(med_reconciliation_id) = zero_none(page.med_reconciliation_id.get()) {
            app.async_load(
                true,
                clone!(app, page => async move {
                    if med_reconciliation_id > 0 && app.confirm("ยืนยันลบรายการ").await {
                        let params = MedReconciliationParams {
                            med_reconciliation_id: Some(med_reconciliation_id),
                            ..Default::default()
                        };
                        // DELETE `EndPoint::IpdMedReconcile`
                        // DELETE `EndPoint::OpdErMedReconcile`
                        match MedReconciliation::call_api_delete(page.visit_type.is_ipd(), &params, app.state()).await {
                            Ok(response) => {
                                app.alert_execute_response(&response, async move {
                                    page.loaded_med_reconciliation_count_spinner.set_neq(false);
                                    page.loaded_med_reconciliation_has_data.set_neq(false);
                                    page.recons_remove_pending.set(Some(med_reconciliation_id));
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

    fn delete_recon_item(med_reconciliation_item_id: u32, page: Rc<Self>, app: Rc<App>) {
        app.async_load(
            true,
            clone!(app, page => async move {
                if med_reconciliation_item_id > 0 && app.confirm("ยืนยันลบรายการ").await {
                    let params = MedReconciliationParams {
                        med_reconciliation_item_id: Some(med_reconciliation_item_id),
                        ..Default::default()
                    };
                    // DELETE `EndPoint::IpdMedReconcile`
                    // DELETE `EndPoint::OpdErMedReconcile`
                    match MedReconciliation::call_api_delete(page.visit_type.is_ipd(), &params, app.state()).await {
                        Ok(response) => {
                            app.alert_execute_response(&response, async move {
                                page.med_reconciliation_items.lock_mut().retain(|item| {
                                    item.med_reconciliation_item_id.get() != med_reconciliation_item_id
                                });
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
        let (is_ipd, is_pre_admit) = page.visit_type.is_ipd_and_is_pre_admit();
        let can_patch = if is_ipd {
            app.endpoint_is_allow(&Method::PATCH, &EndPoint::IpdMedReconcile, is_pre_admit)
        } else {
            app.endpoint_is_allow(&Method::PATCH, &EndPoint::OpdErMedReconcile, false)
        };

        html!("div", {
            .class(class::CARD)
            .children([
                html!("div", {
                    .child(Self::render_note_modal(page.clone(), app.clone()))
                }),
                html!("div", {
                    .class("card-body")
                    .child(html!("div", {
                        .children([
                            html!("span", {
                                .class("ipd_med_reconciliation_time")
                                .text_signal(map_ref!{
                                    let pharm_cdt = page.pharmacist_confirm_datetime.signal_cloned(),
                                    let recon_dt = page.med_reconciliation_datetime.signal_cloned() =>
                                    if !pharm_cdt.is_empty() {
                                        datetime_str_th(pharm_cdt)
                                    } else if !recon_dt.is_empty() {
                                        datetime_str_th(recon_dt)
                                    } else {
                                        String::new()
                                    }
                                })
                            }),
                        ])
                    }))
                    .child_signal(page.med_reconciliation_items.signal_vec_cloned().is_empty().map(clone!(app, page => move |is_empty| {
                        if is_empty {
                            Some(html!("div", {
                                .class(class::BORDER_ROUND)
                                .text("ไม่มียาเดิม")
                            }))
                        } else {
                            Some(doms::table_responsive(class::TABLE_STRIP, clone!(app, page => move |table| { table
                                .children([
                                    html!("thead", {
                                        .child(html!("tr", {
                                            .children([
                                                html!("th", {.attr("scope", "col")
                                                    .text("#")
                                                }),
                                                html!("th", {.attr("scope", "col").class("text-nowrap")
                                                    .text("ชื่อยา")
                                                }),
                                                html!("th", {.attr("scope", "col").class("text-nowrap").style("min-width", "150px")
                                                    .text("วิธีใช้")
                                                }),
                                                html!("th", {.attr("scope", "col").class("text-nowrap")
                                                    .text("จำนวน")
                                                }),
                                                html!("th", {.attr("scope", "col").class("text-nowrap")
                                                    .text("วันที่ได้รับ")
                                                }),
                                                html!("th", {.attr("scope", "col").class("text-nowrap")
                                                    .text("สถานพยาบาล")
                                                }),
                                                html!("th", {.attr("scope", "col").class(class::TABLE_BG_CYAN)
                                                    .visible_signal(page.is_confirmed())
                                                    .text("สั่งใช้")
                                                }),
                                                html!("th", {.attr("scope", "col").class(class::TABLE_BG_CYAN)
                                                    .visible_signal(page.is_confirmed_and_editable())
                                                    .text("เปลี่ยนวิธีใช้")
                                                }),
                                                html!("th", {.attr("scope", "col").class(class::TABLE_BG_CYAN)
                                                    .visible_signal(page.is_confirmed())
                                                    .text("ไม่สั่งใช้")
                                                }),
                                                html!("th", {.attr("scope", "col").class(class::TABLE_BG_CYAN)
                                                    .visible_signal(page.is_confirmed())
                                                    .text("Hold")
                                                }),
                                                html!("th", {.attr("scope", "col").class("text-nowrap")
                                                    .text("Last Dose")
                                                }),
                                                html!("th", {.attr("scope", "col").class("text-nowrap")
                                                    .text("จำนวนยาเหลือ / หมายเหตุ")
                                                }),
                                                html!("th", {.attr("scope", "col").class("text-center")
                                                    .visible_signal(page.can_remove(&app))
                                                    .child(html!("i", {.class(class::FA_TRASH)}))
                                                }),
                                            ])
                                        }))
                                    }),
                                    html!("tbody", {
                                        .children_signal_vec(page.med_reconciliation_items.signal_vec_cloned().enumerate().map(clone!(app, page => move |(i, item)| {
                                            let item_id = item.med_reconciliation_item_id.get().to_string();
                                            html!("tr", {
                                                .children([
                                                    html!("td", {.text(&(i.get().unwrap_or_default() + 1).to_string()).class("text-center")}),
                                                    html!("td", {
                                                        .style("white-space","pre-wrap")
                                                        .text(&item.med_name.lock_ref())
                                                        .apply_if(!item.med_name.lock_ref().is_empty() && !item.custom_med_name.lock_ref().is_empty(), |dom| dom.child(html!("br")))
                                                        .text(&item.custom_med_name.lock_ref())
                                                        .apply_if(!item.allergy_agent.lock_ref().is_empty(), |dom| dom
                                                            .child(html!("br"))
                                                            .child(html!("span", {
                                                                .class("text-danger")
                                                                .text(&["แพ้ยา: ", &item.allergy_agent.lock_ref().as_str(), "=", &item.allergy_agent_symptom.lock_ref().as_str()].concat())
                                                            }))
                                                        )
                                                    }),
                                                    html!("td", {
                                                        .child_signal(page.can_pharm_edit().map(clone!(page, item => move |can_edit| {
                                                            if can_edit {
                                                                Some(html!("textarea" => HtmlTextAreaElement, {
                                                                    .attr("maxlength", "255")
                                                                    .class(class::FORM_CTRL_SM)
                                                                    // .apply(mixins::other_true_signal_disable(not(page.can_pharm_edit())))
                                                                    .apply(mixins::textarea_value_auto_expand(item.old_drugusage.clone(), page.pharm_changed.clone()))
                                                                }))
                                                            } else {
                                                                Some(html!("div", {
                                                                    .text(&item.old_drugusage.lock_ref())
                                                                }))
                                                            }
                                                        })))
                                                    }),
                                                    html!("td", {
                                                        .style("width","80px")
                                                        .child(html!("input" => HtmlInputElement, {
                                                            .attr("type", "number")
                                                            .class(class::FORM_CTRL_SM)
                                                            .apply(mixins::string_value(item.receive_qty.clone(), page.pharm_changed.clone()))
                                                        }))
                                                    }),
                                                    html!("td", {
                                                        .style("width","130px")
                                                        .child(doms::date_picker(
                                                            item.receive_date.clone(),
                                                            page.pharm_changed.clone(), always(false), None,
                                                            |d| d.class(class::FLEX_GROW1).style("min-width","120px"),
                                                            |d| d.class(class::FORM_CTRL_ONLY_SM_R0_L),
                                                            |d| d.class(class::FORM_CTRL_ONLY_SM_R0_L),
                                                            |s| s, always(None),
                                                        ))
                                                    }),
                                                    html!("td", {
                                                        .child(html!("input" => HtmlInputElement, {
                                                            .attr("type", "text")
                                                            .class(class::FORM_CTRL_SM)
                                                            // .apply(mixins::other_true_signal_disable(not(page.can_pharm_edit())))
                                                            .apply(mixins::string_value(item.receive_from.clone(), page.pharm_changed.clone()))
                                                        }))
                                                    }),
                                                    html!("td", {
                                                        .visible_signal(page.is_confirmed())
                                                        .class(class::BG_CYAN_10)
                                                        .child(html!("div", {
                                                            .class(class::FORM_CHK_INL)
                                                            .children([
                                                                html!("input" => HtmlInputElement, {
                                                                    .attr("type", "radio")
                                                                    .attr("id", &["med-reconciliation-item-use-y-", &item_id].concat())
                                                                    .class("form-check-input")
                                                                    .apply(mixins::radio_match(item.used.clone(), page.use_changed.clone(), "Y"))
                                                                    .with_node!(element => {
                                                                        .future(map_ref!{
                                                                            let can_consider = page.can_consider(&app),
                                                                            // let is_doctor_confirmed = page.is_doctor_confirmed(),
                                                                            let allergy_agent = item.allergy_agent.signal_cloned(),
                                                                            let allergy_count = item.allergy_count_force_no_order.signal_cloned(),
                                                                            let used = item.used.signal_cloned() =>
                                                                            !(*can_consider && used != "Y" && allergy_agent.is_empty() && allergy_count.is_zero())
                                                                            // !((*can_consider || (*is_doctor_confirmed && used == "Y")) && allergy_agent.is_empty() && allergy_count.is_zero())
                                                                        }.for_each(move |disabled| {
                                                                            element.set_disabled(disabled);
                                                                            async {}
                                                                        }))
                                                                    })
                                                                }),
                                                                html!("label", {
                                                                    .class(class::FORM_CHK_LBL_NOWRAP)
                                                                    .attr("for", &["med-reconciliation-item-use-y-", &item_id].concat())
                                                                    .style("user-select","none")
                                                                    .text("สั่งใช้")
                                                                })
                                                            ])
                                                        }))
                                                    }),
                                                    html!("td", {
                                                        .visible_signal(page.is_confirmed_and_editable())
                                                        .class(class::BG_CYAN_10)
                                                        .child_signal(page.can_consider(&app).map(clone!(page, item => move |can| {
                                                            if can {
                                                                Some(html!("textarea" => HtmlTextAreaElement, {
                                                                    .attr("placeholder","ปรับเปลี่ยนวิธีใช้ (ระบุ)")
                                                                    .class(class::FORM_CTRL_SM)
                                                                    .apply(mixins::textarea_value_auto_expand(item.changed_drugusage.clone(), page.use_changed.clone()))
                                                                    .with_node!(element => {
                                                                        .future(item.used.signal_cloned().for_each(move |used| {
                                                                            element.set_disabled(used != "Y");
                                                                            async {}
                                                                        }))
                                                                    })
                                                                }))
                                                            } else {
                                                                Some(html!("div", {
                                                                    .style("white-space","pre-wrap")
                                                                    .text(&item.changed_drugusage.lock_ref())
                                                                }))
                                                            }
                                                        })))
                                                    }),
                                                    html!("td", {
                                                        .visible_signal(page.is_confirmed())
                                                        .class(class::BG_CYAN_10)
                                                        .child(html!("div", {
                                                            .class(class::FORM_CHK_INL)
                                                            .children([
                                                                html!("input" => HtmlInputElement, {
                                                                    .attr("type", "radio")
                                                                    .attr("id", &["med-reconciliation-item-use-n-", &item_id].concat())
                                                                    .class("form-check-input")
                                                                    .apply(mixins::radio_match(item.used.clone(), page.use_changed.clone(), "N"))
                                                                    .with_node!(element => {
                                                                        .future(map_ref!{
                                                                            let can_consider = page.can_consider(&app),
                                                                            // let is_doctor_confirmed = page.is_doctor_confirmed(),
                                                                            let used = item.used.signal_cloned() =>
                                                                            !(*can_consider && used != "N")
                                                                        }.for_each(move |disabled| {
                                                                            element.set_disabled(disabled);
                                                                            async {}
                                                                        }))
                                                                    })
                                                                }),
                                                                html!("label", {
                                                                    .class(class::FORM_CHK_LBL_NOWRAP)
                                                                    .attr("for", &["med-reconciliation-item-use-n-", &item_id].concat())
                                                                    .style("user-select","none")
                                                                    .text("ไม่สั่งใช้")
                                                                })
                                                            ])
                                                        }))
                                                    }),
                                                    html!("td", {
                                                        .visible_signal(page.is_confirmed())
                                                        .class(class::BG_CYAN_10)
                                                        .child(html!("div", {
                                                            .class(class::FORM_CHK_INL)
                                                            .children([
                                                                html!("input" => HtmlInputElement, {
                                                                    .attr("type", "radio")
                                                                    .attr("id", &["med-reconciliation-item-use-hold-", &item_id].concat())
                                                                    .class("form-check-input")
                                                                    .apply(mixins::radio_match(item.used.clone(), page.use_changed.clone(), "H"))
                                                                    .with_node!(element => {
                                                                        .future(map_ref!{
                                                                            let can_consider = page.can_consider(&app),
                                                                            // let is_doctor_confirmed = page.is_doctor_confirmed(),
                                                                            let used = item.used.signal_cloned() =>
                                                                            !(*can_consider && used != "H")
                                                                        }.for_each(move |disabled| {
                                                                            element.set_disabled(disabled);
                                                                            async {}
                                                                        }))
                                                                    })
                                                                }),
                                                                html!("label", {
                                                                    .class(class::FORM_CHK_LBL_NOWRAP)
                                                                    .attr("for", &["med-reconciliation-item-use-hold-", &item_id].concat())
                                                                    .style("user-select","none")
                                                                    .text("Hold")
                                                                })
                                                            ])
                                                        }))
                                                    }),
                                                    html!("td", {
                                                        .style("width","210px")
                                                        .style("min-width","210px")
                                                        .child(doms::datetime_picker(
                                                            item.last_dose_taken_time.clone(),
                                                            page.last_changed.clone(), always(false),
                                                            |d| d.class(class::FLEX_GROW1).style("min-width","175px"),
                                                            |d| d.class(class::FORM_CTRL_ONLY_SM_R0_L),
                                                            |d| d.class(class::FORM_CTRL_ONLY_SM_R0_L),
                                                            |s| s, always(None),
                                                        ))
                                                    }),
                                                    html!("td", {
                                                        .style("width","210px")
                                                        .child(html!("textarea" => HtmlTextAreaElement, {
                                                            .attr("maxlength", "255")
                                                            .class(class::FORM_CTRL_SM)
                                                            .apply(mixins::textarea_value_auto_expand(item.last_dose_taken_remark.clone(), page.last_changed.clone()))
                                                        }))
                                                    }),
                                                    html!("td", {
                                                        .visible_signal(page.can_remove(&app))
                                                        .class("text-center")
                                                        .child(html!("button" => HtmlButtonElement, {
                                                            .attr("type", "button")
                                                            .class(class::BTN_SM_RED)
                                                            .child(html!("i", {.class(class::FA_TRASH)}))
                                                            .apply(mixins::click_with_loader_checked(clone!(app, page, item => move || {
                                                                let med_reconciliation_item_id = item.med_reconciliation_item_id.get();
                                                                Self::delete_recon_item(med_reconciliation_item_id, page.clone(), app.clone());
                                                            }), app.state()))
                                                        }))
                                                    }),
                                                ])
                                            })
                                        })))
                                    }),
                                ])
                            })))
                        }
                    })))
                    .apply_if(if is_ipd {
                        app.endpoint_is_allow(&Method::POST, &EndPoint::IpdMedReconcileNoteId, is_pre_admit)
                    } else {
                        app.endpoint_is_allow(&Method::POST, &EndPoint::OpdErMedReconcileNoteId, false)
                    }, |dom| dom
                        .child_signal(map_ref!{
                            let note = page.note.signal_cloned(),
                            let phamacist_confirm_datetime = page.pharmacist_confirm_datetime.signal_cloned(),
                            let doctor_confirm_datetime = page.doctor_confirm_datetime.signal_cloned() =>
                            note.is_empty() && phamacist_confirm_datetime.is_empty() && doctor_confirm_datetime.is_empty()
                        }.map(clone!(page => move |can| {
                            can.then(|| {
                                html!("div", {
                                    .class(class::ROW)
                                    .child(html!("div", {
                                        .class("col")
                                        .child(html!("button", {
                                            .attr("type", "button")
                                            .class(class::BTN_SM_GRAY)
                                            .class(class::FLOAT_RT)
                                            .attr("data-bs-toggle", "modal")
                                            .attr("data-bs-target", &["#medReconciliationNoteFormModal", &page.med_reconciliation_id.get().to_string()].concat())
                                            .child(html!("i", {.class(class::FA_PLUS)}))
                                            .text(" Add Note")
                                        }))
                                    }))
                                })
                            })
                        })))
                    )
                    .child_signal(page.note.signal_cloned().map(clone!(app, page => move |note| {
                        (!note.is_empty()).then(|| {
                            html!("div", {
                                .class(class::CARD)
                                .children([
                                    html!("div", {
                                        .class(class::CARD_HEAD)
                                        .child(html!("span", {.text("Note")}))
                                        .apply_if(if is_ipd {
                                            app.endpoint_is_allow(&Method::POST, &EndPoint::IpdMedReconcileNoteId, is_pre_admit)
                                        } else {
                                            app.endpoint_is_allow(&Method::POST, &EndPoint::OpdErMedReconcileNoteId, false)
                                        }, |dom| dom
                                            .child_signal(map_ref!{
                                                let phamacist_confirm_datetime = page.pharmacist_confirm_datetime.signal_cloned(),
                                                let doctor_confirm_datetime = page.doctor_confirm_datetime.signal_cloned() =>
                                                phamacist_confirm_datetime.is_empty() && doctor_confirm_datetime.is_empty()
                                            }.map(clone!(page => move |can| {
                                                can.then(|| {
                                                    html!("button", {
                                                        .attr("type", "button")
                                                        .class(class::BTN_SM_FR_GRAY)
                                                        .attr("data-bs-toggle", "modal")
                                                        .attr("data-bs-target", &["#medReconciliationNoteFormModal", &page.med_reconciliation_id.get().to_string()].concat())
                                                        .child(html!("i", {.class(class::FA_EDIT)}))
                                                        .text(" Edit Note")
                                                    })
                                                })
                                            })))
                                        )
                                    }),
                                    html!("div", {
                                        .class(class::CARD_BODY_CYANS)
                                        .style("white-space","pre-wrap")
                                        .text(&note)
                                    }),
                                ])
                            })
                        })
                    })))
                    .child(html!("div", {
                        .class("med-reconciliation-form")
                        .child(html!("div", {
                            .class("row")
                            .children([
                                html!("div", {
                                    .class("col")
                                    .child(html!("div", {
                                        .class(class::INPUT_GROUP_T)
                                        .children([
                                            doms::label_group_for("pharmacist_name","ผู้บันทึกรายการ"),
                                            html!("input", {
                                                .attr("type", "text")
                                                .class("form-control")
                                                .attr("id", "pharmacist_name")
                                                .attr("readonly", "readonly")
                                                .prop_signal("value", page.pharmacist_name.signal_cloned())
                                            }),
                                        ])
                                    }))
                                }),
                                html!("div", {
                                    .class("col")
                                    .child(html!("div", {
                                        .class(class::INPUT_GROUP_T)
                                        .children([
                                            doms::label_group_for("doctor_name","แพทย์ผู้พิจารณา"),
                                            html!("input", {
                                                .attr("type", "text")
                                                .class("form-control")
                                                .attr("id", "doctor_name")
                                                .attr("readonly", "readonly")
                                                .prop_signal("value", page.doctor_name.signal_cloned())
                                            }),
                                        ])
                                    }))
                                }),
                            ])
                        }))
                    }))
                }),
                html!("div", {
                    .class("card-footer")
                    .apply_if(page.allow_delete(&app), |dom| { dom
                        .child_signal(map_ref!{
                            let is_no_item = page.med_reconciliation_items.signal_vec_cloned().is_empty(),
                            let phamacist_confirm_datetime = page.pharmacist_confirm_datetime.signal_cloned() =>
                            *is_no_item && phamacist_confirm_datetime.is_empty()
                        }.map(clone!(app, page => move |can| {
                            can.then(|| {
                                html!("button" => HtmlButtonElement, {
                                    .attr("type", "button")
                                    .class(class::BTN_RED)
                                    .class(class::FLOAT_RR_Y1)
                                    .child(html!("i", {.class(class::FA_TRASH)}))
                                    .text(" ลบรายการ")
                                    .apply(mixins::click_with_loader_checked(clone!(app, page => move || {
                                        Self::delete_recon(page.clone(), app.clone());
                                    }), app.state()))
                                })
                            })
                        })))
                    })
                    .apply_if(can_patch && app.has_permission(Permission::MedReconciliationComfirm), |dom| { dom
                        .child_signal(page.can_pharm_edit().map(clone!(app, page => move |can| {
                            can.then(|| {
                                html!("button" => HtmlButtonElement, {
                                    .attr("type", "button")
                                    .class(class::BTN_BLUE)
                                    .class(class::FLOAT_RR_Y1)
                                    .text("ยืนยันรายการ")
                                    .apply(mixins::click_with_loader_checked(clone!(app, page => move || {
                                        Self::pharm_confirm_recon(page.clone(), app.clone());
                                    }), app.state()))
                                })
                            })
                        })))
                    })
                    .apply_if(can_patch, |dom| { dom
                        .child_signal(map_ref!{
                            let is_no_item = page.med_reconciliation_items.signal_vec_cloned().is_empty(),
                            let phamacist_confirm_datetime = page.pharmacist_confirm_datetime.signal_cloned(),
                            let doctor_confirm_datetime = page.doctor_confirm_datetime.signal_cloned(),
                            let is_pharm_current = page.is_pharmacist_current_user_doctor.signal_cloned() =>
                            !is_no_item && doctor_confirm_datetime.is_empty() && !phamacist_confirm_datetime.is_empty() && is_pharm_current == "Y"
                        }.map(clone!(app, page => move |can| {
                            can.then(|| {
                                html!("button" => HtmlButtonElement, {
                                    .attr("type", "button")
                                    .class(class::BTN_GRAY)
                                    .class(class::FLOAT_RR_Y1)
                                    .child(html!("i", {.class(class::FA_EDIT)}))
                                    .text(" ยกเลิกการยืนยันรายการ")
                                    .apply(mixins::click_with_loader_checked(clone!(app, page => move || {
                                        Self::pharm_unconfirm_recon(page.clone(), app.clone());
                                    }), app.state()))
                                })
                            })
                        })))
                    })
                    .apply_if(can_patch && app.has_permission(Permission::MedReconciliationConsider), |dom| { dom
                        .child_signal(map_ref!{
                            let phamacist_confirm_datetime = page.pharmacist_confirm_datetime.signal_cloned(),
                            let doctor_confirm_datetime = page.doctor_confirm_datetime.signal_cloned(),
                            let view_by = page.view_by.signal_cloned(),
                            let is_no_unused = page.med_reconciliation_items.signal_vec_cloned().filter_signal_cloned(|item| {
                                item.used.signal_cloned().map(|s| s.is_empty())
                            }).is_empty(),
                            let use_changed = page.use_changed.signal() =>
                            view_by == "doctor" && !phamacist_confirm_datetime.is_empty() && *is_no_unused && (doctor_confirm_datetime.is_empty() || *use_changed)
                        }.map(clone!(app, page => move |can| {
                            can.then(|| {
                                html!("button" => HtmlButtonElement, {
                                    .attr("type", "button")
                                    .class(class::BTN_BLUE)
                                    .class(class::FLOAT_RR_Y1)
                                    .text("บันทึกผลการพิจารณา")
                                    .apply(mixins::click_with_loader_checked(clone!(app, page => move || {
                                        Self::doctor_confirm_recon(page.clone(), app.clone());
                                    }), app.state()))
                                })
                            })
                        })))
                    })
                    .child_signal(page.can_add_hold_items(&app).map(clone!(app, page => move |can| {
                        can.then(|| {
                            html!("button", {
                                .attr("type", "button")
                                .class(class::BTN_GOLD)
                                .class(class::FLOAT_RR_Y1)
                                .text("เพิ่มรายการที่ Hold ไว้ลงใน Home-Med")
                                .event(clone!(app, page => move |_:events::Click| {
                                    Self::insert_home_med_order("H", page.clone(), app.clone());
                                }))
                            })
                        })
                    })))
                    .child_signal(page.can_add_used_items(&app).map(clone!(app, page => move |can| {
                        can.then(|| {
                            html!("button", {
                                .attr("type", "button")
                                .class(class::BTN_CYAN)
                                .class(class::FLOAT_RR_Y1)
                                .text("เพิ่มรายการที่สั่งใช้ลงใน Home-Med")
                                .event(clone!(app, page => move |_:events::Click| {
                                    Self::insert_home_med_order("Y", page.clone(), app.clone());
                                }))
                            })
                        })
                    })))
                    .child_signal(page.can_add_hold_items(&app).map(clone!(app, page => move |can| {
                        can.then(|| {
                            html!("button", {
                                .attr("type", "button")
                                .class(class::BTN_GOLD)
                                .class(class::FLOAT_RR_Y1)
                                .text("เพิ่มรายการที่ Hold ไว้ลงใน Order")
                                .event(clone!(app, page => move |_:events::Click| {
                                    Self::insert_cont_order("H", page.clone(), app.clone());
                                }))
                            })
                        })
                    })))
                    .child_signal(page.can_add_used_items(&app).map(clone!(app, page => move |can| {
                        can.then(|| {
                            html!("button", {
                                .attr("type", "button")
                                .class(class::BTN_CYAN)
                                .class(class::FLOAT_RR_Y1)
                                .text("เพิ่มรายการที่สั่งใช้ลงใน Order")
                                .event(clone!(app, page => move |_:events::Click| {
                                    Self::insert_cont_order("Y", page.clone(), app.clone());
                                }))
                            })
                        })
                    })))
                    .apply_if(can_patch, |dom| { dom
                        .child_signal(map_ref!{
                            let pharmacist_confirm_datetime = page.pharmacist_confirm_datetime.signal_cloned(),
                            let last_changed = page.last_changed.signal() =>
                            !pharmacist_confirm_datetime.is_empty() && *last_changed
                        }.map(clone!(app, page => move |can| {
                            can.then(|| {
                                html!("button" => HtmlButtonElement, {
                                    .attr("type", "button")
                                    .class(class::BTN_GRAY)
                                    .class(class::FLOAT_RR_Y1)
                                    .text("บันทึกเฉพาะ Last Dose และ จำนวนยาเหลือ / หมายเหตุ")
                                    .apply(mixins::click_with_loader_checked(clone!(app, page => move || {
                                        Self::last_dose_recon(page.clone(), app.clone());
                                    }), app.state()))
                                })
                            })
                        })))
                        .child_signal(map_ref!{
                            let pharmacist_confirm_datetime = page.pharmacist_confirm_datetime.signal_cloned(),
                            let pharm_changed = page.pharm_changed.signal() =>
                            !pharmacist_confirm_datetime.is_empty() && *pharm_changed
                        }.map(clone!(app, page => move |can| {
                            can.then(|| {
                                html!("button" => HtmlButtonElement, {
                                    .attr("type", "button")
                                    .class(class::BTN_GRAY)
                                    .class(class::FLOAT_RR_Y1)
                                    .text("บันทึกเฉพาะ จำนวน, วันที่ได้รับ และ สถานพยาบาล")
                                    .apply(mixins::click_with_loader_checked(clone!(app, page => move || {
                                        Self::receive_recon(page.clone(), app.clone());
                                    }), app.state()))
                                })
                            })
                        })))
                    })
                }),
            ])
        })
    }

    fn load(page: Rc<Self>, app: Rc<App>) {
        let med_reconciliation_id = page.med_reconciliation_id.get();
        if med_reconciliation_id > 0 {
            app.async_load(
                true,
                clone!(app => async move {
                    let result = if page.visit_type.is_ipd() {
                        // GET `EndPoint::IpdMedReconcileNoteId`
                        MedReconciliationNote::call_api_get_ipd(med_reconciliation_id, app.state()).await
                    } else {
                        // GET `EndPoint::OpdErMedReconcileNoteId`
                        MedReconciliationNote::call_api_get_opd_er(med_reconciliation_id, app.state()).await
                    };
                    match result {
                        Ok(Some(response)) => {
                            let note = response.note.unwrap_or_default();
                            page.note.set_neq(note.clone());
                            page.note_prev.set_neq(note);
                            page.pharmacist_confirm_datetime.set_neq(response.phamacist_confirm_datetime.map(|dt| dt.js_string()).unwrap_or_default());
                            page.modal_note_changed.set_neq(false);
                        }
                        Ok(None) => {
                            page.note.set_neq(page.note_prev.get_cloned());
                        }
                        Err(e) => {
                            app.alert_app_error(&e).await;
                            page.note.set_neq(page.note_prev.get_cloned());
                        }
                    }
                }),
            )
        }
    }

    fn save_note(page: Rc<Self>, app: Rc<App>) {
        if let Some(med_reconciliation_id) = zero_none(page.med_reconciliation_id.get()) {
            app.async_load(
                true,
                clone!(app => async move {
                    let result = if page.visit_type.is_ipd() {
                        // POST `EndPoint::IpdMedReconcileNoteId`
                        MedReconciliationNote::call_api_post_ipd(&page.note.lock_ref(), med_reconciliation_id, app.state()).await
                    } else {
                        // POST `EndPoint::OpdErMedReconcileNoteId`
                        MedReconciliationNote::call_api_post_opd_er(&page.note.lock_ref(), med_reconciliation_id, app.state()).await
                    };
                    match result {
                        Ok(response) => {
                            if let Some(error) = &response.error {
                                app.alert_error_with_clipboard(CONTACT_ADMIN, &["ExecuteResponse: ", error].concat()).await;
                                page.note.set_neq(page.note_prev.get_cloned());
                            } else {
                                page.note_prev.set_neq(page.note.get_cloned());
                                page.modal_note_changed.set_neq(false);
                            }
                        }
                        Err(e) => {
                            app.alert_app_error(&e).await;
                            page.note.set_neq(page.note_prev.get_cloned());
                        }
                    }
                }),
            )
        }
    }

    fn render_note_modal(page: Rc<Self>, app: Rc<App>) -> Dom {
        let (is_ipd, is_pre_admit) = page.visit_type.is_ipd_and_is_pre_admit();

        html!("div", {
            .class("modal")
            .attr("id", &["medReconciliationNoteFormModal", &page.med_reconciliation_id.get().to_string()].concat())
            .attr("role", "dialog")
            .child(html!("div", {
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
                            //.attr("id", "med_reconciliation_note_form_modal_body")
                            .child(html!("div", {
                                //.attr("id", "med-reconciliation-note-form")
                                .child(html!("div", {
                                    .class("mb-3")
                                    .child(html!("textarea" => HtmlTextAreaElement, {
                                        .attr("type", "text")
                                        .class("form-control")
                                        .apply(mixins::textarea_value_auto_expand(page.note.clone(), page.modal_note_changed.clone()))
                                    }))
                                }))
                            }))
                        }),
                        html!("div", {
                            .class("modal-footer")
                            .child(html!("button" => HtmlButtonElement, {
                                .attr("type", "button")
                                .class(class::BTN_BLUE)
                                .attr("data-bs-dismiss", "modal")
                                .child(html!("i", {.class(class::FA_SAVE)}))
                                .text(" Save")
                                .apply(mixins::click_with_loader_checked(clone!(app, page => move || {
                                    Self::save_note(page.clone(), app.clone());
                                }), app.state()))
                            }))
                            .apply_if(if is_ipd {
                                app.endpoint_is_allow(&Method::GET, &EndPoint::IpdMedReconcileNoteId, is_pre_admit)
                            } else {
                                app.endpoint_is_allow(&Method::GET, &EndPoint::OpdErMedReconcileNoteId, false)
                            }, |dom| { dom
                                .child(html!("button" => HtmlButtonElement, {
                                    .attr("type", "button")
                                    .class(class::BTN_GRAY)
                                    .attr("data-bs-dismiss", "modal")
                                    .child(html!("i", {.class(class::FA_X)}))
                                    .text(" Cancel")
                                    .apply(mixins::click_with_loader_checked(clone!(app, page => move || {
                                        Self::load(page.clone(), app.clone());
                                    }), app.state()))
                                }))
                            })
                        }),
                    ])
                }))
            }))
        })
    }
}

#[derive(Default)]
struct MedReconItem {
    med_reconciliation_item_id: Mutable<u32>,
    // med_reconciliation_id: Mutable<u32>,
    // an: Mutable<String>,
    icode: Mutable<String>,
    med_name: Mutable<String>,
    custom_med_name: Mutable<String>,
    receive_from: Mutable<String>,
    receive_date: Mutable<String>, // date
    old_drugusage: Mutable<String>,
    changed_drugusage: Mutable<String>,
    receive_qty: Mutable<String>,          // i32
    last_dose_taken_time: Mutable<String>, // datetime
    last_dose_taken_remark: Mutable<String>,
    used: Mutable<String>, // use
    allergy_agent: Mutable<String>,
    allergy_agent_symptom: Mutable<String>,
    allergy_count_force_no_order: Mutable<Decimal>, // decimal
    // common_name: Mutable<String>,
    generic_name: Mutable<String>,
    dosageform: Mutable<String>,
    // show_notify: Mutable<String>,
    // show_notify_text: Mutable<String>,
    due_usage: Mutable<String>,
    due_status: Mutable<String>,
    info: Mutable<String>,
    info_status: Mutable<String>,
}

impl From<MedReconciliationItem> for MedReconItem {
    fn from(item: MedReconciliationItem) -> Self {
        Self {
            med_reconciliation_item_id: Mutable::new(item.med_reconciliation_item_id),
            // med_reconciliation_id: Mutable::new(item.med_reconciliation_id.unwrap_or_default()),
            // an: Mutable::new(item.an.unwrap_or_default()),
            icode: Mutable::new(item.icode.unwrap_or_default()),
            med_name: Mutable::new(item.med_name.unwrap_or_default()),
            custom_med_name: Mutable::new(item.custom_med_name.unwrap_or_default()),
            receive_from: Mutable::new(item.receive_from.unwrap_or_default()),
            receive_date: Mutable::new(item.receive_date.map(|d| d.to_string()).unwrap_or_default()), // date
            old_drugusage: Mutable::new(item.old_drugusage.unwrap_or_default()),
            changed_drugusage: Mutable::new(item.changed_drugusage.unwrap_or_default()),
            receive_qty: Mutable::new(item.receive_qty.map(|i| i.to_string()).unwrap_or_default()), // i32
            last_dose_taken_time: Mutable::new(item.last_dose_taken_time.map(|dt| dt.js_string()).unwrap_or_default()), // datetime
            last_dose_taken_remark: Mutable::new(item.last_dose_taken_remark.unwrap_or_default()),
            used: Mutable::new(item.used.unwrap_or_default()), // use
            allergy_agent: Mutable::new(item.allergy_agent.unwrap_or_default()),
            allergy_agent_symptom: Mutable::new(item.allergy_agent_symptom.unwrap_or_default()),
            allergy_count_force_no_order: Mutable::new(item.allergy_count_force_no_order), // decimal
            // common_name: Mutable::new(item.common_name.unwrap_or_default()),
            generic_name: Mutable::new(item.generic_name.unwrap_or_default()),
            dosageform: Mutable::new(item.dosageform.unwrap_or_default()),
            // show_notify: Mutable::new(item.show_notify.unwrap_or_default()),
            // show_notify_text: Mutable::new(item.show_notify_text.unwrap_or_default()),
            due_usage: Mutable::new(item.due_usage.unwrap_or_default()),
            due_status: Mutable::new(item.due_status.unwrap_or_default()),
            info: Mutable::new(item.info.unwrap_or_default()),
            info_status: Mutable::new(item.info_status.unwrap_or_default()),
        }
    }
}

// TODO convert to From trait
fn new_patch_item(item: &Rc<MedReconItem>) -> MedReconciliationItemPatch {
    MedReconciliationItemPatch {
        med_reconciliation_item_id: item.med_reconciliation_item_id.get(),

        old_drugusage: str_some(item.old_drugusage.get_cloned()),
        receive_qty: item.receive_qty.lock_ref().parse::<i32>().ok(),
        receive_from: str_some(item.receive_from.get_cloned()),
        receive_date: date_8601(&item.receive_date.lock_ref()),

        changed_drugusage: str_some(item.changed_drugusage.get_cloned()),
        last_dose_taken_time: datetime_8601(&item.last_dose_taken_time.lock_ref()),
        last_dose_taken_remark: str_some(item.last_dose_taken_remark.get_cloned()),
        used: str_some(item.used.get_cloned()),
    }
}
