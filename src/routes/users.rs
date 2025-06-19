use axum::{extract::State, http::StatusCode, middleware, routing::{get, patch}, Extension, Json, Router};
use sqlx::{Execute, Postgres, QueryBuilder};

use crate::{database::user::fetch_user_info, models::{dto::{NewUserDetails, NewUserPreferences, UserInfo}, errors::AppResult}, utils::{auth_middleware::auth_middleware, jwt::AccessToken}};

use super::AppState;

pub fn router(state: AppState) -> Router<AppState> {
    Router::new()
        .route("/me", get(user_info))
        .route("/me/details", patch(update_user_details))
        .route("/me/preferences", patch(update_user_preferences))

        .layer(middleware::from_fn_with_state(state, auth_middleware))
}

async fn user_info(
    State(db): State<AppState>,
    token: Extension<AccessToken>,
) -> AppResult<Json<UserInfo>> {
    let info = fetch_user_info(&db, &token.sub).await?;
    Ok(Json(info))
}

async fn update_user_details(
    State(db): State<AppState>,
    token: Extension<AccessToken>,
    new_details: Json<NewUserDetails>,
) -> AppResult<StatusCode> {
    if new_details.is_male.is_none() && new_details.weight.is_none() && new_details.height.is_none() && new_details.date_of_birth.is_none() {
        return Ok(StatusCode::NO_CONTENT); // should we return error instead?
    }

    let mut builder = QueryBuilder::<Postgres>::new("UPDATE user_details SET ");
    let mut separated = builder.separated(", ");

    if let Some(is_male) = new_details.is_male {
        separated.push_unseparated("is_male = ");
        separated.push_bind(is_male);
    }

    if let Some(weight) = new_details.weight {
        separated.push_unseparated("weight = ");
        separated.push_bind(weight);
    }

    if let Some(height) = new_details.height {
        separated.push_unseparated("height = ");
        separated.push_bind(height);
    }

    if let Some(date_of_birth) = new_details.date_of_birth {
        separated.push_unseparated("date_of_birth = ");
        separated.push_bind(date_of_birth);
    }

    builder.push(" WHERE id = ");
    builder.push_bind(token.sub);

    let query = builder.build().sql();

    println!("{query}");

    // sqlx::query(query).execute(&db).await?;

    Ok(StatusCode::NO_CONTENT)
}

async fn update_user_preferences(
    State(db): State<AppState>,
    token: Extension<AccessToken>,
    Json(new_pref): Json<NewUserPreferences>,
) -> AppResult<StatusCode> {
    if new_pref.theme.is_none() && new_pref.language.is_none() {
        return Ok(StatusCode::NO_CONTENT); // should we return error instead?
    }

    let mut builder = QueryBuilder::<Postgres>::new("UPDATE user_preferences SET ");
    let mut separated = builder.separated(", ");

    if let Some(is_male) = new_pref.theme {
        separated.push_unseparated("is_male = ");
        separated.push_bind(is_male);
    }

    if let Some(weight) = new_pref.language {
        separated.push_unseparated("weight = ");
        separated.push_bind(weight);
    }

    let query = builder.build().sql();

    sqlx::query(query).execute(&db).await?;

    Ok(StatusCode::NO_CONTENT)
}
