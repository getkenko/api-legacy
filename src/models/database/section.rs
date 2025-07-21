#![allow(dead_code)]

use chrono::{DateTime, Utc};
use uuid::Uuid;

pub struct SectionIcon {
    pub id: i32,
    pub emoji: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, sqlx::FromRow)]
pub struct UserSection {
    pub id: Uuid,
    pub user_id: Uuid,
    pub index: i32,
    pub icon: Option<String>,
    pub name: String,
}