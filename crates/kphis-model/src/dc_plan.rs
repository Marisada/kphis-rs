use derive_demo::Demo;
use js_sys::JsString;
use serde::{Deserialize, Serialize};
use sqlx::{
    FromRow, MySql, Pool,
    mysql::MySqlQueryResult,
    types::time::{Date, PrimitiveDateTime, Time},
};
use sqlx_binder::MySqlBinder;
use std::rc::Rc;
use time::macros::{date, datetime, time};
use utoipa::{IntoParams, ToSchema};
use wasm_bindgen::JsCast;

use kphis_util::error::{AppError, Source};

use crate::{
    app::AppState,
    endpoint::{EndPoint, QueryString, find_qs},
    fetch::{ExecuteResponse, execute_fetch_vec, execute_fetch_vec_with_u32, fetch_json_api},
};

#[derive(Clone, Default, Deserialize, IntoParams)]
pub struct DischargePlanParams {
    pub dc_plan_id: Option<u32>,
    pub version: Option<i32>,
}

impl QueryString for DischargePlanParams {
    fn from_tuples(params: &[(String, String)]) -> Option<Self> {
        (!params.is_empty()).then(|| Self {
            dc_plan_id: find_qs(params, "dc_plan_id").and_then(|s| s.parse::<u32>().ok()),
            version: find_qs(params, "version").and_then(|s| s.parse::<i32>().ok()),
        })
    }

    fn query_string(&self) -> String {
        let mut queries = Vec::new();

        if let Some(dc_plan_id) = &self.dc_plan_id {
            queries.push(["dc_plan_id=", &dc_plan_id.to_string()].concat());
        }
        if let Some(version) = &self.version {
            queries.push(["version=", &version.to_string()].concat());
        }

        (!queries.is_empty()).then(|| ["?", &queries.join("&")].concat()).unwrap_or_default()
    }
}

