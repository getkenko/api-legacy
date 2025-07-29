use axum::{extract::{Multipart, State}, http::StatusCode, middleware, routing::{get, patch, post}, Extension, Json, Router};

use crate::{models::{dto::users::{DeleteAccountRequest, FullUserView, UpdateUser, UpdateUserDetailsRequest, UpdateUserPreferencesRequest}, errors::AppResult}, security::{jwt::Token, middlewares::auth_middleware}, services::users::{delete_user_account, delete_user_avatar, get_full_user_info, update_user_avatar_from_form, update_user_credentials, update_user_details, update_user_preferences}};

use super::AppState;

pub fn router(state: AppState) -> Router<AppState> {
    Router::new()
        .route("/me", get(user_info).patch(update_user).delete(delete_account))
        .route("/me/details", patch(update_details))
        .route("/me/preferences", patch(update_preferences))
        .route("/me/avatar", post(update_avatar).delete(delete_avatar))

        .layer(middleware::from_fn_with_state(state, auth_middleware))
}

async fn user_info(
    State(state): State<AppState>,
    Extension(token): Extension<Token>,
) -> AppResult<Json<FullUserView>> {
    let user = get_full_user_info(&state.db, token.sub).await?;
    Ok(Json(user))
}

async fn update_user(
    State(state): State<AppState>,
    Extension(token): Extension<Token>,
    Json(update): Json<UpdateUser>,
) -> AppResult<StatusCode> {
    update_user_credentials(&state.db, token.sub, update).await?;
    Ok(StatusCode::NO_CONTENT)
}

async fn update_details(
    State(state): State<AppState>,
    Extension(token): Extension<Token>,
    Json(new_details): Json<UpdateUserDetailsRequest>,
) -> AppResult<StatusCode> {
    update_user_details(&state.db, token.sub, new_details).await?;
    Ok(StatusCode::NO_CONTENT)
}

async fn update_preferences(
    State(state): State<AppState>,
    Extension(token): Extension<Token>,
    Json(preferences): Json<UpdateUserPreferencesRequest>,
) -> AppResult<StatusCode> {
    update_user_preferences(&state.db, token.sub, preferences).await?;
    Ok(StatusCode::NO_CONTENT)
}

async fn update_avatar(
    State(state): State<AppState>,
    Extension(token): Extension<Token>,
    multipart: Multipart,
) -> AppResult<()> {
    update_user_avatar_from_form(&state.db, token.sub, multipart).await?;
    Ok(())
}

async fn delete_avatar(
    State(state): State<AppState>,
    Extension(token): Extension<Token>,
) -> AppResult<StatusCode> {
    delete_user_avatar(&state.db, token.sub).await?;
    Ok(StatusCode::NO_CONTENT)
}

async fn delete_account(
    State(state): State<AppState>,
    Extension(token): Extension<Token>,
    Json(body): Json<DeleteAccountRequest>,
) -> AppResult<StatusCode> {
    delete_user_account(&state.db, token.sub, &body.password).await?;
    Ok(StatusCode::NO_CONTENT)
}