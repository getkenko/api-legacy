// idk how to name this module, either metabolism or nutrition or calorie_math or body_metrics

use crate::models::database::enums::{Sex, WeightGoal};

const KCAL_PER_BODY_KG: u32 = 7700;

pub fn calculate_bmr(weight: f32, height: i32, age: u32, sex: Sex) -> f32 {
    // this formula is the same for all genders
    let base = (10.0 * weight) + (6.25 * height as f32) - (5.0 * age as f32);

    let bmr = match sex {
        Sex::Male => base + 5.0,
        Sex::Female => base - 161.0,
    };

    bmr
}

fn activity_multiplier(activity: i32) -> f32 {
    // im not sure whether there is a better way of doing that
    match activity {
        1 => 1.1,
        2 => 1.2,
        3 => 1.35,
        4 => 1.5,
        5 => 1.7,
        _ => {
            tracing::warn!("Out of range activity detected: {activity}");
            1.0
        }
    }
}

/// Calculates base TDEE which doesnt count user's goal and tempo. Returns rounded calories.
pub fn calc_base_tdee(bmr: f32, workout_activity: i32, idle_activity: i32) -> f32 {
    bmr * activity_multiplier(workout_activity) * activity_multiplier(idle_activity)
}

pub fn calculate_tdee(base_tdee: f32, goal_diff_per_week: f32, weight_goal: WeightGoal) -> f32 {
    let daily_change = (KCAL_PER_BODY_KG as f32 * goal_diff_per_week) / 7.0;

    match weight_goal {
        WeightGoal::Gain => base_tdee + daily_change,
        WeightGoal::Lose => base_tdee - daily_change,
        WeightGoal::Maintain => base_tdee,
    }
}

#[derive(Debug)]
pub struct TargetMacros {
    pub proteins: f32,
    pub fats: f32,
    pub carbohydrates: f32,
}

pub fn calc_target_macros(weight: f32, tdee: f32, weight_goal: WeightGoal) -> TargetMacros {
    let (protein_multiplier, fat_percent) = match weight_goal {
        WeightGoal::Gain => (1.8, 0.2),
        WeightGoal::Lose => (2.0, 0.25),
        WeightGoal::Maintain => (1.6, 0.25),
    };

    let proteins = protein_multiplier * weight;
    let proteins_kcal = proteins * 4.0;

    let fats_kcal = fat_percent * tdee;
    let fats = fats_kcal / 9.0;

    let carbohydrates_kcal = tdee - proteins_kcal - fats_kcal;
    let carbohydrates = carbohydrates_kcal / 4.0;

    TargetMacros {
        proteins,
        fats,
        carbohydrates,
    }
}

pub fn calc_grams_from_dist(tdee: f32, dist: i32, div: f32) -> i32 {
    let percent = dist as f32 / 100.0;
    let kcal = tdee * percent;
    let grams = kcal / div;
    grams.round() as _
}

// pub fn calc_dist_from_grams(tdee: f32, grams: i32, mul: f32) -> i32 {
//     let kcal = grams as f32 * mul;
//     let dist = (kcal / tdee) * 100.0;
//     dist.round() as _
// }