/// Discharge Plan with additional data
#[derive(Clone, Debug, Default, Demo, Deserialize, Serialize, FromRow, ToSchema)]
#[schema(example = json!(DischargePlan::demo()))]
pub struct DischargePlan {
    #[Demo(value = "1")]
    pub dc_plan_id: u32,
    #[Demo(value = "1")]
    pub dx_id: u32,
    #[Demo(value = r#"Some(datetime!(2023-12-31 23:59:59))"#)]
    pub dc_datetime: Option<PrimitiveDateTime>,
    #[Demo(value = r#"Some(String::from("Y"))"#)]
    pub dc_type_ok: Option<String>,
    #[Demo(value = r#"Some(String::from("Y"))"#)]
    pub dc_type_refer: Option<String>,
    #[Demo(value = r#"Some(String::from("Escape"))"#)]
    pub dc_type_other: Option<String>,
    #[Demo(value = r#"Some(String::from("Symptom"))"#)]
    pub dc_symptom: Option<String>,
    #[Demo(value = r#"Some(String::from("Y"))"#)]
    pub inst_none: Option<String>,
    #[Demo(value = r#"Some(String::from("Y"))"#)]
    pub inst_foley: Option<String>,
    #[Demo(value = r#"Some(String::from("Y"))"#)]
    pub inst_ett: Option<String>,
    #[Demo(value = r#"Some(String::from("Y"))"#)]
    pub inst_tt: Option<String>,
    #[Demo(value = r#"Some(String::from("Y"))"#)]
    pub inst_ng: Option<String>,
    #[Demo(value = r#"Some(String::from("Instrument"))"#)]
    pub inst_other: Option<String>,
    #[Demo(value = r#"Some(String::from("Y"))"#)]
    pub with_drug: Option<String>,
    #[Demo(value = r#"Some(String::from("Y"))"#)]
    pub with_appoint: Option<String>,
    #[Demo(value = r#"Some(String::from("Y"))"#)]
    pub with_cert: Option<String>,
    #[Demo(value = r#"Some(String::from("Crutches"))"#)]
    pub with_other: Option<String>,
    #[Demo(value = "Some(date!(2023-12-31))")]
    pub appoint_date: Option<Date>,
    #[Demo(value = "Some(time!(23:59:59))")]
    pub appoint_time: Option<Time>,
    #[Demo(value = r#"Some(String::from("OPD"))"#)]
    pub appoint_place: Option<String>,
    #[Demo(value = r#"Some(String::from("F/U"))"#)]
    pub appoint_for: Option<String>,
    #[Demo(value = r#"Some(String::from("Z Hospital"))"#)]
    pub refer_to: Option<String>,
    #[Demo(value = r#"Some(String::from("Other dx text"))"#)]
    pub dx_text: Option<String>,
    #[Demo(value = r#"Some(String::from("Y"))"#)]
    pub dx_patient_ok: Option<String>,
    #[Demo(value = r#"Some(String::from("Y"))"#)]
    pub dx_relatives_ok: Option<String>,
    #[Demo(value = r#"Some(String::from("Blind Deaf"))"#)]
    pub dx_other: Option<String>,
    #[Demo(value = r#"Some(String::from("007"))"#)]
    pub dx_doctor: Option<String>,
    #[Demo(value = r#"Some(datetime!(2023-12-31 23:59:59))"#)]
    pub dx_datetime: Option<PrimitiveDateTime>,
    #[Demo(value = r#"Some(String::from("Other med text"))"#)]
    pub med_text: Option<String>,
    #[Demo(value = r#"Some(String::from("Y"))"#)]
    pub med_patient_ok: Option<String>,
    #[Demo(value = r#"Some(String::from("Y"))"#)]
    pub med_relatives_ok: Option<String>,
    #[Demo(value = r#"Some(String::from("Blind Deaf"))"#)]
    pub med_other: Option<String>,
    #[Demo(value = r#"Some(String::from("007"))"#)]
    pub med_doctor: Option<String>,
    #[Demo(value = r#"Some(datetime!(2023-12-31 23:59:59))"#)]
    pub med_datetime: Option<PrimitiveDateTime>,
    #[Demo(value = r#"Some(String::from("Other env text"))"#)]
    pub env_text: Option<String>,
    #[Demo(value = r#"Some(String::from("Y"))"#)]
    pub env_patient_ok: Option<String>,
    #[Demo(value = r#"Some(String::from("Y"))"#)]
    pub env_relatives_ok: Option<String>,
    #[Demo(value = r#"Some(String::from("Blind Deaf"))"#)]
    pub env_other: Option<String>,
    #[Demo(value = r#"Some(String::from("007"))"#)]
    pub env_doctor: Option<String>,
    #[Demo(value = r#"Some(datetime!(2023-12-31 23:59:59))"#)]
    pub env_datetime: Option<PrimitiveDateTime>,
    #[Demo(value = r#"Some(String::from("Other tx text"))"#)]
    pub tx_text: Option<String>,
    #[Demo(value = r#"Some(String::from("Y"))"#)]
    pub tx_patient_ok: Option<String>,
    #[Demo(value = r#"Some(String::from("Y"))"#)]
    pub tx_relatives_ok: Option<String>,
    #[Demo(value = r#"Some(String::from("Blind Deaf"))"#)]
    pub tx_other: Option<String>,
    #[Demo(value = r#"Some(String::from("007"))"#)]
    pub tx_doctor: Option<String>,
    #[Demo(value = r#"Some(datetime!(2023-12-31 23:59:59))"#)]
    pub tx_datetime: Option<PrimitiveDateTime>,
    #[Demo(value = r#"Some(String::from("Other health text"))"#)]
    pub health_text: Option<String>,
    #[Demo(value = r#"Some(String::from("Y"))"#)]
    pub health_patient_ok: Option<String>,
    #[Demo(value = r#"Some(String::from("Y"))"#)]
    pub health_relatives_ok: Option<String>,
    #[Demo(value = r#"Some(String::from("Blind Deaf"))"#)]
    pub health_other: Option<String>,
    #[Demo(value = r#"Some(String::from("007"))"#)]
    pub health_doctor: Option<String>,
    #[Demo(value = r#"Some(datetime!(2023-12-31 23:59:59))"#)]
    pub health_datetime: Option<PrimitiveDateTime>,
    #[Demo(value = r#"Some(String::from("Other out text"))"#)]
    pub out_text: Option<String>,
    #[Demo(value = r#"Some(String::from("Y"))"#)]
    pub out_patient_ok: Option<String>,
    #[Demo(value = r#"Some(String::from("Y"))"#)]
    pub out_relatives_ok: Option<String>,
    #[Demo(value = r#"Some(String::from("Blind Deaf"))"#)]
    pub out_other: Option<String>,
    #[Demo(value = r#"Some(String::from("007"))"#)]
    pub out_doctor: Option<String>,
    #[Demo(value = r#"Some(datetime!(2023-12-31 23:59:59))"#)]
    pub out_datetime: Option<PrimitiveDateTime>,
    #[Demo(value = r#"Some(String::from("Other diet text"))"#)]
    pub diet_text: Option<String>,
    #[Demo(value = r#"Some(String::from("Y"))"#)]
    pub diet_patient_ok: Option<String>,
    #[Demo(value = r#"Some(String::from("Y"))"#)]
    pub diet_relatives_ok: Option<String>,
    #[Demo(value = r#"Some(String::from("Blind Deaf"))"#)]
    pub diet_other: Option<String>,
    #[Demo(value = r#"Some(String::from("007"))"#)]
    pub diet_doctor: Option<String>,
    #[Demo(value = r#"Some(datetime!(2023-12-31 23:59:59))"#)]
    pub diet_datetime: Option<PrimitiveDateTime>,
    #[Demo(value = "1")]
    pub version: i32,

    #[Demo(value = r#"String::from("UTI")"#)]
    pub dx_name: String,
    #[Demo(value = r#"Some(String::from("Infection"))"#)]
    pub dx_knowledge: Option<String>,
    #[Demo(value = r#"Some(String::from("Dysuria"))"#)]
    pub dx_revisit: Option<String>,
    #[Demo(value = r#"Some(String::from("Clean"))"#)]
    pub dx_prevention: Option<String>,
    #[Demo(value = r#"Some(String::from("1^Take Regulary|2^Check Expired"))"#)]
    pub meds: Option<String>,
    #[Demo(value = r#"Some(String::from("1^Clean Foot|2^Not Wet"))"#)]
    pub envs: Option<String>,
    #[Demo(value = r#"Some(String::from("1^Pain Killer|2^Antibiotic"))"#)]
    pub txs: Option<String>,
    #[Demo(value = r#"Some(String::from("1^Clean Food|2^Cooked"))"#)]
    pub diets: Option<String>,

    #[Demo(value = r#"Some(String::from("Dr.Doctor"))"#)]
    pub dx_user_name: Option<String>,
    #[Demo(value = r#"Some(String::from("นายแพทย์"))"#)]
    pub dx_entryposition: Option<String>,
    #[Demo(value = r#"Some(String::from("ว.11111"))"#)]
    pub dx_licenseno: Option<String>,

