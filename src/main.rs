use commands::Command;
use std::thread::{self, sleep};
use std::time::{Duration, Instant};
// use mongodb::{bson::doc, options::ClientOptions, Client};
// This trait is required to use `try_next()` on the cursor
// use futures::{stream::TryStreamExt, io};
use crate::db::db_helper;
use rand::{thread_rng, Rng};
use std::collections::HashMap;
use std::io::Write;

mod colony;
mod commands;
mod coordinates;
mod db;
mod planet;

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

        let mut choice_split = choice.split(" ");
        // connect to colony or create a new one.

        // match on the first name of the command
        match choice_split.next().unwrap().trim() {
            "help" => commands::iterate_over_commands(commands),
            "create_colony" => {
                println!("TODO-Implement")
            } // TODO: mongo function
            "test_db" => {
                // tests connection to the database
                let val = db::db_helper::test_connect().await.unwrap();
                println!("{:?}", val);
            }
            "citizens_info" => {
                let val = commands::citizen_info(choice_split).await;
            } // TODO 1: mongo function
            "citizen_action" => {
                println!("TODO-Implement")
            }
            "buildings_info" => {
                println!("TODO-Implement")
            } // TODO 1: mongo function
            "colony_info" => {
                // don't really need to do anything with this right now
                commands::colony_info(choice_split).await;
            }
            "citizen_info" => {
                println!("TODO-Implement")
            } // TODO 1: mongo function
            "market_sell" => {
                println!("TODO-Implement")
            } // TODO 2
            "market_buy" => {
                println!("TODO-Implement")
            } // TODO 2
            "market_list_orders" => {
                println!("TODO-Implement")
            } // TODO 2
            "market_log" => {
                println!("TODO-Implement")
            } // TODO 2
            "quit" => break,
            "q" => break,
            _ => {
                println!("nothing matched, please enter new command")
            }
        }
    }
}

#[tokio::main]
async fn main() {
    // // test tick system/thread sleep system. First to organize my information into a db.
    // let instant_now = Instant::now();
    // let seconds_to_elapse: f64 = 10.0;
    // // we sleep for 2 seconds
    // thread::spawn(move || {
    //     sleep(Duration::new(1, 0));
    //     println!("{}", instant_now.elapsed().as_secs());
    //     while instant_now.elapsed().as_secs_f64() < seconds_to_elapse{
    //         sleep(Duration::new(1, 0));
    //         println!("{}", instant_now.elapsed().as_secs());
    //     }
    // });
    // // it prints '2'
    // println!("{:?} asdf asdf ", instant_now);

    // test, rework this for commands, as needed
    let commands = commands::generate_command_hashmap();
    get_user_input(&commands).await;

    // create a new colony
    let mut new_colony =
        colony::Colony::new("Europa Prime".to_string(), thread_rng().gen_range(5..=100));
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

    // println!("{:?}",now.duration_since(Instant::now()));
    //405811316378249
    //405811316378249
}
