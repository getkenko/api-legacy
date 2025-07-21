#![deny(dead_code)]

use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::models::database::{enums::{MealProductKind, Unit}, meal::{InsertMealProduct, UserMealProduct}, section::UserSection};

const DEFAULT_SECTION_ICON: &str = "❓";

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
pub struct UserSectionView {
    pub id: Uuid,
    pub index: i32,
    pub icon: String,
    pub name: String,
}

impl From<UserSection> for UserSectionView {
    fn from(section: UserSection) -> Self {
        Self {
            id: section.id,
            index: section.index,
            icon: section.icon.unwrap_or(DEFAULT_SECTION_ICON.to_string()),
            name: section.name,
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
            unit: None,
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
    pub unit: Unit,
}

impl From<QuickAddMealProductRequest> for InsertMealProduct {
    fn from(product: QuickAddMealProductRequest) -> Self {
        Self {
            section_id: product.section_id,
            quantity: product.quantity,
            unit: Some(product.unit),
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
    pub id: Uuid,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_id: Option<Uuid>,
    pub unit: Unit,
    pub quantity: i32,
    pub name: String,
    #[serde(flatten)]
    pub macros: Macros,
}

impl From<UserMealProduct> for UserMealProductView {
    fn from(product: UserMealProduct) -> Self {
        let mut macros = Macros::default();
        macros.add(&product);

        Self {
            id: product.id,
            product_id: product.product_id,
            unit: product.unit,
            quantity: product.quantity,
            name: product.name.clone(),
            macros,
        }
    }
}

#[derive(Deserialize)]
pub struct DeleteMealProductsQuery {
    pub section_id: Option<Uuid>,
}