    #[Demo(value = r#"Some(String::from("Dr.Doctor"))"#)]
    pub med_user_name: Option<String>,
    #[Demo(value = r#"Some(String::from("นายแพทย์"))"#)]
    pub med_entryposition: Option<String>,
    #[Demo(value = r#"Some(String::from("ว.11111"))"#)]
    pub med_licenseno: Option<String>,

    #[Demo(value = r#"Some(String::from("Dr.Doctor"))"#)]
    pub env_user_name: Option<String>,
    #[Demo(value = r#"Some(String::from("นายแพทย์"))"#)]
    pub env_entryposition: Option<String>,
    #[Demo(value = r#"Some(String::from("ว.11111"))"#)]
    pub env_licenseno: Option<String>,

    #[Demo(value = r#"Some(String::from("Dr.Doctor"))"#)]
    pub tx_user_name: Option<String>,
    #[Demo(value = r#"Some(String::from("นายแพทย์"))"#)]
    pub tx_entryposition: Option<String>,
    #[Demo(value = r#"Some(String::from("ว.11111"))"#)]
    pub tx_licenseno: Option<String>,

    #[Demo(value = r#"Some(String::from("Dr.Doctor"))"#)]
    pub health_user_name: Option<String>,
    #[Demo(value = r#"Some(String::from("นายแพทย์"))"#)]
    pub health_entryposition: Option<String>,
    #[Demo(value = r#"Some(String::from("ว.11111"))"#)]
    pub health_licenseno: Option<String>,

    #[Demo(value = r#"Some(String::from("Dr.Doctor"))"#)]
    pub out_user_name: Option<String>,
    #[Demo(value = r#"Some(String::from("นายแพทย์"))"#)]
    pub out_entryposition: Option<String>,
    #[Demo(value = r#"Some(String::from("ว.11111"))"#)]
    pub out_licenseno: Option<String>,

    #[Demo(value = r#"Some(String::from("Dr.Doctor"))"#)]
    pub diet_user_name: Option<String>,
    #[Demo(value = r#"Some(String::from("นายแพทย์"))"#)]
    pub diet_entryposition: Option<String>,
    #[Demo(value = r#"Some(String::from("ว.11111"))"#)]
    pub diet_licenseno: Option<String>,
}

impl DischargePlan {
    /// GET `EndPoint::IpdDcPlanAn`
    pub async fn call_api_get_ipd(an: &str, app: Rc<AppState>) -> Result<Vec<Self>, AppError> {
        Self::get(&[&EndPoint::IpdDcPlanAn.base(), an].concat(), app).await
    }

    /// GET `EndPoint::OpdErDcPlanId`
    pub async fn call_api_get_opd_er(opd_er_order_master_id: u32, app: Rc<AppState>) -> Result<Vec<Self>, AppError> {
        Self::get(&[EndPoint::OpdErDcPlanId.base(), opd_er_order_master_id.to_string()].concat(), app).await
    }

    async fn get(path: &str, app: Rc<AppState>) -> Result<Vec<Self>, AppError> {
        match fetch_json_api(path, "GET", None, app).await {
            Ok((response, true)) => {
                let response: Vec<Self> = serde_wasm_bindgen::from_value(response).map_err(|e| Source::SerdeWasm.to_teapot_error(e, "Fetch DischargePlan"))?;
                Ok(response)
            }
            Ok((app_error, false)) => {
                let error: AppError = serde_wasm_bindgen::from_value(app_error).map_err(|e| Source::SerdeWasm.to_teapot_error(e, "Fetch DischargePlan"))?;
                Err(error)
            }
            Err(e) => Err(Source::Js.to_teapot_error(e.dyn_ref::<JsString>().map(|s| s.into()).unwrap_or(String::from("fetch error")), "Fetch Json")),
        }
    }

    /// DELETE `EndPoint::IpdDcPlanAn`
    pub async fn call_api_delete_ipd(an: &str, params: &DischargePlanParams, app: Rc<AppState>) -> Result<Vec<ExecuteResponse>, AppError> {
        execute_fetch_vec(&[&EndPoint::IpdDcPlanAn.base(), an, &params.query_string()].concat(), "DELETE", None, app).await
    }

    /// DELETE `EndPoint::OpdErDcPlanId`
    pub async fn call_api_delete_opd_er(opd_er_order_master_id: u32, params: &DischargePlanParams, app: Rc<AppState>) -> Result<Vec<ExecuteResponse>, AppError> {
        execute_fetch_vec(
            &[EndPoint::OpdErDcPlanId.base(), opd_er_order_master_id.to_string(), params.query_string()].concat(),
            "DELETE",
            None,
            app,
        )
        .await
    }

    pub fn is_all_signed(&self) -> bool {
        self.dx_doctor.is_some()
            && self.med_doctor.is_some()
            && self.env_doctor.is_some()
            && self.tx_doctor.is_some()
            && self.health_doctor.is_some()
            && self.out_doctor.is_some()
            && self.diet_doctor.is_some()
    }
}

