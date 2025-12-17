// Player struct and methods

use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use crate::models::dice_rolls;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Player {
    pub id: String,
    pub username: String,
    pub password_hash: String,
    pub current_location: String,
    pub is_admin: i64, // use 0 or 1, SQLite doesn't have bools
    pub created_at: i64,

    // Player attributes
    pub Dexterity: i64, // Coordination and speed
    pub Strength: i64, // raw physical power
    pub Vitality: i64, // Health and resistance to physical afflictions
    pub Perception: i64, // Noticing stuff, ability to learn quickly
    pub Willpower: i64, // Mental resistance, affects health
    pub Charisma: i64, // Strength of personality, social interactions

    // Derived player stats
    pub current_health: i64,
    pub max_health: i64,
    pub current_stamina: i64,
    pub max_stamina: i64,

    // derived player stats, can be affected by items
    pub initiative: i64, // who gets to go first
    pub physical_defense: i64, // how hard player is to hit
    pub physical_armor: i64, // physical damage mitigation
    pub mystical_defense: i64, // how hard a player is to hit with mental/spell attacks
    pub mystical_armor: i64, // damage mitigation for non-physical attacks
}

impl Player {
    pub async fn new(id: String, username: String, password_hash: String, is_admin: i64) -> Self {
        Self {
            id,
            username,
            password_hash,
            current_location: "room_start".to_string(),
            is_admin,
            created_at: chrono::Utc::now().timestamp(),

            // rolled player attributes
            Dexterity: dice_rolls::random_distribution_roll_result(100.0, 10.0).await,
            Strength: dice_rolls::random_distribution_roll_result(100.0, 10.0).await,
            Vitality: dice_rolls::random_distribution_roll_result(100.0, 10.0).await,
            Perception: dice_rolls::random_distribution_roll_result(100.0, 10.0).await,
            Willpower: dice_rolls::random_distribution_roll_result(100.0, 10.0).await,
            Charisma: dice_rolls::random_distribution_roll_result(100.0, 10.0).await,

            // derived player stats, setting defaults to start
            current_health: 500,
            max_health: 500,
            current_stamina: 500,
            max_stamina: 500,
            
            initiative: 100,
            physical_defense: 100,
            physical_armor: 0,
            mystical_defense: 100,
            mystical_armor: 0,

        }
    }
}
