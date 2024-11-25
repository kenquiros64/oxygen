use anyhow::{Result, Context};
use chrono::Utc;
use polodb_core::{Collection, CollectionT, Database};
use polodb_core::bson::{doc, to_document};
use crate::models::user::User;

const USER_COLLECTION: &str = "users";

// UserRepository is the repository for the users collection
pub struct UserRepository {
    coll: Collection<User>
}

impl UserRepository{
    // new creates a new instance of the UserRepository
    pub fn new(db: &Database) -> Self {
        let coll = db.collection(USER_COLLECTION);
        UserRepository { coll }
    }

    // add_user add a new user in the database
    pub fn add_user(&self, user: User) -> Result<()> {
        // let hashed_password = bcrypt::hash(user.password, bcrypt::DEFAULT_COST).context("Failed to hash password")?;
    
        let new_user = User {
            id: user.id,
            username: user.username,
            password: user.password,
            name: user.name,
            role: user.role,
            created_at: Utc::now().to_rfc3339(),
            last_update: Utc::now().to_rfc3339(),
        };
    
        self.coll.insert_one(new_user).context("Failed to insert user")?;
    
        Ok(())
    }
    
    // fetch_users fetches all users from the database
    pub fn fetch_users(&self) -> Result<Vec<User>> {
        let mut cursor = self.coll.find(doc! {}).run()?;

        let mut users = Vec::new();

        while let Some(value) = cursor.next() {
            let user = value.context("Failed to get user")?;
            users.push(user);
        }

        Ok(users)
    }

    // update_user updates a user in the database
    pub fn update_user(&self, mut user: User) -> Result<(), String> {
        let hashed_password = bcrypt::hash(user.password, bcrypt::DEFAULT_COST).map_err(|e| e.to_string())?;
        user.password = hashed_password;
        
        self.coll.update_one(doc! { "username": &user.username }, doc! { "$set": to_document(&user).unwrap() }).expect("Failed to update user");

        Ok(())
    }

    pub fn clear(&self) -> Result<()> {
        self.coll.delete_many(doc! {})?;
        Ok(())
    }
}