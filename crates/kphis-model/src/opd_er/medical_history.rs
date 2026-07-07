use derive_demo::Demo;
use js_sys::JsString;
use serde::{Deserialize, Serialize};
use sqlx::{
    FromRow,
    types::{
        Decimal,
        time::{Date, PrimitiveDateTime, Time},
    },
};
use std::rc::Rc;
use time::macros::{date, datetime, time};
use utoipa::{IntoParams, ToSchema};
use wasm_bindgen::JsCast;

use kphis_util::{
    error::{AppError, Source},
    util::{str_some, zero_none},
};

use crate::{
    app::AppState,
    endpoint::{EndPoint, QueryString, find_qs},
    fetch::{ExecuteResponse, execute_fetch_vec, execute_fetch_vec_with_u32, fetch_json_api},
};

#[derive(Clone, Default, Deserialize, IntoParams)]
pub struct OpdErMedicalHistoryParams {
    pub opd_er_order_master_id: Option<u32>,
    pub hn: Option<String>,
    pub vn: Option<String>,
    pub visit_datetime: Option<String>,
    // pub age_y: Option<i8>,
    pub only_opdscreen: Option<bool>,
    pub view_by: Option<String>,
}

impl QueryString for OpdErMedicalHistoryParams {
    fn from_tuples(params: &[(String, String)]) -> Option<Self> {
        (!params.is_empty()).then(|| Self {
            opd_er_order_master_id: find_qs(params, "opd_er_order_master_id").and_then(|s| s.parse::<u32>().ok()),
            hn: find_qs(params, "hn"),
            vn: find_qs(params, "vn"),
            visit_datetime: find_qs(params, "visit_datetime"),
            only_opdscreen: find_qs(params, "only_opdscreen").and_then(|s| s.parse::<bool>().ok()),
            view_by: find_qs(params, "view_by"),
        })
    }

    fn query_string(&self) -> String {
        let mut queries = Vec::with_capacity(6);
        if let Some(opd_er_order_master_id) = &self.opd_er_order_master_id {
            queries.push(["opd_er_order_master_id=", &opd_er_order_master_id.to_string()].concat());
        }
        if let Some(hn) = &self.hn {
            queries.push(["hn=", hn].concat());
        }
        if let Some(vn) = &self.vn {
            queries.push(["vn=", vn].concat());
        }
        if let Some(visit_datetime) = &self.visit_datetime {
            queries.push(["visit_datetime=", visit_datetime].concat());
        }
        // if let Some(age_y) = &self.age_y {
        //     queries.push(["age_y=",age_y.to_owned()));
        // }
        if let Some(only_opdscreen) = &self.only_opdscreen {
            queries.push(["only_opdscreen=", &only_opdscreen.to_string()].concat());
        }
        if let Some(view_by) = &self.view_by {
            queries.push(["view_by=", view_by].concat());
        }

        (!queries.is_empty()).then(|| ["?", &queries.join("&")].concat()).unwrap_or_default()
    }
}

impl OpdErMedicalHistoryParams {
    pub fn clean(self) -> Self {
        Self {
            opd_er_order_master_id: self.opd_er_order_master_id.and_then(zero_none),
            hn: self.hn.and_then(str_some),
            vn: self.vn.and_then(str_some),
            visit_datetime: self.visit_datetime.and_then(str_some),
            // age_y: self.age_y.and_then(zero_none),
            only_opdscreen: self.only_opdscreen.and_then(|only| only.then_some(true)),
            view_by: self.view_by.and_then(str_some),
        }
    }

    pub fn valid(&self) -> bool {
        if self.only_opdscreen == Some(true) {
            self.vn.is_some()
        } else {
            self.opd_er_order_master_id.is_some() && self.hn.is_some() && self.vn.is_some() && self.visit_datetime.is_some()
            // self.age_y.is_some()
        }
    }
}

