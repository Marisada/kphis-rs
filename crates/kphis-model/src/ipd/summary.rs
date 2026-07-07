use derive_demo::Demo;
use js_sys::JsString;
use serde::{Deserialize, Serialize};
use sqlx::{
    FromRow,
    types::time::{Date, Time},
};
use std::{rc::Rc, str::FromStr};
use strum::{AsRefStr, EnumIter, EnumString};
use time::{
    PrimitiveDateTime,
    macros::{date, datetime, time},
};
use utoipa::{IntoParams, ToSchema};
use wasm_bindgen::JsCast;

use kphis_util::error::{AppError, Source};

use crate::{
    app::AppState,
    endpoint::{EndPoint, QueryString, find_qs},
    fetch::{ExecuteResponse, execute_fetch, execute_fetch_vec_with_u32, fetch_json_api},
    ipd::his::HisOperationAdmit,
};

#[derive(Clone, Default, Deserialize, IntoParams)]
pub struct SummaryParams {
    pub summary_id: Option<u32>,
    pub an: Option<String>,
}

impl QueryString for SummaryParams {
    fn from_tuples(params: &[(String, String)]) -> Option<Self> {
        (!params.is_empty()).then(|| Self {
            summary_id: find_qs(params, "summary_id").and_then(|s| s.parse::<u32>().ok()),
            an: find_qs(params, "an"),
        })
    }

    fn query_string(&self) -> String {
        let mut queries = Vec::with_capacity(2);
        if let Some(summary_id) = &self.summary_id {
            queries.push(["summary_id=", &summary_id.to_string()].concat());
        }
        if let Some(an) = &self.an {
            queries.push(["an=", an].concat());
        }
        (!queries.is_empty()).then(|| ["?", &queries.join("&")].concat()).unwrap_or_default()
    }
}

/// IPD Summary with associated data
#[derive(Clone, Debug, Demo, Deserialize, Serialize, ToSchema)]
#[schema(example = json!(Summary::demo()))]
pub struct Summary {
    #[Demo(value = "Some(SummaryData::demo())")]
    pub summary: Option<SummaryData>,
    #[Demo(value = "vec![DxData::demo()]")]
    pub dx_data: Vec<DxData>, // ty 2-5
    #[Demo(value = "vec![DoctorData::demo()]")]
    pub doctor_data: Vec<DoctorData>, // ty 1,2
    #[Demo(value = "vec![XRayData::demo()]")]
    pub xray_data: Vec<XRayData>, // xray_items_group 3,4
    #[Demo(value = "Some(DchData::demo())")]
    pub dch_data: Option<DchData>,
    #[Demo(value = "vec![HisOperationAdmit::demo()]")]
    pub or_data: Vec<HisOperationAdmit>,
    #[Demo(value = "vec![LabAlertData::demo()]")]
    pub lab_alert_data: Vec<LabAlertData>,
    #[Demo(value = r#"vec![String::from("Problem")]"#)]
    pub problem_list_data: Vec<String>,
}

impl Summary {
    /// GET `EndPoint::IpdSummary`
    pub async fn call_api_get(params: &SummaryParams, app: Rc<AppState>) -> Result<Self, AppError> {
        match fetch_json_api(&[EndPoint::IpdSummary.base(), params.query_string()].concat(), "GET", None, app).await {
            Ok((response, true)) => {
                let response: Self = serde_wasm_bindgen::from_value(response).map_err(|e| Source::SerdeWasm.to_teapot_error(e, "Fetch Summary"))?;
                Ok(response)
            }
            Ok((app_error, false)) => {
                let error: AppError = serde_wasm_bindgen::from_value(app_error).map_err(|e| Source::SerdeWasm.to_teapot_error(e, "Fetch Summary"))?;
                Err(error)
            }
            Err(e) => Err(Source::Js.to_teapot_error(e.dyn_ref::<JsString>().map(|s| s.into()).unwrap_or(String::from("fetch error")), "Fetch Json")),
        }
    }
}

