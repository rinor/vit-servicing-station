use super::settings::ServiceSettings;
use std::net::SocketAddr;
use std::str::FromStr;

impl Default for ServiceSettings {
    fn default() -> Self {
        Self {
            address: SocketAddr::from_str("0.0.0.0:8080").unwrap(),
            tls: None,
            cors: None,
            db_url: "./db/database.sqlite3".to_string(),
        }
    }
}
