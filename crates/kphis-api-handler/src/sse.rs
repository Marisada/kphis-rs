use axum::{
    Json,
    extract::{Path, Query, State},
    http::HeaderValue,
    http::Method,
    response::{
        IntoResponse,
        sse::{Event, KeepAlive, Sse},
    },
};
use tokio::sync::mpsc;
use tokio_stream::{StreamExt, wrappers::UnboundedReceiverStream};
use tracing::warn;
use ulid::Ulid;

use kphis_api_core::{
    open_api::{DocOne, DocVec},
    state::{ApiState, RequestState, UserState},
};
use kphis_api_query::{sse, user::config};
use kphis_model::{
    fetch::ExecuteResponse,
    sse::{SseData, SseGroup, SseMessage, SseMessageParams, SsePostMessage},
};
use kphis_util::{
    datetime::now,
    error::{AppError, ErrorTitle},
};

pub async fn get_sse(State(app): State<ApiState>) -> impl IntoResponse {
    let (tx, rx) = mpsc::unbounded_channel();
    let rx = UnboundedReceiverStream::new(rx);

    if tx.send(SseMessage::Msg(String::from("Greeting"))).is_err() {
        warn!("Failed send Greeting SSE message");
    }

    app.sse_anonymous_insert(tx).await;

    let stream_map = rx.map(|msg| match msg {
        SseMessage::Msg(message) => Ok::<Event, axum::Error>(Event::default().data(message)),
        SseMessage::GlobalMsg(_) | SseMessage::WardMsg(_) | SseMessage::SpcltyMsg(_) | SseMessage::DirectMsg(_) | SseMessage::Logout(_) => Ok(Event::default().data("Wrong message")),
    });

    // Add "X-Accel-Buffering: no" to header to prevent proxy buffering
    let mut res = Sse::new(stream_map).keep_alive(KeepAlive::default()).into_response();
    res.headers_mut().insert("X-Accel-Buffering", HeaderValue::from_static("no"));
    res
}

/// /api/sse
///
/// Tries to logout user
#[utoipa::path(
    delete,
    path = "/sse",
    responses(DocOne<String>),
)]
pub async fn logout(ctx: RequestState) -> Result<Json<String>, AppError> {
    ctx.user_state.trace_req_by();
    ctx.authorize_and_access_log(&Method::DELETE, false).await?;

    let mut guard = ctx.api_state.online_users.lock().await;
    match guard.remove(&ctx.user_state.state_id) {
        Some(_) => Ok(Json(String::from("OK"))),
        None => Err(AppError::app_404("LogOut").with_title(ErrorTitle::NoUserState)),
    }
}

pub async fn get_sse_by_id(Path(state_id): Path<String>, State(app): State<ApiState>) -> impl IntoResponse {
    let (tx, rx) = mpsc::unbounded_channel();
    let rx = UnboundedReceiverStream::new(rx);

    if tx.send(SseMessage::Msg(String::from("Greeting"))).is_err() {
        warn!("Failed send Greeting SSE message");
    }

    let stream_map = rx.map(|msg| match msg {
        SseMessage::Msg(message) => Ok(Event::default().data(message)),
        SseMessage::GlobalMsg(data) => Event::default().event("globalMsg").json_data(data),
        SseMessage::WardMsg(data) => Event::default().event("wardMsg").json_data(data),
        SseMessage::SpcltyMsg(data) => Event::default().event("spcltyMsg").json_data(data),
        SseMessage::DirectMsg(data) => Event::default().event("directMsg").json_data(data),
        SseMessage::Logout(message) => Ok(Event::default().event("logout").data(message)),
    });

    if let Ok(Ulid(state_id)) = Ulid::from_string(&state_id) {
        match app.online_get(state_id).await {
            Some(UserState { user, .. }) => {
                if let Some(code) = &user.doctorcode {
                    app.sse_insert(code, tx).await;
                } else {
                    app.sse_anonymous_insert(tx).await;
                }

                // Add "X-Accel-Buffering: no" to header to prevent proxy buffering
                let mut res = Sse::new(stream_map).keep_alive(KeepAlive::default()).into_response();
                res.headers_mut().insert("X-Accel-Buffering", HeaderValue::from_static("no"));
                Ok(res)
            }
            None => Err(AppError::app_401("Get SSE")),
        }
    } else {
        Err(AppError::app_401("Get SSE"))
    }
}

