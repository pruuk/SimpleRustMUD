// Database schema creation

use sqlx::SqlitePool;
// use crate::database::player_queries;


pub async fn initialize_database(db: &SqlitePool) -> Result<(), Box<dyn std::error::Error>> {
    // Create players table
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS players (
            id TEXT PRIMARY KEY,
            username TEXT UNIQUE NOT NULL,
            password_hash TEXT NOT NULL,
            current_location TEXT NOT NULL,
            is_admin INTEGER DEFAULT 0,
            created_at INTEGER NOT NULL,
            dexterity INTEGER DEFAULT 100,
            strength INTEGER DEFAULT 100,
            vitality INTEGER DEFAULT 100,
            perception INTEGER DEFAULT 100,
            willpower INTEGER DEFAULT 100,
            charisma INTEGER DEFAULT 100,
            current_health INTEGER DEFAULT 500,
            max_health INTEGER DEFAULT 500,
            current_stamina INTEGER DEFAULT 500,
            max_stamina INTEGER DEFAULT 500,
            initiative INTEGER DEFAULT 100,
            physical_defense INTEGER DEFAULT 100,
            physical_armor INTEGER DEFAULT 0,
            mystical_defense INTEGER DEFAULT 100,
            mystical_armor INTEGER DEFAULT 0
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

    // Create admin if they don't exist
    create_starting_admin(db).await?;

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

async fn create_starting_admin(db: &SqlitePool) -> Result<(), Box<dyn std::error::Error>> {
    let username = "admin";
    let admin_exists: Option<(i64,)> = sqlx::query_as(
        "SELECT COUNT(*) FROM players WHERE username = ?"
    )
    .bind(username)
    .fetch_optional(db)
    .await?;

    if admin_exists.map(|r| r.0).unwrap_or(0) == 0 {
        use argon2::{
            password_hash::{
                rand_core::OsRng,
                PasswordHasher, SaltString
            },
            Argon2
        };
        let id = "admin";
        let username = "admin";
        let password = "12345";
        let salt = SaltString::generate(&mut OsRng);
        let argon2 = argon2::Argon2::default();
        let password_hash = argon2
            .hash_password(password.as_bytes(), &salt)
            .map_err(|e| format!("Password hashing failed: {}", e))?
            .to_string();
        let is_admin = 1; 
        
        sqlx::query(
            r#"
            INSERT INTO players (id, username, password_hash, current_location, is_admin, created_at)
            VALUES (?, ?, ?, ?, ?, ?)
            "#,
        )
        .bind(id)
        .bind(username)
        .bind(password_hash)
        .bind("room_start")
        .bind(is_admin)
        .bind(chrono::Utc::now().timestamp())
        .execute(db)
        .await?;
    }

    Ok(())
}


