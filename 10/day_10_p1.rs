
use std::fs;
use std::vec::Vec;

fn main() {
    let file_path = "day10.simple.input";
    let file = fs::read_to_string(file_path).unwrap();
    let data = file.lines().collect::<Vec<&str>>();

    // map out file to a 2d vector
    let matrix: Vec<Vec<char>> = data.iter()
                                     .map(|x| x.chars().collect::<Vec<char>>())
                                     .collect::<Vec<Vec<char>>>();

    // for rows in matrix {
    //     for column in rows {
    //         print!("{column}");
    //     }
    //     print!("\n");
    // }
    
    let mut score: i32 = 0;
    println!("Score: {score}");
}