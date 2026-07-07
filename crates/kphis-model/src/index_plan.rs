use derive_demo::Demo;
use js_sys::JsString;
use serde::{Deserialize, Serialize};
use sqlx::{
    FromRow, MySql, Pool,
    mysql::MySqlQueryResult,
    types::time::{Date, PrimitiveDateTime, Time},
};
use sqlx_binder::MySqlBinder;
use std::{collections::HashSet, rc::Rc, time::Duration};
use time::{
    format_description::well_known::Iso8601,
    macros::{date, datetime, time},
};
use utoipa::{IntoParams, ToSchema};
use wasm_bindgen::JsCast;

use kphis_util::{
    datetime::{JsTime, date_8601, date_th_opt, datetime_from_opt, datetime_th_opt, js_now, time_hm_opt},
    error::{AppError, Source},
};

use crate::{
    app::{AppState, VisitTypeId},
    endpoint::{EndPoint, QueryString, find_qs},
    fetch::{ExecuteResponse, execute_fetch, execute_fetch_vec_with_u32, fetch_json_api},
    index_action::{IndexAction, IndexActionOnly},
};

/// Index Plan Date with today marking
#[derive(Clone, Demo, Deserialize, Serialize, FromRow, ToSchema)]
#[schema(example = json!(IndexPlanDate::demo()))]
pub struct IndexPlanDate {
    #[Demo(value = "date!(2023-12-31)")]
    pub plan_date: Date,
    #[Demo(value = "true")]
    pub is_today: bool,
}

impl IndexPlanDate {
    pub fn string(&self) -> String {
        [self.plan_date.to_string(), if self.is_today { String::from("1") } else { String::from("0") }].join("|")
    }

    pub fn from_string(value: &str) -> Option<Self> {
        let tuple = value.split('|').collect::<Vec<&str>>();
        if tuple.len() == 2 {
            Date::parse(tuple[0], &Iso8601::DEFAULT).ok().map(|date| Self {
                plan_date: date,
                is_today: tuple[1] == "1",
            })
        } else {
            None
        }
    }

    /// GET `EndPoint::IpdIndexPlanDateAn`
    pub async fn call_api_get(an: &str, app: Rc<AppState>) -> Result<Vec<Self>, AppError> {
        match fetch_json_api(&[&EndPoint::IpdIndexPlanDateAn.base(), an].concat(), "GET", None, app).await {
            Ok((response, true)) => {
                let response: Vec<Self> = serde_wasm_bindgen::from_value(response).map_err(|e| Source::SerdeWasm.to_teapot_error(e, "Fetch IndexPlanDate"))?;
                Ok(response)
            }
            Ok((app_error, false)) => {
                let error: AppError = serde_wasm_bindgen::from_value(app_error).map_err(|e| Source::SerdeWasm.to_teapot_error(e, "Fetch IndexPlanDate"))?;
                Err(error)
            }
            Err(e) => Err(Source::Js.to_teapot_error(e.dyn_ref::<JsString>().map(|s| s.into()).unwrap_or(String::from("fetch error")), "Fetch Json")),
        }
    }
}

impl PartialEq for IndexPlanDate {
    fn eq(&self, other: &Self) -> bool {
        self.plan_date == other.plan_date
    }
}

#[derive(Clone, Default, Deserialize, IntoParams)]
pub struct IndexPlanParams {
    pub plan_id: Option<u32>,
    pub order_item_id: Option<u32>,
    pub order_item_type: Option<String>,
    pub an: Option<String>,
    pub opd_er_order_master_id: Option<u32>,
    pub plan_date: Option<Date>,
    pub start_plan_date: Option<Date>,
    pub end_plan_date: Option<Date>,
    pub plan_sch_type: Option<String>,
    pub nurse_assign: Option<String>,
    pub without_order: Option<String>,
    pub ward: Option<String>,
    pub passcode: Option<String>,
    pub patient: Option<String>,
}

impl QueryString for IndexPlanParams {
    fn from_tuples(params: &[(String, String)]) -> Option<Self> {
        (!params.is_empty()).then(|| Self {
            plan_id: find_qs(params, "plan_id").and_then(|s| s.parse::<u32>().ok()),
            order_item_id: find_qs(params, "order_item_id").and_then(|s| s.parse::<u32>().ok()),
            order_item_type: find_qs(params, "order_item_type"),
            an: find_qs(params, "an"),
            opd_er_order_master_id: find_qs(params, "opd_er_order_master_id").and_then(|s| s.parse::<u32>().ok()),
            plan_date: find_qs(params, "plan_date").and_then(|s| date_8601(&s)),
            start_plan_date: find_qs(params, "start_plan_date").and_then(|s| date_8601(&s)),
            end_plan_date: find_qs(params, "end_plan_date").and_then(|s| date_8601(&s)),
            plan_sch_type: find_qs(params, "plan_sch_type"),
            nurse_assign: find_qs(params, "nurse_assign"),
            without_order: find_qs(params, "without_order"),
            ward: find_qs(params, "ward"),
            passcode: find_qs(params, "passcode"),
            patient: find_qs(params, "patient"),
        })
    }

