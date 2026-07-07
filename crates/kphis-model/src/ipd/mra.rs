use derive_demo::Demo;
use js_sys::JsString;
use serde_derive::{Deserialize, Serialize};
use sqlx::{FromRow, MySql, Pool, mysql::MySqlQueryResult};
use sqlx_binder::MySqlBinder;
use std::rc::Rc;
use time::{Date, macros::date};
use utoipa::{IntoParams, ToSchema};
use wasm_bindgen::JsCast;

use kphis_util::{
    datetime::js_now,
    error::{AppError, Source},
};

use crate::{
    app::AppState,
    endpoint::{EndPoint, QueryString, find_qs},
    fetch::{ExecuteResponse, execute_fetch, fetch_json_api},
};

#[derive(Clone, Debug, Default, Deserialize, Serialize, IntoParams)]
pub struct MraParams {
    // pub ward: Option<String>,
    pub an: Option<String>,
    // pub dch_doctor: Option<String>,
    // pub start_dchdate: Option<Date>,
    // pub end_dchdate: Option<Date>,
    /// for delete only
    pub mra_id: Option<u32>,
}

impl QueryString for MraParams {
    fn from_tuples(params: &[(String, String)]) -> Option<Self> {
        (!params.is_empty()).then(|| Self {
            // ward: find_qs(params, "ward"),
            an: find_qs(params, "an"),
            // dch_doctor: find_qs(params, "dch_doctor"),
            // start_dchdate: find_qs(params, "start_dchdate").and_then(|s| date_8601(&s)),
            // end_dchdate: find_qs(params, "end_dchdate").and_then(|s| date_8601(&s)),
            mra_id: find_qs(params, "mra_id").and_then(|s| s.parse::<u32>().ok()),
        })
    }

    fn query_string(&self) -> String {
        let mut queries = Vec::with_capacity(6);
        // if let Some(ward) = &self.ward {
        //     queries.push(["ward=", ward].concat());
        // }
        if let Some(an) = &self.an {
            queries.push(["an=", an].concat());
        }
        // if let Some(dch_doctor) = &self.dch_doctor {
        //     queries.push(["dch_doctor=", dch_doctor].concat());
        // }
        // if let Some(start_dchdate) = &self.start_dchdate {
        //     queries.push(["start_dchdate=", &start_dchdate.to_string()].concat());
        // }
        // if let Some(end_dchdate) = &self.end_dchdate {
        //     queries.push(["end_dchdate=", &end_dchdate.to_string()].concat());
        // }
        if let Some(mra_id) = &self.mra_id {
            queries.push(["mra_id=", &mra_id.to_string()].concat());
        }

        (!queries.is_empty()).then(|| ["?", &queries.join("&")].concat()).unwrap_or_default()
    }
}

/// IPD Medical Record Audit
#[derive(Demo, Deserialize, Serialize, FromRow, MySqlBinder, ToSchema)]
#[schema(example = json!(IpdMra::demo()))]
pub struct IpdMra {
    #[Demo(value = r#"String::from("660001234")"#)]
    pub an: String,
    #[Demo(value = r#"Some(String::from("0001234"))"#)]
    pub hn: Option<String>,
    #[Demo(value = "Some(date!(2023-12-31))")]
    pub adm_date: Option<Date>,
    #[Demo(value = "Some(date!(2023-12-31))")]
    pub dch_date: Option<Date>,
    #[Demo(value = r#"Some(String::from("Mr.Auditor"))"#)]
    pub auditor: Option<String>,
    #[Demo(value = "Some(date!(2023-12-31))")]
    pub audit_date: Option<Date>,
    /// - I : Internal
    /// - E : Extermal
    #[Demo(value = r#"String::from("I")"#)]
    pub audit_type: String,
    #[Demo(value = "false")]
    pub is_psychiatry: bool,
    #[Demo(value = "false")]
    pub is_not_sorted: bool,
    #[Demo(value = "false")]
    pub is_unknown: bool,
    /// I(nadequate), P(roblem), N(o problem)
    #[Demo(value = r#"Some(String::from("N"))"#)]
    pub overall: Option<String>,
    /// using when overall=P
    #[Demo(value = r#"Some(String::from("Some issue"))"#)]
    pub overall_text: Option<String>,

