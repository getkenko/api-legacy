use axum::{extract::{Path, Query, State}, routing::get, Json, Router};

use crate::{database::product::{fetch_products, find_product}, models::{dto::products::{ProductView, SearchProductQuery}, errors::{AppError, AppResult}}};

use super::AppState;

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/", get(search_products))
        .route("/barcode/{barcode}", get(find_product_by_barcode))
}

async fn search_products(
    State(state): State<AppState>,
    Query(search): Query<SearchProductQuery>,
) -> AppResult<Json<Vec<ProductView>>> {
    let products = fetch_products(&state.db, &search.query).await?;
    let products_view = products
        .into_iter()
        .map(|p| ProductView::from(p))
        .collect::<Vec<_>>();
    Ok(Json(products_view))
}

async fn find_product_by_barcode(
    State(state): State<AppState>,
    Path(barcode): Path<i32>,
) -> AppResult<Json<ProductView>> {
    let product = find_product(&state.db, barcode)
        .await?
        .ok_or(AppError::ProductNotFound)?;
    let product_view = ProductView::from(product);

    Ok(Json(product_view))
}
