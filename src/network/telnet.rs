// Telnet server handling

use crate::game::{GameState, process_command};
use crate::models::Session;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::net::TcpStream;
use tokio::sync::mpsc;
use std::sync::Arc;

pub async fn handle_telnet_client(
    stream: TcpStream,
    state: Arc<GameState>,
) -> Result<(), Box<dyn std::error::Error>> {
    let (reader, mut writer) = stream.into_split();
    let mut reader = BufReader::new(reader);

    {
        let sessions = state.sessions.read().await;
        if sessions.len() >= state.config.max_users {
            writer.write_all(b"Server full. Try again later.\n").await?;
            return Ok(());
        }
    }

    if let Err(e) = writer.write_all(b"Welcome to the MUD!\n").await {
        eprintln!("Failed to send welcome: {}", e);
        return Err(e.into());
    }
    
    if let Err(e) = writer.write_all(b"Login (L) or Register (R)? ").await {
        eprintln!("Failed to send prompt: {}", e);
        return Err(e.into());
    }
    
    if let Err(e) = writer.flush().await {
        eprintln!("Failed to flush: {}", e);
        return Err(e.into());
    }

    let mut choice = String::new();
    if let Err(e) = reader.read_line(&mut choice).await {
        eprintln!("Failed to read choice: {}", e);
        return Err(e.into());
    }
    
    println!("Client chose: {}", choice.trim());

    let player = match choice.trim().to_uppercase().as_str() {
        "L" => {
            writer.write_all(b"Username: ").await?;
            writer.flush().await?;
            let mut username = String::new();
            reader.read_line(&mut username).await?;

            writer.write_all(b"Password: ").await?;
            writer.flush().await?;
            let mut password = String::new();
            reader.read_line(&mut password).await?;

            match state.authenticate(username.trim(), password.trim()).await {
                Ok(p) => p,
                Err(e) => {
                    writer.write_all(format!("Error: {}\n", e).as_bytes()).await?;
                    return Ok(());
                }
            }
        }
        "R" => {
            writer.write_all(b"Choose username: ").await?;
            writer.flush().await?;
            let mut username = String::new();
            reader.read_line(&mut username).await?;

            writer.write_all(b"Choose password: ").await?;
            writer.flush().await?;
            let mut password = String::new();
            reader.read_line(&mut password).await?;

            match state.register_player(username.trim(), password.trim()).await {
                Ok(p) => {
                    writer.write_all(b"Registration successful!\n").await?;
                    p
                }
                Err(e) => {
                    writer.write_all(format!("Error: {}\n", e).as_bytes()).await?;
                    return Ok(());
                }
            }
        }
        _ => {
            writer.write_all(b"Invalid choice.\n").await?;
            return Ok(());
        }
    };

    writer.write_all(format!("Welcome, {}!\n", player.username).as_bytes()).await?;

    let (tx, mut rx) = mpsc::unbounded_channel();
    let session_id = player.id.clone();
    
    {
        let mut sessions = state.sessions.write().await;
        sessions.insert(
            session_id.clone(),
            Session::new(player.id.clone(), player.username.clone(), tx),
        );
    }

    let look_result = process_command(state.clone(), &player.id, "look").await;
    writer.write_all(look_result.as_bytes()).await?;
    writer.flush().await?;

    let mut line = String::new();
    loop {
        tokio::select! {
            result = reader.read_line(&mut line) => {
                match result {
                    Ok(0) | Err(_) => break,
                    Ok(_) => {
                        if line.trim() == "quit" {
                            break;
                        }
                        
                        let response = process_command(state.clone(), &player.id, &line).await;
                        writer.write_all(response.as_bytes()).await?;
                        writer.flush().await?;
                        line.clear();
                    }
                }
            }
            Some(msg) = rx.recv() => {
                writer.write_all(msg.as_bytes()).await?;
                writer.flush().await?;
            }
        }
    }

    {
        let mut sessions = state.sessions.write().await;
        sessions.remove(&session_id);
    }

    Ok(())
}
