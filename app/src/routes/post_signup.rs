use axum::{
    http::StatusCode, 
    Json, 
    extract::State
};
use datatypes::{ErrMsgStruct, SuccMsgStruct, RegistrationStruct};
use sqlx::{Pool, Postgres};
use common_regex_rs::{is_email};

pub async fn signup_route(
    State(psql): State<Pool<Postgres>>, 
    Json(body): Json<RegistrationStruct>) -> (StatusCode, Result<Json<SuccMsgStruct>, Json<ErrMsgStruct>>) {
    
    if &body.email.is_empty() | !is_email(&body.email) {
        let err_msg: ErrMsgStruct = ErrMsgStruct {
            err_msg: "Provide a valid email"
        };
        return (StatusCode::BAD_REQUEST, Err(Json(err_msg)))
    }

    let err_msg: ErrMsgStruct = ErrMsgStruct {
        err_msg: "Provide a valid email"
    };
    return (StatusCode::BAD_REQUEST, Err(Json(err_msg)))
}