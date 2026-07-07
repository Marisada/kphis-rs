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
    fetch::{ExecuteResponse, execute_fetch_vec, execute_fetch_vec_with_u32, fetch_json_api},
};

#[derive(Clone, Default, Deserialize, IntoParams)]
pub struct FocusNoteParams {
    pub start_date: Option<Date>,
    pub end_date: Option<Date>,
    /// Some(0) is NULL, None is not defined
    pub fclist_id: Option<u32>,
    pub fcnote_id: Option<u32>,
    pub version: Option<i32>,
}

impl QueryString for FocusNoteParams {
    fn from_tuples(params: &[(String, String)]) -> Option<Self> {
        (!params.is_empty()).then(|| Self {
            start_date: find_qs(params, "start_date").and_then(|s| date_8601(&s)),
            end_date: find_qs(params, "end_date").and_then(|s| date_8601(&s)),
            fclist_id: find_qs(params, "fclist_id").and_then(|s| s.parse::<u32>().ok()),
            fcnote_id: find_qs(params, "fcnote_id").and_then(|s| s.parse::<u32>().ok()),
            version: find_qs(params, "version").and_then(|s| s.parse::<i32>().ok()),
        })
    }

    fn query_string(&self) -> String {
        let mut queries = Vec::with_capacity(4);

        if let Some(start_date) = &self.start_date {
            queries.push(["start_date=", &start_date.to_string()].concat());
        }
        if let Some(end_date) = &self.end_date {
            queries.push(["end_date=", &end_date.to_string()].concat());
        }
        if let Some(fclist_id) = &self.fclist_id {
            queries.push(["fclist_id=", &fclist_id.to_string()].concat());
        }
        if let Some(fcnote_id) = &self.fcnote_id {
            queries.push(["fcnote_id=", &fcnote_id.to_string()].concat());
        }
        if let Some(version) = &self.version {
            queries.push(["version=", &version.to_string()].concat());
        }

        (!queries.is_empty()).then(|| ["?", &queries.join("&")].concat()).unwrap_or_default()
    }
}

/// Focus Note with associated data
#[derive(Clone, Debug, Demo, Deserialize, Serialize, FromRow, ToSchema)]
#[schema(example = json!(FocusNote::demo()))]
pub struct FocusNote {
    #[Demo(value = "1")]
    pub fcnote_id: u32,
    #[Demo(value = r#"Some(String::from("Sick"))"#)]
    pub general_symptoms: Option<String>,
    #[Demo(value = "Some(1)")]
    pub fclist_id: Option<u32>,
    #[Demo(value = r#"Some(String::from("Assessment"))"#)]
    pub assessment: Option<String>,
    /// NOT USE
    #[Demo(default)]
    intvt_id: Option<String>,
    #[Demo(value = r#"Some(String::from("Intervention"))"#)]
    pub intvt_text: Option<String>,
    #[Demo(value = r#"Some(String::from("Evaluation"))"#)]
    pub evalution: Option<String>,
    /// NOT USE
    #[Demo(default)]
    dlc_id: Option<String>,
    #[Demo(value = r#"Some(String::from("Daily Care"))"#)]
    pub dlc_text: Option<String>,
    #[Demo(value = r#"Some(String::from("Other Note"))"#)]
    pub other: Option<String>,

    #[Demo(value = "Some(date!(2023-12-31))")]
    pub fcnote_date: Option<Date>,
    #[Demo(value = "Some(time!(23:59:59))")]
    pub fcnote_time: Option<Time>,
    #[Demo(value = r#"Some(String::from("1"))"#)]
    pub fcnote_patient_type: Option<String>,
    #[Demo(value = "1")]
    pub version: i32,

    #[Demo(value = "Some(1)")]
    smp_id: Option<u32>,
    #[Demo(value = "Some(1)")]
    focus_id: Option<u32>,
    #[Demo(value = r#"Some(String::from("Focus"))"#)]
    pub focus_text: Option<String>,
    #[Demo(value = r#"Some(String::from("FOCUS"))"#)]
    pub focus_name: Option<String>,
    #[Demo(value = r#"Some(String::from("1^Intervention|9999^อื่นๆ"))"#)]
    pub intvts: Option<String>,
    #[Demo(value = r#"Some(String::from("1^Care|2^Care"))"#)]
    pub dlcs: Option<String>,
    #[Demo(value = r#"Some(String::from("009/99/91NWC10PTESHVK95PNAAA.webp,009/99/91NWC10PTESHVK95PNAAB.webp"))"#)]
    pub a_imgs: Option<String>,
    #[Demo(value = r#"Some(String::from("009/99/91NWC10PTESHVK95PNAAA.webp,009/99/91NWC10PTESHVK95PNAAB.webp"))"#)]
    pub e_imgs: Option<String>,