/// /api/sse-group
///
/// Tries to set message group for requested user
#[utoipa::path(
    post,
    path = "/sse-group",
    request_body = SseGroup,
    responses(DocOne<String>),
)]
pub async fn post_sse_group(ctx: RequestState, Json(payload): Json<SseGroup>) -> Result<Json<String>, AppError> {
    if let Some(doctorcode) = &ctx.user_state.user.doctorcode {
        config::insert_dup_config_sse(&payload, &ctx.user_state.user.loginname, &ctx.api_state.db_pool, &ctx.api_state.kphis_extra()).await?;
        ctx.api_state.sse_wards_update(doctorcode, &payload.wards).await;
        ctx.api_state.sse_spcltys_update(doctorcode, &payload.spclty_ids).await;
        ctx.api_state.online_update_msg_group(ctx.user_state.state_id, &payload).await;

        Ok(Json(String::from("Ok")))
    } else {
        Ok(Json(String::from("No doctorcode")))
    }
}

/// /api/sse-message
///
/// Get old SseMessage, return list of SseMessage
#[utoipa::path(
    get,
    path = "/sse-message",
    responses(DocVec<SseMessage>),
    params(SseMessageParams),
)]
pub async fn get_sse_message(Query(params): Query<SseMessageParams>, ctx: RequestState) -> Result<Json<Vec<SseMessage>>, AppError> {
    ctx.user_state.trace_req_by();
    ctx.authorize_and_access_log(&Method::GET, false).await?;

    let response = sse::get_sse_message(
        &params,
        &ctx.user_state.user.loginname,
        &ctx.user_state.user.doctorcode,
        &ctx.user_state.user.wards,
        &ctx.user_state.user.spclty_ids,
        &ctx.api_state.db_pool,
        &ctx.api_state.kphis_log(),
    )
    .await?;

    Ok(Json(response))
}

/// /api/sse-message
///
/// Tries to send SSE sending request
#[utoipa::path(
    post,
    path = "/sse-message",
    request_body = SsePostMessage,
    responses(DocOne<String>),
)]
pub async fn post_sse_message(ctx: RequestState, Json(payload): Json<SsePostMessage>) -> Result<Json<String>, AppError> {
    let message_datetime = now();
    let insert_response = sse::post_sse_message(
        &payload,
        message_datetime,
        &ctx.user_state.user.doctorcode,
        &ctx.user_state.user.name,
        &ctx.api_state.db_pool,
        &ctx.api_state.kphis_log(),
    )
    .await?;

    let message_id = insert_response.last_insert_id as u32;

    // SSE always send to everyone including sender but
    let data = SseData::from_post_message(&payload, message_id, message_datetime, &ctx.user_state.user.doctorcode, &ctx.user_state.user.name, false);
    let SsePostMessage { person, ward, spclty_id, .. } = payload;

    if let Some(t_person) = &person
        && !t_person.is_empty()
    {
        ctx.api_state.sse_direct_msg(t_person, &data).await;
    }
    if let Some(t_ward) = &ward {
        ctx.api_state.sse_ward_msg(t_ward, &data).await;
    }
    if let Some(t_spclty) = &spclty_id {
        ctx.api_state.sse_spclty_msg(*t_spclty, &data).await;
    }
    if person.is_none() && ward.is_none() && spclty_id.is_none() {
        ctx.api_state.sse_global_msg(&data).await;
    }

    Ok(Json(String::from("OK")))
}

/// /api/sse-message
///
/// Tries to mark SSE message_ids as readed
#[utoipa::path(
    patch,
    path = "/sse-message",
    request_body = Vec<u32>,
    responses(DocOne<ExecuteResponse>),
)]
pub async fn patch_sse_messages(ctx: RequestState, Json(payload): Json<Vec<u32>>) -> Result<Json<ExecuteResponse>, AppError> {
    ctx.user_state.trace_req_by();
    ctx.authorize_and_access_log(&Method::PATCH, false).await?;

    let result = sse::patch_sse_messages(&ctx.user_state.user.loginname, &payload, &ctx.api_state.db_pool, &ctx.api_state.kphis_log()).await?;

    Ok(Json(result))
}