/// OPD-ER Medical History
#[derive(Clone, Default, Demo, Deserialize, Serialize, ToSchema)]
#[schema(example = json!(OpdErMedicalHistory::demo()))]
pub struct OpdErMedicalHistory {
    #[Demo(value = "Some(OpdScreenHistory::demo())")]
    pub opdscreen: Option<OpdScreenHistory>,
    #[Demo(value = r#"vec![String::from("PENICILLIN=Rash")]"#)]
    pub hosxp_drugallergy: Vec<String>,
    #[Demo(value = r#"vec![String::from("2023-12-31, Appendectomy, Dr.Doctor, Best Hospital")]"#)]
    pub hosxp_operation_history: Vec<String>,
    #[Demo(value = r#"vec![String::from("I10 : Essential Hypertension (PDX)")]"#)]
    pub hosxp_diagnosis: Vec<String>,
    #[Demo(value = r#"vec![String::from("PENICILLIN=Rash")]"#)]
    pub hosxp_drug_history: Vec<String>,
    #[Demo(value = "Some(VitalSignHistory::demo())")]
    pub vs_kphis: Option<VitalSignHistory>,
}

impl OpdErMedicalHistory {
    /// GET `EndPoint::OpdErMedicalHistory`
    pub async fn call_api_get(params: &OpdErMedicalHistoryParams, app: Rc<AppState>) -> Result<Self, AppError> {
        match fetch_json_api(&[EndPoint::OpdErMedicalHistory.base(), params.query_string()].concat(), "GET", None, app).await {
            Ok((response, true)) => {
                let response: Self = serde_wasm_bindgen::from_value(response).map_err(|e| Source::SerdeWasm.to_teapot_error(e, "Fetch OpdErMedicalHistory"))?;
                Ok(response)
            }
            Ok((app_error, false)) => {
                let error: AppError = serde_wasm_bindgen::from_value(app_error).map_err(|e| Source::SerdeWasm.to_teapot_error(e, "Fetch OpdErMedicalHistory"))?;
                Err(error)
            }
            Err(e) => Err(Source::Js.to_teapot_error(e.dyn_ref::<JsString>().map(|s| s.into()).unwrap_or(String::from("fetch error")), "Fetch Json")),
        }
    }
}

/// Opd-Screen of OPD-ER Medical History
#[derive(Clone, Demo, Deserialize, Serialize, FromRow, ToSchema)]
#[schema(example = json!(OpdScreenHistory::demo()))]
pub struct OpdScreenHistory {
    #[Demo(value = "Some(date!(2023-12-31))")]
    pub vstdate: Option<Date>,
    #[Demo(value = "Some(time!(23:59:59))")]
    pub vsttime: Option<Time>,
    #[Demo(value = "Some(50.0)")]
    pub bw: Option<f64>,
    #[Demo(value = "Some(170)")]
    pub height: Option<i32>,
    #[Demo(value = "Some(7)")]
    pub pain_score: Option<i32>,
    #[Demo(value = "Some(80.0)")]
    pub bpd: Option<f64>,
    #[Demo(value = "Some(120.0)")]
    pub bps: Option<f64>,
    #[Demo(value = "Some(80.0)")]
    pub pulse: Option<f64>,
    #[Demo(value = "Some(20.0)")]
    pub rr: Option<f64>,
    #[Demo(value = "Some(37.5)")]
    pub temperature: Option<f64>,
    #[Demo(value = r#"Some(String::from("Sick"))"#)]
    pub cc: Option<String>,
    #[Demo(value = r#"Some(String::from("Present Hx"))"#)]
    pub hpi: Option<String>,
    #[Demo(value = r#"Some(String::from("Look well"))"#)]
    pub pe_ga_text: Option<String>,
    #[Demo(value = r#"Some(String::from("Y"))"#)]
    pub pe_heent: Option<String>,
    #[Demo(value = r#"Some(String::from("Look well"))"#)]
    pub pe_heent_text: Option<String>,
    #[Demo(value = r#"Some(String::from("Y"))"#)]
    pub pe_heart: Option<String>,
    #[Demo(value = r#"Some(String::from("Look well"))"#)]
    pub pe_heart_text: Option<String>,
    #[Demo(value = r#"Some(String::from("Y"))"#)]
    pub pe_lung: Option<String>,
    #[Demo(value = r#"Some(String::from("Look well"))"#)]
    pub pe_lung_text: Option<String>,
    #[Demo(value = r#"Some(String::from("Y"))"#)]
    pub pe_ab: Option<String>,
    #[Demo(value = r#"Some(String::from("Look well"))"#)]
    pub pe_ab_text: Option<String>,
    #[Demo(value = r#"Some(String::from("Y"))"#)]
    pub pe_ext: Option<String>,
    #[Demo(value = r#"Some(String::from("Look well"))"#)]
    pub pe_ext_text: Option<String>,
    #[Demo(value = r#"Some(String::from("Y"))"#)]
    pub pe_neuro: Option<String>,
    #[Demo(value = r#"Some(String::from("Look well"))"#)]
    pub pe_neuro_text: Option<String>,
    #[Demo(value = r#"Some(String::from("Look well"))"#)]
    pub pe: Option<String>,

