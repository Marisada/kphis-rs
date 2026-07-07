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
    fetch::fetch_json_api,
};

/// OPD-ER Order for Pharmacy Monitoring
#[derive(Demo, Deserialize, Serialize, ToSchema)]
#[schema(example = json!(OpdErOrderPharmacyMonitor::demo()))]
pub struct OpdErOrderPharmacyMonitor {
    #[Demo(value = "vec![OpdErOrderPharmacy::demo()]")]
    pub orders: Vec<OpdErOrderPharmacy>,
    // #[Demo(value = r#"Some(String::from("2023-12-31 23:59:59"))"#)]
    // pub new_last_order_time: Option<String>,
    // #[Demo(value = "vec![OpdErOrderPharmacyMaster::demo()]")]
    // pub order_masters: Vec<OpdErOrderPharmacyMaster>,
}

impl OpdErOrderPharmacyMonitor {
    /// GET `EndPoint::OpdErOrderPharmacy`
    pub async fn call_api_get(params: &OpdErOrderPharmacyParams, app: Rc<AppState>) -> Result<Self, AppError> {
        match fetch_json_api(&[EndPoint::OpdErOrderPharmacy.base(), params.clone().query_string()].concat(), "GET", None, app).await {
            Ok((response, true)) => {
                let response: Self = serde_wasm_bindgen::from_value(response).map_err(|e| Source::SerdeWasm.to_teapot_error(e, "Fetch OrderPharmacyMonitor"))?;
                Ok(response)
            }
            Ok((app_error, false)) => {
                let error: AppError = serde_wasm_bindgen::from_value(app_error).map_err(|e| Source::SerdeWasm.to_teapot_error(e, "Fetch OrderPharmacyMonitor"))?;
                Err(error)
            }
            Err(e) => Err(Source::Js.to_teapot_error(e.dyn_ref::<JsString>().map(|s| s.into()).unwrap_or(String::from("fetch error")), "Fetch Json")),
        }
    }
}

#[derive(Clone, Default, Deserialize, Serialize, IntoParams)]
pub struct OpdErOrderPharmacyParams {
    pub patient: Option<String>,
    pub order_date_from: Option<Date>,
    pub order_date_to: Option<Date>,
    pub is_discharged: Option<String>,
}

impl QueryString for OpdErOrderPharmacyParams {
    fn from_tuples(params: &[(String, String)]) -> Option<Self> {
        (!params.is_empty()).then(|| Self {
            patient: find_qs(params, "patient"),
            order_date_from: find_qs(params, "order_date_from").and_then(|s| date_8601(&s)),
            order_date_to: find_qs(params, "order_date_to").and_then(|s| date_8601(&s)),
            is_discharged: find_qs(params, "is_discharged"),
        })
    }

    fn query_string(&self) -> String {
        let mut queries = Vec::with_capacity(4);
        if let Some(patient) = &self.patient {
            queries.push(["patient=", patient].concat());
        }
        if let Some(order_date_from) = &self.order_date_from {
            queries.push(["order_date_from=", &order_date_from.to_string()].concat());
        }
        if let Some(order_date_to) = &self.order_date_to {
            queries.push(["order_date_to=", &order_date_to.to_string()].concat());
        }
        if let Some(is_discharged) = &self.is_discharged {
            queries.push(["is_discharged=", is_discharged].concat());
        }
        (!queries.is_empty()).then(|| ["?", &queries.join("&")].concat()).unwrap_or_default()
    }
}

/// OPD-ER Patient data for Pharmaceutical use
#[derive(Clone, Demo, Deserialize, Serialize, FromRow, ToSchema)]
#[schema(example = json!(OpdErOrderPharmacy::demo()))]
pub struct OpdErOrderPharmacy {
    #[Demo(value = "1")]
    pub count_stat: i32,
    #[Demo(value = "1")]
    pub count_not_accept: i32,
    #[Demo(value = "1")]
    pub count_accept: i32,
    #[Demo(value = "1")]
    pub count_check: i32,
    #[Demo(value = "1")]
    pub count_done: i32,
    #[Demo(value = "1")]
    pub count_pharm_notify: i32,
    #[Demo(value = "1")]
    pub count_item_not_accept_stat: i32,
    #[Demo(value = "1")]
    pub count_item_not_accept_homemed: i32,
    #[Demo(value = "1")]
    pub count_item_accept_stat: i32,
    #[Demo(value = "1")]
    pub count_item_accept_homemed: i32,

