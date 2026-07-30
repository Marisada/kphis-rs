use serde::{Deserialize, Serialize};
use std::rc::Rc;

use kphis_util::error::{AppError, Source};

use crate::{app::AppState, fetch::fetch_json_api};

#[derive(Debug, Deserialize, Serialize)]
pub struct Antibiograms {
    pub label: String,
    pub url: String,
}

impl Antibiograms {
    /// GET antibiograms.json
    pub async fn get(app: Rc<AppState>) -> Result<Vec<Self>, AppError> {
        match fetch_json_api("/local/jsons/antibiograms.json", "GET", None, app).await {
            Ok((response, true)) => {
                let response: Vec<Self> = serde_wasm_bindgen::from_value(response).map_err(|e| Source::SerdeWasm.to_teapot_error(e, "Fetch Antibiograms"))?;
                Ok(response)
            }
            Ok((app_error, false)) => {
                let error: AppError = serde_wasm_bindgen::from_value(app_error).map_err(|e| Source::SerdeWasm.to_teapot_error(e, "Fetch Antibiograms"))?;
                Err(error)
            }
            Err(e) => Err(AppError::new_from_js(e, "Fetch Json")),
        }
    }
}
