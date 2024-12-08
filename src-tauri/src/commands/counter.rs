use crate::repository::counter;
use crate::state::AppState;
use std::fmt::format;

#[tauri::command]
pub fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

// #[tauri::command]
// pub fn increment_stop_counter(
//     state: tauri::State<AppState>,
//     route_id: i32,
//     stop_id: i32,
//     hour: i32,
//     minute: i32,
// ) -> i32 {
//     let db = state.sled_db.lock().unwrap();
//     let key = format("{}-{}-{}-{}", route_id, stop_id, hour, minute);
//     // counter::increment_counter(&*db, key)
// }
//
// #[tauri::command]
// pub fn get_stop_counter(
//     state: tauri::State<AppState>,
//     route_id: i32,
//     stop_id: i32,
//     hour: i32,
//     minute: i32,
// ) -> i32 {
//     let db = state.sled_db.lock().unwrap();
//     counter::get_counter(&*db, route_id, stop_id, hour, minute)
// }