/// IPD Summary
#[derive(Clone, Debug, Demo, Deserialize, Serialize, FromRow, ToSchema)]
#[schema(example = json!(SummaryData::demo()))]
pub struct SummaryData {
    #[Demo(value = "1")]
    pub summary_id: u32,
    #[Demo(value = r#"String::from("660001234")"#)]
    pub an: String,
    #[Demo(value = r#"Some(String::from("Essential Hypertension"))"#)]
    pub principal_diagnosis: Option<String>,
    #[Demo(value = r#"Some(String::from("I10"))"#)]
    pub principal_diagnosis_icd10: Option<String>,
    #[Demo(value = r#"Some(String::from("Room No."))"#)]
    pub operating_room: Option<String>,
    #[Demo(value = r#"Some(String::from("Y"))"#)]
    // NON-OR PROCEDURE
    pub tracheostomy: Option<String>,
    #[Demo(value = r#"Some(String::from("Y"))"#)]
    pub mechanical_ventilation: Option<String>,
    #[Demo(value = r#"Some(String::from("Y"))"#)]
    pub packed_redcells: Option<String>,
    #[Demo(value = r#"Some(String::from("Y"))"#)]
    pub fresh_frozen_plasma: Option<String>,
    #[Demo(value = r#"Some(String::from("Y"))"#)]
    pub platelets: Option<String>,
    #[Demo(value = r#"Some(String::from("Y"))"#)]
    pub cryoprecipitate: Option<String>,
    #[Demo(value = r#"Some(String::from("Y"))"#)]
    pub whole_blood: Option<String>,
    #[Demo(value = r#"Some(String::from("Y"))"#)]
    pub chemotherapy: Option<String>,
    #[Demo(value = r#"Some(String::from("Y"))"#)]
    pub hemodialysis: Option<String>,
    #[Demo(value = r#"Some(String::from("Y"))"#)]
    pub non_or_other: Option<String>,
    #[Demo(value = r#"Some(String::from("Non-Operative Note"))"#)]
    pub non_or_other_text: Option<String>,
    // SPECIAL INVESTIGATION
    #[Demo(value = r#"Some(String::from("Y"))"#)]
    pub computer_tomography: Option<String>,
    #[Demo(value = r#"Some(String::from("CT Report"))"#)]
    pub computer_tomography_text: Option<String>,
    #[Demo(value = r#"Some(String::from("Y"))"#)]
    pub mri: Option<String>,
    #[Demo(value = r#"Some(String::from("MRI Report"))"#)]
    pub mri_text: Option<String>,
    #[Demo(value = r#"Some(String::from("Y"))"#)]
    pub special_other: Option<String>,
    #[Demo(value = r#"Some(String::from("Special-Investigation Note"))"#)]
    pub special_other_text: Option<String>,

    #[Demo(value = r#"Some(String::from("02"))"#)]
    pub discharge_status: Option<String>,
    #[Demo(value = r#"Some(String::from("04"))"#)]
    pub discharge_type: Option<String>,
    #[Demo(value = r#"Some(String::from("99999"))"#)]
    pub hospital_refer: Option<String>,
    // refer hospital
    #[Demo(value = r#"Some(String::from("โรงพยาบาลศูนย์"))"#)]
    pub hosptype: Option<String>, // read only
    #[Demo(value = r#"Some(String::from("Super Hospital"))"#)]
    pub hospname: Option<String>, // read only
    // audit
    #[Demo(value = r#"Some(String::from("Auditor"))"#)]
    pub coder_name: Option<String>,
    #[Demo(value = r#"Some(String::from("I10"))"#)]
    pub principal_diagnosis_code: Option<String>,
    #[Demo(value = r#"Some(String::from("E11.9, R57.1"))"#)]
    pub pre_admission_comorbidity_codes: Option<String>,
    #[Demo(value = r#"Some(String::from("A09.9, J00"))"#)]
    pub post_admission_comorbidity_codes: Option<String>,
    #[Demo(value = r#"Some(String::from("E87.6, E04.1"))"#)]
    pub other_diagnosis_codes: Option<String>,
    #[Demo(value = r#"Some(String::from("V32.02"))"#)]
    pub external_cause_codes: Option<String>,
    #[Demo(value = r#"Some(String::from("12.34"))"#)]
    pub main_procedure_code: Option<String>,
    #[Demo(value = r#"Some(String::from("23.45, 34.56"))"#)]
    pub other_procedure_codes: Option<String>,
    #[Demo(value = r#"Some(String::from("review"))"#)]
    pub status: Option<String>,
}

