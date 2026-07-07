use derive_demo::Demo;
use js_sys::JsString;
use serde_derive::{Deserialize, Serialize};
use sqlx::FromRow;
use std::rc::Rc;
use time::{
    Date, PrimitiveDateTime, Time,
    macros::{date, datetime, time},
};
use utoipa::{IntoParams, ToSchema};
use wasm_bindgen::JsCast;

use kphis_util::{
    datetime::date_8601,
    error::{AppError, Source},
};

use crate::{
    app::AppState,
    endpoint::{EndPoint, QueryString, find_qs},
    fetch::{ExecuteResponse, execute_fetch, fetch_json_api},
    image::scan_his::ScanImage,
};

/// CBC's wbc and band with associated data
#[derive(Clone, Debug, Demo, Deserialize, FromRow, Serialize, ToSchema)]
#[schema(example = json!(LabWbcBand::demo()))]
pub struct LabWbcBand {
    #[Demo(value = r#"Some(String::from("0001234"))"#)]
    pub hn: Option<String>,
    #[Demo(value = r#"Some(String::from("661231235959"))"#)]
    pub vn: Option<String>, // HosXp store vn and an in this column
    #[Demo(value = "1")]
    pub lab_order_number: i32,
    #[Demo(value = "Some(date!(2023-12-31))")]
    pub order_date: Option<Date>,
    #[Demo(value = "Some(date!(2023-12-31))")]
    pub report_date: Option<Date>,
    #[Demo(value = "Some(date!(2023-12-31))")]
    pub receive_date: Option<Date>,
    #[Demo(value = r#"Some(String::from("8,888"))"#)]
    pub wbc: Option<String>,
    #[Demo(value = r#"Some(String::from("8"))"#)]
    pub band: Option<String>,
}

impl LabWbcBand {
    /// GET `EndPoint::LabWbcKeyValue`
    pub async fn call_api_get(key: &str, value: &str, app: Rc<AppState>) -> Result<Vec<Self>, AppError> {
        match fetch_json_api(&[&EndPoint::LabWbcKeyValue.base(), key, "/", value].concat(), "GET", None, app).await {
            Ok((response, true)) => {
                let response: Vec<Self> = serde_wasm_bindgen::from_value(response).map_err(|e| Source::SerdeWasm.to_teapot_error(e, "Fetch LabWbcBand"))?;
                Ok(response)
            }
            Ok((app_error, false)) => {
                let error: AppError = serde_wasm_bindgen::from_value(app_error).map_err(|e| Source::SerdeWasm.to_teapot_error(e, "Fetch LabWbcBand"))?;
                Err(error)
            }
            Err(e) => Err(Source::Js.to_teapot_error(e.dyn_ref::<JsString>().map(|s| s.into()).unwrap_or(String::from("fetch error")), "Fetch Json")),
        }
    }
}

// same as LabDetail with extra fields
// extra fields: confirm_report, doctor_name, hn, lab_confirm_state, lab_order_number, report_name, approve_staff
// used for head-only (empty 'lab_items_group') and for report
/// Lab Head with Item-Group
#[derive(Clone, Debug, Demo, Deserialize, FromRow, Serialize, ToSchema)]
#[schema(example = json!(LabHead::demo()))]
pub struct LabHead {
    #[Demo(value = "1")]
    pub lab_order_number: i32,
    #[Demo(value = r#"Some(String::from("661231235959"))"#)]
    pub vn: Option<String>, // HosXp store vn and an in this column
    #[Demo(value = r#"Some(String::from("0001234"))"#)]
    pub hn: Option<String>,
    #[Demo(value = "Some(date!(2023-12-31))")]
    pub order_date: Option<Date>,
    #[Demo(value = "Some(time!(23:59:59))")]
    pub order_time: Option<Time>,
    #[Demo(value = r#"Some(String::from("OPD"))"#)]
    pub department: Option<String>,
    #[Demo(value = "Some(date!(2023-12-31))")]
    pub receive_date: Option<Date>,
    #[Demo(value = "Some(time!(23:59:59))")]
    pub receive_time: Option<Time>,
    #[Demo(value = "Some(date!(2023-12-31))")]
    pub report_date: Option<Date>,
    #[Demo(value = "Some(time!(23:59:59))")]
    pub report_time: Option<Time>,
    #[Demo(value = r#"Some(String::from("Y"))"#)]
    pub confirm_report: Option<String>,
    #[Demo(value = r#"Some(String::from("Dr.Doctor"))"#)]
    pub doctor_name: Option<String>,
    #[Demo(value = r#"Some(String::from("CBC"))"#)]
    pub form_name: Option<String>,
    #[Demo(value = r#"Some(String::from("CBC"))"#)]
    pub lab_name_cc: Option<String>,
    #[Demo(value = r#"Some(String::from("โลหิต"))"#)]
    pub specimen_name_cc: Option<String>,
    #[Demo(value = r#"String::from("1")"#)]
    pub lab_confirm_state: String,
    #[Demo(value = r#"String::from("Y")"#)]
    pub lab_read_status: String,
    #[Demo(value = r#"Some(String::from("Mr.Reader"))"#)]
    pub lab_read_user: Option<String>,
    #[Demo(value = "Some(datetime!(2023-12-31 23:59:59))")]
    pub lab_read_datetime: Option<PrimitiveDateTime>,
    #[Demo(value = r#"Some(String::from("Mr.Report Tech"))"#)]
    pub reporter_name: Option<String>,
    #[Demo(value = r#"Some(String::from("Mr.Approve Tech"))"#)]
    pub approve_staff: Option<String>,
    #[sqlx(skip)]
    #[Demo(value = "vec![LabItemsGroup::demo()]")]
    pub lab_items_group: Vec<LabItemsGroup>,
    #[sqlx(skip)]
    #[Demo(value = "vec![ScanImage::demo()]")]
    pub scan_images: Vec<ScanImage>,
}

