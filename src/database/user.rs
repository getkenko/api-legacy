use sqlx::PgPool;
use uuid::Uuid;

use crate::{models::{database::AccountState, dto::RegisterUserData, errors::{AppError, AppResult}}, utils::{jwt::AccessToken, password::hash_password}};

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

pub async fn find_user_token_data(db: &PgPool, user_id: &Uuid) -> AppResult<AccessToken> {
    // TODO: do something more than returning unathorized if the user doesn't exist
    // e.g. remove cookies

    let user = sqlx::query!(
        r#"SELECT display_name, account_state AS "account_state: AccountState" FROM users WHERE id = $1"#,
        user_id,
    )
    .fetch_optional(db)
    .await?
    .ok_or(AppError::Unathorized)?;

    if user.account_state != AccountState::Active {
        return Err(AppError::AccountNotActive(user.account_state));
    }

    let token = AccessToken::new(user_id, &user.display_name);
    Ok(token)
}

pub async fn insert_user(db: &PgPool, user_data: &RegisterUserData) -> AppResult<()> {
    let password = hash_password(&user_data.password).map_err(AppError::Crypto)?;

    sqlx::query!(
        "INSERT INTO users (username, display_name, email, password, is_male, date_of_birth) VALUES ($1, $2, $3, $4, $5, $6)",
        user_data.username, user_data.username, user_data.email, password, user_data.is_male, user_data.date_of_birth,
    )
    .execute(db)
    .await?;
    Ok(())
}
