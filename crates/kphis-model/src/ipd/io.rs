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
use time::{
    format_description::well_known::Iso8601,
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
    fetch::{ExecuteResponse, execute_fetch_vec, execute_fetch_vec_with_u32, fetch_json_api},
    shift::NurseShift,
};

/// IO Date with today marking
#[derive(Clone, Demo, Deserialize, Serialize, FromRow, ToSchema)]
#[schema(example = json!(IoDate::demo()))]
pub struct IoDate {
    #[Demo(value = "date!(2023-12-31)")]
    pub io_date: Date,
    #[Demo(value = "true")]
    pub is_today: bool,
}

impl IoDate {
    pub fn string(&self) -> String {
        [self.io_date.to_string(), if self.is_today { String::from("1") } else { String::from("0") }].join("|")
    }

    pub fn from_string(value: &str) -> Option<Self> {
        let tuple = value.split('|').collect::<Vec<&str>>();
        if tuple.len() == 2 {
            Date::parse(tuple[0], &Iso8601::DEFAULT).ok().map(|date| Self {
                io_date: date,
                is_today: tuple[1] == "1",
            })
        } else {
            None
        }
    }

    /// GET `EndPoint::IpdIoDateAn`
    pub async fn call_api_get_ipd(an: &str, app: Rc<AppState>) -> Result<Vec<Self>, AppError> {
        Self::get(&[&EndPoint::IpdIoDateAn.base(), an].concat(), app).await
    }

    /// GET `EndPoint::OpdErIoDateId`
    pub async fn call_api_get_opd_er(opd_er_order_master_id: u32, app: Rc<AppState>) -> Result<Vec<Self>, AppError> {
        Self::get(&[EndPoint::OpdErIoDateId.base(), opd_er_order_master_id.to_string()].concat(), app).await
    }

    async fn get(path: &str, app: Rc<AppState>) -> Result<Vec<Self>, AppError> {
        match fetch_json_api(path, "GET", None, app).await {
            Ok((response, true)) => {
                let response: Vec<Self> = serde_wasm_bindgen::from_value(response).map_err(|e| Source::SerdeWasm.to_teapot_error(e, "Fetch IoDate"))?;
                Ok(response)
            }
            Ok((app_error, false)) => {
                let error: AppError = serde_wasm_bindgen::from_value(app_error).map_err(|e| Source::SerdeWasm.to_teapot_error(e, "Fetch IoDate"))?;
                Err(error)
            }
            Err(e) => Err(Source::Js.to_teapot_error(e.dyn_ref::<JsString>().map(|s| s.into()).unwrap_or(String::from("fetch error")), "Fetch Json")),
        }
    }
}

impl PartialEq for IoDate {
    fn eq(&self, other: &Self) -> bool {
        self.io_date == other.io_date
    }
}

#[derive(Clone, Default, Deserialize, IntoParams)]
pub struct IoParams {
    pub an: Option<String>,
    pub opd_er_order_master_id: Option<u32>,
    pub start_date: Option<Date>,
    pub end_date: Option<Date>,
    pub io_id: Option<u32>,
    pub version: Option<i32>,
}

impl QueryString for IoParams {
    fn from_tuples(params: &[(String, String)]) -> Option<Self> {
        (!params.is_empty()).then(|| Self {
            an: find_qs(params, "an"),
            opd_er_order_master_id: find_qs(params, "opd_er_order_master_id").and_then(|s| s.parse::<u32>().ok()),
            start_date: find_qs(params, "start_date").and_then(|s| date_8601(&s)),
            end_date: find_qs(params, "end_date").and_then(|s| date_8601(&s)),
            io_id: find_qs(params, "io_id").and_then(|s| s.parse::<u32>().ok()),
            version: find_qs(params, "version").and_then(|s| s.parse::<i32>().ok()),
        })
    }

    fn query_string(&self) -> String {
        let mut queries = Vec::with_capacity(6);
        if let Some(an) = &self.an {
            queries.push(["an=", an].concat());
        }
        if let Some(opd_er_order_master_id) = &self.opd_er_order_master_id {
            queries.push(["opd_er_order_master_id=", &opd_er_order_master_id.to_string()].concat());
        }
        if let Some(start_date) = &self.start_date {
            queries.push(["start_date=", &start_date.to_string()].concat());
        }
        if let Some(end_date) = &self.end_date {
            queries.push(["end_date=", &end_date.to_string()].concat());
        }
        if let Some(io_id) = &self.io_id {
            queries.push(["io_id=", &io_id.to_string()].concat());
        }
        if let Some(version) = &self.version {
            queries.push(["version=", &version.to_string()].concat());
        }

        (!queries.is_empty()).then(|| ["?", &queries.join("&")].concat()).unwrap_or_default()
    }
}

