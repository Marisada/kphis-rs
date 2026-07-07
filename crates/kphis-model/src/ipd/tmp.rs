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
pub struct TmpParams {
    pub smp_id: Option<u32>,
    pub subgroup: Option<u32>,
    pub id: Option<u32>,
    /// Some(false) is None<br>
    /// Some(true) will not include `subgroup = 0`
    pub strict: Option<bool>,
}

impl QueryString for TmpParams {
    fn from_tuples(params: &[(String, String)]) -> Option<Self> {
        (!params.is_empty()).then(|| Self {
            smp_id: find_qs(params, "smp_id").and_then(|s| s.parse::<u32>().ok()),
            subgroup: find_qs(params, "subgroup").and_then(|s| s.parse::<u32>().ok()),
            id: find_qs(params, "id").and_then(|s| s.parse::<u32>().ok()),
            strict: find_qs(params, "strict").and_then(|s| s.parse::<bool>().ok()),
        })
    }

    fn query_string(&self) -> String {
        let mut queries = Vec::with_capacity(4);
        if let Some(smp_id) = &self.smp_id {
            queries.push(["smp_id=", &smp_id.to_string()].concat());
        }
        if let Some(subgroup) = &self.subgroup {
            queries.push(["subgroup=", &subgroup.to_string()].concat());
        }
        if let Some(id) = &self.id {
            queries.push(["id=", &id.to_string()].concat());
        }
        if let Some(strict) = &self.strict {
            queries.push(["strict=", &strict.to_string()].concat());
        }

        (!queries.is_empty()).then(|| ["?", &queries.join("&")].concat()).unwrap_or_default()
    }
}

/// Nursing Template Group
#[derive(Clone, Demo, Deserialize, Serialize, FromRow, ToSchema)]
#[schema(example = json!(TmpGroup::demo()))]
pub struct TmpGroup {
    #[Demo(value = "1")]
    pub smp_id: u32,
    #[Demo(value = r#"Some(String::from("Group"))"#)]
    pub smp_name: Option<String>,
    #[Demo(value = "Some(1)")]
    pub smp_group: Option<u32>,
    #[Demo(value = "Some(1)")]
    pub smp_order: Option<u32>,
    #[Demo(value = r#"Some(String::from("Y"))"#)]
    pub smp_status: Option<String>,
}

impl TmpGroup {
    /// GET `EndPoint::IpdTmpGroup`
    pub async fn call_api_get(params: &TmpParams, app: Rc<AppState>) -> Result<Vec<Self>, AppError> {
        match fetch_json_api(&[EndPoint::IpdTmpGroup.base(), params.clone().query_string()].concat(), "GET", None, app).await {
            Ok((response, true)) => {
                let response: Vec<Self> = serde_wasm_bindgen::from_value(response).map_err(|e| Source::SerdeWasm.to_teapot_error(e, "Fetch TmpGroup"))?;
                Ok(response)
            }
            Ok((app_error, false)) => {
                let error: AppError = serde_wasm_bindgen::from_value(app_error).map_err(|e| Source::SerdeWasm.to_teapot_error(e, "Fetch TmpGroup"))?;
                Err(error)
            }
            Err(e) => Err(Source::Js.to_teapot_error(e.dyn_ref::<JsString>().map(|s| s.into()).unwrap_or(String::from("fetch error")), "Fetch Json")),
        }
    }

    /// POST `EndPoint::IpdTmpGroup`
    pub async fn call_api_post(&self, app: Rc<AppState>) -> Result<ExecuteResponse, AppError> {
        let body_json = serde_json::to_string(self).map_err(|e| Source::SerdeJson.to_teapot_error(e, "Send TmpGroup"))?;

        let body = serde_wasm_bindgen::to_value(&body_json).map_err(|e| Source::SerdeWasm.to_teapot_error(e, "Send TmpGroup"))?;

        execute_fetch(&EndPoint::IpdTmpGroup.base(), "POST", Some(&body), app).await
    }

    /// DELETE `EndPoint::IpdTmpGroup`
    pub async fn call_api_delete(params: &TmpParams, app: Rc<AppState>) -> Result<ExecuteResponse, AppError> {
        execute_fetch(&[EndPoint::IpdTmpGroup.base(), params.query_string()].concat(), "DELETE", None, app).await
    }
}

