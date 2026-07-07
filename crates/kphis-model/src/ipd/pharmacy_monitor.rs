use derive_demo::Demo;
use js_sys::JsString;
use serde::{Deserialize, Serialize};
use sqlx::{
    FromRow,
    types::time::{Date, PrimitiveDateTime},
};
use std::rc::Rc;
use time::macros::datetime;
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

/// IPD Order for Pharmacy Monitoring
#[derive(Demo, Deserialize, Serialize, ToSchema)]
#[schema(example = json!(IpdOrderPharmacyMonitor::demo()))]
pub struct IpdOrderPharmacyMonitor {
    #[Demo(value = "vec![IpdOrderPharmacy::demo()]")]
    pub orders: Vec<IpdOrderPharmacy>,
    // #[Demo(value = r#"Some(String::from("2023-12-31 23:59:59"))"#)]
    // pub new_last_order_time: Option<String>,
    // #[Demo(value = "vec![PharmacyIpt::demo()]")]
    // pub ipts: Vec<PharmacyIpt>,
    #[Demo(value = "vec![PharmacyIpt::demo()]")]
    pub admits: Vec<PharmacyIpt>,
}

impl IpdOrderPharmacyMonitor {
    /// GET `EndPoint::IpdOrderPharmacy`
    pub async fn call_api_get(params: &IpdOrderPharmacyParams, app: Rc<AppState>) -> Result<Self, AppError> {
        match fetch_json_api(&[EndPoint::IpdOrderPharmacy.base(), params.clone().query_string()].concat(), "GET", None, app).await {
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
pub struct IpdOrderPharmacyParams {
    pub wards: Option<String>, // comma delimited
    pub inverse_ward_select: Option<String>,
    pub doctor_in_charge: Option<String>,
    pub patient: Option<String>,
    pub order_date_from: Option<Date>,
    pub order_date_to: Option<Date>,
    pub is_discharged: Option<String>, // Y, N, ""
}

impl QueryString for IpdOrderPharmacyParams {
    fn from_tuples(params: &[(String, String)]) -> Option<Self> {
        (!params.is_empty()).then(|| Self {
            wards: find_qs(params, "wards"),
            inverse_ward_select: find_qs(params, "inverse_ward_select"),
            doctor_in_charge: find_qs(params, "doctor_in_charge"),
            patient: find_qs(params, "patient"),
            order_date_from: find_qs(params, "order_date_from").and_then(|s| date_8601(&s)),
            order_date_to: find_qs(params, "order_date_to").and_then(|s| date_8601(&s)),
            is_discharged: find_qs(params, "is_discharged"),
        })
    }

    fn query_string(&self) -> String {
        let mut queries = Vec::with_capacity(7);
        if let Some(wards) = &self.wards {
            queries.push(["wards=", wards].concat());
        }
        if let Some(inverse_ward_select) = &self.inverse_ward_select {
            queries.push(["inverse_ward_select=", inverse_ward_select].concat());
        }
        if let Some(doctor_in_charge) = &self.doctor_in_charge {
            queries.push(["doctor_in_charge=", doctor_in_charge].concat());
        }
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

/// IPD Patient data for Pharmaceutical use
#[derive(Clone, Demo, Deserialize, Serialize, FromRow, ToSchema)]
#[schema(example = json!(IpdOrderPharmacy::demo()))]
pub struct IpdOrderPharmacy {
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

    #[Demo(value = r#"Some(String::from("0001234"))"#)]
    pub hn: Option<String>,
    #[Demo(value = r#"String::from("660001234")"#)]
    pub an: String,
    #[Demo(value = r#"Some(String::from("อายุรกรรม - ตึกชาย"))"#)]
    pub sname: Option<String>,
    #[Demo(value = r#"Some(String::from("ตึกชาย"))"#)]
    pub ward_name: Option<String>,
    #[Demo(value = r#"Some(String::from("C01"))"#)]
    pub bedno: Option<String>,
    #[Demo(value = r#"Some(String::from("Mr.Patient Sicker"))"#)]
    pub pname: Option<String>,
    #[Demo(value = "Some(8888.8)")]
    pub income: Option<f64>,
    #[Demo(value = r#"Some(String::from("UC"))"#)]
    pub rtcode: Option<String>,
    #[Demo(value = r#"Some(String::from("บัตรประกันสุขภาพถ้วนหน้า ในเขต"))"#)]
    pub rtname: Option<String>,
    #[Demo(value = r#"Some(String::from("Dr.Doctor"))"#)]
    pub admdoctor_name: Option<String>,
    #[Demo(value = "Some(1)")]
    pub admdate: Option<i32>,
    #[Demo(value = "Some(33)")]
    pub age_y: Option<i8>,
    #[Demo(value = "Some(3)")]
    pub age_m: Option<i8>,
    #[Demo(value = "Some(3)")]
    pub age_d: Option<i8>,
    #[Demo(value = r#"Some(String::from("With Approval"))"#)]
    pub dchtype_name: Option<String>,
    #[Demo(value = r#"Some(String::from("Complete Recovery"))"#)]
    pub dchstts_name: Option<String>,

    #[Demo(value = r#"Some(datetime!(2023-12-31 23:59:59))"#)]
    pub min_order_date_time: Option<PrimitiveDateTime>,
    #[Demo(value = r#"Some(datetime!(2023-12-31 23:59:59))"#)]
    pub max_order_date_time: Option<PrimitiveDateTime>,
    // #[Demo(value = r#"String::from("Y")"#)]
    // pub containing_med_order_item: String,
}

/// IPD Patient data for Pharmaceutical use from HIS
#[derive(Clone, Demo, Deserialize, Serialize, FromRow, ToSchema)]
#[schema(example = json!(PharmacyIpt::demo()))]
pub struct PharmacyIpt {
    #[Demo(value = r#"Some(String::from("0001234"))"#)]
    pub hn: Option<String>,
    #[Demo(value = r#"String::from("660001234")"#)]
    pub an: String,
    #[Demo(value = r#"Some(String::from("อายุรกรรม - ตึกชาย"))"#)]
    pub sname: Option<String>,
    #[Demo(value = r#"Some(String::from("ตึกชาย"))"#)]
    pub ward_name: Option<String>,
    #[Demo(value = r#"Some(String::from("C01"))"#)]
    pub bedno: Option<String>,
    #[Demo(value = r#"Some(String::from("Mr.Patient Sicker"))"#)]
    pub pname: Option<String>,
    #[Demo(value = "Some(8888.8)")]
    pub income: Option<f64>,
    #[Demo(value = r#"Some(String::from("UC"))"#)]
    pub rtcode: Option<String>,
    #[Demo(value = r#"Some(String::from("บัตรประกันสุขภาพถ้วนหน้า ในเขต"))"#)]
    pub rtname: Option<String>,
    #[Demo(value = r#"Some(String::from("Dr.Doctor"))"#)]
    pub admdoctor_name: Option<String>,
    #[Demo(value = "Some(1)")]
    pub admdate: Option<i32>,
    #[Demo(value = "Some(33)")]
    pub age_y: Option<i8>,
    #[Demo(value = "Some(3)")]
    pub age_m: Option<i8>,
    #[Demo(value = "Some(3)")]
    pub age_d: Option<i8>,
    #[Demo(value = r#"Some(String::from("With Approval"))"#)]
    pub dchtype_name: Option<String>,
    #[Demo(value = r#"Some(String::from("Complete Recovery"))"#)]
    pub dchstts_name: Option<String>,
    #[Demo(value = r#"Some(String::from("Dr.Doctor"))"#)]
    pub incharge_doctor_name: Option<String>,
    #[Demo(value = r#"Some(String::from("Dr.Doctor"))"#)]
    pub kphis_incharge_doctor_name: Option<String>,

    #[Demo(value = "Some(datetime!(2023-12-31 23:59:59))")]
    pub max_vs_datetime: Option<PrimitiveDateTime>,
    #[Demo(value = r#"Some(datetime!(2023-12-31 23:59:59))"#)]
    pub max_fcnote_datetime: Option<PrimitiveDateTime>,
    #[Demo(value = r#"Some(datetime!(2023-12-31 23:59:59))"#)]
    pub max_order_datetime: Option<PrimitiveDateTime>,
}
