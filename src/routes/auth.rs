use axum::{Json, Router, extract::State, http::StatusCode, routing::post};
use serde::{Deserialize, Serialize};

use crate::{
    models::{
        dto::auth::{LoginRequest, LoginResponse, RegisterRequest},
        errors::{AppError, AppResult},
    },
    services::auth::{
        check_email_availability, check_username_availability, process_login, process_register,
    },
};

use super::AppState;

#[derive(Deserialize)]
pub struct CheckUsernameRequest {
    username: String,
}

#[derive(Deserialize)]
pub struct CheckEmailRequest {
    email: String,
}

#[derive(Serialize)]
pub struct AvailabilityResponse {
    available: bool,
}

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/login", post(login))
        .route("/register", post(register))
        .route("/check-username", post(check_username))
        .route("/check-email", post(check_email))
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

async fn check_username(
    State(state): State<AppState>,
    Json(req): Json<CheckUsernameRequest>,
) -> AppResult<Json<AvailabilityResponse>> {
    match check_username_availability(&state.db, &req.username).await {
        Ok(_) => Ok(Json(AvailabilityResponse { available: true })),
        Err(AppError::UsernameTaken) => Ok(Json(AvailabilityResponse { available: false })),
        Err(e) => Err(e),
    }
}

async fn check_email(
    State(state): State<AppState>,
    Json(req): Json<CheckEmailRequest>,
) -> AppResult<Json<AvailabilityResponse>> {
    match check_email_availability(&state.db, &req.email).await {
        Ok(_) => Ok(Json(AvailabilityResponse { available: true })),
        Err(AppError::EmailTaken) => Ok(Json(AvailabilityResponse { available: false })),
        Err(e) => Err(e),
    }
}
