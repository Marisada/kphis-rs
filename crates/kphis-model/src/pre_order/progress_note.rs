use derive_demo::Demo;
use js_sys::JsString;
use serde::{Deserialize, Serialize};
use sqlx::{
    FromRow,
    types::time::{Date, PrimitiveDateTime, Time},
};
use std::rc::Rc;
use time::macros::{date, datetime, time};
use utoipa::{IntoParams, ToSchema};
use wasm_bindgen::JsCast;

use kphis_util::{
    datetime::date_8601,
    error::{AppError, Source},
};

use crate::{
    app::AppState,
    endpoint::{EndPoint, QueryString, find_qs},
    fetch::{ExecuteResponse, execute_fetch, execute_fetch_vec_with_u32, fetch_json_api},
    progress_note::{ProgressNoteItemSave, ProgressNoteTypeName},
};

#[derive(Clone, Default, Deserialize, IntoParams)]
pub struct PreProgressNoteParams {
    pub pre_order_master_id: Option<u32>,
    pub progress_note_id: Option<u32>,
    pub progress_note_date: Option<Date>,
}

impl QueryString for PreProgressNoteParams {
    fn from_tuples(params: &[(String, String)]) -> Option<Self> {
        (!params.is_empty()).then(|| Self {
            pre_order_master_id: find_qs(params, "pre_order_master_id").and_then(|s| s.parse::<u32>().ok()),
            progress_note_id: find_qs(params, "progress_note_id").and_then(|s| s.parse::<u32>().ok()),
            progress_note_date: find_qs(params, "progress_note_date").and_then(|s| date_8601(&s)),
        })
    }

    fn query_string(&self) -> String {
        let mut queries = Vec::with_capacity(3);
        if let Some(pre_order_master_id) = &self.pre_order_master_id {
            queries.push(["pre_order_master_id=", &pre_order_master_id.to_string()].concat());
        }
        if let Some(progress_note_id) = &self.progress_note_id {
            queries.push(["progress_note_id=", &progress_note_id.to_string()].concat());
        }
        if let Some(progress_note_date) = &self.progress_note_date {
            queries.push(["progress_note_date=", &progress_note_date.to_string()].concat());
        }
        (!queries.is_empty()).then(|| ["?", &queries.join("&")].concat()).unwrap_or_default()
    }
}

/// Pre-Progress Note with Items
#[derive(Clone, Demo, Deserialize, Serialize, FromRow, ToSchema)]
#[schema(example = json!(PreProgressNote::demo()))]
pub struct PreProgressNote {
    #[Demo(value = "1")]
    pub progress_note_id: u32,
    #[Demo(value = "1")]
    pub pre_order_master_id: u32,
    #[Demo(value = "date!(2023-12-31)")]
    pub progress_note_date: Date,
    #[Demo(value = "time!(23:59:59)")]
    pub progress_note_time: Time,
    #[Demo(value = r#"String::from("doctor")"#)]
    pub progress_note_owner_type: String,
    #[Demo(value = r#"String::from("007")"#)]
    pub progress_note_doctor: String,

    #[Demo(value = r#"Some(String::from("Dr.Doctor"))"#)]
    pub order_doctor_name: Option<String>,
    #[Demo(value = "Some(true)")]
    pub order_doctor_is_intern: Option<bool>,
    #[Demo(value = r#"Some(String::from("ว00000"))"#)]
    pub doctor_licenseno: Option<String>,
    #[Demo(value = r#"Some(String::from("นายแพทย์"))"#)]
    pub entryposition: Option<String>,

    #[Demo(value = "datetime!(2023-12-31 23:59:59)")]
    pub create_datetime: PrimitiveDateTime,

    #[sqlx(skip)]
    #[Demo(value = "vec![PreProgressNoteItemType::demo()]")]
    pub progress_note_item_types: Vec<PreProgressNoteItemType>,
}

