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

use kphis_util::error::{AppError, Source};

use crate::{
    app::AppState,
    endpoint::{EndPoint, QueryString, find_qs},
    fetch::{ExecuteResponse, execute_fetch, execute_fetch_vec, execute_fetch_vec_with_u32, fetch_json_api},
    order::{OrderItemSave, OrderTypeName},
};

#[derive(Clone, Default, Deserialize, IntoParams)]
pub struct PreOrderParams {
    pub pre_order_master_id: Option<u32>,
    pub order_id: Option<u32>,
    pub order_type: Option<String>,
    pub order_confirm: Option<String>,
    pub order_owner_type: Option<String>, // comma delimited
    pub view_by: Option<String>,
}

impl QueryString for PreOrderParams {
    fn from_tuples(params: &[(String, String)]) -> Option<Self> {
        (!params.is_empty()).then(|| Self {
            pre_order_master_id: find_qs(params, "pre_order_master_id").and_then(|s| s.parse::<u32>().ok()),
            order_id: find_qs(params, "order_id").and_then(|s| s.parse::<u32>().ok()),
            order_type: find_qs(params, "order_type"),
            order_confirm: find_qs(params, "order_confirm"),
            order_owner_type: find_qs(params, "order_owner_type"),
            view_by: find_qs(params, "view_by"),
        })
    }

    fn query_string(&self) -> String {
        let mut queries = Vec::with_capacity(6);
        if let Some(pre_order_master_id) = &self.pre_order_master_id {
            queries.push(["pre_order_master_id=", &pre_order_master_id.to_string()].concat());
        }
        if let Some(order_id) = &self.order_id {
            queries.push(["order_id=", &order_id.to_string()].concat());
        }
        if let Some(order_type) = &self.order_type {
            queries.push(["order_type=", order_type].concat());
        }
        if let Some(order_confirm) = &self.order_confirm {
            queries.push(["order_confirm=", order_confirm].concat());
        }
        if let Some(order_owner_type) = &self.order_owner_type {
            queries.push(["order_owner_type=", order_owner_type].concat());
        }
        if let Some(view_by) = &self.view_by {
            queries.push(["view_by=", view_by].concat());
        }
        (!queries.is_empty()).then(|| ["?", &queries.join("&")].concat()).unwrap_or_default()
    }
}

/// IPD Pre-Order with Items
#[derive(Clone, Demo, Deserialize, Serialize, FromRow, ToSchema)]
#[schema(example = json!(PreOrder::demo()))]
pub struct PreOrder {
    #[Demo(value = "1")]
    pub order_id: u32,
    #[Demo(value = "1")]
    pub pre_order_master_id: u32,
    #[Demo(value = "date!(2023-12-31)")]
    pub order_date: Date,
    #[Demo(value = "time!(23:59:59)")]
    pub order_time: Time,
    #[Demo(value = r#"String::from("007")"#)]
    pub order_doctor: String,
    #[Demo(value = r#"String::from("med")"#)]
    pub order_type: String,
    #[Demo(value = r#"String::from("doctor")"#)]
    pub order_owner_type: String,
    #[Demo(value = r#"String::from("Y")"#)]
    pub order_confirm: String,
    #[Demo(value = r#"Some(String::from("008"))"#)]
    pub nurse_accept: Option<String>,
    #[Demo(value = "Some(datetime!(2023-12-31 23:59:59))")]
    pub nurse_accept_time: Option<PrimitiveDateTime>,
    #[Demo(value = r#"Some(String::from("009"))"#)]
    pub pharmacist_accept: Option<String>,
    #[Demo(value = "Some(datetime!(2023-12-31 23:59:59))")]
    pub pharmacist_accept_time: Option<PrimitiveDateTime>,
    #[Demo(value = r#"Some(String::from("010"))"#)]
    pub pharmacist_check: Option<String>,
    #[Demo(value = "Some(datetime!(2023-12-31 23:59:59))")]
    pub pharmacist_check_time: Option<PrimitiveDateTime>,
    #[Demo(value = r#"Some(String::from("011"))"#)]
    pub pharmacist_done: Option<String>,
    #[Demo(value = "Some(datetime!(2023-12-31 23:59:59))")]
    pub pharmacist_done_time: Option<PrimitiveDateTime>,
    #[Demo(value = r#"Some(String::from("done"))"#)]
    pub pharmacist_order_status: Option<String>,

    #[Demo(value = r#"Some(String::from("Dr.Doctor"))"#)]
    pub order_doctor_name: Option<String>,
    #[Demo(value = "Some(true)")]
    pub order_doctor_is_intern: Option<bool>,
    #[Demo(value = r#"Some(String::from("ว00000"))"#)]
    pub doctor_licenseno: Option<String>,
    #[Demo(value = r#"Some(String::from("Miss.Nurse"))"#)]
    pub nurse_accept_name: Option<String>,
    #[Demo(value = r#"Some(String::from("Mr.Pharmacist"))"#)]
    pub pharmacist_accept_name: Option<String>,
    #[Demo(value = r#"Some(String::from("Mr.Checker"))"#)]
    pub pharmacist_check_name: Option<String>,
    #[Demo(value = r#"Some(String::from("Mr.Payer"))"#)]
    pub pharmacist_done_name: Option<String>,

