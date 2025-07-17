use uuid::Uuid;

use crate::models::database::enums::Unit;

pub struct Product {
    pub id: Uuid,
    pub name: String,
    pub barcode: i64,
    pub ingredients: String,
    pub unit: Unit,
    pub quantity: i32,
    pub calories: i32,
    pub proteins: i32,
    pub fats: i32,
    pub carbohydrates: i32,
}