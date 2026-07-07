use bitcode::{Decode, Encode};
use derive_demo::Demo;
use js_sys::JsString;
use serde_derive::{Deserialize, Serialize};
use sqlx::{
    FromRow,
    types::{
        Decimal,
        time::{Date, Time},
    },
};
use std::rc::Rc;
use time::macros::{date, time};
use utoipa::{IntoParams, ToSchema};
use wasm_bindgen::JsCast;

use kphis_util::{
    error::{AppError, Source},
    // fuzzy_search::{fuzzy_compare, trigrams},
};

use crate::{
    app::AppState,
    endpoint::{EndPoint, QueryString, find_qs},
    fetch::fetch_json_api,
    ipd::summary::DxData,
};

/// Lab Search-Box
#[derive(Debug, Demo, Deserialize, Serialize, FromRow, ToSchema)]
#[schema(example = json!(LabSearchbox::demo()))]
pub struct LabSearchbox {
    #[Demo(value = r#"String::from("HEMATOLOGY")"#)]
    pub form_name: String,
    #[Demo(value = r#"Some(String::from("CBC"))"#)]
    pub component_caption: Option<String>,
    #[Demo(value = r#"Some(String::from("checkbox_group"))"#)]
    pub component_type: Option<String>,
    #[Demo(value = "Some(1)")]
    pub lab_items_code: Option<i32>,
}

impl LabSearchbox {
    /// GET `EndPoint::SearchBoxLabText`
    pub async fn call_api_get(search_text: &str, app: Rc<AppState>) -> Result<Vec<Self>, AppError> {
        match fetch_json_api(&[EndPoint::SearchBoxLabText.base(), urlencoding::encode(search_text).into_owned()].concat(), "GET", None, app).await {
            Ok((response, true)) => {
                let response: Vec<Self> = serde_wasm_bindgen::from_value(response).map_err(|e| Source::SerdeWasm.to_teapot_error(e, "Fetch LabSearchbox"))?;
                Ok(response)
            }
            Ok((app_error, false)) => {
                let error: AppError = serde_wasm_bindgen::from_value(app_error).map_err(|e| Source::SerdeWasm.to_teapot_error(e, "Fetch LabSearchbox"))?;
                Err(error)
            }
            Err(e) => Err(Source::Js.to_teapot_error(e.dyn_ref::<JsString>().map(|s| s.into()).unwrap_or(String::from("fetch error")), "Fetch Json")),
        }
    }
}

/// X-Ray Search-Box
#[derive(Debug, Demo, Deserialize, Serialize, FromRow, ToSchema)]
#[schema(example = json!(XraySearchbox::demo()))]
pub struct XraySearchbox {
    #[Demo(value = "1")]
    pub xray_items_code: i32,
    #[Demo(value = r#"Some(String::from("Film CXR (PA)"))"#)]
    pub xray_items_name: Option<String>,
    #[Demo(value = r#"Some(String::from("X-Ray"))"#)]
    pub group_name: Option<String>,
}

impl XraySearchbox {
    /// GET `EndPoint::SearchBoxXrayText`
    pub async fn call_api_get(search_text: &str, app: Rc<AppState>) -> Result<Vec<Self>, AppError> {
        match fetch_json_api(&[EndPoint::SearchBoxXrayText.base(), urlencoding::encode(search_text).into_owned()].concat(), "GET", None, app).await {
            Ok((response, true)) => {
                let response: Vec<Self> = serde_wasm_bindgen::from_value(response).map_err(|e| Source::SerdeWasm.to_teapot_error(e, "Fetch XraySearchbox"))?;
                Ok(response)
            }
            Ok((app_error, false)) => {
                let error: AppError = serde_wasm_bindgen::from_value(app_error).map_err(|e| Source::SerdeWasm.to_teapot_error(e, "Fetch XraySearchbox"))?;
                Err(error)
            }
            Err(e) => Err(Source::Js.to_teapot_error(e.dyn_ref::<JsString>().map(|s| s.into()).unwrap_or(String::from("fetch error")), "Fetch Json")),
        }
    }
}

