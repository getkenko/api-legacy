use std::collections::HashMap;

use axum::{Extension, Json, Router, extract::{Path, State}, http::StatusCode, middleware, routing::{delete, get, post}};
use chrono::NaiveDate;
use uuid::Uuid;

use crate::{database::meal::{check_meal_item_exists, check_user_meal_section_exists, delete_meal_item, fetch_user_meal_sections, fetch_user_meals_products, insert_meal_product}, models::{database::meal::InsertMealProduct, dto::meals::{AddMealProductRequest, MealMacroResponse, QuickAddMealProductRequest, UserMealProductView, UserMealSectionView}, errors::{AppError, AppResult}}, security::{jwt::Token, middlewares::auth_middleware}};

use super::AppState;

pub fn router(state: AppState) -> Router<AppState> {
    Router::new()
        .route("/sections", get(user_sections))
        .route("/products/{product_id}", delete(delete_meal_product))
        .route("/{date}", get(user_meals))
        .route("/{date}/macro", get(meal_macro))
        .route("/{date}/products", post(add_meal_product))
        .route("/{date}/products/quick", post(quick_add_meal_product))
        .layer(middleware::from_fn_with_state(state, auth_middleware))
}

async fn meal_macro(
    State(state): State<AppState>,
    Extension(token): Extension<Token>,
    Path(date): Path<NaiveDate>,
) -> AppResult<Json<MealMacroResponse>> {
    let products = fetch_user_meals_products(&state.db, token.sub, date).await?;
    let day_macro = MealMacroResponse::from_meals_products(&products);
    Ok(Json(day_macro))
}

async fn user_meals(
    State(state): State<AppState>,
    Extension(token): Extension<Token>,
    Path(date): Path<NaiveDate>,
) -> AppResult<Json<HashMap<Uuid, Vec<UserMealProductView>>>> {
    let mut map: HashMap<Uuid, Vec<UserMealProductView>> = HashMap::new();

    let meals_products = fetch_user_meals_products(&state.db, token.sub, date).await?;

    for meal_product in meals_products.iter() {
        let meal_product_view = UserMealProductView::from(meal_product); // explicit on purpose

        if let Some(section) = map.get_mut(&meal_product.section_id) { // already exists
            section.push(meal_product_view);
        } else { // create new key-value pair
            map.insert(meal_product.section_id, vec![meal_product_view]);
        }
    }

    Ok(Json(map))
}

// sections
async fn user_sections(
    State(state): State<AppState>,
    Extension(token): Extension<Token>,
) -> AppResult<Json<Vec<UserMealSectionView>>> {
    let sections = fetch_user_meal_sections(&state.db, token.sub).await?;
    let sections_view = sections
        .into_iter()
        .map(|s| UserMealSectionView::from(s))
        .collect::<Vec<_>>();

    Ok(Json(sections_view))
}

// products
async fn add_meal_product(
    State(state): State<AppState>,
    Extension(token): Extension<Token>,
    Path(date): Path<NaiveDate>,
    Json(product): Json<AddMealProductRequest>,
) -> AppResult<StatusCode> {
    let section_exists = check_user_meal_section_exists(&state.db, product.section_id).await?;
    if !section_exists {
        return Err(AppError::MealSectionNotFound);
    }

    let insert = InsertMealProduct::from(product);
    insert_meal_product(&state.db, token.sub, date, insert).await?;

    Ok(StatusCode::CREATED)
}

async fn quick_add_meal_product(
    State(state): State<AppState>,
    Extension(token): Extension<Token>,
    Path(date): Path<NaiveDate>,
    Json(product): Json<QuickAddMealProductRequest>,
) -> AppResult<StatusCode> {
    let section_exists = check_user_meal_section_exists(&state.db, product.section_id).await?;
    if !section_exists {
        return Err(AppError::MealSectionNotFound);
    }

    let insert = InsertMealProduct::from(product);
    insert_meal_product(&state.db, token.sub, date, insert).await?;

    Ok(StatusCode::CREATED)
}

async fn delete_meal_product(
    State(state): State<AppState>,
    Path(meal_product_id): Path<Uuid>,
) -> AppResult<StatusCode> {
    if !check_meal_item_exists(&state.db, meal_product_id).await? {
        return Err(AppError::MealProductNotFound);
    }

    delete_meal_item(&state.db, meal_product_id).await?;

    Ok(StatusCode::NO_CONTENT)
}
