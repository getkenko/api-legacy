use uuid::Uuid;

use crate::models::database::enums::MealProductKind;

pub struct UserMealProduct {
    pub id: Uuid,
    pub section_id: Uuid,
    pub product_id: Option<Uuid>,
    pub quantity: i32,
    pub name: String,
    pub calories: i32,
    pub proteins: i32,
    pub fats: i32,
    pub carbohydrates: i32,
}

pub struct InsertMealProduct {
    pub section_id: Uuid,
    pub quantity: i32,
    pub kind: MealProductKind,

    pub product_id: Option<Uuid>,

    pub name: Option<String>,
    pub calories: Option<i32>,
    pub proteins: Option<i32>,
    pub fats: Option<i32>,
    pub carbohydrates: Option<i32>,
}

#[allow(dead_code)]
#[derive(sqlx::FromRow)]
pub struct UserMealSection {
    pub id: Uuid,
    pub user_id: Uuid,
    pub index: i32,
    pub label: String,
}
