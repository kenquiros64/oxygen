use crate::models::user::User;
use crate::services::auth;
use crate::models::error::ErrorResponse;

#[tauri::command]
pub fn login(username: String, password: String) -> Result<User, ErrorResponse> {
    println!("Logging in user: {}-{}", username, password);
    auth::login(&username, &password)
}


#[tauri::command]
pub async fn register(user: User) -> Result<(), ErrorResponse> {
    auth::register(user).await
}
