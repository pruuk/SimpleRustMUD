// Command processing

use crate::game::GameState;
use crate::database::player_queries;
use std::sync::Arc;

pub async fn process_command(state: Arc<GameState>, player_id: &str, cmd: &str) -> String {
    let parts: Vec<&str> = cmd.trim().split_whitespace().collect();
    if parts.is_empty() {
        return String::new();
    }

    match parts[0].to_lowercase().as_str() {
        "look" => handle_look(state, player_id).await,
        "say" => handle_say(state, player_id, &parts).await,
        "inventory" | "inv" => handle_inventory(state, player_id).await,
        "help" => handle_help().await,
        "quit" => "Goodbye!\n".to_string(),
        _ => "Unknown command. Type 'help' for available commands.\n".to_string(),
    }
}

async fn handle_look(state: Arc<GameState>, player_id: &str) -> String {
    let player = player_queries::get_player_by_id(&state.db, player_id)
        .await
        .unwrap();

    let room = state.get_room(&player.current_location).await.unwrap();
    let objects = state.get_objects_in_container(&room.id).await.unwrap();
    let players = state.get_players_in_room(&room.id).await.unwrap();

    let mut response = format!("{}\n{}\n\n", room.name, room.description);
    
    if !objects.is_empty() {
        response.push_str("You see:\n");
        for obj in objects {
            response.push_str(&format!("  - {}\n", obj.name));
        }
    }

    let other_players: Vec<_> = players.iter()
        .filter(|p| p.id != player_id)
        .collect();
    
    if !other_players.is_empty() {
        response.push_str("\nPlayers here:\n");
        for p in other_players {
            response.push_str(&format!("  - {}\n", p.username));
        }
    }

    response
}

async fn handle_say(state: Arc<GameState>, player_id: &str, parts: &[&str]) -> String {
    if parts.len() < 2 {
        return "Say what?\n".to_string();
    }
    
    let message = parts[1..].join(" ");
    let player = player_queries::get_player_by_id(&state.db, player_id)
        .await
        .unwrap();

    let broadcast_msg = format!("{} says: {}\n", player.username, message);
    let _ = state.broadcast_tx.send(broadcast_msg.clone());
    broadcast_msg
}

async fn handle_inventory(state: Arc<GameState>, player_id: &str) -> String {
    let items = state.get_objects_in_container(player_id).await.unwrap();
    if items.is_empty() {
        "Your inventory is empty.\n".to_string()
    } else {
        let mut response = "Inventory:\n".to_string();
        for item in items {
            response.push_str(&format!("  - {}: {}\n", item.name, item.description));
        }
        response
    }
}

async fn handle_help() -> String {
    "Available commands:\n\
     - look: Examine your surroundings\n\
     - say <message>: Speak to others in the room\n\
     - inventory/inv: Check your inventory\n\
     - quit: Exit the game\n\
     - help: Show this message\n".to_string()
}
