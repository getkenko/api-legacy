use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Serialize, Deserialize, sqlx::Type)]
#[serde(rename_all = "camelCase")]
#[sqlx(type_name = "sex_enum", rename_all = "snake_case")]
pub enum Sex {
    Male,
    Female,
}

#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize, sqlx::Type, strum_macros::Display)]
#[serde(rename_all = "camelCase")]
#[sqlx(type_name = "account_state_enum", rename_all = "snake_case")]
pub enum AccountState {
    Active,
    Suspended,
    Deleted,
    Inactive,
}

#[derive(Clone, Copy, Serialize, Deserialize, sqlx::Type)]
#[serde(rename_all = "camelCase")]
#[sqlx(type_name = "theme_enum", rename_all = "snake_case")]
pub enum Theme {
    System,
    Dark,
    Light,
}

#[derive(Clone, Copy, Serialize, Deserialize, sqlx::Type)]
#[serde(rename_all = "lowercase")]
#[sqlx(type_name = "language_enum", rename_all = "lowercase")]
pub enum Language {
    En,
    Pl,
}

#[derive(Clone, Copy, sqlx::Type)]
#[sqlx(type_name = "meal_product_kind_enum", rename_all = "snake_case")]
pub enum MealProductKind {
    FromDatabase,
    QuickAdd,
}

#[derive(Debug, Clone, Copy, Deserialize, sqlx::Type)]
#[serde(rename_all = "snake_case")]
#[sqlx(type_name = "weight_unit_enum", rename_all = "snake_case")]
pub enum WeightUnit {
    Kg,
    Lb,
}

#[derive(Debug, Clone, Copy, Deserialize, sqlx::Type)]
#[serde(rename_all = "snake_case")]
#[sqlx(type_name = "height_unit_enum", rename_all = "snake_case")]
pub enum HeightUnit {
    Cm,
    FtIn,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, sqlx::Type)]
#[serde(rename_all = "camelCase")]
#[sqlx(type_name = "weight_goal_enum", rename_all = "snake_case")]
pub enum WeightGoal {
    Gain,
    Lose,
    Maintain,
}

#[derive(Clone, Copy, Deserialize, sqlx::Type)]
#[serde(rename_all = "lowercase")]
#[sqlx(type_name = "user_origin_enum", rename_all = "lowercase")]
pub enum UserOrigin {
    Instagram,
    TikTok,
    X, // this is big twitter!!!!! nobody calls it X!!!!
    Twitch,
    Facebook,
    YouTube,
    Other,
}

#[derive(Clone, Copy, Serialize, Deserialize, sqlx::Type)]
#[serde(rename_all = "camelCase")]
#[sqlx(type_name = "diet_kind_enum", rename_all = "snake_case")]
pub enum DietKind {
    Vegetarian,
    Vegan,
    Pescatarian,
    Ketogenic,
    Classic,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "unit_enum", rename_all = "snake_case")]
pub enum Unit {
    #[serde(rename = "g")]
    Grams,
    #[serde(rename = "ml")]
    Milliliters,
}