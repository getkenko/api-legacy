use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, PartialEq, Serialize, Deserialize, sqlx::Type, strum_macros::Display)]
#[serde(rename_all = "camelCase")]
#[sqlx(type_name = "account_state_enum", rename_all = "snake_case")]
pub enum AccountState {
    Active,
    Suspended,
    Deleted,
    Inactive,
}

pub struct User {
    pub id: Uuid,
    pub username: String,
    pub display_name: String,
    pub email: String,
    pub password: String,
    pub is_male: bool,
    pub date_of_birth: NaiveDate,
    pub avatar_url: Option<String>,
    pub account_state: AccountState,
    pub created_at: DateTime<Utc>,
}