    fn query_string(&self) -> String {
        let mut queries = Vec::with_capacity(14);
        if let Some(plan_id) = &self.plan_id {
            queries.push(["plan_id=", &plan_id.to_string()].concat());
        }
        if let Some(order_item_id) = &self.order_item_id {
            queries.push(["order_item_id=", &order_item_id.to_string()].concat());
        }
        if let Some(order_item_type) = &self.order_item_type {
            queries.push(["order_item_type=", &order_item_type.to_owned()].concat());
        }
        if let Some(an) = &self.an {
            queries.push(["an=", an].concat());
        }
        if let Some(opd_er_order_master_id) = &self.opd_er_order_master_id {
            queries.push(["opd_er_order_master_id=", &opd_er_order_master_id.to_string()].concat());
        }
        if let Some(plan_date) = &self.plan_date {
            queries.push(["plan_date=", &plan_date.to_string()].concat());
        }
        if let Some(start_plan_date) = &self.start_plan_date {
            queries.push(["start_plan_date=", &start_plan_date.to_string()].concat());
        }
        if let Some(end_plan_date) = &self.end_plan_date {
            queries.push(["end_plan_date=", &end_plan_date.to_string()].concat());
        }
        if let Some(plan_sch_type) = &self.plan_sch_type {
            queries.push(["plan_sch_type=", plan_sch_type].concat());
        }
        if let Some(nurse_assign) = &self.nurse_assign {
            queries.push(["nurse_assign=", nurse_assign].concat());
        }
        if let Some(without_order) = &self.without_order {
            queries.push(["without_order=", without_order].concat());
        }
        if let Some(ward) = &self.ward {
            queries.push(["ward=", ward].concat());
        }
        if let Some(passcode) = &self.passcode {
            queries.push(["passcode=", passcode].concat());
        }
        if let Some(patient) = &self.patient {
            queries.push(["patient=", patient].concat());
        }

        (!queries.is_empty()).then(|| ["?", &queries.join("&")].concat()).unwrap_or_default()
    }
}

/// Action of Order Items
#[derive(Clone, Debug, Demo, Default, Deserialize, Serialize, ToSchema)]
#[schema(example = json!(IndexPlan::demo()))]
pub struct IndexPlan {
    /// for generic over `an` or `opd_er_order_master_id` only
    #[Demo(value = r#"VisitTypeId::demo_ipd(String::from("660001234"))"#)]
    pub visit_type: VisitTypeId,
    #[Demo(value = "1")]
    pub plan_id: u32,
    #[Demo(value = "Some(1)")]
    pub order_item_id: Option<u32>,
    #[Demo(value = r#"Some(String::from("Plan Detail"))"#)]
    pub plan_detail: Option<String>,
    #[Demo(value = "Some(date!(2023-12-31))")]
    pub plan_date: Option<Date>,
    #[Demo(value = "Some(time!(23:59:59))")]
    pub plan_time: Option<Time>,
    /// stat, date, time
    #[Demo(value = r#"Some(String::from("stat"))"#)]
    pub plan_sch_type: Option<String>,
    #[Demo(value = "Some(date!(2023-12-31))")]
    pub order_date: Option<Date>,
    #[Demo(value = "Some(time!(23:59:59))")]
    pub order_time: Option<Time>,

    /// oneday, continuous
    #[Demo(value = r#"Some(String::from("oneday"))"#)]
    pub order_type: Option<String>,
    #[Demo(value = "Some(datetime!(2023-12-31 23:59:59))")]
    pub off_by_datetime: Option<PrimitiveDateTime>,
    #[Demo(value = "vec![IndexAction::demo()]")]
    pub actions: Vec<IndexAction>,
}

impl IndexPlan {
    pub fn is_continuous_or_none(&self) -> bool {
        self.order_type.as_ref().map(|order_type| order_type.as_str() == "continuous").unwrap_or(true)
    }

    /// DELETE `EndPoint::IpdIndexPlanId`
    /// DELETE `EndPoint::OpdErIndexPlanId`
    pub async fn call_api_delete(is_ipd: bool, plan_id: u32, app: Rc<AppState>) -> Result<ExecuteResponse, AppError> {
        let ep = if is_ipd { EndPoint::IpdIndexPlanId } else { EndPoint::OpdErIndexPlanId };
        execute_fetch(&[ep.base(), plan_id.to_string()].concat(), "DELETE", None, app).await
    }
}

