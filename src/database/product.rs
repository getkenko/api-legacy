use sqlx::PgPool;
use uuid::Uuid;

use crate::models::database::Product;

pub async fn fetch_product_by_id(db: &PgPool, id: Uuid) -> sqlx::Result<Product> {
    let product = sqlx::query_as!(
        Product,
        "SELECT id, name, barcode, ingredients, calories, proteins, fats, carbohydrates FROM products WHERE id = $1",
        id,
    )
    .fetch_one(db)
    .await?;

    Ok(product)
}

pub async fn find_product_by_barcode(db: &PgPool, barcode: i32) -> sqlx::Result<Option<Product>> {
    let product = sqlx::query_as!(
        Product,
        "SELECT id, name, barcode, ingredients, calories, proteins, fats, carbohydrates FROM products WHERE barcode = $1 LIMIT 1",
        barcode,
    )
    .fetch_optional(db)
    .await?;

    Ok(product)
}

pub async fn fetch_products_with_query(db: &PgPool, query: &str) -> sqlx::Result<Vec<Product>> {
    // swaglord: i hate this query and i hate sqlx type checking even more
    let products = sqlx::query_as!(
        Product,
        r#"
        WITH
        fts_results AS (
            SELECT *, 1 AS rank_source
            FROM products
            WHERE search_vector @@ plainto_tsquery('english', $1)
            LIMIT 10
        ),
        fzf_results AS (
            SELECT *, 2 AS rank_source
            FROM products
            WHERE (
                similarity(name, $1) > 0.3 OR
                similarity(ingredients, $1) > 0.3
            )
            AND id NOT IN (SELECT id FROM fts_results)
            ORDER BY GREATEST(
                similarity(name, $1),
                similarity(ingredients, $1)
            ) DESC
            LIMIT 10
        )
        SELECT id AS "id!", name AS "name!", barcode AS "barcode!", ingredients AS "ingredients!", calories AS "calories!", proteins AS "proteins!", fats AS "fats!", carbohydrates AS "carbohydrates!" FROM fts_results
        UNION ALL
        SELECT id AS "id!", name AS "name!", barcode AS "barcode!", ingredients AS "ingredients!", calories AS "calories!", proteins AS "proteins!", fats AS "fats!", carbohydrates AS "carbohydrates!" FROM fzf_results
        LIMIT 10
        "#,
        query,
    )
    .fetch_all(db)
    .await?;

    Ok(products)
}
