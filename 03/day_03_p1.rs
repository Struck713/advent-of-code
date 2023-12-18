
use std::fs;
use std::vec::Vec;
use std::collections::HashSet;

// const ENGINE_SIZE: usize = 140;
const ENGINE_SIZE: usize = 140;
const ENGINE_SIZE_I32: i32 = 140;
const SYMBOL: u32 = 1111;

fn find_offset(engine: &[u32], index: usize, offset: i32) -> bool {
    let new_index = (index as i32) + offset;
    if new_index >= 0 && new_index < ENGINE_SIZE_I32 * ENGINE_SIZE_I32 {
        engine[new_index as usize] != 0
    } else {
        false
    }
}

fn main() {
    let file_path = "day3.input";
    
    let mut engine = [0u32; ENGINE_SIZE * ENGINE_SIZE];

    let mut y = 0;
    for line in fs::read_to_string(file_path).unwrap().lines() {
        let mut x = 0;
        let characters: Vec<char> = line.chars().collect();
        loop {
            if x >= ENGINE_SIZE { break; }
            if characters[x] != '.' {
                if characters[x].is_digit(10) {
                    let starting_index = x;
                    let mut number: u32 = 0;
                    while x < ENGINE_SIZE && characters[x].is_digit(10) {
                        let digit = characters[x].to_digit(10).unwrap();
                        number = (number * 10) + digit;
                        x += 1;
                    }

                    for offset_x in starting_index..x {
                        engine[ENGINE_SIZE * y + offset_x] = number;
                    }

                    x -= 1;
                } else {
                    engine[ENGINE_SIZE * y + x] = SYMBOL;
                }
            }
            x += 1;
        }
        y += 1;
    }


    let mut totals: Vec<u32> = Vec::new();
    let mut index = 0;
    loop {
        if index >= ENGINE_SIZE * ENGINE_SIZE { break };
        
        let mut nums: HashSet<u32> = HashSet::new();
        if engine[index] == SYMBOL {
            if find_offset(&engine, index, 1) { nums.insert(engine[index + 1]); }
            if find_offset(&engine, index, -1) { nums.insert(engine[index - 1]); } 
            if find_offset(&engine, index, ENGINE_SIZE_I32) { nums.insert(engine[index + ENGINE_SIZE]); }
            if find_offset(&engine, index, ENGINE_SIZE_I32 - 1) { nums.insert(engine[index + ENGINE_SIZE - 1]); }
            if find_offset(&engine, index, ENGINE_SIZE_I32 + 1) { nums.insert(engine[index + ENGINE_SIZE + 1]); }
            if find_offset(&engine, index, -ENGINE_SIZE_I32) { nums.insert(engine[index - ENGINE_SIZE]); } 
            if find_offset(&engine, index, -ENGINE_SIZE_I32 - 1) { nums.insert(engine[index - ENGINE_SIZE - 1]); } 
            if find_offset(&engine, index, -ENGINE_SIZE_I32 + 1) { nums.insert(engine[index - ENGINE_SIZE + 1]); } 
        }
        
        for num in nums {
            totals.push(num);
        }

        index += 1;
    }

    println!("Score: {}", totals.iter().sum::<u32>());
}