impl IndexActionStatus for IndexPlan {
    fn order_type(&self) -> Option<String> {
        self.order_type.clone()
    }
    fn order_date(&self) -> Option<Date> {
        self.order_date
    }
    fn order_time(&self) -> Option<Time> {
        self.order_time
    }
    fn off_by_datetime(&self) -> Option<PrimitiveDateTime> {
        self.off_by_datetime
    }
    fn plan_sch_type(&self) -> Option<String> {
        self.plan_sch_type.clone()
    }
    fn plan_date(&self) -> Option<Date> {
        self.plan_date
    }
    fn plan_time(&self) -> Option<Time> {
        self.plan_time
    }
    fn actions(&self) -> impl Iterator<Item = Rc<IndexAction>> {
        self.actions.clone().into_iter().map(Rc::new)
    }
}

#[derive(Clone, Demo, Deserialize, Serialize, FromRow, MySqlBinder)]
pub struct IndexPlanOnly {
    #[Demo(value = "1")]
    pub plan_id: u32,
    #[Demo(value = "Some(1)")]
    pub order_item_id: Option<u32>,
    #[Demo(value = r#"Some(String::from("Detail"))"#)]
    pub plan_detail: Option<String>,
    #[Demo(value = "Some(date!(2023-12-31))")]
    pub plan_date: Option<Date>,
    #[Demo(value = "Some(time!(23:59:59))")]
    pub plan_time: Option<Time>,
    #[Demo(value = r#"Some(String::from("stat"))"#)]
    pub plan_sch_type: Option<String>,
    #[Demo(value = r#"String::from("user")"#)]
    pub create_user: String,
    #[Demo(value = "datetime!(2023-12-31 23:59:59)")]
    pub create_datetime: PrimitiveDateTime,
    #[Demo(value = r#"String::from("user")"#)]
    pub update_user: String,
    #[Demo(value = "datetime!(2023-12-31 23:59:59)")]
    pub update_datetime: PrimitiveDateTime,
    #[Demo(value = "1")]
    pub version: i32,

    #[sqlx(skip)]
    #[sqlx_binder(skip)]
    #[Demo(value = "vec![IndexActionOnly::demo()]")]
    pub index_actions: Vec<IndexActionOnly>,
}

impl PartialEq for IndexPlanOnly {
    fn eq(&self, other: &Self) -> bool {
        // self.plan_id == other.plan_id &&
        // self.order_item_id == other.order_item_id &&
        self.plan_detail == other.plan_detail
            && self.plan_date == other.plan_date
            && self.plan_time == other.plan_time
            && self.plan_sch_type == other.plan_sch_type
            && self.create_user == other.create_user
            && self.create_datetime == other.create_datetime
            && self.update_user == other.update_user
            && self.update_datetime == other.update_datetime
            && self.version == other.version
            && if self.index_actions.len() == other.index_actions.len() {
                self.index_actions.iter().zip(other.index_actions.iter()).all(|(a, b)| a.eq(b))
            } else {
                false
            }
    }
}

// /// Index Plan with associated data
// #[derive(Clone, Demo, Deserialize, Serialize, ToSchema)]
// #[schema(example = json!(IndexPlanPlus::demo()))]
// pub struct IndexPlanPlus {
//     /// for generic over `an` or `opd_er_order_master_id` only
//     #[Demo(value = r#"VisitTypeId::demo_ipd(String::from("660001234"))"#)]
//     pub visit_type: VisitTypeId,
//     #[Demo(value = r#"Some(String::from("0001234"))"#)]
//     pub hn: Option<String>,

//     #[Demo(value = "1")]
//     pub plan_id: u32,
//     #[Demo(value = r#"Some(String::from("Detail"))"#)]
//     pub plan_detail: Option<String>,
//     #[Demo(value = "Some(date!(2023-12-31))")]
//     pub plan_date: Option<Date>,
//     #[Demo(value = "Some(time!(23:59:59))")]
//     pub plan_time: Option<Time>,
//     /// stat, date, time
//     #[Demo(value = r#"Some(String::from("stat"))"#)]
//     pub plan_sch_type: Option<String>,

//     #[Demo(value = "Some(1)")]
//     pub order_item_id: Option<u32>,
//     #[Demo(value = "Some(1)")]
//     pub order_id: Option<u32>,
//     #[Demo(value = r#"Some(String::from("รับประทานครั้งละ1 เม็ด ทุก 6 ชั่วโมง"))"#)]
//     pub order_item_detail: Option<String>,
//     #[Demo(value = r#"Some(String::from("Y"))"#)]
//     pub stat: Option<String>,
//     #[Demo(value = r#"Some(String::from("1000222"))"#)]
//     pub icode: Option<String>,
//     #[Demo(value = "Some(1)")]
//     pub off_order_item_id: Option<u32>,
//     #[Demo(value = r#"Some(String::from("med"))"#)]
//     pub off_order_item_type: Option<String>,
//     #[Demo(value = r#"Some(String::from("รับประทานครั้งละ1 เม็ด ทุก 6 ชั่วโมง"))"#)]
//     pub off_order_item_detail: Option<String>,
//     #[Demo(value = r#"Some(String::from("med"))"#)]
//     pub order_item_type: Option<String>,
//     #[Demo(value = r#"Some(String::from("Incharge"))"#)]
//     pub nurse_assign: Option<String>,

