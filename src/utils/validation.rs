use crate::models::errors::{AppError, AppResult};

pub fn validate_username(username: &str) -> AppResult<()> {
    // length: 4 < x < 16
    // chars: a-Z, 0-9

    if username.len() < 4 || username.len() > 16 {
        return Err(AppError::BadUsernameLength);
    }

    let alphanumeric = username.chars().all(|c| c.is_alphanumeric());
    if !alphanumeric {
        return Err(AppError::InvalidUsername);
    }

    Ok(())
}

pub fn validate_email(email: &str) -> AppResult<()> {
    Ok(())
}

pub fn validate_password(password: &str) -> AppResult<()> {
    Ok(())
}
