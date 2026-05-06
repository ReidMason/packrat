use axum::Json;
use axum::extract::State;
use axum::http::StatusCode;
use packrat_application::{UserCommandError, create_user};
use packrat_domain::user::Email;

use crate::dto::{CreateUserDto, ErrorBody, SuccessBody, UserDto};
use crate::state::AppState;

pub async fn create_user_handler(
    State(state): State<AppState>,
    Json(body): Json<CreateUserDto>,
) -> Result<(StatusCode, Json<SuccessBody<UserDto>>), (StatusCode, Json<ErrorBody>)> {
    let email = body.email.trim();
    if email.is_empty() {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(ErrorBody::message("email must not be empty")),
        ));
    }
    if !email.contains('@') {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(ErrorBody::message("email must contain @")),
        ));
    }

    match create_user(state.user_command.as_ref(), Email::from(email)).await {
        Ok(user) => Ok((
            StatusCode::CREATED,
            Json(SuccessBody::new(UserDto::from_user(user))),
        )),
        Err(UserCommandError::DuplicateEmail) => Err((
            StatusCode::CONFLICT,
            Json(ErrorBody::message("email already registered")),
        )),
        Err(UserCommandError::Persist(msg)) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ErrorBody::message(msg)),
        )),
    }
}
