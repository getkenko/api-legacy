use sqlx::{PgExecutor, PgPool, Postgres, QueryBuilder};
use uuid::Uuid;

use crate::models::database::section::{SectionIcon, UserSection};

pub async fn check_meal_section_exists(db: &PgPool, user_id: Uuid, section_id: Uuid) -> sqlx::Result<bool> {
    let section = sqlx::query!(
        r#"SELECT EXISTS ( SELECT 1 FROM user_sections WHERE id = $1 AND user_id = $2 ) AS "exists!""#,
        section_id, user_id,
    )
    .fetch_one(db)
    .await?;

    Ok(section.exists)
}

pub async fn check_icon_exists(db: &PgPool, icon_id: i32) -> sqlx::Result<bool> {
    let icon = sqlx::query!(
        r#"SELECT EXISTS ( SELECT 1 FROM section_icons WHERE id = $1 ) AS "exists!""#,
        icon_id,
    )
    .fetch_one(db)
    .await?;

    Ok(icon.exists)
}

pub async fn fetch_available_icons(db: &PgPool) -> sqlx::Result<Vec<SectionIcon>> {
    sqlx::query_as!(
        SectionIcon,
        "SELECT * FROM section_icons",
    )
    .fetch_all(db)
    .await
}

pub async fn find_meal_section(db: impl PgExecutor<'_>, user_id: Uuid, section_id: Uuid) -> sqlx::Result<Option<UserSection>> {
    sqlx::query_as!(
        UserSection,
        r#"
        SELECT section.id, section.user_id, section.index, icon.emoji AS "icon: Option<String>", section.name
        FROM user_sections section
        LEFT JOIN section_icons icon ON icon.id = section.icon_id
        WHERE section.user_id = $1 AND section.id = $2
        LIMIT 1
        "#,
        user_id, section_id,
    )
    .fetch_optional(db)
    .await
}

pub async fn fetch_meal_sections(db: &PgPool, user_id: Uuid) -> sqlx::Result<Vec<UserSection>> {
    sqlx::query_as!(
        UserSection,
        r#"
        SELECT section.id, section.user_id, section.index, icon.emoji AS "icon: Option<String>", section.name
        FROM user_sections section
        LEFT JOIN section_icons icon ON icon.id = section.icon_id
        WHERE section.user_id = $1
        ORDER BY section.index
        "#,
        user_id,
    )
    .fetch_all(db)
    .await
}

pub async fn find_last_section_index(
    db: impl PgExecutor<'_>,
    user_id: Uuid,
    exclude_id: Option<Uuid>,
) -> sqlx::Result<Option<i32>> {
    let section = sqlx::query!(
        "SELECT index FROM user_sections WHERE user_id = $1 AND id != $2 ORDER BY index DESC LIMIT 1",
        user_id, exclude_id.unwrap_or(Uuid::new_v4()),
    )
    .fetch_optional(db)
    .await?;

    Ok(section.map(|s| s.index))
}

pub async fn insert_meal_section(
    db: &PgPool,
    user_id: Uuid,
    index: i32,
    name: String,
) -> sqlx::Result<UserSection> {
    sqlx::query_as!(
        UserSection,
        "
        WITH inserted AS (
            INSERT INTO user_sections (user_id, index, name)
            VALUES ($1, $2, $3)
            RETURNING *
        )
        SELECT section.id, section.user_id, section.index, icon.emoji AS icon, section.name
        FROM inserted section
        LEFT JOIN section_icons icon ON icon.id = section.icon_id
        ",
        user_id, index, name,
    )
    .fetch_one(db)
    .await
}

pub async fn update_meal_section(
    db: impl PgExecutor<'_>,
    user_id: Uuid,
    section_id: Uuid,
    index: Option<i32>,
    name: Option<String>,
    icon_id: Option<i32>,
) -> sqlx::Result<UserSection> {
    let mut builder = QueryBuilder::new("WITH updated AS (");

    // 'WITH' block construction
    builder.push("UPDATE user_sections SET ");
    let mut separated = builder.separated(",");

    if let Some(index) = index {
        separated.push("index=");
        separated.push_bind_unseparated(index);
    }

    if let Some(name) = name {
        separated.push("name=");
        separated.push_bind_unseparated(name);
    }

    if let Some(icon) = icon_id {
        separated.push("icon_id=");
        separated.push_bind_unseparated(icon);
    }

    builder
        .push(" WHERE id=").push_bind(section_id)
        .push(" AND user_id=").push_bind(user_id);

    // finish 'WITH' block
    builder.push(" RETURNING *) ");

    // combine result with icon join
    builder.push("SELECT section.id, section.user_id, section.index, icon.emoji AS icon, section.name ");
    builder.push("FROM updated section ");
    builder.push("LEFT JOIN section_icons icon ON icon.id = section.icon_id");

    builder
        .build_query_as::<UserSection>()
        .fetch_one(db)
        .await
}

pub async fn update_section_indices(
    db: impl PgExecutor<'_>,
    user_id: Uuid,
    increase_index: bool,
    source_index: i32,
    target_index: i32,
) -> sqlx::Result<()> {
    let mut builder = QueryBuilder::<Postgres>::new("UPDATE user_sections SET index = ");

    builder
        .push(if increase_index { "index + 1" } else { "index - 1" })
        .push(" WHERE user_id = ").push_bind(user_id).push(" AND index ");

    builder
        .push(if increase_index { ">= " } else { "<= " })
        .push_bind(target_index)
        .push(" AND index ")
        .push(if increase_index { "< "} else { "> "})
        .push_bind(source_index);

    builder
        .build()
        .execute(db)
        .await?;

    Ok(())
}

pub async fn delete_meal_section(db: impl PgExecutor<'_>, user_id: Uuid, section_id: Uuid) -> sqlx::Result<i32> {
    let section = sqlx::query!(
        "DELETE FROM user_sections WHERE id = $1 AND user_id = $2 RETURNING index",
        section_id, user_id,
    )
    .fetch_one(db)
    .await?;

    Ok(section.index)
}

pub async fn reset_meal_sections(db: &PgPool, user_id: Uuid) -> sqlx::Result<Vec<UserSection>> {
    let mut tx = db.begin().await?;

    sqlx::query!("DELETE FROM user_sections WHERE user_id = $1", user_id)
        .execute(&mut *tx)
        .await?;

    let sections = sqlx::query_as!(
        UserSection,
        "
        WITH inserted AS (
            INSERT INTO user_sections (user_id, index, name)
            VALUES ($1, $2, $3), ($1, $4, $5), ($1, $6, $7), ($1, $8, $9)
            RETURNING *
        )
        SELECT section.id, section.user_id, section.index, icon.emoji AS icon, section.name
        FROM inserted section
        LEFT JOIN section_icons icon ON icon.id = section.icon_id
        ",
        user_id,
        0, "Breakfast",
        1, "Lunch",
        2, "Dinner",
        3, "Snacks",
    )
    .fetch_all(&mut *tx)
    .await?;

    tx.commit().await?;

    Ok(sections)
}
