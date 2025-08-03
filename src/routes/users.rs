use axum::{extract::{Multipart, State}, http::StatusCode, middleware, routing::{get, patch, post}, Extension, Json, Router};

use crate::{models::{dto::users::{DeleteAccountRequest, FullUserView, UpdateUser, UpdateUserDetailsRequest, UpdateUserGoalsDto, UpdateUserPreferencesDto}, errors::AppResult}, security::{jwt::Token, middlewares::auth_middleware}, services::user_service};

use super::AppState;

pub fn router(state: AppState) -> Router<AppState> {
    Router::new()
        .route("/me", get(user_info).patch(update_user).delete(delete_account))
        .route("/me/details", patch(update_details))
        .route("/me/preferences", patch(update_preferences))
        .route("/me/goals", patch(update_goals))
        .route("/me/avatar", post(update_avatar).delete(delete_avatar))

        .layer(middleware::from_fn_with_state(state, auth_middleware))
}

async fn user_info(
    State(state): State<AppState>,
    Extension(token): Extension<Token>,
) -> AppResult<Json<FullUserView>> {
    let user = user_service::get_full_info(&state.db, token.sub).await?;
    Ok(Json(user))
}

async fn update_user(
    State(state): State<AppState>,
    Extension(token): Extension<Token>,
    Json(update): Json<UpdateUser>,
) -> AppResult<StatusCode> {
    user_service::update_credentials(&state.db, token.sub, update).await?;
    Ok(StatusCode::NO_CONTENT)
}

async fn update_details(
    State(state): State<AppState>,
    Extension(token): Extension<Token>,
    Json(new_details): Json<UpdateUserDetailsRequest>,
) -> AppResult<StatusCode> {
    user_service::update_details(&state.db, token.sub, new_details).await?;
    Ok(StatusCode::NO_CONTENT)
}

async fn update_preferences(
    State(state): State<AppState>,
    Extension(token): Extension<Token>,
    Json(preferences): Json<UpdateUserPreferencesDto>,
) -> AppResult<StatusCode> {
    user_service::update_preferences(&state.db, token.sub, preferences).await?;
    Ok(StatusCode::NO_CONTENT)
}

async fn update_goals(
    State(state): State<AppState>,
    Extension(token): Extension<Token>,
    Json(goals): Json<UpdateUserGoalsDto>,
) -> AppResult<StatusCode> {
    user_service::update_goals(&state.db, token.sub, goals).await?;
    Ok(StatusCode::NO_CONTENT)
}

async fn update_avatar(
    State(state): State<AppState>,
    Extension(token): Extension<Token>,
    multipart: Multipart,
) -> AppResult<()> {
    user_service::update_avatar(&state.db, token.sub, multipart).await?;
    Ok(())
}

async fn delete_avatar(
    State(state): State<AppState>,
    Extension(token): Extension<Token>,
) -> AppResult<StatusCode> {
    user_service::delete_avatar(&state.db, token.sub).await?;
    Ok(StatusCode::NO_CONTENT)
}

async fn delete_account(
    State(state): State<AppState>,
    Extension(token): Extension<Token>,
    Json(body): Json<DeleteAccountRequest>,
) -> AppResult<StatusCode> {
    user_service::delete(&state.db, token.sub, &body.password).await?;
    Ok(StatusCode::NO_CONTENT)
}