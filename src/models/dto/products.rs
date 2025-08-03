#![deny(dead_code)]

use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::models::database::{enums::Unit, product::Product};

#[derive(Deserialize)]
pub struct SearchProductQuery {
    pub query: String,
}

#[derive(Serialize)]
pub struct ProductView {
    pub id: Uuid,
    pub name: String,
    pub barcode: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub brand: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ingredients: Option<String>,
    pub unit: Unit,
    pub quantity: i32,
    pub calories: i32,
    pub proteins: i32,
    pub fats: i32,
    pub carbohydrates: i32,
}

impl From<Product> for ProductView {
    fn from(product: Product) -> Self {
        Self {
            id: product.id,
            name: product.name,
            barcode: product.barcode,
            brand: product.brand,
            ingredients: product.ingredients,
            unit: product.unit,
            quantity: product.quantity,
            calories: product.calories,
            proteins: product.proteins,
            fats: product.fats,
            carbohydrates: product.carbohydrates,
        }
    }
}