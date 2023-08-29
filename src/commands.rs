use chrono::Utc;
use mongodb::bson::de;
use serde::{Deserialize, Serialize};
use serde_json;
use serde_json::{from_value, Deserializer, Value};
use std::collections::HashMap;
use std::fs;
use std::fs::File;
use std::fs::OpenOptions;
use std::io::prelude::*;
use std::io::Write;
use std::io::{BufRead, BufReader};
use std::path::Path;
use std::str::Split;

use crate::db::db_helper;
use crate::plants::Plant;
use crate::plants::PlantTypes;
use crate::{colony::Colony, coordinates::CardinalDirection};

#[derive(Debug)]
pub struct Command {
    name: String,
    description: String,
}
impl Command {
    pub fn new(_name: String, _description: String) -> Command {
        return Command {
            name: _name,
            description: _description,
        };
    }
}

/// This is where you implement and add new commands, the main program will always require this hashmap to know the current populated list of commands
/// This is really in practice only used for the help command. This has largely gone/been un-utilized due to very custom implementations.
/// This could/should change in the future as the choice gotten from the console is the key in this hashmap.
/// Command given
pub fn generate_command_hashmap() -> HashMap<String, Command> {
    let mut commands = HashMap::new();
    commands.insert(
        "help".to_string(),
        Command::new("help".to_string(), "placeholder".to_string()),
    );
    commands.insert(
        "create_colony".to_string(),
        Command::new("create_colony".to_string(), "placeholder".to_string()),
    );
    commands.insert(
        "test_db".to_string(),
        Command::new("test_db".to_string(), "placeholder".to_string()),
    );
    commands.insert(
        "citizens_info".to_string(),
        Command::new("citizens_info".to_string(), "placeholder".to_string()),
    );
    commands.insert(
        "citizen_action".to_string(),
        Command::new("citizen_action".to_string(), "placeholder".to_string()),
    );
    commands.insert(
        "buildings_info".to_string(),
        Command::new("buildings_info".to_string(), "placeholder".to_string()),
    );
    commands.insert(
        "colony_info".to_string(),
        Command::new("colony_info".to_string(), "placeholder".to_string()),
    );
    commands.insert(
        "citizen_info".to_string(),
        Command::new("citizen_info".to_string(), "placeholder".to_string()),
    );
    commands.insert(
        "market_sell".to_string(),
        Command::new("market_sell".to_string(), "placeholder".to_string()),
    );
    commands.insert(
        "market_buy".to_string(),
        Command::new("market_buy".to_string(), "placeholder".to_string()),
    );
    commands.insert(
        "market_list_orders".to_string(),
        Command::new("market_list_orders".to_string(), "placeholder".to_string()),
    );
    commands.insert(
        "market_log".to_string(),
        Command::new("market_log".to_string(), "placeholder".to_string()),
    );
    commands.insert(
        "quit".to_string(),
        Command::new("quit".to_string(), "placeholder".to_string()),
    );
    commands.insert(
        "q".to_string(),
        Command::new("q".to_string(), "placeholder".to_string()),
    );
    return commands;
}

// ---------------------------------------------------------------
// COMMAND/TERMINAL FUNCTIONALITY
// i.e. commands that change something about the terminal or provide information to the terminal
// walker, texas hamster
// goodest gobbles steve
pub fn iterate_over_commands(commands: &HashMap<String, Command>) {
    for (key, value) in commands.iter() {
        println!("\t{} - {} ", value.name, value.description);
    }
}

pub async fn colony_info(mut choice_split: Split<'_, &str>) -> String {
    // get the colony name from the trimmed information
    let mut colony_name = match choice_split.next() {
        Some(name) => name,
        None => "",
    };
    colony_name = colony_name.trim();
    if colony_name.is_empty() {
        println!("please enter a valid colony name argument");
        return String::from("");
    }

    println!("COLONY NAME GIVEN:\n{}", &colony_name);
    //connect to the colony by getting the colony-name
    let db = db_helper::get_db("colony_db").await.unwrap();
    // println!("{:?}", db.name());
    db_helper::print_colony_information(db, "colony_collection", &colony_name).await;
    return String::from("not_empty");
}

// citizens array struct only used for serializing/deserializing data
// #[derive(Serialize, Deserialize)]
// struct Citizen {
//     name: String,
//     age: u32,
//     occupation: String,
//     hp: u32,
//     hunger: u32,
//     thirst: u32,
//     x_location: i32,
//     y_location: i32,