    // discharge Summary: Dx, op
    #[Demo(value = "false")]
    pub sd_m: bool,
    #[Demo(value = "false")]
    pub sd_n: bool,
    #[Demo(value = "Some(true)")]
    pub sd_1: Option<bool>,
    #[Demo(value = "Some(true)")]
    pub sd_2: Option<bool>,
    #[Demo(value = "Some(true)")]
    pub sd_3: Option<bool>,
    #[Demo(value = "Some(true)")]
    pub sd_4: Option<bool>,
    #[Demo(value = "Some(true)")]
    pub sd_5: Option<bool>,
    #[Demo(value = "Some(true)")]
    pub sd_6: Option<bool>,
    #[Demo(value = "Some(true)")]
    pub sd_7: Option<bool>,
    #[Demo(value = "Some(true)")]
    pub sd_8: Option<bool>,
    #[Demo(value = "Some(true)")]
    pub sd_9: Option<bool>,
    #[Demo(value = r#"Some(String::from("Some note"))"#)]
    pub sd_text: Option<String>,

    // discharge Summary: Other
    #[Demo(value = "false")]
    pub so_m: bool,
    #[Demo(value = "false")]
    pub so_n: bool,
    #[Demo(value = "Some(true)")]
    pub so_1: Option<bool>,
    #[Demo(value = "Some(true)")]
    pub so_2: Option<bool>,
    #[Demo(value = "Some(true)")]
    pub so_3: Option<bool>,
    #[Demo(value = "Some(true)")]
    pub so_4: Option<bool>,
    #[Demo(value = "Some(true)")]
    pub so_5: Option<bool>,
    #[Demo(value = "Some(true)")]
    pub so_6: Option<bool>,
    #[Demo(value = "Some(true)")]
    pub so_7: Option<bool>,
    #[Demo(value = r#"Some(String::from("Some note"))"#)]
    pub so_text: Option<String>,

    // Informed Consent
    #[Demo(value = "false")]
    pub ic_m: bool,
    #[Demo(value = "false")]
    pub ic_n: bool,
    #[Demo(value = "Some(true)")]
    pub ic_1: Option<bool>,
    #[Demo(value = "Some(true)")]
    pub ic_2: Option<bool>,
    #[Demo(value = "Some(true)")]
    pub ic_3: Option<bool>,
    #[Demo(value = "Some(true)")]
    pub ic_4: Option<bool>,
    #[Demo(value = "Some(true)")]
    pub ic_5: Option<bool>,
    #[Demo(value = "Some(true)")]
    pub ic_6: Option<bool>,
    #[Demo(value = "Some(true)")]
    pub ic_7: Option<bool>,
    #[Demo(value = "Some(true)")]
    pub ic_8: Option<bool>,
    #[Demo(value = "Some(true)")]
    pub ic_9: Option<bool>,
    #[Demo(value = r#"Some(String::from("Some note"))"#)]
    pub ic_text: Option<String>,

    // History
    #[Demo(value = "false")]
    pub hx_m: bool,
    #[Demo(value = "false")]
    pub hx_n: bool,
    #[Demo(value = "Some(true)")]
    pub hx_1: Option<bool>,
    #[Demo(value = "Some(true)")]
    pub hx_2: Option<bool>,
    #[Demo(value = "Some(true)")]
    pub hx_3: Option<bool>,
    #[Demo(value = "Some(true)")]
    pub hx_4: Option<bool>,
    #[Demo(value = "Some(true)")]
    pub hx_5: Option<bool>,
    #[Demo(value = "Some(true)")]
    pub hx_6: Option<bool>,
    #[Demo(value = "Some(true)")]
    pub hx_7: Option<bool>,
    #[Demo(value = "Some(true)")]
    pub hx_8: Option<bool>,
    #[Demo(value = "Some(true)")]
    pub hx_9: Option<bool>,
    #[Demo(value = r#"Some(String::from("Some note"))"#)]
    pub hx_text: Option<String>,

    // Physical Exam
    #[Demo(value = "false")]
    pub pe_m: bool,
    #[Demo(value = "false")]
    pub pe_n: bool,
    #[Demo(value = "Some(true)")]
    pub pe_1: Option<bool>,
    #[Demo(value = "Some(true)")]
    pub pe_2: Option<bool>,
    #[Demo(value = "Some(true)")]
    pub pe_3: Option<bool>,
    #[Demo(value = "Some(true)")]
    pub pe_4: Option<bool>,
    #[Demo(value = "Some(true)")]
    pub pe_5: Option<bool>,
    #[Demo(value = "Some(true)")]
    pub pe_6: Option<bool>,
    #[Demo(value = "Some(true)")]
    pub pe_7: Option<bool>,
    #[Demo(value = "Some(true)")]
    pub pe_8: Option<bool>,
    #[Demo(value = "Some(true)")]
    pub pe_9: Option<bool>,
    #[Demo(value = r#"Some(String::from("Some note"))"#)]
    pub pe_text: Option<String>,

