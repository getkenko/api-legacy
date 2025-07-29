use sqlx::{PgExecutor, PgPool, Postgres, QueryBuilder};
use uuid::Uuid;

use crate::models::database::{enums::{Language, Theme}, user::{FullUser, InsertUser, User, UserConflicts, UserDetailsWithGoalsAndPreferences, UserNutrition}};

const DEFAULT_PROTEIN_DIST: i32 = 25;
const DEFAULT_FAT_DIST: i32 = 25;
const DEFAULT_CARB_DIST: i32 = 50;

pub async fn check_user_exists(db: &PgPool, user_id: Uuid) -> sqlx::Result<bool> {
    let user = sqlx::query!(
        r#"SELECT EXISTS ( SELECT 1 FROM users WHERE id = $1 ) AS "exists!""#,
        user_id,
    )
    .fetch_one(db)
    .await?;

    Ok(user.exists)
}

pub async fn find_user(db: &PgPool, user_id: Uuid) -> sqlx::Result<Option<User>> {
    sqlx::query_as!(
        User,
        r#"
        SELECT id, username, display_name, email, password, avatar_url, account_state AS "account_state: _", created_at
        FROM users
        WHERE id = $1
        LIMIT 1
        "#,
        user_id,
    )
    .fetch_optional(db)
    .await
}

pub async fn find_user_by_email(db: &PgPool, email: &str) -> sqlx::Result<Option<User>> {
    sqlx::query_as!(
        User,
        r#"
        SELECT id, username, display_name, email, password, avatar_url, account_state AS "account_state: _", created_at
        FROM users
        WHERE email = $1
        LIMIT 1
        "#,
        email,
    )
    .fetch_optional(db)
    .await
}

pub async fn fetch_full_user(db: &PgPool, id: Uuid) -> sqlx::Result<FullUser> {
    sqlx::query_as!(
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
            ud.sex AS "sex: _",
            ud.weight,
            ud.height,
            ud.date_of_birth,
            ud.idle_activity,
            ud.workout_activity,
            ud.diet_kind AS "diet_kind: _",
            up.theme AS "theme: _",
            up.language AS "language: _",
            up.weight_unit AS "weight_unit: _",
            up.height_unit AS "height_unit: _",
            ug.weight_goal AS "weight_goal: _",
            ug.goal_diff_per_week,
            un.bmr,
            un.base_tdee,
            un.tdee,
            un.protein_target,
            un.fat_target,
            un.carb_target,
            un.protein_dist,
            un.fat_dist,
            un.carb_dist
        FROM users u
        INNER JOIN user_details ud ON u.id = ud.user_id
        INNER JOIN user_preferences up ON u.id = up.user_id
        INNER JOIN user_goals ug ON u.id = ug.user_id
        INNER JOIN user_nutrients un ON u.id = un.user_id
        WHERE u.id = $1
        "#,
        id,
    )
    .fetch_one(db)
    .await
}

// Weight, height, age, sex, idle activity, workout activity, weight goal, goal diff per week
pub async fn fetch_user_details_with_goals(
    db: &PgPool,
    user_id: Uuid,
) -> sqlx::Result<UserDetailsWithGoalsAndPreferences> {
    sqlx::query_as!(
        UserDetailsWithGoalsAndPreferences,
        r#"
        SELECT
            det.sex AS "sex: _",
            det.weight,
            det.height,
            det.date_of_birth,
            det.idle_activity,
            det.workout_activity,
            go.goal_diff_per_week,
            go.weight_goal AS "weight_goal: _",
            pref.weight_unit AS "weight_unit: _",
            pref.height_unit AS "height_unit: _"
        FROM user_details det
        JOIN user_goals go ON go.user_id = det.user_id
        JOIN user_preferences pref ON pref.user_id = det.user_id
        WHERE det.user_id = $1
        "#,
        user_id,
    )
    .fetch_one(db)
    .await
}

pub async fn fetch_user_conflicts_opt(
    db: &PgPool,
    username: Option<String>,
    email: Option<String>,
) -> sqlx::Result<UserConflicts> {
    let mut builder = QueryBuilder::new("SELECT ");
    let mut separated = builder.separated(", ");

    if let Some(username) = username {
        separated
            .push("EXISTS ( SELECT 1 FROM users WHERE username = ")
            .push_bind_unseparated(username)
            .push_unseparated(" ) AS username_taken");
    } else {
        // we need to do a fallback otherwise error will be thrown because SQLx has fucked up struct mapping :DDDDDDD
        separated.push("NULL::bool AS username_taken");
    }

    if let Some(email) = email {
        separated
            .push("EXISTS ( SELECT 1 FROM users WHERE email = ")
            .push_bind_unseparated(email)
            .push_unseparated(" ) AS email_taken");
    } else {
        separated.push("NULL::bool AS email_taken");
    }

    builder
        .build_query_as::<UserConflicts>()
        .fetch_one(db)
        .await
}