    #[Demo(value = r#"Some(String::from("Dr.Doctor"))"#)]
    pub user_name: Option<String>,
    #[Demo(value = r#"Some(String::from("นายแพทย์"))"#)]
    pub entryposition: Option<String>,
    #[Demo(value = r#"Some(String::from("ว.11111"))"#)]
    pub licenseno: Option<String>,
    #[Demo(value = r#"Some(String::from("007"))"#)]
    pub doctorcode: Option<String>,
}

impl FocusNote {
    /// GET `EndPoint::IpdFocusNoteAn`
    pub async fn call_api_get_ipd(an: &str, params: &FocusNoteParams, app: Rc<AppState>) -> Result<Vec<Self>, AppError> {
        Self::get(&[&EndPoint::IpdFocusNoteAn.base(), an, &params.query_string()].concat(), app).await
    }

    /// GET `EndPoint::OpdErFocusNoteId`
    pub async fn call_api_get_opd_er(opd_er_order_master_id: u32, params: &FocusNoteParams, app: Rc<AppState>) -> Result<Vec<Self>, AppError> {
        Self::get(&[EndPoint::OpdErFocusNoteId.base(), opd_er_order_master_id.to_string(), params.query_string()].concat(), app).await
    }

    async fn get(path: &str, app: Rc<AppState>) -> Result<Vec<Self>, AppError> {
        match fetch_json_api(path, "GET", None, app).await {
            Ok((response, true)) => {
                let response: Vec<Self> = serde_wasm_bindgen::from_value(response).map_err(|e| Source::SerdeWasm.to_teapot_error(e, "Fetch FocusNote"))?;
                Ok(response)
            }
            Ok((app_error, false)) => {
                let error: AppError = serde_wasm_bindgen::from_value(app_error).map_err(|e| Source::SerdeWasm.to_teapot_error(e, "Fetch FocusNote"))?;
                Err(error)
            }
            Err(e) => Err(Source::Js.to_teapot_error(e.dyn_ref::<JsString>().map(|s| s.into()).unwrap_or(String::from("fetch error")), "Fetch Json")),
        }
    }

    /// DELETE `EndPoint::IpdFocusNoteAn`
    pub async fn call_api_delete_ipd(an: &str, params: &FocusNoteParams, app: Rc<AppState>) -> Result<Vec<ExecuteResponse>, AppError> {
        execute_fetch_vec(&[&EndPoint::IpdFocusNoteAn.base(), an, &params.query_string()].concat(), "DELETE", None, app).await
    }

    /// DELETE `EndPoint::OpdErFocusNoteId`
    pub async fn call_api_delete_opd_er(opd_er_order_master_id: u32, params: &FocusNoteParams, app: Rc<AppState>) -> Result<Vec<ExecuteResponse>, AppError> {
        execute_fetch_vec(
            &[EndPoint::OpdErFocusNoteId.base(), opd_er_order_master_id.to_string(), params.query_string()].concat(),
            "DELETE",
            None,
            app,
        )
        .await
    }
}

impl PartialEq for FocusNote {
    fn eq(&self, other: &Self) -> bool {
        self.fcnote_id == other.fcnote_id && self.fclist_id == other.fclist_id && self.fcnote_date == other.fcnote_date && self.fcnote_time == other.fcnote_time
    }
}

#[derive(Clone, Default, Deserialize, IntoParams)]
pub struct FocusNoteSaveParams {
    pub hn: Option<String>,
    pub ward: Option<String>,
}

impl QueryString for FocusNoteSaveParams {
    fn from_tuples(params: &[(String, String)]) -> Option<Self> {
        (!params.is_empty()).then(|| Self {
            hn: find_qs(params, "hn"),
            ward: find_qs(params, "ward"),
        })
    }

