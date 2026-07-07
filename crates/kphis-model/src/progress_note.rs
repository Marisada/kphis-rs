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
    app::{AppState, VisitTypeId},
    endpoint::{EndPoint, QueryString, find_qs},
    fetch::{ExecuteResponse, execute_fetch, execute_fetch_vec_with_u32, fetch_json_api},
    pre_order::progress_note::{PreProgressNote, PreProgressNoteItem, PreProgressNoteItemType},
};

#[derive(Clone, Default, Deserialize, IntoParams)]
pub struct ProgressNoteParams {
    pub progress_note_id: Option<u32>,
    pub an: Option<String>,
    pub opd_er_order_master_id: Option<u32>,
    pub progress_note_date: Option<Date>,
    pub progress_note_owner_type: Option<String>,
}

impl QueryString for ProgressNoteParams {
    fn from_tuples(params: &[(String, String)]) -> Option<Self> {
        (!params.is_empty()).then(|| Self {
            progress_note_id: find_qs(params, "progress_note_id").and_then(|s| s.parse::<u32>().ok()),
            an: find_qs(params, "an"),
            opd_er_order_master_id: find_qs(params, "opd_er_order_master_id").and_then(|s| s.parse::<u32>().ok()),
            progress_note_date: find_qs(params, "progress_note_date").and_then(|s| date_8601(&s)),
            progress_note_owner_type: find_qs(params, "progress_note_owner_type"),
        })
    }

    fn query_string(&self) -> String {
        let mut queries = Vec::with_capacity(5);
        if let Some(progress_note_id) = &self.progress_note_id {
            queries.push(["progress_note_id=", &progress_note_id.to_string()].concat());
        }
        if let Some(an) = &self.an {
            queries.push(["an=", an].concat());
        }
        if let Some(opd_er_order_master_id) = &self.opd_er_order_master_id {
            queries.push(["opd_er_order_master_id=", &opd_er_order_master_id.to_string()].concat());
        }
        if let Some(progress_note_date) = &self.progress_note_date {
            queries.push(["progress_note_date=", &progress_note_date.to_string()].concat());
        }
        if let Some(progress_note_owner_type) = &self.progress_note_owner_type {
            queries.push(["progress_note_owner_type=", progress_note_owner_type].concat());
        }
        (!queries.is_empty()).then(|| ["?", &queries.join("&")].concat()).unwrap_or_default()
    }
}

/// Progress Note with Item
#[derive(Clone, Demo, Deserialize, Serialize, ToSchema)]
#[schema(example = json!(ProgressNote::demo()))]
pub struct ProgressNote {
    /// for generic over `an` or `opd_er_order_master_id` only
    #[Demo(value = r#"VisitTypeId::demo_ipd(String::from("660001234"))"#)]
    pub visit_type: VisitTypeId,
    #[Demo(value = "1")]
    pub progress_note_id: u32,
    #[Demo(value = "date!(2023-12-31)")]
    pub progress_note_date: Date,
    #[Demo(value = "time!(23:59:59)")]
    pub progress_note_time: Time,
    #[Demo(value = r#"String::from("nurse")"#)]
    pub progress_note_owner_type: String,
    #[Demo(value = r#"String::from("008")"#)]
    pub progress_note_doctor: String,
    #[Demo(value = "Some(datetime!(2023-12-31 23:59:59))")]
    pub progress_note_enter_datetime: Option<PrimitiveDateTime>,

    #[Demo(value = "Some(1)")]
    pub pre_order_progress_note_id: Option<u32>,
    #[Demo(value = "Some(date!(2023-12-31))")]
    pub pre_order_progress_note_date: Option<Date>,
    #[Demo(value = "Some(time!(23:59:59))")]
    pub pre_order_progress_note_time: Option<Time>,

    #[Demo(value = r#"Some(String::from("Dr.Doctor"))"#)]
    pub order_doctor_name: Option<String>,
    #[Demo(value = r#"Some(true)"#)]
    pub order_doctor_is_intern: Option<bool>,
    #[Demo(value = r#"Some(String::from("ว00000"))"#)]
    pub doctor_licenseno: Option<String>,
    #[Demo(value = r#"Some(String::from("นายแพทย์"))"#)]
    pub entryposition: Option<String>,

    #[Demo(value = r#"Some(String::from("009/99/91NWC10PTESHVK95PNAAA.webp,009/99/91NWC10PTESHVK95PNAAB.webp"))"#)]
    pub imgs: Option<String>,

    #[Demo(value = "datetime!(2023-12-31 23:59:59)")]
    pub create_datetime: PrimitiveDateTime,

    #[Demo(value = "vec![ProgressNoteItemType::demo()]")]
    pub progress_note_item_types: Vec<ProgressNoteItemType>,
}