//     #[Demo(value = "Some(date!(2023-12-31))")]
//     pub order_date: Option<Date>,
//     #[Demo(value = "Some(time!(23:59:59))")]
//     pub order_time: Option<Time>,
//     #[Demo(value = r#"Some(String::from("007"))"#)]
//     pub order_doctor: Option<String>,
//     #[Demo(value = r#"Some(String::from("oneday"))"#)]
//     pub order_type: Option<String>,
//     #[Demo(value = r#"Some(String::from("doctor"))"#)]
//     pub order_owner_type: Option<String>,
//     #[Demo(value = r#"Some(String::from("Y"))"#)]
//     pub order_confirm: Option<String>,
//     #[Demo(value = r#"Some(String::from("PARACETAMOL 500 mg. เม็ด"))"#)]
//     pub med_name: Option<String>,
//     #[Demo(value = r#"Some(String::from("TABLET"))"#)]
//     pub dosageform: Option<String>,
//     #[Demo(value = "Some(0)")]
//     pub displaycolor: Option<i32>,
//     #[Demo(value = r#"Some(String::from("PARACETAMOL 500 mg. เม็ด"))"#)]
//     pub off_med_name: Option<String>,
//     #[Demo(value = "Some(0)")]
//     pub off_displaycolor: Option<i32>,

//     #[Demo(value = "Some(datetime!(2023-12-31 23:59:59))")]
//     pub off_by_datetime: Option<PrimitiveDateTime>,

//     #[Demo(value = r#"Some(String::from("C01"))"#)]
//     pub bedno: Option<String>,
//     #[Demo(value = r#"Some(String::from("Mr.Patient Sicker"))"#)]
//     pub patient_name: Option<String>,
//     #[Demo(value = r#"Some(String::from("PENICILLIN=Rash"))"#)]
//     pub drugallergy: Option<String>,
//     #[Demo(value = r#"Some(String::from("PENICILLIN=Rash"))"#)]
//     pub er_drugallergy_history: Option<String>,

//     #[Demo(value = "Some(1)")]
//     pub admission_note_id: Option<u32>,
//     #[Demo(value = r#"Some(String::from("PENICILLIN^Rash"))"#)]
//     pub allergy_drug_history: Option<String>,
//     #[Demo(value = r#"Some(String::from("PENICILLIN=Rash"))"#)]
//     pub allergy_drug_history_hosxp: Option<String>,
//     #[Demo(value = r#"Some(String::from("009"))"#)]
//     pub allergy_drug_pharmacy_check_person: Option<String>,
//     #[Demo(value = "Some(datetime!(2023-12-31 23:59:59))")]
//     pub allergy_drug_pharmacy_check_datetime: Option<PrimitiveDateTime>,

//     #[Demo(value = r#"Some(String::from("Note"))"#)]
//     pub note: Option<String>,
//     #[Demo(value = "vec![IndexAction::demo()]")]
//     pub actions: Vec<IndexAction>,
// }

// impl IndexPlanPlus {
//     /// GET /ipd/index-plan
//     pub async fn get_ipd(params: &IndexPlanParams, app: Rc<AppState>) -> Result<Vec<Self>, AppError> {
//         match fetch_json_api(&[EndPoint::IpdIndexPlan.base(), params.clone().query_string()].concat(), "GET", None, app).await {
//             Ok((response, true)) => {
//                 let response: Vec<Self> = serde_wasm_bindgen::from_value(response).map_err(|e| Source::SerdeWasm.to_teapot_error(e, "Fetch IpdIndexPlan"))?;
//                 Ok(response)
//             }
//             Ok((app_error, false)) => {
//                 let error: AppError = serde_wasm_bindgen::from_value(app_error).map_err(|e| Source::SerdeWasm.to_teapot_error(e, "Fetch IpdIndexPlan"))?;
//                 Err(error)
//             }
//             Err(e) => Err(Source::Js.to_teapot_error(e.dyn_ref::<JsString>().map(|s| s.into()).unwrap_or(String::from("fetch error")), "Fetch Json")),
//         }
//     }
//     /// GET /opd-er/index-plan
//     pub async fn get_opd_er(params: &IndexPlanParams, app: Rc<AppState>) -> Result<Vec<Self>, AppError> {
//         match fetch_json_api(&[EndPoint::OpdErIndexPlan.base(), params.clone().query_string()].concat(), "GET", None, app).await {
//             Ok((response, true)) => {
//                 let response: Vec<Self> = serde_wasm_bindgen::from_value(response).map_err(|e| Source::SerdeWasm.to_teapot_error(e, "Fetch OpdErIndexPlan"))?;
//                 Ok(response)
//             }
//             Ok((app_error, false)) => {
//                 let error: AppError = serde_wasm_bindgen::from_value(app_error).map_err(|e| Source::SerdeWasm.to_teapot_error(e, "Fetch OpdErIndexPlan"))?;
//                 Err(error)
//             }
//             Err(e) => Err(Source::Js.to_teapot_error(e.dyn_ref::<JsString>().map(|s| s.into()).unwrap_or(String::from("fetch error")), "Fetch Json")),
//         }
//     }
// }

