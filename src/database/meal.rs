use chrono::NaiveDate;
use sqlx::PgPool;
use uuid::Uuid;

use crate::models::database::meal::{InsertMealProduct, UserMealProduct};

pub async fn check_meal_product_exists(db: &PgPool, product_id: Uuid) -> sqlx::Result<bool> {
    let meal_product = sqlx::query!(
        r#"SELECT EXISTS ( SELECT 1 FROM meal_products WHERE id = $1 ) AS "exists!""#,
        product_id,
    )
    .fetch_one(db)
    .await?;

    Ok(meal_product.exists)
}

pub async fn fetch_user_meal_product_count(
    db: &PgPool,
    user_id: Uuid,
    date: NaiveDate,
) -> sqlx::Result<i64> {
    let products = sqlx::query!(
        r#"
        SELECT count(*) AS "count!"
        FROM meal_products p
        INNER JOIN user_meals meals ON meals.id = p.meal_id
        WHERE meals.user_id = $1 AND meals.date = $2
        "#,
        user_id, date,
    )
    .fetch_one(db)
    .await?;

    Ok(products.count)
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
            mp.id,
            um.section_id,
            mp.product_id,
            mp.quantity,
            coalesce(mp.name, p.name) AS "name!",
            coalesce(mp.calories, p.calories) AS "calories!",
            coalesce(mp.proteins, p.proteins) AS "proteins!",
            coalesce(mp.fats, p.fats) AS "fats!",
            coalesce(mp.carbohydrates, p.carbohydrates) AS "carbohydrates!"
        FROM meal_products mp
        LEFT JOIN products p ON p.id = mp.product_id
        INNER JOIN user_meals um ON um.id = mp.meal_id
        WHERE um.date = $1 AND um.user_id = $2
        "#,
        date, user_id,
    )
    .fetch_all(db)
    .await
}

pub async fn insert_meal_product(
    db: &PgPool,
    user_id: Uuid,
    date: NaiveDate,
    product: InsertMealProduct,
) -> sqlx::Result<UserMealProduct> {
    let mut tx = db.begin().await?;

    // insert row into user_meals with user_meal_sections uuid
    let meal_id = sqlx::query!(
        "INSERT INTO user_meals (user_id, section_id, date) VALUES ($1, $2, $3) RETURNING id",
        user_id, product.section_id, date,
    )
    .fetch_one(&mut *tx)
    .await?
    .id;

    // create meal product in database
    let meal_product_id = sqlx::query!(
        "
        INSERT INTO meal_products (meal_id, kind, quantity, product_id, name, calories, proteins, fats, carbohydrates)
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
        RETURNING id
        ",
        meal_id,
        product.kind as _,
        product.quantity,
        product.product_id,
        product.name,
        product.calories,
        product.proteins,
        product.fats,
        product.carbohydrates,
    )
    .fetch_one(&mut *tx)
    .await?
    .id;

    // select details for response
    let new_product = sqlx::query!(
        r#"
        SELECT
            coalesce(mp.name, p.name) AS "name!", coalesce(mp.calories, p.calories) AS "calories!",
            coalesce(mp.proteins, p.proteins) AS "proteins!", coalesce(mp.fats, p.fats) AS "fats!",
            coalesce(mp.carbohydrates, p.carbohydrates) AS "carbohydrates!"
        FROM meal_products mp
        LEFT JOIN products p ON p.id = mp.product_id
        WHERE mp.id = $1
        "#,
        meal_product_id,
    )
    .fetch_one(&mut *tx)
    .await?;

    tx.commit().await?;

    Ok(UserMealProduct {
        id: meal_product_id,
        section_id: product.section_id,
        product_id: product.product_id,
        quantity: product.quantity,
        name: new_product.name,
        calories: new_product.calories,
        proteins: new_product.proteins,
        fats: new_product.fats,
        carbohydrates: new_product.carbohydrates,
    })
}

pub async fn delete_meal_product(db: &PgPool, product_id: Uuid) -> sqlx::Result<()> {
    sqlx::query!("DELETE FROM meal_products WHERE id = $1", product_id)
        .execute(db)
        .await?;

    Ok(())
}

pub async fn delete_meal_products_for_date(db: &PgPool, date: NaiveDate, section_id: Option<Uuid>) -> sqlx::Result<()> {
    if let Some(section_id) = section_id {
        sqlx::query!(
            "
            DELETE FROM meal_products
            WHERE id IN (
                SELECT prod.id
                FROM user_meals meal
                INNER JOIN meal_products prod ON prod.meal_id = meal.id
                WHERE meal.date = $1 AND meal.section_id = $2
            )
            ",
            date, section_id,
        )
    } else {
        sqlx::query!(
            "
            DELETE FROM meal_products
            WHERE id IN (
                SELECT prod.id
                FROM user_meals meal
                INNER JOIN meal_products prod ON prod.meal_id = meal.id
                WHERE meal.date = $1
            )
            ",
            date,
        )
    }
    .execute(db)
    .await?;

    Ok(())
}
