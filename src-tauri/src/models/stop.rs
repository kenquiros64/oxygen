use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Stop {
    pub name: String,
    pub code: String,
    pub fare: i32,
    pub gold_fare: i32,
    pub is_main: bool,
}