impl PreProgressNote {
    /// GET `EndPoint::IpdPreOrderProgressNote`
    pub async fn call_api_get(params: &PreProgressNoteParams, app: Rc<AppState>) -> Result<Vec<Self>, AppError> {
        match fetch_json_api(&[EndPoint::IpdPreOrderProgressNote.base(), params.clone().query_string()].concat(), "GET", None, app).await {
            Ok((response, true)) => {
                let response: Vec<Self> = serde_wasm_bindgen::from_value(response).map_err(|e| Source::SerdeWasm.to_teapot_error(e, "Fetch PreProgressNote"))?;
                Ok(response)
            }
            Ok((app_error, false)) => {
                let error: AppError = serde_wasm_bindgen::from_value(app_error).map_err(|e| Source::SerdeWasm.to_teapot_error(e, "Fetch PreProgressNote"))?;
                Err(error)
            }
            Err(e) => Err(Source::Js.to_teapot_error(e.dyn_ref::<JsString>().map(|s| s.into()).unwrap_or(String::from("fetch error")), "Fetch Json")),
        }
    }

    /// DELETE `EndPoint::IpdPreOrderProgressNoteId`
    pub async fn call_api_delete(pre_progress_note_id: u32, app: Rc<AppState>) -> Result<ExecuteResponse, AppError> {
        execute_fetch(&[EndPoint::IpdPreOrderProgressNoteId.base(), pre_progress_note_id.to_string()].concat(), "DELETE", None, app).await
    }
}

/// Type of Pre-Progress Note Item
#[derive(Clone, Demo, Deserialize, Serialize, ToSchema)]
#[schema(example = json!(PreProgressNoteItemType::demo()))]
pub struct PreProgressNoteItemType {
    #[Demo(value = "ProgressNoteTypeName::demo_note()")]
    pub progress_note_item_type: ProgressNoteTypeName,
    #[Demo(value = "vec![PreProgressNoteItem::demo()]")]
    pub progress_note_items: Vec<PreProgressNoteItem>,
}

/// Item of Pre-Progress Note
#[derive(Clone, Demo, Deserialize, Serialize, FromRow, ToSchema)]
#[schema(example = json!(PreProgressNoteItem::demo()))]
pub struct PreProgressNoteItem {
    #[Demo(value = "1")]
    pub progress_note_item_id: u32,
    #[Demo(value = "Some(1)")]
    pub progress_note_id: Option<u32>,
    #[Demo(value = "Some(1)")]
    pub pre_order_master_id: Option<u32>,
    #[Demo(value = r#"Some(String::from("note"))"#)]
    pub progress_note_item_type: Option<String>,
    #[Demo(value = r#"Some(String::from("Detail1"))"#)]
    pub progress_note_item_detail: Option<String>,
    #[Demo(value = r#"Some(String::from("Detail2"))"#)]
    pub progress_note_item_detail_2: Option<String>,
}

/// Pre-Pregress Note for save
#[derive(Demo, Deserialize, Serialize, FromRow, ToSchema)]
#[schema(example = json!(PreProgressNoteSave::demo()))]
pub struct PreProgressNoteSave {
    #[Demo(value = "1")]
    pub progress_note_id: u32,
    #[Demo(value = "1")]
    pub pre_order_master_id: u32,
    #[Demo(value = "date!(2023-12-31)")]
    pub progress_note_date: Date,
    #[Demo(value = "time!(23:59:59)")]
    pub progress_note_time: Time,
    #[Demo(value = r#"String::from("doctor")"#)]
    pub progress_note_owner_type: String,
    #[Demo(value = r#"String::from("007")"#)]
    pub progress_note_doctor: String,
    #[sqlx(skip)]
    #[Demo(value = "vec![ProgressNoteItemSave::demo()]")]
    pub progress_note_items: Vec<ProgressNoteItemSave>,
}

impl PreProgressNoteSave {
    /// POST `EndPoint::IpdPreOrderProgressNote`
    pub async fn call_api_post(&self, app: Rc<AppState>) -> Result<(u32, Vec<ExecuteResponse>), AppError> {
        let body_json = serde_json::to_string(self).map_err(|e| Source::SerdeJson.to_teapot_error(e, "Send PreProgressNoteSave"))?;

        let body = serde_wasm_bindgen::to_value(&body_json).map_err(|e| Source::SerdeWasm.to_teapot_error(e, "Send PreProgressNoteSave"))?;

        execute_fetch_vec_with_u32(&EndPoint::IpdPreOrderProgressNote.base(), "POST", Some(&body), app).await
    }
}
