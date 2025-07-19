use sqlx::PgPool;
use uuid::Uuid;

use crate::{database::user_nutrients_repo, models::{dto::nutrients::{UpdateMacrosDistribution, UpdateMacrosTarget}, errors::{AppResult, ValidationError}}, utils::nutrition::calc_grams_from_dist};

pub async fn update_user_macros_distribution(db: &PgPool, user_id: Uuid, dist: UpdateMacrosDistribution) -> AppResult<()> {
    // make sure the values are non-negative
    if dist.protein < 0 || dist.fat < 0 || dist.carb < 0 {
        return Err(ValidationError::NegativeDistribution)?;
    }

    // check if total distribution is equal to 100%
    let sum = dist.protein + dist.fat + dist.carb;
    if sum > 100 {
        return Err(ValidationError::DistributionAbove100)?;
    } else if sum < 100 {
        return Err(ValidationError::DistributionBelow100)?;
    }

    // fetch user TDEE (needed for calculations)
    let tdee = user_nutrients_repo::fetch_user_nutrients_tdee(db, user_id).await?;

    // calculate target macros from distribution
    let protein_target = calc_grams_from_dist(tdee, dist.protein, 4.0);
    let fat_target = calc_grams_from_dist(tdee, dist.fat, 9.0);
    let carb_target = calc_grams_from_dist(tdee, dist.carb, 4.0);

    user_nutrients_repo::update_user_nutrients_macros(db, user_id, Some(dist.protein), Some(dist.fat), Some(dist.carb), protein_target, fat_target, carb_target).await?;

    Ok(())
}

// override macro distribution by entering invidual values (distribution should be recalculated)
pub async fn update_user_macros_target(db: &PgPool, user_id: Uuid, target: UpdateMacrosTarget) -> AppResult<()> {
    // make sure target macros are not below 0
    if target.protein < 0 || target.fat < 0 || target.carb < 0 {
        return Err(ValidationError::NegativeMacrosTarget)?;
    }

    user_nutrients_repo::update_user_nutrients_macros(db, user_id, None, None, None, target.protein, target.fat, target.carb).await?;

    Ok(())
}