    #[Demo(value = "1")]
    pub opd_er_order_master_id: u32,
    #[Demo(value = "Some(date!(2023-12-31))")]
    pub order_date: Option<Date>,
    #[Demo(value = "Some(time!(23:59:59))")]
    pub order_time: Option<Time>,
    #[Demo(value = r#"Some(datetime!(2023-12-31 23:59:59))"#)]
    pub order_date_time: Option<PrimitiveDateTime>,
    #[Demo(value = r#"Some(String::from("Note"))"#)]
    pub note: Option<String>,
    #[Demo(value = "Some(1)")]
    pub er_patient_status_id: Option<u32>,

    #[Demo(value = r#"Some(String::from("0001234"))"#)]
    pub hn: Option<String>,
    #[Demo(value = r#"Some(String::from("661231235959"))"#)]
    pub vn: Option<String>,
    #[Demo(value = r#"Some(String::from("1"))"#)]
    pub bedno: Option<String>,
    #[Demo(value = r#"Some(String::from("Mr.Patient Sicker"))"#)]
    pub fullname: Option<String>,
    #[Demo(value = "Some(33)")]
    pub age_y: Option<i8>,
    #[Demo(value = "Some(3)")]
    pub age_m: Option<i8>,
    #[Demo(value = "Some(3)")]
    pub age_d: Option<i8>,

    #[Demo(value = r#"Some(datetime!(2023-12-31 23:59:59))"#)]
    pub min_order_date_time: Option<PrimitiveDateTime>,
    #[Demo(value = r#"Some(datetime!(2023-12-31 23:59:59))"#)]
    pub max_order_date_time: Option<PrimitiveDateTime>,
    // #[Demo(value = r#"String::from("Y")"#)]
    // pub containing_med_order_item: String,
}

// /// OPD-ER Order Master for Pharmaceutical use
// #[derive(Clone, Demo, Deserialize, Serialize, FromRow, ToSchema)]
// #[schema(example = json!(OpdErOrderPharmacyMaster::demo()))]
// pub struct OpdErOrderPharmacyMaster {
//     #[Demo(value = "1")]
//     pub opd_er_order_master_id: u32,
//     #[Demo(value = "Some(date!(2023-12-31))")]
//     pub order_date: Option<Date>,
//     #[Demo(value = "Some(time!(23:59:59))")]
//     pub order_time: Option<Time>,
//     #[Demo(value = r#"Some(String::from("2023-12-31 23:59:59"))"#)]
//     pub order_date_time: Option<String>,
//     #[Demo(value = r#"Some(String::from("Note"))"#)]
//     pub note: Option<String>,

//     #[Demo(value = r#"Some(String::from("0001234"))"#)]
//     pub hn: Option<String>,
//     #[Demo(value = r#"Some(String::from("661231235959"))"#)]
//     pub vn: Option<String>,
//     #[Demo(value = r#"Some(String::from("1"))"#)]
//     pub bedno: Option<String>,
//     #[Demo(value = r#"Some(String::from("Mr.Patient Sicker"))"#)]
//     pub fullname: Option<String>,
//     #[Demo(value = "Some(33)")]
//     pub age_y: Option<i8>,
//     #[Demo(value = "Some(3)")]
//     pub age_m: Option<i8>,
//     #[Demo(value = "Some(3)")]
//     pub age_d: Option<i8>,

//     #[Demo(value = "Some(datetime!(2023-12-31 23:59:59))")]
//     pub max_vs_datetime: Option<PrimitiveDateTime>,
//     #[Demo(value = r#"Some(String::from("2023-12-31 23:59:59"))"#)]
//     pub max_fcnote_datetime: Option<String>,
//     #[Demo(value = r#"Some(String::from("2023-12-31 23:59:59"))"#)]
//     pub max_order_datetime: Option<String>,
// }
