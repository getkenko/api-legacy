mod cache;
mod database;
mod models;
mod routes;
mod security;
mod services;
mod utils;

use std::net::SocketAddr;

use dotenvy_macro::dotenv;
use sqlx::postgres::PgPoolOptions;
use tokio::net::TcpListener;
use tracing::Level;

use crate::{cache::Cache, utils::logger::setup_logger};

const DATABASE_URL: &str = dotenv!("DATABASE_URL");
const REDIS_URL: &str = dotenv!("REDIS_URL");
const BIND_URL: &str = dotenv!("BIND_URL");

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
    let router = routes::router(db, cache).into_make_service_with_connect_info::<SocketAddr>(); // so we can extract client's remote address

    let listener = TcpListener::bind(BIND_URL).await?;
    tracing::info!("API available at http://{}", listener.local_addr()?);

    axum::serve(listener, router).await?;

    Ok(())
}
