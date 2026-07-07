use derive_demo::Demo;
use js_sys::JsString;
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, types::time::PrimitiveDateTime};
use std::rc::Rc;
use time::macros::datetime;
use utoipa::ToSchema;
use wasm_bindgen::JsCast;

use kphis_util::error::{AppError, Source};

use crate::{app::AppState, endpoint::EndPoint, fetch::fetch_json_api};

/// Operation from HIS
#[derive(Clone, Debug, Demo, Deserialize, Serialize, FromRow, ToSchema)]
#[schema(example = json!(HisOperationAdmit::demo()))]
pub struct HisOperationAdmit {
    #[Demo(value = "1")]
    pub operation_id: i32,
    #[Demo(value = r#"Some(String::from("660001234"))"#)]
    pub an: Option<String>,
    #[Demo(value = r#"Some(String::from("TR (66.32)"))"#)]
    pub name: Option<String>,
    #[Demo(value = r#"Some(String::from("6632"))"#)]
    pub icd9: Option<String>,
    #[Demo(value = "Some(datetime!(2023-12-31 23:59:59))")]
    pub begin_datetime: Option<PrimitiveDateTime>,
    #[Demo(value = "Some(datetime!(2023-12-31 23:59:59))")]
    pub end_datetime: Option<PrimitiveDateTime>,
    #[Demo(value = r#"Some(String::from("Dr.Doctor"))"#)]
    pub doctor_name: Option<String>,
}

impl HisOperationAdmit {
    /// GET `EndPoint::HisOperationAdmitAn`
    pub async fn call_api_get(an: &str, app: Rc<AppState>) -> Result<Vec<Self>, AppError> {
        match fetch_json_api(&[&EndPoint::HisOperationAdmitAn.base(), an].concat(), "GET", None, app).await {
            Ok((response, true)) => {
                let response: Vec<Self> = serde_wasm_bindgen::from_value(response).map_err(|e| Source::SerdeWasm.to_teapot_error(e, "Fetch HisOperationAdmit"))?;
                Ok(response)
            }
            Ok((app_error, false)) => {
                let error: AppError = serde_wasm_bindgen::from_value(app_error).map_err(|e| Source::SerdeWasm.to_teapot_error(e, "Fetch HisOperationAdmit"))?;
                Err(error)
            }
            Err(e) => Err(Source::Js.to_teapot_error(e.dyn_ref::<JsString>().map(|s| s.into()).unwrap_or(String::from("fetch error")), "Fetch Json")),
        }
    }
}

/// MedPlanIpd from HIS
#[derive(Clone, Debug, Demo, Deserialize, Serialize, FromRow, ToSchema)]
#[schema(example = json!(HisMedPlanIpd::demo()))]
pub struct HisMedPlanIpd {
    #[Demo(value = "1")]
    pub med_plan_number: i32,
    #[Demo(value = r#"Some(String::from("1000222"))"#)]
    pub icode: Option<String>,
    #[Demo(value = r#"Some(String::from("C"))"#)]
    pub orderstatus: Option<String>,
    #[Demo(value = "Some(datetime!(2023-12-31 00:00:00))")]
    pub orderdate: Option<PrimitiveDateTime>,
    #[Demo(value = r#"Some(String::from("PARACETAMOL 500 mg. เม็ด"))"#)]
    pub med_name: Option<String>,
    #[Demo(value = r#"Some(String::from("TABLET"))"#)]
    pub dosageform: Option<String>,
    #[Demo(value = r#"Some(String::from("รับประทาน ครั้งละ 1 เม็ด วันละ 1 เวลา"))"#)]
    pub drug_usage: Option<String>,
}

