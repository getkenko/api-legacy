use axum::extract::Multipart;
use sqlx::PgPool;
use tokio::{fs::File, io::AsyncWriteExt};
use uuid::Uuid;

use crate::{database::user::{fetch_full_user, fetch_user_units, update_user_avatar, update_user_details_opt, update_user_preferences_opt}, models::{database::enums::{HeightUnit, WeightUnit}, dto::users::{FullUserView, UpdateUserDetailsRequest, UpdateUserPreferencesRequest}, errors::{AppResult, ValidationError}}, utils::{conversion::{ft_in_to_cm, lb_to_kg, st_lb_to_kg}, validation::{is_activity_in_range, validate_date_of_birth}}};

pub async fn get_full_user_info(db: &PgPool, user_id: Uuid) -> AppResult<FullUserView> {
    let user = fetch_full_user(db, user_id).await?;
    let user_view = FullUserView::from(user);
    Ok(user_view)
}

fn weight_from_unit(unit: WeightUnit, details: &UpdateUserDetailsRequest) -> AppResult<f32> {
    let weight = match unit {
        WeightUnit::Kg => details.weight_kg.ok_or(ValidationError::MissingKgWeight)?,
        WeightUnit::Lb => {
            let lb = details.weight_lb.ok_or(ValidationError::MissingLbWeight)?;
            lb_to_kg(lb)
        }
        WeightUnit::StLb => {
            let st = details.weight_st.ok_or(ValidationError::MissingStLbWeight)?;
            let lb = details.weight_lb.ok_or(ValidationError::MissingStLbWeight)?;
            st_lb_to_kg(st, lb)
        }
    };

    if weight <= 0.0 || weight >= 10000.0 {
        return Err(ValidationError::InvalidWeight)?;
    }

    Ok(weight)
}

fn height_from_unit(unit: HeightUnit, details: &UpdateUserDetailsRequest) -> AppResult<i32> {
    let height = match unit {
        HeightUnit::Cm => details.height_cm.ok_or(ValidationError::MissingCmHeight)?,
        HeightUnit::FtIn => {
            let ft = details.height_ft.ok_or(ValidationError::MissingFtInHeight)?;
            let inches = details.height_in.ok_or(ValidationError::MissingFtInHeight)?;
            ft_in_to_cm(ft, inches)
        }
    };

    if height <= 0 || height >= 300 {
        return Err(ValidationError::InvalidHeight)?;
    }

    Ok(height)
}

pub async fn update_user_details(
    db: &PgPool,
    user_id: Uuid,
    details: UpdateUserDetailsRequest,
) -> AppResult<()> {
    let is_weight_provided = details.weight_kg.is_some() || details.weight_lb.is_some() || details.weight_st.is_some();
    let is_height_provided = details.height_cm.is_some() || details.height_ft.is_some() || details.height_in.is_some();

    if 
    details.sex.is_none() && !is_weight_provided && !is_height_provided &&
    details.date_of_birth.is_none() && details.idle_activity.is_none() &&
    details.workout_activity.is_none()
    {
        return Err(ValidationError::NoFieldsToUpdate)?;
    }

    // piratesoftware code type shit
    let (weight, height) = if is_weight_provided || is_height_provided {
        let (weight_unit, height_unit) = fetch_user_units(db, user_id).await?;

        let weight = weight_from_unit(weight_unit, &details).map(Some)?;
        let height = height_from_unit(height_unit, &details).map(Some)?;

        (weight, height)
    } else {
        (None, None)
    };

    if let Some(dob) = details.date_of_birth {
        validate_date_of_birth(dob)?;
    }

    if let Some(activity) = details.idle_activity {
        if !is_activity_in_range(activity) {
            return Err(ValidationError::InvalidIdleActivity)?;
        }
    }

    if let Some(activity) = details.workout_activity {
        if !is_activity_in_range(activity) {
            return Err(ValidationError::InvalidWorkoutActivity)?;
        }
    }

    update_user_details_opt(
        db,
        user_id,
        details.sex,
        weight,
        height,
        details.date_of_birth,
        details.idle_activity,
        details.workout_activity,
    ).await?;

    Ok(())
}

pub async fn update_user_preferences(db: &PgPool, user_id: Uuid, preferences: UpdateUserPreferencesRequest) -> AppResult<()> {
    if preferences.theme.is_none() && preferences.language.is_none() {
        return Err(ValidationError::NoFieldsToUpdate)?;
    }

    update_user_preferences_opt(db, user_id, preferences.theme, preferences.language).await?;

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
            update_user_avatar(db, user_id, Some(image_path)).await?;
        }
    }

    Ok(())
}

pub async fn delete_user_avatar(db: &PgPool, user_id: Uuid) -> AppResult<()> {
    update_user_avatar(db, user_id, None).await?;
    Ok(())
}
