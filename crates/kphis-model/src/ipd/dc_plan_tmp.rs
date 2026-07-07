use derive_demo::Demo;
use js_sys::JsString;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use std::rc::Rc;
use utoipa::{IntoParams, ToSchema};
use wasm_bindgen::JsCast;

use kphis_util::error::{AppError, Source};

use crate::{
    app::AppState,
    endpoint::{EndPoint, QueryString, find_qs},
    fetch::{ExecuteResponse, execute_fetch, fetch_json_api},
};

#[derive(Clone, Default, Deserialize, IntoParams)]
pub struct DcPlanTmpParams {
    pub id: Option<u32>,
}

impl QueryString for DcPlanTmpParams {
    fn from_tuples(params: &[(String, String)]) -> Option<Self> {
        (!params.is_empty()).then(|| Self {
            id: find_qs(params, "id").and_then(|s| s.parse::<u32>().ok()),
        })
    }

    fn query_string(&self) -> String {
        let mut queries = Vec::with_capacity(1);
        if let Some(id) = &self.id {
            queries.push(["id=", &id.to_string()].concat());
        }

        (!queries.is_empty()).then(|| ["?", &queries.join("&")].concat()).unwrap_or_default()
    }
}

/// Nursing Discharge Plan Template : Diagnosis
#[derive(Clone, Demo, Deserialize, Serialize, FromRow, ToSchema)]
#[schema(example = json!(DcPlanTmpDx::demo()))]
pub struct DcPlanTmpDx {
    #[Demo(value = "1")]
    pub dx_id: u32,
    #[Demo(value = r#"Some(String::from("UTI"))"#)]
    pub dx_name: Option<String>,
    #[Demo(value = r#"Some(String::from("Infection of urinary tract"))"#)]
    pub dx_knowledge: Option<String>,
    #[Demo(value = r#"Some(String::from("Dysuria"))"#)]
    pub dx_revisit: Option<String>,
    #[Demo(value = r#"Some(String::from("Clean"))"#)]
    pub dx_prevention: Option<String>,
}

impl DcPlanTmpDx {
    /// GET `EndPoint::IpdDcPlanTmpDx`
    pub async fn call_api_get(params: &DcPlanTmpParams, app: Rc<AppState>) -> Result<Vec<Self>, AppError> {
        match fetch_json_api(&[EndPoint::IpdDcPlanTmpDx.base(), params.clone().query_string()].concat(), "GET", None, app).await {
            Ok((response, true)) => {
                let response: Vec<Self> = serde_wasm_bindgen::from_value(response).map_err(|e| Source::SerdeWasm.to_teapot_error(e, "Fetch DcPlanTmpDx"))?;
                Ok(response)
            }
            Ok((app_error, false)) => {
                let error: AppError = serde_wasm_bindgen::from_value(app_error).map_err(|e| Source::SerdeWasm.to_teapot_error(e, "Fetch DcPlanTmpDx"))?;
                Err(error)
            }
            Err(e) => Err(Source::Js.to_teapot_error(e.dyn_ref::<JsString>().map(|s| s.into()).unwrap_or(String::from("fetch error")), "Fetch Json")),
        }
    }

    /// POST `EndPoint::IpdDcPlanTmpDx`
    pub async fn call_api_post(&self, app: Rc<AppState>) -> Result<ExecuteResponse, AppError> {
        let body_json = serde_json::to_string(self).map_err(|e| Source::SerdeJson.to_teapot_error(e, "Send DcPlanTmpDx"))?;

        let body = serde_wasm_bindgen::to_value(&body_json).map_err(|e| Source::SerdeWasm.to_teapot_error(e, "Send DcPlanTmpDx"))?;

        execute_fetch(&EndPoint::IpdDcPlanTmpDx.base(), "POST", Some(&body), app).await
    }

    /// DELETE `EndPoint::IpdDcPlanTmpDx`
    pub async fn call_api_delete(params: &DcPlanTmpParams, app: Rc<AppState>) -> Result<ExecuteResponse, AppError> {
        execute_fetch(&[EndPoint::IpdDcPlanTmpDx.base(), params.query_string()].concat(), "DELETE", None, app).await
    }
}

/// Nursing Discharge Plan Template : Medication
#[derive(Clone, Demo, Deserialize, Serialize, FromRow, ToSchema)]
#[schema(example = json!(DcPlanTmpMed::demo()))]
pub struct DcPlanTmpMed {
    #[Demo(value = "1")]
    pub med_id: u32,
    #[Demo(value = r#"Some(String::from("Take medication regulary"))"#)]
    pub med_text: Option<String>,
}

