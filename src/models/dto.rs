#![deny(dead_code)]

use chrono::{DateTime, NaiveDate, Utc};
use dotenvy_macro::dotenv;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{models::{database::{enums::{DietKind, HeightUnit, Language, MealProductKind, Theme, UserOrigin, WeightGoal, WeightUnit}, meal::{InsertMealProduct, UserMealProduct, UserMealSection}, product::Product, user::{FullUser, InsertUser}}, errors::{AppError, AppResult}}, security::password::hash_password, utils::conversion::{cm_to_ft_in, ft_in_to_cm, kg_to_lb, kg_to_st_lb, lb_to_kg, st_lb_to_kg}};

const CDN_URL: &str = dotenv!("CDN_URL");
const DEFAULT_AVATAR_URL: &str = dotenv!("DEFAULT_AVATAR_URL");

// AUTH
#[derive(Deserialize)]
pub struct LoginCredentials {
    pub email: String,
    pub password: String,
}

#[derive(Serialize)]
pub struct LoginResponse {
    pub token: String,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RegisterUserData {
    pub username: String,
    pub email: String,
    pub password: String,
    pub is_male: bool,

    pub weight_unit: WeightUnit,
    pub weight_kg: Option<f32>,
    pub weight_lb: Option<f32>,
    pub weight_st: Option<f32>,

    pub height_unit: HeightUnit,
    pub height_cm: Option<i32>,
    pub height_ft: Option<i32>,
    pub height_in: Option<i32>,

    pub date_of_birth: NaiveDate,
    pub idle_activity: i32,
    pub workout_activity: i32,
    pub weight_goal: WeightGoal,
    pub goal_diff_per_week: f32,
    pub diet_kind: DietKind,
    pub origin: UserOrigin,
}

impl RegisterUserData {
    fn get_weight_and_height(&self) -> AppResult<(f32, i32)> {
        let weight = match self.weight_unit {
            WeightUnit::Kg => self.weight_kg.ok_or(AppError::MissingKgWeight)?,
            WeightUnit::Lb => {
                let lb = self.weight_lb.ok_or(AppError::MissingLbWeight)?;
                lb_to_kg(lb)
            }
            WeightUnit::StLb => {
                let st = self.weight_st.ok_or(AppError::MissingStLbWeight)?;
                let lb = self.weight_lb.ok_or(AppError::MissingStLbWeight)?;
                st_lb_to_kg(st, lb)
            }
        };

        if weight <= 0.0 {
            return Err(AppError::NegativeWeight);
        }

        let height = match self.height_unit {
            HeightUnit::Cm => self.height_cm.ok_or(AppError::MissingCmHeight)?,
            HeightUnit::FtIn => {
                let ft = self.height_ft.ok_or(AppError::MissingFtInHeight)?;
                let inch = self.height_in.ok_or(AppError::MissingFtInHeight)?;
                ft_in_to_cm(ft, inch)
            }
        };

        if height <= 0 {
            return Err(AppError::NegativeHeight);
        }

        Ok((weight, height))
    }
}

impl TryFrom<RegisterUserData> for InsertUser {
    type Error = AppError;