    #[Demo(value = "Some(4.0)")]
    pub gcs_e: Option<f64>,
    #[Demo(value = "Some(5.0)")]
    pub gcs_m: Option<f64>,
    #[Demo(value = "Some(6.0)")]
    pub gcs_v: Option<f64>,

    #[Demo(value = r#"Some(String::from("ผู้ป่วยฉุกเฉิน"))"#)]
    pub er_pt_type_name: Option<String>,
    #[Demo(value = r#"Some(String::from("Urgency : เร่งด่วน"))"#)]
    pub er_emergency_type_name: Option<String>,
    #[Demo(value = r#"Some(String::from("Urgency (Level3)"))"#)]
    pub er_emergency_level_name: Option<String>,
    #[Demo(value = r#"Some(String::from("อายุรกรรม"))"#)]
    pub er_spclty_name: Option<String>,
}

/// Vital Sign of OPD-ER Medical History
#[derive(Clone, Demo, Deserialize, Serialize, FromRow, ToSchema)]
#[schema(example = json!(VitalSignHistory::demo()))]
pub struct VitalSignHistory {
    #[Demo(value = "datetime!(2023-12-31 23:59:59)")]
    pub vs_datetime: PrimitiveDateTime,
    #[Demo(value = "Some(Decimal::new(50,0))")]
    pub bw: Option<Decimal>,
    #[Demo(value = "Some(170)")]
    pub height: Option<i32>,
    #[Demo(value = "Some(7)")]
    pub pain: Option<i32>,
    #[Demo(value = "Some(99)")]
    pub sat: Option<u32>,
    #[Demo(value = "Some(Decimal::new(3,0))")]
    pub right_pupil: Option<Decimal>,
    #[Demo(value = "Some(Decimal::new(3,0))")]
    pub left_pupil: Option<Decimal>,
    #[Demo(value = "Some(4)")]
    pub eye: Option<i32>,
    #[Demo(value = r#"Some(String::from("5"))"#)]
    pub verbal: Option<String>,
    #[Demo(value = "Some(6)")]
    pub movement: Option<i32>,
    #[Demo(value = "Some(Decimal::new(333,1))")]
    pub hct: Option<Decimal>,
    #[Demo(value = r#"Some(String::from("88"))"#)]
    pub dtx: Option<String>,
    #[Demo(value = "Some(Decimal::new(33,3))")]
    pub bl: Option<Decimal>,
    #[Demo(value = "Some(Decimal::new(375,1))")]
    pub bt: Option<Decimal>,
    #[Demo(value = "Some(80)")]
    pub pr: Option<u32>,
    #[Demo(value = "Some(20)")]
    pub rr: Option<u32>,
    #[Demo(value = "Some(120)")]
    pub sbp: Option<u32>,
    #[Demo(value = "Some(80)")]
    pub dbp: Option<u32>,
    // #[Demo(value = r#"Some(String::from("Y"))"#)]
    // respirator: Option<String>,
    // #[Demo(value = r#"Some(String::from("Y"))"#)]
    // inotrope: Option<String>,
    // #[Demo(value = "Some(1)")]
    // conscious_id: Option<u32>,
    // #[Demo(value = r#"Some(String::from("1"))"#)]
    // urine_amount: Option<String>,
    // #[Demo(value = r#"Some(String::from("1"))"#)]
    // urine_duration: Option<String>,
    #[Demo(value = r#"Some(String::from("2023-12-31 23:59:59|37.0|86|20|112|||1|||96|||1|||8.5|4|5|6||1"))"#)]
    pub ews_concat: Option<String>,
}