impl ProgressNote {
    /// GET `EndPoint::IpdOrderProgressNote`
    pub async fn call_api_get_ipd(params: &ProgressNoteParams, app: Rc<AppState>) -> Result<Vec<Self>, AppError> {
        Self::get(&[EndPoint::IpdOrderProgressNote.base(), params.clone().query_string()].concat(), app).await
    }
    /// GET `EndPoint::OpdErOrderProgressNote`
    pub async fn call_api_get_opd_er(params: &ProgressNoteParams, app: Rc<AppState>) -> Result<Vec<Self>, AppError> {
        Self::get(&[EndPoint::OpdErOrderProgressNote.base(), params.clone().query_string()].concat(), app).await
    }

    async fn get(path: &str, app: Rc<AppState>) -> Result<Vec<Self>, AppError> {
        match fetch_json_api(path, "GET", None, app).await {
            Ok((response, true)) => {
                let response: Vec<Self> = serde_wasm_bindgen::from_value(response).map_err(|e| Source::SerdeWasm.to_teapot_error(e, "Fetch ProgressNote"))?;
                Ok(response)
            }
            Ok((app_error, false)) => {
                let error: AppError = serde_wasm_bindgen::from_value(app_error).map_err(|e| Source::SerdeWasm.to_teapot_error(e, "Fetch ProgressNote"))?;
                Err(error)
            }
            Err(e) => Err(Source::Js.to_teapot_error(e.dyn_ref::<JsString>().map(|s| s.into()).unwrap_or(String::from("fetch error")), "Fetch Json")),
        }
    }

    /// DELETE `EndPoint::IpdOrderProgressNoteId`
    /// DELETE `EndPoint::OpdErOrderProgressNoteId`
    pub async fn call_api_delete(is_ipd: bool, progress_note_id: u32, app: Rc<AppState>) -> Result<ExecuteResponse, AppError> {
        let endpoint = if is_ipd { EndPoint::IpdOrderProgressNoteId } else { EndPoint::OpdErOrderProgressNoteId };
        execute_fetch(&[endpoint.base(), progress_note_id.to_string()].concat(), "DELETE", None, app).await
    }
}

impl From<Rc<PreProgressNote>> for ProgressNote {
    fn from(item: Rc<PreProgressNote>) -> Self {
        ProgressNote {
            visit_type: VisitTypeId::Visit(String::new()),
            progress_note_id: item.progress_note_id,
            progress_note_date: item.progress_note_date,
            progress_note_time: item.progress_note_time,
            progress_note_owner_type: item.progress_note_owner_type.clone(),
            progress_note_doctor: item.progress_note_doctor.clone(),
            progress_note_enter_datetime: None,
            pre_order_progress_note_id: None,
            pre_order_progress_note_date: None,
            pre_order_progress_note_time: None,
            order_doctor_name: item.order_doctor_name.clone(),
            order_doctor_is_intern: item.order_doctor_is_intern.clone(),
            doctor_licenseno: item.doctor_licenseno.clone(),
            entryposition: item.entryposition.clone(),
            imgs: None,
            create_datetime: item.create_datetime,
            progress_note_item_types: item.progress_note_item_types.clone().into_iter().map(ProgressNoteItemType::from).collect(),
        }
    }
}

/// Type of Progress Note Item
#[derive(Clone, Demo, Deserialize, Serialize, ToSchema)]
#[schema(example = json!(ProgressNoteItemType::demo()))]
pub struct ProgressNoteItemType {
    #[Demo(value = "ProgressNoteTypeName::demo_note()")]
    pub progress_note_item_type: ProgressNoteTypeName,
    #[Demo(value = "vec![ProgressNoteItem::demo()]")]
    pub progress_note_items: Vec<ProgressNoteItem>,
}

impl From<PreProgressNoteItemType> for ProgressNoteItemType {
    fn from(item: PreProgressNoteItemType) -> Self {
        ProgressNoteItemType {
            progress_note_item_type: item.progress_note_item_type,
            progress_note_items: item.progress_note_items.into_iter().map(ProgressNoteItem::from).collect(),
        }
    }
}

/// Item of Progress Note
#[derive(Clone, Demo, Deserialize, Serialize, ToSchema)]
#[schema(example = json!(ProgressNoteItem::demo()))]
pub struct ProgressNoteItem {
    /// for generic over `an` or `opd_er_order_master_id` only
    #[Demo(value = r#"VisitTypeId::demo_ipd(String::from("660001234"))"#)]
    pub visit_type: VisitTypeId,
    #[Demo(value = "1")]
    pub progress_note_item_id: u32,
    #[Demo(value = "Some(1)")]
    pub progress_note_id: Option<u32>,
    #[Demo(value = r#"Some(String::from("note"))"#)]
    pub progress_note_item_type: Option<String>,
    #[Demo(value = r#"Some(String::from("Detail1"))"#)]
    pub progress_note_item_detail: Option<String>,
    #[Demo(value = r#"Some(String::from("Detail2"))"#)]
    pub progress_note_item_detail_2: Option<String>,
}

