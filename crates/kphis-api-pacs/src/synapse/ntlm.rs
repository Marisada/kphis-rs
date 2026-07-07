use axum::body::Bytes;
use base64::{Engine, engine::general_purpose::STANDARD};
use reqwest::{Method, header};
use std::collections::HashMap;
use winauth::NextBytes;

use kphis_util::error::{AppError, Source};

const SYNAPSE_URL: &str = "http://syn5ndh/WorkflowUI/";
const NTLM: &str = "NTLM";

#[derive(Debug)]
enum AspNetState {
    /// redirect to AspNetAuth::next_path
    Redirect(Method, Vec<u8>, bool),
    /// get WWW-Authenticate : NTLM
    Negotiation,
    /// get WWW-Authenticate : NTLM + Type 2 message
    Challenge(String),
    /// always be the last iterate
    Final,
    /// return Content length
    Finished(Bytes),
    /// return Client Error
    ClientError(reqwest::Error),
    /// return Error as `(StatusCode, Message)`
    Error((u16, String)),
    /// return specific error
    NotSupported,
}

// impl AspNetState {
//     fn ty(&self) -> String {
//         match self {
//             Self::Redirect(method, _, _) => ["Redirect ", &method.to_string()].concat(),
//             Self::Negotiation => String::from("Negotiation"),
//             Self::Challenge(_) => String::from("Challenge"),
//             Self::Final => String::from("Final"),
//             Self::Finished(_) => String::from("Finished"),
//             Self::ClientError(_) => String::from("ClientError"),
//             Self::Error((_, _)) => String::from("Error"),
//             Self::NotSupported => String::from("NotSupported"),
//         }
//     }
// }

#[derive(Debug, Default)]
struct SynapsePreAuth {
    pub action: String,
    pub variables: HashMap<String, String>,
    pub preauthapis_success: usize,
}
impl SynapsePreAuth {
    fn set(&mut self, name: &str, value: Option<String>) {
        if !name.is_empty() {
            if let Some(v) = value {
                self.variables.insert(name.to_string(), v);
            }
        }
    }
    fn preauthapis(&self) -> Vec<String> {
        if let Some(apis) = self.variables.get("preauthapis") {
            apis.as_str().split(',').map(|s| s.to_string()).collect()
        } else {
            Vec::new()
        }
    }
}

pub struct AspNetAuth<'a> {
    client: reqwest::Client,
    sspi: winauth::NtlmV2Client<'a>,
    state: AspNetState,
    is_next: bool,
    next_path: String,

    final_method: Method,
    final_path: String,
    final_body: Vec<u8>,
    final_is_json: bool,
}

impl<'a> AspNetAuth<'a> {
    /// with GET method
    pub fn new(method: Method, path: &str, body: &[u8], is_json: bool, user: &'a str, password: &'a str, domain: Option<&'a str>, pacs_client: &reqwest::Client) -> Self {
        let sspi = winauth::NtlmV2ClientBuilder::new().build(domain, user, password);
        Self {
            client: pacs_client.clone(),
            sspi,
            state: AspNetState::Redirect(method.clone(), body.to_vec(), is_json),
            is_next: true,
            next_path: path.to_owned(),

            final_path: path.to_owned(),
            final_method: method,
            final_body: body.to_vec(),
            final_is_json: is_json,
        }
    }

    pub fn set_next_request(&mut self, method: Method, path: &str, body: &[u8], is_json: bool) {
        self.state = AspNetState::Redirect(method.to_owned(), body.to_vec(), is_json);
        self.is_next = true;
        self.next_path = path.to_owned();
        self.final_path = path.to_string();
        self.final_method = method;
        self.final_body = body.to_vec();
        self.final_is_json = is_json;
    }

    pub async fn run(&mut self) -> Result<Bytes, AppError> {
        while self.is_ntlm_running() {
            self.request().await
        }

        match &self.state {
            AspNetState::Finished(bytes) => Ok(bytes.to_owned()),
            AspNetState::ClientError(e) => Err(Source::PacsClient.to_error(500, e, "NTLM")),
            AspNetState::Error((status, message)) => Err(Source::PacsClient.to_error(500, &[&status.to_string(), ":", message].concat(), "NTLM")),
            AspNetState::Negotiation | AspNetState::Redirect(_, _, _) | AspNetState::Challenge(_) | AspNetState::Final | AspNetState::NotSupported => {
                Err(Source::PacsClient.to_error(500, "Unexpected !!", "NTLM"))
            }
        }
    }

