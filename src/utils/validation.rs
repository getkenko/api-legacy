use chrono::{NaiveDate, TimeDelta, Utc};
use regex::Regex;

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
    // check if email is empty or doesn't contain '@'
    if email.is_empty() || !email.contains('@') {
        return Err(AppError::InvalidEmailFormat);
    }

    // split domain and user part
    let parts = email.split('@').collect::<Vec<_>>();
    let (user_part, domain_part) = (parts[0], parts[1]);

    // validate length of each part
    if user_part.len() > 64 || domain_part.len() > 255 {
        return Err(AppError::EmailTooLong);
    }

    // check user part w/ regex
    let user_regex = Regex::new(r"^[a-zA-Z0-9.!#$%&'*+/=?^_`{|}~-]+\z").unwrap();
    if !user_regex.is_match(user_part) {
        return Err(AppError::InvalidEmailFormat);
    }

    // validate domain part
    let domain_regex = Regex::new(r"^[a-zA-Z0-9](?:[a-zA-Z0-9-]{0,61}[a-zA-Z0-9])?(?:\.[a-zA-Z0-9](?:[a-zA-Z0-9-]{0,61}[a-zA-Z0-9])?)*$").unwrap();
    if !domain_regex.is_match(domain_part) {
        return Err(AppError::InvalidEmailFormat);
    }

    Ok(())
}

// TODO: check if password contains at least one uppercase and lowercase letter
pub fn validate_password(password: &str) -> AppResult<()> {
    // length: 8 < x < 2048
    // contains: 2 symbols, 3 digits

    if password.len() < 8 || password.len() > 2048 {
        return Err(AppError::BadPasswordLength);
    }

    let symbols = password
        .chars()
        .filter(|c| {
            c.is_ascii_punctuation() ||
            "!@#$%^&*()-_=+[]{}|;:'\",.<>?/\\`~".contains(*c)
        })
        .count();
    let digits = password.chars().filter(|c| c.is_ascii_digit()).count();

    if symbols < 2 {
        return Err(AppError::PasswordNotEnoughSymbols);
    } else if digits < 3 {
        return Err(AppError::PasswordNotEnoughDigits);
    }

    Ok(())
}

pub fn validate_activity(idle: i32, workout: i32) -> AppResult<()> {
    if idle < 1 || idle > 5 {
        return Err(AppError::ActivityNotInRange("Idle".to_string()));
    } else if workout < 1 || workout > 5 {
        return Err(AppError::ActivityNotInRange("Workout".to_string()));
    }

    Ok(())
}

/// Validates provided date of birth, minimum 208 weeks (~4 years) in past
pub fn validate_date_of_birth(dob: NaiveDate) -> AppResult<()> {
    if Utc::now().date_naive() - dob < TimeDelta::weeks(208) { // 208 weeks ~ 4 years
        return Err(AppError::DateOfBirthInFuture);
    }

    Ok(())
}

/// Validates if provided date is in the past or today
pub fn validate_meal_date(date: NaiveDate) -> AppResult<()> {
    if Utc::now().date_naive() - date <= TimeDelta::days(0) {
        return Err(AppError::MealDateInFuture);
    }

    Ok(())
}
