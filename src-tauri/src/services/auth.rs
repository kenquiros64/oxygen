use crate::db::{mongo, polo};
use crate::models::user::User;
use anyhow::Context;
use bcrypt;
use futures::TryFutureExt;
use crate::enums::error::ErrorType;

// login logs in a user
pub fn login(username: &str, password: &str) -> Result<User, ErrorType> {
    // Attempt to connect to the database
    let polo = polo::connect_polo();

    let polo_repo = crate::repository::local::user::UserRepository::new(&polo);

    // Find the user in the repository
    let user = polo_repo
        .find_user(username)
        .map_err(|e| ErrorType::Error(e.to_string()))?;

    if user.is_none() {
        return Err(ErrorType::UserNotFound);
    }

    let user = user.unwrap();

    // Verify the password
    println!("PASSWORD: {}", &user.password);
    let verified = bcrypt::verify(password, &user.password)
        .map_err(|e| ErrorType::Error(e.to_string()))?;
    println!("VERIFIED: {}", verified);

    if !verified {
        return Err(ErrorType::InvalidPassword);
    }

    Ok(user)
}

// register registers a new user
pub async fn register(user: User) -> Result<(), ErrorType> {
    let mongodb = mongo::connect_mongo()
        .await
        .map_err(|e| ErrorType::Error(e.to_string()))?;
    let mongo_repo = crate::repository::remote::user::UserRepository::new(&mongodb);

    // Connect to Polo
    let polo = polo::connect_polo();
    let polo_repo = crate::repository::local::user::UserRepository::new(&polo);

    // Add user to MongoDB
    if let Err(e) = mongo_repo.add_user(user.clone()).await {
        eprintln!("Failed to add user to MongoDB: {}", e);
        return Err(e);
    }
    println!("User added to MongoDB successfully.");

    // Add user to Polo
    polo_repo
        .add_user(user)
        .map_err(|e| ErrorType::Error(e.to_string()))?;

    println!("User added to Polo successfully.");
    Ok(())
}
