use chrono::NaiveDate;
use sqlx::PgPool;
use uuid::Uuid;

use crate::models::{database::User, dto::UserInfo};

pub async fn find_user_by_id(db: &PgPool, id: &Uuid) -> sqlx::Result<Option<User>> {
    let user = sqlx::query_as!(
        User,
        r#"
        SELECT
            id, username, display_name, email, password, avatar_url, account_state AS "account_state: _", created_at
        FROM
            users
        WHERE
            id = $1
        LIMIT 1
        "#,
        id,
    )
    .fetch_optional(db)
    .await?;

    Ok(user)
}

pub async fn find_user_by_email(db: &PgPool, email: &str) -> sqlx::Result<Option<User>> {
    let user = sqlx::query_as!(
        User,
        r#"
        SELECT
            id, username, display_name, email, password, avatar_url, account_state AS "account_state: _", created_at
        FROM
            users
        WHERE
            email = $1
        LIMIT 1
        "#,
        email,
    )
    .fetch_optional(db)
    .await?;

    Ok(user)
}

pub async fn fetch_user_info(db: &PgPool, id: &Uuid) -> sqlx::Result<UserInfo> {
    let info = sqlx::query_as!(
        UserInfo,
        r#"
        SELECT
            u.username,
            u.display_name,
            u.email,
            u.avatar_url,
            u.created_at,
            ud.is_male,
            ud.weight,
            ud.height,
            ud.date_of_birth,
            up.theme AS "theme: _",
            up.language AS "language: _"
        FROM users u
        INNER JOIN user_details ud ON u.id = ud.user_id
        INNER JOIN user_preferences up ON u.id = up.user_id
        WHERE u.id = $1
        "#,
        id,
    )
    .fetch_one(db)
    .await?;

    Ok(info)
}

#[derive(Default)]
pub struct UserConflicts {
    pub username_taken: bool,
    pub email_taken: bool,
}

pub async fn find_user_conflicts(db: &PgPool, username: &str, email: &str) -> sqlx::Result<UserConflicts> {
    let conflicts = sqlx::query_as!(
        UserConflicts,
        r#"
        SELECT
            username = $1 AS "username_taken!",
            email = $2 AS "email_taken!"
        FROM
            users
        WHERE
            username = $1 OR
            email = $2
        LIMIT 1
        "#,
        username, email,
    )
    .fetch_optional(db)
    .await?;

    Ok(conflicts.unwrap_or(UserConflicts::default()))
}

pub async fn insert_user(
    db: &PgPool,
    username: &str,
    email: &str,
    password: &str,
) -> sqlx::Result<()> {
    sqlx::query!(
        "INSERT INTO users (username, email, password) VALUES ($1, $2, $3)",
        username, email, password,
    )
    .execute(db)
    .await?;

    Ok(())
}
