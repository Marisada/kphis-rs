use derive_demo::Demo;
use js_sys::JsString;
use serde::{Deserialize, Serialize};
use sqlx::{
    FromRow,
    types::{
        Decimal,
        time::{Date, PrimitiveDateTime, Time},
    },
};
use std::rc::Rc;
use time::macros::{date, datetime, time};
use utoipa::{IntoParams, ToSchema};
use wasm_bindgen::JsCast;

use kphis_util::{
    datetime::{date_th, datetime_th},
    error::{AppError, Source},
    util::sanity_dot_space,
};

use crate::{
    app::{AppState, VisitTypeId},
    endpoint::{EndPoint, QueryString, find_qs},
    fetch::{ExecuteResponse, execute_fetch, execute_fetch_vec, execute_fetch_vec_with_u32, fetch_json_api},
};

/// Medical Reconciliation Header
#[derive(Clone, Demo, Deserialize, Serialize, ToSchema)]
#[schema(example = json!(MedReconciliationHeader::demo()))]
pub struct MedReconciliationHeader {
    #[Demo(value = r#"VisitTypeId::demo_ipd(String::from("660001234"))"#)]
    pub visit_type: VisitTypeId,
    #[Demo(value = "Some(datetime!(2023-12-31 23:59:59))")]
    pub visit_datetime: Option<PrimitiveDateTime>,
    #[Demo(value = r#"Some(String::from("0001234"))"#)]
    pub hn: Option<String>,
}

impl MedReconciliationHeader {
    /// GET `EndPoint::MedReconcileHn`
    pub async fn call_api_get(hn: &str, app: Rc<AppState>) -> Result<Vec<Self>, AppError> {
        match fetch_json_api(&[&EndPoint::MedReconcileHn.base(), hn].concat(), "GET", None, app).await {
            Ok((response, true)) => {
                let response: Vec<Self> = serde_wasm_bindgen::from_value(response).map_err(|e| Source::SerdeWasm.to_teapot_error(e, "Fetch MedReconcileHead"))?;
                Ok(response)
            }
            Ok((app_error, false)) => {
                let error: AppError = serde_wasm_bindgen::from_value(app_error).map_err(|e| Source::SerdeWasm.to_teapot_error(e, "Fetch MedReconcileHead"))?;
                Err(error)
            }
            Err(e) => Err(Source::Js.to_teapot_error(e.dyn_ref::<JsString>().map(|s| s.into()).unwrap_or(String::from("fetch error")), "Fetch Json")),
        }
    }
}

#[derive(Clone, Default, Deserialize, IntoParams)]
pub struct MedReconciliationParams {
    pub hn: Option<String>,
    pub vn: Option<String>,
    pub an: Option<String>,
    pub opd_er_order_master_id: Option<u32>,
    pub med_reconciliation_id: Option<u32>,
    pub med_reconciliation_item_id: Option<u32>,
    pub used: Option<String>,
    pub patch: Option<String>, // for PATCH api [doctor,pharm,unconfirm,last]
}

impl QueryString for MedReconciliationParams {
    fn from_tuples(params: &[(String, String)]) -> Option<Self> {
        (!params.is_empty()).then(|| Self {
            hn: find_qs(params, "hn"),
            vn: find_qs(params, "vn"),
            an: find_qs(params, "an"),
            opd_er_order_master_id: find_qs(params, "opd_er_order_master_id").and_then(|s| s.parse::<u32>().ok()),
            med_reconciliation_id: find_qs(params, "med_reconciliation_id").and_then(|s| s.parse::<u32>().ok()),
            med_reconciliation_item_id: find_qs(params, "med_reconciliation_item_id").and_then(|s| s.parse::<u32>().ok()),
            used: find_qs(params, "used"),
            patch: find_qs(params, "patch"),
        })
    }

