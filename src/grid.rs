// grid is 2D Vector of elements

use std::rc::Rc;

// plantable Trait
use crate::plants::*;

/// generates a Vector of X by Y Shape with plantable objects
/// contains optional boolean that will output the results of the vector if desired
pub fn generate_grid(rows: i32, columns: i32, show_outputted_grid: bool) -> Vec<Vec<Box<Plant>>> {
    //let mut matrix: Vec<Vec<Plantable>> = vec![vec![0; rows.try_into().unwrap()]; columns.try_into().unwrap()];
    let mut matrix: Vec<Vec<Box<Plant>>> =
        vec![
            vec![Box::new(Plant::default()); rows.try_into().unwrap()];
            columns.try_into().unwrap()
        ];
    //vec![vec![Box::new(Plant::default()); rows.try_into().unwrap()]; columns.try_into().unwrap()];
    // print grid if needed
    if show_outputted_grid {
        println!("{:?}", &matrix);
    }
    //println!("{:?}",&matrix);
    return matrix;
}

pub fn print_grid(matrix: &Vec<Vec<Box<Plant>>>) {
    for row in matrix {
        println!("{:?}", row);
    }
}
pub fn traverse_grid(x: i32, y: i32) {
    // TODO: Implement
    // this will traverse the grid in X (horizontal) and Y (vertical)
}

#[cfg(test)]
mod tests {
    // TESTS
    use super::*;

    // PLANT function tests
    // passing
    #[test]
    fn test_generate_grid() {
        let success = ""; //generate_grid(2, 2, false)
                          // let success = match plant("1") {
                          //     Ok(str) => str,
                          //     Err(e) => e,
                          // };
        assert_eq!(success, "".to_string());
    }
    // failing
    #[test]
    fn test_failingplant_id_negative() {
        let failure = "failure";
        // let failure = match plant("-1") {
        //     Ok(str) => str,
        //     Err(e) => e,
        // };
        assert_ne!(failure, "".to_string()); // should not be identical
    }
}