/// Dischar Plan for save
#[derive(Clone, Demo, Deserialize, Serialize, MySqlBinder, ToSchema)]
#[schema(example = json!(DischargePlanSave::demo()))]
pub struct DischargePlanSave {
    #[Demo(value = "1")]
    pub dc_plan_id: u32,
    #[Demo(value = "1")]
    pub dx_id: u32,
    #[Demo(value = r#"Some(datetime!(2023-12-31 23:59:59))"#)]
    pub dc_datetime: Option<PrimitiveDateTime>,
    #[Demo(value = r#"Some(String::from("Y"))"#)]
    pub dc_type_ok: Option<String>,
    #[Demo(value = r#"Some(String::from("Y"))"#)]
    pub dc_type_refer: Option<String>,
    #[Demo(value = r#"Some(String::from("Escape"))"#)]
    pub dc_type_other: Option<String>,
    #[Demo(value = r#"Some(String::from("Symptom"))"#)]
    pub dc_symptom: Option<String>,
    #[Demo(value = r#"Some(String::from("Y"))"#)]
    pub inst_none: Option<String>,
    #[Demo(value = r#"Some(String::from("Y"))"#)]
    pub inst_foley: Option<String>,
    #[Demo(value = r#"Some(String::from("Y"))"#)]
    pub inst_ett: Option<String>,
    #[Demo(value = r#"Some(String::from("Y"))"#)]
    pub inst_tt: Option<String>,
    #[Demo(value = r#"Some(String::from("Y"))"#)]
    pub inst_ng: Option<String>,
    #[Demo(value = r#"Some(String::from("Instrument"))"#)]
    pub inst_other: Option<String>,
    #[Demo(value = r#"Some(String::from("Y"))"#)]
    pub with_drug: Option<String>,
    #[Demo(value = r#"Some(String::from("Y"))"#)]
    pub with_appoint: Option<String>,
    #[Demo(value = r#"Some(String::from("Y"))"#)]
    pub with_cert: Option<String>,
    #[Demo(value = r#"Some(String::from("Crutches"))"#)]
    pub with_other: Option<String>,
    #[Demo(value = "Some(date!(2023-12-31))")]
    pub appoint_date: Option<Date>,
    #[Demo(value = "Some(time!(23:59:59))")]
    pub appoint_time: Option<Time>,
    #[Demo(value = r#"Some(String::from("OPD"))"#)]
    pub appoint_place: Option<String>,
    #[Demo(value = r#"Some(String::from("F/U"))"#)]
    pub appoint_for: Option<String>,
    #[Demo(value = r#"Some(String::from("Z Hospital"))"#)]
    pub refer_to: Option<String>,
    #[Demo(value = r#"Some(String::from("Other dx text"))"#)]
    pub dx_text: Option<String>,
    #[Demo(value = r#"Some(String::from("Y"))"#)]
    pub dx_patient_ok: Option<String>,
    #[Demo(value = r#"Some(String::from("Y"))"#)]
    pub dx_relatives_ok: Option<String>,
    #[Demo(value = r#"Some(String::from("Blind Deaf"))"#)]
    pub dx_other: Option<String>,
    #[Demo(value = r#"Some(String::from("007"))"#)]
    pub dx_doctor: Option<String>,
    #[Demo(value = r#"Some(datetime!(2023-12-31 23:59:59))"#)]
    pub dx_datetime: Option<PrimitiveDateTime>,
    #[Demo(value = r#"Some(String::from("Other med text"))"#)]
    pub med_text: Option<String>,
    #[Demo(value = r#"Some(String::from("Y"))"#)]
    pub med_patient_ok: Option<String>,
    #[Demo(value = r#"Some(String::from("Y"))"#)]
    pub med_relatives_ok: Option<String>,
    #[Demo(value = r#"Some(String::from("Blind Deaf"))"#)]
    pub med_other: Option<String>,
    #[Demo(value = r#"Some(String::from("007"))"#)]
    pub med_doctor: Option<String>,
    #[Demo(value = r#"Some(datetime!(2023-12-31 23:59:59))"#)]
    pub med_datetime: Option<PrimitiveDateTime>,
    #[Demo(value = r#"Some(String::from("Other env text"))"#)]
    pub env_text: Option<String>,
    #[Demo(value = r#"Some(String::from("Y"))"#)]
    pub env_patient_ok: Option<String>,
    #[Demo(value = r#"Some(String::from("Y"))"#)]
    pub env_relatives_ok: Option<String>,
    #[Demo(value = r#"Some(String::from("Blind Deaf"))"#)]
    pub env_other: Option<String>,
    #[Demo(value = r#"Some(String::from("007"))"#)]
    pub env_doctor: Option<String>,
    #[Demo(value = r#"Some(datetime!(2023-12-31 23:59:59))"#)]
    pub env_datetime: Option<PrimitiveDateTime>,
    #[Demo(value = r#"Some(String::from("Other tx text"))"#)]
    pub tx_text: Option<String>,
    #[Demo(value = r#"Some(String::from("Y"))"#)]
    pub tx_patient_ok: Option<String>,
    #[Demo(value = r#"Some(String::from("Y"))"#)]
    pub tx_relatives_ok: Option<String>,
    #[Demo(value = r#"Some(String::from("Blind Deaf"))"#)]
    pub tx_other: Option<String>,
    #[Demo(value = r#"Some(String::from("007"))"#)]
    pub tx_doctor: Option<String>,
    #[Demo(value = r#"Some(datetime!(2023-12-31 23:59:59))"#)]
    pub tx_datetime: Option<PrimitiveDateTime>,
    #[Demo(value = r#"Some(String::from("Other health text"))"#)]
    pub health_text: Option<String>,
    #[Demo(value = r#"Some(String::from("Y"))"#)]
    pub health_patient_ok: Option<String>,
    #[Demo(value = r#"Some(String::from("Y"))"#)]
    pub health_relatives_ok: Option<String>,
    #[Demo(value = r#"Some(String::from("Blind Deaf"))"#)]
    pub health_other: Option<String>,
    #[Demo(value = r#"Some(String::from("007"))"#)]
    pub health_doctor: Option<String>,
    #[Demo(value = r#"Some(datetime!(2023-12-31 23:59:59))"#)]
    pub health_datetime: Option<PrimitiveDateTime>,
    #[Demo(value = r#"Some(String::from("Other out text"))"#)]
    pub out_text: Option<String>,
    #[Demo(value = r#"Some(String::from("Y"))"#)]
    pub out_patient_ok: Option<String>,
    #[Demo(value = r#"Some(String::from("Y"))"#)]
    pub out_relatives_ok: Option<String>,
    #[Demo(value = r#"Some(String::from("Blind Deaf"))"#)]
    pub out_other: Option<String>,
    #[Demo(value = r#"Some(String::from("007"))"#)]
    pub out_doctor: Option<String>,
    #[Demo(value = r#"Some(datetime!(2023-12-31 23:59:59))"#)]
    pub out_datetime: Option<PrimitiveDateTime>,
    #[Demo(value = r#"Some(String::from("Other diet text"))"#)]
    pub diet_text: Option<String>,
    #[Demo(value = r#"Some(String::from("Y"))"#)]
    pub diet_patient_ok: Option<String>,
    #[Demo(value = r#"Some(String::from("Y"))"#)]
    pub diet_relatives_ok: Option<String>,
    #[Demo(value = r#"Some(String::from("Blind Deaf"))"#)]
    pub diet_other: Option<String>,
    #[Demo(value = r#"Some(String::from("007"))"#)]
    pub diet_doctor: Option<String>,
    #[Demo(value = r#"Some(datetime!(2023-12-31 23:59:59))"#)]
    pub diet_datetime: Option<PrimitiveDateTime>,