// }
pub async fn citizen_info(mut choice_split: Split<'_, &str>) -> String {
    // get the colony name from the trimmed information
    let mut colony_name = match choice_split.next() {
        Some(name) => name,
        None => "",
    };
    colony_name = colony_name.trim();
    if colony_name.is_empty() {
        println!("please enter a valid colony name argument");
        return String::from("");
    }

    println!("INDIVIDUAL CITIZEN INFORMATION:\n{}", &colony_name);
    //connect to the colony by getting the colony-name
    let db = db_helper::get_db("colony_db").await.unwrap();
    // println!("{:?}", db.name());
    let colony_info =
        db_helper::get_colony_information(db, "colony_collection", &colony_name).await;

    for citizen_array in colony_info {
        println!("{:?}", citizen_array.get_array("citizens").unwrap());
    }

    return String::from("not_empty");
}

// ---------------------------------------------------------------
// GAME FUNCTIONALITY
// i.e. commands that change something about the game
/// Used to move a citizen, requires a colony instance
pub fn move_citizen(colony_instance: Colony, direction: CardinalDirection, step_amount: u32) {
    // match direction {
    //     CardinalDirection::NORTH => {self.citizens[0].y_location = self.citizens[0].y_location + step_amount as i32}
    //     CardinalDirection::SOUTH => {self.citizens[0].y_location = self.citizens[0].y_location - step_amount as i32}
    //     CardinalDirection::EAST => {self.citizens[0].y_location = self.citizens[0].y_location + step_amount as i32}
    //     CardinalDirection::WEST => {self.citizens[0].y_location = self.citizens[0].y_location - step_amount as i32}
    // }
    // println!("{:?}", self.citizens[0]);
}

/// Used to plant a new plant in the colony
pub fn plant(plantid: &str) -> Result<String, String> {
    // check if plantid is an integer and propogate error up
    check_is_i32(plantid)?;

    println!("plant command called with plantid: {}", plantid);
    // let mut plant: Plant;
    let plant: Plant = match plantid.trim() {
        "1" => {
            let mut plant = Plant {
                id: 1,
                name: "Zuccini".to_string(),
                plant_type: PlantTypes::Zuccini,
                planted_timestamp: Utc::now().timestamp(),
                growth_time_sec: 10,
                harvested_timestamp: Utc::now().timestamp(),
            };
            // set harvestable timestamp
            plant.harvested_timestamp = plant.harvested_timestamp + plant.growth_time_sec;
            plant
        }
        "2" => {
            let mut plant = Plant {
                id: 2,
                name: "Peach".to_string(),
                plant_type: PlantTypes::Peach,
                planted_timestamp: Utc::now().timestamp(),
                growth_time_sec: 10,
                harvested_timestamp: Utc::now().timestamp(),
            };
            plant.harvested_timestamp = plant.harvested_timestamp + plant.growth_time_sec;
            plant
        }
        _ => {
            let mut plant = Plant {
                id: 0,
                name: "".to_string(),
                plant_type: PlantTypes::None,
                planted_timestamp: 0,
                growth_time_sec: 0,
                harvested_timestamp: 0,
            };
            plant.harvested_timestamp = plant.harvested_timestamp + plant.growth_time_sec;
            plant
        }
    };

    // check if plant is null
    if plant.id == 0 || plant.plant_type == PlantTypes::None {
        println!("No plant Id exists");
        return Err("No plant Id exists".to_string());
    }

    // serialize data
    let mut serialized_plant_data = serde_json::to_string(&plant).unwrap();
    //serialized_plant_data.push(','); // add comma at end of data, not included otherwise. can cause an error when reading data though

    let dt = Utc::now();
    let timestamp: i64 = dt.timestamp();
    println!("Current timestamp is {}", timestamp);
    // write to farm.txt
    write_to_farmtxt(serialized_plant_data, "farm.txt", false);
    return Ok("".to_string());
}

