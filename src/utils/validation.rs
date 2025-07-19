use chrono::{NaiveDate, TimeDelta, Utc};
use regex::Regex;

use crate::models::errors::{AppResult, ValidationError};

pub const MIN_USERNAME_LEN: usize = 2;
pub const MAX_USERNAME_LEN: usize = 32;

pub const MAX_EMAIL_USER_LEN: usize = 64;
pub const MAX_EMAIL_DOMAIN_LEN: usize = 255;

pub const MIN_PASSWORD_LEN: usize = 8;
pub const MAX_PASSWORD_LEN: usize = 1024;

pub const MIN_ACTIVITY: i32 = 1;
pub const MAX_ACTIVITY: i32 = 5;

pub fn validate_username(username: &str) -> AppResult<()> {
    if username.len() < MIN_USERNAME_LEN || username.len() > MAX_USERNAME_LEN {
        return Err(ValidationError::BadUsernameLength)?;
    }

    let valid_chars = username.chars().all(|c| c.is_alphanumeric() || c == '.' || c == '_');
    if !valid_chars || username.starts_with('.') || username.starts_with('_') {
        return Err(ValidationError::InvalidUsername)?;
    }

    let mut prev = None;
    for c in username.chars() {
        if let Some(p) = prev {
            if c == '.' && c == p {
                return Err(ValidationError::InvalidUsername)?;
            }
        }
        prev = Some(c);
    }

    Ok(())
}

pub fn validate_email(email: &str) -> AppResult<()> {
    let parts = email.split('@').collect::<Vec<_>>();
    let (user_part, domain_part) = (parts[0], parts[1]);

    // validate length of each part
    if user_part.len() > MAX_EMAIL_USER_LEN || domain_part.len() > MAX_EMAIL_DOMAIN_LEN {
        return Err(ValidationError::EmailTooLong)?;
    }

    let regex = Regex::new(r"^[^\s@]+@[^\s@]+\.[^\s@]+$").unwrap();
    if !regex.is_match(email) {
        return Err(ValidationError::InvalidEmailFormat)?;
    }

    Ok(())
}

pub fn validate_password(password: &str) -> AppResult<()> {
    if password.len() < MIN_PASSWORD_LEN || password.len() > MAX_PASSWORD_LEN {
        return Err(ValidationError::BadPasswordLength)?;
    }

    Ok(())
}

pub fn is_activity_in_range(activity: i32) -> bool {
    activity >= MIN_ACTIVITY && activity <= MAX_ACTIVITY
}

/// Validates provided date of birth, minimum 208 weeks (~4 years) in past
pub fn validate_date_of_birth(dob: NaiveDate) -> AppResult<()> {
    if Utc::now().date_naive() - dob < TimeDelta::weeks(208) { // 208 weeks ~ 4 years
        return Err(ValidationError::DateOfBirthInFuture)?;
    }

    Ok(())
}