/// IV-Fluid Search-Box
#[derive(Debug, Demo, Deserialize, Serialize, FromRow, ToSchema)]
#[schema(example = json!(IvfluidSearchbox::demo()))]
pub struct IvfluidSearchbox {
    #[Demo(value = r#"String::from("1000215")"#)]
    pub icode: String,
    #[Demo(value = r#"Some(String::from("NSS 0.9 % ขวด (1,000 ml.)"))"#)]
    pub ivfluid_name: Option<String>,
    #[Demo(value = "Some(0)")]
    pub displaycolor: Option<i32>,
}

impl IvfluidSearchbox {
    /// GET `EndPoint::SearchBoxIvfluidText`
    pub async fn call_api_get(search_text: &str, app: Rc<AppState>) -> Result<Vec<Self>, AppError> {
        match fetch_json_api(&[EndPoint::SearchBoxIvfluidText.base(), urlencoding::encode(search_text).into_owned()].concat(), "GET", None, app).await {
            Ok((response, true)) => {
                let response: Vec<Self> = serde_wasm_bindgen::from_value(response).map_err(|e| Source::SerdeWasm.to_teapot_error(e, "Fetch IvfluidSearchbox"))?;
                Ok(response)
            }
            Ok((app_error, false)) => {
                let error: AppError = serde_wasm_bindgen::from_value(app_error).map_err(|e| Source::SerdeWasm.to_teapot_error(e, "Fetch IvfluidSearchbox"))?;
                Err(error)
            }
            Err(e) => Err(Source::Js.to_teapot_error(e.dyn_ref::<JsString>().map(|s| s.into()).unwrap_or(String::from("fetch error")), "Fetch Json")),
        }
    }
}

/// Medication Search-Box
#[derive(Debug, Demo, Deserialize, Serialize, FromRow, ToSchema)]
#[schema(example = json!(MedSearchbox::demo()))]
pub struct MedSearchbox {
    #[Demo(value = r#"String::from("1000227")"#)]
    pub icode: String,
    #[Demo(value = r#"Some(String::from("PARACETAMOL 500 mg. เม็ด"))"#)]
    pub med_name: Option<String>,
    #[Demo(value = "Some(0)")]
    pub displaycolor: Option<i32>,
    #[Demo(value = r#"Some(String::from("TABLET"))"#)]
    pub dosageform: Option<String>,
    #[Demo(value = r#"Some(String::from("1"))"#)]
    pub drugusage: Option<String>,
    #[Demo(value = r#"Some(String::from("CrCl < 30 dose xx"))"#)]
    pub due_usage: Option<String>,
    #[Demo(value = r#"Some(String::from("Y"))"#)]
    pub due_status: Option<String>,
    #[Demo(value = r#"Some(String::from("รับประทานครั้งละ 1 เม็ด ทุก 6 ชั่วโมง"))"#)]
    pub usage: Option<String>,
    #[Demo(value = r#"Some(String::from("Use me gently"))"#)]
    pub info: Option<String>,
    #[Demo(value = r#"Some(String::from("Y"))"#)]
    pub info_status: Option<String>,
    #[Demo(value = r#"Some(String::from("PARACETAMOL"))"#)]
    pub generic_name: Option<String>,
    #[Demo(value = r#"Some(String::from("Y"))"#)]
    pub show_notify: Option<String>,
    #[Demo(value = r#"Some(String::from("***ALERT!!!***"))"#)]
    pub show_notify_text: Option<String>,
    #[Demo(value = r#"Some(String::from("PENICILLIN"))"#)]
    pub allergy_agent: Option<String>,
    #[Demo(value = r#"Some(String::from("PENICILLIN=Rash"))"#)]
    pub allergy_agent_symptom: Option<String>,
    #[Demo(value = "Decimal::new(0,0)")]
    pub allergy_count_force_no_order: Decimal,
}

impl MedSearchbox {
    /// GET `EndPoint::SearchBoxMedHnText`
    pub async fn call_api_get(hn: &str, search_text: &str, app: Rc<AppState>) -> Result<Vec<Self>, AppError> {
        match fetch_json_api(&[&EndPoint::SearchBoxMedHnText.base(), hn, "/", urlencoding::encode(search_text).as_ref()].concat(), "GET", None, app).await {
            Ok((response, true)) => {
                let response: Vec<Self> = serde_wasm_bindgen::from_value(response).map_err(|e| Source::SerdeWasm.to_teapot_error(e, "Fetch MedSearchbox"))?;
                Ok(response)
            }
            Ok((app_error, false)) => {
                let error: AppError = serde_wasm_bindgen::from_value(app_error).map_err(|e| Source::SerdeWasm.to_teapot_error(e, "Fetch MedSearchbox"))?;
                Err(error)
            }
            Err(e) => Err(Source::Js.to_teapot_error(e.dyn_ref::<JsString>().map(|s| s.into()).unwrap_or(String::from("fetch error")), "Fetch Json")),
        }
    }
}

