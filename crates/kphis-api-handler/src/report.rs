use axum::{Json, extract::Query, http::Method};

use kphis_api_core::{
    open_api::{DocOne, DocVec},
    state::RequestState,
};
use kphis_api_query::report;
use kphis_model::{
    fetch::ExecuteResponse,
    report::{CustomReport, ReportQuery, ReportTemplateParams},
};
use kphis_util::error::AppError;

/// /api/report/custom
///
/// Get custom report template by params, return list of custom report template
#[utoipa::path(
    get,
    path = "/report/custom",
    responses(DocVec<CustomReport>),
    params(ReportTemplateParams),
)]
pub async fn get_custom_report(Query(params): Query<ReportTemplateParams>, ctx: RequestState) -> Result<Json<Vec<CustomReport>>, AppError> {
    ctx.user_state.trace_req_by();
    ctx.authorize_and_access_log(&Method::GET, false).await?;

    let response = if params.compact.unwrap_or_default() {
        report::select_report_template_compact(&params, &ctx.api_state.db_pool, &ctx.api_state.kphis_extra()).await?
    } else {
        report::select_report_template(&params, &ctx.api_state.db_pool, &ctx.api_state.hosxp(), &ctx.api_state.kphis_extra()).await?
    };

    Ok(Json(response))
}

/// /api/report/raw-query
///
/// Get custom report query result by payload, return JSON string
#[utoipa::path(
    post,
    path = "/report/raw-query",
    request_body = ReportQuery,
    responses(DocOne<String>),
)]
pub async fn post_query_to_json_string(ctx: RequestState, Json(payload): Json<ReportQuery>) -> Result<Json<String>, AppError> {
    ctx.user_state.trace_req_by();
    ctx.authorize_and_access_log(&Method::POST, false).await?;

    let response = report::select_raw_query_to_json_string(
        &payload.statement,
        &payload.statement_params,
        &payload.ids,
        &ctx.api_state.app_asset_cache,
        &ctx.api_state.db_pool,
        &ctx.api_state.hosxp(),
        &ctx.api_state.kphis(),
        &ctx.api_state.kphis_extra(),
        &ctx.api_state.kphis_log(),
    )
    .await?;

    Ok(Json(response))
}

/// /api/report/custom
///
/// Tries to create/edit Custom Report
/// - Payload's `template_name` must unique
#[utoipa::path(
    post,
    path = "/report/custom",
    request_body = CustomReport,
    responses(DocOne<ExecuteResponse>),
)]
pub async fn post_custom_report(ctx: RequestState, Json(payload): Json<CustomReport>) -> Result<Json<ExecuteResponse>, AppError> {
    ctx.user_state.trace_req_by();
    ctx.authorize_and_access_log(&Method::POST, false).await?;

    if payload.template_id > 0 {
        let response = report::update_report_template(&payload, &ctx.user_state.user.loginname, &ctx.api_state.db_pool, &ctx.api_state.kphis_extra()).await?;
        Ok(Json(response))
    } else if report::select_report_template_exists(&payload.template_name, &ctx.api_state.db_pool, &ctx.api_state.kphis_extra()).await? {
        Err(AppError::app_400("POST ReportCustom"))
    } else {
        let response = report::insert_report_template(&payload, &ctx.user_state.user.loginname, &ctx.api_state.db_pool, &ctx.api_state.kphis_extra()).await?;
        Ok(Json(response))
    }
}

/// /api/report/custom
///
/// Tries to delete Custom Template by params
/// - Query parameter `template_id` must not null
#[utoipa::path(
    delete,
    path = "/report/custom",
    responses(DocOne<ExecuteResponse>),
    params(ReportTemplateParams),
)]
pub async fn delete_custom_report(Query(params): Query<ReportTemplateParams>, ctx: RequestState) -> Result<Json<ExecuteResponse>, AppError> {
    ctx.user_state.trace_req_by();
    ctx.authorize_and_access_log(&Method::DELETE, false).await?;

    if let Some(template_id) = params.template_id {
        let response = report::delete_report_template(template_id, &ctx.api_state.db_pool, &ctx.api_state.kphis_extra()).await?;

        Ok(Json(response))
    } else {
        Err(AppError::app_400("Delete CustomReport"))
    }
}
