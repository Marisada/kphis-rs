use derive_demo::Demo;
use js_sys::JsString;
use serde_derive::{Deserialize, Serialize};
use sqlx::{
    FromRow,
    types::time::{Date, PrimitiveDateTime, Time},
};
use std::rc::Rc;
use time::macros::{date, datetime, time};
use utoipa::{IntoParams, ToSchema};
use wasm_bindgen::JsCast;

use kphis_util::{
    error::{AppError, Source},
    util::str_some,
};

use crate::{
    app::AppState,
    endpoint::{EndPoint, QueryString, find_qs},
    fetch::{ExecuteResponse, execute_fetch_vec, execute_fetch_vec_with_u32, fetch_json_api},
};

#[derive(Clone, Debug, Deserialize, Serialize, IntoParams)]
pub struct ConsultParams {
    #[param(required, example = 1)]
    pub consult_id: Option<u32>,
    #[param(required, example = 1)]
    pub version: Option<i32>,
}

impl QueryString for ConsultParams {
    fn from_tuples(params: &[(String, String)]) -> Option<Self> {
        (!params.is_empty()).then(|| Self {
            consult_id: find_qs(params, "consult_id").and_then(|s| s.parse::<u32>().ok()),
            version: find_qs(params, "version").and_then(|s| s.parse::<i32>().ok()),
        })
    }

    fn query_string(&self) -> String {
        let mut queries = Vec::with_capacity(2);
        if let Some(consult_id) = &self.consult_id {
            queries.push(["consult_id=", &consult_id.to_string()].concat());
        }
        if let Some(version) = &self.version {
            queries.push(["version=", &version.to_string()].concat());
        }

        (!queries.is_empty()).then(|| ["?", &queries.join("&")].concat()).unwrap_or_default()
    }
}

#[derive(Clone, Debug, Default, Deserialize, Serialize, IntoParams)]
pub struct IpdConsultListParams {
    pub spclty: Option<String>,
    pub search_consult_status: Option<String>,
    pub consult_dr_search: Option<String>,
    pub consult_dr_reply_search: Option<String>,
    pub search_consult_emergency: Option<String>,
    pub patient: Option<String>,
}

impl QueryString for IpdConsultListParams {
    fn from_tuples(params: &[(String, String)]) -> Option<Self> {
        (!params.is_empty()).then(|| Self {
            spclty: find_qs(params, "spclty"),
            search_consult_status: find_qs(params, "search_consult_status"),
            consult_dr_search: find_qs(params, "consult_dr_search"),
            consult_dr_reply_search: find_qs(params, "consult_dr_reply_search"),
            search_consult_emergency: find_qs(params, "search_consult_emergency"),
            patient: find_qs(params, "patient"),
        })
    }

    fn query_string(&self) -> String {
        let mut queries = Vec::with_capacity(6);
        if let Some(spclty) = &self.spclty {
            queries.push(["spclty=", spclty].concat());
        }
        if let Some(search_consult_status) = &self.search_consult_status {
            queries.push(["search_consult_status=", search_consult_status].concat());
        }
        if let Some(consult_dr_search) = &self.consult_dr_search {
            queries.push(["consult_dr_search=", consult_dr_search].concat());
        }
        if let Some(consult_dr_reply_search) = &self.consult_dr_reply_search {
            queries.push(["consult_dr_reply_search=", consult_dr_reply_search].concat());
        }
        if let Some(search_consult_emergency) = &self.search_consult_emergency {
            queries.push(["search_consult_emergency=", search_consult_emergency].concat());
        }
        if let Some(patient) = &self.patient {
            queries.push(["patient=", patient].concat());
        }

        (!queries.is_empty()).then(|| ["?", &queries.join("&")].concat()).unwrap_or_default()
    }
}