    #[sqlx_binder(skip)]
    #[Demo(value = "1")]
    pub version: i32,
    #[sqlx_binder(skip)]
    #[Demo(value = "vec![1,2]")]
    pub med_ids: Vec<u32>,
    #[sqlx_binder(skip)]
    #[Demo(value = "vec![1,2]")]
    pub env_ids: Vec<u32>,
    #[sqlx_binder(skip)]
    #[Demo(value = "vec![1,2]")]
    pub tx_ids: Vec<u32>,
    #[sqlx_binder(skip)]
    #[Demo(value = "vec![1,2]")]
    pub diet_ids: Vec<u32>,
}

impl DischargePlanSave {
    /// POST `EndPoint::IpdDcPlanAn`
    pub async fn call_api_post_ipd(&self, an: &str, app: Rc<AppState>) -> Result<(u32, Vec<ExecuteResponse>), AppError> {
        let body_json = serde_json::to_string(self).map_err(|e| Source::SerdeJson.to_teapot_error(e, "Send DischargePlanSave"))?;
        let body = serde_wasm_bindgen::to_value(&body_json).map_err(|e| Source::SerdeWasm.to_teapot_error(e, "Send DischargePlanSave"))?;
        execute_fetch_vec_with_u32(&[&EndPoint::IpdDcPlanAn.base(), an].concat(), "POST", Some(&body), app).await
    }

    /// POST `EndPoint::OpdErDcPlanId`
    pub async fn call_api_post_opd_er(&self, opd_er_order_master_id: u32, app: Rc<AppState>) -> Result<(u32, Vec<ExecuteResponse>), AppError> {
        let body_json = serde_json::to_string(self).map_err(|e| Source::SerdeJson.to_teapot_error(e, "Send DischargePlanSave"))?;
        let body = serde_wasm_bindgen::to_value(&body_json).map_err(|e| Source::SerdeWasm.to_teapot_error(e, "Send DischargePlanSave"))?;
        execute_fetch_vec_with_u32(&[EndPoint::OpdErDcPlanId.base(), opd_er_order_master_id.to_string()].concat(), "POST", Some(&body), app).await
    }
}

