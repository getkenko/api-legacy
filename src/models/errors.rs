use axum::{extract::multipart, http::StatusCode, response::{IntoResponse, Response}, Json};
use convert_case::{Case, Casing};
use serde::Serialize;

use crate::{models::database::enums::AccountState, services::sections::USER_SECTION_LIMIT};

// TODO: Use constants for validation errors
// TODO: separate errors into smaller enums (e.g. ValidationError that is embedded into AppError)
// because some errors share the same status code therefore code can be cleaned

pub type AppResult<T> = Result<T, AppError>;

#[derive(Debug, thiserror::Error, strum_macros::AsRefStr, strum_macros::IntoStaticStr)]
pub enum AppError {
    // MIDDLEWARES
    #[error("You need to be signed in to access this resource")]
    Unathorized,
    #[error("Failed to parse authorization token because it contains invalid symbols")]
    TokenInvalidSymbols,
    #[error("Invalid authorization header format, does it begin with 'Bearer '?")]
    InvalidAuthFormat,
    #[error("You are being rate limited")]
    RateLimit,

    #[error("No fields provided to be updated")]
    NoFieldsToUpdate,

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

    #[error("Invalid weight provided, make sure you filled weightKg")]
    MissingKgWeight,
    #[error("Invalid weight provided, make sure you filled weightLb")]
    MissingLbWeight,
    #[error("Invalid weight provided, make sure you filled weightSt and weightLb")]
    MissingStLbWeight,

    #[error("Invalid height provided, make sure you filled heightCm")]
    MissingCmHeight,
    #[error("Invalid height provided, make sure you filled heightFt and heightIn")]
    MissingFtInHeight,

    #[error("Weight must be greater than zero")]
    NegativeWeight,
    #[error("Height must be greater than zero")]
    NegativeHeight,
    
    #[error("Invalid date of birth provided! It cannot be a date in future")]
    DateOfBirthInFuture,

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
    #[error("You have reached the maximum amount of meal products for this day")]
    MealProductLimitReached,
    #[error("Provided meal date is in the future")]
    MealDateInFuture,
    #[error("Empty meal product name provided")]
    MealProductEmptyName,
    #[error("The meal product macros must be non-negative number greater or equal to 0")]
    MealProductNegativeMacros,
    #[error("Meal product quantity cannot be less than one")]
    MealProductInvalidQuantity,

    // USERS
    #[error("Unknown file type uploaded, only JPEG and PNG files are accepted")]
    UnknownFileType,
    #[error("Invalid weight provided, it must be greater than zero")]
    InvalidWeight,
    #[error("Invalid height provided, it must be greater than zero")]
    InvalidHeight,

    // SECTIONS
    #[error("Section with this ID could not be found")]
    SectionNotFound,
    #[error("You have reached the maximum amount of sections")]
    SectionLimitReached,
    #[error("This index is already used by another section, please use unique one")]
    SectionIndexTaken,
    #[error("Invalid section index! It must be within the (0 - {USER_SECTION_LIMIT}) range")]
    InvalidSectionIndex,
    #[error("Invalid section name! It cannot be empty")]
    SectionHasEmptyName,

    // NUTRIENTS
    #[error("Macros distribution can't be below zero")]
    NegativeDistribution,
    #[error("Distribution sum above 100%")]
    DistributionAbove100,
    #[error("Distribution sum below 100%")]
    DistributionBelow100,
    #[error("Macronutrients must be greater or equal to 0")]
    NegativeMacrosTarget,

    // 3RD PARTY
    #[error("Internal database error")]
    Database(#[from] sqlx::Error),
    #[error("Internal redis error")]
    Redis(#[from] redis::RedisError),
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
            Self::UsernameTaken | Self::EmailTaken | Self::SectionIndexTaken => StatusCode::CONFLICT,
            Self::RateLimit => StatusCode::TOO_MANY_REQUESTS,

            Self::UnknownFileType | Self::BadUsernameLength | Self::InvalidUsername | Self::InvalidEmailFormat | Self::EmailTooLong |
            Self::BadPasswordLength | Self::PasswordNotEnoughSymbols | Self::PasswordNotEnoughDigits | Self::ActivityNotInRange(_) |
            Self::TokenInvalidSymbols | Self::InvalidAuthFormat | Self::MissingKgWeight | Self::MissingLbWeight |
            Self::MissingStLbWeight | Self::MissingCmHeight | Self::MissingFtInHeight | Self::NegativeHeight | Self::NegativeWeight |
            Self::NoFieldsToUpdate | Self::NegativeDistribution | Self::DistributionAbove100 | Self::NegativeMacrosTarget | Self::DistributionBelow100 |
            Self::DateOfBirthInFuture | Self::MealDateInFuture | Self::MealProductNegativeMacros | Self::MealProductEmptyName |
            Self::MealProductInvalidQuantity | Self::InvalidSectionIndex | Self::SectionHasEmptyName | Self::InvalidHeight |
            Self::InvalidWeight => StatusCode::BAD_REQUEST,

            Self::MealSectionNotFound | Self::MealProductNotFound | Self::ProductNotFound | Self::SectionNotFound => StatusCode::NOT_FOUND,
            Self::SectionLimitReached | Self::MealProductLimitReached => StatusCode::UNPROCESSABLE_ENTITY,

            Self::Io(_) | Self::Database(_) | Self::Redis(_) | Self::Crypto(_) | Self::Jwt(_) | Self::Multipart(_) => StatusCode::INTERNAL_SERVER_ERROR,
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
            status_code: code.as_u16(),
            error_id: self.as_ref().to_case(Case::Kebab),
            error: self.to_string(),
        });

        (code, body).into_response()
    }
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct ErrorResponse {
    status_code: u16,
    error_id: String,
    error: String,
}
