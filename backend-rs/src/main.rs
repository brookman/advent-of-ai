mod agent;
mod auth;
mod check;
mod completion;
mod error;
mod health_check;
mod route;
mod task;
mod traits;

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

const DEFAULT_USER_TOKEN: &str = "SwexCamp2024!";
const DEFAULT_ADMIN_TOKEN: &str = "SwexCamp2024Admin!";
const DEFAULT_DATABASE_URL: &str = "sqlite://sqlite.db";
const ADDRESS: &str = "0.0.0.0:8000";

#[tokio::main]
async fn main() -> Result<()> {
    dotenv::dotenv().ok();

    let user_token = env::var("USER_TOKEN").unwrap_or(DEFAULT_USER_TOKEN.into());
    let admin_token = env::var("ADMIN_TOKEN").unwrap_or(DEFAULT_ADMIN_TOKEN.into());
    let database_url = env::var("DATABASE_URL").unwrap_or(DEFAULT_DATABASE_URL.into());

    let router = create_router(&user_token, &admin_token);

    let db = init_db(database_url.trim()).await?;

    let app = router.layer(create_cors_layer()?).layer(Extension(db));

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