#[derive(Debug, Demo, Deserialize, Serialize, MySqlBinder, FromRow)]
pub struct DischargePlanOnly {
    #[Demo(value = "1")]
    pub dc_plan_id: u32,
    #[Demo(value = "1")]
    pub dx_id: u32,
    #[Demo(value = r#"Some(String::from("Y"))"#)]
    pub dc_type_ok: Option<String>,
    #[Demo(value = r#"Some(String::from("Y"))"#)]
    pub dc_type_refer: Option<String>,
    #[Demo(value = r#"Some(String::from("Escape"))"#)]
    pub dc_type_other: Option<String>,
    #[Demo(value = r#"Some(String::from("Symptom"))"#)]
    pub dc_symptom: Option<String>,
    #[Demo(value = r#"Some(String::from("Y"))"#)]
    pub inst_none: Option<String>,
    #[Demo(value = r#"Some(String::from("Y"))"#)]
    pub inst_foley: Option<String>,
    #[Demo(value = r#"Some(String::from("Y"))"#)]
    pub inst_ett: Option<String>,
    #[Demo(value = r#"Some(String::from("Y"))"#)]
    pub inst_tt: Option<String>,
    #[Demo(value = r#"Some(String::from("Y"))"#)]
    pub inst_ng: Option<String>,
    #[Demo(value = r#"Some(String::from("Instrument"))"#)]
    pub inst_other: Option<String>,
    #[Demo(value = r#"Some(String::from("Y"))"#)]
    pub with_drug: Option<String>,
    #[Demo(value = r#"Some(String::from("Y"))"#)]
    pub with_appoint: Option<String>,
    #[Demo(value = r#"Some(String::from("Y"))"#)]
    pub with_cert: Option<String>,
    #[Demo(value = r#"Some(String::from("Crutches"))"#)]
    pub with_other: Option<String>,
    #[Demo(value = "Some(date!(2023-12-31))")]
    pub appoint_date: Option<Date>,
    #[Demo(value = "Some(time!(23:59:59))")]
    pub appoint_time: Option<Time>,
    #[Demo(value = r#"Some(String::from("OPD"))"#)]
    pub appoint_place: Option<String>,
    #[Demo(value = r#"Some(String::from("F/U"))"#)]
    pub appoint_for: Option<String>,
    #[Demo(value = r#"Some(String::from("Z Hospital"))"#)]
    pub refer_to: Option<String>,
    #[Demo(value = r#"Some(String::from("Other dx text"))"#)]
    pub dx_text: Option<String>,
    #[Demo(value = r#"Some(String::from("Y"))"#)]
    pub dx_patient_ok: Option<String>,
    #[Demo(value = r#"Some(String::from("Y"))"#)]
    pub dx_relatives_ok: Option<String>,
    #[Demo(value = r#"Some(String::from("Blind Deaf"))"#)]
    pub dx_other: Option<String>,
    #[Demo(value = r#"Some(String::from("007"))"#)]
    pub dx_doctor: Option<String>,
    #[Demo(value = r#"Some(datetime!(2023-12-31 23:59:59))"#)]
    pub dx_datetime: Option<PrimitiveDateTime>,
    #[Demo(value = r#"Some(String::from("Other med text"))"#)]
    pub med_text: Option<String>,
    #[Demo(value = r#"Some(String::from("Y"))"#)]
    pub med_patient_ok: Option<String>,
    #[Demo(value = r#"Some(String::from("Y"))"#)]
    pub med_relatives_ok: Option<String>,
    #[Demo(value = r#"Some(String::from("Blind Deaf"))"#)]
    pub med_other: Option<String>,
    #[Demo(value = r#"Some(String::from("007"))"#)]
    pub med_doctor: Option<String>,
    #[Demo(value = r#"Some(datetime!(2023-12-31 23:59:59))"#)]
    pub med_datetime: Option<PrimitiveDateTime>,
    #[Demo(value = r#"Some(String::from("Other env text"))"#)]
    pub env_text: Option<String>,
    #[Demo(value = r#"Some(String::from("Y"))"#)]
    pub env_patient_ok: Option<String>,
    #[Demo(value = r#"Some(String::from("Y"))"#)]
    pub env_relatives_ok: Option<String>,
    #[Demo(value = r#"Some(String::from("Blind Deaf"))"#)]
    pub env_other: Option<String>,
    #[Demo(value = r#"Some(String::from("007"))"#)]
    pub env_doctor: Option<String>,
    #[Demo(value = r#"Some(datetime!(2023-12-31 23:59:59))"#)]
    pub env_datetime: Option<PrimitiveDateTime>,
    #[Demo(value = r#"Some(String::from("Other tx text"))"#)]
    pub tx_text: Option<String>,
    #[Demo(value = r#"Some(String::from("Y"))"#)]
    pub tx_patient_ok: Option<String>,
    #[Demo(value = r#"Some(String::from("Y"))"#)]
    pub tx_relatives_ok: Option<String>,
    #[Demo(value = r#"Some(String::from("Blind Deaf"))"#)]
    pub tx_other: Option<String>,
    #[Demo(value = r#"Some(String::from("007"))"#)]
    pub tx_doctor: Option<String>,
    #[Demo(value = r#"Some(datetime!(2023-12-31 23:59:59))"#)]
    pub tx_datetime: Option<PrimitiveDateTime>,
    #[Demo(value = r#"Some(String::from("Other health text"))"#)]
    pub health_text: Option<String>,
    #[Demo(value = r#"Some(String::from("Y"))"#)]
    pub health_patient_ok: Option<String>,
    #[Demo(value = r#"Some(String::from("Y"))"#)]
    pub health_relatives_ok: Option<String>,
    #[Demo(value = r#"Some(String::from("Blind Deaf"))"#)]
    pub health_other: Option<String>,
    #[Demo(value = r#"Some(String::from("007"))"#)]
    pub health_doctor: Option<String>,
    #[Demo(value = r#"Some(datetime!(2023-12-31 23:59:59))"#)]
    pub health_datetime: Option<PrimitiveDateTime>,
    #[Demo(value = r#"Some(String::from("Other out text"))"#)]
    pub out_text: Option<String>,
    #[Demo(value = r#"Some(String::from("Y"))"#)]
    pub out_patient_ok: Option<String>,
    #[Demo(value = r#"Some(String::from("Y"))"#)]
    pub out_relatives_ok: Option<String>,
    #[Demo(value = r#"Some(String::from("Blind Deaf"))"#)]
    pub out_other: Option<String>,
    #[Demo(value = r#"Some(String::from("007"))"#)]
    pub out_doctor: Option<String>,
    #[Demo(value = r#"Some(datetime!(2023-12-31 23:59:59))"#)]
    pub out_datetime: Option<PrimitiveDateTime>,
    #[Demo(value = r#"Some(String::from("Other diet text"))"#)]
    pub diet_text: Option<String>,
    #[Demo(value = r#"Some(String::from("Y"))"#)]
    pub diet_patient_ok: Option<String>,
    #[Demo(value = r#"Some(String::from("Y"))"#)]
    pub diet_relatives_ok: Option<String>,
    #[Demo(value = r#"Some(String::from("Blind Deaf"))"#)]
    pub diet_other: Option<String>,
    #[Demo(value = r#"Some(String::from("007"))"#)]
    pub diet_doctor: Option<String>,
    #[Demo(value = r#"Some(datetime!(2023-12-31 23:59:59))"#)]
    pub diet_datetime: Option<PrimitiveDateTime>,
    #[Demo(value = r#"String::from("user")"#)]
    pub create_user: String,
    #[Demo(value = "datetime!(2023-12-31 23:59:59)")]
    pub create_datetime: PrimitiveDateTime,
    #[Demo(value = r#"String::from("user")"#)]
    pub update_user: String,
    #[Demo(value = "datetime!(2023-12-31 23:59:59)")]
    pub update_datetime: PrimitiveDateTime,
    #[Demo(value = "1")]
    pub version: i32,
    #[sqlx(skip)]
    #[sqlx_binder(skip)]
    #[Demo(value = "vec![DischargePlanMedItemOnly::demo()]")]
    pub dc_plan_med_items: Vec<DischargePlanMedItemOnly>,
    #[sqlx(skip)]
    #[sqlx_binder(skip)]
    #[Demo(value = "vec![DischargePlanEnvItemOnly::demo()]")]
    pub dc_plan_env_items: Vec<DischargePlanEnvItemOnly>,
    #[sqlx(skip)]
    #[sqlx_binder(skip)]
    #[Demo(value = "vec![DischargePlanTxItemOnly::demo()]")]
    pub dc_plan_tx_items: Vec<DischargePlanTxItemOnly>,
    #[sqlx(skip)]
    #[sqlx_binder(skip)]
    #[Demo(value = "vec![DischargePlanDietItemOnly::demo()]")]
    pub dc_plan_diet_items: Vec<DischargePlanDietItemOnly>,
}

