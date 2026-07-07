use derive_demo::Demo;
use js_sys::JsString;
use serde_derive::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};
use std::rc::Rc;
use strum::EnumIter;
use time::{PrimitiveDateTime, macros::datetime};
use utoipa::ToSchema;
use wasm_bindgen::JsCast;
use web_sys::{File, FormData};

use kphis_util::error::{AppError, Source};

use crate::{
    PATH_PREFIX_IMAGE, PATH_PREFIX_THUMB,
    app::AppState,
    endpoint::EndPoint,
    fetch::{ExecuteResponse, bytes_to_blob, execute_fetch, fetch_json_api, file_to_bytes, post_multipart},
    image::{image_from_raw_image, pdf_to_image, webp_creator},
};

// we save ImageUsage in database as u8
// PLEASE ADD TO THE BOTTOM
// change order will make image to show in the wrong place
#[derive(Clone, Debug, Default, Demo, Serialize_repr, Deserialize_repr, PartialEq, ToSchema)]
#[schema(example = json!(ImageUsage::demo_ipd_progress_note()))]
#[repr(u8)]
pub enum ImageUsage {
    #[default]
    Unknown,
    IpdDrAdmissionNote,       // usage_key_id = admission_note_id
    OpdErMedicalHistory,      // usage_key_id = opd_er_order_master_id
    IpdProgressNote,          // usage_key_id = progress_note_id
    OpdErProgressNote,        // usage_key_id = progress_note_id
    IpdFocusNoteAssessment,   // usage_key_id = fcnote_id
    OpdErFocusNoteAssessment, // usage_key_id = fcnote_id
    IpdFocusNoteEvaluation,   // usage_key_id = fcnote_id
    OpdErFocusNoteEvaluation, // usage_key_id = fcnote_id
    IpdConsultData,           // usage_key_id = consult_id
    IpdConsultFinding,        // usage_key_id = consult_id
    IpdDocument,
    OpdErDocument,
}

impl ImageUsage {
    pub fn new_from_str(s: &str) -> Self {
        match s {
            "1" => Self::IpdDrAdmissionNote,
            "2" => Self::OpdErMedicalHistory,
            "3" => Self::IpdProgressNote,
            "4" => Self::OpdErProgressNote,
            "5" => Self::IpdFocusNoteAssessment,
            "6" => Self::OpdErFocusNoteAssessment,
            "7" => Self::IpdFocusNoteEvaluation,
            "8" => Self::OpdErFocusNoteEvaluation,
            "9" => Self::IpdConsultData,
            "10" => Self::IpdConsultFinding,
            "11" => Self::IpdDocument,
            "12" => Self::OpdErDocument,
            _ => Self::Unknown,
        }
    }
    pub fn new_from_u8(u: u8) -> Self {
        match u {
            1 => Self::IpdDrAdmissionNote,
            2 => Self::OpdErMedicalHistory,
            3 => Self::IpdProgressNote,
            4 => Self::OpdErProgressNote,
            5 => Self::IpdFocusNoteAssessment,
            6 => Self::OpdErFocusNoteAssessment,
            7 => Self::IpdFocusNoteEvaluation,
            8 => Self::OpdErFocusNoteEvaluation,
            9 => Self::IpdConsultData,
            10 => Self::IpdConsultFinding,
            11 => Self::IpdDocument,
            12 => Self::OpdErDocument,
            _ => Self::Unknown,
        }
    }