    fn query_string(&self) -> String {
        let mut queries = Vec::with_capacity(8);
        if let Some(hn) = &self.hn {
            queries.push(["hn=", hn].concat());
        }
        if let Some(vn) = &self.vn {
            queries.push(["vn=", vn].concat());
        }
        if let Some(an) = &self.an {
            queries.push(["an=", an].concat());
        }
        if let Some(opd_er_order_master_id) = &self.opd_er_order_master_id {
            queries.push(["opd_er_order_master_id=", &opd_er_order_master_id.to_string()].concat());
        }
        if let Some(med_reconciliation_id) = &self.med_reconciliation_id {
            queries.push(["med_reconciliation_id=", &med_reconciliation_id.to_string()].concat());
        }
        if let Some(med_reconciliation_item_id) = &self.med_reconciliation_item_id {
            queries.push(["med_reconciliation_item_id=", &med_reconciliation_item_id.to_string()].concat());
        }
        if let Some(used) = &self.used {
            queries.push(["used=", used].concat());
        }
        if let Some(patch) = &self.patch {
            queries.push(["patch=", patch].concat());
        }

        (!queries.is_empty()).then(|| ["?", &queries.join("&")].concat()).unwrap_or_default()
    }
}

/// IPD Medical Reconciliation with Items
#[derive(Clone, Demo, Deserialize, Serialize, ToSchema)]
#[schema(example = json!(MedReconciliation::demo()))]
pub struct MedReconciliation {
    /// for generic over `an` or `opd_er_order_master_id` only
    #[Demo(value = r#"VisitTypeId::demo_ipd(String::from("660001234"))"#)]
    pub visit_type: VisitTypeId,
    #[Demo(value = "1")]
    pub med_reconciliation_id: u32,
    #[Demo(value = r#"Some(String::from("009"))"#)]
    pub pharmacist: Option<String>,
    #[Demo(value = r#"Some(String::from("Note"))"#)]
    pub note: Option<String>,
    #[Demo(value = r#"Some(String::from("007"))"#)]
    pub doctor: Option<String>,
    #[Demo(value = "Some(datetime!(2023-12-31 23:59:59))")]
    pub med_reconciliation_datetime: Option<PrimitiveDateTime>,
    #[Demo(value = "Some(datetime!(2023-12-31 23:59:59))")]
    pub phamacist_confirm_datetime: Option<PrimitiveDateTime>,
    #[Demo(value = "Some(datetime!(2023-12-31 23:59:59))")]
    pub doctor_confirm_datetime: Option<PrimitiveDateTime>,
    #[Demo(value = r#"Some(String::from("Mr.Pharmacist"))"#)]
    pub pharmacist_name: Option<String>,
    #[Demo(value = r#"Some(String::from("Dr.Doctor"))"#)]
    pub doctor_name: Option<String>,
    #[Demo(value = r#"String::from("Y")"#)]
    pub is_pharmacist_current_user_doctor: String,

    #[Demo(value = "vec![MedReconciliationItem::demo()]")]
    pub med_reconciliation_items: Vec<MedReconciliationItem>,
}

impl MedReconciliation {
    /// GET `EndPoint::IpdMedReconcile`<br>
    /// GET `EndPoint::OpdErMedReconcile`
    pub async fn call_api_get(is_ipd: bool, params: &MedReconciliationParams, app: Rc<AppState>) -> Result<Vec<Self>, AppError> {
        let ep = if is_ipd { EndPoint::IpdMedReconcile } else { EndPoint::OpdErMedReconcile };
        match fetch_json_api(&[ep.base(), params.query_string()].concat(), "GET", None, app).await {
            Ok((response, true)) => {
                let response: Vec<Self> = serde_wasm_bindgen::from_value(response).map_err(|e| Source::SerdeWasm.to_teapot_error(e, "Fetch MedReconciliation"))?;
                Ok(response)
            }
            Ok((app_error, false)) => {
                let error: AppError = serde_wasm_bindgen::from_value(app_error).map_err(|e| Source::SerdeWasm.to_teapot_error(e, "Fetch MedReconciliation"))?;
                Err(error)
            }
            Err(e) => Err(Source::Js.to_teapot_error(e.dyn_ref::<JsString>().map(|s| s.into()).unwrap_or(String::from("fetch error")), "Fetch Json")),
        }
    }