/// IPD Summary for save
#[derive(Clone, Debug, Demo, Deserialize, Serialize, ToSchema)]
#[schema(example = json!(SummarySave::demo()))]
pub struct SummarySave {
    #[Demo(value = "SummaryDataSave::demo()")]
    pub summary: SummaryDataSave,
    #[Demo(value = "vec![DxData::demo()]")]
    pub dx2_data: Vec<DxData>,
    #[Demo(value = "vec![DxData::demo()]")]
    pub dx3_data: Vec<DxData>,
    #[Demo(value = "vec![DxData::demo()]")]
    pub dx4_data: Vec<DxData>,
    #[Demo(value = "vec![DxData::demo()]")]
    pub dx5_data: Vec<DxData>,
    #[Demo(value = "true")]
    pub attending_doctor: bool,
    #[Demo(value = "true")]
    pub approve_doctor: bool,
}

impl SummarySave {
    /// POST `EndPoint::IpdSummary`
    pub async fn call_api_post(&self, app: Rc<AppState>) -> Result<(u32, Vec<ExecuteResponse>), AppError> {
        let body_json = serde_json::to_string(self).map_err(|e| Source::SerdeJson.to_teapot_error(e, "Send SummarySave"))?;

        let body = serde_wasm_bindgen::to_value(&body_json).map_err(|e| Source::SerdeWasm.to_teapot_error(e, "Send SummarySave"))?;

        execute_fetch_vec_with_u32(&EndPoint::IpdSummary.base(), "POST", Some(&body), app).await
    }

    pub fn is_summary_locked(&self) -> bool {
        self.summary.is_summary_locked()
    }
}

/// IPD Summary for saving summary data
#[derive(Clone, Debug, Demo, Deserialize, Serialize, FromRow, ToSchema)]
#[schema(example = json!(SummaryDataSave::demo()))]
pub struct SummaryDataSave {
    #[Demo(value = "1")]
    pub summary_id: u32,
    #[Demo(value = r#"String::from("660001234")"#)]
    pub an: String,
    #[Demo(value = r#"Some(String::from("Essential Hypertension"))"#)]
    pub principal_diagnosis: Option<String>,
    #[Demo(value = r#"Some(String::from("I10"))"#)]
    pub principal_diagnosis_icd10: Option<String>,
    #[Demo(value = r#"Some(String::from("Room No."))"#)]
    pub operating_room: Option<String>,
    #[Demo(value = r#"Some(String::from("Y"))"#)]
    pub tracheostomy: Option<String>,
    #[Demo(value = r#"Some(String::from("Y"))"#)]
    pub mechanical_ventilation: Option<String>,
    #[Demo(value = r#"Some(String::from("Y"))"#)]
    pub packed_redcells: Option<String>,
    #[Demo(value = r#"Some(String::from("Y"))"#)]
    pub fresh_frozen_plasma: Option<String>,
    #[Demo(value = r#"Some(String::from("Y"))"#)]
    pub platelets: Option<String>,
    #[Demo(value = r#"Some(String::from("Y"))"#)]
    pub cryoprecipitate: Option<String>,
    #[Demo(value = r#"Some(String::from("Y"))"#)]
    pub whole_blood: Option<String>,
    #[Demo(value = r#"Some(String::from("Y"))"#)]
    pub computer_tomography: Option<String>,
    #[Demo(value = r#"Some(String::from("CT Report"))"#)]
    pub computer_tomography_text: Option<String>,
    #[Demo(value = r#"Some(String::from("Y"))"#)]
    pub chemotherapy: Option<String>,
    #[Demo(value = r#"Some(String::from("Y"))"#)]
    pub mri: Option<String>,
    #[Demo(value = r#"Some(String::from("MRI Report"))"#)]
    pub mri_text: Option<String>,
    #[Demo(value = r#"Some(String::from("Y"))"#)]
    pub hemodialysis: Option<String>,
    #[Demo(value = r#"Some(String::from("Y"))"#)]
    pub non_or_other: Option<String>,
    #[Demo(value = r#"Some(String::from("Non-Operative Note"))"#)]
    pub non_or_other_text: Option<String>,
    #[Demo(value = r#"Some(String::from("Y"))"#)]
    pub special_other: Option<String>,
    #[Demo(value = r#"Some(String::from("Non-Operative Note"))"#)]
    pub special_other_text: Option<String>,
    #[Demo(value = r#"Some(String::from("02"))"#)]
    pub discharge_status: Option<String>,
    #[Demo(value = r#"Some(String::from("04"))"#)]
    pub discharge_type: Option<String>,
    #[Demo(value = r#"Some(String::from("99999"))"#)]
    pub hospital_refer: Option<String>,
    #[Demo(value = r#"Some(String::from("review"))"#)]
    pub status: Option<String>,
}

