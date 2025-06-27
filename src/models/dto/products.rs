use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::models::database::product::Product;

#[derive(Deserialize)]
pub struct SearchProductQuery {
    pub query: String,
}

#[derive(Serialize)]
pub struct ProductView {
    pub id: Uuid,
    pub name: String,
    pub barcode: i32,
    pub ingredients: String,
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
            ingredients: product.ingredients,
            calories: product.calories,
            proteins: product.proteins,
            fats: product.fats,
            carbohydrates: product.carbohydrates,
        }
    }
}