/// Nursing Template Sub-Group
#[derive(Clone, Demo, Deserialize, Serialize, FromRow, ToSchema)]
#[schema(example = json!(TmpSubGroup::demo()))]
pub struct TmpSubGroup {
    #[Demo(value = "1")]
    pub smp_id: u32,
    #[Demo(value = "1")]
    pub subgroup: u32,
    #[Demo(value = r#"Some(String::from("SubGroup"))"#)]
    pub subgroup_name: Option<String>,
    #[Demo(value = "Some(1)")]
    pub subgroup_order: Option<u32>,
    #[Demo(value = r#"Some(String::from("Y"))"#)]
    pub subgroup_status: Option<String>,
}

impl TmpSubGroup {
    /// GET `EndPoint::IpdTmpSubgroup`
    pub async fn call_api_get(params: &TmpParams, app: Rc<AppState>) -> Result<Vec<Self>, AppError> {
        match fetch_json_api(&[EndPoint::IpdTmpSubgroup.base(), params.clone().query_string()].concat(), "GET", None, app).await {
            Ok((response, true)) => {
                let response: Vec<Self> = serde_wasm_bindgen::from_value(response).map_err(|e| Source::SerdeWasm.to_teapot_error(e, "Fetch TmpSubGroup"))?;
                Ok(response)
            }
            Ok((app_error, false)) => {
                let error: AppError = serde_wasm_bindgen::from_value(app_error).map_err(|e| Source::SerdeWasm.to_teapot_error(e, "Fetch TmpSubGroup"))?;
                Err(error)
            }
            Err(e) => Err(Source::Js.to_teapot_error(e.dyn_ref::<JsString>().map(|s| s.into()).unwrap_or(String::from("fetch error")), "Fetch Json")),
        }
    }

    /// POST `EndPoint::IpdTmpSubgroup`
    pub async fn call_api_post(&self, app: Rc<AppState>) -> Result<ExecuteResponse, AppError> {
        let body_json = serde_json::to_string(self).map_err(|e| Source::SerdeJson.to_teapot_error(e, "Send TmpSubGroup"))?;

        let body = serde_wasm_bindgen::to_value(&body_json).map_err(|e| Source::SerdeWasm.to_teapot_error(e, "Send TmpSubGroup"))?;

        execute_fetch(&EndPoint::IpdTmpSubgroup.base(), "POST", Some(&body), app).await
    }

    /// DELETE `EndPoint::IpdTmpSubgroup`
    pub async fn call_api_delete(params: &TmpParams, app: Rc<AppState>) -> Result<ExecuteResponse, AppError> {
        execute_fetch(&[EndPoint::IpdTmpSubgroup.base(), params.query_string()].concat(), "DELETE", None, app).await
    }
}

/// Nursing Template Focus
#[derive(Clone, Demo, Deserialize, Serialize, FromRow, ToSchema)]
#[schema(example = json!(TmpFocus::demo()))]
pub struct TmpFocus {
    #[Demo(value = "1")]
    pub focus_id: u32,
    #[Demo(value = "1")]
    pub smp_id: u32,
    #[Demo(value = "1")]
    pub subgroup: u32,
    #[Demo(value = r#"Some(String::from("Focus"))"#)]
    pub focus_name: Option<String>,
    #[Demo(value = "Some(1)")]
    pub focus_order: Option<u32>,
    #[Demo(value = r#"Some(String::from("Y"))"#)]
    pub focus_status: Option<String>,
}

impl TmpFocus {
    /// GET `EndPoint::IpdTmpFocus`
    pub async fn call_api_get(params: &TmpParams, app: Rc<AppState>) -> Result<Vec<Self>, AppError> {
        match fetch_json_api(&[EndPoint::IpdTmpFocus.base(), params.clone().query_string()].concat(), "GET", None, app).await {
            Ok((response, true)) => {
                let response: Vec<Self> = serde_wasm_bindgen::from_value(response).map_err(|e| Source::SerdeWasm.to_teapot_error(e, "Fetch TmpFocus"))?;
                Ok(response)
            }
            Ok((app_error, false)) => {
                let error: AppError = serde_wasm_bindgen::from_value(app_error).map_err(|e| Source::SerdeWasm.to_teapot_error(e, "Fetch TmpFocus"))?;
                Err(error)
            }
            Err(e) => Err(Source::Js.to_teapot_error(e.dyn_ref::<JsString>().map(|s| s.into()).unwrap_or(String::from("fetch error")), "Fetch Json")),
        }
    }

