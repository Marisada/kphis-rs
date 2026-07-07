use dominator::{Dom, clone, events, html, with_node};
use futures_signals::{
    map_ref,
    signal::{Mutable, Signal, SignalExt, always, not},
    signal_vec::{MutableVec, SignalVecExt},
};
use std::rc::Rc;
use web_sys::{HtmlButtonElement, HtmlInputElement, HtmlSelectElement, HtmlTextAreaElement};

use kphis_model::{
    app::VisitTypeId,
    dc_plan::{DischargePlan, DischargePlanParams, DischargePlanSave},
    endpoint::EndPoint,
    fetch::Method,
    ipd::dc_plan_tmp::{DcPlanTmpDiet, DcPlanTmpDx, DcPlanTmpEnv, DcPlanTmpMed, DcPlanTmpParams, DcPlanTmpTx},
    patient_info::PatientInfo,
    report::{SystemReport, TypstReport},
    user::permission::Permission,
};
use kphis_ui_app::App;
use kphis_ui_core::{binding::NiceSelect, class, doms, mixins};
use kphis_util::{
    datetime::{JsTime, date_8601, datetime_8601, datetime_from_opt, js_now, time_8601},
    util::{str_some, zero_none},
};

use crate::gadget::pdf_button::PdfButtons;

/// - GET `EndPoint::IpdDcPlanAn`
/// - GET `EndPoint::OpdErDcPlanId`
/// - GET `EndPoint::IpdDcPlanTmpDx`
/// - GET `EndPoint::IpdDcPlanTmpMed`
/// - GET `EndPoint::IpdDcPlanTmpEnv`
/// - GET `EndPoint::IpdDcPlanTmpTx`
/// - GET `EndPoint::IpdDcPlanTmpDiet`
/// - POST `EndPoint::IpdDcPlanAn` (guarded, remove 'บันทึกข้อมูล' btn)
/// - POST `EndPoint::OpdErDcPlanId` (guarded, remove 'บันทึกข้อมูล' btn)
/// - DELETE `EndPoint::IpdDcPlanAn` (guarded, remove 'ลบ' btn)
/// - DELETE `EndPoint::OpdErDcPlanId` (guarded, remove 'ลบ' btn)
#[derive(Clone, Default)]
pub struct DcPlanCpn {
    patient: Mutable<Option<Rc<PatientInfo>>>,
    loaded: Mutable<bool>,
    redraw_dx_selector: Mutable<bool>,
    all_dxs: MutableVec<Rc<DcPlanTmpDx>>,
    all_meds: MutableVec<Rc<DcPlanTmpMed>>,
    all_envs: MutableVec<Rc<DcPlanTmpEnv>>,
    all_txs: MutableVec<Rc<DcPlanTmpTx>>,
    all_diets: MutableVec<Rc<DcPlanTmpDiet>>,

    loaded_dc_plans: Mutable<bool>,
    checked: Mutable<bool>,
    dc_plans: MutableVec<Rc<DischargePlan>>,
    dc_plan_selected: Mutable<Option<Rc<DischargePlan>>>,

    changed: Mutable<bool>,
    dc_plan_id: Mutable<u32>,
    dx_id: Mutable<String>,
    dc_datetime: Mutable<String>,
    dc_type_ok: Mutable<String>,
    dc_type_refer: Mutable<String>,
    dc_type_other: Mutable<String>,
    dc_symptom: Mutable<String>,
    inst_none: Mutable<String>,
    inst_foley: Mutable<String>,
    inst_ett: Mutable<String>,
    inst_tt: Mutable<String>,
    inst_ng: Mutable<String>,
    inst_other: Mutable<String>,
    with_drug: Mutable<String>,
    with_appoint: Mutable<String>,
    with_cert: Mutable<String>,
    with_other: Mutable<String>,
    appoint_date: Mutable<String>,
    appoint_time: Mutable<String>,
    appoint_place: Mutable<String>,
    appoint_for: Mutable<String>,
    refer_to: Mutable<String>,
    dx_text: Mutable<String>,
    dx_patient_ok: Mutable<String>,
    dx_relatives_ok: Mutable<String>,
    dx_other: Mutable<String>,
    dx_doctor: Mutable<String>,
    dx_datetime: Mutable<String>,
    med_text: Mutable<String>,
    med_patient_ok: Mutable<String>,
    med_relatives_ok: Mutable<String>,
    med_other: Mutable<String>,
    med_doctor: Mutable<String>,
    med_datetime: Mutable<String>,
    env_text: Mutable<String>,
    env_patient_ok: Mutable<String>,
    env_relatives_ok: Mutable<String>,
    env_other: Mutable<String>,
    env_doctor: Mutable<String>,
    env_datetime: Mutable<String>,
    tx_text: Mutable<String>,
    tx_patient_ok: Mutable<String>,
    tx_relatives_ok: Mutable<String>,
    tx_other: Mutable<String>,
    tx_doctor: Mutable<String>,
    tx_datetime: Mutable<String>,
    health_text: Mutable<String>,
    health_patient_ok: Mutable<String>,
    health_relatives_ok: Mutable<String>,
    health_other: Mutable<String>,
    health_doctor: Mutable<String>,
    health_datetime: Mutable<String>,
    out_text: Mutable<String>,
    out_patient_ok: Mutable<String>,
    out_relatives_ok: Mutable<String>,
    out_other: Mutable<String>,
    out_doctor: Mutable<String>,
    out_datetime: Mutable<String>,
    diet_text: Mutable<String>,
    diet_patient_ok: Mutable<String>,
    diet_relatives_ok: Mutable<String>,
    diet_other: Mutable<String>,
    diet_doctor: Mutable<String>,
    diet_datetime: Mutable<String>,
    version: Mutable<i32>,

    dx_name: Mutable<String>,
    dx_knowledge: Mutable<String>,
    dx_revisit: Mutable<String>,
    dx_prevention: Mutable<String>,
    med_ids: MutableVec<u32>,
    env_ids: MutableVec<u32>,
    tx_ids: MutableVec<u32>,
    diet_ids: MutableVec<u32>,
    dx_user_name: Mutable<String>,
    dx_entryposition: Mutable<String>,
    med_user_name: Mutable<String>,
    med_entryposition: Mutable<String>,
    env_user_name: Mutable<String>,
    env_entryposition: Mutable<String>,
    tx_user_name: Mutable<String>,
    tx_entryposition: Mutable<String>,
    health_user_name: Mutable<String>,
    health_entryposition: Mutable<String>,
    out_user_name: Mutable<String>,
    out_entryposition: Mutable<String>,
    diet_user_name: Mutable<String>,
    diet_entryposition: Mutable<String>,
}

impl DcPlanCpn {
    pub fn new(patient: Mutable<Option<Rc<PatientInfo>>>) -> Rc<Self> {
        Rc::new(Self { patient, ..Default::default() })
    }

    fn is_ipd_and_is_pre_admit(&self) -> impl Signal<Item = (bool, bool)> + use<> {
        self.patient
            .signal_cloned()
            .map(|opt| opt.as_ref().map(|pt| pt.visit_type.is_ipd_and_is_pre_admit()).unwrap_or_default())
    }

