pub mod antibiogram;
pub mod app;
pub mod avatar;
pub mod claim;
pub mod dc_plan;
pub mod drug_use_duration;
pub mod emr;
pub mod endpoint;
pub mod fetch;
pub mod focus_list;
pub mod focus_note;
pub mod image;
pub mod index_action;
pub mod index_monitor;
pub mod index_plan;
pub mod ipd;
pub mod lab;
pub mod med_reconcile;
pub mod opd_er;
pub mod order;
pub mod pacs;
pub mod patient_info;
pub mod post_admit;
pub mod pre_admit;
pub mod pre_order;
pub mod prescription;
pub mod progress_note;
pub mod refer_note;
pub mod refer_out;
pub mod report;
pub mod route;
pub mod score;
pub mod search;
pub mod select_utils;
pub mod shift;
pub mod sse;
pub mod tab;
pub mod timer;
pub mod transform;
pub mod user;
pub mod vital_sign;
pub mod xray;

pub const API_PREFIX: &str = "/api";
pub const SSE_GET_PREFIX: &str = "/sse";
pub const CUSTOM_REPORT_PREFIX: &str = "/customs";
pub const SERVER_ENTITY: &'static str = include_str!("../../../ENTITY");
// pub const SERVER_ENTITY: &'static str = "MY-SERVER";
pub const IAT_DELAY_ALLOW_MIN: u64 = 9;
pub const SCREEN_WIDTH_EXTRA: f64 = 1750.0;
pub const LEFT_PANEL_MIN_WIDTH: &str = "800px";

// 16x8=128, 16x64=1024
pub const IMAGE_MAX_SIZE_SQUARE: u32 = 1024;
pub const THUMB_MAX_SIZE_SQUARE: u32 = 128;
pub const PATH_PREFIX_IMAGE: &str = "images";
pub const PATH_PREFIX_THUMB: &str = "thumbs";

pub const IMG_PREFIX: &str = "/img";
pub const PATH_API_PATIENT_IMAGE: &str = "/patient/{hn}";
pub const PATH_PREFIX_PATIENT_IMAGE: &str = "/img/patient/";

pub const PATH_API_XRAY_THUMBNAIL: &str = "/xray/thumbnail";
pub const PATH_API_XRAY_IMAGE: &str = "/xray/image";
pub const PATH_PREFIX_API_XRAY_THUMBNAIL: &str = "/img/xray/thumbnail";
pub const PATH_PREFIX_API_XRAY_IMAGE: &str = "/img/xray/image";

pub const ASSETS_PREFIX: &str = "/assets";

pub const DEFAULT_USER_IMAGE: &str = "statics/picture/user.svg";
pub const DEFAULT_WARN_IMAGE: &str = "statics/picture/warn.svg";

pub const A4_WIDTH: f64 = 595.0;
pub const A4_HEIGHT: f64 = 842.0;
pub const DEFAULT_SVG_REPORT: &str = r#"<svg version="1.1" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 595 842" height="100%"></svg>"#;
pub const DEFAULT_SVG_USER: &str = r##"<?xml version="1.0" encoding="UTF-8"?><svg version="1.1" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 500 600"><path d="m250 96a126 126 0 0 0 0 252 126 126 0 0 0 0-252zm-83 311c-78 0-141 72-141 162 0 17 12 31 27 31h394c15 0 27-14 27-31 0-90-63-162-141-162h-166z" fill="#888"/></svg>"##;

pub const SCALAR_PREFIX: &str = "/scalar";
pub const PROMETHEUS_PREFIX: &str = "/metrics";