/// OPD-ER Trauma History
#[derive(Clone, Demo, Deserialize, Serialize, FromRow, ToSchema)]
#[schema(example = json!(TraumaHistory::demo()))]
pub struct TraumaHistory {
    #[Demo(value = "1")]
    pub opd_er_pe_id: u32,
    #[Demo(value = "1")]
    pub opd_er_order_master_id: u32,
    /// Airway & restriction c-spine [1=Patent, 2=C-spine protection, 3=Non-patent]
    #[Demo(value = r#"Some(String::from("3"))"#)]
    pub arc: Option<String>,
    #[Demo(value = r#"Some(String::from("Non-Patent"))"#)]
    pub arc_npc_text: Option<String>,
    #[Demo(value = r#"Some(String::from("Chest wall"))"#)]
    pub breathing_chest_wall: Option<String>,
    #[Demo(value = r#"Some(String::from("Lung"))"#)]
    pub breathing_lung: Option<String>,
    /// 1=Stable, 2=Shock
    #[Demo(value = r#"Some(String::from("2"))"#)]
    pub circulation_shock: Option<String>,
    #[Demo(value = r#"Some(String::from("Shock"))"#)]
    pub circulation_shock_text: Option<String>,
    #[Demo(value = r#"Some(String::from("Y"))"#)]
    pub circulation_other: Option<String>,
    #[Demo(value = r#"Some(String::from("Other circulation"))"#)]
    pub circulation_other_text: Option<String>,
    #[Demo(value = "Some(date!(2023-12-31))")]
    pub circulation_efast_date: Option<Date>,
    #[Demo(value = "Some(time!(23:59:59))")]
    pub circulation_efast_time: Option<Time>,
    #[Demo(value = r#"Some(String::from("007"))"#)]
    pub circulation_doctor: Option<String>,
    #[Demo(value = r#"Some(String::from("Dr.Doctor"))"#)]
    pub circulation_doctor_name: Option<String>,
    /// P, N
    #[Demo(value = r#"Some(String::from("P"))"#)]
    pub circulation: Option<String>,
    #[Demo(value = r#"Some(String::from("Cul-se-sac"))"#)]
    pub circulation_positive_text: Option<String>,
    #[Demo(value = "Some(4)")]
    pub disability_e: Option<i32>,
    #[Demo(value = r#"Some(String::from("5"))"#)]
    pub disability_v: Option<String>,
    #[Demo(value = "Some(6)")]
    pub disability_m: Option<i32>,
    #[Demo(value = "Some(Decimal::new(3,0))")]
    pub disability_pupil_rt: Option<Decimal>,
    #[Demo(value = "Some(Decimal::new(3,0))")]
    pub disability_pupil_lt: Option<Decimal>,
    #[Demo(value = r#"Some(String::from("Other disability"))"#)]
    pub disability_other: Option<String>,
    #[Demo(value = r#"Some(String::from("Exposure"))"#)]
    pub exposure: Option<String>,
    #[Demo(value = r#"Some(String::from("007"))"#)]
    pub doctor_pe: Option<String>,
    #[Demo(value = r#"Some(String::from("Dr.Doctor"))"#)]
    pub doctor_name: Option<String>,

    #[Demo(value = r#"Some(String::from("009/99/91NWC10PTESHVK95PNAAA.webp,009/99/91NWC10PTESHVK95PNAAB.webp"))"#)]
    pub imgs: Option<String>,

    #[Demo(value = "1")]
    pub version: i32,
}