impl IpdConsultListParams {
    pub fn not_empty(self) -> Self {
        Self {
            spclty: self.spclty.and_then(str_some),
            search_consult_status: self.search_consult_status.and_then(str_some),
            consult_dr_search: self.consult_dr_search.and_then(str_some),
            consult_dr_reply_search: self.consult_dr_reply_search.and_then(str_some),
            search_consult_emergency: self.search_consult_emergency.and_then(str_some),
            patient: self.patient.and_then(str_some),
        }
    }
}

/// IPD Consult for table
#[derive(Demo, Deserialize, Serialize, FromRow, ToSchema)]
#[schema(example = json!(IpdConsultList::demo()))]
pub struct IpdConsultList {
    #[Demo(value = "1")]
    pub consult_id: u32,
    #[Demo(value = r#"Some(String::from("0001234"))"#)]
    pub hn: Option<String>,
    #[Demo(value = r#"Some(String::from("660001234"))"#)]
    pub an: Option<String>,
    #[Demo(value = r#"Some(String::from("C01"))"#)]
    pub bedno: Option<String>,
    #[Demo(value = r#"Some(String::from("Mr.Patient Sicker"))"#)]
    pub fullname: Option<String>,
    #[Demo(value = "Some(8888.8)")]
    pub income: Option<f64>,
    #[Demo(value = r#"Some(datetime!(2023-12-31 23:59:59))"#)]
    pub regdatetime: Option<PrimitiveDateTime>,
    #[Demo(value = "Some(1)")]
    pub admdate: Option<i32>,
    #[Demo(value = "Some(33)")]
    pub age_y: Option<i8>,
    #[Demo(value = "Some(3)")]
    pub age_m: Option<i8>,
    #[Demo(value = "Some(3)")]
    pub age_d: Option<i8>,
    #[Demo(value = r#"Some(String::from("ชาย"))"#)]
    pub sex_name: Option<String>,
    #[Demo(value = r#"Some(String::from("UC"))"#)]
    pub rtcode: Option<String>,
    #[Demo(value = r#"Some(String::from("บัตรประกันสุขภาพถ้วนหน้า ในเขต"))"#)]
    pub rtname: Option<String>,
    #[Demo(value = r#"Some(String::from("Dr.Doctor, Dr.Another"))"#)]
    pub kphis_incharge_doctor_name: Option<String>,
    #[Demo(value = r#"Some(String::from("Dr.Doctor"))"#)]
    pub string_consult_reply_name: Option<String>,
    #[Demo(value = r#"Some(String::from("อายุรกรรมชาย"))"#)]
    pub spclty_name: Option<String>,
    #[Demo(value = r#"Some(String::from("Dr.Doctor"))"#)]
    pub consult_doctorcode_mention_name: Option<String>,
    #[Demo(value = r#"Some(String::from("ตึกชาย"))"#)]
    pub ward_name: Option<String>,
    #[Demo(value = r#"Some(String::from("Y"))"#)]
    pub consult_status: Option<String>,
    #[Demo(value = "Some(date!(2023-12-31))")]
    pub consult_date: Option<Date>,
    #[Demo(value = "Some(time!(23:59:59))")]
    pub consult_time: Option<Time>,
    #[Demo(value = r#"Some(String::from("1"))"#)]
    pub consult_emergency: Option<String>,
    #[Demo(value = "Some(datetime!(2023-12-31 23:59:59))")]
    pub consult_datetime_create_reply: Option<PrimitiveDateTime>,
    #[Demo(value = "Some(datetime!(2023-12-31 23:59:59))")]
    pub consult_datetime_update_reply: Option<PrimitiveDateTime>,
}

