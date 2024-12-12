use tauri::{AppHandle, Emitter};
use crate::services::sync::routes::sync_routes;
use crate::services::sync::users::sync_users;
use crate::enums::error::ErrorType;
use crate::models::error::ErrorResponse as Error;
use crate::services::connectivity::is_internet_available;

// sync syncs routes and users from mongodb to polo db
#[tauri::command]
pub async fn sync_data(app: AppHandle) {
    // Check for internet availability
    if let Err(e) = is_internet_available().await {
        if let Err(err) = app.emit("sync-finished", e) {
            eprintln!("Failed to emit 'sync-finished' event: {}", err);
        }
        return;
    }

    // Collect errors during the sync process
    let mut errors = Vec::new();

    // Sync routes
    if let Err(e) = sync_routes().await {
        errors.push(format!("Error while syncing routes: {}", e));
    }

    // Sync users
    if let Err(e) = sync_users().await {
        errors.push(format!("Error while syncing users: {}", e));
    }

    // Emit the final result based on errors
    if errors.is_empty() {
        if let Err(err) = app.emit("sync-finished", None as Option<Error>) {
            eprintln!("Failed to emit 'sync-finished' event: {}", err);
        }
    } else {
        let error_message = errors.join("; ");
        if let Err(err) = app.emit("sync-finished", Error::new(ErrorType::Error, &error_message)) {
            eprintln!("Failed to emit 'sync-finished' event: {}", err);
        }
    }
}