// use in both `DrugDuplicateCheck` and `DrugInteractionCheck`
#[derive(Clone, Default, Deserialize, IntoParams)]
pub struct DrugCheckParams {
    pub an: Option<String>,
    pub generic_name: Option<String>,
    pub exclude_order_id: Option<u32>,
    pub off_order_item_ids: Option<String>, // comma delimited
    pub additional_icodes: Option<String>,  // comma delimited
}

impl QueryString for DrugCheckParams {
    fn from_tuples(params: &[(String, String)]) -> Option<Self> {
        (!params.is_empty()).then(|| Self {
            an: find_qs(params, "an"),
            generic_name: find_qs(params, "generic_name"),
            exclude_order_id: find_qs(params, "exclude_order_id").and_then(|s| s.parse::<u32>().ok()),
            off_order_item_ids: find_qs(params, "off_order_item_ids"),
            additional_icodes: find_qs(params, "additional_icodes"),
        })
    }

    fn query_string(&self) -> String {
        let mut queries = Vec::with_capacity(5);
        if let Some(an) = &self.an {
            queries.push(["an=", an].concat());
        }
        if let Some(generic_name) = &self.generic_name {
            queries.push(["generic_name=", generic_name].concat());
        }
        if let Some(exclude_order_id) = &self.exclude_order_id {
            queries.push(["exclude_order_id=", &exclude_order_id.to_string()].concat());
        }
        if let Some(off_order_item_ids) = &self.off_order_item_ids {
            queries.push(["off_order_item_ids=", off_order_item_ids].concat());
        }
        if let Some(additional_icodes) = &self.additional_icodes {
            queries.push(["additional_icodes=", additional_icodes].concat());
        }
        (!queries.is_empty()).then(|| ["?", &queries.join("&")].concat()).unwrap_or_default()
    }
}

/// Drug-Duplication Checker
#[derive(Clone, Demo, Deserialize, Serialize, FromRow, ToSchema)]
#[schema(example = json!(DrugDuplicateCheck::demo()))]
pub struct DrugDuplicateCheck {
    #[Demo(value = r#"Some(String::from("1000227"))"#)]
    pub icode: Option<String>,
    #[Demo(value = r#"Some(String::from("PARACETAMOL 500 mg. เม็ด"))"#)]
    pub med_name: Option<String>,
    #[Demo(value = r#"Some(String::from("รับประทานครั้งละ1 เม็ด ทุก 6 ชั่วโมง"))"#)]
    pub order_item_detail: Option<String>,
    #[Demo(value = "Some(date!(2023-12-31))")]
    pub order_date: Option<Date>,
    #[Demo(value = "Some(time!(23:59:59))")]
    pub order_time: Option<Time>,
}

impl DrugDuplicateCheck {
    /// GET `EndPoint::SearchBoxMedDuplicate`
    pub async fn call_api_get(params: &DrugCheckParams, app: Rc<AppState>) -> Result<Vec<Self>, AppError> {
        match fetch_json_api(&[EndPoint::SearchBoxMedDuplicate.base(), params.clone().query_string()].concat(), "GET", None, app).await {
            Ok((response, true)) => {
                let response: Vec<Self> = serde_wasm_bindgen::from_value(response).map_err(|e| Source::SerdeWasm.to_teapot_error(e, "Fetch DrugDuplicateCheck"))?;
                Ok(response)
            }
            Ok((app_error, false)) => {
                let error: AppError = serde_wasm_bindgen::from_value(app_error).map_err(|e| Source::SerdeWasm.to_teapot_error(e, "Fetch DrugDuplicateCheck"))?;
                Err(error)
            }
            Err(e) => Err(Source::Js.to_teapot_error(e.dyn_ref::<JsString>().map(|s| s.into()).unwrap_or(String::from("fetch error")), "Fetch Json")),
        }
    }
}

