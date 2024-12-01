use crate::services::sync::routes::sync_routes;
use crate::services::sync::users::sync_users;
use anyhow::Context;

// sync syncs routes and users from mongodb to polo db
#[tauri::command]
pub async fn sync() -> Result<(), anyhow::Error> {
    sync_routes().await.context("Failed to sync routes")?;

    sync_users().await.context("Failed to sync users")?;

    println!("Sync completed successfully.");
    Ok(())
}