impl TraumaHistory {
    /// GET `EndPoint::OpdErMedicalHistoryTrauma`
    pub async fn call_api_get(params: &OpdErMedicalHistoryParams, app: Rc<AppState>) -> Result<Option<Self>, AppError> {
        match fetch_json_api(&[EndPoint::OpdErMedicalHistoryTrauma.base(), params.query_string()].concat(), "GET", None, app).await {
            Ok((response, true)) => {
                let response: Option<Self> = serde_wasm_bindgen::from_value(response).map_err(|e| Source::SerdeWasm.to_teapot_error(e, "Fetch TraumaHistory"))?;
                Ok(response)
            }
            Ok((app_error, false)) => {
                let error: AppError = serde_wasm_bindgen::from_value(app_error).map_err(|e| Source::SerdeWasm.to_teapot_error(e, "Fetch TraumaHistory"))?;
                Err(error)
            }
            Err(e) => Err(Source::Js.to_teapot_error(e.dyn_ref::<JsString>().map(|s| s.into()).unwrap_or(String::from("fetch error")), "Fetch Json")),
        }
    }

    /// POST `EndPoint::OpdErMedicalHistoryTrauma`
    pub async fn call_api_post(&self, app: Rc<AppState>) -> Result<(u32, Vec<ExecuteResponse>), AppError> {
        let body_json = serde_json::to_string(self).map_err(|e| Source::SerdeJson.to_teapot_error(e, "Send TraumaHistory"))?;

        let body = serde_wasm_bindgen::to_value(&body_json).map_err(|e| Source::SerdeWasm.to_teapot_error(e, "Send TraumaHistory"))?;

        execute_fetch_vec_with_u32(&EndPoint::OpdErMedicalHistoryTrauma.base(), "POST", Some(&body), app).await
    }
}

/// OPD-ER Allergy History
#[derive(Clone, Demo, Deserialize, Serialize, FromRow, ToSchema)]
#[schema(example = json!(AllergyHistory::demo()))]
pub struct AllergyHistory {
    #[Demo(value = "1")]
    pub er_allergy_history_id: u32,
    #[Demo(value = "Some(1)")]
    pub opd_er_order_master_id: Option<u32>,
    #[Demo(value = r#"Some(String::from("PENICILLIN"))"#)]
    pub er_allergy_history_agent: Option<String>,
    #[Demo(value = r#"Some(String::from("Rash"))"#)]
    pub er_allergy_history_symptom: Option<String>,
    #[Demo(value = r#"Some(String::from("007"))"#)]
    pub er_allergy_history_doctorcode: Option<String>,
    #[Demo(value = r#"Some(String::from("Dr.Doctor"))"#)]
    pub doctor_name: Option<String>,
    #[Demo(value = "1")]
    pub version: i32,
}

impl AllergyHistory {
    /// GET `EndPoint::OpdErMedicalHistoryAllergy`
    pub async fn call_api_get(params: &OpdErMedicalHistoryParams, app: Rc<AppState>) -> Result<Vec<Self>, AppError> {
        match fetch_json_api(&[EndPoint::OpdErMedicalHistoryAllergy.base(), params.query_string()].concat(), "GET", None, app).await {
            Ok((response, true)) => {
                let response: Vec<Self> = serde_wasm_bindgen::from_value(response).map_err(|e| Source::SerdeWasm.to_teapot_error(e, "Fetch AllergyHistory"))?;
                Ok(response)
            }
            Ok((app_error, false)) => {
                let error: AppError = serde_wasm_bindgen::from_value(app_error).map_err(|e| Source::SerdeWasm.to_teapot_error(e, "Fetch AllergyHistory"))?;
                Err(error)
            }
            Err(e) => Err(Source::Js.to_teapot_error(e.dyn_ref::<JsString>().map(|s| s.into()).unwrap_or(String::from("fetch error")), "Fetch Json")),
        }
    }

    /// POST `EndPoint::OpdErMedicalHistoryAllergy`
    pub async fn call_api_post(this: &[Self], app: Rc<AppState>) -> Result<Vec<ExecuteResponse>, AppError> {
        let body_json = serde_json::to_string(&this).map_err(|e| Source::SerdeJson.to_teapot_error(e, "Send TraumaHistory"))?;

        let body = serde_wasm_bindgen::to_value(&body_json).map_err(|e| Source::SerdeWasm.to_teapot_error(e, "Send TraumaHistory"))?;

        execute_fetch_vec(&EndPoint::OpdErMedicalHistoryAllergy.base(), "POST", Some(&body), app).await
    }
}

