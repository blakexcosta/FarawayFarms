#![allow(warnings)] // disable warnings for time

use crate::commands::{harvest, plant};
use crate::plants::{Plant, PlantTypes};
use commands::Command;
use std::thread::{self, sleep};
use std::time::{Duration, Instant};
// use mongodb::{bson::doc, options::ClientOptions, Client};
// This trait is required to use `try_next()` on the cursor
// use futures::{stream::TryStreamExt, io};
use crate::db::db_helper;
use rand::{thread_rng, Rng};
use rascii_art::{render_to, RenderOptions};
use std::collections::HashMap;
use std::io::Write;
use grid::*;

mod grid;
mod colony;
mod commands;
mod coordinates;
mod db;
mod planet;
mod plants;
mod ui;

/*
BACKLOG:
[]- Generate list of plantable plants + their harvest times with
[]- harvestall() command. Harvests all plants on the farm. Example command is: `harvestall 1`. This would harvest all plants that have an id of 1 (i.e zuccinis)
[] - Garden/animal ascii art
    []- print out information on specific plant + prints out rascii art associated with said plant.
[]- add localized, compact sqlite/sqlx db. Otherwise txt files gonna be huge eventually.
[]- Remote saves (sqlite/sqlx/postgres)?...
[] - Fix tests so they don't write to farm.txt and inventory.txt (lookup rust staging area for tests?). Happens in plant + harvest methods
    [] - Fix plant tests
    [] - Fix harvest tests
Player-driven-market
    []- Market where can sell plants.
Expanded local saves
[] - https://github.com/ratatui-org/ratatui alpha implementation


IN PROGRESS:
[X] - Grid System
    [X] - Design grids in x by x area
    [X] - Create grid.rs file
    [X] - Map out interfaces
        [NOPE] - Add Plantable Trait to vector of grid.rs plants
    [X] - Implement accordingly
        [X] - Implement traverse_grid
        [X] - Create update_grid method to change the plant in there.
        [X] - Stream Time!
        [X] - render grid to the screen in this format, with X's indicating planted item
            [_][X][_]
            [_][X][_]
            [_][_][X]
        [X] - clean up grid code + misc code as needed so it works well together
[] - Inventory System
    [] - Design + initial research, Write tasks
    [] - Other Task -> dictated by Task Above. 
[] - TUI: 
    [] - Review TUI code
    [] - Layout for homescreen TUI
    [] - Review state management from rat-test, it is inside app.rs and ui.rs. You can see this code in action in ratatui folder. Then running  `cargo run --example demo`. I just copied it into rat-test for reference for myself.
    [] - fix showgrid, remove hardcoded placeholder.

------------------------------------
Alpha:
[X] - Grid
[] - Inventory
Water Generation
Money/Value
Plant
Watering
Harvest
[] - RASCII alpha placeholder implementation with plants
    [] - `showplant id`
    [] - display the plant at its current stage
[X] - TUI - Experimentation (ratatui-org/ratatui: Rust library to build rich terminal user interfaces (TUIs) and dashboards (github.com)
[X] - RASCII - Experimentation


Beta:
Signup
Login
Upgrade
Market
In Game Tutorial/Online Docs


Release:
Signup for new accounts
login capabilities via terminal
username x password
password/account recovery
Ability to plant items
maintaining items via terminal (watering)
Water Absorption rates for plants
Fertilizers for plants
Ability to harvest items for value
value/money system
ascii art in terminal for each plant
docs for each plant
art for each stage of plant life cycle
realtime player driven market (Rocket API/Backend with Postgres/Mongo)
buy orders (`buy zuccini 2dollars at price of )
sell orders (`sell zuccini 2dollars 5`)
Farm size/grid (plant from grid, harvest from grid)
10x10 grid to start
------------------------------------

*/
// reviewing some docs
async fn get_user_input(commands: &HashMap<String, Command>) {
    println!("FARAWAY FARMS REMOTE MANAGEMENT TERMINAL");
    println!("Welcome to Faraway Farms Remote Management Terminal! Type your commands are Below:");
    // start program and get user input
    let mut choice = String::new();
    let mut grid: Vec<Vec<Box<Plant>>> = Vec::new();
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
        match choice_split.next().unwrap().to_lowercase().trim() {
            "help" => commands::iterate_over_commands(commands),
            "plant" => {
                match choice_split.next() {
                    Some(plantid) => {
                        // catch error from plant function
                        match plant(plantid) {
                            Ok(_) => (),
                            Err(e) => println!("{}", e),
                        }
                    },
                    None => println!(
                        "No plantID was given, please give in the format of `plant <plantid>` ex: plant 1"
                    )
                }
            }
            "harvest" => {
                match choice_split.next() {
                    Some(plantid) => {
                        // catch error from harvest function
                        match harvest(plantid) {
                            Ok(_) => (),
                            Err(e) => println!("{}", e),
                        }
                    },
                    None => println!(
                        "No plantID was given, please give in the format of `harvest <plantid>` ex: plant 1"
                    )
                };
            }
            "render" => {
                match choice_split.next() {
                    Some(filepath) => {
                        // catch error from harvest function
                        
                        let mut buffer = String::new();

                        render_to(
                            //r"/path/to/image.png",
                            //r".\resources\castle.png",
                            filepath.trim(),
                            &mut buffer,
                            &RenderOptions::new()
                                .width(50) // recommend width of 50, 75 or 100
                                //.height(60)
                                .colored(true)
                                //.charset(&["."])
                                //.charset(&[".", "-", "|", "_", "#", "=",]),
                                // .charset(&[".", ",", "-", "*", "|", "_", "#", "=", "<",">","/","L"]),
                                // .charset(&[".", ",", "-", "*", "£", "$", "#"]),
                        )
                        .unwrap();

                        println!("{}", buffer);
                    },
                    None => println!(
                        "No image file path was given, please give in the format of `render <filepath>` \nex:\n\t`render ./resouces/dogwater_beach.jpg`\nOR\n\t`render .\\resouces\\dogwater_beach.jpg`"
                    )
                };
            }
            "grid" => {
                // get x
                match choice_split.next() {
                    Some(x_val) => {
                        // get y
                        match choice_split.next() {
                            Some(y_val) => {
                                //println!("y_val: {}", y_val);
                                let row: i32 = match x_val.trim().parse() {
                                    Ok(val) => val,
                                    Err(_) => {
                                        println!("Invalid x_val");
                                        0
                                    }
                                };
                                let column: i32 = match y_val.trim().parse() {
                                    Ok(val) => val,
                                    Err(_) => {
                                        println!("Invalid y_val");
                                        0
                                    }
                                };
                                // check values are valid
                                if row == 0 || column == 0 {
                                }
                                else{
                                    // generate grid
                                    grid = grid::generate_grid(row, column, false);
                                    println!("NEW GRID:");
                                    grid::print_grid(&grid);
                                    //println!("{:?}\n",grid::traverse_grid((row).try_into().unwrap(), (column).try_into().unwrap(), &grid));
                                    // grid::update_grid(1, 1, &mut grid, Plant{
                                    //     id: 50,
                                    //     name: "test".to_string(),
                                    //     plant_type: PlantTypes::Zuccini,
                                    //     planted_timestamp: 0,
                                    //     growth_time_sec: 0,
                                    //     harvested_timestamp: 0
                                    // });
                                    // println!("\n");
                                    // grid::print_grid(&grid);
                                }
                            }
                            None => println!("No y_val given")
                        }
                    },
                    None => println!("No x_val given")
                };
            }
            "showgrid" => {
                // show the generated grid
                // check the length is greater than 0
                if grid.len() > 0 {
                    let mut new_plant = Plant::default();
                    new_plant.id = 1;
                    new_plant.plant_type = PlantTypes::Peach;
                    grid::update_grid(1, 2, &mut grid, new_plant);
                    grid::print_grid_simplified(&grid);
                }
                else {
                    println!("Grid is empty");
                }
            }
            "ui" => {
                // show the generated grid
                // check the length is greater than 0
                // match choice_split.next() {
                //     Some(x_val) => {
                //     },
                //     None => println!("No x_val given")
                // };
                println!("UI TBD");
                match ui::ui_start() {
                    Ok(_) => (),
                    Err(e) => println!("ERROR SPINNING UP UI:{:?}", e),
                };
            }
            "citizens_info" => {
                let val = commands::citizen_info(choice_split).await;
            }
            // TODO: mongo function
            // "create_colony" => {
            //     println!("TODO-Implement")
            // } // TODO: mongo function
            // "test_db" => {
            //     // tests connection to the database
            //     let val = db::db_helper::test_connect().await.unwrap();
            //     println!("{:?}", val);
            // }
            // "citizens_info" => {
            //     let val = commands::citizen_info(choice_split).await;
            // } // TODO 1: mongo function
            // "citizen_action" => {
            //     println!("TODO-Implement")
            // }
            // "buildings_info" => {
            //     println!("TODO-Implement")
            // } // TODO 1: mongo function
            // "colony_info" => {
            //     // don't really need to do anything with this right now
            //     commands::colony_info(choice_split).await;
            // }
            // "citizen_info" => {
            //     println!("TODO-Implement")
            // } // TODO 1: mongo function
            // "market_sell" => {
            //     println!("TODO-Implement")
            // } // TODO 2
            // "market_buy" => {
            //     println!("TODO-Implement")
            // } // TODO 2
            // "market_list_orders" => {
            //     println!("TODO-Implement")
            // } // TODO 2
            // "market_log" => {
            //     println!("TODO-Implement")
            // } // TODO 2
            "quit" => break,
            "q" => break,
            _ => {
                println!("nothing matched, please enter new command");
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
