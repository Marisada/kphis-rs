use js_sys::JsString;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use std::rc::Rc;
use utoipa::ToSchema;
use wasm_bindgen::JsCast;

use kphis_util::error::{AppError, Source};

use crate::{app::AppState, endpoint::EndPoint, fetch::fetch_json_api};

/// Amount of all OPD-ER Documents
#[derive(Clone, Default, Deserialize, Serialize, FromRow, ToSchema)]
#[schema(example = json!(OpdErDocumentExists::default()))]
pub struct OpdErDocumentExists {
    pub has_data_er_master_id: bool,
    pub has_data_index_plan: bool,
    pub has_data_vital_sign: bool,
    pub has_data_order: bool,
    pub has_data_progress_note: bool,
    pub has_data_focus_list: bool,
    pub has_data_focus_note: bool,
    pub has_data_io: bool,
    pub has_data_lab: bool,
    pub has_data_med_reconciliation: bool,
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
}

impl OpdErDocumentExists {
    /// GET `EndPoint::OpdErDocumentListVnId`
    pub async fn call_api_get(vn: &str, opd_er_order_master_id: u32, app: Rc<AppState>) -> Result<Self, AppError> {
        match fetch_json_api(&[&EndPoint::OpdErDocumentListVnId.base(), vn, "/", &opd_er_order_master_id.to_string()].concat(), "GET", None, app).await {
            Ok((response, true)) => {
                let response: Self = serde_wasm_bindgen::from_value(response).map_err(|e| Source::SerdeWasm.to_teapot_error(e, "Fetch OpdErDocumentExists"))?;
                Ok(response)
            }
            Ok((app_error, false)) => {
                let error: AppError = serde_wasm_bindgen::from_value(app_error).map_err(|e| Source::SerdeWasm.to_teapot_error(e, "Fetch OpdErDocumentExists"))?;
                Err(error)
            }
            Err(e) => Err(Source::Js.to_teapot_error(e.dyn_ref::<JsString>().map(|s| s.into()).unwrap_or(String::from("fetch error")), "Fetch Json")),
        }
    }
}
