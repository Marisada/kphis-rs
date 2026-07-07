use derive_demo::Demo;
use js_sys::JsString;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use std::{cmp::Ordering, rc::Rc};
use time::{PrimitiveDateTime, macros::datetime};
use utoipa::{IntoParams, ToSchema};
use wasm_bindgen::JsCast;

use kphis_util::{
    error::{AppError, Source},
    util::str_some,
};

use crate::{
    app::AppState,
    endpoint::{EndPoint, QueryString, find_qs},
    fetch::{ExecuteResponse, execute_fetch, execute_fetch_text, fetch_json_api},
    route::Route,
};

/// SSE message sending to server
#[derive(Clone, Debug, Demo, Default, Deserialize, Serialize, ToSchema)]
#[schema(example = json!(SsePostMessage::demo()))]
pub struct SsePostMessage {
    #[Demo(value = r#"String::from("Message")"#)]
    pub message: String,
    #[Demo(value = r#"Some(String::from("007"))"#)]
    pub person: Option<String>,
    #[Demo(value = r#"Some(String::from("01"))"#)]
    pub ward: Option<String>,
    #[Demo(value = r#"Some(1)"#)]
    pub spclty_id: Option<u32>,
    #[Demo(value = "Some(Route::demo_info())")]
    pub route: Option<Route>,
    #[Demo(value = "Some(SseData::demo())")]
    pub reference: Option<SseData>,
}

impl SsePostMessage {
    /// POST `EndPoint::SseMessage`
    pub async fn call_api_post(&self, app: Rc<AppState>) -> Result<String, AppError> {
        let path = EndPoint::SseMessage.base();

        let body_json = serde_json::to_string(self).map_err(|e| Source::SerdeJson.to_teapot_error(e, "Send SSE"))?;

        let body = serde_wasm_bindgen::to_value(&body_json).map_err(|e| Source::SerdeWasm.to_teapot_error(e, "Send SSE"))?;

        execute_fetch_text(&path, "POST", Some(&body), app).await
    }
}

/// SSE Message returned from server.
#[derive(Clone, Debug, Demo, Serialize, Deserialize, ToSchema)]
#[schema(example = json!(SseMessage::demo_direct_msg(SseData::demo())))]
pub enum SseMessage {
    Msg(String),
    GlobalMsg(SseData),
    WardMsg(SseData),
    SpcltyMsg(SseData),
    DirectMsg(SseData),
    Logout(String),
}

impl SseMessage {
    /// GET `EndPoint::SseMessage`
    pub async fn call_api_get(params: &SseMessageParams, app: Rc<AppState>) -> Result<Vec<Self>, AppError> {
        match fetch_json_api(&[EndPoint::SseMessage.base(), params.query_string()].concat(), "GET", None, app).await {
            Ok((response, true)) => {
                let response: Vec<Self> = serde_wasm_bindgen::from_value(response).map_err(|e| Source::SerdeWasm.to_teapot_error(e, "Fetch SseMessage"))?;
                Ok(response)
            }
            Ok((app_error, false)) => {
                let error: AppError = serde_wasm_bindgen::from_value(app_error).map_err(|e| Source::SerdeWasm.to_teapot_error(e, "Fetch SseMessage"))?;
                Err(error)
            }
            Err(e) => Err(Source::Js.to_teapot_error(e.dyn_ref::<JsString>().map(|s| s.into()).unwrap_or(String::from("fetch error")), "Fetch Json")),
        }
    }

    /// PATCH `EndPoint::SseMessage`
    pub async fn call_api_patch(message_ids: &[u32], app: Rc<AppState>) -> Result<ExecuteResponse, AppError> {
        let body_json = serde_json::to_string(message_ids).map_err(|e| Source::SerdeWasm.to_teapot_error(e, "PATCH SseMessage"))?;
        let body = serde_wasm_bindgen::to_value(&body_json).map_err(|e| Source::SerdeWasm.to_teapot_error(e, "PATCH SseMessage"))?;
        execute_fetch(&EndPoint::SseMessage.base(), "PATCH", Some(&body), app).await
    }
}

