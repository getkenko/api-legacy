use sqlx::PgPool;

use crate::{database::product_repo, models::{dto::products::ProductView, errors::{AppError, AppResult}}};

pub async fn query_products(db: &PgPool, query: &str) -> AppResult<Vec<ProductView>> {
    let products = product_repo::fetch_products(db, query).await?;
    let products_view = products
        .into_iter()
        .map(|p| ProductView::from(p))
        .collect::<Vec<_>>();

    Ok(products_view)
}

pub async fn get_product_by_barcode(db: &PgPool, barcode: i64) -> AppResult<ProductView> {
    let product = product_repo::find_product(db, barcode)
        .await?
        .ok_or(AppError::ProductNotFound)?;
    let product_view = ProductView::from(product);
    
    Ok(product_view)
}