    /// - POST `EndPoint::IpdMedReconcile`
    /// - POST `EndPoint::OpdErMedReconcile`
    pub async fn call_api_post(is_ipd: bool, items: &[MedReconciliationItemSave], params: &MedReconciliationParams, app: Rc<AppState>) -> Result<(u32, Vec<ExecuteResponse>), AppError> {
        let ep = if is_ipd { EndPoint::IpdMedReconcile } else { EndPoint::OpdErMedReconcile };
        let body_json = serde_json::to_string(&items).unwrap();
        let body = serde_wasm_bindgen::to_value(&body_json).unwrap();
        execute_fetch_vec_with_u32(&[ep.base(), params.query_string()].concat(), "POST", Some(&body), app).await
    }

    /// - PATCH `EndPoint::IpdMedReconcile`
    /// - PATCH `EndPoint::OpdErMedReconcile`
    pub async fn call_api_patch(is_ipd: bool, items: &[MedReconciliationItemPatch], params: &MedReconciliationParams, app: Rc<AppState>) -> Result<Vec<ExecuteResponse>, AppError> {
        let ep = if is_ipd { EndPoint::IpdMedReconcile } else { EndPoint::OpdErMedReconcile };
        let body_json = serde_json::to_string(&items).unwrap();
        let body = serde_wasm_bindgen::to_value(&body_json).unwrap();
        execute_fetch_vec(&[ep.base(), params.query_string()].concat(), "PATCH", Some(&body), app).await
    }

    /// - DELETE `EndPoint::IpdMedReconcile`
    /// - DELETE `EndPoint::OpdErMedReconcile`
    pub async fn call_api_delete(is_ipd: bool, params: &MedReconciliationParams, app: Rc<AppState>) -> Result<ExecuteResponse, AppError> {
        let ep = if is_ipd { EndPoint::IpdMedReconcile } else { EndPoint::OpdErMedReconcile };
        execute_fetch(&[ep.base(), params.query_string()].concat(), "DELETE", None, app).await
    }
}