/// Attempts to harvest all plant in the format of `harvest <plantid>`
/// Ex:) `harvest 1`
pub fn harvest(plantid: &str) -> Result<String, String> {
    // check if plantid is an integer and propogate error up
    check_is_i32(plantid)?;

    // inventory filename
    let inventory_filename = "inventory.txt";
    // read from farm.txt
    let mut plant_data = read_from_farmtxt("farm.txt");
    let mut plant_data = match plant_data {
        Ok(plant_data) => {
            //println!("Plant data: {:?}", plant_data);
            plant_data
        }
        Err(e) => {
            println!("Error: {:?}", e);
            vec![]
        }
    };
    // check if vector is empty
    if plant_data.is_empty() {
        println!("Error when Reading! Empty plant data vec as a result.");
        return Err("Error when Reading! Empty plant data vec as a result.".to_string());
    }
    // determine first possible, available plant if plant is harvestable
    let mut harvestable = false;
    for plant in &plant_data {
        // get first available plant that matches plantid
        if (plant.id == plantid.trim().parse::<i32>().unwrap()) {
            // check if harvestable
            if (Utc::now().timestamp() >= plant.harvested_timestamp) {
                println!("Plant is harvestable!, Harvesting...");
                harvestable = true;
                // add to inventory.txt
                write_to_inventorytxt(serde_json::to_string(&plant).unwrap(), inventory_filename);
                break; // break the loop so only writing one plant
            } else {
                println!(
                    "Not harvestable yet! Reading in: {:?} seconds",
                    plant.harvested_timestamp - Utc::now().timestamp()
                );
            }
        }
    }
    // remove plant from farm.txt.
    if harvestable {
        plant_data.remove(0); // remove plant
        let mut string_buf = String::new(); // convert plant data to string so write properly to file
        for plant in plant_data {
            // println!("{:?}", plant);
            string_buf = string_buf + &serde_json::to_string(&plant).unwrap();
        }
        // write updated data to farm.txt
        write_to_farmtxt(string_buf, "farm.txt", true);
        return Ok("".to_string());
    } else {
        return Err("Plant is not harvestable!".to_string());
    }
}

/// this is a extra helper function to check if plantid is an integer with some added fucntionality printed to screen
/// Used to call inline on function
fn check_is_i32(plantid: &str) -> Result<(), String> {
    if is_i32(plantid) == false {
        //println!("plantid is not an integer");
        return Err("plantid is not an integer".to_string());
    } else {
        return Ok(());
    }
}

/// checks is a plantid is an integer and returns a bool if not
fn is_i32(plantid: &str) -> bool {
    let val = match plantid.trim().parse::<i32>() {
        Ok(val) => val,
        Err(_) => -1,
    };

    if val == -1 {
        return false;
    } else {
        return true;
    }
}

fn write_to_farmtxt(data: String, filename: &str, overwrite: bool) {
    // if file does not exist, create it
    if !Path::new(filename).exists() {
        let mut file = File::create(filename).expect("Failed to create file");
        // write data
        // file.writeall(b"Hello, World!")
        //     .expect("Failed to write to file");
    }
    // write data to file
    // overwrite file is override flag exist
    if overwrite {
        fs::write(filename, data.as_bytes());
    } else {
        let mut file = OpenOptions::new()
            .write(true)
            .append(true)
            .open(filename)
            .unwrap();
        file.write_all(data.as_bytes())
            .expect("Failed to write to file");
    }
}

fn read_from_farmtxt(filename: &str) -> Result<Vec<Plant>, std::io::Error> {
    let mut deserialized_plants: Vec<Plant> = vec![]; // new empty vector
    let file = File::open(filename)?;
    let reader = BufReader::new(file);
    let deserializer = Deserializer::from_reader(reader).into_iter::<Value>(); // create a deserializer to iterate over deserialized values

    for value in deserializer {
        let value = value?;
        // Process each JSON object
        let mut json_struct: Plant = serde_json::from_value(value)?; // convert Value to Plant
        deserialized_plants.push(json_struct);
    }
    Ok(deserialized_plants)
}

fn write_to_inventorytxt(data: String, filename: &str) {
    // if file does not exist, create it
    if !Path::new(filename).exists() {
        let mut file = File::create(filename).expect("Failed to create file");
        // write data
        // file.writeall(b"Hello, World!")
        //     .expect("Failed to write to file");
    }

    // write data to file
    let mut file = OpenOptions::new()
        .write(true)
        .append(true)
        .open(filename)
        .unwrap();
    file.write_all(data.as_bytes())
        .expect("Failed to write to file");
}

fn read_from_inventorytxt() {}

// ----------------------------------------
#[cfg(test)]
mod tests {
    // TESTS
    use super::plant;
    use super::{harvest, read_from_farmtxt};

