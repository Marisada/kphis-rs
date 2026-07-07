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
    fetch::{ExecuteResponse, execute_fetch_with_u32, fetch_json_api},
};

/// OPD-ER Order Master Checker
#[derive(Clone, Debug, Demo, Deserialize, Serialize, FromRow, ToSchema)]
#[schema(example = json!(OpdErOrderMasterCheck::demo()))]
pub struct OpdErOrderMasterCheck {
    #[Demo(value = "1")]
    pub opd_er_order_master_id: u32,
    #[Demo(value = "date!(2023-12-31)")]
    pub order_date: Date,
}
impl OpdErOrderMasterCheck {
    /// GET `EndPoint::OpdErOrderMasterCheckVn`
    pub async fn call_api_get(vn: &str, app: Rc<AppState>) -> Result<Vec<Self>, AppError> {
        match fetch_json_api(&[&EndPoint::OpdErOrderMasterCheckVn.base(), vn].concat(), "GET", None, app).await {
            Ok((response, true)) => {
                let response: Vec<Self> = serde_wasm_bindgen::from_value(response).map_err(|e| Source::SerdeWasm.to_teapot_error(e, "Fetch OrderMasterCheck"))?;
                Ok(response)
            }
            Ok((app_error, false)) => {
                let error: AppError = serde_wasm_bindgen::from_value(app_error).map_err(|e| Source::SerdeWasm.to_teapot_error(e, "Fetch OrderMasterCheck"))?;
                Err(error)
            }
            Err(e) => Err(Source::Js.to_teapot_error(e.dyn_ref::<JsString>().map(|s| s.into()).unwrap_or(String::from("fetch error")), "Fetch Json")),
        }
    }
}

#[derive(Clone, Default, Deserialize, IntoParams)]
pub struct OpdErOrderMasterParams {
    pub opd_er_order_master_id: Option<u32>,
    pub order_doctor: Option<String>,
    pub order_date: Option<Date>,
    pub start_order_date: Option<Date>,
    pub end_order_date: Option<Date>,
    pub hn: Option<String>,
    pub vn: Option<String>,
    pub vstdate: Option<Date>,
    pub qn: Option<String>,
    pub bedno: Option<String>,
    pub er_patient_status_id: Option<u32>,
    pub er_dch_type_id: Option<u32>,
}

impl QueryString for OpdErOrderMasterParams {
    fn from_tuples(params: &[(String, String)]) -> Option<Self> {
        (!params.is_empty()).then(|| Self {
            opd_er_order_master_id: find_qs(params, "opd_er_order_master_id").and_then(|s| s.parse::<u32>().ok()),
            order_doctor: find_qs(params, "order_doctor"),
            order_date: find_qs(params, "order_date").and_then(|s| date_8601(&s)),
            start_order_date: find_qs(params, "start_order_date").and_then(|s| date_8601(&s)),
            end_order_date: find_qs(params, "end_order_date").and_then(|s| date_8601(&s)),
            hn: find_qs(params, "hn"),
            vn: find_qs(params, "vn"),
            vstdate: find_qs(params, "vstdate").and_then(|s| date_8601(&s)),
            qn: find_qs(params, "qn"),
            bedno: find_qs(params, "bedno"),
            er_patient_status_id: find_qs(params, "er_patient_status_id").and_then(|s| s.parse::<u32>().ok()),
            er_dch_type_id: find_qs(params, "er_dch_type_id").and_then(|s| s.parse::<u32>().ok()),
        })
    }

    fn query_string(&self) -> String {
        let mut queries = Vec::with_capacity(12);
        if let Some(opd_er_order_master_id) = &self.opd_er_order_master_id {
            queries.push(["opd_er_order_master_id=", &opd_er_order_master_id.to_string()].concat());
        }
        if let Some(order_doctor) = &self.order_doctor {
            queries.push(["order_doctor=", order_doctor].concat());
        }
        if let Some(order_date) = &self.order_date {
            queries.push(["order_date=", &order_date.to_string()].concat());
        }
        if let Some(start_order_date) = &self.start_order_date {
            queries.push(["start_order_date=", &start_order_date.to_string()].concat());
        }
        if let Some(end_order_date) = &self.end_order_date {
            queries.push(["end_order_date=", &end_order_date.to_string()].concat());
        }
        if let Some(hn) = &self.hn {
            queries.push(["hn=", hn].concat());
        }
        if let Some(vn) = &self.vn {
            queries.push(["vn=", vn].concat());
        }
        if let Some(vstdate) = &self.vstdate {
            queries.push(["vstdate=", &vstdate.to_string()].concat());
        }
        if let Some(qn) = &self.qn {
            queries.push(["qn=", qn].concat());
        }
        if let Some(bedno) = &self.bedno {
            queries.push(["bedno=", bedno].concat());
        }
        if let Some(er_patient_status_id) = &self.er_patient_status_id {
            queries.push(["er_patient_status_id=", &er_patient_status_id.to_string()].concat());
        }
        if let Some(er_dch_type_id) = &self.er_dch_type_id {
            queries.push(["er_dch_type_id=", &er_dch_type_id.to_string()].concat());
        }

        (!queries.is_empty()).then(|| ["?", &queries.join("&")].concat()).unwrap_or_default()
    }
}

