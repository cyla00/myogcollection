use crate::structs::PsqlState;
use axum::{
    http::StatusCode,
    response::Response,
    extract::{Request, State},
    middleware::{self, Next}
};
use sqlx::{Pool, Postgres};

pub async fn auth_middleware(State(db): State<PsqlState>, req: Request, next: Next) -> Response {
    println!("middleware checked");
    next.run(req).await
}