/// Drug-Interaction Checker
#[derive(Clone, Demo, Deserialize, Serialize, FromRow, ToSchema)]
#[schema(example = json!(DrugInteractionCheck::demo()))]
pub struct DrugInteractionCheck {
    #[Demo(value = r#"Some(String::from("PARACETAMOL"))"#)]
    pub drugname1: Option<String>,
    #[Demo(value = r#"Some(String::from("WARFARIN"))"#)]
    pub drugname2: Option<String>,
    #[Demo(value = r#"Some(String::from("Note"))"#)]
    pub note: Option<String>,
    #[Demo(value = "Some(1)")]
    pub severity: Option<i32>,
    #[Demo(value = r#"Some(String::from("Y"))"#)]
    pub not_allow: Option<String>,
}

impl DrugInteractionCheck {
    /// GET `EndPoint::SearchBoxMedInteraction`
    pub async fn call_api_get(params: &DrugCheckParams, app: Rc<AppState>) -> Result<Vec<Self>, AppError> {
        match fetch_json_api(&[EndPoint::SearchBoxMedInteraction.base(), params.clone().query_string()].concat(), "GET", None, app).await {
            Ok((response, true)) => {
                let response: Vec<Self> = serde_wasm_bindgen::from_value(response).map_err(|e| Source::SerdeWasm.to_teapot_error(e, "Fetch DrugInteractionCheck"))?;
                Ok(response)
            }
            Ok((app_error, false)) => {
                let error: AppError = serde_wasm_bindgen::from_value(app_error).map_err(|e| Source::SerdeWasm.to_teapot_error(e, "Fetch DrugInteractionCheck"))?;
                Err(error)
            }
            Err(e) => Err(Source::Js.to_teapot_error(e.dyn_ref::<JsString>().map(|s| s.into()).unwrap_or(String::from("fetch error")), "Fetch Json")),
        }
    }
}

/// Patient Search-Box
#[derive(Debug, Demo, Deserialize, Serialize, FromRow, ToSchema)]
#[schema(example = json!(PatientSearchbox::demo()))]
pub struct PatientSearchbox {
    #[Demo(value = r#"Some(String::from("0001234"))"#)]
    pub hn: Option<String>,
    #[Demo(value = r#"Some(String::from("Mr.Patient"))"#)]
    pub ptname: Option<String>,
    #[Demo(value = r#"Some(String::from("Mr.Father"))"#)]
    pub fathername: Option<String>,
    #[Demo(value = r#"Some(String::from("Mrs.Mother"))"#)]
    pub mathername: Option<String>,
    #[Demo(value = r#"Some(String::from("Y"))"#)]
    pub admit: Option<String>,
    #[Demo(value = r#"Some(String::from("ABC123"))"#)]
    pub passport_no: Option<String>,
    #[Demo(value = r#"Some(String::from("1111111111111"))"#)]
    pub cid: Option<String>,
}

impl PatientSearchbox {
    /// GET `EndPoint::SearchBoxPatientText`
    pub async fn call_api_get(search_text: &str, app: Rc<AppState>) -> Result<Vec<Self>, AppError> {
        match fetch_json_api(&[EndPoint::SearchBoxPatientText.base(), urlencoding::encode(search_text).into_owned()].concat(), "GET", None, app).await {
            Ok((response, true)) => {
                let response: Vec<Self> = serde_wasm_bindgen::from_value(response).map_err(|e| Source::SerdeWasm.to_teapot_error(e, "Fetch PatientSearchbox"))?;
                Ok(response)
            }
            Ok((app_error, false)) => {
                let error: AppError = serde_wasm_bindgen::from_value(app_error).map_err(|e| Source::SerdeWasm.to_teapot_error(e, "Fetch PatientSearchbox"))?;
                Err(error)
            }
            Err(e) => Err(Source::Js.to_teapot_error(e.dyn_ref::<JsString>().map(|s| s.into()).unwrap_or(String::from("fetch error")), "Fetch Json")),
        }
    }
}

