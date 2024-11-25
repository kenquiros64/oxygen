use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Time {
    pub id: Option<i32>,
    pub hour: i32,
    pub minute: i32,
}