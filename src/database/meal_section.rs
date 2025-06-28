use sqlx::{PgPool, Postgres, QueryBuilder};
use uuid::Uuid;

use crate::models::{database::meal_section::UserMealSection};

pub async fn check_meal_section_exists(db: &PgPool, user_id: Uuid, section_id: Uuid) -> sqlx::Result<bool> {
    let section = sqlx::query!(
        r#"SELECT EXISTS ( SELECT 1 FROM user_meal_sections WHERE id = $1 AND user_id = $2 ) AS "exists!""#,
        section_id, user_id,
    )
    .fetch_one(db)
    .await?;

    Ok(section.exists)
}

pub async fn fetch_user_section_count(db: &PgPool, user_id: Uuid) -> sqlx::Result<i64> {
    let res = sqlx::query!(
        r#"SELECT count(*) AS "count!" FROM user_meal_sections WHERE user_id = $1"#,
        user_id,
    )
    .fetch_one(db)
    .await?;

    Ok(res.count)
}

pub async fn fetch_meal_sections(db: &PgPool, user_id: Uuid) -> sqlx::Result<Vec<UserMealSection>> {
    sqlx::query_as!(
        UserMealSection,
        "SELECT * FROM user_meal_sections WHERE user_id = $1",
        user_id,
    )
    .fetch_all(db)
    .await
}

pub async fn insert_meal_section(
    db: &PgPool,
    user_id: Uuid,
    index: i32,
    label: String,
) -> sqlx::Result<UserMealSection> {
    sqlx::query_as!(
        UserMealSection,
        "INSERT INTO user_meal_sections (user_id, index, label) VALUES ($1, $2, $3) RETURNING *",
        user_id, index, label,
    )
    .fetch_one(db)
    .await
}

pub async fn update_meal_section(
    db: &PgPool,
    user_id: Uuid,
    section_id: Uuid,
    index: Option<i32>,
    label: Option<String>,
) -> sqlx::Result<UserMealSection> {
    let mut builder = QueryBuilder::<Postgres>::new("UPDATE user_meal_sections SET ");
    let mut separated = builder.separated(", ");

    if let Some(index) = index {
        separated.push("index = ");
        separated.push_bind_unseparated(index);
    }

    if let Some(label) = label {
        separated.push("label = ");
        separated.push_bind_unseparated(label);
    }

    builder
        .push(" WHERE id = ")
        .push_bind(section_id)
        .push(" AND user_id = ")
        .push_bind(user_id)
        .push(" RETURNING *");

    builder
        .build_query_as::<UserMealSection>()
        .fetch_one(db)
        .await
}

pub async fn delete_meal_section(db: &PgPool, user_id: Uuid, section_id: Uuid) -> sqlx::Result<()> {
    sqlx::query!(
        "DELETE FROM user_meal_sections WHERE id = $1 AND user_id = $2",
        user_id, section_id,
    )
    .execute(db)
    .await?;

    Ok(())
}

pub async fn reset_meal_sections(db: &PgPool, user_id: Uuid) -> sqlx::Result<Vec<UserMealSection>> {
    let mut tx = db.begin().await?;

    sqlx::query!("DELETE FROM user_meal_sections WHERE user_id = $1", user_id)
        .execute(&mut *tx)
        .await?;

    let sections = sqlx::query_as!(
        UserMealSection,
        "
        INSERT INTO user_meal_sections (user_id, index, label)
        VALUES ($1, $2, $3), ($1, $4, $5), ($1, $6, $7)
        RETURNING *
        ",
        user_id,
        0, "Breakfast",
        1, "Lunch",
        2, "Dinner",
    )
    .fetch_all(&mut *tx)
    .await?;

    tx.commit().await?;

    Ok(sections)
}
