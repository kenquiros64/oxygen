use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Report {
    pub id: i32,
    pub username: String,
    pub created_at: String,
    pub partial_closed_at: String,
    pub closed_at: String,
}