/// Items of IPD Medical Reconciliation
#[derive(Clone, Demo, Deserialize, Serialize, ToSchema)]
#[schema(example = json!(MedReconciliationItem::demo()))]
pub struct MedReconciliationItem {
    /// for generic over `an` or `opd_er_order_master_id` only
    #[Demo(value = r#"VisitTypeId::demo_ipd(String::from("660001234"))"#)]
    pub visit_type: VisitTypeId,
    #[Demo(value = "1")]
    pub med_reconciliation_item_id: u32,
    #[Demo(value = "Some(1)")]
    pub med_reconciliation_id: Option<u32>,
    #[Demo(value = r#"Some(String::from("1000222"))"#)]
    pub icode: Option<String>,
    /// In-Hospital drug name
    #[Demo(value = r#"Some(String::from("PARACETAMOL 500 mg. เม็ด"))"#)]
    pub med_name: Option<String>,
    /// Out-Hospital drug name
    #[Demo(value = r#"Some(String::from("Paracetamol"))"#)]
    pub custom_med_name: Option<String>,
    #[Demo(value = r#"Some(String::from("Store"))"#)]
    pub receive_from: Option<String>,
    #[Demo(value = "Some(date!(2023-12-31))")]
    pub receive_date: Option<Date>,
    /// Drug usage of previous prescription
    #[Demo(value = r#"Some(String::from("1 prn"))"#)]
    pub old_drugusage: Option<String>,
    /// Drug usage of this prescription
    #[Demo(value = r#"Some(String::from("2 prn"))"#)]
    pub changed_drugusage: Option<String>,
    #[Demo(value = "Some(10)")]
    pub receive_qty: Option<i32>,
    #[Demo(value = "Some(datetime!(2023-12-31 23:59:59))")]
    pub last_dose_taken_time: Option<PrimitiveDateTime>,
    #[Demo(value = r#"Some(String::from("Remark"))"#)]
    pub last_dose_taken_remark: Option<String>,
    #[Demo(value = r#"Some(String::from("Y"))"#)]
    pub used: Option<String>, // use
    #[Demo(value = r#"Some(String::from("PARACETAMOL"))"#)]
    pub allergy_agent: Option<String>,
    #[Demo(value = r#"Some(String::from("PARACETAMOL=Rash"))"#)]
    pub allergy_agent_symptom: Option<String>,
    #[Demo(value = "Decimal::new(1,0)")]
    pub allergy_count_force_no_order: Decimal,
    // /// Default drug usage of this drug
    // #[Demo(value = r#"Some(String::from("รับประทานครั้งละ 1 เม็ด เวลามีอาการ"))"#)]
    // pub usage: Option<String>,
    #[Demo(value = r#"Some(String::from("PARACETAMOL"))"#)]
    pub generic_name: Option<String>,
    #[Demo(value = r#"Some(String::from("TABLET"))"#)]
    pub dosageform: Option<String>,
    #[Demo(value = r#"Some(String::from("Y"))"#)]
    pub show_notify: Option<String>,
    #[Demo(value = r#"Some(String::from("***ALERT!!!***"))"#)]
    pub show_notify_text: Option<String>,
    #[Demo(value = r#"Some(String::from("CrCl < 30 dose xx"))"#)]
    pub due_usage: Option<String>,
    #[Demo(value = r#"Some(String::from("Y"))"#)]
    pub due_status: Option<String>,

    #[Demo(value = r#"Some(String::from("Use me gently"))"#)]
    pub info: Option<String>,
    #[Demo(value = r#"Some(String::from("Y"))"#)]
    pub info_status: Option<String>,
}

impl MedReconciliationItem {
    pub fn med_rec_info(&self) -> String {
        let (old_usage, now_title) = if self.is_med_rec_change_usage() {
            (self.old_drugusage.as_ref().map(|s| ["วิธีใช้เดิม: ", s, "\n"].concat()).unwrap_or_default(), "วิธีใช้ใหม่: ")
        } else {
            (String::new(), "วิธีใช้: ")
        };
        [
            old_usage,
            self.changed_drugusage.as_ref().map(|s| [now_title, s].concat()).unwrap_or_default(),
            self.receive_from.as_ref().map(|s| ["\nได้รับจาก: ", s].concat()).unwrap_or_default(),
            self.receive_qty.map(|i| ["\nปริมาณ: ", &i.to_string()].concat()).unwrap_or_default(),
            self.receive_date.map(|d| ["\nเมื่อวันที่: ", &date_th(&d)].concat()).unwrap_or_default(),
            self.last_dose_taken_time.map(|dt| ["\nรับประทานครั้งล่าสุด: ", &datetime_th(&dt)].concat()).unwrap_or_default(),
            self.last_dose_taken_remark.as_ref().map(|s| ["\nหมายเหตุ: ", s].concat()).unwrap_or_default(),
        ]
        .concat()
    }

    pub fn is_med_rec_change_usage(&self) -> bool {
        if let (Some(old), Some(new)) = (&self.old_drugusage, &self.changed_drugusage) {
            sanity_dot_space(old) != sanity_dot_space(new)
        } else {
            false
        }
    }
}

