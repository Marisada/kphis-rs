use axum::{Json, http::Method};
use rand::Rng;

use kphis_api_core::{
    open_api::{DocOne, DocVec},
    state::RequestState,
};
use kphis_api_query::ipd::passcode;
use kphis_model::ipd::passcode::{ConfigIpdWardPasscode, PasscodeGenRequest, PasscodeGenRequestMode, PasscodeGenResponse};
use kphis_util::error::AppError;

use crate::user::his::verify_password;

// kphis-config-ipd-ward-passcode-data.php
/// /api/ipd/passcode
///
/// Get list of IPD Ward locked by Passcode, return list of IPD Ward
#[utoipa::path(
    get,
    path = "/ipd/passcode",
    responses(DocVec<ConfigIpdWardPasscode>),
)]
pub async fn get_ipd_ward_passcode(ctx: RequestState) -> Result<Json<Vec<ConfigIpdWardPasscode>>, AppError> {
    ctx.user_state.trace_req_by();
    ctx.authorize_and_access_log(&Method::GET, false).await?;

    let response = passcode::get_ward_passcode(&ctx.user_state.user.loginname, &ctx.api_state.db_pool, &ctx.api_state.hosxp(), &ctx.api_state.kphis()).await?;

    Ok(Json(response))
}

// kphis-config-ipd-ward-passcode-gen.php
/// /api/ipd/passcode
///
/// Tries to create/edit IPD Ward Passcode
#[utoipa::path(
    post,
    path = "/ipd/passcode",
    request_body = PasscodeGenRequest,
    responses(DocOne<PasscodeGenResponse>),
)]
pub async fn post_ipd_ward_passcode(ctx: RequestState, Json(payload): Json<PasscodeGenRequest>) -> Result<Json<PasscodeGenResponse>, AppError> {
    ctx.user_state.trace_req_by();
    ctx.authorize_and_access_log(&Method::POST, false).await?;

    // Check privilege
    if !ctx.user_state.user.can_passcode {
        tracing::warn!("user {} cannot change ward passcode", &ctx.user_state.user.name);
        return Err(AppError::app_401("Post WardPasscode"));
    }
    // Check password
    if let Err(e) = verify_password(&ctx.user_state.user.passweb, &payload.password) {
        tracing::warn!("user {} failed to verify with {}", &ctx.user_state.user.name, e.message);
        return Err(e);
    }

    match payload.mode {
        PasscodeGenRequestMode::Gen => {
            // generate 4 digit number passcode in String type
            // use scope to drop ThreadRng, !send removed
            let passcode = {
                let mut rng = rand::rng();
                format!("{:04}", rng.random_range(0..10_000))
            };

            let response = passcode::post_ward_passcode(&payload.ward, &passcode, &ctx.user_state.user.loginname, &ctx.api_state.db_pool, &ctx.api_state.kphis()).await?;

            Ok(Json(if response > 0 { PasscodeGenResponse::new(Some(passcode)) } else { PasscodeGenResponse::new(None) }))
        }
        PasscodeGenRequestMode::Remove => {
            let response = passcode::delete_ward_passcode(&payload.ward, &ctx.api_state.db_pool, &ctx.api_state.kphis()).await?;

            Ok(Json(if response > 0 {
                PasscodeGenResponse::new(Some(String::from("Removed")))
            } else {
                PasscodeGenResponse::new(None)
            }))
        }
    }
}
