use std::collections::HashMap;

use axum::{extract::{Path, Query, State}, http::StatusCode, middleware, response::IntoResponse, routing::{delete, get, post}, Extension, Json, Router};
use chrono::NaiveDate;
use uuid::Uuid;

use crate::{models::{dto::meals::{AddMealProductRequest, DeleteMealProductsQuery, MealDayMacrosResponse, QuickAddMealProductRequest, UserMealProductView}, errors::AppResult}, security::{jwt::Token, middlewares::auth_middleware}, services::meals::{add_meal_product_for_date, calc_meal_day_macros, delete_meal_product, delete_meal_products_date, get_user_meals_for_date, quick_add_meal_product_for_date}};

use super::AppState;

pub fn router(state: AppState) -> Router<AppState> {
    Router::new()
        .route("/products/{product_id}", delete(handle_delete_meal_product))
        .route("/{date}", get(user_meals))
        .route("/{date}/macros", get(meal_day_macros))
        .route("/{date}/products", post(add_meal_product).delete(delete_meal_products))
        .route("/{date}/products/quick", post(quick_add_meal_product))

        .layer(middleware::from_fn_with_state(state, auth_middleware))
}

async fn meal_day_macros(
    State(state): State<AppState>,
    Extension(token): Extension<Token>,
    Path(date): Path<NaiveDate>,
) -> AppResult<Json<MealDayMacrosResponse>> {
    let macros = calc_meal_day_macros(&state.db, token.sub, date).await?;
    Ok(Json(macros))
}

async fn user_meals(
    State(state): State<AppState>,
    Extension(token): Extension<Token>,
    Path(date): Path<NaiveDate>,
) -> AppResult<Json<HashMap<Uuid, Vec<UserMealProductView>>>> {
    let meals = get_user_meals_for_date(&state.db, token.sub, date).await?;
    Ok(Json(meals))
}

async fn add_meal_product(
    State(state): State<AppState>,
    Extension(token): Extension<Token>,
    Path(date): Path<NaiveDate>,
    Json(product): Json<AddMealProductRequest>,
) -> AppResult<impl IntoResponse> {
    let product = add_meal_product_for_date(&state.db, token.sub, date, product).await?;
    Ok((StatusCode::CREATED, Json(product)))
}

async fn quick_add_meal_product(
    State(state): State<AppState>,
    Extension(token): Extension<Token>,
    Path(date): Path<NaiveDate>,
    Json(product): Json<QuickAddMealProductRequest>,
) -> AppResult<impl IntoResponse> {
    let product = quick_add_meal_product_for_date(&state.db, token.sub, date, product).await?;
    Ok((StatusCode::CREATED, Json(product)))
}

async fn handle_delete_meal_product(
    State(state): State<AppState>,
    Path(product_id): Path<Uuid>,
) -> AppResult<StatusCode> {
    delete_meal_product(&state.db, product_id).await?;
    Ok(StatusCode::NO_CONTENT)
}

async fn delete_meal_products(
    State(state): State<AppState>,
    Path(date): Path<NaiveDate>,
    Query(opt): Query<DeleteMealProductsQuery>,
) -> AppResult<StatusCode> {
    delete_meal_products_date(&state.db, date, opt.section_id).await?;
    Ok(StatusCode::NO_CONTENT)
}