    /// POST `EndPoint::IpdTmpFocus`
    pub async fn call_api_post(&self, app: Rc<AppState>) -> Result<ExecuteResponse, AppError> {
        let body_json = serde_json::to_string(self).map_err(|e| Source::SerdeJson.to_teapot_error(e, "Send TmpFocus"))?;

        let body = serde_wasm_bindgen::to_value(&body_json).map_err(|e| Source::SerdeWasm.to_teapot_error(e, "Send TmpFocus"))?;

        execute_fetch(&EndPoint::IpdTmpFocus.base(), "POST", Some(&body), app).await
    }

    /// DELETE `EndPoint::IpdTmpFocus`
    pub async fn call_api_delete(params: &TmpParams, app: Rc<AppState>) -> Result<ExecuteResponse, AppError> {
        execute_fetch(&[EndPoint::IpdTmpFocus.base(), params.query_string()].concat(), "DELETE", None, app).await
    }
}

/// Nursing Template Goal
#[derive(Clone, Demo, Deserialize, Serialize, FromRow, ToSchema)]
#[schema(example = json!(TmpGoal::demo()))]
pub struct TmpGoal {
    #[Demo(value = "1")]
    pub goal_id: u32,
    #[Demo(value = "1")]
    pub smp_id: u32,
    #[Demo(value = "1")]
    pub subgroup: u32,
    #[Demo(value = r#"Some(String::from("Goal"))"#)]
    pub goal_name: Option<String>,
    #[Demo(value = "Some(1)")]
    pub goal_order: Option<u32>,
    #[Demo(value = r#"Some(String::from("Y"))"#)]
    pub goal_status: Option<String>,
}

impl TmpGoal {
    /// GET `EndPoint::IpdTmpGoal`
    pub async fn call_api_get(params: &TmpParams, app: Rc<AppState>) -> Result<Vec<Self>, AppError> {
        match fetch_json_api(&[EndPoint::IpdTmpGoal.base(), params.clone().query_string()].concat(), "GET", None, app).await {
            Ok((response, true)) => {
                let response: Vec<Self> = serde_wasm_bindgen::from_value(response).map_err(|e| Source::SerdeWasm.to_teapot_error(e, "Fetch TmpGoal"))?;
                Ok(response)
            }
            Ok((app_error, false)) => {
                let error: AppError = serde_wasm_bindgen::from_value(app_error).map_err(|e| Source::SerdeWasm.to_teapot_error(e, "Fetch TmpGoal"))?;
                Err(error)
            }
            Err(e) => Err(Source::Js.to_teapot_error(e.dyn_ref::<JsString>().map(|s| s.into()).unwrap_or(String::from("fetch error")), "Fetch Json")),
        }
    }

    /// POST `EndPoint::IpdTmpGoal`
    pub async fn call_api_post(&self, app: Rc<AppState>) -> Result<ExecuteResponse, AppError> {
        let body_json = serde_json::to_string(self).map_err(|e| Source::SerdeJson.to_teapot_error(e, "Send TmpGoal"))?;

        let body = serde_wasm_bindgen::to_value(&body_json).map_err(|e| Source::SerdeWasm.to_teapot_error(e, "Send TmpGoal"))?;

        execute_fetch(&EndPoint::IpdTmpGoal.base(), "POST", Some(&body), app).await
    }

    /// DELETE `EndPoint::IpdTmpGoal`
    pub async fn call_api_delete(params: &TmpParams, app: Rc<AppState>) -> Result<ExecuteResponse, AppError> {
        execute_fetch(&[EndPoint::IpdTmpGoal.base(), params.query_string()].concat(), "DELETE", None, app).await
    }
}

/// Nursing Template Intervention
#[derive(Clone, Demo, Deserialize, Serialize, FromRow, ToSchema)]
#[schema(example = json!(TmpIntvt::demo()))]
pub struct TmpIntvt {
    #[Demo(value = "1")]
    pub intvt_id: u32,
    #[Demo(value = "1")]
    pub smp_id: u32,
    #[Demo(value = "1")]
    pub subgroup: u32,
    #[Demo(value = r#"Some(String::from("Intervention"))"#)]
    pub intvt_name: Option<String>,
    #[Demo(value = "Some(1)")]
    pub intvt_order: Option<u32>,
    #[Demo(value = r#"Some(String::from("Y"))"#)]
    pub intvt_status: Option<String>,
}

