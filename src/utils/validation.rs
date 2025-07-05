use chrono::{NaiveDate, TimeDelta, Utc};
use regex::Regex;

use crate::models::errors::{AppResult, ValidationError};

pub const MIN_USERNAME_LEN: usize = 4;
pub const MAX_USERNAME_LEN: usize = 16;

pub const MAX_EMAIL_USER_LEN: usize = 64;
pub const MAX_EMAIL_DOMAIN_LEN: usize = 255;

pub const MIN_PASSWORD_LEN: usize = 8;
pub const MAX_PASSWORD_LEN: usize = 1024;
pub const PASSWORD_SYMBOLS: usize = 2;
pub const PASSWORD_DIGITS: usize = 3;

pub const MIN_ACTIVITY: i32 = 1;
pub const MAX_ACTIVITY: i32 = 5;

pub fn validate_username(username: &str) -> AppResult<()> {
    if username.len() < MIN_USERNAME_LEN || username.len() > MAX_USERNAME_LEN {
        return Err(ValidationError::BadUsernameLength)?;
    }

    let alphanumeric = username.chars().all(|c| c.is_alphanumeric());
    if !alphanumeric {
        return Err(ValidationError::InvalidUsername)?;
    }

    Ok(())
}

pub fn validate_email(email: &str) -> AppResult<()> {
    // check if email is empty or doesn't contain '@'
    if email.is_empty() || !email.contains('@') {
        return Err(ValidationError::InvalidEmailFormat)?;
    }

    // split domain and user part
    let parts = email.split('@').collect::<Vec<_>>();
    let (user_part, domain_part) = (parts[0], parts[1]);

    // validate length of each part
    if user_part.len() > MAX_EMAIL_USER_LEN || domain_part.len() > MAX_EMAIL_DOMAIN_LEN {
        return Err(ValidationError::EmailTooLong)?;
    }

    // check user part w/ regex
    let user_regex = Regex::new(r"^[a-zA-Z0-9.!#$%&'*+/=?^_`{|}~-]+\z").unwrap();
    if !user_regex.is_match(user_part) {
        return Err(ValidationError::InvalidEmailFormat)?;
    }

    // validate domain part
    let domain_regex = Regex::new(r"^[a-zA-Z0-9](?:[a-zA-Z0-9-]{0,61}[a-zA-Z0-9])?(?:\.[a-zA-Z0-9](?:[a-zA-Z0-9-]{0,61}[a-zA-Z0-9])?)*$").unwrap();
    if !domain_regex.is_match(domain_part) {
        return Err(ValidationError::InvalidEmailFormat)?;
    }

    Ok(())
}

// TODO: check if password contains at least one uppercase and lowercase letter
pub fn validate_password(password: &str) -> AppResult<()> {
    if password.len() < MIN_PASSWORD_LEN || password.len() > MAX_PASSWORD_LEN {
        return Err(ValidationError::BadPasswordLength)?;
    }

    let symbols = password
        .chars()
        .filter(|c| {
            c.is_ascii_punctuation() ||
            "!@#$%^&*()-_=+[]{}|;:'\",.<>?/\\`~".contains(*c)
        })
        .count();
    let digits = password.chars().filter(|c| c.is_ascii_digit()).count();

    if symbols < PASSWORD_SYMBOLS {
        return Err(ValidationError::PasswordNotEnoughSymbols)?;
    }
    if digits < PASSWORD_DIGITS {
        return Err(ValidationError::PasswordNotEnoughDigits)?;
    }

    Ok(())
}

pub fn validate_activity(idle: i32, workout: i32) -> AppResult<()> {
    if idle < MIN_ACTIVITY || idle > MAX_ACTIVITY {
        return Err(ValidationError::ActivityNotInRange("Idle".to_string()))?;
    }
    
    if workout < MIN_ACTIVITY || workout > MAX_ACTIVITY {
        return Err(ValidationError::ActivityNotInRange("Workout".to_string()))?;
    }

    Ok(())
}

/// Validates provided date of birth, minimum 208 weeks (~4 years) in past
pub fn validate_date_of_birth(dob: NaiveDate) -> AppResult<()> {
    if Utc::now().date_naive() - dob < TimeDelta::weeks(208) { // 208 weeks ~ 4 years
        return Err(ValidationError::DateOfBirthInFuture)?;
    }

    Ok(())
}

/// Validates if provided date is in the past or today
pub fn validate_meal_date(date: NaiveDate) -> AppResult<()> {
    if Utc::now().date_naive() - date <= TimeDelta::days(0) {
        return Err(ValidationError::MealDateInFuture)?;
    }

    Ok(())
}
