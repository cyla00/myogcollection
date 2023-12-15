mod routes;
mod structs;

use tokio_postgres::{NoTls, Error};
#[allow(unused_imports)]
use axum::{
    routing::{get, post},
    http::StatusCode,
    Json, Router,
};
#[allow(unused_imports)]
use serde::{Deserialize, Serialize};
use dotenv::dotenv;
use std::sync::Arc;
#[allow(unused_imports)]
use std::env;

#[tokio::main]
async fn main() -> Result<(), Error> {

    dotenv().ok();
    let _db_user = env::var("DB_USER").unwrap();
    let _db_pwd = env::var("DB_PWD").unwrap();
    let _db_name = env::var("DB_NAME").unwrap();
    let _db_host = env::var("DB_HOST").unwrap();
    let _db_port = env::var("DB_PORT").unwrap();

    let  (db, connection) = 
        tokio_postgres::connect(format!("postgresql://{}:{}@{}/{}", _db_user, _db_pwd, _db_host, _db_name).as_str(), NoTls).await.unwrap();

    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("connection error: {}", e);
        }
    });
    
    tracing_subscriber::fmt::init();
    let app = Router::new()
        .route("/", get(routes::get_index::index))
        .with_state(Arc::new(db));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
    Ok(())
}