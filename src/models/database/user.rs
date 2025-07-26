#![allow(dead_code)]

use chrono::{DateTime, NaiveDate, Utc};
use uuid::Uuid;

use crate::models::database::enums::{AccountState, DietKind, HeightUnit, Language, Sex, Theme, UserOrigin, WeightGoal, WeightUnit};

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
    pub sex: Sex,
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

    // user_nutrients
    pub bmr: f32,
    pub base_tdee: f32,
    pub tdee: f32,
    pub protein_target: i32,
    pub fat_target: i32,
    pub carb_target: i32,
    pub protein_dist: Option<i32>,
    pub fat_dist: Option<i32>,
    pub carb_dist: Option<i32>,
}

#[derive(sqlx::FromRow)]
pub struct UserConflicts {
    pub username_taken: Option<bool>,
    pub email_taken: Option<bool>,
}

pub struct InsertUser {
    pub username: String,
    pub display_name: String,
    pub email: String,
    pub password: String,

    pub sex: Sex,
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

pub struct UserNutrition {
    pub bmr: f32,
    pub base_tdee: f32,
    pub tdee: f32,
    pub protein_target: i32,
    pub fat_target: i32,
    pub carb_target: i32,
}