// impl IndexActionStatus for IndexPlanPlus {
//     fn order_type(&self) -> Option<String> {
//         self.order_type.clone()
//     }
//     fn order_date(&self) -> Option<Date> {
//         self.order_date
//     }
//     fn order_time(&self) -> Option<Time> {
//         self.order_time
//     }
//     fn off_by_datetime(&self) -> Option<PrimitiveDateTime> {
//         self.off_by_datetime
//     }
//     fn plan_sch_type(&self) -> Option<String> {
//         self.plan_sch_type.clone()
//     }
//     fn plan_date(&self) -> Option<Date> {
//         self.plan_date
//     }
//     fn plan_time(&self) -> Option<Time> {
//         self.plan_time
//     }
//     fn actions(&self) -> impl Iterator<Item = Rc<IndexAction>> {
//         self.actions.clone().into_iter().map(Rc::new)
//     }
// }

/// Index Plan for save
#[derive(Debug, Demo, Deserialize, Serialize, ToSchema)]
#[schema(example = json!(IndexPlanSave::demo()))]
pub struct IndexPlanSave {
    /// for generic over `an` or `opd_er_order_master_id` only
    #[Demo(value = r#"VisitTypeId::demo_ipd(String::from("660001234"))"#)]
    pub visit_type: VisitTypeId,

    #[Demo(value = "Some(1)")]
    pub plan_id: Option<u32>,
    #[Demo(value = "Some(1)")]
    pub order_item_id: Option<u32>,

    #[Demo(value = r#"Some(String::from("Detail"))"#)]
    pub plan_detail: Option<String>,
    #[Demo(value = "Some(date!(2023-12-31))")]
    pub plan_date: Option<Date>,
    #[Demo(value = "Some(time!(23:59:59))")]
    pub plan_time: Option<Time>,
    /// stat, date, time
    #[Demo(value = r#"Some(String::from("stat"))"#)]
    pub plan_sch_type: Option<String>,
}

impl IndexPlanSave {
    /// - POST `EndPoint::IpdIndexPlan
    /// - POST `EndPoint::OpdErIndexPlan`
    pub async fn call_api_post(&self, app: Rc<AppState>) -> Result<(u32, Vec<ExecuteResponse>), AppError> {
        let (path, is_valid) = match &self.visit_type {
            VisitTypeId::Ipd(an) | VisitTypeId::PreAdmit(an) => (EndPoint::IpdIndexPlan, !an.is_empty()),
            VisitTypeId::OpdEr(_, opd_er_order_master_id) => (EndPoint::OpdErIndexPlan, *opd_er_order_master_id > 0),
            VisitTypeId::Visit(_) => (EndPoint::Unknown, false),
        };

        if is_valid {
            let body_json = serde_json::to_string(self).map_err(|e| Source::SerdeJson.to_teapot_error(e, "Send IndexPlanSave"))?;
            let body = serde_wasm_bindgen::to_value(&body_json).map_err(|e| Source::SerdeWasm.to_teapot_error(e, "Send IndexPlanSave"))?;

            execute_fetch_vec_with_u32(&path.base(), "POST", Some(&body), app).await
        } else {
            Err(AppError::app_400("Check IndexPlanSave"))
        }
    }
}

/// IPD Medical Order and Pay from HOSxP
#[derive(Debug, Demo, Deserialize, Serialize, FromRow, ToSchema)]
#[schema(example = json!(IpdIndexMedPay::demo()))]
pub struct IpdIndexMedPay {
    #[Demo(value = r#"String::from("1000222")"#)]
    pub icode: String,
    #[Demo(value = r#"Some(String::from("PARACETAMOL 500 mg. เม็ด"))"#)]
    pub med_name: Option<String>,
    #[Demo(value = "Some(0)")]
    pub displaycolor: Option<i32>,
    #[Demo(value = r#"Some(String::from("TABLET"))"#)]
    pub dosageform: Option<String>,
    #[Demo(value = "Some(10)")]
    pub med_order_qty: Option<i32>,
    #[Demo(value = "Some(10)")]
    pub med_pay_qty: Option<i32>,
    #[Demo(value = r#"Some(String::from("Y"))"#)]
    pub pay_flag: Option<String>, // "Y","N"
    #[Demo(value = "Some(datetime!(2023-12-31 23:59:59))")]
    pub order_date_time: Option<PrimitiveDateTime>,
    #[Demo(value = "Some(datetime!(2023-12-31 23:59:59))")]
    pub pay_date_time: Option<PrimitiveDateTime>,
    #[Demo(value = r#"Some(String::from("user"))"#)]
    pub entryuser: Option<String>,
    #[Demo(value = r#"Some(String::from("Nurse"))"#)]
    pub entryposition: Option<String>,
}

