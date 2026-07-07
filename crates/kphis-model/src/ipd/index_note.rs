use derive_demo::Demo;
use js_sys::JsString;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use std::rc::Rc;
use utoipa::{IntoParams, ToSchema};
use wasm_bindgen::JsCast;

use kphis_util::error::{AppError, Source};

use crate::{
    app::AppState,
    endpoint::{EndPoint, QueryString, find_qs},
    fetch::{ExecuteResponse, execute_fetch, execute_fetch_with_u32, fetch_json_api},
};

#[derive(Clone, Default, Deserialize, IntoParams)]
pub struct IndexNoteParams {
    pub nurse_index_note_id: Option<u32>,
    pub an: Option<String>,
}

impl QueryString for IndexNoteParams {
    fn from_tuples(params: &[(String, String)]) -> Option<Self> {
        (!params.is_empty()).then(|| Self {
            nurse_index_note_id: find_qs(params, "nurse_index_note_id").and_then(|s| s.parse::<u32>().ok()),
            an: find_qs(params, "an"),
        })
    }

    fn query_string(&self) -> String {
        let mut queries = Vec::with_capacity(2);
        if let Some(nurse_index_note_id) = &self.nurse_index_note_id {
            queries.push(["nurse_index_note_id=", &nurse_index_note_id.to_string()].concat());
        }
        if let Some(an) = &self.an {
            queries.push(["an=", an].concat());
        }

        (!queries.is_empty()).then(|| ["?", &queries.join("&")].concat()).unwrap_or_default()
    }
}

/// Nursing Index Note
#[derive(Clone, Default, Demo, Deserialize, Serialize, FromRow, ToSchema)]
#[schema(example = json!(IndexNote::demo()))]
pub struct IndexNote {
    #[Demo(value = "1")]
    pub nurse_index_note_id: u32,
    #[Demo(value = r#"Some(String::from("660001234"))"#)]
    pub an: Option<String>,
    #[Demo(value = r#"Some(String::from("Note"))"#)]
    pub nurse_index_note: Option<String>,
}

impl IndexNote {
    /// GET `EndPoint::IpdIndexNote`
    pub async fn call_api_get(params: &IndexNoteParams, app: Rc<AppState>) -> Result<Vec<Self>, AppError> {
        match fetch_json_api(&[EndPoint::IpdIndexNote.base(), params.clone().query_string()].concat(), "GET", None, app).await {
            Ok((response, true)) => {
                let response: Vec<Self> = serde_wasm_bindgen::from_value(response).map_err(|e| Source::SerdeWasm.to_teapot_error(e, "Fetch IndexNote"))?;
                Ok(response)
            }
            Ok((app_error, false)) => {
                let error: AppError = serde_wasm_bindgen::from_value(app_error).map_err(|e| Source::SerdeWasm.to_teapot_error(e, "Fetch IndexNote"))?;
                Err(error)
            }
            Err(e) => Err(Source::Js.to_teapot_error(e.dyn_ref::<JsString>().map(|s| s.into()).unwrap_or(String::from("fetch error")), "Fetch Json")),
        }
    }

    /// POST `EndPoint::IpdIndexNote`
    pub async fn call_api_post(&self, app: Rc<AppState>) -> Result<(u32, ExecuteResponse), AppError> {
        let body_json = serde_json::to_string(self).map_err(|e| Source::SerdeJson.to_teapot_error(e, "Send IndexNote"))?;

        let body = serde_wasm_bindgen::to_value(&body_json).map_err(|e| Source::SerdeWasm.to_teapot_error(e, "Send IndexNote"))?;

        execute_fetch_with_u32(&EndPoint::IpdIndexNote.base(), "POST", Some(&body), app).await
    }

    /// DELETE `EndPoint::IpdIndexNoteId`
    pub async fn call_api_delete(nurse_index_note_id: u32, app: Rc<AppState>) -> Result<ExecuteResponse, AppError> {
        execute_fetch(&[EndPoint::IpdIndexNoteId.base(), nurse_index_note_id.to_string()].concat(), "DELETE", None, app).await
    }
}
