// Game object database operations

use crate::models::GameObject;
use sqlx::SqlitePool;

pub async fn create_object(
    db: &SqlitePool,
    object: &GameObject,
) -> Result<(), String> {
    sqlx::query(
        r#"
        INSERT INTO game_objects (id, name, description, object_type, container_id, properties, created_at)
        VALUES (?, ?, ?, ?, ?, ?, ?)
        "#,
    )
    .bind(&object.id)
    .bind(&object.name)
    .bind(&object.description)
    .bind(&object.object_type)
    .bind(&object.container_id)
    .bind(&object.properties)
    .bind(object.created_at)
    .execute(db)
    .await
    .map_err(|e| format!("Failed to create object: {}", e))?;

    Ok(())
}

pub async fn get_room(
    db: &SqlitePool,
    room_id: &str,
) -> Result<GameObject, String> {
    sqlx::query_as("SELECT * FROM game_objects WHERE id = ? AND object_type = 'room'")
        .bind(room_id)
        .fetch_one(db)
        .await
        .map_err(|_| "Room not found".to_string())
}

pub async fn get_objects_in_container(
    db: &SqlitePool,
    container_id: &str,
) -> Result<Vec<GameObject>, String> {
    sqlx::query_as("SELECT * FROM game_objects WHERE container_id = ?")
        .bind(container_id)
        .fetch_all(db)
        .await
        .map_err(|e| format!("Failed to fetch objects: {}", e))
}

pub async fn delete_object(
    db: &SqlitePool,
    object_id: &str,
) -> Result<(), String> {
    sqlx::query("DELETE FROM game_objects WHERE id = ?")
        .bind(object_id)
        .execute(db)
        .await
        .map_err(|e| format!("Failed to delete object: {}", e))?;

    Ok(())
}