pub enum IndexPlanType {
    /// TIME + ONEDAY, STAT
    SingleTime,
    /// TIME + CONTINUOUS
    SameTime,
    /// PRN + ONEDAY
    WholeDay,
    /// PRN + CONTINUOUS
    EveryDay,
    /// NULL
    Undefined,
}

impl IndexPlanType {
    fn is_continuous_plan(&self) -> bool {
        match self {
            Self::SingleTime => false,
            Self::SameTime => true,
            Self::WholeDay => false,
            Self::EveryDay => true,
            Self::Undefined => false,
        }
    }
}

pub trait IndexActionStatus {
    fn order_type(&self) -> Option<String>;
    fn order_date(&self) -> Option<Date>;
    fn order_time(&self) -> Option<Time>;
    fn off_by_datetime(&self) -> Option<PrimitiveDateTime>;
    fn plan_sch_type(&self) -> Option<String>;
    fn plan_date(&self) -> Option<Date>;
    fn plan_time(&self) -> Option<Time>;
    fn actions(&self) -> impl Iterator<Item = Rc<IndexAction>>;

    fn schedule_title(&self) -> String {
        self.plan_sch_type()
            .and_then(|sch_type| match sch_type.as_str() {
                "stat" => Some(String::from("STAT")),
                "time" => self.plan_time().map(|plan_time| plan_time.js_string()),
                "date" => Some(String::from("PRN")),
                _ => None,
            })
            .unwrap_or_default()
    }

    fn schedule(&self) -> String {
        let is_cont = self.order_type().map(|order_type| order_type.as_str() == "continuous").unwrap_or_default();
        let plan_date = self.plan_date();
        let plan_time = self.plan_time();
        self.plan_sch_type()
            .and_then(|sch_type| {
                match sch_type.as_str() {
                    "stat" => Some(["Stat ", &date_th_opt(&plan_date), " ", &time_hm_opt(&plan_time)].concat()),
                    "time" => {
                        if is_cont {
                            Some([&String::from("ทุกวัน"), " เวลา ", &time_hm_opt(&plan_time), " ตั้งแต่ ", &date_th_opt(&plan_date), " เป็นต้นไป"].concat())
                        } else {
                            Some([&date_th_opt(&plan_date), " เวลา ", &time_hm_opt(&plan_time)].concat())
                        }
                    }
                    "date" => {
                        if is_cont {
                            Some(["ตั้งแต่ ", &date_th_opt(&plan_date), " ", &time_hm_opt(&plan_time), " เป็นต้นไป"].concat())
                        } else {
                            // 24*60*60 = 86400
                            let dt_ended_opt = datetime_from_opt(plan_date, plan_time).map(|dt| dt + Duration::new(86399, 0));
                            Some(["ระหว่าง ", &date_th_opt(&plan_date), " ", &time_hm_opt(&plan_time), " ถึง ", &datetime_th_opt(&dt_ended_opt)].concat())
                        }
                    }
                    _ => None,
                }
            })
            .unwrap_or_default()
    }

