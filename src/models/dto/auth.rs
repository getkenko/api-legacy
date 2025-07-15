#![deny(dead_code)]

use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

use crate::{models::{database::{enums::{DietKind, HeightUnit, Sex, UserOrigin, WeightGoal, WeightUnit}, user::{InsertUser, UserConflicts}}, errors::{AppError, AppResult, ValidationError}}, security::password::hash_password, utils::conversion::{ft_in_to_cm, lb_to_kg, st_lb_to_kg}};

#[derive(Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

#[derive(Serialize)]
pub struct LoginResponse {
    pub token: String,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RegisterRequest {
    pub username: String,
    pub email: String,
    pub password: String,
    pub sex: Sex,

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

impl RegisterRequest {
    fn get_weight_and_height(&self) -> AppResult<(f32, i32)> {
        let weight = match self.weight_unit {
            WeightUnit::Kg => self.weight_kg.ok_or(ValidationError::MissingKgWeight)?,
            WeightUnit::Lb => {
                let lb = self.weight_lb.ok_or(ValidationError::MissingLbWeight)?;
                lb_to_kg(lb)
            }
            WeightUnit::StLb => {
                let st = self.weight_st.ok_or(ValidationError::MissingStLbWeight)?;
                let lb = self.weight_lb.ok_or(ValidationError::MissingStLbWeight)?;
                st_lb_to_kg(st, lb)
            }
        };

        if weight <= 0.0 || weight >= 10000.0 {
            return Err(ValidationError::NegativeWeight)?;
        }

        let height = match self.height_unit {
            HeightUnit::Cm => self.height_cm.ok_or(ValidationError::MissingCmHeight)?,
            HeightUnit::FtIn => {
                let ft = self.height_ft.ok_or(ValidationError::MissingFtInHeight)?;
                let inch = self.height_in.ok_or(ValidationError::MissingFtInHeight)?;
                ft_in_to_cm(ft, inch)
            }
        };

        if height <= 0 || height >= 300 {
            return Err(ValidationError::NegativeHeight)?;
        }

        Ok((weight, height))
    }
}

impl TryFrom<RegisterRequest> for InsertUser {
    type Error = AppError;

    fn try_from(user: RegisterRequest) -> Result<Self, Self::Error> {
        let (weight, height) = user.get_weight_and_height()?;
        let password = hash_password(&user.password).map_err(AppError::Crypto)?;

        Ok(Self {
            username: user.username.clone(),
            display_name: user.username,
            email: user.email,
            password,
            sex: user.sex,
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

#[derive(Deserialize)]
pub struct CheckAvailabilityQuery {
    pub username: String,
    pub email: String,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UserConflictsView {
    pub username_available: bool,
    pub email_available: bool,
}

impl From<UserConflicts> for UserConflictsView {
    fn from(conflicts: UserConflicts) -> Self {
        Self {
            username_available: !conflicts.username_taken,
            email_available: !conflicts.email_taken,
        }
    }
}
