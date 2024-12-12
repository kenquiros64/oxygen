pub mod commands;
pub mod db;
pub mod models;
pub mod repository;
pub mod services;
pub mod state;
pub mod enums;

use crate::db::mongo;
use state::AppState;
use std::sync::Mutex;
use tauri::async_runtime::spawn;
use tauri::Manager;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_notification::init())
        .plugin(tauri_plugin_store::Builder::new().build())
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
                // if let Err(e) = services::sync::routes::sync_routes().await {
                //     eprintln!("Error syncing routes: {:#}", e);
                // }
                // let mongodb = mongo::connect_mongo()
                //     .await
                //     .expect("Failed to connect to MongoDB");

                // let repo = repository::remote::user::UserRepository::new(&mongodb);
                // if let Err(e) = repo.add_user(
                //     models::user::User {
                //         id: ObjectId::new().into(),
                //         username: "admin".to_string(),
                //         password: "test".to_string(),
                //         name: "admin".to_string(),
                //         role: "admin".to_string(),
                //         created_at: chrono::Utc::now().to_rfc3339(),
                //         last_update: chrono::Utc::now().to_rfc3339(),
                //     }
                // ).await {
                //     eprintln!("Error adding route: {:?}", e);
                // }

                // let routerepo = repository::remote::route::RoutesRepository::new(&mongodb);
                // if let Err(e) = routerepo.add_route(models::route::Route {
                //     id: Option::from(ObjectId::new()),
                //     departure: "Squires".to_string(),
                //     destination: "La Alegría".to_string(),
                //     stops: vec![
                //         Stop { name: "Stop 1".to_string(), code: "1".to_string(), fare: 100, gold_fare: 0, is_main: false },
                //         Stop { name: "Stop 2".to_string(), code: "2".to_string(), fare: 150, gold_fare: 0, is_main: false },
                //         Stop { name: "Stop 3".to_string(), code: "3".to_string(), fare: 200, gold_fare: 100, is_main: true },
                //     ],
                //     timetable: vec![
                //         Time { hour: 10, minute: 30 },
                //         Time { hour: 12, minute: 50 },
                //         Time { hour: 1, minute: 50 },
                //     ],
                //     holiday_timetable:vec![
                //         Time { hour: 10, minute: 30 },
                //         Time { hour: 12, minute: 40 },
                //         Time { hour: 1, minute: 50 },
                //     ],
                // }).await {
                //     eprintln!("Error adding route: {:#}", e);
                // }
            });

            Ok(())
        })
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![
            // SYNC
            commands::sync::sync_data,
            // ROUTES
            commands::routes::fetch_routes,

            // COUNTER
            commands::counter::check_and_reset_counter,
            commands::counter::get_stop_counters,
            commands::counter::increment_stop_counter,
            commands::counter::bulk_counts_by_prefixes,

            // AUTH
            commands::auth::login,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
