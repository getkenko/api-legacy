use sqlx::PgPool;
use uuid::Uuid;

use crate::{database::section_repo, models::{dto::{meals::UserSectionView, sections::{NewSectionRequest, UpdateSectionRequest}}, errors::{AppError, AppResult, ValidationError}}};

pub const USER_SECTION_LIMIT: i32 = 10;

pub async fn create_new_section(
    db: &PgPool,
    user_id: Uuid,
    section: NewSectionRequest,
) -> AppResult<UserSectionView> {
    // validate
    if section.name.is_empty() {
        return Err(ValidationError::SectionHasEmptyName)?;
    }

    let last_section_index = section_repo::find_last_section_index(db, user_id, None).await?.unwrap_or(-1);
    if last_section_index >= USER_SECTION_LIMIT {
        return Err(AppError::SectionLimitReached);
    }

    // check if icon exists
    if !section_repo::check_icon_exists(db, section.icon).await? {
        return Err(AppError::IconNotFound);
    }

    let section = section_repo::insert_meal_section(db, user_id, last_section_index + 1, section.name, section.icon).await?;
    Ok(section.into())
}

pub async fn get_user_sections_layout(
    db: &PgPool,
    user_id: Uuid,
) -> AppResult<Vec<UserSectionView>> {
    let sections = section_repo::fetch_meal_sections(db, user_id).await?;
    let sections_view = sections
        .into_iter()
        .map(|s| UserSectionView::from(s))
        .collect::<Vec<_>>();

    Ok(sections_view)
}

pub async fn update_user_section(
    db: &PgPool,
    user_id: Uuid,
    section_id: Uuid,
    update: UpdateSectionRequest,
) -> AppResult<UserSectionView> {
    // validate user input
    if update.index.is_none() && update.name.is_none() && update.icon.is_none() {
        return Err(ValidationError::NoFieldsProvided)?;
    }

    let mut tx = db.begin().await?;

    let section = section_repo::find_meal_section(&mut *tx, user_id, section_id)
        .await?
        .ok_or(AppError::SectionNotFound)?;
    
    if let Some(target_index) = update.index {
        let increase = section.index - target_index >= 0;
        let last_section_index = section_repo::find_last_section_index(&mut *tx, user_id, Some(section_id)).await?.unwrap_or(0);

        if target_index > USER_SECTION_LIMIT || target_index < 0 || target_index > last_section_index {
            return Err(ValidationError::InvalidSectionIndex)?;
        }

        section_repo::update_section_indices(&mut *tx, user_id, increase, section.index, target_index).await?;
    }

    if let Some(name) = &update.name {
        if name.is_empty() {
            return Err(ValidationError::SectionHasEmptyName)?;
        }
    }

    if let Some(icon) = update.icon {
        let exists = section_repo::check_icon_exists(db, icon).await?;
        if !exists {
            return Err(AppError::IconNotFound);
        }
    }

    let updated_section = section_repo::update_meal_section(&mut *tx, user_id, section_id, update.index, update.name, update.icon).await?;
    tx.commit().await?;

    Ok(updated_section.into())
}

pub async fn delete_user_section(db: &PgPool, user_id: Uuid, section_id: Uuid) -> AppResult<()> {
    if !section_repo::check_meal_section_exists(db, user_id, section_id).await? {
        return Err(AppError::SectionNotFound);
    }

    let mut tx = db.begin().await?;

    let deleted_index = section_repo::delete_meal_section(&mut *tx, user_id, section_id).await?;

    // update indices so they're still in sequence
    section_repo::update_section_indices(&mut *tx, user_id, false, deleted_index, 999).await?;

    tx.commit().await?;

    Ok(())
}

pub async fn reset_user_section_layout(
    db: &PgPool,
    user_id: Uuid,
) -> AppResult<Vec<UserSectionView>> {
    let sections = section_repo::reset_meal_sections(db, user_id).await?;
    let views = sections
        .into_iter()
        .map(|s| UserSectionView::from(s))
        .collect::<Vec<_>>();

    Ok(views)
}
