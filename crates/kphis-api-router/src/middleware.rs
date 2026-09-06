use axum::{
    body::Body,
    extract::{ConnectInfo, OriginalUri, State},
    http::Request,
    middleware,
    response::Response,
};
use axum_extra::{
    TypedHeader,
    headers::{Authorization, authorization::Bearer},
};
use std::net::{IpAddr, SocketAddr};
use ulid::Ulid;

use kphis_api_core::{state::ApiState, token::get_claim_public};
use kphis_api_query::log;
use kphis_util::{
    datetime::get_timestamp_server,
    error::{AppError, ErrorTitle, Source},
};

/// set `SocketAddr` to request's Extension
pub async fn real_ip_middleware(State(state): State<ApiState>, ConnectInfo(socket_addr): ConnectInfo<SocketAddr>, mut request: Request<Body>, next: middleware::Next) -> Response {
    let real_addr = state
        .app_config
        .real_ip_header
        .as_ref()
        .and_then(|real_ip_header| request.headers().get(real_ip_header))
        .and_then(|v| v.to_str().ok())
        .and_then(|v| v.parse::<IpAddr>().ok())
        .map(|ip| SocketAddr::new(ip, socket_addr.port()))
        .unwrap_or(socket_addr);

    request.extensions_mut().insert(real_addr);

    next.run(request).await
}

/// set `u128` or `TokenState` to request's Extension
pub async fn token_id_middleware(State(state): State<ApiState>, bearer_opt: Option<TypedHeader<Authorization<Bearer>>>, mut request: Request<Body>, next: middleware::Next) -> Response {
    if let Some(TypedHeader(Authorization(bearer))) = bearer_opt {
        let token = bearer.token();
        match get_claim_public(token, &state.paseto.public) {
            Ok(claims) => match get_timestamp_server() {
                Ok(now_ts) => {
                    if claims.iat > now_ts || claims.exp < now_ts {
                        request.extensions_mut().insert(AppError::app_401("Verify Token").with_title(ErrorTitle::Security));
                    } else {
                        match Ulid::from_string(&claims.sub) {
                            Ok(Ulid(state_id)) => {
                                request.extensions_mut().insert(state_id);
                            }
                            Err(e) => {
                                request.extensions_mut().insert(Source::UlidDecode.to_error(401, e, "Claims"));
                            }
                        }
                    }
                }
                Err(e) => {
                    request.extensions_mut().insert(e);
                }
            },
            Err(e) => {
                request.extensions_mut().insert(e);
            }
        }
    }

    next.run(request).await
}

/// logging response
pub async fn log_middleware(State(state): State<ApiState>, request: Request<Body>, next: middleware::Next) -> Response {
    let method = request.method().to_owned();
    let req_uri = request.uri().to_owned();
    let uri = request.extensions().get::<OriginalUri>().map(|uri| uri.to_string()).unwrap_or(req_uri.to_string());
    let real_addr = request.extensions().get::<SocketAddr>().map(|addr| addr.to_string());
    let req_error_opt = request.extensions().get::<AppError>().map(|e| e.string_inline());

    let req_loginname = match request.extensions().get::<u128>() {
        Some(state_id) => state.online_get(*state_id).await.map(|user| user.user.loginname.clone()),
        None => None,
    };

    // END REQUEST
    let response = next.run(request).await;
    // START RESPONSE

    let status = response.status();

    let resp_loginname = match response.extensions().get::<Option<u128>>().copied().flatten() {
        Some(state_id) => state.online_get(state_id).await.map(|user| user.user.loginname.clone()),
        None => None,
    };
    // use any response (after log-in) or request (before log-out) loginname
    let loginname = resp_loginname.or(req_loginname);

    let resp_error_opt = response.extensions().get::<AppError>().map(|e| e.string_inline());
    let error_opt = req_error_opt.or(resp_error_opt);

    let access_detail = serde_json::json!({
        "method": method.to_string(),
        "uri": uri.to_string(),
        "status": status.to_string(),
        "error": error_opt,
    })
    .to_string();

    let cred = match (&loginname, &real_addr) {
        (Some(name), Some(addr)) => ["from ", name, "@", addr].concat(),
        (Some(name), None) => ["from ", name].concat(),
        (None, Some(addr)) => ["from ", addr].concat(),
        (None, None) => String::new(),
    };
    if let Some(error) = &error_opt {
        tracing::error!("error handling request {} with status {} : {}", cred, status, error);
    } else {
        tracing::info!("handling request {} with status {}", cred, status);
    }

    if loginname.is_some() || !state.access_log_only_authorized() {
        if let Err(e) = log::insert_access_log(&loginname, &real_addr, &access_detail, &state.db_pool, &state.kphis_log()).await {
            tracing::error!("error writing access log to database: {}", e.message);
        }
    }

    response
}
