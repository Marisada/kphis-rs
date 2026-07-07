use bitcode::{Decode, Encode};
use derive_demo::Demo;
use serde_derive::Serialize;
use sqlx::FromRow;
use utoipa::ToSchema;

/// HTML select element option with color
#[derive(Clone, Debug, Demo, Decode, Encode, Hash, PartialEq, Serialize, ToSchema)]
#[schema(example = json!(ColorSelectOption::demo(String::from("1"), String::from("Item1"))))]
pub struct ColorSelectOption {
    pub key: String,
    pub value: String,
    #[Demo(value = r##"String::from("#888888")"##)]
    pub color: String,
}

/// HTML select element option without color
#[derive(Clone, Debug, Demo, Decode, Encode, FromRow, Hash, PartialEq, Serialize, ToSchema)]
#[schema(example = json!(SelectOption::demo(String::from("1"), String::from("Item1"))))]
pub struct SelectOption {
    pub key: String,
    pub value: String,
}

// use std::sync::Rc;

// use crate::{
//     fetch::fetch_json_api,
//     app::App,
//     error::{AppError, Source},
// };

// /// `GET` api, return multiple SelectOption
// pub async fn fetch_select_options(path: &str, app: Rc<App>) -> Result<Vec<SelectOption>, AppError> {
//     match fetch_json_api(path, "GET", None, app).await {
//         Ok((response, true)) => {
//             let response: Vec<SelectOption> = serde_wasm_bindgen::from_value(response).map_err(|e| Source::SerdeWasm.to_error(500, e))?;
//             Ok(response)
//         }
//         Ok((app_error, false)) => {
//             let error: AppError = serde_wasm_bindgen::from_value(app_error).map_err(|e| Source::SerdeWasm.to_error(500, e))?;
//             Err(error)
//         }
//         Err(e) => {
//             Err(Source::Js.to_error(500, e.dyn_ref::<JsString>().map(|s| s.into()).unwrap_or(String::from("fetch error"))))
//         }
//     }
// }
