use redis::Connection;
use serde::{Deserialize, Serialize};
use chrono::{DateTime, Local};
use std::sync::Arc;
use sqlx::{Pool, Postgres};

#[derive(Serialize, Deserialize, Debug)]
pub struct User {
    pub id: String,
    pub username: String,
    pub email: String,
    pub password: String,
    pub active: bool,
    pub created_at: DateTime<Local>,
    pub last_login: Option<DateTime<Local>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Pattern {
    pub id: String,
    pub owner_id: String,
    pub title: String,
    pub pattern_description: String,
    pub gallery_paths: Vec<String>,
    pub pattern_path: String,
    pub materials: Vec<String>,
    pub tools: Vec<String>,
    pub category: String,
    pub created_at: DateTime<Local>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Comment {
    pub id: String,
    pub owner_id: String,
    pub pattern_id: String,
    pub comment: String,
    pub created_at: DateTime<Local>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct ErrMsgStruct {
    pub err_msg: &'static str,
}
#[derive(Serialize, Deserialize, Clone)]
pub struct SuccMsgStruct {
    pub succ_msg: &'static str,
    pub token: Option<&'static str>
}

#[derive(Serialize, Deserialize, Clone)]
pub struct RegistrationStruct {
    pub username: String,
    pub email: String,
    pub password: String,
}