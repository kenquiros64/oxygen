use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Ticket {
    pub id: i32,
    pub departure: String,
    pub destination: String,
    pub stop: String,
    pub username: String,
    pub fare: i32,
    pub time: String,
    pub is_gold: bool,
    pub is_null: bool,
    pub report_id: i32,
    pub created_at: String,
    pub updated_at: String,
}
