
use std::fs;
use std::vec::Vec;

fn main() {
    let file_path = "day9.input";
    let file = fs::read_to_string(file_path).unwrap();
    
    let mut score: i32 = 0;
    for line in file.lines() {
        let initial_layer = line.split_whitespace()
                            .into_iter()
                            .map(|s| s.parse::<i32>().unwrap())
                            .collect::<Vec<i32>>();

        let mut layers: Vec<Vec<i32>> = Vec::new();
        layers.push(initial_layer);

        // build tables
        let mut layer = layers.last().unwrap();
        while !layer.iter().all(|x| x == &0) {
            let mut next_layer: Vec<i32> = Vec::new();
            for index in 1..layer.len() {
                let last = layer[index - 1];
                let current = layer[index];
                next_layer.push(current - last);
            }
            layers.push(next_layer);
            layer = layers.last().unwrap();
        }

        // solve table
        let mut value_to_solve = 0;
        for layer in layers.iter().rev() {
            value_to_solve = value_to_solve + layer.last().unwrap();
        }
        score += value_to_solve;
    }

    println!("Score: {score}");
}