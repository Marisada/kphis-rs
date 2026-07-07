use derive_demo::Demo;
use js_sys::JsString;
use serde_derive::{Deserialize, Serialize};
use sqlx::FromRow;
use std::rc::Rc;
use utoipa::{IntoParams, ToSchema};
use wasm_bindgen::JsCast;

use kphis_util::error::{AppError, Source};

use crate::{
    app::AppState,
    endpoint::{EndPoint, QueryString, find_qs},
    fetch::{ExecuteResponse, execute_fetch_vec, execute_fetch_vec_with_u32, fetch_json_api},
};

#[derive(Clone, Default, Debug, Deserialize, Serialize, IntoParams)]
pub struct DoctorInChargeParams {
    pub an: Option<String>,
    pub doctor_in_charge_id: Option<u32>,
    pub version: Option<i32>,
}

impl QueryString for DoctorInChargeParams {
    fn from_tuples(params: &[(String, String)]) -> Option<Self> {
        (!params.is_empty()).then(|| Self {
            an: find_qs(params, "an"),
            doctor_in_charge_id: find_qs(params, "doctor_in_charge_id").and_then(|s| s.parse::<u32>().ok()),
            version: find_qs(params, "version").and_then(|s| s.parse::<i32>().ok()),
        })
    }

    fn query_string(&self) -> String {
        let mut queries = Vec::with_capacity(3);
        if let Some(an) = &self.an {
            queries.push(["an=", an].concat());
        }
        if let Some(doctor_in_charge_id) = &self.doctor_in_charge_id {
            queries.push(["doctor_in_charge_id=", &doctor_in_charge_id.to_string()].concat());
        }
        if let Some(version) = &self.version {
            queries.push(["version=", &version.to_string()].concat());
        }

        (!queries.is_empty()).then(|| ["?", &queries.join("&")].concat()).unwrap_or_default()
    }
}

/// IPD Doctor who care this patient data
#[derive(Demo, Deserialize, Serialize, FromRow, ToSchema)]
#[schema(example = json!(IpdDoctorInCharge::demo()))]
pub struct IpdDoctorInCharge {
    #[Demo(value = "1")]
    pub doctor_in_charge_id: u32,
    #[Demo(value = r#"Some(String::from("660001234"))"#)]
    pub an: Option<String>,
    #[Demo(value = r#"Some(String::from("0001234"))"#)]
    pub hn: Option<String>,
    #[Demo(value = r#"Some(String::from("007"))"#)]
    pub doctor: Option<String>,
    #[Demo(value = r#"Some(String::from("1"))"#)]
    pub spclty: Option<String>,
    #[Demo(value = r#"Some(String::from("Y"))"#)]
    pub status: Option<String>,
    #[Demo(value = r#"Some(String::from("on"))"#)]
    pub activated: Option<String>,
    #[Demo(value = "1")]
    pub version: i32,
    #[Demo(value = r#"Some(String::from("Dr.Doctor"))"#)]
    pub doctor_name: Option<String>,
    #[Demo(value = r#"Some(String::from("อายุรกรรมชาย"))"#)]
    pub spclty_name: Option<String>,
}

impl IpdDoctorInCharge {
    /// GET `EndPoint::IpdDoctorInCharge`
    pub async fn call_api_get(params: &DoctorInChargeParams, app: Rc<AppState>) -> Result<Vec<Self>, AppError> {
        match fetch_json_api(&[EndPoint::IpdDoctorInCharge.base(), params.query_string()].concat(), "GET", None, app).await {
            Ok((response, true)) => {
                let response: Vec<Self> = serde_wasm_bindgen::from_value(response).map_err(|e| Source::SerdeWasm.to_teapot_error(e, "Fetch IpdDoctorInCharge"))?;
                Ok(response)
            }
            Ok((app_error, false)) => {
                let error: AppError = serde_wasm_bindgen::from_value(app_error).map_err(|e| Source::SerdeWasm.to_teapot_error(e, "Fetch IpdDoctorInCharge"))?;
                Err(error)
            }
            Err(e) => Err(Source::Js.to_teapot_error(e.dyn_ref::<JsString>().map(|s| s.into()).unwrap_or(String::from("fetch error")), "Fetch Json")),
        }
    }

    /// POST `EndPoint::IpdDoctorInCharge`
    pub async fn call_api_post(&self, app: Rc<AppState>) -> Result<(u32, Vec<ExecuteResponse>), AppError> {
        let body_json = serde_json::to_string(self).map_err(|e| Source::SerdeJson.to_teapot_error(e, "Send IpdDoctorInCharge"))?;

        let body = serde_wasm_bindgen::to_value(&body_json).map_err(|e| Source::SerdeWasm.to_teapot_error(e, "Send IpdDoctorInCharge"))?;

        execute_fetch_vec_with_u32(&EndPoint::IpdDoctorInCharge.base(), "POST", Some(&body), app).await
    }

    /// DELETE `EndPoint::IpdDoctorInCharge`
    pub async fn call_api_delete(params: &DoctorInChargeParams, app: Rc<AppState>) -> Result<Vec<ExecuteResponse>, AppError> {
        execute_fetch_vec(&[EndPoint::IpdDoctorInCharge.base(), params.query_string()].concat(), "DELETE", None, app).await
    }
}
