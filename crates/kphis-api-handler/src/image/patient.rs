use axum::{
    body::Bytes,
    extract::{Path, State},
    http::{Response, header},
};
use http_body_util::Full;
use kphis_api_core::open_api::DocImage;
use tracing::warn;

use kphis_api_core::state::ApiState;
use kphis_api_query::image::patient;
use kphis_model::{DEFAULT_SVG_USER, image::get_image_info};
use kphis_util::error::{AppError, Source};

// return patient image file
/// /img/patient/{hn}
///
/// Get image file by HN, return image file
#[utoipa::path(
    get,
    path = "/patient/{hn}",
    responses(DocImage),
    params(
        ("hn" = String, Path, description = "Patient Hospital Number(HN)", example = "0001234"),
    ),
)]
pub async fn get_patient_image(Path(hn): Path<String>, State(app): State<ApiState>) -> Result<Response<Full<Bytes>>, AppError> {
    match patient::get_patient_image(&hn, &app.db_pool, &app.hosxp()).await {
        Ok(Some(image_bytes)) => match get_image_info(&image_bytes) {
            Ok(image_info) => {
                let body = Full::new(Bytes::from(image_bytes));
                Response::builder()
                    .status(200)
                    .header(header::CONTENT_TYPE, image_info.mime_type)
                    .header(header::CACHE_CONTROL, "private,max-age=1234567,immutable")
                    .body(body)
                    .map_err(|e| Source::App.to_error(500, e, "GetPatientImage"))
            }
            Err(e) => {
                warn!("Error get patient image: {}", e.message);
                svg_file_response()
            }
        },
        Ok(None) => svg_file_response(),
        Err(e) => {
            warn!("Error get patient image: {}", e.message);
            svg_file_response()
        }
    }
}

fn svg_file_response() -> Result<Response<Full<Bytes>>, AppError> {
    let user_svg = DEFAULT_SVG_USER.as_bytes();
    let body = Full::new(Bytes::from(user_svg.to_vec()));
    Response::builder()
        .status(200)
        .header(header::CONTENT_TYPE, "image/svg+xml")
        .header(header::CACHE_CONTROL, "private,max-age=1234567,immutable")
        .body(body)
        .map_err(|e| Source::App.to_error(500, e, "GetDefaultImage"))
}