    pub fn label(&self) -> &'static str {
        match self {
            Self::Unknown => "UNSPECIFIED",
            Self::IpdDrAdmissionNote => "IPD-ADMISSION-NOTE-DOCTOR",
            Self::OpdErMedicalHistory => "OPD-ER-MEDICAL-HISTORY",
            Self::IpdProgressNote => "IPD-PROGRESS-NOTE",
            Self::OpdErProgressNote => "OPD-ER-PROGRESS-NOTE",
            Self::IpdFocusNoteAssessment => "IPD-FOCUS-NOTE-ASSESSMENT",
            Self::OpdErFocusNoteAssessment => "OPD-ER-FOCUS-NOTE-ASSESSMENT",
            Self::IpdFocusNoteEvaluation => "IPD-FOCUS-NOTE-EVALUATION",
            Self::OpdErFocusNoteEvaluation => "OPD-ER-FOCUS-NOTE-EVALUATION",
            Self::IpdConsultData => "IPD-CONSULT-DATA",
            Self::IpdConsultFinding => "IPD-CONSULT-FINDNIG",
            Self::IpdDocument => "IPD-DOCUMENT",
            Self::OpdErDocument => "OPD-ER-DOCUMENT",
        }
    }
}

// we save DocumentType value in database as u8
// PLEASE ADD TO THE BOTTOM
// change order will make image to show in the wrong place
#[derive(Clone, Debug, Default, Demo, Serialize_repr, Deserialize_repr, PartialEq, ToSchema, EnumIter)]
#[schema(example = json!(DocumentType::demo_informed_consent()))]
#[repr(u8)]
pub enum DocumentType {
    #[default]
    #[strum(disabled)]
    Unknown,
    InformedConsent,
    InsureCheck,
    ReferIn,
    ReferOut,
    CulturePatho,
    Blood,
    SpecialLab,
    EKG,
    Xray,
    CT,
    MRI,
    Operation,
    Anesthesia,
    Labour,
    Physiotherapy,
    AlternativeRx,
    Nutrition,
    Others,
    OtherSpClinic,
    OPDcard,
    Finance,
}

