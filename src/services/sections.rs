use sqlx::PgPool;
use uuid::Uuid;

use crate::{database::meal_section_repo, models::{dto::{meals::UserMealSectionView, sections::{NewSectionRequest, UpdateSectionRequest}}, errors::{AppError, AppResult, ValidationError}}};

pub const USER_SECTION_LIMIT: i32 = 10;

pub async fn create_new_section(
    db: &PgPool,
    user_id: Uuid,
    section: NewSectionRequest,
) -> AppResult<UserMealSectionView> {
    // validate: index in range (0-USER_SECTION_LIMIT), label not empty
    if section.index < 0 || section.index >= USER_SECTION_LIMIT {
        return Err(ValidationError::InvalidSectionIndex)?;
    } else if section.label.is_empty() {
        return Err(ValidationError::SectionHasEmptyName)?;
    }

    let count = meal_section_repo::fetch_user_section_count(db, user_id).await?;
    if count >= USER_SECTION_LIMIT {
        return Err(AppError::SectionLimitReached);
    }

    let section = match meal_section_repo::insert_meal_section(db, user_id, section.index, section.label).await {
        Ok(s) => UserMealSectionView::from(s),
        Err(why) => {
            if let Some(err) = why.as_database_error() {
                if err.is_unique_violation() {
                    return Err(AppError::SectionIndexTaken);
                }
            }

            return Err(AppError::Database(why));
        }
    };

    Ok(section)
}

pub async fn get_user_sections_layout(
    db: &PgPool,
    user_id: Uuid,
) -> AppResult<Vec<UserMealSectionView>> {
    let sections = meal_section_repo::fetch_meal_sections(db, user_id).await?;
    let sections_view = sections
        .into_iter()
        .map(|s| UserMealSectionView::from(s))
        .collect::<Vec<_>>();

    Ok(sections_view)
}

pub async fn update_user_section(
    db: &PgPool,
    user_id: Uuid,
    section_id: Uuid,
    update: UpdateSectionRequest,
) -> AppResult<UserMealSectionView> {
    // validate user input
    if update.index.is_none() && update.label.is_none() {
        return Err(ValidationError::NoFieldsToUpdate)?;
    }

    let mut tx = db.begin().await?;

    let section = meal_section_repo::find_meal_section(&mut *tx, user_id, section_id)
        .await?
        .ok_or(AppError::SectionNotFound)?;
    
    if let Some(target_index) = update.index {
        let increase = section.index - target_index >= 0;
        let last_section_index = meal_section_repo::find_last_section_index(&mut *tx, user_id, section_id).await?.unwrap_or(0);

        if target_index > USER_SECTION_LIMIT || target_index < 0 || target_index > last_section_index {
            return Err(ValidationError::InvalidSectionIndex)?;
        }

        meal_section_repo::update_section_indices(&mut *tx, user_id, increase, section.index, target_index).await?;
    }

    if let Some(label) = &update.label {
        if label.is_empty() {
            return Err(ValidationError::SectionHasEmptyName)?;
        }
    }

    let updated_section = meal_section_repo::update_meal_section(&mut *tx, user_id, section_id, update.index, update.label).await?;
    tx.commit().await?;

    Ok(updated_section.into())
}

pub async fn delete_user_section(db: &PgPool, user_id: Uuid, section_id: Uuid) -> AppResult<()> {
    if !meal_section_repo::check_meal_section_exists(db, user_id, section_id).await? {
        return Err(AppError::SectionNotFound);
    }

    let mut tx = db.begin().await?;

    let deleted_index = meal_section_repo::delete_meal_section(&mut *tx, user_id, section_id).await?;

    // update indices so they're still in sequence
    meal_section_repo::update_section_indices(&mut *tx, user_id, false, deleted_index, 999).await?;

    tx.commit().await?;

    Ok(())
}

pub async fn reset_user_section_layout(
    db: &PgPool,
    user_id: Uuid,
) -> AppResult<Vec<UserMealSectionView>> {
    let sections = meal_section_repo::reset_meal_sections(db, user_id).await?;
    let views = sections
        .into_iter()
        .map(|s| UserMealSectionView::from(s))
        .collect::<Vec<_>>();

    Ok(views)
}