    #[sqlx(skip)]
    #[Demo(value = "vec![PreOrderItemType::demo()]")]
    pub order_item_types: Vec<PreOrderItemType>,
}

impl PreOrder {
    /// GET `EndPoint::IpdPreOrderOrder`
    pub async fn call_api_get(params: &PreOrderParams, app: Rc<AppState>) -> Result<Vec<Self>, AppError> {
        match fetch_json_api(&[EndPoint::IpdPreOrderOrder.base(), params.clone().query_string()].concat(), "GET", None, app).await {
            Ok((response, true)) => {
                let response: Vec<Self> = serde_wasm_bindgen::from_value(response).map_err(|e| Source::SerdeWasm.to_teapot_error(e, "Fetch PreOrder"))?;
                Ok(response)
            }
            Ok((app_error, false)) => {
                let error: AppError = serde_wasm_bindgen::from_value(app_error).map_err(|e| Source::SerdeWasm.to_teapot_error(e, "Fetch PreOrder"))?;
                Err(error)
            }
            Err(e) => Err(Source::Js.to_teapot_error(e.dyn_ref::<JsString>().map(|s| s.into()).unwrap_or(String::from("fetch error")), "Fetch Json")),
        }
    }

    /// DELETE `EndPoint::IpdPreOrderOrderId`
    pub async fn call_api_delete(order_id: u32, app: Rc<AppState>) -> Result<ExecuteResponse, AppError> {
        execute_fetch(&[EndPoint::IpdPreOrderOrderId.base(), order_id.to_string()].concat(), "DELETE", None, app).await
    }
}

/// Type of IPD Pre-Order Item
#[derive(Clone, Demo, Deserialize, Serialize, ToSchema)]
#[schema(example = json!(PreOrderItemType::demo()))]
pub struct PreOrderItemType {
    #[Demo(value = "OrderTypeName::demo_med()")]
    pub order_item_type: OrderTypeName,
    #[Demo(value = "vec![PreOrderItem::demo()]")]
    pub order_items: Vec<PreOrderItem>,
}

/// Item of IPD Pre-Order
#[derive(Clone, Demo, Deserialize, Serialize, FromRow, ToSchema)]
#[schema(example = json!(PreOrderItem::demo()))]
pub struct PreOrderItem {
    #[Demo(value = "1")]
    pub order_item_id: u32,
    #[Demo(value = "1")]
    pub pre_order_master_id: u32,
    #[Demo(value = "Some(1)")]
    pub order_id: Option<u32>,
    #[Demo(value = r#"Some(String::from("med"))"#)]
    pub order_item_type: Option<String>,
    #[Demo(value = r#"Some(String::from("รับประทานครั้งละ 1 เม็ด เวลามีอาการ"))"#)]
    pub order_item_detail: Option<String>,
    #[Demo(value = r#"Some(String::from("Y"))"#)]
    pub stat: Option<String>,
    #[Demo(value = "Some(1)")]
    pub off_order_item_id: Option<u32>,
    #[Demo(value = r#"Some(String::from("1000222"))"#)]
    pub icode: Option<String>,

    #[Demo(value = r#"Some(String::from("รับประทานครั้งละ 1 เม็ด เวลามีอาการ"))"#)]
    pub off_order_item_detail: Option<String>,
    #[Demo(value = r#"Some(String::from("PARACETAMOL 500 mg. เม็ด"))"#)]
    pub med_name: Option<String>,
    #[Demo(value = "Some(0)")]
    pub displaycolor: Option<i32>,
    #[Demo(value = r#"Some(String::from("PARACETAMOL"))"#)]
    pub generic_name: Option<String>,
    #[Demo(value = r#"Some(String::from("TABLET"))"#)]
    pub dosageform: Option<String>,
    #[Demo(value = r#"Some(String::from("1000222"))"#)]
    pub off_icode: Option<String>,
    #[Demo(value = r#"Some(String::from("PARACETAMOL 500 mg. เม็ด"))"#)]
    pub off_med_name: Option<String>,
    #[Demo(value = "Some(0)")]
    pub off_displaycolor: Option<i32>,
    #[Demo(value = r#"Some(String::from("PENICILLIN=Rash"))"#)]
    pub allergy_agent_symptom: Option<String>,
}

