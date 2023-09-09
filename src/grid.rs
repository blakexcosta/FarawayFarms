
    // grid is 2D Vector of elements

// plantable Trait

/// generates a Vector of X by Y Shape with plantable objects
/// contains optional boolean that will output the results of the vector if desired 
pub fn generate_grid(rows: i32, columns: i32, show_outputted_grid: bool) -> Vec<Vec<i32>> {
    let mut matrix: Vec<Vec<i32>> = vec![vec![0; rows.try_into().unwrap()]; columns.try_into().unwrap()];

    // print grid if needed
    if show_outputted_grid == true{
        println!("{:?}", &matrix);
    }
    //println!("{:?}",&matrix);
    return matrix;
}

pub fn print_grid(matrix: &Vec<Vec<i32>>){
    for row in matrix{
        println!("{:?}",row);
    }
}
pub fn traverse_grid(x: i32, y: i32) {
    // TODO: Implement
    // this will traverse the grid in X (horizontal) and Y (vertical)
}