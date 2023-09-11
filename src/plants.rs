use chrono::offset::Utc;
use serde::{Deserialize, Serialize};
use serde_json;

pub trait Plantable: Clone {}

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone, Copy)]
pub enum PlantTypes {
    None,
    Zuccini,
    Peach,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Plant {
    pub id: i32,
    pub name: String,
    pub plant_type: PlantTypes,
    pub planted_timestamp: i64,
    pub growth_time_sec: i64,
    pub harvested_timestamp: i64,
}

impl Plant {
    pub fn default() -> Self {
        Plant {
            id: -1,
            name: "".to_string(),
            plant_type: PlantTypes::None,
            planted_timestamp: -1,
            growth_time_sec: -1,
            harvested_timestamp: -1,
        }
    }
}

impl Plantable for Plant {}