impl IpdConsultList {
    /// GET `EndPoint::IpdConsult`
    pub async fn call_api_get(request: &IpdConsultListParams, app: Rc<AppState>) -> Result<Vec<Self>, AppError> {
        match fetch_json_api(&[EndPoint::IpdConsult.base(), request.clone().query_string()].concat(), "GET", None, app).await {
            Ok((response, true)) => {
                let response: Vec<Self> = serde_wasm_bindgen::from_value(response).map_err(|e| Source::SerdeWasm.to_teapot_error(e, "Fetch IpdConsultList"))?;
                Ok(response)
            }
            Ok((app_error, false)) => {
                let error: AppError = serde_wasm_bindgen::from_value(app_error).map_err(|e| Source::SerdeWasm.to_teapot_error(e, "Fetch IpdConsultList"))?;
                Err(error)
            }
            Err(e) => Err(Source::Js.to_teapot_error(e.dyn_ref::<JsString>().map(|s| s.into()).unwrap_or(String::from("fetch error")), "Fetch Json")),
        }
    }
}

/// IPD Consult with all name
#[derive(Clone, Default, Demo, Deserialize, Serialize, FromRow, ToSchema)]
#[schema(example = json!(ConsultWithName::demo()))]
pub struct ConsultWithName {
    #[Demo(value = "1")]
    pub consult_id: u32,
    #[Demo(value = "Some(1)")]
    pub consult_type: Option<u32>,
    #[Demo(value = r#"Some(String::from("01"))"#)]
    pub consult_ward: Option<String>,
    #[Demo(value = r#"Some(String::from("1"))"#)]
    pub consult_emergency: Option<String>,
    #[Demo(value = r#"Some(String::from("007"))"#)]
    pub consult_doctorcode_mention: Option<String>,
    #[Demo(value = r#"Some(String::from("1"))"#)]
    pub consult_spclty: Option<String>,
    #[Demo(value = "Some(date!(2023-12-31))")]
    pub consult_date: Option<Date>,
    #[Demo(value = "Some(time!(23:59:59))")]
    pub consult_time: Option<Time>,
    #[Demo(value = r#"Some(String::from("Consult detail"))"#)]
    pub consult_data: Option<String>,
    #[Demo(value = "Some(datetime!(2023-12-31 23:59:59))")]
    pub consult_datetime_create_reply: Option<PrimitiveDateTime>,
    #[Demo(value = "Some(datetime!(2023-12-31 23:59:59))")]
    pub consult_datetime_update_reply: Option<PrimitiveDateTime>,
    #[Demo(value = r#"Some(String::from("Findings"))"#)]
    pub consult_finding: Option<String>,
    #[Demo(value = r#"Some(String::from("Diagnosis"))"#)]
    pub consult_diagnosis: Option<String>,
    #[Demo(value = r#"Some(String::from("Recommendations"))"#)]
    pub consult_recommendation: Option<String>,
    #[Demo(value = r#"Some(String::from("Y"))"#)]
    pub consult_status: Option<String>,
    #[Demo(value = r#"String::from("660001234")"#)]
    pub an: String,

    #[Demo(value = r#"Some(String::from("อายุรกรรมชาย"))"#)]
    pub spcltyname: Option<String>,
    #[Demo(value = r#"Some(String::from("ใบ Consult ทั่วไป"))"#)]
    pub consult_type_name: Option<String>,
    #[Demo(value = r#"Some(String::from("ด่วน"))"#)]
    pub consult_emergency_name: Option<String>,
    #[Demo(value = r#"Some(String::from("Dr.Doctor"))"#)]
    pub consult_doctorcode_mention_name: Option<String>,
    #[Demo(value = r#"Some(String::from("Dr.Doctor"))"#)]
    pub string_consult_request_name: Option<String>,
    #[Demo(value = r#"Some(String::from("Dr.Doctor"))"#)]
    pub string_consult_reply_name: Option<String>,

    #[Demo(value = r#"Some(String::from("009/99/91NWC10PTESHVK95PNAAA.webp,009/99/91NWC10PTESHVK95PNAAB.webp"))"#)]
    pub d_imgs: Option<String>,
    #[Demo(value = r#"Some(String::from("009/99/91NWC10PTESHVK95PNAAA.webp,009/99/91NWC10PTESHVK95PNAAB.webp"))"#)]
    pub f_imgs: Option<String>,
}

