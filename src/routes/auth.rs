use axum::{extract::State, http::StatusCode, routing::post, Json, Router};

use crate::{models::{dto::{LoginCredentials, LoginResponse, RegisterUserData}, errors::AppResult}, services::auth::{process_login, process_register}};

use super::AppState;

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/login", post(login))
        .route("/register", post(register))
}

async fn login(
    State(db): State<AppState>,
    Json(creds): Json<LoginCredentials>,
) -> AppResult<Json<LoginResponse>> {
    let res = process_login(&db, creds).await?;
    Ok(Json(res))
}

async fn register(
    State(db): State<AppState>,
    Json(user_data): Json<RegisterUserData>,
) -> AppResult<StatusCode> {
    process_register(&db, user_data).await?;
    Ok(StatusCode::CREATED)
}