    // Progress Note
    #[Demo(value = "false")]
    pub pn_m: bool,
    #[Demo(value = "false")]
    pub pn_n: bool,
    #[Demo(value = "Some(true)")]
    pub pn_1: Option<bool>,
    #[Demo(value = "Some(true)")]
    pub pn_2: Option<bool>,
    #[Demo(value = "Some(true)")]
    pub pn_3: Option<bool>,
    #[Demo(value = "Some(true)")]
    pub pn_4: Option<bool>,
    #[Demo(value = "Some(true)")]
    pub pn_5: Option<bool>,
    #[Demo(value = "Some(true)")]
    pub pn_6: Option<bool>,
    #[Demo(value = "Some(true)")]
    pub pn_7: Option<bool>,
    #[Demo(value = "Some(true)")]
    pub pn_8: Option<bool>,
    #[Demo(value = "Some(true)")]
    pub pn_9: Option<bool>,
    #[Demo(value = r#"Some(String::from("Some note"))"#)]
    pub pn_text: Option<String>,

    // Consultation Record
    #[Demo(value = "true")]
    pub cr_na: bool,
    #[Demo(value = "false")]
    pub cr_m: bool,
    #[Demo(value = "false")]
    pub cr_n: bool,
    #[Demo(value = "Some(true)")]
    pub cr_1: Option<bool>,
    #[Demo(value = "Some(true)")]
    pub cr_2: Option<bool>,
    #[Demo(value = "Some(true)")]
    pub cr_3: Option<bool>,
    #[Demo(value = "Some(true)")]
    pub cr_4: Option<bool>,
    #[Demo(value = "Some(true)")]
    pub cr_5: Option<bool>,
    #[Demo(value = "Some(true)")]
    pub cr_6: Option<bool>,
    #[Demo(value = "Some(true)")]
    pub cr_7: Option<bool>,
    #[Demo(value = "Some(true)")]
    pub cr_8: Option<bool>,
    #[Demo(value = "Some(true)")]
    pub cr_9: Option<bool>,
    #[Demo(value = r#"Some(String::from("Some note"))"#)]
    pub cr_text: Option<String>,

    // Anesthetic Record
    #[Demo(value = "true")]
    pub ar_na: bool,
    #[Demo(value = "false")]
    pub ar_m: bool,
    #[Demo(value = "false")]
    pub ar_n: bool,
    #[Demo(value = "Some(true)")]
    pub ar_1: Option<bool>,
    #[Demo(value = "Some(true)")]
    pub ar_2: Option<bool>,
    #[Demo(value = "Some(true)")]
    pub ar_3: Option<bool>,
    #[Demo(value = "Some(true)")]
    pub ar_4: Option<bool>,
    #[Demo(value = "Some(true)")]
    pub ar_5: Option<bool>,
    #[Demo(value = "Some(true)")]
    pub ar_6: Option<bool>,
    #[Demo(value = "Some(true)")]
    pub ar_7: Option<bool>,
    #[Demo(value = "Some(true)")]
    pub ar_8: Option<bool>,
    #[Demo(value = "Some(true)")]
    pub ar_9: Option<bool>,
    #[Demo(value = r#"Some(String::from("Some note"))"#)]
    pub ar_text: Option<String>,

    // Operative Note
    #[Demo(value = "true")]
    pub on_na: bool,
    #[Demo(value = "false")]
    pub on_m: bool,
    #[Demo(value = "false")]
    pub on_n: bool,
    #[Demo(value = "Some(true)")]
    pub on_1: Option<bool>,
    #[Demo(value = "Some(true)")]
    pub on_2: Option<bool>,
    #[Demo(value = "Some(true)")]
    pub on_3: Option<bool>,
    #[Demo(value = "Some(true)")]
    pub on_4: Option<bool>,
    #[Demo(value = "Some(true)")]
    pub on_5: Option<bool>,
    #[Demo(value = "Some(true)")]
    pub on_6: Option<bool>,
    #[Demo(value = "Some(true)")]
    pub on_7: Option<bool>,
    #[Demo(value = "Some(true)")]
    pub on_8: Option<bool>,
    #[Demo(value = "Some(true)")]
    pub on_9: Option<bool>,
    #[Demo(value = r#"Some(String::from("Some note"))"#)]
    pub on_text: Option<String>,

