use derive_demo::Demo;
use js_sys::JsString;
use serde_derive::{Deserialize, Serialize};
use std::rc::Rc;
use utoipa::{IntoParams, ToSchema};
use wasm_bindgen::JsCast;

use kphis_util::error::{AppError, Source};

use crate::{
    app::{AppState, VisitTypeId},
    endpoint::{EndPoint, QueryString, find_qs},
    fetch::fetch_json_api,
};

pub enum AvatarEnum {
    Ipd(AvatarWard),
    OpdEr(AvatarOpdEr),
}

impl AvatarEnum {
    pub fn visit_type(&self, an_len: usize) -> VisitTypeId {
        match self {
            AvatarEnum::Ipd(ward) => ward.visit_type(an_len),
            AvatarEnum::OpdEr(opd_er) => opd_er.visit_type(),
        }
    }
}

impl From<&AvatarWard> for AvatarEnum {
    fn from(image: &AvatarWard) -> Self {
        Self::Ipd(image.clone())
    }
}

impl From<&AvatarOpdEr> for AvatarEnum {
    fn from(image: &AvatarOpdEr) -> Self {
        Self::OpdEr(image.clone())
    }
}

#[derive(Clone, Default, Deserialize, IntoParams)]
pub struct AvatarParams {
    pub search: Option<String>,
    pub ward: Option<String>,
}

impl AvatarParams {
    pub fn is_empty(&self) -> bool {
        self.search.is_none() && self.ward.is_none()
    }
}

impl QueryString for AvatarParams {
    fn from_tuples(params: &[(String, String)]) -> Option<Self> {
        (!params.is_empty()).then(|| Self {
            search: find_qs(params, "search"),
            ward: find_qs(params, "ward"),
        })
    }

    fn query_string(&self) -> String {
        let mut queries = Vec::with_capacity(2);
        if let Some(search) = &self.search {
            queries.push(["search=", search].concat());
        }
        if let Some(ward) = &self.ward {
            queries.push(["ward=", ward].concat());
        }

        (!queries.is_empty()).then(|| ["?", &queries.join("&")].concat()).unwrap_or_default()
    }
}

/// IPD Patient's avatar data
#[derive(Clone, Debug, Demo, Deserialize, Serialize, ToSchema)]
#[schema(example = json!(AvatarWard::demo()))]
pub struct AvatarWard {
    #[Demo(value = r#"String::from("660001234")"#)]
    pub an: String,
    #[Demo(value = r#"Some(String::from("0001234"))"#)]
    pub hn: Option<String>,
    #[Demo(value = r#"Some(String::from("C01"))"#)]
    pub bedno: Option<String>,
    #[Demo(value = r#"Some(String::from("Mr Any User"))"#)]
    pub pname: Option<String>,
    #[Demo(value = "true")]
    pub discharge_order_exists: bool,
}

impl AvatarWard {
    /// GET `EndPoint::AvatarIpd`<br>
    /// get patient in selected ward/search
    pub async fn call_api_get(params: &AvatarParams, app: Rc<AppState>) -> Result<Vec<Self>, AppError> {
        match fetch_json_api(&[EndPoint::AvatarIpd.base(), params.query_string()].concat(), "GET", None, app).await {
            Ok((response, true)) => {
                let response: Vec<Self> = serde_wasm_bindgen::from_value(response).map_err(|e| Source::SerdeWasm.to_teapot_error(e, "Fetch AvatarIpd"))?;
                Ok(response)
            }
            Ok((app_error, false)) => {
                let error: AppError = serde_wasm_bindgen::from_value(app_error).map_err(|e| Source::SerdeWasm.to_teapot_error(e, "Fetch AvatarIpd"))?;
                Err(error)
            }
            Err(e) => Err(Source::Js.to_teapot_error(e.dyn_ref::<JsString>().map(|s| s.into()).unwrap_or(String::from("fetch error")), "Fetch Json")),
        }
    }

    pub fn visit_type(&self, an_len: usize) -> VisitTypeId {
        let an = self.an.to_owned();
        if an.len() == an_len { VisitTypeId::Ipd(an) } else { VisitTypeId::PreAdmit(an) }
    }
}

/// OPD-ER Patient's avatar data
#[derive(Clone, Debug, Demo, Deserialize, Serialize, ToSchema)]
#[schema(example = json!(AvatarOpdEr::demo()))]
pub struct AvatarOpdEr {
    #[Demo(value = "1")]
    pub opd_er_order_master_id: u32,
    #[Demo(value = r#"Some(String::from("661231235959"))"#)]
    pub vn: Option<String>,
    #[Demo(value = r#"Some(String::from("0001234"))"#)]
    pub hn: Option<String>,
    #[Demo(value = r#"Some(String::from("C01"))"#)]
    pub display_bedno: Option<String>,
    #[Demo(value = r#"Some(String::from("B"))"#)]
    pub bed_type_name: Option<String>,
    #[Demo(value = r##"Some(String::from("#98c2de"))"##)]
    pub bed_type_color: Option<String>,
    #[Demo(value = r#"Some(String::from("Mr Any User"))"#)]
    pub pname: Option<String>,
}

impl AvatarOpdEr {
    /// GET `EndPoint::AvatarOpdEr`<br>
    /// get patient in er with image as 'data:image/png;base64,xxx' string
    pub async fn call_api_get(app: Rc<AppState>) -> Result<Vec<Self>, AppError> {
        match fetch_json_api(&EndPoint::AvatarOpdEr.base(), "GET", None, app).await {
            Ok((response, true)) => {
                let response: Vec<Self> = serde_wasm_bindgen::from_value(response).map_err(|e| Source::SerdeWasm.to_teapot_error(e, "Fetch AvatarOpdEr"))?;
                Ok(response)
            }
            Ok((app_error, false)) => {
                let error: AppError = serde_wasm_bindgen::from_value(app_error).map_err(|e| Source::SerdeWasm.to_teapot_error(e, "Fetch AvatarOpdEr"))?;
                Err(error)
            }
            Err(e) => Err(Source::Js.to_teapot_error(e.dyn_ref::<JsString>().map(|s| s.into()).unwrap_or(String::from("fetch error")), "Fetch Json")),
        }
    }

    pub fn visit_type(&self) -> VisitTypeId {
        VisitTypeId::OpdEr(self.vn.clone().unwrap_or_default(), self.opd_er_order_master_id)
    }
}
