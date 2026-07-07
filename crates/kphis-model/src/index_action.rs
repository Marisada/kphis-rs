use derive_demo::Demo;
use serde::{Deserialize, Serialize};
use sqlx::{
    FromRow, MySql, Pool,
    mysql::MySqlQueryResult,
    types::time::{Date, PrimitiveDateTime, Time},
};
use sqlx_binder::MySqlBinder;
use std::{cmp::Ordering, rc::Rc};
use time::macros::{date, datetime, time};
use utoipa::ToSchema;

use kphis_util::{
    datetime::datetime_from_opt,
    error::{AppError, Source},
};

use crate::{
    app::{AppState, VisitTypeId},
    endpoint::EndPoint,
    fetch::{ExecuteResponse, execute_fetch, execute_fetch_vec_with_u32},
    index_monitor::{IndexMonitor, IndexMonitorOnly},
    order::OrderItem,
};

/// Index Action
#[derive(Clone, Default, Debug, Demo, Deserialize, Eq, Serialize, ToSchema)]
#[schema(example = json!(IndexAction::demo()))]
pub struct IndexAction {
    /// for generic over `an` or `opd_er_order_master_id` only
    #[Demo(value = r#"VisitTypeId::demo_ipd(String::from("660001234"))"#)]
    pub visit_type: VisitTypeId,
    #[Demo(value = "Some(1)")]
    pub plan_id: Option<u32>,
    #[Demo(value = "Some(1)")]
    pub action_id: Option<u32>,
    #[Demo(value = "Some(1)")]
    pub vs_id: Option<u32>,

    #[Demo(value = "Some(datetime!(2023-12-31 23:59:59))")]
    pub check_datetime: Option<PrimitiveDateTime>,
    #[Demo(value = r#"Some(String::from("007"))"#)]
    pub check_person: Option<String>,
    #[Demo(value = r#"Some(String::from("Miss.Nurse"))"#)]
    pub check_person_name: Option<String>,
    #[Demo(value = r#"Some(String::from("Nurse"))"#)]
    pub check_person_entryposition: Option<String>,

    #[Demo(value = r#"Some(String::from("Result"))"#)]
    pub action_result: Option<String>,
    #[Demo(value = r#"Some(String::from("Remark"))"#)]
    pub action_remark: Option<String>,
    #[Demo(value = "Some(date!(2023-12-31))")]
    pub action_date: Option<Date>,
    #[Demo(value = "Some(time!(23:59:59))")]
    pub action_time: Option<Time>,
    /// NOT USE
    #[Demo(default)]
    pub action_report_back: Option<String>,
    #[Demo(value = r#"Some(String::from("Y"))"#)]
    pub action_blood_had: Option<String>,
    #[Demo(value = r#"Some(String::from("007"))"#)]
    pub action_person_1: Option<String>,
    #[Demo(value = r#"Some(String::from("009"))"#)]
    pub action_person_2: Option<String>,
    #[Demo(value = r#"Some(String::from("Miss.Nurse"))"#)]
    pub action_person_1_name: Option<String>,
    #[Demo(value = r#"Some(String::from("Miss.Another"))"#)]
    pub action_person_2_name: Option<String>,
    #[Demo(value = r#"Some(String::from("Nurse"))"#)]
    pub action_person_1_entryposition: Option<String>,
    #[Demo(value = r#"Some(String::from("Nurse"))"#)]
    pub action_person_2_entryposition: Option<String>,

    #[Demo(value = "true")]
    pub has_monitor: bool,
    #[Demo(value = "vec![IndexMonitor::demo()]")]
    pub monitors: Vec<IndexMonitor>,
}

impl Ord for IndexAction {
    fn cmp(&self, other: &Self) -> Ordering {
        let a_dt_opt = datetime_from_opt(self.action_date, self.action_time);
        let b_dt_opt = datetime_from_opt(other.action_date, other.action_time);
        a_dt_opt
            .zip(b_dt_opt)
            .map(|(a, b)| a.cmp(&b))
            .unwrap_or_else(|| self.action_id.zip(other.action_id).map(|(a, b)| a.cmp(&b)).unwrap_or(self.visit_type.cmp(&other.visit_type)))
    }
}