/// OPD-ER Order Master for List
#[derive(Clone, Demo, Deserialize, Serialize, FromRow, ToSchema)]
#[schema(example = json!(OpdErOrderMasterList::demo()))]
pub struct OpdErOrderMasterList {
    #[Demo(value = "1")]
    pub opd_er_order_master_id: u32,
    #[Demo(value = "Some(1)")]
    pub bedno: Option<u32>,
    #[Demo(value = r#"Some(String::from("1"))"#)]
    pub display_bedno: Option<String>,
    #[Demo(value = r#"Some(String::from("R"))"#)]
    pub bed_type: Option<String>,
    #[Demo(value = r#"Some(String::from("แดง"))"#)]
    pub bed_type_name: Option<String>,
    #[Demo(value = r##"Some(String::from("#e47e7e"))"##)]
    pub bed_type_color: Option<String>,
    #[Demo(value = "Some(1)")]
    pub bed_type_display_order: Option<i32>,
    #[Demo(value = "Some(1)")]
    pub er_patient_status_id: Option<u32>,
    #[Demo(value = r#"Some(String::from("รอตรวจ"))"#)]
    pub er_patient_status_name: Option<String>,
    #[Demo(value = "Some(1)")]
    pub opd_er_patient_status_display_order: Option<i16>, // list
    #[Demo(value = "Some(1)")]
    pub er_dch_type_id: Option<u32>,
    #[Demo(value = r#"Some(String::from("กลับบ้าน"))"#)]
    pub er_dch_type_name: Option<String>,
    #[Demo(value = "Some(1)")]
    pub opd_er_dch_type_display_order: Option<i16>, // list
    #[Demo(value = "Some(date!(2023-12-31))")]
    pub discharge_date: Option<Date>,
    #[Demo(value = "Some(time!(23:59:59))")]
    pub discharge_time: Option<Time>,
    #[Demo(value = r#"Some(datetime!(2023-12-31 23:59:59))"#)]
    pub discharge_date_time: Option<PrimitiveDateTime>, // list
    #[Demo(value = r#"Some(String::from("Note"))"#)]
    pub note: Option<String>,
    #[Demo(value = r#"Some(String::from("661231235959"))"#)]
    pub vn: Option<String>,
    #[Demo(value = "Some(1)")]
    pub oqueue: Option<i32>,
    #[Demo(value = r#"Some(String::from("0001234"))"#)]
    pub hn: Option<String>,
    #[Demo(value = r#"Some(String::from("660001234"))"#)]
    pub an: Option<String>,
    #[Demo(value = "Some(date!(2023-12-31))")]
    pub vstdate: Option<Date>,
    #[Demo(value = "Some(time!(23:59:59))")]
    pub vsttime: Option<Time>,
    #[Demo(value = r#"Some(datetime!(2023-12-31 23:59:59))"#)]
    pub vstdate_time: Option<PrimitiveDateTime>,
    #[Demo(value = "Some(date!(2023-12-31))")]
    pub order_date: Option<Date>,
    #[Demo(value = "Some(time!(23:59:59))")]
    pub order_time: Option<Time>,
    #[Demo(value = r#"Some(datetime!(2023-12-31 23:59:59))"#)]
    pub order_date_time: Option<PrimitiveDateTime>,
    #[Demo(value = r#"String::from("007")"#)]
    pub order_doctor: String,
    #[Demo(value = r#"Some(String::from("Dr.Doctor"))"#)]
    pub order_doctor_name: Option<String>,
    #[Demo(value = "Some(true)")]
    pub order_doctor_is_intern: Option<bool>,
    #[Demo(value = r#"Some(String::from("Mr.Patient Socker"))"#)]
    pub ptname: Option<String>,
    #[Demo(value = "Some(date!(2023-12-31))")]
    pub birthday: Option<Date>,
    #[Demo(value = "Some(33)")]
    pub age_y: Option<i16>,
    #[Demo(value = "Some(3)")]
    pub age_m: Option<i16>,
    #[Demo(value = "Some(3)")]
    pub age_d: Option<i16>,
    #[Demo(value = r#"Some(String::from("ชาย"))"#)]
    pub sex_name: Option<String>,
    #[Demo(value = r#"Some(String::from("UC"))"#)]
    pub rtcode: Option<String>,
    #[Demo(value = r#"Some(String::from("บัตรประกันสุขภาพถ้วนหน้า ในเขต"))"#)]
    pub rtname: Option<String>,
    #[Demo(value = r#"Some(String::from("Dr.Doctor"))"#)]
    pub all_order_doctor_name: Option<String>,
    #[Demo(value = r#"Some(datetime!(2023-12-31 23:59:59))"#)]
    pub max_order_date_time: Option<PrimitiveDateTime>,
    #[Demo(value = "1")]
    pub count_nurse_not_accept: i64,
    #[Demo(value = "1")]
    pub count_discharge_order: i64,
    #[Demo(value = "1")]
    pub count_stat_order_nurse_not_accept: i64,
    #[Demo(value = r#"Some(String::from("2023-12-31 23:59:59|37.0|86|20|112|||1|||96|||1|||8.5|4|5|6||1"))"#)]
    pub ews_concat: Option<String>,
}

