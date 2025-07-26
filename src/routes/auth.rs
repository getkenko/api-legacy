use axum::{extract::{Query, State}, http::StatusCode, routing::{get, post}, Json, Router};

use crate::{models::{dto::auth::{CheckAvailabilityQuery, LoginRequest, LoginResponse, RegisterRequest, UserConflictsView}, errors::AppResult}, services::auth::{check_user_credentials_availability, process_login, process_register}};

use super::AppState;

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/login", post(login))
        .route("/register", post(register))
        .route("/check", get(check_availability))
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

async fn check_availability(
    State(state): State<AppState>,
    Query(to_check): Query<CheckAvailabilityQuery>,
) -> AppResult<Json<UserConflictsView>> {
    let availability = check_user_credentials_availability(&state.db, to_check).await?;
    Ok(Json(availability))
}