impl ConsultWithName {
    /// GET `EndPoint::IpdConsultAn`
    pub async fn call_api_get(an: &str, app: Rc<AppState>) -> Result<Vec<Self>, AppError> {
        match fetch_json_api(&[&EndPoint::IpdConsultAn.base(), an].concat(), "GET", None, app).await {
            Ok((response, true)) => {
                let response: Vec<Self> = serde_wasm_bindgen::from_value(response).map_err(|e| Source::SerdeWasm.to_teapot_error(e, "Fetch ConsultWithName"))?;
                Ok(response)
            }
            Ok((app_error, false)) => {
                let error: AppError = serde_wasm_bindgen::from_value(app_error).map_err(|e| Source::SerdeWasm.to_teapot_error(e, "Fetch ConsultWithName"))?;
                Err(error)
            }
            Err(e) => Err(Source::Js.to_teapot_error(e.dyn_ref::<JsString>().map(|s| s.into()).unwrap_or(String::from("fetch error")), "Fetch Json")),
        }
    }
}

/// IPD Consult with version
#[derive(Clone, Default, Demo, Deserialize, Serialize, FromRow, ToSchema)]
#[schema(example = json!(Consult::demo()))]
pub struct Consult {
    #[Demo(value = "1")]
    pub consult_id: u32,
    #[Demo(value = "Some(1)")]
    pub consult_type: Option<u32>,
    #[Demo(value = r#"Some(String::from("01"))"#)]
    pub consult_ward: Option<String>,
    #[Demo(value = r#"Some(String::from("1"))"#)]
    pub consult_emergency: Option<String>,
    #[Demo(value = r#"Some(String::from("007"))"#)]
    pub consult_doctorcode_mention: Option<String>,
    #[Demo(value = r#"Some(String::from("1"))"#)]
    pub consult_spclty: Option<String>,
    #[Demo(value = "Some(date!(2023-12-31))")]
    pub consult_date: Option<Date>,
    #[Demo(value = "Some(time!(23:59:59))")]
    pub consult_time: Option<Time>,
    #[Demo(value = r#"Some(String::from("Consult detail"))"#)]
    pub consult_data: Option<String>,
    #[Demo(value = "Some(datetime!(2023-12-31 23:59:59))")]
    pub consult_datetime_create_reply: Option<PrimitiveDateTime>,
    #[Demo(value = "Some(datetime!(2023-12-31 23:59:59))")]
    pub consult_datetime_update_reply: Option<PrimitiveDateTime>,
    #[Demo(value = r#"Some(String::from("Findings"))"#)]
    pub consult_finding: Option<String>,
    #[Demo(value = r#"Some(String::from("Diagnosis"))"#)]
    pub consult_diagnosis: Option<String>,
    #[Demo(value = r#"Some(String::from("Recommendations"))"#)]
    pub consult_recommendation: Option<String>,
    #[Demo(value = r#"Some(String::from("Y"))"#)]
    pub consult_status: Option<String>,
    #[Demo(value = r#"String::from("660001234")"#)]
    pub an: String,
    #[Demo(value = "1")]
    pub version: i32,

    #[Demo(value = r#"Some(String::from("Dr.Doctor"))"#)]
    pub string_consult_request_name: Option<String>,
    #[Demo(value = r#"Some(String::from("Dr.Doctor"))"#)]
    pub string_consult_reply_name: Option<String>,
}

impl Consult {
    /// GET `EndPoint::IpdConsultId`
    pub async fn call_api_get(consult_id: u32, app: Rc<AppState>) -> Result<Option<Self>, AppError> {
        match fetch_json_api(&[EndPoint::IpdConsultId.base(), consult_id.to_string()].concat(), "GET", None, app).await {
            Ok((response, true)) => {
                let response: Option<Self> = serde_wasm_bindgen::from_value(response).map_err(|e| Source::SerdeWasm.to_teapot_error(e, "Fetch Consult"))?;
                Ok(response)
            }
            Ok((app_error, false)) => {
                let error: AppError = serde_wasm_bindgen::from_value(app_error).map_err(|e| Source::SerdeWasm.to_teapot_error(e, "Fetch Consult"))?;
                Err(error)
            }
            Err(e) => Err(Source::Js.to_teapot_error(e.dyn_ref::<JsString>().map(|s| s.into()).unwrap_or(String::from("fetch error")), "Fetch Json")),
        }
    }

