use crate::models::time::Time;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Timetable {
    pub id: Option<i32>,
    pub times: Vec<Time>,
}
