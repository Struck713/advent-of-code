
use std::fs;
use std::vec::Vec;
use std::collections::HashSet;

fn main() {
    let file_path = "day11.simple.input";
    let file = fs::read_to_string(file_path).unwrap();
    let data = file.lines().collect::<Vec<&str>>();

    // map out file to a 2d vector
    let mut matrix: Vec<Vec<char>> = data.iter()
                                     .map(|x| x.chars().collect::<Vec<char>>())
                                     .collect::<Vec<Vec<char>>>();


    let mut columns: HashSet<usize> = HashSet::new();
    let mut hashes: Vec<(usize, usize)> = Vec::new();
    let mut hash_index = 0;
    for row_index in 0..matrix.len() {
        for column_index in 0..matrix[row_index].len() {
            let character = matrix[row_index][column_index];
            if character == '#' {
                columns.insert(column_index);
                hashes.push((column_index, row_index));
            }
        }
    }

    for row_index in 0..matrix.len() {
        let mut column_index = 0;
        loop {
            if column_index >= matrix[row_index].len() {
                break;
            }
            if !columns.iter().any(|x| x == &column_index) {
                matrix[row_index].insert(column_index - 1, '.');
                column_index += 1;
            }
            column_index += 1;
        }
    }

    for row_index in 0..matrix.len() {
        for column_index in 0..matrix[row_index].len() {
            print!("{}", matrix[row_index][column_index]);
        }
        println!();
    }

    let mut score = 0;
    println!("Score: {score}");
}