impl DcPlanTmpMed {
    /// GET `EndPoint::IpdDcPlanTmpMed`
    pub async fn call_api_get(params: &DcPlanTmpParams, app: Rc<AppState>) -> Result<Vec<Self>, AppError> {
        match fetch_json_api(&[EndPoint::IpdDcPlanTmpMed.base(), params.clone().query_string()].concat(), "GET", None, app).await {
            Ok((response, true)) => {
                let response: Vec<Self> = serde_wasm_bindgen::from_value(response).map_err(|e| Source::SerdeWasm.to_teapot_error(e, "Fetch DcPlanTmpMed"))?;
                Ok(response)
            }
            Ok((app_error, false)) => {
                let error: AppError = serde_wasm_bindgen::from_value(app_error).map_err(|e| Source::SerdeWasm.to_teapot_error(e, "Fetch DcPlanTmpMed"))?;
                Err(error)
            }
            Err(e) => Err(Source::Js.to_teapot_error(e.dyn_ref::<JsString>().map(|s| s.into()).unwrap_or(String::from("fetch error")), "Fetch Json")),
        }
    }

    /// POST `EndPoint::IpdDcPlanTmpMed`
    pub async fn call_api_post(&self, app: Rc<AppState>) -> Result<ExecuteResponse, AppError> {
        let body_json = serde_json::to_string(self).map_err(|e| Source::SerdeJson.to_teapot_error(e, "Send DcPlanTmpMed"))?;

        let body = serde_wasm_bindgen::to_value(&body_json).map_err(|e| Source::SerdeWasm.to_teapot_error(e, "Send DcPlanTmpMed"))?;

        execute_fetch(&EndPoint::IpdDcPlanTmpMed.base(), "POST", Some(&body), app).await
    }

    /// DELETE `EndPoint::IpdDcPlanTmpMed`
    pub async fn call_api_delete(params: &DcPlanTmpParams, app: Rc<AppState>) -> Result<ExecuteResponse, AppError> {
        execute_fetch(&[EndPoint::IpdDcPlanTmpMed.base(), params.query_string()].concat(), "DELETE", None, app).await
    }
}

/// Nursing Discharge Plan Template : Environment
#[derive(Clone, Demo, Deserialize, Serialize, FromRow, ToSchema)]
#[schema(example = json!(DcPlanTmpEnv::demo()))]
pub struct DcPlanTmpEnv {
    #[Demo(value = "1")]
    pub env_id: u32,
    #[Demo(value = r#"Some(String::from("Prevent Germ"))"#)]
    pub env_text: Option<String>,
}

impl DcPlanTmpEnv {
    /// GET `EndPoint::IpdDcPlanTmpEnv`
    pub async fn call_api_get(params: &DcPlanTmpParams, app: Rc<AppState>) -> Result<Vec<Self>, AppError> {
        match fetch_json_api(&[EndPoint::IpdDcPlanTmpEnv.base(), params.clone().query_string()].concat(), "GET", None, app).await {
            Ok((response, true)) => {
                let response: Vec<Self> = serde_wasm_bindgen::from_value(response).map_err(|e| Source::SerdeWasm.to_teapot_error(e, "Fetch DcPlanTmpEnv"))?;
                Ok(response)
            }
            Ok((app_error, false)) => {
                let error: AppError = serde_wasm_bindgen::from_value(app_error).map_err(|e| Source::SerdeWasm.to_teapot_error(e, "Fetch DcPlanTmpEnv"))?;
                Err(error)
            }
            Err(e) => Err(Source::Js.to_teapot_error(e.dyn_ref::<JsString>().map(|s| s.into()).unwrap_or(String::from("fetch error")), "Fetch Json")),
        }
    }

    /// POST `EndPoint::IpdDcPlanTmpEnv`
    pub async fn call_api_post(&self, app: Rc<AppState>) -> Result<ExecuteResponse, AppError> {
        let body_json = serde_json::to_string(self).map_err(|e| Source::SerdeJson.to_teapot_error(e, "Send DcPlanTmpEnv"))?;

        let body = serde_wasm_bindgen::to_value(&body_json).map_err(|e| Source::SerdeWasm.to_teapot_error(e, "Send DcPlanTmpEnv"))?;

        execute_fetch(&EndPoint::IpdDcPlanTmpEnv.base(), "POST", Some(&body), app).await
    }

    /// DELETE `EndPoint::IpdDcPlanTmpEnv`
    pub async fn call_api_delete(params: &DcPlanTmpParams, app: Rc<AppState>) -> Result<ExecuteResponse, AppError> {
        execute_fetch(&[EndPoint::IpdDcPlanTmpEnv.base(), params.query_string()].concat(), "DELETE", None, app).await
    }
}

/// Nursing Discharge Plan Template : Treatment
#[derive(Clone, Demo, Deserialize, Serialize, FromRow, ToSchema)]
#[schema(example = json!(DcPlanTmpTx::demo()))]
pub struct DcPlanTmpTx {
    #[Demo(value = "1")]
    pub tx_id: u32,
    #[Demo(value = r#"Some(String::from("Antibiotics"))"#)]
    pub tx_text: Option<String>,
}

