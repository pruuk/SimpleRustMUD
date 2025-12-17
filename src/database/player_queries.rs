// Player database operations

use crate::models::Player;
use sqlx::SqlitePool;

pub async fn create_player(
    db: &SqlitePool,
    player: &Player,
) -> Result<(), String> {
    sqlx::query(
        r#"
        INSERT INTO players (id, username, password_hash, current_location, created_at)
        VALUES (?, ?, ?, ?, ?)
        "#,
    )
    .bind(&player.id)
    .bind(&player.username)
    .bind(&player.password_hash)
    .bind(&player.current_location)
    .bind(&player.is_admin)
    .bind(player.created_at)
    .execute(db)
    .await
    .map_err(|e| format!("Failed to create player: {}", e))?;

    Ok(())
}

pub async fn get_player_by_username(
    db: &SqlitePool,
    username: &str,
) -> Result<Option<Player>, String> {
    sqlx::query_as("SELECT * FROM players WHERE username = ?")
        .bind(username)
        .fetch_optional(db)
        .await
        .map_err(|e| format!("Database error: {}", e))
}

pub async fn get_player_by_id(
    db: &SqlitePool,
    player_id: &str,
) -> Result<Player, String> {
    sqlx::query_as("SELECT * FROM players WHERE id = ?")
        .bind(player_id)
        .fetch_one(db)
        .await
        .map_err(|e| format!("Player not found: {}", e))
}

pub async fn get_players_in_room(
    db: &SqlitePool,
    room_id: &str,
) -> Result<Vec<Player>, String> {
    sqlx::query_as("SELECT * FROM players WHERE current_location = ?")
        .bind(room_id)
        .fetch_all(db)
        .await
        .map_err(|e| format!("Failed to fetch players: {}", e))
}

pub async fn update_player_location(
    db: &SqlitePool,
    player_id: &str,
    new_location: &str,
) -> Result<(), String> {
    sqlx::query("UPDATE players SET current_location = ? WHERE id = ?")
        .bind(new_location)
        .bind(player_id)
        .execute(db)
        .await
        .map_err(|e| format!("Failed to update location: {}", e))?;

    Ok(())
}