    // Labour Record
    #[Demo(value = "true")]
    pub lr_na: bool,
    #[Demo(value = "false")]
    pub lr_m: bool,
    #[Demo(value = "false")]
    pub lr_n: bool,
    #[Demo(value = "Some(true)")]
    pub lr_1: Option<bool>,
    #[Demo(value = "Some(true)")]
    pub lr_2: Option<bool>,
    #[Demo(value = "Some(true)")]
    pub lr_3: Option<bool>,
    #[Demo(value = "Some(true)")]
    pub lr_4: Option<bool>,
    #[Demo(value = "Some(true)")]
    pub lr_5: Option<bool>,
    #[Demo(value = "Some(true)")]
    pub lr_6: Option<bool>,
    #[Demo(value = "Some(true)")]
    pub lr_7: Option<bool>,
    #[Demo(value = "Some(true)")]
    pub lr_8: Option<bool>,
    #[Demo(value = "Some(true)")]
    pub lr_9: Option<bool>,
    #[Demo(value = r#"Some(String::from("Some note"))"#)]
    pub lr_text: Option<String>,

    // Rehabilitation Record
    #[Demo(value = "true")]
    pub rr_na: bool,
    #[Demo(value = "false")]
    pub rr_m: bool,
    #[Demo(value = "false")]
    pub rr_n: bool,
    #[Demo(value = "Some(true)")]
    pub rr_1: Option<bool>,
    #[Demo(value = "Some(true)")]
    pub rr_2: Option<bool>,
    #[Demo(value = "Some(true)")]
    pub rr_3: Option<bool>,
    #[Demo(value = "Some(true)")]
    pub rr_4: Option<bool>,
    #[Demo(value = "Some(true)")]
    pub rr_5: Option<bool>,
    #[Demo(value = "Some(true)")]
    pub rr_6: Option<bool>,
    #[Demo(value = "Some(true)")]
    pub rr_7: Option<bool>,
    #[Demo(value = "Some(true)")]
    pub rr_8: Option<bool>,
    #[Demo(value = "Some(true)")]
    pub rr_9: Option<bool>,
    #[Demo(value = r#"Some(String::from("Some note"))"#)]
    pub rr_text: Option<String>,

    // Nurse Note
    #[Demo(value = "false")]
    pub nn_m: bool,
    #[Demo(value = "false")]
    pub nn_n: bool,
    #[Demo(value = "Some(true)")]
    pub nn_1: Option<bool>,
    #[Demo(value = "Some(true)")]
    pub nn_2: Option<bool>,
    #[Demo(value = "Some(true)")]
    pub nn_3: Option<bool>,
    #[Demo(value = "Some(true)")]
    pub nn_4: Option<bool>,
    #[Demo(value = "Some(true)")]
    pub nn_5: Option<bool>,
    #[Demo(value = "Some(true)")]
    pub nn_6: Option<bool>,
    #[Demo(value = "Some(true)")]
    pub nn_7: Option<bool>,
    #[Demo(value = "Some(true)")]
    pub nn_8: Option<bool>,
    #[Demo(value = "Some(true)")]
    pub nn_9: Option<bool>,
    #[Demo(value = "Some(true)")]
    pub nn_sub: Option<bool>,
    #[Demo(value = r#"Some(String::from("Some note"))"#)]
    pub nn_text: Option<String>,

    #[Demo(value = "1")]
    pub mra_id: u32, // AUTO_INCREMENT
}

#[rustfmt::skip]
impl IpdMra {
    /// GET `EndPoint::IpdMra`
    pub async fn call_api_get(params: &MraParams, app: Rc<AppState>) -> Result<Vec<Self>, AppError> {
        match fetch_json_api(&[EndPoint::IpdMra.base(), params.query_string()].concat(), "GET", None, app).await {
            Ok((response, true)) => {
                let response: Vec<Self> = serde_wasm_bindgen::from_value(response).map_err(|e| Source::SerdeWasm.to_teapot_error(e, "Fetch IpdMra"))?;
                Ok(response)
            }
            Ok((app_error, false)) => {
                let error: AppError = serde_wasm_bindgen::from_value(app_error).map_err(|e| Source::SerdeWasm.to_teapot_error(e, "Fetch IpdMra"))?;
                Err(error)
            }
            Err(e) => Err(Source::Js.to_teapot_error(e.dyn_ref::<JsString>().map(|s| s.into()).unwrap_or(String::from("fetch error")), "Fetch Json")),
        }
    }

