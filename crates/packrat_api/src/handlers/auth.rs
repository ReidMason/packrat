use axum::Json;
use axum::extract::State;
use axum::http::StatusCode;
use packrat_application::{LoginResponse, LoginUseCase};

use crate::dto::{ErrorBody, LoginDto, LoginRequestDto, SuccessBody};
use crate::state::AppState;

pub async fn login_handler(
    State(state): State<AppState>,
    Json(body): Json<LoginRequestDto>,
) -> Result<(StatusCode, Json<SuccessBody<LoginDto>>), (StatusCode, Json<ErrorBody>)> {
    let email = body.email.trim();
    let password = body.password.trim();
    if email.is_empty() || password.is_empty() {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(ErrorBody::message(
                "email and password must not be empty".to_string(),
            )),
        ));
    }

    let use_case = LoginUseCase::new(
        state.user_query.as_ref().clone(),
        state.user_session_command.as_ref().clone(),
    );

    match use_case
        .execute(email.to_string(), password.to_string())
        .await
    {
        Ok(LoginResponse { user_id, token }) => Ok((
            StatusCode::OK,
            Json(SuccessBody::new(LoginDto { user_id, token })),
        )),
        Err(msg) if msg == "Invalid email" || msg == "Invalid password" => Err((
            StatusCode::UNAUTHORIZED,
            Json(ErrorBody::message(
                "Invalid email or password".to_string(),
            )),
        )),
        Err(msg) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ErrorBody::message(msg)),
        )),
    }
}
