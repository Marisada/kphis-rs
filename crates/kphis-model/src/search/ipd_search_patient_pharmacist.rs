use derive_demo::Demo;
use js_sys::JsString;
use serde_derive::{Deserialize, Serialize};
use sqlx::{
    FromRow,
    types::time::{Date, PrimitiveDateTime, Time},
};
use std::rc::Rc;
use time::macros::{date, datetime, time};
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
pub struct IpdSearchPatientPharmacistRequest {
    pub ward: Option<String>,
    pub doctor_in_charge: Option<String>,
    pub drug_allergy_check: Option<String>,
    pub patient: Option<String>,
}

impl QueryString for IpdSearchPatientPharmacistRequest {
    fn from_tuples(params: &[(String, String)]) -> Option<Self> {
        (!params.is_empty()).then(|| Self {
            ward: find_qs(params, "ward"),
            doctor_in_charge: find_qs(params, "doctor_in_charge"),
            drug_allergy_check: find_qs(params, "drug_allergy_check"),
            patient: find_qs(params, "patient"),
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
        if let Some(drug_allergy_check) = &self.drug_allergy_check {
            queries.push(["drug_allergy_check=", drug_allergy_check].concat());
        }
        if let Some(patient) = &self.patient {
            queries.push(["patient=", patient].concat());
        }

        (!queries.is_empty()).then(|| ["?", &queries.join("&")].concat()).unwrap_or_default()
    }
}

impl IpdSearchPatientPharmacistRequest {
    pub fn not_empty(self) -> Self {
        Self {
            ward: self.ward.and_then(str_some),
            doctor_in_charge: self.doctor_in_charge.and_then(str_some),
            drug_allergy_check: self.drug_allergy_check.and_then(str_some),
            patient: self.patient.and_then(str_some),
        }
    }
}

/// IPD Search Patient result for Pharmacist
#[derive(Demo, Deserialize, Serialize, FromRow, ToSchema)]
#[schema(example = json!(IpdSearchPatientPharmacistResponse::demo()))]
pub struct IpdSearchPatientPharmacistResponse {
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
    #[Demo(value = "Some(1)")]
    pub admission_note_id: Option<u32>,
    #[Demo(value = r#"Some(String::from("PENICILLIN=Rash"))"#)]
    pub allergy_drug_history: Option<String>,
    #[Demo(value = r#"Some(String::from("PENICILLIN=Rash"))"#)]
    pub allergy_drug_history_hosxp: Option<String>,
    #[Demo(value = r#"Some(String::from("009"))"#)]
    pub allergy_drug_pharmacy_check_person: Option<String>,
    #[Demo(value = "Some(datetime!(2023-12-31 23:59:59))")]
    pub allergy_drug_pharmacy_check_datetime: Option<PrimitiveDateTime>,
    /// ยังไม่มีบันทึกแรกรับ, รอประเมิน, ประเมินแล้ว
    #[Demo(value = r#"Some(String::from("ยังไม่มีบันทึกแรกรับ"))"#)]
    pub drug_allergy_check_status: Option<String>,
    #[Demo(value = r#"Some(String::from("Mr.Pharmacist"))"#)]
    pub allergy_drug_pharmacy_check_person_name: Option<String>,
    #[Demo(value = "true")]
    pub mr_unconfirmed_exists: bool,
    #[Demo(value = "true")]
    pub mr_confirmed_exists: bool,
    #[Demo(value = "true")]
    pub need_monitor: bool,
}

impl IpdSearchPatientPharmacistResponse {
    /// GET `EndPoint::SearchPharmacist`
    pub async fn call_api_get(request: &IpdSearchPatientPharmacistRequest, app: Rc<AppState>) -> Result<Vec<Self>, AppError> {
        match fetch_json_api(&[EndPoint::SearchPharmacist.base(), request.clone().query_string()].concat(), "GET", None, app).await {
            Ok((response, true)) => {
                let response: Vec<Self> = serde_wasm_bindgen::from_value(response).map_err(|e| Source::SerdeWasm.to_teapot_error(e, "Fetch IpdSearchPatientPharmacist"))?;
                Ok(response)
            }
            Ok((app_error, false)) => {
                let error: AppError = serde_wasm_bindgen::from_value(app_error).map_err(|e| Source::SerdeWasm.to_teapot_error(e, "Fetch IpdSearchPatientPharmacist"))?;
                Err(error)
            }
            Err(e) => Err(Source::Js.to_teapot_error(e.dyn_ref::<JsString>().map(|s| s.into()).unwrap_or(String::from("fetch error")), "Fetch Json")),
        }
    }
}
