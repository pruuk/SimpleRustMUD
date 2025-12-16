// Database schema creation

use sqlx::SqlitePool;

pub async fn initialize_database(db: &SqlitePool) -> Result<(), Box<dyn std::error::Error>> {
    // Create players table
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS players (
            id TEXT PRIMARY KEY,
            username TEXT UNIQUE NOT NULL,
            password_hash TEXT NOT NULL,
            current_location TEXT NOT NULL,
            created_at INTEGER NOT NULL
        )
        "#,
    )
    .execute(db)
    .await?;

    // Create game_objects table
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS game_objects (
            id TEXT PRIMARY KEY,
            name TEXT NOT NULL,
            description TEXT NOT NULL,
            object_type TEXT NOT NULL,
            container_id TEXT,
            properties TEXT NOT NULL,
            created_at INTEGER NOT NULL,
            FOREIGN KEY (container_id) REFERENCES game_objects(id)
        )
        "#,
    )
    .execute(db)
    .await?;

    // Create starting room if it doesn't exist
    create_starting_room(db).await?;

    // create exits table for room exits
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS room_exits (
            room_id TEXT NOT NULL,
            direction TEXT NOT NULL,
            destination_id TEXT NOT NULL,
            PRIMARY KEY (room_id, direction),
            FOREIGN KEY (room_id) REFERENCES game_objects(id),
            FOREIGN KEY (destination_id) REFERENCES game_objects(id)
        )
        "#,
    )
    .execute(db)
    .await?;    

    Ok(())
}

async fn create_starting_room(db: &SqlitePool) -> Result<(), Box<dyn std::error::Error>> {
    let start_room_id = "room_start";
    let room_exists: Option<(i64,)> = sqlx::query_as(
        "SELECT COUNT(*) FROM game_objects WHERE id = ?"
    )
    .bind(start_room_id)
    .fetch_optional(db)
    .await?;

    if room_exists.map(|r| r.0).unwrap_or(0) == 0 {
        sqlx::query(
            r#"
            INSERT INTO game_objects (id, name, description, object_type, container_id, properties, created_at)
            VALUES (?, ?, ?, ?, ?, ?, ?)
            "#,
        )
        .bind(start_room_id)
        .bind("Starting Room")
        .bind("A simple room with stone walls. The beginning of your adventure.")
        .bind("room")
        .bind(None::<String>)
        .bind("{}")
        .bind(chrono::Utc::now().timestamp())
        .execute(db)
        .await?;
    }

    Ok(())
}