impl ProgressNoteItem {
    /// GET `EndPoint::IpdOrderProgressPrevious`
    pub async fn call_api_get_previous(params: &ProgressNoteParams, app: Rc<AppState>) -> Result<Vec<Self>, AppError> {
        match fetch_json_api(&[EndPoint::IpdOrderProgressPrevious.base(), params.clone().query_string()].concat(), "GET", None, app).await {
            Ok((response, true)) => {
                let response: Vec<Self> = serde_wasm_bindgen::from_value(response).map_err(|e| Source::SerdeWasm.to_teapot_error(e, "Fetch ProgressNoteItem"))?;
                Ok(response)
            }
            Ok((app_error, false)) => {
                let error: AppError = serde_wasm_bindgen::from_value(app_error).map_err(|e| Source::SerdeWasm.to_teapot_error(e, "Fetch ProgressNoteItem"))?;
                Err(error)
            }
            Err(e) => Err(Source::Js.to_teapot_error(e.dyn_ref::<JsString>().map(|s| s.into()).unwrap_or(String::from("fetch error")), "Fetch Json")),
        }
    }
}

impl From<PreProgressNoteItem> for ProgressNoteItem {
    fn from(item: PreProgressNoteItem) -> Self {
        ProgressNoteItem {
            visit_type: VisitTypeId::Visit(String::new()),
            progress_note_item_id: item.progress_note_item_id,
            progress_note_id: item.progress_note_id,
            progress_note_item_type: item.progress_note_item_type,
            progress_note_item_detail: item.progress_note_item_detail,
            progress_note_item_detail_2: item.progress_note_item_detail_2,
        }
    }
}

/// Type of Progress Note
#[derive(Copy, Clone, Demo, PartialEq, PartialOrd, Deserialize, Serialize, ToSchema)]
#[serde(rename_all = "kebab-case")]
#[schema(example = json!(ProgressNoteTypeName::demo_note()))]
pub enum ProgressNoteTypeName {
    ProblemList,
    Note,
    Subjective,
    Objective,
    Assessment,
    Plan,
}
impl ProgressNoteTypeName {
    pub fn string(&self) -> &'static str {
        match self {
            Self::ProblemList => "Problem",
            Self::Note => "Note",
            Self::Subjective => "Subjective",
            Self::Objective => "Objective",
            Self::Assessment => "Assessment",
            Self::Plan => "Plan",
        }
    }
    pub fn from_string(text: &str) -> Self {
        match text {
            "problem-list" => Self::ProblemList,
            "subjective" => Self::Subjective,
            "objective" => Self::Objective,
            "assessment" => Self::Assessment,
            "plan" => Self::Plan,
            _ => Self::Note,
        }
    }
}

/// Progress Note for save
#[derive(Debug, Demo, Deserialize, Serialize, ToSchema)]
#[schema(example = json!(ProgressNoteSave::demo()))]
pub struct ProgressNoteSave {
    /// for generic over `an` or `opd_er_order_master_id` only
    #[Demo(value = r#"VisitTypeId::demo_ipd(String::from("660001234"))"#)]
    pub visit_type: VisitTypeId,
    #[Demo(value = "Some(1)")]
    pub progress_note_id: Option<u32>,
    #[Demo(value = "Some(date!(2023-12-31))")]
    pub progress_note_for_past_date: Option<Date>,
    #[Demo(value = "Some(time!(23:59:59))")]
    pub progress_note_for_past_time: Option<Time>,
    #[Demo(value = r#"String::from("008")"#)]
    pub progress_note_doctor: String,
    #[Demo(value = r#"String::from("nurse")"#)]
    pub progress_note_owner_type: String,
    #[Demo(value = "vec![ProgressNoteItemSave::demo()]")]
    pub progress_note_items: Vec<ProgressNoteItemSave>,
}

impl ProgressNoteSave {
    /// - POST `EndPoint::IpdOrderProgressNote`
    /// - POST `EndPoint::OpdErOrderProgressNote`
    pub async fn call_api_post(&self, app: Rc<AppState>) -> Result<(u32, Vec<ExecuteResponse>), AppError> {
        let (path, is_valid) = match &self.visit_type {
            VisitTypeId::Ipd(an) | VisitTypeId::PreAdmit(an) => (EndPoint::IpdOrderProgressNote, !an.is_empty()),
            VisitTypeId::OpdEr(_, opd_er_order_master_id) => (EndPoint::OpdErOrderProgressNote, *opd_er_order_master_id > 0),
            VisitTypeId::Visit(_) => (EndPoint::Unknown, false),
        };

        if is_valid {
            let body_json = serde_json::to_string(self).map_err(|e| Source::SerdeJson.to_teapot_error(e, "Send ProgressNoteSave"))?;
            let body = serde_wasm_bindgen::to_value(&body_json).map_err(|e| Source::SerdeWasm.to_teapot_error(e, "Send ProgressNoteSave"))?;

            execute_fetch_vec_with_u32(&path.base(), "POST", Some(&body), app).await
        } else {
            Err(AppError::app_400("Check ProgressNoteSave"))
        }
    }
}

