use datatypes::RedisState;
use axum::{
    http::StatusCode,
    response::Response,
    extract::{Request, State},
    middleware::{self, Next}
};
use std::sync::Arc;
// use redis::Connection;
// use sqlx::{Pool, Postgres};

pub async fn auth_middleware(State(redis): State<Arc<RedisState>>, req: Request, next: Next) -> Result<Response, StatusCode> {
    println!("middleware checked");

    // Ok(next.run(req).await)
    Err(StatusCode::UNAUTHORIZED)
}