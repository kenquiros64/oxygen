use crate::models::route::Route;
use crate::services::local::routes;

// fetch_routes fetches routes from polo db
#[tauri::command]
pub fn fetch_routes() -> Result<Vec<Route>, String> {
    routes::fetch_routes()
        .map_err(|e| format!("Failed to fetch routes: {}", e))
}
