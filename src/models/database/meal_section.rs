use uuid::Uuid;

#[allow(dead_code)]
#[derive(sqlx::FromRow)]
pub struct UserMealSection {
    pub id: Uuid,
    pub user_id: Uuid,
    pub index: i32,
    pub label: String,
}