impl SummaryDataSave {
    pub fn is_summary_locked(&self) -> bool {
        self.status.as_ref().map(|s| AuditStatus::from_str(s).unwrap_or_default().is_summary_locked()).unwrap_or_default()
    }
}

/// IPD Summary for saving audit data
#[derive(Clone, Debug, Demo, Deserialize, Serialize, FromRow, ToSchema)]
#[schema(example = json!(SummaryCodeSave::demo()))]
pub struct SummaryCodeSave {
    #[Demo(value = "1")]
    pub summary_id: u32,
    #[Demo(value = r#"String::from("660001234")"#)]
    pub an: String,
    #[Demo(value = r#"Some(String::from("Auditor"))"#)]
    pub coder_name: Option<String>,
    #[Demo(value = r#"Some(String::from("I10"))"#)]
    pub principal_diagnosis_code: Option<String>,
    #[Demo(value = r#"Some(String::from("E11.9, R57.1"))"#)]
    pub pre_admission_comorbidity_codes: Option<String>,
    #[Demo(value = r#"Some(String::from("A09.9, J00"))"#)]
    pub post_admission_comorbidity_codes: Option<String>,
    #[Demo(value = r#"Some(String::from("E87.6, E04.1"))"#)]
    pub other_diagnosis_codes: Option<String>,
    #[Demo(value = r#"Some(String::from("V32.02"))"#)]
    pub external_cause_codes: Option<String>,
    #[Demo(value = r#"Some(String::from("12.34"))"#)]
    pub main_procedure_code: Option<String>,
    #[Demo(value = r#"Some(String::from("23.45, 34.56"))"#)]
    pub other_procedure_codes: Option<String>,
    #[Demo(value = r#"Some(String::from("review"))"#)]
    pub status: Option<String>,
}

impl SummaryCodeSave {
    /// PATCH `EndPoint::IpdSummary`
    pub async fn call_api_patch(&self, app: Rc<AppState>) -> Result<ExecuteResponse, AppError> {
        let body_json = serde_json::to_string(self).map_err(|e| Source::SerdeJson.to_teapot_error(e, "Send SummaryCodeSave"))?;

        let body = serde_wasm_bindgen::to_value(&body_json).map_err(|e| Source::SerdeWasm.to_teapot_error(e, "Send SummaryCodeSave"))?;

        execute_fetch(&EndPoint::IpdSummary.base(), "PATCH", Some(&body), app).await
    }
}

/// IPD Summary Status data
#[derive(Clone, Debug, Demo, Deserialize, Serialize, FromRow, ToSchema)]
#[schema(example = json!(SummaryStatus::demo()))]
pub struct SummaryStatus {
    #[Demo(value = r#"Some(String::from("review"))"#)]
    pub status: Option<String>,
}

impl SummaryStatus {
    /// GET `EndPoint::IpdSummaryStatusId`
    pub async fn call_api_get(summary_id: u32, app: Rc<AppState>) -> Result<Option<Self>, AppError> {
        match fetch_json_api(&[EndPoint::IpdSummaryStatusId.base(), summary_id.to_string()].concat(), "GET", None, app).await {
            Ok((response, true)) => {
                let response: Option<Self> = serde_wasm_bindgen::from_value(response).map_err(|e| Source::SerdeWasm.to_teapot_error(e, "Fetch SummaryStatus"))?;
                Ok(response)
            }
            Ok((app_error, false)) => {
                let error: AppError = serde_wasm_bindgen::from_value(app_error).map_err(|e| Source::SerdeWasm.to_teapot_error(e, "Fetch SummaryStatus"))?;
                Err(error)
            }
            Err(e) => Err(Source::Js.to_teapot_error(e.dyn_ref::<JsString>().map(|s| s.into()).unwrap_or(String::from("fetch error")), "Fetch Json")),
        }
    }

