#![allow(dead_code)]

mod runtime;
use runtime::SystemWorld;

use gloo_events::EventListener;
use js_sys::{Array, JsString, Object, global};
use std::{path::PathBuf, sync::mpsc};
use typst_layout::PagedDocument;
use typst_library::diag::{EcoString, FileError};
use wasm_bindgen::{JsCast, JsValue};
use web_sys::{DedicatedWorkerGlobalScope, XmlHttpRequest, XmlHttpRequestResponseType};

use kphis_model::{endpoint::EndPoint, user::his::LoginResponse};
use kphis_worker::{JsMessage, MessageSend};

// try removing `kphis_worker::api! {` + `}` container to debug
kphis_worker::api! {

    pub async fn svg(template: String, data: String, token: String) -> Vec<u8> {
        match compile(template, data, kphis_util::util::str_some(token)) {
            Ok(document) => {
                let v = document.pages().iter().map(|page| {
                    let size = page.frame.size();
                    kphis_model::report::TypstSvg {
                        width: size.x.to_pt(),
                        height: size.y.to_pt(),
                        svg: typst_svg::svg(&page, &typst_svg::SvgOptions::default()),
                    }
                }).collect::<Vec<kphis_model::report::TypstSvg>>();
                bitcode::encode(&v)
            }
            Err(e) => {
                console_log(&e);
                Vec::new()
            }
        }
    }

    pub async fn pdf(template: String, data: String, title: String, author: String, user: String, token: String) -> Vec<u8> {
        match compile(template, data, kphis_util::util::str_some(token)) {
            Ok(mut document) => {
                let now = js_sys::Date::now() as i64;
                let dt = time::OffsetDateTime::from_unix_timestamp(now / 1000).unwrap();
                let timestamp = typst_library::foundations::Datetime::from_ymd_hms(
                    dt.year(),
                    dt.month().into(),
                    dt.day(),
                    dt.hour(),
                    dt.minute(),
                    dt.second(),
                ).map(typst_pdf::Timestamp::new_utc);
                let info = document.info_mut();
                info.title = kphis_util::util::str_some(title).map(|t| t.into());
                info.author = vec![author.into(), user.into()];
                let pdf_option = typst_pdf::PdfOptions {
                    ident: typst_library::foundations::Smart::Auto,
                    creator: typst_library::foundations::Smart::Auto,
                    timestamp,
                    page_ranges: None,
                    standards: typst_pdf::PdfStandards::default(),
                    tagged: false,
                    pretty: false,
                    signer: None,
                };
                match typst_pdf::pdf(&document, &pdf_option) {
                    Ok(result) => result,
                    Err(errors) => {
                        let message = errors.iter().map(|e| e.message.as_str()).collect::<Vec<&str>>().join(", ");
                        console_log(&message);
                        Vec::new()
                    }
                }
            }
            Err(e) => {
                console_log(&e);
                Vec::new()
            }
        }
    }
}

fn compile(template: String, data: String, token: Option<String>) -> Result<PagedDocument, String> {
    let mut world = SystemWorld::new();
    world.compile(&template, &data, load_file, token)
}

fn load_file(path: PathBuf, token: &Option<String>) -> Result<Vec<u8>, FileError> {
    if let Some(url) = path.to_str() {
        let (status, body) = xhr(url, token).map_err(|e| FileError::Other(e.dyn_ref::<JsString>().map(|s| Into::<String>::into(s)).map(EcoString::from)))?;
        match status {
            200 => Ok(body),
            // Unauthorized, get refresh token
            401 => {
                let (token_status, token_body) = xhr(&EndPoint::User.base(), &None).map_err(|e| FileError::Other(e.dyn_ref::<JsString>().map(|s| Into::<String>::into(s)).map(EcoString::from)))?;
                if token_status == 200 {
                    // retry only once
                    let res = serde_json::from_slice::<LoginResponse>(&token_body).map_err(|e| FileError::Other(Some(EcoString::from(&e.to_string()))))?;
                    let (last_status, last_body) = xhr(url, &Some(res.token)).map_err(|e| FileError::Other(e.dyn_ref::<JsString>().map(|s| Into::<String>::into(s)).map(EcoString::from)))?;
                    if last_status == 200 {
                        Ok(last_body)
                    } else {
                        Err(FileError::Other(Some(EcoString::from(String::from_utf8_lossy(&last_body)))))
                    }
                } else {
                    Err(FileError::Other(Some(EcoString::from(String::from_utf8_lossy(&token_body)))))
                }
            }
            _ => Err(FileError::Other(Some(EcoString::from(String::from_utf8_lossy(&body))))),
        }
    } else {
        Err(FileError::NotFound(path))
    }
}

// return (status, body)
fn xhr(url: &str, token: &Option<String>) -> Result<(u16, Vec<u8>), JsValue> {
    let xhr = XmlHttpRequest::new()?;
    xhr.set_response_type(XmlHttpRequestResponseType::Arraybuffer);

    let (sender, receiver) = mpsc::channel::<Result<(u16, Vec<u8>), JsValue>>();
    let _onload = EventListener::new(&xhr, "load", {
        let xhr = xhr.clone();
        let sender = sender.clone();
        move |_| {
            let status = xhr.status().unwrap_or_default();
            if let Err(_e) = sender.send(xhr.response().map(|v| (status, js_sys::Uint8Array::new(&v).to_vec()))) {
                log::error!("error mpsc channal sending on_load");
            }
        }
    });
    let _on_error = EventListener::new(&xhr, "error", move |_| {
        if let Err(_e) = sender.send(Err(JsValue::from_str("Failed to load URL"))) {
            log::error!("error mpsc channal sending on_error");
        }
    });

    xhr.open_with_async("GET", url, false)?;
    if let Some(bearer) = token {
        // must call 'set_request_header()' after 'open()' but before 'send()'
        // https://developer.mozilla.org/en-US/docs/Web/API/XMLHttpRequest/setRequestHeader
        xhr.set_request_header("Authorization", &["Bearer ", bearer].concat())?;
    }
    xhr.send()?;

    loop {
        if let Ok(res) = receiver.try_recv() {
            break res;
        }
    }
}

// see kphis-worker/src/lib.rs:181
/// 'log.debug' with 'alert' side effect
fn console_log(msg: &str) {
    let worker = global().dyn_into::<DedicatedWorkerGlobalScope>().unwrap();
    let value = String::into_js(String::from(msg), &Array::new());
    let message: JsMessage = Object::new().unchecked_into();
    message.set_id(0);
    message.set_value(&value);
    if let Err(e) = worker.post_message(&message) {
        let message = e.dyn_ref::<JsString>().map(|s| s.into()).unwrap_or(String::from("Cannot post_message"));
        log::error!("{}", message);
    }
}
