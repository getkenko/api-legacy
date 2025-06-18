use sqlx::PgPool;

use crate::{database::user::{find_user_by_email, find_user_conflicts, insert_user}, models::{database::AccountState, dto::{LoginCredentials, LoginResponse, RegisterUserData}, errors::{AppError, AppResult}}, utils::{jwt::{AccessToken, AuthToken, RefreshToken}, password::{hash_password, verify_password}, validation::{validate_email, validate_password, validate_username}}};

pub async fn process_login(db: &PgPool, creds: LoginCredentials) -> AppResult<LoginResponse> {
    // try to find the user
    let user = find_user_by_email(db, &creds.email)
        .await?
        .ok_or(AppError::InvalidCredentials)?;

    // check if password matches
    if !verify_password(&creds.password, &user.password).map_err(AppError::Crypto)? {
        return Err(AppError::InvalidCredentials);
    }

    if user.account_state != AccountState::Active {
        return Err(AppError::AccountNotActive(user.account_state));
    }

    // create auth tokens
    let access = AccessToken::new(&user.id, &user.display_name);
    let refresh = RefreshToken::new(&user.id);

    let tokens = LoginResponse {
        access: access.encode()?,
        refresh: refresh.encode()?,
    };

    Ok(tokens)
}

pub async fn process_register(db: &PgPool, user_data: RegisterUserData) -> AppResult<()> {
    validate_username(&user_data.username)?;
    validate_email(&user_data.email)?;
    validate_password(&user_data.password)?;

    let conflicts = find_user_conflicts(db, &user_data.username, &user_data.email).await?;
    if conflicts.username_taken {
        return Err(AppError::UsernameTaken);
    } else if conflicts.email_taken {
        return Err(AppError::EmailTaken);
    }

    let password = hash_password(&user_data.password).map_err(AppError::Crypto)?;
    insert_user(db, &user_data.username, &user_data.email, &password).await?;

    // TODO: insert birth date and is_male

    Ok(())
}
