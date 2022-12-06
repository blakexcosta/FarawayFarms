
use std::collections::HashMap;
use std::io::{Write};
use std::str::Split;

use crate::db::db_helper;
use crate::{colony::Colony, coordinates::CardinalDirection};

#[derive(Debug)]
pub struct Command {
    name: String,
    description: String
}
impl Command {
    pub fn new(_name: String, _description: String) -> Command {
        return Command { name: _name, description: _description};
    }
}

/// This is where you implement and add new commands, the main program will always require this hashmap to know the current populated list of commands
/// This is really in practice only used for the help command. This has largely gone/been un-utilized due to very custom implementations.
/// This could/should change in the future as the choice gotten from the console is the key in this hashmap.
/// Command given
pub fn generate_command_hashmap() -> HashMap<String, Command>{
    let mut commands = HashMap::new();
    commands.insert("help".to_string(), Command::new("help".to_string(), "placeholder".to_string()));
    commands.insert("create_colony".to_string(), Command::new("create_colony".to_string(), "placeholder".to_string()));
    commands.insert("test_db".to_string(), Command::new("test_db".to_string(), "placeholder".to_string()));
    commands.insert("citizens_info".to_string(), Command::new("citizens_info".to_string(), "placeholder".to_string()));
    commands.insert("citizen_action".to_string(), Command::new("citizen_action".to_string(), "placeholder".to_string()));
    commands.insert("buildings_info".to_string(), Command::new("buildings_info".to_string(), "placeholder".to_string()));
    commands.insert("colony_info".to_string(),Command::new("colony_info".to_string(), "placeholder".to_string()));
    commands.insert("citizen_info".to_string(),Command::new("citizen_info".to_string(), "placeholder".to_string()));
    commands.insert("market_sell".to_string(),Command::new("market_sell".to_string(), "placeholder".to_string()));
    commands.insert("market_buy".to_string(),Command::new("market_buy".to_string(), "placeholder".to_string()));
    commands.insert("market_list_orders".to_string(),Command::new("market_list_orders".to_string(), "placeholder".to_string()));
    commands.insert("market_log".to_string(),Command::new("market_log".to_string(), "placeholder".to_string()));
    commands.insert("quit".to_string(),Command::new("quit".to_string(), "placeholder".to_string()));
    commands.insert("q".to_string(),Command::new("q".to_string(), "placeholder".to_string()));
    return commands;
}


// ---------------------------------------------------------------
// COMMAND/TERMINAL FUNCTIONALITY
// i.e. commands that change something about the terminal or provide information to the terminal
// walker, texas hamster
// goodest gobbles steve
pub fn iterate_over_commands(commands: &HashMap<String, Command>){
    for (key, value) in commands.iter() {
        println!("\t{} - {} ", value.name, value.description);
    }
}

pub async fn colony_info(mut choice_split: Split<'_, &str>) -> String{
    // get the colony name from the trimmed information
    let mut colony_name = match choice_split.next(){
        Some(name) => name,
        None => ""
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
pub async fn citizen_info(mut choice_split: Split<'_, &str>) -> String{
    // get the colony name from the trimmed information
    let mut colony_name = match choice_split.next(){
        Some(name) => name,
        None => ""
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
    let colony_info = db_helper::get_colony_information(db, "colony_collection", &colony_name).await;
    
    for citizen_array in colony_info{
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