impl DocumentType {
    pub fn new_from_str(s: &str) -> Self {
        match s {
            "1" => Self::InformedConsent,
            "2" => Self::InsureCheck,
            "3" => Self::ReferIn,
            "4" => Self::ReferOut,
            "5" => Self::CulturePatho,
            "6" => Self::Blood,
            "7" => Self::SpecialLab,
            "8" => Self::EKG,
            "9" => Self::Xray,
            "10" => Self::CT,
            "11" => Self::MRI,
            "12" => Self::Operation,
            "13" => Self::Anesthesia,
            "14" => Self::Labour,
            "15" => Self::Physiotherapy,
            "16" => Self::AlternativeRx,
            "17" => Self::Nutrition,
            "18" => Self::Others,
            "19" => Self::OtherSpClinic,
            "20" => Self::OPDcard,
            "21" => Self::Finance,
            _ => Self::Unknown,
        }
    }
    pub fn new_from_u8(u: u8) -> Self {
        match u {
            1 => Self::InformedConsent,
            2 => Self::InsureCheck,
            3 => Self::ReferIn,
            4 => Self::ReferOut,
            5 => Self::CulturePatho,
            6 => Self::Blood,
            7 => Self::SpecialLab,
            8 => Self::EKG,
            9 => Self::Xray,
            10 => Self::CT,
            11 => Self::MRI,
            12 => Self::Operation,
            13 => Self::Anesthesia,
            14 => Self::Labour,
            15 => Self::Physiotherapy,
            16 => Self::AlternativeRx,
            17 => Self::Nutrition,
            18 => Self::Others,
            19 => Self::OtherSpClinic,
            20 => Self::OPDcard,
            21 => Self::Finance,
            _ => Self::Unknown,
        }
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            Self::InformedConsent => "1",
            Self::InsureCheck => "2",
            Self::ReferIn => "3",
            Self::ReferOut => "4",
            Self::CulturePatho => "5",
            Self::Blood => "6",
            Self::SpecialLab => "7",
            Self::EKG => "8",
            Self::Xray => "9",
            Self::CT => "10",
            Self::MRI => "11",
            Self::Operation => "12",
            Self::Anesthesia => "13",
            Self::Labour => "14",
            Self::Physiotherapy => "15",
            Self::AlternativeRx => "16",
            Self::Nutrition => "17",
            Self::Others => "18",
            Self::OtherSpClinic => "19",
            Self::OPDcard => "20",
            Self::Finance => "21",
            Self::Unknown => "0",
        }
    }

    pub fn label(&self) -> &'static str {
        match self {
            Self::InformedConsent => "ใบยินยอม",
            Self::InsureCheck => "ใบตรวจสอบสิทธิ์",
            Self::ReferIn => "ใบ Refer-In",
            Self::ReferOut => "ใบ Refer-Out",
            Self::CulturePatho => "ผลการเพาะเชื้อ/ชิ้นเนื้อ",
            Self::Blood => "ข้อมูลการให้เลือด",
            Self::SpecialLab => "ผลตรวจพิเศษ",
            Self::EKG => "EKG",
            Self::Xray => "ผลตรวจ X-Ray",
            Self::CT => "ผลตรวจ CT Scan",
            Self::MRI => "ผลตรวจ MRI",
            Self::Operation => "บันทึกการผ่าตัด",
            Self::Anesthesia => "บันทึกการระงับความรู้สึก",
            Self::Labour => "บ้นทึกห้องคลอด/หลังคลอด",
            Self::Physiotherapy => "บันทึกกายภาพบำบัด",
            Self::AlternativeRx => "บันทึกการแพทย์ทางเลือก",
            Self::Nutrition => "บันทึกโภชนาการ",
            Self::Others => "เอกสารอื่นๆ",
            Self::OtherSpClinic => "เอกสารสำคัญทางคลินิกอื่นๆ",
            Self::OPDcard => "OPD card",
            Self::Finance => "เอกสารใบค่าใช้จ่าย",
            Self::Unknown => "เลือกเอกสาร",
        }
    }

    /// POST `EndPoint::IpdDocumentScanAn`
    pub async fn call_api_post_ipd(&self, an: &str, app: Rc<AppState>) -> Result<ExecuteResponse, AppError> {
        let body_json = serde_json::to_string(self).map_err(|e| Source::SerdeJson.to_teapot_error(e, "Send IpdDocumentType"))?;

        let body = serde_wasm_bindgen::to_value(&body_json).map_err(|e| Source::SerdeWasm.to_teapot_error(e, "Send IpdDocumentType"))?;

        execute_fetch(&[&EndPoint::IpdDocumentScanAn.base(), an].concat(), "POST", Some(&body), app).await
    }

    /// POST `EndPoint::OpdErDocumentScanId`
    pub async fn call_api_post_opd_er(&self, opd_er_order_master_id: u32, app: Rc<AppState>) -> Result<ExecuteResponse, AppError> {
        let body_json = serde_json::to_string(self).map_err(|e| Source::SerdeJson.to_teapot_error(e, "Send OpdErDocumentType"))?;

        let body = serde_wasm_bindgen::to_value(&body_json).map_err(|e| Source::SerdeWasm.to_teapot_error(e, "Send OpdErDocumentType"))?;

        execute_fetch(&[EndPoint::OpdErDocumentScanId.base(), opd_er_order_master_id.to_string()].concat(), "POST", Some(&body), app).await
    }

    /// DELETE `EndPoint::IpdDocumentScanAn`
    pub async fn call_api_delete_ipd(&self, an: &str, app: Rc<AppState>) -> Result<ExecuteResponse, AppError> {
        let body_json = serde_json::to_string(self).map_err(|e| Source::SerdeJson.to_teapot_error(e, "Delete IpdDocumentType"))?;

        let body = serde_wasm_bindgen::to_value(&body_json).map_err(|e| Source::SerdeWasm.to_teapot_error(e, "Delete IpdDocumentType"))?;

        execute_fetch(&[&EndPoint::IpdDocumentScanAn.base(), an].concat(), "DELETE", Some(&body), app).await
    }

    /// DELETE `EndPoint::OpdErDocumentScanId`
    pub async fn call_api_delete_opd_er(&self, opd_er_order_master_id: u32, app: Rc<AppState>) -> Result<ExecuteResponse, AppError> {
        let body_json = serde_json::to_string(self).map_err(|e| Source::SerdeJson.to_teapot_error(e, "Delete OpdErDocumentType"))?;

        let body = serde_wasm_bindgen::to_value(&body_json).map_err(|e| Source::SerdeWasm.to_teapot_error(e, "Delete OpdErDocumentType"))?;

        execute_fetch(&[EndPoint::OpdErDocumentScanId.base(), opd_er_order_master_id.to_string()].concat(), "Delete", Some(&body), app).await
    }
}

