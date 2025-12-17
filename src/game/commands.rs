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
        "north" | "n" => handle_move(state, player_id, "north").await,
        "south" | "s" => handle_move(state, player_id, "south").await,
        "east" | "e" => handle_move(state, player_id, "east").await,
        "west" | "w" => handle_move(state, player_id, "west").await,
        "up" | "u" => handle_move(state, player_id, "up").await,
        "down" | "d" => handle_move(state, player_id, "down").await,
        "say" => handle_say(state, player_id, &parts).await,
        "inventory" | "inv" => handle_inventory(state, player_id).await,
        "@dig" => handle_admin_dig(state, player_id, &parts).await,
        "@create" => handle_admin_create(state, player_id, &parts).await,
        "@desc" => handle_admin_desc(state, player_id, &parts).await,
        "help" => handle_help().await,
        "quit" => "Goodbye!\n".to_string(),
        _ => "Unknown command. Type 'help' for available commands.\n".to_string(),
    }
}

// admin commands
async fn handle_admin_dig(state: Arc<GameState>, player_id: &str, parts: &[&str]) -> String {
    let player = player_queries::get_player_by_id(&state.db, player_id)
        .await
        .unwrap();
    
    if player.is_admin == 0 {
        return "You don't have permission to do that.\n".to_string();
    }
    
    if parts.len() < 4 {
        return "Usage: @dig <direction> <room_name> <room_description>\n".to_string();
    }
    
    let direction_input = parts[1];
    let room_name = parts[2];
    let room_desc = parts[3..].join(" ");

    // parse direction to ensure full string in room exits
    let direction = match direction_input {
        "north" | "n" => "north",
        "south" | "s" => "south",
        "east" | "e" => "east",
        "west" | "w" => "west",
        "up" | "u" => "up",
        "down" | "d" => "down",
        _ => "enter",
    };
    
    // Create new room
    let new_room = state.create_object(room_name, &room_desc, "room", None).await.unwrap();
    
    // Add exit from current room to new room
    state.add_exit(&player.current_location, direction, &new_room.id).await.unwrap();
    
    // Add return exit
    let opposite = match direction {
        "north" | "n" => "south",
        "south" | "s" => "north",
        "east" | "e" => "west",
        "west" | "w" => "east",
        "up" | "u" => "down",
        "down" | "d" => "up",
        _ => "back",
    };
    state.add_exit(&new_room.id, opposite, &player.current_location).await.unwrap();
    
    format!("Room created! Exit '{}' added.\n", direction)
}

async fn handle_admin_create(state: Arc<GameState>, player_id: &str, parts: &[&str]) -> String {
    let player = player_queries::get_player_by_id(&state.db, player_id)
        .await
        .unwrap();
    
    if player.is_admin == 0 {
        return "You don't have permission to do that.\n".to_string();
    }
    
    if parts.len() < 3 {
        return "Usage: @create <item_name> <description>\n".to_string();
    }
    
    let item_name = parts[1];
    let item_desc = parts[2..].join(" ");
    
    // Create item in current room
    state.create_object(item_name, &item_desc, "item", Some(&player.current_location)).await.unwrap();
    
    format!("Created '{}'.\n", item_name)
}

async fn handle_admin_desc(state: Arc<GameState>, player_id: &str, parts: &[&str]) -> String {
    let player = player_queries::get_player_by_id(&state.db, player_id)
        .await
        .unwrap();
    
    if player.is_admin == 0 {
        return "You don't have permission to do that.\n".to_string();
    }
    
    if parts.len() < 2 {
        return "Usage: @desc <new description>\n".to_string();
    }
    
    let new_desc = parts[1..].join(" ");
    
    sqlx::query("UPDATE game_objects SET description = ? WHERE id = ?")
        .bind(new_desc)
        .bind(&player.current_location)
        .execute(&state.db)
        .await
        .unwrap();
    
    "Room description updated.\n".to_string()
}

// regular commands
async fn handle_look(state: Arc<GameState>, player_id: &str) -> String {
    let player = player_queries::get_player_by_id(&state.db, player_id)
        .await
        .unwrap();
    let room = state.get_room(&player.current_location).await.unwrap();
    let objects = state.get_objects_in_container(&room.id).await.unwrap();
    let players = state.get_players_in_room(&room.id).await.unwrap();
    let exits = state.get_exits(&room.id).await.unwrap();

    let mut response = format!("{}\n{}\n", room.name, room.description);
    
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

    // Show exits
    if !exits.is_empty() {
        response.push_str("Exits: ");
        let exit_list: Vec<String> = exits.iter().map(|(dir, _)| dir.clone()).collect();
        response.push_str(&exit_list.join(", "));
        response.push_str("\n\n");
    }
    // Show an empty line if no exits
    if exits.is_empty() {
        response.push_str("Exits: None");
        response.push_str("\n\n");
    }

    response
}

async fn handle_move(state: Arc<GameState>, player_id: &str, direction: &str) -> String {
    let player = player_queries::get_player_by_id(&state.db, player_id)
        .await
        .unwrap();
    let room = state.get_room(&player.current_location).await.unwrap();
    let exits = state.get_exits(&room.id).await.unwrap();
    
    if let Some((_, dest)) = exits.iter().find(|(dir, _)| dir == direction) {
        state.move_player_to_room(player_id, dest).await.unwrap();
        
        // Notify others in old room
        let msg = format!("{} leaves {}.\n", player.username, direction);
        let _ = state.broadcast_tx.send(msg);
        
        // Auto-look in new room
        handle_look(state.clone(), player_id).await
        // process_command(state.clone(), player_id, "look").await
    } else {
        format!("You can't go that way.\n")
    }
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
     - move: type in a direction such as 'west' or 'w' if an exit exists\n\
     - say <message>: Speak to others in the room\n\
     - inventory/inv: Check your inventory\n\
     - quit: Exit the game\n\
     - help: Show this message\n
     
     Admin ONLY commands:\n\
     - @dig: Creates a new room. Usage: @ dig <direction> <room_name> <room_description>\n\
     - @create: Creates a new object. Usage: @create <item_name> <description>\n\
     - #desc: Updates current room's description. Usage: @desc <description>\n".to_string()

}