/// OPD-ER Screeing History
#[derive(Clone, Demo, Deserialize, Serialize, FromRow, ToSchema)]
#[schema(example = json!(NurseScreeningHistory::demo()))]
pub struct NurseScreeningHistory {
    #[Demo(value = "1")]
    pub opd_er_screening_id: u32,
    #[Demo(value = "1")]
    pub opd_er_order_master_id: u32,
    #[Demo(value = r#"Some(String::from("3"))"#)]
    pub screening_emergency_level: Option<String>,
    #[Demo(value = r#"Some(String::from("1"))"#)]
    pub screening_spclty: Option<String>,
    #[Demo(value = "Some(date!(2023-12-31))")]
    pub screening_arrive_date: Option<Date>,
    #[Demo(value = "Some(time!(23:59:59))")]
    pub screening_arrive_time: Option<Time>,
    #[Demo(value = "Some(date!(2023-12-31))")]
    pub screening_date: Option<Date>,
    #[Demo(value = "Some(time!(23:59:59))")]
    pub screening_time: Option<Time>,
    #[Demo(value = "Some(date!(2023-12-31))")]
    pub screening_report_date: Option<Date>,
    #[Demo(value = "Some(time!(23:59:59))")]
    pub screening_report_time: Option<Time>,
    #[Demo(value = "Some(date!(2023-12-31))")]
    pub screening_see_doctor_date: Option<Date>,
    #[Demo(value = "Some(time!(23:59:59))")]
    pub screening_see_doctor_time: Option<Time>,
    #[Demo(value = r#"Some(String::from("007"))"#)]
    pub screening_doctor_doctorcode: Option<String>,
    #[Demo(value = r#"Some(String::from("008"))"#)]
    pub screening_nurse_doctorcode: Option<String>,
    #[Demo(value = r#"Some(String::from("Dr.Doctor"))"#)]
    pub doctor_name: Option<String>,
    #[Demo(value = r#"Some(String::from("Miss.Nurse"))"#)]
    pub nurse_name: Option<String>,
    #[Demo(value = "1")]
    pub version: i32,
}

impl NurseScreeningHistory {
    /// GET `EndPoint::OpdErMedicalHistoryScreen`
    pub async fn call_api_get(params: &OpdErMedicalHistoryParams, app: Rc<AppState>) -> Result<Option<Self>, AppError> {
        match fetch_json_api(&[EndPoint::OpdErMedicalHistoryScreen.base(), params.query_string()].concat(), "GET", None, app).await {
            Ok((response, true)) => {
                let response: Option<Self> = serde_wasm_bindgen::from_value(response).map_err(|e| Source::SerdeWasm.to_teapot_error(e, "Fetch ScreenHistory"))?;
                Ok(response)
            }
            Ok((app_error, false)) => {
                let error: AppError = serde_wasm_bindgen::from_value(app_error).map_err(|e| Source::SerdeWasm.to_teapot_error(e, "Fetch ScreenHistory"))?;
                Err(error)
            }
            Err(e) => Err(Source::Js.to_teapot_error(e.dyn_ref::<JsString>().map(|s| s.into()).unwrap_or(String::from("fetch error")), "Fetch Json")),
        }
    }

    /// POST `EndPoint::OpdErMedicalHistoryScreen`
    pub async fn call_api_post(&self, params: &OpdErMedicalHistoryParams, app: Rc<AppState>) -> Result<(u32, Vec<ExecuteResponse>), AppError> {
        let body_json = serde_json::to_string(self).map_err(|e| Source::SerdeJson.to_teapot_error(e, "Send ScreenHistory"))?;

        let body = serde_wasm_bindgen::to_value(&body_json).map_err(|e| Source::SerdeWasm.to_teapot_error(e, "Send ScreenHistory"))?;

        execute_fetch_vec_with_u32(&[EndPoint::OpdErMedicalHistoryScreen.base(), params.query_string()].concat(), "POST", Some(&body), app).await
    }
}

