use polodb_core::Database;

// connect_polo initializes the local database
pub fn connect_polo() -> Database {
    Database::open_path("polo_db").unwrap()
}