    // PLANT function tests
    // passing
    #[test]
    fn test_workingplant_id1() {
        let success = match plant("1") {
            Ok(str) => str,
            Err(e) => e,
        };
        assert_eq!(success, "".to_string());
    }
    #[test]
    fn test_workingplant_id2() {
        let success = match plant("2") {
            Ok(str) => str,
            Err(e) => e,
        };
        assert_eq!(success, "".to_string());
    }
    // failing
    #[test]
    fn test_failingplant_id_negative() {
        let failure = match plant("-1") {
            Ok(str) => str,
            Err(e) => e,
        };
        assert_ne!(failure, "".to_string()); // should not be identical
    }
    #[test]
    fn test_failingplant_id_large() {
        let failure = match plant("6942069") {
            Ok(str) => str,
            Err(e) => e,
        };
        assert_ne!(failure, "".to_string()); // should not be identical
    }
    #[test]
    fn plant_throws_error_on_bad_plantid() {
        let failure = match plant("a") {
            Ok(str) => str,
            Err(e) => e,
        };
        assert_ne!(failure, "".to_string()); // should not be identical
    }

    // HARVEST function tests
    // passing
    #[test]
    fn passing_harvest_1() {
        // plant plants
        match plant("1") {
            Ok(str) => str,
            Err(e) => e,
        };
        match plant("2") {
            Ok(str) => str,
            Err(e) => e,
        };
        match plant("1") {
            Ok(str) => str,
            Err(e) => e,
        };
        // test a harvest
        let success = match harvest("1") {
            Ok(str) => str,
            Err(e) => e,
        };
        assert_eq!(success, "".to_string());
    }
    #[test]
    fn passing_harvest_2() {
        // plant plants
        match plant("1") {
            Ok(str) => str,
            Err(e) => e,
        };
        match plant("2") {
            Ok(str) => str,
            Err(e) => e,
        };
        match plant("1") {
            Ok(str) => str,
            Err(e) => e,
        };
        // test a harvest
        let success = match harvest("2") {
            Ok(str) => str,
            Err(e) => e,
        };
        assert_eq!(success, "".to_string());
    }
    // failing
    #[test]
    fn failing_harvest_1() {
        // plant plants
        match plant("1") {
            Ok(str) => str,
            Err(e) => e,
        };
        match plant("2") {
            Ok(str) => str,
            Err(e) => e,
        };
        match plant("1") {
            Ok(str) => str,
            Err(e) => e,
        };
        // test a harvest
        let failure = match harvest("12341234") {
            Ok(str) => str,
            Err(e) => e,
        };
        assert_ne!(failure, "".to_string()); // it should not be identical
    }
    #[test]
    fn failing_harvest_2() {
        // plant plants
        match plant("1") {
            Ok(str) => str,
            Err(e) => e,
        };
        match plant("2") {
            Ok(str) => str,
            Err(e) => e,
        };
        match plant("1") {
            Ok(str) => str,
            Err(e) => e,
        };
        // test a harvest
        let failure = match harvest("-1") {
            Ok(str) => str,
            Err(e) => e,
        };
        assert_ne!(failure, "".to_string()); // it should not be identical
    }
    #[test]
    fn failing_harvest_3() {
        // plant plants
        match plant("1") {
            Ok(str) => str,
            Err(e) => e,
        };
        match plant("2") {
            Ok(str) => str,
            Err(e) => e,
        };
        match plant("1") {
            Ok(str) => str,
            Err(e) => e,
        };
        // test a harvest
        let failure = match harvest("-6942069") {
            Ok(str) => str,
            Err(e) => e,
        };
        assert_ne!(failure, "".to_string()); // it should not be identical
    }
    #[test]
    fn harvest_throws_error_on_bad_plantid() {
        let failure = match harvest("a") {
            Ok(str) => str,
            Err(e) => e,
        };
        assert_ne!(failure, "".to_string()); // should not be identical
    }

    // write_to_farmtxt function tests - only meant to be called in context of plant/harvest oriented methods

    // read_from_farmtxt tests - only meant to be called in context of plant/harvest oriented methods
    #[test]
    fn test_normal_read_from_farmtxt() {
        let result = read_from_farmtxt("farm.txt");
        let success: bool = match result {
            Ok(plant_data) => {
                println!("Plant data: {:?}", plant_data);
                true
            }
            Err(e) => {
                println!("Error: {:?}", e);
                false
            }
        };
        assert!(success);
    }
    #[test]
    fn test_read_from_farmtxt_bad_file() {
        let result = read_from_farmtxt("asdf.txt");
        let success: bool = match result {
            Ok(plant_data) => {
                println!("Plant data: {:?}", plant_data);
                true
            }
            Err(e) => {
                println!("Error: {:?}", e);
                false
            }
        };
        assert!(success == false);
    }

    // write_to_inventorytxt tests - only meant to be called in context of plant/harvest oriented methods

    // read_from_inventorytxt tests - only meant to be called in context of plant/harvest oriented methods
}

// ----------------------------------------
