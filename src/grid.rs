
    // grid is 2D Vector of elements

// plantable Trait

// generates a Vector of X by Y Shape with plantable objects
pub fn generate_grid(rows: i32, columns: i32){
    let mut matrix: Vec<Vec<i32>> = vec![vec![0; rows.try_into().unwrap()]; columns.try_into().unwrap()];
    println!("{:?}",&matrix);
}

pub fn traverse_grid(x: i32, y: i32) {
    // TODO: Implement
    // this will traverse the grid in X (horizontal) and Y (vertical)
}