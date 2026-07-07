use derive_demo::Demo;
use js_sys::JsString;
use serde_derive::{Deserialize, Serialize};
use sqlx::FromRow;
use std::rc::Rc;
use time::{
    Date, PrimitiveDateTime, Time,
    macros::{date, datetime, time},
};
use utoipa::ToSchema;
use wasm_bindgen::JsCast;

use kphis_util::error::{AppError, Source};

use crate::{app::AppState, endpoint::EndPoint, fetch::fetch_json_api};

/// Xray Report
#[derive(Clone, Debug, Demo, Deserialize, FromRow, Serialize, ToSchema)]
#[schema(example = json!(XrayReport::demo()))]
pub struct XrayReport {
    #[Demo(value = "1")]
    pub xn: i32,
    #[Demo(value = r#"Some(String::from("0001234"))"#)]
    pub hn: Option<String>,
    #[Demo(value = r#"Some(String::from("661231235959"))"#)]
    pub vn: Option<String>,
    #[Demo(value = r#"Some(String::from("660001234"))"#)]
    pub an: Option<String>,
    #[Demo(value = r#"Some(String::from("Y"))"#)]
    pub confirm: Option<String>,
    #[Demo(value = r#"Some(String::from("Film CXR (PA)"))"#)]
    pub xray_items_name: Option<String>,
    #[Demo(value = "Some(date!(2023-12-31))")]
    pub request_date: Option<Date>,
    #[Demo(value = "Some(time!(23:59:59))")]
    pub request_time: Option<Time>,
    #[Demo(value = "Some(date!(2023-12-31))")]
    pub examined_date: Option<Date>,
    #[Demo(value = "Some(time!(23:59:59))")]
    pub examined_time: Option<Time>,
    #[Demo(value = r#"Some(String::from("Mr.Xray"))"#)]
    pub technician_name: Option<String>,
    #[Demo(value = r#"Some(String::from("Dr.Doctor"))"#)]
    pub request_doctor_name: Option<String>,
    #[Demo(value = r#"String::from("Y")"#)]
    pub xray_read_status: String,
    #[Demo(value = r#"Some(String::from("Mr.Reader"))"#)]
    pub xray_read_user: Option<String>,
    #[Demo(value = "Some(datetime!(2023-12-31 23:59:59))")]
    pub xray_read_datetime: Option<PrimitiveDateTime>,
}

impl XrayReport {
    /// GET `EndPoint::XrayReportHn`
    pub async fn call_api_get(hn: &str, app: Rc<AppState>) -> Result<Vec<Self>, AppError> {
        match fetch_json_api(&[&EndPoint::XrayReportHn.base(), hn].concat(), "GET", None, app).await {
            Ok((response, true)) => {
                let response: Vec<Self> = serde_wasm_bindgen::from_value(response).map_err(|e| Source::SerdeWasm.to_teapot_error(e, "Fetch XrayReportHn"))?;
                Ok(response)
            }
            Ok((app_error, false)) => {
                let error: AppError = serde_wasm_bindgen::from_value(app_error).map_err(|e| Source::SerdeWasm.to_teapot_error(e, "Fetch XrayReportHn"))?;
                Err(error)
            }
            Err(e) => Err(Source::Js.to_teapot_error(e.dyn_ref::<JsString>().map(|s| s.into()).unwrap_or(String::from("fetch error")), "Fetch Json")),
        }
    }
}