    /// - POST `EndPoint::IpdMra`
    /// - PUT `EndPoint::IpdMra`
    pub async fn call_api_save(&self, app: Rc<AppState>) -> Result<ExecuteResponse, AppError> {
        let body_json = serde_json::to_string(self).map_err(|e| Source::SerdeJson.to_teapot_error(e, "Send IpdMra"))?;
        let method = if self.mra_id > 0 {"PUT"} else {"POST"};
        let body = serde_wasm_bindgen::to_value(&body_json).map_err(|e| Source::SerdeWasm.to_teapot_error(e, "Send IpdMra"))?;

        execute_fetch(&EndPoint::IpdMra.base(), method, Some(&body), app).await
    }

    /// DELETE `EndPoint::IpdMra`
    pub async fn call_api_delete(params: &MraParams, app: Rc<AppState>) -> Result<ExecuteResponse, AppError> {
        execute_fetch(&[EndPoint::IpdMra.base(), params.query_string()].concat(), "DELETE", None, app).await
    }

    pub fn sd_full(&self) -> usize {[self.sd_1,self.sd_2,self.sd_3,self.sd_4,self.sd_5,self.sd_6,self.sd_7,self.sd_8,self.sd_9,].iter().filter_map(|x| *x).count()}
    pub fn so_full(&self) -> usize {[self.so_1,self.so_2,self.so_3,self.so_4,self.so_5,self.so_6,self.so_7,].iter().filter_map(|x| *x).count()}
    pub fn ic_full(&self) -> usize {[self.ic_1,self.ic_2,self.ic_3,self.ic_4,self.ic_5,self.ic_6,self.ic_7,self.ic_8,self.ic_9,].iter().filter_map(|x| *x).count()}
    pub fn hx_full(&self) -> usize {[self.hx_1,self.hx_2,self.hx_3,self.hx_4,self.hx_5,self.hx_6,self.hx_7,self.hx_8,self.hx_9,].iter().filter_map(|x| *x).count()}
    pub fn pe_full(&self) -> usize {[self.pe_1,self.pe_2,self.pe_3,self.pe_4,self.pe_5,self.pe_6,self.pe_7,self.pe_8,self.pe_9,].iter().filter_map(|x| *x).count()}
    pub fn pn_full(&self) -> usize {[self.pn_1,self.pn_2,self.pn_3,self.pn_4,self.pn_5,self.pn_6,self.pn_7,self.pn_8,self.pn_9,].iter().filter_map(|x| *x).count()}
    pub fn cr_full(&self) -> usize {if self.cr_na {0} else {[self.cr_1,self.cr_2,self.cr_3,self.cr_4,self.cr_5,self.cr_6,self.cr_7,self.cr_8,self.cr_9,].iter().filter_map(|x| *x).count()}}
    pub fn ar_full(&self) -> usize {if self.ar_na {0} else {[self.ar_1,self.ar_2,self.ar_3,self.ar_4,self.ar_5,self.ar_6,self.ar_7,self.ar_8,self.ar_9,].iter().filter_map(|x| *x).count()}}
    pub fn on_full(&self) -> usize {if self.on_na {0} else {[self.on_1,self.on_2,self.on_3,self.on_4,self.on_5,self.on_6,self.on_7,self.on_8,self.on_9,].iter().filter_map(|x| *x).count()}}
    pub fn lr_full(&self) -> usize {if self.lr_na {0} else {[self.lr_1,self.lr_2,self.lr_3,self.lr_4,self.lr_5,self.lr_6,self.lr_7,self.lr_8,self.lr_9,].iter().filter_map(|x| *x).count()}}
    pub fn rr_full(&self) -> usize {if self.rr_na {0} else {[self.rr_1,self.rr_2,self.rr_3,self.rr_4,self.rr_5,self.rr_6,self.rr_7,self.rr_8,self.rr_9,].iter().filter_map(|x| *x).count()}}
    pub fn nn_full(&self) -> usize {[self.nn_1,self.nn_2,self.nn_3,self.nn_4,self.nn_5,self.nn_6,self.nn_7,self.nn_8,self.nn_9,].iter().filter_map(|x| *x).count()}

