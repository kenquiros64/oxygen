use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Stop {
    pub id: Option<i32>,
    pub name: String,
    pub code: String,
    pub fare: i32,
    pub gold_fare: i32,
    pub is_main: bool,
}
