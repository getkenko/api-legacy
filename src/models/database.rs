use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

// ENUMS
#[derive(Debug, PartialEq, Serialize, Deserialize, sqlx::Type, strum_macros::Display)]
#[serde(rename_all = "camelCase")]
#[sqlx(type_name = "account_state_enum", rename_all = "snake_case")]
pub enum AccountState {
    Active,
    Suspended,
    Deleted,
    Inactive,
}

#[derive(Serialize, Deserialize, sqlx::Type)]
#[serde(rename_all = "camelCase")]
#[sqlx(type_name = "theme_enum", rename_all = "snake_case")]
pub enum Theme {
    System,
    Dark,
    Light,
}

#[derive(Serialize, Deserialize, sqlx::Type)]
#[serde(rename_all = "camelCase")]
#[sqlx(type_name = "language_enum", rename_all = "snake_case")]
pub enum Language {
    English,
    Polish,
}

#[derive(sqlx::Type)]
#[sqlx(type_name = "meal_product_kind_enum", rename_all = "snake_case")]
pub enum MealProductKind {
    FromDatabase,
    QuickAdd,
}

#[derive(PartialEq, Serialize, Deserialize, sqlx::Type)]
#[serde(rename_all = "camelCase")]
#[sqlx(type_name = "measurement_system_enum", rename_all = "snake_case")]
pub enum MeasurementSystem {
    Metric,
    Imperial,
}

#[derive(Serialize, Deserialize, sqlx::Type)]
#[serde(rename_all = "camelCase")]
#[sqlx(type_name = "weight_goal_enum", rename_all = "snake_case")]
pub enum WeightGoal {
    Gain,
    Lose,
    Maintain,
}

#[derive(Deserialize, sqlx::Type)]
#[serde(rename_all = "camelCase")]
#[sqlx(type_name = "user_origin_enum", rename_all = "lowercase")]
pub enum UserOrigin {
    Instagram,
    TikTok,
    Twitter,
    Twitch,
    Facebook,
    YouTube,
    Other,
}

#[derive(Serialize, Deserialize, sqlx::Type)]
#[serde(rename_all = "camelCase")]
#[sqlx(type_name = "user_diet_enum", rename_all = "snake_case")]
pub enum DietKind {
    Vegetarian,
    Vegan,
    Pescatarian,
    Ketogenic,
    Classic,
}

// STRUCTS
pub struct User {
    pub id: Uuid,
    pub username: String,
    pub display_name: String,
    pub email: String,
    pub password: String,
    pub avatar_url: Option<String>,
    pub account_state: AccountState,
    pub created_at: DateTime<Utc>,
}

// struct containing user, user_details and user_preferences
pub struct FullUser {
    pub id: Uuid,

    pub username: String,
    pub display_name: String,
    pub email: String,
    pub password: String,
    pub avatar_url: Option<String>,
    pub account_state: AccountState,
    pub created_at: DateTime<Utc>,

    pub is_male: bool,
    pub weight: f32,
    pub height: i32,
    pub date_of_birth: NaiveDate,
    pub idle_activity: i32,
    pub workout_activity: i32,
    pub diet_kind: DietKind,

    pub theme: Theme,
    pub language: Language,
    pub measurement_system: MeasurementSystem,

    pub weight_goal: WeightGoal,
    pub goal_diff_per_week: f32,
}

#[derive(Serialize)]
pub struct Product {
    pub id: Uuid,
    pub name: String,
    pub barcode: i32,
    pub ingredients: String,
    pub calories: i32,
    pub proteins: i32,
    pub fats: i32,
    pub carbohydrates: i32,
}

pub struct MealProduct {
    pub id: Uuid,
    pub kind: MealProductKind,
    pub product_id: Option<Uuid>,
    pub name: Option<String>,
    pub calories: Option<i32>,
    pub proteins: Option<i32>,
    pub fats: Option<i32>,
    pub carbohydrates: Option<i32>,
}

pub struct AddMealProduct {
    pub date: NaiveDate,
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

impl AddMealProduct {
    pub fn from_database(date: NaiveDate, section_id: Uuid, quantity: i32, product_id: Uuid) -> Self {
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
