mod doc;
mod route;

use axum::{
    Router,
    handler::HandlerWithoutStateExt,
    http::{
        StatusCode,
        header::{self, HeaderName, HeaderValue},
    },
    routing::get,
};
use axum_prometheus::PrometheusMetricLayer;
use tower_http::{services::ServeDir, set_header::SetResponseHeaderLayer};
use utoipa::OpenApi;

use kphis_api_core::{scalar, state::ApiState};

pub fn new_router(state: &ApiState) -> Router {
    // Handler
    let handle_404 = handle_404.into_service();
    let root_dir = ServeDir::new("volume/pwa")
        .precompressed_br()
        .precompressed_gzip()
        //.precompressed_deflate()
        //.precompressed_zstd()
        .not_found_service(handle_404);
    let images_dir = ServeDir::new(["volume/", kphis_model::PATH_PREFIX_IMAGE].concat());
    let thumbs_dir = ServeDir::new(["volume/", kphis_model::PATH_PREFIX_THUMB].concat());
    // Create metric layer
    let (prometheus_layer, metric_handle) = PrometheusMetricLayer::pair();

    // Create main router
    Router::new()
        // `/api/..`
        .nest(kphis_model::API_PREFIX, route::api_router(state.clone()))
        // `/sse` GET only
        .nest(kphis_model::SSE_GET_PREFIX, route::sse_get_router(state.clone()))
        // '/assets/..'
        .nest(kphis_model::ASSETS_PREFIX, route::assets_router(state.clone()))
        .nest(kphis_model::CUSTOM_REPORT_PREFIX, route::custom_template_router(state.clone()))
        // `/img/..`
        .nest(kphis_model::IMG_PREFIX, route::img_router(state.clone()))
        //  `/images/..`
        .nest_service(&["/", kphis_model::PATH_PREFIX_IMAGE].concat(), images_dir)
        // `/thumbs/..`
        .nest_service(&["/", kphis_model::PATH_PREFIX_THUMB].concat(), thumbs_dir)
        // // Swagger UI
        // .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", ApiDoc::openapi()))
        // // Redoc UI
        // .merge(Redoc::with_url("/redoc", ApiDoc::openapi()))
        // // RapiDoc UI
        // // There is no need to create `RapiDoc::with_openapi` because the OpenApi is served
        // // via SwaggerUi instead we only make rapidoc to point to the existing doc.
        // .merge(RapiDoc::new("/api-docs/openapi.json").path("/rapidoc"))
        // // Alternative to above
        // // .merge(RapiDoc::with_openapi("/api-docs/openapi2.json", ApiDoc::openapi()).path("/rapidoc"))
        // // Scalar UI
        // // using utoipa_scalar
        // .merge(Scalar::with_url("/scalar", ApiDoc::openapi()))
        // // using local Scalar html template
        .route(kphis_model::SCALAR_PREFIX, get(|| async move { axum::response::Html(scalar::html(doc::ApiDoc::openapi())) }))
        .route(kphis_model::PROMETHEUS_PREFIX, get(|| async move { metric_handle.render() }))
        .fallback_service(root_dir)
        // some router already set CACHE_CONTROL, so we use `if_not_present` to fill the rest with `no-store`
        .layer(SetResponseHeaderLayer::if_not_present(header::CACHE_CONTROL, HeaderValue::from_static("no-store")))
        .layer(SetResponseHeaderLayer::if_not_present(
            HeaderName::from_static("cross-origin-resource-policy"),
            HeaderValue::from_static("same-origin"),
        ))
        .layer(SetResponseHeaderLayer::if_not_present(
            HeaderName::from_static("x-frame-options"),
            HeaderValue::from_static("SAMEORIGIN"),
        ))
        .layer(SetResponseHeaderLayer::if_not_present(
            HeaderName::from_static("x-content-type-options"),
            HeaderValue::from_static("nosniff"),
        ))
        .layer(prometheus_layer)
}

async fn handle_404() -> (StatusCode, &'static str) {
    (StatusCode::NOT_FOUND, "Not found")
}
