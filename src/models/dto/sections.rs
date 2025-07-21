#![deny(dead_code)]

use serde::{Deserialize, Serialize};

use crate::models::database::section::SectionIcon;

#[derive(Deserialize)]
pub struct UpdateSectionRequest {
    pub index: Option<i32>,
    pub name: Option<String>,
}

#[derive(Deserialize)]
pub struct NewSectionRequest {
    pub name: String,
}

#[derive(Serialize)]
pub struct SectionIconView {
    pub id: i32,
    pub emoji: String,
}

impl From<SectionIcon> for SectionIconView {
    fn from(icon: SectionIcon) -> Self {
        Self {
            id: icon.id,
            emoji: icon.emoji,
        }
    }
}