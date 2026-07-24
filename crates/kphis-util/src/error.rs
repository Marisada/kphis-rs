use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde_derive::{Deserialize, Serialize};

#[cfg(not(all(target_arch = "wasm32", any(target_os = "unknown", target_os = "none"))))]
use ulid::Ulid;

#[cfg(all(target_arch = "wasm32", any(target_os = "unknown", target_os = "none")))]
use crate::datetime::js_now;

pub const CONTACT_ADMIN: &str = "เกิดข้อผิดพลาด! กรุณาติดต่อผู้ดูแลระบบ";

/// Sources of error
#[derive(Debug, Deserialize, Serialize, PartialEq, utoipa::ToSchema)]
#[schema(example = "App")]
pub enum Source {
    App,
    Base64,
    BitCode,
    CmdClient,
    Cms,
    Decimal,
    Hayro,
    Image,
    Io,
    Js,
    Orion,
    ParseBool,
    ParseFloat,
    ParseInt,
    Pasetors,
    PasswordHash,
    PacsClient,
    SerdeJson,
    SerdeWasm,
    SQLx,
    SystemTime,
    Time,
    Totp,
    Typst,
    UlidDecode,
    UrlEncoding,
    X509,
}

impl Source {
    /// error with `ErrorTitle::ContactAdmin`
    pub fn to_error<S>(self, status: u16, message: S, action: &str) -> AppError
    where
        S: std::string::ToString,
    {
        AppError {
            status,
            error_id: new_eror_id(),
            source: self,
            message: message.to_string(),
            action: action.to_owned(),
            title: ErrorTitle::ContactAdmin,
        }
    }
    /// error with `IM_A_TEAPOT(418)`, `ErrorTitle::ContactAdmin`
    pub fn to_teapot_error<S>(self, message: S, action: &str) -> AppError
    where
        S: std::string::ToString,
    {
        AppError {
            status: StatusCode::IM_A_TEAPOT.as_u16(),
            error_id: new_eror_id(),
            source: self,
            message: message.to_string(),
            action: action.to_owned(),
            title: ErrorTitle::ContactAdmin,
        }
    }

    pub fn string(&self) -> &'static str {
        // everyting about Cipher/Security algorithm use `CryptoXXX` to hide information
        match self {
            Self::App => "App",
            Self::Base64 => "Base64",
            Self::BitCode => "BitCode",
            Self::Cms => "CryptoPdf",
            Self::CmdClient => "CommandClient",
            Self::Decimal => "Decimal",
            Self::Hayro => "Hayro",
            Self::Io => "IO",
            Self::Image => "Image",
            Self::Js => "Js",
            Self::Orion => "CryptoSeal",
            Self::PacsClient => "PACsClient",
            Self::ParseBool => "ParseBool",
            Self::ParseFloat => "ParseFloat",
            Self::ParseInt => "ParseInt",
            Self::Pasetors => "CryptoToken",
            Self::PasswordHash => "PasswordHash",
            Self::SerdeJson => "SerdeJson",
            Self::SerdeWasm => "SerdeWasm",
            Self::SQLx => "SQLx",
            Self::SystemTime => "SystemTime",
            Self::Time => "Time",
            Self::Totp => "TOTP",
            Self::Typst => "Typst",
            Self::UlidDecode => "UlidDecode",
            Self::UrlEncoding => "UrlEncoding",
            Self::X509 => "CryptoPdf",
        }
    }
}

/// Title of error
#[derive(Debug, Deserialize, Serialize, PartialEq, utoipa::ToSchema)]
#[schema(example = "ContactAdmin")]
pub enum ErrorTitle {
    ContactAdmin,
    PreAdmitAdmited,
    AdmitRevoked,
    NoUserState,
    Security,
}

impl ErrorTitle {
    pub fn string(&self) -> &'static str {
        match self {
            Self::ContactAdmin => CONTACT_ADMIN,
            Self::PreAdmitAdmited => "Admit ใน HOSxP แล้ว",
            Self::AdmitRevoked => "ยกเลิก Admit ใน HOSxP แล้ว",
            Self::NoUserState => "ท่านออกจากระบบแล้ว",
            Self::Security => "ระบบความปลอดภัย",
        }
    }
}

