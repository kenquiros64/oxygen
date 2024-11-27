use serde::{Deserialize, Serialize};
use mongodb::bson::oid::ObjectId;

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    #[serde(rename = "_id")] // MongoDB uses "_id" for unique identifiers
    pub id: Option<ObjectId>, // or Option<ObjectId> if it's nullable
    pub username: String,
    pub password: String,
    pub name: String,
    pub role: String,
    pub created_at: String,
    pub last_update: String,
}

impl User {
    pub(crate) fn clone(&self) -> User {
        User {
            id: self.id.clone(),
            username: self.username.clone(),
            password: self.password.clone(),
            name: self.name.clone(),
            role: self.role.clone(),
            created_at: self.created_at.clone(),
            last_update: self.last_update.clone(),
        }
    }
}