    pub fn sd_score(&self) -> usize {if self.sd_m || self.sd_n {0} else {[self.sd_1,self.sd_2,self.sd_3,self.sd_4,self.sd_5,self.sd_6,self.sd_7,self.sd_8,self.sd_9,].iter().filter(|x| **x == Some(true)).count()}}
    pub fn so_score(&self) -> usize {if self.so_m || self.so_n {0} else {[self.so_1,self.so_2,self.so_3,self.so_4,self.so_5,self.so_6,self.so_7,].iter().filter(|x| **x == Some(true)).count()}}
    pub fn ic_score(&self) -> usize {if self.ic_m || self.ic_n {0} else {[self.ic_1,self.ic_2,self.ic_3,self.ic_4,self.ic_5,self.ic_6,self.ic_7,self.ic_8,self.ic_9,].iter().filter(|x| **x == Some(true)).count()}}
    pub fn hx_score(&self) -> usize {if self.hx_m || self.hx_n {0} else {[self.hx_1,self.hx_2,self.hx_3,self.hx_4,self.hx_5,self.hx_6,self.hx_7,self.hx_8,self.hx_9,].iter().filter(|x| **x == Some(true)).count()}}
    pub fn pe_score(&self) -> usize {if self.pe_m || self.pe_n {0} else {[self.pe_1,self.pe_2,self.pe_3,self.pe_4,self.pe_5,self.pe_6,self.pe_7,self.pe_8,self.pe_9,].iter().filter(|x| **x == Some(true)).count()}}
    pub fn pn_score(&self) -> usize {if self.pn_m || self.pn_n {0} else {[self.pn_1,self.pn_2,self.pn_3,self.pn_4,self.pn_5,self.pn_6,self.pn_7,self.pn_8,self.pn_9,].iter().filter(|x| **x == Some(true)).count()}}
    pub fn cr_score(&self) -> usize {if self.cr_na || self.cr_m || self.cr_n {0} else {[self.cr_1,self.cr_2,self.cr_3,self.cr_4,self.cr_5,self.cr_6,self.cr_7,self.cr_8,self.cr_9,].iter().filter(|x| **x == Some(true)).count()}}
    pub fn ar_score(&self) -> usize {if self.ar_na || self.ar_m || self.ar_n {0} else {[self.ar_1,self.ar_2,self.ar_3,self.ar_4,self.ar_5,self.ar_6,self.ar_7,self.ar_8,self.ar_9,].iter().filter(|x| **x == Some(true)).count()}}
    pub fn on_score(&self) -> usize {if self.on_na || self.on_m || self.on_n {0} else {[self.on_1,self.on_2,self.on_3,self.on_4,self.on_5,self.on_6,self.on_7,self.on_8,self.on_9,].iter().filter(|x| **x == Some(true)).count()}}
    pub fn lr_score(&self) -> usize {if self.lr_na || self.lr_m || self.lr_n {0} else {[self.lr_1,self.lr_2,self.lr_3,self.lr_4,self.lr_5,self.lr_6,self.lr_7,self.lr_8,self.lr_9,].iter().filter(|x| **x == Some(true)).count()}}
    pub fn rr_score(&self) -> usize {if self.rr_na || self.rr_m || self.rr_n {0} else {[self.rr_1,self.rr_2,self.rr_3,self.rr_4,self.rr_5,self.rr_6,self.rr_7,self.rr_8,self.rr_9,].iter().filter(|x| **x == Some(true)).count()}}
    pub fn nn_score(&self) -> usize {if self.nn_m || self.nn_n {0} else {
        [self.nn_1,self.nn_2,self.nn_3,self.nn_4,self.nn_5,self.nn_6,self.nn_7,self.nn_8,self.nn_9,].iter().filter(|x| **x == Some(true)).count() - (if self.nn_sub == Some(true) {1} else {0})
    }}