/// IPD Medical Reconciliation for create
#[derive(Debug, Demo, Deserialize, Serialize, ToSchema)]
#[schema(example = json!(MedReconciliationItemSave::demo()))]
pub struct MedReconciliationItemSave {
    #[Demo(value = r#"Some(String::from("1000222"))"#)]
    pub icode: Option<String>,
    #[Demo(value = r#"Some(String::from("PARACETAMOL 500 mg. เม็ด"))"#)]
    pub med_name: Option<String>,
    #[Demo(value = r#"Some(String::from("Paracetamol"))"#)]
    pub custom_med_name: Option<String>,
    #[Demo(value = r#"Some(String::from("Store"))"#)]
    pub receive_from: Option<String>,
    #[Demo(value = "Some(date!(2023-12-31))")]
    pub receive_date: Option<Date>,
    #[Demo(value = r#"Some(String::from("1 prn"))"#)]
    pub old_drugusage: Option<String>,
    #[Demo(value = "Some(10)")]
    pub receive_qty: Option<i32>,
}

/// IPD Medical Reconciliation for edit
#[derive(Debug, Demo, Deserialize, Serialize, ToSchema)]
#[schema(example = json!(MedReconciliationItemPatch::demo()))]
pub struct MedReconciliationItemPatch {
    #[Demo(value = "1")]
    pub med_reconciliation_item_id: u32,

    #[Demo(value = r#"Some(String::from("Store"))"#)]
    pub receive_from: Option<String>,
    #[Demo(value = "Some(date!(2023-12-31))")]
    pub receive_date: Option<Date>,
    #[Demo(value = r#"Some(String::from("1 prn"))"#)]
    pub old_drugusage: Option<String>,
    #[Demo(value = "Some(10)")]
    pub receive_qty: Option<i32>,

    #[Demo(value = r#"Some(String::from("2 prn"))"#)]
    pub changed_drugusage: Option<String>,
    #[Demo(value = "Some(datetime!(2023-12-31 23:59:59))")]
    pub last_dose_taken_time: Option<PrimitiveDateTime>,
    #[Demo(value = r#"Some(String::from("Remark"))"#)]
    pub last_dose_taken_remark: Option<String>,
    #[Demo(value = r#"Some(String::from("Y"))"#)]
    pub used: Option<String>,
}

/// IPD Medical Reconciliation from HIS
#[derive(Clone, Demo, Deserialize, Serialize, FromRow, ToSchema)]
#[schema(example = json!(MedReconciliationDetail::demo()))]
pub struct MedReconciliationDetail {
    #[Demo(value = "1")]
    medication_reconciliation_detail_id: i32,
    #[Demo(value = "Some(1)")]
    medication_reconciliation_id: Option<i32>,
    #[Demo(value = r#"Some(String::from("PARACETAMOL 500 mg. เม็ด"))"#)]
    pub medication_name: Option<String>,
    #[Demo(value = r#"Some(String::from("from XXX"))"#)]
    pub receive_location: Option<String>,
    #[Demo(value = "Some(date!(2023-12-31))")]
    pub last_receive_date: Option<Date>,
    #[Demo(value = "Some(1)")]
    doctor_reconciliation_command_id: Option<i32>,
    #[Demo(value = "Some(1)")]
    medication_reconciliation_manage_id: Option<i32>,
    #[Demo(value = r#"Some(String::from("Cause"))"#)]
    medication_change_cause: Option<String>,
    #[Demo(value = r#"Some(String::from("1 prn"))"#)]
    pub usage_name: Option<String>,
    #[Demo(default)]
    hos_guid: Option<String>,
    #[Demo(value = "Some(10)")]
    pub qty: Option<i32>,
    #[Demo(value = "Some(datetime!(2023-12-31 23:59:59))")]
    first_entry_date: Option<PrimitiveDateTime>,
}