impl PartialOrd for IndexAction {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let a_dt_opt = datetime_from_opt(self.action_date, self.action_time);
        let b_dt_opt = datetime_from_opt(other.action_date, other.action_time);
        a_dt_opt.zip(b_dt_opt).map(|(a, b)| a.cmp(&b))
    }
}

impl PartialEq for IndexAction {
    fn eq(&self, other: &Self) -> bool {
        self.action_id.zip(other.action_id).map(|(a, b)| a.eq(&b)).unwrap_or(self.visit_type.eq(&other.visit_type))
    }
}

impl IndexAction {
    /// - POST `EndPoint::IpdIndexAction`
    /// - POST `EndPoint::OpdErIndexAction`
    pub async fn call_api_post(&self, app: Rc<AppState>) -> Result<(u32, Vec<ExecuteResponse>), AppError> {
        let (path, is_valid) = match &self.visit_type {
            VisitTypeId::Ipd(an) | VisitTypeId::PreAdmit(an) => (EndPoint::IpdIndexAction, !an.is_empty()),
            VisitTypeId::OpdEr(_, opd_er_order_master_id) => (EndPoint::OpdErIndexAction, *opd_er_order_master_id > 0),
            VisitTypeId::Visit(_) => (EndPoint::Unknown, false),
        };

        if is_valid {
            let body_json = serde_json::to_string(self).map_err(|e| Source::SerdeJson.to_teapot_error(e, "Send IndexActionSave"))?;
            let body = serde_wasm_bindgen::to_value(&body_json).map_err(|e| Source::SerdeWasm.to_teapot_error(e, "Send IndexActionSave"))?;

            execute_fetch_vec_with_u32(&path.base(), "POST", Some(&body), app).await
        } else {
            Err(AppError::app_400("Check IndexActionSave"))
        }
    }
    /// - DELETE `EndPoint::IpdIndexActionId`
    /// - DELETE `EndPoint::OpdErIndexActionId`
    pub async fn call_api_delete(is_ipd: bool, action_id: u32, app: Rc<AppState>) -> Result<ExecuteResponse, AppError> {
        let ep = if is_ipd { EndPoint::IpdIndexActionId } else { EndPoint::OpdErIndexActionId };
        execute_fetch(&[ep.base(), action_id.to_string()].concat(), "DELETE", None, app).await
    }

    /// (x, y) : x = Last(Abnormal/Normal), y = HAD(complete/incomplete)
    pub fn last_monitor_status(&self, order_item: &OrderItem) -> (Option<bool>, Option<bool>) {
        let mut monitors = self
            .monitors
            .clone()
            .into_iter()
            .filter(|monitor| monitor.monitor_abnormal.is_some() && monitor.monitor_datetime.is_some())
            .collect::<Vec<IndexMonitor>>();
        monitors.sort_by(|a, b| {
            if let (Some(adt), Some(bdt)) = (a.monitor_datetime, b.monitor_datetime) {
                bdt.cmp(&adt)
            } else {
                Ordering::Equal
            }
        });
        let monitor_len = monitors.len();
        let last = monitors.first().map(|monitor| monitor.monitor_abnormal.as_ref().map(|s| s == "Y").unwrap_or_default());
        let had = order_item.monitor_status.as_ref().map(|s| s == "Y").unwrap_or_default().then(|| {
            monitors
                .first()
                .map(|monitor| {
                    let count_fail = order_item.monitor_count.map(|c| c as usize > monitor_len).unwrap_or_default();
                    let diff_minutes = if let (Some(action_dt), Some(monitor_dt)) = (datetime_from_opt(self.action_date, self.action_time), monitor.monitor_datetime) {
                        (monitor_dt - action_dt).whole_minutes()
                    } else {
                        0
                    };
                    let duration_fail = order_item.monitor_duration.map(|d| d as i64 > diff_minutes).unwrap_or_default();
                    !count_fail && !duration_fail
                })
                .unwrap_or_default()
        });

        (last, had)
    }

