pub mod auth;
pub mod users;

use axum::Router;
use sqlx::PgPool;
use tower_http::cors::CorsLayer;

pub type AppState = PgPool;

pub fn router(db: PgPool) -> Router {
    Router::new()
        .nest("/auth", auth::router())
        .nest("/users", users::router(db.clone()))

        .with_state(db)
        .layer(CorsLayer::very_permissive())
}
