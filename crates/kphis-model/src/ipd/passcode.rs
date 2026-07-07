use derive_demo::Demo;
use js_sys::JsString;
use serde_derive::{Deserialize, Serialize};
use std::rc::Rc;
use utoipa::ToSchema;
use wasm_bindgen::JsCast;

use kphis_util::error::{AppError, Source};

use crate::{app::AppState, endpoint::EndPoint, fetch::fetch_json_api};

/// Ward with Passcode requirement marking
#[derive(Clone, Debug, Demo, Deserialize, Serialize, ToSchema)]
#[schema(example = json!(ConfigIpdWardPasscode::demo()))]
pub struct ConfigIpdWardPasscode {
    #[Demo(value = r#"String::from("01")"#)]
    pub ward: String,
    #[Demo(value = r#"String::from("ตึกชาย")"#)]
    pub ward_name: String,
    #[Demo(value = "true")]
    pub using_passcode: bool,
}
impl ConfigIpdWardPasscode {
    /// GET `EndPoint::IpdPasscode`<br>
    /// get ward passcode list (both using/not using passcode)
    pub async fn call_api_get(app: Rc<AppState>) -> Result<Vec<Self>, AppError> {
        match fetch_json_api(&EndPoint::IpdPasscode.base(), "GET", None, app).await {
            Ok((response, true)) => {
                let response: Vec<Self> = serde_wasm_bindgen::from_value(response).map_err(|e| Source::SerdeWasm.to_teapot_error(e, "Fetch Json"))?;
                Ok(response)
            }
            Ok((app_error, false)) => {
                let error: AppError = serde_wasm_bindgen::from_value(app_error).map_err(|e| Source::SerdeWasm.to_teapot_error(e, "Fetch Json"))?;
                Err(error)
            }
            Err(e) => Err(Source::Js.to_teapot_error(e.dyn_ref::<JsString>().map(|s| s.into()).unwrap_or(String::from("fetch error")), "Fetch Json")),
        }
    }
}

/// Request for Passcode
#[derive(Debug, Demo, Deserialize, Serialize, ToSchema)]
#[schema(example = json!(PasscodeGenRequest::demo()))]
pub struct PasscodeGenRequest {
    #[Demo(value = r#"String::from("01")"#)]
    pub ward: String,
    #[Demo(value = r#"String::from("$argon2id$v=19$m=19456,t=2,p=1$/XFasj8WyfzzGzV2fnouWQ$OfHASwUrzgJmchn9LvM+T7IHtvI//W+BMgBe70jDnqU")"#)]
    pub password: String,
    #[Demo(value = "PasscodeGenRequestMode::demo_gen()")]
    pub mode: PasscodeGenRequestMode,
}

impl PasscodeGenRequest {
    /// POST `EndPoint::IpdPasscode`
    pub async fn call_api_post(&self, app: Rc<AppState>) -> Result<PasscodeGenResponse, AppError> {
        let body_json = serde_json::to_string(&self).map_err(|e| Source::SerdeJson.to_teapot_error(e, "Send PasscodeGenRequest"))?;
        let body = serde_wasm_bindgen::to_value(&body_json).map_err(|e| Source::SerdeWasm.to_teapot_error(e, "Send PasscodeGenRequest"))?;

        match fetch_json_api(&EndPoint::IpdPasscode.base(), "POST", Some(&body), app).await {
            Ok((response, true)) => {
                let response: PasscodeGenResponse = serde_wasm_bindgen::from_value(response).map_err(|e| Source::SerdeWasm.to_teapot_error(e, "Send PasscodeGenRequest"))?;
                Ok(response)
            }
            Ok((app_error, false)) => {
                let error: AppError = serde_wasm_bindgen::from_value(app_error).map_err(|e| Source::SerdeWasm.to_teapot_error(e, "Send PasscodeGenRequest"))?;
                Err(error)
            }
            Err(e) => Err(Source::Js.to_teapot_error(e.dyn_ref::<JsString>().map(|s| s.into()).unwrap_or(String::from("fetch error")), "Fetch Json")),
        }
    }
}

/// Generated Passcode
#[derive(Demo, Deserialize, Serialize, ToSchema)]
#[schema(example = json!(PasscodeGenResponse::demo()))]
pub struct PasscodeGenResponse {
    #[Demo(value = r#"Some(String::from("1234"))"#)]
    pub passcode: Option<String>,
}

impl PasscodeGenResponse {
    pub fn new(passcode: Option<String>) -> Self {
        Self { passcode }
    }
}

/// Mode of Passcode Request
#[derive(Debug, Demo, Deserialize, Serialize, ToSchema)]
#[schema(example = json!(PasscodeGenRequestMode::demo_gen()))]
pub enum PasscodeGenRequestMode {
    Gen,
    Remove,
}
