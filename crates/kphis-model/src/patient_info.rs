use derive_demo::Demo;
use js_sys::JsString;
use serde::{Deserialize, Serialize};
use sqlx::types::{
    Decimal,
    time::{Date, PrimitiveDateTime, Time},
};
use std::rc::Rc;
use time::macros::{date, datetime, time};
use utoipa::ToSchema;
use wasm_bindgen::JsCast;

use kphis_util::{
    error::{AppError, Source},
    util::str_some,
};

use crate::{
    DEFAULT_USER_IMAGE, PATH_PREFIX_PATIENT_IMAGE,
    app::{AppState, VisitTypeId},
    endpoint::EndPoint,
    fetch::{ExecuteResponse, execute_fetch, fetch_json_api},
};

/// Patient Information
#[derive(Clone, Demo, Default, Deserialize, Serialize, ToSchema)]
#[schema(example = json!(PatientInfo::demo()))]
pub struct PatientInfo {
    #[Demo(value = r#"VisitTypeId::demo_ipd(String::from("660001234"))"#)]
    pub visit_type: VisitTypeId,
    /// for OPD-ER only
    #[Demo(value = "Some(1)")]
    pub opd_er_order_master_id: Option<u32>,
    /// for Pre-Admit only
    #[Demo(value = "Some(1)")]
    pub pre_admit_master_id: Option<u32>,
    // #[Demo(value = "Some(1)")]
    // pub admission_note_id: Option<u32>,
    #[Demo(value = r#"Some(String::from("0001234"))"#)]
    pub hn: Option<String>,
    #[Demo(value = r#"Some(String::from("660001234"))"#)]
    pub an: Option<String>,
    #[Demo(value = r#"Some(String::from("661231235959"))"#)]
    pub vn: Option<String>,

    #[Demo(value = r#"Some(String::from("1111111111111"))"#)]
    pub cid: Option<String>,
    #[Demo(value = r#"Some(String::from("ABC123"))"#)]
    pub passport_no: Option<String>,

    #[Demo(value = r#"Some(String::from("Mr."))"#)]
    pub pname: Option<String>,
    #[Demo(value = r#"Some(String::from("Patient"))"#)]
    pub fname: Option<String>,
    #[Demo(value = r#"Some(String::from("Sicker"))"#)]
    pub lname: Option<String>,
    #[Demo(value = "Some(date!(1993-12-31))")]
    pub birthday: Option<Date>,
    #[Demo(value = r#"Some(String::from("1"))"#)]
    pub sex: Option<String>,
    /// admit weight in grams
    #[Demo(value = "Some(50000)")]
    pub bw: Option<i32>,

    #[Demo(value = "Some(33)")]
    pub age_y: Option<i64>,
    #[Demo(value = "Some(3)")]
    pub age_m: Option<i64>,
    #[Demo(value = "Some(3)")]
    pub age_d: Option<i64>,

    #[Demo(value = r#"Some(String::from("88"))"#)]
    pub pttype: Option<String>,
    #[Demo(value = "Some(8888.8)")]
    pub income: Option<f64>,

    #[Demo(value = r#"Some(String::from("88 moo 8"))"#)]
    pub homeaddr: Option<String>,
    #[Demo(value = r#"Some(String::from("888-888-8888"))"#)]
    pub hometel: Option<String>,
    #[Demo(value = r#"Some(String::from("888-888-8888"))"#)]
    pub worktel: Option<String>,
    #[Demo(value = r#"Some(String::from("88 moo 8"))"#)]
    pub workaddr: Option<String>,
    #[Demo(value = r#"Some(String::from("888-888-8888"))"#)]
    pub informtel: Option<String>,
    #[Demo(value = r#"Some(String::from("88 moo 8"))"#)]
    pub informaddr: Option<String>,
    #[Demo(value = r#"Some(String::from("Mr.Inform"))"#)]
    pub informname: Option<String>,
    #[Demo(value = r#"Some(String::from("Friend"))"#)]
    pub informrelation: Option<String>,

    #[Demo(value = "Some(date!(2023-12-31))")]
    pub vstdate: Option<Date>,
    #[Demo(value = "Some(time!(23:59:59))")]
    pub vsttime: Option<Time>,