/// OPD-ER Consult History
#[derive(Clone, Demo, Deserialize, Serialize, FromRow, ToSchema)]
#[schema(example = json!(ConsultHistory::demo()))]
pub struct ConsultHistory {
    #[Demo(value = "1")]
    pub er_consult_id: u32,
    #[Demo(value = "Some(1)")]
    pub opd_er_order_master_id: Option<u32>,
    #[Demo(value = r#"Some(String::from("1"))"#)]
    pub er_consult_ward: Option<String>,
    #[Demo(value = r#"Some(String::from("อายุรกรรมชาย"))"#)]
    pub er_consult_ward_name: Option<String>,
    #[Demo(value = "Some(date!(2023-12-31))")]
    pub er_consult_date: Option<Date>,
    #[Demo(value = "Some(time!(23:59:59))")]
    pub er_consult_time: Option<Time>,
    #[Demo(value = r#"Some(String::from("011"))"#)]
    pub er_consult_doctor_reply: Option<String>,
    #[Demo(value = "Some(date!(2023-12-31))")]
    pub er_consult_date_reply: Option<Date>,
    #[Demo(value = "Some(time!(23:59:59))")]
    pub er_consult_time_reply: Option<Time>,
    #[Demo(value = r#"Some(String::from("007"))"#)]
    pub er_consult_doctorcode: Option<String>,
    #[Demo(value = r#"Some(String::from("Doctor"))"#)]
    pub doctor_name: Option<String>,
    #[Demo(value = "1")]
    pub version: i32,
}

impl ConsultHistory {
    /// GET `EndPoint::OpdErMedicalHistoryConsult`
    pub async fn call_api_get(params: &OpdErMedicalHistoryParams, app: Rc<AppState>) -> Result<Vec<Self>, AppError> {
        match fetch_json_api(&[EndPoint::OpdErMedicalHistoryConsult.base(), params.query_string()].concat(), "GET", None, app).await {
            Ok((response, true)) => {
                let response: Vec<Self> = serde_wasm_bindgen::from_value(response).map_err(|e| Source::SerdeWasm.to_teapot_error(e, "Fetch ConsultHistory"))?;
                Ok(response)
            }
            Ok((app_error, false)) => {
                let error: AppError = serde_wasm_bindgen::from_value(app_error).map_err(|e| Source::SerdeWasm.to_teapot_error(e, "Fetch ConsultHistory"))?;
                Err(error)
            }
            Err(e) => Err(Source::Js.to_teapot_error(e.dyn_ref::<JsString>().map(|s| s.into()).unwrap_or(String::from("fetch error")), "Fetch Json")),
        }
    }

    /// POST `EndPoint::OpdErMedicalHistoryConsult`
    pub async fn call_api_post(this: &[Self], app: Rc<AppState>) -> Result<Vec<ExecuteResponse>, AppError> {
        let body_json = serde_json::to_string(&this).map_err(|e| Source::SerdeJson.to_teapot_error(e, "Send ConsultHistory"))?;

        let body = serde_wasm_bindgen::to_value(&body_json).map_err(|e| Source::SerdeWasm.to_teapot_error(e, "Send ConsultHistory"))?;

        execute_fetch_vec(&EndPoint::OpdErMedicalHistoryConsult.base(), "POST", Some(&body), app).await
    }
}

/// OPD-ER Scan History
#[derive(Clone, Demo, Deserialize, Serialize, FromRow, ToSchema)]
#[schema(example = json!(ScanHistory::demo()))]
pub struct ScanHistory {
    #[Demo(value = "1")]
    pub opd_er_document_scan_id: u32,
    #[Demo(value = "1")]
    pub opd_er_order_master_id: u32,
    #[Demo(value = r#"Some(String::from("Y"))"#)]
    pub opd_er_document_scan: Option<String>,
    #[Demo(value = r#"Some(String::from("007"))"#)]
    pub opd_er_document_scan_doctorcode: Option<String>,
    #[Demo(value = "1")]
    pub version: i32,
}