    // 'wait' is (not expired) and (not acted at ref datetime)
    // 'missed' is (has any not-acted-in-time (> 30 minutes late; except 'date' sch_type)) or (has any not-acted-day)
    // 'done' is not missed everyday until end
    /// if ref_date is None then fallback with now().date() and ref_time is now().time()<br>
    /// if ref_date is Some and ref_time is None == in the past, then fallback with ref_date:23:59:59<br>
    /// NOTE: NOT detect act after off
    fn check_wait_missed_done(&self, ref_date: Option<Date>, ref_time: Option<Time>, app: Rc<AppState>) -> (bool, bool, bool) {
        let off_by_datetime = self.off_by_datetime();
        let plan_date = self.plan_date();
        let plan_time = self.plan_time();

        let now = js_now();
        let ref_time = if ref_date.is_some() { ref_time.or(Time::from_hms(23, 59, 59).ok()) } else { Some(now.time()) };
        let ref_date = ref_date.or(Some(now.date()));

        // use prior datetime between off_datetime and ref, if cannot compare then use any datetime
        // is_off is has off_datetime and off is before ref
        let (end_datetime, is_offed) = if let Some(r_dt) = datetime_from_opt(ref_date, ref_time) {
            if let Some(off_dt) = off_by_datetime {
                let is_off = off_dt < r_dt;
                let res = if is_off { off_dt } else { r_dt };
                (Some(res), is_off)
            } else {
                (Some(r_dt), false)
            }
        } else {
            (off_by_datetime.or(Some(now)), off_by_datetime.is_some())
        };
        // days beyond plan until end_datetime
        let days_to_end = self.days_passed(end_datetime);
        let days_acted = self.actions().filter_map(|action| action.action_date).collect::<HashSet<Date>>().len();
        let is_ref_over_24hr_from_plan = if let (Some(p_next_dt), Some(r_dt)) = (datetime_from_opt(plan_date.and_then(|d| d.next_day()), plan_time), datetime_from_opt(ref_date, ref_time)) {
            r_dt > p_next_dt
        } else {
            false
        };
        // ref not acted
        let is_ref_missed = self.actions().all(|action| is_none_or_missed(action.action_date, action.action_time, ref_date, ref_time));
        // check plan shift vs ref shift
        let (same_shift_date, same_shift) = if let (Some(p_date), Some(p_time), Some(r_date), Some(r_time)) = (plan_date, plan_time, ref_date, ref_time) {
            match (app.cal_shift(p_date, p_time), app.cal_shift(r_date, r_time)) {
                (Some((ps_date, ps)), Some((rs_date, rs))) => (ps_date == rs_date, ps == rs),
                _ => (false, false),
            }
        } else {
            (false, false)
        };

        match self.index_plan_type() {
            IndexPlanType::SingleTime => {
                let is_acted_in_time = self.actions().any(|action| {
                    if let (Some(a_dt), Some(p_dt)) = (datetime_from_opt(action.action_date, action.action_time), datetime_from_opt(plan_date, plan_time)) {
                        let half_hr = Duration::new(30 * 60, 0);
                        let start = p_dt - half_hr;
                        let end = p_dt + half_hr;
                        start <= a_dt && a_dt <= end
                    } else {
                        false
                    }
                });
                (
                    !is_offed && !is_ref_over_24hr_from_plan && is_ref_missed && same_shift_date && same_shift, // wait
                    !is_acted_in_time,                                                                          // missed
                    is_offed || is_acted_in_time,                                                               // done
                )
            }
            IndexPlanType::SameTime => {
                // any acted not in_time
                let has_any_missed_action = self.actions().any(|action| is_none_or_missed(action.action_date, action.action_time, plan_date, plan_time));
                let is_everyday_not_missed = {
                    let mut result = true;
                    for day in 0..=days_to_end {
                        if self
                            .actions()
                            .any(|action| !is_none_or_missed(action.action_date, action.action_time, plan_date.map(|d| d + Duration::new(24 * 60 * 60 * (day as u64), 0)), plan_time))
                        {
                            continue;
                        } else {
                            result = false;
                            break;
                        }
                    }
                    result
                };
                (
                    !is_offed && is_ref_missed && same_shift,            // wait
                    (days_to_end > days_acted) || has_any_missed_action, // missed
                    is_offed && is_everyday_not_missed,                  // done
                )
            }
            IndexPlanType::EveryDay => (
                !is_offed,                             // wait
                days_to_end > days_acted,              // missed
                is_offed && days_to_end == days_acted, // done
            ),
            IndexPlanType::WholeDay => {
                let is_acted_within_24hr = self.actions().any(|action| {
                    if let (Some(a_dt), Some(p_dt)) = (datetime_from_opt(action.action_date, action.action_time), datetime_from_opt(plan_date, plan_time)) {
                        let start = p_dt - Duration::new(30 * 60, 0);
                        let end = p_dt + Duration::new(((24 * 60) + 30) * 60, 0);
                        start <= a_dt && a_dt <= end
                    } else {
                        false
                    }
                });
                (
                    !is_offed && !is_ref_over_24hr_from_plan, // wait
                    !is_acted_within_24hr,                    // missed
                    is_offed || is_acted_within_24hr,         // done
                )
            }
            IndexPlanType::Undefined => (false, false, false),
        }
    }

    fn days_passed(&self, ref_datetime: Option<PrimitiveDateTime>) -> usize {
        if let (Some(r_datetime), Some(p_dt)) = (ref_datetime, datetime_from_opt(self.plan_date(), self.plan_time())) {
            (r_datetime - p_dt).whole_days() as usize
        } else {
            0
        }
    }

    fn is_none_or_missed(&self, action_date: Option<Date>, action_time: Option<Time>) -> bool {
        is_none_or_missed(action_date, action_time, self.plan_date(), self.plan_time())
    }

