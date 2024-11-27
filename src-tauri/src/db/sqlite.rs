
use tauri::{AppHandle, Manager};
use std::path::PathBuf;
use rusqlite::{Connection, Error};

// initialize_tables initializes the SQLite database tables
pub fn initialize_tables(connection: &Connection) -> Result<(), rusqlite::Error> {
    let create_tickets_table = r#"
        CREATE TABLE IF NOT EXISTS tickets (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            departure TEXT NOT NULL,
            destination TEXT NOT NULL,
            stop TEXT NOT NULL,
            username TEXT NOT NULL,
            fare REAL NOT NULL,
            time TEXT NOT NULL,
            is_gold BOOLEAN NOT NULL,
            is_null BOOLEAN NOT NULL,
            report_id INTEGER NOT NULL,
            created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
            updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
            FOREIGN KEY (report_id) REFERENCES reports (id)
        );
    "#;

    let create_reports_table = r#"
        CREATE TABLE IF NOT EXISTS reports (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            username TEXT NOT NULL,
            created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
            partial_closed_at TIMESTAMP,
            closed_at TIMESTAMP
        );
    "#;

    connection.execute(create_tickets_table, [])?;
    connection.execute(create_reports_table, [])?;

    println!("All tables initialized successfully.");
    Ok(())
}

// get_db_path returns the path to the SQLite database file
pub fn get_db_path(app_handle: &AppHandle) -> PathBuf {
    // Development mode path
    if cfg!(debug_assertions) {
        let mut dev_path = std::env::current_dir().expect("failed to get current directory");
        dev_path.push("db");
        dev_path.push("oxygen.db");

        if let Some(parent) = dev_path.parent() {
            std::fs::create_dir_all(parent).expect("failed to create development database directory");
        } else {
            panic!("Invalid path: no parent directory found for development database path");
        }

        return dev_path;
    }

    // Production mode path
    let path_resolver = app_handle.path();
    let mut prod_path = path_resolver
        .app_local_data_dir()
        .expect("failed to determine local data directory");

    prod_path.push("oxygen");
    prod_path.push("oxygen.db");

    if let Some(parent) = prod_path.parent() {
        std::fs::create_dir_all(parent).expect("failed to create production database directory");
    } else {
        panic!("Invalid path: no parent directory found for production database path");
    }

    prod_path
}

// initialize initializes the SQLite database
pub fn initialize(app_handle: &tauri::AppHandle) -> Result<Connection, Error> {
    let db_path = get_db_path(app_handle);
    println!("Using database path: {:?}", db_path);

    if let Some(parent_dir) = db_path.parent() {
        std::fs::create_dir_all(parent_dir).expect("failed to create database directory");
    }

    let connection = Connection::open(db_path)?;

    initialize_tables(&connection)?;

    Ok(connection)
}
