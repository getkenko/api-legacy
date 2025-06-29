use axum::extract::Multipart;
use sqlx::PgPool;
use tokio::{fs::File, io::AsyncWriteExt};
use uuid::Uuid;

use crate::{database::user::{fetch_full_user, update_user_avatar, update_user_details_opt, update_user_preferences_opt}, models::{dto::users::{FullUserView, UpdateUserDetailsRequest, UpdateUserPreferencesRequest}, errors::{AppError, AppResult}}};

pub async fn get_full_user_info(db: &PgPool, user_id: Uuid) -> AppResult<FullUserView> {
    let user = fetch_full_user(db, user_id).await?;
    let user_view = FullUserView::from(user);
    Ok(user_view)
}

pub async fn update_user_details(db: &PgPool, user_id: Uuid, details: UpdateUserDetailsRequest) -> AppResult<()> {
    if details.is_male.is_none() && details.weight.is_none() && details.height.is_none() && details.date_of_birth.is_none() {
        return Err(AppError::NoFieldsToUpdate);
    }

    // TODO: convert the weight/height using correct units
    // TODO: validate user input (weight/height/dob)

    update_user_details_opt(db, user_id, details.is_male, details.weight, details.height, details.date_of_birth).await?;

    Ok(())
}

pub async fn update_user_preferences(db: &PgPool, user_id: Uuid, preferences: UpdateUserPreferencesRequest) -> AppResult<()> {
    if preferences.theme.is_none() && preferences.language.is_none() {
        return Err(AppError::NoFieldsToUpdate);
    }

    update_user_preferences_opt(db, user_id, preferences.theme, preferences.language).await?;

    Ok(())
}

pub async fn update_user_avatar_from_form(db: &PgPool, user_id: Uuid, mut form: Multipart) -> AppResult<()> {
    while let Some(field) = form.next_field().await? {
        if field.name() == Some("avatar") {
            let data = field.bytes().await?;
            let mime = infer::get(&data).ok_or(AppError::UnknownFileType)?;

            // check if it's accepted image format
            if mime.mime_type() != "image/png" && mime.mime_type() != "image/jpeg" {
                return Err(AppError::UnknownFileType)?;
            }

            // create file name
            let file_name = format!("{}.{}", Uuid::new_v4(), mime.extension());
            let image_path = format!("avatars/{file_name}"); // used in CDN url
            let file_path = format!("public/{image_path}"); // full file path with CDN directory

            // save to disk
            let mut file = File::create(file_path).await?;
            file.write_all(&data).await?;

            // update user's avatar in database
            update_user_avatar(db, user_id, Some(image_path)).await?;
        }
    }

    Ok(())
}

pub async fn delete_user_avatar(db: &PgPool, user_id: Uuid) -> AppResult<()> {
    update_user_avatar(db, user_id, None).await?;
    Ok(())
}
