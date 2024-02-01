// takes body: "system_info", "ip"
// takes headers: Basic auth

use axum::{
    Json, 
    http::StatusCode,
    extract::State,
};
use redis::{Commands, Connection, RedisError, RedisResult};
use std::sync::{Arc, Mutex};
use axum_extra::{
    extract::cookie::{CookieJar, Cookie},
    headers::{
        authorization::Basic, Authorization
    }, 
    TypedHeader,
};
use datatypes::{ErrMsgStruct, SuccMsgStruct, LoginBodyInformation};
use sqlx::{
    Pool, Postgres, Row
};
use crate::password_manager::password_verification;
use uuid::Uuid;
use chrono::Local;
use ipgeolocate::{Locator, Service};
use dotenv::dotenv;
use std::env;


pub async fn login_route(
    State((redis, psql)): State<(Arc<Mutex<Connection>>, Pool<Postgres>)>,
    TypedHeader(auth): TypedHeader<Authorization<Basic>>,
    jar: CookieJar,
    Json(body): Json<LoginBodyInformation>
) -> (StatusCode, Result<Json<SuccMsgStruct>, Json<ErrMsgStruct>>) {

    let (auth_password, auth_email) = (auth.password(), auth.username());
    
    let check_user = sqlx::query("
        SELECT * FROM users WHERE email=$1;
    ")
    .bind(auth_email)
    .fetch_one(&psql).await;

    match check_user {
        Ok(user) => {

            let active: bool = user.get("active");
            if !active {
                let err_msg: ErrMsgStruct = ErrMsgStruct {
                    err_msg: "Verify your email before connecting"
                };
                return (StatusCode::UNAUTHORIZED, Err(Json(err_msg)))
            }

            let fetched_password:String = user.get("password"); 
            let fetched_id:String = user.get("id"); 

            let password_check = password_verification(fetched_password, auth_password.to_string());
            if !password_check {
                let err_msg: ErrMsgStruct = ErrMsgStruct {
                    err_msg: "Incorrect crendentials"
                };
                return (StatusCode::UNAUTHORIZED, Err(Json(err_msg)))
            }

            let new_session_key = Uuid::new_v4().to_string();
            let service = Service::IpApi;
            match Locator::get(&body.ip, service).await {
                Ok(geolocation) => {
                    let set_session: RedisResult<String> = redis.lock().unwrap()
                        .hset_multiple(&new_session_key, &[
                            ("owner_id", &fetched_id),
                            ("system", &body.system_info),
                            ("ip", &body.ip),
                            ("geolocation", &format!("{}, {}", geolocation.city, &geolocation.country)),
                            ("created_at", &Local::now().to_string())
                        ]);


                    dotenv().ok();

                    // let _ = jar.add(Cookie::new("session_id", new_session_key));

                    match set_session {
                        Ok(ok) => {
                            println!("{ok:?}");
                            
                            let succ_msg: SuccMsgStruct = SuccMsgStruct {
                                succ_msg: "Successfully connected",
                            };
                            return (StatusCode::OK, Ok(Json(succ_msg)))
                        }
                        Err(err)   => {
                            println!("{err:?}");
                            let err_msg: ErrMsgStruct = ErrMsgStruct {
                                err_msg: "An error occurred, please retry later"
                            };
                            return (StatusCode::BAD_GATEWAY, Err(Json(err_msg)))
                        }
                    }
                }
                Err(err) => {
                    println!("{err:?}");
                    let err_msg: ErrMsgStruct = ErrMsgStruct {
                        err_msg: "Please reconnect in a few minutes"
                    };
                    return (StatusCode::UNAUTHORIZED, Err(Json(err_msg)))
                }
            };
        }
        Err(_) => {
            let err_msg: ErrMsgStruct = ErrMsgStruct {
                err_msg: "Incorrect credentials"
            };
            return (StatusCode::UNAUTHORIZED, Err(Json(err_msg)))
        }
    }
}