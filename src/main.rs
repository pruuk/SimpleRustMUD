mod config;
mod models;
mod database;
mod game;
mod network;

use config::ServerConfig;
use game::state::GameState;
use network::telnet::handle_telnet_client;
use std::sync::Arc;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = ServerConfig::default();
    let state = Arc::new(GameState::new(config.clone()).await?);

    println!("MUD Server starting...");
    println!("Telnet port: {}", config.telnet_port);
    println!("Max users: {}", config.max_users);

    let listener = TcpListener::bind(format!("0.0.0.0:{}", config.telnet_port)).await?;
    println!("Listening for telnet connections...");

    loop {
        let (stream, addr) = listener.accept().await?;
        println!("New connection from: {}", addr);
        
        let state_clone = state.clone();
        tokio::spawn(async move {
            if let Err(e) = handle_telnet_client(stream, state_clone).await {
                eprintln!("Error handling client: {}", e);
            }
        });
    }
}