/// Item of Progress Note for save
#[derive(Debug, Demo, Deserialize, Serialize, ToSchema)]
#[schema(example = json!(ProgressNoteItemSave::demo()))]
pub struct ProgressNoteItemSave {
    #[Demo(value = r#"Some(String::from("note"))"#)]
    pub progress_note_item_type: Option<String>,
    #[Demo(value = r#"Some(String::from("Detail1"))"#)]
    pub progress_note_item_detail: Option<String>,
    #[Demo(value = r#"Some(String::from("Detail2"))"#)]
    pub progress_note_item_detail_2: Option<String>,
}

#[derive(Demo, Deserialize, Serialize, FromRow)]
pub struct ProgressNoteOnly {
    #[Demo(value = "1")]
    pub progress_note_id: u32,
    #[Demo(value = "date!(2023-12-31)")]
    pub progress_note_date: Date,
    #[Demo(value = "time!(23:59:59)")]
    pub progress_note_time: Time,
    #[Demo(value = r#"String::from("nurse")"#)]
    pub progress_note_owner_type: String,
    #[Demo(value = r#"String::from("008")"#)]
    pub progress_note_doctor: String,
    #[Demo(value = "Some(datetime!(2023-12-31 23:59:59))")]
    pub progress_note_enter_datetime: Option<PrimitiveDateTime>,
    #[Demo(value = "Some(1)")]
    pub pre_order_progress_note_id: Option<u32>,
    #[Demo(value = "Some(date!(2023-12-31))")]
    pub pre_order_progress_note_date: Option<Date>,
    #[Demo(value = "Some(time!(23:59:59))")]
    pub pre_order_progress_note_time: Option<Time>,
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
    #[sqlx(skip)]
    #[Demo(value = "vec![ProgressNoteItemOnly::demo()]")]
    pub progress_note_items: Vec<ProgressNoteItemOnly>,
}

impl PartialEq for ProgressNoteOnly {
    fn eq(&self, other: &Self) -> bool {
        // self.progress_note_id == other.progress_note_id &&
        self.progress_note_date == other.progress_note_date
            && self.progress_note_time == other.progress_note_time
            && self.progress_note_owner_type == other.progress_note_owner_type
            && self.progress_note_doctor == other.progress_note_doctor
            && self.progress_note_enter_datetime == other.progress_note_enter_datetime
            && self.pre_order_progress_note_id == other.pre_order_progress_note_id
            && self.pre_order_progress_note_date == other.pre_order_progress_note_date
            && self.pre_order_progress_note_time == other.pre_order_progress_note_time
            && self.create_user == other.create_user
            && self.create_datetime == other.create_datetime
            && self.update_user == other.update_user
            && self.update_datetime == other.update_datetime
            && self.version == other.version
            && if self.progress_note_items.len() == other.progress_note_items.len() {
                self.progress_note_items.iter().zip(other.progress_note_items.iter()).all(|(a, b)| a.eq(b))
            } else {
                false
            }
    }
}

#[derive(Demo, Deserialize, Serialize, FromRow)]
pub struct ProgressNoteItemOnly {
    #[Demo(value = "1")]
    pub progress_note_item_id: u32,
    #[Demo(value = "Some(1)")]
    pub progress_note_id: Option<u32>,
    #[Demo(value = r#"Some(String::from("note"))"#)]
    pub progress_note_item_type: Option<String>,
    #[Demo(value = r#"Some(String::from("Detail1"))"#)]
    pub progress_note_item_detail: Option<String>,
    #[Demo(value = r#"Some(String::from("Detail2"))"#)]
    pub progress_note_item_detail_2: Option<String>,
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

impl PartialEq for ProgressNoteItemOnly {
    fn eq(&self, other: &Self) -> bool {
        // self.progress_note_item_id == other.progress_note_item_id &&
        // self.progress_note_id == other.progress_note_id &&
        self.progress_note_item_type == other.progress_note_item_type
            && self.progress_note_item_detail == other.progress_note_item_detail
            && self.progress_note_item_detail_2 == other.progress_note_item_detail_2
            && self.create_user == other.create_user
            && self.create_datetime == other.create_datetime
            && self.update_user == other.update_user
            && self.update_datetime == other.update_datetime
            && self.version == other.version
    }
}
