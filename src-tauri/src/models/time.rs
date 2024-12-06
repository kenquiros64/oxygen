use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Time {
    pub hour: i32,
    pub minute: i32,
}