/// Item of IPD Medical Pre-Order
#[derive(Debug, Demo, Deserialize, Serialize, FromRow, ToSchema)]
#[schema(example = json!(PreMedOrderItem::demo()))]
pub struct PreMedOrderItem {
    #[Demo(value = "Some(1)")]
    pub pre_order_master_id: Option<u32>,
    #[Demo(value = r#"Some(String::from("1000222"))"#)]
    pub icode: Option<String>,
    #[Demo(value = r#"Some(String::from("PARACETAMOL 500 mg. เม็ด"))"#)]
    pub med_name: Option<String>,
    #[Demo(value = r#"Some(String::from("PARACETAMOL"))"#)]
    pub generic_name: Option<String>,
    #[Demo(value = r#"Some(String::from("รับประทานครั้งละ 1 เม็ด เวลามีอาการ"))"#)]
    pub order_item_detail: Option<String>,
    #[Demo(value = "Some(1)")]
    pub off_by_order_id: Option<u32>,
    #[Demo(value = "Some(0)")]
    pub displaycolor: Option<i32>,
}

/// Command for copy/edit Pre-Order into Another Type
#[derive(Clone, Debug, Demo, Deserialize, Serialize, ToSchema)]
#[schema(example = json!(PreOrderIntoCommand::demo()))]
pub struct PreOrderIntoCommand {
    /// pre-order, template
    #[Demo(value = r#"Some(String::from("template"))"#)]
    pub from: Option<String>,
    /// order, pre-order, opd-er-order
    #[Demo(value = r#"Some(String::from("order"))"#)]
    pub into: Option<String>,
    /// id of from
    #[Demo(value = r#"Some(String::from("1"))"#)]
    pub from_id: Option<String>,
    /// id of into
    #[Demo(value = r#"Some(String::from("660001234"))"#)]
    pub into_id: Option<String>,
}

impl PreOrderIntoCommand {
    pub fn is_valid(&self) -> bool {
        self.from.is_some() && self.into.is_some() && self.from_id.is_some() && self.into_id.is_some()
    }

    /// POST `EndPoint::IpdPreOrderInto`
    pub async fn call_api_post(&self, app: Rc<AppState>) -> Result<Vec<ExecuteResponse>, AppError> {
        let body_json = serde_json::to_string(&self).map_err(|e| Source::SerdeWasm.to_teapot_error(e, "Send PreOrderIntoCommand"))?;
        let body = serde_wasm_bindgen::to_value(&body_json).map_err(|e| Source::SerdeWasm.to_teapot_error(e, "Send PreOrderIntoCommand"))?;
        execute_fetch_vec(&EndPoint::IpdPreOrderInto.base(), "POST", Some(&body), app).await
    }
}

/// Pre-Order for save
#[derive(Demo, Deserialize, Serialize, FromRow, ToSchema)]
#[schema(example = json!(PreOrderSave::demo()))]
pub struct PreOrderSave {
    #[Demo(value = "1")]
    pub order_id: u32,
    #[Demo(value = "1")]
    pub pre_order_master_id: u32,
    #[Demo(value = "date!(2023-12-31)")]
    pub order_date: Date,
    #[Demo(value = "time!(23:59:59)")]
    pub order_time: Time,
    #[Demo(value = r#"String::from("007")"#)]
    pub order_doctor: String,
    #[Demo(value = r#"String::from("oneday")"#)]
    pub order_type: String,
    #[Demo(value = r#"String::from("doctor")"#)]
    pub order_owner_type: String,
    #[Demo(value = r#"String::from("Y")"#)]
    pub order_confirm: String,
    #[sqlx(skip)]
    #[Demo(value = "vec![OrderItemSave::demo()]")]
    pub order_items: Vec<OrderItemSave>,
}

impl PreOrderSave {
    /// POST `EndPoint::IpdPreOrderOrder`
    pub async fn call_api_post(&self, app: Rc<AppState>) -> Result<(u32, Vec<ExecuteResponse>), AppError> {
        let body_json = serde_json::to_string(self).map_err(|e| Source::SerdeJson.to_teapot_error(e, "Send OrderSave"))?;

        let body = serde_wasm_bindgen::to_value(&body_json).map_err(|e| Source::SerdeWasm.to_teapot_error(e, "Send OrderSave"))?;

        execute_fetch_vec_with_u32(&EndPoint::IpdPreOrderOrder.base(), "POST", Some(&body), app).await
    }
}

// /// Item of Pre-Order for save
// #[derive(Clone, Demo, Deserialize, Serialize, FromRow, ToSchema)]
// #[schema(example = json!(PreOrderItemSave::demo()))]
// pub struct PreOrderItemSave {
//     #[Demo(value = "1")]
//     pub order_item_id: u32,
//     #[Demo(value = "1")]
//     pub pre_order_master_id: u32,
//     #[Demo(value = "Some(1)")]
//     pub order_id: Option<u32>,
//     #[Demo(value = r#"Some(String::from("med"))"#)]
//     pub order_item_type: Option<String>,
//     #[Demo(value = r#"Some(String::from("รับประทานครั้งละ 1 เม็ด เวลามีอาการ"))"#)]
//     pub order_item_detail: Option<String>,
//     #[Demo(value = r#"Some(String::from("Y"))"#)]
//     pub stat: Option<String>,
//     #[Demo(value = "Some(1)")]
//     pub off_order_item_id: Option<u32>,
//     #[Demo(value = r#"Some(String::from("1000222"))"#)]
//     pub icode: Option<String>,
// }