    /// PUT `EndPoint::IpdSummaryStatusId`
    pub async fn call_api_put(&self, summary_id: u32, app: Rc<AppState>) -> Result<ExecuteResponse, AppError> {
        let body_json = serde_json::to_string(self).map_err(|e| Source::SerdeJson.to_teapot_error(e, "Send SummaryStatus"))?;

        let body = serde_wasm_bindgen::to_value(&body_json).map_err(|e| Source::SerdeWasm.to_teapot_error(e, "Send SummaryStatus"))?;

        execute_fetch(&[EndPoint::IpdSummaryStatusId.base(), summary_id.to_string()].concat(), "PUT", Some(&body), app).await
    }
}

/// Diagnosis and ICD code
#[derive(Clone, Debug, Demo, Deserialize, Serialize, FromRow, ToSchema)]
#[schema(example = json!(DxData::demo()))]
pub struct DxData {
    /// 1=primary, 2=pre-admission, 3=post-admission, 4=other, 5=external, 9=procedure
    #[Demo(value = "2")]
    pub ty: i32,
    /// ICD10 uppercase without dot
    #[Demo(value = r#"Some(String::from("E11"))"#)]
    pub icd: Option<String>,
    #[Demo(value = r#"Some(String::from("Type 2 diabetes mellitus"))"#)]
    pub detail: Option<String>,
}

impl DxData {
    pub fn new(ty: i32, icd: Option<String>, detail: Option<String>) -> Rc<Self> {
        Rc::new(Self { ty, detail, icd })
    }
}

/// Doctor attending/approving Summary
#[derive(Clone, Debug, Demo, Deserialize, Serialize, FromRow, ToSchema)]
#[schema(example = json!(DoctorData::demo()))]
pub struct DoctorData {
    /// 1=attending, 2=approve
    #[Demo(value = "1")]
    pub ty: i32,
    #[Demo(value = r#"String::from("007")"#)]
    pub doctor: String,
    #[Demo(value = r#"Some(String::from("Dr.Doctor"))"#)]
    pub doctor_name: Option<String>,
    #[Demo(value = r#"Some(String::from("ว00000"))"#)]
    pub licenseno: Option<String>,
}

/// X-Ray data for Summary
#[derive(Clone, Debug, Demo, Deserialize, Serialize, FromRow, ToSchema)]
#[schema(example = json!(XRayData::demo()))]
pub struct XRayData {
    #[Demo(value = "Some(3)")]
    pub xray_items_group: Option<i32>,
    #[Demo(value = r#"Some(String::from("CT Scan"))"#)]
    pub xray_items_name: Option<String>,
    #[Demo(value = "Some(date!(2023-12-31))")]
    pub examined_date: Option<Date>,
    #[Demo(value = "Some(time!(23:59:59))")]
    pub examined_time: Option<Time>,
}

/// Discharge data for Summary
#[derive(Clone, Debug, Demo, Deserialize, Serialize, FromRow, ToSchema)]
#[schema(example = json!(DchData::demo()))]
pub struct DchData {
    #[Demo(value = r#"String::from("660001234")"#)]
    pub an: String,
    #[Demo(value = "Some(date!(2023-12-31))")]
    pub dchdate: Option<Date>,
    #[Demo(value = "Some(time!(23:59:59))")]
    pub dchtime: Option<Time>,
    #[Demo(value = r#"Some(String::from("1"))"#)]
    pub dchstts: Option<String>,
    #[Demo(value = r#"Some(String::from("Complete Recovery"))"#)]
    pub dchstts_name: Option<String>,
    #[Demo(value = r#"Some(String::from("1"))"#)]
    pub dchtype: Option<String>,
    #[Demo(value = r#"Some(String::from("With Approval"))"#)]
    pub dchtype_name: Option<String>,
}

