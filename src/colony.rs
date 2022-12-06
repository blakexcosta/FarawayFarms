use names::Generator;
use rand::{Rng, thread_rng};
use crate::planet::{Planet, Biome};



// STRUCTS AND HELPER FUNCTIONS FOR CITIZEN/COLONY SETUP. 
// this is not intended for functionality besides initial state management/setup for a colony and its citizens, all command functionality are implemented via
// the commands.rs file

#[derive(Debug)]
pub enum CitizenOccupation{
    Farmer,
    Soldier,
    Engineer,
}

#[derive(Debug)]
pub struct Citizen {
    name: String,
    age: u32,
    occupation: CitizenOccupation,
    hp: u32,
    hunger: u32,
    thirst: u32,
    x_location: i32,
    y_location: i32,
}
pub struct Colony {
    pub name: String,
    pub planet: Planet,
    pub citizen_number: u32,
    pub citizens: Vec<Citizen>,

}
impl Colony {
    /// create a new colony with name, number of citizens, and citizen occupations
    pub fn new(_name: String, _citizen_number: u32) -> Colony{
        // generate a new planet
        let mut rand_planet_name = Generator::default().next().to_owned().unwrap().to_uppercase();
        let mut rand_planet_size = thread_rng().gen_range(500..=2000);
        let _planet = Planet::new(rand_planet_name, Biome::Desert, rand_planet_size, rand_planet_size);
        return Colony{ 
            name: _name,
            planet: _planet,
            citizen_number: _citizen_number,
            citizens: Self::generate_citizens(_citizen_number),
        };
    }

/*
/** 
* Paste one or more documents here
*/
{
  "_id": {
    "$oid": "638e9ab93efafdbb3caead7d"
  }
  "colony_name": "Europa-Prime",
  "colony_citizen_number": 30,
  "citizens": [ 
    { "name": "toothsome-geese"},
  ]
}
*/

    /*
         { name: "toothsome-geese", age: 34, occupation: Engineer, hp: 85, hunger: 62, thirst: 1, x_location: 0, y_location: 0 }
         { name: "halting-balls", age: 38, occupation: Soldier, hp: 63, hunger: 87, thirst: 3, x_location: 0, y_location: 0 }
         { name: "tiresome-selection", age: 96, occupation: Engineer, hp: 97, hunger: 29, thirst: 98, x_location: 0, y_location: 0 }
         { name: "foregoing-field", age: 27, occupation: Engineer, hp: 81, hunger: 35, thirst: 95, x_location: 0, y_location: 0 }
         { name: "hospitable-route", age: 24, occupation: Engineer, hp: 72, hunger: 46, thirst: 50, x_location: 0, y_location: 0 }
         { name: "lethal-giraffe", age: 22, occupation: Engineer, hp: 58, hunger: 88, thirst: 21, x_location: 0, y_location: 0 }
         { name: "wakeful-monkey", age: 89, occupation: Soldier, hp: 93, hunger: 95, thirst: 19, x_location: 0, y_location: 0 }
         { name: "four-watch", age: 73, occupation: Farmer, hp: 98, hunger: 100, thirst: 28, x_location: 0, y_location: 0 }
         { name: "difficult-lawyer", age: 77, occupation: Farmer, hp: 53, hunger: 67, thirst: 0, x_location: 0, y_location: 0 }
         { name: "disastrous-eggs", age: 90, occupation: Engineer, hp: 99, hunger: 51, thirst: 95, x_location: 0, y_location: 0 }
         { name: "curvy-transport", age: 38, occupation: Farmer, hp: 72, hunger: 26, thirst: 17, x_location: 0, y_location: 0 }
         { name: "violent-reason", age: 54, occupation: Farmer, hp: 51, hunger: 88, thirst: 6, x_location: 0, y_location: 0 }
         { name: "misty-thread", age: 42, occupation: Soldier, hp: 73, hunger: 44, thirst: 52, x_location: 0, y_location: 0 }
         { name: "nauseating-houses", age: 69, occupation: Engineer, hp: 62, hunger: 80, thirst: 31, x_location: 0, y_location: 0 }
         { name: "shaggy-writing", age: 24, occupation: Farmer, hp: 83, hunger: 99, thirst: 63, x_location: 0, y_location: 0 }
         { name: "scintillating-song", age: 36, occupation: Soldier, hp: 58, hunger: 85, thirst: 41, x_location: 0, y_location: 0 }
         { name: "voracious-acoustics", age: 67, occupation: Engineer, hp: 86, hunger: 49, thirst: 91, x_location: 0, y_location: 0 }
         { name: "shut-spring", age: 86, occupation: Engineer, hp: 64, hunger: 82, thirst: 87, x_location: 0, y_location: 0 }
         { name: "easy-top", age: 8, occupation: Farmer, hp: 76, hunger: 26, thirst: 18, x_location: 0, y_location: 0 }
         { name: "loutish-food", age: 55, occupation: Farmer, hp: 98, hunger: 100, thirst: 75, x_location: 0, y_location: 0 }
         { name: "long-term-insect", age: 95, occupation: Soldier, hp: 86, hunger: 73, thirst: 19, x_location: 0, y_location: 0 }
    */

    /// Private function to generate citizens, to be used only as 
    /// a helper to Colony::new()
    fn generate_citizens(_citizen_number: u32) -> Vec<Citizen>{
        let mut citizen_vec = Vec::new();
        let mut counter = 0;
        // generate new citizens
        let mut generator = Generator::default();
        let mut rng = thread_rng();
        let mut rng_occupation = thread_rng();
        while counter < _citizen_number{
            // generate random name, age (less than 100), occupation
            // generate a random occupation
            let random_occupation = match rng_occupation.gen_range(0..=2) {
                0 => CitizenOccupation::Farmer,
                1 => CitizenOccupation::Soldier,
                2 => CitizenOccupation::Engineer,
                _ => panic!("RNG Out Of Range Citizen Occupation generation in colony.rs")
            };
            // populate the list of random citizens
            citizen_vec.push(Citizen{ 
                name: generator.next().to_owned().unwrap(), 
                age: rng.gen_range(0..=100),
                occupation: random_occupation,
                hp: rng.gen_range(50..=100),
                hunger: rng.gen_range(0..=100),
                thirst: rng.gen_range(0..=100),
                x_location: 0, // all citizens start at 0,0
                y_location: 0, // all citizens start at 0,0
            });
            counter = counter + 1;
        }
        return citizen_vec;
    }


}