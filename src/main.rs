mod routes;
mod structs;

use axum::{
    routing::{get, post},
    http::StatusCode,
    Json, Router,
};

use dotenv::dotenv;
use std::env;

use routes::get_index::index_page;
use routes::get_login::login_page;
use routes::get_signup::signup_page;
use routes::post_login::login_route;
use routes::post_signup::signup_route;

use structs::{ User, Patern, Comment };
use chrono::{ DateTime, Utc };

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    
    dotenv().ok();
    let port = env::var("SERVER_PORT").unwrap();
    let api_version = env::var("API_VERSION").unwrap();
    let db_name = env::var("DB_NAME").unwrap();
    let db_urs = env::var("DB_USR").unwrap();
    let db_pwd = env::var("DB_PWD").unwrap();
    let db_host = env::var("DB_HOST").unwrap();
    let db_port = env::var("DB_PORT").unwrap();



    let app = Router::new()
        .route("/", get(index_page))
        .route("/signup", get(signup_page))
        .route("/login", get(login_page))
        .route(format!("/api/{}/signup", api_version).as_str(), get(signup_route))
        .route(format!("/api/{}/login", api_version).as_str(), get(login_route));

    
    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", port)).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
