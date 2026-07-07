use derive_demo::Demo;
use js_sys::JsString;
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, MySql, Pool, mysql::MySqlQueryResult};
use sqlx_binder::MySqlBinder;
use std::rc::Rc;
use time::{
    Date, PrimitiveDateTime,
    macros::{date, datetime},
};
use utoipa::{IntoParams, ToSchema};
use wasm_bindgen::JsCast;

use kphis_util::{
    datetime::datetime_from_opt,
    error::{AppError, Source},
    util::str_some,
};

use crate::{
    app::AppState,
    endpoint::{EndPoint, QueryString, find_qs},
    fetch::{ExecuteResponse, execute_fetch, execute_fetch_vec_with_u32, fetch_json_api},
    ipd::summary::SummaryData,
    patient_info::PatientInfo,
};

#[derive(Clone, Default, Deserialize, IntoParams)]
pub struct SummaryAuditParams {
    pub an: Option<String>,
    pub summary_id: Option<u32>,
    pub summary_audit_id: Option<u32>,
}

impl QueryString for SummaryAuditParams {
    fn from_tuples(params: &[(String, String)]) -> Option<Self> {
        (!params.is_empty()).then(|| Self {
            an: find_qs(params, "an"),
            summary_id: find_qs(params, "summary_id").and_then(|s| s.parse::<u32>().ok()),
            summary_audit_id: find_qs(params, "summary_audit_id").and_then(|s| s.parse::<u32>().ok()),
        })
    }

    fn query_string(&self) -> String {
        let mut queries = Vec::with_capacity(3);
        if let Some(an) = &self.an {
            queries.push(["an=", an].concat());
        }
        if let Some(summary_id) = &self.summary_id {
            queries.push(["summary_id=", &summary_id.to_string()].concat());
        }
        if let Some(summary_audit_id) = &self.summary_audit_id {
            queries.push(["summary_audit_id=", &summary_audit_id.to_string()].concat());
        }
        (!queries.is_empty()).then(|| ["?", &queries.join("&")].concat()).unwrap_or_default()
    }
}

/// IPD Summary Audit
#[derive(Clone, Debug, Demo, Deserialize, Serialize, FromRow, MySqlBinder, ToSchema)]
#[schema(example = json!(SummaryAudit::demo()))]
pub struct SummaryAudit {
    #[Demo(value = "1")]
    pub summary_audit_id: u32,
    #[Demo(value = "1")]
    pub summary_id: u32,
    /// UC, OFC, LGO, SSS
    #[Demo(value = r#"Some(String::from("UC"))"#)]
    pub payer: Option<String>,
    /// - I : Internal
    /// - E : Extermal
    #[Demo(value = r#"String::from("I")"#)]
    pub audit_type: String,
    /// - `N` : No authentication
    /// - `T` : Text Signature
    /// - `C` : Cursive Signature
    /// - `D` : Digital Signature
    #[Demo(value = r#"Some(String::from("N"))"#)]
    pub doctor_auth: Option<String>,
    #[Demo(value = r#"Some(String::from("0001234"))"#)]
    pub com_hn: Option<String>,
    #[Demo(value = r#"Some(String::from("660001234"))"#)]
    pub com_an: Option<String>,
    #[Demo(value = "Some(datetime!(2024-01-01 11:11:11))")]
    pub com_adm_datetime: Option<PrimitiveDateTime>,
    /// MUST NOT NULL
    #[Demo(value = "Some(datetime!(2024-01-01 11:11:11))")]
    pub com_dch_datetime: Option<PrimitiveDateTime>,
    #[Demo(value = "Some(0)")]
    pub com_leaveday: Option<i32>,
    /// 1=M, 2=F
    #[Demo(value = r#"Some(String::from("1"))"#)]
    pub com_sex: Option<String>,
    #[Demo(value = "Some(date!(1965-12-06))")]
    pub com_birthday: Option<Date>,
    /// in Grams
    #[Demo(value = "Some(55000)")]
    pub com_bw: Option<i32>,
    #[Demo(value = r#"Some(String::from("02"))"#)]
    pub com_dchstts: Option<String>,
    #[Demo(value = r#"Some(String::from("01"))"#)]
    pub com_dchtype: Option<String>,
    #[Demo(value = r#"Some(String::from("1111111111111"))"#)]
    pub com_pid: Option<String>,
    #[Demo(value = r#"Some(String::from("11500"))"#)]
    pub com_drg: Option<String>,
    #[Demo(value = "Some(0.57670)")]
    pub com_rw: Option<f64>,
    #[Demo(value = "Some(0.57670)")]
    pub com_adjrw: Option<f64>,
    #[Demo(value = r#"Some(String::from("0001234"))"#)]
    pub rev_hn: Option<String>,
    #[Demo(value = r#"Some(String::from("660001234"))"#)]
    pub rev_an: Option<String>,
    #[Demo(value = "Some(datetime!(2024-01-01 11:11:11))")]
    pub rev_adm_datetime: Option<PrimitiveDateTime>,
    #[Demo(value = "Some(datetime!(2024-01-01 11:11:11))")]
    pub rev_dch_datetime: Option<PrimitiveDateTime>,
    #[Demo(value = "Some(0)")]
    pub rev_leaveday: Option<i32>,
    /// 1=M, 2=F
    #[Demo(value = r#"Some(String::from("1"))"#)]
    pub rev_sex: Option<String>,
    #[Demo(value = "Some(date!(1965-12-06))")]
    pub rev_birthday: Option<Date>,
    /// in Grams
    #[Demo(value = "Some(55000)")]
    pub rev_bw: Option<i32>,
    #[Demo(value = r#"Some(String::from("02"))"#)]
    pub rev_dchstts: Option<String>,
    #[Demo(value = r#"Some(String::from("01"))"#)]
    pub rev_dchtype: Option<String>,
    #[Demo(value = r#"Some(String::from("1111111111111"))"#)]
    pub rev_pid: Option<String>,
    #[Demo(value = r#"Some(String::from("11500"))"#)]
    pub rev_drg: Option<String>,
    #[Demo(value = "Some(0.57670)")]
    pub rev_rw: Option<f64>,
    #[Demo(value = "Some(0.57670)")]
    pub rev_adjrw: Option<f64>,
    #[Demo(value = r#"String::from("0")"#)]
    pub sa: String,
    #[Demo(value = r#"String::from("0")"#)]
    pub ca: String,

