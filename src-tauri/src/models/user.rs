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
