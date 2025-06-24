use chrono::NaiveDate;
use sqlx::PgPool;
use uuid::Uuid;

use crate::models::{database::{AddMealProduct, MealProduct}, dto::UserMealSectionView};

pub async fn fetch_user_meal_sections(db: &PgPool, user_id: &Uuid) -> sqlx::Result<Vec<UserMealSectionView>> {
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

pub async fn fetch_meal_product(db: &PgPool, id: &Uuid) -> sqlx::Result<MealProduct> {
    let product = sqlx::query_as!(
        MealProduct,
        r#"SELECT id, kind AS "kind: _", product_id, name, calories, proteins, fats, carbohydrates FROM meal_products WHERE id = $1"#,
        id,
    )
    .fetch_one(db)
    .await?;

    Ok(product)
}

// fetches all meal products for user's date, returning vector of pair of meal product with quantity
pub async fn fetch_user_meal_products_for_date(
    db: &PgPool,
    user_id: &Uuid,
    date: NaiveDate,
) -> sqlx::Result<Vec<(MealProduct, i32)>> {
    let mut products = vec![];

    // fetch user_meals for this date
    let meal_ids = sqlx::query!(
        "SELECT id FROM user_meals WHERE user_id = $1 AND date = $2",
        user_id, date,
    )
    .fetch_all(db)
    .await?
    .into_iter()
    .map(|m| m.id)
    .collect::<Vec<_>>();

    // fetch all meal_items for this user_meals id
    let meal_items = sqlx::query!(
        "SELECT meal_product_id, quantity FROM meal_items WHERE meal_id IN (SELECT unnest($1::UUID[]))",
        &meal_ids,
    )
    .fetch_all(db)
    .await?;

    // fetch all meal_products for this meal_items product ids
    for meal_item in meal_items {
        let meal_product = fetch_meal_product(db, &meal_item.meal_product_id).await?;
        products.push((meal_product, meal_item.quantity));
    }

    Ok(products)
}

pub async fn add_meal_product(db: &PgPool, user_id: &Uuid, add_product: AddMealProduct) -> sqlx::Result<()> {
    // create meal product in database
    let meal_product = sqlx::query!(
        "
        INSERT INTO meal_products (kind, product_id, name, calories, proteins, fats, carbohydrates)
        VALUES ($1, $2, $3, $4, $5, $6, $7)
        RETURNING id
        ",
        add_product.kind as _, add_product.product_id, add_product.name, add_product.calories,
        add_product.proteins, add_product.fats, add_product.carbohydrates,
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
    sqlx::query!("
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