    fn try_from(user: RegisterUserData) -> Result<Self, Self::Error> {
        let (weight, height) = user.get_weight_and_height()?;
        let password = hash_password(&user.password).map_err(AppError::Crypto)?;

        Ok(Self {
            username: user.username.clone(),
            display_name: user.username,
            email: user.email,
            password,
            is_male: user.is_male,
            weight,
            height,
            date_of_birth: user.date_of_birth,
            idle_activity: user.idle_activity,
            workout_activity: user.workout_activity,
            diet_kind: user.diet_kind,
            weight_unit: user.weight_unit,
            height_unit: user.height_unit,
            weight_goal: user.weight_goal,
            goal_diff_per_week: user.goal_diff_per_week,
            origin: user.origin,
        })
    }
}

// USERS
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FullUserView {
    pub id: Uuid,

    pub username: String,
    pub display_name: String,
    pub email: String,
    pub avatar_url: String,

    pub is_male: bool,
    // sending back weight and height in string because they're converted to user selected units
    pub weight: String,
    pub height: String,
    pub date_of_birth: NaiveDate,
    pub diet_kind: DietKind,

    pub theme: Theme,
    pub language: Language,

    pub created_at: DateTime<Utc>,
}

impl From<FullUser> for FullUserView {
    fn from(user: FullUser) -> Self {
        let avatar_url = user.avatar_url.unwrap_or(DEFAULT_AVATAR_URL.to_string());

        // convert weight and height to user preferred unit
        // swaglord: i dont think api should return 'frontend' data to user
        // but holy fuck i aint adding 247821 struct fields because earth cant
        // use a single measurement system ffs
        let weight = match user.weight_unit {
            WeightUnit::Kg => format!("{}kg", user.weight),
            WeightUnit::Lb => format!("{:.2}lb", kg_to_lb(user.weight)),
            WeightUnit::StLb => {
                let (st, lb) = kg_to_st_lb(user.weight);
                format!("{st}st {lb}lb")
            }
        };

        let height = match user.height_unit {
            HeightUnit::Cm => format!("{}cm", user.height),
            HeightUnit::FtIn => {
                let (ft, inch) = cm_to_ft_in(user.height);
                format!("{ft}' {inch}\"")
            }
        };

        Self {
            id: user.id,

            username: user.username,
            display_name: user.display_name,
            email: user.email,
            avatar_url: format!("{CDN_URL}/{avatar_url}"),

            is_male: user.is_male,
            weight,
            height,
            date_of_birth: user.date_of_birth,
            diet_kind: user.diet_kind,

            theme: user.theme,
            language: user.language,

            created_at: user.created_at,
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

#[derive(Serialize)]
pub struct ProductView {
    pub id: Uuid,
    pub name: String,
    pub barcode: i32,
    pub ingredients: String,
    pub calories: i32,
    pub proteins: i32,
    pub fats: i32,
    pub carbohydrates: i32,
}

impl From<Product> for ProductView {
    fn from(product: Product) -> Self {
        Self {
            id: product.id,
            name: product.name,
            barcode: product.barcode,
            ingredients: product.ingredients,
            calories: product.calories,
            proteins: product.proteins,
            fats: product.fats,
            carbohydrates: product.carbohydrates,
        }
    }
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
    pub fn from_meals_products(meals_products: &[UserMealProduct]) -> Self {
        let mut s = Self::default();

        for p in meals_products {
            s.add_raw(p.calories, p.proteins, p.fats, p.carbohydrates);
        }

        s
    }

    fn add_raw(&mut self, calories: i32, proteins: i32, fats: i32, carbohydrates: i32) {
        self.calories += calories;
        self.proteins += proteins;
        self.fats += fats;
        self.carbohydrates += carbohydrates;
    }
}

#[derive(Serialize)]
pub struct UserMealSectionView {
    pub id: Uuid,
    pub index: i32,
    pub label: String,
}

impl From<UserMealSection> for UserMealSectionView {
    fn from(section: UserMealSection) -> Self {
        Self {
            id: section.id,
            index: section.index,
            label: section.label,
        }
    }
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AddProduct {
    pub section_id: Uuid,
    pub product_id: Uuid,
    pub quantity: i32,
}

impl From<AddProduct> for InsertMealProduct {
    fn from(product: AddProduct) -> Self {
        Self {
            section_id: product.section_id,
            quantity: product.quantity,
            kind: MealProductKind::FromDatabase,
            product_id: Some(product.product_id),
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
pub struct QuickAddProduct {
    pub section_id: Uuid,
    pub name: String,
    pub calories: i32,
    pub proteins: i32,
    pub fats: i32,
    pub carbohydrates: i32,
    pub quantity: i32,
}

impl From<QuickAddProduct> for InsertMealProduct {
    fn from(product: QuickAddProduct) -> Self {
        Self {
            section_id: product.section_id,
            quantity: product.quantity,
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
pub struct UserMealProductView {
    pub product_id: Option<Uuid>,
    pub quantity: i32,
    pub name: String,
    pub calories: i32,
    pub proteins: i32,
    pub fats: i32,
    pub carbohydrates: i32,
}

impl From<&UserMealProduct> for UserMealProductView {
    fn from(product: &UserMealProduct) -> Self {
        Self {
            product_id: product.product_id,
            quantity: product.quantity,
            name: product.name.clone(),
            calories: product.calories,
            proteins: product.proteins,
            fats: product.fats,
            carbohydrates: product.carbohydrates,
        }
    }
}
