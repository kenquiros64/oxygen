use crate::models::user::User;
use anyhow::{Context, Result};
use polodb_core::bson::{doc, to_document};
use polodb_core::{Collection, CollectionT, Database};

const USER_COLLECTION: &str = "users";

// UserRepository is the repository for the users collection
pub struct UserRepository {
    coll: Collection<User>,
}

impl UserRepository {
    // new creates a new instance of the UserRepository
    pub fn new(db: &Database) -> Self {
        let coll = db.collection(USER_COLLECTION);
        UserRepository { coll }
    }

    // add_user add a new user in the database
    pub fn add_user(&self, user: User) -> Result<(), anyhow::Error> {
        self.coll
            .insert_one(user)
            .context("Failed to insert user")?;

        Ok(())
    }

    // find_user finds a user in the database
    pub fn find_user(&self, username: &str) -> Result<Option<User>> {
        let user = self.coll.find_one(doc! { "username": username })?;
        Ok(user)
    }

    // fetch_users fetches all users from the database
    pub fn fetch_users(&self) -> Result<Vec<User>, anyhow::Error> {
        let mut cursor = self.coll.find(doc! {}).run()?;

        let mut users = Vec::new();

        while let Some(value) = cursor.next() {
            let user = value.context("Failed to get user")?;
            users.push(user);
        }

        Ok(users)
    }

    // update_user updates a user in the database
    pub fn update_user(&self, user: User) -> Result<(), anyhow::Error> {
        self.coll
            .update_one(
                doc! { "username": &user.username },
                doc! { "$set": to_document(&user).unwrap() },
            )
            .context("Failed to update user")?;

        Ok(())
    }

    // clear removes all users from the database
    pub fn clear(&self) -> Result<()> {
        self.coll.delete_many(doc! {})?;
        Ok(())
    }
}
