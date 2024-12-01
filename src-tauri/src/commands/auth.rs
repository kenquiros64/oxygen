use crate::models::user::User;
use crate::services::auth;
use anyhow::Context;

#[tauri::command]
pub fn login(username: &str, password: &str) -> Result<User, anyhow::Error> {
    let user = auth::login(username, password).context("Failed to login")?;
    println!("Logged in as {}", user.username);

    Ok(user)
}

#[tauri::command]
pub async fn register(user: User) -> Result<(), anyhow::Error> {
    auth::register(user.clone())
        .await
        .context("Failed to register")?;
    println!("Registered user {}", user.username);

    Ok(())
}
