// Player struct and methods

use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Player {
    pub id: String,
    pub username: String,
    pub password_hash: String,
    pub current_location: String,
    pub is_admin: i64, // use 0 or 1, SQLite doesn't have bools
    pub created_at: i64,
}

impl Player {
    pub fn new(id: String, username: String, password_hash: String, is_admin: i64) -> Self {
        Self {
            id,
            username,
            password_hash,
            current_location: "room_start".to_string(),
            is_admin,
            created_at: chrono::Utc::now().timestamp(),
        }
    }
}
