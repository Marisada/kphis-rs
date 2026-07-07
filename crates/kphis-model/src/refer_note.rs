use derive_demo::Demo;
use js_sys::JsString;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use std::rc::Rc;
use time::{
    Date, PrimitiveDateTime, Time,
    macros::{date, datetime, time},
};
use utoipa::ToSchema;
use wasm_bindgen::JsCast;

use kphis_util::error::{AppError, Source};

use crate::{
    app::AppState,
    endpoint::EndPoint,
    fetch::{ExecuteResponse, execute_fetch, fetch_json_api},
};

/// KPHIS refernote
#[derive(Clone, Debug, Demo, Deserialize, Serialize, FromRow, ToSchema)]
#[schema(example = json!(ReferNote::demo()))]
pub struct ReferNote {
    #[Demo(value = "1")]
    pub refernote_id: u32,
    #[Demo(value = r#"String::from("660001234")"#)]
    pub vn: String,
    #[Demo(value = r#"Some(String::from("0001234"))"#)]
    pub hn: Option<String>,
    #[Demo(value = r#"Some(String::from("11707"))"#)]
    pub refer_hospcode: Option<String>,
    #[Demo(value = "Some(date!(2023-12-31))")]
    pub refer_date: Option<Date>,
    #[Demo(value = "Some(time!(23:59:59))")]
    pub refer_time: Option<Time>,
    #[Demo(value = r#"Some(String::from("มค 0027.311/09/999"))"#)]
    pub docno: Option<String>,
    #[Demo(value = r#"Some(String::from("OLD DM"))"#)]
    pub pmh: Option<String>,
    #[Demo(value = r#"Some(String::from("1D PTA BLAH BLAH"))"#)]
    pub hpi: Option<String>,
    #[Demo(value = r#"Some(String::from("HCT 33%"))"#)]
    pub lab_text: Option<String>,
    #[Demo(value = r#"Some(String::from("IV CEFTAZIDIME"))"#)]
    pub treatment_text: Option<String>,
    #[Demo(value = r#"Some(String::from("Something bad"))"#)]
    pub other_text: Option<String>,
    #[Demo(value = r#"Some(String::from("Acute Abdomen"))"#)]
    pub diagnosis_text: Option<String>,
    #[Demo(value = r#"Some(String::from("For proper management"))"#)]
    pub request_text: Option<String>,

    #[Demo(value = r#"Some(String::from("PAIN"))"#)]
    pub cc: Option<String>,
    #[Demo(value = r#"Some(String::from("MASS AT HEAD"))"#)]
    pub pe: Option<String>,

    #[Demo(value = r#"Some(String::from("009"))"#)]
    pub doctor: Option<String>,
    #[Demo(value = "Some(datetime!(2023-12-31 23:59:59))")]
    pub update_datetime: Option<PrimitiveDateTime>,

    // JOINED
    #[Demo(value = r#"Some(String::from("รพ.มหาสารคาม"))"#)]
    pub refer_hospcode_name: Option<String>,
    // JOINED
    #[Demo(value = r#"Some(String::from("Dr Doctor"))"#)]
    pub doctor_name: Option<String>,
}

impl ReferNote {
    /// GET `EndPoint::ReferNoteVnan`
    pub async fn call_api_get(vnan: &str, app: Rc<AppState>) -> Result<Vec<Self>, AppError> {
        match fetch_json_api(&[&EndPoint::ReferNoteVnan.base(), vnan].concat(), "GET", None, app).await {
            Ok((response, true)) => {
                let response: Vec<Self> = serde_wasm_bindgen::from_value(response).map_err(|e| Source::SerdeWasm.to_teapot_error(e, "Fetch ReferNote"))?;
                Ok(response)
            }
            Ok((app_error, false)) => {
                let error: AppError = serde_wasm_bindgen::from_value(app_error).map_err(|e| Source::SerdeWasm.to_teapot_error(e, "Fetch ReferNote"))?;
                Err(error)
            }
            Err(e) => Err(Source::Js.to_teapot_error(e.dyn_ref::<JsString>().map(|s| s.into()).unwrap_or(String::from("fetch error")), "Fetch Json")),
        }
    }
}

/// KPHIS refernote + refer_vital_sign to be save
#[derive(Demo, Deserialize, Serialize, ToSchema)]
#[schema(example = json!(ReferNoteSave::demo()))]
pub struct ReferNoteSave {
    #[Demo(value = "Some(1)")]
    pub refernote_id: Option<u32>,
    #[Demo(value = r#"String::from("660001234")"#)]
    pub vn: String,
    #[Demo(value = r#"Some(String::from("0001234"))"#)]
    pub hn: Option<String>,
    #[Demo(value = r#"Some(String::from("11707"))"#)]
    pub refer_hospcode: Option<String>,
    #[Demo(value = "Some(date!(2024-03-31))")]
    pub refer_date: Option<Date>,
    #[Demo(value = "Some(time!(23:59:59))")]
    pub refer_time: Option<Time>,
    #[Demo(value = r#"Some(String::from("มค 0027.311/09/999"))"#)]
    pub docno: Option<String>,
    #[Demo(value = r#"Some(String::from("Pneumonia"))"#)]
    pub pre_diagnosis: Option<String>,
    #[Demo(value = r#"Some(String::from("OLD DM"))"#)]
    pub pmh: Option<String>,
    #[Demo(value = r#"Some(String::from("1D PTA BLAH BLAH"))"#)]
    pub hpi: Option<String>,
    #[Demo(value = r#"Some(String::from("HCT 33%"))"#)]
    pub lab_text: Option<String>,
    #[Demo(value = r#"Some(String::from("IV CEFTAZIDIME"))"#)]
    pub treatment_text: Option<String>,
    #[Demo(value = r#"Some(String::from("Something bad"))"#)]
    pub other_text: Option<String>,
    #[Demo(value = r#"Some(String::from("Acute Abdomen"))"#)]
    pub diagnosis_text: Option<String>,
    #[Demo(value = r#"Some(String::from("For proper management"))"#)]
    pub request_text: Option<String>,

    #[Demo(value = r#"Some(String::from("PAIN"))"#)]
    pub cc: Option<String>,
    #[Demo(value = r#"Some(String::from("MASS AT HEAD"))"#)]
    pub pe: Option<String>,
}

impl ReferNoteSave {
    /// - POST `EndPoint::ReferNoteVnan`
    pub async fn call_api_post(&self, vnan: &str, app: Rc<AppState>) -> Result<ExecuteResponse, AppError> {
        let body_json = serde_json::to_string(self).map_err(|e| Source::SerdeJson.to_teapot_error(e, "Send ReferNoteSave"))?;
        let body = serde_wasm_bindgen::to_value(&body_json).map_err(|e| Source::SerdeWasm.to_teapot_error(e, "Send HisReferNoteSave"))?;

        execute_fetch(&[&EndPoint::ReferNoteVnan.base(), vnan].concat(), "POST", Some(&body), app).await
    }
}