    #[sqlx_binder(skip)]
    #[Demo(value = r#"Some(String::from("Mr.User"))"#)]
    pub create_username: Option<String>,
    #[sqlx_binder(skip)]
    #[Demo(value = r#"Some(datetime!(2024-01-11 11:11:11))"#)]
    pub create_datetime: Option<PrimitiveDateTime>,
    #[sqlx_binder(skip)]
    #[Demo(value = r#"Some(datetime!(2024-01-11 11:11:11))"#)]
    pub update_datetime: Option<PrimitiveDateTime>,

    #[sqlx_binder(skip)]
    #[sqlx(skip)]
    #[Demo(value = "vec![SummaryAuditItem::demo()]")]
    pub summary_audit_items: Vec<SummaryAuditItem>,
}

impl SummaryAudit {
    pub fn new(patient_opt: &Option<Rc<PatientInfo>>, summary_opt: &Option<Rc<SummaryData>>) -> Option<Rc<Self>> {
        if let (Some(patient), Some(summary)) = (&patient_opt, &summary_opt) {
            datetime_from_opt(patient.dchdate, patient.dchtime).map(|com_dch_datetime| {
                Rc::new(Self {
                    summary_audit_id: 0,
                    summary_id: summary.summary_id,
                    payer: None,
                    audit_type: String::from("I"),
                    doctor_auth: Some(String::from("T")),
                    com_hn: patient.hn.clone(),
                    com_an: str_some(summary.an.clone()),
                    com_adm_datetime: datetime_from_opt(patient.regdate, patient.regtime),
                    com_dch_datetime: Some(com_dch_datetime),
                    com_leaveday: patient.leave_home_day,
                    com_sex: patient.sex.clone(),
                    com_birthday: patient.birthday,
                    com_bw: patient.bw,
                    com_dchstts: summary.discharge_status.clone(),
                    com_dchtype: summary.discharge_type.clone(),
                    com_pid: patient.cid.clone(),
                    com_drg: None,
                    com_rw: None,
                    com_adjrw: None,
                    rev_hn: None,
                    rev_an: None,
                    rev_adm_datetime: None,
                    rev_dch_datetime: None,
                    rev_leaveday: None,
                    rev_sex: None,
                    rev_birthday: None,
                    rev_bw: None,
                    rev_dchstts: None,
                    rev_dchtype: None,
                    rev_pid: None,
                    rev_drg: None,
                    rev_rw: None,
                    rev_adjrw: None,
                    sa: String::new(),
                    ca: String::new(),
                    create_username: None,
                    create_datetime: None,
                    update_datetime: None,
                    summary_audit_items: Vec::new(),
                })
            })
        } else {
            None
        }
    }

