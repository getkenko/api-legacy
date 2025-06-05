use axum::{extract::State, http::StatusCode, routing::post, Json, Router};

use crate::{database::user::{check_user_conflicts, insert_user}, models::{database::{AccountState, User}, dto::{LoginCredentials, RegisterUserData}, errors::{AppError, AppResult}}, utils::{password::verify_password, validation::{validate_email, validate_password, validate_username}}};

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
    // try to find the user
    let user = sqlx::query!(
        r#"
        SELECT id, display_name, password, account_state AS "account_state: AccountState"
        FROM users
        WHERE email = $1
        LIMIT 1
        "#,
        credentials.email,
    )
    .fetch_optional(&db)
    .await?
    .ok_or(AppError::InvalidCredentials)?;

    // check if password matches
    if !verify_password(&credentials.password, &user.password).map_err(AppError::Crypto)? {
        return Err(AppError::InvalidCredentials);
    }

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