    pub fn full(&self) -> usize {
        self.sd_full() + self.so_full() + self.ic_full() + self.hx_full() + self.pe_full() + self.pn_full() + self.cr_full() + self.ar_full() + self.on_full() + self.lr_full() + self.rr_full() + self.nn_full()
    }
    pub fn score(&self) -> usize {
        self.sd_score() + self.so_score() + self.ic_score() + self.hx_score() + self.pe_score() + self.pn_score() + self.cr_score() + self.ar_score() + self.on_score() + self.lr_score() + self.rr_score() + self.nn_score()
    }

    pub fn new(
        hn: &Option<String>,
        an: &str,
        adm_date: Option<Date>,
        dch_date: Option<Date>,
        auditor: &Option<String>,
    ) -> Self {
        Self {
            hn: hn.to_owned(),
            an: an.to_owned(),
            adm_date,
            dch_date,
            auditor: auditor.to_owned(),
            audit_date: Some(js_now().date()),
            audit_type: String::from("I"),
            is_psychiatry: false,
            is_not_sorted: false,
            is_unknown: false,
            overall: None,
            overall_text: None,
            sd_m: false, sd_n: false, sd_1: Some(true), sd_2: Some(true), sd_3: Some(true), sd_4: Some(true), sd_5: Some(true), sd_6: Some(true), sd_7: Some(true), sd_8: Some(true), sd_9: Some(true), sd_text: None,
            so_m: false, so_n: false, so_1: Some(true), so_2: Some(true), so_3: Some(true), so_4: Some(true), so_5: Some(true), so_6: Some(true), so_7: Some(true), so_text: None,
            ic_m: false, ic_n: false, ic_1: Some(true), ic_2: Some(true), ic_3: Some(true), ic_4: Some(true), ic_5: Some(true), ic_6: Some(true), ic_7: Some(true), ic_8: Some(true), ic_9: Some(true), ic_text: None,
            hx_m: false, hx_n: false, hx_1: Some(true), hx_2: Some(true), hx_3: Some(true), hx_4: Some(true), hx_5: Some(true), hx_6: Some(true), hx_7: Some(true), hx_8: Some(true), hx_9: Some(true), hx_text: None,
            pe_m: false, pe_n: false, pe_1: Some(true), pe_2: Some(true), pe_3: Some(true), pe_4: Some(true), pe_5: Some(true), pe_6: Some(true), pe_7: Some(true), pe_8: Some(true), pe_9: Some(true), pe_text: None,
            pn_m: false, pn_n: false, pn_1: Some(true), pn_2: Some(true), pn_3: Some(true), pn_4: Some(true), pn_5: Some(true), pn_6: Some(true), pn_7: Some(true), pn_8: Some(true), pn_9: Some(true), pn_text: None,
            cr_na: true,
            cr_m: false, cr_n: false, cr_1: Some(true), cr_2: Some(true), cr_3: Some(true), cr_4: Some(true), cr_5: Some(true), cr_6: Some(true), cr_7: Some(true), cr_8: Some(true), cr_9: Some(true), cr_text: None,
            ar_na: true,
            ar_m: false, ar_n: false, ar_1: Some(true), ar_2: Some(true), ar_3: Some(true), ar_4: Some(true), ar_5: Some(true), ar_6: Some(true), ar_7: Some(true), ar_8: Some(true), ar_9: Some(true), ar_text: None,
            on_na: true,
            on_m: false, on_n: false, on_1: Some(true), on_2: Some(true), on_3: Some(true), on_4: Some(true), on_5: Some(true), on_6: Some(true), on_7: Some(true), on_8: Some(true), on_9: Some(true), on_text: None,
            lr_na: true,
            lr_m: false, lr_n: false, lr_1: Some(true), lr_2: Some(true), lr_3: Some(true), lr_4: Some(true), lr_5: Some(true), lr_6: Some(true), lr_7: Some(true), lr_8: Some(true), lr_9: Some(true), lr_text: None,
            rr_na: true,
            rr_m: false, rr_n: false, rr_1: Some(true), rr_2: Some(true), rr_3: Some(true), rr_4: Some(true), rr_5: Some(true), rr_6: Some(true), rr_7: Some(true), rr_8: Some(true), rr_9: Some(true), rr_text: None,
            nn_m: false, nn_n: false, nn_1: Some(true), nn_2: Some(true), nn_3: Some(true), nn_4: Some(true), nn_5: Some(true), nn_6: Some(true), nn_7: Some(true), nn_8: Some(true), nn_9: Some(true), nn_sub: Some(false), nn_text: None,
            mra_id: 0,
        }
    }
}
