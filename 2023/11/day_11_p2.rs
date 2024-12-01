
use std::fs;
use std::vec::Vec;
use std::collections::HashSet;

fn main() {
    let file_path = "day11.input";
    let file = fs::read_to_string(file_path).unwrap();
    let data = file.lines().collect::<Vec<&str>>();

    // map out file to a 2d vector
    let matrix: Vec<Vec<char>> = data.iter()
                                     .map(|x| x.chars().collect::<Vec<char>>())
                                     .collect::<Vec<Vec<char>>>();

    // set of all columns that have a hash in them
    let mut columns: HashSet<usize> = HashSet::new();
    // set of all rows that don't have a hash in them
    let mut rows: HashSet<usize> = HashSet::new();
    for row_index in 0..matrix.len() {
        let mut dot_count = 0;
        for column_index in 0..matrix[row_index].len() {
            let character = matrix[row_index][column_index];
            if character == '#' {
                columns.insert(column_index);
            } else {
                dot_count += 1;
            }
        }
        if dot_count == matrix[row_index].len() {
            rows.insert(row_index);
        }
    }

    
    // vector of all hash locations
    let mut hashes: Vec<(usize, usize)> = Vec::new();
    for row_index in 0..matrix.len() {
        for column_index in 0..matrix[row_index].len() {
            if matrix[row_index][column_index] == '#' {
                hashes.push((row_index, column_index));
            }
        }
    }
    
    const SIZE_OF_EMPTY: u64 = 1000000;
    let mut score: u64 = 0;
    for row_index in 0..hashes.len() {
        let (row_1, column_1) = hashes[row_index];
        for column_index in 0..row_index {
            let (row_2, column_2) = hashes[column_index];
            for row in usize::min(row_1, row_2)..usize::max(row_1, row_2) {
                score += if rows.contains(&row) { SIZE_OF_EMPTY } else { 1 };
            }
            for column in usize::min(column_1, column_2)..usize::max(column_1, column_2) {
                score += if !columns.contains(&column) { SIZE_OF_EMPTY } else { 1 };
            }
        }
    }

    println!("Score: {}", score);
}