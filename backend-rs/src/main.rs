mod agent;
mod check;
mod completion;
mod dtos;
mod error;
mod handler;
mod model;
mod route;
mod task;
mod traits;
mod types;

use std::env;

use anyhow::{Ok, Result};
use axum::{
    http::{
        header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE},
        HeaderValue, Method,
    },
    Extension,
};
use route::create_router;
use sqlx::{sqlite::SqlitePoolOptions, Pool, Sqlite};
use tower_http::cors::CorsLayer;

const DEFAULT_BEAREER_TOKEN: &str = "SwexCamp2024!";
const DEFAULT_DATABASE_URL: &str = "sqlite://sqlite.db";
const ADDRESS: &str = "0.0.0.0:8000";

#[tokio::main]
async fn main() -> Result<()> {
    dotenv::dotenv().ok();

    let bearer_token = env::var("BEARER_TOKEN").unwrap_or(DEFAULT_BEAREER_TOKEN.into());
    let database_url = env::var("DATABASE_URL").unwrap_or(DEFAULT_DATABASE_URL.into());

    let db = init_db(database_url.trim()).await?;

    let app = create_router(&bearer_token)
        .layer(create_cors_layer()?)
        .layer(Extension(db));

    println!("🚀 Server started successfully");

    let listener = tokio::net::TcpListener::bind(ADDRESS).await?;
    axum::serve(listener, app).await?;
    Ok(())
}

async fn init_db(database_url: &str) -> Result<Pool<Sqlite>> {
    let pool = SqlitePoolOptions::new().connect(database_url).await?;
    sqlx::migrate!("./migrations").run(&pool).await?;
    Ok(pool)
}

fn create_cors_layer() -> Result<CorsLayer> {
    Ok(CorsLayer::new()
        .allow_origin("http://localhost:3000".parse::<HeaderValue>()?)
        .allow_methods([Method::GET, Method::POST, Method::PATCH, Method::DELETE])
        .allow_credentials(true)
        .allow_headers([AUTHORIZATION, ACCEPT, CONTENT_TYPE]))
}
