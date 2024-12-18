use crate::db::mongo;
use crate::db::polo;
use crate::repository::local::user::UserRepository as PoloUsersRepository;
use crate::repository::remote::user::UserRepository as MongoUsersRepository;
use anyhow::{Context, Result};

// sync_users syncs the users from mongodb to polodb
pub async fn sync_users() -> Result<(), anyhow::Error> {
    let mongodb = mongo::connect_mongo()
        .await
        .context("Failed to connect to MongoDB")?;

    let polo = polo::connect_polo();

    let polo_repo = PoloUsersRepository::new(&polo);
    let mongo_repo = MongoUsersRepository::new(&mongodb);

    let users = mongo_repo
        .fetch_users()
        .await
        .context("Failed to fetch users from MongoDB")?;

    match polo_repo.clear().context("Failed to clear PoloDB") {
        Ok(_) => println!("PoloDB cleared successfully."),
        Err(e) => println!("Failed to clear PoloDB: {}", e),
    }

    for user in users {
        polo_repo
            .add_user(user)
            .context("Failed to add user to PoloDB")?;
    }

    println!("User synchronization completed successfully.");
    Ok(())
}