impl ScanHistory {
    /// GET `EndPoint::OpdErMedicalHistoryScan`
    pub async fn call_api_get(params: &OpdErMedicalHistoryParams, app: Rc<AppState>) -> Result<Option<Self>, AppError> {
        match fetch_json_api(&[EndPoint::OpdErMedicalHistoryScan.base(), params.query_string()].concat(), "GET", None, app).await {
            Ok((response, true)) => {
                let response: Option<Self> = serde_wasm_bindgen::from_value(response).map_err(|e| Source::SerdeWasm.to_teapot_error(e, "Fetch ScanHistory"))?;
                Ok(response)
            }
            Ok((app_error, false)) => {
                let error: AppError = serde_wasm_bindgen::from_value(app_error).map_err(|e| Source::SerdeWasm.to_teapot_error(e, "Fetch ScanHistory"))?;
                Err(error)
            }
            Err(e) => Err(Source::Js.to_teapot_error(e.dyn_ref::<JsString>().map(|s| s.into()).unwrap_or(String::from("fetch error")), "Fetch Json")),
        }
    }

    /// POST `EndPoint::OpdErMedicalHistoryScan`
    pub async fn call_api_post(&self, app: Rc<AppState>) -> Result<(u32, Vec<ExecuteResponse>), AppError> {
        let body_json = serde_json::to_string(self).map_err(|e| Source::SerdeJson.to_teapot_error(e, "Send ScanHistory"))?;

        let body = serde_wasm_bindgen::to_value(&body_json).map_err(|e| Source::SerdeWasm.to_teapot_error(e, "Send ScanHistory"))?;

        execute_fetch_vec_with_u32(&EndPoint::OpdErMedicalHistoryScan.base(), "POST", Some(&body), app).await
    }
}

/// OPD-ER Set-Fast-Tract History
#[derive(Clone, Demo, Deserialize, Serialize, FromRow, ToSchema)]
#[schema(example = json!(SetFtHistory::demo()))]
pub struct SetFtHistory {
    #[Demo(value = "1")]
    pub set_ft_id: u32,
    #[Demo(value = "Some(1)")]
    pub opd_er_order_master_id: Option<u32>,
    #[Demo(value = "Some(date!(2023-12-31))")]
    pub set_ft_date: Option<Date>,
    #[Demo(value = "Some(time!(23:59:59))")]
    pub set_ft_time: Option<Time>,
    #[Demo(value = r#"Some(String::from("007"))"#)]
    pub set_ft_doctorcode: Option<String>,
    #[Demo(value = r#"Some(String::from("Dr.Doctor"))"#)]
    pub doctor_name: Option<String>,
    #[Demo(value = "1")]
    pub version: i32,
}

impl SetFtHistory {
    /// GET `EndPoint::OpdErMedicalHistoryFt`
    pub async fn call_api_get(params: &OpdErMedicalHistoryParams, app: Rc<AppState>) -> Result<Option<Self>, AppError> {
        match fetch_json_api(&[EndPoint::OpdErMedicalHistoryFt.base(), params.query_string()].concat(), "GET", None, app).await {
            Ok((response, true)) => {
                let response: Option<Self> = serde_wasm_bindgen::from_value(response).map_err(|e| Source::SerdeWasm.to_teapot_error(e, "Fetch SetFtHistory"))?;
                Ok(response)
            }
            Ok((app_error, false)) => {
                let error: AppError = serde_wasm_bindgen::from_value(app_error).map_err(|e| Source::SerdeWasm.to_teapot_error(e, "Fetch SetFtHistory"))?;
                Err(error)
            }
            Err(e) => Err(Source::Js.to_teapot_error(e.dyn_ref::<JsString>().map(|s| s.into()).unwrap_or(String::from("fetch error")), "Fetch Json")),
        }
    }

    /// POST `EndPoint::OpdErMedicalHistoryFt`
    pub async fn call_api_post(&self, app: Rc<AppState>) -> Result<(u32, Vec<ExecuteResponse>), AppError> {
        let body_json = serde_json::to_string(self).map_err(|e| Source::SerdeJson.to_teapot_error(e, "Send SetFtHistory"))?;

        let body = serde_wasm_bindgen::to_value(&body_json).map_err(|e| Source::SerdeWasm.to_teapot_error(e, "Send SetFtHistory"))?;

        execute_fetch_vec_with_u32(&EndPoint::OpdErMedicalHistoryFt.base(), "POST", Some(&body), app).await
    }
}
