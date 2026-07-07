use derive_demo::Demo;
use js_sys::JsString;
use serde_derive::{Deserialize, Serialize};
use sqlx::FromRow;
use std::rc::Rc;
use time::{
    Date, PrimitiveDateTime, Time,
    macros::{date, datetime, time},
};
use utoipa::{IntoParams, ToSchema};
use wasm_bindgen::JsCast;

use kphis_util::{
    error::{AppError, Source},
    util::str_some,
};

use crate::{
    app::AppState,
    endpoint::{EndPoint, QueryString, find_qs},
    fetch::fetch_json_api,
};

#[derive(Clone, Debug, Default, Deserialize, Serialize, IntoParams)]
pub struct IpdSearchPatientNurseRequest {
    pub ward: Option<String>,
    pub doctor_in_charge: Option<String>,
    pub patient: Option<String>,
    pub passcode: Option<String>,
}

impl QueryString for IpdSearchPatientNurseRequest {
    fn from_tuples(params: &[(String, String)]) -> Option<Self> {
        (!params.is_empty()).then(|| Self {
            ward: find_qs(params, "ward"),
            doctor_in_charge: find_qs(params, "doctor_in_charge"),
            patient: find_qs(params, "patient"),
            passcode: find_qs(params, "passcode"),
        })
    }

    fn query_string(&self) -> String {
        let mut queries = Vec::with_capacity(4);
        if let Some(ward) = &self.ward {
            queries.push(["ward=", ward].concat());
        }
        if let Some(doctor_in_charge) = &self.doctor_in_charge {
            queries.push(["doctor_in_charge=", doctor_in_charge].concat());
        }
        if let Some(patient) = &self.patient {
            queries.push(["patient=", patient].concat());
        }
        if let Some(passcode) = &self.passcode {
            queries.push(["passcode=", passcode].concat());
        }

        (!queries.is_empty()).then(|| ["?", &queries.join("&")].concat()).unwrap_or_default()
    }
}

impl IpdSearchPatientNurseRequest {
    pub fn not_empty(self) -> Self {
        Self {
            ward: self.ward.and_then(str_some),
            doctor_in_charge: self.doctor_in_charge.and_then(str_some),
            patient: self.patient.and_then(str_some),
            passcode: self.passcode.and_then(str_some),
        }
    }
}

/// IPD Search Patient result for Nurse
#[derive(Demo, Deserialize, Serialize, FromRow, ToSchema)]
#[schema(example = json!(IpdSearchPatientNurseResponse::demo()))]
pub struct IpdSearchPatientNurseResponse {
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
    #[Demo(value = "Some(date!(2023-12-31))")]
    pub regdate: Option<Date>,
    #[Demo(value = "Some(time!(23:59:59))")]
    pub regtime: Option<Time>,
    #[Demo(value = r#"Some(datetime!(2023-12-31 23:59:59))"#)]
    pub regdatetime: Option<PrimitiveDateTime>,
    #[Demo(value = r#"Some(String::from("Mr.Patient Sicker"))"#)]
    pub fullname: Option<String>,
    #[Demo(value = "Some(date!(1993-12-31))")]
    pub birthday: Option<Date>,
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
    #[Demo(value = r#"Some(String::from("ชาย"))"#)]
    pub sex_name: Option<String>,
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
    #[Demo(value = r#"Some(String::from("Dr.Doctor"))"#)]
    pub consult_reply_name: Option<String>,
    #[Demo(value = r#"Some(String::from("2023-12-31 23:59:59|36.0|102|20|110|N|N|1|1|1|92|||1|||5.6|4|5|6||"))"#)]
    pub ews_concat: Option<String>,
    #[Demo(value = r#"Some(String::from("1"))"#)]
    pub max_fcnote_patient_type: Option<String>,
    #[Demo(value = r#"Some(datetime!(2023-12-31 23:59:59))"#)]
    pub max_fcnote_datetime: Option<PrimitiveDateTime>,
    #[Demo(value = r#"Some(datetime!(2023-12-31 23:59:59))"#)]
    pub max_order_datetime: Option<PrimitiveDateTime>,
    /// ward passcode status
    #[Demo(value = "1")]
    pub wp_status: i8,
    #[Demo(value = "true")]
    pub nurse_not_accept_exists: bool,
    #[Demo(value = "true")]
    pub discharge_order_exists: bool,
    #[Demo(value = "true")]
    pub stat_order_nurse_not_accept_exists: bool,
    #[Demo(value = "true")]
    pub dr_admission_note_exists: bool,
    #[Demo(value = "true")]
    pub nurse_admission_note_exists: bool,
    #[Demo(value = "true")]
    pub lab_unreported_exists: bool,
    #[Demo(value = "true")]
    pub lab_unreaded_exists: bool,
    #[Demo(value = "true")]
    pub xray_unreaded_exists: bool,
    #[Demo(value = "true")]
    pub mr_unconfirmed_exists: bool,
    #[Demo(value = "true")]
    pub mr_confirmed_exists: bool,
    #[Demo(value = "true")]
    pub need_monitor: bool,
}

impl IpdSearchPatientNurseResponse {
    /// GET `EndPoint::SearchNurse`
    pub async fn call_api_get(request: &IpdSearchPatientNurseRequest, app: Rc<AppState>) -> Result<Vec<Self>, AppError> {
        match fetch_json_api(&[EndPoint::SearchNurse.base(), request.clone().query_string()].concat(), "GET", None, app).await {
            Ok((response, true)) => {
                let response: Vec<Self> = serde_wasm_bindgen::from_value(response).map_err(|e| Source::SerdeWasm.to_teapot_error(e, "Fetch IpdSearchPatientNurse"))?;
                Ok(response)
            }
            Ok((app_error, false)) => {
                let error: AppError = serde_wasm_bindgen::from_value(app_error).map_err(|e| Source::SerdeWasm.to_teapot_error(e, "Fetch IpdSearchPatientNurse"))?;
                Err(error)
            }
            Err(e) => Err(Source::Js.to_teapot_error(e.dyn_ref::<JsString>().map(|s| s.into()).unwrap_or(String::from("fetch error")), "Fetch Json")),
        }
    }
}
