use derive_demo::Demo;
use js_sys::JsString;
use serde_derive::{Deserialize, Serialize};
use std::rc::Rc;
use utoipa::{IntoParams, ToSchema};
use wasm_bindgen::JsCast;

use kphis_util::{
    error::{AppError, Source},
    util::str_some,
};

use crate::{
    app::AppState,
    endpoint::{EndPoint, QueryString, find_qs},
    fetch::fetch_json_api,
};

use super::ImageBase64;

/// Image row count from HIS
#[derive(Clone, Debug, Default, Demo, Deserialize, Serialize, sqlx::FromRow, ToSchema)]
#[schema(example = json!(ScanHisExists::demo()))]
pub struct ScanHisExists {
    /// hos.patient_opd_scan (1 image per row)
    #[Demo(value = "true")]
    pub has_scan: bool,
    /// hos.pe_image (0-10 image per row)
    #[Demo(value = "true")]
    pub has_pe: bool,
    /// hos.er_image (0-5 image per row)
    #[Demo(value = "true")]
    pub has_er: bool,
    /// hos.lab_order_image (0-5 image per row)
    #[Demo(value = "true")]
    pub has_lab: bool,
}

impl ScanHisExists {
    pub fn is_not_empty(&self) -> bool {
        self.has_scan || self.has_pe || self.has_er || self.has_lab
    }
}

#[derive(sqlx::FromRow)]
pub struct ScanHis5 {
    image1: Option<Vec<u8>>,
    image2: Option<Vec<u8>>,
    image3: Option<Vec<u8>>,
    image4: Option<Vec<u8>>,
    image5: Option<Vec<u8>>,
    image1_note: Option<String>,
    image2_note: Option<String>,
    image3_note: Option<String>,
    image4_note: Option<String>,
    image5_note: Option<String>,
}

impl ScanHis5 {
    pub fn to_vec(&self) -> Vec<ScanImage> {
        let mut res = Vec::with_capacity(5);

        if let Some(bytes) = self.image1.as_ref() {
            if let Some(sc_img) = match ImageBase64::from_bytes(bytes) {
                Ok(opt) => opt.map(|image| ScanImage {
                    image,
                    note: self.image1_note.clone(),
                }),
                Err(e) => Some(ScanImage::new_warn(&e.message)),
            } {
                res.push(sc_img);
            }
        }
        if let Some(bytes) = self.image2.as_ref() {
            if let Some(sc_img) = match ImageBase64::from_bytes(bytes) {
                Ok(opt) => opt.map(|image| ScanImage {
                    image,
                    note: self.image2_note.clone(),
                }),
                Err(e) => Some(ScanImage::new_warn(&e.message)),
            } {
                res.push(sc_img);
            }
        }
        if let Some(bytes) = self.image3.as_ref() {
            if let Some(sc_img) = match ImageBase64::from_bytes(bytes) {
                Ok(opt) => opt.map(|image| ScanImage {
                    image,
                    note: self.image3_note.clone(),
                }),
                Err(e) => Some(ScanImage::new_warn(&e.message)),
            } {
                res.push(sc_img);
            }
        }
        if let Some(bytes) = self.image4.as_ref() {
            if let Some(sc_img) = match ImageBase64::from_bytes(bytes) {
                Ok(opt) => opt.map(|image| ScanImage {
                    image,
                    note: self.image4_note.clone(),
                }),
                Err(e) => Some(ScanImage::new_warn(&e.message)),
            } {
                res.push(sc_img);
            }
        }
        if let Some(bytes) = self.image5.as_ref() {
            if let Some(sc_img) = match ImageBase64::from_bytes(bytes) {
                Ok(opt) => opt.map(|image| ScanImage {
                    image,
                    note: self.image5_note.clone(),
                }),
                Err(e) => Some(ScanImage::new_warn(&e.message)),
            } {
                res.push(sc_img);
            }
        }

        res
    }
}

#[derive(sqlx::FromRow)]
pub struct ScanHis10 {
    image1: Option<Vec<u8>>,
    image2: Option<Vec<u8>>,
    image3: Option<Vec<u8>>,
    image4: Option<Vec<u8>>,
    image5: Option<Vec<u8>>,
    image6: Option<Vec<u8>>,
    image7: Option<Vec<u8>>,
    image8: Option<Vec<u8>>,
    image9: Option<Vec<u8>>,
    image10: Option<Vec<u8>>,
    image1_note: Option<String>,
    image2_note: Option<String>,
    image3_note: Option<String>,
    image4_note: Option<String>,
    image5_note: Option<String>,
    image6_note: Option<String>,
    image7_note: Option<String>,
    image8_note: Option<String>,
    image9_note: Option<String>,
    image10_note: Option<String>,
}

