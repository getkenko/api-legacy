use sqlx::PgPool;

use crate::models::{dto::RegisterUserData, errors::{AppError, AppResult}};

pub async fn check_user_conflicts(db: &PgPool, user_data: &RegisterUserData) -> AppResult<()> {
    let user_check = sqlx::query!(
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
        user_data.username, user_data.email,
    )
    .fetch_optional(db)
    .await?;

    if let Some(user_check) = user_check {
        if user_check.username_taken {
            return Err(AppError::UsernameTaken);
        } else if user_check.email_taken {
            return Err(AppError::EmailTaken);
        }
    }

    Ok(())
}

pub async fn insert_user(db: &PgPool, user_data: &RegisterUserData) -> Result<(), sqlx::Error> {
    sqlx::query!(
        "INSERT INTO users (username, display_name, email, password, gender, date_of_birth) VALUES ($1, $2, $3, $4, $5, $6)",
        user_data.username, user_data.username, user_data.email, user_data.password, user_data.gender as _, user_data.date_of_birth,
    )
    .execute(db)
    .await?;
    Ok(())
}