pub async fn fetch_user_tdee(db: &PgPool, user_id: Uuid) -> sqlx::Result<f32> {
    let user = sqlx::query!(
        "SELECT tdee FROM user_nutrients WHERE user_id = $1",
        user_id,
    )
    .fetch_one(db)
    .await?;

    Ok(user.tdee)
}

pub async fn insert_user(db: &PgPool, user: InsertUser, nutrition: UserNutrition) -> sqlx::Result<()> {
    let mut tx = db.begin().await?;

    // insert user
    let user_id = sqlx::query!(
        "
        INSERT INTO users (username, display_name, email, password)
        VALUES ($1, $2, $3, $4)
        RETURNING id
        ",
        user.username, user.display_name, user.email, user.password,
    )
    .fetch_one(&mut *tx)
    .await?
    .id;

    // insert details
    sqlx::query!(
        "
        INSERT INTO user_details (user_id, sex, weight, height, date_of_birth, idle_activity, workout_activity, diet_kind)
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
        ",
        user_id, user.sex as _, user.weight, user.height, user.date_of_birth, user.idle_activity, user.workout_activity, user.diet_kind as _,
    )
    .execute(&mut *tx)
    .await?;

    // create preferences
    sqlx::query!(
        "INSERT INTO user_preferences (user_id, weight_unit, height_unit) VALUES ($1, $2, $3)",
        user_id, user.weight_unit as _, user.height_unit as _,
    )
    .execute(&mut *tx)
    .await?;

    // setup meal sections
    sqlx::query!(
        "
        INSERT INTO user_sections (user_id, index, name) VALUES
        ($1, $2, $3), ($1, $4, $5), ($1, $6, $7), ($1, $8, $9)
        ",
        user_id,
        0, "Breakfast",
        1, "Lunch",
        2, "Dinner",
        3, "Snacks",
    )
    .execute(&mut *tx)
    .await?;

    // insert values into user goals
    sqlx::query!(
        "INSERT INTO user_goals (user_id, weight_goal, goal_diff_per_week) VALUES ($1, $2, $3)",
        user_id, user.weight_goal as _, user.goal_diff_per_week,
    )
    .execute(&mut *tx)
    .await?;

    // create metrics table
    sqlx::query!(
        "INSERT INTO user_metrics (user_id, origin) VALUES ($1, $2)",
        user_id, user.origin as _,
    )
    .execute(&mut *tx)
    .await?;

    // insert calculated values into user_nutritions
    sqlx::query!(
        "
        INSERT INTO
            user_nutrients (user_id, bmr, base_tdee, tdee, protein_target, fat_target, carb_target, protein_dist, fat_dist, carb_dist)
        VALUES
            ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)
        ",
        user_id, nutrition.bmr, nutrition.base_tdee, nutrition.tdee,
        nutrition.protein_target, nutrition.fat_target, nutrition.carb_target,
        DEFAULT_PROTEIN_DIST, DEFAULT_FAT_DIST, DEFAULT_CARB_DIST,
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

pub async fn update_credentials(db: &PgPool, user_id: Uuid, update: &User) -> sqlx::Result<()> {
    sqlx::query!(
        "
        UPDATE users
        SET username = $2, display_name = $3, password = $4, email = $5
        WHERE id = $1
        ",
        user_id, update.username, update.display_name, update.password, update.email,
    )
    .execute(db)
    .await?;

    Ok(())
}

pub async fn update_user_details(db: impl PgExecutor<'_>, user_id: Uuid, update: &UserDetailsWithGoalsAndPreferences) -> sqlx::Result<()> {
    sqlx::query!(
        "
        UPDATE user_details
        SET sex = $2, weight = $3, height = $4, date_of_birth = $5, idle_activity = $6, workout_activity = $7
        WHERE user_id = $1
        ",
        user_id, update.sex as _, update.weight, update.height,
        update.date_of_birth, update.idle_activity, update.workout_activity,
    )
    .execute(db)
    .await?;

    Ok(())
}

pub async fn update_user_preferences_opt(
    db: &PgPool,
    user_id: Uuid,
    theme: Option<Theme>,
    language: Option<Language>,
) -> sqlx::Result<()> {
    let mut builder = QueryBuilder::<Postgres>::new("UPDATE user_preferences SET ");
    let mut separated = builder.separated(", ");

    if let Some(theme) = theme {
        separated.push("theme = ");
        separated.push_bind_unseparated(theme);
    }

    if let Some(language) = language {
        separated.push("language = ");
        separated.push_bind_unseparated(language);
    }

    builder
        .push(" WHERE user_id = ")
        .push_bind(user_id);

    builder.build().execute(db).await?;

    Ok(())
}

pub async fn delete_user(db: &PgPool, user_id: Uuid) -> sqlx::Result<()> {
    sqlx::query!(
        "DELETE FROM users WHERE id = $1",
        user_id,
    )
    .execute(db)
    .await?;

    Ok(())
}
