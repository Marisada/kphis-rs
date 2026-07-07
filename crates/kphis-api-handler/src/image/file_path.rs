use axum::{
    Json,
    extract::{Multipart, Path},
    http::Method,
};
use sqlx::mysql::MySqlQueryResult;

use kphis_api_core::{
    open_api::{DocOne, DocVec},
    state::RequestState,
};
use kphis_api_query::image::file_path;
use kphis_model::{
    PATH_PREFIX_IMAGE, PATH_PREFIX_THUMB,
    fetch::ExecuteResponse,
    image::file_path::{ImagePath, ImageSave, ImageUsage},
};
use kphis_util::error::{AppError, Source};

/// /api/image
///
///  Try to save Image wih mutipart to disk and database, return a list of ImagePath
#[utoipa::path(
    post,
    path = "/image",
    request_body(content = inline(ImageSave), content_type = "multipart/form-data"),
    responses(DocVec<ImagePath>),
)]
pub async fn post_image_file(ctx: RequestState, multipart: Multipart) -> Result<Json<Vec<ImagePath>>, AppError> {
    ctx.user_state.trace_req_by();
    ctx.authorize_and_access_log(&Method::POST, false).await?;

    let files = from_multipart(multipart).await?;
    let image_paths = if !files.is_empty() {
        file_path::post_image_file(&files, &ctx.user_state.user.loginname, &ctx.api_state.db_pool, &ctx.api_state.kphis_extra()).await?
    } else {
        Vec::new()
    };

    Ok(Json(image_paths))
}

async fn from_multipart(mut multipart: Multipart) -> Result<Vec<ImageSave>, AppError> {
    let mut forms = Vec::new();
    while let Ok(Some(field)) = multipart.next_field().await {
        let name = field.name().ok_or(Source::App.to_error(400, "No field 'name'", "ParseFileMultipart"))?.to_owned();
        if [PATH_PREFIX_IMAGE, PATH_PREFIX_THUMB].contains(&name.as_str()) {
            let file_name = field.file_name().ok_or(Source::App.to_error(400, "No field 'file_name'", "ParseFileMultipart"))?.to_owned();
            let content_type = field
                .content_type()
                .map(|ct| {
                    if ct == "image/webp" {
                        Ok(ct)
                    } else {
                        Err(Source::App.to_error(400, "'content_type' not `webp`", "ParseFileMultipart"))
                    }
                })
                .ok_or(Source::App.to_error(400, "No field 'content_type'", "ParseFileMultipart"))??
                .to_owned();
            let bytes = field.bytes().await.map_err(|e| Source::App.to_error(500, e, "ParseFileMultipart"))?;
            let form = ImageSave {
                name,
                file_name,
                content_type,
                bytes: bytes.to_vec(),
            };
            forms.push(form);
        }
    }

    Ok(forms)
}

/// /api/image
///
/// Try to edit 'title' of ImagePath
#[utoipa::path(
    patch,
    path = "/image",
    request_body = ImagePath,
    responses(DocOne<ExecuteResponse>),
)]
pub async fn patch_image_path(ctx: RequestState, Json(payload): Json<ImagePath>) -> Result<Json<ExecuteResponse>, AppError> {
    ctx.user_state.trace_req_by();
    ctx.authorize_and_access_log(&Method::PUT, false).await?;

    let result = file_path::patch_image_path(&payload.title, payload.image_id, &ctx.user_state.user.loginname, &ctx.api_state.db_pool, &ctx.api_state.kphis_extra()).await?;

    Ok(Json(ExecuteResponse::from_query_result(result, "Put ImagePath")))
}

/// /api/image
///
/// Try to delete file, ImagePath, and associate ImageUsage
#[utoipa::path(
    delete,
    path = "/image",
    request_body = Vec<u32>,
    responses(DocOne<ExecuteResponse>),
)]
pub async fn delete_image_file(ctx: RequestState, Json(payload): Json<Vec<u32>>) -> Result<Json<ExecuteResponse>, AppError> {
    ctx.user_state.trace_req_by();
    ctx.authorize_and_access_log(&Method::DELETE, false).await?;

    let result = if !payload.is_empty() {
        file_path::delete_image_file(&payload, &ctx.user_state.user.loginname, &ctx.api_state.db_pool, &ctx.api_state.kphis_extra()).await?
    } else {
        MySqlQueryResult::default()
    };

    Ok(Json(ExecuteResponse::from_query_result(result, "Delete ImageFile")))
}

/// /api/image-usage-id/{usage_id}/{usage_key_id}
///
/// Get ImagePath by Usage and ID, return a list of ImagePath
#[utoipa::path(
    get,
    path = "/image-usage-id/{usage_id}/{usage_key_id}",
    responses(DocVec<ImagePath>),
    params(
        ("usage_id" = u32, Path, description = "ID of usage ex: IpdProgressNote", example = "1"),
        ("usage_key_id" = u32, Path, description = "ID of usage key ex: ProgressNoteId", example = "1")
    ),
)]
pub async fn get_image_usage_id(Path((usage_id, usage_key_id)): Path<(ImageUsage, u32)>, ctx: RequestState) -> Result<Json<Vec<ImagePath>>, AppError> {
    ctx.user_state.trace_req_by();
    ctx.authorize_and_access_log(&Method::GET, false).await?;

    let result = file_path::get_image_usage_id(usage_id as u32, usage_key_id, &ctx.api_state.db_pool, &ctx.api_state.hosxp(), &ctx.api_state.kphis_extra()).await?;

    Ok(Json(result))
}

/// /api/image-usage
///
/// Try insert ImagePath to image_usage
#[utoipa::path(
    post,
    path = "/image-usage",
    request_body = Vec<ImagePath>,
    responses(DocOne<ExecuteResponse>),
)]
pub async fn post_image_usage(ctx: RequestState, Json(payloads): Json<Vec<ImagePath>>) -> Result<Json<ExecuteResponse>, AppError> {
    ctx.user_state.trace_req_by();
    ctx.authorize_and_access_log(&Method::POST, false).await?;

    let result = if !payloads.is_empty() {
        file_path::post_image_usage(&payloads, &ctx.user_state.user.loginname, &ctx.api_state.db_pool, &ctx.api_state.kphis_extra()).await?
    } else {
        MySqlQueryResult::default()
    };

    Ok(Json(ExecuteResponse::from_query_result(result, "Insert ImageUsage")))
}

/// /api/image-usage
///
/// Try to delete ImagePath from image_usage
#[utoipa::path(
    delete,
    path = "/image-usage",
    request_body = Vec<u32>,
    responses(DocOne<ExecuteResponse>),
)]
pub async fn delete_image_usage(ctx: RequestState, Json(payloads): Json<Vec<u32>>) -> Result<Json<ExecuteResponse>, AppError> {
    ctx.user_state.trace_req_by();
    ctx.authorize_and_access_log(&Method::DELETE, false).await?;

    let result = if !payloads.is_empty() {
        file_path::delete_image_usage(&payloads, &ctx.user_state.user.loginname, &ctx.api_state.db_pool, &ctx.api_state.kphis_extra()).await?
    } else {
        MySqlQueryResult::default()
    };

    Ok(Json(ExecuteResponse::from_query_result(result, "Delete ImageUsage")))
}
