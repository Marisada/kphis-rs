use derive_demo::Demo;
use js_sys::JsString;
use serde::{Deserialize, Serialize};
use sqlx::{
    FromRow,
    types::time::{Date, Time},
};
use std::rc::Rc;
use time::macros::{date, time};
use utoipa::ToSchema;
use wasm_bindgen::JsCast;

use kphis_util::error::{AppError, Source};

use crate::{app::AppState, endpoint::EndPoint, fetch::fetch_json_api};

/// OPD Medical Item from HIS
#[derive(Clone, Demo, Deserialize, Serialize, FromRow, ToSchema)]
#[schema(example = json!(OpdMed::demo()))]
pub struct OpdMed {
    #[Demo(default)]
    pub hos_guid: String,
    #[Demo(value = r#"Some(String::from("1000222"))"#)]
    pub icode: Option<String>,
    #[Demo(value = "Some(1)")]
    pub item_no: Option<i8>,
    #[Demo(value = r#"Some(String::from("H"))"#)]
    pub item_type: Option<String>,
    #[Demo(value = r#"Some(String::from("PARACETAMOL 500 mg. เม็ด"))"#)]
    pub item_name: Option<String>,
    #[Demo(value = r#"Some(String::from("รับประทานครั้งละ 1 เม็ด ทุก 4-6 ชม. เวลามีอาการ"))"#)]
    pub usage: Option<String>,
    #[Demo(value = "Some(10)")]
    pub qty: Option<i32>,
    #[Demo(value = r#"Some(String::from("02"))"#)]
    pub paidst: Option<String>,
    #[Demo(value = r#"Some(String::from("1 prt"))"#)]
    pub shortlist: Option<String>,
    #[Demo(value = "Some(0)")]
    pub displaycolor: Option<i32>,
    #[Demo(value = r#"Some(String::from("รับประทานครั้งละ 1 เม็ด"))"#)]
    pub name1: Option<String>,
    #[Demo(value = r#"Some(String::from("ทุก 4-6 ชม."))"#)]
    pub name2: Option<String>,
    #[Demo(value = r#"Some(String::from("เวลามีอาการ"))"#)]
    pub name3: Option<String>,
    #[Demo(value = r#"Some(String::from("1111111"))"#)]
    pub sp_use: Option<String>,
    #[Demo(value = "Some(date!(2023-12-31))")]
    pub rxdate: Option<Date>,
    #[Demo(value = "Some(time!(23:59:59))")]
    pub rxtime: Option<Time>,
    #[Demo(value = r#"Some(String::from("661231235959"))"#)]
    pub vn: Option<String>,
    #[Demo(value = r#"Some(String::from("660001234"))"#)]
    pub an: Option<String>,
}

impl OpdMed {
    /// GET `EndPoint::OpdErHisMedVn`
    pub async fn call_api_get(vn: &str, app: Rc<AppState>) -> Result<Vec<Self>, AppError> {
        match fetch_json_api(&[&EndPoint::OpdErHisMedVn.base(), vn].concat(), "GET", None, app).await {
            Ok((response, true)) => {
                let response: Vec<Self> = serde_wasm_bindgen::from_value(response).map_err(|e| Source::SerdeWasm.to_teapot_error(e, "Fetch OpdMed"))?;
                Ok(response)
            }
            Ok((app_error, false)) => {
                let error: AppError = serde_wasm_bindgen::from_value(app_error).map_err(|e| Source::SerdeWasm.to_teapot_error(e, "Fetch OpdMed"))?;
                Err(error)
            }
            Err(e) => Err(Source::Js.to_teapot_error(e.dyn_ref::<JsString>().map(|s| s.into()).unwrap_or(String::from("fetch error")), "Fetch Json")),
        }
    }
}
