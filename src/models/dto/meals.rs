#![deny(dead_code)]

use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::models::database::{enums::MealProductKind, meal::{InsertMealProduct, UserMealProduct}, meal::UserMealSection};

#[derive(Default, Serialize)]
pub struct Macros {
    calories: i32,
    proteins: i32,
    fats: i32,
    carbohydrates: i32,
}

impl Macros {
    pub fn add(&mut self, product: &UserMealProduct) {
        // if product is 'quick add' then we dont want to multiply the macros because they were provided 'as is'
        let mul = if product.product_id.is_some() {
            product.quantity as f32 / 100.0
        } else {
            1.0
        };

        self.calories += (product.calories as f32 * mul).round() as i32;
        self.proteins += (product.proteins as f32 * mul).round() as i32;
        self.fats += (product.fats as f32 * mul).round() as i32;
        self.carbohydrates += (product.carbohydrates as f32 * mul).round() as i32;
    }
}

#[derive(Serialize)]
pub struct MealDayMacrosResponse {
    #[serde(flatten)]
    pub per_section: HashMap<Uuid, Macros>,
    pub total: Macros,
}

#[derive(Serialize)]
pub struct UserMealSectionView {
    pub id: Uuid,
    pub index: i32,
    pub label: String,
}

impl From<UserMealSection> for UserMealSectionView {
    fn from(section: UserMealSection) -> Self {
        Self {
            id: section.id,
            index: section.index,
            label: section.label,
        }
    }
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AddMealProductRequest {
    pub section_id: Uuid,
    pub product_id: Uuid,
    pub quantity: i32,
}

impl From<AddMealProductRequest> for InsertMealProduct {
    fn from(product: AddMealProductRequest) -> Self {
        Self {
            section_id: product.section_id,
            quantity: product.quantity,
            kind: MealProductKind::FromDatabase,
            product_id: Some(product.product_id),
            name: None,
            calories: None,
            proteins: None,
            fats: None,
            carbohydrates: None,
        }
    }
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct QuickAddMealProductRequest {
    pub section_id: Uuid,
    pub name: String,
    pub calories: i32,
    pub proteins: i32,
    pub fats: i32,
    pub carbohydrates: i32,
    pub quantity: i32,
}

impl From<QuickAddMealProductRequest> for InsertMealProduct {
    fn from(product: QuickAddMealProductRequest) -> Self {
        Self {
            section_id: product.section_id,
            quantity: product.quantity,
            kind: MealProductKind::QuickAdd,
            product_id: None,
            name: Some(product.name),
            calories: Some(product.calories),
            proteins: Some(product.proteins),
            fats: Some(product.fats),
            carbohydrates: Some(product.carbohydrates),
        }
    }
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UserMealProductView {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_id: Option<Uuid>,
    pub quantity: i32,
    pub name: String,
<<<<<<< Updated upstream
    pub calories: i32,
    pub proteins: i32,
    pub fats: i32,
    pub carbohydrates: i32,
=======
    
    #[serde(flatten)]
    pub macros: Macros,
>>>>>>> Stashed changes
}

impl From<UserMealProduct> for UserMealProductView {
    fn from(product: UserMealProduct) -> Self {
        let mut macros = Macros::default();
        macros.add(&product);

        Self {
            product_id: product.product_id,
            quantity: product.quantity,
            name: product.name.clone(),
<<<<<<< Updated upstream
            calories: product.calories,
            proteins: product.proteins,
            fats: product.fats,
            carbohydrates: product.carbohydrates,
=======
            macros,
>>>>>>> Stashed changes
        }
    }
}

#[derive(Deserialize)]
pub struct DeleteMealProductsQuery {
    pub section_id: Option<Uuid>,
}
