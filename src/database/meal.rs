use chrono::NaiveDate;
use sqlx::PgPool;
use uuid::Uuid;

use crate::models::{database::{AddMealProduct, UserMealProduct}, dto::UserMealSectionView};

pub async fn fetch_user_meal_sections(
    db: &PgPool,
    user_id: &Uuid,
) -> sqlx::Result<Vec<UserMealSectionView>> {
    let sections = sqlx::query_as!(
        UserMealSectionView,
        "SELECT id, index, label FROM user_meal_sections WHERE user_id = $1",
        user_id,
    )
    .fetch_all(db)
    .await?;

    Ok(sections)
}

pub async fn fetch_user_meal_section_exists(db: &PgPool, section_id: &Uuid) -> sqlx::Result<bool> {
    let section = sqlx::query!(
        r#"SELECT EXISTS ( SELECT 1 FROM user_meal_sections WHERE id = $1 ) AS "exists!""#,
        section_id,
    )
    .fetch_one(db)
    .await?;

    Ok(section.exists)
}

pub async fn check_meal_item_exists(db: &PgPool, meal_product_id: &Uuid) -> sqlx::Result<bool> {
    let product = sqlx::query!(
        r#"SELECT EXISTS ( SELECT 1 FROM meal_items WHERE meal_product_id = $1 ) AS "exists!""#,
        meal_product_id,
    )
    .fetch_one(db)
    .await?;

    Ok(product.exists)
}

pub async fn fetch_user_meals_products(
    db: &PgPool,
    user_id: Uuid,
    date: NaiveDate,
) -> sqlx::Result<Vec<UserMealProduct>> {
    sqlx::query_as!(
        UserMealProduct,
        r#"
        SELECT
            um.section_id,
            mp.product_id,
            mi.quantity,
            coalesce(mp.name, p.name) AS "name!",
            coalesce(mp.calories, p.calories) AS "calories!",
            coalesce(mp.proteins, p.proteins) AS "proteins!",
            coalesce(mp.fats, p.fats) AS "fats!",
            coalesce(mp.carbohydrates, p.carbohydrates) AS "carbohydrates!"
        FROM meal_products mp
        LEFT JOIN products p ON p.id = mp.product_id
        INNER JOIN meal_items mi ON mi.meal_product_id = mp.id
        INNER JOIN user_meals um ON um.id = mi.meal_id
        WHERE um.date = $1 AND um.user_id = $2
        "#,
        date, user_id,
    )
    .fetch_all(db)
    .await
}

pub async fn add_meal_product(
    db: &PgPool,
    user_id: &Uuid,
    add_product: AddMealProduct,
) -> sqlx::Result<()> {
    // create meal product in database
    let meal_product = sqlx::query!(
        "
        INSERT INTO meal_products (kind, product_id, name, calories, proteins, fats, carbohydrates)
        VALUES ($1, $2, $3, $4, $5, $6, $7)
        RETURNING id
        ",
        add_product.kind as _,
        add_product.product_id,
        add_product.name,
        add_product.calories,
        add_product.proteins,
        add_product.fats,
        add_product.carbohydrates,
    )
    .fetch_one(db)
    .await?;

    // insert row into user_meals with user_meal_sections uuid
    let meal = sqlx::query!(
        "INSERT INTO user_meals (user_id, section_id, date) VALUES ($1, $2, $3) RETURNING id",
        user_id, add_product.section_id, add_product.date,
    )
    .fetch_one(db)
    .await?;

    // insert row into meal_items with user_meals uuid
    sqlx::query!(
        "
        INSERT INTO meal_items (meal_id, meal_product_id, quantity) VALUES ($1, $2, $3)",
        meal.id, meal_product.id, add_product.quantity,
    )
    .execute(db)
    .await?;

    Ok(())
}

pub async fn delete_meal_item(db: &PgPool, meal_product_id: &Uuid) -> sqlx::Result<()> {
    sqlx::query!(
        "DELETE FROM meal_items WHERE meal_product_id = $1",
        meal_product_id,
    )
    .execute(db)
    .await?;

    Ok(())
}
