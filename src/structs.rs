use serde::{Deserialize, Serialize};
use chrono::{ DateTime, Utc, Local };

#[derive(Serialize, Deserialize, Debug)]
pub struct User {
    pub id: String,
    pub username: String,
    pub email: String,
    pub password: String,
    // pub created_at: DateTime<Local>,
    pub active: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Patern {
    pub id: String,
    pub owner_id: String,
    pub title: String,
    pub pattern_description: String,
    pub gallery_paths: Vec<(String, String)>,
    pub pattern_path: String,
    pub materials: Vec<(String, String)>,
    pub tools: Vec<(String, String)>,
    pub category: String,
    // pub created_at: DateTime<Utc>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Comment {
    pub id: String,
    pub owner_id: String,
    pub pattern_id: String,
    pub comment: String,
    // pub created_at: DateTime<Utc>,
}
