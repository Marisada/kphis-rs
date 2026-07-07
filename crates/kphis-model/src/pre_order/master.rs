use derive_demo::Demo;
use js_sys::JsString;
use serde::{Deserialize, Serialize};
use sqlx::{
    FromRow,
    types::time::{Date, PrimitiveDateTime, Time},
};
use std::rc::Rc;
use time::macros::{date, datetime, time};
use utoipa::{IntoParams, ToSchema};
use wasm_bindgen::JsCast;

use kphis_util::{
    datetime::date_8601,
    error::{AppError, Source},
};

use crate::{
    app::AppState,
    endpoint::{EndPoint, QueryString, find_qs},
    fetch::{ExecuteResponse, execute_fetch_vec, execute_fetch_with_u32, fetch_json_api},
};

#[derive(Clone, Default, Deserialize, IntoParams)]
pub struct PreOrderMasterParams {
    pub pre_order_master_id: Option<u32>,
    pub hn: Option<String>,
    pub start_order_date: Option<Date>,
    pub end_order_date: Option<Date>,
    pub order_doctor: Option<String>,
    pub include_shared_template: Option<String>,
    pub pre_order_type: Option<String>,
    pub template_name: Option<String>,
    pub used: Option<String>,
}

impl QueryString for PreOrderMasterParams {
    fn from_tuples(params: &[(String, String)]) -> Option<Self> {
        (!params.is_empty()).then(|| Self {
            pre_order_master_id: find_qs(params, "pre_order_master_id").and_then(|s| s.parse::<u32>().ok()),
            hn: find_qs(params, "hn"),
            start_order_date: find_qs(params, "start_order_date").and_then(|s| date_8601(&s)),
            end_order_date: find_qs(params, "end_order_date").and_then(|s| date_8601(&s)),
            order_doctor: find_qs(params, "order_doctor"),
            include_shared_template: find_qs(params, "include_shared_template"),
            pre_order_type: find_qs(params, "pre_order_type"),
            template_name: find_qs(params, "template_name"),
            used: find_qs(params, "used"),
        })
    }

    fn query_string(&self) -> String {
        let mut queries = Vec::with_capacity(9);
        if let Some(pre_order_master_id) = &self.pre_order_master_id {
            queries.push(["pre_order_master_id=", &pre_order_master_id.to_string()].concat());
        }
        if let Some(hn) = &self.hn {
            queries.push(["hn=", hn].concat());
        }
        if let Some(start_order_date) = &self.start_order_date {
            queries.push(["start_order_date=", &start_order_date.to_string()].concat());
        }
        if let Some(end_order_date) = &self.end_order_date {
            queries.push(["end_order_date=", &end_order_date.to_string()].concat());
        }
        if let Some(order_doctor) = &self.order_doctor {
            queries.push(["order_doctor=", order_doctor].concat());
        }
        if let Some(include_shared_template) = &self.include_shared_template {
            queries.push(["include_shared_template=", include_shared_template].concat());
        }
        if let Some(pre_order_type) = &self.pre_order_type {
            queries.push(["pre_order_type=", pre_order_type].concat());
        }
        if let Some(template_name) = &self.template_name {
            queries.push(["template_name=", template_name].concat());
        }
        if let Some(used) = &self.used {
            queries.push(["used=", used].concat());
        }

        (!queries.is_empty()).then(|| ["?", &queries.join("&")].concat()).unwrap_or_default()
    }
}

/// IPD Pre-Order Master
#[derive(Clone, Demo, Deserialize, Serialize, FromRow, ToSchema)]
#[schema(example = json!(PreOrderMaster::demo()))]
pub struct PreOrderMaster {
    #[Demo(value = "1")]
    pub pre_order_master_id: u32,
    #[Demo(value = r#"Some(String::from("0001234"))"#)]
    pub hn: Option<String>,
    #[Demo(value = r#"Some(String::from("Y"))"#)]
    pub used: Option<String>,
    #[Demo(value = "Some(date!(2023-12-31))")]
    pub order_date: Option<Date>,
    #[Demo(value = "Some(time!(23:59:59))")]
    pub order_time: Option<Time>,
    #[Demo(value = r#"Some(datetime!(2023-12-31 23:59:59))"#)]
    pub order_date_time: Option<PrimitiveDateTime>,
    #[Demo(value = "Some(date!(2023-12-31))")]
    pub order_for_date: Option<Date>,
    /// NOT USE
    #[Demo(default)]
    pub order_for_time: Option<Time>,
    /// NOT USE
    #[Demo(default)]
    pub order_for_date_time: Option<String>,
    #[Demo(value = r#"Some(String::from("Named"))"#)]
    pub template_name: Option<String>,
    #[Demo(value = r#"Some(String::from("Y"))"#)]
    pub shared_template: Option<String>,
    #[Demo(value = r#"String::from("007")"#)]
    pub order_doctor: String,
    #[Demo(value = r#"Some(String::from("Dr.Doctor"))"#)]
    pub order_doctor_name: Option<String>,
    #[Demo(value = "Some(true)")]
    pub order_doctor_is_intern: Option<bool>,
    /// template, appointment, opd
    #[Demo(value = r#"String::from("opd")"#)]
    pub pre_order_type: String,
    #[Demo(value = r#"Some(String::from("Mr.Patient Sicker"))"#)]
    pub fullname: Option<String>,
}

