use axum::Json;
use axum::body::Body;
use axum::extract::Request;
use axum::extract::State;
use axum::http::{HeaderMap, StatusCode};
use axum::middleware::Next;
use axum::response::{IntoResponse, Response};
use packrat_application::UserSessionQueryPort;
use packrat_domain::aggregates::user_session::TokenHash;
use packrat_domain::user::UserId;

use crate::dto::ErrorBody;
use crate::state::AppState;

#[derive(Clone, Copy, Debug)]
pub struct AuthSession {
    pub user_id: UserId,
}

fn parse_bearer_token(headers: &HeaderMap) -> Option<String> {
    let value = headers
        .get(axum::http::header::AUTHORIZATION)?
        .to_str()
        .ok()?;
    let rest = value
        .strip_prefix("Bearer ")
        .or_else(|| value.strip_prefix("bearer "))?;
    let token = rest.trim();
    if token.is_empty() {
        return None;
    }
    Some(token.to_string())
}

pub async fn require_session(
    State(state): State<AppState>,
    mut request: Request<Body>,
    next: Next,
) -> Response {
    let unauthorized = (
        StatusCode::UNAUTHORIZED,
        Json(ErrorBody::message(
            "missing or invalid Authorization: Bearer <token>".to_string(),
        )),
    )
        .into_response();

    let Some(raw) = parse_bearer_token(request.headers()) else {
        return unauthorized;
    };

    let Some(token_hash) = TokenHash::from_login_token_hex(&raw) else {
        return unauthorized;
    };

    let session = match state
        .user_session_query
        .as_ref()
        .get_by_token(&token_hash)
        .await
    {
        Ok(Some(s)) => s,
        Ok(None) => return unauthorized,
        Err(e) => {
            tracing::error!(error = %e, "session lookup failed");
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorBody::message(e.to_string())),
            )
                .into_response();
        }
    };

    request.extensions_mut().insert(AuthSession {
        user_id: session.user_id,
    });

    next.run(request).await
}
