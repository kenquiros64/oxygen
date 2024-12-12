use futures::TryFutureExt;
use crate::enums::error::ErrorType;
use crate::models::route::Route;
use crate::services::local::routes;
use crate::models::error::ErrorResponse as Error;

// fetch_routes fetches routes from polo db
#[tauri::command]
pub fn fetch_routes() -> Result<Vec<Route>, Error> {
    let output = routes::fetch_routes().map_err(|e| Error::new(
        ErrorType::Error,
        &format!("error while verifying credentials: {}", e.to_string()))
    )?;

    Ok(output)
}
