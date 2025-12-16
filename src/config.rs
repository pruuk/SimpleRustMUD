// Configuration structs

#[derive(Clone)]
pub struct ServerConfig {
    pub max_users: usize,
    pub database_url: String,
    pub telnet_port: u16,
}

impl Default for ServerConfig {
    fn default() -> Self {
        Self {
            max_users: 100,
            database_url: "sqlite://./mud.db".to_string(),
            telnet_port: 4000,
        }
    }
}
