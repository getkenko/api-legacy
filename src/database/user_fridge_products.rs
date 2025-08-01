use chrono::NaiveDate;
use sqlx::PgPool;
use uuid::Uuid;

use crate::models::database::fridge::FridgeProduct;

pub async fn insert(
    db: &PgPool,
    user_id: Uuid,
    name: String,
    quantity: i32,
    expiration: Option<NaiveDate>,
) -> sqlx::Result<FridgeProduct> {
    sqlx::query_as!(
        FridgeProduct,
        "
        INSERT INTO user_fridge_products (user_id, name, quantity, expiration)
        VALUES ($1, $2, $3, $4)
        RETURNING *
        ",
        user_id, name, quantity, expiration,
    )
    .fetch_one(db)
    .await
}

pub async fn fetch_all(db: &PgPool, user_id: Uuid) -> sqlx::Result<Vec<FridgeProduct>> {
    sqlx::query_as!(
        FridgeProduct,
        "SELECT * FROM user_fridge_products WHERE user_id = $1",
        user_id,
    )
    .fetch_all(db)
    .await
}

pub async fn update(
    db: &PgPool,
    user_id: Uuid,
    product_id: Uuid,
    name: Option<String>,
    quantity: Option<i32>,
    expiration: Option<NaiveDate>,
) -> sqlx::Result<FridgeProduct> {
    sqlx::query_as!(
        FridgeProduct,
        "
        UPDATE user_fridge_products
        SET name = COALESCE($3, name), quantity = COALESCE($4, quantity), expiration = COALESCE($5, expiration)
        WHERE user_id = $1 AND id = $2
        RETURNING *
        ",
        user_id, product_id,
        name, quantity, expiration,
    )
    .fetch_one(db)
    .await
}

pub async fn delete(db: &PgPool, user_id: Uuid, product_id: Uuid) -> sqlx::Result<()> {
    sqlx::query!(
        "DELETE FROM user_fridge_products WHERE user_id = $1 AND id = $2",
        user_id, product_id,
    )
    .execute(db)
    .await?;

    Ok(())
}