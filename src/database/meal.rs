use chrono::NaiveDate;
use sqlx::PgPool;
use uuid::Uuid;

use crate::models::database::MealProductKind;

pub async fn fetch_user_meal_section_exists(db: &PgPool, section_id: &Uuid) -> sqlx::Result<bool> {
    let section = sqlx::query!(
        r#"SELECT EXISTS ( SELECT 1 FROM user_meal_sections WHERE id = $1 ) AS "exists!""#,
        section_id,
    )
    .fetch_one(db)
    .await?;

    Ok(section.exists)
}

// TODO: use struct with constructor for 'quick add' and 'from database'
pub async fn add_meal_product(
    db: &PgPool,
    kind: MealProductKind,
    product_id: Option<Uuid>,
    label: Option<String>,
    calories: Option<i32>,
    proteins: Option<i32>,
    fats: Option<i32>,
    carbohydrates: Option<i32>,
    quantity: i32,
    section_id: &Uuid,
    date: NaiveDate,
) -> sqlx::Result<()> {
    // create meal product in database
    let meal_product = sqlx::query!(
        "
        INSERT INTO meal_products (type, product_id, label, calories, proteins, fats, carbohydrates)
        VALUES ($1, $2, $3, $4, $5, $6, $7) RETURNING id
        ",
        kind as _, product_id, label, calories, proteins, fats, carbohydrates,
    )
    .fetch_one(db)
    .await?;

    // insert row into user_meals with user_meal_sections uuid
    let meal = sqlx::query!(
        "INSERT INTO user_meals (section_id, date) VALUES ($1, $2) RETURNING id",
        section_id, date,
    )
    .fetch_one(db)
    .await?;

    // insert row into meal_items with user_meals uuid
    sqlx::query!("
        INSERT INTO meal_items (meal_id, meal_product_id, quantity) VALUES ($1, $2, $3)",
        meal.id, meal_product.id, quantity,
    )
    .execute(db)
    .await?;

    Ok(())
}
