// Websocket handler
use crate::Arc;
use crate::GameState;
use axum::{
    extract::{ws::WebSocket, WebSocketUpgrade, State},
    response::IntoResponse,
    routing::get,
    Router,
};

pub async fn ws_handler(
    ws: WebSocketUpgrade,
    State(state): State<Arc<GameState>>,
) -> impl IntoResponse {
    ws.on_upgrade(|socket| handle_websocket(socket, state))
}

pub async fn handle_websocket(socket: WebSocket, state: Arc<GameState>) {
    // Similar to handle_telnet_client but using WebSocket
    // TODO: need to adapt the read/write logic for WebSocket frames
}