impl PartialEq for DischargePlanOnly {
    fn eq(&self, other: &Self) -> bool {
        // self.dc_plan_id == other.dc_plan_id &&
        // self.dx_id == other.dx_id &&
        self.dc_type_ok == other.dc_type_ok
            && self.dc_type_refer == other.dc_type_refer
            && self.dc_type_other == other.dc_type_other
            && self.dc_symptom == other.dc_symptom
            && self.inst_none == other.inst_none
            && self.inst_foley == other.inst_foley
            && self.inst_ett == other.inst_ett
            && self.inst_tt == other.inst_tt
            && self.inst_ng == other.inst_ng
            && self.inst_other == other.inst_other
            && self.with_drug == other.with_drug
            && self.with_appoint == other.with_appoint
            && self.with_cert == other.with_cert
            && self.with_other == other.with_other
            && self.appoint_date == other.appoint_date
            && self.appoint_time == other.appoint_time
            && self.appoint_place == other.appoint_place
            && self.appoint_for == other.appoint_for
            && self.refer_to == other.refer_to
            && self.dx_text == other.dx_text
            && self.dx_patient_ok == other.dx_patient_ok
            && self.dx_relatives_ok == other.dx_relatives_ok
            && self.dx_other == other.dx_other
            && self.dx_doctor == other.dx_doctor
            && self.dx_datetime == other.dx_datetime
            && self.med_text == other.med_text
            && self.med_patient_ok == other.med_patient_ok
            && self.med_relatives_ok == other.med_relatives_ok
            && self.med_other == other.med_other
            && self.med_doctor == other.med_doctor
            && self.med_datetime == other.med_datetime
            && self.env_text == other.env_text
            && self.env_patient_ok == other.env_patient_ok
            && self.env_relatives_ok == other.env_relatives_ok
            && self.env_other == other.env_other
            && self.env_doctor == other.env_doctor
            && self.env_datetime == other.env_datetime
            && self.tx_text == other.tx_text
            && self.tx_patient_ok == other.tx_patient_ok
            && self.tx_relatives_ok == other.tx_relatives_ok
            && self.tx_other == other.tx_other
            && self.tx_doctor == other.tx_doctor
            && self.tx_datetime == other.tx_datetime
            && self.health_text == other.health_text
            && self.health_patient_ok == other.health_patient_ok
            && self.health_relatives_ok == other.health_relatives_ok
            && self.health_other == other.health_other
            && self.health_doctor == other.health_doctor
            && self.health_datetime == other.health_datetime
            && self.out_text == other.out_text
            && self.out_patient_ok == other.out_patient_ok
            && self.out_relatives_ok == other.out_relatives_ok
            && self.out_other == other.out_other
            && self.out_doctor == other.out_doctor
            && self.out_datetime == other.out_datetime
            && self.diet_text == other.diet_text
            && self.diet_patient_ok == other.diet_patient_ok
            && self.diet_relatives_ok == other.diet_relatives_ok
            && self.diet_other == other.diet_other
            && self.diet_doctor == other.diet_doctor
            && self.diet_datetime == other.diet_datetime
            && self.create_user == other.create_user
            && self.create_datetime == other.create_datetime
            && self.update_user == other.update_user
            && self.update_datetime == other.update_datetime
            && self.version == other.version
            && if self.dc_plan_med_items.len() == other.dc_plan_med_items.len() {
                self.dc_plan_med_items.iter().zip(other.dc_plan_med_items.iter()).all(|(a, b)| a.eq(b))
            } else {
                false
            }
            && if self.dc_plan_env_items.len() == other.dc_plan_env_items.len() {
                self.dc_plan_env_items.iter().zip(other.dc_plan_env_items.iter()).all(|(a, b)| a.eq(b))
            } else {
                false
            }
            && if self.dc_plan_tx_items.len() == other.dc_plan_tx_items.len() {
                self.dc_plan_tx_items.iter().zip(other.dc_plan_tx_items.iter()).all(|(a, b)| a.eq(b))
            } else {
                false
            }
            && if self.dc_plan_diet_items.len() == other.dc_plan_diet_items.len() {
                self.dc_plan_diet_items.iter().zip(other.dc_plan_diet_items.iter()).all(|(a, b)| a.eq(b))
            } else {
                false
            }
    }
}

