mod models;
mod utils;
mod security;
mod database;
mod cache;
mod services;
mod routes;

use std::net::SocketAddr;

use dotenvy_macro::dotenv;
use sqlx::postgres::PgPoolOptions;
use tokio::net::TcpListener;
use tracing::Level;

use crate::{cache::Cache, utils::logger::setup_logger};

// TODO: custom extractors for database/cache

const DATABASE_URL: &str = dotenv!("DATABASE_URL");
const REDIS_URL: &str = dotenv!("REDIS_URL");

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    setup_logger(Level::INFO);
    tracing::info!("Logger initialized");

    // initialize database connection
    let db = PgPoolOptions::new()
        .max_connections(10)
        .connect(DATABASE_URL)
        .await?;
    tracing::info!("Connection with database established");

    // initialize redis connection
    let cache = Cache::new(REDIS_URL).await?;
    tracing::info!("Connection with redis established");

    // setup axum web server
    let router = routes::router(db, cache)
        .into_make_service_with_connect_info::<SocketAddr>(); // so we can extract client's remote address

    let listener = TcpListener::bind("127.0.0.1:3000").await?;
    tracing::info!("API available at http://{}", listener.local_addr()?);

    axum::serve(listener, router).await?;

    Ok(())
}
