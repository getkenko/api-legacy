use axum::{extract::{Path, State}, http::StatusCode, middleware, response::IntoResponse, routing::{get, patch, post}, Extension, Json, Router};
use uuid::Uuid;

use crate::{models::{dto::{meals::UserMealSectionView, sections::{NewSectionRequest, UpdateSectionRequest}}, errors::AppResult}, routes::AppState, security::{jwt::Token, middlewares::auth_middleware}, services::sections::{create_new_section, delete_user_section, get_user_sections_layout, reset_user_section_layout, update_user_section}};

pub fn router(state: AppState) -> Router<AppState> {
    Router::new()
        .route("/", get(user_sections).post(new_section))
        .route("/{id}", patch(update_section).delete(delete_section))
        .route("/reset", post(reset_sections))

        .layer(middleware::from_fn_with_state(state, auth_middleware))
}

async fn new_section(
    State(state): State<AppState>,
    Extension(token): Extension<Token>,
    Json(section): Json<NewSectionRequest>,
) -> AppResult<impl IntoResponse> {
    let section = create_new_section(&state.db, token.sub, section).await?;
    Ok((StatusCode::CREATED, Json(section)))
}

async fn user_sections(
    State(state): State<AppState>,
    Extension(token): Extension<Token>,
) -> AppResult<Json<Vec<UserMealSectionView>>> {
    let sections = get_user_sections_layout(&state.db, token.sub).await?;
    Ok(Json(sections))
}

async fn update_section(
    State(state): State<AppState>,
    Extension(token): Extension<Token>,
    Path(section_id): Path<Uuid>,
    Json(update): Json<UpdateSectionRequest>,
) -> AppResult<Json<UserMealSectionView>> {
    let section = update_user_section(&state.db, token.sub, section_id, update).await?;
    Ok(Json(section))
}

async fn delete_section(
    State(state): State<AppState>,
    Extension(token): Extension<Token>,
    Path(section_id): Path<Uuid>,
) -> AppResult<StatusCode> {
    delete_user_section(&state.db, token.sub, section_id).await?;
    Ok(StatusCode::NO_CONTENT)
}

async fn reset_sections(
    State(state): State<AppState>,
    Extension(token): Extension<Token>,
) -> AppResult<Json<Vec<UserMealSectionView>>> {
    let sections = reset_user_section_layout(&state.db, token.sub).await?;
    Ok(Json(sections))
}