    fn is_ntlm_running(&self) -> bool {
        self.is_next
    }
    // return is_finished
    async fn request(&mut self) {
        // dbg!(&self.state.ty());
        let builder = match &self.state {
            AspNetState::Redirect(method, body, is_json) => {
                let req = self.client.request(method.into(), &self.next_path).body(body.to_vec());
                if *is_json { req.header(header::CONTENT_TYPE, "application/json;charset=utf-8") } else { req }
            }
            AspNetState::Negotiation => {
                if let Ok(Some(next_bytes)) = self.sspi.next_bytes(None) {
                    self.client
                        .request(Method::GET, &self.next_path)
                        .header(header::AUTHORIZATION, format!("{} {}", NTLM, STANDARD.encode(next_bytes)))
                } else {
                    self.client.request(Method::GET, &self.next_path)
                }
            }
            AspNetState::Challenge(message) => {
                let challenge = message.trim_start_matches(NTLM).trim_start();
                let in_bytes = STANDARD.decode(challenge).ok();
                if let Ok(Some(next_bytes)) = self.sspi.next_bytes(in_bytes.as_deref()) {
                    self.client
                        .request(Method::GET, &self.next_path)
                        .header(header::AUTHORIZATION, format!("{} {}", NTLM, STANDARD.encode(next_bytes)))
                } else {
                    self.client.request(Method::GET, &self.next_path)
                }
            }
            AspNetState::Final => {
                self.is_next = false;
                let req = self.client.request(self.final_method.clone(), &self.final_path).body(self.final_body.to_vec());
                if self.final_is_json {
                    req.header(header::CONTENT_TYPE, "application/json;charset=utf-8")
                } else {
                    req
                }
            }
            AspNetState::Finished(_) | AspNetState::ClientError(_) | AspNetState::Error(_) | AspNetState::NotSupported => self.client.request(Method::GET, &self.next_path),
        };

        match builder.send().await {
            Ok(res) => {
                let redirect = res.headers().get(header::LOCATION).and_then(|v| v.to_str().ok());
                match redirect {
                    Some(path) => {
                        self.state = AspNetState::Redirect(Method::GET, Vec::new(), false);
                        self.next_path = path.to_string();
                    }
                    None => {
                        let status = res.status().as_u16();
                        let headers = res.headers().clone();
                        let is_html = headers.get(header::CONTENT_TYPE).and_then(|value| value.to_str().ok()).map_or(false, |str| str.contains("html"));
                        let body = res.bytes().await.unwrap_or(Bytes::new());
                        if [200, 302, 401].contains(&status) {
                            let www_auth = headers.get(header::WWW_AUTHENTICATE).and_then(|v| v.to_str().ok());
                            // NTLM
                            if let Some(message) = www_auth {
                                // get type 0 NTLM message
                                if message == NTLM {
                                    self.state = AspNetState::Negotiation;
                                // get type 2 NTLM message
                                } else if message.starts_with(NTLM) {
                                    self.state = AspNetState::Challenge(message.to_string());
                                // Authenticate schema is not 'NTLM'
                                } else {
                                    self.state = AspNetState::NotSupported;
                                    self.is_next = false;
                                }
                            // HTML
                            } else if is_html {
                                // let body = res.bytes().await.unwrap_or(bytes::Bytes::new());
                                let pre_auth_opt = std::str::from_utf8(&body).ok().and_then(|html| tl::parse(html, tl::ParserOptions::default()).ok()).and_then(|vdom| {
                                    let parser = vdom.parser();
                                    if let Some(form) = vdom.nodes().iter().find(|node| node.as_tag().map_or(false, |tag| tag.name() == "form")) {
                                        let action_opt = get_attr(form, "action");
                                        if let Some(action) = action_opt {
                                            let mut pre_auth = SynapsePreAuth { action, ..Default::default() };
                                            if let Some(child) = form.children() {
                                                child.top().iter().for_each(|handle| {
                                                    if let Some((name, value)) = handle.get(parser).map(|node| {
                                                        let name = get_attr(node, "name").unwrap_or_default();
                                                        let value = get_attr(node, "value");
                                                        (name, value)
                                                    }) {
                                                        pre_auth.set(&name, value);
                                                    }
                                                })
                                            }
                                            Some(pre_auth)
                                        } else {
                                            None // no <form action=??
                                        }
                                    } else {
                                        None // no <form>
                                    }
                                });
                                if let Some(mut pre_auth) = pre_auth_opt {
                                    let apis = pre_auth.preauthapis();
                                    let apis_len = apis.len();
                                    if let Some(token) = pre_auth.variables.get("access_token") {
                                        for api in apis.iter() {
                                            let path = [api, "?at=", token].concat();
                                            if self.client.request(Method::GET, &path).send().await.is_ok() {
                                                pre_auth.preauthapis_success += 1;
                                            }
                                        }
                                    }
                                    if pre_auth.preauthapis_success == apis_len {
                                        if let Ok(resp) = self.client.request(Method::POST, SYNAPSE_URL).form(&pre_auth.variables).send().await {
                                            if resp.status().is_success() {
                                                self.state = AspNetState::Final;
                                                self.next_path = pre_auth.action;
                                            } else {
                                                self.state = AspNetState::Error((status, ["POST pre_auth.variables ", &String::from_utf8_lossy(&body)].concat()));
                                                // self.state = AspNetState::Finished(body);
                                                self.is_next = false;
                                            }
                                        } else {
                                            self.state = AspNetState::Error((status, ["POST pre_auth.variables ", &String::from_utf8_lossy(&body)].concat()));
                                            // self.state = AspNetState::Finished(body);
                                            self.is_next = false;
                                        }
                                    } else {
                                        self.state = AspNetState::Error((status, ["pre_auth.preauthapis_success != apis_len ", &String::from_utf8_lossy(&body)].concat()));
                                        // self.state = AspNetState::Finished(body);
                                        self.is_next = false;
                                    }
                                } else {
                                    // not valid <from action=??> html
                                    self.state = AspNetState::Error((status, ["Not valid <from action=??> html ", &String::from_utf8_lossy(&body)].concat()));
                                    // self.state = AspNetState::Finished(body);
                                    self.is_next = false;
                                }
                            // for failed first 'Target' url
                            } else if status == 401 {
                                if let Ok(dummy) = self.client.request(Method::GET, SYNAPSE_URL).send().await {
                                    // let url_encoded = urlencoding::encode(SYNAPSE_URL);
                                    // self.next_path = dummy.url().as_str().replace(url_encoded.as_ref(), &urlencoding::encode(&self.next_path));
                                    self.next_path = dummy.url().to_string();
                                    self.state = AspNetState::Redirect(Method::GET, Vec::new(), false);
                                } else {
                                    self.state = AspNetState::Error((status, ["Failed retry after 401 ", &String::from_utf8_lossy(&body)].concat()));
                                    // self.state = AspNetState::Finished(body);
                                    self.is_next = false;
                                }
                            // 200 / 302 success
                            } else {
                                self.state = AspNetState::Finished(body);
                                self.is_next = false;
                            }
                        // No Content: delete sessionID
                        } else if status == 204 {
                            self.state = AspNetState::Finished(body);
                            self.is_next = false;
                        // unxepected result
                        } else {
                            self.state = AspNetState::Error((status, ["Unexpected ", &String::from_utf8_lossy(&body)].concat()));
                            self.is_next = false;
                        }
                    }
                }
            }
            Err(e) => {
                // dbg!(&e);
                self.state = AspNetState::ClientError(e);
                self.is_next = false;
            }
        }
    }
}

fn get_attr(node: &tl::Node, name: &str) -> Option<String> {
    node.as_tag()
        .and_then(|tag| tag.attributes().get(name).flatten().and_then(|bytes| std::str::from_utf8(bytes.as_bytes()).ok()).map(|s| s.to_string()))
}
