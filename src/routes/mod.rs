mod auth;
mod users;
mod products;
mod meals;
mod sections;
mod nutrients;

use axum::{extract::DefaultBodyLimit, middleware, Router};
use sqlx::PgPool;
use tower_http::{cors::CorsLayer, limit::RequestBodyLimitLayer, services::ServeDir};

use crate::{cache::Cache, security::middlewares::rate_limit_middleware};

#[derive(Clone)]
pub struct AppState {
    pub db: PgPool,
    pub cache: Cache,
}

pub fn router(db: PgPool, cache: Cache) -> Router {
    let state = AppState { db, cache };

    Router::new()
        .nest("/auth", auth::router())
        .nest("/users", users::router(state.clone()))
        .nest("/products", products::router())
        .nest("/meals", meals::router(state.clone()))
        .nest("/sections", sections::router(state.clone()))
        .nest("/nutrients", nutrients::router(state.clone()))
        .nest_service("/public", ServeDir::new("public"))

        .with_state(state.clone())
        .layer((
            middleware::from_fn_with_state(state, rate_limit_middleware),
            DefaultBodyLimit::disable(),
            RequestBodyLimitLayer::new(10_485_760),
            // TODO: set strict cors before production
            CorsLayer::very_permissive(),
        ))
}
