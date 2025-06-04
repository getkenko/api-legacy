use axum::Router;
use sqlx::PgPool;
use tower_http::cors::CorsLayer;

pub mod auth;

type AppState = PgPool;

pub fn router(db: PgPool) -> Router {
    Router::new()
        .nest("/auth", auth::router())
        .with_state(db)
        .layer(CorsLayer::very_permissive())
}
