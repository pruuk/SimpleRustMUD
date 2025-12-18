// Module declarations

pub mod telnet;
pub mod websocket;  // Future feature

pub use telnet::handle_telnet_client;
pub use websocket::ws_handler;