/// IPD IO with shift calculated
#[derive(Clone, Demo, Serialize, Deserialize, ToSchema)]
#[schema(example = json!(IoShift::demo()))]
pub struct IoShift {
    #[Demo(value = "1")]
    pub io_id: u32,
    #[Demo(value = "date!(2023-12-31)")]
    pub io_date: Date,
    #[Demo(value = "time!(23:59:59)")]
    pub io_time: Time,
    #[Demo(value = r#"Some(String::from("iv"))"#)]
    pub io_parenteral_type: Option<String>,
    #[Demo(value = r#"Some(String::from("0.9% NSS"))"#)]
    pub io_parenteral_name: Option<String>,
    #[Demo(value = "Some(Decimal::new(1000,0))")]
    pub io_parenteral_amount: Option<Decimal>,
    #[Demo(value = "Some(Decimal::new(700,0))")]
    pub io_parenteral_absorb: Option<Decimal>,
    #[Demo(value = "Some(Decimal::new(300,0))")]
    pub io_parenteral_carry_forward: Option<Decimal>,
    #[Demo(value = r#"Some(String::from("Remark"))"#)]
    pub io_parenteral_remark: Option<String>,
    #[Demo(value = r#"Some(String::from("น้ำเปล่า"))"#)]
    pub io_oral_name: Option<String>,
    #[Demo(value = "Some(300)")]
    pub io_oral_amount: Option<i32>,
    #[Demo(value = "Some(200)")]
    pub io_oral_absorb: Option<i32>,
    #[Demo(value = "Some(100)")]
    pub io_oral_carry_forward: Option<i32>,
    #[Demo(value = r#"Some(String::from("Remark"))"#)]
    pub io_oral_remark: Option<String>,
    #[Demo(value = r#"Some(String::from("urine"))"#)]
    pub io_output_type: Option<String>,
    #[Demo(value = "Some(500)")]
    pub io_output_amount: Option<i32>,
    #[Demo(value = r#"Some(String::from("Remark"))"#)]
    pub io_output_remark: Option<String>,
    #[Demo(value = "1")]
    pub version: i32,
    #[Demo(value = r#"Some(String::from("Miss.Nurse"))"#)]
    pub user_name: Option<String>,
    #[Demo(value = r#"Some(String::from("พยาบาล"))"#)]
    pub entryposition: Option<String>,
    #[Demo(value = "Some(date!(2023-12-31))")]
    pub shift_date: Option<Date>, // None for uploading
    #[Demo(value = "Some(NurseShift::demo_day())")]
    pub shift: Option<NurseShift>, // None for uploading
    #[Demo(value = r#"Some(String::from("660001234"))"#)]
    pub an: Option<String>,
    #[Demo(value = "Some(1)")]
    pub opd_er_order_master_id: Option<u32>,
}

impl IoShift {
    pub fn shift(&self, app: Rc<AppState>) -> Option<(Date, NurseShift)> {
        app.cal_shift(self.io_date, self.io_time)
    }

    /// GET `EndPoint::IpdIo`
    pub async fn call_api_get_ipd(params: &IoParams, app: Rc<AppState>) -> Result<Vec<Self>, AppError> {
        Self::get(&[EndPoint::IpdIo.base(), params.query_string()].concat(), app).await
    }

    /// GET `EndPoint::OpdErIo`
    pub async fn call_api_get_opd_er(params: &IoParams, app: Rc<AppState>) -> Result<Vec<Self>, AppError> {
        Self::get(&[EndPoint::OpdErIo.base(), params.query_string()].concat(), app).await
    }

    async fn get(path: &str, app: Rc<AppState>) -> Result<Vec<Self>, AppError> {
        match fetch_json_api(path, "GET", None, app).await {
            Ok((response, true)) => {
                let response: Vec<Self> = serde_wasm_bindgen::from_value(response).map_err(|e| Source::SerdeWasm.to_teapot_error(e, "Fetch IoShift"))?;
                Ok(response)
            }
            Ok((app_error, false)) => {
                let error: AppError = serde_wasm_bindgen::from_value(app_error).map_err(|e| Source::SerdeWasm.to_teapot_error(e, "Fetch IoShift"))?;
                Err(error)
            }
            Err(e) => Err(Source::Js.to_teapot_error(e.dyn_ref::<JsString>().map(|s| s.into()).unwrap_or(String::from("fetch error")), "Fetch Json")),
        }
    }

