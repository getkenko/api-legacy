use axum::{extract::multipart, http::StatusCode, response::{IntoResponse, Response}, Json};
use serde::Serialize;

use crate::models::database::AccountState;

// TODO: Use constants for validation errors

pub type AppResult<T> = Result<T, AppError>;

#[derive(Debug, thiserror::Error)]
pub enum AppError {
    #[error("You need to be signed in to access this resource")]
    Unathorized,
    #[error("Failed to parse authorization token because it contains invalid symbols")]
    TokenInvalidSymbols,
    #[error("Invalid authorization header format, does it begin with 'Bearer '?")]
    InvalidAuthFormat,

    // AUTH
    #[error("Invalid email and password combination")]
    InvalidCredentials,
    #[error("Your account is {0}")]
    AccountNotActive(AccountState),
    #[error("This username is already taken")]
    UsernameTaken,
    #[error("This email address is already linked to an existing account")]
    EmailTaken,
    #[error("{0} activity has an invalid value, it must be between 1 and 5")]
    ActivityNotInRange(String),

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

    // PRODUCTS
    #[error("No product found")]
    ProductNotFound,

    // MEALS
    #[error("Meal section with this ID not found")]
    MealSectionNotFound,
    #[error("Meal product with this ID not found")]
    MealProductNotFound,

    // USERS
    #[error("Unknown file type uploaded, only JPEG and PNG files are accepted")]
    UnknownFileType,

    // 3RD PARTY
    #[error("Internal database error")]
    Database(#[from] sqlx::Error),
    #[error("Internal cryptographic error")]
    Crypto(argon2::password_hash::Error),
    #[error("Internal jwt error")]
    Jwt(#[from] jsonwebtoken::errors::Error),
    #[error("Internal error when receiving multipart form")]
    Multipart(#[from] multipart::MultipartError),
    #[error("Internal I/O error occured")]
    Io(#[from] std::io::Error),
}

impl AppError {
    fn status_code(&self) -> StatusCode {
        match self {
            Self::Unathorized | Self::InvalidCredentials | Self::AccountNotActive(_) => StatusCode::UNAUTHORIZED,
            Self::UsernameTaken | Self::EmailTaken => StatusCode::CONFLICT,

            Self::UnknownFileType | Self::BadUsernameLength | Self::InvalidUsername | Self::InvalidEmailFormat | Self::EmailTooLong |
            Self::BadPasswordLength | Self::PasswordNotEnoughSymbols | Self::PasswordNotEnoughDigits | Self::ActivityNotInRange(_) |
            Self::TokenInvalidSymbols | Self::InvalidAuthFormat => StatusCode::BAD_REQUEST,

            Self::MealSectionNotFound | Self::MealProductNotFound | Self::ProductNotFound => StatusCode::NOT_FOUND,

            Self::Io(_) | Self::Database(_) | Self::Crypto(_) | Self::Jwt(_) | Self::Multipart(_) => StatusCode::INTERNAL_SERVER_ERROR,
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
