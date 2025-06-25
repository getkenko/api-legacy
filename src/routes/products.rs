use axum::{extract::{Path, Query, State}, routing::get, Json, Router};

use crate::{database::product::{fetch_products_with_query, find_product_by_barcode}, models::{database::Product, dto::SearchProduct, errors::{AppError, AppResult}}};

use super::AppState;

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/", get(search_products))
        .route("/barcode/{barcode}", get(find_product))
}

async fn search_products(
    State(state): State<AppState>,
    Query(search): Query<SearchProduct>,
) -> AppResult<Json<Vec<Product>>> {
    let products = fetch_products_with_query(&state.db, &search.query).await?;
    Ok(Json(products))
}

async fn find_product(
    State(state): State<AppState>,
    Path(barcode): Path<i32>,
) -> AppResult<Json<Product>> {
    let product = find_product_by_barcode(&state.db, barcode)
        .await?
        .ok_or(AppError::ProductNotFound)?;

    Ok(Json(product))
}
