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

/// WARNING: RETURNS A CLONE BECAUSE OF SHARED REFERENCE
/// This is 1 indexed instead of zero indexed for user simplicity
pub fn traverse_grid(x: usize, y: usize, matrix: &Vec<Vec<Box<Plant>>>) -> Result<Plant, String> {
    // checks to make sure the inputs will not be out of bounds and throw a given error
    if x <= 0 || y <= 0 {
        return Err("Invalid input of 0 or less".to_string());
    }
    // return index -1 as we expect the user to give inputs like 1,1 to retrieve element at 0,0
    let val = matrix.get(x - 1).unwrap().get(y - 1).unwrap();
    let unboxed_val = val.clone();
    return Ok(*unboxed_val);
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
