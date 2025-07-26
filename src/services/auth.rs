use chrono::Utc;
use sqlx::PgPool;

use crate::{database::user_repo, models::{database::{enums::AccountState, user::{InsertUser, UserNutrition}}, dto::auth::{CheckAvailabilityQuery, LoginRequest, LoginResponse, RegisterRequest, UserConflictsView}, errors::{AppError, AppResult, ValidationError}}, security::{jwt::Token, password::verify_password}, utils::{nutrition::{calc_base_tdee, calc_target_macros, calculate_bmr, calculate_tdee}, validation::{is_activity_in_range, validate_date_of_birth, validate_email, validate_password, validate_username}}};

pub async fn process_login(db: &PgPool, creds: LoginRequest) -> AppResult<LoginResponse> {
    // try to find the user
    let user = user_repo::find_user_by_email(db, &creds.email)
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
    validate_date_of_birth(user_data.date_of_birth)?;
    if !is_activity_in_range(user_data.idle_activity) {
        return Err(ValidationError::InvalidIdleActivity)?;
    }
    if !is_activity_in_range(user_data.workout_activity) {
        return  Err(ValidationError::InvalidWorkoutActivity)?;
    }

    // check if username and/or email is already used by someone else
    let conflicts = user_repo::fetch_user_conflicts_opt(db, Some(user_data.username.clone()), Some(user_data.email.clone())).await?;
    if conflicts.username_taken.unwrap() {
        return Err(AppError::UsernameTaken);
    } else if conflicts.email_taken.unwrap() {
        return Err(AppError::EmailTaken);
    }

    // insert user into the database
    let insert = InsertUser::try_from(user_data)?;

    // calculate user body metrics
    let age = Utc::now()
        .date_naive()
        .years_since(insert.date_of_birth)
        .unwrap_or(18);

    let bmr = calculate_bmr(insert.weight, insert.height, age, insert.sex);
    let base_tdee = calc_base_tdee(bmr, insert.workout_activity, insert.idle_activity);
    let tdee = calculate_tdee(base_tdee, insert.goal_diff_per_week, insert.weight_goal);
    let macros = calc_target_macros(insert.weight, tdee, insert.weight_goal);

    let nutrition = UserNutrition {
        bmr: bmr,
        base_tdee: base_tdee,
        tdee: tdee,
        protein_target: macros.proteins.round() as _,
        fat_target: macros.fats.round() as _,
        carb_target: macros.carbohydrates.round() as _,
    };

    // insert user to database
    user_repo::insert_user(db, insert, nutrition).await?;

    Ok(())
}

pub async fn check_user_credentials_availability(db: &PgPool, to_check: CheckAvailabilityQuery) -> AppResult<UserConflictsView> {
    if to_check.email.is_none() && to_check.username.is_none() {
        return Err(ValidationError::NoFieldsProvided)?;
    }

    let conflicts = user_repo::fetch_user_conflicts_opt(db, to_check.username, to_check.email).await?;
    let view = UserConflictsView::from(conflicts);
    Ok(view)
}
