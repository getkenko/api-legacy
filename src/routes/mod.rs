pub mod auth;
pub mod users;
pub mod products;
pub mod meals;

use axum::{extract::DefaultBodyLimit, middleware, Router};
use sqlx::PgPool;
use tower_http::{cors::CorsLayer, limit::RequestBodyLimitLayer, services::ServeDir};

use crate::security::middlewares::rate_limit_middleware;

pub type AppState = PgPool;

pub fn router(db: PgPool) -> Router {
    Router::new()
        .nest("/auth", auth::router())
        .nest("/users", users::router())
        .nest("/products", products::router())
        .nest("/meals", meals::router())
        .nest_service("/public", ServeDir::new("public"))

        .with_state(db)
        .layer((
            middleware::from_fn(rate_limit_middleware),
            DefaultBodyLimit::disable(),
            RequestBodyLimitLayer::new(10_485_760),
            // TODO: set strict cors before production
            CorsLayer::very_permissive(),
        ))
}
