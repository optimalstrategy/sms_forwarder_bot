use std::net::IpAddr;

use axum::{
    body::{Body, Bytes},
    extract::{Json, Query},
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::{get, post},
    Extension, Router,
};
use axum_client_ip::SecureClientIp;
use axum_limit::{Key, LimitPerSecond, LimitState};
use serde::Deserialize;
use teloxide::prelude::Requester;

use crate::setup::AppContext;

pub fn api_router() -> Router {
    Router::new()
        .route("/-/__heartbeat__", get(async || "Alive"))
        .route("/check_user", get(check_user))
        .route("/forward", post(forward_sms))
        .with_state(LimitState::<ClientIP>::default())
}

#[derive(Deserialize)]
struct UsernameAndCodeQueryParams {
    username: String,
    code: String,
}

async fn check_user(
    context: Extension<AppContext>,
    Query(UsernameAndCodeQueryParams { username, code }): Query<UsernameAndCodeQueryParams>,
    _: LimitPerSecond<7, ClientIP>,
) -> Result<impl IntoResponse, AppError> {
    let mut conn = context.db.acquire().await?;
    let user = crate::queries::find_user_by_username(&mut *conn, &username).await?;

    if let Some(user) = user.as_ref() {
        if user.codes.contains(&code) {
            return Ok((StatusCode::OK, "OK"));
        }
    }

    Ok((StatusCode::NOT_FOUND, "Invalid code or username."))
}

#[derive(Deserialize)]
struct ForwardedSmsBody {
    /// The phone number of the sender
    address: Option<String>,
    /// The timestamp when the message was received
    date: Option<String>,
    /// The content of the message
    body: Option<String>,
}

impl ForwardedSmsBody {
    pub fn to_message(&self) -> String {
        let address = self.address.as_deref().unwrap_or("<>");
        let body = self.body.as_deref().unwrap_or("<>");
        let date = self.date.as_deref().unwrap_or("<>");
        format!("New SMS message from '{address}':\n{body}\n\nDate: {date}.")
    }
}

async fn forward_sms(
    context: Extension<AppContext>,
    _: LimitPerSecond<7, ClientIP>,
    Query(UsernameAndCodeQueryParams { username, code }): Query<UsernameAndCodeQueryParams>,
    raw_body: Bytes,
) -> Result<impl IntoResponse, AppError> {
    let Ok(Json(body)) = Json::<ForwardedSmsBody>::from_bytes(&raw_body) else {
        return Ok((StatusCode::UNPROCESSABLE_ENTITY, "Invalid JSON body."));
    };

    let mut conn = context.db.acquire().await?;
    let Some(user) = crate::queries::find_user_by_username(&mut *conn, &username).await? else {
        return Ok((StatusCode::NOT_FOUND, "Invalid code or username."));
    };

    if !user.codes.contains(&code) {
        return Ok((StatusCode::NOT_FOUND, "Invalid code or username."));
    }

    context
        .bot
        .send_message(user.chat_id, body.to_message())
        .await?;

    Ok((StatusCode::OK, ""))
}

#[allow(unused)]
struct AppError(anyhow::Error);

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        tracing::error!("{:?}", self.0);
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            "An unexpected error has occurred.",
        )
            .into_response()
    }
}

impl<E> From<E> for AppError
where
    E: Into<anyhow::Error>,
{
    fn from(err: E) -> Self {
        Self(err.into())
    }
}

#[derive(Clone, Copy, Hash, PartialEq, Eq)]
struct ClientIP(IpAddr);

impl Key for ClientIP {
    type Extractor = SecureClientIp;

    fn from_extractor(extractor: &Self::Extractor) -> Self {
        Self(extractor.0)
    }
}