#[derive(Debug, Demo, Deserialize, Serialize, FromRow)]
pub struct DischargePlanMedItemOnly {
    #[Demo(value = "1")]
    pub med_item_id: u32,
    #[Demo(value = "Some(1)")]
    pub dc_plan_id: Option<u32>,
    #[Demo(value = "Some(1)")]
    pub med_id: Option<u32>,
    #[Demo(value = r#"String::from("user")"#)]
    pub create_user: String,
    #[Demo(value = "datetime!(2023-12-31 23:59:59)")]
    pub create_datetime: PrimitiveDateTime,
    #[Demo(value = r#"String::from("user")"#)]
    pub update_user: String,
    #[Demo(value = "datetime!(2023-12-31 23:59:59)")]
    pub update_datetime: PrimitiveDateTime,
    #[Demo(value = "1")]
    pub version: i32,
}

impl PartialEq for DischargePlanMedItemOnly {
    fn eq(&self, other: &Self) -> bool {
        // self.med_item_id == other.med_item_id &&
        // self.dc_plan_id == other.dc_plan_id &&
        self.med_id == other.med_id
            && self.create_user == other.create_user
            && self.create_datetime == other.create_datetime
            && self.update_user == other.update_user
            && self.update_datetime == other.update_datetime
            && self.version == other.version
    }
}

#[derive(Debug, Demo, Deserialize, Serialize, FromRow)]
pub struct DischargePlanEnvItemOnly {
    #[Demo(value = "1")]
    pub env_item_id: u32,
    #[Demo(value = "Some(1)")]
    pub dc_plan_id: Option<u32>,
    #[Demo(value = "Some(1)")]
    pub env_id: Option<u32>,
    #[Demo(value = r#"String::from("user")"#)]
    pub create_user: String,
    #[Demo(value = "datetime!(2023-12-31 23:59:59)")]
    pub create_datetime: PrimitiveDateTime,
    #[Demo(value = r#"String::from("user")"#)]
    pub update_user: String,
    #[Demo(value = "datetime!(2023-12-31 23:59:59)")]
    pub update_datetime: PrimitiveDateTime,
    #[Demo(value = "1")]
    pub version: i32,
}

impl PartialEq for DischargePlanEnvItemOnly {
    fn eq(&self, other: &Self) -> bool {
        // self.env_item_id == other.env_item_id &&
        // self.dc_plan_id == other.dc_plan_id &&
        self.env_id == other.env_id
            && self.create_user == other.create_user
            && self.create_datetime == other.create_datetime
            && self.update_user == other.update_user
            && self.update_datetime == other.update_datetime
            && self.version == other.version
    }
}

#[derive(Debug, Demo, Deserialize, Serialize, FromRow)]
pub struct DischargePlanTxItemOnly {
    #[Demo(value = "1")]
    pub tx_item_id: u32,
    #[Demo(value = "Some(1)")]
    pub dc_plan_id: Option<u32>,
    #[Demo(value = "Some(1)")]
    pub tx_id: Option<u32>,
    #[Demo(value = r#"String::from("user")"#)]
    pub create_user: String,
    #[Demo(value = "datetime!(2023-12-31 23:59:59)")]
    pub create_datetime: PrimitiveDateTime,
    #[Demo(value = r#"String::from("user")"#)]
    pub update_user: String,
    #[Demo(value = "datetime!(2023-12-31 23:59:59)")]
    pub update_datetime: PrimitiveDateTime,
    #[Demo(value = "1")]
    pub version: i32,
}

impl PartialEq for DischargePlanTxItemOnly {
    fn eq(&self, other: &Self) -> bool {
        // self.tx_item_id == other.tx_item_id &&
        // self.dc_plan_id == other.dc_plan_id &&
        self.tx_id == other.tx_id
            && self.create_user == other.create_user
            && self.create_datetime == other.create_datetime
            && self.update_user == other.update_user
            && self.update_datetime == other.update_datetime
            && self.version == other.version
    }
}

#[derive(Debug, Demo, Deserialize, Serialize, FromRow)]
pub struct DischargePlanDietItemOnly {
    #[Demo(value = "1")]
    pub diet_item_id: u32,
    #[Demo(value = "Some(1)")]
    pub dc_plan_id: Option<u32>,
    #[Demo(value = "Some(1)")]
    pub diet_id: Option<u32>,
    #[Demo(value = r#"String::from("user")"#)]
    pub create_user: String,
    #[Demo(value = "datetime!(2023-12-31 23:59:59)")]
    pub create_datetime: PrimitiveDateTime,
    #[Demo(value = r#"String::from("user")"#)]
    pub update_user: String,
    #[Demo(value = "datetime!(2023-12-31 23:59:59)")]
    pub update_datetime: PrimitiveDateTime,
    #[Demo(value = "1")]
    pub version: i32,
}

impl PartialEq for DischargePlanDietItemOnly {
    fn eq(&self, other: &Self) -> bool {
        // self.diet_item_id == other.diet_item_id &&
        // self.dc_plan_id == other.dc_plan_id &&
        self.diet_id == other.diet_id
            && self.create_user == other.create_user
            && self.create_datetime == other.create_datetime
            && self.update_user == other.update_user
            && self.update_datetime == other.update_datetime
            && self.version == other.version
    }
}
