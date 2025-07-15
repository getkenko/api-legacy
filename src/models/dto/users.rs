#![deny(dead_code)]

use chrono::{DateTime, NaiveDate, Utc};
use dotenvy_macro::dotenv;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{models::database::{enums::{DietKind, HeightUnit, Language, Sex, Theme, WeightUnit}, user::FullUser}, utils::conversion::{cm_to_ft_in, kg_to_lb, kg_to_st_lb}};

const CDN_URL: &str = dotenv!("CDN_URL");
const DEFAULT_AVATAR_URL: &str = dotenv!("DEFAULT_AVATAR_URL");

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FullUserView {
    pub id: Uuid,

    pub username: String,
    pub display_name: String,
    pub email: String,
    pub avatar_url: String,

    pub sex: Sex,
    // sending back weight and height in string because they're converted to user selected units
    pub weight: String,
    pub height: String,
    pub date_of_birth: NaiveDate,
    pub diet_kind: DietKind,

    pub theme: Theme,
    pub language: Language,

    pub bmr: f32,
    pub base_tdee: f32,
    pub tdee: f32,
    pub protein_target: i32,
    pub fat_target: i32,
    pub carb_target: i32,
    pub protein_dist: Option<i32>,
    pub fat_dist: Option<i32>,
    pub carb_dist: Option<i32>,

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
            WeightUnit::Kg => format!("{} kg", user.weight),
            WeightUnit::Lb => format!("{:.2} lb", kg_to_lb(user.weight)),
            WeightUnit::StLb => {
                let (st, lb) = kg_to_st_lb(user.weight);
                format!("{st:.2} st {lb:.2} lb")
            }
        };

        let height = match user.height_unit {
            HeightUnit::Cm => format!("{} cm", user.height),
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

            sex: user.sex,
            weight,
            height,
            date_of_birth: user.date_of_birth,
            diet_kind: user.diet_kind,

            theme: user.theme,
            language: user.language,

            bmr: user.bmr,
            base_tdee: user.base_tdee,
            tdee: user.tdee,
            protein_target: user.protein_target,
            fat_target: user.fat_target,
            carb_target: user.carb_target,
            protein_dist: user.protein_dist,
            fat_dist: user.fat_dist,
            carb_dist: user.carb_dist,

            created_at: user.created_at,
        }
    }
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateUserDetailsRequest {
    pub sex: Option<Sex>,

    pub weight_kg: Option<f32>,
    pub weight_lb: Option<f32>,
    pub weight_st: Option<f32>,

    pub height_cm: Option<i32>,
    pub height_ft: Option<i32>,
    pub height_in: Option<i32>,

    pub date_of_birth: Option<NaiveDate>,
    pub idle_activity: Option<i32>,
    pub workout_activity: Option<i32>,
}

#[derive(Deserialize)]
pub struct UpdateUserPreferencesRequest {
    pub theme: Option<Theme>,
    pub language: Option<Language>,
}