impl TmpIntvt {
    /// GET `EndPoint::IpdTmpIntvt`
    pub async fn call_api_get(params: &TmpParams, app: Rc<AppState>) -> Result<Vec<Self>, AppError> {
        match fetch_json_api(&[EndPoint::IpdTmpIntvt.base(), params.clone().query_string()].concat(), "GET", None, app).await {
            Ok((response, true)) => {
                let response: Vec<Self> = serde_wasm_bindgen::from_value(response).map_err(|e| Source::SerdeWasm.to_teapot_error(e, "Fetch TmpIntvt"))?;
                Ok(response)
            }
            Ok((app_error, false)) => {
                let error: AppError = serde_wasm_bindgen::from_value(app_error).map_err(|e| Source::SerdeWasm.to_teapot_error(e, "Fetch TmpIntvt"))?;
                Err(error)
            }
            Err(e) => Err(Source::Js.to_teapot_error(e.dyn_ref::<JsString>().map(|s| s.into()).unwrap_or(String::from("fetch error")), "Fetch Json")),
        }
    }

    /// POST `EndPoint::IpdTmpIntvt`
    pub async fn call_api_post(&self, app: Rc<AppState>) -> Result<ExecuteResponse, AppError> {
        let body_json = serde_json::to_string(self).map_err(|e| Source::SerdeJson.to_teapot_error(e, "Send TmpIntvt"))?;

        let body = serde_wasm_bindgen::to_value(&body_json).map_err(|e| Source::SerdeWasm.to_teapot_error(e, "Send TmpIntvt"))?;

        execute_fetch(&EndPoint::IpdTmpIntvt.base(), "POST", Some(&body), app).await
    }

    /// DELETE `EndPoint::IpdTmpIntvt`
    pub async fn call_api_delete(params: &TmpParams, app: Rc<AppState>) -> Result<ExecuteResponse, AppError> {
        execute_fetch(&[EndPoint::IpdTmpIntvt.base(), params.query_string()].concat(), "DELETE", None, app).await
    }
}

/// Nursing Template Daily-Care
#[derive(Clone, Demo, Deserialize, Serialize, FromRow, ToSchema)]
#[schema(example = json!(TmpDlc::demo()))]
pub struct TmpDlc {
    #[Demo(value = "1")]
    pub dlc_id: u32,
    #[Demo(value = r#"String::from("Daily Care")"#)]
    pub dlc_name: String,
    #[Demo(value = "Some(1)")]
    pub dlc_order: Option<u32>,
}

impl TmpDlc {
    /// GET `EndPoint::IpdTmpDlc`
    pub async fn call_api_get(params: &TmpParams, app: Rc<AppState>) -> Result<Vec<Self>, AppError> {
        match fetch_json_api(&[EndPoint::IpdTmpDlc.base(), params.clone().query_string()].concat(), "GET", None, app).await {
            Ok((response, true)) => {
                let response: Vec<Self> = serde_wasm_bindgen::from_value(response).map_err(|e| Source::SerdeWasm.to_teapot_error(e, "Fetch TmpDlc"))?;
                Ok(response)
            }
            Ok((app_error, false)) => {
                let error: AppError = serde_wasm_bindgen::from_value(app_error).map_err(|e| Source::SerdeWasm.to_teapot_error(e, "Fetch TmpDlc"))?;
                Err(error)
            }
            Err(e) => Err(Source::Js.to_teapot_error(e.dyn_ref::<JsString>().map(|s| s.into()).unwrap_or(String::from("fetch error")), "Fetch Json")),
        }
    }

    /// POST `EndPoint::IpdTmpDlc`
    pub async fn call_api_post(&self, app: Rc<AppState>) -> Result<ExecuteResponse, AppError> {
        let body_json = serde_json::to_string(self).map_err(|e| Source::SerdeJson.to_teapot_error(e, "Send TmpDlc"))?;

        let body = serde_wasm_bindgen::to_value(&body_json).map_err(|e| Source::SerdeWasm.to_teapot_error(e, "Send TmpDlc"))?;

        execute_fetch(&EndPoint::IpdTmpDlc.base(), "POST", Some(&body), app).await
    }

    /// DELETE `EndPoint::IpdTmpDlc`
    pub async fn call_api_delete(params: &TmpParams, app: Rc<AppState>) -> Result<ExecuteResponse, AppError> {
        execute_fetch(&[EndPoint::IpdTmpDlc.base(), params.query_string()].concat(), "DELETE", None, app).await
    }
}