    fn query_string(&self) -> String {
        let mut queries = Vec::with_capacity(2);

        if let Some(hn) = &self.hn {
            queries.push(["hn=", hn].concat());
        }
        if let Some(ward) = &self.ward {
            queries.push(["ward=", ward].concat());
        }

        (!queries.is_empty()).then(|| ["?", &queries.join("&")].concat()).unwrap_or_default()
    }
}

/// Focus Note for save
#[derive(Clone, Demo, Deserialize, Serialize, ToSchema)]
#[schema(example = json!(FocusNoteSave::demo()))]
pub struct FocusNoteSave {
    #[Demo(value = "Some(1)")]
    pub fcnote_id: Option<u32>,
    #[Demo(value = r#"Some(String::from("Sick"))"#)]
    pub general_symptoms: Option<String>,
    #[Demo(value = "Some(1)")]
    pub fclist_id: Option<u32>,
    #[Demo(value = r#"Some(String::from("Assessment"))"#)]
    pub assessment: Option<String>,
    #[Demo(value = "vec![1,9999]")]
    pub intvt_ids: Vec<u32>,
    #[Demo(value = r#"Some(String::from("Intervention"))"#)]
    pub intvt_text: Option<String>,
    #[Demo(value = r#"Some(String::from("Evaluation"))"#)]
    pub evalution: Option<String>,
    #[Demo(value = "vec![1,2]")]
    pub dlc_ids: Vec<u32>,
    #[Demo(value = r#"Some(String::from("Daily Care"))"#)]
    pub dlc_text: Option<String>,
    #[Demo(value = r#"Some(String::from("Other Note"))"#)]
    pub other: Option<String>,

    #[Demo(value = "Some(date!(2023-12-31))")]
    pub fcnote_date: Option<Date>,
    #[Demo(value = "Some(time!(23:59:59))")]
    pub fcnote_time: Option<Time>,
    #[Demo(value = r#"Some(String::from("1"))"#)]
    pub fcnote_patient_type: Option<String>,
    #[Demo(value = "1")]
    pub version: i32,
}

impl FocusNoteSave {
    /// POST `EndPoint::IpdFocusNoteAn`
    pub async fn call_api_post_ipd(&self, an: &str, params: &FocusNoteSaveParams, app: Rc<AppState>) -> Result<(u32, Vec<ExecuteResponse>), AppError> {
        self.save(&[&EndPoint::IpdFocusNoteAn.base(), an, &params.query_string()].concat(), app).await
    }

    /// POST `EndPoint::OpdErFocusNoteId`
    pub async fn call_api_post_opd_er(&self, opd_er_order_master_id: u32, app: Rc<AppState>) -> Result<(u32, Vec<ExecuteResponse>), AppError> {
        self.save(&[EndPoint::OpdErFocusNoteId.base(), opd_er_order_master_id.to_string()].concat(), app).await
    }

    async fn save(&self, path: &str, app: Rc<AppState>) -> Result<(u32, Vec<ExecuteResponse>), AppError> {
        let body_json = serde_json::to_string(self).map_err(|e| Source::SerdeJson.to_teapot_error(e, "Send FocusNoteSave"))?;
        let body = serde_wasm_bindgen::to_value(&body_json).map_err(|e| Source::SerdeWasm.to_teapot_error(e, "Send FocusNoteSave"))?;
        execute_fetch_vec_with_u32(path, "POST", Some(&body), app).await
    }
}

