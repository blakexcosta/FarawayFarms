
use std::collections::HashMap;

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
/// 
pub fn generate_command_hashmap() -> HashMap<String, Command>{
    let mut commands = HashMap::new();
    commands.insert("help".to_string(), Command::new("help".to_string(), "placeholder".to_string()));
    commands.insert("citizens_info".to_string(), Command::new("citizens_info".to_string(), "placeholder".to_string()));
    commands.insert("citizen_action".to_string(), Command::new("citizen_action".to_string(), "placeholder".to_string()));
    commands.insert("buildings_info".to_string(), Command::new("buildings_info".to_string(), "placeholder".to_string()));
    commands.insert("colony_info".to_string(),Command::new("colony_info".to_string(), "placeholder".to_string()));
    commands.insert("citizen_info".to_string(),Command::new("citizen_info".to_string(), "placeholder".to_string()));
    commands.insert("market_sell".to_string(),Command::new("market_sell".to_string(), "placeholder".to_string()));
    commands.insert("market_buy".to_string(),Command::new("market_buy".to_string(), "placeholder".to_string()));
    commands.insert("market_list_orders".to_string(),Command::new("market_list_orders".to_string(), "placeholder".to_string()));
    commands.insert("market_log".to_string(),Command::new("market_log".to_string(), "placeholder".to_string()));
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