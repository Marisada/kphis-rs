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
    fetch::fetch_json_api,
};

#[derive(Clone, Debug, Default, Deserialize, Serialize, IntoParams)]
pub struct PostAdmitParams {
    pub ward: Option<String>,
    pub inscl: Option<String>,
    pub adm_doctor: Option<String>,
    pub dch_doctor: Option<String>,
    pub patient: Option<String>,
    pub passcode: Option<String>,
    pub start_dchdate: Option<Date>,
    pub end_dchdate: Option<Date>,
    pub summary_status: Option<String>,
}

impl QueryString for PostAdmitParams {
    fn from_tuples(params: &[(String, String)]) -> Option<Self> {
        (!params.is_empty()).then(|| Self {
            ward: find_qs(params, "ward"),
            inscl: find_qs(params, "inscl"),
            adm_doctor: find_qs(params, "adm_doctor"),
            dch_doctor: find_qs(params, "dch_doctor"),
            patient: find_qs(params, "patient"),
            passcode: find_qs(params, "passcode"),
            start_dchdate: find_qs(params, "start_dchdate").and_then(|s| date_8601(&s)),
            end_dchdate: find_qs(params, "end_dchdate").and_then(|s| date_8601(&s)),
            summary_status: find_qs(params, "summary_status"),
        })
    }

    fn query_string(&self) -> String {
        let mut queries = Vec::with_capacity(9);
        if let Some(ward) = &self.ward {
            queries.push(["ward=", ward].concat());
        }
        if let Some(inscl) = &self.inscl {
            queries.push(["inscl=", inscl].concat());
        }
        if let Some(adm_doctor) = &self.adm_doctor {
            queries.push(["adm_doctor=", adm_doctor].concat());
        }
        if let Some(dch_doctor) = &self.dch_doctor {
            queries.push(["dch_doctor=", dch_doctor].concat());
        }
        if let Some(patient) = &self.patient {
            queries.push(["patient=", patient].concat());
        }
        if let Some(passcode) = &self.passcode {
            queries.push(["passcode=", passcode].concat());
        }
        if let Some(start_dchdate) = &self.start_dchdate {
            queries.push(["start_dchdate=", &start_dchdate.to_string()].concat());
        }
        if let Some(end_dchdate) = &self.end_dchdate {
            queries.push(["end_dchdate=", &end_dchdate.to_string()].concat());
        }
        if let Some(summary_status) = &self.summary_status {
            queries.push(["summary_status=", summary_status].concat());
        }

        (!queries.is_empty()).then(|| ["?", &queries.join("&")].concat()).unwrap_or_default()
    }
}

/// Post-Admit Patient List
#[derive(Demo, Deserialize, Serialize, FromRow, ToSchema)]
#[schema(example = json!(PostAdmitList::demo()))]
pub struct PostAdmitList {
    #[Demo(value = r#"Some(String::from("0001234"))"#)]
    pub hn: Option<String>,
    #[Demo(value = r#"String::from("660001234")"#)]
    pub an: String,
    #[Demo(value = r#"Some(String::from("ตึกชาย"))"#)]
    pub ward_name: Option<String>,
    #[Demo(value = "Some(date!(2023-12-31))")]
    pub regdate: Option<Date>,
    #[Demo(value = "Some(time!(23:59:59))")]
    pub regtime: Option<Time>,
    #[Demo(value = r#"Some(String::from("Mr.Patient Sicker"))"#)]
    pub fullname: Option<String>,
    #[Demo(value = r#"Some(String::from("Dr.Doctor"))"#)]
    pub admdoctor_name: Option<String>,
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
    #[Demo(value = r#"Some(String::from("01"))"#)]
    pub dchtype: Option<String>,
    #[Demo(value = r#"Some(String::from("With Approval"))"#)]
    pub dchtype_name: Option<String>,
    #[Demo(value = r#"Some(String::from("01"))"#)]
    pub dchstts: Option<String>,
    #[Demo(value = r#"Some(String::from("Complete Recovery"))"#)]
    pub dchstts_name: Option<String>,
    #[Demo(value = "Some(date!(2024-01-11))")]
    pub dchdate: Option<Date>,
    #[Demo(value = "Some(time!(23:59:59))")]
    pub dchtime: Option<Time>,
    #[Demo(value = r#"Some(String::from("Dr.Doctor"))"#)]
    pub dchdoctor_name: Option<String>,
    #[Demo(value = r#"Some(datetime!(2023-12-31 23:59:59))"#)]
    pub max_order_datetime: Option<PrimitiveDateTime>,
    #[Demo(value = r#"Some(datetime!(2023-12-31 23:59:59))"#)]
    pub max_progress_note_datetime: Option<PrimitiveDateTime>,
    /// ward passcode status
    #[Demo(value = "1")]
    pub wp_status: i8,
    #[Demo(value = r#"Some(String::from("review"))"#)]
    pub summary_status: Option<String>,
    #[Demo(value = "true")]
    pub dr_admission_note_exists: bool,
    #[Demo(value = "true")]
    pub attending_doctor_exists: bool,
    #[Demo(value = "true")]
    pub approve_doctor_exists: bool,
    #[Demo(value = "1")]
    pub summary_audit_count: i64,
    #[Demo(value = "1")]
    pub mra_count: i64,
}

impl PostAdmitList {
    /// GET `EndPoint::IpdPostAdmitList`
    pub async fn call_api_get(params: &PostAdmitParams, app: Rc<AppState>) -> Result<Vec<Self>, AppError> {
        match fetch_json_api(&[EndPoint::IpdPostAdmitList.base(), params.clone().query_string()].concat(), "GET", None, app).await {
            Ok((response, true)) => {
                let response: Vec<Self> = serde_wasm_bindgen::from_value(response).map_err(|e| Source::SerdeWasm.to_teapot_error(e, "Fetch PostAdmitList"))?;
                Ok(response)
            }
            Ok((app_error, false)) => {
                let error: AppError = serde_wasm_bindgen::from_value(app_error).map_err(|e| Source::SerdeWasm.to_teapot_error(e, "Fetch PostAdmitList"))?;
                Err(error)
            }
            Err(e) => Err(Source::Js.to_teapot_error(e.dyn_ref::<JsString>().map(|s| s.into()).unwrap_or(String::from("fetch error")), "Fetch Json")),
        }
    }
}

/// GET `EndPoint::IpdPostAdmitCount`
pub async fn get_post_admit_count(app: Rc<AppState>) -> Result<i64, AppError> {
    match fetch_json_api(&EndPoint::IpdPostAdmitCount.base(), "GET", None, app).await {
        Ok((response, true)) => {
            let response: i64 = serde_wasm_bindgen::from_value(response).map_err(|e| Source::SerdeWasm.to_teapot_error(e, "Fetch PostAdmitCount"))?;
            Ok(response)
        }
        Ok((app_error, false)) => {
            let error: AppError = serde_wasm_bindgen::from_value(app_error).map_err(|e| Source::SerdeWasm.to_teapot_error(e, "Fetch PostAdmitCount"))?;
            Err(error)
        }
        Err(e) => Err(Source::Js.to_teapot_error(e.dyn_ref::<JsString>().map(|s| s.into()).unwrap_or(String::from("fetch error")), "Fetch Json")),
    }
}
