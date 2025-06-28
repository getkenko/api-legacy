use sqlx::PgPool;
use uuid::Uuid;

use crate::{database::meal_section::{check_meal_section_exists, delete_meal_section, fetch_meal_sections, fetch_user_section_count, insert_meal_section, reset_meal_sections, update_meal_section}, models::{dto::{meals::UserMealSectionView, sections::{NewSectionRequest, UpdateSectionRequest}}, errors::{AppError, AppResult}}};

const USER_SECTION_LIMIT: i64 = 10;

pub async fn create_new_section(
    db: &PgPool,
    user_id: Uuid,
    section: NewSectionRequest,
) -> AppResult<UserMealSectionView> {
    let count = fetch_user_section_count(db, user_id).await?;
    if count >= USER_SECTION_LIMIT {
        return Err(AppError::SectionLimitReached);
    }

    let section = insert_meal_section(db, user_id, section.index, section.label).await?;
    let section_view = UserMealSectionView::from(section);
    Ok(section_view)
}

pub async fn get_user_sections_layout(
    db: &PgPool,
    user_id: Uuid,
) -> AppResult<Vec<UserMealSectionView>> {
    let sections = fetch_meal_sections(db, user_id).await?;
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
    if update.index.is_none() && update.label.is_none() {
        return Err(AppError::NoFieldsToUpdate);
    }

    if !check_meal_section_exists(db, user_id, section_id).await? {
        return Err(AppError::SectionNotFound);
    }

    let section = update_meal_section(db, user_id, section_id, update.index, update.label).await?;
    let section_view = UserMealSectionView::from(section);
    Ok(section_view)
}

pub async fn delete_user_section(
    db: &PgPool,
    user_id: Uuid,
    section_id: Uuid,
) -> AppResult<()> {
    if !check_meal_section_exists(db, user_id, section_id).await? {
        return Err(AppError::SectionNotFound);
    }
    
    delete_meal_section(db, user_id, section_id).await?;

    Ok(())
}

pub async fn reset_user_section_layout(
    db: &PgPool,
    user_id: Uuid,
) -> AppResult<Vec<UserMealSectionView>> {
    let sections = reset_meal_sections(db, user_id).await?;
    let views = sections
        .into_iter()
        .map(|s| UserMealSectionView::from(s))
        .collect::<Vec<_>>();

    Ok(views)
}