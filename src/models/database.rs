use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, sqlx::Type)]
#[serde(rename_all = "camelCase")]
#[sqlx(type_name = "gender_enum", rename_all = "snake_case")]
pub enum Gender {
    Male,
    Female,
    Other,
}
