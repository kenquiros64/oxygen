use bcrypt;
use anyhow::{Result, Context};
use chrono::Utc;
use mongodb::{Collection, Database};
use futures::stream::StreamExt;
use mongodb::bson::{doc, to_document};
use mongodb::bson::oid::ObjectId;
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

    // add_user adds a new user in the database if username does not exist
    pub async fn add_user(&self, user: User) -> Result<(), anyhow::Error> {
        let user_exists = self.coll.find_one(doc! { "username": &user.username })
            .await
            .context("Failed to check if user exists")?;

        if user_exists.is_some() {
            return Err(anyhow::anyhow!("User already exists"));
        }

        let hashed_password = bcrypt::hash(user.password, bcrypt::DEFAULT_COST).context("Failed to hash password")?;

        let new_user = User {
            id: Some(ObjectId::new()),
            username: user.username,
            password: hashed_password,
            name: user.name,
            role: user.role,
            created_at: Utc::now().to_rfc3339(),
            last_update: Utc::now().to_rfc3339(),
        };

        self.coll.insert_one(new_user)
            .await
            .context("Failed to insert user")?;

        Ok(())
    }
    
    // fetch_users fetches all users from the database
    pub async fn fetch_users(&self) -> Result<Vec<User>, anyhow::Error> {
        let mut cursor = self.coll.find(doc! {})
            .await?;

            let mut users = Vec::new();

            while let Some(user) = cursor.next().await {
                users.push(user.context("Failed to get user")?);
            }

        Ok(users)
    }


    // update_user updates a user in the database
    pub async fn update_user(&self, mut user: User) -> Result<(), anyhow::Error> {
        let hashed_password = bcrypt::hash(user.password, bcrypt::DEFAULT_COST).context("Failed to hash password")?;
        user.password = hashed_password;
        
        self.coll.update_one(doc! { "username": &user.username }, doc! { "$set": to_document(&user).unwrap() })
            .await
            .context("Failed to update user")?;

        Ok(())
    }
}