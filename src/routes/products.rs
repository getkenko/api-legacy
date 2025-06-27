use axum::{extract::{Path, Query, State}, routing::get, Json, Router};

use crate::{models::{dto::products::{ProductView, SearchProductQuery}, errors::AppResult}, services::products::{get_product_by_barcode, query_products}};

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
    let products = query_products(&state.db, &search.query).await?;
    Ok(Json(products))
}

async fn find_product_by_barcode(
    State(state): State<AppState>,
    Path(barcode): Path<i32>,
) -> AppResult<Json<ProductView>> {
    let product = get_product_by_barcode(&state.db, barcode).await?;
    Ok(Json(product))
}
