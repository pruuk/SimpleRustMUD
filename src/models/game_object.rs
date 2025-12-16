// Game Object struct and methods

use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct GameObject {
    pub id: String,
    pub name: String,
    pub description: String,
    pub object_type: String,
    pub container_id: Option<String>,
    pub properties: String,
    pub created_at: i64,
}

impl GameObject {
    pub fn new(
        id: String,
        name: String,
        description: String,
        object_type: String,
        container_id: Option<String>,
    ) -> Self {
        Self {
            id,
            name,
            description,
            object_type,
            container_id,
            properties: "{}".to_string(),
            created_at: chrono::Utc::now().timestamp(),
        }
    }
}