impl LabHead {
    /// GET `EndPoint::LabHead`
    pub async fn call_api_get(params: &LabHeadParams, app: Rc<AppState>) -> Result<Vec<Self>, AppError> {
        match fetch_json_api(&[EndPoint::LabHead.base(), params.query_string()].concat(), "GET", None, app).await {
            Ok((response, true)) => {
                let response: Vec<Self> = serde_wasm_bindgen::from_value(response).map_err(|e| Source::SerdeWasm.to_teapot_error(e, "Fetch LabHead"))?;
                Ok(response)
            }
            Ok((app_error, false)) => {
                let error: AppError = serde_wasm_bindgen::from_value(app_error).map_err(|e| Source::SerdeWasm.to_teapot_error(e, "Fetch LabHead"))?;
                Err(error)
            }
            Err(e) => Err(Source::Js.to_teapot_error(e.dyn_ref::<JsString>().map(|s| s.into()).unwrap_or(String::from("fetch error")), "Fetch Json")),
        }
    }

    /// POST `EndPoint::LabReadId`
    pub async fn call_api_post_readed(lab_order_number: i32, app: Rc<AppState>) -> Result<ExecuteResponse, AppError> {
        execute_fetch(&[EndPoint::LabReadId.base(), lab_order_number.to_string()].concat(), "POST", None, app).await
    }

    /// DELETE `EndPoint::LabReadId`
    pub async fn call_api_delete_readed(lab_order_number: i32, app: Rc<AppState>) -> Result<ExecuteResponse, AppError> {
        execute_fetch(&[EndPoint::LabReadId.base(), lab_order_number.to_string()].concat(), "DELETE", None, app).await
    }
}

/// Lab Item-Group with Items
#[derive(Clone, Debug, Demo, Deserialize, FromRow, Serialize, ToSchema)]
#[schema(example = json!(LabItemsGroup::demo()))]
pub struct LabItemsGroup {
    #[Demo(value = "Some(1)")]
    pub lab_items_group: Option<i32>,
    #[Demo(value = r#"Some(String::from("HEMATOLOGY"))"#)]
    pub lab_items_group_name: Option<String>,
    #[sqlx(skip)]
    #[Demo(value = "vec![LabItem::demo()]")]
    pub lab_items: Vec<LabItem>,
}

/// Lab Item
#[derive(Clone, Debug, Demo, Deserialize, FromRow, Serialize, ToSchema)]
#[schema(example = json!(LabItem::demo()))]
pub struct LabItem {
    #[Demo(value = r#"Some(String::from("WBC"))"#)]
    pub lab_items_name_ref: Option<String>,
    #[Demo(value = r#"Some(String::from("5,000-10,000"))"#)]
    pub lab_items_normal_value_ref: Option<String>,
    #[Demo(value = r#"Some(String::from("8,888"))"#)]
    pub lab_order_result: Option<String>,
    #[Demo(value = r#"Some(String::from("Y"))"#)]
    pub staff_lock_result: Option<String>,
    #[Demo(value = r#"Some(String::from("Note"))"#)]
    pub lab_order_remark: Option<String>,
    #[Demo(value = "Some(1)")]
    pub lab_items_group: Option<i32>,
    #[Demo(value = r#"Some(String::from("HEMATOLOGY"))"#)]
    pub lab_items_group_name: Option<String>,
    #[Demo(value = "Some(1)")]
    pub lab_items_code: Option<i32>,
    #[Demo(value = r#"Some(String::from("cell/mm3"))"#)]
    pub lab_items_unit: Option<String>,
    #[Demo(value = r#"Some(String::from("0001234"))"#)]
    pub hn: Option<String>,
    #[Demo(value = "Some(1)")]
    pub lab_order_number: Option<i32>,
    #[Demo(value = r#"Some(String::from("661231235959"))"#)]
    pub vn: Option<String>,
    #[Demo(value = "Some(date!(2023-12-31))")]
    pub order_date: Option<Date>,
    #[Demo(value = "Some(time!(23:59:59))")]
    pub order_time: Option<Time>,
    #[Demo(value = "Some(date!(2023-12-31))")]
    pub receive_date: Option<Date>,
    #[Demo(value = "Some(time!(23:59:59))")]
    pub receive_time: Option<Time>,
    #[Demo(value = "Some(date!(2023-12-31))")]
    pub report_date: Option<Date>,
    #[Demo(value = "Some(time!(23:59:59))")]
    pub report_time: Option<Time>,
    /// compute at client side
    #[sqlx(skip)]
    #[Demo(value = r#"vec![Some(String::from("7,777")),None]"#)]
    pub prev_lab_order_results: Vec<Option<String>>,
}

