use derive_demo::Demo;
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, types::time::PrimitiveDateTime};
use std::{cmp::Ordering, rc::Rc};
use time::macros::datetime;
use utoipa::ToSchema;

use kphis_util::error::{AppError, Source};

use crate::{
    app::{AppState, VisitTypeId},
    endpoint::EndPoint,
    fetch::{ExecuteResponse, execute_fetch, execute_fetch_vec_with_u32},
};

/// Index Action
#[derive(Clone, Default, Debug, Demo, Deserialize, Eq, Serialize, ToSchema)]
#[schema(example = json!(IndexMonitor::demo()))]
pub struct IndexMonitor {
    /// for generic over `an` or `opd_er_order_master_id` only
    #[Demo(value = r#"VisitTypeId::demo_ipd(String::from("660001234"))"#)]
    pub visit_type: VisitTypeId,
    #[Demo(value = "1")]
    pub action_id: u32,
    #[Demo(value = "Some(1)")]
    pub monitor_id: Option<u32>,

    #[Demo(value = "Some(datetime!(2023-12-31 23:59:59))")]
    pub monitor_datetime: Option<PrimitiveDateTime>,
    #[Demo(value = r#"String::from("008")"#)]
    pub monitor_doctor: String,
    #[Demo(value = r#"Some(String::from("Y"))"#)]
    pub monitor_abnormal: Option<String>,
    #[Demo(value = r#"Some(String::from("Result"))"#)]
    pub monitor_result: Option<String>,
    #[Demo(value = r#"Some(String::from("Remark"))"#)]
    pub monitor_remark: Option<String>,

    #[Demo(value = r#"Some(String::from("Miss.Nurse"))"#)]
    pub monitor_doctor_name: Option<String>,
}

impl Ord for IndexMonitor {
    fn cmp(&self, other: &Self) -> Ordering {
        self.monitor_datetime
            .zip(other.monitor_datetime)
            .map(|(a, b)| a.cmp(&b))
            .unwrap_or_else(|| self.monitor_id.zip(other.monitor_id).map(|(a, b)| a.cmp(&b)).unwrap_or(self.visit_type.cmp(&other.visit_type)))
    }
}

impl PartialOrd for IndexMonitor {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.monitor_datetime.zip(other.monitor_datetime).map(|(a, b)| a.cmp(&b))
    }
}

impl PartialEq for IndexMonitor {
    fn eq(&self, other: &Self) -> bool {
        self.monitor_id.zip(other.monitor_id).map(|(a, b)| a.eq(&b)).unwrap_or(self.visit_type.eq(&other.visit_type))
    }
}

impl IndexMonitor {
    /// - POST `EndPoint::IpdIndexMonitor`
    /// - POST `EndPoint::OpdErIndexMonitor`
    pub async fn call_api_post(&self, app: Rc<AppState>) -> Result<(u32, Vec<ExecuteResponse>), AppError> {
        let (path, is_valid) = match &self.visit_type {
            VisitTypeId::Ipd(an) | VisitTypeId::PreAdmit(an) => (EndPoint::IpdIndexMonitor, !an.is_empty()),
            VisitTypeId::OpdEr(_, opd_er_order_master_id) => (EndPoint::OpdErIndexMonitor, *opd_er_order_master_id > 0),
            VisitTypeId::Visit(_) => (EndPoint::Unknown, false),
        };

        if is_valid {
            let body_json = serde_json::to_string(self).map_err(|e| Source::SerdeJson.to_teapot_error(e, "Send IndexMontiorSave"))?;
            let body = serde_wasm_bindgen::to_value(&body_json).map_err(|e| Source::SerdeWasm.to_teapot_error(e, "Send IndexMonitorSave"))?;

            execute_fetch_vec_with_u32(&path.base(), "POST", Some(&body), app).await
        } else {
            Err(AppError::app_400("Check IndexMonitorSave"))
        }
    }
    /// - DELETE `EndPoint::IpdIndexMonitorId`
    /// - DELETE `EndPoint::OpdErIndexMonitorId`
    pub async fn call_api_delete(is_ipd: bool, monitor_id: u32, app: Rc<AppState>) -> Result<ExecuteResponse, AppError> {
        let ep = if is_ipd { EndPoint::IpdIndexMonitorId } else { EndPoint::OpdErIndexMonitorId };
        execute_fetch(&[ep.base(), monitor_id.to_string()].concat(), "DELETE", None, app).await
    }
}

#[derive(Clone, Demo, Deserialize, Serialize, FromRow)]
pub struct IndexMonitorOnly {
    #[Demo(value = "1")]
    pub monitor_id: u32,
    #[Demo(value = "1")]
    pub action_id: u32,

    #[Demo(value = "Some(datetime!(2023-12-31 23:59:59))")]
    pub monitor_datetime: Option<PrimitiveDateTime>,
    #[Demo(value = r#"String::from("008")"#)]
    pub monitor_doctor: String,
    #[Demo(value = r#"Some(String::from("Y"))"#)]
    pub monitor_abnormal: Option<String>,
    #[Demo(value = r#"Some(String::from("Result"))"#)]
    pub monitor_result: Option<String>,
    #[Demo(value = r#"Some(String::from("Remark"))"#)]
    pub monitor_remark: Option<String>,

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
}

impl PartialEq for IndexMonitorOnly {
    fn eq(&self, other: &Self) -> bool {
        // monitor_id == other.monitor_id &&
        // self.action_id == other.action_id &&
        self.monitor_datetime == other.monitor_datetime
            && self.monitor_abnormal == other.monitor_abnormal
            && self.monitor_result == other.monitor_result
            && self.monitor_remark == other.monitor_remark
            && self.create_user == other.create_user
            && self.create_datetime == other.create_datetime
            && self.update_user == other.update_user
            && self.update_datetime == other.update_datetime
            && self.version == other.version
    }
}
