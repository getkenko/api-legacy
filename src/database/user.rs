use chrono::NaiveDate;
use sqlx::PgPool;
use uuid::Uuid;

use crate::models::database::{FullUser, MeasurementSystem, User, DietKind, UserOrigin, WeightGoal};

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

pub async fn fetch_full_user(db: &PgPool, id: &Uuid) -> sqlx::Result<FullUser> {
    let info = sqlx::query_as!(
        FullUser,
        r#"
        SELECT
            u.id,
            u.username,
            u.display_name,
            u.email,
            u.password,
            u.avatar_url,
            u.account_state AS "account_state: _",
            u.created_at,
            ud.is_male,
            ud.weight,
            ud.height,
            ud.date_of_birth,
            ud.idle_activity,
            ud.workout_activity,
            ud.diet_kind AS "diet_kind: _",
            up.theme AS "theme: _",
            up.language AS "language: _",
            up.measurement_system AS "measurement_system: _",
            ug.weight_goal AS "weight_goal: _",
            ug.goal_diff_per_week
        FROM users u
        INNER JOIN user_details ud ON u.id = ud.user_id
        INNER JOIN user_preferences up ON u.id = up.user_id
        INNER JOIN user_goals ug ON u.id = ug.user_id
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

pub async fn insert_user_data(
    db: &PgPool,
    username: &str,
    display_name: &str,
    email: &str,
    password: &str,
    is_male: bool,
    weight: f32,
    height: i32,
    date_of_birth: NaiveDate,
    idle_activity: i32,
    workout_activity: i32,
    measurement_system: MeasurementSystem,
    weight_goal: WeightGoal,
    goal_diff_per_week: f32,
    diet_kind: DietKind,
    user_origin: UserOrigin,
) -> sqlx::Result<()> {
    let mut tx = db.begin().await?;

    // insert user
    let user = sqlx::query!(
        "
        INSERT INTO users (username, display_name, email, password)
        VALUES ($1, $2, $3, $4)
        RETURNING id
        ",
        username, display_name, email, password,
    )
    .fetch_one(&mut *tx)
    .await?;

    // insert details
    sqlx::query!(
        "
        INSERT INTO user_details (user_id, is_male, weight, height, date_of_birth, idle_activity, workout_activity, diet_kind)
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
        ",
        user.id, is_male, weight, height, date_of_birth, idle_activity, workout_activity, diet_kind as _,
    )
    .execute(&mut *tx)
    .await?;

    // create preferences
    sqlx::query!(
        "INSERT INTO user_preferences (user_id, measurement_system) VALUES ($1, $2)",
        user.id, measurement_system as _,
    )
    .execute(&mut *tx)
    .await?;

    // setup meal sections
    sqlx::query!(
        "
        INSERT INTO user_meal_sections (user_id, index, label) VALUES
        ($1, $2, $3), ($1, $4, $5), ($1, $6, $7)
        ",
        user.id,
        0, "Breakfast",
        1, "Launch",
        2, "Dinner",
    )
    .execute(&mut *tx)
    .await?;

    // insert values into user goals
    sqlx::query!(
        "INSERT INTO user_goals (user_id, weight_goal, goal_diff_per_week) VALUES ($1, $2, $3)",
        user.id, weight_goal as _, goal_diff_per_week,
    )
    .execute(&mut *tx)
    .await?;

    // create metrics table
    sqlx::query!(
        "INSERT INTO user_metrics (user_id, origin) VALUES ($1, $2)",
        user.id, user_origin as _,
    )
    .execute(&mut *tx)
    .await?;

    tx.commit().await?;

    Ok(())
}

pub async fn update_user_avatar(db: &PgPool, user_id: Uuid, avatar: Option<String>) -> sqlx::Result<()> {
    sqlx::query!(
        "UPDATE users SET avatar_url = $1 WHERE id = $2",
        avatar, user_id,
    )
    .execute(db)
    .await?;

    Ok(())
}
