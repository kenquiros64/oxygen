use rusqlite::{Connection, Error};
use tauri::{AppHandle, Manager};
use std::path::PathBuf;

// initialize_tables initializes the SQLite database tables
pub fn initialize_tables(connection: &Connection) -> Result<(), rusqlite::Error> {
    let create_users_table = r#"
        CREATE TABLE IF NOT EXISTS users (
            username TEXT PRIMARY KEY,
            password TEXT NOT NULL,
            name TEXT NOT NULL,
            role TEXT NOT NULL
        );
    "#;

    let create_routes_table = r#"
        CREATE TABLE IF NOT EXISTS routes (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            departure TEXT NOT NULL,
            destination TEXT NOT NULL,
            timetable INTEGER NOT NULL,
            holiday_timetable INTEGER NOT NULL,
            FOREIGN KEY (timetable) REFERENCES timetables (id),
            FOREIGN KEY (holiday_timetable) REFERENCES timetables (id)
        );
    "#;

    let create_stops_table = r#"
        CREATE TABLE IF NOT EXISTS stops (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            route_id INTEGER NOT NULL,
            name TEXT NOT NULL,
            code TEXT UNIQUE NOT NULL,
            position INTEGER NOT NULL,
            fare INTEGER NOT NULL,
            gold_fare INTEGER NOT NULL,
            is_main BOOLEAN NOT NULL,
            FOREIGN KEY (route_id) REFERENCES routes (id)
        );
    "#;

    let create_tickets_table = r#"
        CREATE TABLE IF NOT EXISTS tickets (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            username TEXT NOT NULL,
            route_name TEXT NOT NULL,
            stop_name TEXT NOT NULL,
            fare REAL NOT NULL,
            is_gold BOOLEAN NOT NULL,
            time TEXT NOT NULL,
            is_null BOOLEAN NOT NULL,
            created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
            updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
            report_id INTEGER NOT NULL,
            FOREIGN KEY (username) REFERENCES users (username),
            FOREIGN KEY (report_id) REFERENCES reports (id)
        );
    "#;

    let create_reports_table = r#"
        CREATE TABLE IF NOT EXISTS reports (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            user TEXT NOT NULL,
            created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
            updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
            FOREIGN KEY (user) REFERENCES users (username)
        );
    "#;

    let create_timetables_table = r#"
        CREATE TABLE IF NOT EXISTS timetables (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT NOT NULL
        );
    "#;

    let create_times_table = r#"
        CREATE TABLE IF NOT EXISTS times (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            hour INTEGER NOT NULL,
            minute INTEGER NOT NULL
        );
    "#;

    let create_report_tickets_table = r#"
        CREATE TABLE IF NOT EXISTS report_tickets (
            report_id INTEGER NOT NULL,
            ticket_id INTEGER NOT NULL,
            FOREIGN KEY (report_id) REFERENCES reports (id),
            FOREIGN KEY (ticket_id) REFERENCES tickets (id)
        );
    "#;

    connection.execute(create_users_table, [])?;
    connection.execute(create_routes_table, [])?;
    connection.execute(create_stops_table, [])?;
    connection.execute(create_tickets_table, [])?;
    connection.execute(create_reports_table, [])?;
    connection.execute(create_timetables_table, [])?;
    connection.execute(create_times_table, [])?;
    connection.execute(create_report_tickets_table, [])?;

    println!("All tables initialized successfully.");
    Ok(())
}

// get_db_path returns the path to the SQLite database file
pub fn get_db_path(app_handle: &AppHandle) -> PathBuf {
    // Development mode path
    if cfg!(debug_assertions) {
        let mut dev_path = std::env::current_dir().expect("failed to get current directory");
        dev_path.push("dev_data");
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