    pub fn new_from_parts(patient_opt: &Option<Rc<PatientInfo>>, summary_opt: &Option<Rc<SummaryData>>, summary_audit_items: Vec<SummaryAuditItem>) -> Option<Rc<Self>> {
        if let (Some(patient), Some(summary)) = (&patient_opt, &summary_opt) {
            datetime_from_opt(patient.dchdate, patient.dchtime).map(|com_dch_datetime| {
                Rc::new(Self {
                    summary_audit_id: 0,
                    summary_id: summary.summary_id,
                    payer: None,
                    audit_type: String::from("I"),
                    doctor_auth: Some(String::from("T")),
                    com_hn: patient.hn.clone(),
                    com_an: str_some(summary.an.clone()),
                    com_adm_datetime: datetime_from_opt(patient.regdate, patient.regtime),
                    com_dch_datetime: Some(com_dch_datetime),
                    com_leaveday: patient.leave_home_day,
                    com_sex: patient.sex.clone(),
                    com_birthday: patient.birthday,
                    com_bw: patient.bw,
                    com_dchstts: summary.discharge_status.clone(),
                    com_dchtype: summary.discharge_type.clone(),
                    com_pid: patient.cid.clone(),
                    com_drg: None,
                    com_rw: None,
                    com_adjrw: None,
                    rev_hn: None,
                    rev_an: None,
                    rev_adm_datetime: None,
                    rev_dch_datetime: None,
                    rev_leaveday: None,
                    rev_sex: None,
                    rev_birthday: None,
                    rev_bw: None,
                    rev_dchstts: None,
                    rev_dchtype: None,
                    rev_pid: None,
                    rev_drg: None,
                    rev_rw: None,
                    rev_adjrw: None,
                    sa: String::new(),
                    ca: String::new(),
                    create_username: None,
                    create_datetime: None,
                    update_datetime: None,
                    summary_audit_items,
                })
            })
        } else {
            None
        }
    }

    /// GET `EndPoint::IpdSummaryAudit`
    pub async fn call_api_get(params: &SummaryAuditParams, app: Rc<AppState>) -> Result<Vec<Self>, AppError> {
        match fetch_json_api(&[EndPoint::IpdSummaryAudit.base(), params.query_string()].concat(), "GET", None, app).await {
            Ok((response, true)) => {
                let response: Vec<Self> = serde_wasm_bindgen::from_value(response).map_err(|e| Source::SerdeWasm.to_teapot_error(e, "Fetch SummaryAudit"))?;
                Ok(response)
            }
            Ok((app_error, false)) => {
                let error: AppError = serde_wasm_bindgen::from_value(app_error).map_err(|e| Source::SerdeWasm.to_teapot_error(e, "Fetch SummaryAudit"))?;
                Err(error)
            }
            Err(e) => Err(Source::Js.to_teapot_error(e.dyn_ref::<JsString>().map(|s| s.into()).unwrap_or(String::from("fetch error")), "Fetch Json")),
        }
    }

    /// - POST `EndPoint::IpdSummaryAudit`
    pub async fn call_api_save(&self, app: Rc<AppState>) -> Result<(u32, Vec<ExecuteResponse>), AppError> {
        let body_json = serde_json::to_string(self).map_err(|e| Source::SerdeJson.to_teapot_error(e, "Send SummaryAudit"))?;
        let body = serde_wasm_bindgen::to_value(&body_json).map_err(|e| Source::SerdeWasm.to_teapot_error(e, "Send SummaryAudit"))?;

        execute_fetch_vec_with_u32(&EndPoint::IpdSummaryAudit.base(), "POST", Some(&body), app).await
    }

    /// DELETE `EndPoint::IpdSummaryAudit`
    pub async fn call_api_delete(params: &SummaryAuditParams, app: Rc<AppState>) -> Result<ExecuteResponse, AppError> {
        execute_fetch(&[EndPoint::IpdSummaryAudit.base(), params.query_string()].concat(), "DELETE", None, app).await
    }
}

/// IPD Summary Audit Items
#[derive(Clone, Debug, Default, Demo, Deserialize, Serialize, FromRow, ToSchema)]
#[schema(example = json!(SummaryAudit::demo()))]
pub struct SummaryAuditItem {
    #[Demo(value = "1")]
    pub summary_audit_item_id: u32,
    #[Demo(value = "1")]
    pub summary_audit_id: u32,
    #[Demo(value = "1")]
    pub summary_id: u32,
    /// PDx, SDx, ODx, Op
    #[Demo(value = r#"Some(String::from("PDx"))"#)]
    pub ty: Option<String>,
    #[Demo(value = r#"Some(String::from("Diarrhea"))"#)]
    pub sum_dx: Option<String>,
    #[Demo(value = r#"Some(String::from("A099"))"#)]
    pub sum_icd: Option<String>,
    #[Demo(value = r#"Some(String::from("A099"))"#)]
    pub com_icd: Option<String>,
    #[Demo(value = r#"Some(String::from("Gastroenteritis and colitis of unspecified origin"))"#)]
    pub rev_dx: Option<String>,
    #[Demo(value = r#"Some(String::from("A099"))"#)]
    pub rev_icd: Option<String>,
    #[Demo(value = r#"String::from("0")"#)]
    pub sa: String,
    #[Demo(value = r#"String::from("0")"#)]
    pub ca: String,
    #[Demo(value = r#"Some(String::from("Note"))"#)]
    pub remark: Option<String>,
}

impl SummaryAuditItem {
    pub fn new(ty: &str, summary_id: u32, sum_dx: &Option<String>, sum_icd: &Option<String>, com_icd: &Option<String>) -> Self {
        Self {
            summary_id,
            ty: str_some(ty.to_owned()),
            sum_dx: sum_dx.to_owned(),
            sum_icd: sum_icd.to_owned(),
            com_icd: com_icd.to_owned(),
            ..Default::default()
        }
    }
}