/// OPD-Visit Search-Box
#[derive(Debug, Demo, Deserialize, Serialize, FromRow, ToSchema)]
#[schema(example = json!(OpdVisitSearchbox::demo()))]
pub struct OpdVisitSearchbox {
    #[Demo(value = r#"Some(String::from("661231235959"))"#)]
    pub vn: Option<String>,
    #[Demo(value = r#"Some(String::from("0001234"))"#)]
    pub hn: Option<String>,
    #[Demo(value = "Some(date!(2023-12-31))")]
    pub vstdate: Option<Date>,
    #[Demo(value = "Some(time!(23:59:59))")]
    pub vsttime: Option<Time>,
    #[Demo(value = r#"Some(String::from("Mr.Patient"))"#)]
    pub ptname: Option<String>,
    #[Demo(value = r#"Some(String::from("Mr.Father"))"#)]
    pub fathername: Option<String>,
    #[Demo(value = r#"Some(String::from("Mrs.Mother"))"#)]
    pub mathername: Option<String>,
    #[Demo(value = r#"Some(String::from("Y"))"#)]
    pub admit: Option<String>,
    #[Demo(value = r#"Some(String::from("ABC123"))"#)]
    pub passport_no: Option<String>,
    #[Demo(value = r#"Some(String::from("1111111111111"))"#)]
    pub cid: Option<String>,
}

impl OpdVisitSearchbox {
    /// GET `EndPoint::SearchBoxOpdVisitModeText`
    pub async fn call_api_get(search_text: &str, mode: &str, app: Rc<AppState>) -> Result<Vec<Self>, AppError> {
        match fetch_json_api(
            &[&EndPoint::SearchBoxOpdVisitModeText.base(), mode, "/", urlencoding::encode(search_text).as_ref()].concat(),
            "GET",
            None,
            app,
        )
        .await
        {
            Ok((response, true)) => {
                let response: Vec<Self> = serde_wasm_bindgen::from_value(response).map_err(|e| Source::SerdeWasm.to_teapot_error(e, "Fetch OpdVisitSearchbox"))?;
                Ok(response)
            }
            Ok((app_error, false)) => {
                let error: AppError = serde_wasm_bindgen::from_value(app_error).map_err(|e| Source::SerdeWasm.to_teapot_error(e, "Fetch OpdVisitSearchbox"))?;
                Err(error)
            }
            Err(e) => Err(Source::Js.to_teapot_error(e.dyn_ref::<JsString>().map(|s| s.into()).unwrap_or(String::from("fetch error")), "Fetch Json")),
        }
    }
}

#[derive(Clone, Default, PartialEq)]
pub enum OpdVisitSearchType {
    #[default]
    All,
    Hn,
    Qn,
    Vn,
    PtName,
    Cid,
}
impl OpdVisitSearchType {
    pub fn new(mode_text: &str) -> Self {
        match mode_text {
            "hn" => OpdVisitSearchType::Hn,
            "qn" => OpdVisitSearchType::Qn,
            "vn" => OpdVisitSearchType::Vn,
            "pt-name" => OpdVisitSearchType::PtName,
            "cid" => OpdVisitSearchType::Cid,
            _ => OpdVisitSearchType::All,
        }
    }
    pub fn string(&self) -> &'static str {
        match self {
            OpdVisitSearchType::Hn => "hn",
            OpdVisitSearchType::Qn => "qn",
            OpdVisitSearchType::Vn => "vn",
            OpdVisitSearchType::PtName => "pt-name",
            OpdVisitSearchType::Cid => "cid",
            OpdVisitSearchType::All => "all",
        }
    }
    pub fn not_all(&self) -> bool {
        !matches!(self, OpdVisitSearchType::All)
    }
    pub fn wildcard(&self, search_text: &str) -> String {
        let text = search_text.trim();
        match self {
            OpdVisitSearchType::Hn => ["%", text].concat(),
            OpdVisitSearchType::Qn => text.to_owned(),
            OpdVisitSearchType::Vn => ["%", text].concat(),
            OpdVisitSearchType::PtName => ["%", text, "%"].concat(),
            OpdVisitSearchType::Cid => ["%", text, "%"].concat(),
            OpdVisitSearchType::All => text.to_owned(),
        }
    }
}

/// ICD10 for fuzzy search
#[derive(Clone, Debug, Default, Demo, Decode, Encode, Hash, FromRow, Serialize, ToSchema)]
#[schema(example = json!(Icd10::demo()))]
pub struct Icd10 {
    /// ICD10 uppercase without dot or `???`
    #[Demo(value = r#"String::from("I10")"#)]
    pub icd10: String,
    #[Demo(value = r#"Some(String::from("Essential (primary) hypertension"))"#)]
    pub ename: Option<String>,
    // #[Demo(value = r#"Some(String::from("โรคความดันโลหิตสูง"))"#)]
    // pub tname: Option<String>,
    // #[Demo(value = r#"Some(String::from("Hypertension"))"#)]
    // pub sumdx_keyword: Option<String>,
    // #[Demo(value = r#"Some(String::from("ESSENTIAL HYPERTENSION (ความดันโลหิตสูงที่ไม่มีสาเหตุนำ)"))"#)]
    // pub icd_codemap_code: Option<String>,
}

