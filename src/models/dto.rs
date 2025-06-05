use chrono::NaiveDate;
use serde::Deserialize;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LoginCredentials {
    pub email: String,
    pub password: String,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RegisterUserData {
    pub username: String,
    pub display_name: String,
    pub email: String,
    pub password: String,
    pub is_male: bool,
    pub date_of_birth: NaiveDate,
}
