use axum::extract::Multipart;
use chrono::Utc;
use sqlx::PgPool;
use tokio::{fs::File, io::AsyncWriteExt};
use uuid::Uuid;

use crate::{database::{user_nutrients_repo, user_repo}, models::{dto::users::{FullUserView, UpdateUser, UpdateUserDetailsRequest, UpdateUserPreferencesRequest}, errors::{AppError, AppResult, ValidationError}}, security::password::{hash_password, verify_password}, utils::{conversion::{height_from_unit, weight_from_unit}, nutrition::{calc_base_tdee, calc_target_macros, calculate_bmr, calculate_tdee}, validation::{is_activity_in_range, validate_date_of_birth}}};

pub async fn get_full_user_info(db: &PgPool, user_id: Uuid) -> AppResult<FullUserView> {
    let user = user_repo::fetch_full_user(db, user_id).await?;
    let user_view = FullUserView::from(user);
    Ok(user_view)
}

pub async fn update_user_credentials(db: &PgPool, user_id: Uuid, update: UpdateUser) -> AppResult<()> {
    if update.display_name.is_none() && update.username.is_none() && update.password.is_none() && update.email.is_none() {
        return Err(ValidationError::NoFieldsProvided)?;
    }

    let mut user = user_repo::find_user(db, user_id)
        .await?
        .ok_or(AppError::UserNotFound)?;

    if !verify_password(&update.current_password, &user.password).map_err(AppError::Crypto)? {
        return Err(AppError::IncorrectPassword);
    }

    if let Some(username) = update.username {
        user.username = username;
    }

    if let Some(display_name) = update.display_name {
        user.display_name = display_name;
    }

    if let Some(password) = update.password {
        user.password = hash_password(&password).map_err(AppError::Crypto)?;
    }

    if let Some(email) = update.email {
        user.email = email;
    }

    user_repo::update_credentials(db, user_id, &user).await?;

    Ok(())
}

pub async fn update_user_details(
    db: &PgPool,
    user_id: Uuid,
    details: UpdateUserDetailsRequest,
) -> AppResult<()> {
    let is_weight_provided = details.weight_kg.is_some() || details.weight_lb.is_some() || details.weight_st.is_some();
    let is_height_provided = details.height_cm.is_some() || details.height_ft.is_some() || details.height_in.is_some();

    if details.sex.is_none() && !is_weight_provided && !is_height_provided &&
    details.date_of_birth.is_none() && details.idle_activity.is_none() &&
    details.workout_activity.is_none() && details.diet_kind.is_none()
    {
        return Err(ValidationError::NoFieldsProvided)?;
    }

    // we do fetch -> update because we need to use some of the values anyways
    let mut user = user_repo::fetch_user_details_with_goals(db, user_id).await?;

    if is_weight_provided {
        user.weight = weight_from_unit(&user.weight_unit, &details)?;
    }

    if is_height_provided {
        user.height = height_from_unit(&user.height_unit, &details)?;
    }

    if let Some(dob) = details.date_of_birth {
        validate_date_of_birth(dob)?;
        user.date_of_birth = dob;
    }

    if let Some(activity) = details.idle_activity {
        if !is_activity_in_range(activity) {
            return Err(ValidationError::InvalidIdleActivity)?;
        }
        user.idle_activity = activity;
    }

    if let Some(activity) = details.workout_activity {
        if !is_activity_in_range(activity) {
            return Err(ValidationError::InvalidWorkoutActivity)?;
        }
        user.workout_activity = activity;
    }

    let mut tx = db.begin().await?;
    user_repo::update_user_details(&mut *tx, user_id, &user).await?;

    // update user nutrients (any value updated in this function has impact on them)
    let age = Utc::now().date_naive().years_since(user.date_of_birth).unwrap_or(18);

    let bmr = calculate_bmr(user.weight, user.height, age, user.sex);
    let base_tdee = calc_base_tdee(bmr, user.workout_activity, user.idle_activity);
    let tdee = calculate_tdee(base_tdee, user.goal_diff_per_week, user.weight_goal);
    let macros = calc_target_macros(user.weight, tdee, user.weight_goal);
    user_nutrients_repo::update_user_nutrients(&mut *tx, user_id, bmr, base_tdee, tdee, &macros).await?;
    
    tx.commit().await?;

    Ok(())
}

pub async fn update_user_preferences(db: &PgPool, user_id: Uuid, preferences: UpdateUserPreferencesRequest) -> AppResult<()> {
    if preferences.theme.is_none() && preferences.language.is_none() {
        return Err(ValidationError::NoFieldsProvided)?;
    }

    user_repo::update_user_preferences_opt(db, user_id, preferences.theme, preferences.language).await?;

    Ok(())
}

pub async fn update_user_avatar_from_form(db: &PgPool, user_id: Uuid, mut form: Multipart) -> AppResult<()> {
    while let Some(field) = form.next_field().await? {
        if field.name() == Some("avatar") {
            let data = field.bytes().await?;
            let mime = infer::get(&data).ok_or(ValidationError::UnknownFileType)?;

            // check if it's accepted image format
            if mime.mime_type() != "image/png" && mime.mime_type() != "image/jpeg" {
                return Err(ValidationError::UnknownFileType)?;
            }

            // create file name
            let file_name = format!("{}.{}", Uuid::new_v4(), mime.extension());
            let image_path = format!("avatars/{file_name}"); // used in CDN url
            let file_path = format!("public/{image_path}"); // full file path with CDN directory

            // save to disk
            let mut file = File::create(file_path).await?;
            file.write_all(&data).await?;

            // update user's avatar in database
            user_repo::update_user_avatar(db, user_id, Some(image_path)).await?;
        }
    }

    Ok(())
}

pub async fn delete_user_avatar(db: &PgPool, user_id: Uuid) -> AppResult<()> {
    user_repo::update_user_avatar(db, user_id, None).await?;
    Ok(())
}

pub async fn delete_user_account(db: &PgPool, user_id: Uuid, password: &str) -> AppResult<()> {
    let user = user_repo::find_user(db, user_id)
        .await?
        .ok_or(AppError::UserNotFound)?;
    
    // check if password matches
    let password_matches = verify_password(password, &user.password).map_err(AppError::Crypto)?;
    if !password_matches {
        return Err(AppError::IncorrectPassword);
    }

    // delete user's account
    user_repo::delete_user(db, user_id).await?;

    Ok(())
}