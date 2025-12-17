// Game state struct

use crate::config::ServerConfig;
use crate::database;
use crate::models::{Player, GameObject, Session};
use sqlx::sqlite::{SqlitePool, SqlitePoolOptions};
use tokio::sync::{broadcast, RwLock};
use std::collections::HashMap;
use std::sync::Arc;
use uuid::Uuid;

pub struct GameState {
    pub db: SqlitePool,
    pub sessions: Arc<RwLock<HashMap<String, Session>>>,
    pub broadcast_tx: broadcast::Sender<String>,
    pub config: ServerConfig,
}

impl GameState {
    pub async fn new(config: ServerConfig) -> Result<Self, Box<dyn std::error::Error>> {
        // Create database file if it doesn't exist
        if !std::path::Path::new("mud.db").exists() {
            std::fs::File::create("mud.db")?;
        }
        
        let db = SqlitePoolOptions::new()
            .max_connections(5)
            .connect(&config.database_url)
            .await?;

        // Initialize database schema
        database::initialize_database(&db).await?;

        let (tx, _) = broadcast::channel(100);

        Ok(Self {
            db,
            sessions: Arc::new(RwLock::new(HashMap::new())),
            broadcast_tx: tx,
            config,
        })
    }

    pub async fn authenticate(&self, username: &str, password: &str) -> Result<Player, String> {
        let player = database::player_queries::get_player_by_username(&self.db, username).await?;

        match player {
            Some(p) => {
                use argon2::password_hash::{PasswordHash, PasswordVerifier};
                let parsed_hash = PasswordHash::new(&p.password_hash)
                    .map_err(|_| "Invalid password hash".to_string())?;
                
                argon2::Argon2::default()
                    .verify_password(password.as_bytes(), &parsed_hash)
                    .map_err(|_| "Invalid credentials".to_string())?;
                
                Ok(p)
            }
            None => Err("Invalid credentials".to_string()),
        }
    }

    pub async fn register_player(&self, username: &str, password: &str, is_admin: i64) -> Result<Player, String> {
        use argon2::{
            password_hash::{
                rand_core::OsRng,
                PasswordHash, PasswordHasher, PasswordVerifier, SaltString
            },
            Argon2
        };    
        let salt = SaltString::generate(&mut OsRng);
        let argon2 = argon2::Argon2::default();
        let password_hash = argon2
            .hash_password(password.as_bytes(), &salt)
            .map_err(|e| format!("Password hashing failed: {}", e))?
            .to_string();
        let player = Player::new(
            Uuid::new_v4().to_string(),
            username.to_string(),
            password_hash,
            is_admin,
        ).await;

        database::player_queries::create_player(&self.db, &player).await?;

        Ok(player)
    }

    pub async fn get_room(&self, room_id: &str) -> Result<GameObject, String> {
        database::object_queries::get_room(&self.db, room_id).await
    }

    pub async fn get_objects_in_container(&self, container_id: &str) -> Result<Vec<GameObject>, String> {
        database::object_queries::get_objects_in_container(&self.db, container_id).await
    }

    pub async fn get_players_in_room(&self, room_id: &str) -> Result<Vec<Player>, String> {
        database::player_queries::get_players_in_room(&self.db, room_id).await
    }

    pub async fn add_exit(&self, room_id: &str, direction: &str, destination_id: &str) -> Result<(), String> {
        // Verify both rooms exist
        self.get_room(room_id).await?;
        self.get_room(destination_id).await?;
        
        sqlx::query(
            "INSERT OR REPLACE INTO room_exits (room_id, direction, destination_id) VALUES (?, ?, ?)"
        )
        .bind(room_id)
        .bind(direction.to_lowercase())
        .bind(destination_id)
        .execute(&self.db)
        .await
        .map_err(|e| format!("Failed to add exit: {}", e))?;
        
        Ok(())
    }
    
    pub async fn get_exits(&self, room_id: &str) -> Result<Vec<(String, String)>, String> {
        let exits: Vec<(String, String)> = sqlx::query_as(
            "SELECT direction, destination_id FROM room_exits WHERE room_id = ?"
        )
        .bind(room_id)
        .fetch_all(&self.db)
        .await
        .map_err(|e| format!("Failed to get exits: {}", e))?;
        
        Ok(exits)
    }
    
    pub async fn move_player_to_room(&self, player_id: &str, new_room_id: &str) -> Result<(), String> {
        self.get_room(new_room_id).await?;
        
        sqlx::query("UPDATE players SET current_location = ? WHERE id = ?")
            .bind(new_room_id)
            .bind(player_id)
            .execute(&self.db)
            .await
            .map_err(|e| format!("Failed to move player: {}", e))?;
        
        Ok(())
    }    

    pub async fn create_object(
        &self,
        name: &str,
        description: &str,
        object_type: &str,
        container_id: Option<&str>,
    ) -> Result<GameObject, String> {
        let obj = GameObject::new(
            Uuid::new_v4().to_string(),
            name.to_string(),
            description.to_string(),
            object_type.to_string(),
            container_id.map(|s| s.to_string()),
        );

        database::object_queries::create_object(&self.db, &obj).await?;

        Ok(obj)
    }
}
