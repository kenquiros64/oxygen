pub mod db;
pub mod models;
pub mod repository;
pub mod commands;
pub mod state;
pub mod services;

use std::sync::{Mutex};
use tauri::Manager;
use state::AppState;
use crate::db::mongo;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            let connection = match db::sqlite::initialize(&app.handle()) {
                Ok(conn) => {
                    println!("Database initialized successfully.");
                    conn
                }
                Err(e) => {
                    eprintln!("Failed to initialize database: {:?}", e);
                    return Err(e.into());
                }
            };

            let counter_db = db::sled::initialize_counter_db();



            // Check and reset counters based on the stored date
            repository::counter::check_and_reset_counter(&counter_db);

            // Set up the AppState with the initialized connection
            app.manage(AppState {
                db: Mutex::new(connection),
                sled_db: Mutex::new(counter_db.clone()),
            });

            tauri::async_runtime::spawn(async {
                // if let Err(e) = services::sync::users::sync_users().await {
                //     eprintln!("Error syncing users: {:#}", e);
                // }
                let mongodb = mongo::connect_mongo()
                    .await
                    .expect("Failed to connect to MongoDB");

                let repo = repository::remote::user::UserRepository::new(&mongodb);
                if let Err(e) = repo.add_user(models::user::User {
                    id: None,
                    username: "admin".to_string(),
                    password: "test".to_string(),
                    name: "admin".to_string(),
                    role: "admin".to_string(),
                    created_at: chrono::Utc::now().to_rfc3339(),
                    last_update: chrono::Utc::now().to_rfc3339(),
                }).await {
                    eprintln!("Error adding user: {:#}", e);
                }
            });



            Ok(())
        })
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![
            commands::counter::greet,
            commands::counter::increment_stop_counter,
            commands::counter::get_stop_counter
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