    // plan must visible first
    /// - Continuous: no ref_date return return all actions
    /// - OneDay: return all actions
    fn actions_visible(&self, ref_date: Option<Date>) -> Vec<Rc<IndexAction>> {
        if let Some(r_date) = ref_date
            && self.index_plan_type().is_continuous_plan()
        {
            self.actions().filter(|action| if let Some(a_date) = action.action_date { a_date == r_date } else { false }).collect()
        } else {
            self.actions().collect()
        }
    }

    fn actions_with_datetime(&self) -> Vec<Rc<IndexAction>> {
        self.actions().into_iter().filter(|action| action.action_date.is_some() && action.action_time.is_some()).collect()
    }

    fn index_plan_type(&self) -> IndexPlanType {
        let is_cont = self.order_type().as_ref().map(|order_type| order_type.as_str() == "continuous").unwrap_or_default();
        self.plan_sch_type()
            .as_ref()
            .map(|sch_type| match sch_type.as_str() {
                "stat" => IndexPlanType::SingleTime,
                "time" => {
                    if is_cont {
                        IndexPlanType::SameTime
                    } else {
                        IndexPlanType::SingleTime
                    }
                }
                "date" => {
                    if is_cont {
                        IndexPlanType::EveryDay
                    } else {
                        IndexPlanType::WholeDay
                    }
                }
                _ => IndexPlanType::Undefined,
            })
            .unwrap_or(IndexPlanType::Undefined)
    }

    // order-item must visible first
    /// ref_date is None will return true
    fn is_plan_visible(&self, ref_date: Option<Date>) -> bool {
        if let Some(r_date) = ref_date {
            // let off_by_datetime = self.off_by_datetime();
            let plan_date = self.plan_date();
            // get `plan` and `next-plan` date
            if let Some(p_date) = plan_date {
                // if let (Some(p_date), Some(np_date)) = (plan_date, plan_date.and_then(|d| d.next_day())) {
                // CONTINUOUS ALL
                if self.index_plan_type().is_continuous_plan() {
                    // let r_end_dt = PrimitiveDateTime::new(r_date, Time::MIDNIGHT - Duration::from_secs(1));
                    // CONTINUOUS + OFFED: show before offed
                    // if let Some(off_dt) = off_by_datetime {
                    //     let limited_off_dt = if off_dt < r_end_dt { off_dt } else { r_end_dt };
                    //     p_date <= r_date && r_date <= limited_off_dt.date()
                    // // CONTINUOUS:
                    // } else {
                    p_date <= r_date
                    // }
                    // // ONE-DAY + OFFED: show in order-date (allow plan before order)
                    // } else if let Some(off_dt) = off_by_datetime {
                    //     let off_date = off_dt.date();
                    //     if off_date <= np_date {
                    //         p_date <= r_date && r_date <= off_date
                    //     } else {
                    //         p_date <= r_date && r_date <= np_date
                    //     }
                    // ONE-DAY: always visible (when order-item visible)
                } else {
                    // p_date <= r_date && r_date <= np_date
                    true
                }
            } else {
                false
            }
        } else {
            true
        }
    }
}

/// MISSD means `action and plan time different more than 30 minutes``
fn is_none_or_missed(action_date: Option<Date>, action_time: Option<Time>, plan_date: Option<Date>, plan_time: Option<Time>) -> bool {
    if let (Some(a_dt), Some(p_dt)) = (datetime_from_opt(action_date, action_time), datetime_from_opt(plan_date, plan_time)) {
        (a_dt - p_dt).abs() > Duration::new(30 * 60, 0)
    } else {
        true
    }
}

#[cfg(test)]
#[rustfmt::skip]
pub mod tests {

    use time::macros::{date, time};
    use super::*;

    #[test]
    pub fn test_is_none_or_missed() {
        assert!(is_none_or_missed(Some(date!(2024-01-15)), Some(time!(11:29)), Some(date!(2024-01-15)), Some(time!(12:00))));
        assert!(!is_none_or_missed(Some(date!(2024-01-15)), Some(time!(11:30)), Some(date!(2024-01-15)), Some(time!(12:00))));
        assert!(!is_none_or_missed(Some(date!(2024-01-15)), Some(time!(12:30)), Some(date!(2024-01-15)), Some(time!(12:00))));
        assert!(is_none_or_missed(Some(date!(2024-01-15)), Some(time!(12:31)), Some(date!(2024-01-15)), Some(time!(12:00))));

        assert!(is_none_or_missed(None, Some(time!(12:00)), Some(date!(2024-01-15)), Some(time!(12:00))));
        assert!(is_none_or_missed(Some(date!(2024-01-15)), None, Some(date!(2024-01-15)), Some(time!(12:00))));
        assert!(is_none_or_missed(Some(date!(2024-01-15)), Some(time!(12:00)), None, Some(time!(12:00))));
        assert!(is_none_or_missed(Some(date!(2024-01-15)), Some(time!(12:00)), Some(date!(2024-01-15)), None));
    }
}
