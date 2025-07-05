use sqlx::PgPool;
use uuid::Uuid;

pub async fn update_user_nutrients_macros(
    db: &PgPool,
    user_id: Uuid,
    protein_dist: Option<i32>,
    fat_dist: Option<i32>,
    carb_dist: Option<i32>,
    protein_target: i32,
    fat_target: i32,
    carb_target: i32,
) -> sqlx::Result<()> {
    sqlx::query!(
        "
        UPDATE user_nutrients
        SET
            protein_dist = $2,
            fat_dist = $3,
            carb_dist = $4,
            protein_target = $5,
            fat_target = $6,
            carb_target = $7
        WHERE user_id = $1
        ",
        user_id,
        protein_dist, fat_dist, carb_dist,
        protein_target, fat_target, carb_target,
    )
    .execute(db)
    .await?;

    Ok(())
}

pub async fn fetch_user_nutrients_tdee(db: &PgPool, user_id: Uuid) -> sqlx::Result<f32> {
    let nutrients = sqlx::query!(
        "SELECT tdee FROM user_nutrients WHERE user_id = $1",
        user_id,
    )
    .fetch_one(db)
    .await?;

    Ok(nutrients.tdee)
}
