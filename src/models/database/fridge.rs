use chrono::{DateTime, NaiveDate, Utc};
use uuid::Uuid;

#[allow(dead_code)]
pub struct FridgeProduct {
    pub id: Uuid,
    pub user_id: Uuid,
    pub name: String,
    pub quantity: i32,
    pub expiration: Option<NaiveDate>,
    pub created_at: DateTime<Utc>,
}