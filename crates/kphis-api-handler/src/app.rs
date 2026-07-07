use axum::{
    Json,
    body::Bytes,
    extract::{Path, State},
    http::{Method, Response, header},
};
use http_body_util::Full;

use kphis_api_core::{
    open_api::{DocBytes, DocOne},
    state::{ApiState, RequestState},
};
use kphis_api_query::{app as app_query, pre_admit};
use kphis_util::{
    datetime::get_timestamp_server,
    error::{AppError, Source},
};

/// /assets
///
/// Get all assets needed by client as application/octet-stream
/// Assets includes all Select element options
#[utoipa::path(get, path = "/assets", responses(DocBytes))]
pub async fn get_app_asset(
    // headers: header::HeaderMap,
    State(mut app): State<ApiState>,
) -> Result<Response<Full<Bytes>>, AppError> {
    // let if_none_match_opt = headers.get("if-none-match").and_then(|hv| hv.to_str().ok());
    // let etag = if let Ok(lock) = app.app_asset_cache_etag.read() {
    //     lock.clone()
    // } else {
    //     String::from("AAAAAAdbzRU=")
    // };
    // if if_none_match_opt.map(|req_etag| req_etag == etag.as_str()).unwrap_or_default() {
    //     Err(AppError::new_server(500, "Not Modified", "Get AppAssets").with_code(StatusCode::NOT_MODIFIED))
    // } else {
    let now = get_timestamp_server()?;
    // reload if cache expired
    if app.app_asset_cache_exp < now {
        app.reload_app_asset();
    }
    // send old app_asset
    let app_asset_bytes = {
        let lock = app.app_asset_bytes_cache.lock().await;
        lock.clone()
    };

    let body = Full::new(Bytes::from(app_asset_bytes));
    Response::builder()
        .header(header::CONTENT_TYPE, "application/octet-stream")
        .body(body)
        .map_err(|e| Source::App.to_error(500, e, "Get AppAsset"))
    // }
}

/// /assets
///
/// Set app_asset_cache_exp == now
#[utoipa::path(patch, path = "/assets", responses(DocOne<bool>))]
pub async fn patch_assets(State(mut app): State<ApiState>) -> Result<Json<bool>, AppError> {
    app.app_asset_cache_exp = get_timestamp_server()?;
    // if let Ok(mut lock) = app.app_asset_cache_etag.write() {
    //     *lock = String::from("AAAAAAdbzRU=");
    // }

    Ok(Json(true))
}

/// /api/exists-key-id/{key}/{id}
///
/// Get amount of row by key and id, return amount of row
#[utoipa::path(
    get,
    path = "/exists-key-id/{key}/{id}",
    responses(DocOne<bool>),
    params(
        ("key" = String, Path, description = "Keyword", example = "med-reconcile-dr-unconfirm"),
        ("id" = String, Path, description = "Id to be checked, id is different for each keyword, ex: 'AN' for 'med-reconcile-dr-unconfirm'", example = "660001234")
    ),
)]
pub async fn get_exists(Path((key, id)): Path<(String, String)>, ctx: RequestState) -> Result<Json<bool>, AppError> {
    ctx.user_state.trace_req_by();
    ctx.authorize_and_access_log(&Method::GET, false).await?;

    let response = if key.as_str() == "an" {
        pre_admit::any_an_exists(&id, &ctx.api_state.db_pool, &ctx.api_state.kphis(), &ctx.api_state.kphis_extra()).await?
    } else {
        app_query::get_exists(&key, &id, &ctx.api_state.db_pool, &ctx.api_state.hosxp(), &ctx.api_state.kphis()).await?
    };

    Ok(Json(response))
}