impl OpdErOrderMasterList {
    /// GET `EndPoint::OpdErOrderMaster`
    pub async fn call_api_get(params: &OpdErOrderMasterParams, app: Rc<AppState>) -> Result<Vec<Self>, AppError> {
        match fetch_json_api(&[EndPoint::OpdErOrderMaster.base(), params.clone().query_string()].concat(), "GET", None, app).await {
            Ok((response, true)) => {
                let response: Vec<Self> = serde_wasm_bindgen::from_value(response).map_err(|e| Source::SerdeWasm.to_teapot_error(e, "Fetch OrderMasterList"))?;
                Ok(response)
            }
            Ok((app_error, false)) => {
                let error: AppError = serde_wasm_bindgen::from_value(app_error).map_err(|e| Source::SerdeWasm.to_teapot_error(e, "Fetch OrderMasterList"))?;
                Err(error)
            }
            Err(e) => Err(Source::Js.to_teapot_error(e.dyn_ref::<JsString>().map(|s| s.into()).unwrap_or(String::from("fetch error")), "Fetch Json")),
        }
    }
}

/// OPD-ER Order Master
#[derive(Clone, Demo, Deserialize, Serialize, FromRow, ToSchema)]
#[schema(example = json!(OpdErOrderMaster::demo()))]
pub struct OpdErOrderMaster {
    #[Demo(value = "1")]
    pub opd_er_order_master_id: u32,
    #[Demo(value = "Some(1)")]
    pub bedno: Option<u32>,
    #[Demo(value = r#"Some(String::from("1"))"#)]
    pub display_bedno: Option<String>,
    #[Demo(value = r#"Some(String::from("R"))"#)]
    pub bed_type: Option<String>,
    #[Demo(value = r#"Some(String::from("แดง"))"#)]
    pub bed_type_name: Option<String>,
    #[Demo(value = r##"Some(String::from("#e47e7e"))"##)]
    pub bed_type_color: Option<String>,
    #[Demo(value = "Some(1)")]
    pub bed_type_display_order: Option<i32>,
    #[Demo(value = "Some(1)")]
    pub er_patient_status_id: Option<u32>,
    #[Demo(value = r#"Some(String::from("รอตรวจ"))"#)]
    pub er_patient_status_name: Option<String>,
    #[Demo(value = "Some(1)")]
    pub er_dch_type_id: Option<u32>,
    #[Demo(value = r#"Some(String::from("กลับบ้าน"))"#)]
    pub er_dch_type_name: Option<String>,
    #[Demo(value = "Some(date!(2023-12-31))")]
    pub discharge_date: Option<Date>,
    #[Demo(value = "Some(time!(23:59:59))")]
    pub discharge_time: Option<Time>,
    #[Demo(value = r#"Some(String::from("N"))"#)]
    pub pass_24_hour_from_dch: Option<String>,
    #[Demo(value = r#"Some(String::from("Note"))"#)]
    pub note: Option<String>,
    #[Demo(value = r#"Some(String::from("661231235959"))"#)]
    pub vn: Option<String>,
    #[Demo(value = "Some(1)")]
    pub oqueue: Option<i32>,
    #[Demo(value = r#"Some(String::from("0001234"))"#)]
    pub hn: Option<String>,
    #[Demo(value = r#"Some(String::from("660001234"))"#)]
    pub an: Option<String>,
    // #[Demo(value = r#"Some(String::from("N"))"#)]
    // pub admit_flag: Option<String>,
    #[Demo(value = "Some(date!(2023-12-31))")]
    pub vstdate: Option<Date>,
    #[Demo(value = "Some(time!(23:59:59))")]
    pub vsttime: Option<Time>,
    #[Demo(value = r#"Some(datetime!(2023-12-31 23:59:59))"#)]
    pub vstdate_time: Option<PrimitiveDateTime>,
    #[Demo(value = "Some(date!(2023-12-31))")]
    pub order_date: Option<Date>,
    #[Demo(value = "Some(time!(23:59:59))")]
    pub order_time: Option<Time>,
    #[Demo(value = r#"Some(datetime!(2023-12-31 23:59:59))"#)]
    pub order_date_time: Option<PrimitiveDateTime>,
    #[Demo(value = r#"String::from("007")"#)]
    pub order_doctor: String,
    #[Demo(value = r#"Some(String::from("Dr.Doctor"))"#)]
    pub order_doctor_name: Option<String>,
    #[Demo(value = "Some(true)")]
    pub order_doctor_is_intern: Option<bool>,
    #[Demo(value = r#"Some(String::from("Mr.Patient Sicker"))"#)]
    pub ptname: Option<String>,
    #[Demo(value = "Some(33)")]
    pub age_y: Option<i16>,
    #[Demo(value = "Some(3)")]
    pub age_m: Option<i16>,
    #[Demo(value = "Some(3)")]
    pub age_d: Option<i16>,
    #[Demo(value = r#"Some(String::from("PENICILLIN"))"#)]
    pub er_drugallergy_history: Option<String>,
}