impl ScanHis10 {
    pub fn to_vec(&self) -> Vec<ScanImage> {
        let mut res = Vec::with_capacity(10);

        if let Some(bytes) = self.image1.as_ref() {
            if let Some(sc_img) = match ImageBase64::from_bytes(bytes) {
                Ok(opt) => opt.map(|image| ScanImage {
                    image,
                    note: self.image1_note.clone(),
                }),
                Err(e) => Some(ScanImage::new_warn(&e.message)),
            } {
                res.push(sc_img);
            }
        }
        if let Some(bytes) = self.image2.as_ref() {
            if let Some(sc_img) = match ImageBase64::from_bytes(bytes) {
                Ok(opt) => opt.map(|image| ScanImage {
                    image,
                    note: self.image2_note.clone(),
                }),
                Err(e) => Some(ScanImage::new_warn(&e.message)),
            } {
                res.push(sc_img);
            }
        }
        if let Some(bytes) = self.image3.as_ref() {
            if let Some(sc_img) = match ImageBase64::from_bytes(bytes) {
                Ok(opt) => opt.map(|image| ScanImage {
                    image,
                    note: self.image3_note.clone(),
                }),
                Err(e) => Some(ScanImage::new_warn(&e.message)),
            } {
                res.push(sc_img);
            }
        }
        if let Some(bytes) = self.image4.as_ref() {
            if let Some(sc_img) = match ImageBase64::from_bytes(bytes) {
                Ok(opt) => opt.map(|image| ScanImage {
                    image,
                    note: self.image4_note.clone(),
                }),
                Err(e) => Some(ScanImage::new_warn(&e.message)),
            } {
                res.push(sc_img);
            }
        }
        if let Some(bytes) = self.image5.as_ref() {
            if let Some(sc_img) = match ImageBase64::from_bytes(bytes) {
                Ok(opt) => opt.map(|image| ScanImage {
                    image,
                    note: self.image5_note.clone(),
                }),
                Err(e) => Some(ScanImage::new_warn(&e.message)),
            } {
                res.push(sc_img);
            }
        }
        if let Some(bytes) = self.image6.as_ref() {
            if let Some(sc_img) = match ImageBase64::from_bytes(bytes) {
                Ok(opt) => opt.map(|image| ScanImage {
                    image,
                    note: self.image6_note.clone(),
                }),
                Err(e) => Some(ScanImage::new_warn(&e.message)),
            } {
                res.push(sc_img);
            }
        }
        if let Some(bytes) = self.image7.as_ref() {
            if let Some(sc_img) = match ImageBase64::from_bytes(bytes) {
                Ok(opt) => opt.map(|image| ScanImage {
                    image,
                    note: self.image7_note.clone(),
                }),
                Err(e) => Some(ScanImage::new_warn(&e.message)),
            } {
                res.push(sc_img);
            }
        }
        if let Some(bytes) = self.image8.as_ref() {
            if let Some(sc_img) = match ImageBase64::from_bytes(bytes) {
                Ok(opt) => opt.map(|image| ScanImage {
                    image,
                    note: self.image8_note.clone(),
                }),
                Err(e) => Some(ScanImage::new_warn(&e.message)),
            } {
                res.push(sc_img);
            }
        }
        if let Some(bytes) = self.image9.as_ref() {
            if let Some(sc_img) = match ImageBase64::from_bytes(bytes) {
                Ok(opt) => opt.map(|image| ScanImage {
                    image,
                    note: self.image9_note.clone(),
                }),
                Err(e) => Some(ScanImage::new_warn(&e.message)),
            } {
                res.push(sc_img);
            }
        }
        if let Some(bytes) = self.image10.as_ref() {
            if let Some(sc_img) = match ImageBase64::from_bytes(bytes) {
                Ok(opt) => opt.map(|image| ScanImage {
                    image,
                    note: self.image10_note.clone(),
                }),
                Err(e) => Some(ScanImage::new_warn(&e.message)),
            } {
                res.push(sc_img);
            }
        }

        res
    }
}

#[derive(Clone, Default, Deserialize, IntoParams)]
pub struct ScanImageParams {
    /// [opd, er, pe, lab]
    #[param(required, example = "opd")]
    pub key: Option<String>,
    #[param(required, example = "661231235959")]
    pub vn: Option<String>,
    pub an: Option<String>,
}

impl QueryString for ScanImageParams {
    fn from_tuples(params: &[(String, String)]) -> Option<Self> {
        (!params.is_empty()).then(|| Self {
            key: find_qs(params, "key"),
            vn: find_qs(params, "vn"),
            an: find_qs(params, "an"),
        })
    }

    fn query_string(&self) -> String {
        let mut queries = Vec::with_capacity(3);

        if let Some(key) = &self.key {
            queries.push(["key=", &key.to_owned()].concat());
        }
        if let Some(vn) = &self.vn {
            queries.push(["vn=", &vn.to_owned()].concat());
        }
        if let Some(an) = &self.an {
            queries.push(["an=", an].concat());
        }

        (!queries.is_empty()).then(|| ["?", &queries.join("&")].concat()).unwrap_or_default()
    }
}

/// Image with note
#[derive(Clone, Debug, Demo, Deserialize, Serialize, ToSchema)]
#[schema(example = json!(ScanImage::demo()))]
pub struct ScanImage {
    #[Demo(value = "ImageBase64::new_warn()")]
    pub image: ImageBase64,
    #[Demo(value = r#"Some(String::from("Note"))"#)]
    pub note: Option<String>,
}

impl ScanImage {
    pub fn new_warn(warn: &str) -> Self {
        Self {
            image: ImageBase64::new_warn(),
            note: str_some(warn.to_owned()),
        }
    }

    /// GET `EndPoint::ScanHisImage`<br>
    /// get image from key [opd, er, pe, lab]
    pub async fn call_api_get(params: &ScanImageParams, app: Rc<AppState>) -> Result<Vec<Self>, AppError> {
        match fetch_json_api(&[EndPoint::ScanHisImage.base(), params.query_string()].concat(), "GET", None, app).await {
            Ok((response, true)) => {
                let response: Vec<Self> = serde_wasm_bindgen::from_value(response).map_err(|e| Source::SerdeWasm.to_teapot_error(e, "Fetch ScanImage"))?;
                Ok(response)
            }
            Ok((app_error, false)) => {
                let error: AppError = serde_wasm_bindgen::from_value(app_error).map_err(|e| Source::SerdeWasm.to_teapot_error(e, "Fetch ScanImage"))?;
                Err(error)
            }
            Err(e) => Err(Source::Js.to_teapot_error(e.dyn_ref::<JsString>().map(|s| s.into()).unwrap_or(String::from("fetch error")), "Fetch Json")),
        }
    }
}