/// Image in Multipart Form
#[derive(Demo, Serialize, ToSchema)]
#[schema(example = json!(ImageSave::demo()))]
pub struct ImageSave {
    #[Demo(value = r#"String::from(PATH_PREFIX_THUMB)"#)]
    pub name: String,
    #[Demo(value = r#"String::from("01J/G0/M004KYHATX7J2W7MB28X4.webp")"#)]
    pub file_name: String,
    #[Demo(value = r#"String::from("image/webp")"#)]
    pub content_type: String,
    #[schema(content_media_type = "application/octet-stream")]
    #[Demo(value = r#"vec![52,49,46,46]"#)]
    pub bytes: Vec<u8>,
}

// Flow of data
// 1. upload file: request = Multipart, response = [ImageFile] without additional data
// 2. set usage: request = ImageFile with additional data, response = ExecureResponse
// 3. set title: request = ImageFile with additional data, response = ExecureResponse
// 4. delete usage: request = [image_usage_id], response = ExecureResponse
// 5. delete image: request = [image_id], response = ExecureResponse
// NOTE: we use ImageUsage so we impl FromRow manually
/// Image path inside `image` or `thumb` root<br>
/// image_usage_id, image_usage and crete_username is additional data
#[derive(Clone, Debug, Demo, Deserialize, Serialize, ToSchema)]
#[schema(example = json!(ImagePath::demo()))]
pub struct ImagePath {
    #[Demo(value = "1")]
    pub image_id: u32,
    #[Demo(value = r#"String::from("01J/G0/M004KYHATX7J2W7MB28X4.webp")"#)]
    pub path: String,
    #[Demo(value = r#"Some(String::from("Mass at back"))"#)]
    pub title: Option<String>,
    // #[Demo(value = r#"String::from("user")"#)]
    // pub create_user: String,
    #[Demo(value = "datetime!(2023-12-31 23:59:59)")]
    pub create_datetime: PrimitiveDateTime,

    // additional data
    /// MUST use in request
    #[Demo(value = "Some(1)")]
    pub image_usage_id: Option<u32>,
    /// MUST use in request
    #[Demo(value = r#"Some(ImageUsage::demo_ipd_progress_note())"#)]
    pub usage_id: Option<ImageUsage>,
    /// MUST use in request
    #[Demo(value = "Some(1)")]
    pub usage_key_id: Option<u32>,
    /// read only
    #[Demo(value = r#"Some(String::from("Dr.Doctor"))"#)]
    pub create_username: Option<String>,
}

impl ImagePath {
    /// GET `EndPoint::ImageUsageId`
    pub async fn call_api_get(usage_id: &ImageUsage, usage_key_id: u32, app: Rc<AppState>) -> Result<Vec<Self>, AppError> {
        match fetch_json_api(
            &[&EndPoint::ImageUsageId.base(), &(usage_id.clone() as u32).to_string(), "/", &usage_key_id.to_string()].concat(),
            "GET",
            None,
            app,
        )
        .await
        {
            Ok((response, true)) => {
                let response: Vec<Self> = serde_wasm_bindgen::from_value(response).map_err(|e| Source::SerdeWasm.to_teapot_error(e, "Fetch ImageUsage"))?;
                Ok(response)
            }
            Ok((app_error, false)) => {
                let error: AppError = serde_wasm_bindgen::from_value(app_error).map_err(|e| Source::SerdeWasm.to_teapot_error(e, "Fetch ImageUsage"))?;
                Err(error)
            }
            Err(e) => Err(Source::Js.to_teapot_error(e.dyn_ref::<JsString>().map(|s| s.into()).unwrap_or(String::from("fetch error")), "Fetch Json")),
        }
    }