/// Lab data for Summary
#[derive(Clone, Debug, Demo, Deserialize, Serialize, FromRow, ToSchema)]
#[schema(example = json!(LabAlertData::demo()))]
pub struct LabAlertData {
    #[Demo(value = r#"Some(String::from("0001234"))"#)]
    pub hn: Option<String>,
    #[Demo(value = "Some(80)")]
    pub lab_items_code: Option<i32>,
    #[Demo(value = "Some(5)")]
    pub display_order: Option<i32>,
    #[Demo(value = r#"Some(String::from("Na"))"#)]
    pub lab_items_name_ref: Option<String>,
    #[Demo(value = r#"Some(String::from("130-150"))"#)]
    pub lab_items_normal_value_ref: Option<String>,
    #[Demo(value = r#"Some(String::from("135.5"))"#)]
    pub lab_order_result: Option<String>,
    #[Demo(value = r#"Some(String::from("mmol/L"))"#)]
    pub lab_items_unit: Option<String>,
    #[Demo(value = "Some(130.0)")]
    pub range_check_min: Option<f64>,
    #[Demo(value = "Some(150.0)")]
    pub range_check_max: Option<f64>,
    #[Demo(value = r#"Some(String::from("Y"))"#)]
    pub staff_lock_result: Option<String>,
    #[Demo(value = r#"Some(String::from("Remark"))"#)]
    pub lab_order_remark: Option<String>,
    #[Demo(value = "Some(3)")]
    pub lab_items_group: Option<i32>,
    #[Demo(value = r#"Some(String::from("BIOCHEMISTRY"))"#)]
    pub lab_items_group_name: Option<String>,
    #[Demo(value = "1")]
    pub lab_order_number: i32,
    #[Demo(value = r#"Some(String::from("661231235959"))"#)]
    pub vn: Option<String>,
    #[Demo(value = "Some(date!(2023-12-31))")]
    pub order_date: Option<Date>,
    #[Demo(value = "Some(time!(23:59:59))")]
    pub order_time: Option<Time>,
    #[Demo(value = "Some(date!(2023-12-31))")]
    pub receive_date: Option<Date>,
    #[Demo(value = "Some(time!(23:59:59))")]
    pub receive_time: Option<Time>,
    #[Demo(value = "Some(date!(2023-12-31))")]
    pub report_date: Option<Date>,
    #[Demo(value = "Some(time!(23:59:59))")]
    pub report_time: Option<Time>,
}

#[derive(Clone, Default, PartialEq, EnumIter, EnumString, AsRefStr)]
#[strum(serialize_all = "snake_case")]
pub enum AuditStatus {
    #[default]
    Null,
    Approve,
    Review,
    Code,
    Audit,
    Claim,
    Appeal,
    Done,
}

impl AuditStatus {
    pub fn from_summary_status(summary_status: SummaryStatus) -> Self {
        match summary_status.status {
            Some(s) => match s.as_str() {
                "review" => Self::Review,
                "code" => Self::Code,
                "audit" => Self::Audit,
                "claim" => Self::Claim,
                "appeal" => Self::Appeal,
                "done" => Self::Done,
                _ => Self::Null,
            },
            None => Self::Null,
        }
    }

    pub fn as_data(&self) -> Option<String> {
        match self {
            Self::Null => None,
            Self::Approve => None,
            Self::Review => Some(String::from("review")),
            Self::Code => Some(String::from("code")),
            Self::Audit => Some(String::from("audit")),
            Self::Claim => Some(String::from("claim")),
            Self::Appeal => Some(String::from("appeal")),
            Self::Done => Some(String::from("done")),
        }
    }

    /// use with table `ipd_summary_2` AS `summary_2`
    pub fn sql_where_having(&self) -> (&'static str, &'static str) {
        match self {
            Self::Null => (" AND summary_2.status IS NULL", " AND NOT attending_doctor_exists"),
            Self::Approve => (" AND summary_2.status IS NULL", " AND attending_doctor_exists AND NOT approve_doctor_exists"),
            Self::Review => (" AND summary_2.status='review'", ""),
            // support original KPHIS
            Self::Code => (" AND (summary_2.status IS NULL OR summary_2.status='code')", " AND attending_doctor_exists AND approve_doctor_exists"),
            Self::Audit => (" AND summary_2.status='audit'", ""),
            Self::Claim => (" AND summary_2.status='claim'", ""),
            Self::Appeal => (" AND summary_2.status='appeal'", ""),
            Self::Done => (" AND summary_2.status='done'", ""),
        }
    }

