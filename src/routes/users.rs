use axum::{extract::{Multipart, State}, http::StatusCode, middleware, routing::{get, patch, post}, Extension, Json, Router};
use sqlx::{Postgres, QueryBuilder};
use tokio::{fs::File, io::AsyncWriteExt};
use uuid::Uuid;

use crate::{database::user::fetch_full_user, models::{dto::{FullUserView, NewUserDetails, NewUserPreferences}, errors::{AppError, AppResult}}, security::{jwt::Token, middlewares::auth_middleware}};

use super::AppState;

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/me", get(user_info))
        .route("/me/details", patch(update_user_details))
        .route("/me/preferences", patch(update_user_preferences))
        .route("/me/avatar", post(update_avatar).delete(delete_avatar))

        .layer(middleware::from_fn(auth_middleware))
}

async fn user_info(
    State(db): State<AppState>,
    Extension(token): Extension<Token>,
) -> AppResult<Json<FullUserView>> {
    // fetch full user data and convert it to user data view
    let user = fetch_full_user(&db, &token.sub).await?;
    // i could use .into() but explicitly converting makes the code more readable
    let user_view = FullUserView::from(user);
    Ok(Json(user_view))
}

async fn update_user_details(
    State(db): State<AppState>,
    Extension(token): Extension<Token>,
    new_details: Json<NewUserDetails>,
) -> AppResult<StatusCode> {
    if new_details.is_male.is_none() && new_details.weight.is_none() && new_details.height.is_none() && new_details.date_of_birth.is_none() {
        return Ok(StatusCode::NO_CONTENT); // should we return error instead?
    }

    let mut builder = QueryBuilder::<Postgres>::new("UPDATE user_details SET ");
    let mut separated = builder.separated(", ");

    if let Some(is_male) = new_details.is_male {
        separated.push("is_male = ");
        separated.push_bind_unseparated(is_male);
    }

    if let Some(weight) = new_details.weight {
        separated.push("weight = ");
        separated.push_bind_unseparated(weight);
    }

    if let Some(height) = new_details.height {
        separated.push("height = ");
        separated.push_bind_unseparated(height);
    }

    if let Some(date_of_birth) = new_details.date_of_birth {
        separated.push("date_of_birth = ");
        separated.push_bind_unseparated(date_of_birth);
    }

    builder.push(" WHERE user_id = ");
    builder.push_bind(token.sub);

    builder.build().execute(&db).await?;

    Ok(StatusCode::NO_CONTENT)
}

async fn update_user_preferences(
    State(db): State<AppState>,
    Extension(token): Extension<Token>,
    Json(new_pref): Json<NewUserPreferences>,
) -> AppResult<StatusCode> {
    if new_pref.theme.is_none() && new_pref.language.is_none() {
        return Ok(StatusCode::NO_CONTENT); // should we return error instead?
    }

    let mut builder = QueryBuilder::<Postgres>::new("UPDATE user_preferences SET ");
    let mut separated = builder.separated(", ");

    if let Some(is_male) = new_pref.theme {
        separated.push("is_male = ");
        separated.push_bind_unseparated(is_male);
    }

    if let Some(weight) = new_pref.language {
        separated.push("weight = ");
        separated.push_bind_unseparated(weight);
    }

    builder.push(" WHERE user_id = ");
    builder.push_bind(token.sub);

    builder.build().execute(&db).await?;

    Ok(StatusCode::NO_CONTENT)
}

async fn update_avatar(
    State(db): State<AppState>,
    Extension(token): Extension<Token>,
    mut multipart: Multipart,
) -> AppResult<()> {
    while let Some(field) = multipart.next_field().await? {
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
            sqlx::query!(
                "UPDATE users SET avatar_url = $1 WHERE id = $2",
                image_path, token.sub,
            )
            .execute(&db)
            .await?;
        }
    }

    Ok(())
}

async fn delete_avatar(
    State(db): State<AppState>,
    Extension(token): Extension<Token>,
) -> AppResult<StatusCode> {
    sqlx::query!("UPDATE users SET avatar_url = NULL WHERE id = $1", token.sub)
        .execute(&db)
        .await?;

    Ok(StatusCode::NO_CONTENT)
}
