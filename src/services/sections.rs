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

    if let Some(index) = update.index {
        if index < 0 || index >= USER_SECTION_LIMIT {
            return Err(ValidationError::InvalidSectionIndex)?;
        }
    }

    if let Some(label) = &update.label {
        if label.is_empty() {
            return Err(ValidationError::SectionHasEmptyName)?;
        }
    }

    // check if meal section id is correct
    if !meal_section_repo::check_meal_section_exists(db, user_id, section_id).await? {
        return Err(AppError::SectionNotFound);
    }

    let section = meal_section_repo::update_meal_section(db, user_id, section_id, update.index, update.label).await?;
    let section_view = UserMealSectionView::from(section);
    Ok(section_view)
}

pub async fn delete_user_section(db: &PgPool, user_id: Uuid, section_id: Uuid) -> AppResult<()> {
    if !meal_section_repo::check_meal_section_exists(db, user_id, section_id).await? {
        return Err(AppError::SectionNotFound);
    }

    meal_section_repo::delete_meal_section(db, user_id, section_id).await?;

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