impl MedReconciliationDetail {
    /// GET `EndPoint::IpdMedReconcileHosxpAn`
    pub async fn call_api_get_ipd(an: &str, app: Rc<AppState>) -> Result<Vec<Self>, AppError> {
        match fetch_json_api(&[&EndPoint::IpdMedReconcileHosxpAn.base(), an].concat(), "GET", None, app).await {
            Ok((response, true)) => {
                let response: Vec<Self> = serde_wasm_bindgen::from_value(response).map_err(|e| Source::SerdeWasm.to_teapot_error(e, "Fetch IndexPlan"))?;
                Ok(response)
            }
            Ok((app_error, false)) => {
                let error: AppError = serde_wasm_bindgen::from_value(app_error).map_err(|e| Source::SerdeWasm.to_teapot_error(e, "Fetch IndexPlan"))?;
                Err(error)
            }
            Err(e) => Err(Source::Js.to_teapot_error(e.dyn_ref::<JsString>().map(|s| s.into()).unwrap_or(String::from("fetch error")), "Fetch Json")),
        }
    }
}

/// IPD Medical Reconciliation Last-Dose
#[derive(Clone, Demo, Deserialize, Serialize, FromRow, ToSchema)]
#[schema(example = json!(AdmissionNoteLastDose::demo()))]
pub struct AdmissionNoteLastDose {
    #[Demo(value = "Some(datetime!(2023-12-31 23:59:59))")]
    pub last_dose_taken_time: Option<PrimitiveDateTime>,
    #[Demo(value = r#"Some(String::from("Remark"))"#)]
    pub last_dose_taken_remark: Option<String>,
}

impl AdmissionNoteLastDose {
    /// GET `EndPoint::IpdMedReconcileLastDoseAn`
    pub async fn call_api_get(an: &str, app: Rc<AppState>) -> Result<Option<Self>, AppError> {
        match fetch_json_api(&[&EndPoint::IpdMedReconcileLastDoseAn.base(), an].concat(), "GET", None, app).await {
            Ok((response, true)) => {
                let response: Option<Self> = serde_wasm_bindgen::from_value(response).map_err(|e| Source::SerdeWasm.to_teapot_error(e, "Fetch IndexPlan"))?;
                Ok(response)
            }
            Ok((app_error, false)) => {
                let error: AppError = serde_wasm_bindgen::from_value(app_error).map_err(|e| Source::SerdeWasm.to_teapot_error(e, "Fetch IndexPlan"))?;
                Err(error)
            }
            Err(e) => Err(Source::Js.to_teapot_error(e.dyn_ref::<JsString>().map(|s| s.into()).unwrap_or(String::from("fetch error")), "Fetch Json")),
        }
    }
}

/// IPD Medical Reconciliation Note with associated data
#[derive(Clone, Demo, Deserialize, Serialize, FromRow, ToSchema)]
#[schema(example = json!(MedReconciliationNote::demo()))]
pub struct MedReconciliationNote {
    #[Demo(value = "1")]
    med_reconciliation_id: u32,
    #[Demo(value = "Some(datetime!(2023-12-31 23:59:59))")]
    pub phamacist_confirm_datetime: Option<PrimitiveDateTime>,
    #[Demo(value = r#"Some(String::from("Note"))"#)]
    pub note: Option<String>,
}

impl MedReconciliationNote {
    /// GET `EndPoint::IpdMedReconcileNoteId`
    pub async fn call_api_get_ipd(med_reconciliation_id: u32, app: Rc<AppState>) -> Result<Option<Self>, AppError> {
        match fetch_json_api(&[EndPoint::IpdMedReconcileNoteId.base(), med_reconciliation_id.to_string()].concat(), "GET", None, app).await {
            Ok((response, true)) => {
                let response: Option<Self> = serde_wasm_bindgen::from_value(response).map_err(|e| Source::SerdeWasm.to_teapot_error(e, "Fetch MedReconcileNote"))?;
                Ok(response)
            }
            Ok((app_error, false)) => {
                let error: AppError = serde_wasm_bindgen::from_value(app_error).map_err(|e| Source::SerdeWasm.to_teapot_error(e, "Fetch MedReconcileNote"))?;
                Err(error)
            }
            Err(e) => Err(Source::Js.to_teapot_error(e.dyn_ref::<JsString>().map(|s| s.into()).unwrap_or(String::from("fetch error")), "Fetch Json")),
        }
    }