    /// POST `EndPoint::Image`
    pub async fn call_api_post_files_returning(files: &[File], app: Rc<AppState>) -> Result<Vec<Self>, AppError> {
        let form_data = FormData::new().map_err(|e| {
            let message: String = e.dyn_ref::<JsString>().map(|s| s.into()).unwrap_or_default();
            AppError::new_teapot(&["Error new FromData: ", &message].concat(), "Post File Returning")
        })?;

        for (id, file) in files.iter().enumerate() {
            // prepare file
            let file_bytes = file_to_bytes(&file).await.map_err(|e| {
                let message: String = e.dyn_ref::<JsString>().map(|s| s.into()).unwrap_or_default();
                AppError::new_teapot(&["Error file_to_bytes (image): ", &message].concat(), "Post File Returning")
            })?;

            // create DynamicImage
            let is_pdf = file.type_() == "application/pdf";
            let any_image = if is_pdf { pdf_to_image(file_bytes)? } else { image_from_raw_image(&file_bytes, &file.name())? };
            let (image, thumb) = webp_creator(any_image, is_pdf)?;

            // create Blob
            let image_blob = bytes_to_blob(&image, "image/webp").map_err(|e| {
                let message: String = e.dyn_ref::<JsString>().map(|s| s.into()).unwrap_or_default();
                AppError::new_teapot(&["Error bytes_to_blob (image): ", &message].concat(), "Post File Returning")
            })?;
            let thumb_blob = bytes_to_blob(&thumb, "image/webp").map_err(|e| {
                let message: String = e.dyn_ref::<JsString>().map(|s| s.into()).unwrap_or_default();
                AppError::new_teapot(&["Error bytes_to_blob (thumb): ", &message].concat(), "Post File Returning")
            })?;

            // append blob to FormData
            let filename = [&id.to_string(), ".webp"].concat();
            form_data.append_with_blob_and_filename(PATH_PREFIX_IMAGE, &image_blob, &filename).map_err(|e| {
                let message: String = e.dyn_ref::<JsString>().map(|s| s.into()).unwrap_or_default();
                AppError::new_teapot(&["Error append_with_blob_and_filename (image): ", &message].concat(), "Post File Returning")
            })?;
            form_data.append_with_blob_and_filename(PATH_PREFIX_THUMB, &thumb_blob, &filename).map_err(|e| {
                let message: String = e.dyn_ref::<JsString>().map(|s| s.into()).unwrap_or_default();
                AppError::new_teapot(&["Error append_with_blob_and_filename (thumb): ", &message].concat(), "Post File Returning")
            })?;
        }

        match post_multipart(&EndPoint::Image.base(), &form_data, app).await {
            Ok((response, true)) => {
                let response: Vec<Self> = serde_wasm_bindgen::from_value(response).map_err(|e| Source::SerdeWasm.to_teapot_error(e, "Post ImageFile"))?;
                Ok(response)
            }
            Ok((app_error, false)) => {
                let error: AppError = serde_wasm_bindgen::from_value(app_error).map_err(|e| Source::SerdeWasm.to_teapot_error(e, "Post ImageFile"))?;
                Err(error)
            }
            Err(e) => Err(Source::Js.to_teapot_error(e.dyn_ref::<JsString>().map(|s| s.into()).unwrap_or(String::from("fetch error")), "Fetch Json")),
        }
    }

