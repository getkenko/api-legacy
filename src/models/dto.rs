#![deny(dead_code)]

use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};

use super::database::{Language, Theme};

// AUTH
#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LoginCredentials {
    pub email: String,
    pub password: String,
}

#[derive(Serialize)]
pub struct LoginResponse {
    pub access: String,
    pub refresh: String,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RegisterUserData {
    pub username: String,
    pub email: String,
    pub password: String,
    pub is_male: bool,
    pub weight: f32,
    pub height: i32,
    pub date_of_birth: NaiveDate,
}

// USERS
#[derive(Serialize)]
pub struct UserInfo {
    // user
    pub username: String,
    pub display_name: String,
    pub email: String,
    pub avatar_url: Option<String>,
    pub created_at: DateTime<Utc>,

    // details
    pub is_male: bool,
    pub weight: f32,
    pub height: i32,
    pub date_of_birth: NaiveDate,

    // preferences
    pub theme: Theme,
    pub language: Language,
}

#[derive(Deserialize)]
pub struct NewUserDetails {
    pub is_male: Option<bool>,
    pub weight: Option<f32>,
    pub height: Option<i32>,
    pub date_of_birth: Option<NaiveDate>,
}

#[derive(Deserialize)]
pub struct NewUserPreferences {
    pub theme: Option<Theme>,
    pub language: Option<Language>,
}
