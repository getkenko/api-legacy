#![deny(dead_code)]

use chrono::{DateTime, NaiveDate, Utc};
use dotenvy_macro::dotenv;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::utils::conversion::metric_height_to_imperial;

use super::database::{FullUser, Language, MeasurementSystem, Product, Theme, WeightGoal};

const DEFAULT_AVATAR_URL: &str = dotenv!("DEFAULT_AVATAR_URL");

// AUTH
#[derive(Deserialize)]
pub struct LoginCredentials {
    pub email: String,
    pub password: String,
}

#[derive(Serialize)]
pub struct LoginResponse {
    pub access: String,
    pub refresh: String,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RegisterUserData {
    pub username: String,
    pub email: String,
    pub password: String,
    pub is_male: bool,
    pub weight: f32,
    pub height: i32,
    pub date_of_birth: NaiveDate,
    pub measurement_system: MeasurementSystem,
    pub idle_activity: i32,
    pub workout_activity: i32,
    pub weight_goal: WeightGoal,
    pub goal_diff_per_week: f32,
}

// USERS
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FullUserView {
    pub username: String,
    pub display_name: String,
    pub email: String,
    pub avatar_url: String,
    pub created_at: DateTime<Utc>,

    pub is_male: bool,
    pub weight: f32,
    pub height: String,
    pub date_of_birth: NaiveDate,

    pub theme: Theme,
    pub language: Language,
    pub measurement_system: MeasurementSystem,
}

impl From<FullUser> for FullUserView {
    fn from(user: FullUser) -> Self {
        let avatar_url = user.avatar_url.unwrap_or(DEFAULT_AVATAR_URL.to_string());
        let height = if user.measurement_system == MeasurementSystem::Imperial {
            let (feet, inch) = metric_height_to_imperial(user.height);
            format!("{feet}' {inch}\"")
        } else {
            user.height.to_string()
        };

        Self {
            username: user.username,
            display_name: user.display_name,
            email: user.email,
            avatar_url,
            created_at: user.created_at,

            is_male: user.is_male,
            weight: user.weight,
            height,
            date_of_birth: user.date_of_birth,

            theme: user.theme,
            language: user.language,
            measurement_system: user.measurement_system,
        }
    }
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NewUserDetails {
    pub is_male: Option<bool>,
    pub weight: Option<f32>,
    pub height: Option<i32>,
    pub date_of_birth: Option<NaiveDate>,
}

#[derive(Deserialize)]
pub struct NewUserPreferences {
    pub theme: Option<Theme>,
    pub language: Option<Language>,
}

// PRODUCTS
#[derive(Deserialize)]
pub struct SearchProduct {
    pub query: String,
}

// MEALS
#[derive(Default, Serialize)]
pub struct MealDayMacro {
    pub calories: i32,
    pub proteins: i32,
    pub fats: i32,
    pub carbohydrates: i32,
}

impl MealDayMacro {
    pub fn add_raw(&mut self, calories: i32, proteins: i32, fats: i32, carbohydrates: i32) {
        self.calories += calories;
        self.proteins += proteins;
        self.fats += fats;
        self.carbohydrates += carbohydrates;
    }

    pub fn add_product(&mut self, product: &Product, quantity: i32) {
        // to get macro for this quantity from 100g:
        // macro_for_100g * (quantity / 100)

        // swaglord: not the best name but hey, at least its a closure
        let from_quant = |val: i32| -> i32 {
            val * (quantity / 100)
        };

        self.add_raw(from_quant(product.calories), from_quant(product.proteins), from_quant(product.fats), from_quant(product.carbohydrates));
    }
}

#[derive(Serialize)]
pub struct UserMealSectionView {
    pub id: Uuid,
    pub index: i32,
    pub label: String,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AddProduct {
    pub section_id: Uuid,
    pub product_id: Uuid,
    pub quantity: i32,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct QuickAddProduct {
    pub section_id: Uuid,
    pub name: String,
    pub calories: i32,
    pub proteins: i32,
    pub fats: i32,
    pub carbohydrates: i32,
    pub quantity: i32,
}
