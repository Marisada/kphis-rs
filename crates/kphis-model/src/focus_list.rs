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
pub struct FocusListParams {
    pub start_date: Option<Date>,
    pub end_date: Option<Date>,
    pub status: Option<String>,
    pub fclist_id: Option<u32>,
    pub version: Option<i32>,
}

impl QueryString for FocusListParams {
    fn from_tuples(params: &[(String, String)]) -> Option<Self> {
        (!params.is_empty()).then(|| Self {
            start_date: find_qs(params, "start_date").and_then(|s| date_8601(&s)),
            end_date: find_qs(params, "end_date").and_then(|s| date_8601(&s)),
            status: find_qs(params, "status"),
            fclist_id: find_qs(params, "fclist_id").and_then(|s| s.parse::<u32>().ok()),
            version: find_qs(params, "version").and_then(|s| s.parse::<i32>().ok()),
        })
    }

    fn query_string(&self) -> String {
        let mut queries = Vec::with_capacity(5);

        if let Some(start_date) = &self.start_date {
            queries.push(["start_date=", &start_date.to_string()].concat());
        }
        if let Some(end_date) = &self.end_date {
            queries.push(["end_date=", &end_date.to_string()].concat());
        }
        if let Some(status) = &self.status {
            queries.push(["status=", status].concat());
        }
        if let Some(fclist_id) = &self.fclist_id {
            queries.push(["fclist_id=", &fclist_id.to_string()].concat());
        }
        if let Some(version) = &self.version {
            queries.push(["version=", &version.to_string()].concat());
        }

        (!queries.is_empty()).then(|| ["?", &queries.join("&")].concat()).unwrap_or_default()
    }
}

/// Focus List with associated data
#[derive(Clone, Demo, Deserialize, Serialize, FromRow, ToSchema)]
#[schema(example = json!(FocusList::demo()))]
pub struct FocusList {
    #[Demo(value = "1")]
    pub fclist_id: u32,
    #[Demo(value = "1")]
    pub smp_id: u32,
    #[Demo(value = "1")]
    pub focus_id: u32,
    #[Demo(value = r#"Some(String::from("Focus"))"#)]
    pub focus_text: Option<String>,
    #[Demo(value = r#"Some(String::from("1"))"#)]
    goal_id: Option<String>,
    #[Demo(value = r#"Some(String::from("Goal"))"#)]
    pub goal_text: Option<String>,
    #[Demo(value = "Some(date!(2023-12-31))")]
    pub fclist_stdate: Option<Date>,
    #[Demo(value = "time!(23:59:59)")]
    pub fclist_sttime: Time,
    #[Demo(value = "Some(date!(2023-12-31))")]
    pub fclist_enddate: Option<Date>,
    #[Demo(value = "Some(time!(23:59:59))")]
    pub fclist_endtime: Option<Time>,
    /// 1 = active, 2 = solved
    #[Demo(value = r#"String::from("1")"#)]
    pub fclist_status: String,
    #[Demo(value = "1")]
    pub version: i32,

    #[Demo(value = r#"String::from("user")"#)]
    create_user_fclist: String,
    #[Demo(value = "Some(1)")]
    pub subgroup: Option<u32>,
    #[Demo(value = r#"Some(String::from("FOCUS"))"#)]
    pub focus_name: Option<String>,
    #[Demo(value = r#"Some(String::from("Y"))"#)]
    focus_status: Option<String>,
    #[Demo(value = r#"Some(String::from("With Approval"))"#)]
    pub dchtype_name: Option<String>,
    #[Demo(value = r#"Some(String::from("01"))"#)]
    pub dchtype: Option<String>,
    #[Demo(value = "Some(date!(2023-12-31))")]
    pub dchdate: Option<Date>,
    #[Demo(value = "Some(time!(23:59:59))")]
    pub dchtime: Option<Time>,

    #[Demo(value = r#"Some(String::from("Ms.Nurse"))"#)]
    pub create_user_name: Option<String>,
    #[Demo(value = r#"Some(String::from("1234567"))"#)]
    pub create_user_licenseno: Option<String>,
    #[Demo(value = r#"Some(String::from("พยาบาลวิชาชีพปฏิบัติการ"))"#)]
    pub create_user_entryposition: Option<String>,
    #[Demo(value = r#"Some(String::from("Ms.Nurse"))"#)]
    pub update_user_name: Option<String>,
    #[Demo(value = r#"Some(String::from("1234567"))"#)]
    pub update_user_licenseno: Option<String>,
    #[Demo(value = r#"Some(String::from("พยาบาลวิชาชีพปฏิบัติการ"))"#)]
    pub update_user_entryposition: Option<String>,

