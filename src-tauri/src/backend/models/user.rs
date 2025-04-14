use serde::{Deserialize, Serialize};
use chrono::NaiveDateTime;

#[allow(dead_code)]
#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct User {
    pub id: i64,
    pub full_name: String,
    pub email: String,
    pub username: String,
    pub password_hash: Option<String>,
    pub spotify_id: Option<String>,
    pub created_at: NaiveDateTime,
}

#[derive(Debug, Deserialize)]
pub struct Register {
    pub name: String,
    pub username: String,
    pub email: String,
    pub password: String,
}

#[derive(Debug, Deserialize)]
pub struct Login {
    pub username: String,
    pub email: String,
    pub password: String,
}

#[derive(Debug, Serialize)]
pub struct UserResponse {
    pub id: u32,
    pub username: String,
    pub email: String,
    pub token: String,
}
