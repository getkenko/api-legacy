use chrono::{DateTime, NaiveDate, Utc};
use uuid::Uuid;

use crate::models::database::enums::{AccountState, DietKind, HeightUnit, Language, Theme, UserOrigin, WeightGoal, WeightUnit};

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

#[derive(Default)]
pub struct UserConflicts {
    pub username_taken: bool,
    pub email_taken: bool,
}

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