    /// - POST `EndPoint::IpdIo`
    /// - POST `EndPoint::OpdErIo`
    pub async fn call_api_post(&self, app: Rc<AppState>) -> Result<(u32, Vec<ExecuteResponse>), AppError> {
        let path = match (self.an.is_some(), self.opd_er_order_master_id.is_some()) {
            (true, false) => Ok(EndPoint::IpdIo.base()),
            (false, true) => Ok(EndPoint::OpdErIo.base()),
            (_, _) => Err(Source::App.to_teapot_error("Bad Payload", "Send IoShift")),
        }?;
        let body_json = serde_json::to_string(self).map_err(|e| Source::SerdeJson.to_teapot_error(e, "Send IoShift"))?;
        let body = serde_wasm_bindgen::to_value(&body_json).map_err(|e| Source::SerdeWasm.to_teapot_error(e, "Send IoShift"))?;
        execute_fetch_vec_with_u32(&path, "POST", Some(&body), app).await
    }

    /// - DELETE `EndPoint::IpdIo`
    /// - DELETE `EndPoint::OpdErIo`
    pub async fn call_api_delete(is_ipd: bool, params: &IoParams, app: Rc<AppState>) -> Result<Vec<ExecuteResponse>, AppError> {
        let endpoint = if is_ipd { EndPoint::IpdIo } else { EndPoint::OpdErIo };
        execute_fetch_vec(&[endpoint.base(), params.query_string()].concat(), "DELETE", None, app).await
    }
}

#[derive(Demo, Serialize, Deserialize)]
pub struct IoOnly {
    #[Demo(value = "1")]
    pub io_id: u32,
    #[Demo(value = "date!(2023-12-31)")]
    pub io_date: Date,
    #[Demo(value = "time!(23:59:59)")]
    pub io_time: Time,
    #[Demo(value = r#"Some(String::from("iv"))"#)]
    pub io_parenteral_type: Option<String>,
    #[Demo(value = r#"Some(String::from("0.9% NSS"))"#)]
    pub io_parenteral_name: Option<String>,
    #[Demo(value = "Some(Decimal::new(1000,0))")]
    pub io_parenteral_amount: Option<Decimal>,
    #[Demo(value = "Some(Decimal::new(700,0))")]
    pub io_parenteral_absorb: Option<Decimal>,
    #[Demo(value = "Some(Decimal::new(300,0))")]
    pub io_parenteral_carry_forward: Option<Decimal>,
    #[Demo(value = r#"Some(String::from("Remark"))"#)]
    pub io_parenteral_remark: Option<String>,
    #[Demo(value = r#"Some(String::from("น้ำเปล่า"))"#)]
    pub io_oral_name: Option<String>,
    #[Demo(value = "Some(300)")]
    pub io_oral_amount: Option<i32>,
    #[Demo(value = "Some(200)")]
    pub io_oral_absorb: Option<i32>,
    #[Demo(value = "Some(100)")]
    pub io_oral_carry_forward: Option<i32>,
    #[Demo(value = r#"Some(String::from("Remark"))"#)]
    pub io_oral_remark: Option<String>,
    #[Demo(value = r#"Some(String::from("urine"))"#)]
    pub io_output_type: Option<String>,
    #[Demo(value = "Some(500)")]
    pub io_output_amount: Option<i32>,
    #[Demo(value = r#"Some(String::from("Remark"))"#)]
    pub io_output_remark: Option<String>,
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

impl PartialEq for IoOnly {
    fn eq(&self, other: &Self) -> bool {
        // self.io_id == other.io_id &&
        self.io_date == other.io_date
            && self.io_time == other.io_time
            && self.io_parenteral_type == other.io_parenteral_type
            && self.io_parenteral_name == other.io_parenteral_name
            && self.io_parenteral_amount == other.io_parenteral_amount
            && self.io_parenteral_absorb == other.io_parenteral_absorb
            && self.io_parenteral_carry_forward == other.io_parenteral_carry_forward
            && self.io_parenteral_remark == other.io_parenteral_remark
            && self.io_oral_name == other.io_oral_name
            && self.io_oral_amount == other.io_oral_amount
            && self.io_oral_absorb == other.io_oral_absorb
            && self.io_oral_carry_forward == other.io_oral_carry_forward
            && self.io_oral_remark == other.io_oral_remark
            && self.io_output_type == other.io_output_type
            && self.io_output_amount == other.io_output_amount
            && self.io_output_remark == other.io_output_remark
            && self.create_user == other.create_user
            && self.create_datetime == other.create_datetime
            && self.update_user == other.update_user
            && self.update_datetime == other.update_datetime
            && self.version == other.version
    }
}