    /// GET `EndPoint::OpdErMedReconcileNoteId`
    pub async fn call_api_get_opd_er(med_reconciliation_id: u32, app: Rc<AppState>) -> Result<Option<Self>, AppError> {
        match fetch_json_api(&[EndPoint::OpdErMedReconcileNoteId.base(), med_reconciliation_id.to_string()].concat(), "GET", None, app).await {
            Ok((response, true)) => {
                let response: Option<Self> = serde_wasm_bindgen::from_value(response).map_err(|e| Source::SerdeWasm.to_teapot_error(e, "Fetch MedReconcileNote"))?;
                Ok(response)
            }
            Ok((app_error, false)) => {
                let error: AppError = serde_wasm_bindgen::from_value(app_error).map_err(|e| Source::SerdeWasm.to_teapot_error(e, "Fetch MedReconcileNote"))?;
                Err(error)
            }
            Err(e) => Err(Source::Js.to_teapot_error(e.dyn_ref::<JsString>().map(|s| s.into()).unwrap_or(String::from("fetch error")), "Fetch Json")),
        }
    }

    /// POST `EndPoint::IpdMedReconcileNoteId`
    pub async fn call_api_post_ipd(note: &String, med_reconciliation_id: u32, app: Rc<AppState>) -> Result<ExecuteResponse, AppError> {
        let body_json = serde_json::to_string(note).map_err(|e| Source::SerdeJson.to_teapot_error(e, "Send MedReconcile"))?;

        let body = serde_wasm_bindgen::to_value(&body_json).map_err(|e| Source::SerdeWasm.to_teapot_error(e, "Send MedReconcile"))?;

        execute_fetch(&[EndPoint::IpdMedReconcileNoteId.base(), med_reconciliation_id.to_string()].concat(), "POST", Some(&body), app).await
    }

    /// POST `EndPoint::OpdErMedReconcileNoteId`
    pub async fn call_api_post_opd_er(note: &String, med_reconciliation_id: u32, app: Rc<AppState>) -> Result<ExecuteResponse, AppError> {
        let body_json = serde_json::to_string(note).map_err(|e| Source::SerdeJson.to_teapot_error(e, "Send MedReconcile"))?;

        let body = serde_wasm_bindgen::to_value(&body_json).map_err(|e| Source::SerdeWasm.to_teapot_error(e, "Send MedReconcile"))?;

        execute_fetch(&[EndPoint::OpdErMedReconcileNoteId.base(), med_reconciliation_id.to_string()].concat(), "POST", Some(&body), app).await
    }
}

/// IPD Medical Reconciliation Visit for Remed
#[derive(Clone, Demo, Deserialize, Serialize, FromRow, ToSchema)]
#[schema(example = json!(ReMedVisit::demo()))]
pub struct ReMedVisit {
    #[Demo(value = r#"Some(String::from("661231235959"))"#)]
    pub vn: Option<String>,
    #[Demo(value = r#"Some(String::from("660001234"))"#)]
    pub an: Option<String>,
    #[Demo(value = "Some(date!(2023-12-31))")]
    pub vstdate: Option<Date>,
    #[Demo(value = "Some(time!(23:59:59))")]
    pub vsttime: Option<Time>,
    #[Demo(value = "true")]
    pub opd_item_exists: bool,
    #[Demo(value = "true")]
    pub ipd_home_med_item_exists: bool,
}