    #[Demo(value = r#"Some(String::from("PENICILLIN=Rash"))"#)]
    pub drugallergy: Option<String>, // agent=symptom, ..
    #[Demo(value = r#"Some(String::from("PENICILLIN=Rash"))"#)]
    pub er_drugallergy_history: Option<String>, // agent=symptom, ..

    // IPD ONLY (START)
    #[Demo(value = r#"Some(String::from("PENICILLIN^Rash"))"#)]
    pub allergy_drug_history: Option<String>,
    #[Demo(value = r#"Some(String::from("PENICILLIN=Rash"))"#)]
    pub allergy_drug_history_hosxp: Option<String>,
    #[Demo(value = r#"Some(String::from("user"))"#)]
    pub allergy_drug_pharmacy_check_person: Option<String>,
    #[Demo(value = "Some(datetime!(2023-12-31 23:59:59))")]
    pub allergy_drug_pharmacy_check_datetime: Option<PrimitiveDateTime>,

    #[Demo(value = "Some(date!(2023-12-31))")]
    pub regdate: Option<Date>,
    #[Demo(value = "Some(time!(23:59:59))")]
    pub regtime: Option<Time>,
    #[Demo(value = "Some(1)")]
    pub admdate: Option<i32>,
    #[Demo(value = "Some(1)")]
    pub leave_home_day: Option<i32>,
    #[Demo(value = "Some(date!(2023-12-31))")]
    pub dchdate: Option<Date>,
    #[Demo(value = "Some(time!(23:59:59))")]
    pub dchtime: Option<Time>,
    #[Demo(value = r#"Some(String::from("01"))"#)]
    pub dchstts: Option<String>,
    #[Demo(value = r#"Some(String::from("01"))"#)]
    pub dchtype: Option<String>,
    #[Demo(value = r#"Some(String::from("01"))"#)]
    pub ward: Option<String>,
    #[Demo(value = r#"Some(String::from("01"))"#)]
    pub spclty: Option<String>,
    #[Demo(value = r#"Some(String::from("C01"))"#)]
    pub bedno: Option<String>,
    #[Demo(value = "Some(3333)")]
    pub birth_weight: Option<i32>,

    #[Demo(value = r#"Some(String::from("Sick"))"#)]
    pub chief_complaints: Option<String>,
    #[Demo(value = "Some(1)")]
    pub g: Option<i32>,
    #[Demo(value = r#"Some(String::from("1"))"#)]
    pub p: Option<String>,
    #[Demo(value = "Some(1)")]
    pub last_child: Option<i32>,
    #[Demo(value = "Some(date!(2022-11-11))")]
    pub lmp: Option<Date>,
    #[Demo(value = "Some(date!(2023-11-11))")]
    pub edc: Option<Date>,
    #[Demo(value = r#"Some(String::from("40"))"#)]
    pub gestational_age: Option<String>,
    #[Demo(value = r#"Some(String::from("1"))"#)]
    pub gestational_day: Option<String>,

    #[Demo(value = r#"Some(String::from("Complete Recovery"))"#)]
    pub dchstts_name: Option<String>,
    #[Demo(value = r#"Some(String::from("With Approval"))"#)]
    pub dchtype_name: Option<String>,
    #[Demo(value = r#"Some(String::from("ตึกชาย"))"#)]
    pub ward_name: Option<String>,
    #[Demo(value = r#"Some(String::from("อายุรกรรม"))"#)]
    pub spclty_name: Option<String>,
    // IPD ONLY (END)
    #[Demo(value = r#"Some(String::from("ชาย"))"#)]
    pub sex_name: Option<String>,
    #[Demo(value = r#"Some(String::from("บุคคลในครอบครัว อสม."))"#)]
    pub pttype_name: Option<String>,
    #[Demo(value = r#"Some(String::from("นักข่าว"))"#)]
    pub occupation_name: Option<String>,
    #[Demo(value = r#"Some(String::from("พุทธ"))"#)]
    pub religion_name: Option<String>,
    #[Demo(value = r#"Some(String::from("ไทย"))"#)]
    pub citizenship_name: Option<String>,
    #[Demo(value = r#"Some(String::from("ไทย"))"#)]
    pub nationality_name: Option<String>,
    #[Demo(value = r#"Some(String::from("โสด"))"#)]
    pub marrystatus_name: Option<String>,