    pub fn load_dc_plans(page: Rc<Self>, app: Rc<App>) {
        let visit_type_opt = page.patient.lock_ref().as_ref().map(|pt| pt.visit_type());
        if let Some(visit_type) = visit_type_opt {
            page.dc_plans.lock_mut().clear();
            app.async_load(
                true,
                clone!(app, page => async move {
                    let result_opt = match visit_type {
                        VisitTypeId::Ipd(an) | VisitTypeId::PreAdmit(an) => {
                            // GET `EndPoint::IpdDcPlanAn`
                            Some(DischargePlan::call_api_get_ipd(&an, app.state()).await)
                        }
                        VisitTypeId::OpdEr(_, opd_er_order_master_id) => {
                            // GET `EndPoint::OpdErDcPlanId`
                            Some(DischargePlan::call_api_get_opd_er(opd_er_order_master_id, app.state()).await)
                        }
                        VisitTypeId::Visit(_) => None,
                    };
                    if let Some(result) = result_opt {
                        match result {
                            Ok(dc_plans) => {
                                page.checked.set(!dc_plans.is_empty());
                                page.dc_plans.lock_mut().extend(dc_plans.into_iter().map(Rc::new));
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

    fn load_all_tmp(page: Rc<Self>, app: Rc<App>) {
        app.async_load(
            true,
            clone!(app => async move {
                // GET `EndPoint::IpdDcPlanTmpDx`
                match DcPlanTmpDx::call_api_get(&DcPlanTmpParams::default(), app.state()).await {
                    Ok(responses) => {
                        let mut lock = page.all_dxs.lock_mut();
                        lock.extend(responses.into_iter().map(Rc::new));
                    }
                    Err(e) => {
                        app.alert_app_error(&e).await;
                    }
                }
                // GET `EndPoint::IpdDcPlanTmpMed`
                match DcPlanTmpMed::call_api_get(&DcPlanTmpParams::default(), app.state()).await {
                    Ok(responses) => {
                        let mut lock = page.all_meds.lock_mut();
                        lock.extend(responses.into_iter().map(Rc::new));
                    }
                    Err(e) => {
                        app.alert_app_error(&e).await;
                    }
                }
                // GET `EndPoint::IpdDcPlanTmpEnv`
                match DcPlanTmpEnv::call_api_get(&DcPlanTmpParams::default(), app.state()).await {
                    Ok(responses) => {
                        let mut lock = page.all_envs.lock_mut();
                        lock.extend(responses.into_iter().map(Rc::new));
                    }
                    Err(e) => {
                        app.alert_app_error(&e).await;
                    }
                }
                // GET `EndPoint::IpdDcPlanTmpTx`
                match DcPlanTmpTx::call_api_get(&DcPlanTmpParams::default(), app.state()).await {
                    Ok(responses) => {
                        let mut lock = page.all_txs.lock_mut();
                        lock.extend(responses.into_iter().map(Rc::new));
                    }
                    Err(e) => {
                        app.alert_app_error(&e).await;
                    }
                }
                // GET `EndPoint::IpdDcPlanTmpDiet`
                match DcPlanTmpDiet::call_api_get(&DcPlanTmpParams::default(), app.state()).await {
                    Ok(responses) => {
                        let mut lock = page.all_diets.lock_mut();
                        lock.extend(responses.into_iter().map(Rc::new));
                    }
                    Err(e) => {
                        app.alert_app_error(&e).await;
                    }
                }
            }),
        )
    }

    fn new_dc_plan(&self) {
        self.dc_plan_id.set_neq(0);
        self.dx_id.set_neq(String::new());
        self.dc_datetime.set_neq(String::new());
        self.dc_type_ok.set_neq(String::new());
        self.dc_type_refer.set_neq(String::new());
        self.dc_type_other.set_neq(String::new());
        self.dc_symptom.set_neq(String::new());
        self.inst_none.set_neq(String::new());
        self.inst_foley.set_neq(String::new());
        self.inst_ett.set_neq(String::new());
        self.inst_tt.set_neq(String::new());
        self.inst_ng.set_neq(String::new());
        self.inst_other.set_neq(String::new());
        self.with_drug.set_neq(String::new());
        self.with_appoint.set_neq(String::new());
        self.with_cert.set_neq(String::new());
        self.with_other.set_neq(String::new());
        self.appoint_date.set_neq(String::new());
        self.appoint_time.set_neq(String::new());
        self.appoint_place.set_neq(String::new());
        self.appoint_for.set_neq(String::new());
        self.refer_to.set_neq(String::new());
        self.dx_text.set_neq(String::new());
        self.dx_patient_ok.set_neq(String::new());
        self.dx_relatives_ok.set_neq(String::new());
        self.dx_other.set_neq(String::new());
        self.dx_doctor.set_neq(String::new());
        self.dx_datetime.set_neq(String::new());
        self.med_text.set_neq(String::new());
        self.med_patient_ok.set_neq(String::new());
        self.med_relatives_ok.set_neq(String::new());
        self.med_other.set_neq(String::new());
        self.med_doctor.set_neq(String::new());
        self.med_datetime.set_neq(String::new());
        self.env_text.set_neq(String::new());
        self.env_patient_ok.set_neq(String::new());
        self.env_relatives_ok.set_neq(String::new());
        self.env_other.set_neq(String::new());
        self.env_doctor.set_neq(String::new());
        self.env_datetime.set_neq(String::new());
        self.tx_text.set_neq(String::new());
        self.tx_patient_ok.set_neq(String::new());
        self.tx_relatives_ok.set_neq(String::new());
        self.tx_other.set_neq(String::new());
        self.tx_doctor.set_neq(String::new());
        self.tx_datetime.set_neq(String::new());
        self.health_text.set_neq(String::new());
        self.health_patient_ok.set_neq(String::new());
        self.health_relatives_ok.set_neq(String::new());
        self.health_other.set_neq(String::new());
        self.health_doctor.set_neq(String::new());
        self.health_datetime.set_neq(String::new());
        self.out_text.set_neq(String::new());
        self.out_patient_ok.set_neq(String::new());
        self.out_relatives_ok.set_neq(String::new());
        self.out_other.set_neq(String::new());
        self.out_doctor.set_neq(String::new());
        self.out_datetime.set_neq(String::new());
        self.diet_text.set_neq(String::new());
        self.diet_patient_ok.set_neq(String::new());
        self.diet_relatives_ok.set_neq(String::new());
        self.diet_other.set_neq(String::new());
        self.diet_doctor.set_neq(String::new());
        self.diet_datetime.set_neq(String::new());
        self.version.set_neq(1);

        self.dx_name.set_neq(String::new());
        self.dx_knowledge.set_neq(String::new());
        self.dx_revisit.set_neq(String::new());
        self.dx_prevention.set_neq(String::new());
        self.med_ids.lock_mut().clear();
        self.env_ids.lock_mut().clear();
        self.tx_ids.lock_mut().clear();
        self.diet_ids.lock_mut().clear();
        self.dx_user_name.set_neq(String::new());
        self.dx_entryposition.set_neq(String::new());
        self.med_user_name.set_neq(String::new());
        self.med_entryposition.set_neq(String::new());
        self.env_user_name.set_neq(String::new());
        self.env_entryposition.set_neq(String::new());
        self.tx_user_name.set_neq(String::new());
        self.tx_entryposition.set_neq(String::new());
        self.health_user_name.set_neq(String::new());
        self.health_entryposition.set_neq(String::new());
        self.out_user_name.set_neq(String::new());
        self.out_entryposition.set_neq(String::new());
        self.diet_user_name.set_neq(String::new());
        self.diet_entryposition.set_neq(String::new());

        self.redraw_dx_selector.set(true);
        self.changed.set_neq(false);
    }

    fn set_dc_plan(&self, dc_plan: Rc<DischargePlan>) {
        self.dc_plan_id.set_neq(dc_plan.dc_plan_id);
        self.dx_id.set_neq(dc_plan.dx_id.to_string());
        self.dc_datetime.set_neq(dc_plan.dc_datetime.map(|dt| dt.js_string()).unwrap_or_default());
        self.dc_type_ok.set_neq(dc_plan.dc_type_ok.clone().unwrap_or_default());
        self.dc_type_refer.set_neq(dc_plan.dc_type_refer.clone().unwrap_or_default());
        self.dc_type_other.set_neq(dc_plan.dc_type_other.clone().unwrap_or_default());
        self.dc_symptom.set_neq(dc_plan.dc_symptom.clone().unwrap_or_default());
        self.inst_none.set_neq(dc_plan.inst_none.clone().unwrap_or_default());
        self.inst_foley.set_neq(dc_plan.inst_foley.clone().unwrap_or_default());
        self.inst_ett.set_neq(dc_plan.inst_ett.clone().unwrap_or_default());
        self.inst_tt.set_neq(dc_plan.inst_tt.clone().unwrap_or_default());
        self.inst_ng.set_neq(dc_plan.inst_ng.clone().unwrap_or_default());
        self.inst_other.set_neq(dc_plan.inst_other.clone().unwrap_or_default());
        self.with_drug.set_neq(dc_plan.with_drug.clone().unwrap_or_default());
        self.with_appoint.set_neq(dc_plan.with_appoint.clone().unwrap_or_default());
        self.with_cert.set_neq(dc_plan.with_cert.clone().unwrap_or_default());
        self.with_other.set_neq(dc_plan.with_other.clone().unwrap_or_default());
        self.appoint_date.set_neq(dc_plan.appoint_date.map(|d| d.to_string()).unwrap_or_default());
        self.appoint_time.set_neq(dc_plan.appoint_time.map(|t| t.js_string()).unwrap_or_default());
        self.appoint_place.set_neq(dc_plan.appoint_place.clone().unwrap_or_default());
        self.appoint_for.set_neq(dc_plan.appoint_for.clone().unwrap_or_default());
        self.refer_to.set_neq(dc_plan.refer_to.clone().unwrap_or_default());
        self.dx_text.set_neq(dc_plan.dx_text.clone().unwrap_or_default());
        self.dx_patient_ok.set_neq(dc_plan.dx_patient_ok.clone().unwrap_or_default());
        self.dx_relatives_ok.set_neq(dc_plan.dx_relatives_ok.clone().unwrap_or_default());
        self.dx_other.set_neq(dc_plan.dx_other.clone().unwrap_or_default());
        self.dx_doctor.set_neq(dc_plan.dx_doctor.clone().unwrap_or_default());
        self.dx_datetime.set_neq(dc_plan.dx_datetime.map(|dt| dt.js_string()).unwrap_or_default());
        self.med_text.set_neq(dc_plan.med_text.clone().unwrap_or_default());
        self.med_patient_ok.set_neq(dc_plan.med_patient_ok.clone().unwrap_or_default());
        self.med_relatives_ok.set_neq(dc_plan.med_relatives_ok.clone().unwrap_or_default());
        self.med_other.set_neq(dc_plan.med_other.clone().unwrap_or_default());
        self.med_doctor.set_neq(dc_plan.med_doctor.clone().unwrap_or_default());
        self.med_datetime.set_neq(dc_plan.med_datetime.map(|dt| dt.js_string()).unwrap_or_default());
        self.env_text.set_neq(dc_plan.env_text.clone().unwrap_or_default());
        self.env_patient_ok.set_neq(dc_plan.env_patient_ok.clone().unwrap_or_default());
        self.env_relatives_ok.set_neq(dc_plan.env_relatives_ok.clone().unwrap_or_default());
        self.env_other.set_neq(dc_plan.env_other.clone().unwrap_or_default());
        self.env_doctor.set_neq(dc_plan.env_doctor.clone().unwrap_or_default());
        self.env_datetime.set_neq(dc_plan.env_datetime.map(|dt| dt.js_string()).unwrap_or_default());
        self.tx_text.set_neq(dc_plan.tx_text.clone().unwrap_or_default());
        self.tx_patient_ok.set_neq(dc_plan.tx_patient_ok.clone().unwrap_or_default());
        self.tx_relatives_ok.set_neq(dc_plan.tx_relatives_ok.clone().unwrap_or_default());
        self.tx_other.set_neq(dc_plan.tx_other.clone().unwrap_or_default());
        self.tx_doctor.set_neq(dc_plan.tx_doctor.clone().unwrap_or_default());
        self.tx_datetime.set_neq(dc_plan.tx_datetime.map(|dt| dt.js_string()).unwrap_or_default());
        self.health_text.set_neq(dc_plan.health_text.clone().unwrap_or_default());
        self.health_patient_ok.set_neq(dc_plan.health_patient_ok.clone().unwrap_or_default());
        self.health_relatives_ok.set_neq(dc_plan.health_relatives_ok.clone().unwrap_or_default());
        self.health_other.set_neq(dc_plan.health_other.clone().unwrap_or_default());
        self.health_doctor.set_neq(dc_plan.health_doctor.clone().unwrap_or_default());
        self.health_datetime.set_neq(dc_plan.health_datetime.map(|dt| dt.js_string()).unwrap_or_default());
        self.out_text.set_neq(dc_plan.out_text.clone().unwrap_or_default());
        self.out_patient_ok.set_neq(dc_plan.out_patient_ok.clone().unwrap_or_default());
        self.out_relatives_ok.set_neq(dc_plan.out_relatives_ok.clone().unwrap_or_default());
        self.out_other.set_neq(dc_plan.out_other.clone().unwrap_or_default());
        self.out_doctor.set_neq(dc_plan.out_doctor.clone().unwrap_or_default());
        self.out_datetime.set_neq(dc_plan.out_datetime.map(|dt| dt.js_string()).unwrap_or_default());
        self.diet_text.set_neq(dc_plan.diet_text.clone().unwrap_or_default());
        self.diet_patient_ok.set_neq(dc_plan.diet_patient_ok.clone().unwrap_or_default());
        self.diet_relatives_ok.set_neq(dc_plan.diet_relatives_ok.clone().unwrap_or_default());
        self.diet_other.set_neq(dc_plan.diet_other.clone().unwrap_or_default());
        self.diet_doctor.set_neq(dc_plan.diet_doctor.clone().unwrap_or_default());
        self.diet_datetime.set_neq(dc_plan.diet_datetime.map(|dt| dt.js_string()).unwrap_or_default());
        self.version.set_neq(dc_plan.version);

        self.dx_name.set_neq(dc_plan.dx_name.clone());
        self.dx_knowledge.set_neq(dc_plan.dx_knowledge.clone().unwrap_or_default());
        self.dx_revisit.set_neq(dc_plan.dx_revisit.clone().unwrap_or_default());
        self.dx_prevention.set_neq(dc_plan.dx_prevention.clone().unwrap_or_default());
        let med_ids = dc_plan
            .meds
            .clone()
            .unwrap_or_default()
            .split('|')
            .flat_map(|g| g.split('^').take(1))
            .map(|s| s.parse::<u32>())
            .collect::<Result<Vec<u32>, std::num::ParseIntError>>()
            .unwrap_or_default();
        {
            let mut lock = self.med_ids.lock_mut();
            lock.replace_cloned(med_ids);
        }
        let env_ids = dc_plan
            .envs
            .clone()
            .unwrap_or_default()
            .split('|')
            .flat_map(|g| g.split('^').take(1))
            .map(|s| s.parse::<u32>())
            .collect::<Result<Vec<u32>, std::num::ParseIntError>>()
            .unwrap_or_default();
        {
            let mut lock = self.env_ids.lock_mut();
            lock.replace_cloned(env_ids);
        }
        let tx_ids = dc_plan
            .txs
            .clone()
            .unwrap_or_default()
            .split('|')
            .flat_map(|g| g.split('^').take(1))
            .map(|s| s.parse::<u32>())
            .collect::<Result<Vec<u32>, std::num::ParseIntError>>()
            .unwrap_or_default();
        {
            let mut lock = self.tx_ids.lock_mut();
            lock.replace_cloned(tx_ids);
        }
        let diet_ids = dc_plan
            .diets
            .clone()
            .unwrap_or_default()
            .split('|')
            .flat_map(|g| g.split('^').take(1))
            .map(|s| s.parse::<u32>())
            .collect::<Result<Vec<u32>, std::num::ParseIntError>>()
            .unwrap_or_default();
        {
            let mut lock = self.diet_ids.lock_mut();
            lock.replace_cloned(diet_ids);
        }
        self.dx_user_name.set_neq(dc_plan.dx_user_name.clone().unwrap_or_default());
        self.dx_entryposition.set_neq(dc_plan.dx_entryposition.clone().unwrap_or_default());
        self.med_user_name.set_neq(dc_plan.med_user_name.clone().unwrap_or_default());
        self.med_entryposition.set_neq(dc_plan.med_entryposition.clone().unwrap_or_default());
        self.env_user_name.set_neq(dc_plan.env_user_name.clone().unwrap_or_default());
        self.env_entryposition.set_neq(dc_plan.env_entryposition.clone().unwrap_or_default());
        self.tx_user_name.set_neq(dc_plan.tx_user_name.clone().unwrap_or_default());
        self.tx_entryposition.set_neq(dc_plan.tx_entryposition.clone().unwrap_or_default());
        self.health_user_name.set_neq(dc_plan.health_user_name.clone().unwrap_or_default());
        self.health_entryposition.set_neq(dc_plan.health_entryposition.clone().unwrap_or_default());
        self.out_user_name.set_neq(dc_plan.out_user_name.clone().unwrap_or_default());
        self.out_entryposition.set_neq(dc_plan.out_entryposition.clone().unwrap_or_default());
        self.diet_user_name.set_neq(dc_plan.diet_user_name.clone().unwrap_or_default());
        self.diet_entryposition.set_neq(dc_plan.diet_entryposition.clone().unwrap_or_default());

        self.redraw_dx_selector.set(true);
        self.changed.set_neq(false);
    }

    fn finalized(&self) -> Option<DischargePlanSave> {
        let dx_id_opt = self.dx_id.lock_ref().parse::<u32>().ok();
        if let Some(dx_id) = dx_id_opt {
            Some(DischargePlanSave {
                dc_plan_id: self.dc_plan_id.get(),
                dx_id,
                dc_datetime: datetime_8601(&self.dc_datetime.lock_ref()),
                dc_type_ok: str_some(self.dc_type_ok.get_cloned()),
                dc_type_refer: str_some(self.dc_type_refer.get_cloned()),
                dc_type_other: str_some(self.dc_type_other.lock_ref().trim().to_owned()),
                dc_symptom: str_some(self.dc_symptom.get_cloned()),
                inst_none: str_some(self.inst_none.get_cloned()),
                inst_foley: str_some(self.inst_foley.get_cloned()),
                inst_ett: str_some(self.inst_ett.get_cloned()),
                inst_tt: str_some(self.inst_tt.get_cloned()),
                inst_ng: str_some(self.inst_ng.get_cloned()),
                inst_other: str_some(self.inst_other.lock_ref().trim().to_owned()),
                with_drug: str_some(self.with_drug.get_cloned()),
                with_appoint: str_some(self.with_appoint.get_cloned()),
                with_cert: str_some(self.with_cert.get_cloned()),
                with_other: str_some(self.with_other.lock_ref().trim().to_owned()),
                appoint_date: date_8601(&self.appoint_date.lock_ref()),
                appoint_time: time_8601(&self.appoint_time.lock_ref()),
                appoint_place: str_some(self.appoint_place.get_cloned()),
                appoint_for: str_some(self.appoint_for.get_cloned()),
                refer_to: str_some(self.refer_to.get_cloned()),
                dx_text: str_some(self.dx_text.get_cloned()),
                dx_patient_ok: str_some(self.dx_patient_ok.get_cloned()),
                dx_relatives_ok: str_some(self.dx_relatives_ok.get_cloned()),
                dx_other: str_some(self.dx_other.get_cloned()),
                dx_doctor: str_some(self.dx_doctor.get_cloned()),
                dx_datetime: datetime_8601(&self.dx_datetime.lock_ref()),
                med_text: str_some(self.med_text.get_cloned()),
                med_patient_ok: str_some(self.med_patient_ok.get_cloned()),
                med_relatives_ok: str_some(self.med_relatives_ok.get_cloned()),
                med_other: str_some(self.med_other.get_cloned()),
                med_doctor: str_some(self.med_doctor.get_cloned()),
                med_datetime: datetime_8601(&self.med_datetime.lock_ref()),
                env_text: str_some(self.env_text.get_cloned()),
                env_patient_ok: str_some(self.env_patient_ok.get_cloned()),
                env_relatives_ok: str_some(self.env_relatives_ok.get_cloned()),
                env_other: str_some(self.env_other.get_cloned()),
                env_doctor: str_some(self.env_doctor.get_cloned()),
                env_datetime: datetime_8601(&self.env_datetime.lock_ref()),
                tx_text: str_some(self.tx_text.get_cloned()),
                tx_patient_ok: str_some(self.tx_patient_ok.get_cloned()),
                tx_relatives_ok: str_some(self.tx_relatives_ok.get_cloned()),
                tx_other: str_some(self.tx_other.get_cloned()),
                tx_doctor: str_some(self.tx_doctor.get_cloned()),
                tx_datetime: datetime_8601(&self.tx_datetime.lock_ref()),
                health_text: str_some(self.health_text.get_cloned()),
                health_patient_ok: str_some(self.health_patient_ok.get_cloned()),
                health_relatives_ok: str_some(self.health_relatives_ok.get_cloned()),
                health_other: str_some(self.health_other.get_cloned()),
                health_doctor: str_some(self.health_doctor.get_cloned()),
                health_datetime: datetime_8601(&self.health_datetime.lock_ref()),
                out_text: str_some(self.out_text.get_cloned()),
                out_patient_ok: str_some(self.out_patient_ok.get_cloned()),
                out_relatives_ok: str_some(self.out_relatives_ok.get_cloned()),
                out_other: str_some(self.out_other.get_cloned()),
                out_doctor: str_some(self.out_doctor.get_cloned()),
                out_datetime: datetime_8601(&self.out_datetime.lock_ref()),
                diet_text: str_some(self.diet_text.get_cloned()),
                diet_patient_ok: str_some(self.diet_patient_ok.get_cloned()),
                diet_relatives_ok: str_some(self.diet_relatives_ok.get_cloned()),
                diet_other: str_some(self.diet_other.get_cloned()),
                diet_doctor: str_some(self.diet_doctor.get_cloned()),
                diet_datetime: datetime_8601(&self.diet_datetime.lock_ref()),
                med_ids: self.med_ids.lock_ref().to_vec(),
                env_ids: self.env_ids.lock_ref().to_vec(),
                tx_ids: self.tx_ids.lock_ref().to_vec(),
                diet_ids: self.diet_ids.lock_ref().to_vec(),
                version: self.version.get(),
            })
        } else {
            None
        }
    }

    fn submit_form(page: Rc<Self>, app: Rc<App>) {
        if let Some(saver) = page.finalized() {
            let visit_type_opt = page.patient.lock_ref().as_ref().map(|pt| pt.visit_type());
            if let Some(visit_type) = visit_type_opt {
                app.async_load(
                    true,
                    clone!(app => async move {
                        let saver_result_opt = match visit_type {
                            VisitTypeId::Ipd(an) | VisitTypeId::PreAdmit(an) => {
                                // POST `EndPoint::IpdDcPlanAn`
                                Some(saver.call_api_post_ipd(&an, app.state()).await)
                            }
                            VisitTypeId::OpdEr(_, opd_er_order_master_id) => {
                                // POST `EndPoint::OpdErDcPlanId`
                                Some(saver.call_api_post_opd_er(opd_er_order_master_id, app.state()).await)
                            }
                            VisitTypeId::Visit(_) => None,
                        };
                        if let Some(saver_result) = saver_result_opt {
                            match saver_result {
                                Ok((_id, responses)) => {
                                    app.alert_execute_responses(&responses, async move {
                                        // app.alert("บันทึกข้อมูลสำเร็จ");
                                        page.dc_plan_selected.set(None);
                                        page.new_dc_plan();
                                        page.loaded_dc_plans.set(false);
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

    fn delete_dc_plan(page: Rc<Self>, app: Rc<App>) {
        let visit_type_opt = page.patient.lock_ref().as_ref().map(|pt| pt.visit_type());
        if let (Some(dc_plan_id), Some(version), Some(visit_type)) = (zero_none(page.dc_plan_id.get()), zero_none(page.version.get()), visit_type_opt) {
            app.async_load(
                true,
                clone!(app, page => async move {
                    if app.confirm("ยืนยันรายการ").await {
                        let params = DischargePlanParams {
                            dc_plan_id: Some(dc_plan_id),
                            version: Some(version),
                        };
                        let result_opt = match visit_type {
                            VisitTypeId::Ipd(an) | VisitTypeId::PreAdmit(an) => {
                                // DELETE `EndPoint::IpdDcPlanAn`
                                Some(DischargePlan::call_api_delete_ipd(&an, &params, app.state()).await)
                            }
                            VisitTypeId::OpdEr(_, opd_er_order_master_id) => {
                                // DELETE `EndPoint::OpdErDcPlanId`
                                Some(DischargePlan::call_api_delete_opd_er(opd_er_order_master_id, &params, app.state()).await)
                            }
                            VisitTypeId::Visit(_) => None,
                        };
                        if let Some(result) = result_opt {
                            match result {
                                Ok(responses) => {
                                    app.alert_execute_responses(&responses, async move {
                                        page.dc_plan_selected.set(None);
                                        page.new_dc_plan();
                                        page.loaded_dc_plans.set(false);
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
        html!("div", {
            .future(map_ref!(
                let busy = app.loader_is_loading(),
                let loaded = page.loaded.signal() =>
                !busy && !loaded
            ).for_each(clone!(app, page => move |ready| {
                if ready {
                    Self::load_all_tmp(page.clone(), app.clone());
                    page.loaded.set_neq(true);
                }
                async {}
            })))
            .future(map_ref!(
                let busy = app.loader_is_loading(),
                let loaded = page.loaded_dc_plans.signal() =>
                !busy && !loaded
            ).for_each(clone!(app, page => move |ready| {
                if ready {
                    Self::load_dc_plans(page.clone(), app.clone());
                    page.loaded_dc_plans.set_neq(true);
                }
                async {}
            })))
            .attr("role", "tabpanel")
            .attr("aria-labelledby", "nav-discharge-plan-tab")
            .class("col")
            .child(html!("div", {
                .child(html!("div", {
                    .class(class::FLEX_WRAP_T)
                    .children([
                        html!("div", {
                            .children_signal_vec(page.dc_plans.signal_vec_cloned().map(clone!(page => move |dc_plan| {
                                let dc_plan_id = dc_plan.dc_plan_id;
                                html!("button", {
                                    .attr("type", "button")
                                    .class(class::BTN_LB2)
                                    .class_signal("btn-primary", page.dc_plan_selected.signal_cloned().map(move |opt| opt.as_ref().map(|dc_plan_selected| dc_plan_selected.dc_plan_id == dc_plan_id).unwrap_or_default()))
                                    .class_signal("btn-secondary", page.dc_plan_selected.signal_cloned().map(move |opt| opt.as_ref().map(|dc_plan_selected| dc_plan_selected.dc_plan_id != dc_plan_id).unwrap_or(true)))
                                    .text(&dc_plan.dx_name)
                                    .apply_if(dc_plan.is_all_signed(), |dom| dom.child(html!("i", {.class(class::FA_CHECK_GREEN_R)})))
                                    .event(clone!(page => move |_:events::Click| {
                                        page.dc_plan_selected.set(Some(dc_plan.clone()));
                                        page.set_dc_plan(dc_plan.clone());
                                    }))
                                })
                            })))
                        }),
                        html!("div", {
                            .class(class::PY_RX)
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
                                        .text(" Add discharge plan")
                                        .event(clone!(page => move |_: events::Click| {
                                            page.dc_plan_selected.set(Some(Rc::new(DischargePlan::default())));
                                            page.new_dc_plan();
                                        }))
                                    })
                                })
                            })))
                            .child_signal(page.patient.signal_cloned().map(clone!(app, page => move |opt| {
                                opt.map(|patient| {
                                    match patient.visit_type() {
                                        VisitTypeId::Ipd(an) | VisitTypeId::PreAdmit(an) => {
                                            html!("div",{
                                                .class("float-end")
                                                .children(PdfButtons::buttons(
                                                    PdfButtons::new(
                                                        TypstReport::from_system_with_coercion(SystemReport::IpdDischargePlan, &app.state().report_coercions()),
                                                        Mutable::new(an.clone()),
                                                        page.checked.clone(),
                                                        page.changed.clone(),
                                                        clone!(page => move || {serde_json::json!({
                                                            "id": an,
                                                            "patient": patient,
                                                            "dc_plans": page.dc_plans.lock_ref().to_vec(),
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
                                                        TypstReport::from_system_with_coercion(SystemReport::OpdErDischargePlan, &app.state().report_coercions()),
                                                        Mutable::new(vn.clone()),
                                                        page.checked.clone(),
                                                        page.changed.clone(),
                                                        clone!(page => move || {serde_json::json!({
                                                            "id": vn,
                                                            "patient": patient,
                                                            "dc_plans": page.dc_plans.lock_ref().to_vec(),
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
                }))
                .child(Self::render_form(page.clone(), app.clone()))
            }))
        })
    }

    fn render_form(page: Rc<Self>, app: Rc<App>) -> Dom {
        html!("div", {
            .visible_signal(page.dc_plan_selected.signal_cloned().map(|dc_plan_selected| dc_plan_selected.is_some()))
            .class(class::CARD)
            .child(html!("div", {
                .children([
                    html!("div", {
                        .class(class::CARD_HEAD)
                        .text("Discharge Plan")
                        .child(html!("div", {
                            .class(class::FLOAT_RR)
                            .child(html!("button", {
                                .attr("type", "button")
                                .attr("id", "close_btn")
                                .class("btn-close")
                                .event(clone!(page => move |_: events::Click| {
                                    page.dc_plan_selected.set(None);
                                }))
                            }))
                        }))
                    }),
                    html!("div", {
                        .class("card-body")
                        .children([
                            html!("div", {
                                .class(class::ROW_T)
                                .children([
                                    Self::render_info(page.clone()),
                                    Self::render_dx(page.clone(), app.clone()),
                                    Self::render_med(page.clone(), app.clone()),
                                    Self::render_env(page.clone(), app.clone()),
                                    Self::render_tx(page.clone(), app.clone()),
                                    Self::render_health(page.clone(), app.clone()),
                                    Self::render_out(page.clone(), app.clone()),
                                    Self::render_diet(page.clone(), app.clone()),
                                ])
                            }),
                            html!("div", {
                                .class("row")
                                .child(html!("div", {
                                    .class(class::COL_SM12_R)
                                    .child_signal(page.is_ipd_and_is_pre_admit().map(clone!(app, page => move |(is_ipd, is_pre_admit)| {
                                        (if is_ipd {
                                            app.endpoint_is_allow(&Method::POST, &EndPoint::IpdDcPlanAn, is_pre_admit)
                                        } else {
                                            app.endpoint_is_allow(&Method::POST, &EndPoint::OpdErDcPlanId, false)
                                        }).then(|| {
                                            html!("button" => HtmlButtonElement, {
                                                .attr("type", "button")
                                                .class(class::BTN_SM_L)
                                                .class_signal("btn-primary", page.changed.signal())
                                                .class_signal("btn-secondary", not(page.changed.signal()))
                                                .child(html!("i", {.class(class::FA_SAVE)}))
                                                .text(" บันทึกข้อมูล")
                                                .apply(mixins::click_with_loader_checked_or_true_disable_signal(clone!(app, page => move || {
                                                    Self::submit_form(page.clone(), app.clone());
                                                }), not(page.changed.signal()), app.state()))
                                            })
                                        })
                                    })))
                                    .child_signal(page.is_ipd_and_is_pre_admit().map(clone!(app, page => move |(is_ipd, is_pre_admit)| {
                                        (if is_ipd {
                                            app.has_permission(Permission::IpdNurseNoteEdit) || (is_pre_admit && app.has_permission(Permission::OpdErNurseNoteEdit))
                                        } else {
                                            app.has_permission(Permission::OpdErNurseNoteEdit)
                                        }).then(|| {
                                            html!("button" => HtmlButtonElement, {
                                                .attr("type", "button")
                                                .class(class::BTN_SM_L_GRAY)
                                                .child(html!("i", {.class(class::FA_UNDO)}))
                                                .text(" ยกเลิกการแก้ไข")
                                                .event(clone!(page => move |_:events::Click| {
                                                    if let Some(dc_plan_selected) = page.dc_plan_selected.get_cloned() {
                                                        page.set_dc_plan(dc_plan_selected);
                                                    }
                                                }))
                                            })
                                        })
                                    })))
                                    .child_signal(map_ref!{
                                        let (is_ipd, is_pre_admit) = page.is_ipd_and_is_pre_admit(),
                                        let dc_plan_id = page.dc_plan_id.signal() =>
                                        (*dc_plan_id, *is_ipd, *is_pre_admit)
                                    }.map(clone!(app, page => move |(id, is_ipd, is_pre_admit)| (
                                        id > 0 &&
                                        if is_ipd {
                                            app.endpoint_is_allow(&Method::DELETE, &EndPoint::IpdDcPlanAn, is_pre_admit)
                                        } else {
                                            app.endpoint_is_allow(&Method::DELETE, &EndPoint::OpdErDcPlanId, false)
                                        }
                                    ).then(|| {
                                        html!("button" => HtmlButtonElement, {
                                            .attr("type", "button")
                                            .class(class::BTN_SM_RED)
                                            .child(html!("i", {.class(class::FA_TRASH)}))
                                            .text(" ลบ")
                                            .apply(mixins::click_with_loader_checked(clone!(app, page => move || {
                                                Self::delete_dc_plan(page.clone(), app.clone());
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

    fn render_info(page: Rc<Self>) -> Dom {
        html!("div", {
            .children([
                html!("div", {
                    .class(class::ROW_AUTO_SM_G2_CT)
                    .children([
                        html!("div", {
                            .class("col-12")
                            .child(html!("b", {.text("วันที่จำหน่าย")}))
                        }),
                        html!("div", {
                            .class("col-12")
                            .child(html!("div", {
                                .class(class::INPUT_GROUP_SM)
                                .children([
                                    doms::datetime_picker(
                                        page.dc_datetime.clone(),
                                        page.changed.clone(), always(false),
                                        |d| d.class(class::FLEX_GROW1).style("min-width","175px"),
                                        |d| d.class(class::FORM_CTRL_ONLY_SM_R0),
                                        |d| d.class(class::FORM_CTRL_ONLY_SM_R0),
                                        |s| s,
                                        always(None),
                                    ),
                                    html!("button", {
                                        .attr("type", "button")
                                        .class(class::BTN_BLUE)
                                        .child(html!("i", {.class(class::FA_MAGIC)}))
                                        .event(clone!(page => move |_:events::Click| {
                                            let now = js_now();
                                            let dc_datetime = page.patient.lock_ref().as_ref().map(|pt| {
                                                if pt.visit_type.is_ipd() {
                                                    datetime_from_opt(pt.dchdate, pt.dchtime).unwrap_or(now)
                                                } else {
                                                    pt.latest_vs_datetime.unwrap_or(now)
                                                }
                                            }).unwrap_or(now);
                                            page.dc_datetime.set_neq(dc_datetime.js_string());
                                            page.changed.set_neq(true);
                                        }))
                                    }),
                                ])
                            }))
                        }),
                        html!("div", {
                            .class("col-12")
                            .child(html!("b", {.text("โดย")}))
                        }),
                        html!("div", {
                            .class("col-12")
                            .child(html!("div", {
                                .class("form-check")
                                .children([
                                    doms::checkbox_container(page.dc_type_ok.clone(), page.changed.clone(), "dc_type_ok"),
                                    doms::label_check_for("dc_type_ok","แพทย์อนุญาต"),
                                ])
                            }))
                        }),
                        html!("div", {
                            .class("col-12")
                            .child(html!("div", {
                                .class("form-check")
                                .children([
                                    doms::checkbox_container(page.dc_type_refer.clone(), page.changed.clone(), "dc_type_refer"),
                                    doms::label_check_for("dc_type_refer","Refer"),
                                ])
                            }))
                        }),
                    ])
                    .children(doms::checkbox_not_empty_with_text_container(
                        page.dc_type_other.clone(),
                        page.changed.clone(), "dc_type_other", "อื่นๆ"
                    ).into_iter().map(|d| html!("div", {.class("col-12").child(d)})))
                }),
                html!("div", {
                    .class(class::ROW_T)
                    .child(html!("div", {
                        .class("col-12")
                        .children([
                            html!("label", {
                                .attr("for", "dc_symptom")
                                .class(class::FORM_LBL_BOLD)
                                .text("อาการและอาการแสดงก่อนจำหน่าย")
                            }),
                            html!("input" => HtmlInputElement, {
                                .attr("type", "text")
                                .class("form-control")
                                .attr("id", "dc_symptom")
                                .apply(mixins::string_value(page.dc_symptom.clone(), page.changed.clone()))
                            }),
                        ])
                    }))
                }),
                html!("div", {
                    .class(class::ROW_AUTO_SM_G2_CT)
                    .children([
                        html!("div", {
                            .class("col-12")
                            .child(html!("b", {.text("อุปกรณ์ที่ติดตัวผู้ป่วย")}))
                        }),
                        html!("div", {
                            .class("col-12")
                            .child(html!("div", {
                                .class("form-check")
                                .children([
                                    doms::checkbox_toggle_texts_container(page.inst_none.clone(), vec![
                                        page.inst_foley.clone(),
                                        page.inst_ett.clone(),
                                        page.inst_tt.clone(),
                                        page.inst_ng.clone(),
                                        page.inst_other.clone(),
                                    ], page.changed.clone(), "inst_none"),
                                    doms::label_check_for("inst_none","ไม่มี"),
                                ])
                            }))
                        }),
                        html!("div", {
                            .class("col-12")
                            .child(html!("div", {
                                .class("form-check")
                                .children([
                                    doms::checkbox_toggle_texts_container(page.inst_foley.clone(), vec![page.inst_none.clone()], page.changed.clone(), "inst_foley"),
                                    doms::label_check_for("inst_foley","สายสวนปัสสาวะ"),
                                ])
                            }))
                        }),
                        html!("div", {
                            .class("col-12")
                            .child(html!("div", {
                                .class("form-check")
                                .children([
                                    doms::checkbox_toggle_texts_container(page.inst_ett.clone(), vec![page.inst_none.clone()], page.changed.clone(), "inst_ett"),
                                    doms::label_check_for("inst_ett","ET-Tube"),
                                ])
                            }))
                        }),
                        html!("div", {
                            .class("col-12")
                            .child(html!("div", {
                                .class("form-check")
                                .children([
                                    doms::checkbox_toggle_texts_container(page.inst_tt.clone(), vec![page.inst_none.clone()], page.changed.clone(), "inst_tt"),
                                    doms::label_check_for("inst_tt","TT-Tube"),
                                ])
                            }))
                        }),
                        html!("div", {
                            .class("col-12")
                            .child(html!("div", {
                                .class("form-check")
                                .children([
                                    doms::checkbox_toggle_texts_container(page.inst_ng.clone(), vec![page.inst_none.clone()], page.changed.clone(), "inst_ng"),
                                    doms::label_check_for("inst_ng","NG-Tube"),
                                ])
                            }))
                        }),
                    ])
                    .children(doms::checkbox_not_empty_texts_with_text_container(
                        page.inst_other.clone(),
                        vec![page.inst_none.clone()],
                        page.changed.clone(), "inst_other", "อื่นๆ"
                    ).into_iter().map(|d| html!("div", {.class("col-12").child(d)})))
                }),
                html!("div", {
                    .class(class::ROW_AUTO_SM_G2_CT)
                    .children([
                        html!("div", {
                            .class("col-12")
                            .child(html!("b", {.text("สิ่งที่ผู้ป่วยได้รับก่อนจำหน่าย")}))
                        }),
                        html!("div", {
                            .class("col-12")
                            .child(html!("div", {
                                .class("form-check")
                                .children([
                                    doms::checkbox_container(page.with_drug.clone(), page.changed.clone(), "with_drug"),
                                    doms::label_check_for("with_drug","ยา"),
                                ])
                            }))
                        }),
                        html!("div", {
                            .class("col-12")
                            .child(html!("div", {
                                .class("form-check")
                                .children([
                                    doms::checkbox_container(page.with_appoint.clone(), page.changed.clone(), "with_appoint"),
                                    doms::label_check_for("with_appoint","ใบนัด"),
                                ])
                            }))
                        }),
                        html!("div", {
                            .class("col-12")
                            .child(html!("div", {
                                .class("form-check")
                                .children([
                                    doms::checkbox_container(page.with_cert.clone(), page.changed.clone(), "with_cert"),
                                    doms::label_check_for("with_cert","ใบรับรองแพทย์"),
                                ])
                            }))
                        }),
                    ])
                    .children(doms::checkbox_not_empty_with_text_container(
                        page.with_other.clone(),
                        page.changed.clone(), "with_other", "อื่นๆ"
                    ).into_iter().map(|d| html!("div", {.class("col-12").child(d)})))
                }),
            ])
        })
    }

    fn render_dx(page: Rc<Self>, app: Rc<App>) -> Dom {
        html!("div", {
            .class(class::BOX_ROUND_T)
            .children([
                html!("div", {
                    .class(class::BOLD_BB_T2_PSB2)
                    .text("D : Diagnosis")
                }),
                doms::form_inline_group_sm(clone!(page, app => move |group| { group
                    .children([
                        doms::label_group_for("dx_select","ชื่อโรคที่เจ็บป่วย"),
                        html!("div", {
                            .class(class::FLEX_GROW1)
                            .style("max-width", "640px")
                            .future(page.redraw_dx_selector.signal().for_each(clone!(page, app => move |redraw| {
                                if redraw {
                                    if let Some(elm) = app.get_id("dx_select") {
                                        NiceSelect::new_default_with_value(&elm, &page.dx_id.lock_ref());
                                    }
                                    page.redraw_dx_selector.set_neq(false);
                                }
                                async {}
                            })))
                            .child(html!("select" => HtmlSelectElement, {
                                .class(class::FORM_CTRL_SM)
                                .attr("id", "dx_select")
                                .children([
                                    html!("option", {
                                        .attr("value", "")
                                        .text("เลือก")
                                    }),
                                ])
                                .children_signal_vec(page.all_dxs.signal_vec_cloned().map(|dx| {
                                    html!("option", {
                                        .attr("value", &dx.dx_id.to_string())
                                        .text(&dx.dx_name.clone().unwrap_or_default())
                                    })
                                }))
                                .apply(mixins::string_value_select(page.dx_id.clone(), page.changed.clone()))
                                .future(page.dx_id.signal_cloned().for_each(clone!(page => move |dx_id_str| {
                                    if let Some(dx_id) = dx_id_str.parse::<u32>().ok() {
                                        if let Some(dx) = page.all_dxs.lock_ref().iter().find(|dx| dx.dx_id == dx_id) {
                                            page.dx_knowledge.set_neq(dx.dx_knowledge.clone().unwrap_or_default());
                                            page.dx_revisit.set_neq(dx.dx_revisit.clone().unwrap_or_default());
                                            page.dx_prevention.set_neq(dx.dx_prevention.clone().unwrap_or_default());
                                        }
                                    }
                                    async {}
                                })))
                            }))
                        }),
                    ])
                })),
                html!("div", {
                    .class(class::BOLD_Y)
                    .text("- ความรู้เกี่ยวกับโรคที่เจ็บป่วย")
                }),
                html!("div", {
                    .class("mb-2")
                    .text_signal(page.dx_knowledge.signal_cloned())
                }),
                html!("div", {
                    .class("col-12")
                    .child(html!("b", {.text("- อื่นๆ")}))
                }),
                html!("div", {
                    .class("col-12")
                    .child(html!("textarea" => HtmlTextAreaElement, {
                        .class(class::FORM_CTRL_SM)
                        .apply(mixins::textarea_value_auto_expand(page.dx_text.clone(), page.changed.clone()))
                    }))
                }),
                Self::signer_container("dx", page, app),
            ])
        })
    }

    fn render_med(page: Rc<Self>, app: Rc<App>) -> Dom {
        html!("div", {
            .class(class::BOX_ROUND_T)
            .children([
                html!("div", {
                    .class(class::BOLD_BB_T2_PSB2)
                    .text("M : Medication")
                }),
                html!("div", {
                    .children_signal_vec(page.all_meds.signal_vec_cloned().map(clone!(page => move |med| {
                        let item_id = med.med_id;
                        let item_text = med.med_text.clone().unwrap_or_default();
                        let id = ["med_", &item_id.to_string()].concat();
                        check_boxes_list(&id, item_id, &item_text, page.med_ids.clone(), page.changed.clone())
                    })))
                }),
                html!("div", {
                    .class("col-12")
                    .child(html!("b", {.text("- อื่นๆ")}))
                }),
                html!("div", {
                    .class("col-12")
                    .child(html!("textarea" => HtmlTextAreaElement, {
                        .class(class::FORM_CTRL_SM)
                        .apply(mixins::textarea_value_auto_expand(page.med_text.clone(), page.changed.clone()))
                    }))
                }),
                Self::signer_container("med", page, app),
            ])
        })
    }

    fn render_env(page: Rc<Self>, app: Rc<App>) -> Dom {
        html!("div", {
            .class(class::BOX_ROUND_T)
            .children([
                html!("div", {
                    .class(class::BOLD_BB_T2_PSB2)
                    .text("E : Environment")
                }),
                html!("div", {
                    .children_signal_vec(page.all_envs.signal_vec_cloned().map(clone!(page => move |env| {
                        let item_id = env.env_id;
                        let item_text = env.env_text.clone().unwrap_or_default();
                        let id = ["env_", &item_id.to_string()].concat();
                        check_boxes_list(&id, item_id, &item_text, page.env_ids.clone(), page.changed.clone())
                    })))
                }),
                html!("div", {
                    .class("col-12")
                    .child(html!("b", {.text("- อื่นๆ")}))
                }),
                html!("div", {
                    .class("col-12")
                    .child(html!("textarea" => HtmlTextAreaElement, {
                        .class(class::FORM_CTRL_SM)
                        .apply(mixins::textarea_value_auto_expand(page.env_text.clone(), page.changed.clone()))
                    }))
                }),
                Self::signer_container("env", page, app),
            ])
        })
    }

    fn render_tx(page: Rc<Self>, app: Rc<App>) -> Dom {
        html!("div", {
            .class(class::BOX_ROUND_T)
            .children([
                html!("div", {
                    .class(class::BOLD_BB_T2_PSB2)
                    .text("T : Treatment")
                }),
                html!("div", {
                    .class(class::BOLD_Y)
                    .text("- ข้อควรปฏิบัติ")
                }),
                html!("div", {
                    .children_signal_vec(page.all_txs.signal_vec_cloned().map(clone!(page => move |tx| {
                        let item_id = tx.tx_id;
                        let item_text = tx.tx_text.clone().unwrap_or_default();
                        let id = ["tx_", &item_id.to_string()].concat();
                        check_boxes_list(&id, item_id, &item_text, page.tx_ids.clone(), page.changed.clone())
                    })))
                }),
                html!("div", {
                    .class(class::BOLD_Y)
                    .text("- อาการเร่งด่วนที่ต้องมา รพ.")
                }),
                html!("div", {
                    .class("mb-2")
                    .text_signal(page.dx_revisit.signal_cloned())
                }),
                html!("div", {
                    .class("col-12")
                    .child(html!("b", {.text("- อื่นๆ")}))
                }),
                html!("div", {
                    .class("col-12")
                    .child(html!("textarea" => HtmlTextAreaElement, {
                        .class(class::FORM_CTRL_SM)
                        .apply(mixins::textarea_value_auto_expand(page.tx_text.clone(), page.changed.clone()))
                    }))
                }),
                Self::signer_container("tx", page, app),
            ])
        })
    }

    fn render_health(page: Rc<Self>, app: Rc<App>) -> Dom {
        html!("div", {
            .class(class::BOX_ROUND_T)
            .children([
                html!("div", {
                    .class(class::BOLD_BB_T2_PSB2)
                    .text("H : Health")
                }),
                html!("div", {
                    .class(class::BOLD_Y)
                    .text("- การฟื้นฟูสภาพร่างกาย, การป้องกันภาวะแทรกซ้อน")
                }),
                html!("div", {
                    .class("mb-2")
                    .text_signal(page.dx_prevention.signal_cloned())
                }),
                html!("div", {
                    .class("col-12")
                    .child(html!("b", {.text("- อื่นๆ")}))
                }),
                html!("div", {
                    .class("col-12")
                    .child(html!("textarea" => HtmlTextAreaElement, {
                        .class(class::FORM_CTRL_SM)
                        .apply(mixins::textarea_value_auto_expand(page.health_text.clone(), page.changed.clone()))
                    }))
                }),
                Self::signer_container("health", page, app),
            ])
        })
    }

    fn render_out(page: Rc<Self>, app: Rc<App>) -> Dom {
        html!("div", {
            .class(class::BOX_ROUND_T)
            .children([
                html!("div", {
                    .class(class::BOLD_BB_T2_PSB2)
                    .text("O : Out Patient Referral")
                }),
                html!("div", {
                    .class(class::ROW_AUTO_SM_G2_CT)
                    .children([
                        html!("div", {
                            .class("col-12")
                            .child(html!("b", {.text("- นัดครั้งต่อไปวันที่")}))
                        }),
                        html!("div", {
                            .class("col-12")
                            .child(doms::date_picker(
                                page.appoint_date.clone(),
                                page.changed.clone(), always(false), None,
                                |d| d.style("min-width","120px"),
                                |d| d.class("form-control-sm"),
                                |d| d.class("form-control-sm"),
                                |s| s, always(None),
                            ))
                        }),
                        html!("div", {
                            .class("col-12")
                            .child(html!("b", {.text("เวลา")}))
                        }),
                        html!("div", {
                            .class("col-12")
                            .child(doms::time_picker(
                                page.appoint_time.clone(),
                                page.changed.clone(), always(false), None,
                                |d| d.style("min-width","95px"),
                                |d| d.class("form-control-sm"),
                                |d| d.class("form-control-sm"),
                                |s| s, always(None),
                            ))
                        }),
                        html!("div", {
                            .class("col-12")
                            .child(html!("b", {.text("สถานที่")}))
                        }),
                        html!("div", {
                            .class("col-12")
                            .child(html!("input" => HtmlInputElement, {
                                .attr("type", "text")
                                .class(class::FORM_CTRL_SM)
                                .apply(mixins::string_value(page.appoint_place.clone(), page.changed.clone()))
                            }))
                        }),
                        html!("div", {
                            .class("col-12")
                            .child(html!("b", {.text("เพื่อตรวจ")}))
                        }),
                        html!("div", {
                            .class("col-12")
                            .child(html!("input" => HtmlInputElement, {
                                .attr("type", "text")
                                .class(class::FORM_CTRL_SM)
                                .apply(mixins::string_value(page.appoint_for.clone(), page.changed.clone()))
                            }))
                        }),
                    ])
                }),
                html!("div", {
                    .class(class::ROW_AUTO_SM_G2_CT)
                    .children([
                        html!("div", {
                            .class("col-12")
                            .child(html!("b", {.text("- ส่งต่อผู้ป่วยไปที่")}))
                        }),
                        html!("div", {
                            .class("col-12")
                            .child(html!("input" => HtmlInputElement, {
                                .attr("type", "text")
                                .class(class::FORM_CTRL_SM)
                                .apply(mixins::string_value(page.refer_to.clone(), page.changed.clone()))
                            }))
                        }),
                    ])
                }),
                html!("div", {
                    .class("col-12")
                    .child(html!("b", {.text("- อื่นๆ")}))
                }),
                html!("div", {
                    .class("col-12")
                    .child(html!("textarea" => HtmlTextAreaElement, {
                        .class(class::FORM_CTRL_SM)
                        .apply(mixins::textarea_value_auto_expand(page.out_text.clone(), page.changed.clone()))
                    }))
                }),
                Self::signer_container("out", page, app),
            ])
        })
    }

    fn render_diet(page: Rc<Self>, app: Rc<App>) -> Dom {
        html!("div", {
            .class(class::BOX_ROUND_T)
            .children([
                html!("div", {
                    .class(class::BOLD_BB_T2_PSB2)
                    .text("D : Diet")
                }),
                html!("div", {
                    .class(class::BOLD_Y)
                    .text("- อาหารที่ควรงด หรือควรรับประทาน")
                }),
                html!("div", {
                    .children_signal_vec(page.all_diets.signal_vec_cloned().map(clone!(page => move |diet| {
                        let item_id = diet.diet_id;
                        let item_text = diet.diet_text.clone().unwrap_or_default();
                        let id = ["diet_", &item_id.to_string()].concat();
                        check_boxes_list(&id, item_id, &item_text, page.diet_ids.clone(), page.changed.clone())
                    })))
                }),
                html!("div", {
                    .class("col-12")
                    .child(html!("b", {.text("- อื่นๆ")}))
                }),
                html!("div", {
                    .class("col-12")
                    .child(html!("textarea" => HtmlTextAreaElement, {
                        .class(class::FORM_CTRL_SM)
                        .apply(mixins::textarea_value_auto_expand(page.diet_text.clone(), page.changed.clone()))
                    }))
                }),
                Self::signer_container("diet", page, app),
            ])
        })
    }

    fn signer_container(id_suffix: &'static str, page: Rc<Self>, app: Rc<App>) -> Dom {
        if let Some((pt_ok_mutable, rel_ok_mutable, other_ok_mutable, doctor_mutable, user_name_mutable, entryposition_mutable, datetime_mutable)) = match id_suffix {
            "dx" => Some((
                page.dx_patient_ok.clone(),
                page.dx_relatives_ok.clone(),
                page.dx_other.clone(),
                page.dx_doctor.clone(),
                page.dx_user_name.clone(),
                page.dx_entryposition.clone(),
                page.dx_datetime.clone(),
            )),
            "med" => Some((
                page.med_patient_ok.clone(),
                page.med_relatives_ok.clone(),
                page.med_other.clone(),
                page.med_doctor.clone(),
                page.med_user_name.clone(),
                page.med_entryposition.clone(),
                page.med_datetime.clone(),
            )),
            "env" => Some((
                page.env_patient_ok.clone(),
                page.env_relatives_ok.clone(),
                page.env_other.clone(),
                page.env_doctor.clone(),
                page.env_user_name.clone(),
                page.env_entryposition.clone(),
                page.env_datetime.clone(),
            )),
            "tx" => Some((
                page.tx_patient_ok.clone(),
                page.tx_relatives_ok.clone(),
                page.tx_other.clone(),
                page.tx_doctor.clone(),
                page.tx_user_name.clone(),
                page.tx_entryposition.clone(),
                page.tx_datetime.clone(),
            )),
            "health" => Some((
                page.health_patient_ok.clone(),
                page.health_relatives_ok.clone(),
                page.health_other.clone(),
                page.health_doctor.clone(),
                page.health_user_name.clone(),
                page.health_entryposition.clone(),
                page.health_datetime.clone(),
            )),
            "out" => Some((
                page.out_patient_ok.clone(),
                page.out_relatives_ok.clone(),
                page.out_other.clone(),
                page.out_doctor.clone(),
                page.out_user_name.clone(),
                page.out_entryposition.clone(),
                page.out_datetime.clone(),
            )),
            "diet" => Some((
                page.diet_patient_ok.clone(),
                page.diet_relatives_ok.clone(),
                page.diet_other.clone(),
                page.diet_doctor.clone(),
                page.diet_user_name.clone(),
                page.diet_entryposition.clone(),
                page.diet_datetime.clone(),
            )),
            _ => None,
        } {
            html!("div", {
                .class(class::ROW_AUTO_SM_G2_JRB)
                .class("border-top")
                .children([
                    html!("div", {
                        .class("col-12")
                        .child(html!("b", {.text("ผลการประเมิน")}))
                    }),
                    html!("div", {
                        .class("col-12")
                        .child(html!("div", {
                            .class("form-check")
                            .children([
                                doms::checkbox_container(pt_ok_mutable, page.changed.clone(), &["dc_patient_ok_", id_suffix].concat()),
                                doms::label_check_for(&["dc_patient_ok_", id_suffix].concat(),"ผู้ป่วยเข้าใจ"),
                            ])
                        }))
                    }),
                    html!("div", {
                        .class("col-12")
                        .child(html!("div", {
                            .class("form-check")
                            .children([
                                doms::checkbox_container(rel_ok_mutable, page.changed.clone(), &["dx_relatives_ok", id_suffix].concat()),
                                doms::label_check_for(&["dx_relatives_ok", id_suffix].concat(),"ญาติเข้าใจ"),
                            ])
                        }))
                    }),
                ])
                .children(doms::checkbox_not_empty_with_text_container(
                    other_ok_mutable,
                    page.changed.clone(), &["dx_other", id_suffix].concat(), "อื่นๆ"
                ).into_iter().map(|d| html!("div", {.class("col-12").child(d)})))
                .children([
                    html!("div", {
                        .class("col-12")
                        .child(html!("div", {
                            .class(class::INPUT_GROUP_SM)
                            .children([
                                html!("button", {
                                    .attr("type", "button")
                                    .class(class::BTN_BLUE)
                                    .text("ลงชื่อ")
                                    .event(clone!(page, doctor_mutable, user_name_mutable, entryposition_mutable, datetime_mutable => move |_:events::Click| {
                                        datetime_mutable.set_neq(js_now().js_string());
                                        doctor_mutable.set_neq(app.doctor_code().unwrap_or_default());
                                        user_name_mutable.set_neq(app.doctor_name().unwrap_or_default());
                                        entryposition_mutable.set_neq(app.doctor_entryposition().unwrap_or_default());
                                        page.changed.set_neq(true);
                                    }))
                                }),
                                html!("label", {
                                    .class("input-group-text")
                                    .text_signal(user_name_mutable.signal_cloned())
                                }),
                                doms::datetime_picker(
                                    datetime_mutable.clone(),
                                    page.changed.clone(), always(false),
                                    |d| d.class(class::FLEX_GROW1).style("min-width","175px"),
                                    |d| d.class(class::FORM_CTRL_ONLY_SM_R0),
                                    |d| d.class(class::FORM_CTRL_ONLY_SM_R0),
                                    |s| s,
                                    always(None),
                                ),
                                html!("button", {
                                    .attr("type", "button")
                                    .class(class::BTN_RED)
                                    .child(html!("i", {.class(class::FA_X)}))
                                    .event(clone!(page => move |_:events::Click| {
                                        doctor_mutable.set_neq(String::new());
                                        user_name_mutable.set_neq(String::new());
                                        entryposition_mutable.set_neq(String::new());
                                        datetime_mutable.set_neq(String::new());
                                        page.changed.set_neq(true);
                                    }))
                                }),
                            ])
                        }))
                    }),
                ])
            })
        } else {
            Dom::empty()
        }
    }
}

fn check_boxes_list(id: &str, item_id: u32, item_text: &str, items: MutableVec<u32>, changed: Mutable<bool>) -> Dom {
    html!("div", {
        .class(class::FORM_CHK_COL_SM12)
        .children([
            html!("input" => HtmlInputElement, {
                .attr("type", "checkbox")
                .attr("id", &id)
                .class("form-check-input")
                .attr("value", &item_id.to_string())
                .with_node!(element => {
                    .future(items.signal_vec_cloned().for_each(clone!(element, items => move |_v| {
                        element.set_checked(items.lock_ref().contains(&item_id));
                        async {}
                    })))
                    .event(move |_: events::Change| {
                        if element.checked() {
                            items.lock_mut().push(item_id);
                        } else {
                            items.lock_mut().retain(|x| *x != item_id);
                        }
                        changed.set_neq(true);
                    })
                })
            }),
            doms::label_check_for_selectable(&id, &item_text),
        ])
    })
}
