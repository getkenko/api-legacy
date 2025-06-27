use axum::{extract::State, http::StatusCode, routing::post, Json, Router};

use crate::{models::{dto::auth::{LoginRequest, LoginResponse, RegisterRequest}, errors::AppResult}, services::auth::{process_login, process_register}};

use super::AppState;

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/login", post(login))
        .route("/register", post(register))
}

async fn login(
    State(state): State<AppState>,
    Json(creds): Json<LoginRequest>,
) -> AppResult<Json<LoginResponse>> {
    let res = process_login(&state.db, creds).await?;
    Ok(Json(res))
}

async fn register(
    State(state): State<AppState>,
    Json(user_data): Json<RegisterRequest>,
) -> AppResult<StatusCode> {
    process_register(&state.db, user_data).await?;
    Ok(StatusCode::CREATED)
}