impl LabItem {
    /// GET `EndPoint::LabItem`
    pub async fn call_api_get(params: &LabItemParams, app: Rc<AppState>) -> Result<Vec<Self>, AppError> {
        match fetch_json_api(&[EndPoint::LabItem.base(), params.query_string()].concat(), "GET", None, app).await {
            Ok((response, true)) => {
                let response: Vec<Self> = serde_wasm_bindgen::from_value(response).map_err(|e| Source::SerdeWasm.to_teapot_error(e, "Fetch LabItem"))?;
                Ok(response)
            }
            Ok((app_error, false)) => {
                let error: AppError = serde_wasm_bindgen::from_value(app_error).map_err(|e| Source::SerdeWasm.to_teapot_error(e, "Fetch LabItem"))?;
                Err(error)
            }
            Err(e) => Err(Source::Js.to_teapot_error(e.dyn_ref::<JsString>().map(|s| s.into()).unwrap_or(String::from("fetch error")), "Fetch Json")),
        }
    }
}

#[derive(Clone, Default, Deserialize, IntoParams)]
pub struct LabHeadParams {
    pub vn: Option<String>,
    pub hn: Option<String>,
    /// use with hn
    pub id: Option<i32>,
    pub start_date: Option<Date>,
    pub end_date: Option<Date>,
    pub prev: Option<i32>,
    pub only_head: Option<bool>,
    pub with_scan: Option<bool>,
}

impl QueryString for LabHeadParams {
    fn from_tuples(params: &[(String, String)]) -> Option<Self> {
        (!params.is_empty()).then(|| Self {
            vn: find_qs(params, "vn"),
            hn: find_qs(params, "hn"),
            id: find_qs(params, "id").and_then(|s| s.parse::<i32>().ok()),
            start_date: find_qs(params, "start_date").and_then(|s| date_8601(&s)),
            end_date: find_qs(params, "end_date").and_then(|s| date_8601(&s)),
            prev: find_qs(params, "prev").and_then(|s| s.parse::<i32>().ok()),
            only_head: find_qs(params, "only_head").map(|s| s.as_str() == "true"),
            with_scan: find_qs(params, "with_scan").map(|s| s.as_str() == "true"),
        })
    }

    fn query_string(&self) -> String {
        let mut queries = Vec::with_capacity(7);

        if let Some(vn) = &self.vn {
            queries.push(["vn=", vn].concat());
        }
        if let Some(hn) = &self.hn {
            queries.push(["hn=", hn].concat());
        }
        if let Some(id) = &self.id {
            queries.push(["id=", &id.to_string()].concat());
        }
        if let Some(start_date) = &self.start_date {
            queries.push(["start_date=", &start_date.to_string()].concat());
        }
        if let Some(end_date) = &self.end_date {
            queries.push(["end_date=", &end_date.to_string()].concat());
        }
        if let Some(limit) = &self.prev {
            queries.push(["prev=", &limit.to_string()].concat());
        }
        if self.only_head.unwrap_or_default() {
            queries.push(String::from("only_head=true"));
        }
        if self.with_scan.unwrap_or_default() {
            queries.push(String::from("with_scan=true"));
        }

        (!queries.is_empty()).then(|| ["?", &queries.join("&")].concat()).unwrap_or_default()
    }
}

#[derive(Clone, Default, Deserialize, IntoParams)]
pub struct LabItemParams {
    pub hn: Option<String>,
    pub vn: Option<String>,
    pub lab_items_code: Option<i32>,
}

impl QueryString for LabItemParams {
    fn from_tuples(params: &[(String, String)]) -> Option<Self> {
        (!params.is_empty()).then(|| Self {
            hn: find_qs(params, "hn"),
            vn: find_qs(params, "vn"),
            lab_items_code: find_qs(params, "lab_items_code").and_then(|s| s.parse::<i32>().ok()),
        })
    }

    fn query_string(&self) -> String {
        let mut queries = Vec::with_capacity(3);

        if let Some(hn) = &self.hn {
            queries.push(["hn=", hn].concat());
        }
        if let Some(vn) = &self.vn {
            queries.push(["vn=", vn].concat());
        }
        if let Some(lab_items_code) = &self.lab_items_code {
            queries.push(["lab_items_code=", &lab_items_code.to_string()].concat());
        }

        (!queries.is_empty()).then(|| ["?", &queries.join("&")].concat()).unwrap_or_default()
    }
}
