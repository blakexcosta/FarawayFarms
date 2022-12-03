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