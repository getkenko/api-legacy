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
    carbs: i32,
}

impl Macros {
    pub fn add(&mut self, product: &UserMealProduct) {
        self.calories += product.calories;
        self.proteins += product.proteins;
        self.fats += product.fats;
        self.carbs += product.carbohydrates;
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
    pub carbs: i32,
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
            carbohydrates: Some(product.carbs),
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
    pub calories: i32,
    pub proteins: i32,
    pub fats: i32,
    pub carbohydrates: i32,
}

impl From<UserMealProduct> for UserMealProductView {
    fn from(product: UserMealProduct) -> Self {
        Self {
            product_id: product.product_id,
            quantity: product.quantity,
            name: product.name.clone(),
            calories: product.calories,
            proteins: product.proteins,
            fats: product.fats,
            carbohydrates: product.carbohydrates,
        }
    }
}

#[derive(Deserialize)]
pub struct DeleteMealProductsQuery {
    pub section_id: Option<Uuid>,
}
