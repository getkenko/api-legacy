use uuid::Uuid;

use crate::models::database::enums::MealProductKind;

pub struct UserMealProduct {
    pub section_id: Uuid,
    pub product_id: Option<Uuid>,
    pub quantity: i32,
    pub name: String,
    pub calories: i32,
    pub proteins: i32,
    pub fats: i32,
    pub carbohydrates: i32,
}

#[allow(dead_code)]
pub struct UserMealSection {
    pub id: Uuid,
    pub user_id: Uuid,
    pub index: i32,
    pub label: String,
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

/*
impl AddMealProduct {
    pub fn from_database(
        date: NaiveDate,
        section_id: Uuid,
        quantity: i32,
        product_id: Uuid,
    ) -> Self {
        Self {
            date,
            section_id,
            quantity,
            kind: MealProductKind::FromDatabase,

            product_id: Some(product_id),

            name: None,
            calories: None,
            proteins: None,
            fats: None,
            carbohydrates: None,
        }
    }

    #[allow(clippy::too_many_arguments)]
    pub fn quick_add(
        date: NaiveDate,
        section_id: Uuid,
        quantity: i32,
        name: String,
        calories: i32,
        proteins: i32,
        fats: i32,
        carbohydrates: i32,
    ) -> Self {
        Self {
            date,
            section_id,
            quantity,
            kind: MealProductKind::QuickAdd,

            product_id: None,

            name: Some(name),
            calories: Some(calories),
            proteins: Some(proteins),
            fats: Some(fats),
            carbohydrates: Some(carbohydrates),
        }
    }
}
*/