impl ReMedVisit {
    /// GET `EndPoint::IpdMedReconcileRemedVisitHn`
    pub async fn call_api_get(hn: &str, app: Rc<AppState>) -> Result<Vec<Self>, AppError> {
        match fetch_json_api(&[&EndPoint::IpdMedReconcileRemedVisitHn.base(), hn].concat(), "GET", None, app).await {
            Ok((response, true)) => {
                let response: Vec<Self> = serde_wasm_bindgen::from_value(response).map_err(|e| Source::SerdeWasm.to_teapot_error(e, "Fetch IndexPlan"))?;
                Ok(response)
            }
            Ok((app_error, false)) => {
                let error: AppError = serde_wasm_bindgen::from_value(app_error).map_err(|e| Source::SerdeWasm.to_teapot_error(e, "Fetch IndexPlan"))?;
                Err(error)
            }
            Err(e) => Err(Source::Js.to_teapot_error(e.dyn_ref::<JsString>().map(|s| s.into()).unwrap_or(String::from("fetch error")), "Fetch Json")),
        }
    }
}

/// IPD Medical Reconciliation Remed
#[derive(Clone, Demo, Deserialize, Serialize, FromRow, ToSchema)]
#[schema(example = json!(ReMedMedication::demo()))]
pub struct ReMedMedication {
    #[Demo(default)]
    pub hos_guid: String,
    #[Demo(value = r#"Some(String::from("1000222"))"#)]
    pub icode: Option<String>,
    #[Demo(value = "Some(1)")]
    item_no: Option<i8>,
    #[Demo(value = r#"Some(String::from("H"))"#)]
    item_type: Option<String>,
    #[Demo(value = r#"Some(String::from("PARACETAMOL 500 mg. เม็ด"))"#)]
    pub item_name: Option<String>,
    #[Demo(value = r#"Some(String::from("รับประทานครั้งละ 1 เม็ด เวลามีอาการ"))"#)]
    pub usage: Option<String>,
    #[Demo(value = "Some(10)")]
    pub qty: Option<i32>,
    #[Demo(value = r#"Some(String::from("1 prt"))"#)]
    shortlist: Option<String>,
    #[Demo(value = "Some(0)")]
    displaycolor: Option<i32>,
    #[Demo(value = r#"Some(String::from("รับประทานครั้งละ 1 เม็ด"))"#)]
    pub name1: Option<String>,
    #[Demo(value = r#"Some(String::from("ทุก 4-6 ชั่วโมง"))"#)]
    pub name2: Option<String>,
    #[Demo(value = r#"Some(String::from("เวลามีไข้"))"#)]
    pub name3: Option<String>,
    #[Demo(value = "Some(5.0)")]
    sum_price: Option<f64>,
    #[Demo(value = "Some(0.5)")]
    unitprice: Option<f64>,
    #[Demo(value = r#"Some(String::from("02"))"#)]
    paidst: Option<String>,
    #[Demo(value = r#"Some(String::from("0000016"))"#)]
    sp_use: Option<String>,
    #[Demo(value = "Some(date!(2023-12-31))")]
    pub rxdate: Option<Date>,
}

impl ReMedMedication {
    /// GET `EndPoint::IpdMedReconcileRemedMed`
    pub async fn call_api_get(params: &MedReconciliationParams, app: Rc<AppState>) -> Result<Vec<Self>, AppError> {
        match fetch_json_api(&[EndPoint::IpdMedReconcileRemedMed.base(), params.query_string()].concat(), "GET", None, app).await {
            Ok((response, true)) => {
                let response: Vec<Self> = serde_wasm_bindgen::from_value(response).map_err(|e| Source::SerdeWasm.to_teapot_error(e, "Fetch IndexPlan"))?;
                Ok(response)
            }
            Ok((app_error, false)) => {
                let error: AppError = serde_wasm_bindgen::from_value(app_error).map_err(|e| Source::SerdeWasm.to_teapot_error(e, "Fetch IndexPlan"))?;
                Err(error)
            }
            Err(e) => Err(Source::Js.to_teapot_error(e.dyn_ref::<JsString>().map(|s| s.into()).unwrap_or(String::from("fetch error")), "Fetch Json")),
        }
    }
}