/// SseMessage's inner Data, as in database
#[derive(Clone, Debug, Demo, Serialize, Deserialize, Eq, FromRow, ToSchema)]
#[schema(example = json!(SseData::demo()))]
pub struct SseData {
    #[Demo(value = "1")]
    pub message_id: u32,
    #[Demo(value = "Some(datetime!(2023-12-31 23:59:59))")]
    pub message_datetime: Option<PrimitiveDateTime>,
    #[Demo(value = r#"Some(String::from("Message"))"#)]
    pub message: Option<String>,
    #[Demo(value = r#"String::from("007")"#)]
    pub sender_code: String,
    #[Demo(value = r#"Some(String::from("Mr.Sender"))"#)]
    pub sender_name: Option<String>,
    #[Demo(value = r#"Some(String::from("009"))"#)]
    pub person: Option<String>,
    #[Demo(value = r#"Some(String::from("01"))"#)]
    pub ward: Option<String>,
    #[Demo(value = r#"Some(1)"#)]
    pub spclty_id: Option<u32>,
    #[Demo(value = r##"Some(String::from("#/info"))"##)]
    pub route: Option<String>,
    #[Demo(
        value = r##"Some(String::from("{\"message_id\": 1,\"message_datetime\": \"2023-12-31 11:11:11\",\"message\": \"Previous\",\"sender_code\": \"009\",\"sender_name\": \"Mr.Previous\",\"person\": \"007\",\"ward\": \"01\",\"spclty_id\": 1,\"route\": \"#/info\",\"reference\": null,\"readed\": 0}"))"##
    )]
    pub reference: Option<String>,
    #[Demo(value = "true")]
    pub readed: bool,
}

impl SseData {
    pub fn from_post_message(message: &SsePostMessage, message_id: u32, message_datetime: PrimitiveDateTime, sender_code: &Option<String>, sender_name: &str, readed: bool) -> Self {
        Self {
            message_id,
            message_datetime: Some(message_datetime),
            message: str_some(message.message.to_owned()),
            sender_code: sender_code.clone().unwrap_or_default(),
            sender_name: str_some(sender_name.to_owned()),
            person: message.person.to_owned(),
            ward: message.ward.to_owned(),
            spclty_id: message.spclty_id,
            route: message.route.as_ref().map(|r| r.string()),
            reference: message.reference.as_ref().and_then(|data| serde_json::to_string(data).ok()),
            readed,
        }
    }
}

impl PartialEq for SseData {
    fn eq(&self, other: &Self) -> bool {
        self.message_id == other.message_id
    }
}

impl Ord for SseData {
    fn cmp(&self, other: &Self) -> Ordering {
        self.message_id.cmp(&other.message_id)
    }
}

impl PartialOrd for SseData {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Clone, Default, PartialEq)]
pub enum SseMenuTab {
    #[default]
    Global,
    Ward,
    Spclty,
    Private,
    Compose,
    Config,
}

impl SseMenuTab {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Global => "ประกาศ",
            Self::Ward => "หอผู้ป่วย",
            Self::Spclty => "แผนก",
            Self::Private => "ส่วนตัว",
            Self::Compose => "เขียน",
            Self::Config => "ตั้งค่า",
        }
    }
}

/// SSE group sending to server
#[derive(Clone, Debug, Demo, Serialize, Deserialize, ToSchema)]
#[schema(example = json!(SseGroup::demo()))]
pub struct SseGroup {
    #[Demo(value = r#"vec![String::from("01")]"#)]
    pub wards: Vec<String>,
    #[Demo(value = r#"vec![1]"#)]
    pub spclty_ids: Vec<u32>,
}

impl SseGroup {
    /// POST `EndPoint::SseGroup`
    pub async fn call_api_post(group: &SseGroup, app: Rc<AppState>) -> Result<String, AppError> {
        let body_json = serde_json::to_string(group).map_err(|e| Source::SerdeWasm.to_teapot_error(e, "POST SseGroup"))?;
        let body = serde_wasm_bindgen::to_value(&body_json).map_err(|e| Source::SerdeWasm.to_teapot_error(e, "POST SseGroup"))?;
        execute_fetch_text(&EndPoint::SseGroup.base(), "POST", Some(&body), app).await
    }
}

#[derive(Clone, Default, Deserialize, IntoParams)]
pub struct SseMessageParams {
    pub cat: Option<String>,
    pub min_id: Option<u32>,
}

impl QueryString for SseMessageParams {
    fn from_tuples(params: &[(String, String)]) -> Option<Self> {
        (!params.is_empty()).then(|| Self {
            cat: find_qs(params, "cat"),
            min_id: find_qs(params, "min_id").and_then(|s| s.parse::<u32>().ok()),
        })
    }

    fn query_string(&self) -> String {
        let mut queries = Vec::with_capacity(2);

        if let Some(cat) = &self.cat {
            queries.push(["cat=", cat].concat());
        }
        if let Some(min_id) = &self.min_id {
            queries.push(["min_id=", &min_id.to_string()].concat());
        }

        (!queries.is_empty()).then(|| ["?", &queries.join("&")].concat()).unwrap_or_default()
    }
}
