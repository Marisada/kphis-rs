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

use kphis_util::error::{AppError, Source};

use crate::{
    app::AppState,
    endpoint::{EndPoint, QueryString, find_qs},
    fetch::{ExecuteResponse, execute_fetch, execute_fetch_vec, fetch_json_api},
};

#[derive(Clone, Debug, Default, Deserialize, Serialize, IntoParams)]
pub struct PreAdmitParams {
    pub status: Option<String>,
    pub doctor_in_charge: Option<String>,
    pub patient: Option<String>,
    pub all: Option<String>,
}

impl QueryString for PreAdmitParams {
    fn from_tuples(params: &[(String, String)]) -> Option<Self> {
        (!params.is_empty()).then(|| Self {
            status: find_qs(params, "status"),
            doctor_in_charge: find_qs(params, "doctor_in_charge"),
            patient: find_qs(params, "patient"),
            all: find_qs(params, "all"),
        })
    }

    fn query_string(&self) -> String {
        let mut queries = Vec::with_capacity(4);
        if let Some(status) = &self.status {
            queries.push(["status=", status].concat());
        }
        if let Some(doctor_in_charge) = &self.doctor_in_charge {
            queries.push(["doctor_in_charge=", doctor_in_charge].concat());
        }
        if let Some(patient) = &self.patient {
            queries.push(["patient=", patient].concat());
        }
        if let Some(all) = &self.all {
            queries.push(["all=", all].concat());
        }

        (!queries.is_empty()).then(|| ["?", &queries.join("&")].concat()).unwrap_or_default()
    }
}

#[derive(Demo, Serialize, Deserialize, FromRow, ToSchema)]
#[schema(example = json!(PreAdmitList::demo()))]
pub struct PreAdmitList {
    #[Demo(value = r#"String::from("661231235959")"#)]
    pub vn: String,
    #[Demo(value = r#"Some(String::from("0001234"))"#)]
    pub hn: Option<String>,
    #[Demo(value = r#"Some(String::from("660001234"))"#)]
    pub an: Option<String>,
    #[Demo(value = r#"Some(String::from("ตึกชาย"))"#)]
    pub ward_name: Option<String>,
    #[Demo(value = r#"Some(String::from("Mr.Patient Sicker"))"#)]
    pub fullname: Option<String>,
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
    #[Demo(value = "Some(date!(2023-12-31))")]
    pub vstdate: Option<Date>,
    #[Demo(value = "Some(time!(23:59:59))")]
    pub vsttime: Option<Time>,
    #[Demo(value = r#"Some(String::from("Dr.Doctor"))"#)]
    pub all_order_doctor_name: Option<String>,
    #[Demo(value = r#"Some(datetime!(2023-12-31 23:59:59))"#)]
    pub max_order_datetime: Option<PrimitiveDateTime>,
    #[Demo(value = "true")]
    pub dr_admission_note_exists: bool,
    #[Demo(value = "Some(1)")]
    pub mr_unconfirmed_count: Option<i64>,
    #[Demo(value = "Some(1)")]
    pub mr_confirmed_count: Option<i64>,
}

impl PreAdmitList {
    /// GET `EndPoint::IpdPreAdmit`
    pub async fn call_api_get(params: &PreAdmitParams, app: Rc<AppState>) -> Result<Vec<Self>, AppError> {
        match fetch_json_api(&[EndPoint::IpdPreAdmit.base(), params.clone().query_string()].concat(), "GET", None, app).await {
            Ok((response, true)) => {
                let response: Vec<Self> = serde_wasm_bindgen::from_value(response).map_err(|e| Source::SerdeWasm.to_teapot_error(e, "Fetch PreAdmitList"))?;
                Ok(response)
            }
            Ok((app_error, false)) => {
                let error: AppError = serde_wasm_bindgen::from_value(app_error).map_err(|e| Source::SerdeWasm.to_teapot_error(e, "Fetch PreAdmitList"))?;
                Err(error)
            }
            Err(e) => Err(Source::Js.to_teapot_error(e.dyn_ref::<JsString>().map(|s| s.into()).unwrap_or(String::from("fetch error")), "Fetch Json")),
        }
    }
}

#[derive(Demo, Deserialize, FromRow)]
pub struct PreAdmitOnly {
    #[Demo(value = "1")]
    pub pre_admit_master_id: u32,
    #[Demo(value = r#"String::from("661231235959")"#)]
    pub vn: String,
    #[Demo(value = r#"Some(String::from("660001234"))"#)]
    pub an: Option<String>,
    #[Demo(value = r#"Some(String::from("660001111"))"#)]
    pub prev_an: Option<String>,
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

/// IPD Pre-Order Master for save
#[derive(Debug, Demo, Deserialize, Serialize, ToSchema)]
#[schema(example = json!(PreAdmitSave::demo()))]
pub struct PreAdmitSave {
    #[Demo(value = r#"String::from("661231235959")"#)]
    pub vn: String,
}

impl PreAdmitSave {
    /// POST `EndPoint::IpdPreAdmit`
    pub async fn call_api_post(&self, app: Rc<AppState>) -> Result<ExecuteResponse, AppError> {
        let body_json = serde_json::to_string(self).map_err(|e| Source::SerdeJson.to_teapot_error(e, "Send PreAdmitSave"))?;

        let body = serde_wasm_bindgen::to_value(&body_json).map_err(|e| Source::SerdeWasm.to_teapot_error(e, "Send PreAdmitSave"))?;

        execute_fetch(&EndPoint::IpdPreAdmit.base(), "POST", Some(&body), app).await
    }
}

/// Patch command for IPD Pre-Order Master for save
#[derive(Debug, Demo, Deserialize, Serialize, ToSchema)]
#[schema(example = json!(PreAdmitPatch::demo_sync_an(String::from("660001234"))))]
pub enum PreAdmitPatch {
    /// revoke AN (if exists) and change all data to VN
    RevokeAn(String),
    /// revoke (VN, AN) and change all data to VN
    RevokeVnAn(String, String),
    /// Check An in `ipt` and `ipt_pre_admit_master` and correct them
    SyncAn(String),
    /// Check Vn in `ipt` and `ipt_pre_admit_master` and correct them
    SyncVn(String),
    // /// Forced setting AN and change all data (`with-VN`, `to-AN`)
    // SetAn(String, String),
}

impl PreAdmitPatch {
    /// PATCH `EndPoint::IpdPreAdmit`
    pub async fn call_api_patch(&self, app: Rc<AppState>) -> Result<Vec<ExecuteResponse>, AppError> {
        let body_json = serde_json::to_string(self).map_err(|e| Source::SerdeJson.to_teapot_error(e, "Send PreAdmitPatch"))?;

        let body = serde_wasm_bindgen::to_value(&body_json).map_err(|e| Source::SerdeWasm.to_teapot_error(e, "Send PreAdmitPatch"))?;

        execute_fetch_vec(&EndPoint::IpdPreAdmit.base(), "PATCH", Some(&body), app).await
    }
}
