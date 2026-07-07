use derive_demo::Demo;
use js_sys::JsString;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use std::rc::Rc;
use utoipa::ToSchema;
use wasm_bindgen::JsCast;

use kphis_util::error::{AppError, Source};

use crate::{app::AppState, endpoint::EndPoint, fetch::fetch_json_api, image::file_path::DocumentType};

/// Exists of all IPD Documents
#[derive(Clone, Default, Deserialize, Serialize, FromRow, ToSchema)]
#[schema(example = json!(IpdDocumentExists::default()))]
pub struct IpdDocumentExists {
    pub has_data_summary2: bool,
    pub has_data_dr_admission_note: bool,
    pub has_data_nurse_admission_note: bool,
    pub has_data_order: bool,
    pub has_data_progress_note: bool,
    pub has_data_dr_consult: bool,
    pub has_data_focus_list: bool,
    pub has_data_focus_note: bool,
    pub has_data_vital_sign: bool,
    pub has_data_io: bool,
    pub has_data_index_plan: bool,
    pub has_data_lab: bool,
    pub has_data_xray: bool,
    pub has_data_ct: bool,
    pub has_data_mri: bool,
    pub has_data_med_reconciliation: bool,
    pub has_data_med_reconciliation_hosxp: bool,
    pub has_data_operation: bool,
    pub has_data_refer_out: bool,
    pub has_scan_consent: bool,
    pub has_scan_insure: bool,
    pub has_scan_refer_in: bool,
    pub has_scan_refer_out: bool,
    pub has_scan_culture: bool,
    pub has_scan_blood: bool,
    pub has_scan_special: bool,
    pub has_scan_ekg: bool,
    pub has_scan_xray: bool,
    pub has_scan_ct: bool,
    pub has_scan_mri: bool,
    pub has_scan_oper: bool,
    pub has_scan_anes: bool,
    pub has_scan_labour: bool,
    pub has_scan_physio: bool,
    pub has_scan_alt_med: bool,
    pub has_scan_nutrition: bool,
    pub has_scan_others: bool,
    pub has_scan_other_sp_clinic: bool,
    pub has_scan_opd_card: bool,
    pub has_scan_finance: bool,
    pub opd_er_order_master_id: Option<u32>,
}

impl IpdDocumentExists {
    /// GET `EndPoint::IpdDocumentListVnAn`
    pub async fn call_api_get(vn: &str, an: &str, app: Rc<AppState>) -> Result<Self, AppError> {
        match fetch_json_api(&[&EndPoint::IpdDocumentListVnAn.base(), vn, "/", an].concat(), "GET", None, app).await {
            Ok((response, true)) => {
                let response: Self = serde_wasm_bindgen::from_value(response).map_err(|e| Source::SerdeWasm.to_teapot_error(e, "Fetch IpdDocumentExists"))?;
                Ok(response)
            }
            Ok((app_error, false)) => {
                let error: AppError = serde_wasm_bindgen::from_value(app_error).map_err(|e| Source::SerdeWasm.to_teapot_error(e, "Fetch IpdDocumentExists"))?;
                Err(error)
            }
            Err(e) => Err(Source::Js.to_teapot_error(e.dyn_ref::<JsString>().map(|s| s.into()).unwrap_or(String::from("fetch error")), "Fetch Json")),
        }
    }
}

/// DateTime of specific IPD Documents
#[derive(Clone, Default, Demo, Deserialize, Serialize, FromRow, ToSchema)]
#[schema(example = json!(IpdDocumentDatetime::demo()))]
pub struct IpdDocumentDatetime {
    #[Demo(value = r#"Some(String::from("2023-12-31 23:59:59|2023-12-31 23:59:59"))"#)]
    pub dr_admission_note: Option<String>,
    #[Demo(value = r#"Some(String::from("2023-12-31 23:59:59|2023-12-31 23:59:59"))"#)]
    pub nurse_admission_note: Option<String>,
    #[Demo(value = r#"Some(String::from("2023-12-31 23:59:59|2023-12-31 23:59:59"))"#)]
    pub summary2: Option<String>,
}

