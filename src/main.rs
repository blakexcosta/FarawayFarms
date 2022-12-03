use commands::Command;
// use mongodb::{bson::doc, options::ClientOptions, Client};
// This trait is required to use `try_next()` on the cursor
// use futures::{stream::TryStreamExt, io};
use std::collections::HashMap;
use std::io::{Write};
use rand::{Rng, thread_rng};


mod db;
mod colony;
mod planet;
mod coordinates;
mod commands;



async fn get_user_input(commands: &HashMap<String, Command>) {
    println!("RUSTY COLONIES");
    println!("Welcome to Rusty Colonies The Following Commands are Below");
    // start program and get user input 
    let mut choice = String::new();
    while choice.trim() != "quit" {
        choice.clear();
        print!(":> ");
        std::io::stdout().flush().expect("Cannot flush stdout");
        std::io::stdin()
            .read_line(&mut choice)
            .expect("Cannot read user input");
        // example of splitting on commands


        //TODO: ADD REST OF COMMANDS TO MATCH ON. First do terminal commands, then do non terminal commands/action commands. action commands will require code refactor/rework.
        let mut choice_split = choice.split(" ");
        // match on the first name of the command
        match choice_split.next().unwrap().trim() {
            "help" => {commands::iterate_over_commands(commands)},
            "connect_db" => {    // connect to database
                let val = db::db_helper::connect().await.unwrap();
                println!("{:?}", val);
            },
            "citizens_info" => {println!("TODO-Implement")},
            "citizen_action" => {println!("TODO-Implement")},
            "buildings_info" => {println!("TODO-Implement")},
            "colony_info" => {println!("TODO-Implement")},
            "citizen_info" => {println!("TODO-Implement")},
            "market_sell" => {println!("TODO-Implement")},
            "market_buy" => {println!("TODO-Implement")},
            "market_list_orders" => {println!("TODO-Implement")},
            "market_log" => {println!("TODO-Implement")},
            "quit" => {break},
            "q" => {break},
            _ => { println!("nothing matched, please enter new command")}
        }
    }
}

#[tokio::main]
async fn main(){
    // test, rework this for commands, as needed
    let commands = commands::generate_command_hashmap();
    get_user_input(&commands).await;

    // create a new colony
    let mut new_colony = colony::Colony::new(
        "Europa Prime".to_string(),
        thread_rng().gen_range(5..=100)
    );
    // print out the new colony information
    println!("------------------------------------\nCOLONY INFORMATION: ");
    println!("COLONY NAME: {}", new_colony.name);
    println!("COLONY CITIZEN NUMBER: {}", new_colony.citizen_number);
    println!("COLONY CITIZEN LIST:");
    for citizen in &new_colony.citizens {
        println!("\t{:?}", citizen);
    }
    println!("COLONY PLANET:{:?}", new_colony.planet);
    // testing movement of citizens
    // new_colony.move_citizen(CardinalDirection::WEST, 20);
    // new_colony.move_citizen(CardinalDirection::WEST, 20);
    // new_colony.move_citizen(CardinalDirection::EAST, 5);
    // Ok(())


    
}