    // same as Dom::had_monitor_status() but return &'static str
    pub fn had_monitor_status(&self, order_item: &OrderItem, show_only_had: bool) -> &'static str {
        match self.last_monitor_status(order_item) {
            (Some(is_abnormal), had) => {
                match had {
                    // Complete HAD count/duration
                    Some(true) => {
                        if is_abnormal {
                            "⚠"
                        } else {
                            "🗹"
                        }
                    }
                    // Incomplete HAD count/duration
                    Some(false) => {
                        if is_abnormal {
                            "⧗"
                        } else {
                            "⧖"
                        }
                    }
                    // No HAD count/duration
                    None => {
                        if is_abnormal {
                            "⚠"
                        } else {
                            "✔"
                        }
                    }
                }
            }
            (None, had) => {
                if show_only_had {
                    ""
                } else if had.is_some() {
                    "⧖"
                } else {
                    "🗸"
                }
            }
        }
    }
}

#[derive(Clone, Demo, Deserialize, Serialize, FromRow, MySqlBinder)]
pub struct IndexActionOnly {
    #[Demo(value = "1")]
    pub action_id: u32,
    #[Demo(value = "Some(1)")]
    pub plan_id: Option<u32>,

    #[Demo(value = "Some(datetime!(2023-12-31 23:59:59))")]
    pub check_datetime: Option<PrimitiveDateTime>,
    #[Demo(value = r#"Some(String::from("007"))"#)]
    pub check_person: Option<String>,

    #[Demo(value = r#"Some(String::from("Result"))"#)]
    pub action_result: Option<String>,
    #[Demo(value = r#"Some(String::from("Remark"))"#)]
    pub action_remark: Option<String>,
    #[Demo(value = "Some(date!(2023-12-31))")]
    pub action_date: Option<Date>,
    #[Demo(value = "Some(time!(23:59:59))")]
    pub action_time: Option<Time>,
    /// NOT USE
    #[Demo(default)]
    pub action_report_back: Option<String>,
    #[Demo(value = r#"Some(String::from("Y"))"#)]
    pub action_blood_had: Option<String>,
    #[Demo(value = r#"Some(String::from("007"))"#)]
    pub action_person_1: Option<String>,
    #[Demo(value = r#"Some(String::from("009"))"#)]
    pub action_person_2: Option<String>,
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

    #[sqlx_binder(skip)]
    #[Demo(value = "true")]
    pub has_monitor: bool,

    #[sqlx(skip)]
    #[sqlx_binder(skip)]
    #[Demo(value = "vec![IndexMonitorOnly::demo()]")]
    pub index_monitors: Vec<IndexMonitorOnly>,
}

impl PartialEq for IndexActionOnly {
    fn eq(&self, other: &Self) -> bool {
        // action_id == other.action_id &&
        // self.plan_id == other.plan_id &&
        self.check_datetime == other.check_datetime
            && self.check_person == other.check_person
            && self.action_result == other.action_result
            && self.action_remark == other.action_remark
            && self.action_date == other.action_date
            && self.action_time == other.action_time
            && self.action_report_back == other.action_report_back
            && self.action_blood_had == other.action_blood_had
            && self.action_person_1 == other.action_person_1
            && self.action_person_2 == other.action_person_2
            && self.create_user == other.create_user
            && self.create_datetime == other.create_datetime
            && self.update_user == other.update_user
            && self.update_datetime == other.update_datetime
            && self.version == other.version
            && if self.index_monitors.len() == other.index_monitors.len() {
                self.index_monitors.iter().zip(other.index_monitors.iter()).all(|(a, b)| a.eq(b))
            } else {
                false
            }
    }
}