impl IpdDocumentDatetime {
    /// GET `EndPoint::IpdDocumentDatetimeAn`
    pub async fn call_api_get(an: &str, app: Rc<AppState>) -> Result<Self, AppError> {
        match fetch_json_api(&[&EndPoint::IpdDocumentDatetimeAn.base(), an].concat(), "GET", None, app).await {
            Ok((response, true)) => {
                let response: Self = serde_wasm_bindgen::from_value(response).map_err(|e| Source::SerdeWasm.to_teapot_error(e, "Fetch OpdErDocumentCount"))?;
                Ok(response)
            }
            Ok((app_error, false)) => {
                let error: AppError = serde_wasm_bindgen::from_value(app_error).map_err(|e| Source::SerdeWasm.to_teapot_error(e, "Fetch OpdErDocumentCount"))?;
                Err(error)
            }
            Err(e) => Err(Source::Js.to_teapot_error(e.dyn_ref::<JsString>().map(|s| s.into()).unwrap_or(String::from("fetch error")), "Fetch Json")),
        }
    }
}

/// Detail of Document Scan
#[derive(Clone, Default, Demo, Deserialize, Serialize, ToSchema)]
#[schema(example = json!(DocumentScan::demo()))]
pub struct DocumentScan {
    #[Demo(value = "1")]
    pub document_id: u32,
    #[Demo(value = "DocumentType::demo_informed_consent()")]
    pub document_type_id: DocumentType,
    #[Demo(value = "true")]
    pub has_image: bool,
}

impl DocumentScan {
    /// GET `EndPoint::IpdDocumentScanAn`
    pub async fn call_api_get_ipd(an: &str, app: Rc<AppState>) -> Result<Vec<Self>, AppError> {
        match fetch_json_api(&[&EndPoint::IpdDocumentScanAn.base(), an].concat(), "GET", None, app).await {
            Ok((response, true)) => {
                let response: Vec<Self> = serde_wasm_bindgen::from_value(response).map_err(|e| Source::SerdeWasm.to_teapot_error(e, "Fetch IpdDocumentType"))?;
                Ok(response)
            }
            Ok((app_error, false)) => {
                let error: AppError = serde_wasm_bindgen::from_value(app_error).map_err(|e| Source::SerdeWasm.to_teapot_error(e, "Fetch IpdDocumentType"))?;
                Err(error)
            }
            Err(e) => Err(Source::Js.to_teapot_error(e.dyn_ref::<JsString>().map(|s| s.into()).unwrap_or(String::from("fetch error")), "Fetch Json")),
        }
    }

    /// GET `EndPoint::OpdErDocumentScanId`
    pub async fn call_api_get_opd_er(opd_er_order_master_id: u32, app: Rc<AppState>) -> Result<Vec<Self>, AppError> {
        match fetch_json_api(&[EndPoint::OpdErDocumentScanId.base(), opd_er_order_master_id.to_string()].concat(), "GET", None, app).await {
            Ok((response, true)) => {
                let response: Vec<Self> = serde_wasm_bindgen::from_value(response).map_err(|e| Source::SerdeWasm.to_teapot_error(e, "Fetch OpdErDocumentType"))?;
                Ok(response)
            }
            Ok((app_error, false)) => {
                let error: AppError = serde_wasm_bindgen::from_value(app_error).map_err(|e| Source::SerdeWasm.to_teapot_error(e, "Fetch OpdErDocumentType"))?;
                Err(error)
            }
            Err(e) => Err(Source::Js.to_teapot_error(e.dyn_ref::<JsString>().map(|s| s.into()).unwrap_or(String::from("fetch error")), "Fetch Json")),
        }
    }
}
