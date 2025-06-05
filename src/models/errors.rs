use axum::{http::StatusCode, response::{IntoResponse, Response}, Json};
use serde::Serialize;

// TODO: Print internal errors to the console

pub type AppResult<T> = Result<T, AppError>;

#[derive(Debug, thiserror::Error)]
pub enum AppError {
    #[error("Invalid email and password combination")]
    InvalidCredentials,
    #[error("This username is already taken")]
    UsernameTaken,
    #[error("This email address is already linked to an existing account")]
    EmailTaken,

    #[error("Internal database error")]
    Database(#[from] sqlx::Error),
    #[error("Internal cryptographic error")]
    Crypto(argon2::password_hash::Error),
}

impl AppError {
    fn status_code(&self) -> StatusCode {
        match self {
            Self::InvalidCredentials => StatusCode::UNAUTHORIZED,
            Self::UsernameTaken | Self::EmailTaken => StatusCode::CONFLICT,

            Self::Database(_) | Self::Crypto(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let code = self.status_code();
        let body = Json(ErrorResponse {
            code: code.as_u16(),
            error: self.to_string(),
        });

        (code, body).into_response()
    }
}

#[derive(Serialize)]
struct ErrorResponse {
    code: u16,
    error: String,
}
