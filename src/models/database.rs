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

#[derive(Deserialize, sqlx::Type)]
#[serde(rename_all = "snake_case")]
#[sqlx(type_name = "weight_unit_enum", rename_all = "snake_case")]
pub enum WeightUnit {
    Kg,
    Lb,
    StLb,
}

#[derive(Deserialize, sqlx::Type)]
#[serde(rename_all = "snake_case")]
#[sqlx(type_name = "height_unit_enum", rename_all = "snake_case")]
pub enum HeightUnit {
    Cm,
    FtIn,
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
#[sqlx(type_name = "diet_kind_enum", rename_all = "snake_case")]
pub enum DietKind {
    Vegetarian,
    Vegan,
    Pescatarian,
    Ketogenic,
    Classic,
}

// STRUCTS
pub struct InsertUser {
    pub username: String,
    pub display_name: String,
    pub email: String,
    pub password: String,

    pub is_male: bool,
    pub weight: f32,
    pub height: i32,
    pub date_of_birth: NaiveDate,
    pub idle_activity: i32,
    pub workout_activity: i32,
    pub diet_kind: DietKind,

    pub weight_unit: WeightUnit,
    pub height_unit: HeightUnit,

    pub weight_goal: WeightGoal,
    pub goal_diff_per_week: f32,

    pub origin: UserOrigin,
}

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

pub struct FullUser {
    pub id: Uuid,

    // users
    pub username: String,
    pub display_name: String,
    pub email: String,
    pub password: String,
    pub avatar_url: Option<String>,
    pub account_state: AccountState,
    pub created_at: DateTime<Utc>,

    // user_details
    pub is_male: bool,
    pub weight: f32,
    pub height: i32,
    pub date_of_birth: NaiveDate,
    pub idle_activity: i32,
    pub workout_activity: i32,
    pub diet_kind: DietKind,

    // user_preferences
    pub theme: Theme,
    pub language: Language,
    pub weight_unit: WeightUnit,
    pub height_unit: HeightUnit,

    // user_goals
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
