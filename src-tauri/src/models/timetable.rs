use serde::{Deserialize, Serialize};
use crate::models::time::Time;

#[derive(Debug, Serialize, Deserialize)]
pub struct Timetable {
    pub id: Option<i32>,
    pub times : Vec<Time>,
}