
    // grid is 2D Vector of elements

use std::rc::Rc;

// plantable Trait
use crate::plants::*;

/// generates a Vector of X by Y Shape with plantable objects
/// contains optional boolean that will output the results of the vector if desired 
pub fn generate_grid(rows: i32, columns: i32, show_outputted_grid: bool) -> Vec<Vec<Box<Plant>>> {
    
    //let mut matrix: Vec<Vec<Plantable>> = vec![vec![0; rows.try_into().unwrap()]; columns.try_into().unwrap()];
    let mut matrix: Vec<Vec<Box<Plant>>> =  vec![vec![Box::new(Plant::default()); rows.try_into().unwrap()]; columns.try_into().unwrap()];
    //vec![vec![Box::new(Plant::default()); rows.try_into().unwrap()]; columns.try_into().unwrap()];
    // print grid if needed
    if show_outputted_grid{
        println!("{:?}", &matrix);
    }
    //println!("{:?}",&matrix);
    return matrix;
}

pub fn print_grid(matrix: &Vec<Vec<Box<Plant>>>){
    for row in matrix{
        println!("{:?}",row);
    }
}
pub fn traverse_grid(x: i32, y: i32) {
    // TODO: Implement
    // this will traverse the grid in X (horizontal) and Y (vertical)
}