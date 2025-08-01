use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::models::database::fridge::FridgeProduct;

#[derive(Deserialize)]
pub struct CreateFridgeProductDto {
    pub name: String,
    pub quantity: i32,
    pub expiration: Option<NaiveDate>,
}

#[derive(Serialize)]
pub struct FridgeProductDto {
    pub id: Uuid,
    pub name: String,
    pub quantity: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expiration: Option<NaiveDate>,
    pub created_at: DateTime<Utc>,
}

impl From<FridgeProduct> for FridgeProductDto {
    fn from(p: FridgeProduct) -> Self {
        Self {
            id: p.id,
            name: p.name,
            quantity: p.quantity,
            expiration: p.expiration,
            created_at: p.created_at,
        }
    }
}

#[derive(Deserialize)]
pub struct UpdateFridgeProductDto {
    pub name: Option<String>,
    pub quantity: Option<i32>,
    pub expiration: Option<NaiveDate>,
}