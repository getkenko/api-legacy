use std::collections::HashMap;

use axum::{Extension, Json, Router, extract::{Path, State}, http::StatusCode, middleware, routing::{delete, get, post}};
use chrono::NaiveDate;
use uuid::Uuid;

use crate::{models::{dto::meals::{AddMealProductRequest, MealMacroResponse, QuickAddMealProductRequest, UserMealProductView, UserMealSectionView}, errors::AppResult}, security::{jwt::Token, middlewares::auth_middleware}, services::meals::{add_meal_product_for_date, calculate_meal_day_macro, delete_meal_product, get_user_meals_for_date, get_user_sections_layout, quick_add_meal_product_for_date}};

use super::AppState;

pub fn router(state: AppState) -> Router<AppState> {
    Router::new()
        .route("/sections", get(user_sections))
        .route("/products/{product_id}", delete(handle_delete_meal_product))
        .route("/{date}", get(user_meals))
        .route("/{date}/macro", get(meal_day_macro))
        .route("/{date}/products", post(add_meal_product))
        .route("/{date}/products/quick", post(quick_add_meal_product))
        .layer(middleware::from_fn_with_state(state, auth_middleware))
}

// index
async fn meal_day_macro(
    State(state): State<AppState>,
    Extension(token): Extension<Token>,
    Path(date): Path<NaiveDate>,
) -> AppResult<Json<MealMacroResponse>> {
    let day_macro = calculate_meal_day_macro(&state.db, token.sub, date).await?;
    Ok(Json(day_macro))
}

async fn user_meals(
    State(state): State<AppState>,
    Extension(token): Extension<Token>,
    Path(date): Path<NaiveDate>,
) -> AppResult<Json<HashMap<Uuid, Vec<UserMealProductView>>>> {
    let meals = get_user_meals_for_date(&state.db, token.sub, date).await?;
    Ok(Json(meals))
}

// sections
async fn user_sections(
    State(state): State<AppState>,
    Extension(token): Extension<Token>,
) -> AppResult<Json<Vec<UserMealSectionView>>> {
    let sections = get_user_sections_layout(&state.db, token.sub).await?;
    Ok(Json(sections))
}

// products
async fn add_meal_product(
    State(state): State<AppState>,
    Extension(token): Extension<Token>,
    Path(date): Path<NaiveDate>,
    Json(product): Json<AddMealProductRequest>,
) -> AppResult<StatusCode> {
    add_meal_product_for_date(&state.db, token.sub, date, product).await?;
    Ok(StatusCode::CREATED)
}

async fn quick_add_meal_product(
    State(state): State<AppState>,
    Extension(token): Extension<Token>,
    Path(date): Path<NaiveDate>,
    Json(product): Json<QuickAddMealProductRequest>,
) -> AppResult<StatusCode> {
    quick_add_meal_product_for_date(&state.db, token.sub, date, product).await?;
    Ok(StatusCode::CREATED)
}

async fn handle_delete_meal_product(
    State(state): State<AppState>,
    Path(product_id): Path<Uuid>,
) -> AppResult<StatusCode> {
    delete_meal_product(&state.db, product_id).await?;
    Ok(StatusCode::NO_CONTENT)
}
