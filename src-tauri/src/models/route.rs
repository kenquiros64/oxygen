use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};
use crate::models::stop::Stop;
use crate::models::timetable::Timetable;

#[derive(Debug, Serialize, Deserialize)]
pub struct Route {
    #[serde(rename = "_id")] // MongoDB uses "_id" for unique identifiers
    pub id: Option<ObjectId>,     // `id` is optional because it is auto-incremented
    pub departure: String,           
    pub destination: String,     
    pub stops: Vec<Stop>,
    pub timetable: Timetable,
    pub holiday_timetable: Timetable,
}