impl DcPlanTmpTx {
    /// GET `EndPoint::IpdDcPlanTmpTx`
    pub async fn call_api_get(params: &DcPlanTmpParams, app: Rc<AppState>) -> Result<Vec<Self>, AppError> {
        match fetch_json_api(&[EndPoint::IpdDcPlanTmpTx.base(), params.clone().query_string()].concat(), "GET", None, app).await {
            Ok((response, true)) => {
                let response: Vec<Self> = serde_wasm_bindgen::from_value(response).map_err(|e| Source::SerdeWasm.to_teapot_error(e, "Fetch DcPlanTmpTx"))?;
                Ok(response)
            }
            Ok((app_error, false)) => {
                let error: AppError = serde_wasm_bindgen::from_value(app_error).map_err(|e| Source::SerdeWasm.to_teapot_error(e, "Fetch DcPlanTmpTx"))?;
                Err(error)
            }
            Err(e) => Err(Source::Js.to_teapot_error(e.dyn_ref::<JsString>().map(|s| s.into()).unwrap_or(String::from("fetch error")), "Fetch Json")),
        }
    }

    /// POST `EndPoint::IpdDcPlanTmpTx`
    pub async fn call_api_post(&self, app: Rc<AppState>) -> Result<ExecuteResponse, AppError> {
        let body_json = serde_json::to_string(self).map_err(|e| Source::SerdeJson.to_teapot_error(e, "Send DcPlanTmpTx"))?;

        let body = serde_wasm_bindgen::to_value(&body_json).map_err(|e| Source::SerdeWasm.to_teapot_error(e, "Send DcPlanTmpTx"))?;

        execute_fetch(&EndPoint::IpdDcPlanTmpTx.base(), "POST", Some(&body), app).await
    }

    /// DELETE `EndPoint::IpdDcPlanTmpTx`
    pub async fn call_api_delete(params: &DcPlanTmpParams, app: Rc<AppState>) -> Result<ExecuteResponse, AppError> {
        execute_fetch(&[EndPoint::IpdDcPlanTmpTx.base(), params.query_string()].concat(), "DELETE", None, app).await
    }
}

/// Nursing Discharge Plan Template : Diet
#[derive(Clone, Demo, Deserialize, Serialize, FromRow, ToSchema)]
#[schema(example = json!(DcPlanTmpDiet::demo()))]
pub struct DcPlanTmpDiet {
    #[Demo(value = "1")]
    pub diet_id: u32,
    #[Demo(value = r#"Some(String::from("Clean food"))"#)]
    pub diet_text: Option<String>,
}

impl DcPlanTmpDiet {
    /// GET `EndPoint::IpdDcPlanTmpDiet`
    pub async fn call_api_get(params: &DcPlanTmpParams, app: Rc<AppState>) -> Result<Vec<Self>, AppError> {
        match fetch_json_api(&[EndPoint::IpdDcPlanTmpDiet.base(), params.clone().query_string()].concat(), "GET", None, app).await {
            Ok((response, true)) => {
                let response: Vec<Self> = serde_wasm_bindgen::from_value(response).map_err(|e| Source::SerdeWasm.to_teapot_error(e, "Fetch DcPlanTmpDiet"))?;
                Ok(response)
            }
            Ok((app_error, false)) => {
                let error: AppError = serde_wasm_bindgen::from_value(app_error).map_err(|e| Source::SerdeWasm.to_teapot_error(e, "Fetch DcPlanTmpDiet"))?;
                Err(error)
            }
            Err(e) => Err(Source::Js.to_teapot_error(e.dyn_ref::<JsString>().map(|s| s.into()).unwrap_or(String::from("fetch error")), "Fetch Json")),
        }
    }

    /// POST `EndPoint::IpdDcPlanTmpDiet`
    pub async fn call_api_post(&self, app: Rc<AppState>) -> Result<ExecuteResponse, AppError> {
        let body_json = serde_json::to_string(self).map_err(|e| Source::SerdeJson.to_teapot_error(e, "Send DcPlanTmpDiet"))?;

        let body = serde_wasm_bindgen::to_value(&body_json).map_err(|e| Source::SerdeWasm.to_teapot_error(e, "Send DcPlanTmpDiet"))?;

        execute_fetch(&EndPoint::IpdDcPlanTmpDiet.base(), "POST", Some(&body), app).await
    }

    /// DELETE `EndPoint::IpdDcPlanTmpDiet`
    pub async fn call_api_delete(params: &DcPlanTmpParams, app: Rc<AppState>) -> Result<ExecuteResponse, AppError> {
        execute_fetch(&[EndPoint::IpdDcPlanTmpDiet.base(), params.query_string()].concat(), "DELETE", None, app).await
    }
}
