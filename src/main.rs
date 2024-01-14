mod routes;
mod structs;

#[allow(unused)]
use axum::{
    routing::{get, post},
    http::StatusCode,
    Json, Router,
};

use dotenv::dotenv;
use sqlx::postgres::PgPoolOptions;
use std::{
    time::Duration,
    sync::Arc,
};
use std::env;
use routes::*;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    
    dotenv().ok();
    let port = env::var("SERVER_PORT").unwrap();
    let api_version = env::var("API_VERSION").unwrap();
    let psql_url = env::var("PSQL_URL").unwrap();
    let redis_url = env::var("REDIS_URL").unwrap();

    let psql = PgPoolOptions::new()
        .max_connections(5)
        .acquire_timeout(Duration::from_secs(3))
        .connect(&psql_url)
        .await
        .expect("can't connect to psql database");

    let redis_client = redis::Client::open(redis_url).expect("Failed to create Redis client");
    let redis = redis_client.get_connection().expect("Failed to connect redis");

    let app = Router::new()
        .route("/", get(get_index::index_page))
        .route("/signup", get(get_signup::signup_page))
        .route("/login", get(get_login::login_page))
        .route(format!("/api/{}/signup", api_version).as_str(), get(post_signup::signup_route))
        .route(format!("/api/{}/login", api_version).as_str(), get(post_login::login_route))
        .with_state(psql)
        .with_state(Arc::new(redis));
    
    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", port)).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
