use axum::{extract::{Path, State}, http::StatusCode, routing::{delete, post}, Json, Router};
use uuid::Uuid;

use crate::{database::meal::{add_meal_product, fetch_user_meal_section_exists}, models::{database::MealProductKind, dto::{AddProduct, QuickAddProduct}, errors::{AppError, AppResult}}};

use super::AppState;

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/products", post(add_product))
        .route("/products/quick", post(quick_add_product)) // should we instead use query parameter in /products?
        .route("/products/{product_id}", delete(delete_meal_product))
}

async fn add_product(
    State(db): State<AppState>,
    Json(product): Json<AddProduct>,
) -> AppResult<StatusCode> {
    let section_exists = fetch_user_meal_section_exists(&db, &product.section_id).await?;
    if !section_exists {
        return Err(AppError::MealSectionNotFound);
    }

    add_meal_product(&db, MealProductKind::FromDatabase, Some(product.product_id), None, None, None, None, None, product.quantity, &product.section_id, product.date).await?;
    Ok(StatusCode::CREATED)
}

async fn quick_add_product(
    State(db): State<AppState>,
    Json(product): Json<QuickAddProduct>,
) -> AppResult<StatusCode> {
    let section_exists = fetch_user_meal_section_exists(&db, &product.section_id).await?;
    if !section_exists {
        return Err(AppError::MealSectionNotFound);
    }

    add_meal_product(&db, MealProductKind::QuickAdd, None, Some(product.label), Some(product.calories), Some(product.proteins), Some(product.fats), Some(product.carbohydrates), product.quantity, &product.section_id, product.date).await?;
    Ok(StatusCode::CREATED)
}

async fn delete_meal_product(
    State(db): State<AppState>,
    Path(product_id): Path<Uuid>,
) -> AppResult<StatusCode> {
    // find meal product
    let product = sqlx::query!(r#"SELECT EXISTS ( SELECT 1 FROM meal_items WHERE meal_product_id = $1 ) AS "exists!""#, product_id)
        .fetch_one(&db)
        .await?;

    if !product.exists {
        return Err(AppError::MealProductNotFound);
    }

    // delete it
    sqlx::query!("DELETE FROM meal_items WHERE meal_product_id = $1", product_id)
        .execute(&db)
        .await?;

    Ok(StatusCode::NO_CONTENT)
}
