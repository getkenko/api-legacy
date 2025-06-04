use axum::{extract::State, http::StatusCode, routing::post, Json, Router};

use crate::{database::user::{check_user_conflicts, insert_user}, models::{dto::{LoginCredentials, RegisterUserData}, errors::AppResult}, utils::validation::{validate_email, validate_password, validate_username}};

use super::AppState;

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/login", post(login))
        .route("/register", post(register))
}

async fn login(
    State(db): State<AppState>,
    Json(credentials): Json<LoginCredentials>,
) -> AppResult<()> {
    // check if user with provided email and password exists
    // create JWT tokens
    // return 'em

    Ok(())
}

async fn register(
    State(db): State<AppState>,
    Json(user_data): Json<RegisterUserData>,
) -> AppResult<StatusCode> {
    validate_username(&user_data.username)?;
    validate_email(&user_data.email)?;
    validate_password(&user_data.password)?;

    check_user_conflicts(&db, &user_data).await?;

    insert_user(&db, &user_data).await?;

    Ok(StatusCode::CREATED)
}
