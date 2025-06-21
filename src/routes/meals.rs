use axum::{extract::{Path, State}, http::StatusCode, routing::{delete, get, post}, Extension, Json, Router};
use chrono::NaiveDate;
use uuid::Uuid;

use crate::{database::{meal::{add_meal_product, check_meal_item_exists, delete_meal_item, fetch_user_meal_products_for_date, fetch_user_meal_section_exists, fetch_user_meal_sections}, product::fetch_product_by_id}, models::{database::{AddMealProduct, MealProduct, MealProductKind, Product}, dto::{AddProduct, MealDayMacro, QuickAddProduct, UserMealSectionView}, errors::{AppError, AppResult}}, utils::jwt::AccessToken};

use super::AppState;

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/sections", get(user_sections))
        .route("/{date}/macro", get(meal_macro))
        .route("/{date}/products", post(add_product))
        .route("/{date}/products/quick", post(quick_add_product)) // should we instead use query parameter in /products?
        .route("/{date}/products/{product_id}", delete(delete_meal_product))
}

async fn meal_macro(
    State(db): State<AppState>,
    Extension(token): Extension<AccessToken>,
    Path(date): Path<NaiveDate>,
) -> AppResult<Json<MealDayMacro>> {
    let mut macro_sum = MealDayMacro::default();
    let products = fetch_user_meal_products_for_date(&db, &token.sub, date).await?;

    for (product, quantity) in products {
        match product.kind {
            MealProductKind::QuickAdd =>
                // database already makes sure that these fields are set when kind is quick add
                macro_sum.add_raw(product.calories.unwrap(), product.proteins.unwrap(), product.fats.unwrap(), product.carbohydrates.unwrap()),
            MealProductKind::FromDatabase => {
                let product = fetch_product_by_id(&db, product.product_id.unwrap()).await?;
                macro_sum.add_product(&product, quantity);
            }
        }
    }

    Ok(Json(macro_sum))
}

// sections
async fn user_sections(
    State(db): State<AppState>,
    Extension(token): Extension<AccessToken>,
) -> AppResult<Json<Vec<UserMealSectionView>>> {
    let sections = fetch_user_meal_sections(&db, &token.sub).await?;
    Ok(Json(sections))
}

// products
async fn add_product(
    State(db): State<AppState>,
    Path(date): Path<NaiveDate>,
    Json(product): Json<AddProduct>,
) -> AppResult<StatusCode> {
    let section_exists = fetch_user_meal_section_exists(&db, &product.section_id).await?;
    if !section_exists {
        return Err(AppError::MealSectionNotFound);
    }

    let add_product = AddMealProduct::from_database(date, product.section_id, product.quantity, product.product_id);
    add_meal_product(&db, add_product).await?;

    Ok(StatusCode::CREATED)
}

async fn quick_add_product(
    State(db): State<AppState>,
    Path(date): Path<NaiveDate>,
    Json(product): Json<QuickAddProduct>,
) -> AppResult<StatusCode> {
    let section_exists = fetch_user_meal_section_exists(&db, &product.section_id).await?;
    if !section_exists {
        return Err(AppError::MealSectionNotFound);
    }

    let add_product = AddMealProduct::quick_add(date, product.section_id, product.quantity, product.name, product.calories, product.proteins, product.fats, product.carbohydrates);
    add_meal_product(&db, add_product).await?;

    Ok(StatusCode::CREATED)
}

async fn delete_meal_product(
    State(db): State<AppState>,
    Path(meal_product_id): Path<Uuid>,
) -> AppResult<StatusCode> {
    if !check_meal_item_exists(&db, &meal_product_id).await? {
        return Err(AppError::MealProductNotFound);
    }

    delete_meal_item(&db, &meal_product_id).await?;

    Ok(StatusCode::NO_CONTENT)
}
