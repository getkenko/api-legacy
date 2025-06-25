mod models;
mod utils;
mod security;
mod database;
mod services;
mod routes;

use axum::extract::DefaultBodyLimit;
use sqlx::postgres::PgPoolOptions;
use tokio::net::TcpListener;
use tower_http::{cors::CorsLayer, limit::RequestBodyLimitLayer, services::ServeDir};

#[tokio::main]
async fn main() {
    // load .env variables
    dotenvy::dotenv().unwrap();
    let db_url = dotenvy::var("DATABASE_URL").unwrap();
    println!("Environment variables loaded");

    // initialize database connection
    let db = PgPoolOptions::new()
        .max_connections(10)
        .connect(&db_url)
        .await
        .unwrap();
    println!("Connection with database established");

    // setup axum web server
    let router = routes::router(db)
        .nest_service("/public", ServeDir::new("public"))
        .layer((
            DefaultBodyLimit::disable(),
            RequestBodyLimitLayer::new(10_485_760),
            // TODO: set strict cors only for official website before production
            CorsLayer::very_permissive(),
        ));
    let listener = TcpListener::bind("127.0.0.1:3000").await.unwrap();
    println!("API available at http://{}", listener.local_addr().unwrap());

    axum::serve(listener, router).await.unwrap();
}
