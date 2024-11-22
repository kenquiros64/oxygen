mod db;
mod state;
mod commands;

use std::sync::{Mutex};
use tauri::Manager;
use state::AppState;

use db::init as init_db;


#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            let connection = match init_db::sqlite::initialize(&app.handle()) {
                Ok(conn) => {
                    println!("Database initialized successfully.");
                    conn
                }
                Err(e) => {
                    eprintln!("Failed to initialize database: {:?}", e);
                    return Err(e.into());
                }
            };


            let counter_db = init_db::sled::initialize_counter_db();

            // Check and reset counters based on the stored date
            db::counter::check_and_reset_counter(&counter_db);

            // Set up the AppState with the initialized connection
            app.manage(AppState {
                db: Mutex::new(connection),
                sled_db: Mutex::new(counter_db.clone()),
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