    #[Demo(value = r#"Some(String::from("1^GOAL|999^อื่นๆ"))"#)]
    pub goals: Option<String>,
    #[Demo(value = "true")]
    pub used: bool,
}

impl FocusList {
    /// GET `EndPoint::IpdFocusListAn`
    pub async fn call_api_get_ipd(an: &str, params: &FocusListParams, app: Rc<AppState>) -> Result<Vec<Self>, AppError> {
        Self::get(&[&EndPoint::IpdFocusListAn.base(), an, &params.query_string()].concat(), app).await
    }

    /// GET `EndPoint::OpdErFocusListId`
    pub async fn call_api_get_opd_er(opd_er_order_master_id: u32, params: &FocusListParams, app: Rc<AppState>) -> Result<Vec<Self>, AppError> {
        Self::get(&[EndPoint::OpdErFocusListId.base(), opd_er_order_master_id.to_string(), params.query_string()].concat(), app).await
    }

    async fn get(path: &str, app: Rc<AppState>) -> Result<Vec<Self>, AppError> {
        match fetch_json_api(path, "GET", None, app).await {
            Ok((response, true)) => {
                let response: Vec<Self> = serde_wasm_bindgen::from_value(response).map_err(|e| Source::SerdeWasm.to_teapot_error(e, "Fetch FocusList"))?;
                Ok(response)
            }
            Ok((app_error, false)) => {
                let error: AppError = serde_wasm_bindgen::from_value(app_error).map_err(|e| Source::SerdeWasm.to_teapot_error(e, "Fetch FocusList"))?;
                Err(error)
            }
            Err(e) => Err(Source::Js.to_teapot_error(e.dyn_ref::<JsString>().map(|s| s.into()).unwrap_or(String::from("fetch error")), "Fetch Json")),
        }
    }

    /// DELETE `EndPoint::IpdFocusListAn`
    pub async fn call_api_delete_ipd(an: &str, params: &FocusListParams, app: Rc<AppState>) -> Result<Vec<ExecuteResponse>, AppError> {
        execute_fetch_vec(&[&EndPoint::IpdFocusListAn.base(), an, &params.query_string()].concat(), "DELETE", None, app).await
    }

    /// DELETE `EndPoint::OpdErFocusListId`
    pub async fn call_api_delete_opd_er(opd_er_order_master_id: u32, params: &FocusListParams, app: Rc<AppState>) -> Result<Vec<ExecuteResponse>, AppError> {
        execute_fetch_vec(
            &[EndPoint::OpdErFocusListId.base(), opd_er_order_master_id.to_string(), params.query_string()].concat(),
            "DELETE",
            None,
            app,
        )
        .await
    }
}

#[derive(Clone, Default, Deserialize, IntoParams)]
pub struct FocusListSaveParams {
    pub hn: Option<String>,
}

impl QueryString for FocusListSaveParams {
    fn from_tuples(params: &[(String, String)]) -> Option<Self> {
        (!params.is_empty()).then(|| Self { hn: find_qs(params, "hn") })
    }

    fn query_string(&self) -> String {
        let mut queries = Vec::with_capacity(1);

        if let Some(hn) = &self.hn {
            queries.push(["hn=", hn].concat());
        }

        (!queries.is_empty()).then(|| ["?", &queries.join("&")].concat()).unwrap_or_default()
    }
}

/// Focus List for save
#[derive(Clone, Demo, Deserialize, Serialize, ToSchema)]
#[schema(example = json!(FocusListSave::demo()))]
pub struct FocusListSave {
    #[Demo(value = "Some(1)")]
    pub fclist_id: Option<u32>,
    #[Demo(value = "1")]
    pub smp_id: u32,
    #[Demo(value = "1")]
    pub focus_id: u32,
    #[Demo(value = r#"Some(String::from("Focus"))"#)]
    pub focus_text: Option<String>,
    // goal_id: String,
    #[Demo(value = "vec![1,999]")]
    pub goal_ids: Vec<u32>,
    #[Demo(value = r#"Some(String::from("Goal"))"#)]
    pub goal_text: Option<String>,
    #[Demo(value = "Some(date!(2023-12-31))")]
    pub fclist_stdate: Option<Date>,
    #[Demo(value = "time!(23:59:59)")]
    pub fclist_sttime: Time,
    #[Demo(value = "Some(date!(2023-12-31))")]
    pub fclist_enddate: Option<Date>,
    #[Demo(value = "Some(time!(23:59:59))")]
    pub fclist_endtime: Option<Time>,
    /// 1 = active, 2 = solved
    #[Demo(value = r#"String::from("1")"#)]
    pub fclist_status: String,
    #[Demo(value = "1")]
    pub version: i32,
}

