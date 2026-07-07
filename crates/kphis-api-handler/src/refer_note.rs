use axum::{Json, extract::Path, http::Method};

use kphis_api_core::{
    open_api::{DocOne, DocVec},
    state::RequestState,
};
use kphis_api_query::refer_note;
use kphis_model::{
    fetch::ExecuteResponse,
    refer_note::{ReferNote, ReferNoteSave},
};
use kphis_util::error::AppError;

/// /api/refer-note-vnan/{vnan}
///
/// Get Refer-Note data from KPHIS by VN or AN, return list of Refer-Note data
#[utoipa::path(
    get,
    path = "/refer-note-vnan/{vnan}",
    responses(DocVec<ReferNote>),
    params(
        ("vnan" = String, Path, description = "Visit Number or Admit Number", example = "660001234"),
    ),
)]
pub async fn get_refernote(Path(vnan): Path<String>, ctx: RequestState) -> Result<Json<Vec<ReferNote>>, AppError> {
    ctx.user_state.trace_req_by();
    ctx.authorize_and_access_log(&Method::GET, false).await?;

    let response = refer_note::select_refernote(&vnan, &ctx.api_state.db_pool, &ctx.api_state.hosxp(), &ctx.api_state.kphis_extra()).await?;

    Ok(Json(response))
}

/// /api/refer-note-vnan/{vnan}
///
/// Tries to insert or update Refer-Note data in KPHIS
#[utoipa::path(
    post,
    path = "/refer-note-vnan/{vnan}",
    request_body = ReferNoteSave,
    responses(DocOne<ExecuteResponse>),
    params(
        ("vnan" = String, Path, description = "Visit Number or Admit Number", example = "660001234"),
    ),
)]
pub async fn post_refernote(Path(vnan): Path<String>, ctx: RequestState, Json(payload): Json<ReferNoteSave>) -> Result<Json<ExecuteResponse>, AppError> {
    ctx.user_state.trace_req_by();
    ctx.authorize_and_access_log(&Method::POST, false).await?;

    if payload.vn == vnan {
        let result = refer_note::post_refernote(
            &payload,
            &ctx.user_state.user.doctorcode,
            &ctx.user_state.user.loginname,
            &ctx.api_state.db_pool,
            &ctx.api_state.kphis_extra(),
        )
        .await?;

        Ok(Json(result))
    } else {
        Err(AppError::app_400("Post ReferNote"))
    }
}
