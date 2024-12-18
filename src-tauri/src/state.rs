use rusqlite::Connection;
use sled::Db;
use std::sync::Mutex;

// Define the AppState struct to hold app state
pub struct AppState {
    pub db: Mutex<Connection>,
    pub sled_db: Mutex<Db>,
}
