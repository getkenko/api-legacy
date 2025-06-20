use axum::{http::StatusCode, response::{IntoResponse, Response}, Json};
use serde::Serialize;

use crate::models::database::AccountState;

// TODO: Use constants for validation errors

pub type AppResult<T> = Result<T, AppError>;

#[derive(Debug, thiserror::Error)]
pub enum AppError {
    #[error("You need to be signed in to access this resource")]
    Unathorized,

    // AUTH
    #[error("Invalid email and password combination")]
    InvalidCredentials,
    #[error("Your account is {0}")]
    AccountNotActive(AccountState),

    #[error("This username is already taken")]
    UsernameTaken,
    #[error("This email address is already linked to an existing account")]
    EmailTaken,

    // VALIDATION
    #[error("Bad username length! It must be between 4 and 16")]
    BadUsernameLength,
    #[error("Invalid username provided, it should consist of alphanumeric characters")]
    InvalidUsername,

    #[error("Email address has invalid format")]
    InvalidEmailFormat,
    #[error("The provided email address is too long")]
    EmailTooLong,

    #[error("Bad password length. It must be between 8 and 2048 characters")]
    BadPasswordLength,
    #[error("Password requires at least 2 special characters")]
    PasswordNotEnoughSymbols,
    #[error("Password requires at least 3 digits")]
    PasswordNotEnoughDigits,

    // MEALS
    #[error("Meal section with this ID not found")]
    MealSectionNotFound,
    #[error("Meal product with this ID not found")]
    MealProductNotFound,

    // 3RD PARTY
    #[error("Internal database error")]
    Database(#[from] sqlx::Error),
    #[error("Internal cryptographic error")]
    Crypto(argon2::password_hash::Error),
    #[error("Internal jwt error")]
    Jwt(#[from] jsonwebtoken::errors::Error),
}

impl AppError {
    fn status_code(&self) -> StatusCode {
        match self {
            Self::Unathorized | Self::InvalidCredentials | Self::AccountNotActive(_) => StatusCode::UNAUTHORIZED,
            Self::UsernameTaken | Self::EmailTaken => StatusCode::CONFLICT,

            Self::BadUsernameLength | Self::InvalidUsername | Self::InvalidEmailFormat | Self::EmailTooLong | Self::BadPasswordLength | Self::PasswordNotEnoughSymbols | Self::PasswordNotEnoughDigits => StatusCode::BAD_REQUEST,

            Self::MealSectionNotFound | Self::MealProductNotFound => StatusCode::NOT_FOUND,

            Self::Database(_) | Self::Crypto(_) | Self::Jwt(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let code = self.status_code();

        if code == StatusCode::INTERNAL_SERVER_ERROR {
            eprintln!("ERROR: {self:?}");
        }

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
