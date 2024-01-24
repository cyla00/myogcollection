use axum::{
    Json, 
    http::StatusCode,
    extract::State,
};
use redis::{Commands, Connection, RedisError};
use std::sync::Arc;
use axum_extra::{headers::{authorization::Basic, Authorization}, TypedHeader};
use datatypes::{ErrMsgStruct, SuccMsgStruct};
use sqlx::{
    Pool, Postgres, Row
};
use crate::password_manager::password_verification;
use uuid::Uuid;


pub async fn login_route(
    State((redis, psql)): State<(Arc<Connection>, Pool<Postgres>)>,
    TypedHeader(auth): TypedHeader<Authorization<Basic>>
) -> (StatusCode, Result<Json<SuccMsgStruct>, Json<ErrMsgStruct>>) {

    let (auth_password, auth_email) = (auth.password(), auth.username());
    
    let check_password_query = sqlx::query("
        SELECT password FROM users WHERE email=$1;
    ")
    .bind(auth_email)
    .fetch_one(&psql).await;

    match check_password_query {
        Ok(user) => {
            let fetched_password:String = user.get("password"); 

            let password_check = password_verification(auth_password.to_string(), fetched_password);
            if !password_check {
                let err_msg: ErrMsgStruct = ErrMsgStruct {
                    err_msg: "Incorrect crendentials"
                };
                return (StatusCode::UNAUTHORIZED, Err(Json(err_msg)))
            }

            // let test: Result<String, RedisError> = redis.set("sessionid", Uuid::new_v4().to_string());

            // let ok: Result<String, RedisError> = redis.get("my_key");

            // match ok {
            //     Ok(key) => {
            //         println!("{key:?}");
            //     }
            //     Err(err) => {
            //         println!("{err:?}");
            //     }
            // }
            
            let succ_msg: SuccMsgStruct = SuccMsgStruct {
                succ_msg: "success"
            };
            return (StatusCode::OK, Ok(Json(succ_msg)))
        }
        Err(err) => {
            println!("{err:?}");
            let err_msg: ErrMsgStruct = ErrMsgStruct {
                err_msg: "An error occurred, retry later"
            };
            return (StatusCode::BAD_GATEWAY, Err(Json(err_msg)))
        }
    }
}