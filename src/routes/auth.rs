use axum::{extract::State, http::StatusCode, routing::post, Json, Router};

use crate::{database::user::{check_user_conflicts, insert_user}, models::{database::{AccountState}, dto::{LoginCredentials, LoginResponse, RegisterUserData}, errors::{AppError, AppResult}}, utils::{jwt::{AccessToken, AuthToken, RefreshToken}, password::verify_password, validation::{validate_email, validate_password, validate_username}}};

use super::AppState;

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/login", post(login))
        .route("/register", post(register))
}

async fn login(
    State(db): State<AppState>,
    Json(credentials): Json<LoginCredentials>,
) -> AppResult<Json<LoginResponse>> {
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

    if user.account_state != AccountState::Active {
        return Err(AppError::AccountNotActive(user.account_state));
    }

    // create JWT tokens
    let access = AccessToken::new(&user.id, &user.display_name);
    let refresh = RefreshToken::new(&user.id);

    // return em
    let body = LoginResponse {
        access: access.encode()?,
        refresh: refresh.encode()?,
    };

    Ok(Json(body))
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