impl HisMedPlanIpd {
    /// GET `EndPoint::HisMedPlanIpdAn`
    pub async fn call_api_get(an: &str, app: Rc<AppState>) -> Result<Vec<Self>, AppError> {
        match fetch_json_api(&[&EndPoint::HisMedPlanIpdAn.base(), an].concat(), "GET", None, app).await {
            Ok((response, true)) => {
                let response: Vec<Self> = serde_wasm_bindgen::from_value(response).map_err(|e| Source::SerdeWasm.to_teapot_error(e, "Fetch HisMedPlanIpd"))?;
                Ok(response)
            }
            Ok((app_error, false)) => {
                let error: AppError = serde_wasm_bindgen::from_value(app_error).map_err(|e| Source::SerdeWasm.to_teapot_error(e, "Fetch HisMedPlanIpd"))?;
                Err(error)
            }
            Err(e) => Err(Source::Js.to_teapot_error(e.dyn_ref::<JsString>().map(|s| s.into()).unwrap_or(String::from("fetch error")), "Fetch Json")),
        }
    }
}

/// Summary Diagnosis from HIS
#[derive(Clone, Debug, Demo, Deserialize, Serialize, FromRow, ToSchema)]
#[schema(example = json!(HisIptDiag::demo()))]
pub struct HisIptDiag {
    #[Demo(value = "1")]
    pub ipt_diag_id: i32,
    #[Demo(value = r#"Some(String::from("660001234"))"#)]
    pub an: Option<String>,
    #[Demo(value = r#"Some(String::from("1"))"#)]
    pub diagtype: Option<String>,
    #[Demo(value = r#"Some(String::from("A099)"))"#)]
    pub icd10: Option<String>,
}

impl HisIptDiag {
    /// GET `EndPoint::HisIptDiagAn`
    pub async fn call_api_get(an: &str, app: Rc<AppState>) -> Result<Vec<Self>, AppError> {
        match fetch_json_api(&[&EndPoint::HisIptDiagAn.base(), an].concat(), "GET", None, app).await {
            Ok((response, true)) => {
                let response: Vec<Self> = serde_wasm_bindgen::from_value(response).map_err(|e| Source::SerdeWasm.to_teapot_error(e, "Fetch HisIptDiag"))?;
                Ok(response)
            }
            Ok((app_error, false)) => {
                let error: AppError = serde_wasm_bindgen::from_value(app_error).map_err(|e| Source::SerdeWasm.to_teapot_error(e, "Fetch HisIptDiag"))?;
                Err(error)
            }
            Err(e) => Err(Source::Js.to_teapot_error(e.dyn_ref::<JsString>().map(|s| s.into()).unwrap_or(String::from("fetch error")), "Fetch Json")),
        }
    }
}

/// Summary Procedure from HIS
#[derive(Clone, Debug, Demo, Deserialize, Serialize, FromRow, ToSchema)]
#[schema(example = json!(HisIptDiag::demo()))]
pub struct HisIptOprt {
    #[Demo(value = "1")]
    pub iptoprt_id: i32,
    #[Demo(value = r#"Some(String::from("660001234"))"#)]
    pub an: Option<String>,
    #[Demo(value = r#"Some(String::from("9904)"))"#)]
    pub icd9: Option<String>,
}

impl HisIptOprt {
    /// GET `EndPoint::HisIptOprtAn`
    pub async fn call_api_get(an: &str, app: Rc<AppState>) -> Result<Vec<Self>, AppError> {
        match fetch_json_api(&[&EndPoint::HisIptOprtAn.base(), an].concat(), "GET", None, app).await {
            Ok((response, true)) => {
                let response: Vec<Self> = serde_wasm_bindgen::from_value(response).map_err(|e| Source::SerdeWasm.to_teapot_error(e, "Fetch HisIptOprt"))?;
                Ok(response)
            }
            Ok((app_error, false)) => {
                let error: AppError = serde_wasm_bindgen::from_value(app_error).map_err(|e| Source::SerdeWasm.to_teapot_error(e, "Fetch HisIptOprt"))?;
                Err(error)
            }
            Err(e) => Err(Source::Js.to_teapot_error(e.dyn_ref::<JsString>().map(|s| s.into()).unwrap_or(String::from("fetch error")), "Fetch Json")),
        }
    }
}
