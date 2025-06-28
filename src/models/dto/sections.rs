use serde::Deserialize;

#[derive(Deserialize)]
pub struct UpdateSectionRequest {
    pub index: Option<i32>,
    pub label: Option<String>,
}

#[derive(Deserialize)]
pub struct NewSectionRequest {
    pub index: i32,
    pub label: String,
}