use redis::Connection;
use axum::{
    http::StatusCode, 
    Json, 
    extract::State
};
use datatypes::{ErrMsgStruct, SuccMsgStruct, RegistrationStruct, User};
use sqlx::{Pool, Postgres};
use common_regex_rs::{is_email, is_good_password};
use chrono::Local;
use uuid::Uuid;
use crate::password_manager::password_hashing;
use std::sync::Arc;

pub async fn signup_route(
    State((_redis, psql)): State<(Arc<Connection>, Pool<Postgres>)>, 
    Json(body): Json<RegistrationStruct>) -> (StatusCode, Result<Json<SuccMsgStruct>, Json<ErrMsgStruct>>) {
    
    if body.email.is_empty() | !is_email(&body.email) {
        let err_msg: ErrMsgStruct = ErrMsgStruct {
            err_msg: "Provide a valid email"
        };
        return (StatusCode::BAD_REQUEST, Err(Json(err_msg)))
    }

    if body.username.is_empty() {
        let err_msg: ErrMsgStruct = ErrMsgStruct {
            err_msg: "Provide a valid username"
        };
        return (StatusCode::BAD_REQUEST, Err(Json(err_msg)))
    }

    if body.password.is_empty() || !is_good_password(&body.password) {
        let err_msg: ErrMsgStruct = ErrMsgStruct {
            err_msg: "Provide a stronger password"
        };
        return (StatusCode::BAD_REQUEST, Err(Json(err_msg)))
    }
    
    let hashed_pass = password_hashing(body.password);

    let new_user: User = User {
        id: Uuid::new_v4().to_string(),
        username: body.username,
        email: body.email,
        password: hashed_pass.clone(),
        active: false,
        created_at: Local::now(),
        last_login: None,
    };

    let username_check = sqlx::query("
        SELECT * FROM users WHERE username=$1;
    ").bind(&new_user.username).execute(&psql).await;

    match username_check {
        Ok(user) => {
            if user.rows_affected() != 0 {
                let err_msg: ErrMsgStruct = ErrMsgStruct {
                    err_msg: "Username not available"
                };
                return (StatusCode::BAD_REQUEST, Err(Json(err_msg)))
            }

            let email_check = sqlx::query("
                SELECT * FROM users WHERE email=$1;
            ").bind(&new_user.email).execute(&psql).await;

            match email_check {
                Ok(user) => {
                    if user.rows_affected() != 0 {
                        let err_msg: ErrMsgStruct = ErrMsgStruct {
                            err_msg: "An account with that email exists"
                        };
                        return (StatusCode::BAD_REQUEST, Err(Json(err_msg)))
                    }

                    let user_check = sqlx::query("
                        INSERT INTO users ( id, username, email, password, active, created_at, last_login ) 
                        VALUES ($1, $2, $3, $4, $5, $6, $7) ON CONFLICT (email) DO NOTHING;
                    ")
                    .bind(new_user.id)
                    .bind(new_user.username)
                    .bind(new_user.email)
                    .bind(new_user.password)
                    .bind(new_user.active)
                    .bind(new_user.created_at)
                    .bind(new_user.last_login)
                    .execute(&psql).await;

                    match user_check {
                        Ok(_) => {
                            let succ_msg: SuccMsgStruct = SuccMsgStruct {
                                succ_msg: "Successfully registered"
                            };
                            return (StatusCode::BAD_REQUEST, Ok(Json(succ_msg)))
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
                Err(err) => {
                    println!("{err:?}");
                    let err_msg: ErrMsgStruct = ErrMsgStruct {
                        err_msg: "An error occurred, retry later"
                    };
                    return (StatusCode::BAD_GATEWAY, Err(Json(err_msg)))
                }
            }
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