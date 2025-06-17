use chrono::NaiveDate;
use sqlx::PgPool;
use uuid::Uuid;

use crate::models::database::User;

pub async fn find_user_by_id(db: &PgPool, id: &Uuid) -> sqlx::Result<Option<User>> {
    let user = sqlx::query_as!(
        User,
        r#"
        SELECT
            id, username, display_name, email, password, is_male, date_of_birth, avatar_url, account_state AS "account_state: _", created_at
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
            id, username, display_name, email, password, is_male, date_of_birth, avatar_url, account_state AS "account_state: _", created_at
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
    is_male: bool,
    date_of_birth: NaiveDate,
) -> sqlx::Result<()> {
    sqlx::query!(
        "INSERT INTO users (username, email, password, is_male, date_of_birth) VALUES ($1, $2, $3, $4, $5)",
        username, email, password, is_male, date_of_birth,
    )
    .execute(db)
    .await?;

    Ok(())
}
