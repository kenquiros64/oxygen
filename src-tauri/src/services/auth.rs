use anyhow::Context;
use crate::db::{mongo, polo};
use crate::models::user::User;
use bcrypt;

// login logs in a user
pub fn login(username: &str, password: &str) -> Result<User, anyhow::Error> {
    let polo = polo::connect_polo();

    let polo_repo = crate::repository::local::user::UserRepository::new(&polo);

    let user = polo_repo.find_user(username).unwrap();

    if user.is_none() {
        return Err(anyhow::anyhow!("User not found"));
    }

    let user = user.unwrap();

    let verified = bcrypt::verify(password, &user.password).context("Failed to verify password")?;
    if !verified {
        return Err(anyhow::anyhow!("Invalid password"));
    }

    Ok(user)
}

// register registers a new user
pub async fn register(user: User) -> Result<(), anyhow::Error> {
    let mongodb = mongo::connect_mongo()
        .await
        .context("Failed to connect to MongoDB")?;
    let mongo_repo = crate::repository::remote::user::UserRepository::new(&mongodb);

    // Connect to Polo
    let polo = polo::connect_polo();
    let polo_repo = crate::repository::local::user::UserRepository::new(&polo);

    // Add user to MongoDB
    if let Err(e) = mongo_repo.add_user(user.clone()).await {
        eprintln!("Failed to add user to MongoDB: {}", e);
        return Err(e).context("Adding user to MongoDB failed");
    }
    println!("User added to MongoDB successfully.");

    // Add user to Polo
    polo_repo
        .add_user(user)
        .context("Failed to add user to Polo")?;

    println!("User added to Polo successfully.");
    Ok(())
}