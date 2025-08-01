use sqlx::PgPool;
use uuid::Uuid;

use crate::{database::fridge_repo, models::{database::fridge::FridgeProduct, dto::fridge::{CreateFridgeProductDto, UpdateFridgeProductDto}, errors::AppResult}};

pub async fn add_product(db: &PgPool, user_id: Uuid, product: CreateFridgeProductDto) -> AppResult<FridgeProduct> {
    let product = fridge_repo::insert(db, user_id, product.name, product.quantity, product.expiration).await?;
    Ok(product)
}

pub async fn get_products(db: &PgPool, user_id: Uuid) -> AppResult<Vec<FridgeProduct>> {
    let products = fridge_repo::fetch_all(db, user_id).await?;
    Ok(products)
}

pub async fn update_product(db: &PgPool, user_id: Uuid, product_id: Uuid, update: UpdateFridgeProductDto) -> AppResult<FridgeProduct> {
    let updated_product = fridge_repo::update(db, user_id, product_id, update.name, update.quantity, update.expiration).await?;
    Ok(updated_product)
}

pub async fn delete_product(db: &PgPool, user_id: Uuid, product_id: Uuid) -> AppResult<()> {
    fridge_repo::delete(db, user_id, product_id).await?;
    Ok(())
}