impl PartialEq for Icd10 {
    fn eq(&self, other: &Self) -> bool {
        self.icd10 == other.icd10 && self.ename == other.ename
    }
}

impl From<&DxData> for Icd10 {
    fn from(item: &DxData) -> Self {
        Self {
            icd10: item.icd.clone().unwrap_or(String::from("???")),
            ename: item.detail.clone(),
            // tname: None,
            // sumdx_keyword: None,
            // icd_codemap_code: None,
        }
    }
}

impl Icd10 {
    pub fn new(icd10: &Option<String>, ename: &Option<String>) -> Option<Rc<Self>> {
        ename.is_some().then(|| {
            Rc::new(Self {
                icd10: icd10.to_owned().unwrap_or(String::from("???")),
                ename: ename.clone(),
                // tname: None,
                // sumdx_keyword: None,
                // icd_codemap_code: None,
            })
        })
    }

    // pub fn new_owned(icd10: &Option<String>, ename: &Option<String>) -> Option<Self> {
    //     ename.is_some().then(|| {
    //         Self {
    //             icd10: icd10.to_owned().unwrap_or(String::from("???")),
    //             ename: ename.clone(),
    //             // tname: None,
    //             // sumdx_keyword: None,
    //             // icd_codemap_code: None,
    //         }
    //     })
    // }

    pub fn from_raw(code: &str, desc: &str) -> Option<Rc<Self>> {
        (!desc.is_empty()).then(|| {
            let icd10 = if code.is_empty() { String::from("???") } else { code.to_owned() };
            Rc::new(Self { icd10, ename: Some(desc.to_owned()) })
        })
    }

    // pub fn search_icd10_with_prefix(prefix: &str, list: &[Self]) -> Vec<Rc<(Self, f32, u8)>> {
    //     if prefix.len() > 2 {
    //         let mut result = list.iter().filter(|value| value.icd10.starts_with(prefix)).collect::<Vec<&Self>>();
    //         result.sort_by(|a, b| a.icd10.cmp(&b.icd10));
    //         result.into_iter().map(|item| Rc::new((item.to_owned(), 1.0, 1))).collect()
    //     } else {
    //         Vec::new()
    //     }
    // }

    // pub fn search_icd10_exact(icd10: &str, list: &[Self]) -> Option<Self> {
    //     list.iter().find(|value| value.icd10.as_str() == icd10).cloned()
    // }

    // /// return (Self, weight, column)
    // pub fn fuzzy_search_best_n(s: &str, list: &[Self], n: usize) -> Vec<Rc<(Self, f32, u8)>> {
    //     let chars_count = s.chars().count() + 1;
    //     let trigrams_a = trigrams(s);
    //     let mut res = list
    //         .iter()
    //         .filter_map(|value| {
    //             let mut res = 0.0;
    //             let mut col = 0;
    //             let icd10 = value.icd10.as_str();
    //             if icd10.chars().count() > 0 {
    //                 let r = fuzzy_compare(&trigrams_a, chars_count, icd10);
    //                 if r > res {
    //                     res = r;
    //                     col = 1;
    //                 }
    //             }
    //             if res < 1.0 {
    //                 if let Some(ename) = value.ename.as_ref() {
    //                     if ename.chars().count() > 0 {
    //                         let r = fuzzy_compare(&trigrams_a, chars_count, ename);
    //                         if r > res {
    //                             res = r;
    //                             col = 2;
    //                         }
    //                     }
    //                 }
    //             }
    //             if res < 1.0 {
    //                 if let Some(tname) = value.tname.as_ref() {
    //                     if tname.chars().count() > 0 {
    //                         let r = fuzzy_compare(&trigrams_a, chars_count, tname);
    //                         if r > res {
    //                             res = r;
    //                             col = 3;
    //                         }
    //                     }
    //                 }
    //             }
    //             if res < 1.0 {
    //                 if let Some(sumdx_keyword) = value.sumdx_keyword.as_ref() {
    //                     if sumdx_keyword.chars().count() > 0 {
    //                         let r = fuzzy_compare(&trigrams_a, chars_count, sumdx_keyword);
    //                         if r > res {
    //                             res = r;
    //                             col = 4;
    //                         }
    //                     }
    //                 }
    //             }
    //             if res < 1.0 {
    //                 if let Some(icd_codemap_code) = value.icd_codemap_code.as_ref() {
    //                     if icd_codemap_code.chars().count() > 0 {
    //                         let r = fuzzy_compare(&trigrams_a, chars_count, icd_codemap_code);
    //                         if r > res {
    //                             res = r;
    //                             col = 5;
    //                         }
    //                     }
    //                 }
    //             }
    //             (res > 0.0).then(|| (value.clone(), res, col))
    //         })
    //         .collect::<Vec<(Self, f32, u8)>>();
    //     res.sort_by(|(_, d1, _), (_, d2, _)| d2.total_cmp(d1));
    //     res.into_iter().take(n).map(Rc::new).collect()
    // }
}

