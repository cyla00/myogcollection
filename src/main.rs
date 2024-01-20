mod routes;
mod structs;
mod middlewares;


#[allow(unused)]
use axum::{
    routing::{get, post},
    http::StatusCode,
    Json, Router,
    middleware::{self, Next},
    extract::State
};

use redis::{Client, Connection};
use dotenv::dotenv;
use middlewares::auth;
use sqlx::{postgres::PgPoolOptions, Pool, Postgres};
use tokio::net::TcpListener;
use std::{
    time::Duration,
    sync::Arc
};
use std::env;
use routes::*;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    
    dotenv().ok();
    let port: String = env::var("SERVER_PORT").unwrap();
    let api_version: String = env::var("API_VERSION").unwrap();
    let psql_url: String = env::var("PSQL_URL").unwrap();
    let redis_url: String = env::var("REDIS_URL").unwrap();

    let psql: Pool<Postgres> = PgPoolOptions::new()
        .max_connections(5)
        .acquire_timeout(Duration::from_secs(3))
        .connect(&psql_url)
        .await
        .expect("can't connect to psql database");
    
    let redis_client: Client = redis::Client::open(redis_url).expect("Failed to create Redis client");
    let redis: Connection = redis_client.get_connection().expect("Failed to connect redis");

    // template GET routes
    let template_routes: Router = Router::new()
        .route("/", get(get_index::index_page))
        .route("/signup", get(get_signup::signup_page))
        .route("/login", get(get_login::login_page));

    // API routes no AUTH
    let unauth_api_routes: Router = Router::new()
        .route("/signup", post(post_signup::signup_route))
        .route("/login", post(post_login::login_route));

    let redis_state: structs::RedisState = structs::RedisState { 
        redis
    };

    // API routes AUTH required
    let auth_api_routes: Router = Router::new()
        .route("/create-pattern", post(post_pattern::create_pattern_route))
        .route_layer(middleware::from_fn_with_state(Arc::new(redis_state), auth::auth_middleware))
        .with_state(psql);

    // API endpoints nesting
    let routes: Router = Router::new()
        .nest("/", template_routes)
        .nest(format!("/api/{}", api_version).as_str(), unauth_api_routes) 
        .nest(format!("/api/{}", api_version).as_str(), auth_api_routes);

    let app: Router = Router::new()
        .nest("/", routes);
        
    
    let listener: TcpListener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", port)).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
