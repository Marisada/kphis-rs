use derive_demo::Demo;
use js_sys::JsString;
use serde_derive::{Deserialize, Serialize};
use std::rc::Rc;
use time::{PrimitiveDateTime, macros::datetime};
use utoipa::{IntoParams, ToSchema};
use wasm_bindgen::JsCast;

use kphis_util::error::{AppError, Source};

use crate::{
    app::AppState,
    endpoint::{EndPoint, QueryString, find_qs},
    fetch::{ExecuteResponse, execute_fetch, fetch_json_api},
};

pub struct PacsConfig {
    pub pacs_host: String,
    pub pacs_host_is_kphis_broker: bool,
    pub pacs_user: String,
    pub pacs_password: String,
    pub pacs_data_source: String,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize, IntoParams)]
pub struct PacsParams {
    pub file_path: Option<String>,
    pub study_uid: Option<String>,
    pub series_uid: Option<String>,
    pub object_uid: Option<String>,
}

impl QueryString for PacsParams {
    fn from_tuples(params: &[(String, String)]) -> Option<Self> {
        (!params.is_empty()).then(|| Self {
            file_path: find_qs(params, "file_path"),
            study_uid: find_qs(params, "study_uid"),
            series_uid: find_qs(params, "series_uid"),
            object_uid: find_qs(params, "object_uid"),
        })
    }

    fn query_string(&self) -> String {
        let mut queries = Vec::with_capacity(4);
        if let Some(file_path) = &self.file_path {
            queries.push(["file_path=", file_path].concat());
        }
        if let Some(study_uid) = &self.study_uid {
            queries.push(["study_uid=", study_uid].concat());
        }
        if let Some(series_uid) = &self.series_uid {
            queries.push(["series_uid=", series_uid].concat());
        }
        if let Some(object_uid) = &self.object_uid {
            queries.push(["object_uid=", object_uid].concat());
        }

        (!queries.is_empty()).then(|| ["?", &queries.join("&")].concat()).unwrap_or_default()
    }
}

#[derive(Clone, Demo, Debug, Deserialize, Serialize, ToSchema)]
#[schema(example = json!(PacsXnData::demo()))]
pub struct PacsXnData {
    #[Demo(value = r#"String::from("Patient")"#)]
    pub fname: String,
    #[Demo(value = r#"String::from("Sicker")"#)]
    pub lname: String,
    #[Demo(value = r#"String::from("Still")"#)]
    pub mname: String,
    #[Demo(value = r#"String::from("Mr")"#)]
    pub sname: String,
    #[Demo(value = "Some(datetime!(2023-12-31 23:59:59))")]
    pub birth: Option<PrimitiveDateTime>,
    #[Demo(value = r#"String::from("1")"#)]
    pub ext_id: String,
    // M. F
    #[Demo(value = r#"String::from("M")"#)]
    pub gender: String,
    #[Demo(value = r#"Some(String::from("1-2345-67-8910"))"#)]
    pub study_uid: Option<String>,
    #[Demo(value = r#"vec![PacsImageData::demo()]"#)]
    pub images: Vec<PacsImageData>,
}

impl PacsXnData {
    /// GET `EndPoint::XrayPacsXn`
    pub async fn call_api_get(xn: i32, app: Rc<AppState>) -> Result<Self, AppError> {
        match fetch_json_api(&[EndPoint::XrayPacsXn.base(), xn.to_string()].concat(), "GET", None, app).await {
            Ok((response, true)) => {
                let response: Self = serde_wasm_bindgen::from_value(response).map_err(|e| Source::SerdeWasm.to_teapot_error(e, "Fetch XrayPacsXn"))?;
                Ok(response)
            }
            Ok((app_error, false)) => {
                let error: AppError = serde_wasm_bindgen::from_value(app_error).map_err(|e| Source::SerdeWasm.to_teapot_error(e, "Fetch XrayPacsXn"))?;
                Err(error)
            }
            Err(e) => Err(Source::Js.to_teapot_error(e.dyn_ref::<JsString>().map(|s| s.into()).unwrap_or(String::from("fetch error")), "Fetch Json")),
        }
    }

    /// POST `EndPoint::XrayReadId`
    pub async fn call_api_post_readed(xn: i32, app: Rc<AppState>) -> Result<ExecuteResponse, AppError> {
        execute_fetch(&[EndPoint::XrayReadId.base(), xn.to_string()].concat(), "POST", None, app).await
    }

    /// DELETE `EndPoint::XrayReadId`
    pub async fn call_api_delete_readed(xn: i32, app: Rc<AppState>) -> Result<ExecuteResponse, AppError> {
        execute_fetch(&[EndPoint::XrayReadId.base(), xn.to_string()].concat(), "DELETE", None, app).await
    }
}

#[derive(Clone, Demo, Debug, Deserialize, Serialize, ToSchema)]
#[schema(example = json!(PacsImageData::demo()))]
pub struct PacsImageData {
    #[Demo(value = r#"String::from("1-2345-67-8910")"#)]
    pub study_uid: String,
    #[Demo(value = r#"String::from("1-2345-67-8910")"#)]
    pub series_uid: String,
    #[Demo(value = r#"String::from("1-2345-67-8910")"#)]
    pub object_uid: String,
    #[Demo(value = "Some(datetime!(2023-12-31 23:59:59))")]
    pub series_datetime: Option<PrimitiveDateTime>,
    #[Demo(value = r#"Some(String::from("R Arm AP"))"#)]
    pub label: Option<String>,
    #[Demo(value = r#"String::from("/image.jpg")"#)]
    pub file_path: String,
}