impl PreOrderMaster {
    /// GET `EndPoint::IpdPreOrderMaster`
    pub async fn call_api_get(params: &PreOrderMasterParams, app: Rc<AppState>) -> Result<Vec<Self>, AppError> {
        match fetch_json_api(&[EndPoint::IpdPreOrderMaster.base(), params.clone().query_string()].concat(), "GET", None, app).await {
            Ok((response, true)) => {
                let response: Vec<Self> = serde_wasm_bindgen::from_value(response).map_err(|e| Source::SerdeWasm.to_teapot_error(e, "Fetch PreOrderMaster"))?;
                Ok(response)
            }
            Ok((app_error, false)) => {
                let error: AppError = serde_wasm_bindgen::from_value(app_error).map_err(|e| Source::SerdeWasm.to_teapot_error(e, "Fetch PreOrderMaster"))?;
                Err(error)
            }
            Err(e) => Err(Source::Js.to_teapot_error(e.dyn_ref::<JsString>().map(|s| s.into()).unwrap_or(String::from("fetch error")), "Fetch Json")),
        }
    }

    /// DELETE `EndPoint::IpdPreOrderMasterId`
    pub async fn call_api_delete(pre_order_master_id: u32, app: Rc<AppState>) -> Result<Vec<ExecuteResponse>, AppError> {
        execute_fetch_vec(&[EndPoint::IpdPreOrderMasterId.base(), pre_order_master_id.to_string()].concat(), "DELETE", None, app).await
    }
}

/// IPD Pre-Order Master for save
#[derive(Debug, Demo, Deserialize, Serialize, ToSchema)]
#[schema(example = json!(PreOrderMasterSave::demo()))]
pub struct PreOrderMasterSave {
    #[Demo(value = "Some(1)")]
    pub pre_order_master_id: Option<u32>,
    #[Demo(value = r#"String::from("opd")"#)]
    pub pre_order_type: String,
    #[Demo(value = "Some(date!(2023-12-31))")]
    pub order_date: Option<Date>,
    #[Demo(value = "Some(time!(23:59:59))")]
    pub order_time: Option<Time>,
    #[Demo(value = "Some(date!(2023-12-31))")]
    pub order_for_date: Option<Date>,
    /// NOT USE
    #[Demo(default)]
    pub order_for_time: Option<Time>,
    #[Demo(value = r#"String::from("007")"#)]
    pub order_doctor: String,
    #[Demo(value = r#"Some(String::from("0001234"))"#)]
    pub hn: Option<String>,
    #[Demo(value = r#"Some(String::from("Named"))"#)]
    pub template_name: Option<String>,
    #[Demo(value = r#"Some(String::from("Y"))"#)]
    pub shared_template: Option<String>,
    #[Demo(value = r#"Some(String::from("Y"))"#)]
    pub used: Option<String>,
}

impl PreOrderMasterSave {
    /// POST `EndPoint::IpdPreOrderMaster`
    pub async fn call_api_post(&self, app: Rc<AppState>) -> Result<(u32, ExecuteResponse), AppError> {
        let body_json = serde_json::to_string(self).map_err(|e| Source::SerdeJson.to_teapot_error(e, "Send PreOrderMasterSave"))?;

        let body = serde_wasm_bindgen::to_value(&body_json).map_err(|e| Source::SerdeWasm.to_teapot_error(e, "Send PreOrderMasterSave"))?;

        execute_fetch_with_u32(&EndPoint::IpdPreOrderMaster.base(), "POST", Some(&body), app).await
    }
}
