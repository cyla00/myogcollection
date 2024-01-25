use axum::{
    http::StatusCode,
    response::Response,
    extract::{Request, State},
    middleware::Next,
};
use std::sync::{Arc, Mutex};
use redis::{Commands, Connection};

pub async fn auth_middleware(State(redis): State<Arc<Mutex<Connection>>>, req: Request, next: Next) -> Result<Response, StatusCode> {
    println!("middleware checked");
    // Ok(next.run(req).await)
    Err(StatusCode::UNAUTHORIZED)
}