    /// DELETE `EndPoint::IpdConsult`
    pub async fn call_api_delete(params: &ConsultParams, app: Rc<AppState>) -> Result<Vec<ExecuteResponse>, AppError> {
        execute_fetch_vec(&[EndPoint::IpdConsult.base(), params.query_string()].concat(), "DELETE", None, app).await
    }
}

/// IPD Consult for save
#[derive(Debug, Demo, Deserialize, Serialize, ToSchema)]
#[schema(example = json!(ConsultSave::demo()))]
pub struct ConsultSave {
    #[Demo(value = "Some(1)")]
    pub consult_id: Option<u32>,
    /// edit, reply
    #[Demo(value = r#"String::from("reply")"#)]
    pub consult_mode: String,

    #[Demo(value = r#"String::from("660001234")"#)]
    pub an: String,
    #[Demo(value = "Some(1)")]
    pub consult_type: Option<u32>,
    #[Demo(value = r#"Some(String::from("01"))"#)]
    pub consult_ward: Option<String>,
    #[Demo(value = r#"Some(String::from("1"))"#)]
    pub consult_emergency: Option<String>,
    #[Demo(value = r#"Some(String::from("007"))"#)]
    pub consult_doctorcode_mention: Option<String>,
    #[Demo(value = r#"Some(1)"#)]
    pub consult_spclty: Option<u32>,
    #[Demo(value = "Some(date!(2023-12-31))")]
    pub consult_date: Option<Date>,
    #[Demo(value = "Some(time!(23:59:59))")]
    pub consult_time: Option<Time>,
    #[Demo(value = r#"Some(String::from("Consult detail"))"#)]
    pub consult_data: Option<String>,

    #[Demo(value = "vec![DoctorCodeSave::demo()]")]
    pub consult_doctorcode_requests: Vec<DoctorCodeSave>,

    #[Demo(value = r#"Some(String::from("Findings"))"#)]
    pub consult_finding: Option<String>,
    #[Demo(value = r#"Some(String::from("Diagnosis"))"#)]
    pub consult_diagnosis: Option<String>,
    #[Demo(value = r#"Some(String::from("Recommendations"))"#)]
    pub consult_recommendation: Option<String>,

    #[Demo(value = "vec![DoctorCodeSave::demo()]")]
    pub consult_doctorcode_replies: Vec<DoctorCodeSave>,

    #[Demo(value = "1")]
    pub version: i32,
}

impl ConsultSave {
    /// POST `EndPoint::IpdConsult`
    pub async fn call_api_post(&self, app: Rc<AppState>) -> Result<(u32, Vec<ExecuteResponse>), AppError> {
        let body_json = serde_json::to_string(self).map_err(|e| Source::SerdeJson.to_teapot_error(e, "Send ConsultSave"))?;

        let body = serde_wasm_bindgen::to_value(&body_json).map_err(|e| Source::SerdeWasm.to_teapot_error(e, "Send ConsultSave"))?;

        execute_fetch_vec_with_u32(&EndPoint::IpdConsult.base(), "POST", Some(&body), app).await
    }
}

/// Doctor Code for IPD Consult
#[derive(Debug, Demo, Deserialize, Serialize, ToSchema)]
#[schema(example = json!(DoctorCodeSave::demo()))]
pub struct DoctorCodeSave {
    #[Demo(value = r#"Some(String::from("007"))"#)]
    pub person1: Option<String>,
    #[Demo(value = r#"Some(String::from("009"))"#)]
    pub person2: Option<String>,
}
