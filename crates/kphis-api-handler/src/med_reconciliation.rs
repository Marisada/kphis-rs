use axum::{Json, extract::Path, http::Method};

use kphis_api_core::{open_api::DocVec, state::RequestState};
use kphis_api_query::med_reconciliation;
use kphis_model::med_reconcile::MedReconciliationHeader;
use kphis_util::error::AppError;

/// /api/med-reconcile-hn/{hn}
///
/// Get Med Reconciliation Header of HN (IPD and OPD_ER)
#[utoipa::path(
    get,
    path = "/med-reconcile-hn/{hn}",
    responses(DocVec<MedReconciliationHeader>),
    params(
        ("hn" = String, Path, description = "Hospital Number", example = "0001234"),
    ),
)]
pub async fn get_med_reconciliation_head(Path(hn): Path<String>, ctx: RequestState) -> Result<Json<Vec<MedReconciliationHeader>>, AppError> {
    ctx.user_state.trace_req_by();
    ctx.authorize_and_access_log(&Method::GET, false).await?;

    let response = med_reconciliation::get_med_reconciliation_header(&hn, ctx.api_state.hosxp_an_len(), &ctx.api_state.db_pool, &ctx.api_state.hosxp(), &ctx.api_state.kphis()).await?;

    Ok(Json(response))
}