/// Hospital Search-Box
#[derive(Clone, Debug, Demo, Deserialize, Serialize, FromRow, ToSchema)]
#[schema(example = json!(HospSearchBox::demo()))]
pub struct HospSearchBox {
    #[Demo(value = r#"String::from("11111")"#)]
    pub id: String,
    // pub text: Option<String>,
    #[Demo(value = r#"Some(String::from("โรงพยาบาลชุมชน"))"#)]
    pub hosptype: Option<String>,
    #[Demo(value = r#"Some(String::from("Best Hospital"))"#)]
    pub hospname: Option<String>,
    #[Demo(value = r#"Some(String::from("Thailand"))"#)]
    pub addrname: Option<String>,
}

impl HospSearchBox {
    pub fn new(code: &Option<String>, hosp_type: &Option<String>, hosp_name: &Option<String>) -> Option<Rc<Self>> {
        code.clone().map(|id| {
            Rc::new(Self {
                id,
                hosptype: hosp_type.clone(),
                hospname: hosp_name.clone(),
                addrname: None,
            })
        })
    }

    pub fn text(&self) -> String {
        [self.id.clone(), self.hosptype.clone().unwrap_or_default(), self.hospname.clone().unwrap_or_default()].join(" ")
    }

    /// GET `EndPoint::SearchBoxHospText`
    pub async fn call_api_get(search_text: &str, app: Rc<AppState>) -> Result<Vec<Self>, AppError> {
        match fetch_json_api(&[EndPoint::SearchBoxHospText.base(), urlencoding::encode(search_text).into_owned()].concat(), "GET", None, app).await {
            Ok((response, true)) => {
                let response: Vec<Self> = serde_wasm_bindgen::from_value(response).map_err(|e| Source::SerdeWasm.to_teapot_error(e, "Fetch HospSearchbox"))?;
                Ok(response)
            }
            Ok((app_error, false)) => {
                let error: AppError = serde_wasm_bindgen::from_value(app_error).map_err(|e| Source::SerdeWasm.to_teapot_error(e, "Fetch HospSearchbox"))?;
                Err(error)
            }
            Err(e) => Err(Source::Js.to_teapot_error(e.dyn_ref::<JsString>().map(|s| s.into()).unwrap_or(String::from("fetch error")), "Fetch Json")),
        }
    }
}

/// Drug Usage from HOSxP
#[derive(Clone, Debug, Demo, Decode, Encode, Hash, FromRow, Serialize, ToSchema)]
#[schema(example = json!(DrugUsage::demo()))]
pub struct DrugUsage {
    #[Demo(value = r#"String::from("1")"#)]
    pub drugusage: String,
    #[Demo(value = r#"Some(String::from("13PT"))"#)]
    pub code: Option<String>,
    #[Demo(value = r#"Some(String::from("รับประทาน ครั้งละ 1 เม็ด"))"#)]
    pub name1: Option<String>,
    #[Demo(value = r#"Some(String::from("วันละ 3 เวลา"))"#)]
    pub name2: Option<String>,
    #[Demo(value = r#"Some(String::from("หลังอาหาร เช้า - กลางวัน - เย็น"))"#)]
    pub name3: Option<String>,
}

impl PartialEq for DrugUsage {
    fn eq(&self, other: &Self) -> bool {
        self.drugusage == other.drugusage
    }
}
