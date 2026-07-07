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
    fetch::{ExecuteResponse, fetch_json_api},
};

#[derive(Clone, Default, Deserialize, IntoParams)]
pub struct DrugUseDurationParams {
    pub icode: Option<String>,
    pub med_name: Option<String>,
    pub due_status: Option<String>,
    pub monitor_status: Option<String>,
    pub info_status: Option<String>,
}

impl QueryString for DrugUseDurationParams {
    fn from_tuples(params: &[(String, String)]) -> Option<Self> {
        (!params.is_empty()).then(|| Self {
            icode: find_qs(params, "icode"),
            med_name: find_qs(params, "med_name"),
            due_status: find_qs(params, "due_status"),
            monitor_status: find_qs(params, "monitor_status"),
            info_status: find_qs(params, "info_status"),
        })
    }

    fn query_string(&self) -> String {
        let mut queries = Vec::with_capacity(2);
        if let Some(icode) = &self.icode {
            queries.push(["icode=", icode].concat());
        }
        if let Some(med_name) = &self.med_name {
            queries.push(["med_name=", med_name].concat());
        }
        if let Some(due_status) = &self.due_status {
            queries.push(["due_status=", due_status].concat());
        }
        if let Some(monitor_status) = &self.monitor_status {
            queries.push(["monitor_status=", monitor_status].concat());
        }
        if let Some(info_status) = &self.info_status {
            queries.push(["info_status=", info_status].concat());
        }
        (!queries.is_empty()).then(|| ["?", &queries.join("&")].concat()).unwrap_or_default()
    }
}

#[derive(Clone, Debug, Default, Demo, Deserialize, Serialize, FromRow, ToSchema)]
#[schema(example = json!(DrugUseDuration::demo()))]
pub struct DrugUseDuration {
    #[Demo(value = r#"String::from("1000222")"#)]
    pub icode: String,
    #[Demo(value = r#"Some(String::from("PARACETAMOL 500 mg. เม็ด"))"#)]
    pub med_name: Option<String>,
    #[Demo(value = r#"Some(String::from("2g OD when CrCl 10-30"))"#)]
    pub usage: Option<String>,
    #[Demo(value = "Some(7)")]
    pub duration1: Option<i16>,
    #[Demo(value = r##"Some(String::from("#00FF00"))"##)]
    pub exceed_duration1_color: Option<String>,
    #[Demo(value = "Some(15)")]
    pub duration2: Option<i16>,
    #[Demo(value = r##"Some(String::from("#0000FF"))"##)]
    pub exceed_duration2_color: Option<String>,
    #[Demo(value = "Some(30)")]
    pub duration3: Option<i16>,
    #[Demo(value = r##"Some(String::from("#FF0000"))"##)]
    pub exceed_duration3_color: Option<String>,
    #[Demo(value = r#"Some(String::from("Y"))"#)]
    pub status: Option<String>,

    #[Demo(value = r#"Some(String::from("Record BP q 1 hr"))"#)]
    pub monitor: Option<String>,
    #[Demo(value = "Some(5)")]
    pub monitor_count: Option<u8>,
    /// minutes
    #[Demo(value = "Some(120)")]
    pub monitor_duration: Option<u32>,
    #[Demo(value = r#"Some(String::from("Y"))"#)]
    pub monitor_status: Option<String>,

    #[Demo(value = r#"Some(String::from("Use me gentle"))"#)]
    pub info: Option<String>,
    #[Demo(value = r#"Some(String::from("Y"))"#)]
    pub info_status: Option<String>,
}

impl DrugUseDuration {
    /// GET `EndPoint::DrugUseDuration`
    pub async fn call_api_get(params: &DrugUseDurationParams, app: Rc<AppState>) -> Result<Vec<Self>, AppError> {
        match fetch_json_api(&[EndPoint::DrugUseDuration.base(), params.clone().query_string()].concat(), "GET", None, app).await {
            Ok((response, true)) => {
                let response: Vec<Self> = serde_wasm_bindgen::from_value(response).map_err(|e| Source::SerdeWasm.to_teapot_error(e, "Fetch DrugUseDuration"))?;
                Ok(response)
            }
            Ok((app_error, false)) => {
                let error: AppError = serde_wasm_bindgen::from_value(app_error).map_err(|e| Source::SerdeWasm.to_teapot_error(e, "Fetch DrugUseDuration"))?;
                Err(error)
            }
            Err(e) => Err(Source::Js.to_teapot_error(e.dyn_ref::<JsString>().map(|s| s.into()).unwrap_or(String::from("fetch error")), "Fetch Json")),
        }
    }

    /// POST `EndPoint::DrugUseDuration`
    pub async fn call_api_post(payload: &Self, app: Rc<AppState>) -> Result<ExecuteResponse, AppError> {
        let body_json = serde_json::to_string(payload).map_err(|e| Source::SerdeJson.to_teapot_error(e, "Post DrugUseDuration"))?;
        let body = serde_wasm_bindgen::to_value(&body_json).map_err(|e| Source::SerdeWasm.to_teapot_error(e, "Post DrugUseDuration"))?;

        match fetch_json_api(&EndPoint::DrugUseDuration.base(), "POST", Some(&body), app).await {
            Ok((response, true)) => {
                let response: ExecuteResponse = serde_wasm_bindgen::from_value(response).map_err(|e| Source::SerdeWasm.to_teapot_error(e, "Post DrugUseDuration"))?;
                Ok(response)
            }
            Ok((app_error, false)) => {
                let error: AppError = serde_wasm_bindgen::from_value(app_error).map_err(|e| Source::SerdeWasm.to_teapot_error(e, "Post DrugUseDuration"))?;
                Err(error)
            }
            Err(e) => Err(Source::Js.to_teapot_error(e.dyn_ref::<JsString>().map(|s| s.into()).unwrap_or(String::from("fetch error")), "Fetch Json")),
        }
    }
}