#[derive(Debug, Demo, Deserialize, Serialize, FromRow)]
pub struct FocusNoteOnly {
    #[Demo(value = "1")]
    pub fcnote_id: u32,
    #[Demo(value = r#"Some(String::from("Sick"))"#)]
    pub general_symptoms: Option<String>,
    #[Demo(value = "Some(1)")]
    pub fclist_id: Option<u32>,
    #[Demo(value = r#"Some(String::from("Assessment"))"#)]
    pub assessment: Option<String>,
    // pub intvt_id: Option<String>,
    #[Demo(value = r#"Some(String::from("Intervention"))"#)]
    pub intvt_text: Option<String>,
    #[Demo(value = r#"Some(String::from("Evaluation"))"#)]
    pub evalution: Option<String>,
    // pub dlc_id: Option<String>,
    #[Demo(value = r#"Some(String::from("Daily Care"))"#)]
    pub dlc_text: Option<String>,
    #[Demo(value = r#"Some(String::from("Other Note"))"#)]
    pub other: Option<String>,
    #[Demo(value = "Some(date!(2023-12-31))")]
    pub fcnote_date: Option<Date>,
    #[Demo(value = "Some(time!(23:59:59))")]
    pub fcnote_time: Option<Time>,
    #[Demo(value = r#"Some(String::from("1"))"#)]
    pub fcnote_patient_type: Option<String>,
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
    #[Demo(value = "vec![FocusNoteIntvtItemOnly::demo()]")]
    pub focus_note_intvt_items: Vec<FocusNoteIntvtItemOnly>,
    #[sqlx(skip)]
    #[Demo(value = "vec![FocusNoteDlcItemOnly::demo()]")]
    pub focus_note_dlc_items: Vec<FocusNoteDlcItemOnly>,
}

impl PartialEq for FocusNoteOnly {
    fn eq(&self, other: &Self) -> bool {
        // self.fcnote_id == other.fcnote_id &&
        self.general_symptoms == other.general_symptoms &&
        // self.fclist_id == other.fclist_id &&
        self.assessment == other.assessment &&
        // self.intvt_id == other.intvt_id &&
        self.intvt_text == other.intvt_text &&
        self.evalution == other.evalution &&
        // self.dlc_id == other.dlc_id &&
        self.dlc_text == other.dlc_text &&
        self.other == other.other &&
        self.fcnote_date == other.fcnote_date &&
        self.fcnote_time == other.fcnote_time &&
        self.fcnote_patient_type == other.fcnote_patient_type &&
        self.create_user == other.create_user &&
        self.create_datetime == other.create_datetime &&
        self.update_user == other.update_user &&
        self.update_datetime == other.update_datetime &&
        self.version == other.version &&
        if self.focus_note_intvt_items.len() == other.focus_note_intvt_items.len() {
            self.focus_note_intvt_items.iter().zip(other.focus_note_intvt_items.iter()).all(|(a, b)| a.eq(b))
        } else {
            false
        } &&
        if self.focus_note_dlc_items.len() == other.focus_note_dlc_items.len() {
            self.focus_note_dlc_items.iter().zip(other.focus_note_dlc_items.iter()).all(|(a, b)| a.eq(b))
        } else {
            false
        }
    }
}

#[derive(Debug, Demo, Deserialize, Serialize, FromRow)]
pub struct FocusNoteIntvtItemOnly {
    #[Demo(value = "1")]
    pub fcnote_intvt_item_id: u32,
    #[Demo(value = "Some(1)")]
    pub fcnote_id: Option<u32>,
    #[Demo(value = "Some(1)")]
    pub intvt_id: Option<u32>,
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

impl PartialEq for FocusNoteIntvtItemOnly {
    fn eq(&self, other: &Self) -> bool {
        // self.fcnote_intvt_item_id == other.fcnote_intvt_item_id &&
        // self.fcnote_id == other.fcnote_id &&
        self.intvt_id == other.intvt_id
            && self.create_user == other.create_user
            && self.create_datetime == other.create_datetime
            && self.update_user == other.update_user
            && self.update_datetime == other.update_datetime
            && self.version == other.version
    }
}

#[derive(Debug, Demo, Deserialize, Serialize, FromRow)]
pub struct FocusNoteDlcItemOnly {
    #[Demo(value = "1")]
    pub fcnote_dlc_item_id: u32,
    #[Demo(value = "Some(1)")]
    pub fcnote_id: Option<u32>,
    #[Demo(value = "Some(1)")]
    pub dlc_id: Option<u32>,
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

impl PartialEq for FocusNoteDlcItemOnly {
    fn eq(&self, other: &Self) -> bool {
        // self.fcnote_dlc_item_id == other.fcnote_dlc_item_id &&
        // self.fcnote_id == other.fcnote_id &&
        self.dlc_id == other.dlc_id
            && self.create_user == other.create_user
            && self.create_datetime == other.create_datetime
            && self.update_user == other.update_user
            && self.update_datetime == other.update_datetime
            && self.version == other.version
    }
}
