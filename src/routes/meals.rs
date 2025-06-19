use axum::{http::StatusCode, Router};

use crate::models::errors::AppResult;

use super::AppState;

pub fn router() -> Router<AppState> {
    Router::new()
}

async fn quick_add_product() -> AppResult<StatusCode> {
    Ok(StatusCode::CREATED)
}

