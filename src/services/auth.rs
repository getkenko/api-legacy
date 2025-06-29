use sqlx::PgPool;

use crate::{database::user::{fetch_user_conflicts, find_user, insert_user}, models::{database::{enums::AccountState, user::InsertUser}, dto::auth::{LoginRequest, LoginResponse, RegisterRequest}, errors::{AppError, AppResult}}, security::{jwt::Token, password::verify_password}, utils::validation::{validate_email, validate_password, validate_username}};

pub async fn process_login(db: &PgPool, creds: LoginRequest) -> AppResult<LoginResponse> {
    // try to find the user
    let user = find_user(db, &creds.email)
        .await?
        .ok_or(AppError::InvalidCredentials)?;

    // check if password matches
    if !verify_password(&creds.password, &user.password).map_err(AppError::Crypto)? {
        return Err(AppError::InvalidCredentials);
    }

    if user.account_state != AccountState::Active {
        return Err(AppError::AccountNotActive(user.account_state));
    }

    // create auth token
    let token = Token::new(user.id, user.display_name, user.email).encode()?;

    Ok(LoginResponse { token })
}

pub async fn process_register(db: &PgPool, user_data: RegisterRequest) -> AppResult<()> {
    validate_username(&user_data.username)?;
    validate_email(&user_data.email)?;
    validate_password(&user_data.password)?;

    // check if idle activity and workout activity are in the range (1-5)
    if user_data.idle_activity < 1 || user_data.idle_activity > 5 {
        return Err(AppError::ActivityNotInRange("Idle".to_string()));
    } else if user_data.workout_activity < 1 || user_data.workout_activity > 5 {
        return Err(AppError::ActivityNotInRange("Workout".to_string()));
    }

    // TODO: validate other fields like weight/height, date of birth

    // check if username and/or email is already used by someone else
    let conflicts = fetch_user_conflicts(db, &user_data.username, &user_data.email).await?;
    if conflicts.username_taken {
        return Err(AppError::UsernameTaken);
    } else if conflicts.email_taken {
        return Err(AppError::EmailTaken);
    }

    // insert user into the database
    let insert = InsertUser::try_from(user_data)?;
    insert_user(db, insert).await?;

    Ok(())
}
