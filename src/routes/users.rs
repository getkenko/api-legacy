use axum::{http::StatusCode, middleware, routing::get, Router};

use crate::utils::auth_middleware::auth_middleware;

use super::AppState;

pub fn router(state: AppState) -> Router<AppState> {
    Router::new()
        .route("/me", get(authed))
        .layer(middleware::from_fn_with_state(state, auth_middleware))
}

async fn authed() -> StatusCode {
    StatusCode::IM_A_TEAPOT
}