    /// POST `EndPoint::ImageUsage`
    pub async fn call_api_post(images: &[Self], app: Rc<AppState>) -> Result<ExecuteResponse, AppError> {
        let body_json = serde_json::to_string(images).map_err(|e| Source::SerdeJson.to_teapot_error(e, "Post ImageUsage"))?;
        let body = serde_wasm_bindgen::to_value(&body_json).map_err(|e| Source::SerdeWasm.to_teapot_error(e, "Post ImageUsage"))?;

        match fetch_json_api(&EndPoint::ImageUsage.base(), "POST", Some(&body), app).await {
            Ok((response, true)) => {
                let response: ExecuteResponse = serde_wasm_bindgen::from_value(response).map_err(|e| Source::SerdeWasm.to_teapot_error(e, "Post ImageUsage"))?;
                Ok(response)
            }
            Ok((app_error, false)) => {
                let error: AppError = serde_wasm_bindgen::from_value(app_error).map_err(|e| Source::SerdeWasm.to_teapot_error(e, "Post ImageUsage"))?;
                Err(error)
            }
            Err(e) => Err(Source::Js.to_teapot_error(e.dyn_ref::<JsString>().map(|s| s.into()).unwrap_or(String::from("fetch error")), "Fetch Json")),
        }
    }

    /// PATCH `EndPoint::Image`
    pub async fn call_api_patch(&self, app: Rc<AppState>) -> Result<ExecuteResponse, AppError> {
        let body_json = serde_json::to_string(self).map_err(|e| Source::SerdeJson.to_teapot_error(e, "Patch Image"))?;
        let body = serde_wasm_bindgen::to_value(&body_json).map_err(|e| Source::SerdeWasm.to_teapot_error(e, "Patch Image"))?;

        match fetch_json_api(&EndPoint::Image.base(), "PATCH", Some(&body), app).await {
            Ok((response, true)) => {
                let response: ExecuteResponse = serde_wasm_bindgen::from_value(response).map_err(|e| Source::SerdeWasm.to_teapot_error(e, "Patch Image"))?;
                Ok(response)
            }
            Ok((app_error, false)) => {
                let error: AppError = serde_wasm_bindgen::from_value(app_error).map_err(|e| Source::SerdeWasm.to_teapot_error(e, "Patch Image"))?;
                Err(error)
            }
            Err(e) => Err(Source::Js.to_teapot_error(e.dyn_ref::<JsString>().map(|s| s.into()).unwrap_or(String::from("fetch error")), "Fetch Json")),
        }
    }

    /// DELETE `EndPoint::ImageUsage`
    pub async fn call_api_delete(image_usage_id: &[u32], app: Rc<AppState>) -> Result<ExecuteResponse, AppError> {
        let body_json = serde_json::to_string(image_usage_id).map_err(|e| Source::SerdeJson.to_teapot_error(e, "Delete ImageUsage"))?;
        let body = serde_wasm_bindgen::to_value(&body_json).map_err(|e| Source::SerdeWasm.to_teapot_error(e, "Delete ImageUsage"))?;

        match fetch_json_api(&EndPoint::ImageUsage.base(), "DELETE", Some(&body), app).await {
            Ok((response, true)) => {
                let response: ExecuteResponse = serde_wasm_bindgen::from_value(response).map_err(|e| Source::SerdeWasm.to_teapot_error(e, "Delete ImageUsage"))?;
                Ok(response)
            }
            Ok((app_error, false)) => {
                let error: AppError = serde_wasm_bindgen::from_value(app_error).map_err(|e| Source::SerdeWasm.to_teapot_error(e, "Delete ImageUsage"))?;
                Err(error)
            }
            Err(e) => Err(Source::Js.to_teapot_error(e.dyn_ref::<JsString>().map(|s| s.into()).unwrap_or(String::from("fetch error")), "Fetch Json")),
        }
    }
}

impl PartialEq for ImagePath {
    fn eq(&self, other: &Self) -> bool {
        self.image_id == other.image_id
    }
}
