
use std::fs;
use std::vec::Vec;

// const ENGINE_SIZE: usize = 10;
const ENGINE_SIZE: usize = 10;
const SYMBOL: u32 = 1000;

fn safe_find(engine: &[u32], index: usize) -> bool {
    if index > 0 && index < ENGINE_SIZE * ENGINE_SIZE {
        engine[index] == SYMBOL
    } else {
        false
    }
}

fn main() {
    let file_path = "day3.simple.input";
    
    let mut engine = [0u32; ENGINE_SIZE * ENGINE_SIZE];
    let mut score = 0;

    let mut y = 0;
    for line in fs::read_to_string(file_path).unwrap().lines() {
        let mut x = 0;
        for character in line.chars() {
            if character != '.' {
                engine[ENGINE_SIZE * y + x] += 
                    if character.is_digit(10) { 
                        character.to_digit(10).unwrap() 
                    } else { 
                        SYMBOL
                    };
            }
            x += 1;
        }
        y += 1;
    }

    let mut index = 0;
    for _ in 0.. {
        if index >= ENGINE_SIZE * ENGINE_SIZE { break };

        if engine[index] == SYMBOL {
            // let mut number = 0;
            // let mut number_size = 0;
            // while engine[index] != 0 && engine[index] != SYMBOL {
            //     number = (number * 10) + engine[index];
            //     number_size += 1;
            //     index += 1;
            // }
        }

        index += 1;
    }

    println!("Score: {}", score);
}