use std::collections::HashMap;
use crate::repository::counter;
use crate::state::AppState;

#[tauri::command]
pub fn check_and_reset_counter(
    state: tauri::State<AppState>,
) {
    let db = state.sled_db.lock().unwrap();
    counter::check_and_reset_counter(&*db);
}

#[tauri::command]
pub fn increment_stop_counter(
    state: tauri::State<AppState>,
    key: String,
    qty: i32,
) -> i32 {
    let db = state.sled_db.lock().unwrap();
    counter::increment_counter(&*db, key, qty)
}

#[tauri::command]
pub fn get_stop_counters(
    state: tauri::State<AppState>,
    key: String,
) -> HashMap<String, i32> {
    let db = state.sled_db.lock().unwrap();
    counter::get_stop_counters(&*db, key)
}

#[tauri::command]
pub fn bulk_counts_by_prefixes(
    state: tauri::State<AppState>,
    prefixes: Vec<String>, // List of prefixes (e.g., ["route1-stopA-10:30", "route2-stopB-12:45"])
) -> HashMap<String, i32> {
    let db = state.sled_db.lock().unwrap();
    counter::bulk_get_counter(&*db, prefixes)
}