    pub fn status_text(&self) -> &'static str {
        match self {
            Self::Null => "ยังไม่สรุป",
            Self::Approve => "รอ Approve",
            Self::Review => "รอ Review",
            Self::Code => "รอ Coder",
            Self::Audit => "รอ Audit",
            Self::Claim => "รอ Claim",
            Self::Appeal => "รอ อุทธรณ์",
            Self::Done => "สิ้นสุด",
        }
    }

    pub fn color_class(&self) -> &'static str {
        match self {
            Self::Null => "text-bg-secondary",
            Self::Approve => "text-bg-secondary",
            Self::Review => "text-bg-danger",
            Self::Code => "text-bg-info",
            Self::Audit => "text-bg-warning",
            Self::Claim => "text-bg-success",
            Self::Appeal => "text-bg-danger",
            Self::Done => "text-bg-primary",
        }
    }

    pub fn btn_class(&self) -> &'static str {
        match self {
            Self::Null => "btn-outline-secondary",
            Self::Approve => "btn-outline-secondary",
            Self::Review => "btn-outline-danger",
            Self::Code => "btn-outline-info",
            Self::Audit => "btn-outline-warning",
            Self::Claim => "btn-outline-success",
            Self::Appeal => "btn-outline-danger",
            Self::Done => "btn-outline-primary",
        }
    }

    pub fn is_summary_locked(&self) -> bool {
        matches!(self, Self::Claim | Self::Done)
    }

    pub fn is_list_with_date_limit(&self) -> bool {
        matches!(self, Self::Null | Self::Done)
    }
}

/// Summary Note for communication with doctor, coder and auditor
#[derive(Clone, Debug, Demo, Deserialize, Serialize, FromRow, ToSchema)]
#[schema(example = json!(SummaryNote::demo()))]
pub struct SummaryNote {
    #[Demo(value = "1")]
    pub summary_note_id: u32,
    #[Demo(value = "1")]
    pub summary_id: u32,
    #[Demo(value = r#"Some(String::from("0001234"))"#)]
    pub note: Option<String>,
    #[Demo(value = r#"Some(String::from("007"))"#)]
    pub doctor: Option<String>,
    #[Demo(value = r#"Some(String::from("Dr.Doctor"))"#)]
    pub doctor_name: Option<String>,
    #[Demo(value = "datetime!(2023-12-31 23:59:59)")]
    pub update_datetime: PrimitiveDateTime,
}

impl SummaryNote {
    /// GET `EndPoint::IpdSummaryNoteId`
    pub async fn call_api_get(summary_id: u32, app: Rc<AppState>) -> Result<Vec<Self>, AppError> {
        match fetch_json_api(&[EndPoint::IpdSummaryNoteId.base(), summary_id.to_string()].concat(), "GET", None, app).await {
            Ok((response, true)) => {
                let response: Vec<Self> = serde_wasm_bindgen::from_value(response).map_err(|e| Source::SerdeWasm.to_teapot_error(e, "Fetch SummaryNote"))?;
                Ok(response)
            }
            Ok((app_error, false)) => {
                let error: AppError = serde_wasm_bindgen::from_value(app_error).map_err(|e| Source::SerdeWasm.to_teapot_error(e, "Fetch SummaryNote"))?;
                Err(error)
            }
            Err(e) => Err(Source::Js.to_teapot_error(e.dyn_ref::<JsString>().map(|s| s.into()).unwrap_or(String::from("fetch error")), "Fetch Json")),
        }
    }
}

/// IPD Summary Note for save
#[derive(Clone, Default, Debug, Demo, Deserialize, Serialize, FromRow, ToSchema)]
#[schema(example = json!(SummaryNoteSave::demo()))]
pub struct SummaryNoteSave {
    /// PATCH and DELETE
    #[Demo(value = "Some(1)")]
    pub note_id: Option<u32>,
    /// POST, PATCH
    #[Demo(value = r#"String::from("Please do something..")"#)]
    pub note: String,
}

impl SummaryNoteSave {
    /// - POST `EndPoint::IpdSummaryNoteId`
    /// - PATCH `EndPoint::IpdSummaryNoteId`
    /// - DELETE `EndPoint::IpdSummaryNoteId`
    pub async fn call_api_save(&self, method: &str, summary_id: u32, app: Rc<AppState>) -> Result<ExecuteResponse, AppError> {
        let body_json = serde_json::to_string(self).map_err(|e| Source::SerdeJson.to_teapot_error(e, "Send SummaryNoteSave"))?;

        let body = serde_wasm_bindgen::to_value(&body_json).map_err(|e| Source::SerdeWasm.to_teapot_error(e, "Send SummaryNoteSave"))?;

        execute_fetch(&[EndPoint::IpdSummaryNoteId.base(), summary_id.to_string()].concat(), method, Some(&body), app).await
    }
}
