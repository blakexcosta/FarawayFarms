use chrono::offset::Utc;
use serde::{Deserialize, Serialize};
use serde_json;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum PlantTypes {
    None,
    Zuccini,
    Peach,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Plant {
    pub id: i32,
    pub name: String,
    pub plant_type: PlantTypes,
    pub planted_timestamp: i64,
    pub growth_time_sec: i64,
    pub harvested_timestamp: i64,
}