impl OpdErOrderMaster {
    /// GET `EndPoint::OpdErOrderMasterId`<br>
    /// zero = None
    pub async fn call_api_get(opd_er_order_master_id: u32, app: Rc<AppState>) -> Result<Option<Self>, AppError> {
        match fetch_json_api(&[EndPoint::OpdErOrderMasterId.base(), opd_er_order_master_id.to_string()].concat(), "GET", None, app).await {
            Ok((response, true)) => {
                let response: Option<Self> = serde_wasm_bindgen::from_value(response).map_err(|e| Source::SerdeWasm.to_teapot_error(e, "Fetch OrderMaster"))?;
                Ok(response)
            }
            Ok((app_error, false)) => {
                let error: AppError = serde_wasm_bindgen::from_value(app_error).map_err(|e| Source::SerdeWasm.to_teapot_error(e, "Fetch OrderMaster"))?;
                Err(error)
            }
            Err(e) => Err(Source::Js.to_teapot_error(e.dyn_ref::<JsString>().map(|s| s.into()).unwrap_or(String::from("fetch error")), "Fetch Json")),
        }
    }
}

/// OPD-ER Order Master for save
#[derive(Debug, Default, Demo, Deserialize, Serialize, ToSchema)]
#[schema(example = json!(OpdErOrderMasterSave::demo()))]
pub struct OpdErOrderMasterSave {
    #[Demo(value = "Some(1)")]
    pub opd_er_order_master_id: Option<u32>,
    #[Demo(value = r#"Some(String::from("661231235959"))"#)]
    pub vn: Option<String>,
    // #[Demo(value = r#"Some(String::from("660001234"))"#)]
    // pub an: Option<String>,
    #[Demo(value = r#"Some(String::from("Note"))"#)]
    pub note: Option<String>,
    #[Demo(value = "Some(1)")]
    pub bedno: Option<u32>,
    #[Demo(value = "Some(1)")]
    pub er_patient_status_id: Option<u32>,
    #[Demo(value = "Some(1)")]
    pub er_dch_type_id: Option<u32>,
    #[Demo(value = "Some(date!(2023-12-31))")]
    pub discharge_date: Option<Date>,
    #[Demo(value = "Some(time!(23:59:59))")]
    pub discharge_time: Option<Time>,
}

impl OpdErOrderMasterSave {
    /// POST `EndPoint::OpdErOrderMaster`
    pub async fn call_api_post(&self, app: Rc<AppState>) -> Result<(u32, ExecuteResponse), AppError> {
        let body_json = serde_json::to_string(self).map_err(|e| Source::SerdeJson.to_teapot_error(e, "Send OrderMasterSave"))?;

        let body = serde_wasm_bindgen::to_value(&body_json).map_err(|e| Source::SerdeWasm.to_teapot_error(e, "Send OrderMasterSave"))?;

        execute_fetch_with_u32(&EndPoint::OpdErOrderMaster.base(), "POST", Some(&body), app).await
    }
}
