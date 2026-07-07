use axum::{
    Json,
    body::Bytes,
    extract::{Path, Query, State},
    http::{Method, Response, header},
};
use http_body_util::Full;

use kphis_api_core::{
    open_api::{DocImage, DocVec},
    state::{ApiState, RequestState},
};
use kphis_api_pacs::{get_image, get_thumbnail, get_xn_data};
use kphis_model::pacs::{PacsParams, PacsXnData};
use kphis_util::error::{AppError, Source};

/// /api/xray/pacs/xn/{xn}
///
/// Get list of PACs's XnData by PARAMS, return a list of XnData
#[utoipa::path(
    get,
    path = "/xray/pacs/xn/{xn}",
    responses(DocVec<PacsXnData>),
    params(
        ("xn" = String, Path, description = "X-Ray Number: XN", example = "1")
    ),
)]
pub async fn get_pacs_xn(Path(xn): Path<i32>, ctx: RequestState) -> Result<Json<PacsXnData>, AppError> {
    ctx.user_state.trace_req_by();
    ctx.authorize_and_access_log(&Method::GET, false).await?;

    if let Some(pacs_config) = &ctx.api_state.app_config.pacs_config {
        let result = get_xn_data(xn, &ctx.api_state.pacs_client, pacs_config).await?;

        Ok(Json(result))
    } else {
        Err(AppError::app_403("Get PacsXnData"))
    }
}

// return jpeg file
/// /img/xray/pacs/thumbnail
///
/// Get X-Ray thumbnail file by PARAMS, return jpeg file
#[utoipa::path(get, path = "/xray/pacs/thumbnail", responses(DocImage), params(PacsParams))]
pub async fn get_pacs_thumbnail(Query(params): Query<PacsParams>, State(app): State<ApiState>) -> Result<Response<Full<Bytes>>, AppError> {
    if let Some(pacs_config) = &app.app_config.pacs_config {
        if let (Some(file_path), Some(study_uid), Some(series_uid), Some(object_uid)) = (&params.file_path, &params.study_uid, &params.series_uid, &params.object_uid) {
            let data = get_thumbnail(file_path, study_uid, series_uid, object_uid, &app.pacs_client, pacs_config).await?;
            let body = Full::new(data);
            Response::builder()
                .header(header::CONTENT_TYPE, "image/jpeg")
                .body(body)
                .map_err(|e| Source::App.to_error(500, e, "Get PacsThumbnail"))
        } else {
            Err(AppError::app_400("Get PacsThumbnail"))
        }
    } else {
        Err(AppError::app_403("Get PacsThumbnail"))
    }
}

// return jpeg file
/// /img/xray/pacs/image
///
/// Get X-Ray full file by PARAMS, return jpeg file
#[utoipa::path(get, path = "/xray/pacs/image", responses(DocImage), params(PacsParams))]
pub async fn get_pacs_image(Query(params): Query<PacsParams>, State(app): State<ApiState>) -> Result<Response<Full<Bytes>>, AppError> {
    if let Some(pacs_config) = &app.app_config.pacs_config {
        if let (Some(file_path), Some(study_uid), Some(series_uid), Some(object_uid)) = (&params.file_path, &params.study_uid, &params.series_uid, &params.object_uid) {
            let data = get_image(file_path, study_uid, series_uid, object_uid, &app.pacs_client, pacs_config).await?;
            let body = Full::new(data);
            Response::builder()
                .header(header::CONTENT_TYPE, "image/jpeg")
                .body(body)
                .map_err(|e| Source::App.to_error(500, e, "Get PacsImage"))
        } else {
            Err(AppError::app_400("Get PacsImage"))
        }
    } else {
        Err(AppError::app_403("Get PacsImage"))
    }
}