impl FocusListSave {
    /// POST `EndPoint::IpdFocusListAn`
    pub async fn call_api_post_ipd(&self, an: &str, params: &FocusListSaveParams, app: Rc<AppState>) -> Result<(u32, Vec<ExecuteResponse>), AppError> {
        self.save(&[&EndPoint::IpdFocusListAn.base(), an, &params.query_string()].concat(), app).await
    }

    /// POST `EndPoint::OpdErFocusListId`
    pub async fn call_api_post_opd_er(&self, opd_er_order_master_id: u32, app: Rc<AppState>) -> Result<(u32, Vec<ExecuteResponse>), AppError> {
        self.save(&[EndPoint::OpdErFocusListId.base(), opd_er_order_master_id.to_string()].concat(), app).await
    }

    async fn save(&self, path: &str, app: Rc<AppState>) -> Result<(u32, Vec<ExecuteResponse>), AppError> {
        let body_json = serde_json::to_string(self).map_err(|e| Source::SerdeJson.to_teapot_error(e, "Send FocusListSave"))?;
        let body = serde_wasm_bindgen::to_value(&body_json).map_err(|e| Source::SerdeWasm.to_teapot_error(e, "Send FocusListSave"))?;
        execute_fetch_vec_with_u32(path, "POST", Some(&body), app).await
    }
}

#[derive(Debug, Demo, Deserialize, Serialize, FromRow)]
pub struct FocusListOnly {
    #[Demo(value = "1")]
    pub fclist_id: u32,
    #[Demo(value = "1")]
    pub smp_id: u32,
    #[Demo(value = "1")]
    pub focus_id: u32,
    #[Demo(value = r#"Some(String::from("Focus"))"#)]
    pub focus_text: Option<String>,
    // goal_id: Option<String>,
    #[Demo(value = r#"Some(String::from("Goal"))"#)]
    pub goal_text: Option<String>,
    #[Demo(value = "Some(date!(2023-12-31))")]
    pub fclist_stdate: Option<Date>,
    #[Demo(value = "time!(23:59:59)")]
    pub fclist_sttime: Time,
    #[Demo(value = "Some(date!(2023-12-31))")]
    pub fclist_enddate: Option<Date>,
    #[Demo(value = "Some(time!(23:59:59))")]
    pub fclist_endtime: Option<Time>,
    /// 1 = active, 2 = solved
    #[Demo(value = r#"String::from("1")"#)]
    pub fclist_status: String,
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
    #[Demo(value = "vec![FocusListGoalItemOnly::demo()]")]
    pub focus_list_goal_items: Vec<FocusListGoalItemOnly>,
}

impl PartialEq for FocusListOnly {
    fn eq(&self, other: &Self) -> bool {
        // self.fclist_id == other.fclist_id &&
        self.smp_id == other.smp_id
            && self.focus_id == other.focus_id
            && self.focus_text == other.focus_text
            && self.goal_text == other.goal_text
            && self.fclist_stdate == other.fclist_stdate
            && self.fclist_sttime == other.fclist_sttime
            && self.fclist_enddate == other.fclist_enddate
            && self.fclist_endtime == other.fclist_endtime
            && self.fclist_status == other.fclist_status
            && self.create_user == other.create_user
            && self.create_datetime == other.create_datetime
            && self.update_user == other.update_user
            && self.update_datetime == other.update_datetime
            && self.version == other.version
            && if self.focus_list_goal_items.len() == other.focus_list_goal_items.len() {
                self.focus_list_goal_items.iter().zip(other.focus_list_goal_items.iter()).all(|(a, b)| a.eq(b))
            } else {
                false
            }
    }
}

#[derive(Debug, Demo, Deserialize, Serialize, FromRow)]
pub struct FocusListGoalItemOnly {
    #[Demo(value = "1")]
    pub fclist_item_id: u32,
    #[Demo(value = "Some(1)")]
    pub fclist_id: Option<u32>,
    #[Demo(value = "Some(1)")]
    pub goal_id: Option<u32>,
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

impl PartialEq for FocusListGoalItemOnly {
    fn eq(&self, other: &Self) -> bool {
        // self.fclist_item_id == other.fclist_item_id &&
        // self.fclist_id == other.fclist_id &&
        self.goal_id == other.goal_id
            && self.create_user == other.create_user
            && self.create_datetime == other.create_datetime
            && self.update_user == other.update_user
            && self.update_datetime == other.update_datetime
            && self.version == other.version
    }
}
