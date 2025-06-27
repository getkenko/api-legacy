use serde::{Deserialize, Serialize};

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