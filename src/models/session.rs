// Session struct

use tokio::sync::mpsc;

pub struct Session {
    pub player_id: String,
    pub username: String,
    pub tx: mpsc::UnboundedSender<String>,
}

impl Session {
    pub fn new(player_id: String, username: String, tx: mpsc::UnboundedSender<String>) -> Self {
        Self {
            player_id,
            username,
            tx,
        }
    }
}