    #[Demo(value = "Some(170)")]
    pub latest_height: Option<i32>,
    #[Demo(value = "Some(Decimal::new(50,0))")]
    pub latest_bw: Option<Decimal>, // DECIMAL(7,3) UNSIGNED
    #[Demo(value = "Some(datetime!(2023-12-31 23:59:59))")]
    pub latest_bw_datetime: Option<PrimitiveDateTime>,
    #[Demo(value = "Some(datetime!(2023-12-31 23:59:59))")]
    pub latest_vs_datetime: Option<PrimitiveDateTime>,
    #[Demo(value = "Some(5)")]
    pub mem_ruptured_hours: Option<u16>,
}

impl PatientInfo {
    pub fn is_ipd(&self) -> bool {
        self.visit_type.is_ipd()
    }
    pub fn is_admited(&self) -> bool {
        self.visit_type.is_admited()
    }
    pub fn visit_type(&self) -> VisitTypeId {
        self.visit_type.clone()
    }
    pub fn hn(&self) -> Option<String> {
        self.hn.clone().and_then(str_some)
    }
    pub fn vn(&self) -> Option<String> {
        self.vn.clone().and_then(str_some)
    }
    pub fn regdate(&self) -> Option<Date> {
        if self.is_ipd() { self.regdate } else { self.vstdate }
    }
    pub fn regtime(&self) -> Option<Time> {
        if self.is_ipd() { self.regtime } else { self.vsttime }
    }
    pub fn lastdate(&self) -> Option<Date> {
        if self.is_ipd() { self.dchdate } else { self.latest_vs_datetime.map(|dt| dt.date()) }
    }
    pub fn birthday(&self) -> Option<Date> {
        self.birthday
    }
    pub fn ward(&self) -> Option<String> {
        self.ward.clone()
    }
    pub fn sex(&self) -> Option<String> {
        self.sex.clone()
    }
    pub fn image(&self) -> String {
        if let Some(hn) = self.hn() {
            [PATH_PREFIX_PATIENT_IMAGE, &hn].concat()
        } else {
            DEFAULT_USER_IMAGE.to_owned()
        }
    }

    /// GET `EndPoint::IpdShowPatientMainAn`
    pub async fn call_api_get_an(an: &str, app: Rc<AppState>) -> Result<Option<Self>, AppError> {
        Self::get(&[&EndPoint::IpdShowPatientMainAn.base(), an].concat(), app).await
    }

    /// GET `EndPoint::OpdErShowPatientMainId`
    pub async fn call_api_get_id(opd_er_order_master_id: u32, app: Rc<AppState>) -> Result<Option<Self>, AppError> {
        Self::get(&[EndPoint::OpdErShowPatientMainId.base(), opd_er_order_master_id.to_string()].concat(), app).await
    }

    /// GET `EndPoint::OpdErShowPatientMainVn`
    pub async fn call_api_get_vn(vn: &str, app: Rc<AppState>) -> Result<Option<Self>, AppError> {
        Self::get(&[&EndPoint::OpdErShowPatientMainVn.base(), vn].concat(), app).await
    }

    async fn get(path: &str, app: Rc<AppState>) -> Result<Option<Self>, AppError> {
        match fetch_json_api(path, "GET", None, app).await {
            Ok((response, true)) => {
                let response: Option<Self> = serde_wasm_bindgen::from_value(response).map_err(|e| Source::SerdeWasm.to_teapot_error(e, "Fetch ShowPatientMain"))?;
                Ok(response)
            }
            Ok((app_error, false)) => {
                let error: AppError = serde_wasm_bindgen::from_value(app_error).map_err(|e| Source::SerdeWasm.to_teapot_error(e, "Fetch ShowPatientMain"))?;
                Err(error)
            }
            Err(e) => Err(Source::Js.to_teapot_error(e.dyn_ref::<JsString>().map(|s| s.into()).unwrap_or(String::from("fetch error")), "Fetch Json")),
        }
    }

    /// PATCH `EndPoint::IpdAdmissionNoteDrPharmCheckAn`
    pub async fn call_api_patch(an: &str, app: Rc<AppState>) -> Result<ExecuteResponse, AppError> {
        execute_fetch(&[&EndPoint::IpdAdmissionNoteDrPharmCheckAn.base(), an].concat(), "PATCH", None, app).await
    }
}
