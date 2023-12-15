use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct UserStruct {
    pub id: String,
    pub username: String,
    pub email: String,
    pub password: String,
}