/// API error message
#[derive(Debug, Deserialize, Serialize, utoipa::ToSchema)]
pub struct AppError {
    pub status: u16,
    /// A unique error ID
    #[schema(example = "01ARZ3NDEKTSV4RRFFQ69G5FAV")]
    pub error_id: String,
    /// Source of error
    pub source: Source,
    /// Error message for technician
    #[schema(example = "Tea Pot Broken")]
    pub message: String,
    /// Action that generate this error
    #[schema(example = "MakeTea")]
    pub action: String,
    /// Title message for client
    #[schema(example = ErrorTitle::ContactAdmin)]
    pub title: ErrorTitle,
}

impl AppError {
    /// error with `Source::App`, `ErrorTitle::ContactAdmin`<br>
    /// if need another `Source`, use `Source::to_error()` instead
    pub fn new_server(status: u16, message: &str, action: &str) -> Self {
        Self {
            status,
            error_id: new_eror_id(),
            source: Source::App,
            message: message.to_owned(),
            action: action.to_owned(),
            title: ErrorTitle::ContactAdmin,
        }
    }
    /// error with `Source::App`, `IM_A_TEAPOT(418)`, `ErrorTitle::ContactAdmin`<br>
    /// if need another `Source`, use `Source::to_teapot_error()` instead
    pub fn new_teapot(message: &str, action: &str) -> Self {
        Self {
            status: StatusCode::IM_A_TEAPOT.as_u16(),
            error_id: new_eror_id(),
            source: Source::App,
            message: message.to_owned(),
            action: action.to_owned(),
            title: ErrorTitle::ContactAdmin,
        }
    }
    /// error with `BAD_REQUEST`, `Source::App`, `ErrorTitle::ContactAdmin`<br>
    /// if need another `Source`, use `Source::to_error().with_code(StatusCode::BAD_REQUEST)` instead
    pub fn app_400(action: &str) -> Self {
        Source::App.to_error(400, "ข้อมูลที่ส่ง ไม่ถูกต้อง", action)
    }
    /// error with `UNAUTHORIZED`, `Source::App`, `ErrorTitle::ContactAdmin`<br>
    /// if need another `Source`, use `Source::to_error().with_code(StatusCode::UNAUTHORIZED)` instead
    pub fn app_401(action: &str) -> Self {
        Source::App.to_error(401, "ท่านไม่ได้รับอนุญาต", action)
    }
    /// error with `FORBIDDEN`, `Source::App`, `ErrorTitle::ContactAdmin`<br>
    /// if need another `Source`, use `Source::to_error().with_code(StatusCode::FORBIDDEN)` instead
    pub fn app_403(action: &str) -> Self {
        Source::App.to_error(403, "ท่านไม่มีสิทธิ์เข้าถึงข้อมูล", action)
    }
    /// error with `FORBIDDEN`, `Source::App`, `ErrorTitle::ContactAdmin`<br>
    /// if need another `Source`, use `Source::to_error().with_code(StatusCode::FORBIDDEN)` instead
    pub fn app_403_duplicate(action: &str) -> Self {
        Source::App.to_error(403, "ข้อมูลที่ส่ง ไม่สามารถสร้างซ้ำกันได้ อาจมีผู้อื่นส่งข้อมูลชุดเดียวกันมาก่อนแล้ว กรุณาย้อนกลับ แล้วสร้างใหม่อีกครั้ง", action)
    }
    /// error with `NOT_FOUND`, `Source::App`, `ErrorTitle::ContactAdmin`<br>
    /// if need another `Source`, use `Source::to_error().with_code(StatusCode::NOT_FOUND)` instead
    pub fn app_404(action: &str) -> Self {
        Source::App.to_error(404, "ไม่พบข้อมูลที่ต้องการ", action)
    }

    pub fn with_code(mut self, status: StatusCode) -> Self {
        self.status = status.as_u16();
        self
    }

    pub fn with_title(mut self, title: ErrorTitle) -> Self {
        self.title = title;
        self
    }

    pub fn message(&self) -> String {
        if self.status == 418 { ["(Client)", &self.message].concat() } else { self.message.clone() }
    }

    pub fn string(&self) -> String {
        // serde_json::to_string(self).unwrap_or_default()
        ["source: ", self.source.string(), ", \naction: ", &self.action, ", \nmessage: ", &self.message].concat()
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        (StatusCode::from_u16(self.status).unwrap_or(StatusCode::INTERNAL_SERVER_ERROR), Json(self)).into_response()
    }
}

#[cfg(not(all(target_arch = "wasm32", any(target_os = "unknown", target_os = "none"))))]
fn new_eror_id() -> String {
    Ulid::generate().to_string()
}

#[cfg(all(target_arch = "wasm32", any(target_os = "unknown", target_os = "none")))]
fn new_eror_id() -> String {
    js_now().to_string()
}
