use derive_demo::Demo;
use js_sys::JsString;
use serde_derive::{Deserialize, Serialize};
use sqlx::FromRow;
use std::rc::Rc;
use time::{
    Date, Time,
    macros::{date, time},
};
use utoipa::ToSchema;
use wasm_bindgen::JsCast;

use kphis_util::error::{AppError, Source};

use crate::{app::AppState, endpoint::EndPoint, fetch::fetch_json_api, image::scan_his::ScanHisExists, prescription::NextAppointment};

/// Key parts of EMR Visit
#[derive(Clone, Debug, Demo, Deserialize, FromRow, Serialize, ToSchema)]
#[schema(example = json!(EmrDate::demo()))]
pub struct EmrDate {
    #[Demo(value = r#"String::from("661231235959")"#)]
    pub vn: String,
    #[Demo(value = "Some(date!(2023-12-31))")]
    pub vstdate: Option<Date>,
    #[Demo(value = "Some(time!(23:59:59))")]
    pub vsttime: Option<Time>,
    #[Demo(value = r#"Some(String::from("660001234"))"#)]
    pub an: Option<String>,
    #[Demo(value = "Some(1)")]
    pub opd_er_order_master_id: Option<u32>,
}

impl EmrDate {
    /// GET `EndPoint::EmrDateHn`
    pub async fn call_api_get(hn: &str, app: Rc<AppState>) -> Result<Vec<Self>, AppError> {
        match fetch_json_api(&[&EndPoint::EmrDateHn.base(), hn].concat(), "GET", None, app).await {
            Ok((response, true)) => {
                let response: Vec<Self> = serde_wasm_bindgen::from_value(response).map_err(|e| Source::SerdeWasm.to_teapot_error(e, "Fetch EmrDate"))?;
                Ok(response)
            }
            Ok((app_error, false)) => {
                let error: AppError = serde_wasm_bindgen::from_value(app_error).map_err(|e| Source::SerdeWasm.to_teapot_error(e, "Fetch EmrDate"))?;
                Err(error)
            }
            Err(e) => Err(Source::Js.to_teapot_error(e.dyn_ref::<JsString>().map(|s| s.into()).unwrap_or(String::from("fetch error")), "Fetch Json")),
        }
    }
}

/// EMR Visit from HIS
#[derive(Clone, Debug, Demo, Deserialize, FromRow, Serialize, ToSchema)]
#[schema(example = json!(EmrVisit::demo()))]
pub struct EmrVisit {
    #[Demo(value = r#"String::from("661231235959")"#)]
    pub vn: String,
    #[Demo(value = "Some(date!(2023-12-31))")]
    pub vstdate: Option<Date>,
    #[Demo(value = "Some(time!(23:59:59))")]
    pub vsttime: Option<Time>,
    #[Demo(value = r#"Some(String::from("66001234"))"#)]
    pub an: Option<String>,
    #[Demo(value = r#"Some(String::from("0001234"))"#)]
    pub hn: Option<String>,
    #[Demo(value = r#"Some(String::from("Mr.Patient"))"#)]
    pub ptname: Option<String>,
    #[Demo(value = r#"Some(String::from("33 ปี 3 เดือน 3 วัน"))"#)]
    pub age_th: Option<String>,
    #[Demo(value = r#"Some(String::from("88"))"#)]
    pub pttype: Option<String>,
    #[Demo(value = r#"Some(String::from("บุคคลในครอบครัว อสม."))"#)]
    pub pttype_name: Option<String>,
    #[Demo(value = "Some(120.0)")]
    pub bps: Option<f64>,
    #[Demo(value = "Some(80.0)")]
    pub bpd: Option<f64>,
    #[Demo(value = "Some(170)")]
    pub height: Option<i32>,
    #[Demo(value = "Some(50.0)")]
    pub bw: Option<f64>,
    #[Demo(value = "Some(80.0)")]
    pub pulse: Option<f64>,
    #[Demo(value = "Some(37.5)")]
    pub temperature: Option<f64>,
    #[Demo(value = r#"Some(String::from("Fever"))"#)]
    pub cc: Option<String>,
    #[Demo(value = r#"Some(String::from("Present Hx"))"#)]
    pub hpi: Option<String>,
    #[Demo(value = r#"Some(String::from("Past Hx"))"#)]
    pub pmh: Option<String>,
    #[Demo(value = r#"Some(String::from("Family Hx"))"#)]
    pub fh: Option<String>,
    #[Demo(value = r#"Some(String::from("Social Hx"))"#)]
    pub sh: Option<String>,
    #[Demo(value = "Some(80.0)")]
    pub hr: Option<f64>,
    #[Demo(value = r#"Some(String::from("Look sick"))"#)]
    pub pe: Option<String>,
    #[Demo(value = "Some(20.0)")]
    pub rr: Option<f64>,
    #[Demo(value = "Some(17.3)")]
    pub bmi: Option<f64>,
    #[Demo(value = r#"Some(String::from("มาเอง"))"#)]
    pub ovstist_name: Option<String>,
    #[Demo(value = r#"Some(String::from("Dr.Doctor"))"#)]
    pub doctor_name: Option<String>,
    #[Demo(value = "true")]
    pub has_data_refer_out: bool,
    #[sqlx(skip)]
    #[Demo(value = r#"vec![String::from("I10 : Essential Hypertension (PDX)")]"#)]
    pub diagnoses: Vec<String>,
    #[sqlx(skip)]
    #[Demo(value = r#"vec![String::from("PARACETAMOL 500 mg. 1prtq6(1 เม็ด q 6 ชม) X 10")]"#)]
    pub drugs: Vec<String>,
    #[sqlx(skip)]
    #[Demo(value = r#"vec![String::from("PARACETAMOL 500 mg. 1prtq6(1 เม็ด q 6 ชม) X 10")]"#)]
    pub home_drugs: Vec<String>,
    #[sqlx(skip)]
    #[Demo(value = r#"vec![String::from("ค่าบริการผู้ป่วยนอกในเวลาราชการ(55020) X 1")]"#)]
    pub nondrugs: Vec<String>,
    #[sqlx(skip)]
    #[Demo(value = r#"vec![String::from("Eye Pad X 1")]"#)]
    pub home_nondrugs: Vec<String>,
    #[sqlx(skip)]
    #[Demo(value = "vec![NextAppointment::demo()]")]
    pub next_app: Vec<NextAppointment>,
    /// if Admited, discard OPD
    #[sqlx(skip)]
    #[Demo(value = "ScanHisExists::demo()")]
    pub image_exists: ScanHisExists,
}

impl EmrVisit {
    /// GET `EndPoint::EmrVisitVn`
    pub async fn call_api_get(vn: &str, app: Rc<AppState>) -> Result<Option<Self>, AppError> {
        match fetch_json_api(&[&EndPoint::EmrVisitVn.base(), vn].concat(), "GET", None, app).await {
            Ok((response, true)) => {
                let response: Option<Self> = serde_wasm_bindgen::from_value(response).map_err(|e| Source::SerdeWasm.to_teapot_error(e, "Fetch EmrVisit"))?;
                Ok(response)
            }
            Ok((app_error, false)) => {
                let error: AppError = serde_wasm_bindgen::from_value(app_error).map_err(|e| Source::SerdeWasm.to_teapot_error(e, "Fetch EmrVisit"))?;
                Err(error)
            }
            Err(e) => Err(Source::Js.to_teapot_error(e.dyn_ref::<JsString>().map(|s| s.into()).unwrap_or(String::from("fetch error")), "Fetch Json")),
        }
    }
}
